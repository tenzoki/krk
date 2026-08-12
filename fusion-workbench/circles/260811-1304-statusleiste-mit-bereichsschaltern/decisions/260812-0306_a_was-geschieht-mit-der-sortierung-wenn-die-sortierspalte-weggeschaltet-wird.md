# Was geschieht mit der Sortierung, wenn die Spalte weggeschaltet wird, nach der sortiert ist?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator (Klaerungsrunde bei der Aktivierung)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`, `crates/krk-ui/src/appkit/tabelle.rs:180` (`Spalte`)

---

## Question

Die einzige Stelle, an der ein Spaltenschalter mehr tut als etwas zu verbergen. Wer nach Groesse
sortiert und dann die Spalte Groesse wegschaltet, sieht eine Reihenfolge, deren Schluessel nicht
mehr auf dem Schirm steht.

## Options

1. **Die Sortierung bleibt.** Das Wegschalten verbirgt die Anzeige, nicht die Ordnung.
2. **Die Sortierung faellt auf den Namen zurueck.**
3. **Die Spalte laesst sich nicht wegschalten, solange nach ihr sortiert ist.**

## Antwort

**Moeglichkeit 1: die Sortierung bleibt.** Ein Spaltenschalter verbirgt eine Spalte und tut
sonst nichts.

Moeglichkeit 2 aendert einen Zustand, den der Nutzer nicht angefasst hat, und sie ist nicht
umkehrbar: wer die Spalte wieder einschaltet, bekommt seine alte Sortierung nicht zurueck, weil
niemand sie gemerkt hat. Moeglichkeit 3 macht einen von drei Schaltern zeitweise unwirksam und
verlangt vom Nutzer, den Zusammenhang zwischen Sortierung und Schalter zu kennen, bevor er
klickt.

**Der Preis ist benannt und nicht verschwiegen:** solange die Sortierspalte verborgen ist, zeigt
nichts an, welche Ordnung gilt — der Sortierhinweis sitzt im Kopf der Spalte und geht mit ihr.
Die Ordnung ist damit nicht verloren, nur unsichtbar, und ein erneutes Einschalten der Spalte
zeigt sie wieder. Dass die Statuszeile den Sortierschluessel nicht fuehrt, ist eine Beobachtung
ueber C1 der Runde 1 und keine Folge dieser Antwort; wer den Hinweis dort haben will, stellt eine
eigene Frage.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
