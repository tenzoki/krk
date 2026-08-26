abwurf_pruefen nennt einen Systemaufruf je Zeigerbewegung zu teuer und macht fuer das Schreibrecht einen

---

Der Doc-Kommentar von `abwurf_pruefen` begruendet den Textvergleich "Ziel ist Quellordner" damit, dass
eine genauere Antwort "einen Systemaufruf je Zeigerbewegung" kostete. Drei Zeilen tiefer fragt dieselbe
Funktion bei jeder Zeigerbewegung `abwurf::beschreibbarkeit(&ziel)`, das ueber
`resourceValuesForKeys:` genau so einen Aufruf macht. Das Argument traegt so nicht; der Vergleich
bleibt trotzdem richtig, aus dem zweiten Grund, den der Kommentar nennt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/tabelle.rs:3627-3631`: "Genauer waere sie nur ueber `st_dev` und `st_ino`,
  also ueber einen Systemaufruf je Zeigerbewegung; und selbst dann bliebe sie eine Vorhersage".
- `:3665`: `schreibrecht: abwurf::beschreibbarkeit(&ziel)` — je `validateDrop:`, also je Bewegung
  (`:3583`, `:1020`).
- `appkit/abwurf.rs:224-236`: `NSURL::fileURLWithPath` und `resourceValuesForKeys_error` mit
  `NSURLIsWritableKey`.

Der zweite Grund (`:3629-3634`, "selbst dann bliebe sie eine Vorhersage … entschieden wird … in
`zielpfad` ueber die Naemlichkeit") traegt allein. Der erste ist zu streichen oder die Kostenrechnung
um den Aufruf zu ergaenzen, der schon da ist.

## Umfang

`krk-ui`, `appkit/tabelle.rs`, Doc-Kommentar.
