Das Merkmal `CFRunLoop` in `Cargo.toml` hat keinen Nutzer mehr

---

`Cargo.toml` im Projektwurzelverzeichnis führt `objc2-core-foundation` mit den Merkmalen `std`, `CFArray`, `CFString` und `CFRunLoop`. Das letzte hat seit dem 260805 keinen Nutzer mehr: `crates/krk-ui/src/appkit/fsevents.rs` ist von `FSEventStreamScheduleWithRunLoop` auf `FSEventStreamSetDispatchQueue` gewechselt und braucht weder `CFRunLoop` noch `CFRunLoopMode` noch `kCFRunLoopCommonModes`.

```
$ grep -rn CFRunLoop crates/ --include='*.rs'
$ echo $?
1
```

---

## Warum es zählt

Der Kommentar über dem Eintrag begründet die Merkmalsliste ausdrücklich: "`CFString` fuer den einzelnen Pfad und `CFRunLoop` fuer die Laufschleife, in die der Strom gehaengt wird. Die Vorgabemerkmale zoegen ein halbes Hundert weiterer Typen mit herein, von denen KRK keinen nennt; deshalb `default-features = false`." Die Begründung ist mit dem Wechsel für `CFRunLoop` hinfällig, und ein Kommentar, der eine Verwendung behauptet, die es nicht gibt, schickt den nächsten Leser auf die Suche.

Die Wirkung ist im Übrigen klein: das Merkmal zieht ein paar Typen mehr in die Übersetzung und sonst nichts.

## Was zu tun ist

Das Merkmal `CFRunLoop` aus der Liste entfernen und den Kommentar darüber nachziehen, sodass er nur noch `CFArray` und `CFString` begründet.

## Warum es nicht gleich mitbehoben ist

Der Aufräumdurchgang vom 260805 war ausdrücklich auf `crates/` begrenzt. `Cargo.toml` liegt im Projektwurzelverzeichnis.

---

Herkunft: gefunden beim Beheben von `issues/260804-1451_c_fseventstreamschedulewithrunloop-ist-seit-macos-13-als-veraltet-gekennzeichnet.md` am 260805-0901. Jener Datensatz sagt den Wegfall des Merkmals voraus.

---
Resolved: Die Zeile "CFRunLoop" ist aus der Merkmalsliste von objc2-core-foundation entfernt, der Kommentar begründet jetzt nur noch CFArray und CFString.

Korrektur am Datensatz: die Wirkung war hier zu hoch angesetzt. Das Merkmal zog nicht "ein paar Typen mehr in die Übersetzung", sondern gar nichts, weil objc2-foundation es ohnehin einschaltet und Cargos Merkmalsvereinigung es damit eingeschaltet lässt. Cargo.lock ist nach der Änderung unverändert; der Gewinn liegt allein beim Kommentar, der eine Verwendung behauptete, die es nicht gibt.

Beleg auf drei Beinen: kein Modul nennt einen der Typen (einzige Einfuhr crates/krk-ui/src/appkit/fsevents.rs:89 mit CFArray, CFIndex, CFRetained, CFString; die NSRunLoop-Treffer kommen aus objc2-foundation); keine merkmalsabhängige Übersetzung (nur cfg!(debug_assertions) in krk-bench/src/bericht.rs:73); cargo tree -e features -i objc2-core-foundation nannte vorher zwei Einschalter, danach nur noch objc2-foundation.
