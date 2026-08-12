# Gelten die Spaltenschalter fuer beide Dateifenster gemeinsam oder je Seite?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator (Klaerungsrunde bei der Aktivierung)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`, `crates/krk-ui/src/appkit/tabelle.rs:180` (`Spalte`)

---

## Question

Die Breiten und die Sichtbarkeit der Bereiche werden je Bereich gefuehrt, die Spalten heute gar
nicht. Der Nachtrag des Nutzers sagt nicht, ob ein Spaltenschalter beide Dateifenster trifft oder
nur eines.

## Options

1. **Gemeinsam.** Ein Schalter, beide Listen zeigen dieselben Spalten.
2. **Je Seite.** Sechs Schalter statt drei, oder drei Schalter, die auf das aktive Dateifenster
   wirken.

## Antwort

**Gemeinsam.** Drei Schalter fuer beide Listen.

Drei Gruende. Die Leiste steht **einmal** ueber die volle Fensterbreite und nicht je Dateifenster;
Schalter darin, die nur eine Seite treffen, muessten anzeigen, welche, und dafuer gibt es in einer
Leiste ohne Seitenbezug keinen Platz. Sechs Schalter neben den fuenf Bereichsschaltern waeren elf
Bedienelemente in einer 18 Punkte hohen Zeile. Und die Maxime "supersimpel" spricht gegen eine
Unterscheidung, nach der der Nutzer nicht gefragt hat.

**Der Preis, benannt:** wer links nach Groesse und rechts nach Namen arbeiten will, bekommt in
beiden Listen dieselben Spalten. Eine spaetere Runde kann die Unterscheidung nachtragen, ohne dass
etwas umgebaut werden muesste: aus einem Feld werden zwei.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
