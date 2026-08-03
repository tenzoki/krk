Der Modulkopf der Tastennormalisierung belegt drei Aussagen mit einer Messung, die sie nicht trägt

---

`crates/krk-core/src/tasten/normalisierung.rs:9-22` begründet die Löschung von
`function` und `numericPad` mit der Fn-Messung vom 260802-1137 und nennt
`spikes/fn-tasten/messung-A-neuauswertung.txt` als Beleg. Drei der dort
behaupteten Aussagen stehen in dieser Datei nicht.

---

## Die drei Stellen im Einzelnen

**Erstens: "Fn+F3 und ein nacktes F3 erzeugen dasselbe Ereignis."**
`normalisierung.rs:14-15`. Die Neuauswertung sagt zu genau dieser Frage:

> Frage 2 — Kommen die nackten F3 bis F8 an?
> NICHT MESSBAR AUF DIESEM GERÄT. In Abschnitt 2 kamen zwar 3
> Funktionstasten-keyDown an, aber alle bei gehaltener fn-Taste. […] Abschnitt 2
> wiederholt damit Abschnitt 1 und sagt nichts über die nackten F-Tasten.

(`spikes/fn-tasten/messung-A-neuauswertung.txt:69-72`)

Der Entscheidungsdatensatz `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`
hält denselben Stand fest: "Ob die nackten Funktionstasten auf einem Gerät mit
echter Tastenreihe ankommen, ist unverändert ungemessen."

Das ist zugleich genau der Fehler, den Commit `f865fca`
("fix(spike): Auswertung der F-Tasten-Messung berichtigt") aus der ursprünglichen
Selbstauswertung entfernt hat. Er kehrt hier eine Ebene höher zurück, im
Programmtext.

**Zweitens: "auch bei den Pfeiltasten."** `normalisierung.rs:13`. In
`messung-A.txt` kommt kein einziges Pfeiltasten-Ereignis vor. Das Rohprotokoll
umfasst 17 Ereignisse: `a`, `b`, `c`, F3, F5, F8, fn und Shift links
(`spikes/fn-tasten/messung-A.txt:15-31`). Eine Pfeiltaste wurde nie gedrückt.

**Drittens: "Der Zehnerblock ebenfalls, denn AppKit setzt sein Bit auch bei den
Pfeiltasten."** `normalisierung.rs:21-22`. Das Bit `NSEventModifierFlagNumericPad`
(0x200000) taucht in keinem der 17 gemessenen `roh=`-Werte auf. Die gemessenen
Werte sind ausschließlich `0x00000100`, `0x00800100` und `0x00020102`.

Derselbe Fehlschluss steht in der Prüfung: `crates/krk-core/tests/tasten.rs:24-26`
schreibt "Die Messung vom 260802-1137 zeigt F3 immer mit gesetztem `function`,
gleich ob der Nutzer fn gehalten hat oder nicht." Die Messung zeigt nur den Fall
mit gehaltenem fn.

## Was hier nicht steht

**Der Code ist richtig.** AppKit setzt `function` tatsächlich bei jeder Taste
aus dem Funktionstasten-Unicodebereich und `numericPad` tatsächlich bei den
Pfeiltasten. Die Löschung beider Bits ist die richtige Umsetzung von C3. Der
Defekt liegt nicht im Verhalten, sondern in der Belegkette: die drei Sätze
behaupten Messwissen, das das Projekt an anderer Stelle ausdrücklich als
ungemessen führt.

**Warum das jetzt zählt.** `normalisierung.rs` ist der einzige Teil des
Durchstichs mit echten Prüfungen und der Träger der C3-Abnahme. Ein Modulkopf,
der eine Messdatei nennt, ist die Stelle, an der ein späterer Leser die
Begründung nachschlägt statt sie neu zu prüfen. Der Fehler wird von dort weiter
kopiert; S11 (Belegungsmaschine) baut auf derselben Maske auf.

## Was zu tun ist

Drei Sätze umschreiben, keine Zeile Programmtext ändern:

1. Für `function` bei den F-Tasten bleibt die Messung der Beleg, aber nur für den
   gemessenen Fall: `spikes/fn-tasten/messung-A.txt:17-19` zeigt F3, F5 und F8
   mit gesetztem `function` bei gehaltener fn. Dass ein nacktes F3 dasselbe
   Ereignis liefert, ist als abgeleitete Annahme zu kennzeichnen, mit Verweis auf
   den Nachtrag im Entscheidungsdatensatz, statt als Messergebnis.
2. Für `numericPad` bei den Pfeiltasten die Messdatei als Beleg streichen. Der
   richtige Beleg ist die AppKit-Dokumentation, oder eine eigene kurze Messung.
   Die Sonde aus `history/260803-1309-tastenereignisse-und-pfeiltasten.md`
   taugt dafür nicht: sie hat die Bits selbst gesetzt.
3. Denselben Satz in `crates/krk-core/tests/tasten.rs:24-26` nachziehen.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
