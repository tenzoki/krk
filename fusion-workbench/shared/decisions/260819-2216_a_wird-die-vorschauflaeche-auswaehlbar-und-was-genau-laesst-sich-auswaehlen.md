# Wird die Vorschaufläche auswählbar, und was genau lässt sich auswählen?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md` (der überholte Datensatz); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_*_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md` (C4.8 in Zeile 68, die Umsetzungszusage in Zeile 417); `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C1); `crates/krk-ui/src/appkit/vorschau.rs:1120-1121`; `crates/krk-ui/src/appkit/ereignisse.rs:685-701`

---

## Dieser Datensatz überholt einen anderen, und nur zur Hälfte

Der Datensatz vom 260812-1105 beantwortet zwei Fragen in einem Dokument. **Überholt wird allein die zweite**, ob die Vorschaufläche unauswählbar bleibt. Die erste, was ein Verweis im gerenderten Markdown tut, gilt unverändert weiter: ein Link bekommt Farbe und Unterstreichung, keine Klickwirkung und keinen Zeigefinger, und welche Quellen eine Adresse setzen dürfen, bleibt die erste offene Frage des Circles `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.

**Worauf die Umbenennung zeigen muss.** Der alte Datensatz trägt heute den Marker `_i_` und ist auf `_s_` umzubenennen, mit der Zeile

```
Superseded by: shared/decisions/260819-2216_o_wird-die-vorschauflaeche-auswaehlbar-und-was-genau-laesst-sich-auswaehlen.md — der Nutzer hat die Vorschaufläche am 260819-2210 auswählbar bestellt. Überholt ist allein die zweite Frage jenes Datensatzes; die Antwort auf den Verweis im gerenderten Markdown gilt unverändert weiter.
```

Der Marker wandert erst nach der Abnahme dieses Specs durch den Nutzer, und der Orchestrator setzt ihn. Der Shaper hat den alten Datensatz nicht angefasst.

## Question

Die Vorschaufläche ist seit der Runde 1 weder bearbeitbar noch auswählbar, und das war bis zum 260819 eine Bedingung und keine Nachlässigkeit: eine auswählbare Textfläche nimmt den Fokus als Textsystem, und der Ereignisabgriff reicht danach jede Taste an AppKit weiter, statt die Befehle von KRK auszuführen. Der Nutzer will Text aus der Vorschau kopieren und braucht dafür eine Auswahl. Zu entscheiden ist, **was auswählbar wird**: alles, was die Textfläche zeigt, oder ein Teil davon.

Die Textfläche zeigt sechs Sorten Inhalt: den rohen Text einer Datei, eingefärbten Quelltext, gerendertes Markdown, die Metadaten eines Eintrags, einen Hinweissatz und den Text aus der Zwischenablage. Daneben steht die Bildansicht, und von den beiden ist immer genau eine sichtbar.

## Options

1. **Alles, was die Textfläche zeigt, wird auswählbar; ein Bild nicht.**
   - Folge: ein Schalter kippt, und es entsteht keine Fallunterscheidung über den Inhaltswert. Der Nutzer kann auch aus den Metadaten und aus einem Hinweis kopieren, etwa einen Pfad oder eine Fehlermeldung.
   - Preis: die Auswahl steht auch dort, wo sie selten gebraucht wird.

2. **Allein der Text einer Datei wird auswählbar**, Metadaten und Hinweise nicht.
   - Folge: die Auswahl steht dort, wo der Nutzer sie verlangt hat.
   - Preis: eine Fallunterscheidung über den Inhaltswert an einer Stelle, die heute keine hat, und ein Verhalten, dessen Grund am Bündel nicht zu sehen ist. Ein Nutzer, der eine Fehlermeldung nicht markieren kann, hält es für einen Fehler.

3. **Die Fläche bleibt unauswählbar**, und das Kopieren geht über einen eigenen Befehl, der die ganze Datei nimmt.
   - Folge: der Ereignisabgriff bleibt unberührt, und es entsteht keine zweite angemeldete Fläche.
   - Preis: der Nutzer kann keine Stelle wählen. Ein Befehl, der immer alles kopiert, beantwortet den Wunsch nicht.

## Constraints

- Die Bildansicht bleibt außen vor; ein Bild wird nicht auswählbar.
- Die Textfläche bleibt nicht bearbeitbar. `setEditable(false)` bleibt stehen.
- Die vier Tabbefehle müssen mit dem Fokus in der Vorschau weiter wirken. Das verlangt die Anmeldung der Fläche in `ersthelfer_gehoert_appkit`, und sie ist damit die zweite angemeldete Fläche neben der des Editors.
- `Fokus::Vorschau` muss die Antwort bleiben, auch wenn der Ersthelferrang in der Textanzeige steht.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie kostet eine Zeile mehr als die dritte und eine Fallunterscheidung weniger als die zweite, und sie hält die Regel, die dieses Projekt an anderen Stellen gegen Ausnahmen verteidigt: eine Fläche verhält sich überall gleich.

## Antwort 260819-2210

**Möglichkeit 1.** Wörtlich: „Alles, was die Textfläche zeigt, Bilder nicht."

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented:
Deferred:
Superseded by:
