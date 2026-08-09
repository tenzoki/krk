# S25: Sichern

**Status:** Complete
**Ausführender:** coder
**Datum:** 260809-2353
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken

## Auftrag

S25 („Sichern") aus `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
umsetzen: `cmd+s` schreibt den Stand des Editors in die Datei, in der am
260808-0043 entschiedenen Form, über die Funktion aus S9 und den atomaren
Schreibweg der vier Ablagedateien. Die Vorbedingung stand: S26 hat am
260809-2322 die Rückschreibung aus der Textfläche gebaut, und
`hat_ungesicherten_stand` kann seither wahr werden.

## Was gebaut wurde

### Der Befehl

`Kommando::EditorSichern` hat seinen Zweig in `kommando_ausfuehren` bekommen und
läuft damit nicht mehr in `bereichskommando`, wo `Fokus::Editor` mit `false`
antwortet. Er steht neben den übrigen Editorbefehlen und nicht bei den
Bereichsbefehlen, weil der Editorbereich am Anwendungsdelegierten hängt.

`Anwendungsdelegierter::editor_sichern` trägt die Fallunterscheidung über die
drei Ausgänge, vollständig und ohne Auffangzweig, so wie
`editorausgang_behandeln` es für den Ladeausgang tut.

### Die Stempelprüfung vor dem Schreiben

`Editormodell::sichern` fragt vor dem Schreiben `fremd_geaendert`. Hat die Datei
sich seit dem Öffnen oder dem letzten Sichern verändert, unterbleibt das
Schreiben, und der Grund geht in die Statuszeile. Das ist die Hälfte des neunten
Abnahmekriteriums von C4, die ohne Weiteres zuverlässig ist: sie fragt in dem
Augenblick, in dem es darauf ankommt.

Gefragt wird über **dieselbe** Funktion, die S31 beim Ordnerereignis fragen
wird, und nicht mit einer zweiten, enger geschnittenen Frage daneben. Der Plan
sagt das an S31 ausdrücklich zu: „kein zweiter Mechanismus, sondern dieselbe
Frage an zwei Stellen".

### Die beiden Meldungen

`Editormeldung` hat zwei Varianten bekommen, `Gesichert` und
`SichernGescheitert`. Der Grund des Fehlschlags kommt fertig aus dem Modell und
wird oben nicht ein zweites Mal formuliert — dort wird entschieden, woran es
lag, am Schreiben selbst oder an einer fremden Änderung. Eine zweite
Meldefläche ist nicht entstanden: beide gehen über `editormeldung_zeigen` in die
eine Statuszeile auf Rang 1.

`Sicherungsausgang::NichtsGehalten` geht **nicht** über `Editormeldung`, sondern
über `antwort_zeigen`, nach dem Satz, den S22 für F4 auf leerer Auswahl führt:
eine Meldung des Editors handelt von der gehaltenen Datei, und hier hält er
keine. Erreichbar ist der Zweig kaum, weil `Wirkungsbereich::Editor` den Befehl
nur mit dem Fokus in der Textfläche durchlässt; kommentarlos bleibt er trotzdem
nicht.

### Der Kopf

`Editorbereich::sichern` zieht nach einem gelungenen Sichern `kopf_nachziehen`
nach, sonst trüge der Kopf sein Abweichungszeichen weiter, obwohl das Modell
keine Abweichung mehr meldet. Nach einem gescheiterten bleibt der Kopf, wie er
ist, weil auch die Abweichung bleibt.

## Zwei Entscheidungen, die der Schritt getroffen hat

**Das gelungene Sichern meldet sich, obwohl der Kopf es schon zeigt.** Die
beiden sagen Verschiedenes: der Kopf trägt den Zustand, die Statuszeile die
Antwort auf den Tastendruck. Wer `cmd+s` an einer unveränderten Datei drückt,
sieht am Kopf nichts geschehen und bekäme sonst kommentarlos nichts.

**`Sicherungsausgang::Gesichert` trägt seither den Pfad.** Die Meldung nennt
ihn, und der Aufrufer müsste ihn sonst an einem Modell erfragen, das die Frage
eben beantwortet hat, als es schrieb — mit einem `Option`, das an dieser Stelle
nie leer ist, weil ein leeres `NichtsGehalten` heißt.

## Der Preis, der benannt ist und nicht verschwiegen wird

Eine **verschwundene** Datei gilt nach `fremd_geaendert` als von außen geändert.
Sie wird deshalb nicht neu geschrieben, solange die Wahl aus dem Zustandsbild
des Specs (`Fremd` mit seinen zwei Ausgängen) nicht gebaut ist. Der Stand des
Editors bleibt dabei vollständig stehen, und der Grund steht in der Statuszeile;
verloren geht nichts. Eine Frage, die das Verschwinden vom Ändern trennte, wäre
ein Sonderfall mit eigener Regel an einer Stelle, die genau eine Frage zu
stellen hat.

Ein Wettlauf bleibt und ist mit den Mitteln dieser Runde nicht zu schließen:
zwischen der Stempelfrage und dem `rename` liegt eine Spanne, in der ein fremder
Schreiber zuschlagen kann. Die Prüfung macht das Fenster klein; zu schließen
wäre es allein mit einer Sperre auf der Datei, und die sagt weder C4 noch der
Spec zu.

## Eine Ungenauigkeit in der `Nutzerarbeit` des Schrittes

Der Plan schreibt: „an einer Datei ohne Schreibrecht meldet `cmd+s` den Grund".
Das trifft den Fall nicht ganz. `krk_core::ablage::atomar` schreibt eine
Nachbardatei und benennt sie um, und ein `rename` gelingt auch auf eine
schreibgeschützte **Datei**, solange der **Ordner** darum beschreibbar ist. Die
Probe erzeugt den Fehlschlag deshalb am Ordner.

Am Gerät geprüft, nicht angenommen: ein `mv neu.txt ziel.txt` auf ein `ziel.txt`
mit dem Kennzeichen `uchg` (im Finder „Geschützt") scheitert mit „Operation not
permitted". Wer eine einzelne Datei sperren will, nimmt diesen Weg; die
Prüfliste an den Nutzer nennt beide.

## Geänderte Dateien

- `crates/krk-ui/src/editormodell.rs` — Stempelprüfung vor dem Schreiben,
  `Gesichert` trägt den Pfad, Modulkopf und zwei Doc-Kommentare nachgezogen,
  drei Proben mitgezogen, drei neue Proben.
- `crates/krk-ui/src/appkit/editor.rs` — `Editorbereich::sichern` mit dem
  Kopfnachzug, zwei neue Varianten in `Editormeldung` samt ihren Sätzen,
  Modulkopf und Auslösertabelle nachgezogen, eine neue Probe.
- `crates/krk-ui/src/appkit/anwendung.rs` — der Zweig für
  `Kommando::EditorSichern`, `editor_sichern` mit der Fallunterscheidung über
  die drei Ausgänge.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 15 Testbinärdateien, keine fehlgeschlagene |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0, keine Warnung |
| `cargo fmt --all --check` | 0 |
| `cargo xtask bundle` | baut und signiert `target/KRK.app` |

Vier Proben decken den Schritt ab:

- `eine_von_aussen_geaenderte_datei_wird_nicht_ueberschrieben` — die fremde
  Änderung steht nach dem Sichern unverändert auf der Platte.
- `eine_verschwundene_datei_wird_nicht_neu_geschrieben` — der benannte Preis.
- `ein_gescheitertes_schreiben_laesst_den_stand_stehen` — Stand und Abweichung
  überleben einen Fehlschlag, die Datei bleibt, wie sie war.
- `das_sichern_meldet_gelingen_und_fehlschlag_verschieden` — beide Ausgänge
  melden sich, und sie melden Verschiedenes.

Dazu tragen `ein_eingefuegtes_crlf_landet_nicht_auf_der_platte` und
`die_abweichung_kommt_mit_der_aenderung_und_geht_mit_dem_sichern` weiterhin die
Sicherungsform und den Übergang zurück in den reinen Stand.

**Was ein Agent nicht abnehmen kann**, steht in der Prüfliste an den Nutzer: ob
das Zeichen am Kopf mit `cmd+s` verschwindet, ob die Statuszeile die beiden
Sätze zeigt und ob eine von außen geänderte Datei tatsächlich stehen bleibt. Das
verlangt KRK im Vordergrund.

## Datensätze

- Der Plan trägt S25 auf `[DONE]`, mit fünf Vermerken über das, was der Schritt
  gegenüber seiner Fassung anders oder zusätzlich gebaut hat.
- **Offen und dem Orchestrator vorgelegt:**
  `decisions/260808-0021_a_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`
  ist mit diesem Schritt in Code eingelöst — die Sicherungsform aus S9 hat mit
  `cmd+s` ihren Auslöser bekommen. Der Marker steht weiterhin auf `_a_`, weil
  der Vermerk `Implemented:` den Commit-Hash zitieren muss und dieser Agent
  nicht committet.
