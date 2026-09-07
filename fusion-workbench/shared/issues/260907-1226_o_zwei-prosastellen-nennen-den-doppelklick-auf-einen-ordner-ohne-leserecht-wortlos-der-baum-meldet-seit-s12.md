# Zwei Prosastellen nennen den Doppelklick auf einen Ordner ohne Leserecht wortlos; der Baum meldet seit S12

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`; `260907-1226_*_weist-der-doppelklick-auf-einen-ordner-ohne-leserecht-ab-oder-geht-er-hinein-und-meldet-danach.md`; `crates/krk-core/src/verzeichnis/verweisziel.rs`; `crates/krk-ui/src/tabs.rs` (`lesemeldungen_einziehen`)

---

## Befund

Zwei Stellen haben behauptet, ein Doppelklick auf einen Ordner ohne Leserecht
wechsle wortlos in eine leere Liste:

1. **Der Modulkopf von `crates/krk-core/src/verzeichnis/verweisziel.rs`**, letzter
   Abschnitt: „Ein Doppelklick auf einen gewoehnlichen `Typ::Ordner` ohne
   Leserecht ist heute wortlos".
2. **Der Entscheidungsdatensatz `260815-1749_*`**, sowohl in der Frage („der
   Doppelklick in der Dateiliste wechselt wortlos in eine leere Liste") als auch
   in der Fundstellenliste und in allen drei Möglichkeiten. Die ganze
   Kostenrechnung jenes Datensatzes — ein zusätzlicher Systemaufruf je Einstieg,
   L3 und L10 betroffen — hängt an dieser Behauptung.

**Beide sind falsch, und sie waren es schon, als sie geschrieben wurden.** Der
Lesevorgang gibt die Auskunft von sich aus: `Schwungleser::oeffnen` scheitert
mit `EACCES`, der Lesefaden meldet `Abschluss::Fehler`, und
`krk-ui`s `tabs::lesemeldungen_einziehen` schreibt daraus die Tabmeldung
„… ließ sich nicht vollständig lesen: Permission denied (os error 13)". Dieser
Zweig steht seit `537fda5` (S12) im Baum, also seit lange vor dem Datensatz vom
260815.

Gemessen am 260907 mit einer Probe, die seither im Baum steht:
`tabs::tests::ein_ordner_ohne_leserecht_meldet_sich_aus_dem_lesevorgang`.

## Wie es dazu kam

Beide Stellen haben den Einstiegsweg gelesen — `tabelle::in_zeile_einsteigen`
ruft `ordner_lesen` und ist danach fertig — und daraus geschlossen, dass keine
Meldung entsteht. Der Schluss ist naheliegend und falsch: die Meldung entsteht
**einen Einzugstakt später** in einem anderen Modul, über einen Kanal, den der
Einstiegsweg nicht mehr sieht. Wer den synchronen Weg abliest, sieht sie nicht.

Das ist dieselbe Bauart wie beim Fehlbefund `260810-1102`, wo aus dem Lesen von
`appkit/ereignisse.rs` allein auf einen Defekt geschlossen wurde, den es nicht
gab, weil die Sperre in einer anderen Datei sitzt.

## Was daran behoben ist

Der Modulkopf von `verweisziel.rs` ist am 260907 berichtigt und nennt den Weg
über den Leser samt der Probe. `tabelle::in_zeile_einsteigen` und
`kommandos::pfadeingabe::pruefen` tragen seither je einen Absatz, der sagt, wo
die Meldung herkommt und warum an dieser Stelle keine zweite Prüfung
hinzukommt.

**Nicht behoben ist der Datensatz `260815-1749_*` selbst.** Er ist beantwortet
und wird nach der Ortsregel als Aufzeichnung eines Standes nicht umgeschrieben;
sein `Implemented:`-Nachsatz nennt den Irrtum. Ob ein beantworteter
Entscheidungsdatensatz, dessen **Prämisse** sich als falsch herausstellt, damit
ausreichend behandelt ist oder eine eigene Behandlung braucht, ist die offene
Frage `260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`
in ihrer Nachbarschaft.

## Warum es zählt

Der Nutzer hat am 260907-1210 auf dieser Prämisse einen Preis mitentschieden —
einen zusätzlichen Systemaufruf auf jedem Ordnereinstieg, mit zwei ungemessenen
Zeitzusagen daran. Der Preis war nicht nötig. Eine falsche Prosastelle hat hier
also nicht bloß einen Leser in die Irre geführt, sondern beinahe eine
Entscheidung erkauft, die nichts zu kaufen hatte.
