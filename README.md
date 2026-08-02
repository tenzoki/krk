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
2. eine Identität mit dem Namen `KRK Entwicklung` im Schlüsselbund.

Findet es keine von beiden, **bricht es ab und baut kein Bündel**. Es weicht
nicht auf eine Ad-hoc-Signatur (`codesign -s -`) aus, und das ist der Punkt: eine
Ad-hoc-Signatur bekommt bei jedem Bau einen anderen Hash. Das System hielte dann
jeden Bau für eine andere Anwendung und fragte bei jedem Start erneut nach dem
Zugriff auf die geschützten Ordner. Eine stabile Identität fragt einmal.

Welche Identitäten es gibt:

```sh
security find-identity -p codesigning
```

Die Liste wird ohne `-v` abgefragt. `-v` zeigt nur die als gültig bewerteten
Identitäten, und eine selbstsignierte Identität ohne gesetzte
Vertrauenseinstellung gilt als nicht vertrauenswürdig
(`CSSMERR_TP_NOT_TRUSTED`). Signieren und Prüfen funktionieren mit ihr trotzdem;
die Vertrauenseinstellung braucht erst, wer das Bündel auf einem fremden Rechner
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

### Prüfen, was signiert wurde

```sh
codesign --verify --strict target/KRK.app   # Rückgabewert 0
codesign -dvv target/KRK.app                # nennt die Identität als Authority
```

`flags=0x0(none)` im Kopf der Ausgabe heißt: keine Ad-hoc-Signatur. Eine solche
stünde dort als `flags=0x2(adhoc)`.

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
