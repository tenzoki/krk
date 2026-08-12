# Bekommen die Spaltenschalter Tastenbefehle?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator (Klaerungsrunde bei der Aktivierung)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`, `crates/krk-ui/src/appkit/tabelle.rs:180` (`Spalte`)

---

## Question

Die Directive verlangt fuer die Bereichsschalter Tastatur **und** Maus. Fuer die Spaltenschalter
sagt der Nachtrag nichts.

## Options

1. **Ja, drei neue Kommandos in der ausgelieferten Belegung.**
2. **Ja, drei neue Kommandos, aber ohne ausgelieferte Tastenkombination.**
3. **Nein, nur Maus.**

## Antwort

**Moeglichkeit 2: drei neue Kommandos, in der Belegung gefuehrt, ohne ausgelieferte
Kombination.**

Die Kommandos entstehen ohnehin: ein Klick in der Leiste geht durch dieselbe Modellfunktion wie
ein Tastenbefehl, damit kein zweiter Weg an den Pruefungen vorbei entsteht — dieselbe Bedingung,
die C7 fuer die Abweisung am letzten Dateifenster stellt. Sie sind damit in
`Kommando` aufgezaehlt, tragen einen `Wirkungsbereich` und stehen in der Belegungsansicht und in
der Markdown-Ausgabe der Runde 3.

**Keine ausgelieferte Kombination**, weil die freien Kombinationen knapp sind: die
Auslieferungsbelegung fuehrt 39 frei gewaehlte
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_*_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`),
und eine Spaltensichtbarkeit ist eine Einstellung, die man einmal trifft, kein Handgriff im
Arbeitsfluss. Wer eine Taste dafuer will, traegt sie in `default-keymap.toml` ein; die Datei ist
nach C3 die eine Quelle jeder Belegung und von Hand aenderbar.

**Die Bereichsschalter dagegen bekommen ausgelieferte Kombinationen**, weil die Directive es
verlangt. Drei davon gibt es schon (`leiste_umschalten`, `zweites_fenster_umschalten`,
`vorschau_umschalten`); es fehlen ein Umschalter fuer den Editor und einer fuer das linke
Dateifenster.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
