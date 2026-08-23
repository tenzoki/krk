# `kommando_ausfuehren` liefert nicht „immer `true`" — vier Codestellen und ein Entscheidungsdatensatz sagen es

---

`Anwendungsdelegierter::kommando_ausfuehren` hat zwei Rückgabestellen, nicht eine. Es liefert
`false` für **jeden** Befehl, den `zulaessigkeit::zulaessig` abweist, und `true` für jeden, der
durchkommt. Die Aussage „liefert seit der Runde 7 **immer** `true`" ist als Satz über die Funktion
falsch. Sie steht mit `52fba42` an vier Stellen im Quelltext und seit dem 260823-1350 in der
`Implemented:`-Zeile des Entscheidungsdatensatzes `260813-0053`, der die Regel bindend führt.

---

**Am Baum gelesen, nicht abgeleitet.**

## Was der Code tut

`crates/krk-ui/src/appkit/anwendung.rs:2982` bis `:3243`, die ganze Funktion hat genau zwei
Ausgänge:

```rust
        let lage = self.lage();
        if !zulaessigkeit::zulaessig(kommando, lage) {
            return false;               // anwendung.rs:3003
        }
        …
        if gewirkt {
            self.aufteilung_nachziehen();
            self.sitzung_vormerken();
        }
        true                            // anwendung.rs:3242
```

Gezählt mit `awk 'NR>=2982 && NR<=3243' … | grep -c return`: ein `return`, und das ist das
`return false`.

## Drei Stellen im selben Baum sagen das Gegenteil des neuen Satzes

Keine davon ist mit diesem Commit angefasst worden, alle drei stehen unverändert da:

- `crates/krk-ui/src/appkit/anwendung.rs:5355-5357`: „Solange ein Blatt steht, lässt
  `Self::kommando_ausfuehren` genau **vier** Kommandos durch und **weist jedes weitere ab**."
- `crates/krk-ui/src/messmodus.rs:94-95`: „`Anwendungsdelegierter::kommando_ausfuehren` **weist
  dann jeden Befehl ab**, der einen Wirkungsbereich nennt."
- `crates/krk-ui/src/appkit/blaetter/mod.rs:305`: „weist `Anwendungsdelegierter::kommando_ausfuehren`
  jeden Befehl ab bis auf vier".

Abweisen heißt hier `return false`. `CLAUDE.md` schreibt dieselbe Regel im Abschnitt „Was man nicht
sieht" aus. Ein Baum, der an drei Stellen „weist ab" und an vier „liefert immer `true`" sagt, sagt
zweierlei über dieselbe Zeile.

## Die richtige Fassung steht schon zweimal im Baum

Zwei der mit `52fba42` geänderten Stellen haben sie getroffen, und sie sind die Vorlage für die
Berichtigung:

- `crates/krk-ui/src/appkit/tabelle.rs:2206-2208`: „schluckt seit der Runde 7 **jeden zulässigen**
  Befehl".
- `crates/krk-ui/src/appkit/anwendung.rs:3288-3290`: „seither schluckt der Abgriff **jeden
  zulässigen** Befehl, und ein `Wirkungsbereich::Ueberall` ist mit dem Fokus im Editor zulässig."

Die zweite zeigt zugleich, warum die kurze Fassung nicht genügt: sie muss die Zulässigkeit für
ihren Fall eigens belegen. Genau diesen Beleg lässt die absolute Fassung weg.

## Die vier Codestellen mit der absoluten Fassung

| Stelle | Wortlaut |
|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs:1959` (`terminal_oeffnen`) | „denn `Self::kommando_ausfuehren` liefert seit der Runde 7 immer `true`" |
| `crates/krk-ui/src/appkit/anwendung.rs:6297-6298` (`editor_oeffnen_lassen`) | „weil `Self::kommando_ausfuehren` seit der Runde 7 immer `true` liefert" |
| `crates/krk-ui/src/appkit/anwendung.rs:7075` (`editor_rundweg`) | „weil `Self::kommando_ausfuehren` seit der Runde 7 immer `true` liefert" |
| `crates/krk-ui/src/kommandos/rundweg.rs:120-121` | „den Tastendruck verbraucht `Anwendungsdelegierter::kommando_ausfuehren` **in jedem Fall**, weil es seit der Runde 7 immer `true` liefert" |

`weitere_instanz_starten` (`anwendung.rs:1976-1979`) verweist auf `terminal_oeffnen` und erbt den
Satz über den Verweis; es trägt ihn nicht selbst.

**Die örtliche Folgerung stimmt an allen vier Stellen.** Jede sitzt hinter der
Zulässigkeitsprüfung: `terminal_oeffnen` und `editor_rundweg` sind Zweige des `match`, das erst
nach `if !zulaessig { return false }` läuft, und `editor_oeffnen_lassen` wie `rundweg()` werden
nur von dort aus erreicht. Für diese Aufrufe ist der Tastendruck wirklich verbraucht. Falsch ist
die Verallgemeinerung, mit der begründet wird, und sie steht als Satz über die Funktion da, nicht
als Satz über den Aufrufweg.

## Warum das über Prosaschuld hinausgeht

Der Satz ist aus dem Bericht
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md` in den Datensatz
`shared/issues/260823-1033_c_…` (erster Absatz) gewandert, von dort mit `52fba42` in vier
Doc-Kommentare, und von dort am 260823-1350 in die `Implemented:`-Zeile von
`shared/decisions/260813-0053_i_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`:

> `kommando_ausfuehren` liefert seit jener Runde **ausnahmslos** `true`.

Der Datensatz trägt `_i_` und ist damit bindende Grundlage. Sein eigener Fragetext und seine erste
`Answered:`-Zeile tragen die **richtige** Fassung („der Abgriff schluckt, was zulässig war,
unabhängig davon, was der Befehlsrumpf zurückgibt"), und die Möglichkeit 1, die der Nutzer gewählt
hat, ist ebenfalls richtig formuliert. **Die Entscheidung des Nutzers ist also nicht auf falscher
Grundlage ergangen**; falsch ist allein der zusammenfassende Satz, mit dem die Umsetzung
nachgetragen wurde. Wer künftig nur diese Zeile liest, hält die Zulässigkeitsprüfung für
wirkungslos — und die ist die eine Sperre, an der Blattsperre, Fokusvorbehalt, fremdes
Schlüsselfenster und AppKit-Ersthelfer hängen.

## Vorschlag

Die vier Codestellen auf die Fassung ziehen, die `tabelle.rs:2206` schon trägt: „schluckt jeden
**zulässigen** Befehl". Wo die Stelle begründet, warum ihr eigener `false`-Weg den Tastendruck
trotzdem verbraucht, gehört der Beleg dazu, dass der Befehl an dieser Stelle zulässig war — so wie
`anwendung.rs:3288-3290` es vormacht. Die `Implemented:`-Zeile von `260813-0053` gehört
gleichlautend berichtigt; das Ergebnis der Entscheidung ändert sich nicht.

`shared/issues/260823-1033_c_…` ist eine Aufzeichnung eines Standes und bleibt stehen, wie sie ist.

**Schwere:** Medium. Kein Verhalten ist betroffen, und keine der vier Stellen zieht eine falsche
örtliche Folgerung. Der Satz steht aber in einem `_i_`-Entscheidungsdatensatz und widerspricht drei
anderen Stellen desselben Baums; das ist genau die Klasse, gegen die `260823-1032` und
`260823-0730` geschrieben sind.

**Gefunden:** coderev, Auslieferungsdurchsicht `28cbb7b..b58e9d1`, Baumstand `b58e9d1`

**Domain:** code

**Cross-references:** `shared/decisions/260813-0053_i_…`, `shared/issues/260823-1033_c_…`,
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`

---
Resolved:
