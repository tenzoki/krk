# Vier offene Befunde der Durchsicht von `3b128c3` abgetragen

**Agent:** coder
**Status:** Complete
**Auftrag:** T6, die vier Befunde `shared/issues/260815-2202`, `-2205`, `-2206` und `-2207`
**Dateigrenze:** `crates/krk-ui/src/appkit/tabelle.rs` samt Prüfmodul, eingehalten

---

## Ergebnis in einem Satz

Alle vier sind abgetragen und auf `_c_` umbenannt, in einer einzigen Datei, und `make check`
läuft mit Exit 0 durch.

## Was je Befund geschehen ist

**`260815-2202` — der falsche Satz über L3 und L10.** Die zwei Codestellen sind auf eine
Formulierung gezogen, die den Baum trifft. Am Doc-Kommentar von `namensform` steht jetzt
zuerst, was die Sache entscheidet — die Gleichheit mit dem `--` der Spalte `Größe` —, und in
einem eigenen Absatz darunter, warum das Verweisziel nicht erfragt wird: ein `stat` je
sichtbarer Zeile stünde in der Zeichenschleife der Dateiliste, und die misst **keine** der
zehn Zusagen aus C8, weil L2, L3 und L10 kopflos laufen (`krk-bench/src/messen.rs:1199`). Der
Absatz sagt ausdrücklich, dass das der stärkere Grund ist. Der Kommentar in der Probe
`allein_ein_ordner_traegt_den_schraegstrich` sagt dasselbe in einem Satz und verweist auf
`namensform`.

Der Baum ist gegengeprüft: außer diesen zwei Stellen führt keine Code- oder Prosadatei den
Satz mehr. Der Entscheid `shared/decisions/260815-2056_i_…` trägt die Berichtigung seit dem
260815-2210 als Nachtrag, die Commit-Nachricht von `3b128c3` ändert niemand. Die drei
verbleibenden Nennungen von „L3 und L10" unter `crates/` (`tabelle.rs:1500`,
`verzeichnis/eintrag.rs:157`, `verzeichnis/verweisziel.rs:23`) meinen den Lesevorgang und den
Sortierschlüssel und sind richtig.

**`260815-2205` — die fehlende Zählprobe.** Zwei Proben stehen im Prüfmodul, beide über
`crate::quellbaum` und in der Form der Filterprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
(`krk-core/tests/verzeichnis.rs`).

`das_ordnerzeichen_entsteht_an_genau_einer_stelle` zählt dreimal: keine andere Datei des
Baums führt `namensform`, `ohne_ordnerzeichen` oder `ORDNERZEICHEN` in einer Code-Zeile; jede
der zwei Regeln hat über `aufrufstellen` genau einen Rufer im Code dieser Datei vor ihrem
Prüfmodul; und das Zeichen selbst steht in genau drei Code-Zeilen.
`die_anzeigeform_hat_genau_zwei_leser` hält die andere Hälfte: ein Namenszellentext wird über
`stringValue` an genau zwei Stellen gelesen.

**Die Probe hat beim ersten Lauf sofort einen Treffer geholt, und behoben ist die Wurzel und
nicht die Zahl.** `krk-bench/src/bericht.rs` nennt eine Probe
`der_kurzstempel_passt_zur_namensform_des_projekts` — dasselbe deutsche Wort mitten in einem
längeren Bezeichner, kein zweiter Bau. Die Nadel zieht jetzt über `fuehrt_den_namen` dieselbe
Bezeichnergrenze, die `quellbaum::aufrufstellen` für ihre Seite schon zieht.

**`260815-2206` — der zweite Leser des Ziels.** An der Setzstelle in
`DateifensterDelegierter::feld` steht ein Absatz in der Form der `clickedRow`-Notiz des
Modulkopfs, und `Namensfeld::delegierter` trägt die Rückverweisung. **Der Befund nannte zwei
Leser, es sind drei** — `bearbeitung_beendet` ist seit dem Nutzerentscheid vom 260816-0935
dazugekommen —, deshalb nennt die Notiz die Methoden und keine Zahl. Der SAFETY-Block trägt
jetzt auch die geprüfte Zusage des Zurücklesens mit Fundstelle (`NSControl.h:24`).

Die vorgeschlagene Meldung in der Statuszeile ist **nicht** gebaut, und der Grund steht am
Code: eine Meldung erreicht die Statuszeile allein über die Quelle, an die die Zelle nur über
genau diesen Delegierten kommt. Wer `None` melden wollte, bräuchte das, was `None` gerade
sagt, dass es fehlt.

**`260815-2207` — das unbehandelte Nein der Oberklasse.** Behoben, aber nicht als
`else`-Zweig: es gibt hier zwei Größen und nicht eine. Ein Merker `abgelegt` hält fest, ob
diese Methode überhaupt etwas weggenommen hatte — eine Datei trug nie ein Zeichen —, und ein
`match` über `(angenommen, abgelegt)` trägt die drei Zweige ohne Auffangzweig. Der
`(false, true)`-Zweig setzt die gemerkte Anzeigeform zurück.

## Die Abwägung zu `260815-2207`, die der Auftrag verlangt hat

Teurer als der Zustand ist die Behebung nicht: ein Merker, drei Zweige, keine neue Regel,
**keine** neue AppKit-Methode und damit keine Zeile im Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen`.

Sie ist auch kein toter Code im Sinne des Vorbilds aus `260815-2203`. Dort war der Fall
**gemessen** unmöglich, und ein Schutz hätte eine widerlegte Behauptung über AppKit getragen.
Hier ist der Ausgang nicht widerlegt, sondern ungemessen, und der Rückgabewert der Oberklasse
stand ohnehin schon in einer Variablen. Der Unterschied ist „widerlegt" gegen „ungemessen",
und er entscheidet verschieden. Der Kommentar am `match` hält den `inference:`-Charakter des
Befunds ausdrücklich fest, statt ihn zu überschreiben.

## Ein Widerstand, der die Prosa geformt hat

Der erste Entwurf legte die lange Begründung als Doc-Kommentar an `wird_ersthelfer`. Das
sprengte das Rekursionslimit des Makros `define_class!`
(`__extract_method_attributes_inner`, ein Rekursionsschritt je Attribut). Der Doc-Kommentar
der Methode ist deshalb auf drei Zeilen gekürzt, und die Begründung steht als Kommentarblock
am `match` — für den Leser dieselbe Stelle, für das Makro kein Attribut. **Wer die
Doc-Kommentare der Methoden in dieser `define_class!`-Hülle weiter wachsen lässt, läuft in
denselben Übersetzungsfehler**; die vorgeschlagene Abhilfe des Übersetzers wäre
`#![recursion_limit]` an der Kistenwurzel und damit eine zweite Datei.

## Was ausdrücklich nicht geändert ist

- **Kein Verhalten außer dem einen Zweig.** Das Ordnerzeichen (`3b128c3`), der Aufschub
  (`27dca57`) und die Anzeigeform an jedem Ende (`2c5a1b5`) bleiben, was sie sind.
- **Die zwei offenen Befunde `260816-0040` und `260816-0055`** sind nicht angefasst.
- **`crates/krk-ui/src/quellbaum.rs`** ist gelesen und nicht geändert.
- **Der Entscheid `260815-2056_i_…`** ist nicht angefasst: er trägt die Berichtigung schon.

## Abnahme

`make check` — Exit 0. Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist in diesem Lauf durchgelaufen.

## Datensätze

- `shared/issues/260815-2202` — `Resolved:` angehängt, `_o_` → `_c_`.
- `shared/issues/260815-2205` — `Resolved:` angehängt, `_o_` → `_c_`.
- `shared/issues/260815-2206` — `Resolved:` angehängt, `_o_` → `_c_`.
- `shared/issues/260815-2207` — `Resolved:` angehängt, `_o_` → `_c_`.

Nicht committet, wie beauftragt.
