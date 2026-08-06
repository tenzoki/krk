# KRK

KRK ist ein Dateimanager für macOS in der Tradition der Norton-Commander-Bedienung:
zwei Dateifenster nebeneinander, alles über die Tastatur erreichbar. Editor und
Git-Anbindung folgen in späteren Runden.

Diese Datei beschreibt, wie KRK gebaut, signiert und versioniert wird. Was KRK
können soll, steht im Spec und im Implementierungsplan unter
`fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/planning/`.

## Voraussetzungen

| Werkzeug | Stand | Woher |
|---|---|---|
| Rust | 1.97.1, festgeschrieben in `rust-toolchain.toml` | `rustup` |
| `codesign`, `plutil`, `vtool`, `security` | mit macOS ausgeliefert | Command Line Tools |
| macOS | 15 oder neuer | — |

Ein vollständiges Xcode ist für den Bau **nicht** nötig. Die Command Line Tools
genügen; `xcode-select -p` darf auf `/Library/Developer/CommandLineTools` zeigen.
Erst die Auslieferung an Dritte braucht mehr, weil sie eine Developer-ID-Identität
und eine Beglaubigung durch Apple verlangt.

## Bauen

```sh
cargo build --workspace          # übersetzt alle vier Mitglieder
cargo test  --workspace          # fährt die Tests
cargo clippy --workspace --all-targets
cargo fmt --all --check
```

Der Workspace hat vier Mitglieder: `crates/krk-core` (Kern, kein AppKit),
`crates/krk-ui` (das Binärziel `krk`), `crates/krk-bench` (Prüfordner und
kopflose Messstrecke) und `xtask` (dieses Bauwerkzeug).

`.cargo/config.toml` setzt `MACOSX_DEPLOYMENT_TARGET = "15.0"` für jeden Bau.
Nachweisen lässt sich das am fertigen Binärprogramm:

```sh
vtool -show-build-version target/KRK.app/Contents/MacOS/krk   # meldet minos 15.0
```

### Das Bündel bauen

```sh
cargo xtask bundle
```

`cargo xtask` ist kein eingebautes Cargo-Kommando, sondern der Alias aus dem
Abschnitt `[alias]` der `.cargo/config.toml`.

Der Befehl übersetzt das Binärziel im Profil `release`, legt `target/KRK.app`
neu an, kopiert `resources/Info.plist` mit eingesetzter Version, schreibt
`Contents/PkgInfo` und signiert das Bündel. Ergebnis:

```text
target/KRK.app/
└── Contents/
    ├── Info.plist      Kopie von resources/Info.plist, Version eingesetzt
    ├── PkgInfo         die acht Bytes APPL????
    ├── MacOS/krk       das übersetzte Binärziel
    └── Resources/      noch leer
```

Das Profil ist `release`, weil dasselbe Bündel später die Zeitzusagen aus dem
Spec misst. Zahlen aus einem unoptimierten Bau sagen über diese Zusagen nichts
aus.

**Warum es das Bündel überhaupt gibt.** Ein nacktes Binärprogramm aus dem
Terminal erbt die Freigaben des Terminals und löst keine eigene Rückfrage aus.
Der Zugriff auf Schreibtisch, Dokumente, Downloads, Wechselmedien und
Netzlaufwerke läuft über den Systemmechanismus für Transparenz, Zustimmung und
Kontrolle, und der greift am signierten Anwendungsbündel an. Nur am Bündel ist
also prüfbar, ob KRK diese Zusagen einhält.

## Signierung

`cargo xtask bundle` sucht die Identität in dieser Reihenfolge:

1. die Umgebungsvariable `KRK_SIGN_IDENTITY`, falls sie einen nichtleeren Wert hat;
2. eine Identität mit dem Namen `KRK Entwicklung` im Schlüsselbund;
3. die einzige gültige Identität im Schlüsselbund, falls es genau eine gibt.

Greift keine der drei, **bricht der Bau ab und baut kein Bündel**. Er weicht
nicht auf eine Ad-hoc-Signatur (`codesign -s -`) aus, und das ist der Punkt: eine
Ad-hoc-Signatur bekommt bei jedem Bau einen anderen Hash. Das System hielte dann
jeden Bau für eine andere Anwendung und fragte bei jedem Start erneut nach dem
Zugriff auf die geschützten Ordner. Eine stabile Identität fragt einmal.

Stufe 3 greift bei null gültigen Identitäten nicht und bei mehr als einer auch
nicht, weil die Wahl dann geraten wäre. In beiden Fällen nennt die
Abbruchmeldung die Wege: die ausdrückliche Wahl über `KRK_SIGN_IDENTITY` oder
das Anlegen von `KRK Entwicklung`. Bei mehreren gültigen Identitäten zählt sie
außerdem auf, welche gefunden wurden.

Welche Identitäten es gibt:

```sh
security find-identity -p codesigning      # alle, auch die nicht als gültig bewerteten
security find-identity -v -p codesigning   # nur die gültigen
```

**Die beiden Stufen fragen verschieden ab, und das ist Absicht.** Stufe 2 fragt
ohne `-v`. `-v` zeigt nur die als gültig bewerteten Identitäten, und eine
selbstsignierte Identität ohne gesetzte Vertrauenseinstellung gilt als nicht
vertrauenswürdig (`CSSMERR_TP_NOT_TRUSTED`). Signieren und Prüfen funktionieren
mit ihr trotzdem. Wer `KRK Entwicklung` angelegt hat, hat sich für diese
Identität entschieden, und der Bau hat sie nicht auszusortieren.

Stufe 3 fragt mit `-v`. Dort wählt der Bau aus einer Menge aus, ohne dass jemand
die Wahl getroffen hätte, und automatisch gewählt werden soll nur, was auch
signieren kann. Ohne `-v` griffe die Stufe sonst nach einem abgelaufenen
Zertifikat oder einem, dessen Kette sich nicht aufbaut, und der Bau scheiterte
danach an einer Meldung von `codesign`, die den Grund nicht nennt (siehe den
Abschnitt zur abgelaufenen Zertifikatskette weiter unten).

Die Vertrauenseinstellung braucht erst, wer das Bündel auf einem fremden Rechner
an Gatekeeper vorbeibringen will, und dafür ist ohnehin eine Developer-ID nötig.

### Entwicklungsidentität anlegen

Einmalig, ohne Xcode. Zwei Wege führen zum selben Ergebnis.

**Weg 1: Schlüsselbundverwaltung.** Menü `Schlüsselbundverwaltung` →
`Zertifikatsassistent` → `Ein Zertifikat erstellen`. Name `KRK Entwicklung`,
Identitätstyp `Selbstsigniertes Root-Zertifikat`, Zertifikatstyp `Codesignatur`.

**Weg 2: auf der Kommandozeile.** Schlüssel und Zertifikat erzeugen, in einer
PKCS#12-Datei bündeln und in den Anmeldeschlüsselbund importieren:

```sh
openssl req -x509 -newkey rsa:2048 -keyout krk-entwicklung-key.pem \
  -out krk-entwicklung-cert.pem -days 3650 -nodes \
  -subj "/CN=KRK Entwicklung" \
  -addext "basicConstraints=critical,CA:true" \
  -addext "keyUsage=critical,digitalSignature" \
  -addext "extendedKeyUsage=critical,codeSigning"

openssl pkcs12 -export -out krk-entwicklung.p12 \
  -inkey krk-entwicklung-key.pem -in krk-entwicklung-cert.pem \
  -name "KRK Entwicklung" -passout pass:krk \
  -macalg sha1 -certpbe PBE-SHA1-3DES -keypbe PBE-SHA1-3DES

security import krk-entwicklung.p12 -k ~/Library/Keychains/login.keychain-db \
  -P krk -T /usr/bin/codesign
```

Die drei Algorithmus-Angaben im zweiten Befehl sind nötig, nicht schmückend:
OpenSSL 3 schreibt PKCS#12 sonst mit Verfahren, die der Schlüsselbund nicht
liest, und der Import scheitert mit `MAC verification failed`.

Beim ersten Signieren fragt macOS einmal, ob `codesign` auf den privaten
Schlüssel zugreifen darf. `Immer erlauben` beantwortet das dauerhaft.

Die drei erzeugten Dateien enthalten den privaten Schlüssel und gehören nicht
ins Repository. Nach dem Import sind sie entbehrlich:

```sh
rm krk-entwicklung-key.pem krk-entwicklung-cert.pem krk-entwicklung.p12
```

### Abgelaufene Zertifikatskette (`errSecInternalComponent`)

`codesign` scheitert mit dieser Meldung, obwohl Zertifikat und privater
Schlüssel im Anmeldeschlüsselbund liegen:

```text
Warning: unable to build chain to self-signed root for signer
  "Apple Development: <Name> (<Kennung>)"
errSecInternalComponent
```

Dazu passt: `security find-identity -p codesigning` zeigt die Identität,
`security find-identity -v -p codesigning` meldet null gültige.

**Die Meldung deutet in die falsche Richtung.** Sie nennt die eigene Identität,
und die ist in Ordnung; das Zwischenzertifikat, an dem es liegt, erwähnt sie mit
keinem Wort. Im System-Schlüsselbund liegt das Apple-Zwischenzertifikat in einer
alten Fassung, abgelaufen am 2023-02-07. Ein heute von Apple ausgestelltes
Entwicklerzertifikat kommt dagegen von der G3-Instanz (`issuer=CN=Apple
Worldwide Developer Relations Certification Authority, OU=G3`), und ohne deren
Zertifikat baut sich die Kette zur Apple Root CA nicht auf.

Die Behebung holt das aktuelle Zwischenzertifikat in den Anmeldeschlüsselbund:

```sh
curl -fsS -o AppleWWDRCAG3.cer https://www.apple.com/certificateauthority/AppleWWDRCAG3.cer
security import AppleWWDRCAG3.cer -k ~/Library/Keychains/login.keychain-db
```

Danach meldet `security find-identity -v -p codesigning` die Identität als
gültig, und `codesign -dvv target/KRK.app` zeigt die vollständige Kette bis zur
Apple Root CA samt `TeamIdentifier`.

Das abgelaufene alte Zwischenzertifikat im System-Schlüsselbund muss dafür
**nicht** entfernt werden; die Kette baut sich neben ihm richtig auf. Es zu
entfernen verlangt erhöhte Rechte und ist erst der nächste Versuch, falls der
Fehler trotz vorhandenem G3-Zertifikat bleibt.

### Prüfen, was signiert wurde

```sh
codesign --verify --strict target/KRK.app   # Rückgabewert 0
codesign -dvv target/KRK.app                # nennt die Identität als Authority
```

`flags=0x0(none)` im Kopf der Ausgabe heißt: keine Ad-hoc-Signatur. Eine solche
stünde dort als `flags=0x2(adhoc)`.

## Auslieferung

```sh
cargo xtask release
```

Der Befehl baut das Auslieferungspaket in sechs Stationen; jede bricht mit
einer benennenden Meldung ab, wenn ihre Voraussetzung fehlt:

1. **AppKit-Grenze prüfen.** Keine `use objc2`-Zeile außerhalb von
   `crates/krk-ui/src/appkit/`. `#![deny(unsafe_code)]` erzwingt die Grenze
   nur zur Hälfte, weil ein großer Teil der `objc2`-Bindungen als sicher
   deklariert ist und außerhalb anstandslos übersetzt; diese Prüfung trägt die
   andere Hälfte. Von Hand ist es dieselbe Vorschrift wie im Plan:
   ```sh
   grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src --include='*.rs' \
     | grep -v '^crates/krk-ui/src/appkit/'   # darf keine Zeile ausgeben
   ```
2. **Beide Ziele übersetzen.** `x86_64-apple-darwin` und
   `aarch64-apple-darwin`, dieselben zwei wie in `rust-toolchain.toml`. Fehlt
   eines, nennt der Abbruch das Ziel und das Kommando `rustup target add`.
3. **`lipo`.** Die beiden Binärdateien werden zu
   `target/universal/krk` zusammengefügt und sofort geprüft:
   `lipo -archs` muss `x86_64 arm64` melden.
4. **Montage.** Dasselbe Bündel wie `cargo xtask bundle`, aus denselben
   Funktionen, samt Versionsersetzung — nur wandert die universelle
   Binärdatei nach `Contents/MacOS`.
5. **Signieren.** Dieselbe dreistufige Identitätssuche wie bei `bundle`, nur
   sucht die zweite Stufe nach dem Namensanfang `Developer ID Application`
   statt nach `KRK Entwicklung`. Signiert wird mit gehärteter
   Laufzeitumgebung und gesichertem Zeitstempel
   (`codesign --options runtime --timestamp`); beides verlangt die
   Beglaubigung. Nachprüfen: `codesign -dv --verbose=4 target/KRK.app` zeigt
   `flags=0x10000(runtime)`.
6. **Beglaubigen.** `xcrun notarytool submit --wait` reicht das Bündel als
   Zip bei Apple ein, `xcrun stapler staple` heftet das Urteil an.

Die sechste Station hat zwei äußere Voraussetzungen, und nur sie: das
vollständige Xcode (die Command Line Tools führen weder `notarytool` noch
`stapler`) und ein Apple-Entwicklerkonto. Fehlt eines von beidem, bricht
allein die Beglaubigung ab, und das universell gebaute, signierte Bündel
bleibt unter `target/KRK.app` liegen — für die lokale Arbeit ist es voll
brauchbar.

Die Zugangsdaten des Entwicklerkontos erwartet der Befehl als
Schlüsselbundprofil, dessen Name in der Umgebungsvariablen
`KRK_NOTARY_PROFILE` steht. Einmalig hinterlegen:

```sh
xcrun notarytool store-credentials <Profilname> \
  --apple-id <Apple-ID> --team-id <Team-Kennung> \
  --password <app-spezifisches-Passwort>

KRK_NOTARY_PROFILE=<Profilname> cargo xtask release
```

Findet die Identitätssuche keine Developer-ID, aber genau eine gültige andere
Identität (Stufe 3), läuft der Bau mit ihr durch und sagt dazu, dass die
Beglaubigung ein so signiertes Bündel nicht annehmen wird. So bleiben Bau,
`lipo` und die Signierung mit gehärteter Laufzeitumgebung auch auf einem
Gerät ohne Entwicklerkonto prüfbar.

## Versionspflege

Die Version steht an **einer** Stelle: im Feld `version` unter
`[workspace.package]` der `Cargo.toml`. Jedes Mitglied erbt sie über
`version.workspace = true`.

`resources/Info.plist` trägt bei `CFBundleShortVersionString` nur den
Platzhalter `__KRK_VERSION__`. `cargo xtask bundle` ersetzt ihn beim Kopieren
durch die geerbte Version; die Quelldatei bleibt unangetastet. Findet der Bau
den Platzhalter nicht, bricht er ab und baut kein Bündel — so kann weder eine
veraltete Zahl noch ein versionsloses Bündel unbemerkt entstehen.

Eine neue Version wird also allein in der `Cargo.toml` gesetzt. Nachzuführen ist
nichts. Nachprüfen lässt sich das Ergebnis am gebauten Bündel:

```sh
plutil -extract CFBundleShortVersionString raw target/KRK.app/Contents/Info.plist
```

`CFBundleVersion` in der `Info.plist` ist etwas anderes: die Baunummer. Sie steht
nirgends ein zweites Mal und wird von Hand gepflegt.
