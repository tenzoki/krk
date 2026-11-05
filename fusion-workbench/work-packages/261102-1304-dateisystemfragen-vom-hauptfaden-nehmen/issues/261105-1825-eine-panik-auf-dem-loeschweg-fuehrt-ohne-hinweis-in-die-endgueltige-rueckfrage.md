Eine Panik auf dem Löschweg führt ohne Hinweis auf den internen Fehler in die endgültige Rückfrage
---
Schwere: Niedrig. Braucht eine Panik, von der keine bekannt ist, und danach den ausdrücklichen Klick. Der Nutzer bekommt aber die falsche Begründung und nicht den Papierkorb, der ginge.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/nebenfaden.rs:99` bis `:101`: `catch_unwind`, danach `startfehler(abgebrochen())`. `crates/krk-ui/src/appkit/anwendung.rs:8759`: `|_fehler| Loeschbefund::unentschieden()` verwirft den Fehler. Der Doc-Kommentar `:8741` benennt das Verhalten. `Text::FrageAbgebrochen` erscheint auf diesem Weg also nie. Die Tafel führt auf `Endgueltiggrund::PapierkorbUngewiss`, und das Blatt sagt: „KRK konnte nicht feststellen, ob dieser Ort einen Papierkorb hat“ (`crates/krk-core/src/sprache/tabelle/de.rs:758`).

**Fehlerbild, abgeleitet.** `lokaler_loeschbefund` (`anwendung.rs:1061`) kann nach der Papierkorbfrage in `umfang::zaehlen` oder `arbeitsbaum` in Panik geraten. Dann ist der Papierkorb längst geklärt und wird trotzdem nicht angeboten. Der Entscheid vom 261019 (`crates/krk-ui/src/kommandos/loeschwarnung.rs:20` bis `:35`) meint nicht auflösbare Ordner und Namen ohne UTF-8, keinen internen Fehler. Ob er diesen Fall decken soll, hat der Nutzer nicht gesagt. Die Meldung der Panik steht allein auf stderr.

Sicher im engeren Sinn bleibt es: nichts geschieht ohne Klick, die Vorgabe ist „Abbrechen“, die Schaltfläche ist als zerstörend gekennzeichnet. `catch_unwind` greift auch im Release, denn `panic` ist weder in `Cargo.toml` noch in `.cargo/config.toml` noch im `Makefile` gesetzt.

**Richtung.** Ich schlage vor, Panik und Startfehler vom Befund zu trennen, etwa mit `T = Result<Loeschbefund, Fehler>`: kein Blatt, stattdessen `Text::FrageAbgebrochen` in der Statuszeile, und der Nutzer löst noch einmal aus. Wenn der Nutzer das endgültige Angebot auch hier will, genügt ein Satz am Entscheid.

**Abnahme.** Eine Probe zeigt, dass ein abgebrochener Befund kein `PapierkorbUngewiss` ergibt, oder der Entscheid nennt den Fall ausdrücklich.
