Die fünf beantworteten Entscheidungsdatensätze tragen zwei `Answered:`-Zeilen und einen veralteten Kopf

---

Alle fünf Datensätze unter `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/`, die am 260808-0017 von `_o_` auf `_a_` gewandert sind, haben zwei Fehler derselben Herkunft:

1. **Zwei `Answered:`-Zeilen je Datei.** Der leere Vorlagenblock aus der Anlage steht noch da, und darunter steht ein zweiter Block mit der gefüllten Zeile. Wer `grep -m1 '^Answered:'` fährt, bekommt die leere.
2. **`**Status:**` im Kopf steht auf `open`**, während der Dateiname `_a_` trägt und die gefüllte Zeile die Antwort führt. Die Wertliste in `rules/fusion-workbench-conventions.md`, Abschnitt `## Decision Record Template`, sieht zum Marker `_a_` den Wert `answered` vor.

Betroffen sind alle fünf:

| Datei | leere Zeile | gefüllte Zeile |
|---|---|---|
| `260807-2147_a_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md` | 51 | 57 |
| `260807-2147_a_traegt-eine-textmarke-auch-einen-bereich-oder-nur-eine-stelle.md` | 51 | 57 |
| `260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md` | 49 | 55 |
| `260807-2147_a_wie-greift-die-nachfrage-bei-der-sitzungssicherung.md` | 49 | 55 |
| `260807-2147_a_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md` | 53 | 59 |

---

## Warum das zählt

Der Zustand einer Frage steht an drei Orten: im Marker des Dateinamens, in der Kopfzeile `**Status:**` und in der Annotationszeile am Fuß. Zwei der drei widersprechen sich hier dem dritten. Ein Reconciler, der über die Kopfzeile geht, hält alle fünf Fragen für offen und stellt sie erneut; einer, der über die erste `Answered:`-Zeile geht, findet eine leere und kommt zum selben Schluss. Der Spec dieser Runde zitiert alle fünf als beantwortet, und die Abnahme steht darauf.

Derselbe Fehler in kleinerer Form liegt bereits im Circle-Datensatz vor (`issues/260807-2147_o_der-circle-datensatz-steht-auf-t-und-nennt-sich-im-kopf-anticipated.md`). Beide Male hat ein Übergang den Dateinamen mitgezogen und den Kopf nicht.

## Was zu tun ist

Je Datei den leeren Vorlagenblock samt seinem `---` entfernen, so dass eine `Answered:`-Zeile stehen bleibt, und `**Status:**` von `open` auf `answered` setzen. Sonst nichts; der Fragetext, die Möglichkeiten und die gefüllte Antwortzeile bleiben unverändert.

Der Shaper hat den Defekt beim Lesen der fünf Datensätze für die Spec-Überarbeitung gefunden und ihn nicht selbst behoben. Er hat die Datensätze angelegt, die Antworten hat der Orchestrator eingetragen, und der Shaper greift außerhalb des portfolio-activation-Modus in keinen fremden Eintrag ein.

**Aufgefallen bei:** Spec-Überarbeitung nach den sechs Festlegungen der Spec-Runde am 260808-0021.

---
Resolved: Am 260808-0043 behoben, der Defekt war seither veraltet. Alle fünf Datensätze tragen genau eine `Answered:`-Zeile, der leere Vorlagenblock ist entfernt, und der Kopf steht auf `**Status:** answered`. Nachgemessen am 260809-1750: `grep -c '^Answered:'` liefert für jeden der fünf 1, und alle acht Datensätze des Circles melden `**Status:** answered`.
