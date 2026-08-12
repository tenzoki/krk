`make check` bleibt auch nach Schritt 5 rot: die drei Spaltenkennungen warten auf Schritt 7

---

Schritt 5 des Plans `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`
nennt als Abnahme `make check`. Sie ist nach Schritt 5 nicht erreichbar, und zwar aus
demselben Grund, aus dem sie es nach Schritt 4 nicht war: die Belegungsdatei fuehrt seit
Schritt 4 fuenf Kennungen ohne Kommando, und Schritt 5 baut nur zwei davon. Die drei
Spaltenschalter bekommen ihr Kommando erst in Schritt 7.

---

**Schwere:** mittel (der Baum ist rot, das Verhalten der Anwendung ist unberuehrt)
**Gefunden:** coder, bei der Abnahme von Schritt 5 (`make check`, Exit 2; das `cargo test`
darin bricht mit Exit 101 ab)
**Betroffen:** `crates/krk-ui/src/belegungsmodell.rs` (`bereich`, `nach_bereichen`),
`resources/default-keymap.toml`, der Plan selbst
**Domain:** code

## Der Befund, am 260812-0548 gemessen

Nach Schritt 5 und dem Nachtrag der drei Proben aus
`issues/260812-0533_*_drei-proben-stehen-gegen-die-neuen-belegungseintraege-….md`:

- `cargo build --workspace` — gruen.
- `cargo fmt --all --check` — gruen.
- `cargo clippy --workspace --all-targets -- -D warnings` — gruen.
- `cargo test --workspace --no-fail-fast` — **28 Fehlschlaege, alle in `krk-ui`**, und
  jeder einzelne nennt `spalte_groesse_umschalten`. `krk-core` ist gruen, einschliesslich
  der beiden Proben, die Schritt 4 rot zurueckgelassen hatte.

Die Stelle ist eine: `belegungsmodell::bereich` findet fuer eine Kennung ohne Kommando nur
dann einen Funktionsbereich, wenn sie namentlich im zweiten Zweig steht, und dort stehen
allein die sechs vom Menue zugestellten Textbefehle. `nach_bereichen` bricht daraufhin laut
ab, und mit ihm jede Probe, die die Belegungsansicht oder die Markdown-Ausgabe ueber die
Auslieferungsbelegung baut. Von den fuenf Kennungen des Schrittes 4 ist das nach Schritt 5
noch fuer drei so: `spalte_groesse_umschalten`, `spalte_datum_umschalten`,
`spalte_typ_umschalten`.

**Keiner der 28 Fehlschlaege nennt `erstes_fenster_umschalten` oder `editor_umschalten`.**
Die beiden Kommandos aus Schritt 5 sind vollstaendig eingeordnet; nachgemessen mit
`cargo test -p krk-ui --no-fail-fast 2>&1 | grep -iE "erstes_fenster|editor_umschalten"`,
ohne Treffer.

## Warum das kein Fehlschlag des Schrittes 5 ist

Der Plan ordnet die Belegungsdatei (Schritt 4) bewusst **vor** die Kommandos, die sie
nennen, und begruendet es unter `## Vorgehen`: umgekehrt liefe die Probe
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` fuer die Dauer eines
Schrittes rot, und `clippy -D warnings` haelt jeden Zwischenstand an, der etwas Unbenutztes
einfuehrt. Der Preis dieser Ordnung ist der rote Baum, und er faellt nicht bei einem
Schritt an, sondern ueber drei: von Schritt 4 bis Schritt 7.

Der ontocoder hat das fuer Schritt 4 bereits festgehalten
(`issues/260812-0533_*`, Abschnitt `## Eine Anmerkung zum Plan`). Der Befund gilt fuer
Schritt 5 unveraendert weiter, und er wird erst mit Schritt 7 hinfaellig.

## Was hilft

1. **Die Abnahme der Schritte 4, 5 und 6 im Plan nachziehen.** `make check` ist fuer keinen
   der drei die richtige Abnahme. Richtig ist: `cargo build`, `cargo fmt --check`,
   `cargo clippy -- -D warnings` gruen, dazu die Proben, die der Schritt selbst anfasst,
   und `cargo test` mit genau den benannten offenen Fehlschlaegen. Wer den Plan das
   naechste Mal anfasst, schreibt es an die drei Schritte.
2. **Schritt 7 schliesst die Luecke**, und zwar in seinem ersten Teil: die drei Varianten
   `SpalteGroesseUmschalten`, `SpalteDatumUmschalten`, `SpalteTypUmschalten` in `Kommando`
   samt ihren Zeilen in `wirkungsbereich` und `bereich_des_kommandos`. Danach greift der
   erste Zweig von `bereich`, und die 28 Proben laufen wieder.
3. **Eine Zusage bleibt bis dahin ungeprueft**: der neue letzte `assert_eq!` in
   `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
   (`crates/krk-ui/src/belegungsausgabe.rs`) haelt fest, dass ab Werk genau die drei
   Spaltenschalter unbelegt sind. Die Probe erreicht ihn heute nicht, weil sie vorher am
   fehlenden Funktionsbereich abbricht. Wer Schritt 7 abnimmt, liest sie eigens nach.

---

Resolved: 260812-0618, coder, mit Schritt 7 des Plans
`planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`.

**`make check` ist grün, Exit 0.** Damit ist der Befund erledigt, und zwar auf dem Weg, den er
unter `## Was hilft` als Punkt 2 vorhergesagt hat: die drei Varianten `SpalteGroesseUmschalten`,
`SpalteDatumUmschalten` und `SpalteTypUmschalten` in `Kommando`, ihre Zeilen in
`wirkungsbereich` (alle drei `Wirkungsbereich::Ueberall`) und in `bereich_des_kommandos` (alle drei
`Funktionsbereich::Dateilisting`). Der erste Zweig von `belegungsmodell::bereich` greift danach,
`nach_bereichen` bricht nicht mehr ab, und die 28 Proben laufen wieder.

Der rote Baum hat wie vorhergesagt über vier Schritte gestanden, von Schritt 4 bis Schritt 7, und
er war der Preis der Reihenfolge und kein Fehlschlag.

**Zwei Proben sind bei der Gelegenheit angeschlagen, die dieser Befund nicht kannte**, weil sie an
gebauten Kommandos hängen und die drei erst mit Schritt 7 entstanden. Beide sind mitbehoben und im
Nachtrag zu `260812-0533_c_drei-proben-stehen-gegen-die-neuen-belegungseintraege-….md`
ausgeschrieben; sie gehören sachlich dorthin, weil es dieselbe Zusage ist.

**Punkt 3 ist geprüft.** Der letzte `assert_eq!` in
`jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` wird jetzt erreicht und hält.

**Punkt 1 ist nicht umgesetzt, und das mit Absicht.** Die Abnahme der Schritte 4, 5 und 6 im Plan
umzuschreiben hieße, ihre Abnahme rückwirkend auf einen Zwischenstand zu senken, den es nicht mehr
gibt: `make check` ist für alle drei jetzt erreichbar und grün. Was der jeweilige Schritt bei
seiner Ausführung vorfand, halten seine Protokolle unter `history/` fest, und die sind
Aufzeichnungen eines Standes und behalten ihn. Am Schritt 7 steht neu, dass er den Baum grün
zurückgibt.
