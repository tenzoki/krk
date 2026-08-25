Die Zusage haengt jetzt am Rufer in einer anderen Kiste, und keine Probe haelt, dass er sie traegt

---

Der Modulkopf von `zippen.rs` sagt seit `dd74b0e`: „Dass dieser Eintrag keine Quelle desselben Laufs
ist, sichert der Rufer und nicht dieser Zweig." Das ist ehrlich und beschreibt den Bau richtig. Es
ist aber von nichts gehalten: keine Probe prueft, dass `zipauftrag_stellen` seine Quellen von
`packziel` bezieht, keine zaehlt die Rufer von `Auftrag::zippen`, und dessen Signatur nimmt zwei
beliebige Listen entgegen. Wer die zwei Zeilen im Rufer zuruecknimmt, bekommt den Defekt zurueck und
alle Proben gruen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Wo die Zusage heute haengt

- `crates/krk-core/src/operation/zippen.rs:53-72` — der Modulkopf gibt die Zusage an den Rufer ab
  und nennt ihn beim Namen: `kommandos::kontextmenue::packziel` der Kiste `krk-ui`.
- `crates/krk-ui/src/kommandos/kontextmenue.rs:453-460` — `packziel` schneidet, gehalten von
  `das_archiv_des_vorigen_laufs_faellt_aus_den_quellen` (`kontextmenue.rs:1095`).
- `crates/krk-ui/src/appkit/anwendung.rs:6125` — das eine Glied dazwischen:
  `let (quellen, ziel) = kontextmenue::packziel(&auswahl.pfade, &ordner);`

Dieses Glied haelt niemand.

## Der Weg zurueck in den Defekt, ohne dass etwas rot wird

Man ersetze in `zipauftrag_stellen` die eine Zeile durch die zwei Zeilen, die vor `dd74b0e` dort
standen:

```rust
let ziel = kontextmenue::archivname(&auswahl.pfade, &ordner);
let _ = self.auftrag_starten(seite, Auftrag::zippen(auswahl.pfade, ziel), ordner, positionen);
```

Danach gilt:

- `packziel` bleibt gerufen? Nein — dann meldete der Uebersetzer es als unbenutzt und `-D warnings`
  hielte den Bau an. Man ruft es also weiter, nimmt aber nur das `ziel` heraus und reicht
  `auswahl.pfade` als Quellen. Damit ist `packziel` benutzt, kein `dead_code`, keine Warnung.
- `das_archiv_des_vorigen_laufs_faellt_aus_den_quellen` prueft `packziel` unmittelbar und bleibt
  **gruen**.
- `jeder_kontextbefehl_erreicht_seine_wirkung` (`crates/krk-ui/src/appkit/anwendung.rs:9064`) fragt
  allein, ob der Rumpf von `zipauftrag_stellen` die Zeichenfolge `Auftrag::zippen(` traegt. Er
  traegt sie und bleibt **gruen**.
- Die Kernproben in `crates/krk-core/tests/operation.rs` sind unberuehrt und bleiben **gruen**.

Der Defekt aus
`260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-papierkorb-wenn-der-archivname-ihrem-namen-gleicht.md`
steht wieder da, und `make check` sagt Exit 0.

## Warum die Signatur nicht hilft

`Auftrag::zippen(quellen: Vec<PathBuf>, ziel: impl Into<PathBuf>)`
(`crates/krk-core/src/operation/auftrag.rs:160-162`) nimmt zwei unabhaengige Listen.

**Die Entpackseite ist besser gestellt, und ihr eigener Doc-Kommentar sagt warum**
(`crates/krk-core/src/operation/auftrag.rs:164-169`): „Genommen werden Paare aus Archivpfad und
Zielordner, **damit die beiden Listen gar nicht erst getrennt uebergeben werden koennen**." Genau
diese Vorsorge fehlt der Packseite; dort genuegt ein zweiter Vektor, um Quellen und Ziel
auseinanderlaufen zu lassen.

**Ein zweiter Rufer ist absehbar.** Der Circle-Datensatz dieser Runde haelt fest, dass die drei
Befehle bewusst ohne Tastenkombination und ohne Hauptmenueeintrag bleiben und deshalb keine
`Kommando`-Variante entsteht (`_*_circle.md:29`). Wer das spaeter nachholt, baut einen zweiten
Ausfuehrungszweig, und der ruft `Auftrag::zippen` nach heutigem Stand ohne jeden Widerstand
unmittelbar. *inference*, kein Planschritt sagt es zu.

## Wie dieses Projekt so etwas sonst haelt

Mit einer Zaehlprobe ueber den Quellbaum. Drei stehen schon da:

- `der_kontextmelder_wird_beim_aufbau_gesetzt` (`anwendung.rs:9018`) — genau ein Rufer, genau diese
  Datei.
- `genau_vier_stellen_sichern_den_zettel` (`anwendung.rs`, `zettelproben`).
- `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`crates/krk-core/tests/verzeichnis.rs`),
  die genau so einen unerlaubten Rufer schon einmal gefangen hat.

Der Kopf von `crate::quellbaum` schreibt aus, warum solche Zahlen sparsam zu setzen sind; er verbietet
sie nicht, und die Zusage stammt hier vom Nutzer.

## Vorschlag

Eine Probe, zwei moegliche Zuschnitte:

1. **`Auftrag::zippen` hat genau einen Rufer, und er steht in `zipauftrag_stellen`.** Eine
   `aufrufstellen`-Zaehlung ueber `quelldateien()` wie beim Kontextmelder. Faengt den zweiten
   Eingang, nicht die Umgehung im bestehenden.
2. **Der Rumpf von `zipauftrag_stellen` traegt `packziel` und nicht `archivname`.** Dieselbe
   Bauform wie das zweite Glied von `jeder_kontextbefehl_erreicht_seine_wirkung`, also
   `rumpf(&datei, "zipauftrag_stellen")` gegen zwei Nadeln. Faengt die Umgehung im bestehenden
   Zweig, nicht den zweiten Eingang.

Beide zusammen kosten zehn Zeilen und schliessen die Kette an ihrem letzten offenen Glied. Was immer
gewaehlt wird, gehoert in den Doc-Kommentar der Probe, was sie **nicht** sieht — so wie es dieser
Baum durchweg haelt.

**Schwere:** mittel. Kein Fehler am heutigen Baum. Eine Nutzerzusage, deren einziger Halt eine Zeile
ist, die keine Probe liest.

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs` (`zipauftrag_stellen` und das Probenmodul
`kontextproben`), mittelbar `crates/krk-core/src/operation/zippen.rs` (der Modulkopf, der die Zusage
abgibt).
