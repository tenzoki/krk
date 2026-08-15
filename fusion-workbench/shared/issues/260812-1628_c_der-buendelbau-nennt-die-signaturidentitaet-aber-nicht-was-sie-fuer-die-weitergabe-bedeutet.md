Der Bündelbau nennt die Signaturidentität, aber nicht, was sie für die Weitergabe bedeutet

---

`cargo xtask bundle` meldet nach dem Signieren genau eine Zeile
(`xtask/src/sign.rs:172-175`):

```
Signiert mit "Apple Development: Kai Stalmann (FJ8U4B3QAC)", gefunden ueber den
Schluesselbund als einzige gueltige Identitaet.
```

Die Zeile ist wahr und vollständig über das, was geschehen ist. Sie sagt nichts über die
Folge: ein so signiertes Bündel wird von Gatekeeper auf **jeder anderen Maschine**
abgewiesen. Wer die Zeile liest, hat keinen Anlass, das zu vermuten — sie klingt nach
gelungener Signatur, und sie ist eine.

---

**Am 260812 eingetreten.** Der Nutzer hat `target/KRK.app` auf einen zweiten Mac kopiert;
dort ist es als mögliche Schadsoftware abgewiesen worden. Nachgemessen an diesem Baum:

```
codesign -dvv target/KRK.app   → Authority=Apple Development: Kai Stalmann (FJ8U4B3QAC)
spctl -a -vvv -t exec …        → rejected
xcrun stapler validate …       → does not have a ticket stapled to it
lipo -info …                   → x86_64 (nicht universell)
```

Der richtige Weg ist gebaut und dokumentiert: `cargo xtask release` fährt sechs Stationen bis
zur Beglaubigung und zum angehefteten Ticket, und `README.md` beschreibt sie. Der Nutzer ist
dort nicht hingekommen, weil ihn nichts darauf gestoßen hat. Die Kosten waren ein
fehlgeschlagener Versuch auf einem zweiten Gerät und die Vermutung, KRK sei beschädigt.

**Ein zweiter Teil derselben Lücke:** `bundle` baut nicht universell, sondern für die
Architektur der Baumaschine. Auch das steht in keiner Ausgabe. Auf einem Apple-Silicon-Mac
braucht das Ergebnis dieses Baumes deshalb zusätzlich Rosetta, selbst wenn die Signatur
stimmte.

Zwei Wege stehen offen. Die Meldung um einen Satz erweitern, der die Folge nennt, etwa „nur
auf dieser Maschine lauffähig; für die Weitergabe `cargo xtask release`" — das kostet drei
Zeilen und trifft jeden Bau. Oder die Ausgabe von `bundle` am Ende um eine Zusammenfassung
ergänzen, die Identitätsart und Architektur nennt und die Weitergabe adressiert. Die zweite
trägt mehr und ist teurer.

Der Fehler wäre, die Zeile zu verschärfen, ohne den Fall der Auslieferungsidentität zu
bedenken: signiert jemand `bundle` über `KRK_SIGN_IDENTITY` mit einer Developer-ID, ist der
Warnsatz falsch. Die Meldung muss die Art der Identität lesen, nicht den Unterbefehl.

Herkunft: gemeinsamer Speicher. Betrifft `xtask` und den Bauweg des ganzen Projekts, nicht
die Directive der Runde 6.

---
Resolved: Der zweite der beiden Wege gefahren, also die Zusammenfassung und nicht der einzelne
Warnsatz. `cargo xtask bundle` gibt nach der Zeile `Buendel: …` einen Abschlusshinweis aus, der
beide Lücken schließt. Geprüft mit `cargo fmt --all --check && cargo clippy --workspace
--all-targets && cargo test --workspace`, Exit 0, `xtask` fährt 96 Proben.

**Warum der teurere Weg der billigere war.** Der Datensatz hält die Zusammenfassung für
teurer, weil sie neben der Identitätsart auch die Architektur nennt. Am Baum gemessen kostet
das fast nichts: `bundle` übersetzt ohne Ziel-Tripel und ist deshalb nie universell, die
Aussage steht also fest und braucht keinen `lipo`-Aufruf zur Laufzeit. Der billige Weg hätte
die Hälfte des Fehlschlags vom 260812 unerklärt gelassen — das Bündel wurde nicht nur
abgewiesen, es war auch nur `x86_64`.

**Die Falle dieses Datensatzes ist beachtet.** Der Hinweis hängt an der Art der Identität und
nicht am Unterbefehl: `sign::weitergabehinweis` liest den Namen gegen `DEVELOPER_ID_PRAEFIX`,
dieselbe Konstante, an der `bestimmen_fuer_release` schon hängt. Eine Entwicklungsidentität
bekommt „bleibt auf dieser Maschine", eine Developer-ID bekommt „richtig signiert, aber nicht
beglaubigt und ohne Ticket". Wer über `KRK_SIGN_IDENTITY` mit einer Developer-ID signiert,
liest also keinen falschen Warnsatz.

**`release` bleibt strukturell frei, nicht über eine Abfrage.** `release::ausfuehren` ruft die
Stationen einzeln und geht nie durch `bundle::bauen`; der Ausgabeort liegt in der
Unterbefehlsverteilung, die `release` nicht erreicht. `messen --alle` geht zwar durch
`bundle::bauen`, bekommt den Hinweis aber ebenfalls nicht — es baut für eine Messung. Eine
Probe hält fest, dass es genau einen Rufer gibt.

**Ein Name für die Architektur.** Der Hinweis meldet sie unter dem Namen, den `lipo` schreibt,
nicht unter dem von `std::env::consts::ARCH`: `release::lipo_name` liest `ZIELE` und
`ARCHITEKTUREN` paarweise, statt eine dritte Namensliste danebenzustellen. Ein unbekannter
Name wird durchgereicht und nicht geraten. Die neu tragende Paarung der beiden Aufzählungen
ist festgehalten statt vorausgesetzt: `const _: () = assert!(ZIELE.len() == ARCHITEKTUREN.len())`
hält die Länge beim Übersetzen, eine Probe die Reihenfolge. Ohne das läse der Nutzer im
Hinweis `aarch64` und in der Ausgabe von `lipo -info`, mit der er nachprüft, `arm64`.

Nicht behoben und als eigener Datensatz aufgenommen: der Hilfetext zu `bundle` in `main.rs`
schweigt weiter zur Weitergabe.
