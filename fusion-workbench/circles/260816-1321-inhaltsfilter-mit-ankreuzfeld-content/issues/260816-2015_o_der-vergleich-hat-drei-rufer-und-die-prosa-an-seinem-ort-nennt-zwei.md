Der Vergleich hat drei Rufer, und die Prosa an seinem Ort nennt zwei — dazu zwei weitere Stellen, die A1 mitgenommen hätte
---
Drei Prosastellen im Baum beschreiben einen Stand, den die Schritte A1 und A2 dieser Runde
abgelöst haben. Keine hält den Bau an, keine bricht ein Abnahmekriterium, und jede ist an
ihrer eigenen Datei nachzulesen.

**Erstens: `traegt_die_folge` hat seit A2 (`7283d55`) drei Rufer, und `filter.rs` sagt an vier
Stellen zwei.** Die Rufer sind `verzeichnis/modell.rs`, `verzeichnis/durchlauf.rs` und seit A2
`verzeichnis/inhalt.rs`; `grep -rl 'traegt_die_folge' crates --include='*.rs'` nennt sie samt
der Heimat. In `crates/krk-core/src/verzeichnis/filter.rs` steht dagegen:

- das Bild im Modulkopf (Zeilen 4–14) führt unter `traegt_die_folge` genau zwei Pfeile,
  `modell::sichtbar` und `durchlauf`; `inhalt` fehlt, obwohl dasselbe Bild die Schwelle aus C1
  bereits mitträgt,
- Zeile 19: „der Vergleich hat zwei",
- Zeile 97: „**Der eine Vergleich.** Seine beiden Rufer sind …",
- Zeile 111: „ihre beiden Rufer haben den Zweig „steht ein Filtertext?" davor".

**Die Zählprobe weiß es besser.** `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`
(`crates/krk-core/tests/verzeichnis.rs:2867`) ist in A2 auf drei nachgezogen worden und ist
grün. Der Baum und die Probe führen drei, die Prosa an der Heimat der Regel führt zwei.

**Zweitens: `crates/krk-core/src/verzeichnis/sys.rs:45-49` verortet den zweiten Aufrufer
außerhalb der Kiste.** Der Satz lautet: „Es sind seit dem Defekt `260810-1247` zwei, und der
zweite liegt ausserhalb der Kiste: die Vorschau in `krk-ui` liest ueber denselben Eingang wie
der Editor." Seit A1 (`5c7f5b9`) liest die Vorschau über
`krk_core::text::datei::bis_zur_grenze_lesen`; beide Aufrufer von `ohne_warten_oeffnen` liegen
damit in `crates/krk-core/src/text/datei.rs`, und keiner liegt außerhalb der Kiste.

**Drittens: `crates/krk-core/src/verzeichnis/sys.rs:285-288` nennt einen Aufrufer von
`ist_deskriptormangel`, wo es zwei sind.** Der Satz sagt: „Der Aufrufer, der die Unterscheidung
braucht, ist [`crate::verzeichnis::durchlauf`]". Seit A1 fragt auch
`text::datei::bis_zur_grenze_lesen` (`datei.rs:609`), und zwar genau dort, wo
`Lesehindernis::Deskriptormangel` von `Lesehindernis::Fehler` getrennt wird — der Zweig, an dem
C3.6 hängt.
---
Gefunden bei Schritt G1 des Plans
`planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, beim Lesen des Spec
`shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` gegen den Baum auf dem
Stand nach F2.

**Kein Abnahmekriterium ist gebrochen.** C6.1 verlangt, dass `traegt_die_folge` genau einmal
im Baum steht — das hält. C6.3 verlangt, dass die Zählprobe den dritten Rufer namentlich nennt
und nicht durch eine bloße Zahl ersetzt ist — das hält ebenfalls. Der Befund betrifft allein
die Prosa, und er steht hier, weil stille Zahlen in Prosa der wiederkehrende Defekt dieses
Projekts sind und CLAUDE.md sie unter „Was man nicht sieht" ausdrücklich als solchen führt.

**Eine gemeinsame Wurzel, und sie ist an den Sitzungsprotokollen abzulesen.** Beide Schritte
haben die Regel erkannt und je eine Stelle nachgezogen, aber nicht alle. A2 hat den
Doc-Kommentar von `name_traegt_den_filter` in `modell.rs` von zwei auf drei Rufer gehoben
(`history/260816-1535-a2-verzeichnis-inhalt.md`, Abschnitt „Eine Doku-Stelle, die der Schritt
falsch gemacht hätte") und die Heimat der Regel dabei ausgelassen. A1 hat `sys.rs:790`
nachgezogen, die Zeile, die `bis_zur_grenze_lesen` als Aufrufer nennt
(`history/260816-1520-a1-lesehuelle-nach-krk-core.md`, Abschnitt „Zwei Doku-Stellen, die der
Umzug falsch gemacht hätte"), und den Modulkopf derselben Datei nicht. Wer eine Stelle sucht,
findet eine; wer alle sucht, braucht ein Muster.

**Ein Muster, das alle drei gefunden hätte**, und es gehört zum Befund:

```sh
grep -rn "beiden Rufer\|zwei Rufer\|Vergleich hat zwei\|zwei Aufrufer\|beiden Aufrufer" \
  crates --include='*.rs'
```

**Nicht behoben.** G1 liest ab und repariert nicht; der Auftrag sagt das ausdrücklich. Die
Behebung ist eine Änderung an vier Zeilen und einem Bild in `filter.rs` und an zwei Absätzen
in `sys.rs`, und sie fasst keine Regel an.
