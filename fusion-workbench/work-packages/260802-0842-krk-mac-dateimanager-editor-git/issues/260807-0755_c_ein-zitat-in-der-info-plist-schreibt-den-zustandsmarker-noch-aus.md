Ein Zitat in der `Info.plist` schreibt den Zustandsmarker noch aus

---

`resources/Info.plist:116` zitiert einen Workbench-Pfad mit ausgeschriebenem
Zustandsmarker `_c_` statt der Sternform:

```
`issues/260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
```

Es ist das **letzte** Zitat dieser Form im gesamten Programm- und Auslieferungsbestand.
Es steht in derselben Datei, die Turn 25 auf die Sternform umgestellt hat, zwölf Zeilen
unter einem Zitat in der neuen Form (`resources/Info.plist:28`).

---

**Gezählt am 260807-0753** über
`grep -rnE '[0-9]{6}-[0-9]{4}_[oapcidsbt]_' --include='*.rs' crates xtask` und dieselbe
Suche über `--include='*.toml'`:

| Bestand | Sternform | ausgeschriebener Marker |
|---|---|---|
| `crates/` und `xtask/`, `.rs` | 80 | 0 |
| `.toml` im Projekt | 15 | 0 |
| `resources/Info.plist` | 1 (Zeile 28) | **1 (Zeile 116)** |

**Das Ziel steht heute richtig.** `260805-0753_c_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
liegt unter `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/` und trägt
tatsächlich `_c_`. Der Defekt ist keine tote Referenz, sondern eine Referenz, die beim
nächsten Markerwechsel des Ziels stillschweigend falsch wird — genau der Fall, gegen
den die Sternform eingeführt wurde. Zehn der 13 Zitate, die Turn 25 umgestellt hat,
waren beim Anfassen bereits auf diese Weise veraltet.

**Warum es durchgerutscht ist.** Die Aufgabe D8 hatte zwei getrennte Quellen: der
Defekt `260806-1320_*_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`
nennt in seinem Titel und seinem Umfang allein die beiden `resources/default-*.toml`.
Die `Info.plist` kam über den anderen Defekt derselben Aufgabe in den Umfang, und dort
ging es um `CFBundleLocalizations`. Kein Auftrag hat die `Info.plist` je auf ihre
Pfadzitate durchgesehen.

---

**Was zu tun ist.** `resources/Info.plist:116` auf
`260805-0753_*_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
umstellen. Ein Handgriff, eine Zeile, kein Wertfeld berührt: die Zeile steht in einem
XML-Kommentar.

**Ausführender:** `ontocoder`. `resources/Info.plist` ist eine Bündelbeschreibung,
keine Programmdatei.

**Dringlichkeit.** Mittel. Nichts bricht, und der Nutzer sieht die Zeile nie. Der Wert
liegt darin, dass die Regel danach ausnahmslos gilt: wer die Sternform in einem neuen
Zitat weglässt, hat kein Vorbild mehr im Bestand, auf das er sich berufen könnte.

**Nicht mitgemeint sind `CLAUDE.md` und die Messberichte.** `CLAUDE.md` führt sechs
Zitate mit ausgeschriebenem Marker, `messungen/` vier. Für beide gilt der Grund der
Sternform nicht in derselben Weise: `CLAUDE.md` wird bei jedem Abgleich ohnehin
angefasst, und ein Messbericht ist ein datierter Befund, der den Stand seines Tages
festhalten darf. Ob die Regel dorthin ausgedehnt wird, ist eine eigene Frage und hier
nicht gestellt.

**Aufgefallen bei:** der ontologischen Durchsicht der Datenänderungen nach Turn 25 der
Sitzung 260806-2257 (Commit `880cb70`, Aufgabe D8).

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1320_c_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`

---
Resolved: `resources/Info.plist:151` trägt jetzt
`issues/260805-0753_*_die-beiden-info-plist-schluessel-gegen-die-systemeintraege-greifen-nicht.md`
in Sternform. Eine Zeile in einem XML-Kommentar, kein Wertfeld berührt. Gegen den
Dateibestand aufgelöst: alle vier Pfadzitate der Datei, die beiden alten und die
beiden am selben Tag neu entstandenen, treffen über eine Auflösung mit dem Marker als
Einzelzeichen-Platzhalter auf je genau eine Datei, keines auf zwei und keines auf
keine. Damit gilt die Sternform im Programm- und Auslieferungsbestand ausnahmslos:
80 Zitate in `.rs`, 15 in `.toml`, 4 in `resources/Info.plist`, kein einziges mit
ausgeschriebenem Marker.
`plutil -lint` und `xmllint --noout` gültig, `__KRK_VERSION__` unberührt,
`make check` grün.
Bericht: `history/260807-0952-ontocoder-entwicklungsregion-auswahlregel-und-das-letzte-pfadzitat.md`.
