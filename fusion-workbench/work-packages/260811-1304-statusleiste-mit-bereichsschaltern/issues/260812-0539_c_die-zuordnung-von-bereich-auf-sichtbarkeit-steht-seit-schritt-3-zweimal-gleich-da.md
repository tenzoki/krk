Die Zuordnung von Bereich auf Sichtbarkeit steht seit Schritt 3 zweimal gleichlautend da

---

`fenstermodell::sichtbar_in` und `aufteilung::sichtbar_im` sind seit Schritt 3 Zeile für Zeile
dieselbe Fallunterscheidung. Bis dahin unterschieden sie sich in genau einem Zweig
(`Bereich::Links => true` gegen das Feld), und dieser Unterschied war der Grund, dass es zwei gab.
Der Grund ist weg, die zweite Fassung steht noch. Der Dokumentationskommentar der ersten nennt sich
weiterhin "**Die eine Zuordnung** von einem `Bereich` auf sein Feld in `Sichtbarkeit`", und das
stimmt nicht mehr.

---

**Schwere:** niedrig (kein falsches Verhalten heute; eine Stelle, die beim nächsten Feld zweimal
nachzuziehen ist, und ein Kommentar, der das Gegenteil des Codes sagt)
**Gefunden:** coderev, Durchsicht der Commits `5e17c9e`, `a2ea876`, `8ffaac2`
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs:238` (`sichtbar_in`),
`crates/krk-ui/src/appkit/aufteilung.rs:454` (`sichtbar_im`)
**Domain:** code

## Die beiden Fassungen

```rust
// crates/krk-ui/src/fenstermodell.rs:238
pub fn sichtbar_in(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich {
        Bereich::Lesezeichen => sichtbar.lesezeichen,
        Bereich::Links => sichtbar.erstes_dateifenster,
        Bereich::Rechts => sichtbar.zweites_dateifenster,
        Bereich::Vorschau => sichtbar.vorschau,
        Bereich::Editor => sichtbar.editor,
    }
}

// crates/krk-ui/src/appkit/aufteilung.rs:454
fn sichtbar_im(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich { /* dieselben fuenf Zeilen */ }
}
```

`aufteilung.rs` spricht `crate::fenstermodell` bereits an (`use crate::fenstermodell::{Bereich,
Zeilenmass};`), und `sichtbar_in` ist `pub`. Der Zugriff kostet also nichts weiter als das
Erweitern der `use`-Zeile; `sichtbar_im` und sein Dokumentationskommentar können ersatzlos
entfallen, der eine Aufrufer steht in `Aufteilung::anwenden` (`aufteilung.rs:252`).

## Warum das hier zählt

Das Projekt hält vier vollständige Fallunterscheidungen über `Bereich` ausdrücklich deshalb, weil
der Übersetzer bei einem sechsten Bereich an jeder von ihnen anhält und eine Einordnung erzwingt.
Zwei davon sind jetzt inhaltsgleich, und die eine sagt in ihrem Kommentar, sie sei die einzige. Wer
sich beim nächsten Feld auf diesen Satz verlässt, zieht eine Stelle nicht nach; der Übersetzer
fängt es zwar ab, aber der Kommentar hat dann bereits in die falsche Richtung gezeigt.

## Abgrenzung

`steht_im` (`aufteilung.rs:441`) bleibt, wovon dieser Datensatz nicht handelt: es fragt die
Unteransichten und nicht eine `Sichtbarkeit`, beantwortet also eine andere Frage. Sein Kommentar
sagt das schon heute richtig.

---

Resolved: 260812-0700, coder. `aufteilung::sichtbar_im` ist ersatzlos entfallen, samt seinem
Dokumentationskommentar; der eine Aufrufer in `Aufteilung::anwenden` ruft jetzt
`fenstermodell::sichtbar_in`, das der `use`-Zeile hinzugefügt ist. Damit stimmt der Satz „Die eine
Zuordnung" an jener Funktion wieder wörtlich, und er sagt seit dieser Änderung auch, dass es
einmal zwei waren und warum.

`steht_im` bleibt, wie der Datensatz es abgrenzt: es fragt die Unteransichten und nicht eine
`Sichtbarkeit`.

**Eine Probe hält es fest:**
`fenstermodell::tests::die_zuordnung_von_bereich_auf_sichtbarkeit_trifft_jedes_feld` setzt der
Reihe nach genau ein Feld und verlangt, dass `sichtbar_in` für alle fünf Bereiche antwortet und
keine zwei auf dasselbe Feld zeigen. Ein sechster Bereich hält weiterhin den Bau an, jetzt aber an
einer Stelle statt an zweien.

**Bei der Gelegenheit ist eine zweite Zuordnung derselben Art zusammengelegt worden**, die noch
niemand gemeldet hatte: `Fenstermodell::breite` schrieb die Abbildung von einem `Bereich` auf sein
Feld in `Breiten` aus, und `wuensche_nachfuehren` hätte sie ein zweites Mal gebraucht. Sie steht
jetzt als freie Funktion `breite_in` neben `sichtbar_in`, und die Methode ruft sie.

Abgenommen mit `make check`, exit 0.
