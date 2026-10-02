`iconutil` liegt nach der neuen Aufrufregel auf der falschen Seite und wird über den Suchpfad gerufen
---
macOS liefert `iconutil` mit, die Regel von `260821-1221` sieht dafür den vollen Pfad vor, der Aufruf nimmt den Suchpfad.
---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `d4f28bc`
**Betrifft:** `xtask/src/bundle.rs` (`symbol_bauen`)
**Verwandt:** `260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md` — die Regel, gegen die der Aufruf steht; `260826-1448_*_iconutil-wird-ueber-den-suchpfad-gerufen-waehrend-kommentar-und-meldung-usr-bin-iconutil-sagen-und-messen-rs-liest-cargo-ein-zweites-mal.md` — hat den Aufruf am 260905 stehenlassen, weil die Regel damals offen war.

## Befund

Der Nutzer hat am 260907 entschieden: mit macOS geliefertes Programm mit vollem Pfad,
nachinstalliertes über den Suchpfad. `iconutil` liegt unter `/usr/bin/iconutil`, gehört also zur
ersten Gruppe. `xtask/src/bundle.rs:463` ruft `Command::new("iconutil")`, also über den Suchpfad.

Es ist die einzige Aufrufstelle in `xtask/`, die nach der Regel falsch liegt. Erhoben mit
`grep -rn 'Command::new(' xtask/src`, jede gefundene Stelle einzeln gegen die Herkunft ihres
Programms gehalten: die sechs mit vollem Pfad gerufenen (`codesign`, `ditto`, `git`, `lipo`,
`security`, `xcrun`) liegen sämtlich unter `/usr/bin` und damit richtig; `rustup`, `gh` und
`cargo` werden nachinstalliert und über den Suchpfad gerufen, also ebenfalls richtig.

Der Aufruf ist beim Schreiben der Regel bewusst nicht angefasst worden: der Auftrag lautete, die
Praxis aufzuschreiben und Abweichungen zu melden, nicht die Aufrufe zu vereinheitlichen. Der
Doc-Kommentar bei `SYMBOLGROESSEN` und der Kopf von `xtask/src/main.rs` nennen die Stelle seitdem
als Abweichung und zeigen auf diesen Datensatz.

Der Aufruf ist bis dahin nie gescheitert: `/usr/bin` steht auf jedem gebräuchlichen `PATH`. Die
Abweichung ist damit keine Störung, sondern eine Stelle, an der die Regel und der Baum
auseinanderfallen — und genau daran erkennt eine spätere Runde eine Regel als unverbindlich.

## Abhilfe

`Command::new("iconutil")` auf `Command::new("/usr/bin/iconutil")` stellen und die Abbruchmeldung
darunter nachziehen: sie sagt heute „wird ueber den Suchpfad gerufen; steht /usr/bin nicht auf
PATH, findet der Aufruf es nicht", und beide Halbsätze wären danach falsch. Der Doc-Kommentar bei
`SYMBOLGROESSEN` und der Absatz zur Ausnahme im Kopf von `xtask/src/main.rs` fallen dann weg.

**Abnahme:** `grep -rnE 'Command::new\("[a-z]' xtask/src` nennt nur noch `rustup`, und
`cargo xtask bundle` erzeugt weiterhin `target/KRK.app/Contents/Resources/KRK.icns`.

**Schwere:** Low.

---
Resolved: 260907-1407 vom coder. `xtask/src/bundle.rs` (`symbol_bauen`) ruft
`Command::new("/usr/bin/iconutil")`. Die Abbruchmeldung darunter ist mitgezogen: sie nennt
statt des Suchpfads die Aufrufregel im Kopf von `xtask/src/main.rs` und sagt, dass ein
fehlendes `/usr/bin/iconutil` eine unvollständige Systeminstallation ist. Der Doc-Kommentar
bei `SYMBOLGROESSEN` nennt den Aufruf jetzt als regelkonform statt als Befund; der Absatz
zur Ausnahme im Kopf von `xtask/src/main.rs` sagt statt der einen falsch liegenden Stelle,
dass der Baum keine trägt.

Abnahme gefahren: `grep -rnE 'Command::new\("[a-z]' xtask/src` nennt nur noch `rustup`
(`xtask/src/release.rs:633`).
