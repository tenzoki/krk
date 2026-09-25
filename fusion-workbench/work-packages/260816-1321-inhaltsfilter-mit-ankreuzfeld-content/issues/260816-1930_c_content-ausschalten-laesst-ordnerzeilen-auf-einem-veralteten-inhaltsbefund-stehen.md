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

---
Resolved: 260816-2230, gemeinsam mit `260816-1931` und `260816-1933` an ihrer Wurzel behoben. Nicht der Zweig in `inhalt_setzen` ist geändert, sondern die Regel dahinter: **ein Befund gilt nur zu der Frage, die ihn erzeugt hat.** Die Frage besteht aus zwei Größen — dem kleingeschriebenen Filtertext und der Angabe, ob der Inhalt mitzählt —, und beide sind genau die Angaben, mit denen `Durchlauf::starten` losläuft. Ändert sich eine, fällt der ganze Vektor auf `Unentschieden`.

Die beiden unsymmetrischen Zweige in `tief_setzen` und `inhalt_setzen` sind dafür durch **eine** Stelle ersetzt, `Ordnermodell::schalter_setzen` (`crates/krk-core/src/verzeichnis/modell.rs`): sie merkt sich `inhalt_wirkt()` vor dem Umlegen, vergleicht danach und setzt zurück, wenn der Wert gekippt ist. Der falsche Doc-Kommentar „weil ihn dann niemand liest" ist damit an beiden Stellen weg, samt der Begründung, die ihn getragen hat.

**Zwei Nebenwirkungen, beide gewollt.** Erstens setzt `tief_setzen(true)` nicht mehr blind zurück: der Stand der tiefen Suche entscheidet, ob die Frage für einen Ordner gestellt wird, und nicht, wie sie ausgeht — er wirft deshalb keine gültige Antwort mehr weg, außer wenn er die Schwelle des Inhaltsfilters kreuzt. Zweitens fällt beim Ausschalten von „Content" auch eine Ordnerzeile mit, die auf einem **Namens**treffer unter sich stand, und kommt mit dem neuen Lauf wieder. Das ist der Preis, den der Datensatz oben benennt; er ist angenommen, weil der Vektor sagt, *dass* etwas darunter lag, und nicht *warum*. Ihn nach dem Grund zu fragen hieße, den Grund über den Befundkanal zu melden und aus einem Wahrheitswert je Auftrag zwei zu machen.

Prüfbar gemacht in `crates/krk-core/tests/verzeichnis.rs`: `das_ausschalten_des_inhaltsfilters_nimmt_auch_die_ordnerzeile_sofort_weg` (die Zusage), `das_ausschalten_nimmt_auch_eine_namentlich_begruendete_ordnerzeile_mit` (der Preis), `das_ausschalten_nimmt_die_inhaltszeilen_weg_und_setzt_den_befund_zurueck` (die umgeschriebene alte Probe) und `ein_befund_gilt_nur_zu_seiner_frage` (die Regel; ersetzt `der_befund_faellt_bei_jeder_aenderung_der_frage_zurueck`). Die Abnahmeliste `messungen/260816-abnahme-inhaltsfilter.md` führt dafür die neuen Beobachtungen **26** und **27** und die nachgezogenen **17** und **21**.

Abnahme: `make check` — exit 0.
