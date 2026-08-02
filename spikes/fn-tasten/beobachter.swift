// KRK — Prüfprogramm zur Fn-Tasten-Annahme aus C3 des Navigator-Specs.
//
// WEGWERF-PRÜFCODE. Kein Produktcode. Keine Tests, keine Architektur, keine
// Fehlerbehandlung über das Nötigste hinaus. Die Frage, die dieses Programm
// beantwortet, und die Bedienung stehen in README.md daneben.
//
// Das Programm schreibt jedes keyDown- und flagsChanged-Ereignis mit, das eine
// gewöhnliche Anwendung im Vordergrund erhält, und wertet die vorgegebene
// Tastenfolge am Ende automatisch aus.

import AppKit
import Foundation

// ---------------------------------------------------------------------------
// Nachschlagetabellen
// ---------------------------------------------------------------------------

/// Virtuelle Tastencodes, die in dieser Messung vorkommen können.
/// Quelle: Carbon HIToolbox, Events.h (kVK_*).
let tastenNamen: [UInt16: String] = [
    122: "F1", 120: "F2", 99: "F3", 118: "F4", 96: "F5", 97: "F6",
    98: "F7", 100: "F8", 101: "F9", 109: "F10", 103: "F11", 111: "F12",
    63: "fn", 56: "Shift links", 60: "Shift rechts",
    59: "Control links", 62: "Control rechts",
    58: "Option links", 61: "Option rechts",
    55: "Command links", 54: "Command rechts",
    57: "CapsLock", 0: "a", 11: "b", 8: "c",
]

func tastenName(_ code: UInt16) -> String {
    tastenNamen[code] ?? "unbenannt"
}

func modifikatorenText(_ flags: NSEvent.ModifierFlags) -> String {
    var teile: [String] = []
    if flags.contains(.capsLock) { teile.append("capsLock") }
    if flags.contains(.shift) { teile.append("shift") }
    if flags.contains(.control) { teile.append("control") }
    if flags.contains(.option) { teile.append("option") }
    if flags.contains(.command) { teile.append("command") }
    if flags.contains(.numericPad) { teile.append("numericPad") }
    if flags.contains(.help) { teile.append("help") }
    if flags.contains(.function) { teile.append("function") }
    return teile.isEmpty ? "keine" : teile.joined(separator: "+")
}

/// Druckbare Zeichen als 'x', alles andere als Unicode-Skalar.
func zeichenText(_ text: String?) -> String {
    guard let text, !text.isEmpty else { return "(leer)" }
    return text.unicodeScalars.map { skalar in
        if skalar.value >= 0x20, skalar.value < 0x7F {
            return "'\(Character(skalar))'"
        }
        return String(format: "U+%04X", skalar.value)
    }.joined(separator: " ")
}

/// Liest die Systemeinstellung "F1, F2 usw. als Standard-Funktionstasten
/// verwenden" aus demselben Bereich, den `defaults read -g` liest.
func fnStateText() -> String {
    let wert = CFPreferencesCopyAppValue(
        "com.apple.keyboard.fnState" as CFString,
        kCFPreferencesAnyApplication
    )
    guard let wert else {
        return "nicht gesetzt — Systemvorgabe, also AUS (nackte F-Tasten steuern Hardware)"
    }
    if let zahl = wert as? NSNumber {
        return zahl.boolValue
            ? "1 — AN (nackte F-Tasten sind gewöhnliche Funktionstasten)"
            : "0 — AUS (nackte F-Tasten steuern Hardware)"
    }
    return "unerwarteter Wert: \(wert)"
}

// ---------------------------------------------------------------------------
// Aufgezeichnetes Ereignis
// ---------------------------------------------------------------------------

struct Ereignis {
    let nummer: Int
    let sekundenSeitStart: TimeInterval
    let uhrzeit: String
    let art: String
    let tastencode: UInt16
    let zeichen: String
    let zeichenOhneModifikatoren: String
    /// Ungefiltert, für den Markerabgleich (a, b, c).
    let rohZeichenOhneModifikatoren: String
    let flags: NSEvent.ModifierFlags
    let rohflags: UInt
    let veraenderung: String
    let istWiederholung: Bool
    /// War die fn-Taste körperlich gedrückt, als dieses Ereignis eintraf?
    /// Wird nachträglich aus der Umschaltspur der Taste 63 gefüllt, nie aus
    /// dem mod=-Feld. Siehe `fnZustandEintragen`.
    var fnGehalten = false

    var zeile: String {
        let kopf = String(
            format: "#%02d  %6.2fs  %@  %-12@  code=%3d (%@)",
            nummer, sekundenSeitStart, uhrzeit, art as NSString,
            Int(tastencode), tastenName(tastencode) as NSString
        )
        var rest = "  mod=\(modifikatorenText(flags))  roh=" +
            String(format: "0x%08lX", rohflags)
        if art == "keyDown" {
            rest = "  zeichen=\(zeichen)  ohneMod=\(zeichenOhneModifikatoren)" + rest
            if istWiederholung { rest += "  (Wiederholung)" }
        } else {
            rest += "  geändert=\(veraenderung)"
        }
        return kopf + rest
    }
}

// ---------------------------------------------------------------------------
// Auswertung
//
// Steht frei und nicht im Fensterobjekt, damit dieselbe Auswertung auch ohne
// neue Messung über ein bereits geschriebenes Protokoll laufen kann
// (`./beobachter --auswerten messung-A.txt`). Eine Auswertung, die sich nur
// beim Messen ergibt, ließe sich nicht nachprüfen.
// ---------------------------------------------------------------------------

/// Trägt in jedes Ereignis ein, ob die fn-Taste in diesem Moment körperlich
/// gedrückt war. Maßgeblich ist allein die Umschaltspur der Taste 63
/// (`flagsChanged` +function / -function).
///
/// Das mod=-Feld eines keyDown taugt dafür ausdrücklich NICHT: AppKit setzt
/// `function` bei jeder Taste aus dem Funktionstasten-Unicodebereich, also auch
/// bei einer nackten F3 ohne gedrückte fn-Taste. Wer den Zustand aus dem
/// keyDown liest, liest ihn falsch.
func fnZustandEintragen(_ liste: [Ereignis]) -> [Ereignis] {
    var gedrueckt = false
    return liste.map { ereignis in
        if ereignis.art == "flagsChanged", ereignis.tastencode == 63 {
            gedrueckt = ereignis.flags.contains(.function)
        }
        var kopie = ereignis
        kopie.fnGehalten = gedrueckt
        return kopie
    }
}

/// Index des ersten keyDown, dessen Zeichen ohne Modifikatoren `marke` ist.
func markenIndex(_ liste: [Ereignis], _ marke: String) -> Int? {
    liste.firstIndex {
        $0.art == "keyDown" && $0.rohZeichenOhneModifikatoren.lowercased() == marke
    }
}

func abschnitt(_ liste: [Ereignis], _ von: Int?, _ bis: Int?) -> [Ereignis] {
    guard let von else { return [] }
    let ende = bis ?? liste.count
    guard von + 1 <= ende, ende <= liste.count else { return [] }
    return Array(liste[(von + 1)..<ende])
}

func abschnittsBericht(_ teil: [Ereignis]) -> String {
    if teil.isEmpty { return "  Kein einziges Ereignis empfangen.\n" }
    var zeilen = ""
    let tasten = teil.filter { $0.art == "keyDown" && !$0.istWiederholung }
    if tasten.isEmpty {
        zeilen += "  Kein keyDown empfangen.\n"
    } else {
        for t in tasten {
            zeilen += "  keyDown  code=\(t.tastencode) (\(tastenName(t.tastencode)))"
            zeilen += "  zeichen=\(t.zeichenOhneModifikatoren)"
            zeilen += "  mod=\(modifikatorenText(t.flags))"
            zeilen += "  fn=\(t.fnGehalten ? "gehalten" : "frei")\n"
        }
    }
    let flaggen = teil.filter { $0.art == "flagsChanged" }
    if flaggen.isEmpty {
        zeilen += "  Kein flagsChanged empfangen.\n"
    } else {
        for f in flaggen {
            zeilen += "  flagsChanged  code=\(f.tastencode) (\(tastenName(f.tastencode)))"
            zeilen += "  geändert=\(f.veraenderung)  mod=\(modifikatorenText(f.flags))\n"
        }
    }
    return zeilen
}

/// Der Teil des Berichts ab "## Auswertung nach Abschnitten". Bekommt die rohe
/// Ereignisliste und den Zustand der Systemeinstellung als Text.
func auswertung(_ roheListe: [Ereignis], fnZustand: String) -> String {
    let ereignisse = fnZustandEintragen(roheListe)

    let iA = markenIndex(ereignisse, "a")
    let iB = markenIndex(ereignisse, "b")
    let iC = markenIndex(ereignisse, "c")

    let mitFn = abschnitt(ereignisse, iA, iB)
    let nackt = abschnitt(ereignisse, iB, iC)
    let modifikatoren = abschnitt(ereignisse, iC, nil)

    var text = """

    ## Auswertung nach Abschnitten

    Marken gefunden: a=\(iA.map(String.init) ?? "FEHLT"), \
    b=\(iB.map(String.init) ?? "FEHLT"), c=\(iC.map(String.init) ?? "FEHLT")

    ### Abschnitt 1 (zwischen 'a' und 'b') — gedrückt wurde Fn+F3, Fn+F5, Fn+F8

    \(abschnittsBericht(mitFn))
    ### Abschnitt 2 (zwischen 'b' und 'c') — gedrückt wurde F3, F5, F8 ohne Fn

    \(abschnittsBericht(nackt))
    ### Abschnitt 3 (nach 'c') — gedrückt wurde Fn allein, dann Shift allein

    \(abschnittsBericht(modifikatoren))

    """

    // Funktionstasten eines Abschnitts, getrennt danach, ob fn dabei körperlich
    // gehalten wurde. Genau diese Trennung entscheidet Frage 1 gegen Frage 2:
    // ohne sie zählt ein mit fn wiederholter Abschnitt 2 als Messung der
    // nackten F-Tasten.
    let fTasten: ([Ereignis], Bool) -> [Ereignis] = { teil, mitGehaltenerFn in
        teil.filter {
            $0.art == "keyDown"
                && tastenName($0.tastencode).hasPrefix("F")
                && $0.fnGehalten == mitGehaltenerFn
        }
    }
    let fTastenMitFn = fTasten(mitFn, true)
    let fTastenMitFnAberOhne = fTasten(mitFn, false)
    let fTastenNackt = fTasten(nackt, false)
    let fTastenNacktAberMitFn = fTasten(nackt, true)

    let fnFlaggen = modifikatoren.filter {
        $0.art == "flagsChanged" && $0.veraenderung.contains("function")
    }
    let shiftFlaggen = modifikatoren.filter {
        $0.art == "flagsChanged" && $0.veraenderung.contains("shift")
    }

    // Eine fehlende Trennmarke bedeutet, dass der Abschnitt nie gedrückt
    // wurde. Ein solcher Abschnitt darf NICHT als "nein" gelesen werden,
    // sonst liest sich ein abgebrochener Durchgang wie ein Messergebnis.
    let trefferListe: ([Ereignis]) -> String = { treffer in
        treffer.map {
            "\(tastenName($0.tastencode))=code \($0.tastencode)/"
                + "mod \(modifikatorenText($0.flags))/"
                + "fn \($0.fnGehalten ? "gehalten" : "frei")"
        }.joined(separator: ", ")
    }
    // Die Marke 'x' in einem Abschnitt heißt: bewusst übersprungen, weil die
    // Tastatur diesen Fall nicht hergibt. Ein übersprungener Abschnitt ist
    // kein "nein".
    let uebersprungen: ([Ereignis]) -> Bool = { teil in
        teil.contains {
            $0.art == "keyDown" && $0.rohZeichenOhneModifikatoren.lowercased() == "x"
        }
    }
    let antwort1: String
    if iA == nil || iB == nil {
        antwort1 = "NICHT GEMESSEN. Die Trennmarken 'a' und 'b' fehlen, "
            + "der Abschnitt wurde nie gedrückt. Durchgang wiederholen."
    } else if uebersprungen(mitFn) {
        antwort1 = "ÜBERSPRUNGEN. Abschnitt 1 trägt die Marke 'x'."
    } else if !fTastenMitFn.isEmpty {
        antwort1 = "JA. \(fTastenMitFn.count) von 3 erwarteten kamen an: "
            + trefferListe(fTastenMitFn)
    } else if !fTastenMitFnAberOhne.isEmpty {
        antwort1 = "NICHT GEMESSEN. In Abschnitt 1 kamen "
            + "\(fTastenMitFnAberOhne.count) Funktionstasten-keyDown an, aber "
            + "keines bei gehaltener fn-Taste: "
            + trefferListe(fTastenMitFnAberOhne) + ". Der Abschnitt misst damit "
            + "nicht Fn+F3 bis Fn+F8. Durchgang wiederholen."
    } else {
        antwort1 = "NEIN. In Abschnitt 1 kam kein einziges Funktionstasten-keyDown an."
    }
    let antwort2: String
    if iB == nil || iC == nil {
        antwort2 = "NICHT GEMESSEN. Die Trennmarken 'b' und 'c' fehlen, "
            + "der Abschnitt wurde nie gedrückt. Durchgang wiederholen."
    } else if uebersprungen(nackt) {
        antwort2 = "ÜBERSPRUNGEN. Abschnitt 2 trägt die Marke 'x'. Auf dieser "
            + "Tastatur ließ sich keine nackte F-Taste erzeugen, etwa weil ein "
            + "Touch Bar die F-Tastenreihe ersetzt. Kein Befund zu Frage 2."
    } else if !fTastenNackt.isEmpty {
        antwort2 = "JA. \(fTastenNackt.count) von 3 erwarteten kamen an: "
            + trefferListe(fTastenNackt)
    } else if !fTastenNacktAberMitFn.isEmpty {
        antwort2 = "NICHT MESSBAR AUF DIESEM GERÄT. In Abschnitt 2 kamen zwar "
            + "\(fTastenNacktAberMitFn.count) Funktionstasten-keyDown an, aber "
            + "alle bei gehaltener fn-Taste: "
            + trefferListe(fTastenNacktAberMitFn) + ". Abschnitt 2 wiederholt "
            + "damit Abschnitt 1 und sagt nichts über die nackten F-Tasten. "
            + "Wo die Tastatur keine physische F-Tastenreihe hat, sondern einen "
            + "Touch Bar, gibt es im Auslieferungszustand ohne gehaltenes fn "
            + "überhaupt keine F3: Frage 2 ist dort nicht stellbar und nur über "
            + "Durchgang B oder C zu beantworten."
    } else {
        antwort2 = "NEIN. In Abschnitt 2 kam kein einziges Funktionstasten-keyDown an; "
            + "das System hat sie vorher verbraucht."
    }
    let antwort4: String
    if iC == nil {
        antwort4 = "NICHT GEMESSEN. Die Trennmarke 'c' fehlt, "
            + "der Abschnitt wurde nie gedrückt. Durchgang wiederholen."
    } else if fnFlaggen.isEmpty {
        antwort4 = "NEIN. In Abschnitt 3 kam kein flagsChanged mit function-Wechsel an."
    } else {
        antwort4 = "JA. \(fnFlaggen.count) Ereignis(se) mit function-Wechsel: "
            + fnFlaggen.map { "code \($0.tastencode) \($0.veraenderung)" }
                .joined(separator: ", ")
    }
    let kontrollprobe: String
    if iC == nil {
        kontrollprobe = "nicht gemessen."
    } else if shiftFlaggen.isEmpty {
        kontrollprobe = "kein shift-Wechsel angekommen. Der Abgriff selbst ist "
            + "damit unbewiesen, jedes NEIN oben ist wertlos. Durchgang wiederholen."
    } else {
        kontrollprobe = "\(shiftFlaggen.count) shift-Wechsel angekommen, "
            + "der Abgriff arbeitet."
    }

    text += """
    ## Abgeleitete Antworten aus diesem Durchgang

    Frage 1 — Kommen Fn+F3 bis Fn+F8 als gewöhnliche Tastenereignisse an?
      \(antwort1)

    Frage 2 — Kommen die nackten F3 bis F8 an?
      \(antwort2)

    Frage 3 — Wirkung der Systemeinstellung?
      Dieser Durchgang misst genau einen Zustand: \(fnZustand)
      Die Antwort braucht beide Durchgänge. Vergleiche diese Datei mit der
      des anderen Durchgangs.

    Frage 4 — Löst die Fn-Taste selbst ein flagsChanged aus?
      \(antwort4)
      Kontrollprobe Shift: \(kontrollprobe)

    Ob fn körperlich gehalten war, entnimmt die Auswertung allein den
    flagsChanged der Taste 63, nicht dem mod=-Feld der einzelnen Taste: AppKit
    setzt `function` bei jeder Taste aus dem Funktionstasten-Unicodebereich,
    auch bei einer nackten F3.

    ## Vergleich mit dem dokumentierten Erwartungswert

    Erwartet nach Carbon HIToolbox und AppKit-Konstanten, nicht gemessen:
      F3=code 99, F4=118, F5=96, F6=97, F7=98, F8=100, fn=63
      Zeichen F3..F8 = U+F706 bis U+F70B
      Modifikator function = Bit 23 (0x800000)
    Weicht die Messung oben davon ab, gilt die Messung.

    """
    return text
}

// ---------------------------------------------------------------------------
// Protokoll zurücklesen
//
// Damit sich eine geschriebene Messung ohne Wiederholung nachrechnen lässt.
// Gelesen werden ausschließlich die Zeilen des rohen Ereignisprotokolls; alles
// andere im Bericht ist daraus abgeleitet und wird neu erzeugt.
// ---------------------------------------------------------------------------

/// Wert hinter `schluessel` bis zum nächsten Doppelleerzeichen.
func feld(_ zeile: String, _ schluessel: String) -> String? {
    guard let treffer = zeile.range(of: schluessel) else { return nil }
    let rest = zeile[treffer.upperBound...].drop(while: { $0 == " " })
    return String(rest).components(separatedBy: "  ")[0]
        .trimmingCharacters(in: .whitespaces)
}

func ereignisseAusBericht(_ text: String) -> [Ereignis] {
    var liste: [Ereignis] = []
    for rohzeile in text.split(separator: "\n", omittingEmptySubsequences: false) {
        let zeile = String(rohzeile)
        guard zeile.hasPrefix("#"), zeile.contains("code=") else { continue }
        let teile = zeile.split(separator: " ").map(String.init)
        guard teile.count > 3, teile[3] == "keyDown" || teile[3] == "flagsChanged"
        else { continue }

        let roh = UInt(feld(zeile, "roh=")?.dropFirst(2) ?? "", radix: 16) ?? 0
        let ohneMod = feld(zeile, "ohneMod=") ?? "—"
        // 'a' → a; U+F706 bleibt stehen und trifft auf keine Marke.
        let rohOhneMod = ohneMod.hasPrefix("'") && ohneMod.hasSuffix("'")
            ? String(ohneMod.dropFirst().dropLast())
            : ""

        liste.append(Ereignis(
            nummer: Int(teile[0].dropFirst()) ?? liste.count + 1,
            sekundenSeitStart: Double(teile[1].dropLast()) ?? 0,
            uhrzeit: teile[2],
            art: teile[3],
            tastencode: UInt16(feld(zeile, "code=")?.prefix(while: \.isNumber) ?? "") ?? 0,
            zeichen: feld(zeile, "zeichen=") ?? "—",
            zeichenOhneModifikatoren: ohneMod,
            rohZeichenOhneModifikatoren: rohOhneMod,
            flags: NSEvent.ModifierFlags(rawValue: roh)
                .intersection(.deviceIndependentFlagsMask),
            rohflags: roh,
            veraenderung: feld(zeile, "geändert=") ?? "—",
            istWiederholung: zeile.contains("(Wiederholung)")
        ))
    }
    return liste
}

/// Der Zustand der Systemeinstellung steht im Kopf der Datei, in der Zeile
/// unter dem Schlüsselnamen. Beim Nachrechnen gilt der Wert von damals, nicht
/// der von heute.
func fnZustandAusBericht(_ text: String) -> String {
    let zeilen = text.split(separator: "\n", omittingEmptySubsequences: false)
    if let i = zeilen.firstIndex(where: { $0.contains("com.apple.keyboard.fnState") }),
       i + 1 < zeilen.count {
        return zeilen[i + 1].trimmingCharacters(in: .whitespaces)
    }
    return "aus der Datei nicht lesbar"
}

// ---------------------------------------------------------------------------
// Anwendung
// ---------------------------------------------------------------------------

let anleitungsText = """
Drücke diese Folge, von oben nach unten. a, b und c sind Trennmarken, nach denen
das Programm auswertet. Genaueres steht in README.md.

   1. a
   2. Fn+F3      3. Fn+F5      4. Fn+F8
   5. b
   6. F3         7. F5         8. F8      ohne Fn
      Gibt diese Tastatur keine nackte F-Taste her, stattdessen einmal x drücken.
   9. c
  10. Fn allein (drücken und loslassen)
  11. Shift allein (drücken und loslassen)

Danach Cmd+Q. Das Ergebnis landet in der Datei aus der Titelzeile oben.
Auf einem Touch-Bar-Mac heißt "Fn+F3": fn halten, F3 auf dem Touch Bar antippen.
Dieses Fenster muss im Vordergrund sein. Öffnet sich Mission Control, schließe es \
und klicke dieses Fenster wieder an.
"""

final class Delegat: NSObject, NSApplicationDelegate, NSWindowDelegate {
    private let etikett: String
    private let ausgabe: URL
    private let start = Date()
    private let uhrformat: DateFormatter = {
        let f = DateFormatter()
        f.dateFormat = "HH:mm:ss.SSS"
        return f
    }()

    private var ereignisse: [Ereignis] = []
    private var letzteFlags: NSEvent.ModifierFlags = []
    private var fenster: NSWindow!
    private var textAnsicht: NSTextView!
    private var monitor: Any?
    private var geschrieben = false

    init(etikett: String, ausgabe: URL) {
        self.etikett = etikett
        self.ausgabe = ausgabe
    }

    // MARK: Aufbau

    func applicationDidFinishLaunching(_ notification: Notification) {
        menueAufbauen()
        fensterAufbauen()

        monitor = NSEvent.addLocalMonitorForEvents(matching: [.keyDown, .flagsChanged]) {
            [weak self] ereignis in
            guard let self else { return ereignis }
            self.aufzeichnen(ereignis)
            // Menübefehle durchlassen, damit Cmd+Q funktioniert. Alle anderen
            // keyDown-Ereignisse schlucken, sonst piept die Anwendung bei jedem
            // Tastendruck.
            if ereignis.type == .keyDown, !ereignis.modifierFlags.contains(.command) {
                return nil
            }
            return ereignis
        }

        NSApp.activate(ignoringOtherApps: true)
        anhaengen("Aufzeichnung läuft. Ergebnisdatei: \(ausgabe.path)\n")
    }

    private func menueAufbauen() {
        let hauptmenue = NSMenu()
        let eintrag = NSMenuItem()
        hauptmenue.addItem(eintrag)
        let anwendungsmenue = NSMenu()
        anwendungsmenue.addItem(
            withTitle: "Beenden und Ergebnis schreiben",
            action: #selector(NSApplication.terminate(_:)),
            keyEquivalent: "q"
        )
        eintrag.submenu = anwendungsmenue
        NSApp.mainMenu = hauptmenue
    }

    private func fensterAufbauen() {
        let breite: CGFloat = 900
        let hoehe: CGFloat = 640
        let anleitungsHoehe: CGFloat = 190

        fenster = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: breite, height: hoehe),
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        )
        fenster.title = "KRK Fn-Tasten-Messung — Durchgang \(etikett) — \(ausgabe.lastPathComponent)"
        fenster.delegate = self
        fenster.center()

        let inhalt = NSView(frame: NSRect(x: 0, y: 0, width: breite, height: hoehe))

        let anleitung = NSTextField(labelWithString: anleitungsText)
        anleitung.font = NSFont.monospacedSystemFont(ofSize: 12, weight: .regular)
        anleitung.usesSingleLineMode = false
        anleitung.maximumNumberOfLines = 0
        anleitung.lineBreakMode = .byWordWrapping
        anleitung.frame = NSRect(
            x: 16, y: hoehe - anleitungsHoehe - 12,
            width: breite - 32, height: anleitungsHoehe
        )
        anleitung.autoresizingMask = [.width, .minYMargin]
        inhalt.addSubview(anleitung)

        let scroll = NSScrollView(frame: NSRect(
            x: 16, y: 16,
            width: breite - 32, height: hoehe - anleitungsHoehe - 40
        ))
        scroll.hasVerticalScroller = true
        scroll.borderType = .bezelBorder
        scroll.autoresizingMask = [.width, .height]

        textAnsicht = NSTextView(frame: scroll.bounds)
        textAnsicht.isEditable = false
        textAnsicht.isSelectable = true
        textAnsicht.font = NSFont.monospacedSystemFont(ofSize: 11, weight: .regular)
        textAnsicht.autoresizingMask = [.width]
        textAnsicht.isVerticallyResizable = true
        textAnsicht.isHorizontallyResizable = false
        textAnsicht.textContainer?.widthTracksTextView = true
        scroll.documentView = textAnsicht
        inhalt.addSubview(scroll)

        fenster.contentView = inhalt
        fenster.makeKeyAndOrderFront(nil)
    }

    // MARK: Aufzeichnung

    private func aufzeichnen(_ ereignis: NSEvent) {
        let art = ereignis.type == .keyDown ? "keyDown" : "flagsChanged"
        let neueFlags = ereignis.modifierFlags.intersection(.deviceIndependentFlagsMask)
        let unterschied = NSEvent.ModifierFlags(
            rawValue: neueFlags.rawValue ^ letzteFlags.rawValue
        )
        var veraenderung = "—"
        if art == "flagsChanged" {
            let gesetzt = NSEvent.ModifierFlags(
                rawValue: unterschied.rawValue & neueFlags.rawValue
            )
            let geloescht = NSEvent.ModifierFlags(
                rawValue: unterschied.rawValue & ~neueFlags.rawValue
            )
            var teile: [String] = []
            if !gesetzt.isEmpty { teile.append("+" + modifikatorenText(gesetzt)) }
            if !geloescht.isEmpty { teile.append("-" + modifikatorenText(geloescht)) }
            veraenderung = teile.isEmpty ? "nichts" : teile.joined(separator: " ")
        }
        letzteFlags = neueFlags

        let rohOhneMod = ereignis.type == .keyDown
            ? (ereignis.charactersIgnoringModifiers ?? "")
            : ""

        let neu = Ereignis(
            nummer: ereignisse.count + 1,
            sekundenSeitStart: Date().timeIntervalSince(start),
            uhrzeit: uhrformat.string(from: Date()),
            art: art,
            tastencode: ereignis.keyCode,
            zeichen: ereignis.type == .keyDown ? zeichenText(ereignis.characters) : "—",
            zeichenOhneModifikatoren: ereignis.type == .keyDown
                ? zeichenText(ereignis.charactersIgnoringModifiers) : "—",
            rohZeichenOhneModifikatoren: rohOhneMod,
            flags: neueFlags,
            rohflags: ereignis.modifierFlags.rawValue,
            veraenderung: veraenderung,
            istWiederholung: ereignis.type == .keyDown ? ereignis.isARepeat : false
        )
        ereignisse.append(neu)
        anhaengen(neu.zeile + "\n")
    }

    private func anhaengen(_ text: String) {
        textAnsicht.textStorage?.append(NSAttributedString(
            string: text,
            attributes: [
                .font: NSFont.monospacedSystemFont(ofSize: 11, weight: .regular),
                .foregroundColor: NSColor.labelColor,
            ]
        ))
        textAnsicht.scrollToEndOfDocument(nil)
    }

    // MARK: Auswertung

    /// Kopf und rohes Protokoll; die Auswertung selbst steht als freie Funktion
    /// weiter oben, damit sie auch über eine geschriebene Datei laufen kann.
    private func bericht() -> String {
        var text = """
        # KRK — Messung der Fn-Tasten-Annahme (C3 des Navigator-Specs)

        Durchgang:            \(etikett)
        Zeitpunkt:            \(ISO8601DateFormatter().string(from: start))
        macOS:                \(ProcessInfo.processInfo.operatingSystemVersionString)
        com.apple.keyboard.fnState beim Start dieses Durchgangs:
                              \(fnStateText())

        Gemessen wird, was eine gewöhnliche AppKit-Anwendung im Vordergrund über
        einen lokalen NSEvent-Abgriff erhält. Keine Bedienungshilfen-Freigabe,
        kein globaler Abgriff, kein Event Tap.

        ## Rohes Ereignisprotokoll


        """

        if ereignisse.isEmpty {
            text += "(leer — es kam kein einziges Ereignis an)\n"
        } else {
            for e in ereignisse { text += e.zeile + "\n" }
        }

        text += "\n" + auswertung(ereignisse, fnZustand: fnStateText())
        return text
    }

    // MARK: Abschluss

    func windowWillClose(_ notification: Notification) {
        NSApp.terminate(nil)
    }

    func applicationWillTerminate(_ notification: Notification) {
        guard !geschrieben else { return }
        geschrieben = true
        if let monitor { NSEvent.removeMonitor(monitor) }
        let text = bericht()
        do {
            try text.write(to: ausgabe, atomically: true, encoding: .utf8)
            FileHandle.standardOutput.write(Data(text.utf8))
            print("\nErgebnis geschrieben nach: \(ausgabe.path)")
        } catch {
            print("Konnte die Ergebnisdatei nicht schreiben: \(error)")
            FileHandle.standardOutput.write(Data(text.utf8))
        }
    }
}

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------

// Nachrechnen statt messen: `./beobachter --auswerten messung-A.txt` liest das
// rohe Ereignisprotokoll aus einer geschriebenen Messung zurück und wertet es
// erneut aus, ohne Fenster und ohne Tastendruck. Die Quelldatei bleibt
// unangetastet; das Ergebnis geht auf die Standardausgabe.
if CommandLine.arguments.count > 2, CommandLine.arguments[1] == "--auswerten" {
    let pfad = CommandLine.arguments[2]
    guard let inhalt = try? String(contentsOfFile: pfad, encoding: .utf8) else {
        print("Kann die Datei nicht lesen: \(pfad)")
        exit(1)
    }
    let gelesen = ereignisseAusBericht(inhalt)
    print("# Neuauswertung von \(pfad)")
    print("")
    print("Nachgerechnet am:     \(ISO8601DateFormatter().string(from: Date()))")
    print("Ereignisse gelesen:   \(gelesen.count)")
    if gelesen.isEmpty {
        print("")
        print("Kein Ereignisprotokoll gefunden. Ist das eine Messdatei?")
        exit(1)
    }
    print(auswertung(gelesen, fnZustand: fnZustandAusBericht(inhalt)))
    exit(0)
}

let rohEtikett = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "A"
let etikett = String(rohEtikett.filter { $0.isLetter || $0.isNumber || $0 == "-" })
let sicheresEtikett = etikett.isEmpty ? "A" : etikett

let programmVerzeichnis = URL(
    fileURLWithPath: CommandLine.arguments[0],
    relativeTo: URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
).standardizedFileURL.deletingLastPathComponent()

let ausgabeDatei = programmVerzeichnis
    .appendingPathComponent("messung-\(sicheresEtikett).txt")

print("KRK Fn-Tasten-Messung, Durchgang \(sicheresEtikett)")
print("com.apple.keyboard.fnState: \(fnStateText())")
print("Ergebnisdatei: \(ausgabeDatei.path)")
print("Das Fenster kommt gleich in den Vordergrund. Tastenfolge steht darin.")

let anwendung = NSApplication.shared
anwendung.setActivationPolicy(.regular)
let delegat = Delegat(etikett: sicheresEtikett, ausgabe: ausgabeDatei)
anwendung.delegate = delegat

// Ctrl+C im Terminal und `kill` sollen die Messung nicht verlieren: beide auf den
// regulären Beendigungsweg umlenken, der die Ergebnisdatei schreibt.
var signalQuellen: [DispatchSourceSignal] = []
for kennung in [SIGINT, SIGTERM] {
    signal(kennung, SIG_IGN)
    let quelle = DispatchSource.makeSignalSource(signal: kennung, queue: .main)
    quelle.setEventHandler { NSApp.terminate(nil) }
    quelle.resume()
    signalQuellen.append(quelle)
}

anwendung.run()
