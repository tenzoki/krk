import AppKit
final class D: NSObject, NSTextViewDelegate {
    let um = UndoManager()
    var aenderungen = 0, pruefungen = 0, notiz = 0, speicher = 0
    func undoManager(for view: NSTextView) -> UndoManager? { um }
    func textDidChange(_ n: Notification) { aenderungen += 1 }
    func textView(_ v: NSTextView, shouldChangeTextIn r: NSRange, replacementString s: String?) -> Bool { pruefungen += 1; return true }
}
let app = NSApplication.shared
app.setActivationPolicy(.regular)
let win = NSWindow(contentRect: NSRect(x: 100, y: 100, width: 300, height: 200), styleMask: [.titled], backing: .buffered, defer: false)
let tv = NSTextView(frame: NSRect(x: 0, y: 0, width: 300, height: 200))
win.contentView = tv
let d = D()
tv.delegate = d
tv.allowsUndo = true
tv.isEditable = true
NotificationCenter.default.addObserver(forName: NSText.didChangeNotification, object: tv, queue: nil) { _ in d.notiz += 1 }
NotificationCenter.default.addObserver(forName: NSTextStorage.didProcessEditingNotification, object: tv.textStorage, queue: nil) { _ in d.speicher += 1 }
win.makeKeyAndOrderFront(nil)
app.activate(ignoringOtherApps: true)
win.makeFirstResponder(tv)
func zeige(_ s: String) { print(s, "text=\(tv.string.debugDescription)", "textDidChange=\(d.aenderungen)", "notif=\(d.notiz)", "shouldChange=\(d.pruefungen)", "storage=\(d.speicher)", "undoMgrIsOurs=\(tv.undoManager === d.um)") }
func taste(_ z: String) {
  for typ in [NSEvent.EventType.keyDown, .keyUp] {
    let e = NSEvent.keyEvent(with: typ, location: .zero, modifierFlags: [], timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: win.windowNumber, context: nil, characters: z, charactersIgnoringModifiers: z, isARepeat: false, keyCode: 0)!
    NSApp.postEvent(e, atStart: false)
  }
}
var schritte: [() -> Void] = [
  { taste("a") }, { taste("b") }, { zeige("nach zwei Tasten") },
  { tv.breakUndoCoalescing(); let r = NSRange(location: 0, length: tv.string.utf16.count); if tv.shouldChangeText(in: r, replacementString: "X") { tv.replaceCharacters(in: r, with: "X"); tv.didChangeText() }; zeige("nach ersetzen") },
  { tv.undoManager?.undo(); zeige("nach undo") },
  { tv.undoManager?.undo(); zeige("nach zweitem undo") },
  { tv.undoManager?.redo(); zeige("nach redo") },
  { tv.isEditable = false; let r = NSRange(location: 0, length: 0); let ja = tv.shouldChangeText(in: r, replacementString: "y"); if ja { tv.replaceCharacters(in: r, with: "y"); tv.didChangeText() }; zeige("nicht bearbeitbar, shouldChangeText=\(ja)") },
  { tv.string = "gelesen"; zeige("nach setString") },
  { exit(0) },
]
func naechster() {
  guard !schritte.isEmpty else { return }
  let s = schritte.removeFirst()
  DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) { s(); naechster() }
}
naechster()
app.run()
