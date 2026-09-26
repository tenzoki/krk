// KRK — Prüfprogramm zum Defekt
// `work-packages/260925-2356-f2-oeffnet-krkhome-statt-notizfenster/issues/260926-0813_*_cmd-z-bei-offener-zelle-nimmt-einen-tabellenumbau-zurueck-…`.
//
// WEGWERF-PRÜFCODE. Kein Produktcode. Keine Tests, keine Architektur, keine
// Fehlerbehandlung über das Nötigste hinaus. Die Frage und die Auswertung
// stehen in README.md daneben.
//
// Gemessen wird eine Eigenschaft von AppKit, keine von KRK: welchen
// Rückgängigverwalter der Feldeditor einer bearbeiteten Zelle in einer
// view-basierten `NSTableView` benutzt, und ob `cmd+z` über das Menü in einer
// offenen Zelle den Verwalter des Fensters erreicht, in dem ein Tabellenumbau
// angemeldet ist. Dazu, was `reloadData` mit einer offenen Zelle tut, und in
// welchem Zustand Ersthelfer und Feldeditor während `controlTextDidEndEditing:`
// stehen.
//
// Die Tastendrücke gehen über `NSApp.postEvent(_:atStart:)` in die eigene
// Ereignisschlange, nicht über osascript.

import AppKit
import Foundation

// ---------------------------------------------------------------------------
// Durchgang
// ---------------------------------------------------------------------------

enum Durchgang: String {
    /// Der Feldeditor, den AppKit von sich aus einsetzt.
    case vorgabe
    /// Ein eigener Feldeditor aus `windowWillReturnFieldEditor:toObject:` mit
    /// eigenem Verwalter (die Empfehlung der Zweitlesung), `allowsUndo` aus
    /// wie bei jedem Feldeditor ab Werk.
    case eigener
    /// Wie `eigener`, dazu `allowsUndo` ein.
    case eigenerundo
    /// Wie `eigenerundo`, und der Feldeditor beantwortet `undo:` und `redo:`
    /// selbst an seinem Verwalter, samt Menüprüfung.
    case eigenerfaengt

    var eigenerFeldeditor: Bool { self != .vorgabe }
}

let durchgang = Durchgang(rawValue: CommandLine.arguments.dropFirst().first ?? "vorgabe") ?? .vorgabe
/// Zweites Argument `ohnepruefung`: der eigene Feldeditor beantwortet
/// `validateMenuItem:` nicht selbst; gemessen wird, was die Oberklasse
/// `NSTextView` dann fuer "Rueckgaengig" antwortet.
let eigenePruefung = CommandLine.arguments.dropFirst(2).first != "ohnepruefung"

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
// Fenster, Menü, Tabelle
// ---------------------------------------------------------------------------

final class EigenerFeldeditor: NSTextView {
    var eigenerVerwalter = UndoManager()
    override var undoManager: UndoManager? { eigenerVerwalter }

    @objc func undo(_ sender: Any?) {
        guard durchgang == .eigenerfaengt else { return weiter(#selector(undo(_:)), sender) }
        eigenerVerwalter.undo()
    }
    @objc func redo(_ sender: Any?) {
        guard durchgang == .eigenerfaengt else { return weiter(#selector(redo(_:)), sender) }
        eigenerVerwalter.redo()
    }
    // Ohne `eigenerfaengt` soll der Weg so laufen, als gäbe es die zwei
    // Methoden nicht: weiter an das nächste Glied, das sie beantwortet.
    func weiter(_ sel: Selector, _ sender: Any?) {
        NSApp.sendAction(sel, to: nil, from: sender)
    }
    override func responds(to aSelector: Selector!) -> Bool {
        if durchgang != .eigenerfaengt && (aSelector == #selector(undo(_:)) || aSelector == #selector(redo(_:))) {
            return false
        }
        return super.responds(to: aSelector)
    }
    // Jede Zelle beginnt mit leerem Stapel: der Feldeditor ist einer für alle
    // Zellen, und ein Rest aus der vorigen Zelle nähme sonst deren Tippen am
    // Text dieser zurück.
    var geleert = 0
    override func becomeFirstResponder() -> Bool {
        let ok = super.becomeFirstResponder()
        if ok && durchgang == .eigenerfaengt { eigenerVerwalter.removeAllActions(); geleert += 1 }
        return ok
    }
    override func validateMenuItem(_ item: NSMenuItem) -> Bool {
        if eigenePruefung {
            if item.action == #selector(undo(_:)) { return eigenerVerwalter.canUndo }
            if item.action == #selector(redo(_:)) { return eigenerVerwalter.canRedo }
        }
        return super.validateMenuItem(item)
    }
}

/// Zeichnet auf, wer `undo:` beantwortet und mit welchem Verwalter.
final class Messfenster: NSWindow {
    @objc func undo(_ sender: Any?) {
        let fe = firstResponder
        notiere("    NSWindow.undo: Ersthelfer \(name(fe)), dessen Verwalter \(name(fe?.undoManager)) canUndo \(fe?.undoManager?.canUndo ?? false), Fensterverwalter canUndo \(undoManager?.canUndo ?? false)")
        let m = class_getInstanceMethod(NSWindow.self, #selector(undo(_:)))!
        typealias F = @convention(c) (AnyObject, Selector, Any?) -> Void
        unsafeBitCast(method_getImplementation(m), to: F.self)(self, #selector(undo(_:)), sender)
    }
}

final class Steuerung: NSObject, NSTableViewDataSource, NSTableViewDelegate, NSTextFieldDelegate,
    NSWindowDelegate
{
    var zeilen = ["A", "B"]
    let tabelle: NSTableView
    var endeBuch: [String] = []
    lazy var eigener: EigenerFeldeditor = {
        let e = EigenerFeldeditor()
        e.isFieldEditor = true
        e.allowsUndo = durchgang != .eigener
        return e
    }()
    var eigenerGefragt = 0
    /// Fall 7: der Umbau im Ende selbst, wie `zelle_festschreiben` in KRK.
    var umbauImEnde = false

    init(tabelle: NSTableView) { self.tabelle = tabelle }

    func numberOfRows(in tableView: NSTableView) -> Int { zeilen.count }

    func tableView(_ tableView: NSTableView, viewFor tableColumn: NSTableColumn?, row: Int) -> NSView? {
        let feld = NSTextField(string: zeilen[row])
        feld.isEditable = true
        feld.isBordered = false
        feld.drawsBackground = false
        feld.delegate = self
        feld.frame = NSRect(x: 26, y: 2, width: 300, height: 20)
        let zelle = NSTableCellView(frame: NSRect(x: 0, y: 0, width: 360, height: 24))
        zelle.addSubview(feld)
        zelle.textField = feld
        return zelle
    }

    func control(_ control: NSControl, textShouldEndEditing fieldEditor: NSText) -> Bool {
        endeBuch.append("textShouldEndEditing: Zeile \(tabelle.row(for: control)) Text '\(fieldEditor.string)'")
        return true
    }

    func controlTextDidEndEditing(_ obj: Notification) {
        let feld = obj.object as? NSTextField
        let fenster = tabelle.window!
        let fe = fenster.firstResponder as? NSTextView
        endeBuch.append(
            "controlTextDidEndEditing: Zeile \(feld.map { tabelle.row(for: $0) } ?? -9) Wert '\(feld?.stringValue ?? "")'"
                + " | Ersthelfer \(name(fenster.firstResponder)) isFieldEditor \(fe?.isFieldEditor ?? false)"
                + " Delegierter des Feldeditors \(name(fe?.delegate as AnyObject?))"
                + " currentEditor \(name(feld?.currentEditor()))")
        if umbauImEnde, let feld {
            umbauImEnde = false
            zeilen[tabelle.row(for: feld)] = feld.stringValue
            tabelle.reloadData()
            endeBuch.append("reloadData im Ende fertig, Ersthelfer \(name(tabelle.window!.firstResponder))")
        }
    }

    func windowWillReturnFieldEditor(_ sender: NSWindow, to client: Any?) -> Any? {
        guard durchgang.eigenerFeldeditor, let feld = client as? NSView, feld.isDescendant(of: tabelle) else {
            return nil
        }
        eigenerGefragt += 1
        return eigener
    }
}

let app = NSApplication.shared
app.setActivationPolicy(.regular)

// Das Menü: Rückgängig und Wiederholen, wie im Hauptmenü von KRK an `undo:`
// und `redo:` gebunden und damit an den Ersthelfer gerichtet.
let hauptmenue = NSMenu()
let appEintrag = NSMenuItem()
hauptmenue.addItem(appEintrag)
appEintrag.submenu = NSMenu()
let bearbeitenEintrag = NSMenuItem()
hauptmenue.addItem(bearbeitenEintrag)
let bearbeiten = NSMenu(title: "Bearbeiten")
bearbeiten.addItem(withTitle: "Rückgängig", action: Selector(("undo:")), keyEquivalent: "z")
let wieder = bearbeiten.addItem(withTitle: "Wiederholen", action: Selector(("redo:")), keyEquivalent: "z")
wieder.keyEquivalentModifierMask = [.command, .shift]
bearbeitenEintrag.submenu = bearbeiten
app.mainMenu = hauptmenue

let fenster = Messfenster(
    contentRect: NSRect(x: 200, y: 200, width: 420, height: 300),
    styleMask: [.titled, .closable], backing: .buffered, defer: false)
fenster.title = "KRK Prüfprogramm: Rückgängig in der Zelle (\(durchgang.rawValue))"

let tabelle = NSTableView()
tabelle.rowHeight = 24
let spalte = NSTableColumn(identifier: NSUserInterfaceItemIdentifier("aufgabe"))
spalte.width = 360
tabelle.addTableColumn(spalte)
tabelle.headerView = nil
let steuerung = Steuerung(tabelle: tabelle)
tabelle.dataSource = steuerung
tabelle.delegate = steuerung
fenster.delegate = steuerung

let rolle = NSScrollView(frame: fenster.contentView!.bounds)
rolle.autoresizingMask = [.width, .height]
rolle.documentView = tabelle
fenster.contentView!.addSubview(rolle)

// Der Umbau: wie `umkehrung_anmelden` in KRK am Verwalter des Fensters, der
// auch der Verwalter der (hier fehlenden) Textfläche und der Tabelle ist.
var umbautenZurueck = 0
func umbauAnmelden() {
    fenster.undoManager!.registerUndo(withTarget: steuerung) { s in
        umbautenZurueck += 1
        s.zeilen = ["A", "B"]
        s.tabelle.reloadData()
    }
    fenster.undoManager!.setActionName("Umbau")
}

// ---------------------------------------------------------------------------
// Tastenweg
// ---------------------------------------------------------------------------

func taste(_ c: String, _ flags: NSEvent.ModifierFlags = [], _ code: UInt16 = 0) {
    let e = NSEvent.keyEvent(
        with: .keyDown, location: .zero, modifierFlags: flags,
        timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: fenster.windowNumber,
        context: nil, characters: c, charactersIgnoringModifiers: c, isARepeat: false, keyCode: code)!
    NSApp.postEvent(e, atStart: false)
}
func cmdZ() { taste("z", .command, 6) }

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
    return "Menü Rückgängig \(r ? "frei" : "grau") Wiederholen \(w ? "frei" : "grau")"
}

func lage(_ marke: String) {
    let fe = fenster.firstResponder as? NSTextView
    let offen = fe?.isFieldEditor == true
    let zeile0 = (tabelle.view(atColumn: 0, row: 0, makeIfNecessary: false) as? NSTableCellView)?.textField?.stringValue ?? "?"
    let zeile1 = (tabelle.view(atColumn: 0, row: 1, makeIfNecessary: false) as? NSTableCellView)?.textField?.stringValue ?? "?"
    notiere(
        "  \(marke.padding(toLength: 30, withPad: " ", startingAt: 0)) Zelle offen \(offen ? "ja  " : "nein")"
            + " Feldeditortext '\(offen ? fe!.string : "-")' Zeilen [\(steuerung.zeilen.joined(separator: ", "))]"
            + " \(menueFrei()) Ersthelfer \(name(fenster.firstResponder)) Felder ['\(zeile0)', '\(zeile1)'] Umbauten zurück \(umbautenZurueck)")
}

/// Die Eigenschaften des Feldeditors während der Bearbeitung, zum Vergleich
/// zwischen dem von AppKit und dem eigenen.
func eigenschaften() {
    guard let fe = fenster.firstResponder as? NSTextView else { return }
    let werte: [(String, Bool)] = [
        ("isRichText", fe.isRichText), ("importsGraphics", fe.importsGraphics),
        ("allowsUndo", fe.allowsUndo), ("isFieldEditor", fe.isFieldEditor),
        ("smartInsertDelete", fe.smartInsertDeleteEnabled),
        ("QuoteSubstitution", fe.isAutomaticQuoteSubstitutionEnabled),
        ("DashSubstitution", fe.isAutomaticDashSubstitutionEnabled),
        ("TextReplacement", fe.isAutomaticTextReplacementEnabled),
        ("SpellingCorrection", fe.isAutomaticSpellingCorrectionEnabled),
        ("ContinuousSpellChecking", fe.isContinuousSpellCheckingEnabled),
        ("GrammarChecking", fe.isGrammarCheckingEnabled),
        ("LinkDetection", fe.isAutomaticLinkDetectionEnabled),
        ("DataDetection", fe.isAutomaticDataDetectionEnabled),
        ("TextCompletion", fe.isAutomaticTextCompletionEnabled),
        ("usesFindBar", fe.usesFindBar), ("usesFontPanel", fe.usesFontPanel),
        ("usesRuler", fe.usesRuler), ("drawsBackground", fe.drawsBackground),
        ("isHorizontallyResizable", fe.isHorizontallyResizable),
        ("isVerticallyResizable", fe.isVerticallyResizable),
        ("widthTracksTextView", fe.textContainer?.widthTracksTextView ?? false),
    ]
    notiere("  Eigenschaften: " + werte.map { "\($0.0)=\($0.1 ? "ja" : "nein")" }.joined(separator: " "))
}

func verwalterBefund() {
    eigenschaften()
    let fe = fenster.firstResponder as? NSTextView
    let vFe = fe?.undoManager
    notiere("  Ersthelfer \(name(fenster.firstResponder)), isFieldEditor \(fe?.isFieldEditor ?? false)")
    notiere("  Verwalter des Feldeditors \(name(vFe)), derselbe wie der des Fensters: \(vFe === fenster.undoManager ? "ja" : "nein")")
    notiere("  Verwalter der Tabelle \(name(tabelle.undoManager)), derselbe wie der des Fensters: \(tabelle.undoManager === fenster.undoManager ? "ja" : "nein")")
    if durchgang.eigenerFeldeditor {
        notiere("  windowWillReturnFieldEditor für die Zelle beantwortet: \(steuerung.eigenerGefragt)-mal, Ersthelfer ist der eigene: \(fenster.firstResponder === steuerung.eigener ? "ja" : "nein")")
    }
}

// Die Schritte laufen nacheinander, mit Leerlauf dazwischen, damit jedes
// gepostete Ereignis ankommt, bevor gemessen wird.
var schritte: [(Double, () -> Void)] = []
func schritt(_ s: Double = 0.25, _ tun: @escaping () -> Void) { schritte.append((s, tun)) }
func fahren(_ i: Int = 0) {
    guard i < schritte.count else { abschliessen(); return }
    nach(schritte[i].0) { schritte[i].1(); fahren(i + 1) }
}

func beginnen(_ zeile: Int) {
    tabelle.selectRowIndexes(IndexSet(integer: zeile), byExtendingSelection: false)
    tabelle.editColumn(0, row: zeile, with: nil, select: true)
}

// Fall 1 (Nutzerprüfung 1 der Zweitlesung): B über A schieben, B öffnen, ohne
// Tippen cmd+z.
schritt {
    notiere("Durchgang \(durchgang.rawValue)")
    notiere("\(ProcessInfo.processInfo.operatingSystemVersionString)")
    notiere("Anwendung aktiv: \(NSApp.isActive ? "ja" : "nein"), Fenster ist Schlüsselfenster: \(fenster.isKeyWindow ? "ja" : "nein")")
    notiere("Fall 1: Umbau (B über A), Zelle der Zeile 0 öffnen, ohne Tippen cmd+z")
    fenster.makeFirstResponder(tabelle)
    steuerung.zeilen = ["B", "A"]
    tabelle.reloadData()
    umbauAnmelden()
    notiere("  Fensterverwalter canUndo nach dem Umbau: \(fenster.undoManager!.canUndo)")
    beginnen(0)
}
schritt { verwalterBefund(); lage("Zelle offen, nichts getippt"); cmdZ() }
schritt { lage("nach cmd+z"); cmdZ() }
schritt { lage("nach zweitem cmd+z") }

// Fall 2 (Nutzerprüfung 2): drei Zeichen tippen, cmd+z viermal.
schritt {
    notiere("Fall 2: frischer Umbau, Zelle öffnen, 'xyz' tippen, cmd+z viermal")
    fenster.makeFirstResponder(tabelle)
    umbautenZurueck = 0
    steuerung.zeilen = ["B", "A"]
    tabelle.reloadData()
    umbauAnmelden()
    beginnen(0)
}
schritt {
    // Ans Ende der Auswahl, dann tippen: der ausgewählte Text wird sonst ersetzt.
    for z in ["x", "y", "z"] { taste(z) }
}
schritt { lage("nach 'xyz'"); cmdZ() }
schritt { lage("nach cmd+z 1"); cmdZ() }
schritt { lage("nach cmd+z 2"); cmdZ() }
schritt { lage("nach cmd+z 3"); cmdZ() }
schritt { lage("nach cmd+z 4") }

// Fall 3: reloadData unter einer offenen Zelle mit getipptem Text — was
// Fall A/B der Zweitlesung unterscheidet.
schritt {
    notiere("Fall 3: Zelle der Zeile 0 öffnen, 'neu' tippen, dann reloadData mit getauschten Zeilen")
    fenster.makeFirstResponder(tabelle)
    steuerung.zeilen = ["A", "B"]
    tabelle.reloadData()
    beginnen(0)
}
schritt { for z in ["n", "e", "u"] { taste(z) } }
schritt {
    lage("vor reloadData")
    steuerung.endeBuch = []
    steuerung.zeilen = ["B", "A"]
    tabelle.reloadData()
    notiere("  Delegiertenwege während reloadData: \(steuerung.endeBuch.isEmpty ? "keine" : steuerung.endeBuch.joined(separator: " || "))")
    lage("gleich nach reloadData")
}
schritt { lage("nach Leerlauf") }

// Fall 4: der Zustand während controlTextDidEndEditing: bei einem Ende über
// makeFirstResponder: (der Weg von `bearbeitung_beenden` in KRK).
schritt {
    notiere("Fall 4: Zelle öffnen, 'ende' tippen, Ende über makeFirstResponder(tabelle)")
    fenster.makeFirstResponder(tabelle)
    steuerung.zeilen = ["A", "B"]
    tabelle.reloadData()
    beginnen(1)
}
schritt { for z in ["e", "n", "d", "e"] { taste(z) } }
schritt {
    steuerung.endeBuch = []
    let ok = fenster.makeFirstResponder(tabelle)
    notiere("  makeFirstResponder: \(ok)")
    for z in steuerung.endeBuch { notiere("  \(z)") }
    lage("danach")
}

// Fall 5: kein Rest aus der vorigen Zelle.
schritt {
    notiere("Fall 5: Zeile 0 öffnen, 'abc' tippen, übernehmen; Zeile 1 öffnen, cmd+z")
    fenster.makeFirstResponder(tabelle)
    steuerung.zeilen = ["A", "B"]
    tabelle.reloadData()
    beginnen(0)
}
schritt { for z in ["a", "b", "c"] { taste(z) } }
schritt {
    fenster.makeFirstResponder(tabelle)
    lage("Zeile 0 übernommen")
    beginnen(1)
}
schritt { lage("Zeile 1 offen"); cmdZ() }
schritt {
    lage("nach cmd+z")
    if durchgang.eigenerFeldeditor {
        notiere("  Stapel des eigenen Feldeditors beim Beginn geleert: \(steuerung.eigener.geleert)-mal")
    }
}

// Fall 6: Fenster verliert und bekommt den Schlüsselrang mitten im Tippen;
// bleibt der Stapel der Zelle?
schritt {
    notiere("Fall 6: Zeile 0 öffnen, 'wx' tippen, Schlüsselrang ab und zurück, cmd+z")
    fenster.makeFirstResponder(tabelle)
    beginnen(0)
}
schritt { for z in ["w", "x"] { taste(z) } }
schritt { fenster.resignKey(); fenster.makeKey() }
schritt { lage("nach Rangwechsel"); cmdZ() }
schritt { lage("nach cmd+z") }

// Fall 7: reloadData innerhalb von controlTextDidEndEditing: (der Weg von
// `zelle_festschreiben` -> `umbau_anwenden` -> `zeilen_zeigen` in KRK).
schritt {
    notiere("Fall 7: Zeile 1 öffnen, 'neu' tippen, Ende über makeFirstResponder, im Ende reloadData")
    fenster.makeFirstResponder(tabelle)
    steuerung.zeilen = ["A", "B"]
    tabelle.reloadData()
    beginnen(1)
}
schritt { for z in ["n", "e", "u"] { taste(z) } }
schritt {
    steuerung.endeBuch = []
    steuerung.umbauImEnde = true
    let ok = fenster.makeFirstResponder(tabelle)
    notiere("  makeFirstResponder: \(ok)")
    for z in steuerung.endeBuch { notiere("  \(z)") }
    lage("danach")
}
schritt { lage("nach Leerlauf") }

func abschliessen() {
    let pfad = URL(fileURLWithPath: CommandLine.arguments[0]).deletingLastPathComponent()
        .appendingPathComponent("messung-\(durchgang.rawValue)\(eigenePruefung ? "" : "-ohnepruefung").txt")
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
