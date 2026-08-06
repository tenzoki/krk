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
