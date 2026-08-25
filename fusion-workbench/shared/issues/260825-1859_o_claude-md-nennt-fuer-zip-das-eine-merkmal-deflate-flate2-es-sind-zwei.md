CLAUDE.md nennt fuer `zip` das eine Merkmal `deflate-flate2`, es sind zwei

---

`CLAUDE.md:82` sagt: "Bei `zip` laesst das **eine** Merkmal `deflate-flate2` genau das
Verfahren uebrig, das jedes Zip-Werkzeug liest." Seit dem 260825 stehen dort zwei Merkmale,
`deflate-flate2` und `unreserved`.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was der Baum traegt

`Cargo.toml` bindet `zip` seit der Umsetzung von Schritt 3, Strang 1 des Plans
`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` mit
`default-features = false` und den zwei Merkmalen `deflate-flate2` und `unreserved`.

`unreserved` steht in `zip`s eigener `Cargo.toml` als `unreserved = []` da und schaltet keine
Abhaengigkeit ein; es hebt allein eine Pruefung auf, die sonst das Zusatzfeld `0x5855`
abwiese, das `ditto(1)` als einziges Zeitfeld liest. Die Begruendung samt Messtabelle steht in
der Wurzel-`Cargo.toml` an der Stelle, an der dieses Vorhaben sie fuer jede fremde Kiste
fuehrt.

**Der uebrige Satz bleibt wahr.** Der Vorgabesatz zoege weiter C-Code herein, `flate2` muss
weiter unmittelbar danebenstehen, und `Cargo.lock` ist mit dem zweiten Merkmal um keinen
Eintrag gewachsen; `cargo tree --workspace -e normal,build` findet weder `cc` noch ein
`-sys`-Paket. Falsch ist allein das Zahlwort.

## Warum das traegt

Der Satz begruendet die Merkmalswahl und liest sich als Aufzaehlung. Wer ihn als vollstaendig
nimmt und `unreserved` beim Aufraeumen streicht, macht `cargo test -p krk-core` rot, genau wie
bei der `flate2`-Zeile, vor der derselbe Satz warnt.

Die Datei steht nicht auf der Liste der Dateien, die der Auftrag dieses Schritts anfassen
darf; deshalb der Datensatz statt der Berichtigung.

**Schwere:** gering. Eine Prosastelle, kein Verhalten.

**Gefunden:** coder, bei der Umsetzung von Schritt 3, Strang 1 des Plans vom 260825-1725

**Betroffen:** `CLAUDE.md:82`

**Domain:** code
