# Passt der Filter auf den Namensanfang oder auf jede Stelle des Namens?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `crates/krk-core/src/verzeichnis/sprungmarke.rs:119-127` (die heutige Regel, `starts_with`); `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md` (C2, sechstes Abnahmekriterium)

---

## Question

Der Nutzer hat entschieden, dass Tippen filtert statt zu springen, aber nicht, wonach der Filter vergleicht. Die abgelöste Sprungmarke verglich mit `starts_with` auf den Namensanfang, ohne Rücksicht auf Groß- und Kleinschreibung. Ein Filter, der diese Regel erbt, findet `notizen.md` unter `no` und nicht unter `zen`. Die Frage muss vor der Planung fallen, weil sie den ganzen Nutzen der Funktion bestimmt: bei einem Ordner mit vierzig Dateien namens `krk-2026-08-*.log` ist ein Filter auf den Namensanfang fast wirkungslos, während ein Filter auf jede Stelle des Namens genau dort trägt. Sie fällt außerdem doppelt ins Gewicht, weil dieselbe Regel in der tiefen Suche über einen ganzen Unterbaum läuft.

## Options

1. **Namensanfang, wie bisher** — der Filter erbt `starts_with` unverändert von der Sprungmarke.
   - Pro: kein Bruch mit dem Verhalten, das der Nutzer heute kennt; die Regel steht schon im Kern und wird nur an einer zweiten Stelle gelesen; die schnellste Prüfung von dreien.
   - Kontra: findet die Dateien nicht, bei denen ein Filter am meisten hilft, nämlich lange Namen mit gemeinsamem Anfang. Der Nutzen bliebe hinter dem zurück, was der Nutzer sich vom Filtern verspricht.
2. **Jede Stelle des Namens** — der Filter nimmt jeden Eintrag, dessen Name die getippte Folge irgendwo enthält.
   - Pro: das Verhalten, das Nutzer von Suchfeldern erwarten; trägt bei langen Namen mit gemeinsamem Anfang; eine Regel für die flache und die tiefe Suche.
   - Kontra: liefert bei kurzen Eingaben mehr Treffer als der Anfangsvergleich; die Reihenfolge der Treffer sagt nicht mehr, wie gut sie passen.
3. **Anfang zuerst, Rest darunter** — beide Vergleiche laufen, und die Treffer am Namensanfang stehen oben.
   - Pro: verbindet die Trefferzahl von Möglichkeit 2 mit der Ordnung von Möglichkeit 1.
   - Kontra: bricht die Sortierung, die der Nutzer über C2 eingestellt hat, weil die Liste dann nach Passgenauigkeit und nicht nach Name, Größe, Datum oder Typ geordnet ist. Zwei Ordnungen in einer Liste sind ein eigenes Erklärungsproblem.

## Constraints

- Groß- und Kleinschreibung bleibt unerheblich, wie bei der Sprungmarke heute und bei der Tippsuche der Belegungsansicht aus der Runde 7. Das ist keine der drei Möglichkeiten, sondern gilt für alle.
- Die Zeichenregel bleibt `krk_core::verzeichnis::sprungmarke::traegt_ein_dateiname`, die zweite Nutzer hat. Eine zweite Zeichenregel entsteht nicht.
- Die eingestellte Sortierung aus C2 der Runde 1 bleibt die Ordnung der Liste, außer die Antwort ist ausdrücklich Möglichkeit 3.
- Ob Umlaute und Akzente beim Vergleich gefaltet werden, also ob `apfel` auch `Äpfel` findet, gehört zur Antwort dazu. Die Namenssortierung faltet heute nicht, sondern ordnet sprachsensitiv über einen Kollationsschlüssel.

## Recommendation

Möglichkeit 2. Der Nutzer hat das Tippen vom Springen auf das Filtern umgestellt, und die Begründung dafür trägt nur, wenn der Filter mehr findet als der Sprung. Möglichkeit 1 wäre derselbe Vergleich mit einer anderen Anzeige und rechtfertigte die Ersetzung eines abgenommenen Kriteriums nicht. Möglichkeit 3 kauft ihre Ordnung damit, dass sie die vom Nutzer gewählte Sortierung außer Kraft setzt, und das ist teurer als der Gewinn.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1610, und schon in seinem ersten Satz am 260814-1520 — jede Stelle des Namens. Woertlich: "In jedem Fall muss die Suche Substrings suchen (\"aaa\" match \"bbbaaaccc\")". Deckt sich mit der Empfehlung. Der Vergleich ist eine Teilzeichenfolge ohne Ruecksicht auf Gross- und Kleinschreibung, wie ihn `krk-ui/src/belegungsmodell.rs:536-541` fuer die Belegungsansicht schon fuehrt.
