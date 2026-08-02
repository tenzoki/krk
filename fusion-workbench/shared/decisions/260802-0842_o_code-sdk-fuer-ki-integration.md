# Welches Code-SDK trägt die spätere KI-Anbindung in KRK?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`, `idea.txt`

---

## Question

Der Entwurf in `idea.txt` nennt als spätere Ausbaustufe die "Integration eines Code-SDKs (o. Ä.), um AI zu integrieren", vor allem für Tool Use und Coding, daneben für Analyse und Textverfassung. Welches SDK gemeint ist, steht dort nicht; der Nutzer hat in seiner Anmerkung selbst offengelassen, ob an dieser Stelle ein Tippfehler steht oder ein bestimmtes Produkt gemeint war, etwa das Claude Code SDK. Auf die Frage in der Klärungsrunde hat er geantwortet, die Entscheidung falle später.

Die KI-Anbindung liegt vollständig außerhalb des Circles `260802-0842-krk-mac-dateimanager-editor-git`. Der Datensatz hält die Frage fest, damit sie nicht verloren geht, und ist bewusst nicht an diesen Circle gebunden.

## Options

Die Optionen sind noch nicht ausgearbeitet. Eine belastbare Gegenüberstellung setzt voraus, dass die technische Grundlage von KRK steht, weil die Sprache und das UI-Toolkit der Anwendung mitbestimmen, welche SDKs überhaupt in Frage kommen. Zum Zeitpunkt der Ablage existiert im Projekt noch kein Code.

Zwei Punkte sind vorab zu klären, sobald die Frage aufgerufen wird:

1. Ob "Code-SDK" ein konkretes Produkt meint oder allgemein eine Bibliothek für den Zugriff auf ein Sprachmodell mit Werkzeugaufrufen.
2. Ob die KI-Anbindung in KRK selbst läuft oder ein bestehendes Werkzeug außerhalb ansteuert.

## Constraints

- Die Antwort darf den Umfang des Circles `260802-0842-krk-mac-dateimanager-editor-git` nicht erweitern. KI-Anbindung jeder Art bleibt dort außen vor.
- Die weitere Ausbaustufe aus dem Entwurf, KRK als Kommandozentrale für Fusion, hängt an derselben Frage und sollte gemeinsam mit ihr betrachtet werden.

## Recommendation

Keine. Der Shaper hat zu dieser Frage noch keine belastbare Grundlage, weil das Projekt keinen Code enthält und die technische Basis von KRK offen ist. Die Frage sollte erst aufgerufen werden, wenn der Navigator, der Editor und die Git-Anbindung stehen.

---
Answered:
Implemented:
Deferred:
Superseded by:
