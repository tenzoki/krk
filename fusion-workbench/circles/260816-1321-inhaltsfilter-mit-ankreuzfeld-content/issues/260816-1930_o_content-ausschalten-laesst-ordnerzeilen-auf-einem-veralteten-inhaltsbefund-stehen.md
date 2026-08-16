Das Ausschalten von „Content" bei eingeschaltetem „Deep" lässt Ordnerzeilen auf einem veralteten Inhaltsbefund stehen

---

C2.9 verlangt, dass das Ausschalten von „Content" **sofort** auf die Liste wirkt und die
Zeilen verschwinden, die allein wegen ihres Inhalts standen. Für eine Datei tut es das. Für
einen **Ordner** bei eingeschaltetem „Deep" tut es das nicht: seine Zeile bleibt stehen,
solange der neue Durchlauf sie nicht neu entschieden hat, und das dauert einen ganzen
Unterbaum lang.

`Ordnermodell::inhalt_setzen` (`crates/krk-core/src/verzeichnis/modell.rs:806-822`) setzt
den Befundvektor nur beim **Einschalten** zurück:

```rust
pub fn inhalt_setzen(&mut self, inhalt: bool) {
    if inhalt && !self.inhalt {
        self.befund_zuruecksetzen();
    }
    self.inhalt = inhalt;
    self.sicht_neu_aufbauen();
}
```

Die Begründung am Doc-Kommentar darüber (`modell.rs:812`) lautet: „Beim Ausschalten bleibt
der Vektor stehen, weil ihn dann **für eine Datei** niemand liest." Das stimmt für Dateien
und übersieht die Ordner. Der Pruefschritt liest denselben Vektor auch für einen Ordner,
und dort hängt er nicht an `inhalt`, sondern an `tief`
(`crates/krk-core/src/verzeichnis/modell.rs:621-632`):

```rust
if !self.tief {
    return true;
}
match self.befund(index as u32) {
    Befund::Treffer => true,
    Befund::Unentschieden | Befund::KeinTreffer => false,
}
```

**Der Ablauf, Schritt für Schritt am Baum gelesen:**

1. „Deep" an, „Content" an, Filtertext fünf Zeichen. Unter `ordner1` liegt genau eine
   Datei, deren **Text** die Folge trägt, und kein Namenstreffer. Der Durchlauf meldet
   `ordner1` als `Befund::Treffer` (`durchlauf.rs:527-529`), die Zeile steht.
2. Der Nutzer schaltet „Content" aus. `inhalt_setzen(false)` läuft ohne Rücksetzen durch,
   `sicht_neu_aufbauen` fragt `sichtbar`, der Ordnerzweig liest den alten `Treffer` — die
   Zeile steht weiter, obwohl ihr einziger Grund gerade abgeschaltet wurde.
3. `DateifensterQuelle::inhaltssuche_umschalten`
   (`crates/krk-ui/src/appkit/tabelle.rs:2081-2096`) stößt danach über
   `durchlauf_nachziehen` einen neuen Lauf mit `inhaltsgrenze: None` an. Erst dessen
   Befund für `ordner1` räumt die Zeile weg.

**Die Spanne ist der ganze Unterbaum.** Der Durchlauf arbeitet die Aufträge der Reihe nach
ab (`durchlauf.rs:330`), `ordner1` kann der letzte sein, und der Modulkopf von
`durchlauf_nachziehen` beziffert einen tiefen Lauf selbst mit „minutenlang"
(`crates/krk-ui/src/tabs.rs:839-841`). Solange zeigt die Liste einen Zustand, den der
Schalter widerlegt.

**Dieselbe Lücke steht spiegelbildlich bei `tief_setzen`.** Sein Doc-Kommentar
(`modell.rs:787`) sagt „Beim Ausschalten bleibt der Vektor stehen, weil ihn dann niemand
liest" — seit dieser Runde liest ihn der Dateizweig über `inhalt_entscheidet`, gleich wie
`tief` steht. Der Satz ist damit unabhängig von der Behebung falsch geworden.

---

**Was die Behebung abwägen muss, und deshalb steht hier keine Vorschrift.** Ein
Rücksetzen auch beim Ausschalten (`if inhalt != self.inhalt && self.tief`) macht die
Aussage sofort richtig, lässt dafür jede tief gefundene Ordnerzeile verschwinden und
während des neuen Laufs wieder erscheinen. Das ist derselbe Handel, den `tief_setzen` beim
Einschalten schon eingeht; ob er beim Ausschalten von „Content" auch der richtige ist, ist
eine Anzeigefrage und keine reine Codefrage. Wird er nicht eingegangen, gehört die Grenze
von C2.9 benannt: sie gilt für Dateien sofort und für Ordner erst nach dem neuen Lauf.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.
Verwandt: `issues/260816-1710_o_ein-rueckwechsel-auf-einen-tab-setzt-seinen-beendeten-durchlauf-nicht-fort.md`
(dieselbe Sorte Anzeige: eine Liste, die vollständiger aussieht, als sie ist).
