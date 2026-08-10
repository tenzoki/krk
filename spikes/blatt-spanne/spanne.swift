// KRK — Prüfprogramm zur Frage aus
// `issues/260810-1207_*_die-spanne-zwischen-dem-schliessen-des-blattes-und-seiner-antwort-ist-ungemessen.md`.
//
// WEGWERF-PRÜFCODE. Kein Produktcode. Keine Tests, keine Architektur, keine
// Fehlerbehandlung über das Nötigste hinaus. Die Frage und die Auswertung
// stehen in README.md daneben.
//
// Gemessen wird eine Reihenfolge von AppKit, keine Eigenschaft von KRK: setzt
// `NSWindow.attachedSheet` schon vor dem Abschlussblock von
// `beginSheetModalForWindow:completionHandler:` auf nil, und läuft die
// Hauptschleife in der Zwischenzeit weiter? Nur wenn beides zutrifft, kommt in
// dieser Spanne ein Tastendruck durch KRKs Sperre.
//
// Das Programm braucht den Vordergrund nicht. Es beantwortet sein Blatt selbst,
// über die zwei Wege, die KRK dafür hat, und wirft seine Tastendrücke in die
// eigene Ereignisschlange statt über osascript.

import AppKit
import Foundation

// ---------------------------------------------------------------------------
// Uhr und Protokoll
// ---------------------------------------------------------------------------

let start = DispatchTime.now().uptimeNanoseconds

/// Millisekunden seit Programmstart.
func jetzt() -> Double {
    Double(DispatchTime.now().uptimeNanoseconds - start) / 1_000_000
}

var protokoll: [String] = []

func notiere(_ zeile: String) {
    let satz = String(format: "%9.3f ms  %@", jetzt(), zeile)
    print(satz)
    protokoll.append(satz)
}

// ---------------------------------------------------------------------------
// Der Weg, auf dem das Blatt geschlossen wird
// ---------------------------------------------------------------------------

/// KRK schließt ein Blatt auf zwei Wegen, und beide werden hier gemessen.
enum Weg: String {
    /// `fenster.endSheet(blattfenster, returnCode:)` — der Weg von
    /// `Blattgriff::abbrechen` und des `Eingabewaechter`s.
    case griff
    /// `knopf.performClick(nil)` — der Weg des Nutzers, der auf eine
    /// Schaltfläche klickt oder ihre Tastenentsprechung drückt.
    case klick
}

let weg = Weg(rawValue: CommandLine.arguments.dropFirst().first ?? "griff") ?? .griff

/// Wann der Schließbefehl fällt, in Millisekunden nach Programmstart. Lang
/// genug, dass das Blatt sicher steht und seine Einfahrt fertig ist.
let schliessenBei = 900.0

/// Wie lange nach dem Abschlussblock noch abgetastet wird.
let nachlauf = 600.0

/// Wie viele Tastendrücke unmittelbar vor dem Schließbefehl in die
/// Ereignisschlange gelegt werden.
let salve = 40

// ---------------------------------------------------------------------------
// Die Messung
// ---------------------------------------------------------------------------

final class Messung: NSObject, NSApplicationDelegate {
    let fenster = NSWindow(
        contentRect: NSRect(x: 300, y: 300, width: 460, height: 180),
        styleMask: [.titled],
        backing: .buffered,
        defer: false
    )
    let warnung = NSAlert()
    var takt: Timer?
    var abgriff: Any?

    // Messwerte
    var blattStandVorher: Bool?
    var tAufruf: Double?
    var tAufrufZurueck: Double?
    var tBlock: Double?
    var blattImBlock: Bool?
    var tErsteNil: Double?
    var takteNachAufruf = 0
    var takteNilVorBlock = 0
    var tastenGesamt = 0
    var tastenInSpanne = 0
    var tastenMitBlatt = 0
    var tastenNachBlock = 0
    var tastenInGegenspanne = 0
    var schliessbefehlGefallen = false
    var fertig = false

    func applicationDidFinishLaunching(_: Notification) {
        fenster.title = "KRK — Messung der Blattspanne"
        fenster.makeKeyAndOrderFront(nil)

        // Derselbe Mechanismus, den KRK für seinen Ereignisabgriff verwendet:
        // ein lokaler Abgriff auf keyDown, der dieselbe Frage stellt wie
        // `blatt_steht`.
        abgriff = NSEvent.addLocalMonitorForEvents(matching: .keyDown) { [weak self] ereignis in
            self?.tastendruckSehen()
            return nil
        }

        warnung.messageText = "Ungesicherter Stand"
        warnung.addButton(withTitle: "Sichern")
        warnung.addButton(withTitle: "Abbrechen")
        warnung.beginSheetModal(for: fenster) { [weak self] antwort in
            guard let self else { return }
            self.tBlock = jetzt()
            self.blattImBlock = self.fenster.attachedSheet != nil
            notiere(
                "Abschlussblock läuft. Antwort \(antwort.rawValue), "
                    + "attachedSheet \(self.blattImBlock! ? "gesetzt" : "nil")"
            )
        }
        notiere("Blatt begonnen, Weg \(weg.rawValue)")

        // Ein Millisekunden-Takt auf der Hauptschleife. Er tastet
        // `attachedSheet` ab und ist zugleich der Beweis, dass die Hauptschleife
        // in der gemessenen Spanne Ereignisse verarbeitet: liefe sie dort nicht,
        // gäbe es in der Spanne keinen Takt.
        let takt = Timer(timeInterval: 0.001, repeats: true) { [weak self] _ in
            self?.taktSchlagen()
        }
        RunLoop.main.add(takt, forMode: .common)
        self.takt = takt
    }

    /// Ein Tastendruck ist im Abgriff angekommen. Was hätte KRKs Sperre gesagt?
    func tastendruckSehen() {
        tastenGesamt += 1
        let blattWeg = fenster.attachedSheet == nil
        let blockGelaufen = tBlock != nil
        switch (blattWeg, blockGelaufen) {
        case (true, false):
            tastenInSpanne += 1
            if tastenInSpanne <= 3 {
                notiere("Tastendruck IN DER SPANNE: attachedSheet nil, Abschlussblock noch nicht gelaufen")
            }
        case (false, false):
            tastenMitBlatt += 1
        case (false, true):
            // Die Gegenrichtung: die Antwort ist ausgefuehrt, attachedSheet
            // meldet aber weiter ein Blatt. KRKs Sperre weist hier ab, obwohl
            // nichts mehr zu schuetzen ist.
            tastenNachBlock += 1
            tastenInGegenspanne += 1
        case (true, true):
            tastenNachBlock += 1
        }
    }

    func taktSchlagen() {
        if fertig { return }

        if !schliessbefehlGefallen {
            if jetzt() < schliessenBei { return }
            blattStandVorher = fenster.attachedSheet != nil
            notiere(
                "Vor dem Schließbefehl: attachedSheet "
                    + "\(blattStandVorher! ? "gesetzt" : "nil")"
            )
            schliessbefehlGefallen = true
            // Eine Salve vorab in die eigene Ereignisschlange. Der
            // Schließbefehl haelt den Hauptfaden fuer die Dauer der
            // Blattanimation; ohne die Salve liegt in dieser Zeit kein
            // Tastendruck bereit, und die Gegenrichtung der Spanne bliebe
            // unbeprobt.
            for _ in 0 ..< salve { tastePosten() }
            notiere("\(salve) Tastendrücke vorab in die Schlange gelegt")
            tAufruf = jetzt()
            notiere("Schließbefehl \(weg.rawValue) …")
            switch weg {
            case .griff:
                fenster.endSheet(warnung.window, returnCode: .alertSecondButtonReturn)
            case .klick:
                warnung.buttons[1].performClick(nil)
            }
            tAufrufZurueck = jetzt()
            notiere(
                "Schließbefehl kehrt zurück. Abschlussblock "
                    + "\(tBlock == nil ? "noch nicht" : "schon") gelaufen, attachedSheet "
                    + "\(fenster.attachedSheet == nil ? "nil" : "gesetzt")"
            )
            return
        }

        takteNachAufruf += 1
        let blattWeg = fenster.attachedSheet == nil
        if blattWeg, tErsteNil == nil {
            tErsteNil = jetzt()
            notiere("attachedSheet erstmals nil")
        }
        if blattWeg, tBlock == nil {
            takteNilVorBlock += 1
        }

        // Jeder Takt wirft einen Tastendruck in die eigene Ereignisschlange.
        // Damit ist die ganze Spanne dicht abgetastet und nicht nur ein
        // Zeitpunkt darin.
        tastePosten()

        if let tBlock, jetzt() > tBlock + nachlauf {
            abschluss()
        } else if jetzt() > schliessenBei + 4000 {
            notiere("Abbruch: der Abschlussblock ist in vier Sekunden nicht gelaufen.")
            abschluss()
        }
    }

    func tastePosten() {
        guard
            let ereignis = NSEvent.keyEvent(
                with: .keyDown,
                location: .zero,
                modifierFlags: [],
                timestamp: ProcessInfo.processInfo.systemUptime,
                windowNumber: fenster.windowNumber,
                context: nil,
                characters: "a",
                charactersIgnoringModifiers: "a",
                isARepeat: false,
                keyCode: 0
            )
        else { return }
        NSApp.postEvent(ereignis, atStart: false)
    }

    // -----------------------------------------------------------------------
    // Auswertung
    // -----------------------------------------------------------------------

    func abschluss() {
        fertig = true
        takt?.invalidate()
        if let abgriff { NSEvent.removeMonitor(abgriff) }

        var bericht: [String] = []
        bericht.append("KRK — Messung: trägt die Spanne zwischen attachedSheet == nil und dem Abschlussblock?")
        bericht.append("Datensatz: issues/260810-1207_*_die-spanne-zwischen-dem-schliessen-des-blattes-und-seiner-antwort-ist-ungemessen.md")
        bericht.append("Zeitpunkt: \(ISO8601DateFormatter().string(from: Date()))")
        bericht.append("macOS: \(systemversion())")
        bericht.append("Gerät: \(gerätemodell())")
        bericht.append("Weg: \(weg.rawValue) — \(wegtext())")
        bericht.append("")
        bericht.append("## Protokoll")
        bericht.append("")
        bericht.append(contentsOf: protokoll)
        bericht.append("")
        bericht.append("## Messwerte")
        bericht.append("")
        bericht.append(zeile("attachedSheet vor dem Schließbefehl", ja(blattStandVorher)))
        bericht.append(zeile("attachedSheet im Abschlussblock", ja(blattImBlock)))
        bericht.append(zeile("Schließbefehl bei", msText(tAufruf)))
        bericht.append(zeile("Schließbefehl kehrte zurück bei", msText(tAufrufZurueck)))
        bericht.append(zeile("attachedSheet erstmals nil bei", msText(tErsteNil)))
        bericht.append(zeile("Abschlussblock lief bei", msText(tBlock)))
        bericht.append(zeile("Spanne (erstes nil bis Abschlussblock)", spanneText()))
        bericht.append(zeile("Takte nach dem Schließbefehl", "\(takteNachAufruf)"))
        bericht.append(zeile("Takte mit nil vor dem Abschlussblock", "\(takteNilVorBlock)"))
        bericht.append(zeile("Tastendrücke im Abgriff", "\(tastenGesamt)"))
        bericht.append(zeile("davon mit stehendem Blatt (KRK sperrt)", "\(tastenMitBlatt)"))
        bericht.append(zeile("davon IN DER SPANNE (KRK sperrt nicht)", "\(tastenInSpanne)"))
        bericht.append(zeile("davon nach dem Abschlussblock", "\(tastenNachBlock)"))
        bericht.append(
            zeile(
                "davon in der Gegenspanne (Antwort ausgeführt, Blatt meldet weiter)",
                "\(tastenInGegenspanne)"
            )
        )
        bericht.append("")
        bericht.append("## Befund")
        bericht.append("")
        bericht.append(contentsOf: befund())

        let text = bericht.joined(separator: "\n") + "\n"
        let ziel = URL(fileURLWithPath: "messung-\(weg.rawValue).txt")
        try? text.write(to: ziel, atomically: true, encoding: .utf8)

        print("")
        print(bericht.suffix(from: bericht.count - befund().count - 3).joined(separator: "\n"))
        print("")
        print("Bericht: \(ziel.path)")
        exit(0)
    }

    func befund() -> [String] {
        var zeilen: [String] = []
        guard let blattImBlock else {
            zeilen.append("NICHT GEMESSEN: der Abschlussblock ist nicht gelaufen.")
            return zeilen
        }
        if blattStandVorher != true {
            zeilen.append(
                "NICHT GEMESSEN: attachedSheet war schon vor dem Schließbefehl nil, das Blatt stand also nie. "
                    + "Ohne stehendes Blatt sagt die Messung über die Spanne nichts."
            )
            return zeilen
        }
        if blattImBlock {
            zeilen.append(
                "DIE SPANNE TRÄGT NICHT. attachedSheet ist im Abschlussblock noch gesetzt, "
                    + "die Sperre aus `blatt_steht` greift also bis zur Antwort einschließlich."
            )
        } else {
            zeilen.append(
                "attachedSheet ist im Abschlussblock schon nil. Ob dazwischen etwas durchkommt, "
                    + "entscheiden die beiden Zahlen darunter."
            )
        }
        if tastenGesamt == 0 {
            zeilen.append(
                "Der Tastenarm der Messung hat nicht getragen: kein einziger geposteter Tastendruck kam im Abgriff an. "
                    + "Es gilt allein der Taktarm."
            )
        }
        if tastenInSpanne > 0 {
            zeilen.append(
                "DIE SPANNE TRÄGT: \(tastenInSpanne) Tastendrücke wurden mit nil-attachedSheet und noch nicht "
                    + "gelaufenem Abschlussblock gesehen. Ein Kommando käme dort durch."
            )
        } else if takteNilVorBlock > 0 {
            zeilen.append(
                "DIE SPANNE TRÄGT: \(takteNilVorBlock) Takte der Hauptschleife liefen mit nil-attachedSheet, "
                    + "bevor der Abschlussblock lief. Die Hauptschleife arbeitet dort, also käme ein Kommando durch."
            )
        } else {
            zeilen.append(
                "Kein Takt und kein Tastendruck fiel zwischen das erste nil und den Abschlussblock. "
                    + "Die Spanne ist in Ereignissen der Hauptschleife gemessen leer."
            )
        }
        if let tErsteNil, let tBlock, tErsteNil > tBlock {
            let dauer = tErsteNil - tBlock
            if tastenInGegenspanne > 0 {
                zeilen.append(
                    String(
                        format:
                            "DIE GEGENSPANNE TRÄGT: attachedSheet meldet noch %.0f ms nach der ausgeführten "
                            + "Antwort ein Blatt, und in dieser Zeit kamen %d Tastendrücke an. KRKs Sperre "
                            + "weist sie ab, obwohl das Blatt beantwortet ist.",
                        dauer,
                        tastenInGegenspanne
                    )
                )
            } else {
                zeilen.append(
                    String(
                        format:
                            "DIE GEGENSPANNE TRÄGT NICHT: attachedSheet meldet zwar noch %.0f ms nach der "
                            + "ausgeführten Antwort ein Blatt, aber in dieser Zeit nimmt der Hauptfaden kein "
                            + "Tastenereignis entgegen. Die %d vorab in die Schlange gelegten Tastendrücke kamen "
                            + "alle erst danach an. Die Sperre weist dort nichts ab, weil dort nichts eintrifft.",
                        dauer,
                        salve
                    )
                )
            }
        }
        return zeilen
    }

    func spanneText() -> String {
        guard let tErsteNil, let tBlock else { return "nicht gemessen" }
        return String(format: "%.3f ms", tBlock - tErsteNil)
    }

    func wegtext() -> String {
        switch weg {
        case .griff: return "endSheet(_:returnCode:), wie Blattgriff::abbrechen und der Eingabewaechter"
        case .klick: return "performClick(nil) auf der zweiten Schaltfläche, wie ein Nutzerklick"
        }
    }
}

// ---------------------------------------------------------------------------
// Kleinkram
// ---------------------------------------------------------------------------

func zeile(_ was: String, _ wert: String) -> String {
    was.padding(toLength: max(was.count, 48), withPad: " ", startingAt: 0) + "  " + wert
}

func ja(_ wert: Bool?) -> String {
    guard let wert else { return "nicht gemessen" }
    return wert ? "gesetzt" : "nil"
}

func msText(_ wert: Double?) -> String {
    guard let wert else { return "nicht gemessen" }
    return String(format: "%.3f ms", wert)
}

func systemversion() -> String {
    let v = ProcessInfo.processInfo.operatingSystemVersion
    return "\(v.majorVersion).\(v.minorVersion).\(v.patchVersion)"
}

func gerätemodell() -> String {
    var groesse = 0
    sysctlbyname("hw.model", nil, &groesse, nil, 0)
    var puffer = [CChar](repeating: 0, count: groesse)
    sysctlbyname("hw.model", &puffer, &groesse, nil, 0)
    return String(cString: puffer)
}

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------

let anwendung = NSApplication.shared
// `.accessory`: das Programm braucht den Vordergrund nicht und soll ihn dem
// Nutzer auch nicht wegnehmen.
anwendung.setActivationPolicy(.accessory)
let messung = Messung()
anwendung.delegate = messung
anwendung.run()
