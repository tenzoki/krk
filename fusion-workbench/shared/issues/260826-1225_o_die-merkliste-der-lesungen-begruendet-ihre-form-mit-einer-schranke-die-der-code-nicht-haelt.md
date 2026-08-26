Die Merkliste der Lesungen begruendet ihre Form mit einer Schranke, die der Code nicht haelt

---

`Lauf::staende` (`crates/krk-core/src/leseprofil/bausteine.rs:354`) ist eine Liste und keine
Abbildung, und der Doc-Kommentar darueber begruendet das so: „es sind hoechstens so viele
Eintraege, wie [`super::HOECHSTENS_LESELAEUFE`] zulaesst, und bei zwoelf davon ist das
Durchgehen der guenstigere Weg als das Streuen" (`:342-344`). Die Schranke haelt nicht.
`Lauf::stand_am` (`:380-393`) legt fuer **jeden verschiedenen Ort** einen Eintrag an, auch fuer
die, an denen gar nicht gelesen wurde, weil der Haushalt erschoepft war. Die Zahl der Eintraege
folgt der Zahl der Ortsangaben im Profil und nicht `HOECHSTENS_LESELAEUFE`.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/leseprofil/bausteine.rs:336-344` (Doc-Kommentar),
`:380-393` (`stand_am`)
**Tree state:** `004ff72`
**Domain:** code

## Der Beleg steht als Probe im Baum

```rust
// crates/krk-core/src/leseprofil/bausteine.rs:380-393
fn stand_am(&self, ort: &Ort) -> Option<Rc<Lesestand>> {
    …
    let stand = self.lesen(ort).map(Rc::new);        // liefert None, wenn der Haushalt leer ist
    self.staende.borrow_mut().push((ort.clone(), stand.clone()));   // gemerkt wird trotzdem
    stand
}
```

`lesen` (`:417-425`) gibt `None` zurueck, sobald `buchen(Haushalt::leselauf_nehmen)` verneint;
der Eintrag wird dann mit `None` gemerkt, und das ist beabsichtigt und im Kommentar bei `:338`
auch ausgeschrieben („Auch das wird gemerkt, denn ein zweiter Versuch am selben Ort scheiterte
genauso"). Genau daraus folgt aber, dass die Zahl der Eintraege nicht an der Zahl der
gelungenen Lesungen haengt.

`crates/krk-core/tests/leseprofil.rs:2673`
(`dreizehn_zaehlbausteine_erreichen_die_grenze_und_der_rest_traegt_den_platzhalter`) faehrt
`HOECHSTENS_LESELAEUFE + 1 = 13` Zeilen mit je eigenem Ort und dazu den erkannten Ordner. Die
Liste traegt in diesem Lauf 14 Eintraege, waehrend `HOECHSTENS_LESELAEUFE` auf 12 steht. Die
Probe selbst prueft den Haushalt und nicht die Liste, faellt also nicht.

## Was zu tun ist

Kein Verhalten ist betroffen: die Liste bleibt klein, weil ein Profil wenige Ortsangaben
traegt, und `dreizehn_zaehlbausteine…` ist die einzige Stelle im Baum, die sie ueber zwoelf
treibt. Zu berichtigen ist die **Begruendung**: die Schranke ist die Zahl der verschiedenen
Ortsangaben des erkannten Profils und nicht `HOECHSTENS_LESELAEUFE`. Dieses Projekt fuehrt eine
Zusage, die nur ein Kommentar haelt und die der Code nicht haelt, als Defekt; wer die Liste
spaeter gegen eine Abbildung tauschen will, rechnet sonst mit der falschen Groesse.

**Gefunden:** coderev, Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
260826-1225.
