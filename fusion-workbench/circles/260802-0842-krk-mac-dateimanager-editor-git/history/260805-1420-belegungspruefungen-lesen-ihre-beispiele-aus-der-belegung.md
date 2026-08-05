# Die Belegungsprüfungen lesen ihre Beispiele aus der Belegung

Status: Complete
Agent: coder
Datum: 260805-1420
Circle: 260802-0842-krk-mac-dateimanager-editor-git
Auftrag: `issues/260805-1356_c_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md`

## Was der Defekt war

`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` führte eine Tabelle
hingeschriebener Paare aus Kombination und Kommando. `oeffnen` ist innerhalb von zwei
Tagen dreimal gewandert, von `return` über `cmd+right` auf `right`, und jedes Mal ist
genau diese Prüfung gebrochen, ohne dass an ihrer Zusage etwas kaputt war. Zweimal hat
jemand die Zeile nachgezogen; der `ontocoder` hat beim dritten Mal die Frage gestellt, ob
die Prüfung ihr Beispiel nicht aus der Belegung lesen sollte, statt es zu wiederholen.

Die Antwort ist ja. Die Zusage der Prüfung braucht die Kombination gar nicht zu kennen:
sie lautet, dass es zu einem gebauten Kommando **eine** ausgelieferte Kombination gibt
und dass der Nachschlag darauf dieses Kommando trifft. Welche es ist, sagt
`resources/default-keymap.toml`. Eine Wiederholung in der Prüfung ist eine zweite
Wahrheit darüber, welche Taste was auslöst, und der Modulkopf von `tasten::belegung`
verbietet genau das für den Produktivpfad seit Schritt 11.

## Was geändert ist

Alles in `crates/krk-core/tests/belegung.rs`. `crates/krk-core/src/tasten/belegung.rs`
brauchte keinen Eingriff: `Kommando::KENNUNGEN` und `Belegung::funktion` reichten aus.

### Die genannte Prüfung

`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` läuft jetzt über
`Kommando::KENNUNGEN`, holt zu jeder Kennung die Funktion aus der Auslieferungsbelegung,
verlangt mindestens eine Kombination und prüft für **jede** ihrer Kombinationen, dass der
Nachschlag darauf dieses Kommando liefert. Gemessen wird damit an 42 gebauten Kommandos
statt an fünf: mehr Zusage als vorher, nicht weniger, und sie wächst mit `Kommando` mit.

Die vier übrigen Zeilen der alten Tabelle (`up`, `down`, `pageup`, `pagedown`) sind mit
derselben Änderung verschwunden.

### Fünf weitere Prüfungen derselben Bauart

Gefunden über eine Probe: `kopieren` in der Kopie von `f5`, `shift+cmd+k` auf
`ctrl+shift+k` umgelegt. Drei Prüfungen fielen um, die von der Umbelegung gar nicht
handeln, zwei weitere hingen an derselben Sorte Literal.

| Prüfung | Was sie hinschrieb | Was sie jetzt tut |
|---|---|---|
| `beide_ausgelieferten_wege_treffen_dieselbe_funktion` | sechs Zeilen aus Funktionstaste, Cmd-Kürzel, Kennung | sucht die Funktionen mit mehreren Wegen selbst und misst an allen |
| `eine_bereits_vergebene_kombination_liefert_einen_konflikt_...` | `shift+cmd+k` und den Namen des Kopierens | `ausgeliefert("kopieren")`, Name aus der Belegung |
| `dieselbe_kombination_zweimal_an_dieselbe_funktion_aendert_nichts` | `f5` | `ausgeliefert("kopieren")` |
| `zuruecksetzen_stellt_die_eingebettete_tabelle_wieder_her` | `f5` und `ctrl+k` | `ausgeliefert("kopieren")` und `frei()` |
| `die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht` | `f5` | jeden ausgelieferten Weg des Kopierens |
| `eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt` | `ctrl+k` | `frei()` |

### Zwei Helfer

```
resources/default-keymap.toml
        │
        ├── ausgeliefert(kennung) ──> eine Kombination, die diese Funktion trägt
        └── frei()                ──> eine Kombination mit Zusatztaste, die keine trägt
```

`frei()` sucht wie `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke`
über `parser::TASTEN` und die fünfzehn Masken und nimmt die erste unvergebene. Der Grund
steht im Datensatz zu `cmd+q` vom 260805-0820: jede hingeschriebene freie Kombination
kann eines Tages belegt werden, und dann fällt eine Prüfung um, die von der Belegung
nicht handelt.

`kombi(text)` bleibt, mit einer Doku, die seinen Einsatzbereich einengt: nur noch für
Kombinationen, an denen die Zusage selbst hängt, etwa die ab Werk freie Eingabetaste.

### Ein stehengebliebener Kommentar

Der Kommentar in `die_ab_werk_freien_kombinationen_kommen_nicht_vor` begründete die freie
Eingabetaste mit "nachdem der Einstieg in den Ordner auf cmd+right gewandert ist". Er
nennt jetzt nur noch, dass der Einstieg **von der Eingabetaste weg** ist, und sagt dazu,
warum das Ziel dort bewusst nicht steht.

## Was bewusst stehenbleibt

`cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt` und
`die_umbelegung_vergleicht_den_zusteller_ebenso` nennen `cmd+a` und `cmd+x`. Sie handeln
vom einzigen ausgelieferten Fall zweier Zusteller auf einer Kombination, und die erste
prüft ihre eigene Voraussetzung mit der Meldung "die Pruefung misst dann nichts". Wandert
`cmd+a`, brechen sie sichtbar und mit einem Satz, der sagt, was zu tun ist. Eine
Umstellung würde ändern, was sie dokumentieren, und ist deshalb nicht Teil dieses Falls.

`der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege` nennt `cmd+x`, `cmd+c`,
`cmd+v`. Dort sind die Kombinationen die Zusage: die Textbefehle des Menüs lösen im
Dateifenster nichts aus. Bekäme eine davon eine Funktion des Dateifensters, wäre der
Fehlschlag echt.

## Nachweis

`resources/default-keymap.toml` ist nicht angefasst. SHA-256 vor der Arbeit und nach ihr:
`4285656823b6722848a38a3503b9f01f43cbafa5ecbef9e832a9ef28d358f064`. `diff` gegen die vor
Beginn gezogene Sicherung ist leer. Die Proben liefen an einer Kopie des Baums unter dem
Temporärverzeichnis, die danach gelöscht wurde.

Drei Proben an der Kopie, mit der fertigen Fassung der Prüfdatei:

| Probe | Ergebnis |
|---|---|
| `oeffnen` um `cmd+right` ergänzt: `tasten = ["right", "cmd+right"]` | 32 von 32 |
| `oeffnen` auf `ctrl+o`, `kopieren` auf `ctrl+shift+k` | 32 von 32 |
| `oeffnen` ganz ohne Kombination: `tasten = []` | 2 Fehlschläge, Meldung "Oeffnen ist gebaut, und oeffnen traegt ab Werk keine Kombination" |

Die dritte Probe ist die Gegenprobe: ohne sie wäre nicht gezeigt, dass die Prüfung
überhaupt noch etwas misst. Dieselbe zweite Probe hat vor der Erweiterung auf die fünf
Nachbarprüfungen drei Fehlschläge erzeugt; danach keinen.

Die vier Abnahmekommandos, alle mit Rückgabewert 0:

```
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

`cargo test -p krk-core --test belegung` meldet 32 von 32. Der Lauf über den ganzen
Arbeitsbereich meldet 13 Testprogramme ohne Fehlschlag.

## Geänderte Dateien

- `crates/krk-core/tests/belegung.rs` (177 Zeilen hinzu, 50 fort)
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-1356_c_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md` (Resolved-Notiz, Marker `_o_` → `_c_`)

Nicht committet, wie beauftragt.
