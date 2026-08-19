Die beiden Meldungen des Terminal-Befehls sind als einzige ihres Moduls ungeprüft

---

`crates/krk-ui/src/kommandos/operationen.rs` ist das Modul der Statuszeilen-Sätze, und sein
Prüfmodul (`operationen.rs:760` und danach) fasst nahezu jede Meldung an: `abbruchzeile`,
`rueckfrage`, `anlegefehler`, `umbenennungsfehler`, `abschlusstext`, `datenmenge`,
`neuer_name` — 22 Proben, und mehrere davon prüfen den Wortlaut auf eine bestimmte Angabe
hin.

Zwei Funktionen desselben Moduls haben keine: `kein_terminal` (`operationen.rs:751`) und
`terminalordner_fehlt` (`operationen.rs:726`), beide aus C11.

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
(`shared/history/260810-1647-orchestrator-session.md`)
**Betroffen:** `crates/krk-ui/src/kommandos/operationen.rs`
**Domain:** code

## Warum das jetzt auffällt

`788c8d8` hat den Wortlaut von `kein_terminal` geändert. Das fünfte Abnahmekriterium von C11
(`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:476`)
verlangt, dass die Meldung **die eingestellte Kennung nennt**; sie tut es weiterhin, geprüft
am Programmtext. Gehalten wird diese Zusage aber von nichts als dem Abnahmelauf, und der ist
Nutzerarbeit und steht für die Runde 2 noch aus. Wer den Satz das nächste Mal umformuliert,
kann die Kennung herausnehmen, ohne dass `make check` etwas sagt.

Der Halbsatz zum Neustart, den dieser Commit angehängt hat, steht auf derselben Lage: er ist
die Antwort auf
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0930_c_die-meldung-zur-buendelkennung-sagt-nicht-dass-settings-toml-erst-beim-start-gelesen-wird.md`,
und ein stiller Verlust fiele erst im Abnahmelauf auf.

## Denkbarer Weg

Zwei Proben im vorhandenen `mod tests` desselben Moduls, in der Form der Nachbarn:

- `die_meldung_zur_buendelkennung_nennt_die_eingestellte_kennung` — `kein_terminal` mit einer
  erfundenen Kennung rufen und prüfen, dass sie in der Antwort vorkommt und der Neustart
  genannt ist. Das schreibt das fünfte Abnahmekriterium von C11 in eine Probe, statt es
  allein im Spec stehen zu lassen.
- `ein_fehlender_terminalordner_nennt_den_pfad` — `terminalordner_fehlt` gegen einen Pfad in
  einem Wegwerfordner, einmal auf eine Datei und einmal auf einen fehlenden Eintrag; die
  Funktion hat drei Zweige und keiner ist angefasst.

Beide kommen ohne AppKit aus, also ohne die Hauptfadenfrage aus
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`.

## Nicht geprüft

Ob der verlängerte Satz in der Statuszeile bei schmalem Fenster hinten abgeschnitten wird.
Die Zeile ist ein `NSTextField::labelWithString` über die volle Breite
(`crates/krk-ui/src/appkit/statuszeile.rs:144`), und in der Datei steht keine Angabe zum
Umbruch- oder Kürzungsverhalten. Der neue Halbsatz steht am **Ende** der Meldung und wäre
damit das erste, was verschwindet. Das ist eine Eigenschaft jeder langen Meldung dieses
Moduls und nicht von diesem Commit eingeführt; gemessen ist es nicht, und es gehört in den
Abnahmelauf, nicht in eine Behauptung hier.

## Dringlichkeit

Gering. Nichts ist heute kaputt. Der Wert liegt darin, ein Abnahmekriterium, das derzeit nur
ein Mensch prüfen kann, in `make check` zu ziehen.

---
Resolved: Zwei Proben in der Form der 22 Nachbarn, am Ende desselben `mod tests` in
`crates/krk-ui/src/kommandos/operationen.rs`:

- `die_meldung_zur_buendelkennung_nennt_die_eingestellte_kennung` prueft, dass die uebergebene
  Kennung im Text steht (das ist es, was das fuenfte Abnahmekriterium der Faehigkeit C11
  verlangt), dass `settings.toml` genannt ist und dass der Neustart-Halbsatz aus Turn 1 dasteht.
- `ein_fehlender_terminalordner_nennt_den_pfad` fasst alle drei Zweige von `terminalordner_fehlt`
  an, ueber `Pruefordner` aus `krk-ui/src/pruefordner.rs`, also ohne AppKit und ohne die
  Hauptfadenfrage.

Damit haengt das fuenfte Abnahmekriterium von C11 nicht mehr allein am ausstehenden Abnahmelauf.
Abgenommen mit `make check`, exit 0; beide Proben laufen gruen.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 2.
