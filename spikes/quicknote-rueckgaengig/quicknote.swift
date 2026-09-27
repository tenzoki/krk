// KRK — Prüfprogramm zum Defekt
// `work-packages/260926-2246-f10-oeffnet-quicknote-mit-fluechtigem-puffer/issues/260927-0225_*_ob-cmd-z-in-einer-leeren-quicknote-die-datei-darunter-erreicht-ist-ungemessen-und-ein-spike-koennte-es-messen.md`.
//
// WEGWERF-PRÜFCODE. Kein Produktcode. Keine Tests, keine Architektur, keine
// Fehlerbehandlung über das Nötigste hinaus. Die Frage und die Auswertung
// stehen in README.md daneben.
//
// Gemessen wird eine Eigenschaft von AppKit, keine von KRK: welchen
// Rückgängigverwalter `cmd+z` und `shift+cmd+z` über das Hauptmenü erreichen,
// wenn der Ersthelfer eine gewöhnliche `NSTextView` ist, deren Delegierter
// `undoManagerForTextView:` mit einem eigenen Verwalter beantwortet, und deren
// eigener Stapel leer ist — während eine zweite, ausgeblendete Textfläche im
// selben Fenster am Verwalter des Fensters angemeldet hat. Das ist die Lage der
// Quicknote über der Datei im Editor (`crates/krk-ui/src/appkit/quicknote.rs`).
//
// Die Tastendrücke gehen über `NSApp.postEvent(_:atStart:)` in die eigene
// Ereignisschlange, nicht über osascript.

import AppKit
import Foundation

var protokoll: [String] = []
func notiere(_ zeile: String) {
    print(zeile)
    protokoll.append(zeile)
}

func name(_ o: AnyObject?) -> String {
    guard let o else { return "nil" }
    return String(describing: type(of: o))
}

// ---------------------------------------------------------------------------
// Die Quicknote: Delegierter mit eigenem Verwalter, wie `verwalter_fuer`
// ---------------------------------------------------------------------------

final class Quicknotedelegierter: NSObject, NSTextViewDelegate {
    let verwalter = UndoManager()
    func undoManager(for view: NSTextView) -> UndoManager? { verwalter }
}

/// Zeichnet auf, wann `undo:` und `redo:` beim Fenster ankommen und welche
/// Verwalter dort zu sehen sind, und reicht dann an die Umsetzung von AppKit
/// weiter.
final class Messfenster: NSWindow {
    var undoAngekommen = 0
    var redoAngekommen = 0

    private func weiter(_ sel: Selector, _ sender: Any?) {
        let m = class_getInstanceMethod(NSWindow.self, sel)!
        typealias F = @convention(c) (AnyObject, Selector, Any?) -> Void
        unsafeBitCast(method_getImplementation(m), to: F.self)(self, sel, sender)
    }

    private func befund(_ wer: String) {
        let fe = firstResponder
        let vFe = fe?.undoManager
        notiere("      \(wer) beim Fenster: Ersthelfer \(name(fe)), dessen Verwalter ist \(vFe === quicknote.verwalter ? "der eigene der Quicknote" : vFe === undoManager ? "der des Fensters" : name(vFe))")
    }

    @objc func undo(_ sender: Any?) {
        undoAngekommen += 1
        befund("undo:")
        weiter(#selector(undo(_:)), sender)
    }

    @objc func redo(_ sender: Any?) {
        redoAngekommen += 1
        befund("redo:")
        weiter(#selector(redo(_:)), sender)
    }
}

let app = NSApplication.shared
app.setActivationPolicy(.regular)

// Das Menü: Rückgängig und Wiederholen, an `undo:` und `redo:` mit Ziel nil
// gebunden wie im Hauptmenü von KRK.
let hauptmenue = NSMenu()
let appEintrag = NSMenuItem()
hauptmenue.addItem(appEintrag)
appEintrag.submenu = NSMenu()
let bearbeitenEintrag = NSMenuItem()
hauptmenue.addItem(bearbeitenEintrag)
let bearbeiten = NSMenu(title: "Bearbeiten")
bearbeiten.addItem(withTitle: "Rückgängig", action: #selector(Messfenster.undo(_:)), keyEquivalent: "z")
let wieder = bearbeiten.addItem(withTitle: "Wiederholen", action: #selector(Messfenster.redo(_:)), keyEquivalent: "z")
wieder.keyEquivalentModifierMask = [.command, .shift]
bearbeitenEintrag.submenu = bearbeiten
app.mainMenu = hauptmenue

let fenster = Messfenster(
    contentRect: NSRect(x: 200, y: 200, width: 460, height: 300),
    styleMask: [.titled, .closable], backing: .buffered, defer: false)
fenster.title = "KRK Prüfprogramm: Rückgängig in der Quicknote"

func textflaeche(_ rahmen: NSRect) -> (NSScrollView, NSTextView) {
    let rolle = NSScrollView(frame: rahmen)
    rolle.autoresizingMask = [.width, .height]
    let t = NSTextView(frame: rahmen)
    t.isEditable = true
    t.isSelectable = true
    t.isRichText = false
    t.allowsUndo = true
    rolle.documentView = t
    return (rolle, t)
}

// Die Datei: eine Textfläche ohne Delegierten, deren Verwalter über die
// Antwortkette der des Fensters ist, wie die Textfläche des Editors.
let rahmen = fenster.contentView!.bounds
let (dateirolle, datei) = textflaeche(rahmen)
fenster.contentView!.addSubview(dateirolle)

// Die Quicknote: deckungsgleich darüber, ausgeblendet bis zum „F10".
let (quickrolle, quick) = textflaeche(rahmen)
let quicknote = Quicknotedelegierter()
quick.delegate = quicknote
quickrolle.isHidden = true
fenster.contentView!.addSubview(quickrolle)

// Eine angemeldete Handlung am Verwalter des Fensters, wie der Umbau einer
// Eintragstabelle (`umkehrung_anmelden`). Ihr Zurücknehmen zählt mit.
var fensterhandlungZurueck = 0
var fensterhandlungWieder = 0
final class Ziel: NSObject {}
let ziel = Ziel()
func fensterhandlungAnmelden() {
    fenster.undoManager!.registerUndo(withTarget: ziel) { _ in
        fensterhandlungZurueck += 1
        fenster.undoManager!.registerUndo(withTarget: ziel) { _ in fensterhandlungWieder += 1 }
    }
    fenster.undoManager!.setActionName("Umbau")
}

// ---------------------------------------------------------------------------
// Tastenweg
// ---------------------------------------------------------------------------

func taste(_ c: String, _ flags: NSEvent.ModifierFlags = [], _ code: UInt16 = 0, ohne: String? = nil) {
    let e = NSEvent.keyEvent(
        with: .keyDown, location: .zero, modifierFlags: flags,
        timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: fenster.windowNumber,
        context: nil, characters: c, charactersIgnoringModifiers: ohne ?? c, isARepeat: false, keyCode: code)!
    NSApp.postEvent(e, atStart: false)
}
// Die Tastennummer der Taste, die auf der Belegung des Messgeraets (German)
// "z" schreibt: kVK_ANSI_Y = 16.
let zTaste: UInt16 = 16
func cmdZ() { taste("z", .command, zTaste) }
/// `shift+cmd+z` geht nicht als gepostetes Ereignis: ein synthetisches
/// Ereignis mit Umschalt- und Befehlstaste hat in diesem Programm in keiner
/// Form `redo:` ausgeloest (`performKeyEquivalent:` meldet ja, beim Fenster kommt
/// nichts an). Gemessen wird deshalb das, was das Menue bei einem Treffer tut:
/// der Eintrag "Wiederholen" wird geprueft, und nur ein freier Eintrag loest
/// seine Aktion aus, an den Ersthelfer gerichtet wie im Hauptmenue von KRK.
func shiftCmdZ() {
    bearbeiten.update()
    if wieder.isEnabled {
        bearbeiten.performActionForItem(at: 1)
    } else {
        notiere("      Wiederholen ist grau: der Menueeintrag loest nichts aus")
    }
}
func tippen(_ s: String) { for c in s { taste(String(c)) } }

func nach(_ s: Double, _ tun: @escaping () -> Void) {
    DispatchQueue.main.asyncAfter(deadline: .now() + s, execute: tun)
}

// ---------------------------------------------------------------------------
// Messung
// ---------------------------------------------------------------------------

func menueFrei() -> String {
    bearbeiten.update()
    let r = bearbeiten.item(at: 0)!.isEnabled
    let w = bearbeiten.item(at: 1)!.isEnabled
    return "Menü Rückgängig \(r ? "frei" : "grau"), Wiederholen \(w ? "frei" : "grau")"
}

func lage(_ marke: String) {
    let fv = fenster.undoManager!
    let qv = quicknote.verwalter
    notiere(
        "    \(marke.padding(toLength: 34, withPad: " ", startingAt: 0))"
            + " Quicknote '\(quick.string)' Datei '\(datei.string)'"
            + " | eigener Verwalter canUndo \(qv.canUndo ? "ja" : "nein") canRedo \(qv.canRedo ? "ja" : "nein")"
            + " | Fensterverwalter canUndo \(fv.canUndo ? "ja" : "nein") canRedo \(fv.canRedo ? "ja" : "nein")"
            + " | Fensterhandlung zurück \(fensterhandlungZurueck) wieder \(fensterhandlungWieder)"
            + " | undo: am Fenster \(fenster.undoAngekommen) redo: \(fenster.redoAngekommen)"
            + " | \(menueFrei())")
}

var schritte: [(Double, () -> Void)] = []
func schritt(_ s: Double = 0.3, _ tun: @escaping () -> Void) { schritte.append((s, tun)) }
func fahren(_ i: Int = 0) {
    guard i < schritte.count else { abschliessen(); return }
    nach(schritte[i].0) { schritte[i].1(); fahren(i + 1) }
}

/// Der Tausch wie `flaeche_waehlen`: die alte Rolle aus, die neue ein, der
/// Ersthelfer in die neue Fläche.
func quicknoteZeigen() {
    dateirolle.isHidden = true
    quickrolle.isHidden = false
    fenster.makeFirstResponder(quick)
}
func quicknoteVerlassen() {
    quickrolle.isHidden = true
    dateirolle.isHidden = false
    fenster.makeFirstResponder(datei)
}

/// Frischer Ausgangsstand: die Datei trägt getippten Text am Verwalter des
/// Fensters und obenauf die angemeldete Fensterhandlung, die Quicknote ist leer
/// und hat einen leeren eigenen Stapel.
func ausgangsstand(_ fall: String) {
    notiere(fall)
    quicknoteVerlassen()
    quick.string = ""
    quicknote.verwalter.removeAllActions()
    fensterhandlungZurueck = 0
    fensterhandlungWieder = 0
    fenster.undoAngekommen = 0
    fenster.redoAngekommen = 0
}

// Vorlauf: Tippen in der Datei landet am Verwalter des Fensters.
schritt {
    notiere("\(ProcessInfo.processInfo.operatingSystemVersionString)")
    notiere("Anwendung aktiv: \(NSApp.isActive ? "ja" : "nein"), Fenster ist Schlüsselfenster: \(fenster.isKeyWindow ? "ja" : "nein")")
    notiere("Vorlauf: 'abc' in die Datei tippen, dann eine Handlung am Verwalter des Fensters anmelden")
    fenster.makeFirstResponder(datei)
}
schritt { tippen("abc") }
schritt {
    datei.breakUndoCoalescing()
    fensterhandlungAnmelden()
    notiere("    Verwalter der Datei ist der des Fensters: \(datei.undoManager === fenster.undoManager ? "ja" : "nein")")
    quicknoteZeigen()
    notiere("    Verwalter der Quicknote ist der eigene: \(quick.undoManager === quicknote.verwalter ? "ja" : "nein"), ist der des Fensters: \(quick.undoManager === fenster.undoManager ? "ja" : "nein")")
    quicknoteVerlassen()
    lage("Ausgang")
}

// Zustand 1: direkt nach dem Öffnen, eigener Stapel leer.
schritt { ausgangsstand("Zustand 1: Quicknote öffnen (Ersthelfer), nichts getippt, cmd+z, dann shift+cmd+z"); quicknoteZeigen() }
schritt { lage("nach dem Öffnen"); cmdZ() }
schritt { lage("nach cmd+z"); cmdZ() }
schritt { lage("nach zweitem cmd+z"); shiftCmdZ() }
schritt { lage("nach shift+cmd+z") }

// Zustand 2: nach setString: und removeAllActions (Kopieren, `nach_kopie_leeren`).
schritt { ausgangsstand("Zustand 2: 'xyz' tippen, dann setString(\"\") und removeAllActions am eigenen Verwalter, cmd+z, shift+cmd+z"); quicknoteZeigen() }
schritt { tippen("xyz") }
schritt {
    lage("nach 'xyz'")
    quick.string = ""
    quicknote.verwalter.removeAllActions()
    lage("nach setString/removeAllActions")
    cmdZ()
}
schritt { lage("nach cmd+z"); shiftCmdZ() }
schritt { lage("nach shift+cmd+z") }

// Zustand 3: nach dem Zurücknehmen der letzten Tipp-Handlung.
schritt { ausgangsstand("Zustand 3: 'qq' tippen, cmd+z (nimmt das Tippen), cmd+z (eigener Undo-Stapel leer), shift+cmd+z zweimal"); quicknoteZeigen() }
schritt { tippen("qq") }
schritt { lage("nach 'qq'"); cmdZ() }
schritt { lage("nach cmd+z 1"); cmdZ() }
schritt { lage("nach cmd+z 2"); shiftCmdZ() }
schritt { lage("nach shift+cmd+z 1"); shiftCmdZ() }
schritt { lage("nach shift+cmd+z 2") }

// Zustand 4: Leeren als Handlung (`Quicknote::leeren`), dann bis zum leeren
// Stapel zurück.
schritt { ausgangsstand("Zustand 4: 'lm' tippen, Leeren als Handlung, cmd+z dreimal"); quicknoteZeigen() }
schritt { tippen("lm") }
schritt {
    lage("nach 'lm'")
    let bereich = NSRange(location: 0, length: (quick.string as NSString).length)
    quick.breakUndoCoalescing()
    if quick.shouldChangeText(in: bereich, replacementString: "") {
        quick.replaceCharacters(in: bereich, with: "")
        quick.didChangeText()
    }
    lage("nach Leeren")
    cmdZ()
}
schritt { lage("nach cmd+z 1"); cmdZ() }
schritt { lage("nach cmd+z 2"); cmdZ() }
schritt { lage("nach cmd+z 3") }

// Zustand 5: `undo:` und `redo:` an der Menueprüfung vorbei, unmittelbar an
// den Ersthelfer gerichtet, bei leerem eigenem Stapel. Misst, welchen
// Verwalter `NSWindow` nimmt, falls ein Weg die Ausgrauung je umginge.
schritt {
    ausgangsstand("Zustand 5: Quicknote öffnen, nichts getippt, undo: und redo: über sendAction an der Menüprüfung vorbei")
    quicknoteZeigen()
}
schritt {
    lage("nach dem Öffnen")
    notiere("      sendAction undo: angenommen: \(NSApp.sendAction(#selector(Messfenster.undo(_:)), to: nil, from: nil) ? "ja" : "nein")")
}
schritt {
    lage("nach undo:")
    notiere("      sendAction redo: angenommen: \(NSApp.sendAction(#selector(Messfenster.redo(_:)), to: nil, from: nil) ? "ja" : "nein")")
}
schritt { lage("nach redo:") }

// Gegenprobe: mit der Datei als Ersthelfer erreicht cmd+z die Fensterhandlung.
schritt { ausgangsstand("Gegenprobe: Datei als Ersthelfer, cmd+z") }
schritt { lage("Datei vorn"); cmdZ() }
schritt { lage("nach cmd+z") }

func abschliessen() {
    let pfad = URL(fileURLWithPath: CommandLine.arguments[0]).deletingLastPathComponent()
        .appendingPathComponent("messung.txt")
    try? (protokoll.joined(separator: "\n") + "\n").write(to: pfad, atomically: true, encoding: .utf8)
    NSApp.terminate(nil)
}

final class Anwendungsdelegierter: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        fenster.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
        nach(0.8) { fahren() }
    }
}

let delegierter = Anwendungsdelegierter()
app.delegate = delegierter
app.run()
