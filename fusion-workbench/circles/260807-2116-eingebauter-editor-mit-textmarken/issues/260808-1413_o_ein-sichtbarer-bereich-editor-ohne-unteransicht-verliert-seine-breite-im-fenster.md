# Ein sichtbarer `Bereich::Editor` ohne Unteransicht verliert seine Breite im Fenster

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde (`git diff 4e86c02..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/aufteilung.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/fenstermodell.rs`
**Cross-references:** Plan S13 (Umsetzungsvermerk), S16, S18, `crates/krk-core/src/ablage/sitzung.rs:190-215`

---

## Der Befund

S13 und S14 haben `Bereich::Editor` und `Sichtbarkeit::editor` in das Modell
gehoben. Die Unteransicht, die dazu gehört, hängt S16 ein. Zwischen beiden
Schritten kann die Sichtbarkeit des Editors auf `true` stehen, ohne dass es eine
Ansicht gibt, die diese Sichtbarkeit tragen könnte, und die Aufteilung rechnet
dann mit einem Bereich, den sie anschließend überspringt.

`crates/krk-ui/src/appkit/aufteilung.rs:331-354`:

```rust
fn auslegen(teiler: &NSSplitView, breiten: &Breiten, sichtbar: &Sichtbarkeit) {
    let gesamt = teiler.frame().size;
    let sichtbare = Bereich::ALLE
        .iter()
        .filter(|bereich| sichtbar_im(sichtbar, **bereich))   // zählt den Editor mit
        .count();
    let trenner = teiler.dividerThickness() * (sichtbare.saturating_sub(1)) as f64;
    let verfuegbar = (gesamt.width - trenner).max(0.0);
    let zugeteilt = crate::fenstermodell::bereichsbreiten(verfuegbar, breiten, sichtbar);

    let mut links = 0.0;
    for bereich in Bereich::ALLE {
        let Some(ansicht) = bereichsansicht(teiler, bereich.index()) else {
            continue;                                          // überspringt den Editor
        };
        ...
    }
}
```

Beides zusammen: `sichtbare` ist 5, also wird ein Trenner zu viel abgezogen;
`bereichsbreiten` weist dem Editor seine Anfangsbreite von 460 Punkten zu
(`fenstermodell.rs:132-139`), und die Schleife setzt sie nirgends. Die vier
Bereiche, die es wirklich gibt, bekommen zusammen `gesamt.width - trenner - 460`
Punkte.

## Wie die Sichtbarkeit heute auf `true` kommt

**Von Hand.** `session.toml` ist nach C7 zum Lesen und Ändern von Hand gedacht,
und `Sichtbarkeit` liest `editor` mit `#[serde(default)]` ein
(`crates/krk-core/src/ablage/sitzung.rs:203-215`). Eine Zeile `editor = true`
unter `[sichtbar]` genügt. Das ist keine erfundene Lage: `Fenstermodell::aus_sitzung`
(`fenstermodell.rs:188-199`) sichert ausdrücklich gegen genau diese Klasse von
Hand-Einträgen ab, mit der Begründung "`aktiv = "rechts"` neben
`zweites_dateifenster = false` ist ein Paar, das `serde` anstandslos einliest".
Für `editor = true` ohne Unteransicht steht die entsprechende Zeile nicht da.

**Ab S5/S6 zusätzlich über die Tastatur.** `fokus_holen`
(`crates/krk-ui/src/appkit/anwendung.rs:1063-1070`) blendet den Bereich **vor**
dem Fokussetzen ein:

```rust
let eingeblendet = match fokus::holt_hervor(ziel) {
    Some(bereich) => self.bereich_einblenden(bereich),
    None => false,
};
let gesetzt = self.fokus_setzen(ziel);
```

`holt_hervor(Fokus::Editor)` liefert seit S3 `Some(Bereich::Editor)`
(`crates/krk-ui/src/kommandos/fokus.rs:147`). Der Platzhalter
`Fokus::Editor => false` in `fokus_setzen` (`anwendung.rs:1110`) weist nur die
zweite Hälfte ab; das Einblenden ist da schon geschehen. Sobald S5 das Kommando
`FokusEditor` bringt und S6 es belegt — beide in Phase A, also **vor** S16 in
Phase C — ist der Fall über einen Tastendruck erreichbar.

## Zweiter Teil desselben Befunds: zwei Antworten auf dieselbe Frage

`sichtbar_im` (`aufteilung.rs:284-292`) beantwortet "ist der Editor sichtbar" aus
dem Modell und sagt heute möglicherweise `true`. `gemessene_sichtbarkeit`
(`aufteilung.rs:316-325`) beantwortet dieselbe Frage aus dem Bildschirm und sagt
**immer** `false`, weil `bereichsansicht(teiler, 4)` ohne Unteransicht `None`
liefert. Beide speisen dasselbe `auslegen`:

- `Aufteilung::anwenden` (`aufteilung.rs:182-189`) übergibt die Modellsicht.
- `neu_auslegen` (`aufteilung.rs:99-104`), der Weg bei jeder Fenstergrößenänderung,
  übergibt die gemessene.

Damit legt dieselbe Fensterzeile je nach Auslöser unterschiedlich aus.
`inference:` Welches Ergebnis der Nutzer am laufenden Bündel zuletzt sieht, hängt
davon ab, welcher der beiden Wege zuletzt lief; das ist ohne Vordergrundlauf
nicht zu entscheiden. Gesichert und aus dem Code allein ablesbar ist der
Widerspruch selbst.

## Was zu tun ist

Zwei Wege stehen zur Wahl; die Entscheidung gehört zu S16 oder S18, nicht in
diesen Defekt.

1. **`sichtbar_im` fragt zusätzlich, ob die Aufteilung den Bereich trägt.** Eine
   Zeile in `auslegen`, die den Filter über `bereichsansicht(...).is_some()`
   führt, hielte Zähler und Zuteilung an derselben Wahrheit wie die Schleife
   darunter.
2. **`Fenstermodell::aus_sitzung` stellt die Zusicherung her**, so wie es sie für
   das aktive Dateifenster schon herstellt: ein Editor ohne gehaltene Datei ist
   nicht sichtbar. Das trägt zugleich die Begründung, die `Sichtbarkeit::default`
   in `sitzung.rs:220-234` schon ausschreibt.

Der gegenseitige Ausschluss aus S18 löst den Befund **nicht** auf: er sorgt
dafür, dass Editor und Vorschau nicht zugleich stehen, nicht dafür, dass ein
sichtbarer Editor eine Ansicht hat.
