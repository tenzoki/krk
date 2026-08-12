# Ueberstehen die Spaltenschalter einen Neustart?

---
**Domain:** code
**Status:** answered
**Filed by:** orchestrator (Klaerungsrunde bei der Aktivierung)
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260811-1732_*_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`, `crates/krk-ui/src/appkit/tabelle.rs:180` (`Spalte`)

---

## Question

C7 der Runde 1 verlangt, dass Tabs, Ordner, Auswahl, Breiten, Sichtbarkeit und Sortierung einen
Neustart ueberleben. Die Spaltensichtbarkeit steht dort nicht, weil es sie noch nicht gab.

## Options

1. **Ja, `Sitzung` waechst um ein Feld.**
2. **Nein, bei jedem Start stehen alle vier Spalten.**

## Antwort

**Ja.** `Sitzung` bekommt ein Feld `spalten: Spaltensichtbarkeit` mit drei `bool` fuer Groesse,
Geaendert und Typ; die Vorgabe ist dreimal `true`, also der heutige Zustand.

Eine Einstellung, die der Nutzer je Sitzung neu treffen muesste, ist keine Einstellung. Alles
andere in dieser Aufzaehlung ueberlebt den Neustart, und eine Ausnahme braeuchte einen Grund, den
es hier nicht gibt.

**Zur Vertraeglichkeit:** `Sitzung` traegt `#[serde(default)]`, eine `session.toml` aus der Zeit
davor bleibt also lesbar und nimmt die Vorgabe an. Die Probe dafuer gehoert nach
`crates/krk-core/tests/ablage.rs`, wo die entsprechende Probe der Editor-Runde schon steht.

**Die Spalte Name traegt keinen Schalter.** Der Nutzer nennt drei von vier, und eine Dateiliste
ohne Namensspalte zeigt nichts, was sie identifiziert. Das Feld fuehrt deshalb drei `bool` und
nicht vier.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
