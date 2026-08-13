Der Doc-Kommentar an `bundle::VERSION` nennt eine Sichtbarkeit, die `PLATZHALTER` nicht trägt

---

`xtask/src/bundle.rs:43-44`, seit D2:

```
/// `pub(crate)` wie [`PLATZHALTER`], seit `release` sie fuer die Tag-Pruefung
/// braucht: dort ist `v` gefolgt von dieser Zahl der Name, den HEAD tragen
```

`PLATZHALTER` steht zwei Zeilen darüber als `pub const PLATZHALTER: &str = "__KRK_VERSION__";`
(`xtask/src/bundle.rs:39`), nicht als `pub(crate)`. `VERSION` selbst ist richtig auf
`pub(crate)` gehoben (`:47`).

---

**Schwere:** niedrig. Folgenlos, weil `xtask` ein Binärziel ohne Bibliotheksziel ist und
`pub` und `pub(crate)` dort dasselbe erreichen. Falsch ist die Aussage trotzdem, und sie steht
als Verweis auf eine benachbarte Zeile, die ein Leser prüfen kann.

**Der Planwortlaut trägt denselben Fehler.** Schritt D2 sagt: „`bundle::VERSION` (`bundle.rs:42`)
wird `pub(crate)`, wie `bundle::PLATZHALTER` (`:39`) es schon ist." Der Ausführer hat die
Behauptung übernommen, statt die genannte Zeile zu lesen — dieselbe Art, wie die Zusage über den
Untergrenzen-Abschnitt in `anwendung.rs` durchgegangen ist
(`260813-1345_o_keywindow-und-isequal-stehen-nicht-im-untergrenzen-abschnitt-von-anwendung-rs.md`).

**Was zu tun ist**

Eines von beidem, nicht beides: entweder den Doc-Kommentar auf „`pub(crate)`, enger als
[`PLATZHALTER`]" bringen, oder `PLATZHALTER` auf `pub(crate)` ziehen und den Satz stehen lassen.
Das zweite ist die kleinere Aussage: keine der beiden Konstanten wird ausserhalb der Kiste
gelesen, und `xtask` hat kein Bibliotheksziel, an dem `pub` etwas bedeutete.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Die Sache, um die es D2 ging, hält: es gibt genau eine Quelle der Versionszahl, und `release`
  liest sie über `bundle::VERSION` (`xtask/src/release.rs:200`), statt die `Cargo.toml` zu
  zerteilen.
