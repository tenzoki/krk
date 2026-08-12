Die Aufzählung `Inhaltsart` wird nur über `matches!` gelesen und hält den Bau bei einer dritten Variante nicht an

---

`Inhaltsart` (`markdown.rs:446-459`) ist die Aufzählung, an der beide neuen
Deckungssätze hängen. Sie hat zwei Werte und keinen Auffangzweig — das ist
richtig. Gelesen wird sie aber an beiden Stellen über `matches!`, und ein
`matches!` trägt einen stillen `_ => false`-Zweig. Eine dritte Variante würde
übersetzen, ohne dass der Bau eine einzige Stelle nennt, und stillschweigend
wie `Zeichen` behandelt, also **nicht** gedeckt.

---

**Gelesen** (`crates/krk-ui/src/markdown.rs`, Stand `c35f8b1`; die beiden
einzigen Lesestellen):

```
markdown.rs:658   matches!(eintrag.inhalt, Inhaltsart::Bloecke),
markdown.rs:796   matches!(eintrag.inhalt, Inhaltsart::Bloecke).then_some(eintrag.quelle.end)
```

Kein `match` über `Inhaltsart` im Baum:

```
grep -n "Inhaltsart" crates/krk-ui/src/markdown.rs
-> zwoelf Treffer: Definition, Feld, fuenf Erzeugerstellen, zwei matches!,
   drei Signatur-/Doc-Stellen. Kein match-Ausdruck.
```

**Warum das hier zählt.** `CLAUDE.md` führt unter „Was man nicht sieht" den
Satz: „Etliche Fallunterscheidungen sind vollständig und haben keinen
Auffangzweig. Das ist Absicht: eine neue Variante hält den Bau an und erzwingt
eine bewusste Einordnung." Genannt sind dort vier gewachsene Aufzählungen —
`Wirkungsbereich`, `Kommando`, `Bereich`, `Fokus` — und drei Stellen, die
jedes neue Kommando nachziehen muss. `Auszeichnung` in
`crate::hervorhebung` trägt denselben Satz in seinem eigenen Doc-Kommentar
(`hervorhebung.rs:279-281`) und wird in `textmerkmale.rs:206-217` auch
tatsächlich erschöpfend gematcht. `Inhaltsart` ist die erste Aufzählung dieser
Art, bei der der Mechanismus nicht greift.

**Der Fall ist nicht hypothetisch.** Der Doc-Kommentar von `Inhaltsart` sagt
selbst, die Unterscheidung sei „die von CommonMark zwischen einem
Containerblock und einem Blattblock". CommonMark kennt mit der
Fußnotendefinition und der Definitionsliste weitere Containerblöcke; beide
sind heute nur deshalb kein Fall, weil `Options::empty()` sie abschaltet
(`markdown.rs:184`). Wer eine Option einschaltet, bekommt vom Übersetzer keine
Liste der nachzuziehenden Stellen, sondern eine still falsche Deckung.

**Ein Zuschnitt** (nicht gewählt): beide Lesestellen könnten statt `matches!`
eine Methode `Inhaltsart::deckt_luecken(self) -> bool` mit einem
erschöpfenden `match` rufen. Das ist eine Stelle statt zwei, und sie hält den
Bau an.

**Gewicht: niedrig.** Heute ist nichts falsch: beide Varianten sind belegt,
und die Zuordnung ist geprüft — `Bloecke` für Zitatblock, Liste und
Listenpunkt, `Zeichen` für Absatz, Überschrift, Quelltextblock, Betonung,
Verweis und den Quelltext in der Zeile, alles Übrige geht über
`Behandlung::Woertlich` und wird gar nicht erst geöffnet. Der Befund ist die
fehlende Sperre für die nächste Variante.

**Herkunft:** Circle der Runde 6, Turn 4, `c35f8b1`.
