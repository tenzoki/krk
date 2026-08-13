Zwei Prosastellen in `anwendung.rs` zählen noch drei Werte der Lage und einen Sonderposten

---

Die Runde hat die `Lage` auf vier Felder und die Menüleiste auf zwei Sonderposten gebracht. Zwei Kommentare in `crates/krk-ui/src/appkit/anwendung.rs` zählen weiter die alten Zahlen. Beide halten den Bau nicht an und ändern kein Verhalten; beide stehen in einer Datei, die der bereits gemeldete Befund `260813-1420_o_vier-modulkoepfe-ausserhalb-der-dateiliste-von-a1-und-a2-nennen-noch-drei-bestandteile.md` **nicht** führt, weil der allein `menue.rs`, `kommandos/mod.rs` und `ereignisse.rs` aufzählt.

---

**Schwere:** niedrig.

**1. Der Zeichenzweig nennt drei Werte** (`crates/krk-ui/src/appkit/anwendung.rs:2549-2552`):

```rust
// **Dieselbe Erhebung wie im Kommandozweig, und dieselben drei
// Werte.** Ein getipptes Zeichen ist kein Kommando: es traegt
// keinen Wirkungsbereich, und `zulaessig` hat ihm nichts zu
// sagen. Die Eingaben der Frage braucht es trotzdem alle drei.
```

Der Kommandozweig fragt seit A1 vier Werte, der Zeichenzweig liest drei davon. „Dieselben drei Werte" ist damit falsch, „alle drei" ebenso: die Frage hat vier Eingaben. Der Doc-Kommentar an `lage` (`:2650`) sagt es bereits richtig („liest drei der vier Werte einzeln heraus"), dieser Kommentar zwanzig Zeilen darüber nicht.

**Am Verhalten ändert sich nichts, und das ist geprüft:** vor einem fremden Schlüsselfenster antwortet `fokus_bei` `Fokus::Anderswo`, und der `match` darunter (`:2565-2581`) liefert für `Anderswo` `false`. Eine vierte Bedingung im Zeichenzweig wäre eine zweite Fassung derselben Sperre; genau das sagt der Plan in seinem Abschnitt „Die Zulässigkeitsregel nach dieser Runde". Falsch ist allein die Zahl im Kommentar.

**2. Die Aufzählung an `validateMenuItem:` nennt einen Sonderposten** (`crates/krk-ui/src/appkit/anwendung.rs:733-735`):

```rust
/// Methode deshalb `true` und ueberlaesst AppKit seine gewohnte
/// Entscheidung; die sechs Textbefehle (C2.8) und der Eintrag der
/// Markdown-Ausgabe (C2.9) behalten damit genau das Verhalten, das sie
/// heute haben, und ihre Ausgrauung kommt weiter aus der Antwortkette.
```

Seit C1 fallen zwei Sonderposten in diesen Zweig, „Über KRK" und die Markdown-Ausgabe. Der Ausführer von Strang C hat die Stelle gesehen und in seinem Sitzungsbericht vermerkt (`history/260813-1244-coder-strang-c-ueber-krk-eintrag.md`, Abschnitt „Was aufgefallen ist und nicht in diesen Strang gehörte`), aber keinen Datensatz angelegt — `anwendung.rs` stand außerhalb der Dateiliste von C2. Der Modulkopf von `menue.rs` beschreibt den Zweig seit derselben Änderung richtig (`menue.rs:69-77`); die beiden Stellen widersprechen sich jetzt.

**Was zu tun ist**

Beide Kommentare nachziehen, sobald ein Schritt `anwendung.rs` ohnehin öffnet. Beim ersten: „dieselben drei Werte" auf „drei der vier Werte" und „alle drei" auf „drei davon"; dazu der Satz, warum der vierte hier nicht eigens gefragt wird. Beim zweiten: „der Eintrag der Markdown-Ausgabe (C2.9)" auf „die beiden Sonderposten (C2.9 und C5.1)".

**Kontext**

- Gefunden bei der Durchsicht von Turn 1 der Runde 8, Bereich `59b0a6c..21dbc59`.
- Schwesterbefund: `260813-1420_o_vier-modulkoepfe-ausserhalb-der-dateiliste-von-a1-und-a2-nennen-noch-drei-bestandteile.md`. Dessen Punkt 1 (`menue.rs`, „die Tafel aus 140 Faellen", heute Zeile 1132) steht weiter offen, obwohl Strang C `menue.rs` geöffnet hat; die dort vorgeschlagene Abhilfe („`menue.rs` fällt in Strang C dieser Runde an") ist nicht eingetreten.

---

**Abgleich 260813-1345: zu Recht offen, beide Stellen unverändert.** `anwendung.rs:2549-2552`
(heute `:2604-2607`) sagt weiter „dieselben drei Werte" und „alle drei"; `:733-735` (heute
`:733-736`) nennt weiter einen Sonderposten. Beide Feststellungen des Datensatzes am Baum
bestätigt, einschliesslich der Begründung, dass am Verhalten nichts hängt.

**Eine dritte Stelle in derselben Datei gehört dazu**, und sie ist ein Grenzfall:
`anwendung.rs:2690-2693` sagt „Bis zur Runde 7 standen hier zwei getrennte Vorbehalte … waehrend
der dritte Bestandteil im Ereignisabgriff wohnte; alle drei stehen jetzt in der einen Regel."
Der Satz beschreibt einen historischen Vorgang und zählt für ihn richtig; drei Zeilen darunter
steht „Die vier Bestandteile" (`:2695`). Als Umfangsangabe gelesen führt er trotzdem in die
Irre, und wer die beiden anderen Stellen anfasst, nimmt ihn mit — ein Halbsatz („die damals
zusammengeführten drei") genügt.

**Punkt 1 des Schwesterbefunds ist weiterhin offen.** `menue.rs:1132` sagt „die Tafel aus 140
Faellen", obwohl Strang C dieselbe Datei geöffnet und fünf andere Prosastellen darin nachgezogen
hat; die Tafel deckt 280 (`zulaessigkeit.rs:435`).
