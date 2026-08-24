# Gilt ein Profil nur für Ordner, oder auch für einzelne Dateien?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`, `krk-ui/src/vorschaumodell.rs`

---

## Question

Der Backlogeintrag nennt im Titel Ordner **und** Dateien, doch jede der sechs skizzierten Zusammenfassungen gehört zu einem Ordner: die Wurzel der fusion-workbench, ein einzelner Speicher wie `shared/analyses/`, `issues/`, `circles/` und ein einzelnes Circle-Verzeichnis. Offen bleibt, ob ein Profil auch für eine einzelne Datei eine Zusammenfassung festlegen darf, etwa für `_t_circle.md` die Directive und die Kopffelder statt der zweihundert Zeilen Markdown. Die Antwort entscheidet, woran die Ortserkennung hängt, und sie entscheidet mit, ob eine getroffene Datei ihre heutige Textvorschau verliert. Sie muss vor dem Spec fallen, weil sie den Zuschnitt der Erkennungsregel bestimmt.

## Options

1. **Nur Ordner** — Ein Profil ersetzt die Metadatenanzeige eines Ordners. Dateien bleiben bei der Dreiteilung aus C6 der Runde 1: Text bis 1 MB, Bild bis 64 MB, sonst Metadaten.
   - Pros: Der Zuschnitt trifft jede der sechs skizzierten Zusammenfassungen. Die bestehende Dateianzeige bleibt unberührt, und die Runde braucht keine zweite Anzeigelage je Datei.
   - Cons: Gerade die Datei, deren Kopf man am häufigsten liest, das Circle-Protokoll, bleibt Rohtext in der Vorschau.
2. **Ordner und Dateien** — Ein Profil darf auch für eine einzelne Datei eine Zusammenfassung festlegen.
   - Pros: Ein Circle-Protokoll zeigte Directive und Kopffelder auf einen Blick, ohne dass der Nutzer scrollt.
   - Cons: Eine getroffene Datei verlöre ihre Textvorschau, und die Runde bräuchte einen Rückweg zum Rohtext, also eine zweite Anzeigelage je Datei und einen Befehl dafür. Der Rohtext steht ohnehin im eingebauten Editor bereit.

## Constraints

Die Anzeigezweige der Vorschau sind heute drei und ohne Auffangzweig (`Inhalt` in `krk-ui/src/vorschaumodell.rs`). Jede Antwort muss die Fallunterscheidung vollständig lassen. Die Zeitzusage L7 gilt für die Textvorschau einer Datei bis 1 MB mit 100 ms; eine Zusammenfassung an der Stelle einer Textdatei arbeitet innerhalb dieser Zusage und nicht daneben.

## Recommendation

Möglichkeit 1. Alle sechs skizzierten Fälle sind Ordnerzusammenfassungen, und der Rohtext einer Datei ist über den eingebauten Editor bereits erreichbar. Die Dateifrage lässt sich später als eigene Runde stellen, ohne dass an der Erkennungsregel etwas zurückgebaut werden müsste.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:51 — Nur Ordner (Möglichkeit 1); Dateien bleiben bei der Dreiteilung aus C6 der Runde 1.
