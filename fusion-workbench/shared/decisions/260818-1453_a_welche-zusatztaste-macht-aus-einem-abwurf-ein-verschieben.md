# Welche Zusatztaste macht aus einem Abwurf ein Verschieben, und wer beantwortet die Frage: KRK oder das System?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `/Applications/Xcode.app/…/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSDragging.h:72` (`draggingSourceOperationMask`), `:118-119` (`draggingEntered:`, `draggingUpdated:`), `:126-127` (`wantsPeriodicDraggingUpdates`); `…/NSTableView.h:783` und `:787` (`validateDrop:`, `acceptDrop:`); `crates/krk-ui/src/appkit/anwendung.rs:4428` (`uebertragen`, der heutige Weg zu Kopieren und Verschieben), `:5302` (`auftrag_stellen`); `crates/krk-core/src/operation/auftrag.rs:19-31` (`Art`); `crates/krk-ui/src/appkit/zwischenablage.rs` (die eine Hülle um `NSPasteboard`); `shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md` (die Haltung der Runde 12 zu zerstörerischen Wegen)

---

## Question

Der Nutzer hat am 260818 für den Abwurf aus einer fremden Anwendung festgelegt: **Kopieren ist die Vorgabe, und `shift` erzwingt ein Verschieben, wo die Quelle es hergibt.** Er hat damit drei angebotene Möglichkeiten verworfen und eine vierte gesetzt, ausdrücklich auf der sicheren Seite: die Vorgabe ist der nicht zerstörerische Weg.

Die Wahl weicht bewusst von der Plattform ab. Der Finder erzwingt mit `cmd` ein Verschieben und mit `opt` ein Kopieren; `shift` trägt beim Ziehen keine Bedeutung. Der Grund, der für `shift` sprach, war, dass es dem System nicht in die Quere kommt: `opt` und `cmd` deutet der Ziehdienst selbst, `shift` nicht.

**Der Grund trägt nur die halbe Strecke, und das ist der Anlass dieses Datensatzes.** Dass `shift` frei ist, heißt nicht, dass die beiden Deutungen sich nicht begegnen. Das Ziel bekommt in `draggingEntered:` und `draggingUpdated:` nicht die rohe Tastenlage, sondern `draggingSourceOperationMask`: die Menge der Vorgänge, die die Quelle anbietet, und diese Menge ist bereits gefiltert, wenn der Nutzer `opt` oder `cmd` hält. Das Ziel darf aus `draggingUpdated:` nur einen Vorgang aus dieser Menge zurückgeben; ein Vorgang außerhalb ist so viel wert wie `NSDragOperationNone`.

Damit gibt es zwei Instanzen, die dieselbe Frage beantworten — „Kopieren oder Verschieben?" —, und sie widersprechen sich in genau dem Fall, der am häufigsten eintritt. Wer aus dem Finder zieht und dabei aus Gewohnheit `cmd` hält, verengt die Menge auf Verschieben. KRK sieht kein `shift`, will also kopieren, und Kopieren steht nicht mehr in der Menge. Umgekehrt: `opt` (Finder-Kopieren) zusammen mit `shift` (KRK-Verschieben) verengt auf Kopieren, während `shift` das Gegenteil verlangt. In beiden Fällen steht das Zeigersymbol, das der Nutzer während des Ziehens sieht, quer zu dem, was KRK nach dem Loslassen täte.

`critical-stance.md` §4 nennt genau diese Lage: eine Fallunterscheidung, die sich nicht überschneidungsfrei machen lässt, ist falsch geschnitten. Die Antwort auf die Frage entscheidet deshalb nicht bloß eine Taste, sondern ob KRK die Frage überhaupt selbst beantwortet.

**Was hier geprüft ist und was nicht.** Geprüft am SDK sind die Signaturen und der Vertrag: `draggingSourceOperationMask` ist die Menge, aus der das Ziel wählt (`NSDragging.h:72`), und `draggingUpdated:` läuft nicht von selbst, solange die Maus stillsteht, wenn das Ziel `wantsPeriodicDraggingUpdates` verneint (`:126-127`). `inference:` — dass der Ziehdienst `opt` und `cmd` in eben dieser Menge auflöst, ist dokumentiertes Verhalten von AppKit, steht aber in keinem Kopf dieses SDK und ist an diesem Baum nicht messbar. `speculation:` — ob AppKit `draggingUpdated:` erneut schickt, wenn der Nutzer eine Zusatztaste drückt, ohne die Maus zu bewegen, ist offen; falls nicht, wechselt die Anzeige des Abwurfziels erst bei der nächsten Mausbewegung.

## Options

1. **Die Plattform beantwortet die Frage, KRK liest die Antwort ab.** Keine Zusatztaste ohne Wirkung: ohne Taste kopiert der Abwurf, mit `cmd` verschiebt er, mit `opt` kopiert er ausdrücklich. KRK wählt aus `draggingSourceOperationMask` und deutet keine Taste selbst.
   - Pro: eine Instanz, eine Antwort. Zeigersymbol und Wirkung stimmen in jedem Fall überein, ohne dass eine Regel das herstellen müsste. Wer aus dem Finder zieht, bekommt in KRK, was er im Finder bekommt.
   - Kontra: `cmd` ist damit die zerstörerische Taste, und sie liegt unter dem Finger, weil der Finder sie dort hat. Genau davor wollte die Nutzerantwort vom 260818 schützen.
   - Was sie verbaut: `shift` als Verschiebetaste, dauerhaft. Eine spätere Umbelegung liefe wieder in diesen Widerspruch.

2. **Der Abwurf kopiert immer. Es gibt keine Zusatztaste.** Verschoben wird zwischen den Dateifenstern weiter mit F6.
   - Pro: die kleinste Regel, die es gibt, und keine zerstörerische Wirkung auf Daten einer fremden Anwendung. Der Widerspruch entsteht gar nicht: KRK gibt immer `NSDragOperationCopy` zurück, das jede Quelle anbietet, die überhaupt Dateien liefert.
   - Kontra: aus einem zweiten KRK-Fenster oder aus dem Finder heraus verschieben geht per Ziehen nie. Der Nutzer hat ein Verschieben ausdrücklich gewollt.
   - Was sie verbaut: das Verschieben per Ziehen in dieser Runde ganz. Es bliebe einer späteren Runde, und die stünde vor derselben Frage.

3. **KRK fragt nach dem Loslassen.** Der Abwurf öffnet ein Blatt „Kopieren / Verschieben / Abbrechen" mit „Kopieren" vorbelegt; während des Ziehens zeigt KRK `NSDragOperationGeneric`, also einen Zeiger ohne Kopier- oder Verschiebezeichen.
   - Pro: keine Zusatztaste, kein Widerspruch, und die Wirkung steht ausgeschrieben da, bevor sie eintritt. Das ist dieselbe Haltung, die die Runde 12 für jeden Löschweg festgelegt hat, und KRK trägt die Blattmaschinerie samt vorbelegtem „Abbrechen" bereits. `NSDragOperationGeneric` ist der dafür vorgesehene Wert und behauptet während des Ziehens nichts, was danach anders ausfiele.
   - Kontra: ein Blatt bei jedem einzelnen Abwurf, auch beim gewöhnlichen Kopieren. Der Spec der Runde 12 hat selbst festgehalten, dass eine Rückfrage, die zu oft kommt, weggeklickt statt gelesen wird.
   - Gegenrede dazu: dort ging es um eine Taste, die auf jeden berichtigten Vertipper folgte. Ein Abwurf ist eine gezielte Handlung mit zwei Händen und passiert selten. Die beiden Fälle sind nicht dieselben.
   - Was sie verbaut: den Abwurf ohne jede Rückfrage. Wer oft kopiert, zahlt jedes Mal einen Tastendruck.

4. **`shift` bleibt, und der Widerspruchsfall wird ausgeschrieben.** Hält der Nutzer zusätzlich `opt` oder `cmd`, gewinnt die Menge des Systems und `shift` bleibt wirkungslos.
   - Pro: die Antwort des Nutzers vom 260818 bleibt stehen, samt ihrer sicheren Vorgabe.
   - Kontra: eine Regel, die der Nutzer nicht sehen kann und die nichts ihm mitteilt. `shift` wirkt mal und mal nicht, abhängig von einer zweiten Taste, von der er nicht weiß, dass sie mitspricht. Das ist der Rand aus Sonderfällen, den `critical-stance.md` §4 als Zeichen eines falschen Schnitts benennt, und er wächst mit jeder weiteren Quelle.
   - Was sie verbaut: nichts dauerhaft, aber sie verschiebt den Widerspruch in den Code, statt ihn aufzulösen.

5. **Erst messen, dann entscheiden.** Eine Vorstudie unter `spikes/` am Referenzgerät klärt drei Fragen: wie `draggingSourceOperationMask` bei gehaltenem `opt`, `cmd` und `shift` tatsächlich ankommt, ob `NSEvent.modifierFlags` in `validateDrop:` die Lage von `shift` trägt, und ob AppKit bei einem Tastenwechsel ohne Mausbewegung erneut anfragt.
   - Pro: der Baum kennt dieses Vorgehen für genau solche Fragen (`spikes/fn-tasten/messung-A.txt` hat den Tastencode von F3 so geklärt). Danach ist die Entscheidung eine über Bedienung und nicht über unbelegtes Verhalten.
   - Kontra: sie hält die Runde auf, und drei der vier Möglichkeiten darüber hängen gar nicht vom Messergebnis ab: 1, 2 und 3 sind unabhängig davon widerspruchsfrei. Gemessen werden müsste nur für Möglichkeit 4.

## Constraints

- **KRK hält genau einen Vorgang** (`anwendung.rs:5348`, `vorgang_laeuft_schon`). Was die Antwort auch sei, sie mündet in denselben `Auftrag` mit `Art::Kopieren` oder `Art::Verschieben` und in dieselbe Konfliktrückfrage; eine zweite Operationsmaschine entsteht nicht.
- **Es gibt genau eine Hülle um `NSPasteboard`** (`appkit/zwischenablage.rs`). Die Ablage eines Ziehvorgangs ist nicht `generalPasteboard`, sondern die aus `draggingPasteboard`. Sie dort zu lesen ist eine begründete Erweiterung jener Hülle, keine zweite daneben.
- **Das Zeigersymbol und die Wirkung müssen übereinstimmen.** Ein Abwurf, der verschiebt, während der Zeiger ein Pluszeichen trug, ist ein Datenverlust mit Ankündigung des Gegenteils.
- **`shift` allein ist im Ziehdienst frei**, aber die Menge, die das Ziel liest, ist es nicht.
- Alle berührten Klassen stehen weit unter der Untergrenze macOS 15: `registerForDraggedTypes:` seit 10.0, `NSDraggingInfo` seit 10.0, `NSFilePromiseReceiver` seit 10.12. Am SDK gelesen.

## Recommendation

Möglichkeit 3, mit Möglichkeit 1 als nächstbester. Sie ist die einzige, die das Verschieben behält, das der Nutzer wollte, ohne zwei Instanzen dieselbe Frage beantworten zu lassen: KRK deutet gar keine Taste, das System hat nichts zu filtern, und was geschieht, steht vor dem Vollzug im Klartext da. Sie fügt daneben keine Bedienweise hinzu, die dieses Programm nicht schon hätte. Möglichkeit 1 ist die ehrliche Alternative für den, dem die Rückfrage zu teuer ist; ihr Preis ist, dass die zerstörerische Wirkung auf der Taste liegt, die der Finger vom Finder her ohnehin hält. Möglichkeit 4 hat den Vorzug, die gegebene Antwort stehen zu lassen, und den Nachteil, dass sie eine unsichtbare Regel dafür einführt.

---
Answered: shared/planning/260818-1510_*_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md §C5 — Moeglichkeit 1: die Plattform beantwortet die Frage. Kopieren ist die Vorgabe, `cmd` verschiebt, `opt` kopiert ausdruecklich; `shift` traegt keine Bedeutung. KRK deutet keine Zusatztaste selbst, sondern waehlt aus `draggingSourceOperationMask`: enthaelt die Menge das Kopieren, kopiert es, sonst verschiebt es. Der Nutzer hat damit seine Antwort vom 260818 (`shift` verschiebt) ersetzt, nachdem der Widerspruch zwischen der eigenen Deutung und der vom System bereits verengten Menge belegt war.
Implemented:
Deferred:
Superseded by:
