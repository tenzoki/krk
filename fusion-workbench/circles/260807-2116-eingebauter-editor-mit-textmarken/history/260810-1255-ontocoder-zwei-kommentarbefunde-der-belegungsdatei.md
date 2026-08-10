# Zwei Kommentarbefunde der Belegungsdatei behoben

**Agent:** ontocoder
**Am:** 260810-1255
**Status:** Complete
**Circle:** `circles/260807-2116-eingebauter-editor-mit-textmarken`
**Auftrag:** die zwei Defektdatensätze der Durchsicht `reviews/260810-1217-ontorev-belegungsdatei-nach-den-drei-kommentarstellen.md`, dazu die Absatzform-Kleinigkeit ohne eigenen Datensatz

---

## Was geändert ist

Eine Datei, `resources/default-keymap.toml`, drei Stellen, alle drei Kommentar:

| Stelle | Vorher | Nachher |
|--------|--------|---------|
| 479 | `Fuenf von ihnen teilen sich den Buchstaben e` | `Vier von ihnen …` |
| 26 | `… ein spaeterer Schritt erst baut, wie es "belegung_ansehen" unten ist.` | Begründung ohne Beispiel, dazu die Aussage, dass die Datei derzeit keinen Eintrag dieser Sorte trägt, und das alte Beispiel als historische Notiz mit Datum |
| 639 | Schlusssatz des Rückgängig-Blocks klebt am historischen Absatz | eine leere Kommentarzeile davor, der Schlusssatz gehört wieder dem ganzen Block |

Der Diff zählt drei Hunks, sechs eingefügte und zwei entfernte Zeilen. Jede berührte Zeile beginnt mit `#`.

## Beide Zahlen selbst nachgezählt, nicht übernommen

Der Auftrag verlangte das Nachzählen ausdrücklich, und beide Datensätze halten der Prüfung stand.

**Der Buchstabe `e`.** Über alle `tasten`-Listen der Datei tragen genau vier Kombinationen den Buchstaben `e`: `cmd+e` (504), `shift+cmd+e` (513), `opt+cmd+e` (521), `ctrl+cmd+e` (530). Die weiteren Treffer eines naiven `grep` auf `e"` sind `delete`, `space` und `home` und keine Taste `e`. Die vier Ebenen, die der Absatz nennt — blank, `shift+cmd`, `opt+cmd`, `ctrl+cmd` — gehen auf diese vier Funktionen ohne Rest auf. Eine fünfte Kombination auf `e` gibt es in der ganzen Datei nicht.

**Die Sorte ohne Kommando.** Über alle 71 Einträge gegen `Kommando::KENNUNGEN` (`crates/krk-core/src/tasten/belegung.rs:417 ff.`, deklariert als 65 Einträge und 65 gefunden):

| Sorte | Anzahl |
|-------|--------|
| trägt eine Kennung aus der Aufzählung | 65 |
| `gehalten_von = "menue"` | 6 |
| `reserviert_fuer` gesetzt | 0 |
| benannt, nicht zugestellt, ohne Kommando | 0 |

Die Menge der Einträge ohne Kennung ist **genau** die Menge der sechs vom Menü zugestellten, geprüft als Mengengleichheit und nicht nur als Zählstand. Die 65 Kennungen entsprechen den 65 Einträgen in beide Richtungen ohne Rest. `belegung_ansehen` trägt sein Kommando an den beiden Stellen, die der Datensatz nennt (`belegung.rs:409` und `:493`), und wird an einer dritten wieder aufgegriffen (`:544`). Ein Ersatzbeispiel für den Kopf gibt es in der Datei also nicht, und der Satz steht jetzt ohne eines.

Ein erster Zähllauf meldete 61 statt 65 Kennungen. Der Fehler lag im Muster und nicht in der Datei: vier Kennungen stehen in `belegung.rs` über zwei Zeilen umbrochen, und ein Muster, das Variante und Zeichenkette in einer Zeile erwartet, übersieht sie. Nachgezählt über den Tabellenkörper als Ganzes stimmen die 65.

## Was nicht angefasst ist

- **Keine Belegungszeile.** Die Datei ohne Kommentar- und Leerzeilen ist vor und nach der Änderung Byte für Byte dieselbe, 290 Nutzzeilen. Geprüft mit demselben Vergleich, den die Durchsicht in ihrem Abschnitt 1 führt.
- **Die zwei Zahlen im Dateikopf** (71 Funktionen, 79 Kombinationen, Zeile 33) stehen unverändert. Ein Agent schreibt parallel die Probe, die sie festnagelt.
- **Nichts unter `crates/**`.** Dort ist ausschließlich gelesen, um beide Datensätze gegen den Code zu prüfen.
- **Keine Sitzungsbuchhaltung.** Weder `activity-log`, noch `CLAUDE.md`, noch die Wurzel-`README.md`.

## Abnahme

| Kommando | Ergebnis |
|----------|----------|
| `cargo test -p krk-core --lib tasten` | exit 0, 21 Proben, keine gescheitert |
| `cargo test --workspace` | exit 0 |
| TOML-Form | die Datei liest sich fehlerfrei ein, 71 `[[funktion]]`-Einträge, 79 Kombinationen, keine doppelte `id`, kein Eintrag ohne `tasten` |

Der grüne Lauf des ganzen Arbeitsbereichs deckt einen Baum ab, in dem parallel drei Agenten unter `crates/` arbeiten. Er sagt, dass die Belegungsdatei den Bau nicht anhält, und nicht, dass der Baum eine Minute später noch derselbe ist.

## Ein Nebeneffekt für den Datensatz 260810-1219

Der neue Satz im Dateikopf behauptet den Bestand der Datei, und keine Probe hält ihn. Das ist dieselbe Lücke, die `issues/260810-1219_o_die-zwei-zahlen-im-kopf-der-belegungsdatei-wachsen-nicht-mit-ihr.md` für die Zahlen 71 und 79 führt. Ein eigener Datensatz daneben wäre eine Dublette; wer 260810-1219 baut, deckt die Stelle mit, wenn die Probe die Entsprechung zwischen `Kommando::KENNUNGEN` und den Einträgen der Datei in beide Richtungen prüft und nicht nur die zwei Zählstände.

## Datensätze

Beide behandelten Defekte tragen unten `Resolved:` mit dem, was getan und was nachgezählt wurde. Die Marker `_o_` sind stehen geblieben; der Nutzer benennt sie um.

- `issues/260810-1217_o_der-editor-abschnitt-der-belegung-zaehlt-fuenf-e-tasten-und-hat-vier.md`
- `issues/260810-1218_o_der-dateikopf-der-belegung-nennt-belegung-ansehen-als-funktion-ohne-kommando.md`
