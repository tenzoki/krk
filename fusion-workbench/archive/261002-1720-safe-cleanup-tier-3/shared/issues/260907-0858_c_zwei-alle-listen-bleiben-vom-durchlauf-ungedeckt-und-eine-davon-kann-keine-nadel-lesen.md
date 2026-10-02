Zwei ALLE-Listen bleiben vom Durchlauf ungedeckt, und keine Nadel dieser Bauform kann sie lesen

---
Der Durchlauf `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`
(`crates/krk-core/tests/baum.rs`) haelt seit dem 260907 elf der dreizehn
`ALLE`-Listen des Baums. Zwei fuehrt er in `UNLESBARE_ALLE_LISTEN` und
uebergeht sie:

- `Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs`). `Datei::Zettel(Zettel)`
  traegt Daten, und die Liste fuehrt eine Zeile je Zettel: sieben Eintraege zu
  sechs Varianten. `gemeinsam::varianten_der_aufzaehlung` bricht an einer
  datentragenden Variante ab, und eine Gleichheit zwischen Liste und
  Aufzaehlung waere hier ohnehin die falsche Zusage. Die richtige lautet: jede
  datenlose Variante genau einmal, und die datentragende einmal je Wert ihres
  Feldes.
- `Spalte::ALLE` (`crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs`). Die
  Aufzaehlung ist modulintern (`enum Spalte`, nicht `pub`), und die Nadel
  findet allein `pub enum <Name> {` in Spalte 0.

Dieselbe Bauart trifft `RANGFOLGE: [Warngrund; 8]`
(`crates/krk-ui/src/kommandos/loeschwarnung.rs`): acht Zeilen zu sieben
Varianten, weil `Warngrund::Umfang(Umfangsgrund)` zwei Wortlaute traegt. Sie
heisst nicht `ALLE` und faellt schon deshalb aus dem Durchlauf.

Akzeptanzprobe: `UNLESBARE_ALLE_LISTEN` ist leer oder nennt allein Faelle, die
eine begruendete Ausnahme sind, und jede der drei genannten Listen ist von
einer Probe gehalten, die die fehlende Variante beim Namen nennt.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
Der Helfer `varianten_der_aufzaehlung` liegt in
`crates/krk-core/tests/gemeinsam/mod.rs`, das diese Sitzung nicht anfassen
durfte (eine zweite Bahn arbeitete darin). Eine Erweiterung der Nadel um
datentragende Varianten und um nicht-oeffentliche Aufzaehlungen war deshalb
nicht moeglich; sie ist der Weg, auf dem beide Ausnahmen fallen. Die
Sichtbarkeit von `Spalte` allein fuer eine Probe anzuheben waere der teurere
Fehler.

Grundlage:
`260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`.

---

## Abgleich 260908, und die Behebung

**Der Befund bestand unveraendert**, und die Sperre, die ihn offen hielt, ist fort: die zweite
Bahn arbeitet nicht mehr in `crates/krk-core/tests/gemeinsam/mod.rs`.

Die Abnahmeprobe ist erfuellt, in beiden Haelften.

**`UNLESBARE_ALLE_LISTEN` traegt noch eine Ausnahme statt zweier.**
`Spalte::ALLE` (`crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs`) ist ausgetragen und
faehrt im Durchlauf mit: `varianten_der_aufzaehlung` findet die Aufzaehlung jetzt **mit und
ohne `pub`**. Die Begruendung steht am Helfer: die Sichtbarkeit sagt, wer die Aufzaehlung
sehen darf, und nichts darueber, ob eine Liste daneben vollstaendig zu halten ist. Die
Sichtbarkeit von `Spalte` anzuheben — der teurere Fehler, den der Datensatz benennt — war
dafuer nicht noetig.

**`Datei::ALLE` bleibt eine begruendete Ausnahme und hat jetzt ihre eigene Probe.**
`die_ablageliste_fuehrt_jede_datei_und_je_einen_zettel` (`crates/krk-core/tests/baum.rs`)
prueft genau die Zusage, die der Datensatz als die richtige benennt: jede datenlose Variante
genau einmal, die datentragende einmal je Wert ihres Feldes, in der Reihenfolge der
Aufzaehlung. Beide Seiten kommen aus dem Quelltext, keine aus der anderen; eine achte
Ablagedatei ohne Zeile in `ALLE` laesst sie rot werden und nennt den Namen.

**`RANGFOLGE` ebenso.** `die_rangfolge_fuehrt_jede_variante_und_jeden_umfangswortlaut`
(`crates/krk-ui/src/kommandos/loeschwarnung.rs`) haelt `[Warngrund; 8]` gegen die zwei
Aufzaehlungen `Warngrund` und `Umfangsgrund`. Sie steht dort und nicht im Durchlauf, weil
`krk-ui` kein Bibliotheksziel hat; gelesen wird ueber `crate::quellbaum::varianten`.

**Das Werkzeug dafuer ist neu und steht an einer Stelle.**
`varianten_mit_nutzlast_der_aufzaehlung` liefert je Variante den Bezeichner und den Wortlaut
ihrer Nutzlast. `varianten_der_aufzaehlung` bricht an einer datentragenden Variante weiter ab
— die Strenge ist gewollt und im Kopf begruendet —, und beide bauen auf **einer** Fassung des
Blocklesers (`variantenzeilen`), damit die Lesart nicht ein zweites Mal dasteht. Daneben liest
`gelistete_namen` in `baum.rs` einen Eintrag mit Nutzlast jetzt im Wortlaut, statt an ihm
abzubrechen; der Durchlauf selbst uebergeht die eine Liste, die welche fuehrt, unveraendert.

Resolved: 260908 — `crates/krk-core/tests/gemeinsam/mod.rs` (Nadel mit und ohne `pub`, neuer
Helfer, ein Blockleser), `crates/krk-core/tests/baum.rs` (eine Ausnahme weniger, Probe fuer
`Datei::ALLE`), `crates/krk-ui/src/kommandos/loeschwarnung.rs` (Probe fuer `RANGFOLGE`).
