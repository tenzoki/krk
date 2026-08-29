Der eingefügte Filtertext hat keine Höchstlänge, und eine lange Zeile macht jeden Rückschritt zum Gang über den Bestand mit langer Nadel
---
`filtertext_aus` (`crates/krk-core/src/zwischenablage.rs`) und `Ordnermodell::text_anhaengen` (`crates/krk-core/src/verzeichnis/modell.rs`) nehmen einen einzeiligen Text jeder Länge an; weder der Spec (A3, A4) noch der Code setzt eine Grenze. Ein Tastendruck bringt ein Zeichen, ein `cmd+v` kann eine Zeile von hunderten Kilobyte bringen (minifiziertes JSON, eine lange URL, eine Logzeile). Die Folgen: der Filtertext übersteht den Ordnerwechsel (A8) und steht ab dann in der Statuszeile (`Filter „…“`); jeder Rückschritt und jeder weitere Anschlag ruft `filter_uebernehmen`, das den ganzen Text kleinschreibt und die Sicht neu aufbaut; und `traegt_die_folge` ruft je Eintrag `str::find` mit dieser Nadel, dessen Vorbereitung (Two-Way) linear in der Nadellänge ist — bei 100.000 Einträgen und 100 KB Nadel Milliarden Byteschritte je Anschlag, auf dem Hauptfaden. Mit „Content“ läuft dieselbe Nadel über jede gelesene Datei. Nicht gemessen; die Größenordnung folgt aus der Bauart von `core::str::pattern::TwoWaySearcher::new`.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** Nutzer (Festlegung), dann coder

Ein Name im Dateisystem trägt höchstens 255 Bytes; ein Filtertext ohne `*`, der länger ist, kann keinen Namen mehr treffen, wohl aber einen Inhalt. Zwei Wege: eine Grenze in `filtertext_aus` mit einem fünften `Einfuegehindernis` (der `match` in `operationen::einfuegen_abgewiesen` hält den Bau an und verlangt den fünften Satz), oder eine ausdrückliche Festlegung „keine Grenze“ am Doc-Kommentar von `filtertext_aus` und `text_anhaengen`, damit die Lücke gewollt und nicht vergessen ist.

Abnahme: entweder eine Probe `filtertext_aus` gegen einen Text über der Grenze mit dem neuen Hindernis und seinem Satz, oder der Doc-Kommentar beider Funktionen nennt das Fehlen der Grenze und den Grund.
