# Erzwingt `krk-ui` die `unsafe`-Grenze mit `deny` oder beobachtet sie sie nur mit `warn`?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-1200_o_abnahmekriterium-von-schritt-6-traegt-denselben-grep-fehler.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Abschnitt `## Aufbau` sowie die Schritte S1 und S6 und die Risikotabelle, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`

---

## Question

Der Plan sagt für die Kiste `krk-ui`, also das Binärziel mit dem AppKit-Anteil, die Übersetzerregel `#![warn(unsafe_code)]` zu, und für die Kiste `krk-core`, also den Kern ohne AppKit, die Regel `#![deny(unsafe_code)]`. Beide Regeln stehen im selben Absatz des Abschnitts `## Aufbau` unter der Überschrift "Durchgesetzt wird die Grenze über zwei Übersetzerregeln". Der Unterschied zwischen ihnen ist kein Feinschliff: `deny` bricht den Bau ab, `warn` schreibt eine Zeile in die Ausgabe und baut weiter.

Aufgefallen ist die Frage bei der Behebung zweier Plandefekte am 260803-1200. Das Abnahmekriterium von Schritt S6 verlangt, ein `grep` nach dem Wort `unsafe` über `crates/krk-ui/src` nenne ausschließlich Dateien unterhalb von `src/appkit/`. Das kann nicht aufgehen, weil `crates/krk-ui/src/main.rs` die Zeile mit der Übersetzerregel selbst trägt und damit die gesuchte Zeichenkette enthält. Für die Schritte S2 und S15 in `krk-core` ist derselbe Fehler bereits behoben, und zwar über eine Prüfung auf das Attribut `#[allow(unsafe_code)]` am Zeilenanfang. Diese Auflösung trägt in `krk-core` nur deshalb, weil der Bau die andere Hälfte des Belegs übernimmt: `deny` lässt ihn scheitern, sobald `unsafe` außerhalb der einen Datei mit der Ausnahme steht. In `krk-ui` fehlt diese Hälfte, solange dort `warn` steht.

Die Frage muss vor der Abnahme von S6 beantwortet sein, weil S6 der Schritt ist, der das Modul `appkit` mit seiner Ausnahme anlegt und damit die Grenze zum ersten Mal überhaupt hat. Sie hätte auch bei der Behebung des Defekts entschieden werden können, gehört aber nicht dorthin: sie hebt eine Festlegung auf, die an drei Stellen des Plans steht.

Ein Nebenbefund gehört zur Frage, weil er die Begründungslage schief stehen lässt. Der Plan begründet für `krk-core` ausführlich, warum dort `deny` und **nicht** `forbid` steht: der Kern braucht die beiden Systemaufrufe `getattrlistbulk` und `copyfile`, `forbid` ließe sich für sie nicht öffnen, und eine eigene Kiste für zwei Funktionen wäre die teurere Antwort. Zur `warn`-Wahl in `krk-ui` sagt der Plan an keiner Stelle etwas. Sie steht als Setzung da, an drei Orten wiederholt, nirgends hergeleitet.

## Options

1. **`krk-ui` trägt `#![deny(unsafe_code)]`, das Modul `appkit` trägt `#[allow(unsafe_code)]`** — dieselbe Bauform wie in `krk-core` mit seinem Modul `verzeichnis::sys`.
   - Pro: die Grenze ist maschinell erzwungen statt nur beobachtet. Ein `unsafe`-Block, der außerhalb von `src/appkit/` entsteht, lässt den Bau scheitern, statt eine Warnung zu erzeugen, die in einem längeren Bauprotokoll untergeht.
   - Pro: die Zusage der Risikotabelle, `unsafe` liege in genau zwei Modulen und sei "durchgesetzt über zwei Übersetzerregeln", wird für beide Kisten wahr. Unter `warn` gilt sie nur für `krk-core`, und die Tabelle sagt trotzdem "durchgesetzt".
   - Pro: das Abnahmekriterium von S6 kann wörtlich die Form der Kriterien von S2 und S15 übernehmen, angepasst auf `krk-ui`. Eine Prüfvorschrift für drei Schritte statt zweier Formen für dieselbe Sache.
   - Contra: die Regel muss geöffnet werden, sobald ein AppKit-Aufruf außerhalb von `src/appkit/` gebraucht wird. Genau das soll der Entwurf verhindern, und wo er es doch nicht kann, ist der abgebrochene Bau der gewollte Widerstand.
   - Contra: der Wechsel ändert eine Zeile in einer bereits umgesetzten Datei, `crates/krk-ui/src/main.rs`. Der Plantext von S1 weicht danach von dem ab, was der Commit zu S1 zeigt.

2. **`krk-ui` behält `#![warn(unsafe_code)]`, und das Abnahmekriterium von S6 prüft stattdessen den Code** — etwa über eine Suche nach `unsafe` als Sprachkonstrukt außerhalb von `src/appkit/`, in den Formen Block, Funktion, Implementierung und Fremdblock.
   - Pro: keine Änderung an einer umgesetzten Datei, keine Änderung an drei Plantextstellen.
   - Contra: die Prüfvorschrift muss vier Schreibweisen von `unsafe` fassen, und jede vergessene Form ist ein Loch. Der Plan hat diese Unvollständigkeit für `krk-core` bereits einmal festgestellt und die Attributprüfung gerade deshalb gewählt.
   - Contra: die Grenze bleibt beobachtbar statt erzwungen. Zwei Kisten, zwei Strenge-Grade, ohne dass der Unterschied im Entwurf einen Grund hätte.
   - Contra: die Setzung bliebe unbegründet. Wer den Plan liest, findet für `krk-core` eine ausformulierte Abwägung zwischen `deny` und `forbid` und für `krk-ui` nichts.

3. **`krk-ui` trägt `#![forbid(unsafe_code)]`, und die AppKit-Hüllen ziehen in eine eigene Kiste** — die Grenze wird zur Kistengrenze.
   - Pro: die schärfste Form. `forbid` lässt sich nicht öffnen, und die Trennung wäre am Bauzuschnitt ablesbar statt an einem Attribut.
   - Contra: eine fünfte Kiste für den Anteil, der ohnehin schon in einem eigenen Modul liegt. Der Plan hat dieselbe Abwägung für `krk-core` geführt und dort gegen die eigene Kiste entschieden, mit derselben Begründung. Zwei Antworten auf dieselbe Frage wären die Sonderregel, die die Maxime "supersimpel" ausschließt.

## Constraints

- Das Modul `appkit` muss `unsafe` benutzen dürfen. Jeder AppKit-Aufruf über `objc2` ist ein unsicherer Fremdaufruf, und der Technologiedatensatz führt das als eine der drei dauerhaften Kosten des Entscheids.
- Die Antwort darf die Zusage des Entwurfs nicht antasten, dass `krk-core` AppKit-frei bleibt und ohne Fenster testbar ist. Sie betrifft allein, wie streng die Grenze in `krk-ui` durchgesetzt wird.
- Was auch immer gilt, muss in einem Abnahmekriterium prüfbar sein, das auf dem tatsächlichen Dateibestand aufgeht. Der Defekt, aus dem diese Frage stammt, ist genau daran gescheitert.
- **Der Entwurf ist von der Antwort nicht berührt.** Beide Möglichkeiten schreiben denselben Code; sie unterscheiden sich darin, was der Übersetzer tut, wenn jemand die Grenze überschreitet.

## Recommendation

**Wir empfehlen Möglichkeit 1.**

Der Grund liegt in der Zusage, die der Plan ohnehin schon macht. Die Risikotabelle nennt den unsicheren AppKit-Aufruf als Risiko und antwortet darauf mit "durchgesetzt über zwei Übersetzerregeln". Eine Warnung setzt nichts durch. Sie meldet, und ob die Meldung jemanden erreicht, hängt daran, ob jemand das Bauprotokoll liest. Unter `deny` hat der Satz einen maschinellen Träger, und die Zusage stimmt für beide Kisten statt für eine.

Der zweite Grund ist die Einheitlichkeit der Prüfvorschrift. Für `krk-core` steht sie seit dem 260803-1200 fest: das Attribut `#[allow(unsafe_code)]` am Zeilenanfang, gefunden in genau einer Datei, zusammen mit dem erfolgreichen Bau. Unter Möglichkeit 1 lässt sich diese Vorschrift auf `krk-ui` übertragen, ohne sie umzubauen. Unter Möglichkeit 2 bräuchte der Plan eine zweite Form für dieselbe Sache, und die zweite Form ist die schwächere.

Gegen Möglichkeit 1 spricht ein Punkt, den wir nicht kleinreden: die Umstellung ändert eine Zeile in `crates/krk-ui/src/main.rs`, und diese Datei stammt aus Schritt S1, der abgenommen ist. Der Plantext von S1 weicht danach von der Commit-Historie ab. Wir halten das für tragbar, sofern S1 den Umstand ausschreibt, damit ein späterer Leser den Unterschied nicht für einen Fehler hält. Der Codewechsel selbst gehört zur Umsetzung von S6, weil `deny` ohne das Modul `appkit` mit seiner Ausnahme nichts zu erlauben hat.

Die Abwägung stützt sich auf den Aufbau des Plans, nicht auf eine Messung. Die Entscheidung liegt beim Nutzer.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`:276 — **Möglichkeit 1: `krk-ui` trägt `#![deny(unsafe_code)]`, das Modul `appkit` trägt `#[allow(unsafe_code)]`.** Nutzerentscheidung am 260803.

Die Begründung des Nutzers deckt sich mit der Empfehlung und schärft sie an einem Punkt: eine Warnung bricht den Bau nicht ab, die Grenze wäre damit nur beobachtbar. Unter `deny` ist sie maschinell erzwungen, und die Zusage der Risikotabelle, `unsafe` sei "durchgesetzt über zwei Übersetzerregeln", wird für beide Kisten wahr statt nur für `krk-core`.

Eingearbeitet in den Plan an fünf Stellen: der Absatz über die zwei Übersetzerregeln in `## Aufbau` (`:276`, jetzt mit der Begründung der Wahl, die der Plan bisher nur für `deny` gegen `forbid` in `krk-core` führte), die Zeile zu `krk-ui` in der Verzeichnisstruktur (`:287`), die `Änderungen` und das `Abnahmekriterium` von S1, die `Änderungen` und das `Abnahmekriterium` von S6 sowie die Zeile der Risikotabelle zum unsicheren AppKit-Aufruf.

Nicht umgesetzt: `crates/krk-ui/src/main.rs` trägt weiterhin `#![warn(unsafe_code)]`. Der Codewechsel gehört zur Umsetzung von S6 und ist dort in den `Änderungen` festgehalten, weil `deny` ohne das Modul `appkit` mit seiner Ausnahme nichts zu erlauben hat. Erst der Commit, der ihn bringt, zieht diesen Datensatz auf `implemented`.
Implemented:
Deferred:
Superseded by:
