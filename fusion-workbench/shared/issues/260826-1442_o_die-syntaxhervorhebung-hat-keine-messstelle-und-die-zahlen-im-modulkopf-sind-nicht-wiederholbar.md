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
