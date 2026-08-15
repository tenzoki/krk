# Meldet der Doppelklick auf einen Ordner ohne Leserecht, oder schweigt er wie heute?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `shared/issues/260815-1713_*_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md` (der Anlass); `crates/krk-ui/src/kommandos/pfadeingabe.rs:67-77`; `crates/krk-ui/src/appkit/tabelle.rs:1417-1452`

---

## Question

Zwei Wege führen in denselben Ordner, und sie verhalten sich verschieden, wenn dieser Ordner
kein Leserecht trägt. Der Pfadsprung meldet es in der Statuszeile; der Doppelklick in der
Dateiliste wechselt wortlos in eine leere Liste.

Die zwei Fundstellen:

1. **`crates/krk-ui/src/kommandos/pfadeingabe.rs:67-77`** — der Pfadsprung meldet. Nachdem
   `std::fs::metadata` ein Verzeichnis gesagt hat, ruft `pruefen` zusätzlich
   `std::fs::read_dir` und gibt bei einem Fehlschlag `Ergebnis::Meldung` zurück. Der
   Kommentar darüber nennt den Grund: C2 verlangt eine Meldung für den nicht lesbaren Pfad,
   und „ein Ordnerwechsel in eine leere Liste waere die wortlose Variante".
2. **`crates/krk-ui/src/appkit/tabelle.rs:1417-1452`, `in_zeile_einsteigen`** — der
   Doppelklick schweigt. Ein `Typ::Ordner` geht ohne jede Prüfung an `ordner_lesen`; fehlt
   das Leserecht, bleibt die Liste leer und die Statuszeile stumm.

**Warum die Frage jetzt zu stellen ist.** Der Wechsel von `verweisziel::bestimmen` auf
`std::fs::metadata` am 260815 hat die Ungleichheit weder erzeugt noch verschoben, aber
sichtbar gemacht. Vorher fragte `bestimmen` mit `open(2)` und meldete deshalb als
Nebenwirkung auch dann, wenn das Ziel einer Verknüpfung ein Verzeichnis ohne Leserecht war —
ein Verhalten, das der gewöhnliche `Typ::Ordner` daneben nie hatte. Seit dem Wechsel verhält
sich die Verknüpfung wie das Verzeichnis selbst, also wortlos. Damit steht der Unterschied
sauber zwischen den zwei Wegen statt quer durch den Doppelklick hindurch, und ist genau
deshalb entscheidbar geworden.

## Options

1. **Der Doppelklick meldet künftig wie der Pfadsprung.** `in_zeile_einsteigen` prüft das
   Leserecht, bevor es `ordner_lesen` ruft, und schreibt sonst eine Zeile in die Statuszeile.
   - Pro: C2 verlangt eine Meldung für den nicht lesbaren Pfad, und diese Möglichkeit erfüllt
     sie auf beiden Wegen. Der Nutzer erfährt, warum die Liste leer bleibt.
   - Kontra: ein zusätzlicher Systemaufruf je Einstieg, auch im häufigen Fall des lesbaren
     Ordners. Betrifft die Zeitzusagen L3 und L10, die seit der Runde 4 nicht mehr gemessen
     sind.
2. **Der Pfadsprung schweigt künftig wie der Doppelklick.** Die `read_dir`-Prüfung in
   `pfadeingabe::pruefen` entfällt.
   - Pro: eine Regel statt zweier, ohne zusätzlichen Aufruf.
   - Kontra: nimmt eine Meldung weg, die C2 ausdrücklich verlangt, und der Kommentar an der
     Stelle begründet sie.
3. **Der Unterschied bleibt.** Die zwei Wege bedienen verschiedene Erwartungen: wer einen
   Pfad tippt, hat ihn vielleicht falsch getippt; wer auf eine Zeile klickt, sieht den
   Eintrag vor sich.
   - Pro: kostet nichts und ändert kein Verhalten.
   - Kontra: zwei Verhalten für dieselbe Lage, die niemand aus dem Code ablesen kann, ohne
     beide Wege zu vergleichen.

## Constraints

- Der Einstiegsweg darf keinen Systemaufruf je **Anzeige** hinzubekommen. Der
  Sortierschlüssel entsteht einmal beim Lesen, und daran hängen L3 und L10.
- Eine Antwort, die beide Wege angleicht, soll an einer Stelle stehen und nicht an zweien.
  `pfadeingabe.rs` nennt sich im Modulkopf „die eine Stelle, die einen Pfad prueft" und warnt
  vor einer zweiten Wahrheit daneben.

## Recommendation

Keine. Der Datensatz ist von einem Agenten gefiled worden, der die Frage beim Bauen gefunden
hat; welches Verhalten das richtige ist, ist eine Frage an den Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:
