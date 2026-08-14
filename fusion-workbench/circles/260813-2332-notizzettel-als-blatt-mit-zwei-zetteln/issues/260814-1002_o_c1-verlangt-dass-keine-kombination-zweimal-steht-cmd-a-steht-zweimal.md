C1 verlangt, dass keine Kombination zweimal steht; cmd+a steht zweimal

---

Das zweite Kriterium der ersten Liste von C1 sagt: „Keine der 82 bestehenden Funktionen
verliert eine Kombination, und keine Kombination steht danach zweimal."

Die erste Hälfte hält: der Unterschied an `resources/default-keymap.toml` gegen `6d05bef`
besteht allein aus Zufügungen — ein `[[funktion]]`-Block und die Kopfzahlen von 82/88 auf
83/90. Keine bestehende Zeile ist angefasst.

Die zweite Hälfte ist so, wie sie dasteht, nicht einlösbar. `cmd+a` steht in der Datei zweimal,
an `:295` (`alle_markieren`) und an `:874`. Die Doppelung ist älter als diese Runde — sie steht
am Stand `6d05bef` genauso — und die Datei erklärt sie an drei Stellen ausdrücklich als keinen
Konflikt (`:114`, `:864`, `:866`): die beiden Funktionen tragen verschiedene Wirkungsbereiche,
und die Belegung lässt sie einander nie begegnen.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Nachgezählt am 260814-1002 über alle
`tasten`-Listen der Datei: 90 Kombinationen, davon 89 verschiedene, und die eine Doppelung ist
`cmd+a`.

**Was das Kriterium meint und was es sagt, gehen auseinander.** Gemeint ist offensichtlich, dass
**diese Runde** keine Doppelung anlegt, und das hält: `f2` und `cmd+k` waren am 260814 frei.
Gesagt ist eine Aussage über den ganzen Dateibestand, und die ist falsch — mit dem einen Fall,
den die Datei selbst als gewollt ausschreibt.

**Warum es aufgeschrieben ist und nicht nur abgehakt.** Am Rundenabschluss wird jedes Kriterium
der ersten Liste einzeln gelesen. Wer dieses liest und nachzählt, findet die Doppelung, und
dann steht er vor der Wahl, das Kriterium für gebrochen zu erklären oder es stillschweigend
weiter zu lesen als „diese Runde legt keine an". Beides ist schlechter als ein Kriterium, das
sagt, was es prüft. Genau diese Sorte Unschärfe hat in dieser Runde schon einmal einen hohen
Befund getragen: C4 sagte zwei Dinge zu, die nur gemeinsam hielten, und der Bau hat den
Widerspruch stillschweigend aufgelöst.

**Was zu tun ist.** Die zweite Hälfte auf die Runde beziehen, etwa: „und die zwei neuen
Kombinationen stehen an keiner anderen Funktion" — samt einem Wort zu der einen bestehenden,
gewollten Doppelung, damit der nächste Zähler nicht denselben Weg geht.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Gemessen mit `grep -o 'tasten = \[[^]]*\]' resources/default-keymap.toml` und einem
  Häufigkeitszähler über die einzelnen Kombinationen, am Stand `79dab20`.
