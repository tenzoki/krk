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

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-1210 beantwortet` — der Doppelklick meldet kuenftig auch; das Abnahmekriterium verlangt eine Meldung fuer den nicht lesbaren Pfad, und diese Wahl erfuellt sie auf beiden Wegen. Der zusaetzliche Systemaufruf auf jedem Ordnereinstieg ist mitentschieden und gehoert in den naechsten Abnahmelauf, weil zwei Zeitzusagen daran haengen; ruled by user, Kai Stalmann <kai@stalmann.org>.

---
Implemented: `crates/krk-core/src/verzeichnis/verweisziel.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/kommandos/pfadeingabe.rs`, `crates/krk-ui/src/tabs.rs` — die gewaehlte Antwort ist erfuellt, und **ohne** den mitentschiedenen Systemaufruf je Ordnereinstieg. **Die Praemisse dieses Datensatzes war falsch:** der Doppelklick war nie wortlos. `Schwungleser::oeffnen` scheitert mit `EACCES`, der Lesefaden meldet `Abschluss::Fehler`, und `tabs::lesemeldungen_einziehen` macht daraus die Tabmeldung „… ließ sich nicht vollständig lesen: Permission denied“; der Zweig steht seit `537fda5` (S12) im Baum, also seit vor diesem Datensatz. Gemessen von der neuen Probe `tabs::tests::ein_ordner_ohne_leserecht_meldet_sich_aus_dem_lesevorgang`. Die Zeitzusagen L3 und L10 bleiben damit unberuehrt und brauchen keinen Abnahmelauf aus diesem Anlass. Berichtigt ist die zweite Stelle, die dasselbe behauptete, der Modulkopf von `verweisziel.rs`; der Befund dazu ist `260907-1226_*_zwei-prosastellen-nennen-den-doppelklick-auf-einen-ordner-ohne-leserecht-wortlos-der-baum-meldet-seit-s12.md`. Was von der Ungleichheit bleibt — der Pfadsprung weist ab, der Doppelklick geht hinein und meldet danach — steht als neue Frage in `260907-1226_*_weist-der-doppelklick-auf-einen-ordner-ohne-leserecht-ab-oder-geht-er-hinein-und-meldet-danach.md`.
