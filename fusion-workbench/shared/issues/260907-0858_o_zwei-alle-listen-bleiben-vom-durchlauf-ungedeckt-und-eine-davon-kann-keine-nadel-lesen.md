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
