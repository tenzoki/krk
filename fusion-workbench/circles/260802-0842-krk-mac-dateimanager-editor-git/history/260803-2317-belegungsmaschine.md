# Belegungsmaschine (Schritt 11)

**Status:** Complete
**Agent:** coder
**Zeitpunkt:** 260803-2317
**Plan:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 11.`
**Nicht committet.** Der `[DONE]`-Vermerk im Plan setzt der Nutzer.

---

## Was entstanden ist

| Datei | Was drinsteht |
|---|---|
| `crates/krk-core/src/tasten/parser.rs` | Die eine Tabelle der Tastencodes (53 Einträge, je mit Herkunft) und die Kombinationsschreibweise. |
| `crates/krk-core/src/tasten/belegung.rs` | Die Belegung: Einlesen, Nachschlag, Zuweisen, Zurücksetzen, Laden und Sichern. |
| `crates/krk-core/src/tasten/konflikt.rs` | Der Konflikt und die Benennung beider beteiligten Funktionen. |
| `crates/krk-core/tests/belegung.rs` | Die Abnahme, 26 Prüfungen. |

## Was geändert ist

| Datei | Was sich geändert hat |
|---|---|
| `crates/krk-core/src/tasten/mod.rs` | `VERDRAHTET`, `kommando()` und das Modul `code` sind fort. Übrig bleibt `Tastendruck`, dazu die drei neuen Module und die Ausfuhren. |
| `crates/krk-core/src/tasten/normalisierung.rs` | `ModMaske::BENANNT` trägt jetzt die Namen und die Reihenfolge der Belegungsdatei (`ctrl`, `opt`, `shift`, `cmd`) statt `command`, `control`, `option`, `shift`. |
| `crates/krk-core/tests/tasten.rs` | Die Prüfungen der verdrahteten Tabelle sind fort; die drei Tastencodes kommen über `code_von_pflicht` aus der Tabelle. |
| `crates/krk-ui/src/appkit/ereignisse.rs` | Der Abgriff hält seine eigene Belegung und schlägt in ihr nach. `CODE_PFEIL_AB` kommt aus der Tabelle. Der Protokollmodus nennt jetzt auch die Kombination und die Funktion. |

`resources/default-keymap.toml` ist unverändert.

## Die Ablösung der Tabelle aus Schritt 7

Die verdrahtete Tabelle ist nicht ergänzt, sondern gelöscht. `grep -rn "VERDRAHTET" crates/` und `grep -rn "tasten::kommando" crates/` finden nichts mehr. Der Weg vom Ereignis bis in das Ordnermodell läuft jetzt so:

```text
NSEvent ──> Tastendruck::aus_ereignis ──> Belegung::nachschlag ──> Funktion
                 (Maske normalisiert)                                 │
                                                        Funktion::kommando()
                                                                      │
                                              DateifensterQuelle::kommando_ausfuehren
```

**Der Abgriff hält die Belegung selbst.** `Tastenabgriff::einrichten` lädt sie beim Einrichten über `belegung::fuer_den_betrieb()`. Damit bleibt `anwendung.rs` unangetastet, so wie die Dateiliste des Schrittes es vorsieht: sie nennt unter `krk-ui` allein `ereignisse.rs`.

**Geschluckt wird nur, was auch ausgeführt wurde.** Die Belegung kennt 46 Funktionen, gebaut sind fünf. Eine Taste, die einer noch ungebauten Funktion gehört, geht unverändert weiter. Ohne diese Regel nähme der Abgriff dem Menü ab Schritt 11 das Cmd+W ab, ohne etwas an seine Stelle zu setzen: `tab_schliessen` liegt in der Belegung auf `cmd+w`, und der Menüeintrag "Fenster schließen" trägt dieselbe Kombination (`issues/260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen.md`, offen). Der Defekt bleibt offen; dieser Schritt hat ihn nur nicht verschlimmert.

## Warum `Kommando` fünf Fälle behält und nicht 46 bekommt

Der Wortschatz der Funktionen steht in `resources/default-keymap.toml` und nirgends sonst. Eine Aufzählung mit 46 Fällen daneben hätte die Kennungen ein zweites Mal geführt, und `DateifensterQuelle::kommando_ausfuehren` in `appkit/tabelle.rs` hätte einen Sammelzweig für 41 Fälle gebraucht, den kein Schritt dieser Runde füllt.

Stattdessen prüft die Belegung eine Nutzerdatei **gegen die Auslieferungsbelegung**: eine Kennung ist gültig, wenn die eingebettete Datei sie führt. `Kommando::KENNUNGEN` bleibt die Liste der fünf Funktionen, für die es in dieser Runde eine Ausführung gibt, und wächst mit den Schritten, die die übrigen bauen. Die Prüfung `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` hält beide Seiten zusammen: ein Tippfehler in einer der fünf Kennungen lässt sie scheitern, statt eine Taste still verstummen zu lassen.

## Die eine Codetabelle

`crates/krk-core/src/tasten/parser.rs` führt 53 Tasten. Belegt ist die Einzigkeit durch `grep -rn ": u16 = " crates/`: außerhalb dieser Datei steht keine Konstante mehr, die einen Tastencode als Zahl trägt.

Damit auch eine Konstante ihre Zahl von dort holen kann, ist `code_von` eine `const fn`. `crates/krk-ui/src/appkit/ereignisse.rs` schreibt

```rust
const CODE_PFEIL_AB: u16 = code_von_pflicht("down");
```

und ein Tippfehler im Namen bricht die Übersetzung ab. Der Defekt `issues/260803-2025_c_der-tastencode-von-pfeil-ab-steht-an-zwei-stellen.md` ist damit geschlossen.

## Gemessen und dokumentiert

Jeder Eintrag der Tabelle trägt eine `Herkunft` mit zwei Fällen:

- `Gemessen { kvk, beleg }` für F3, F5 und F8 (99, 96 und 100), mit `spikes/fn-tasten/messung-A.txt` als Beleg.
- `Dokumentiert { kvk }` für alle übrigen, darunter F4, F6 und F7 mit 118, 97 und 98. Quelle ist die Carbon-Tabelle `kVK_*` in `HIToolbox.framework/Headers/Events.h`.

Zwei Prüfungen halten das fest. `die_gemessenen_drei_sind_gemessen_und_die_dokumentierten_drei_dokumentiert` prüft beide Richtungen für die sechs Funktionstasten; `genau_die_drei_funktionstasten_der_messung_sind_gemessen` prüft, dass niemand einen vierten Eintrag als gemessen nachträgt. Eine dritte, `die_tastencodes_stimmen_mit_der_carbon_tabelle_ueberein`, hält 22 der Zahlen gegen die Carbon-Werte in Hexadezimalschreibweise; ohne sie prüfte nichts, ob `down` wirklich 125 ist.

## Die Nutzerbelegung

`keymap.toml` läuft über `Ablage::laden` aus Schritt 10; ein zweiter Ablageweg ist nicht entstanden. Der semantische Schritt liegt darüber: `belegung::laden` baut aus der gelesenen Datei eine Belegung und macht aus einem inhaltlichen Fehler dieselbe `Ersetzung`, die die Ablage für einen syntaktischen liefert. Jede Meldung geht über `ablage::melden`, auch die für ein fehlendes Benutzerverzeichnis.

`Belegungsdatei::default()` ist die eingebettete Auslieferungsbelegung. Damit liefert `Ablage::laden` bei fehlender oder kaputter Datei den Auslieferungszustand und nicht eine leere Belegung, in der keine Taste mehr etwas tut.

Fünf Fälle führen zum Auslieferungszustand mit Meldung, und jeder hat eine eigene Prüfung: kaputtes TOML, ein unbekanntes Feld (`taste` statt `tasten`), eine unbekannte Funktion, eine falsch geschriebene Kombination und ein Konflikt innerhalb der Nutzerdatei. Die Datei auf der Platte bleibt in jedem Fall liegen, wie Schritt 10 es hält.

**Eine Entscheidung, die der Plan offenließ.** Nennt die Nutzerdatei eine Funktion nicht, tritt sie unbelegt hinzu. Für das Verhalten ändert das nichts, denn die Funktion hat so oder so keine Taste; für die Belegungsansicht aus Schritt 20 macht es den Unterschied, ob der Nutzer eine gelöschte Funktion wieder erreichbar machen kann. Ohne die Ergänzung wäre eine von Hand gelöschte Zeile unumkehrbar gewesen.

## Was die abgesegnete Belegungsdatei den Parser gekostet hat

Nichts. Die 52 Kombinationen lesen sich alle, sind konfliktfrei, und `Belegung → TOML → Belegung` liefert denselben Wert zurück. Der Kopfkommentar hat sich als Vertragsbeschreibung getragen; die Reihenfolge `ctrl, opt, shift, cmd` steht durchgehend, und `fn+` kommt nirgends vor.

Zwei Beobachtungen an der Datei sind als Defekte abgelegt und nicht nebenbei behoben, weil die Datei abgesegnet ist:

- `issues/260803-2317_o_der-kopf-der-belegungsdatei-nennt-eine-annahme-als-gemessen.md` — der Kopf zitiert für den Gleichlauf von F3 mit und ohne fn eine Messung, die dazu nichts sagt.
- `issues/260803-2317_o_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md` — die Belegung über den Tastencode trifft bei `cmd+y` auf einer deutschen Tastatur die Taste Z.

Dazu ein Defekt am Plan: `issues/260803-2317_o_der-include-str-pfad-in-schritt-11-liegt-eine-ebene-zu-hoch.md`.

## Eine Änderung außerhalb der drei neuen Dateien, die Erwähnung verdient

`ModMaske::BENANNT` hieß `command`, `control`, `option`, `shift` und behauptete in seinem Kommentar, das seien die Namen der Belegungsdatei. Das war falsch, und es hätte eine zweite Schreibweise für die Zusatztasten bedeutet: eine zum Lesen der Datei, eine für die Anzeige. Die Liste trägt jetzt `ctrl`, `opt`, `shift`, `cmd` in der Reihenfolge der Datei und dient dem Parser wie der Anzeige. Sichtbare Folge: `--tasten-protokoll` schreibt `maske=shift+cmd` statt `maske=command+shift`, also die Form, die der Nutzer in seine `keymap.toml` übernehmen kann.

## Abnahme

Alle fünf Kommandos mit Rückgabewert 0, gefahren am 260803-2317:

| Kommando | Ergebnis |
|---|---|
| `cargo test -p krk-core --test belegung` | 26 Prüfungen, alle grün |
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 170 Prüfungen, alle grün |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, ohne Warnung |

Die sechs Punkte des Abnahmekriteriums und ihre Prüfung:

| Was verlangt ist | Prüfung |
|---|---|
| Die Auslieferungsbelegung ist konfliktfrei. | `die_auslieferungsbelegung_ist_konfliktfrei` |
| Tastencode 99 trifft dieselbe Funktion, mit und ohne `function`. | `tastencode_99_trifft_dieselbe_funktion_mit_und_ohne_function` |
| Eine vergebene Kombination liefert einen Konflikt mit dem Namen der anderen Funktion. | `eine_bereits_vergebene_kombination_liefert_einen_konflikt_mit_dem_namen_der_anderen_funktion` |
| Eine zweite Kombination an derselben Funktion liefert keinen Konflikt. | `eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt` |
| Zurücksetzen stellt die eingebettete Tabelle wieder her. | `zuruecksetzen_stellt_die_eingebettete_tabelle_wieder_her` |
| Ein unbelegter Buchstabe ohne Zusatztaste fällt auf die Sprungmarke durch. | `ein_unbelegter_buchstabe_ohne_zusatztaste_faellt_auf_die_sprungmarke` (alle 26 Buchstaben) |

Die `unsafe`-Grenze steht unverändert. `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-core/src` nennt genau `verzeichnis/sys.rs`, dasselbe für `crates/krk-ui/src` nennt genau `appkit/mod.rs`. Keine der drei neuen Dateien enthält `unsafe`.

## Drei Punkte für die Nachfolgeschritte

1. **Die Sprungmarke ist angelegt, aber nicht gebaut.** `Nachschlag::Sprungmarke` sagt, dass ein Tastendruck dem Tippen der Anfangsbuchstaben gehört; welches Zeichen er trägt, weiß nur die Oberfläche, denn ein Tastencode benennt eine Stelle und kein Zeichen. Schritt 13 nimmt `NSEvent.characters` dazu.
2. **Der Nachschlag ist eine Schleife über 52 Einträge.** Verglichen werden zwei ganze Zahlen. Sollte eine spätere Messung das doch als Kostenpunkt zeigen, ist der Ort dafür `Belegung::nachschlag` und sonst keiner.
3. **Schritt 20 findet vor:** `Belegung::funktionen()` für die Zeilen der Ansicht, `Funktion::reserviert_fuer()` für die Kennzeichnung von F4, `zuweisen`/`zuruecksetzen`/`sichern` für das Ändern, und `Kombination::aus_tastendruck`, das `None` liefert, wenn eine gedrückte Taste in der Schreibweise keinen Namen hat.
