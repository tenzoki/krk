CLAUDE.md nennt für `Kommando` 68 Varianten, der Baum trägt 75

---

Der Abschnitt „Projektstand" in `CLAUDE.md` sagt: „Am 260811 nachgezählt:
`Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) trägt sieben Werte,
`Kommando` in derselben Datei **68 Varianten**, `Bereich`
(`krk-ui/src/fenstermodell.rs`) fünf und `Fokus` (`krk-ui/src/kommandos/fokus.rs`)
fünf."

Am 260812-2253 nachgezählt trägt `Kommando` **75** Varianten. Die drei anderen
Zahlen stimmen unverändert.

---

**Gemessen** über die Varianten der Aufzählung in
`crates/krk-core/src/tasten/belegung.rs`. Die Runde 6 hat zwei hinzugefügt,
`Kommando::OrdnerDerDatei` und `Kommando::Teilen`; der Plan der Runde nennt den
Sprung als Kriterium C6.1 und beziffert ihn mit „73 auf 75". Die Differenz von
68 auf 73 stammt aus der Runde 5, deren Bereichsleiste fünf Kennungen gebracht
hat; nachgezogen worden ist die Zahl in `CLAUDE.md` dabei nicht.

**Warum die Zahl überhaupt dasteht.** Der Absatz erklärt eine Eigenschaft, die
schon Sitzungen gekostet hat: die vier Aufzählungen haben keinen Auffangzweig,
und wer eine erweitert, hält den Bau an. Der Absatz sagt das selbst und
verweist für die Stellen auf den Übersetzer statt auf eine Liste. Die Zahlen
daneben sind Beiwerk und veralten schneller als der Satz, den sie belegen — die
Datei führt in demselben Abschnitt aus, dass sie Aufstellungen aus genau diesem
Grund schon abgelegt hat.

**Es ist der zweite Zähldefekt derselben Bauart in `CLAUDE.md`**; der erste ist
`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`,
und seine eigene Gegenzahl ist inzwischen ebenfalls veraltet. Beide legen
dieselbe Wahl vor: die Zahl bei jeder Runde nachziehen, oder sie durch etwas
ersetzen, das nicht mitwächst.

**Gewicht:** niedrig. Keine Auswirkung auf den Code; eine falsche Zahl in der
Datei, die jeder Agent zuerst liest.

**Herkunft:** Abgleich der Runde 6 am 260812-2253.

---
Abgleich 260813 (reconciler, Runde 7): **Der Defekt besteht, und die Gegenzahl dieses
Datensatzes ist mit dieser Runde selbst veraltet.** Am 260813 nachgezaehlt ueber
`awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs`: **76 Varianten**
(`belegung.rs:304`). `CLAUDE.md:66` nennt weiterhin 68. Die Runde 7 hat die eine Kennung
`WeitereInstanz` hinzugefuegt (Kriterium C4.1 des Spec sagt 75 auf 76 zu, und der Baum haelt
es).

Das ist der dritte Stand, den dieser Datensatz nennen muesste (68 zitiert, 75 beim Ablegen,
76 heute), und damit das Argument fuer die zweite der beiden hier genannten Moeglichkeiten:
eine Zahl, die jede Runde mitwaechst, gehoert nicht in eine Datei, die keine Probe haelt.
Der Datensatz bleibt offen; die Wahl gehoert dem Nutzer und die Revision von `CLAUDE.md`
nicht in einen Abgleich.

---

**Abgleich 260813-1345 (Runde 8).** Der Befund besteht, und die Zahl in seinem Titel ist selbst
überholt: `Kommando` trägt heute **76** Varianten
(`crates/krk-core/src/tasten/belegung.rs`, beim Abgleich nachgezählt), nicht 75. `CLAUDE.md:66`
nennt weiter 68. Die Runde 8 hat keine Variante angelegt — C6.1 verlangt das ausdrücklich, und
es hält; gewachsen ist die Aufzählung zuletzt in der Runde 7.

Der Titel bleibt, wie er ist: er benennt den Fehler, nicht den Tagesstand. Wer die Zeile in
`CLAUDE.md` berichtigt, schreibt 76 und nicht 75.

Verwandter Befund derselben Sorte, beim selben Abgleich abgelegt:
`shared/issues/260813-1345_o_fuenf-stellen-nennen-79-funktionen-und-73-mit-kommando-die-belegung-fuehrt-82-und-76.md`.

---

**Abgleich 260814-1002 (Runde 9).** Der Befund besteht, und die Gegenzahl ist zum vierten Mal
überholt: `Kommando` trägt **77** Varianten
(`awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs`, am Stand `79dab20`
gezählt), `KENNUNGEN` entsprechend 77 Paare (`belegung.rs:579`). `CLAUDE.md:66` nennt weiter
68. Die Runde 9 hat die eine Variante `Notizzettel` angelegt (`belegung.rs:573`), und der Spec
sagt den Sprung von 76 auf 77 im Abschnitt zu den vollständigen Fallunterscheidungen zu.

Vier Stände sind es jetzt, die dieser Datensatz nennen müsste: 68 zitiert, 75 beim Ablegen,
76 in den Runden 7 und 8, 77 heute. Der Titel bleibt, wie er ist — er benennt den Fehler und
nicht den Tagesstand. Wer die Zeile in `CLAUDE.md` berichtigt, schreibt 77.

**Zwei weitere Zahlen desselben Absatzes sind mit dieser Runde unrichtig geworden**, und beide
gehören nicht in diesen Datensatz, weil sie andere Stellen betreffen: `CLAUDE.md:11` sagt
„Vier Runden sind gefahren" mit einer Tabelle von vier Zeilen, während der Baum bei der
**neunten** steht, und `CLAUDE.md:32` datiert den Projektstand auf 260811-2230. Die Revision
von `CLAUDE.md` gehört nicht in einen Abgleich; die Häufung ist der Grund, sie anzusetzen.

---
Resolved: Die Zahl steht nicht mehr da. `CLAUDE.md` (`## Projektstand`) führt für `Kommando`
heute keine Ziffer mehr, nennt diesen Datensatz als Grund — „sie wächst mit fast jeder Runde
und ist in dieser Datei viermal in vier Tagen falsch geworden" — und gibt stattdessen das
Kommando an, mit dem sich der Stand jederzeit selbst zählen lässt.

**Nachgezählt am 260815-1405:** `Kommando` trägt **78** Varianten, also drei mehr als die 75
dieses Datensatzes. Die drei anderen Zahlen, die `CLAUDE.md` weiter ausschreibt, stimmen
unverändert: `Wirkungsbereich` 7, `Bereich` 5, `Fokus` 5. Diese drei sind seit der Runde 1
stabil, `Kommando` ist es nicht — die Trennung, die `CLAUDE.md` inzwischen zieht, ist damit
am Baum bestätigt.
