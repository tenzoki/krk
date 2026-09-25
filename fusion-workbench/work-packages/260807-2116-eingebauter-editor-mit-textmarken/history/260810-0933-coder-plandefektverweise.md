# Die drei gekürzten Defektverweise im Plan tragen den vollen Dateinamen

**Datum:** 2026-08-10, 09:33
**Status:** Complete
**Agent:** `coder`
**Auftrag:** Behebung von `issues/260810-0918_o_der-plan-zitiert-einen-defekt-mit-einem-zeitstempel-den-sechs-datensaetze-tragen.md`

## Was geändert ist

Eine Stelle in einer Datei: `planning/260808-0140_c_plan-eingebauter-editor-mit-textmarken.md`, Zeile 1353, Abschnitt `### Wie diese sechs Schritte geschnitten sind`. Die Klammer hinter „weil der Plan nach Sachthema schnitt statt nach Übersetzbarkeit" trug drei Verweise in der Form `issues/260808-0931_c_...`; sie trägt jetzt drei vollständige Dateinamen mit der Sternstelle statt des Zustandsmarkers:

- `issues/260808-0931_*_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md`
- `issues/260809-1640_*_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`
- `issues/260808-1413_*_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`

Die Sternstelle erledigt zugleich den Nebenbefund des Datensatzes: der Plan legt unter `## Wie dieser Plan auf Datensätze verweist` fest, dass ein Verweis den Zustandsmarker nicht trägt, und die drei Kürzungen brachen diese Regel.

## Wie die Mehrdeutigkeit entschieden ist

Den Zeitstempel `260808-1413` tragen sechs Datensätze; der Datensatz nennt zwei davon als inhaltlich denkbar. Entschieden ist es an zwei Belegen aus dem Bestand:

1. `260808-1413_c_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md` sagt in seiner eigenen Abschlussnotiz (Zeilen 74 bis 77), dass `### Wie diese sechs Schritte geschnitten sind` ihn „als einen von drei Anlässen" zitiert, und führt die Klausel „keiner hinterlässt eine Zeile, die auf ihren Ablöser wartet" wörtlich an. Ein `grep` nach „sechs Schritte", „Übersetzbarkeit" und „Uebersetzbarkeit" über alle sechs Datensätze und über die beiden eindeutigen findet Treffer nur in diesem einen.
2. Der zweite Kandidat, `260808-1413_c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md`, behauptet nirgends, ein Schritt habe Stellen außerhalb seines Umfangs mitziehen müssen. Sein Gegenstand sind zwei Antworten auf die Frage, ob der Editor im Fenster steht, aufgelöst über den gemeinsamen Ausdruck `steht_im`. Das Vorziehen von S19 steht in `260808-0931`, und der ist in derselben Klammer schon eigens zitiert; ein zweiter Verweis darauf hätte die drei Anlässe auf zwei verkürzt.

Der Marker stützt dasselbe Ergebnis: das Zitat schrieb `_o_`, und von den beiden Kandidaten stand nur `vier-platzhalter` zu diesem Zeitpunkt auf offen.

## Was bewusst nicht angefasst ist

- **Zeile 716** (Umsetzungsvermerk zu S15) zitiert `issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...` in derselben gekürzten Form mit Zustandsmarker. Die Stelle liegt außerhalb des Abschnitts, den der Datensatz unter `**Betroffen:**` führt, und außerhalb des Auftrags. Sie ist eindeutig auflösbar und gehört in den nächsten Abgleich.
- Keine `[DONE]`-Marke, kein Schritt-Text jenseits der drei Verweise, keine Kopfzeile, kein Dateiname. Der Plan behält seinen Marker `_c_`.
- Nichts unter `crates/**` und nichts unter `resources/**`.

## Abnahme

Kein Übersetzer ist betroffen, weil keine Codedatei angefasst ist. Geprüft ist stattdessen, dass jeder eingesetzte Dateiname existiert:

```
ls issues/260808-0931_c_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md
ls issues/260809-1640_c_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md
ls issues/260808-1413_c_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md
```

Alle drei gefunden, Ausgang 0. Ein `grep` über die geänderte Zeile bestätigt die drei Verweise in der Sternform.

## Offen

Die Umbenennung des Markers des Datensatzes von `_o_` auf `_c_` macht der Nutzer.
