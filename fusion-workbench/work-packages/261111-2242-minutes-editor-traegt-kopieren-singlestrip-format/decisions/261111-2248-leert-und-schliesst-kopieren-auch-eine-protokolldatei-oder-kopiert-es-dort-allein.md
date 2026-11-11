# Leert und schließt „Kopieren“ auch eine Protokolldatei, oder kopiert es dort allein?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-2242-minutes-editor-traegt-kopieren-singlestrip-format.md, 261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md, 260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md

---

## Question

Der Auftrag verlangt, dass jede der drei Schaltflächen in einer Datei aus `minutes/` so wirkt wie in der Quicknote. Für Singlestrip und Format ist das eindeutig: dieselbe Umformung an der Auswahl, sonst am ganzen Text, mit `cmd+z` zurückzunehmen. „Kopieren“ tut in der Quicknote drei Dinge: es legt den ganzen Text in die Zwischenablage, leert den Puffer ohne Eintrag im Rückgängig und schließt die Quicknote (`Anwendungsdelegierter::quicknote_kopieren`, `Quicknote::nach_kopie_leeren`). Wörtlich übertragen leerte ein Klick ein Protokoll, und `cmd+z` holte es nicht zurück. Die Frage muss vor dem Codeumbau stehen, weil sie den Ausführungszweig, die Meldungen und die Proben bestimmt.

## Options

1. **Allein kopieren** — der ganze Text der Protokolldatei geht in die Zwischenablage, die Datei und der Editor bleiben, wie sie sind; die Statuszeile nennt die Zahl der Zeichen.
   - Pros: kein Verlust; ein Protokoll ist eine Aufzeichnung und kein Zwischenpuffer; dieselbe Taste (`shift+f10`) ist in beiden Lagen gefahrlos.
   - Cons: „Kopieren“ tut in den zwei Lagen nicht genau dasselbe; der Befehlsname „Quicknote kopieren und schließen“ beschreibt in einem Protokoll nur die erste Hälfte.
2. **Wörtlich wie in der Quicknote** — kopieren, den Text der Datei leeren, die Datei im Editor schließen.
   - Pros: eine Bedeutung der Schaltfläche in beiden Lagen.
   - Cons: leert ein Protokoll auf einen Klick; das Leeren müsste, anders als in der Quicknote, über den Rückgängigstapel des Editors gehen und bliebe bis zum Sichern ein ungesicherter Stand, der beim Schließen die Nachfrage auslöst.
3. **Kopieren und schließen, ohne zu leeren** — der Text geht in die Zwischenablage, die Datei bleibt unverändert und wird im Editor geschlossen.
   - Pros: näher am Befehlsnamen, kein Verlust.
   - Cons: ein ungesicherter Stand hält das Schließen mit der Nachfrage an; wer kopiert, um anderswo einzufügen und weiterzuschreiben, muss die Datei neu öffnen.

## Constraints

Die Zwischenablage erreicht KRK allein über `zwischenablage::text_schreiben`. Ein Weg, der den Stand einer Datei ändert, geht durch `Editorbereich::umbau_anwenden` und meldet eine Handlung im Rückgängigstapel an. Die Quicknote selbst bleibt, wie sie ist.

## Recommendation

Möglichkeit 1. Der Plan ist darauf geschrieben (Entscheidung E7 dort). Leeren und Schließen gehören zur Quicknote, weil ihr Puffer ein Durchgangsort ist; ein Protokoll soll nach dem Kopieren noch dastehen. Eine andere Antwort ändert allein Schritt 5 des Plans und die Sätze in Schritt 2 und 7.
