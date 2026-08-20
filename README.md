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

Ein Kommando, ein Argument, die Versionszahl:

```sh
./release.sh 0.2.0
```

Das ist der ganze Weg. Er reicht durch drei Schichten, von denen jede genau
eine Sache beiträgt und keine zweimal:

```text
./release.sh 0.2.0
  └─ make ausliefern VERSION=0.2.0        Pfad zu cargo, Notarprofil, Reihenfolge
       ├─ cargo xtask version 0.2.0       Zahl setzen, eintragen, taggen
       └─ cargo xtask release             die sieben Stationen
```

`release.sh` ist kein drittes Bauwerkzeug. Das Projekt hat eines, `xtask`, und
eine Hülle darum, das `Makefile`; das Skript ist die zweite Hülle und trägt
keine Logik. Wer lieber tippt, ruft die beiden Kommandos einzeln — es geht
nichts verloren.

**Warum es unten zwei Kommandos sind und nicht eines.** `xtask` liest die
Versionszahl beim Übersetzen, über `env!("CARGO_PKG_VERSION")`. Zwischen dem
Setzen der Zahl und dem Bau des Bündels muss deshalb ein Prozess enden, sonst
trüge die `Info.plist` die alte Zahl, während der Tag die neue nennt. Der
Umweg ist zugleich der Wachposten: Station 1 läuft im neu übersetzten Werkzeug
und vergleicht die frisch eingebackene Zahl mit dem Tag.

### Zahl, Eintrag, Tag

```sh
cargo xtask version 0.2.0
```

Setzt `version` unter `[workspace.package]` der Wurzel-`Cargo.toml`, trägt
`Cargo.toml` und `Cargo.lock` als **eine** Änderung ein und setzt den Tag
`v0.2.0` auf HEAD. Die `Cargo.lock` muss mit, weil sie die Zahl für jedes
Mitglied mitführt; aufgefrischt wird sie nicht von Hand, sondern von
`cargo update --workspace --offline`.

Erlaubt sind genau drei Zahlenteile ohne führendes `v` — das trägt allein der
Tag. Was wann steigt, steht unter „Versionsstufen".

Geprüft wird vor dem ersten Schreiben:

| Vorprüfung | Abbruch, wenn |
|---|---|
| die Zahl | sie nicht aus drei Zahlenteilen besteht |
| das Git-Verzeichnis | keines zu befragen ist |
| der Arbeitsbaum | eine verfolgte Datei geändert ist |
| der Tagname | `v<zahl>` schon vergeben ist |

Der Abbruch am Arbeitsbaum nennt **jede betroffene Datei beim Namen**. Sind es
allein Dateien unter `fusion-workbench/`, ist es der bekannte Befund
`shared/issues/260813-1515_*`: vier Werkbankdateien sind versioniert, die jeder
Agentenlauf neu schreibt.

**Was ein Abbruch hinterlässt.** Bis zum ersten Schreiben nichts. Scheitert
danach das Auffrischen der `Cargo.lock` oder der Eintrag, werden beide Dateien
auf ihren vorigen Stand zurückgeschrieben. Scheitert allein das Setzen des
Tags, bleibt der Eintrag stehen: er ist für sich richtig, und eine Rücknahme
schriebe Geschichte um. Dasselbe gilt für einen Abbruch der sieben Stationen
danach — Eintrag und Tag bleiben. Der Handgriff ist in beiden Fällen derselbe:
`./release.sh 0.2.0` noch einmal. Der Lauf sieht, dass Zahl und Tag schon
stehen, trägt nichts doppelt ein und fährt gleich weiter.

### Das Paket bauen

```sh
cargo xtask release
```

Der Befehl baut das Auslieferungspaket in sieben Stationen; jede bricht mit
einer benennenden Meldung ab, wenn ihre Voraussetzung fehlt. Dazwischen laufen
drei Vorläufe: sie kosten nichts, stehen deshalb früh und tragen einen
Buchstaben statt einer Zahl, weil ihr Ergebnis erst einer späteren Station
dient.

1. **Tag und Arbeitsbaum prüfen.** HEAD muss einen Tag `v<version>` mit der
   Zahl aus `[workspace.package]` tragen, und keine verfolgte Datei darf
   geändert sein; unbeachtete Dateien zählen nicht mit. Treffen beide Befunde
   zu, nennt eine Meldung beide. Die Station ist die billigste des Weges und
   steht ganz vorn, damit ein Abbruch dieser Art keinen Übersetzungslauf
   kostet. Sie liest allein; geschrieben hat der Halbschritt davor. Von Hand
   sind es diese beiden Fragen:
   ```sh
   git tag --points-at HEAD                    # muss v<version> nennen
   git status --porcelain --untracked-files=no # darf nichts ausgeben
   ```
   - *Vorlauf a:* `resources/Info.plist` wird gelesen und die Bündelvorlage
     gebaut, die Station 5 braucht.
2. **AppKit-Grenze prüfen.** Keine `use objc2`-Zeile außerhalb von
   `crates/krk-ui/src/appkit/`. `#![deny(unsafe_code)]` erzwingt die Grenze
   nur zur Hälfte, weil ein großer Teil der `objc2`-Bindungen als sicher
   deklariert ist und außerhalb anstandslos übersetzt; diese Prüfung trägt die
   andere Hälfte. Von Hand ist es dieselbe Vorschrift wie im Plan:
   ```sh
   grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src --include='*.rs' \
     | grep -v '^crates/krk-ui/src/appkit/'   # darf keine Zeile ausgeben
   ```
   - *Vorlauf b:* die Identitätssuche liefert die Identität für Station 6.
   - *Vorlauf c:* die Zielprüfung über `rustup` ist die Voraussetzung von
     Station 3.
3. **Beide Ziele übersetzen.** `x86_64-apple-darwin` und
   `aarch64-apple-darwin`, dieselben zwei wie in `rust-toolchain.toml`. Fehlt
   eines, nennt der Abbruch das Ziel und das Kommando `rustup target add`.
4. **`lipo`.** Die beiden Binärdateien werden zu
   `target/universal/krk` zusammengefügt und sofort geprüft:
   `lipo -archs` muss `x86_64 arm64` melden.
5. **Montage.** Dasselbe Bündel wie `cargo xtask bundle`, aus denselben
   Funktionen, samt Versionsersetzung — nur wandert die universelle
   Binärdatei nach `Contents/MacOS`.
6. **Signieren.** Dieselbe dreistufige Identitätssuche wie bei `bundle`, nur
   sucht die zweite Stufe nach dem Namensanfang `Developer ID Application`
   statt nach `KRK Entwicklung`. Signiert wird mit gehärteter
   Laufzeitumgebung und gesichertem Zeitstempel
   (`codesign --options runtime --timestamp`); beides verlangt die
   Beglaubigung. Nachprüfen: `codesign -dv --verbose=4 target/KRK.app` zeigt
   `flags=0x10000(runtime)`.
7. **Beglaubigen.** `xcrun notarytool submit --wait` reicht das Bündel als
   Zip bei Apple ein, `xcrun stapler staple` heftet das Urteil an.

Die siebte Station hat zwei äußere Voraussetzungen, und nur sie: das
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

### Nur beglaubigen

```sh
./certify-only.sh 0.2.0
```

Der Weg für den Fall, dass der Lauf **erst an der siebten Station** gescheitert
ist: das universelle, mit Developer-ID und gehärteter Laufzeitumgebung
signierte Bündel liegt fertig unter `target/KRK.app`, und allein das Ticket
fehlt. So geschehen am 260820, als der Upload zu Apple in einen Zeitüberlauf
lief (`HTTPClientError.deadlineExceeded`).

Dieselben Schichten wie beim ganzen Weg, eine weniger:

```text
./certify-only.sh 0.2.0
  └─ make beglaubigen VERSION=0.2.0        Pfad zu cargo, Notarprofil
       └─ cargo xtask beglaubigen 0.2.0    die Prüfungen und Station 7
```

**Ein zweites `./release.sh 0.2.0` hilft in dieser Lage nicht.** Es bräche an
Station 1 ab, denn nach dem Lauf trägt HEAD den Tag `v0.2.0` nicht mehr allein,
und der Arbeitsbaum ist inzwischen ein anderer — und es übersetzte beide Ziele
neu, um dasselbe Bündel ein zweites Mal herzustellen.

Geprüft wird zweierlei, und beides am Bündel, das dort liegt:

| Prüfung | Abbruch, wenn |
|---|---|
| die Versionszahl | sie von `CFBundleShortVersionString` der `Info.plist` im Bündel abweicht |
| der Signaturstand | keine `Authority=`-Zeile mit `Developer ID Application` beginnt oder die Merkmalsliste `runtime` nicht nennt |

Die erste Prüfung ist es, die das Argument rechtfertigt: `target/KRK.app`
überlebt jede Sitzung, und ohne sie ginge ein Bündel von vorgestern still bei
Apple ein. Die zweite spart eine sinnlose Einreichung, denn ein mit
`cargo xtask bundle` gebautes Bündel trägt eine Entwicklungsidentität und keine
gehärtete Laufzeitumgebung; Apple weist es ab. Gegen die `Cargo.toml` wird
nicht geprüft — sie sagt, was ein *neuer* Bau trüge, und der findet hier nicht
statt.

**Gebaut wird nichts**: kein Übersetzungslauf, kein `lipo`, keine Montage,
keine Signierung. Fehlt das Bündel, bricht der Befehl ab und nennt
`cargo xtask release`.

**Weder Tag noch Arbeitsbaum werden geprüft, und das ist der Zweck des Wegs.**
Station 1 zu übergehen ist seine Daseinsberechtigung und zugleich seine
Grenze: ein so beglaubigtes Bündel ist nicht durch die Vorprüfungen der
Auslieferungskette gegangen, und es ist nicht gesagt, dass ein Tag den Stand
benennt, aus dem es gebaut wurde. Wer von Grund auf ausliefert, nimmt
`./release.sh <version>`.

## Versionspflege

Die Version steht an **einer** Stelle: im Feld `version` unter
`[workspace.package]` der `Cargo.toml`. Jedes Mitglied erbt sie über
`version.workspace = true`.

`resources/Info.plist` trägt bei `CFBundleShortVersionString` nur den
Platzhalter `__KRK_VERSION__`. `cargo xtask bundle` ersetzt ihn beim Kopieren
durch die geerbte Version; die Quelldatei bleibt unangetastet. Findet der Bau
den Platzhalter nicht, bricht er ab und baut kein Bündel — so kann weder eine
veraltete Zahl noch ein versionsloses Bündel unbemerkt entstehen.

Eine neue Version wird also allein in der `Cargo.toml` gesetzt, und im Baum ist
danach nichts nachzuführen. Von Hand geschieht auch das nicht mehr:
`./release.sh <version>` setzt die Zahl, trägt sie ein, setzt den Tag und
liefert aus (siehe „Auslieferung"). Nachprüfen lässt sich das Ergebnis am
gebauten Bündel:

```sh
plutil -extract CFBundleShortVersionString raw target/KRK.app/Contents/Info.plist
```

`CFBundleVersion` in der `Info.plist` ist etwas anderes: die Baunummer. Sie steht
nirgends ein zweites Mal und wird von Hand gepflegt.

### Versionsstufen

Wann welche der drei Zahlen steigt, misst sich an KRKs eigenen Flächen und
nicht an einer Programmierschnittstelle: KRK ist eine Anwendung und keine
Bibliothek, und die Stelle des Vertrags nehmen die Flächen ein, die der Nutzer
sieht und speichert.

- **Major** steigt, wenn KRK etwas hergibt, worauf sich der Nutzer eingerichtet
  hat: die Bedeutung eines Tastenbefehls ändert sich oder eine Kombination
  fällt weg, eine Datei unter `~/Library/Application Support/KRK/` wird nicht
  mehr gelesen, wie sie geschrieben wurde, das Mindest-Zielsystem steigt, oder
  ein Befehl des Bauwerkzeugs verschwindet oder bedeutet etwas anderes.
- **Minor** steigt bei jeder neuen Fähigkeit, also bei jeder Runde, die eine
  bringt. Ein neuer Befehl in der Tastenbelegung und ein neuer Unterbefehl des
  Bauwerkzeugs zählen dazu.
- **Patch** steigt bei Behebungen ohne neue Fähigkeit.

**Jede Auslieferung bekommt einen Tag `v<version>`, und den setzt das
Werkzeug.** Bis zum 260813-1534 galt das Gegenteil: der Nutzer setzte ihn von
Hand, und das Werkzeug erzeugte unter keinen Umständen einen. Der Nutzer hat
diese Festlegung am selben Tag zurückgenommen, weil sie einen Auslieferungsweg
in einem Kommando unmöglich macht — ein Weg, der den Tag nicht setzt, braucht
zwei Kommandos, und damit entfällt die Ersparnis. Der Entscheid ist
`shared/decisions/260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`;
er überholt `circles/260813-0939-.../decisions/260813-0939_*_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md`.

Der Tag bleibt trotzdem ein bewusster Akt, nur liegt der Vorsatz jetzt im
Argument: wer `./release.sh 0.2.0` tippt, hat die Zahl gewählt, und der Tag
folgt daraus mechanisch. Verschoben wird nie einer — ein vergebener Name hält
den Lauf an.

`v0.1.0` benennt den ersten getaggten Stand und keine Weitergabe. Er steht auf
dem Commit, der die Runde vom 260813 schließt, damit der grüne Fall der
Prüfung an einem echten Lauf zu sehen ist; ein Bündel ist an diesem Tag an
niemanden gegangen. Er ist zugleich der einzige von Hand gesetzte. Rückwirkende
Tags für die Runden davor gibt es nicht: alle liefen auf derselben Zahl
`0.1.0`, und sieben Marken für eine Zahl schrieben eine
Auslieferungsgeschichte, die es nicht gab.

**Was `cargo xtask release` prüft** (Station 1, siehe „Auslieferung"): dass
HEAD einen Tag mit genau diesem Namen trägt, und dass keine verfolgte Datei
des Verzeichnisses geändert ist. Vorgemerkte und nicht vorgemerkte Änderungen
zählen gleich, gelöschte verfolgte Dateien zählen mit.

**Was es nicht prüft:** unbeachtete Dateien. Ein Bauergebnis, eine Notiz oder
ein Messbericht, der nie eingetragen wurde, hält die Auslieferung nicht auf.
Und die Prüfung hängt allein an `release`: `cargo xtask bundle` baut jederzeit
ohne Tag, ebenso jedes Ziel des `Makefile`, das an `bundle` hängt, und
`make check` bleibt unberührt.

**Die Zahl, die KRK anzeigt, ist an jedem Bau dieselbe.** Sie stammt immer aus
der `Cargo.toml`, gleich ob der Bau aus einem getaggten Stand kommt oder nicht;
ein Entwicklungsbau zeigt keine andere Zahl als ein ausgeliefertes Bündel und
sagt an der Anzeige auch nicht, dass er einer ist. Die Deckung der Zahl durch
einen Tag hängt deshalb an der Auslieferung und nicht an jedem Bau: ein
Entwicklungsbündel darf `0.1.0` zeigen, ohne dass der Tag existiert, ein
ausgeliefertes nicht.

Wo die Zahl wohnt und wie sie in die `Info.plist` kommt, steht im Abschnitt
darüber und wird hier nicht wiederholt.
