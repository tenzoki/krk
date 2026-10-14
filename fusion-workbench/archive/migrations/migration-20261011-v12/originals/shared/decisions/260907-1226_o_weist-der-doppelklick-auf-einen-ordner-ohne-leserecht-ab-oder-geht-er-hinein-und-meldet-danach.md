# Weist der Doppelklick auf einen Ordner ohne Leserecht ab, oder geht er hinein und meldet danach?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md` (der Vorgänger, beantwortet am 260907-1210); `260907-1226_*_zwei-prosastellen-nennen-den-doppelklick-auf-einen-ordner-ohne-leserecht-wortlos-der-baum-meldet-seit-s12.md`; `crates/krk-ui/src/kommandos/pfadeingabe.rs`; `crates/krk-ui/src/appkit/tabelle.rs` (`in_zeile_einsteigen`); `crates/krk-ui/src/tabs.rs` (`lesemeldungen_einziehen`)

---

## Frage

Der Vorgängerdatensatz hat gefragt, ob der Doppelklick auf einen Ordner ohne
Leserecht meldet oder schweigt. Der Nutzer hat am 260907-1210 „er meldet"
gewählt. Bei der Umsetzung hat sich herausgestellt, dass er **schon meldet**,
und zwar über einen Weg, den jener Datensatz nicht betrachtet hat: der
Lesevorgang selbst gibt die Auskunft, `Schwungleser::oeffnen` scheitert mit
`EACCES`, der Lesefaden meldet `Abschluss::Fehler`, und
`tabs::lesemeldungen_einziehen` macht daraus die Tabmeldung. Gemessen an einer
Probe, die seither im Baum steht
(`tabs::tests::ein_ordner_ohne_leserecht_meldet_sich_aus_dem_lesevorgang`).

Damit ist die alte Frage beantwortet und eine neue steht daneben, die vorher
von der falschen verdeckt war. **Die zwei Wege unterscheiden sich weiter, nur
an einer anderen Stelle:**

| | Pfadsprung (`shift+cmd+g`, `opt+cmd+g`) | Doppelklick / Rechts-Pfeil |
|---|---|---|
| wechselt den Ordner | nein | ja, in eine leere Liste |
| meldet | ja, sofort | ja, einen Einzugstakt später |
| Wortlaut | „… lässt sich nicht lesen: …" | „… ließ sich nicht vollständig lesen: …" |
| Kosten | ein `read_dir` je Sprung | keine; der Leser antwortet ohnehin |

## Warum die Frage zu stellen ist

Ein Nutzer, der zweimal auf denselben Ordner klickt, steht danach **in** einem
Ordner, den er nicht lesen kann, und muss mit `backspace` wieder heraus. Wer
den Pfad tippt, bleibt stehen, wo er war. Das ist ein sichtbarer Unterschied
für dieselbe Lage, und keiner der beiden Wege liest ihn aus einer Regel ab.

Der zweite Wortlaut sagt außerdem „nicht **vollständig** gelesen", während in
diesem Fall gar nichts gelesen wurde. Für den Fall, für den der Satz gebaut
ist — ein Lesevorgang, der nach einigen Stapeln scheitert — ist er richtig; für
den Ordner ohne Leserecht untertreibt er.

## Möglichkeiten

1. **So lassen.** Der Doppelklick geht hinein und meldet danach, der Pfadsprung
   weist ab.
   - Pro: kostet nichts, ändert kein Verhalten, und beide Wege erfüllen das
     Abnahmekriterium aus C2. Die zwei Erwartungen sind verschieden: wer einen
     Pfad tippt, hat sich vielleicht vertippt; wer auf eine Zeile klickt, sieht
     den Eintrag vor sich und will hinein.
   - Contra: zwei Verhalten für dieselbe Lage, die niemand aus dem Code abliest,
     ohne beide Wege zu vergleichen. Genau der Satz, mit dem der
     Vorgängerdatensatz seine dritte Möglichkeit abgelehnt hat.
2. **Der Doppelklick weist auch ab.** `in_zeile_einsteigen` prüft das Leserecht,
   bevor es `ordner_lesen` ruft.
   - Pro: eine Regel statt zweier, und der Nutzer bleibt stehen, wo er war.
   - Contra: **ein zusätzlicher Systemaufruf auf jedem Ordnereinstieg**, auch im
     häufigen lesbaren Fall. Daran hängen die Zeitzusagen L3 und L10, die seit
     der Runde 4 nicht mehr gemessen sind. Das ist genau der Preis, den der
     Vorgängerdatensatz beziffert hat und den die heutige Umsetzung nicht
     zahlen musste.
3. **Der Pfadsprung geht auch hinein.** Die `read_dir`-Prüfung in
   `pfadeingabe::pruefen` entfällt; beide Wege wechseln und melden aus dem
   Leser.
   - Pro: eine Regel statt zweier, **und ein Systemaufruf weniger** statt eines
     mehr. Der Wortlaut stünde danach an genau einer Stelle.
   - Contra: der Nutzer landet nach einem vertippten Pfad in einem fremden
     Ordner. Und der Kommentar an jener Stelle begründet die Prüfung mit C2 —
     die Begründung wäre nachzuziehen, nicht bloß die Zeile zu streichen.

## Randbedingungen

- Der Einstiegsweg darf keinen Systemaufruf je **Anzeige** hinzubekommen. Der
  Sortierschlüssel entsteht einmal beim Lesen, und daran hängen L3 und L10.
  Möglichkeit 2 fügt einen je **Einstieg** hinzu, nicht je Anzeige; das ist
  weniger, aber nicht nichts.
- Eine Antwort, die beide Wege angleicht, soll an einer Stelle stehen und nicht
  an zweien. `pfadeingabe.rs` nennt sich im Modulkopf „die eine Stelle, die
  einen Pfad prueft".
- Der Wortlaut „nicht vollständig gelesen" gehört dem Leser und deckt mehr Fälle
  als diesen; ihn hier zu ändern, änderte ihn für alle.

## Empfehlung

Keine. Möglichkeit 1 ist die billigste und heute gebaut, Möglichkeit 3 die
einzige, die vereinheitlicht **und** dabei Arbeit spart — sie kehrt aber eine
Meldung um, die C2 ausdrücklich verlangt hat, und das ist eine Frage an den
Nutzer und nicht an einen Agenten.

---
Answered:
Implemented:
Deferred:
Superseded by:
