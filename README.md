# KRK

KRK ist ein Dateimanager mit eingebautem Editor für macOS, in der Tradition der
Norton-Commander-Bedienung: zwei Dateifenster nebeneinander, Lesezeichen- und
Geräteleiste links, Vorschau rechts, alles über die Tastatur erreichbar bei
zusätzlicher Maus- und Trackpad-Unterstützung. Die Git-Anbindung liest und
schreibt nicht: ein eigener Bereich zeigt Branch, Statuszusammenfassung und
Verlauf, und die Dateiliste trägt die Git-Marke jedes Eintrags. Hinzufügen,
Committen, Änderungen verwerfen und der Versions-Schieberegler sind nicht
gebaut.

**Voraussetzung: macOS 15 oder neuer.** Das ausgelieferte Bündel ist beglaubigt
und trägt den Nachweis von Apple angeheftet; es startet deshalb ohne Rückfrage,
auch auf einem Mac ohne Netzverbindung.

## Herunterladen und installieren

Das jeweils neueste Bündel liegt als Zip auf der Releaseseite:

<https://github.com/tenzoki/krk/releases/latest>

1. `KRK-<version>.zip` herunterladen und entpacken. Der Ordner
   `KRK-<version>` trägt danach `KRK.app` und daneben die Anleitung
   `HowTo.md`.
2. KRK beenden, falls es läuft.
3. Die neue Fassung über die alte in `/Applications` kopieren und das Ersetzen
   bestätigen.

Die Anleitung reist im Paket mit, damit sie zur Hand ist, wo die App ankommt.
Sie beschreibt die Bedienung; die vollständige Tastenbelegung gibt die laufende
Anwendung selbst aus, mit **F1**. Im Quellbaum liegt dieselbe Datei als
`HowTo.md` in der Wurzel, und `xtask` backt sie beim Übersetzen ein — fehlt sie,
übersetzt das Bauwerkzeug nicht, statt ein Paket ohne sie auszuliefern.

**Die alte Fassung vorher nicht löschen.** Ein Überkopieren ist gefahrlos, ein
Löschen ist es nicht. Werkzeuge, die eine App samt ihrer Stützdateien entfernen
— der App Deleter von ForkLift ist eines —, nehmen dabei den Ordner
`~/Library/Application Support/KRK/` mit. Dort hält KRK alles, was es sich
merkt: die Lesezeichen, die gesicherte Sitzung, die abweichende Tastenbelegung
und die zwei Notizzettel. Nach so einem Löschen sind sie fort.

Wer doch löschen will, kopiert `~/Library/Application Support/KRK/` vorher an
eine andere Stelle und schreibt die Kopie nach der Installation zurück.

Die Regel ist gemessen und nicht geraten: sie stammt aus der Untersuchung eines
Lesezeichenverlusts nach einer Installation am 260820. Wer im Quellbaum liest,
findet sie unter
`fusion-workbench/shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`;
wer nur das Zip in der Hand hat, braucht diese Datei nicht, denn die Regel steht
hier vollständig. Dasselbe sagt der Text jeder Releaseseite: er steht an **einer**
Stelle, als Konstante `RELEASETEXT` in `xtask/src/veroeffentlichung.rs`, und jede
seiner Aussagen hängt dort an einer eigenen Behauptung der Probe
`der_releasetext_traegt_jede_seiner_aussagen`.

## Neuerungen an den eigenen Dateien übernehmen

Drei Dateien in `~/Library/Application Support/KRK/` gehören dem Nutzer, und KRK
legt jede von ihnen beim ersten Start an und schreibt sie danach nie wieder:
`keymap.toml` mit der Tastenbelegung, `settings.toml` mit den Einstellungen ohne
Oberfläche und `readers.toml` mit den Leseprofilen der Vorschau. Bringt eine
neue Fassung darin einen Eintrag mit, den die eigene Datei nicht führt, kommt er
nicht von selbst an. Die eigene Datei bleibt, wie sie ist, und das ist kein
Fehler: eine unveränderte Datei ist nicht beschädigt.

**KRK sagt es beim Start.** Liegt eine der drei hinter der Auslieferungsfassung
zurück, steht in der Statuszeile eine Zeile, die je betroffener Datei die Zahl
der neuen Einträge nennt und dahinter den Ordner, in dem die eigenen Dateien
liegen. Genannt sind nur die Dateien, an denen es einen Unterschied gibt, und
die Zeile kommt **einmal je Fassung**: der zweite Start derselben Fassung zeigt
sie nicht mehr.

Das Einzelne zeigt der Befehl **„Neuerungen anzeigen"**, im Hauptmenü unter
„Anwendung" und ab Werk auf `opt+cmd+i`. Er öffnet ein Blatt, das für jede der
drei Dateien ihren vollen Pfad nennt, die Namen der Einträge, die nur die
Auslieferungsfassung führt, die Namen der Einträge, die nur die eigene Datei
führt, und einen Satz darüber, was ein fehlender Eintrag an dieser Datei kostet.
Wer eine eigene `keymap.toml` von vor dieser Fassung hat, findet den Befehl
selbst als Beispiel: seine Datei kennt `neuerungen_zeigen` nicht, also steht der
Eintrag bei ihm ohne Kürzel am Fuß der Gruppe „Anwendung", und über das
Hauptmenü ist er trotzdem erreichbar.

**Was ein fehlender Eintrag kostet, ist bei den drei Dateien verschieden.**

- `readers.toml`: Das neue Profil gibt es für KRK nicht. Die Vorschau zeigt an
  dem Ort, den es erkannt hätte, weiter die Metadaten.
- `settings.toml`: Die neue Einstellung wirkt schon, mit dem Wert der
  Auslieferungsfassung. Was fehlt, ist ihr erklärender Kommentarblock — man
  erfährt aus der eigenen Datei nicht, dass es sie gibt.
- `keymap.toml`: KRK hängt eine Funktion, die die eigene Datei nicht nennt,
  unbelegt hinten an ihre Gruppe an. Der Befehl steht im Hauptmenü und tut, was
  er soll, aber ohne Tastenkombination und nicht an seinem Platz.

Die Neuerungen holt man sich in drei Schritten, für jede betroffene Datei
einzeln:

1. KRK beenden.
2. Die Datei beiseitelegen, etwa als `readers.toml.alt` im selben Ordner.
3. KRK starten. Die Datei entsteht neu aus der Auslieferungsfassung, samt allen
   Kommentaren darin.

**Beiseitelegen und nicht löschen.** Es ist derselbe Grund wie beim
Installieren: was KRK sich merkt, liegt außerhalb des Bündels, und ein
Handgriff, der es mitnimmt, hat es genommen. In der beiseitegelegten Datei
stehen die eigenen Einträge und die eigenen Änderungen an den ausgelieferten;
wer sie löscht, hat sie nicht mehr, denn die neu angelegte Datei kennt nur die
Auslieferungsfassung. Aus der beiseitegelegten holt man sie sich Zeile für Zeile
zurück, und das Blatt aus „Neuerungen anzeigen" sagt dabei unter „Nur in Ihrer
Datei", welche Einträge das sind.

---

Alles Weitere richtet sich an den, der KRK baut, signiert und ausliefert.

## Voraussetzungen

| Werkzeug | Stand | Woher |
|---|---|---|
| Rust | festgeschrieben in `rust-toolchain.toml` | `rustup` |
| `codesign`, `plutil`, `vtool`, `security` | mit macOS ausgeliefert | Command Line Tools |
| macOS | 15 oder neuer | — |
| `gh` | nur für die Releaseseite, nicht für den Bau | `brew install gh`, danach `gh auth login` |

Ein vollständiges Xcode ist für den Bau **nicht** nötig; die Command Line Tools
genügen, und `xcode-select -p` darf auf `/Library/Developer/CommandLineTools`
zeigen.

Erst die Auslieferung an Dritte braucht dreierlei mehr: eine
Developer-ID-Identität, für die Beglaubigung das vollständige Xcode samt
Apple-Entwicklerkonto (die Command Line Tools führen weder `notarytool` noch
`stapler`), und `gh` für die Releaseseite. Fehlt eine der drei, bricht allein die
Station ab, die sie verlangt; was bis dahin gebaut ist, bleibt liegen.

## Bauen

```sh
cargo build --workspace          # übersetzt alle vier Mitglieder
cargo test  --workspace          # fährt die Tests
cargo clippy --workspace --all-targets
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

Der Workspace hat vier Mitglieder: `crates/krk-core` (Kern, kein AppKit),
`crates/krk-ui` (das Binärziel `krk`), `crates/krk-bench` (Prüfordner und
kopflose Messstrecke) und `xtask` (dieses Bauwerkzeug). Dieselben fünf Kommandos
in einem Zug fährt `make check`; die übrigen Ziele listet `make help`.

Das fünfte, `cargo doc`, ist seit dem 260906 dabei. Es hält die Verweise in den
Dokumentationskommentaren gegen den Baum: ein Verweis auf einen Namen, den es
nicht mehr gibt, etwa nach einer Umbenennung, bricht den Lauf mit Exit 101 ab.
`--no-deps` gehört
dazu, sonst dokumentiert der Lauf auch die fremden Kisten und meldet deren
Warnungen, die dieses Projekt nicht beheben kann. Am warmen Baum kostet das Ziel
unter einer Sekunde.

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

Der Befehl übersetzt das Binärziel im Profil `release`, legt `target/KRK.app` neu
an, kopiert `resources/Info.plist` mit eingesetzter Version, schreibt
`Contents/PkgInfo` und signiert das Bündel:

```text
target/KRK.app/
└── Contents/
    ├── Info.plist      Kopie von resources/Info.plist, Version eingesetzt
    ├── PkgInfo         die acht Bytes APPL????
    ├── MacOS/krk       das übersetzte Binärziel
    └── Resources/      noch leer
```

Das Profil ist `release`, weil dasselbe Bündel später die Zeitzusagen aus dem
Spec misst; Zahlen aus einem unoptimierten Bau sagen über diese Zusagen nichts
aus. Und gemessen wird am Bündel und nicht am nackten Binärprogramm, weil der
Zugriff auf Schreibtisch, Dokumente, Downloads und Netzlaufwerke über den
Systemmechanismus für Transparenz, Zustimmung und Kontrolle läuft, und der greift
am signierten Anwendungsbündel an.

## Signierung

`cargo xtask bundle` sucht die Identität in dieser Reihenfolge:

1. die Umgebungsvariable `KRK_SIGN_IDENTITY`, falls sie einen nichtleeren Wert hat;
2. eine Identität mit dem Namen `KRK Entwicklung` im Schlüsselbund;
3. die einzige gültige Identität im Schlüsselbund, falls es genau eine gibt.

Greift keine der drei, **bricht der Bau ab und baut kein Bündel**. Auf eine
Ad-hoc-Signatur (`codesign -s -`) weicht er nicht aus: die bekäme bei jedem Bau
einen anderen Hash, das System hielte jeden Bau für eine andere Anwendung und
fragte bei jedem Start erneut nach dem Zugriff auf die geschützten Ordner.

Welche Identitäten es gibt, zeigen zwei Abfragen; ihr Unterschied ist der
zwischen Stufe 2 und Stufe 3:

```sh
security find-identity -p codesigning      # alle, auch die nicht als gültig bewerteten
security find-identity -v -p codesigning   # nur die gültigen
```

### Entwicklungsidentität anlegen

Einmalig, ohne Xcode, über die Schlüsselbundverwaltung: Menü
`Schlüsselbundverwaltung` → `Zertifikatsassistent` → `Ein Zertifikat erstellen`.
Name `KRK Entwicklung`, Identitätstyp `Selbstsigniertes Root-Zertifikat`,
Zertifikatstyp `Codesignatur` — die Vorgabe des Assistenten ist eine andere.
Beim ersten Signieren fragt macOS einmal, ob `codesign` auf den privaten
Schlüssel zugreifen darf; `Immer erlauben` beantwortet das dauerhaft.

### Abgelaufene Zertifikatskette (`errSecInternalComponent`)

Der häufigste Fehler: `codesign` scheitert mit `errSecInternalComponent` und
`unable to build chain to self-signed root`, die erste Abfrage von oben zeigt die
Identität, die zweite meldet null gültige. **Die Meldung deutet in die falsche
Richtung** — sie nennt die eigene Identität, und die ist in Ordnung. Es liegt am
Apple-Zwischenzertifikat, das im System-Schlüsselbund in einer 2023 abgelaufenen
Fassung steht. Die aktuelle in den Anmeldeschlüsselbund holen genügt; das alte
muss **nicht** weichen, denn die Kette baut sich neben ihm richtig auf:

```sh
curl -fsS -o AppleWWDRCAG3.cer https://www.apple.com/certificateauthority/AppleWWDRCAG3.cer
security import AppleWWDRCAG3.cer -k ~/Library/Keychains/login.keychain-db
```

### Prüfen, was signiert wurde

```sh
codesign --verify --strict target/KRK.app   # Rückgabewert 0
codesign -dvv target/KRK.app                # Authority = die Identität,
                                            # flags=0x0(none) = nicht ad hoc
```

## Ein Release machen

Ein Kommando, ein Argument, die Versionszahl:

```sh
./release.sh 0.2.0
```

Das ist der ganze Weg. Er reicht durch drei Schichten, von denen jede genau eine
Sache beiträgt und keine zweimal:

```text
./release.sh 0.2.0
  └─ make ausliefern VERSION=0.2.0        Pfad zu cargo, Notarprofil, Reihenfolge
       ├─ cargo xtask version 0.2.0       Zahl setzen, eintragen, taggen
       └─ cargo xtask release             die acht Stationen
```

**Warum es unten zwei Kommandos sind und nicht eines.** `xtask` liest die
Versionszahl beim Übersetzen, über `env!("CARGO_PKG_VERSION")`. Zwischen dem
Setzen der Zahl und dem Bau des Bündels muss deshalb ein Prozess enden, sonst
trüge die `Info.plist` die alte Zahl, während der Tag die neue nennt.

### Zahl, Eintrag, Tag

`cargo xtask version <version>` setzt `version` unter `[workspace.package]` der
Wurzel-`Cargo.toml`, trägt `Cargo.toml` und `Cargo.lock` als **eine** Änderung
ein und setzt den Tag `v<version>` auf HEAD. Erlaubt sind genau drei Zahlenteile
ohne führendes `v` — das trägt allein der Tag. Was wann steigt, steht unter
„Versionsstufen".

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
zurückgeschrieben. Scheitert allein das Setzen des Tags oder eine der Stationen
danach, bleiben Eintrag und Tag stehen: sie sind für sich richtig, und eine
Rücknahme schriebe Geschichte um. Der Handgriff ist derselbe — `./release.sh
<version>` noch einmal. Der Lauf sieht, dass Zahl und Tag schon stehen, trägt
nichts doppelt ein und fährt gleich weiter.

### Die acht Stationen

`cargo xtask release` baut und verteilt das Paket in acht Stationen; jede bricht
mit einer benennenden Meldung ab, wenn ihre Voraussetzung fehlt.

| | Station | Was sie tut |
|---|---|---|
| 1 | Stand prüfen | HEAD trägt `v<version>` passend zu `[workspace.package]`, keine verfolgte Datei ist geändert, `gh` ist vorhanden und angemeldet, und die Gegenseite führt noch kein Release `v<version>` |
| 2 | AppKit-Grenze | `objc2` steht außerhalb von `crates/krk-ui/src/appkit/` nirgends, weder als `use`-Zeile noch als ausgeschriebener Pfad |
| 3 | Übersetzen | `x86_64-apple-darwin` und `aarch64-apple-darwin`, dieselben zwei wie in `rust-toolchain.toml` |
| 4 | `lipo` | die zwei Binärdateien zu `target/universal/krk`, geprüft mit `lipo -archs` |
| 5 | Montage | dasselbe Bündel wie `cargo xtask bundle`, nur mit der universellen Binärdatei |
| 6 | Signieren | Developer-ID, gehärtete Laufzeitumgebung, gesicherter Zeitstempel |
| 7 | Beglaubigen | `xcrun notarytool submit --wait`, danach `xcrun stapler staple` |
| 8 | Veröffentlichen | Zip aus Bündel und Anleitung packen, HEAD und Tag schieben, Releaseseite anlegen |

Drei Dinge daran sind nicht offensichtlich:

- **Station 1 stellt zwei Fragen, die erst Station 8 braucht:** ob `gh` da und
  angemeldet ist, und ob die Gegenseite dieses Release schon führt. Eine
  fehlende Voraussetzung soll auffallen, solange noch nichts geschehen ist; am
  Kopf der achten Station wären bereits drei Übersetzungsläufe und eine
  Einreichung bei Apple gelaufen. Die zweite Frage geht dabei ins Netz und die
  erste nicht. `cargo xtask bundle` und `make check` bekommen dadurch keine
  Abhängigkeit von `gh`.
- **Station 2 trägt die Hälfte, die `#![deny(unsafe_code)]` nicht trägt.** Ein
  großer Teil der `objc2`-Bindungen ist als sicher deklariert und übersetzte
  außerhalb von `appkit/` anstandslos.
- **Station 8 ist die einzige Wirkung der ganzen Kette, die über dieses Gerät
  hinausgeht und sich nicht zurücknehmen lässt.** HEAD und `refs/tags/v<version>`
  gehen in **einem** Aufruf zur Gegenseite, damit kein Zwischenzustand entsteht,
  in dem der Zweig oben steht und der Tag nicht.

Die Zugangsdaten des Entwicklerkontos erwartet Station 7 als Schlüsselbundprofil,
dessen Name in `KRK_NOTARY_PROFILE` steht. Einmalig hinterlegen:

```sh
xcrun notarytool store-credentials <Profilname> \
  --apple-id <Apple-ID> --team-id <Team-Kennung> \
  --password <app-spezifisches-Passwort>
```

Findet die Identitätssuche keine Developer-ID, aber genau eine gültige andere
Identität, läuft der Bau mit ihr durch und sagt dazu, dass die Beglaubigung ein
so signiertes Bündel nicht annehmen wird. So bleiben Bau, `lipo` und die
Signierung mit gehärteter Laufzeitumgebung auch auf einem Gerät ohne
Entwicklerkonto prüfbar.

### Nur beglaubigen

```sh
./certify-only.sh <version>      # make beglaubigen VERSION=… → cargo xtask beglaubigen
```

Für den Fall, dass der Lauf **erst an Station 7** gescheitert ist: das
universelle, mit Developer-ID und gehärteter Laufzeitumgebung signierte Bündel
liegt fertig unter `target/KRK.app`, und allein das Ticket fehlt. So geschehen am
260820, als der Upload zu Apple in einen Zeitüberlauf lief. **Ein zweites
`./release.sh` hilft hier nicht:** es übersetzte beide Ziele neu, um dasselbe
Bündel herzustellen, und reichte es ein zweites Mal bei Apple ein. An Station 1
hält es dabei nur an, wenn seit dem ersten Lauf etwas eingetragen oder geändert
wurde — `stand_pruefen` fragt allein nach einem passenden Tag auf HEAD und einem
sauberen Arbeitsbaum, und mehrere Tags auf HEAD stören sie nicht. Auch die
vierte Frage der Station greift hier nicht: ist der Lauf an Station 7
gescheitert, hat Station 8 nie eine Releaseseite angelegt, und die Vorabfrage
danach kommt durch. Der Aufwand ist der Grund für diesen Weg, nicht ein Abbruch,
auf den man sich verlassen könnte.

Geprüft wird am Bündel, das dort liegt, und an nichts sonst:

| Prüfung | Abbruch, wenn |
|---|---|
| die Versionszahl | sie von `CFBundleShortVersionString` der `Info.plist` im Bündel abweicht |
| der Signaturstand | das Bündel nicht so signiert ist, dass Apple es annimmt und es danach auf jedem Mac läuft — was `signaturstand_pruefen` (`xtask/src/beglaubigung.rs`) fragt und der Abbruch einzeln benennt |

Die erste rechtfertigt das Argument: `target/KRK.app` überlebt jede Sitzung, und
ohne sie ginge ein Bündel von vorgestern still bei Apple ein. Die zweite spart
eine sinnlose Einreichung, denn ein mit `cargo xtask bundle` gebautes Bündel
trägt eine Entwicklungsidentität und keine gehärtete Laufzeitumgebung.

**Die zweite Zeile nennt hier keine Zahl von Teilfragen**, und der Grund ist,
dass sie schon zweimal falsch geworden ist: der Doc-Kommentar von
`signaturstand_pruefen` nennt aus demselben Grund keine mehr. Wer sie braucht,
liest sie dort.

**Gebaut wird nichts**, und **weder Tag noch Arbeitsbaum werden geprüft**. Station
1 zu übergehen ist der Zweck des Wegs und zugleich seine Grenze: es ist nicht
gesagt, dass ein Tag den Stand benennt, aus dem das Bündel gebaut wurde. Wer von
Grund auf ausliefert, nimmt `./release.sh <version>`.

### Nur veröffentlichen

```sh
~/.cargo/bin/cargo xtask veroeffentlichen <version>
```

Für den Fall, dass der Lauf **erst an Station 8** gescheitert ist: das
beglaubigte Bündel liegt fertig unter `target/KRK.app`, und allein die Weitergabe
fehlt. Der Weg ist Station 8 allein, so wie `cargo xtask beglaubigen` Station 7
allein ist.

**Für diesen Weg gibt es keine Hülle**, weder ein Skript noch ein Ziel im
`Makefile`; ob er eine bekommt, liegt dem Nutzer vor
(`fusion-workbench/shared/decisions/260821-1115_*_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md`).
Solange es keine gibt, trägt der Aufruf den vollen Pfad zu `cargo`, denn auf
diesem Gerät steht `cargo` nicht auf dem Standard-`PATH`. Wer den Pfad lieber
einmal setzt, tut es mit `export PATH="$HOME/.cargo/bin:$PATH"`.

| Schritt | Was geschieht |
|---|---|
| `gh` prüfen | vorhanden (`gh --version` startet) und angemeldet (`gh auth status` gibt null zurück) |
| Tag prüfen | `v<zahl>` steht auf HEAD |
| Ticket prüfen | `target/KRK.app/Contents/CodeResources` beginnt mit den vier Bytes `s8ch` |
| packen | `target/KRK-<zahl>.zip` mit `ditto -c -k --keepParent`, gepackt wird der Ordner `KRK-<zahl>` aus `KRK.app` und `HowTo.md` |
| schieben | `git push origin HEAD refs/tags/v<zahl>`, ein Aufruf |
| anlegen | `gh release create v<zahl>`, öffentlich, mit dem Zip als einziger Datei |

Die drei Prüfungen stehen vorn, und das ist die Zusage des Wegs: bricht er an
einer von ihnen ab, liegt danach kein Zip da und es ist nichts geschoben. Ob das
Ticket hängt, fragt der Befehl an einer Datei und nicht bei Apple — `xcrun
stapler validate` bräuchte Netz, während hier gerade zu prüfen ist, ob das Bündel
den Nachweis *mitbringt*. Fehlt das Ticket, nennt der Abbruch
`./certify-only.sh <zahl>`; fehlt das Bündel, `./release.sh <zahl>`. **Ein
zweiter Lauf mit derselben Zahl legt nichts doppelt an:** vor dem Anlegen steht
die Existenzfrage `gh release view v<zahl>`.

**Den Arbeitsbaum prüft er nicht**, mit derselben Grenze wie beim
Nur-Beglaubigungsweg.

#### Einmal vor dem ersten Lauf: die alten Tags nachschieben

Der Befehl schiebt je Lauf genau den einen Tag, den er veröffentlicht. Die Tags
der Runden davor stehen deshalb nur lokal, solange sie niemand nachgeschoben hat.
Was fehlt, sagt der Vergleich beider Seiten; gibt er nichts aus, ist der Handgriff
getan:

```sh
comm -23 <(git tag -l | sort) \
         <(git ls-remote --tags origin | sed 's|.*refs/tags/||' | sort)
```

**Jeder ausgegebene Name ist einzeln zu prüfen und einzeln zu schieben:**

```sh
git push origin refs/tags/v<zahl>
```

`git push origin --tags` steht hier bewusst nicht. Es schiebt jeden lokalen Tag
und damit auch einen verwaisten. Ein verwaister entsteht so: `cargo xtask
version` setzt den Tag, bevor gebaut wird; scheitert danach eine Station, bleiben
Eintrag und Tag stehen, und wählt der Nutzer für den nächsten Anlauf eine andere
Zahl, sieht dieser Lauf den alten Tag nicht mehr an. Er benennt dann einen lokal
eingetragenen, nie ausgelieferten Stand. **Ein Tag, zu dem keine Releaseseite
gehört, wird nicht nachgeschoben, sondern gelöscht** — `git tag -d v<zahl>`.

Dieselbe Sammelmarke `--tags` steht in `xtask/src/git.rs` in der Liste `MARKEN`,
die das Bauwerkzeug sich selbst untersagt: aus dem Werkzeug käme sie nie hinaus.
Von Hand ist sie so wenig am Platz.

## Versionspflege

Die Version steht an **einer** Stelle: im Feld `version` unter
`[workspace.package]` der `Cargo.toml`. Jedes Mitglied erbt sie über
`version.workspace = true`.

`resources/Info.plist` trägt bei `CFBundleShortVersionString` nur den Platzhalter
`__KRK_VERSION__`. `cargo xtask bundle` ersetzt ihn beim Kopieren durch die
geerbte Version; die Quelldatei bleibt unangetastet. Findet der Bau den
Platzhalter nicht, bricht er ab und baut kein Bündel — so kann weder eine
veraltete Zahl noch ein versionsloses Bündel unbemerkt entstehen. Nachprüfen:

```sh
plutil -extract CFBundleShortVersionString raw target/KRK.app/Contents/Info.plist
```

`CFBundleVersion` in der `Info.plist` ist etwas anderes: die Baunummer. Sie steht
nirgends ein zweites Mal und wird von Hand gepflegt.

**Die Zahl, die KRK anzeigt, ist an jedem Bau dieselbe.** Sie stammt immer aus
der `Cargo.toml`, gleich ob der Bau aus einem getaggten Stand kommt oder nicht.
Die Deckung durch einen Tag hängt deshalb an der Auslieferung und nicht an jedem
Bau: ein Entwicklungsbündel darf eine Zahl zeigen, ohne dass der Tag existiert,
ein ausgeliefertes nicht.

### Versionsstufen

Wann welche der drei Zahlen steigt, misst sich an KRKs eigenen Flächen und nicht
an einer Programmierschnittstelle: KRK ist eine Anwendung und keine Bibliothek,
und die Stelle des Vertrags nehmen die Flächen ein, die der Nutzer sieht und
speichert.

- **Major** steigt, wenn KRK etwas hergibt, worauf sich der Nutzer eingerichtet
  hat: die Bedeutung eines Tastenbefehls ändert sich oder eine Kombination fällt
  weg, eine Datei unter `~/Library/Application Support/KRK/` wird nicht mehr
  gelesen, wie sie geschrieben wurde, das Mindest-Zielsystem steigt, oder ein
  Befehl des Bauwerkzeugs verschwindet oder bedeutet etwas anderes.
- **Minor** steigt bei jeder neuen Fähigkeit, also bei jeder Runde, die eine
  bringt. Ein neuer Befehl in der Tastenbelegung und ein neuer Unterbefehl des
  Bauwerkzeugs zählen dazu.
- **Patch** steigt bei Behebungen ohne neue Fähigkeit.

**Jede Auslieferung bekommt einen Tag `v<version>`, und den setzt das Werkzeug**
(`shared/decisions/260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`).
Der Tag bleibt ein bewusster Akt, nur liegt der Vorsatz im Argument: wer
`./release.sh 0.2.0` tippt, hat die Zahl gewählt, und der Tag folgt daraus
mechanisch. Verschoben wird nie einer — ein vergebener Name hält den Lauf an.

Station 1 von `cargo xtask release` prüft, dass HEAD einen Tag mit genau diesem
Namen trägt und dass keine verfolgte Datei geändert ist; vorgemerkte und nicht
vorgemerkte Änderungen zählen gleich, gelöschte verfolgte Dateien zählen mit.
**Unbeachtete Dateien prüft sie nicht** — ein Bauergebnis, eine Notiz oder ein
Messbericht, der nie eingetragen wurde, hält die Auslieferung nicht auf. Und die
Prüfung hängt allein an `release`: `cargo xtask bundle` baut jederzeit ohne Tag,
ebenso jedes Ziel des `Makefile`, das an `bundle` hängt, und `make check` bleibt
unberührt.
