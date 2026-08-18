# Coder — Schritt 2: Das Kommando in die vier Pflichtstellen

**Datum:** 260818-1740
**Status:** Complete
**Modus:** Dispatch durch den Orchestrator
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 2
**Baumstand beim Beginn:** `8d5baf6` plus die unfestgeschriebenen Schritte 1 und 3

## Was der Auftrag war

Das Kommando `OrdnerAngleichen` in die vier Stellen setzen, die es braucht,
und damit die Lücke schließen, die Schritt 1 im Baum hinterlassen hat: die
Belegungsdatei führte die Funktion `ordner_angleichen` schon, `Kommando` kannte
sie nicht, und `belegungsmodell::nach_bereichen` bricht bei einer Funktion ohne
Funktionsbereich laut ab. **51 Proben liefen deshalb rot**, sämtlich aus diesem
einen Abbruch in `belegungsmodell.rs:831`.

Der Plan sagt unter Schritt 1, der Zwischenstand sei gültig und die Proben
blieben grün. Das trifft nicht zu; der Befund dazu ist vom Nutzer schon gefilt.
An der Arbeit ändert es nichts — die Schritte 1 und 2 sind ein Commit, und
dieser Schritt ist der, der die Lücke schließt.

## Was geändert wurde

**`crates/krk-core/src/tasten/belegung.rs`**

- Variante `OrdnerAngleichen` hinter `OrdnerDerDatei`, mit Doc-Kommentar. Er
  hält zwei Dinge fest: dass die Richtung eine ist und nicht zwei (Quelle immer
  das aktive Dateifenster, Ziel immer das andere), und dass die Ausführung in
  `krk-ui` wohnt, weil sie die Sichtbarkeit von Bereichen braucht.
- `KENNUNGEN`: Länge `78` → `79`, Eintrag
  `(Kommando::OrdnerAngleichen, "ordner_angleichen")` unmittelbar hinter
  `ordner_der_datei`, also an derselben thematischen Stelle wie in der
  Belegungsdatei.
- `Kommando::wirkungsbereich`: in den Arm `Wirkungsbereich::Dateifenster`,
  eingereiht hinter `OrdnerAufwaerts`. Der Kommentar an der Zeile trägt das
  Argument des Plans aus, warum dieser Befehl auf der anderen Seite der Linie
  steht als `ordner_der_datei`: dessen Quelle hängt nicht am Fokus und wird aus
  Vorschau und Editor gedrückt, die Quelle des Angleichens **ist** der
  angezeigte Ordner eines Dateifensters.

**`crates/krk-ui/src/belegungsmodell.rs`**

- `bereich_des_kommandos`: in den Arm `Funktionsbereich::Dateilisting`, neben
  Aufstieg, Ordnersprung und Zwischenablagesprung, mit demselben Grund, den der
  Kommentar dort schon führt — alle setzen den Ordner, den eine Dateiliste
  zeigt. Der Zusatz beantwortet die naheliegende Rückfrage: dass es die
  **andere** Liste ist, die sich bewegt, macht keinen zweiten Ort auf.
- Neue Probe `das_ordnerangleichen_steht_unter_dateilisting` nach dem Vorbild
  von `der_ordnersprung_steht_unter_dateilisting`. Sie prüft die Einordnung und
  dass die Funktion ab Werk eine nichtleere Tastenliste trägt.

## Was ausdrücklich nicht angefasst wurde

- **`NEUE_KENNUNGEN` (`belegungsmodell.rs:1409`)** — am Bestand gelesen und
  bestätigt: `[&str; 13]`, im Namen und im Doc-Kommentar der Editor-Runde
  zugeschnitten („Jede der dreizehn Kennungen, die S6 der Belegungsdatei
  hinzugefügt hat"). Eine vierzehnte Zeile behauptete etwas Falsches über die
  Herkunft der Liste.
- `resources/default-keymap.toml` (Schritt 1, schon im Baum) und
  `crates/krk-ui/src/appkit/anwendung.rs` (Schritt 3, ebenso).
- Der Ausführungszweig. Der Befehl steht jetzt im Hauptmenü und in der
  Belegungsansicht und **tut nichts**; das ist der vom Plan benannte
  Zwischenstand und der Grund für Schritt 4.

## Abnahme

`make check` — Beendigungsstatus `0`. Alle vier Kommandos grün: Bau, Proben,
Clippy unter `-D warnings`, Formatierung. **Keine der 51 Proben ist mehr rot**,
und die neue läuft mit:
`test belegungsmodell::tests::das_ordnerangleichen_steht_unter_dateilisting ... ok`.

Vor dem Lauf geprüft, dass kein Messlauf steht: weder `/tmp` noch `$TMPDIR`
führt eine `krk-messplan-*.toml`.

Nicht festgeschrieben — der Nutzer schreibt die Schritte 1, 2 und 3 zusammen
fest.
