# coder — drei Defekte in `editormodell.rs` und `anwendung.rs`

**Status:** Complete
**Agent:** coder
**Begonnen:** 260810-1015
**Abgeschlossen:** 260810-1032
**Dateigrenze:** `crates/krk-ui/src/editormodell.rs`, `crates/krk-ui/src/appkit/anwendung.rs` —
`appkit/editor.rs`, `krk-core/**`, `resources/**` und das Plandokument sind nicht
angefasst.

## Auftrag

Drei offene Datensätze des aktiven Circles beheben:

1. `260810-0212_o_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`
2. `260810-0418_o_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`
3. `260810-0423_o_der-rueckgabewert-von-bearbeiten-laesst-sich-still-fallenlassen.md`

## Was umgesetzt wurde

### Defekt 1 — die Stücke ohne Aufrufer: gestrichen, alle vier

Die Frage war, **ob** sie gebraucht werden, nicht wie sie am Leben zu halten
sind. Geprüft wurde je Stück, ob eine Stelle der Oberfläche es rufen müsste;
bei keinem gibt es sie. Entfernt aus `editormodell.rs`:

- `Suchlauf::treffer`
- `Editormodell::stempel`
- `Editormodell::haelt_zurueck`
- `Editormodell::suche_beenden`

Der Nachtrag des Datensatzes zählt vier statt der drei im Titel; die vierte
(`stempel`) ist mitgegangen, weil sonst eine Ausnahme ohne Ablaufdatum
stehenbliebe und der Datensatz nur halb geschlossen wäre. Die Datei trägt danach
**kein** `#[allow(dead_code)]` mehr, weder am Kopf noch einzeln.

Die zehn Zusicherungen, die an den vier hingen, sind umgestellt, nicht gefallen:
sechs waren mit einer Nachbarzeile deckungsgleich (an ihrer Stelle steht jetzt
ein Kommentar, der sagt, welche Zeile die Aussage trägt), vier fragen über
`zurueckgehaltenes_uebernehmen() == None`, also über den Weg statt über ein Feld.
Der Modulkopf führt die vier weiter namentlich, im Perfekt.

### Defekt 2 — die Marke der Herkunft

Neue Aufzählung `Oeffnungsherkunft { Befehl, Sitzung }` in `anwendung.rs` und
eine neue Stelle `Anwendungsdelegierter::editor_oeffnen_lassen`, die sie als
**Pflichtargument** nimmt. Alle vier Öffnungswege gehen jetzt durch sie
(`im_editor_oeffnen`, `editor_aus_vorschau`, `textmarke_anspringen`,
`editor_wiederherstellen`); keiner ruft `Editorbereich::datei_oeffnen` noch
selbst, und keiner fasst die `Cell` an.

`editor_aus_sitzung` bezeichnet damit **das zuletzt begonnene** Öffnen. Das ist
die richtige Auskunft, weil höchstens dieses einen Ladeausgang liefert:
`Editormodell::oeffnen` ersetzt den laufenden Ladevorgang, sein Empfänger fällt,
und das `send` des überholten Fadens scheitert still. Der billigere Weg — die
Marke an den drei Befehlswegen löschen — ist nicht gegangen; es gibt nichts zu
löschen, weil kein Weg ohne Angabe der Herkunft übersetzt.

Der Gegenentwurf „Marke als Pfad, Vergleich gegen den gehaltenen Bestand" ist
geprüft und verworfen: `Ladeausgang::Abgewiesen` fasst den Stand nicht an, also
nennt das Modell dort die vorige Datei, und jede Abweisung aus der Sitzung
verlöre ihren Zweig. Der Pfad steckt in `Abweisung`, hat dort aber keinen
Zugriff, und ein `match` daneben wäre eine zweite vollständige
Fallunterscheidung.

### Defekt 3 — `#[must_use]` an `bearbeiten`

`#[must_use = "wandelte das Bearbeiten, ist die Textflaeche nachzuziehen"]`. Die
siebzehn Aufrufe unter `mod tests`, die den Wert nicht lesen, stehen als
`let _ = …`. `appkit/editor.rs` musste nicht angefasst werden: seine beiden
Aufrufe lesen den Wert bereits.

## Neu gefundene Defekte

- `issues/260810-1028_o_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`
  — die Erzwingung aus Defekt 2 endet an der Grenze des Delegierten. Der Weg
  dahin (`datei_oeffnen` nimmt die Herkunft, der `Ausgangsmelder` gibt sie
  zurück) liegt in `appkit/editor.rs`, außerhalb der Dateigrenze.
- `issues/260810-1029_o_die-abkuerzung-fuer-die-gehaltene-datei-bricht-das-laufende-lesen-nicht-ab.md`
  — `Editormodell::oeffnen` liefert `SchonOffen`, ohne den laufenden Ladevorgang
  aufzugeben; damit gehören zwei Ladeausgänge zu einer Folge von Öffnungen, und
  der letzte Befehl des Nutzers wird still überschrieben. Ohne Folgen für Defekt 2.

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt --all --check` | exit 0 |

Formatiert wurde mit `cargo fmt -p krk-ui`, wie der Auftrag es verlangte.

## Was offen bleibt

Die Marker der drei behobenen Datensätze benennt der Nutzer um; der
`Resolved:`-Abschnitt steht in jedem. Die beiden neuen Datensätze stehen auf
`_o_`.
