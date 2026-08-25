# Was tut Unzip, wenn der Zielordner schon dasteht?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_a_circle.md` (Directive, Unzip-Teil); `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/decisions/260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`; `crates/krk-core/src/operation/auftrag.rs` (`Konfliktregel`)

---

## Question

Unzip legt den Inhalt eines Archivs in einen neuen Ordner im angezeigten Ordner, benannt nach dem Archiv. Der Nutzer hat diese Regel in Runde 2 gewählt, und sie lässt den häufigsten Fall offen: derselbe Nutzer entpackt dasselbe Archiv ein zweites Mal, und der Ordner steht bereits da. Für das Zip-Archiv ist der Konfliktfall beantwortet, für den Unzip-Ordner nicht, und die beiden sind nicht dasselbe. Ein Archiv ist eine Datei, die überschrieben oder danebengelegt wird; ein Ordner kann mit einem vorhandenen Ordner verschmelzen, und dann stellt sich die Frage je enthaltenem Eintrag neu.

## Options

1. **Danebenlegen ohne Rückfrage** — der zweite Lauf erzeugt `Projekte 2`, gemeldet in der Statuszeile.
   - Pro: Nichts wird überschrieben, der vorige Stand bleibt unangetastet, und der Nutzer kann beide vergleichen. Keine Rückfrage bei einer Handlung, die nichts zerstört.
   - Contra: Wer dreimal entpackt, hat drei Ordner und muss selbst aufräumen.
2. **Dieselbe Rückfrage wie beim Zip, einmal für den Ordner** — überschreiben, danebenlegen oder abbrechen, bevor ein Eintrag geschrieben wird.
   - Pro: Eine Rückfrage, ein Verhalten für beide Befehle des Kontextmenüs, und der Nutzer entscheidet bewusst.
   - Contra: „Überschreiben“ heißt hier, einen ganzen Ordnerbaum zu löschen, also die folgenreichste Antwort des Blattes an der harmlosesten Stelle. Die Löschrückfrage aus Runde 12 hat für genau solche Wege eine eigene Bestätigung eingeführt.
3. **In den vorhandenen Ordner hineinentpacken, mit der bestehenden Konfliktregel je Eintrag** — der Ordner verschmilzt, und jede kollidierende Datei löst das gewohnte Konfliktblatt aus.
   - Pro: Genau das Verhalten des Kopiervorgangs, den `Konfliktregel` bereits trägt: ein Ordner auf einem Ordner gleichen Namens ist dort ausdrücklich kein Konflikt.
   - Contra: Ein großes Archiv erzeugt eine Kette von Rückfragen. Der Nutzer sieht am Ende einen Ordner, der Inhalte aus zwei Quellen mischt, ohne dass ihm die Mischung angesagt wurde.

## Constraints

Der Kern kennt die Verschmelzung bereits: `Konfliktregel` in `crates/krk-core/src/operation/auftrag.rs` hält fest, dass ein Ordner auf einem Ordner gleichen Namens kein Konflikt ist und sein Inhalt in den vorhandenen wandert. Ein Unzip, das sich anders verhält, ist eine zweite Regel neben dieser und muss begründet sein. Seit Runde 12 geht jedem Löschweg eine Rückfrage voraus, und es gibt nur noch den Weg in den Papierkorb; jede Antwort, die einen vorhandenen Ordner beseitigt, fällt unter diese Regel und darf nicht endgültig löschen.

## Recommendation

Möglichkeit 1. Sie ist die einzige der drei, die weder löscht noch mischt, und sie kostet den Nutzer nichts als einen Ordner, den er selbst wieder wegräumt. Möglichkeit 3 wäre die konsequentere Fortsetzung der Kopierregel, verlangt aber eine Kette von Rückfragen für einen Vorgang, den der Nutzer als eine Handlung angestoßen hat.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:
