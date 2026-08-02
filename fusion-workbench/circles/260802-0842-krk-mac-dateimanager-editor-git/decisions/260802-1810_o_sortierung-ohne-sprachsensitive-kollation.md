# Sortiert KRK Dateinamen sprachsensitiv, und wonach ordnet "Sortierung nach Typ"?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (gemeldet vom coder bei der Umsetzung von Schritt 2)
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Schritt 2 und Abschnitt `## Datenstrukturen`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C1, `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1803-verzeichnisleser-und-ordnermodell.md`

---

## Question

Der Plan verlangt in Schritt 2 vier Sortierungen, jeweils auf- und absteigend: nach Name, Größe, Änderungsdatum und Typ. Zwei davon lässt er unbestimmt, und der `coder` hat für beide begründet vorbelegt, statt eigenmächtig zu entscheiden. Beide Vorbelegungen sind sichtbar und sollten bestätigt oder geändert werden, bevor mehr darauf aufbaut.

**Die Namenssortierung ordnet ohne sprachsensitive Kollation.** Der Sortierschlüssel ignoriert Groß- und Kleinschreibung, vergleicht aber Zeichen für Zeichen nach Unicode-Position. Damit landet `Äpfel` hinter `Zebra`, weil `Ä` in der Unicode-Tabelle nach `Z` steht. Für eine Anwendung mit deutschsprachiger Oberfläche ist das im Alltag sichtbar: jeder Ordner mit Umlauten sortiert an einer Stelle, an der ihn niemand sucht. Der Finder sortiert `Äpfel` vor `Bäume`.

**Die Typsortierung ordnet nach der Aufzählung Ordner, Datei, Verknüpfung.** Da Ordner in jeder Sortierung ohnehin vorne stehen, unterscheidet sie innerhalb der Dateien nur noch Datei von Verknüpfung, und das ist für die meisten Ordner gar keine Unterscheidung. Üblich und vom Nutzer vermutlich erwartet wäre eine Ordnung nach Dateiendung. Der Grund für die Vorbelegung: `Eintrag` trägt im Abschnitt `## Datenstrukturen` kein Feld für die Endung; sie nachzurüsten ist eine Änderung an der Datenstruktur, kein Detail der Sortierfunktion.

Die Frage muss vor Schritt 12 beantwortet sein, weil die Sortierung dort in der Oberfläche sichtbar wird und Nutzererwartungen weckt. Sie blockiert die Schritte 3 bis 8 nicht.

## Options

1. **Beides jetzt richtigstellen** — sprachsensitive Kollation für die Namenssortierung, Endung als Feld in `Eintrag` und als Schlüssel der Typsortierung.
   - Pro: die Sortierung entspricht dann dem, was ein Mac-Nutzer vom Finder kennt, und niemand muss sich später umgewöhnen.
   - Cons: die Kollation ist keine Kleinigkeit. Rusts Standardbibliothek bringt sie nicht mit; entweder kommt eine Abhängigkeit dazu, oder KRK ruft über `objc2-foundation` die Systemfunktion `localizedStandardCompare:` auf. Der zweite Weg passt zur Technologiewahl, kostet aber je Vergleich einen Fremdaufruf, und die Namenssortierung läuft über 100.000 Einträge in einer Zusage von 4 Sekunden. Ob das trägt, ist ungemessen.

2. **Nur die Kollation richtigstellen, Typsortierung lassen** — Umlaute sortieren richtig, "nach Typ" bleibt Ordner/Datei/Verknüpfung.
   - Pro: behebt den sichtbaren Alltagsfehler und lässt die Datenstruktur unberührt.
   - Cons: "Sortierung nach Typ" bleibt eine Funktion, die im Alltag fast nichts tut.

3. **Beides in dieser Runde so lassen, mit Vermerk** — die jetzige Vorbelegung bleibt, die Frage wird für eine spätere Runde festgehalten.
   - Pro: kein Eingriff in eine Schicht, die gerade als Grundlage der Messung dient. Schritt 8 misst unter anderem das vollständige Lesen und Sortieren; eine Kollation über Fremdaufrufe verändert genau diese Messung, bevor überhaupt eine Vergleichszahl vorliegt.
   - Cons: die erste sichtbare Fassung sortiert Umlaute falsch, und der Nutzer sieht das sofort.

## Constraints

- Die Zusagen L3 und L10 aus C8 gelten für das vollständige Lesen **und Sortieren**: 10.000 Einträge in 400 ms warm, 100.000 in 4 s. Jede Änderung an der Sortierung wirkt unmittelbar auf beide.
- Die Maxime "supersimpel" wirkt als Ausschlussgrund. Zwei Sortierpfade nebeneinander, einer schnell und einer sprachsensitiv, wären eine Fallunterscheidung ohne Gegenwert.
- Der Plan hält in Schritt 8 fest, dass eine verfehlte Zusage nicht repariert, sondern als Entscheidungsvorlage zurückgemeldet wird. Eine Änderung an der Sortierung vor der Erstmessung nimmt dieser Regel die Vergleichsgrundlage.

## Recommendation

**Möglichkeit 3 bis zur Messung, danach Möglichkeit 1 oder 2 entscheiden.**

Der Grund liegt in der Reihenfolge, nicht in der Sache. Die Kollation gehört inhaltlich in eine deutschsprachige Anwendung, das ist unstrittig. Aber Schritt 8 ist als Gate gebaut: er misst, ob Rust mit AppKit die zugesagten Zahlen trägt, und bei einer verfehlten Zahl steht der Technologieentscheid zur Debatte. Wird die Sortierung vorher auf Fremdaufrufe umgestellt, misst dieses Gate eine andere Sache, und ein verfehlter Wert ließe sich nicht mehr zuordnen: liegt es an der Technologiewahl oder an der Kollation?

Nach der Messung liegt eine Vergleichszahl vor, und die Kosten der Kollation lassen sich gegen sie halten statt gegen eine Vermutung.

Diese Abwägung stützt sich auf den Aufbau des Plans, nicht auf eine Messung der Kollationskosten. Die Entscheidung liegt beim Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:
