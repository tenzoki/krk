# Planer: die Neuerungsmeldung und die n Startmeldungen

**Datum:** 2026-09-10, 08:18
**Agent:** planner
**Auftrag:** Plan für den Circle `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`,
Executors `coder, ontocoder, analyst`, Domain `code`

## Was entstanden ist

- `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md` — zehn
  Schritte in Abhängigkeitsreihenfolge, acht an `coder`, zwei an `ontocoder`.
- `260910-0818_*_wo-merkt-sich-krk-fuer-welche-fassung-es-die-neuerungen-schon-gemeldet-hat.md`
- `260910-0818_*_wie-erreichen-n-startmeldungen-den-nutzer-wenn-die-eine-zeile-nur-eine-traegt.md`
- `260910-0818_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l4.md`

## Kein Schritt für den analyst, und warum

Der Auftrag führt `analyst` in der Ausführermenge. Kein Schritt dieses Plans liefert ein
strategisches Erzeugnis: die drei Entscheidungsfragen sind vom Planer selbst gefilzt, samt
Möglichkeiten und Folgen, und eine vergleichende Untersuchung daneben wiederholte sie. Die
Zuordnung ist damit Sache dieses Plans und nicht des Aufrufers, und sie lautet: acht
Schritte `coder`, zwei `ontocoder`, keiner `analyst`.

## Vier Befunde am Baum, die den Zuschnitt getragen haben

**Erstens: `keymap.toml` ersetzt die Auslieferungsbelegung nicht so vollständig, wie die
Prosa sagt.** `Belegung::bauen` (`crates/krk-core/src/tasten/belegung.rs`, der Block hinter
der Schleife über die Einträge) nimmt jede Funktion, die die Nutzerdatei nicht nennt,
**unbelegt** hinzu. Ersetzt werden die Tastenkombinationen, nicht der Bestand der
Funktionen. Für diese Runde ist das tragend: ihr eigener Befehl bleibt für einen Nutzer mit
alter `keymap.toml` über das Hauptmenü erreichbar. Ohne diesen Befund hätte der Plan einen
Weg bauen müssen, den es nicht braucht.

**Zweitens: die Gegenrichtung ist bei zwei der drei Dateien bauartbedingt leer.**
`Einstellungsdatei` und `Profildatei` tragen `#[serde(deny_unknown_fields)]`, und
`Belegung::bauen` gibt für eine unbekannte Kennung `Belegungsfehler::UnbekannteFunktion`
zurück. Ein Eintrag, den der Nutzer hat und die Auslieferungsfassung nicht, macht
`settings.toml` und `keymap.toml` beschädigt statt abweichend. Die Zusage „in beide
Richtungen" wird deshalb an `readers.toml` erfüllt und an zweien leer, mit dem Grund
daneben — und eine Probe hält den Grund fest, sonst ist er eine Behauptung im Modulkopf.

**Drittens: eine fehlende Nutzerdatei ist nicht „zurück".** Auf einer frischen Installation
gibt es keine `keymap.toml`; `belegung::laden` legt sie nicht an. Wer sie als leere Datei
läse, meldete dem Nutzer alle 93 Funktionen als Neuerung. Das ist die Stelle, an der die
Frage falsch geschnitten wäre; der Plan schneidet sie um: verglichen wird, was dasteht.

**Viertens: `nach_bereichen` bricht mit `panic` ab**, wenn eine Funktion der Belegung keinen
Funktionsbereich hat (`crates/krk-ui/src/belegungsmodell.rs`). Der Eintrag in
`resources/default-keymap.toml` muss deshalb **nach** dem Kommando kommen und nicht davor.
Zwischen beiden Schritten ist `cargo test -p krk-core` an genau einer Probe rot; das steht
im Plan, damit der Ausführende des Codeschritts nicht die Datei des Datenschritts anfasst.

## Was der Plan nicht entscheidet

Wo der Merker steht, wie n Startmeldungen den Nutzer erreichen, und ob die Runde einen
Abnahmelauf gegen L4 schuldet. Alle drei sind Nutzerfragen mit Folgen über diesen Plan
hinaus, und alle drei liegen als Datensatz mit Möglichkeiten, Folgen und Empfehlung vor. Die
ersten beiden sperren je einen Schritt.

## Nicht angefasst

Der Circle-Datensatz `_t_circle.md` — seine Zeile `**Active spec/plan:**` steht noch auf
`(none yet)`. Der Planer schreibt Circle-Datensätze nicht; das gehört dem Orchestrator.
CLAUDE.md ebenso wenig; der Abgleich gehört an das Ende der Sitzung.
