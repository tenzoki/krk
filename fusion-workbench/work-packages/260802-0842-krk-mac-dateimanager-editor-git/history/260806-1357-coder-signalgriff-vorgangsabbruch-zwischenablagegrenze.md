# Signalgriff der Sitzungssicherung, Vorgang ohne Fertig-Meldung, Bildgrenze der Zwischenablage

**Status:** Complete
**Agent:** coder
**Datum:** 260806-1357

## Auftrag

Drei zusammenhängende Befunde der Durchsicht
`reviews/260806-1335-coderev-turn-23-s6b-vorschau-messstrecke.md` beheben:
`260806-1328` (hoch), `260806-1330` (mittel), `260806-1332` (niedrig). Die
übrigen drei Befunde derselben Durchsicht ausdrücklich nicht anfassen.

## 1. Die Sitzungssicherung überlebt jetzt Strg+C (`260806-1328`)

### Der geprüfte Bestand

Vor der Wahl stand die Frage, ob der Bestand den Fall schon trägt. Geprüft:

- `std` kennt keine Signal-Schnittstelle, weder in `std::os::unix` noch
  anderswo. Ohne Fremdcode bleibt nur ein Weg: `libc::sigaction` mit `unsafe`.
- `libc` steht bereits im Abhängigkeitsbaum, über `objc2-app-kit` und
  `objc2-foundation`. Es unmittelbar zu nennen kostete keine neue Kiste — aber
  einen `unsafe`-Block in `krk-bench` und damit den Grenzstein aus CLAUDE.md,
  der `#![allow(unsafe_code)]` auf `krk-core/src/verzeichnis/sys.rs` und
  `krk-ui/src/appkit/mod.rs` beschränkt. Der Rahmen des Auftrags schließt das
  aus.
- Ein Wächterprozess käme ohne Fremdcode und ohne `unsafe` aus und deckte sogar
  SIGKILL ab: ein Kind in eigener Prozessgruppe, das am EOF seiner Standard-
  eingabe merkt, dass der Elternprozess weg ist, und dann zurückspielt.
  Verworfen, weil er ein neues Fehlerbild schafft — ein verwaister Wächter
  schriebe irgendwann in eine Sitzung, die der Nutzer inzwischen selbst
  angelegt hat — und weil er die aufwendigere von zwei Lösungen für dasselbe
  Ziel ist.

### Der gewählte Weg

Neue Abhängigkeit **`signal-hook` 0.4**, allein in `krk-bench`, mit
`default-features = false, features = ["iterator"]`. Auf macOS bringt sie
`signal-hook-registry` und `errno` mit; `libc` steht ohnehin schon im Baum.
`ctrlc` wäre die andere Kandidatin gewesen, zieht aber `nix` nach, eine
deutlich größere Kiste.

Vier Eigenschaften haben entschieden:

1. Der Griff schreibt im Signalkontext nur in ein Selbstrohr. Das
   Zurückspielen läuft auf einem gewöhnlichen Faden und darf deshalb Dateien
   schreiben und melden.
2. `krk-bench` behält `#![deny(unsafe_code)]`; der Grenzstein bleibt, wo er
   steht.
3. Die Registrierung setzt `SA_RESTART` (geprüft in
   `signal-hook-registry-1.4.8/src/lib.rs:187`). Kein Systemaufruf der
   Messstrecke bricht mit `EINTR` ab, keine gemessene Spanne ändert sich.
4. Kein ausgelieferter Code ist berührt. `krk-ui` und `krk-core` fassen kein
   Signal an.

### Die Umsetzung in `crates/krk-bench/src/messen.rs`

- `static SICHERUNG: Mutex<Option<Sitzungssicherung>>` hält die Sicherung an
  einer Stelle, die auch der Signalfaden erreicht.
- `sitzung_zurueckspielen()` nimmt sie mit `take` heraus. Genau ein Aufrufer
  spielt zurück, der zweite läuft ins Leere.
- `Sitzungswaechter` ist der Rückgabewert von `Sitzungssicherung::anlegen()`.
  Sein `Drop` deckt die bisherigen drei Wege ab: reguläres Ende, `?`-Abbruch,
  Panik.
- `signalwache_starten()` hängt SIGINT, SIGTERM und SIGHUP ein und beendet den
  Prozess mit 128 + Signalnummer, nachdem zurückgespielt ist. Scheitert das
  Einhängen, bricht der Lauf ab, statt ungeschützt zu messen.

### Die Zusage im Kommentar, auf das herabgesetzt, was gilt

Der alte Satz "Ein SIGKILL von aussen ueberlebt auch das nicht; alles darunter
schon" ist ersetzt. Der neue Modulkommentar zählt vollständig auf, was
ungedeckt bleibt:

- SIGKILL und SIGSTOP, weil kein Programm sie abfangen kann.
- Ein Signal, das **nur** `krk-bench` erreicht und nicht den laufenden
  `krk`-Kindprozess, also ein `kill` auf die eine Prozessnummer statt Strg+C
  auf die Vordergrundgruppe. Der Wächter spielt dann zurück, das weiterlaufende
  Kind schreibt beim Beenden aber wieder die Prüfsitzung darüber.

### Der Nachweis am laufenden Gerät

Die echte `session.toml` des Nutzers lag während des ganzen Versuchs außerhalb
des Projekts gesichert (`shasum -a 256` = `417f63ee…`) und steht am Ende
byteweise wieder da, mit demselben Datum.

Erster Versuch ohne Aussage: die `session.toml` des Nutzers **war** bereits die
Prüfsitzung, ein Rest früherer Abnahmeläufe. Der Lauf überschrieb sie also mit
identischem Inhalt, und die Gleichheit am Ende hätte nichts bewiesen. Für den
zweiten Versuch ist eine unterscheidbare Probesitzung eingelegt worden.

Ablauf und Ergebnis:

```
== vorher ==
907acf51e73b601e1152f285d86f28b748f89d336f37402a37265f53cf01005f  …/KRK/session.toml
krk-bench laeuft als 91514 (Prozessgruppe 91514)
== die Pruefsitzung steht in der Ablage (nach Durchgang 4) ==
417f63ee53255ac50a69b9ab0b48546488bc56df5ee74f5d2f357b260dc382d8  …/KRK/session.toml
== Strg+C (SIGINT an die Prozessgruppe) ==
Ausgangswert: 130
== nachher ==
907acf51e73b601e1152f285d86f28b748f89d336f37402a37265f53cf01005f  …/KRK/session.toml
ERGEBNIS: byteweise gleich
krk-bench: Signal 2 empfangen, der Lauf bricht ab. Die Sitzung des Nutzers steht wieder.
== laeuft noch ein krk? ==
keiner
```

Gefahren wurde `krk-bench alle` mit dem gebauten Bündel und den drei
Prüfordnern unter `~/Library/Caches/krk-messplatz`. Das Signal ging über
`kill -INT -PGID` an die ganze Prozessgruppe, also genau an das, was ein Strg+C
im Terminal trifft: `krk-bench` und den laufenden `krk`-Kindprozess. Der
Berichtsordner lag außerhalb des Projekts; das Kopierziel ist nach dem Abbruch
leer geblieben.

Die drei Zwischenschritte tragen den Nachweis: die Prüfsitzung hat die
Probesitzung tatsächlich überschrieben, der Ausgangswert 130 kommt aus dem
Signalgriff, und der Hash danach ist der von vorher.

### Prüfung im Code

`der_signalweg_spielt_zurueck_und_dann_nichts_mehr` fährt die Stelle ab, an der
Signalfaden und Wächter zusammenkommen: der erste Aufruf spielt zurück, der
zweite lässt einen inzwischen vom Nutzer geschriebenen Stand stehen. Ein echtes
Signal lässt sich in einem Prüfprozess nicht auslösen, ohne ihn zu beenden.

## 2. Der Vorgang ohne Fertig-Meldung (`260806-1330`)

`vermitteln` (`crates/krk-ui/src/appkit/anwendung.rs`) merkt sich jetzt, ob es
die Schleife über `Meldung::Fertig` verlassen hat. Schloss der Kanal ohne sie,
trägt `abbruch_ohne_meldung_nachtragen` einen gewöhnlichen `Bericht` mit
`Abschluss::Abgebrochen` nach — dieselbe Bahn wie der reguläre Abschluss,
einschließlich Bündelung und Weckruf. `vorgang_beenden` räumt daraufhin von
selbst auf: Fortschrittszeile weg, Ordner aufgefrischt, `ivars.vorgang` geleert.
Ein zweiter Aufräumweg entsteht nicht.

Die Zahlen des Berichts kommen aus dem letzten Zwischenstand und nicht aus
Nullen: was vor dem Abbruch durchlief, ist übertragen. Die übersprungenen
Einträge kommen aus `stand.uebersprungen`, das bis heute geschrieben und nie
gelesen wurde; der Abbruchweg ist sein erster Leser.

Keine Prüfung: `appkit/anwendung.rs` trägt kein Prüfmodul, und die Funktion
hängt am Weckruf über die Hauptschlange.

## 3. Das Bild aus der Zwischenablage (`260806-1332`)

`inhalt_lesen` (`crates/krk-ui/src/appkit/zwischenablage.rs`) fragt jetzt vor
`to_vec()` die Länge des `NSData` ab und gibt oberhalb der Grenze
`Zwischenablageinhalt::BildZuGross(laenge)` zurück. Die Daten bleiben dabei im
Pasteboard-Server liegen, wo sie ohnehin schon stehen.

**Keine zweite Zahl.** Verwendet wird `vorschaumodell::BILDGRENZE`, dieselbe
Konstante wie im Dateiweg. Die offene Frage, ob 64 MB die richtige Zahl ist
(`260806-1329`), bleibt damit unberührt: wer die Konstante ändert, ändert
beide Wege und den Hinweistext mit.

`Vorschaumodell::zwischenablage_anzeigen` bildet den neuen Fall auf
`Inhalt::Hinweis` ab, den Rückfallweg, den die leere Zwischenablage schon
benutzt. Der Text nennt beide Zahlen und entsteht in `zu_gross_text`, das die
Grenze aus der Konstanten liest. Ein eigener Fall und keine Variante von
`Leer`, weil der Nutzer den Unterschied zwischen "nichts kopiert" und "zu groß"
lesen können muss.

Der Modulkopf von `vorschaumodell.rs` sagt jetzt, was gilt: die Bildgrenze
greift auf beiden Wegen in dieselbe Fläche. Eine Textgrenze braucht der Weg
über die Zwischenablage nicht, weil der Text dort schon als `String` im
Speicher liegt.

Prüfung: `ein_bild_ueber_der_bildgrenze_erscheint_als_hinweis`.

## Nicht angefasst

`260806-1329` (Bildgrenze im Spec), `260806-1331` (Umfang des
Auffrischungsaufschubs) und `260806-1333` (Grenzprüfung über drei Kisten).

## Geänderte Dateien

- `Cargo.toml` — `signal-hook` in den gemeinsamen Abhängigkeiten
- `Cargo.lock`
- `crates/krk-bench/Cargo.toml`
- `crates/krk-bench/src/messen.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/zwischenablage.rs`
- `crates/krk-ui/src/vorschaumodell.rs`

## Abnahme

`make check` läuft grün durch, Ausgangswert 0: Bau, Prüfungen, Clippy mit
`-D warnings`, Format. Die Grenze `#![deny(unsafe_code)]` steht unverändert;
die beiden `#![allow(unsafe_code)]` liegen weiter allein in
`krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`. Keine der
zehn Zeitzusagen aus C8 und keine ihrer Messvorschriften ist berührt.

Nicht committet, wie beauftragt.
