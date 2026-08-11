# Orchestrator Session — 260811-0107

**Directive:** Die geltende Tastenbelegung als Markdown-Datei im Downloads-Ordner.
**Mode:** custom (Circle-Aktivierung)
**Status:** In Arbeit

## Aktivierung

Der Circle `260809-2040-tastenbelegung-als-markdown-in-downloads` ist am 260811-0107 auf
ausdrückliche Wahl des Nutzers aktiviert worden: der Datensatz ging von `_a_` auf `_t_`, und
`fusion-workbench/.active-circle` trägt den Verzeichnisnamen. Damit zeigen alle `OUT_*` in den
Circle; die `SCAN_*` decken den Circle und den gemeinsamen Speicher ab.

**Kein Plane-Push.** `plane.config.yaml` ist die unveränderte Vorlage (`plane.example.com`,
`your-workspace-slug`, Null-UUID), und `$PLANE_API_KEY` ist nicht gesetzt. Der Mirror ist damit
nicht eingerichtet, und der Push entfällt — nicht weil er fehlschlug, sondern weil es nichts zu
spiegeln gibt.

## Vorabprüfung zur fünften Frage

Der Datensatz
`decisions/260809-2040_*_welche-belegung-schreibt-die-ausgabe-bei-offener-belegungsansicht.md`
trägt eine ausdrückliche Aufforderung: ob ein Menüeintrag bei stehendem Blatt noch anschlägt, sei
nicht gemessen und vor der Antwort nachzusehen. Nachgesehen am 260811-0107:

- Die Belegungsansicht wird über `beginSheetModalForWindow_completionHandler`
  (`crates/krk-ui/src/appkit/blaetter/mod.rs:508`) gezeigt, also **dokumentmodal** und nicht über
  `runModal`. Eine eigene Ereignisschleife bringt sie nicht mit.
- Eine eigene `validateMenuItem`-Überschreibung gibt es im ganzen Baum nicht; die Suche liefert
  null Treffer.

`inference:`, nicht gemessen: ein dokumentmodales Blatt lässt die Menüleiste bedienbar, und ohne
eigene Prüfung der Menüeinträge schlägt ein solcher Eintrag an. Damit ist die fünfte Frage beim
Menüweg **nicht** gegenstandslos, sondern zu beantworten. Beim Belegungsweg bleibt sie es, weil
der Ereignisabgriff bei stehendem Blatt allein `abbrechen` durchlässt.

## Verlauf

- 260811-0107 — Circle aktiviert, Pfade neu aufgelöst, fünf offene Fragen dem Nutzer vorgelegt.
