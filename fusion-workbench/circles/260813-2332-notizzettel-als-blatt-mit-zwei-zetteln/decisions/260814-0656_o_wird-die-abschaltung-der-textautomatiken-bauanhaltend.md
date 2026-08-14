# Wird die Abschaltung der Textautomatiken bauanhaltend, oder bleibt sie eine Gewohnheit?

---
**Domain:** code
**Status:** open
**Filed by:** planner (Plan der Runde 9)
**Cross-references:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` (C3, Festlegung „Diese Zusage ist die einzige der Runde, für die der Baum heute nicht von selbst redet"); `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md` (Schritte 9 und 16); `crates/krk-ui/src/appkit/editor.rs` (`textflaeche_bauen`, `EINSTELLUNGEN`); `shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` (dieselbe Frage an einer anderen Gewohnheit)

---

## Question

Die neunte Runde bringt die zweite bearbeitbare `NSTextView` in diesen Baum. Die Frage, welche Automatiken an einer solchen Fläche abgeschaltet gehören, ist einmal beantwortet, an der ersten, und der Plan zieht die Antwort in ein eigenes Modul, damit beide Flächen dieselbe Stelle rufen.

**Was der Baum nicht hält, ist der Aufruf.** Der Spec sagt es unter C3 selbst: `Datei::ALLE` und die Baumprobe zum atomaren Schreiben halten den Bau an, eine bearbeitbare Textfläche ohne abgeschaltete Automatiken übersetzt anstandslos. Wer in einer späteren Runde eine dritte Fläche baut und `automatiken_abschalten` vergisst, bekommt einen grünen Bau, grüne Proben und typografische Anführungszeichen in einer Datei des Nutzers.

Die Frage steht jetzt, weil sie mit der zweiten Fläche zum ersten Mal wirklich eine ist. Bei einer Fläche war „es gibt nur die eine, und sie tut es" eine Aussage, die man nachlesen konnte. Bei zwei ist es eine Regel, und Regeln in diesem Projekt halten den Bau an oder halten nicht.

Der Spec beantwortet sie ausdrücklich nicht, sondern schließt die Lücke über zwei Abnahmekriterien und erklärt sie damit zum Bestandteil der Abnahme und nicht des Plans. Das trägt für diese Runde. Es trägt nicht für die nächste, die eine Fläche hinzufügt, ohne dass jemand den Spec dieser Runde liest.

Es ist dieselbe Frage, die `shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` für die Angabe der macOS-Untergrenze stellt: eine Gewohnheit, die sich nicht von selbst hält, und deren Deckung schon einmal von 33 auf 5 abgesunken war.

## Options

1. **Es bleibt, wie es ist.**
   - Pro: nichts zu bauen. Die zwei Abnahmekriterien aus C3 decken diese Runde, und die nächste Runde kann dieselbe Frage neu stellen.
   - Contra: die Gewohnheit hält sich nicht von selbst, und dieses Projekt hat die Gegenprobe schon geliefert. Der Schaden ist still: ein Zeichen in einer Datei des Nutzers, das er nicht getippt hat, und kein Fehlschlag irgendwo.
2. **Eine Zählprobe am Baum: jede Datei, die `setEditable(true)` in einer Code-Zeile trägt, nennt auch `automatiken_abschalten`.**
   - Pro: klein, in der Bauform, die `crates/krk-core/tests/baum.rs` und `crates/krk-ui/src/quellbaum.rs` schon führen; die Nadel steht zusammengesetzt da wie die übrigen. Eine dritte Fläche fällt beim ersten `make check` auf.
   - Contra: die Nadel bindet an eine Schreibweise. Eine Fläche, die ihre Bearbeitbarkeit über `setValue:forKey:` setzt oder über zwei Zeilen umbricht, entgeht ihr. Der Kopf von `baum.rs` sagt, warum keine Nadel restlos dicht ist; hier wäre der blinde Fleck zu benennen und nicht zu verschweigen.
3. **Ein eigener Typ um die bearbeitbare Fläche: `automatiken_abschalten` wird nicht gerufen, sondern ist der einzige Weg, eine bearbeitbare `NSTextView` überhaupt zu bekommen.**
   - Pro: der Übersetzer hält, nicht eine Nadel. Die Frage kann nicht vergessen werden, weil es keinen zweiten Weg gibt.
   - Contra: der teuerste der drei. `textflaeche_bauen` und die Fläche des Zettels unterscheiden sich in mehr als den Automatiken (Nummernspalte, Rückgängig, Schrift, Bildlaufansicht), und ein gemeinsamer Erzeuger müsste die Unterschiede als Argumente führen. Die Vorschau ist eine dritte `NSTextView`, die gerade **nicht** bearbeitbar ist und trotzdem gebaut werden muss.

## Constraints

- **Eine zweite Aufstellung der Einstellungen entsteht nicht.** `EINSTELLUNGEN` mit seinen 36 geführten Merkmalen und fünf Einordnungen ist die eine Antwort auf die Frage, welche Einstellung wie zur Zusage aus C4 steht. Jede Möglichkeit hier lässt sie unangetastet.
- **Die Messung an einer gebauten Fläche bleibt.** Was `Abgeschaltet` heißt, wird nachgefahren und nicht der Dokumentation entnommen; das ist die Bauform seit den Defekten `260809-1650` und `260810-0416`, und sie wird durch keine der drei Möglichkeiten ersetzt.
- **Die Vorschau ist keine bearbeitbare Fläche und darf von keiner Regel eingefangen werden.** Sie setzt `setEditable(false)` und `setSelectable(false)`, damit sie den Fokus nicht als Textsystem nimmt.

## Recommendation

**Möglichkeit 2, mit dem benannten blinden Fleck.** Sie kostet eine Probe von zwanzig Zeilen, sie steht in einer Bauform, die dieses Projekt fünfmal führt, und sie fängt den Fall, der wirklich eintritt: jemand baut eine dritte Fläche nach dem Vorbild der zweiten und lässt eine Zeile aus. Was sie nicht fängt — eine Fläche, die ihre Bearbeitbarkeit über die Schlüsselwertkodierung setzt —, ist kein Fall, der in diesem Baum je vorgekommen ist, und der Doc-Kommentar sagt es aus, statt es zu verschweigen.

Möglichkeit 3 ist die einzige, die wirklich hält, und sie ist heute zu teuer: die drei Flächen dieses Baums unterscheiden sich in vier Dingen, und ein Erzeuger für alle drei trüge vier Argumente, von denen jedes eine Fallunterscheidung ohne Fall wäre. Sie wird billiger, sobald eine vierte Fläche dazukommt; dann gehört die Frage neu gestellt.

Möglichkeit 1 ist die Antwort, die dieses Projekt bei der Angabe der macOS-Untergrenze schon einmal gegeben hat, und die Deckung dort ist von 33 auf 5 abgesunken, bevor sie von Hand wiederhergestellt wurde.

---
Answered:
Implemented:
Deferred:
Superseded by:
