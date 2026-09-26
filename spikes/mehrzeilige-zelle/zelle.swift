// KRK — Prüfprogramm zu Schritt 4.2 des Plans
// `work-packages/260925-2356-f2-oeffnet-krkhome-statt-notizfenster/plans/260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`.
//
// WEGWERF-PRÜFCODE. Kein Produktcode. Keine Tests, keine Architektur, keine
// Fehlerbehandlung über das Nötigste hinaus. Die Frage und die Auswertung
// stehen in README.md daneben.
//
// Gemessen wird eine Eigenschaft von AppKit, keine von KRK: wächst die Zeile
// einer `NSTableView` mit `usesAutomaticRowHeights`, während im Feldeditor
// eines mehrzeiligen `NSTextField` getippt wird — ohne Zutun, nach
// `noteHeightOfRowsWithIndexesChanged:`, oder erst nach
// `invalidateIntrinsicContentSize` am Feld — und bleibt der Feldeditor dabei
// Ersthelfer?
//
// Die Tastendrücke gehen über `NSApp.postEvent(_:atStart:)` in die eigene
// Ereignisschlange, nicht über osascript.

import AppKit
import Foundation

// ---------------------------------------------------------------------------
// Durchgang
// ---------------------------------------------------------------------------

/// Was nach jeder getippten Zeile zusätzlich geschieht.
enum Durchgang: String {
    /// Nichts. Misst allein, ob die Zeile von selbst wächst.
    case ohne
    /// Nach der ersten Messung `noteHeightOfRowsWithIndexesChanged:`, dann
    /// eine zweite Messung.
    case note
    /// Wie `note`, zuvor aber `invalidateIntrinsicContentSize` am Feld.
    case intrinsisch
    /// Wie `intrinsisch`, ausgelöst aber aus `controlTextDidChange:`, also
    /// bei jedem Anschlag und nicht erst zur Messung — die Form, die 4.3
    /// bauen würde.
    case jeanschlag
    /// Wie `jeanschlag`, das Feld misst seine Höhe während der Bearbeitung
    /// aber am Text des Feldeditors (`Notizfeld.intrinsicContentSize`) statt
    /// an seinem eigenen, während des Tippens unveränderten Wert.
    case feldeditormass
    /// Wie `feldeditormass`, aber ohne `noteHeightOfRowsWithIndexesChanged:`:
    /// genügt das Ungültigmachen der Eigengröße allein?
    case feldeditorohnenote

    /// Misst das Feld seine Höhe am Text des Feldeditors?
    var misstAmFeldeditor: Bool { self == .feldeditormass || self == .feldeditorohnenote }
    /// Wird bei jedem Anschlag nachgemessen statt einmal je Zeile?
    var jeAnschlag: Bool { self == .jeanschlag || misstAmFeldeditor }
}

/// Ein Textfeld, das während der Bearbeitung die Höhe des Feldeditortexts
/// meldet. Allein im Durchgang `feldeditormass` wirksam.
final class Notizfeld: NSTextField {
    override var intrinsicContentSize: NSSize {
        var mass = super.intrinsicContentSize
        guard durchgang.misstAmFeldeditor,
            let editor = currentEditor() as? NSTextView,
            let layout = editor.layoutManager, let behaelter = editor.textContainer
        else { return mass }
        layout.ensureLayout(for: behaelter)
        mass.height = ceil(layout.usedRect(for: behaelter).height + 2 * editor.textContainerInset.height)
        return mass
    }
}

let durchgang =
    Durchgang(rawValue: CommandLine.arguments.dropFirst().first ?? "note") ?? .note

/// Zweites Argument `weich`: das Feld behält die Vorgabe seines vertikalen
/// Stauchwiderstands. Dann gewinnt die Höhenbindung der Zeilenansicht
/// (`NSTableRowView_Encapsulated_Layout_Height`, Rang 500), und nicht einmal
/// eine nie bearbeitete dreizeilige Zelle wächst.
let weich = CommandLine.arguments.dropFirst(2).first == "weich"

var protokoll: [String] = []

func notiere(_ zeile: String) {
    print(zeile)
    protokoll.append(zeile)
}

// ---------------------------------------------------------------------------
// Tabelle
// ---------------------------------------------------------------------------

let themenbreite: CGFloat = 140
let notizbreite: CGFloat = 300
let rand: CGFloat = 4

final class Steuerung: NSObject, NSTableViewDataSource, NSTableViewDelegate,
    NSTextFieldDelegate
{
    var themen = ["Einkauf", "Reise"]
    // Die zweite Notiz ist dreizeilig und wird nie bearbeitet: sie zeigt, dass
    // die automatische Zeilenhöhe außerhalb der Bearbeitung trägt.
    var notizen = ["eine Zeile", "eins\nzwei\ndrei"]
    var felder: [Int: NSTextField] = [:]
    let tabelle: NSTableView

    init(tabelle: NSTableView) {
        self.tabelle = tabelle
    }

    func numberOfRows(in tableView: NSTableView) -> Int { themen.count }

    func tableView(
        _ tableView: NSTableView, viewFor tableColumn: NSTableColumn?, row: Int
    ) -> NSView? {
        let notiz = tableColumn?.identifier.rawValue == "notiz"
        let feld = Notizfeld(wrappingLabelWithString: notiz ? notizen[row] : themen[row])
        feld.isEditable = true
        feld.isSelectable = true
        feld.isBordered = false
        feld.drawsBackground = false
        feld.usesSingleLineMode = false
        feld.maximumNumberOfLines = 0
        feld.lineBreakMode = .byWordWrapping
        feld.cell?.wraps = true
        feld.cell?.isScrollable = false
        feld.preferredMaxLayoutWidth = (notiz ? notizbreite : themenbreite) - 2 * rand
        feld.delegate = self
        if !weich { feld.setContentCompressionResistancePriority(.required, for: .vertical) }
        feld.translatesAutoresizingMaskIntoConstraints = false

        let zelle = NSTableCellView()
        zelle.addSubview(feld)
        zelle.textField = feld
        NSLayoutConstraint.activate([
            feld.leadingAnchor.constraint(equalTo: zelle.leadingAnchor, constant: rand),
            feld.trailingAnchor.constraint(equalTo: zelle.trailingAnchor, constant: -rand),
            feld.topAnchor.constraint(equalTo: zelle.topAnchor, constant: 2),
            feld.bottomAnchor.constraint(equalTo: zelle.bottomAnchor, constant: -2),
        ])
        if notiz { felder[row] = feld }
        return zelle
    }

    // `return` schreibt in der Zelle einen Umbruch, statt sie zu beenden —
    // die Abbildung aus 4.3 (`260926-0112_*_was-tut-return-…`, Möglichkeit 1).
    func control(
        _ control: NSControl, textView: NSTextView, doCommandBy commandSelector: Selector
    ) -> Bool {
        if commandSelector == #selector(NSResponder.insertNewline(_:)) {
            textView.insertNewlineIgnoringFieldEditor(nil)
            return true
        }
        return false
    }

    func controlTextDidChange(_ obj: Notification) {
        guard durchgang.jeAnschlag, let feld = obj.object as? NSTextField else { return }
        feld.invalidateIntrinsicContentSize()
        let zeile = tabelle.row(for: feld)
        if zeile >= 0 && durchgang != .feldeditorohnenote {
            tabelle.noteHeightOfRows(withIndexesChanged: IndexSet(integer: zeile))
        }
    }
}

// ---------------------------------------------------------------------------
// Aufbau
// ---------------------------------------------------------------------------

let app = NSApplication.shared
app.setActivationPolicy(.regular)

let fenster = NSWindow(
    contentRect: NSRect(x: 200, y: 200, width: themenbreite + notizbreite + 40, height: 360),
    styleMask: [.titled, .closable], backing: .buffered, defer: false)
fenster.title = "KRK Prüfprogramm: mehrzeilige Zelle (\(durchgang.rawValue))"

let tabelle = NSTableView()
tabelle.usesAutomaticRowHeights = true
tabelle.rowHeight = 20
for (kennung, titel, breite) in [
    ("thema", "Thema", themenbreite), ("notiz", "Notiz", notizbreite),
] {
    let spalte = NSTableColumn(identifier: NSUserInterfaceItemIdentifier(kennung))
    spalte.title = titel
    spalte.width = breite
    spalte.resizingMask = []
    tabelle.addTableColumn(spalte)
}
let steuerung = Steuerung(tabelle: tabelle)
tabelle.dataSource = steuerung
tabelle.delegate = steuerung

let rolle = NSScrollView(frame: fenster.contentView!.bounds)
rolle.autoresizingMask = [.width, .height]
rolle.documentView = tabelle
rolle.hasVerticalScroller = true
fenster.contentView!.addSubview(rolle)

// ---------------------------------------------------------------------------
// Tastenweg
// ---------------------------------------------------------------------------

var anschlaegeGesehen = 0
var anschlaegeUmgeleitet = 0

// Derselbe Mechanismus wie KRKs Ereignisabgriff. Ist das Fenster nicht
// Schlüsselfenster, weil das Programm nicht in den Vordergrund kam, stellt
// der Abgriff den Anschlag dem Fenster selbst zu und zählt das mit; dann ist
// der Tastenweg nicht der eines echten Anschlags und der Befund sagt es.
NSEvent.addLocalMonitorForEvents(matching: .keyDown) { ereignis in
    anschlaegeGesehen += 1
    if NSApp.keyWindow === fenster { return ereignis }
    anschlaegeUmgeleitet += 1
    fenster.sendEvent(ereignis)
    return nil
}

func tippe(_ text: String) {
    for zeichen in text {
        let ist_umbruch = zeichen == "\n"
        let c = ist_umbruch ? "\r" : String(zeichen)
        let e = NSEvent.keyEvent(
            with: .keyDown, location: .zero, modifierFlags: [],
            timestamp: ProcessInfo.processInfo.systemUptime,
            windowNumber: fenster.windowNumber, context: nil,
            characters: c, charactersIgnoringModifiers: c, isARepeat: false,
            keyCode: ist_umbruch ? 36 : 0)!
        NSApp.postEvent(e, atStart: false)
    }
}

// ---------------------------------------------------------------------------
// Messung
// ---------------------------------------------------------------------------

func ersthelferName() -> String {
    guard let r = fenster.firstResponder else { return "nil" }
    return String(describing: type(of: r))
}

func lage(_ marke: String) -> CGFloat {
    let feld = steuerung.felder[0]!
    let hoehe = tabelle.rect(ofRow: 0).height
    let editor = feld.currentEditor()
    let feldeditorIstErsthelfer = editor != nil && fenster.firstResponder === editor
    let text = (editor as? NSTextView)?.string ?? feld.stringValue
    let editorhoehe = editor?.frame.height ?? 0
    let zeilen = text.split(separator: "\n", omittingEmptySubsequences: false).count
    notiere(
        String(
            format: "  %-26@ Zeilenhöhe %6.1f  Feld %6.1f  intrinsisch %6.1f  Feldeditor %6.1f  Textzeilen %d  Ersthelfer %@  Feldeditor-Ersthelfer %@",
            marke as NSString, hoehe, feld.frame.height, feld.intrinsicContentSize.height,
            editorhoehe, zeilen, ersthelferName() as NSString, (feldeditorIstErsthelfer ? "ja" : "nein") as NSString))
    return hoehe
}

let schritte: [(String, String)] = [
    ("Zeile 1", "erste Zeile"),
    ("Zeile 2", "\nzweite Zeile"),
    ("Zeile 3", "\ndritte Zeile"),
]

func nachLeerlauf(_ sekunden: Double, _ tun: @escaping () -> Void) {
    DispatchQueue.main.asyncAfter(deadline: .now() + sekunden, execute: tun)
}

func schritt(_ i: Int, vorher: CGFloat) {
    if i == schritte.count {
        abschliessen()
        return
    }
    let (name, text) = schritte[i]
    tippe(text)
    nachLeerlauf(0.4) {
        notiere("\(name):")
        let a = lage("ohne Zutun")
        var b = a
        let weiter = {
            let wuchs_ohne = a > vorher + 0.5
            let wuchs_mit = b > vorher + 0.5
            switch durchgang {
            case .ohne:
                notiere("  -> ohne Zutun gewachsen: \(wuchs_ohne ? "ja" : "nein")")
            case .note, .intrinsisch:
                notiere("  -> ohne Zutun gewachsen: \(wuchs_ohne ? "ja" : "nein"), nach Nachmessen gewachsen: \(wuchs_mit ? "ja" : "nein")")
            case .jeanschlag, .feldeditormass, .feldeditorohnenote:
                notiere("  -> mit Nachmessen je Anschlag gewachsen: \(wuchs_ohne ? "ja" : "nein")")
            }
            schritt(i + 1, vorher: b)
        }
        switch durchgang {
        case .ohne, .jeanschlag, .feldeditormass, .feldeditorohnenote:
            weiter()
        case .note, .intrinsisch:
            if durchgang == .intrinsisch { steuerung.felder[0]!.invalidateIntrinsicContentSize() }
            tabelle.noteHeightOfRows(withIndexesChanged: IndexSet(integer: 0))
            nachLeerlauf(0.4) {
                b = lage(
                    durchgang == .note
                        ? "nach noteHeight…" : "nach invalidate+noteHeight")
                weiter()
            }
        }
    }
}

func abschliessen() {
    notiere("Anschläge gesehen \(anschlaegeGesehen), davon am Schlüsselfenster vorbei zugestellt \(anschlaegeUmgeleitet)")
    let pfad = URL(fileURLWithPath: CommandLine.arguments[0])
        .deletingLastPathComponent()
        .appendingPathComponent("messung-\(durchgang.rawValue)\(weich ? "-weich" : "").txt")
    try? (protokoll.joined(separator: "\n") + "\n").write(to: pfad, atomically: true, encoding: .utf8)
    NSApp.terminate(nil)
}

final class Anwendungsdelegierter: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        fenster.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
        nachLeerlauf(0.8) {
            notiere("Durchgang \(durchgang.rawValue)\(weich ? " (weich)" : "")")
            notiere("\(ProcessInfo.processInfo.operatingSystemVersionString)")
            notiere("Anwendung aktiv: \(NSApp.isActive ? "ja" : "nein"), Fenster ist Schlüsselfenster: \(fenster.isKeyWindow ? "ja" : "nein")")
            tabelle.editColumn(1, row: 0, with: nil, select: true)
            nachLeerlauf(0.3) {
                notiere(String(format: "Vergleichszeile mit drei Zeilen, nie bearbeitet: Zeilenhöhe %.1f", tabelle.rect(ofRow: 1).height))
                notiere("Ausgang:")
                let h = lage("nach editColumn")
                schritt(0, vorher: h)
            }
        }
    }
}

let delegierter = Anwendungsdelegierter()
app.delegate = delegierter
app.run()
