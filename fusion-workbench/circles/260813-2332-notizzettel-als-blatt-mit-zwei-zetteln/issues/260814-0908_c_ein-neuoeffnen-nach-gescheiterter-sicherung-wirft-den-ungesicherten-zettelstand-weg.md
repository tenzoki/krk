Ein Neuöffnen oder ein Tabwechsel nach gescheiterter Sicherung wirft den ungesicherten Zettelstand weg

---

C4 sagt zu: „Eine gescheiterte Sicherung, etwa wegen fehlenden Schreibrechts, wirft den
Stand nicht weg und meldet den Grund." `Anwendungsdelegierter::zettel_sichern`
(`crates/krk-ui/src/appkit/anwendung.rs:3440`) hält die Zusage — es ruft
`Zettelmodell::gesichert` nur nach einem gelungenen Schreibvorgang. Zwei Stellen daneben
brechen sie: `notizzettel_zeigen` (`:3276`) und `zettel_wechseln` (`:3358`) rufen beide
`Zettelmodell::oeffnen(zettel, gelesen)`, und dessen Rumpf setzt **beide** Stände des
Zettels auf den Dateiinhalt:

```rust
pub fn oeffnen(&mut self, zettel: Zettel, gelesen: String) {
    self.offener = zettel;
    let stand = &mut self.staende[zettel.index()];
    stand.gehalten.clone_from(&gelesen);   // <- der ungesicherte Stand ist hier fort
    stand.gelesen = gelesen;
}
```

Damit ist der getippte Text weg, den die gescheiterte Sicherung ausdrücklich stehen lassen
sollte, und `zu_sichern()` meldet danach nichts mehr.

---

**Schwere:** hoch. Datenverlust ohne Meldung, gegen ein ausgeschriebenes Abnahmekriterium.

**Der kürzeste Weg dorthin** (kein Ablageordner, oder `note-1.txt` ohne Schreibrecht):

1. `f2`, in Zettel 1 „abc" tippen.
2. `Esc`. `zettel_blatt_geschlossen` (`:3520`) sichert, das Schreiben scheitert,
   `zettel_sicherung_melden` stellt den Grund in die Statuszeile. Das Modell hält „abc"
   weiter als abweichend — bis hierher ist alles richtig.
3. `f2` erneut. `notizzettel_zeigen` liest die Datei frisch, ruft `oeffnen(Erster, alt)`,
   und „abc" ist fort. Keine Meldung sagt es.

Derselbe Ablauf über den Tabwechsel: Zettel 1 ändern, Sicherung scheitert beim Klick auf
Tab 2, zurück auf Tab 1 — `zettel_wechseln` endet mit `oeffnen(Erster, alt)`.

**Die zwei Zusagen stehen im Spec nebeneinander und widersprechen sich**, und der Bau hat
sie stillschweigend zugunsten der zweiten aufgelöst:

- C4: „Eine gescheiterte Sicherung … wirft den Stand nicht weg."
- C4: „Der Zettel liest seine Datei bei jedem Öffnen neu." (mildert die
  Überschreibgefahr zwischen zwei Instanzen)

Beide gelten nur gemeinsam, wenn das Neulesen den **abweichenden** Stand nicht antastet.

**Ein Lösungsweg, nicht der einzige.** `Zettelmodell::oeffnen` bekommt die Regel: weicht
der Zettel ab, wird `gelesen` gesetzt und `gehalten` **nicht**. Der Zettel steht dann
weiter mit dem getippten Text da, bleibt zu sichern, und der Nutzer verliert nichts. Was
er dabei nicht sieht, ist der Stand der anderen Instanz — das ist der Preis, und er ist
kleiner als der jetzige. Die Entscheidung, welcher der beiden Stände gewinnt, gehört vor
den Bau; sie ist im Spec nicht getroffen.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Der Doc-Kommentar an `zettel_sichern` schreibt die verletzte Zusage selbst aus:
  „**Eine gescheiterte Sicherung wirft den Stand nicht weg.**"
- Betroffene Stellen: `crates/krk-ui/src/zettelmodell.rs:124` (`oeffnen`),
  `crates/krk-ui/src/appkit/anwendung.rs:3276` und `:3358`.

---
Resolved: Der getippte Stand gewinnt. `Zettelmodell::oeffnen`
(`crates/krk-ui/src/zettelmodell.rs`) trägt die Regel jetzt selbst: weicht der Zettel von
seiner Datei ab, bleibt sein gehaltener Text stehen und das frisch Gelesene wird verworfen;
nur wo nichts abweicht, wird das Gelesene beides. Der Rückgabewert ist der Text, der in die
Textfläche gehört, und er trägt `#[must_use]` — wer ihn fallenließe, setzte das Gelesene in
die Fläche und hätte denselben Verlust wieder, und der Bau sagt es ihm. Beide Aufrufer
nehmen ihn: `notizzettel_zeigen` und `zettel_wechseln`
(`crates/krk-ui/src/appkit/anwendung.rs`), also sind die zwei Wege des Datensatzes, das
Neuöffnen und der Tabwechsel, mit einer Regel geschlossen und nicht mit zweien.

Die Entscheidung dahinter ist die des Nutzers vom 260814-0925, und sie steht als Zusage im
Spec: `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, C4, Fassung
vom 260814-0925. Der zweite Satz von C4 ist damit eingeschränkt und nicht gestrichen: der
Zettel liest weiter bei jedem Öffnen, und im gewöhnlichen Fall zeigt er, was in der Datei
steht. Der Preis steht dort ebenfalls: wer einen abweichenden Zettel öffnet, sieht den
Stand einer zweiten Instanz von KRK nicht.

Drei Proben am Modell halten es fest (`crates/krk-ui/src/zettelmodell.rs`, Prüfmodul):
`das_oeffnen_setzt_den_abweichenden_stand_nicht_zurueck`,
`ein_sauberer_zettel_bekommt_den_neuen_dateiinhalt` als Gegenprobe, und
`jeder_abweichende_zettel_steht_zur_sicherung_an`. Der Plan ist an sechs Stellen
nachgezogen (`planning/260814-0656_o_plan-…`, Kopfnotiz vom 260814-0941).

`make check` am 260814-0947 gefahren, Rückgabewert 0, „alle vier gruen".
