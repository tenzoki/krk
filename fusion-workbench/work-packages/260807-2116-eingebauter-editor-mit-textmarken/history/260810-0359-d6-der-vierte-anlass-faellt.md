# D6: Der vierte Anlass der Nachfrage fällt, im Code und im Spec

**Status:** Complete
**Agent:** coder
**Datum:** 260810-0359
**Grundlage:** `decisions/260810-0021_a_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`, Nutzerentscheid vom 260810-0250 (Möglichkeit 2)

## Was umgesetzt ist

Die Nachfrage aus C4 steht seit dieser Sitzung an drei Anlässen statt an vier: dem Schließen des Editors, dem Wechsel auf eine andere Datei und dem Beenden. Das Einblenden der Vorschau fragt nicht mehr. Es verliert nichts, weil ein Wechsel der Sichtbarkeit `hidden` an den Ansichten setzt und das Editormodell nicht anfasst; „Verwerfen" verwarf an dieser einen Stelle nichts.

Die Umsetzung trägt zwei Teile, wie der Datensatz sie vorschreibt, und einen dritten, der aus ihnen folgt.

**Im Code** sind die beiden Werte `Anlass::VorschauUmschalten` und `Anlass::VorschauFokus` gefallen, dazu ihre Vorbedingung `vorschau_verdraengt_den_editor` und die beiden Hüllen `vorschau_umschalten` und `fokus_vorschau_holen`. Die Befehle `f3`, `cmd+y` und `shift+cmd+y` rufen jetzt unmittelbar `bereich_umschalten(Bereich::Vorschau)` und `fokus_holen(Fokus::Vorschau)`, so wie die übrigen Bereichs- und Fokusbefehle daneben. Die Aufzählung `Anlass` trägt drei Werte, und die beiden vollständigen Fallunterscheidungen `anlass_ausfuehren` und `anlass_unterbleibt` halten den Bau weiterhin an, sobald ein vierter dazukäme.

**Im Spec** ist das sechste Abnahmekriterium von C4 gestrichen, samt der Begründung, das Verdrängen verliere denselben Stand wie das Schließen. Die Festlegung „Zwei Anlässe sind hinzugekommen" ist auf den einen verbleibenden zurückgenommen, den Wechsel auf eine andere Datei. An ihre Stelle tritt eine Festlegung, die den Wegfall begründet und den Entscheidungsdatensatz nennt, damit der nächste Leser nicht raten muss, warum aus vier Anlässen drei wurden.

**Die Nummern der Abnahmekriterien von C4 haben sich dabei um eins verschoben.** C4 trägt neun statt zehn, und das siebte bis zehnte heißen jetzt das sechste bis neunte. Zehn Fundstellen im Code und sechs im Plan zitierten diese Nummern und sind mitgezogen; der Spec hält die Verschiebung als eigene Festlegung fest, damit ein Zitat aus einem älteren Dokument auflösbar bleibt.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs` | zwei `Anlass`-Werte, `vorschau_verdraengt_den_editor`, `vorschau_umschalten`, `fokus_vorschau_holen` gefallen; Zweige und Zählungen nachgezogen |
| `crates/krk-ui/src/fenstermodell.rs` | Kopf von `umschalten`: warum dem Einblenden keine Nachfrage mehr vorausgeht |
| `crates/krk-ui/src/editormodell.rs` | fünf Zählungen und Kriteriennummern |
| `crates/krk-ui/src/appkit/editor.rs` | eine Zählung |
| `crates/krk-ui/src/appkit/blaetter/ungesichert.rs` | Modulkopf und eine Kriteriennummer |
| `crates/krk-core/src/ablage/sitzung.rs` | eine Kriteriennummer |
| `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` | C4, beide Zustandsbilder, die Ableitung unter `## Was die Abnahme mitentscheidet` |
| `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` | Nachtrag an S28, sechs Kriteriennummern |
| `issues/260810-0359_o_die-erweiterungsnotiz-zaehlt-elf-abnahmekriterien-fuer-c11-gebaut-sind-dreizehn.md` | neu |

## Was geprüft ist

`make check` läuft mit Rückgabewert 0 durch, alle vier Abnahmekommandos grün. `cargo doc --no-deps -p krk-ui --document-private-items` meldet an den geänderten Stellen keine gebrochenen Verweise; die zweiundzwanzig Warnungen der Kiste stehen an anderen Zeilen und sind älter.

Proben waren keine zu entfernen. `anwendung.rs` führt keinen Testblock, und weder die Einbauproben der Kiste noch die eine Integrationsprobe `tests/syntaxkiste.rs` berühren den gefallenen Anlass. Der gegenseitige Ausschluss aus C1 bleibt geprüft: `fenstermodell.rs` fährt ihn weiter über `der_editor_schliesst_die_vorschau_und_die_vorschau_den_editor` und über die Probe, die jede Folge aus zwei Aufrufen abfährt.

## Was offen bleibt

Der Entscheidungsdatensatz steht weiter auf beantwortet (`_a_`). Der Sprung auf umgesetzt (`_i_`) verlangt die Zeile `Implemented:` mit dem Commit, und diese Sitzung committet nicht; er gehört zum Commit dieser Änderung.

Ein Defekt ist neu gefiled: die Erweiterungsnotiz des Specs beziffert C11 mit elf Abnahmekriterien, gebaut sind dreizehn. Er ist älter als diese Änderung und hängt nicht an ihr.

Zehn Stellen im Plan und in den Sitzungsberichten führen weiter „vier Anlässe" als Beschreibung des Standes vom 260810-0021. Die Sitzungsberichte bleiben, wie sie sind: sie beschreiben einen Tag und keinen Sollzustand. Die Stellen im Plan, die den Aufbau der Nachfrage skizzieren (`### Die Nachfrage vor den vier Anlässen`, die Überschrift von S28 und die Vermerke darunter), gehören in den Abschlussschritt S42 und sind dort nachzuziehen; der Nachtrag an S28 hält bis dahin fest, dass die Vorschau-Anlässe nicht nachzubauen sind.
