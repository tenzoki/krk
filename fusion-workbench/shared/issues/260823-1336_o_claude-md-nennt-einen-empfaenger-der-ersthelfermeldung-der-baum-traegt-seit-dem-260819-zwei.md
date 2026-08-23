`CLAUDE.md` nennt einen Empfänger der Ersthelfermeldung, der Baum trägt seit dem 260819 zwei

---

Der Absatz „Jeder Wechsel des Ersthelfers geht durch die Überschreibung von
`makeFirstResponder:` in `appkit/fenster.rs`" in `CLAUDE.md` sagt: „Empfänger ist
`Anwendungsdelegierter::fokusanzeige_nachziehen`". Am Melder hängen seit der Runde 14 **zwei**
Empfänger, und der zweite geht bis `Aufteilung::anwenden` durch — also genau bis dorthin, wohin
derselbe Absatz zusichert, dass dieser Weg nicht führt.

---

**Gemessen am Baumstand `616ad5e`. Nicht durch die neun Commits dieser Sitzung entstanden**: die
Lücke besteht seit `76ceb683` vom 260819 (Runde 14) und ist beim Abgleich der Sitzung
`260823-0442` aufgefallen, weil der offene Befund `260823-0732` genau auf ihr steht.

## Was der Baum trägt

`crates/krk-ui/src/appkit/anwendung.rs`, der Rumpf des Melders:

```rust
fenster.melder_setzen(Box::new(move || {
    if let Some(selbst) = schwach.load() {
        selbst.aktives_dem_ersthelfer_nachziehen();
        selbst.fokusanzeige_nachziehen();
    }
}));
```

Der Kommentar unmittelbar darüber schreibt es aus: „**Zwei Empfaenger haengen daran, und der
erste ist der neuere.**" Und der Doc-Kommentar der Probe
`der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an` sagt die Folge: „Seither haengt
`aktives_dem_ersthelfer_nachziehen` als zweiter daran, und **der** geht ueber `aktives_setzen`
sehr wohl bis `anwenden` durch."

## Was `CLAUDE.md` daraus macht

Zwei Aussagen des Absatzes, und nur die erste ist falsch:

1. „Empfänger ist `Anwendungsdelegierter::fokusanzeige_nachziehen`" — **falsch**, seit dem
   260819. Es sind zwei, und der ungenannte ist der wirksamere.
2. „Es ruft weder `anwenden` noch `setHidden`" — **richtig** und von einer Probe gehalten, aber
   sie trägt nicht mehr, was der Absatz mit ihr begründet. Der Absatz will sagen, dass die
   Meldung nicht auf `anwenden` führen kann; über den zweiten Empfänger führt sie dorthin.

**Der Schaden ist konkret und nicht theoretisch.** Wer `CLAUDE.md` liest und eine Anzeige an den
Fokus hängt, glaubt an einen Auslösepunkt mit einem Empfänger, der die Auslegung nicht anfasst.
Der Befund `260823-0732` beschreibt, was daraus wird: `df8163d` hat `setHidden` vor den
gewollten Fokusumzug gezogen, und ob der Ring dabei betreten wird, hängt an dem Empfänger, den
`CLAUDE.md` nicht führt.

## Vorschlag

Den Satz von einem auf zwei Empfänger stellen, in der Reihenfolge, in der sie laufen, und die
Zusage „ruft weder `anwenden` noch `setHidden`" ausdrücklich auf `fokusanzeige_nachziehen`
beschränken statt auf die Meldung. Die Quelle für beides steht im Baum und braucht keine neue
Erhebung.

**Nicht in diesem Abgleich geändert**: `CLAUDE.md` gehört dem `curator` und nicht dem
`reconciler`; der Auftrag dieser Sitzung sagt es ausdrücklich.

**Schwere:** mittel. Keine Fehlfunktion, aber eine Zusicherung an der Stelle, an der ein
Entwickler sie nachschlägt, und sie ist die Voraussetzung eines offenen Verhaltensbefunds.

**Gefunden:** reconciler, Abgleich der Sitzung `260823-0442` am 260823-1336

**Betroffen:** `CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß"

**Domain:** code

**Verwandt:**
`shared/issues/260823-0732_*_der-neue-nachzug-laeuft-vor-dem-fokusumzug-*` — der offene
Verhaltensbefund, der auf dem ungenannten Empfänger steht.
