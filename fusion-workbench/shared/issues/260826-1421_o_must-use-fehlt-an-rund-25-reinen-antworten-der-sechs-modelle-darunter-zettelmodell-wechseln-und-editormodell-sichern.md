`#[must_use]` fehlt an rund 25 reinen Antworten der sechs Modelle, darunter `Zettelmodell::wechseln` und `Editormodell::sichern`

---

Die sechs Modelldateien unter `crates/krk-ui/src/` tragen die Regel aus `CLAUDE.md` („ein
Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`") an neun
Stellen und schreiben sie an drei davon in langen Doc-Kommentaren aus (`tabs.rs:304-323`,
`editormodell.rs:929-939`, `zettelmodell.rs:166-171`, `:192-203`). Direkt daneben stehen
Antworten derselben Art ohne die Marke. Am Baum gezählt, HEAD `ca8072d`:

| Datei | trägt `#[must_use]` | trägt es nicht, sagt aber dasselbe |
|---|---|---|
| `fenstermodell.rs` | `umschalten` (:650), `einblenden` (:734), `spalte_umschalten` (:585), `traegt_eine_ziehbewegung` (:1185), `wuensche_nachfuehren` (:1220) | `aktiv_setzen` (:478, „ob sich etwas geaendert hat"), `fenster_wechseln` (:490) |
| `tabs.rs` | `Auswahlversuch` (:325), `Einzug` (:352), `durchlauf_nachziehen` (:859) | `waehlen` (:517, „nur dann muss die Ansicht ihren Inhalt austauschen"), `naechster` (:532), `voriger` (:538), `schliessen` (:560) |
| `editormodell.rs` | `bearbeiten` (:940) | `oeffnen` (:747, `Option<Ladeausgang>`), `einziehen` (:855), `zurueckgehaltenes_uebernehmen` (:823), **`sichern` (:986, `Sicherungsausgang`)**, `fremdaenderung_melden` (:1076), `suche_starten` (:1106), `weitersuchen` (:1121), `rueckwaerts_suchen` (:1126), `treffer_ersetzen` (:1180), `alle_treffer_ersetzen` (:1213), `ansicht_umschalten` (:680) |
| `vorschaumodell.rs` | — | `schliessen` (:447), `waehlen` (:477), `naechster` (:463), `voriger` (:468), **`einziehen` (:578, „nur dann muss die Ansicht neu zeichnen")** |
| `leistenmodell.rs` | — | **`gueltigkeit_pruefen` (:303, „nur dann muss die Ansicht neu zeichnen")**, `waehlen` (:398), `auswahl_bewegen` (:412) |
| `zettelmodell.rs` | `oeffnen` (:172), `bearbeiten` (:204) | **`wechseln` (:218, `Wechsel::GewechseltZuSichern`)** |

Die vier fett gesetzten sind die, deren Fallenlassen genau den Schaden hätte, den die
ausgeschriebenen Begründungen daneben beschreiben: ein nicht gesicherter Zettel
(`zettelmodell.rs:192-198` begründet das `#[must_use]` an `bearbeiten` mit „ein vergessenes
Sichern fällt nirgends auf" — `wechseln` ist der zweite Weg zu derselben Entscheidung und trägt
es nicht), ein Sichern, dessen `Gescheitert` niemand liest, eine Vorschau und eine Leiste, die
nicht neu zeichnen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/{fenstermodell,tabs,editormodell,vorschaumodell,leistenmodell,zettelmodell}.rs`
**Baumstand:** `ca8072d`

## Heute bindet jeder Rufer

Nachgesehen: `anwendung.rs:3208` (`fenster_wechseln`), `:4578` (`aktiv_setzen`), `:3993`
(`wechseln`), `:7110` (`sichern`), `editor.rs:1709`, `:1732-1736`, `leiste.rs:303`, `:351`. Kein
Rufer lässt heute einen dieser Werte nackt fallen. Das ist der Grund für „Niedrig" — und der
Grund, warum die Marke gerade jetzt billig ist: sie kostet keine Aufrufstelle eine Änderung.

## Geschwister dieses Befunds

Dieselbe Lücke ist für die übrigen Kisten und Module schon erhoben, jeweils mit eigener Zählung:
`shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-…`,
`260826-1221_*_must-use-traegt-sieben-praedikate-des-verzeichnisbaums-…`,
`260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…`,
`260826-1327_*_must-use-fehlt-in-editor-rs-ganz-und-in-tabelle-rs-…`,
`260826-1335_*_zwei-von-rund-zwanzig-reinen-antworten-der-blaetter-…`. Dieser Datensatz deckt
die sechs Modelle ohne AppKit, die keiner davon nennt.

## Weg

Die Marke an die vier fetten Stellen zuerst, mit Begründung im Attribut wie an `umschalten`;
danach die Prädikate „ob sich etwas geändert hat" in einem Zug. Wer den Wert dann wirklich nicht
braucht, schreibt `let _ =` — die Konvention steht schon (`tabs.rs:314-323`).
