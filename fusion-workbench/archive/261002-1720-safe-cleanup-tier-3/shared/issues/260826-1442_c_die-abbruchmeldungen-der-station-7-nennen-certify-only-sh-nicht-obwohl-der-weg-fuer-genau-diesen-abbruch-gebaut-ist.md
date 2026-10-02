Die Abbruchmeldungen der Station 7 nennen `./certify-only.sh` nicht, obwohl der Weg für genau diesen Abbruch gebaut ist
---
Scheitert die Einreichung oder das Heften, nennt die Meldung Bündelpfad und `notarytool log`, aber keinen Wiederaufnahmebefehl. Allein der Zweig „Profil fehlt" nennt den Nur-Beglaubigungsweg.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/beglaubigung.rs`

## Befund

`beglaubigen` (`beglaubigung.rs:316-392`) hat drei Abbruchzweige nach der Werkzeugprüfung:

| Zweig | Zeilen | nennt `./certify-only.sh` |
|---|---|---|
| `KRK_NOTARY_PROFILE` fehlt | `:323-338` | ja (`:335-336`) |
| `notarytool submit --wait` gescheitert | `:370-377` | nein |
| `stapler staple` gescheitert | `:384-390` | nein |

Der zweite Zweig ist der Fall vom 260820 (`HTTPClientError.deadlineExceeded`, Modulkopf `:10-19`), für den der Weg gebaut wurde. Wer ihn erlebt, liest am Ort des Scheiterns „Das gebaute und signierte Buendel liegt unter …; das Protokoll nennt `xcrun notarytool log`" und nicht den Befehl, der von hier aus weiterfährt. Dasselbe beim Heften; dort gibt es zudem keinen Weg, der allein heftet — `certify-only.sh` reicht neu ein.

Zum Vergleich: die achte Station nennt in `ohne_ticket_meldung` (`veroeffentlichung.rs:362-363`) den Weg, der das Ticket herstellt.

## Abhilfe

In beide späten Zweige dieselbe Abhilfezeile wie in `:335-336`: „Wiederaufnahme ohne Neubau: `./certify-only.sh <zahl>`". Die Zahl steht dem Rufer aus `release` nicht als Argument zur Verfügung; `bundle::VERSION` oder die aus der `Info.plist` gelesene Zahl trägt sie.

**Schwere:** Medium — der Weg existiert, die Meldung, die auf ihn zeigen müsste, schweigt.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, M2

---
Resolved: Beide spaeten Zweige nennen den Weg. `beglaubigen` (`xtask/src/beglaubigung.rs`) nimmt
die Versionszahl als zweites Argument — `ausfuehren` reicht sein Argument durch, `release` die
Zahl aus `env!("CARGO_PKG_VERSION")`, gegen die Station 1 den Tag gehalten hat. Die zwei Meldungen
sind als reine Funktionen herausgezogen, `einreichungsmeldung` und `heftmeldung`, wie
`version_pruefen` und `signaturstand_pruefen` daneben; beide enden auf `wiederaufnahme(zahl)`,
"Wiederaufnahme ohne Neubau, sie reicht dasselbe Buendel noch einmal ein: ./certify-only.sh
<zahl>". Dass der Weg im Heftzweig mehr tut als heften, steht im Doc-Kommentar von
`wiederaufnahme`. Der Zweig "Profil fehlt" nennt die Zahl jetzt ebenfalls, statt `<zahl>`
auszuschreiben. Gehalten von `jeder_abbruch_der_station_sieben_nennt_die_wiederaufnahme`.
