Fünf Verweise zeigen nach dem Zug der Plandokumente ins Leere

---

Der Zug vom 260909 hat vierzehn Anforderungsdokumente und Pläne von `_o_`/`_p_` auf `_c_`
gezogen. Fünf lebende Verweise nennen die alten Namen mit ausgeschriebenem Marker und zeigen
seitdem auf nichts. Alle fünf verstoßen daneben gegen die Zitierform: sie schreiben den
Speicherpfad aus, statt den Dateinamen ohne Speicher mit `_*_` an der Markerstelle zu nennen.

---

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Gefunden:** vom Abgleichlauf des 260909-1021, gemeldet und dort bewusst nicht behoben.

## Die fünf Stellen

| Datei | Zeile | genannter Name | steht heute auf |
|---|---|---|---|
| `circles/260823-2208-…/_b_circle.md` | 6 | `260824-0640_p_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` | `_c_` |
| `circles/260816-1321-…/_b_circle.md` | 7 | `260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` | `_c_` |
| `circles/260816-1321-…/_b_circle.md` | 38 | dieselbe Datei | `_c_` |
| `circles/260816-1321-…/_b_circle.md` | 64 | dieselbe Datei | `_c_` |
| `circles/260816-1321-…/planning/260816-1359_c_plan-inhaltsfilter-der-dateiliste.md` | 5 | dieselbe Datei | `_c_` |

Ein sechster Fund liegt unter `messungen/` und fällt unter die Ortsregel: eine Aufzeichnung
behält ihren damaligen Marker.

## Zwei Hälften mit verschiedenem Stand

**Die fünfte Stelle ist von der Antwort des Nutzers gedeckt.** Sie steht in einem Plan einer
geschlossenen Runde, und der Nutzer hat am 260909 entschieden, dass die Zustandsregel auf
alle Anforderungsdokumente und Pläne des Bestands greift. Ein Verweis darin ist damit
berichtigbar.

**Die vier Stellen in den zwei Runden-Datensätzen sind es nicht.** Ein Runden-Datensatz mit
Abschlussmarke ist Aufzeichnung, und die Antwort vom 260909 spricht von
Anforderungsdokumenten und Plänen, nicht von Runden-Datensätzen. Ob sie berichtigt werden,
ist eine offene Nutzerfrage und wird hier nicht entschieden.

## Was daran zählt

Das Feld `**Active spec/plan:**` hat zwei maschinelle Leser: die Erzeugung der
Rundenübersicht und die Wiederaufnahme einer unterbrochenen Sitzung. Beide lösen den Namen
über eine Suche auf und finden nichts. Kein Leser meldet das; sie liefern eine leere Auskunft.

## Abnahme

Die fünfte Stelle nennt den Dateinamen ohne Speicherpfad und mit `_*_` an der Markerstelle.
Für die vier übrigen liegt eine Nutzerantwort vor, oder dieser Datensatz nennt sie als
bewusst stehen gelassen mit dem Grund.
