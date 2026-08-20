`text_schreiben` hat sein `#[must_use]` bei der Aufteilung nicht mitbekommen

---

Schritt 6 hat den Rumpf von `text_schreiben` nach
`text_auf_ablage_schreiben` gezogen (`crates/krk-ui/src/appkit/zwischenablage.rs:258-272`).
Die neue Funktion trägt `#[must_use]`, die verbliebene Hülle darüber nicht — obwohl beide
denselben Wahrheitswert liefern und für beide gilt, was der Doc-Kommentar sagt: „Der
Aufrufer meldet das in der Statuszeile; wortlos nichts zu tun ist in keinem der beiden
Fälle zulässig."

`CLAUDE.md` sagt unter „Was man nicht sieht": ein Rückgabewert, dessen stilles Fallenlassen
unbemerkt bliebe, bekommt `#[must_use]`. Genau das trifft hier zu.

---

**Kein Schaden heute:** beide Rufer werten den Wert aus
(`crates/krk-ui/src/appkit/tabelle.rs:1553` und `:1581`, je in einem `if`). Der Befund ist
die Ungleichheit, die diese Runde erzeugt hat: dieselbe Antwort ist auf dem einen Weg
geschützt und auf dem anderen nicht, und der ungeschützte ist der, den die beiden
Pfadkopierer aus der Runde 4 nehmen.

**Schwere:** niedrig.
**Baumstand:** `b28cdd6`.

---
Abgleich 260820-0834, gegen `05cb614`: **trifft unveraendert zu.** `zwischenablage.rs` ist seit
`b28cdd6` unberuehrt. Nachgelesen: `#[must_use]` steht in Zeile 258 ueber
`text_auf_ablage_schreiben`, und `text_schreiben` (`:270`) traegt keines, obwohl es denselben
Wahrheitswert weiterreicht. Der Befund bleibt offen.
