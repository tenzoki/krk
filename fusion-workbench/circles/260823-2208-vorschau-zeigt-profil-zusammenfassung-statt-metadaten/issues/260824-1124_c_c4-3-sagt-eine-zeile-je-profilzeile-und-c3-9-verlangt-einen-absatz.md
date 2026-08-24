C4.3 sagt „eine Zeile" je Profilzeile, und C3.9 verlangt vom Feldbaustein einen Absatz

---

C4.3 lautet: „Jede Profilzeile erscheint als eine Zeile aus Beschriftung und Wert, in der
Reihenfolge der Datei. Der Baustein „jüngste N" erscheint als Block aus bis zu N Zeilen unter
seiner Beschriftung." Genannt ist damit **ein** Baustein, der mehr als eine Zeile belegen darf.
C3.9 verlangt aber vom Feldbaustein ausdrücklich einen Absatz: „der Wert ist der Absatz und
nicht seine erste Zeile", und ein Absatz eines Circle-Datensatzes dieser Werkbank steht auf
vier Zeilen. Ein Feldwert kann damit mehrzeilig sein, und C4.3 sagt in ihrem Wortlaut, dass er
es nicht ist.

---

**Gemessen am Baumstand nach Schritt 6, am 260824-1124.**

## Wo der Wortlaut auf einen Bestand trifft, der ihn nicht trägt

Nachgezählt über alle Circle-Datensätze dieser Werkbank:

```sh
for f in fusion-workbench/circles/*/_*_circle.md; do …; done
```

Von achtzehn Circle-Datensätzen trägt genau einer seine Directive auf mehr als einer Zeile,
nämlich `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md` mit vier
Zeilen. Der Fall ist also selten und tritt am mitgelieferten Profil dieser Runde auf, nicht in
einem ausgedachten Beispiel.

## Wie Schritt 6 ihn gelöst hat

`Zusammenfassung::als_text` (`crates/krk-core/src/leseprofil/mod.rs`) trifft eine
überschneidungsfreie und vollständige Unterscheidung mit zwei Fragen: `Wert::Titel` steht immer
unter seiner Beschriftung, weil C4.3 den Block verlangt, und jeder andere Wert genau dann, wenn
er selbst mehr als eine Zeile trägt. Ein einzeiliger Feldwert steht hinter der Beschriftung,
ein mehrzeiliger darunter, eingerückt wie die Titel.

Die Alternative wäre gewesen, den Absatz hinter die Beschriftung zu setzen und seine
Folgezeilen am linken Rand stehen zu lassen; dort liefe er in die Beschriftung der nächsten
Zeile hinein, und der Nutzer sähe zwei Angaben als eine. Die Probe dazu ist
`crates/krk-core/tests/leseprofil.rs::der_text_setzt_einzeilige_werte_hinter_und_mehrzeilige_unter_die_beschriftung`.

## Was zu tun ist

Der Bau ist entschieden, die Buchführung nicht. C4.3 ist eines der Abnahmekriterien, an denen
`## Where this Circle stops` die Runde misst; in ihrem heutigen Wortlaut wäre der zweite Block
als Abweichung abzuhaken, obwohl er C3.9 einlöst. Zu berichtigen ist der Satz im Spec, nicht
der Code: „Jede Profilzeile erscheint als eine Zeile aus Beschriftung und Wert; ein Wert, der
mehr als eine Zeile trägt, steht eingerückt unter seiner Beschriftung, und der Baustein
„jüngste N" tut das immer."

**Schwere:** gering. Kein Fehlverhalten, keine Bauarbeit. Eine Aussage über die Anzeige, die
enger ist als die Anzeige.

**Gefunden:** coder, bei der Umsetzung von Schritt 6 am 260824-1124.

**Betroffen:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`,
`crates/krk-core/src/leseprofil/mod.rs`

**Domain:** code

---
Resolved: C4.3 des Specs traegt seit dem 260824-1224 den Wortlaut, den dieser Datensatz vorschlaegt: ein einzeiliger Wert steht hinter seiner Beschriftung, ein mehrzeiliger eingerueckt darunter, und der Baustein „juengste N" tut das immer. Die Berichtigung steht unter der Kriterienliste von C4 und nennt die verworfene Alternative mit. Kein Code ist angefasst; `Zusammenfassung::als_text` bleibt, wie Schritt 6 es gebaut hat. Die Berichtigung aendert ein freigegebenes Abnahmekriterium inhaltlich und ist dem Nutzer vorzulegen.
