„Verbinden…“ an einer leeren, im Editor offenen `secrets.txt` legt eine zweite PIN fest, und der Editor bleibt mit einem Schlüssel stehen, der die Datei nicht mehr öffnet
---
Hält der Editor eine leere `secrets.txt` (eben mit „PIN festlegen“ geöffnet, noch nie gesichert), lässt „Verbinden…“ das Blatt zu, fragt erneut „PIN festlegen“ und schreibt das erste Chiffrat mit einem zweiten, frischen Salz. Das anschließende Neuladen des Editors scheitert, und Editor und Merker halten danach zwei verschiedene Schlüssel für dieselbe Datei.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Entscheidungen 5, 6, 14, 15)

## Defekt

Der Ablauf, am Quelltext auf `6067ae9` nachgelesen (der Lauf selbst ist nicht gefahren, die Folge ist **inference** aus den Zweigen):

1. `f2`/`f4` auf eine leere `secrets.txt`: der Editor legt mit PIN P1 fest, `tresorweg::geheimnisse_lesen` (`crates/krk-ui/src/tresorweg.rs:212-218`) leitet über `tresor::neuer_schluessel` den Schlüssel K1 mit Salz S1 ab. `Schutz::Verschluesselt { kopf_auf_platte: false }`, `abweichung == false`, und K1 geht als eingegeben an den Merker.
2. „Verbinden…“: `verbindung_erfragen` (`crates/krk-ui/src/appkit/anwendung.rs:9072-9085`) weist allein ab, wenn `haelt_geheimnisse() && hat_ungesicherten_stand()`. `hat_ungesicherten_stand` ist `self.abweichung` (`crates/krk-ui/src/editormodell.rs:1268-1270`) und für die frisch geöffnete leere Datei falsch; das Blatt geht auf.
3. `verbindungsschluessel_holen` (`anwendung.rs:9116-9154`): `groesse == Some(0)`, also `geheimnisweg` → `PinErfragen(Pinform::Festlegen)` (`editormodell.rs:1062-1064`, `geheimnisweg`, Arm `(Some(0), _)`), auch bei gültigem Merker. Der Nutzer legt P2 fest (er kann P1 wiederholen, muss aber nicht), der Faden leitet K2 mit frischem Salz S2 ab und schreibt das Chiffrat.
4. `anhaengen_einziehen` (`anwendung.rs:9235-9240`): der Editor hält `secrets.txt` ohne ungesicherten Stand und lädt über `neu_laden` mit **K1** neu. Salz S1 ≠ S2: `oeffnen_mit` weist mit `PinFalschOderVeraendert` ab; da `Uebernahme::Verwerfend`, gibt es kein `SchluesselAbgewiesen` (`editormodell.rs:1706`, Kommentar „beim Neuladen bleibt es die Abweisung“; `neu_laden` ab `editormodell.rs:1480`), sondern eine Abweisung in der Statuszeile. Der Editor zeigt weiter den leeren Stand mit K1 und `kopf_auf_platte: false`; der Merker hält inzwischen K2 (`gemerkt_falls`).
5. Tippt der Nutzer im Editor weiter und sichert, meldet die Stempelprüfung eine Änderung von außen; wählt er Überschreiben, verschließt der Editor mit K1 und S1: der Eintrag aus „Verbinden…“ ist weg, die Datei trägt P1, der Merker K2, und das nächste Öffnen mit dem Merker scheitert.

Entscheidung 15 deckt den Fall „Editor mit ungesichertem Stand“ ab, aber nicht den eines Editors, dessen Schlüssel noch in keinem Kopf steht. Entscheidung 6 („eine leere Datei geht immer über das Festlegen“) setzt voraus, dass keine zweite Partei schon einen Schlüssel für diese leere Datei hält.

## Richtung der Behebung

Eine Regel an der einen Vorprüfung, statt eines Sonderfalls im Ausgang: `verbindung_erfragen` weist auch dann mit „erst sichern“ ab, wenn der Editor `secrets.txt` hält und ihr Kopf noch nicht auf der Platte steht (die Frage beantwortet das Modell schon über `pin_aenderbar`/`kopf_auf_platte`). Alternativ hält der Editor ab dem Festlegen gar keinen Schlüssel ohne Kopf; das berührt Entscheidung 5 und ginge an den Nutzer.

## Akzeptanz

- Eine Probe am Modell oder am Quelltext hält: hält der Editor `secrets.txt` mit `kopf_auf_platte == false`, öffnet „Verbinden…“ kein Blatt und meldet den Grund.
- Die Probe `der_verbindungsweg_reisst_nirgends_ab` bleibt grün; `make check` grün.

---
Resolved: Die eine Vorpruefung von „Verbinden…“ fragt jetzt `Editormodell::schreibgang_gesperrt` (`crates/krk-ui/src/editormodell.rs`, neu mit `Schreibgangsperre::{Ungesichert, OhneKopf}`), die neben dem ungesicherten Stand auch `Schutz::Verschluesselt { kopf_auf_platte: false }` sperrt; `Anwendungsdelegierter::verbindung_erfragen` (`crates/krk-ui/src/appkit/anwendung.rs`) zeigt dann den Satz `Text::VerbindenPinNochNichtGesichert` (neu in `sprache/schluessel.rs` und den drei Tabellen) und oeffnet kein Blatt. Probe am Modell: `ein_schreibgang_wartet_auf_den_kopf_und_den_gesicherten_stand` (`editormodell.rs`); `der_verbindungsweg_reisst_nirgends_ab` fuehrt die Nadel `.schreibgang_gesperrt()` und bleibt gruen. `HowTo.md`, Tabelle unter „Verbinden…“, nennt die neue Lage.
