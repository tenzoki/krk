import AppKit
final class D: NSObject, NSTextViewDelegate {
    let um = UndoManager()
    var aenderungen = 0
    var pruefungen = 0
    func undoManager(for view: NSTextView) -> UndoManager? { um }
    func textDidChange(_ n: Notification) { aenderungen += 1 }
    func textView(_ v: NSTextView, shouldChangeTextIn r: NSRange, replacementString s: String?) -> Bool { pruefungen += 1; return true }
}
let app = NSApplication.shared
app.setActivationPolicy(.accessory)
let win = NSWindow(contentRect: NSRect(x: 0, y: 0, width: 300, height: 200), styleMask: [.titled], backing: .buffered, defer: false)
let tv = NSTextView(frame: NSRect(x: 0, y: 0, width: 300, height: 200))
win.contentView = tv
let d = D()
tv.delegate = d
tv.allowsUndo = true
tv.isEditable = true
win.makeFirstResponder(tv)
func zeige(_ s: String) { print(s, "text=\(tv.string.debugDescription)", "textDidChange=\(d.aenderungen)", "shouldChange=\(d.pruefungen)", "canUndo=\(d.um.canUndo)") }
var schritte: [() -> Void] = [
  { tv.insertText("abc", replacementRange: NSRange(location: NSNotFound, length: 0)); zeige("nach insertText") },
  { tv.breakUndoCoalescing(); let r = NSRange(location: 0, length: 3); if tv.shouldChangeText(in: r, replacementString: "X") { tv.replaceCharacters(in: r, with: "X"); tv.didChangeText() }; zeige("nach ersetzen") },
  { d.um.undo(); zeige("nach undo (um.undo)") },
  { d.um.redo(); zeige("nach redo") },
  { NSApp.sendAction(Selector(("undo:")), to: nil, from: nil); zeige("nach undo: ueber die Kette") },
  { NSApp.sendAction(Selector(("undo:")), to: nil, from: nil); zeige("nach zweitem undo:") },
  { NSApp.sendAction(Selector(("redo:")), to: nil, from: nil); zeige("nach redo:") },
  { exit(0) },
]
func naechster() {
  guard !schritte.isEmpty else { return }
  let s = schritte.removeFirst()
  DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) { s(); naechster() }
}
naechster()
app.run()
