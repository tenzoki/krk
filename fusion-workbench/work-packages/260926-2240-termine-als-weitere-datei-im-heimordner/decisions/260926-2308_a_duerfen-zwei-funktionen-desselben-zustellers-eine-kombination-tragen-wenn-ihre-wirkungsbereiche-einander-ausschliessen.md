# Dürfen zwei Funktionen desselben Zustellers eine Kombination tragen, wenn ihre Wirkungsbereiche einander ausschließen, und wie findet der Nachschlag dann die richtige?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260926-2253_*_spec-termine-als-weitere-datei-im-heimordner.md` (Annahme A8, T5); `260926-2240-termine-als-weitere-datei-im-heimordner.md` (Arbeitspaket, `**Mode:** autonomous`); der Plan dieses Arbeitspakets, `260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md`, Schritte 3 und 8; `260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md` (die Zustellerregel, die diese Frage erweitert); `260813-0430_*_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md` (die Menüregel für die Zustellerpaare)

---

## Question

Die Directive des Arbeitspakets legt `cmd+1` auf „Termine: Sortierrichtung umkehren“, und der Spec verlangt, dass „Nach Name sortieren“ `cmd+1` behält (A8). Beide Funktionen stellt der Ereignisabgriff zu. Die Konfliktregel der Belegung sagt heute: zwei Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen und denselben Zusteller haben (`Belegung::konflikte`, `Belegung::zuweisen`, `crates/krk-core/src/tasten/belegung.rs`). Unter dieser Regel bricht die eingebettete Auslieferungsbelegung beim ersten Zugriff ab. Und selbst mit einer gelockerten Regel liefert `Belegung::nachschlag` den **ersten** Treffer in der Reihenfolge der Datei. `cmd+1` käme dann nie bei der Termintabelle an, weil der Abgriff „Nach Name sortieren“ findet, das im Editor unzulässig ist, und den Tastendruck an AppKit weiterreicht.

Zu entscheiden sind drei Dinge, die zusammenhängen: woran die Belegung erkennt, dass zwei Funktionen nie zugleich zulässig sein können; wie der Nachschlag zwischen zweien wählt; und welcher der beiden Menüeinträge das Kürzel zeigt, denn AppKit nimmt es dem späteren von zwei gleichen still weg (gemessen am 260813, Doc-Kommentar von `zugestellte_kuerzel` in `crates/krk-ui/src/menuemodell.rs`). Die Antwort bindet über dieses Arbeitspaket hinaus: jede spätere Umbelegung und jede künftige Funktion fällt unter dieselbe Regel.

## Options

1. **Die Seite des Wirkungsbereichs, im Kern entschieden.** Jeder `Wirkungsbereich` bekommt über eine vollständige Fallunterscheidung ohne Auffangzweig eine von drei Seiten: `Editor` (wirkt allein mit dem Fokus im Editor: `Editor`, `Editortext`, `Eintraege`, `Aufgaben`, `Geheimnisse` und die neuen Werte dieses Arbeitspakets), `Ausserhalb` (wirkt nie mit dem Fokus im Editor: `Dateifenster`, `Leiste`, `Tabbereich`, `Navigator`, `Vorschau`) und `Beide` (`Dateibereiche`, `Ueberall`). Zwei Wirkungsbereiche schließen einander genau dann aus, wenn der eine auf `Editor` und der andere auf `Ausserhalb` steht. Eine Funktion ohne Kommando hat keinen Wirkungsbereich und schließt nichts aus. Die Konfliktregel lautet dann: gleiche Kombination, gleicher Zusteller **und** keine Seite schließt die andere aus. Sie steht an einer Stelle, die `konflikte` und `zuweisen` beide fragen. Der Nachschlag sammelt die Treffer des Abgriffs und antwortet mit einer neuen Variante `Nachschlag::Geteilt(erste, zweite)`, wenn es zwei sind; mehr als zwei kann es nicht geben, weil drei paarweise ausschließende Funktionen drei verschiedene Seiten aus zweien bräuchten. Die Oberfläche wählt mit der einen `Lage` der Eingabe über eine reine Funktion in `kommandos/zulaessigkeit.rs`: die zweite, wenn allein sie zulässig ist, sonst die erste. Im Menü behält der Eintrag, der in der Leiste früher steht, das Kürzel, und der spätere zeigt keines. Das ist die Regel, die AppKit ohnehin anwendet, als Rechnung im Modell ausgeschrieben.
   - Pros: Die Frage ist aus Eingaben entscheidbar, die der Kern hat: der Wirkungsbereich ist eine Eigenschaft je Kommando, und die Seite folgt aus der Tafel von `fokus::wirkt`, die eine Probe in `krk-ui` über alle Fokuswerte und alle Paare gegen die Seiten hält. Die Regel gilt für die Auslieferung, für jede Nutzerdatei und für die Umbelegung in F1 gleich, und `cmd+a` und `cmd+f` bleiben unberührt, weil sie am Zusteller getrennt sind. Keine Kennung wird Sonderfall.
   - Cons: Die Regel ist gröber als die Wirklichkeit. Zwei Werte derselben Seite, etwa `Eintraege` und `Editortext`, sind der Form nach nie zugleich zulässig und bleiben trotzdem ein Konflikt. Das ist die sichere Richtung des Fehlers: eine Doppelung, die wirklich zusammenfiele, wird immer gemeldet. Der Nachschlag läuft für einen Treffer bis zum Ende der Liste statt bis zum ersten Treffer. Das kostet nie mehr als ein Anschlag, der keiner Funktion gehört, heute schon kostet. Das Menü zeigt „Nach Name sortieren“ ohne `⌘1`, obwohl `cmd+1` im Dateifenster weiter nach Name sortiert; derselbe Preis, den `cmd+a` seit dem 260813 trägt.
2. **Die genaue Zulässigkeitsfrage, in den Kern gereicht.** `Belegung::bauen` bekommt von außen eine Funktion, die für zwei Kommandos sagt, ob es eine Lage gibt, in der beide zulässig sind, gerechnet über die ganze Tafel aus `kommandos/zulaessigkeit.rs` einschließlich der Form.
   - Pros: Genau, auch innerhalb einer Seite.
   - Cons: Die eingebettete Auslieferungsbelegung wird in einem `LazyLock` im Kern gebaut und kennt keinen Rufer aus `krk-ui`; der Kern hinge für seine eigene Schlüssigkeit an einer Regel der Oberfläche, die Aufrufrichtung kehrte sich um. Jeder Ladeweg (`laden`, `fuer_den_betrieb`, `vom_nutzer`, `zuweisen`) bekäme einen weiteren Parameter.
3. **Eine benannte Ausnahme für `cmd+1`**, wie ein zweites `gehalten_von`: ein Feld in der Belegungsdatei, das ein Paar ausdrücklich erlaubt.
   - Pros: Klein.
   - Cons: Genau die Sonderregel, die der Modulkopf von `tasten/belegung.rs` ausschließt; die nächste Doppelung braucht die nächste Ausnahme, und eine Nutzerdatei könnte sie setzen, wie sie bis zum 260908 den Zusteller setzen konnte.
4. **Eine eigene Taste für die Termine** und keine Änderung der Regel.
   - Pros: Nichts zu bauen an der Belegung.
   - Cons: Widerspricht der Directive, die `cmd+1` nennt; im Spec als A8 ausdrücklich verworfen.

## Constraints

- „Nach Name sortieren“ behält `cmd+1` und wirkt in genau den Lagen, in denen es vor dieser Arbeit zulässig war (T5).
- Kein Konflikt, den die Auslieferung oder die eigene Belegung dieses Geräts heute meldet, darf verloren gehen (Haltestelle 1 des Spec). Nachgesehen am 260926-2300: beide laden heute ohne Konflikt; die einzigen doppelten Kombinationen sind `cmd+a` und `cmd+f`, beide am Zusteller getrennt.
- Die Zulässigkeit wird je Eingabe einmal erhoben (`Anwendungsdelegierter::lage`, Doc-Kommentar); die Wahl zwischen zwei Funktionen darf keine zweite Erhebung einführen.
- Die Probe `keine_zwei_eintraege_tragen_dieselbe_kombination` (`crates/krk-ui/src/menuemodell.rs`) bleibt grün.

## Recommendation

**Möglichkeit 1.** Sie ist die einzige, die die Frage aus Eingaben beantwortet, die der Kern selbst hat, und damit eine Regel statt einer Ausnahme; die Grobheit fällt immer auf die Seite des gemeldeten Konflikts. Der Plan dieses Arbeitspakets baut sie in den Schritten 3 und 8. Das Arbeitspaket läuft nach `**Mode:** autonomous`; der Plan arbeitet auf dieser Empfehlung und legt sie bei der Abnahme zur Durchsicht vor.

---
Answered: 260926-2308_*_plan-termine-als-weitere-datei-im-heimordner.md `## Entscheidungen des Plans` — Möglichkeit 1: die Seite des Wirkungsbereichs im Kern entscheidet, ob zwei Funktionen eine Kombination teilen dürfen; ruled by user, Kai Stalmann <kai@stalmann.org> (Anweisung im Chat 260926-2250, beide Arbeitspakete autonom auszuführen; **Mode:** autonomous auf 260926-2240-termine-als-weitere-datei-im-heimordner)
