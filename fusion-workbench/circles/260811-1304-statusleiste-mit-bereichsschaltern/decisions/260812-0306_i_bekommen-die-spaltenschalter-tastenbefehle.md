# Bekommen die Spaltenschalter Tastenbefehle?

---
**Domain:** code
**Status:** implemented
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
`Kommando` aufgezaehlt, tragen einen `Wirkungsbereich` und stehen in der Belegungsansicht.

> **Korrektur vom 260812-0735.** Der urspruengliche Satz sagte zu, sie stuenden „in der
> Belegungsansicht **und in der Markdown-Ausgabe der Runde 3**". Der zweite Teil war falsch, und
> zwar aus Unkenntnis eines bestehenden Nutzerentscheids: `belegungsausgabe::markdown` nimmt eine
> Funktion nur auf, wenn sie mindestens eine Kombination traegt
> (`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_i_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md`,
> Nutzerentscheid vom 260811-0110, Moeglichkeit 1). Jener Entscheid gilt und wird von dieser
> Runde nicht angetastet — er ist auch der Sache nach richtig, weil ein Dokument ueber die
> Tastenbelegung zu einer Funktion ohne Taste nichts zu zeigen hat. Die drei Spaltenschalter
> sind ueber die Maus in der Bereichsleiste erreichbar und in der Belegungsansicht am Schirm
> sichtbar; wer ihnen eine Kombination zuweist, findet sie danach auch in der Markdown-Ausgabe.
> Gefunden von coderev am 260812-0727, Datensatz
> `issues/260812-0727_*_die-drei-spaltenbefehle-stehen-nicht-in-der-markdown-ausgabe-obwohl-drei-stellen-es-zusagen.md`.

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
Implemented: 90b02d4 — drei Kommandos in `Kommando`, alle `Wirkungsbereich::Ueberall`, in der Belegung mit `tasten = []`, sichtbar in der Belegungsansicht. **Nicht** in der Markdown-Ausgabe; siehe die Korrektur vom 260812-0735 im Abschnitt `## Antwort`.
Deferred:
Superseded by:
