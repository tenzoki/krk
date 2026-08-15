Der Aufrufkommentar in `main.rs` sagt weiter, die Art der Identität entscheide den Hinweis

---

`a46fd1f` hat den Leitsatz des Modulkopfs von `xtask/src/sign.rs` ersetzt, weil er die
falsche Einordnung mittrug: die Verzweigung liest nicht die **Art** der Identität, sondern
ihren **Namen**. Derselbe Satz steht unverändert an der Aufrufstelle.

`xtask/src/main.rs:145-146`:

```rust
// … und `release` faehrt genau den
// Weg, auf den der Hinweis zeigt. Was er sagt, entscheidet die Art
// der Identitaet; siehe [`sign::weitergabehinweis`].
```

---

**Schwere:** mittel. Kein Verhalten, kein Bau. Der Kommentar verweist auf eine Funktion, die
das Gegenteil dessen dokumentiert, was er über sie sagt.
**Gefunden von:** coderev, Durchsicht des Bereichs `a2670db..8c06747`
**Betroffen:** `xtask/src/main.rs:145-146`
**Domain:** code

## Warum das nicht bloß eine stehen gebliebene Formulierung ist

Der Commit `a46fd1f` nennt den ersetzten Leitsatz in seiner eigenen Botschaft: „Der Leitsatz
des Modulkopfs ist ersetzt, er trug die falsche Einordnung mit." Die Abschlussnotiz von
`shared/issues/260815-1444_c_…` schreibt es aus: „Er sagte, unterschieden werde ‚nach der
Art der Identitaet und nicht nach dem Unterbefehl'. Der Teil über den Unterbefehl stimmt;
die Verzweigung liest aber nicht die Art, sondern den Namen, und der Satz hat die falsche
Einordnung mitgetragen."

Der Kommentar in `main.rs` trägt beide Hälften desselben Satzes — den richtigen Teil über
den Unterbefehl und den falschen über die Art — und schickt den Leser mit dem falschen Teil
in die Funktion, deren Doc-Kommentar jetzt ausdrücklich das Gegenteil sagt
(`sign.rs:155-157`: „Die eine Verzweigung sagt dazu, was am **Namen** ablesbar ist, und
nicht mehr").

Wer diesen Kommentar liest und den Hinweis später erweitert, greift zur Identitätsart —
also zu dem `security`-Aufruf, den `260815-1444` ausdrücklich verworfen hat.

## Vorschlag

Den Halbsatz auf das umstellen, was die Funktion tut: was der Hinweis sagt, hängt am Namen
der Identität und nicht an ihrer Art. Ein Satz, und derselbe Verweis bleibt stehen.

## Abgrenzung

Der Rest des Kommentars stimmt und ist mitgeprüft: `bundle::bauen` hat außer `main.rs:140`
nur `messen.rs:45` als Rufer, `release::ausfuehren` geht nicht durch `bundle::bauen`, und
die Bemerkung zur Architektur (`main.rs:148-152`) trifft zu.

## Ablage

Gemeinsamer Speicher. Betrifft den Bauweg des ganzen Projekts und nicht die Directive einer
Runde.
