Die beiden Auslieferungsdateien zitieren Workbench-Pfade mit Zustandsmarker

---

`resources/default-keymap.toml` und `resources/default-settings.toml` führen
14 Zitate von Workbench-Pfaden auf 13 Zeilen, jeweils mit einem
Zustandsmarker im Namen (`_o_`, `_c_`, `_a_` und so fort). Ein solcher Pfad
veraltet zwangsläufig, sobald der zitierte Eintrag seinen Zustand wechselt;
danach findet ihn niemand mehr über `ls`.

---

**Ausführender:** `ontocoder`. Beide Dateien sind Auslieferungsdaten, keine
Programmdateien.

**Die Auflösung ist bekannt und andernorts schon angewandt.** Der
Code-Bestand hat denselben Defekt am 260806 abgeräumt
(`issues/260806-1123_*_modulkopf-zitiert-den-issue-pfad-mit-ueberholtem-marker.md`,
Commit im Turn 23): 62 Zitate in 32 Dateien tragen jetzt die Platzhalterform
mit `*` an der Markerstelle, etwa
`issues/260806-1054_*_belegungsansicht-gruppiert-nach-funktionsbereich.md`.
Dieselbe Form gilt seit jeher in Spec und Plan, mit 209 Vorkommen. Für diese
beiden Dateien ist sie nachzuziehen.

**Warum ein eigener Eintrag.** Die Dateien gehören dem `ontocoder`, die
aufgeräumten Quelltexte dem `coder`; der Auftrag, unter dem die 62 Zitate
entstanden, war ausdrücklich auf Code-Dateien beschränkt. Eine Datenänderung
an der Auslieferungsbelegung geht außerdem über eine Kommentarkorrektur
hinaus: sie berührt eine Datei, gegen die das Abnahmekriterium von S20 die
zurückgesetzte Belegung byteweise vergleicht.

**Dringlichkeit.** Gering, reine Doku-Drift, kein Programmverhalten berührt.

**Aufgefallen bei:** dem Nachziehen der Markerzitate im Code, Turn 23 der
Sitzung 260806-1140.

---
Resolved: Alle Pfadzitate beider Dateien tragen jetzt die Sternform an der
Markerstelle (`resources/default-keymap.toml:8,11,12,61,88,403,438,475,505,524,535`
und `resources/default-settings.toml:9,11`). **Es sind 13 Zitate, nicht 14**; über
`grep -o` gezählt stehen elf in der Belegungsdatei und zwei in der
Einstellungsdatei, auf 13 Zeilen, ohne eine Zeile mit zwei Zitaten. Sechs der elf
stehen über zwei Zeilen verteilt, mit dem Verzeichnisteil am Zeilenende und dem
Dateinamen am Anfang der nächsten; das ist vermutlich die Quelle der Differenz.
**Zehn der 13 waren beim Anfassen bereits veraltet**: sieben Datensätze stehen
heute auf `_i_` statt `_a_` und einer auf `_c_` statt `_o_`. Richtig standen nur
noch das Spec-Zitat und `260805-0753_*_cmd-q-loest-etwas-aus-…`. Jedes der neun
zitierten Ziele existiert; kein Zitat zeigt ins Leere.
**Der byteweise Vergleich aus S20 trägt, weil es ihn nie gab.** Das
Abnahmekriterium vergleicht die zurückgesetzte Belegung in
`~/Library/Application Support/KRK/keymap.toml`, und die entsteht über
`Ablage::sichern` (`crates/krk-core/src/ablage/mod.rs:265-271`) aus
`toml::to_string`. Die Serialisierung kennt keine Kommentare, was der Modulkopf
an derselben Stelle (`:261-264`) ausdrücklich festhält; die Auslieferungsdatei
besteht großenteils aus Kommentaren, die geschriebene Datei aus keinem. Verglichen
wird die Belegung, nicht der Text, und `Belegung::zuruecksetzen`
(`crates/krk-core/src/tasten/belegung.rs:659-661`) baut sie über
`toml::from_str(AUSLIEFERUNGSTEXT)`. **Am Plan ist nichts nachzuziehen.**
`resources/default-settings.toml` wird wörtlich nach
`~/Library/Application Support/KRK/settings.toml` geschrieben
(`ablage/einstellungen.rs:177-183`); die zwei Zitate stehen künftig auch dort.
`make check` grün.
Bericht: `history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`.
