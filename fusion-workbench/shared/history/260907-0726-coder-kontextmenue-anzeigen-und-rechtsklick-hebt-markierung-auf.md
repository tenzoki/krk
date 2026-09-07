# „Im Finder anzeigen" als fünfter Kontextmenü-Eintrag, und der Rechtsklick hebt die Markierung auf

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Zwei Nutzerentscheidungen vom 260907-0703, festgehalten in
`260905-2008-orchestrator-session.md`, Abschnitt
`## Fuenf weitere Entscheidungen am 260907-0703 beantwortet` (Punkt 5 und der Absatz „Dazu eine
neue Anforderung des Nutzers").

## Auftrag 1: „Im Finder anzeigen"

`Kontextbefehl` trägt einen weiteren Wert. Der vorhandene `ImFinderZeigen` heißt jetzt
`ImFinderOeffnen` (er öffnet den angezeigten Ordner), der neue `ImFinderAnzeigen` deckt die
betroffenen Einträge in einem Finder-Fenster auf; damit tragen die zwei Namen die zwei Wirkungen.
Die Methode `Anwendungsdelegierter::im_finder_zeigen` ist zu `im_finder_oeffnen` umbenannt, daneben
steht `im_finder_anzeigen`. Titel: „Im Finder öffnen" und „Im Finder anzeigen", mit Umlaut nach
Punkt 4 desselben Entscheids. Marken 3 und 4; die zwei stehen im Menü nebeneinander.

Der neue Zweig wirkt auf `operationen::betroffene`, also auf dieselbe Menge wie F5, F6 und die
zwei Archivwege. Eine leere Menge meldet `operationen::nichts_anzuzeigen`.

**Der Aufruf selbst gibt nichts zurück.** `NSWorkspace::activateFileViewerSelectingURLs:` liefert
`void`. Ein Scheitern ist an ihm nicht feststellbar, also steht die Meldung davor: über
`terminal::anwendung_vorhanden` wird gefragt, ob das System überhaupt einen Finder nennt, und auf
`false` meldet der Zweig `operationen::kein_finder()` — denselben Satz wie der Nachbarzweig, weil
die Lage dieselbe ist. Die Antwort ist notwendig und nicht hinreichend, und sie ist im Code als
solche ausgeschrieben. Was damit offenbleibt (ein Pfad, den es nicht mehr gibt, bleibt stumm),
steht als offene Frage in
`260907-0726_*_meldet-im-finder-anzeigen-einen-eintrag-der-nicht-mehr-dasteht.md`.

Neu ist das Modul `crates/krk-ui/src/appkit/finder.rs`, ein Modul je Frage wie `standardprogramm`
und `terminal` daneben; die Abgrenzung zu `terminal` steht in seinem Kopf, ebenso der Abschnitt
„Ab welchem macOS die angesprochenen Klassen stehen". In `terminal.rs` ist die Auflösung der
Bündelkennung zu `anwendungsort` herausgezogen, damit sie die eine Stelle des Programms bleibt,
die eine auflöst; `ordner_oeffnen` und das neue `anwendung_vorhanden` gehen beide durch sie.

`nichts_betroffen` nimmt jetzt die ganze `zu`-Nennform statt des nackten Verbs. Grund: „anzeigen"
ist trennbar, und der Rumpf mit festem `zu` hätte „nichts zu anzeigen" geschrieben. Die drei
älteren Sätze lauten Zeichen für Zeichen wie zuvor; eine Probe hält das fest.

## Auftrag 2: Der Rechtsklick hebt die Markierung auf

`operationen::rechtsklick_zielzeile` antwortet unverändert; geändert hat sich, was der Aufrufer
auf `Some(zeile)` tut: `DateifensterQuelle::rechtsklick_auswahl_nachziehen` hebt jetzt zuerst über
`markierung_aendern(Ordnermodell::markierung_aufheben)` jede Markierung auf und setzt danach die
Zeile. Beides hängt an derselben Antwort — `Some` kommt genau dann, wenn die angeklickte Zeile
nicht markiert ist —, also entsteht keine zweite Auswahlregel. Auf einer markierten Zeile bleibt
alles stehen. Ein Klick auf keine Zeile hebt nichts auf; der Entscheid spricht von der unmarkierten
**Zeile**.

Die Reihenfolge ist bindend: `markierung_aendern` stellt über `auswahl_anzeigen` die Auswahl des
Modells wieder her und nähme eine vorher gesetzte Zeile zurück.

## Proben

Neu in `crates/krk-ui/src/kommandos/operationen.rs`:
`nach_dem_rechtsklick_wirkt_der_befehl_auf_die_angeklickte_zeile` (die Hälfte, die der Entscheid
geschlossen hat: sie stellt den Weg des Aufrufers Zug um Zug nach und fragt danach `betroffene`),
`auf_einer_markierten_zeile_bleibt_die_markierung_die_betroffene_menge` (die Gegenprobe zur
Ausnahme) und `der_satz_des_aufdeckens_traegt_die_getrennte_nennform`. Nachgezogen: `TAFEL` und
die Markenprobe in `kontextmenue.rs`, `ZWEIGE` in `anwendung.rs`.

## Was nicht gemacht ist

`crates/krk-ui/src/kommandos/loeschwarnung.rs`, `crates/krk-ui/src/appkit/blaetter/` und
`crates/krk-core/src/operation/mod.rs` sind nicht angefasst — sie gehören einer zweiten Bahn
derselben Sitzung. `CLAUDE.md` nennt weiterhin vier Kontextmenü-Einträge und einen „vierten Wert"
an `Kontextbefehl`; der Befund liegt als
`260907-0726_*_claude-md-nennt-vier-kontextmenue-eintraege-und-drei-ohne-kommando-seit-heute-sind-es-fuenf-und-vier.md`.
Im Code steht kein „Runde 24": es gibt keinen aktiven Circle, und eine Rundennummer ohne Datensatz
wäre eine Behauptung; datiert wird stattdessen mit `260907`.

## Verifikation

`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` und
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` je Exit 0.
