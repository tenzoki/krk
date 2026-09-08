Die Syntaxhervorhebung hat keine Messstelle, und die Zahlen im Modulkopf von `hervorhebung.rs` sind nicht wiederholbar
---
`CLAUDE.md` führt die Geschwindigkeit der Syntaxhervorhebung als vierten, ungemessenen Gegenstand der späteren Messrunde. Der Baum hat dafür keine Stelle: weder eine `#[ignore]`-Probe noch einen Eintrag in `krk-bench`, und die einzige Funktion ohne Vorlage (`formatieren`) steht unter `cfg(test)`. Die sieben Zahlen im Modulkopf (0,3 MB/s, 864 ms, 7 074 ms, 0,39 ms, 129 ms, 780 Byte je Haltepunkt, 2,9 ms Laden) stammen aus einem Lauf vom 260810, den niemand wiederholen kann.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/hervorhebung.rs:77-131`: die Messtabellen im Modulkopf, ohne Angabe, welches Programm sie erzeugt hat.
- `crates/krk-ui/src/hervorhebung.rs:545-558`: `ZUSTANDSABSTAND = 32` ist aus zwei Messungen abgeleitet (780 Byte je Zustand, 0,19 ms je Zeile), beide ohne Messstelle im Baum.
- `crates/krk-ui/src/hervorhebung.rs:1362-1365`: `formatieren` ist `#[cfg(test)]`; `fortschreiben` (`:1300`) ist der einzige Weg des Programms und nimmt nur den gehaltenen Stand.
- `crates/krk-ui/tests/syntaxkiste.rs:10-13` sagt ausdrücklich, dass die Geschwindigkeit dort nicht beantwortet wird.
- `crates/krk-bench/src/messen.rs` kennt keine Größe für die Hervorhebung (`grep -c hervorheb crates/krk-bench/src/messen.rs` → 0).

## Warum das zählt

Die Zusage aus C3 der Runde 2 hängt an der Maxime „superschnell“, und der Spec übergibt sie der Messrunde. Die Messrunde findet keinen Aufruf, den sie fahren könnte, und jede Änderung an `ZUSTANDSABSTAND` oder am Wiedereinstieg ist heute ohne Vergleichszahl.

## Vorschlag

Eine `#[ignore]`-Probe im Prüfmodul von `hervorhebung.rs`, die über `anwendung.rs` (die Datei der Messung vom 260810) und ein Vielfaches davon `fortschreiben` ohne und mit Vorlage fährt und Bytes je Sekunde ausgibt; der Aufruf mit `--ignored` gehört in die Aufstellung der Messrunde. Ob sie in `krk-bench` gehört, ist eine zweite Frage; die Probe genügt der Messrunde.

---
Resolved: Behoben auf dem im Datensatz genannten Vorschlag. `hervorhebung::tests::messstelle_der_einfaerbung` (`crates/krk-ui/src/hervorhebung.rs`) ist eine `#[ignore]`-Probe, die `fortschreiben` ueber `src/appkit/anwendung.rs` und ein Vielfaches davon fuehrt, einmal ohne Vorlage (voller Durchgang) und einmal mit (ein Anschlag in der Mitte), und Bytes je Sekunde samt der Zahl der Haltepunkte ausgibt. Ihr Doc-Kommentar traegt den Aufruf, verlangt `--release` mit Begruendung und sagt, warum `#[ignore]` daran steht: sie hat keine Zusage, an der sie scheitern koennte. Der Modulkopf verweist jetzt auf sie und nennt den Lauf vom 260908 neben denen vom 260810; er zieht die alten Zahlen ausdruecklich nicht nach, weil sie die jenes Laufs sind. Gemessen am 260908, `--release`, `anwendung.rs` (546 429 Bytes, 10 505 Zeilen): voller Durchgang 1,897 s (0,29 MB/s), ein Anschlag in der Mitte 8,7 ms, 329 Haltepunkte; beim Vierfachen 7,795 s (0,28 MB/s), 6,3 ms, 1 314 Haltepunkte. Die 0,3 MB/s des Modulkopfs halten damit. Nicht beantwortet ist die zweite Frage des Datensatzes, ob die Messung nach `krk-bench` gehoert; der Vorschlag stellt sie ausdruecklich zurueck. Geprueft: cargo test -p krk-ui Exit 0.
