Der Abschluss von `260817-1107` begründet die zwei ungeprüften Eigenschaften zu weit

---

Der Datensatz ist auf `_c_` gesetzt mit der Begründung, für die beiden verbleibenden
Eigenschaften sei „am Code dafür nichts mehr zu tun". Der Teil, der wirklich den Vordergrund
verlangt, ist allein die Antwort von AppKit auf einen echten Klick oder ein echtes `Esc`. Die
KRK-seitige Abbildung von „bestätigt" auf „Auftrag mit dieser Auswahl" steht als Abschlussblock
in `anwendung.rs` und ist derselbe Schnitt, den dieselbe Aufgabe für die Vorstufen gerade
gemacht hat.

---

**Schwere:** Niedrig. Kein Fehlverhalten am Code, und der Abschluss ist der Sache nach
berechtigt: die Hauptarbeit des Datensatzes ist geleistet, und zwei der vier Eigenschaften sind
mit einer Tafel über zwölf Fälle geprüft. Der Befund gilt der Begründung, die eine Aussage
über den Code trifft, die ein Gegenbeispiel hat.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `issues/260817-1107_c_der-rumpf-der-schutzschwelle-traegt-keine-probe.md`
(Abschlussnotiz), `crates/krk-ui/src/appkit/anwendung.rs:4700-4728`
**Baumstand:** `ee85950`
**Domain:** code

## Was der Datensatz sagt

> Ungeprüft bleiben „ein Abbruch stellt keinen Auftrag" und „der bestätigte Auftrag trägt die
> gezeigte Auswahl": beide sind Aussagen über den Rückruf des Blattes […] Ein Blatt lässt sich
> unter `libtest` nicht bedienen, und **am Code ist dafür nichts mehr zu tun**; sie gehören in
> den Abnahmelauf.

Der erste Halbsatz stimmt. Der zweite ist zu weit.

## Was am Baum steht

```rust
// crates/krk-ui/src/appkit/anwendung.rs:4712-4724, der Abschlussblock des Blattes
move |bestaetigt| {
    let Some(selbst) = schwach.load() else { return; };
    *selbst.ivars().offenes_blatt.borrow_mut() = None;
    if !bestaetigt { return; }
    let Some((art, auswahl, quellordner)) = bestaetigter.take() else { return; };
    selbst.loeschauftrag_stellen(art, auswahl, quellordner);
}
```

Drei Dinge in einem Abschluss: das Zurücksetzen von `offenes_blatt`, die Frage nach `bestaetigt`
und die Entnahme aus der `Cell`. Das erste braucht AppKit. Das zweite und das dritte sind
**genau die beiden ungeprüften Eigenschaften**, und beide sind reine Rechnung über
`(bestaetigt, Option<(Art, Auswahl, PathBuf)>)`.

Was daran den Vordergrund verlangt, ist eine Schicht darüber: dass AppKit auf einen Klick, auf
`Return` und auf `Esc` den Rückgabewert liefert, den KRK erwartet. Was **nicht** den Vordergrund
verlangt, ist die Abbildung von diesem Rückgabewert auf den Auftrag — und die ist heute schon
zur Hälfte geprüft: `loeschbestaetigung::tests::eine_unbekannte_antwort_stellt_keinen_auftrag`
und `die_ausfuehrende_stelle_zeigt_auf_die_ausfuehrende_schaltflaeche` (`:155`, `:171`) halten
fest, dass `stelle == AUSFUEHRENDE_STELLE` genau die ausführende Schaltfläche trifft. Der
Schritt danach, von `bestaetigt` zum Auftrag, ist der ungeprüfte.

## Richtung

Der Spiegelschnitt zu `vor_der_rueckfrage`. Eine reine Funktion in
`kommandos::loeschwarnung`, etwa

```rust
pub enum Nachstufe { KeinAuftrag, Auftrag }
#[must_use] pub fn nach_der_rueckfrage(bestaetigt: bool, traegt_auswahl: bool) -> Nachstufe
```

mit ausgeschriebener Tafel über vier Fälle, und der Abschlussblock ruft sie. Damit wären beide
Eigenschaften mit derselben Bauform geprüft, mit der dieselbe Aufgabe die Vorstufen prüfbar
gemacht hat, und der Abnahmelauf müsste nur noch belegen, was allein er belegen kann: dass
`Esc`, `Return` und der Klick beim Blatt richtig ankommen.

Ob der Schnitt die Kosten wert ist, entscheidet der Nutzer oder der nächste Plan. Verlangt ist
hier nur, dass die Abschlussnotiz das sagt, statt zu sagen, es sei nichts mehr zu tun. Eine
Aussage über den Code, die ein Gegenbeispiel hat, liest die nächste Runde als bindend.
