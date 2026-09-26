import Foundation
import CoreGraphics
let pid = pid_t(CommandLine.arguments[1])!
let start = Double(CommandLine.arguments[2])!  // seconds to wait before first key
let keys: [(String, CGKeyCode, CGEventFlags)] = [
  ("shift+cmd+return", 36, [.maskShift, .maskCommand]),
  ("cmd+return", 36, [.maskCommand]),
  ("opt+cmd+down", 125, [.maskAlternate, .maskCommand, .maskNumericPad, .maskSecondaryFn]),
  ("opt+cmd+up", 126, [.maskAlternate, .maskCommand, .maskNumericPad, .maskSecondaryFn]),
  ("shift+cmd+x", 7, [.maskShift, .maskCommand]),
  ("shift+cmd+delete", 51, [.maskShift, .maskCommand]),
  ("shift+cmd+p", 35, [.maskShift, .maskCommand]),
  ("f2", 120, [.maskSecondaryFn]),
]
Thread.sleep(forTimeInterval: start)
for (i, (name, code, flags)) in keys.enumerated() {
  if i > 0 { Thread.sleep(forTimeInterval: 1.2) }
  let src = CGEventSource(stateID: .hidSystemState)
  let d = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: true)!
  d.flags = flags
  let u = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: false)!
  u.flags = flags
  d.postToPid(pid); u.postToPid(pid)
  FileHandle.standardError.write("SENT \(name)\n".data(using: .utf8)!)
}
