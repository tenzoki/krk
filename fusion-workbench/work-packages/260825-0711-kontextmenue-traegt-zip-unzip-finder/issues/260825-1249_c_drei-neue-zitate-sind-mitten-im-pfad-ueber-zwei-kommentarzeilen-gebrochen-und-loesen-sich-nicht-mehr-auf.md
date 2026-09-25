Drei neue Zitate sind mitten im Pfad ueber zwei Kommentarzeilen gebrochen und loesen sich nicht mehr auf

---

`dd74b0e` bringt fuenf Verweise auf Defektdatensaetze in den Quelltext. Drei davon sind mitten im
Dateinamen umgebrochen, sodass zwischen `...-in-den-` und `papierkorb-*` ein Zeilenende, ein
Kommentarzeichen und Leerzeichen stehen. Weder `grep` noch ein Glob findet einen solchen Verweis;
gelesen werden kann er nur mit dem Auge und nur von jemandem, der den Umbruch als Umbruch erkennt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Die drei Stellen

1. `crates/krk-core/src/operation/zippen.rs:65-66`

   ```
   //! (`issues/260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-
   //! papierkorb-*`, der kleinere der zwei Wege)
   ```

2. `crates/krk-ui/src/kommandos/kontextmenue.rs:87-88` — derselbe Verweis, derselbe Umbruch.

3. `crates/krk-ui/src/appkit/anwendung.rs:9054-9055`

   ```
   /// (`issues/260825-1144_*_die-probe-befehl-zweig-wirkung-prueft-vorhandensein-
   /// statt-paarung-*`)
   ```

Zwei weitere Verweise desselben Commits stehen auf einer Zeile und sind darum auffindbar, wenn auch
gekuerzt: `kontextmenue.rs:1090` und `anwendung.rs:6111`.

## Was die Konvention verlangt

`rules/fusion-workbench-conventions.md`, Abschnitt `## Filename Patterns`: ein Datensatz wird mit
seinem **vollen Dateinamen** zitiert, allein der Zustandsmarker wird gesternt —
`YYMMDD-HHMM_*_<topic>.md`. Der Zweck ist, dass der Verweis jeden Markerwechsel uebersteht und
auffindbar bleibt.

## Warum der Umbruch schwerer wiegt als die Kuerzung

Die gekuerzte Form `..._ueberschreiben-raeumt-eine-quelle-des-laufs-*` ist in diesem Baum
verbreitete Uebung; `crates/krk-ui/src/appkit/anwendung.rs` fuehrt sie an mindestens acht Stellen
(4332, 4643, 4673, 4680, 8589, 8674, 8703, 8840). Sie loest sich wenigstens als Glob noch auf, und
die dahinterliegende Frage haengt an den offenen Datensaetzen
`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`
und
`shared/issues/260817-1130_*_die-sternform-fuer-zitate-gilt-seit-dem-260815-und-drei-runden-schreiben-den-marker-aus.md`.
Dieser Datensatz ficht die Kuerzung nicht an.

Der Umbruch loest sich **gar nicht** mehr auf. Ein Glob ueber die erste Haelfte trifft dieselbe Datei
zufaellig mit, ein Glob ueber die zweite trifft nichts, und wer den Verweis kopiert, kopiert ein
Kommentarzeichen mit.

**Gleiche Form gibt es schon zweimal im Baum**, beide aus fruehen Commits derselben Runde und von
beiden Durchsichten nicht benannt: `crates/krk-core/src/operation/zippen.rs:40-41` und `93-94`. Der
Befund gilt allen fuenf Stellen.

## Vorschlag

Den Verweis in **eine** Zeile setzen, auch wenn sie ueber die Spaltenbreite hinauslaeuft. `rustfmt`
bricht Kommentarzeilen nicht um, und `cargo fmt --all --check` laeuft mit ueberlangen Kommentarzeilen
gruen — nachgesehen am 260825, der Baum traegt solche Zeilen bereits (`anwendung.rs:4829`, `5721`,
`6035`). Wo die Zeile stoert, gehoert der Verweis in eine eigene Zeile unter den Absatz statt in
seine Mitte.

**Schwere:** gering. Kein Verhalten haengt daran; ein Verweis, der nicht mehr aufloest, verliert
seinen Zweck.

**Betroffen:** `crates/krk-core/src/operation/zippen.rs`,
`crates/krk-ui/src/kommandos/kontextmenue.rs`, `crates/krk-ui/src/appkit/anwendung.rs`.

---
Resolved: Alle fuenf Stellen stehen wieder auf einer Zeile, also auch die zwei
aelteren aus fruehen Commits derselben Runde, denen der Befund ausdruecklich
mitgilt: `crates/krk-core/src/operation/zippen.rs` (drei Stellen),
`crates/krk-ui/src/kommandos/kontextmenue.rs` und
`crates/krk-ui/src/appkit/anwendung.rs`. Der Rest des Absatzes ist umgebrochen,
sodass allein die Zeile mit dem Verweis ueber die Spaltenbreite laeuft.
`cargo fmt --all --check` gibt Exit 0; rustfmt bricht Kommentarzeilen nicht um.
Die Kuerzung auf `...-des-laufs-*` bleibt, wie der Datensatz es vorsieht.
