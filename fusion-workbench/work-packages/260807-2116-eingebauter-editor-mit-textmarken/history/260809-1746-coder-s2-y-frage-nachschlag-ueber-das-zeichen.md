# S2: Buchstaben und Ziffern über das gemeldete Zeichen nachschlagen

**Status:** Complete
**Ausführender:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Planschritt:** S2, `planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`, Phase A, Abschnitt 2
**Bindende Grundlage:** `decisions/260808-0140_i_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`, Weg 3, entschieden vom Nutzer am 260808-0155

---

## Was umgesetzt ist

Der Ereignisabgriff schlägt Buchstaben und Ziffern seither über das **gemeldete
Zeichen** nach, Funktionstasten, den Pfeilblock und die Steuertasten weiter über
den **virtuellen Tastencode**.

Der Zuschnitt steht an einer Stelle und nicht an mehreren. `Tastenkennung` in
`crates/krk-core/src/tasten/parser.rs` trägt die beiden Arten als Aufzählung;
beide Seiten des Vergleichs leiten ihre Kennung aus derselben Regel ab, die
Belegung über `Taste::kennung`, der Tastendruck über `Tastendruck::kennung`.
`Belegung::nachschlag` vergleicht deshalb Maske und Kennung und trägt keinen
Zweig je Tastensorte.

Die Regel selbst ist eine: ein einbuchstabiger Name aus einem
ASCII-Kleinbuchstaben oder einer ASCII-Ziffer ist sein eigenes Zeichen, jeder
andere Name benennt eine Stelle. Es ist dieselbe Regel, nach der
`zeichen_der_taste` in `crates/krk-ui/src/appkit/menue.rs` seit S13b das
Menükürzel bildet.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/tasten/parser.rs` | `Tastenkennung`, `Taste::kennung`, `Taste::zeichen`, `zeichen_der_stelle`, `zeichen_als_kennung`, `taste_mit_zeichen`; `Kombination::aus_tastendruck` sucht über die Kennung; `Kombination::tastendruck` trägt beides; Modulkopf mit der Begründung; fünf neue Proben |
| `crates/krk-core/src/tasten/mod.rs` | `Tastendruck` trägt `zeichen` neben `code`; `aus_ereignis` nimmt das gemeldete Zeichen; `kennung()`; Modulkopf |
| `crates/krk-core/src/tasten/belegung.rs` | `Belegung::nachschlag` vergleicht Maske und Kennung; die Begründung, warum die zweite Nachschlagart hier keine Sonderregel ist; Modulkopf |
| `crates/krk-ui/src/appkit/ereignisse.rs` | `gemeldetes_zeichen` (`charactersByApplyingModifiers:` mit leerer Maske, zweiter Weg über `charactersIgnoringModifiers`), `erstes_zeichen`, `zeichen_des_ereignisses`; `behandeln` reicht das Zeichen durch; `funktion_senden` schickt es mit; `--tasten-protokoll` gibt es aus; Modulkopf; eine neue Probe |
| `crates/krk-core/tests/belegung.rs` | Die y-Probe umgeschrieben, zwei neue Proben, `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen` mit richtiggestellter Begründung |
| `crates/krk-core/tests/tasten.rs` | Vier Aufrufstellen von `aus_ereignis` |
| `crates/krk-ui/src/belegungsmodell.rs` | Zwei Aufrufstellen von `aus_ereignis` in den Proben |

`normalisierung.rs` blieb unberührt: die Zusatztasten sind von der Frage nicht
betroffen, und der Plan nannte die Datei mit dem Vorbehalt "falls nötig".

## Was das für den Nutzer ändert

Auf einer deutschen Tastatur, nachgestellt in
`auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau`:

| Der Nutzer drückt | Code | Zeichen | Ergebnis |
|---|---|---|---|
| ⌘ + Aufschrift **Y** | 6 | `y` | Die Vorschau klappt auf oder zu |
| ⇧⌘ + Aufschrift **Y** | 6 | `y` | Der Fokus springt in die Vorschau |
| ⌘ + Aufschrift **Z** | 16 | `z` | Das Hauptmenü führt das Rückgängig aus |

Auf einer amerikanischen Tastatur bleibt alles, wie es war: dort melden dieselben
Stellen dieselben Zeichen.

## Der Beleg: Zeichen für Buchstaben, Code für Funktionstasten

Vier Proben, und zwei davon sind gegen eine Mutation geprüft, damit sie nicht
leer bestehen.

- `auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau` stellt die
  drei Tastendrücke der Tabelle oben nach. **Mutation:** `Belegung::nachschlag`
  auf `taste().code == druck.code` zurückgestellt → rot; die beiden anderen
  Proben bleiben grün.
- `eine_funktionstaste_wird_weiter_ueber_ihren_code_gefunden` prüft F3 in drei
  Formen: ohne Zeichen, mit `NSF3FunctionKey`, mit gesetztem function-Bit.
  **Mutation:** `zeichen_als_kennung` auf "jedes Zeichen taugt" gestellt → rot;
  die y-Probe bleibt grün. Die beiden Mutationen treffen also genau die eine
  Hälfte, die sie treffen sollen.
- `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` misst
  die Zusage an **jeder** ausgelieferten Kombination statt an Beispielen und
  besteht nur, wenn beide Nachschlagarten vorkommen.
- `jede_taste_traegt_genau_eine_kennung_und_keine_zwei_dieselbe` hält die
  Fallunterscheidung verschieden und vollständig. Die Eindeutigkeit ist die
  Voraussetzung dafür, dass die Konflikterkennung aus C3 weiter trägt: zwei
  Tasten mit derselben Kennung wären zwei Funktionen auf einem Tastendruck, die
  sie nicht sieht.

Dazu `ein_gesendetes_zeichen_findet_seine_funktion_wieder` in `ereignisse.rs`:
der Rundlauf der Messstrecke ohne AppKit, von der Kombination über die Angaben
des synthetischen Ereignisses zurück in den Nachschlag.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 15 Testprogramme grün |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |

`cargo doc --workspace --no-deps` meldet keine neue Warnung; die vorhandenen
sind älter als dieser Schritt.

## Zwei Entscheidungen im Schritt, mit ihrer Begründung

**Die beiden Nachschlagarten sind verschieden und vollständig, und das ist
tragend und nicht schmückend.** Ein Tastendruck ohne brauchbares Zeichen darf
nicht über seinen Code bei einer Taste landen, die über ihr Zeichen
nachgeschlagen wird. Der Fall ist konstruierbar: auf einer französischen
Tastatur meldet die Stelle `kVK_ANSI_Semicolon` ein `m`, und die Stelle
`kVK_ANSI_M` meldet ein Komma. Ohne die Trennung träfen beide dieselbe Funktion,
und die Konflikterkennung sähe es nie. `Kombination::aus_tastendruck` trägt
dieselbe Trennung, damit die Belegungsansicht aufschreibt, was auf der Taste
steht: wer auf einer deutschen Tastatur die Taste mit der Aufschrift Y drückt,
bekommt `y` in seine `keymap.toml` und nicht `z`.

**Zwei Zeichen aus einem Ereignis, weil es zwei Fragen sind.** Der Nachschlag
fragt "welche Taste wurde gedrückt" und nimmt `charactersByApplyingModifiers:`
mit leerer Maske; damit fällt auch die Umschalttaste weg, und `shift+cmd+1`
meldet die `1` und nicht das Ausrufezeichen darüber. Die Sprungmarke aus C2
fragt "welches Zeichen hat der Nutzer getippt" und nimmt weiter `characters`,
samt Großschreibung. Ein gemeinsames Zeichen für beides wäre für eine der beiden
Fragen die falsche Antwort.

## Was nicht gemessen ist

- **Die Wirkung am laufenden Bündel.** Die Tastaturbelegung des Geräts geht ein,
  und keine Probe stellt sie nach; das Abnahmekriterium von S2 sagt es selbst.
  Zu drücken sind ⌘ und die Taste mit der Aufschrift Y (die Vorschau muss auf-
  und zuklappen) und ⌘ und die Taste mit der Aufschrift Z (das Rückgängig des
  Editors muss greifen). `make tasten` gibt zu jedem Druck `tastencode=` und
  `zeichen=` aus; dass die beiden auseinanderlaufen, ist der Beleg.
- **Was `charactersByApplyingModifiers:` an einem selbst gebauten Ereignis
  antwortet.** Ein `NSEvent` lässt sich in einer Probe nicht bauen: AppKit
  braucht dafür den Hauptfaden und eine laufende Ereignisschleife, und der
  Versuch hält den Testlauf an, gemessen am 260809. Deshalb steht in
  `gemeldetes_zeichen` der zweite Weg über `charactersIgnoringModifiers`, der
  genau die Zeichenkette zurückliest, die das Ereignis mitbekommen hat. Er trägt
  beide Ausgänge, statt einen anzunehmen.

## Erledigt und gefunden

**Geschlossen:** `issues/260809-1642_c_auf-einer-deutschen-tastatur-schluckt-cmd-y-das-rueckgaengig-des-editors.md`,
mit Abschlussnotiz und der Tabelle des neuen Verhaltens. Der Vorschlag dieses
Defekts war "S2 vorziehen"; genau das ist geschehen, ohne eine Änderung an
`resources/default-keymap.toml`.

**Auf `_i_` gesetzt:** `decisions/260808-0140_i_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`.

**Neu gemeldet:** `issues/260809-1746_o_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md`.
Die Probe `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen` und
die beiden Planstellen, aus denen sie stammt, haben mit diesem Schritt ihren
Grund verloren: seit S2 wandert keine Stelle mehr. Der Schritt hat die Probe
**stehen lassen** und ihren Doc-Kommentar auf den heutigen Stand gezogen. Sie
wegzunehmen hieße, die offene Nutzerentscheidung aus
`issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`
vorwegzunehmen, die dieselbe Planstelle betrifft.

**Was C8 des Specs angeht:** die ersten beiden Abnahmekriterien sind mit Weg 3
in ihrer heutigen Fassung erfüllbar und brauchen die Umschreibung nicht, die S41
für den Fall von Weg 1 vorgesehen hatte. Das Ankreuzen bleibt `Nutzerarbeit`.

## Nicht angefasst

`crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/editor.rs`
und `crates/krk-ui/src/editormodell.rs` sind für parallel laufende Schritte
reserviert und in diesem Schritt nicht berührt. `resources/default-keymap.toml`
ebenfalls nicht: Weg 3 kommt ohne eine Änderung an der Belegung aus.
