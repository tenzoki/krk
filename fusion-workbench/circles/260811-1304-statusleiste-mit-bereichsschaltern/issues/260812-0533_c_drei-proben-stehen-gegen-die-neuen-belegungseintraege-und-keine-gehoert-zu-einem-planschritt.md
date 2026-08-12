Drei Proben stehen gegen die neuen Belegungseinträge, und keine gehört zu einem Planschritt

---

Schritt 4 des Plans `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`
trägt fünf Funktionen in `resources/default-keymap.toml` nach. Drei bestehende Proben
schlagen danach fehl, und **keine der drei wird von einem späteren Schritt dieses Plans
wieder grün**: die Schritte 5 und 7 fassen die betroffenen Dateien nicht an. Ohne einen
Nachtrag bleibt `make check` bis zum Ende der Runde rot.

---

**Schwere:** mittel (der Baum ist rot, das Verhalten der Anwendung ist unberührt)
**Gefunden:** ontocoder, bei der Abnahme von Schritt 4 (`make check`, Exit 2)
**Betroffen:** `crates/krk-core/tests/belegung.rs`, `crates/krk-ui/src/belegungsausgabe.rs`, `crates/krk-ui/src/appkit/menue.rs`
**Domain:** code

## Die drei Proben, am 260812-0533 gemessen

**1. Eine Zahl steht doppelt.** `die_auslieferungsbelegung_fuehrt_vierundsiebzig_funktionen`
(`crates/krk-core/tests/belegung.rs:1651`) schreibt die 74 als Literal hin, und ihr eigener
Dokumentationskommentar begründet das: die Zahl sei hier die Zusage, weil die Kopfzeile von
`resources/default-keymap.toml` sie nennt. Der Name der Probe trägt sie ein zweites Mal
(`vierundsiebzig`). Gemessen: `left: 79, right: 74`.

Daneben hält `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs:1349`) dieselbe Zusage, ohne eine Zahl zu nennen: sie
liest die Kopfzeile und vergleicht sie mit der Datei. Sie ist grün. **Damit gibt es zwei Proben
für eine Zusage, und nur eine von beiden muss bei jedem Nachtrag umbenannt werden.**

**2. „Jede Funktion trägt mindestens eine Kombination" gilt nicht mehr.**
`jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`
(`crates/krk-core/tests/belegung.rs:147`) verlangt von jeder Funktion ohne `reserviert_fuer`
mindestens eine Kombination und nennt C3 als Grund. Die drei Spaltenschalter tragen ab Werk
keine, und sie tragen ausdrücklich kein `reserviert_fuer`: die Antwort vom 260812-0306
(`decisions/260812-0306_a_bekommen-die-spaltenschalter-tastenbefehle.md`) wählt Möglichkeit 2,
also „in der Belegung geführt, ohne ausgelieferte Kombination". Gemeldet:
`spalte_groesse_umschalten traegt keine Kombination`.

Der Datensatz und die Probe sagen Verschiedenes über dieselbe Sache. **Verbindlich ist der
Datensatz**, also gehört die Ausnahme in die Probe. Sie ist heute an `reserviert_fuer`
geknüpft, und genau dieses Feld schließt Schritt 4 für die drei aus.

**3. „Ab Werk ist keine Funktion unbelegt" gilt nicht mehr.**
`jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
(`crates/krk-ui/src/belegungsausgabe.rs:518`) endet mit der Behauptung, die Markdown-Ausgabe
führe jede Funktion der Auslieferungsbelegung, weil ab Werk keine unbelegt sei. Nach Schritt 4
sind drei unbelegt und fallen aus der Ausgabe; der erste Teil der Probe, „keine unbelegte steht
darin", bleibt richtig und ist die eigentliche Zusage.

## Was nicht in diese Liste gehört

Achtundzwanzig weitere Proben in `krk-ui` scheitern nach Schritt 4 an derselben Stelle:
`belegungsmodell::bereich` (`crates/krk-ui/src/belegungsmodell.rs:529`) findet für die fünf
neuen Kennungen keinen Funktionsbereich, weil sie noch kein Kommando tragen. **Das ist der vom
Plan benannte Zwischenstand und kein Defekt**; die Schritte 5 und 7 geben den fünf ihre
Kommandos, und damit greift der erste Zweig von `bereich`. Dasselbe gilt für
`jede_kennung_ohne_kommando_wird_vom_menue_zugestellt`
(`crates/krk-ui/src/belegungsausgabe.rs:784`).

## Zehn Stellen Prosa nennen die 74 und die 68 mit

Kein Fehlschlag, aber dieselbe Ursache, und beim nächsten Lesen eine falsche Auskunft:
`crates/krk-ui/src/belegungsausgabe.rs:45`, `:48`, `:56`, `:256`, `:545`, `:677`, `:678` und
`crates/krk-ui/src/appkit/menue.rs:84`, `:720`, `:788` rechnen im Text mit 74 Funktionen und
68 Kommandos. Nach Schritt 7 sind es 79 und 73.

## Was hilft

Ein Nachtrag zu Schritt 5 oder ein eigener Schritt, in dieser Reihenfolge:

1. `die_auslieferungsbelegung_fuehrt_vierundsiebzig_funktionen` **ersatzlos streichen**, statt
   die Zahl auf 79 zu setzen und die Probe umzubenennen. Ihre Zusage trägt
   `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` vollständig und ohne
   Literal; was sie darüber hinaus prüft, ist die Anwesenheit der dreizehn Kennungen der
   Editor-Runde, und die gehört an eine Probe ohne Zahl im Namen. Bleibt sie stehen, kostet
   jeder künftige Eintrag in `default-keymap.toml` wieder eine Umbenennung.
2. Die Ausnahme in `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` von
   `reserviert_fuer` lösen: eine Funktion darf ohne Kombination stehen, wenn ein
   Entscheidungsdatensatz es so festlegt. Am billigsten als benannte Liste der drei Kennungen
   im Prüfcode, mit dem Verweis auf den Datensatz vom 260812-0306.
3. In `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` den letzten `assert_eq!`
   auf die drei Spaltenschalter einstellen oder ihn streichen; die Zusage der Probe steht in
   ihren beiden ersten Teilen.
4. Die zehn Prosastellen nachziehen, sobald Schritt 7 die Kommandos gebaut hat.

## Eine Anmerkung zum Plan

Schritt 4 nennt als Abnahme `make check`. Das ist nach dem Befund oben nicht erreichbar: der
Schritt lässt den Baum rot zurück, und zwar zu Recht — vier der fünf Proben, die anschlagen,
messen genau das, was der Zwischenstand ist. Die Abnahme des Schrittes ist deshalb die
Zählprobe `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` samt
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, und beide sind grün. Wer den
Plan das nächste Mal anfasst, schreibt es dort hin.

---

## Stand 260812-0548, nach Schritt 5

**Die drei Vorschläge aus `## Was hilft` sind umgesetzt**, in der dort vorgeschlagenen Form:

1. `die_auslieferungsbelegung_fuehrt_vierundsiebzig_funktionen` ist gestrichen. Der Teil, der
   die Anwesenheit der dreizehn Kennungen der Editor-Runde prüft, steht als
   `die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung` weiter
   (`crates/krk-core/tests/belegung.rs`), ohne Zahl im Namen und ohne Zählung im Rumpf. Er ist
   erhalten und nicht gestrichen worden, weil `jede_kennung_der_kommandos_steht_in_der_
   auslieferungsbelegung` nur elf der dreizehn erreicht: `text_rueckgaengig` und
   `text_wiederholen` tragen kein Kommando.
2. Die Ausnahme in `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` ist
   von `reserviert_fuer` gelöst und steht als benannte Liste `OHNE_KOMBINATION_AB_WERK` der drei
   Spaltenkennungen im Prüfcode, mit Verweis auf den Datensatz vom 260812-0306. Sie erlaubt die
   leere Tastenliste und verlangt sie nicht: der Datensatz begründet die leere Liste mit der
   Knappheit der freien Kombinationen und verbietet keine spätere.
3. Der letzte `assert_eq!` in `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
   (`crates/krk-ui/src/belegungsausgabe.rs`) nennt jetzt die drei Spaltenschalter beim Namen,
   statt eine Zahl zu vergleichen. **Diese Zusage ist noch ungeprüft**: die Probe bricht vorher
   am fehlenden Funktionsbereich der drei Kennungen ab und erreicht die Zeile bis Schritt 7
   nicht.

**Der Datensatz bleibt offen, und zwar wegen Punkt 4.** Die zehn Prosastellen, die mit 74
Funktionen und 68 Kommandos rechnen, sind nicht nachgezogen: die Zahlen sind erst nach Schritt 7
endgültig (dann 79 und 73). Eine der zehn ist mit Punkt 3 nebenbei weggefallen, der Kommentar
"die Datei fuehrt alle 74" in `belegungsausgabe.rs`; neun stehen.

**`make check` ist damit weiterhin rot, aber aus einem anderen Grund als dem dieses Datensatzes.**
Die 28 verbliebenen Fehlschläge nennen alle `spalte_groesse_umschalten` und warten auf die drei
Kommandos aus Schritt 7. Der eigene Datensatz dafür ist
`260812-0548_o_make-check-bleibt-auch-nach-schritt-5-rot-die-drei-spaltenkennungen-warten-auf-schritt-7.md`.

---

Resolved: 260812-0618, coder, mit Schritt 7 des Plans
`planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`.

**Punkt 4 ist erledigt, und damit der Datensatz.** Die Prosastellen tragen jetzt 79 Funktionen
und 73 Kommandos. Beide Zahlen sind nachgezählt und nicht übernommen: `grep -c '^\[\[funktion\]\]'`
über `resources/default-keymap.toml` liefert 79, `grep -c 'gehalten_von'` sechs vom Menü
zugestellte, und `Kommando::KENNUNGEN` steht auf 73 — 79 minus 6 geht auf.

Neun Stellen standen noch (die zehnte, der Kommentar "die Datei fuehrt alle 74" in
`belegungsausgabe.rs`, war mit Punkt 3 weggefallen). Nachgezogen sind sie in acht Änderungen,
weil eine davon zwei Zeilen desselben Satzes trifft:
`crates/krk-ui/src/belegungsausgabe.rs` (Modulkopf dreimal, der Kommentar an der ersten Lage in
`wirkung`, der Doc-Kommentar der Probe `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`)
und `crates/krk-ui/src/appkit/menue.rs` (Modulkopf, `die_sechs_zugestellten`, der Doc-Kommentar
der Messprobe aus S1). Ein `grep` nach `74` und nach `68` findet in beiden Dateien keine Stelle
mehr.

**Zwei weitere Proben derselben Wurzel sind mit Schritt 7 angeschlagen und mitbehoben**, und sie
gehören sachlich in diesen Datensatz, weil es dieselbe Zusage ist: eine Auslieferungsbelegung darf
seit dem Entscheid vom 260812-0306 eine Funktion ohne Kombination führen. Sie schlugen vorher
nicht an, weil sie an gebauten Kommandos hängen und die drei erst mit Schritt 7 entstanden.

1. `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`
   (`crates/krk-core/tests/belegung.rs`) verlangte von jedem gebauten Kommando mindestens eine
   Kombination. Sie nimmt jetzt dieselbe Ausnahme wie Punkt 2 oben.
2. `innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei`
   (`crates/krk-ui/src/belegungsausgabe.rs`) verglich die Zeilen der Markdown-Ausgabe mit **allen**
   Funktionen aus `nach_bereichen`, die Ausgabe schreibt aber nur die belegten. Die Erwartung
   filtert jetzt dieselbe Bedingung, und der Kommentar verweist für die Frage, **welche** Funktionen
   unbelegt sind, auf `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`.

**`OHNE_KOMBINATION_AB_WERK` ist dabei aus dem Rumpf der einen Probe an den Kopf von
`crates/krk-core/tests/belegung.rs` gewandert**, weil zwei Proben sie jetzt lesen. Die Begründung
und der Verweis auf den Datensatz vom 260812-0306 sind mitgezogen; wer eine vierte Funktion ohne
Kombination ausliefert, trägt sie dort nach.

**Die in Punkt 3 ungeprüft gebliebene Zusage ist jetzt geprüft.** Der letzte `assert_eq!` in
`jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` wird erreicht und hält: ab Werk sind
genau die drei Spaltenschalter unbelegt.

**Die Anmerkung zum Plan ist nachgezogen**, aber nicht an den Schritten 4, 5 und 6: die Abnahme
`make check` dort ist mit Schritt 7 wieder erreichbar, und die Protokolle der drei Schritte halten
ihren jeweiligen Zwischenstand bereits fest. Am Schritt 7 steht jetzt, dass er den Baum grün
zurückgibt.
