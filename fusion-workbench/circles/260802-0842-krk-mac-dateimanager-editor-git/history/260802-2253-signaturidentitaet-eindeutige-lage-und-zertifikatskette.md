# Signaturidentität: die eindeutige Lage, und die abgelaufene Kette im README

**Datum:** 260802-2253
**Agent:** coder
**Status:** Complete (abgeschlossen am 260803-1042 durch den orchestrator; die Prüfläufe wurden wiederholt und der Defekt geschlossen)
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-2050_c_signaturidentitaet-wird-nur-unter-einem-festen-namen-gefunden.md`
**Geänderte Dateien:** `xtask/src/sign.rs`, `README.md`
**Nicht angefasst:** `crates/`, `resources/`, `spikes/`, `xtask/src/main.rs`, `xtask/src/bundle.rs`, `Cargo.toml`

## Punkt 1: die dritte Stufe

`sign.rs` bestimmt die Identität jetzt in drei Stufen statt in zwei:

```
1. KRK_SIGN_IDENTITY            ausdrückliche Angabe, schlägt alles
2. Name "KRK Entwicklung"       ohne -v gesucht
3. genau eine gültige           mit -v gesucht
   sonst: Abbruch mit Anleitung
```

Stufe 3 nimmt die Identität nur, wenn `security find-identity -v -p codesigning`
genau einen Eintrag führt. Bei null oder mehr als einem bricht der Bau ab, denn
erst dort wäre die Wahl geraten.

## Zur `-v`-Frage: der Defekt hat recht, und ich habe es nachgemessen

Der Defekt kehrt eine Entwurfsentscheidung vom 260802-1927 um. Damals suchte
`sign.rs` bewusst ohne `-v`, weil eine selbstsignierte Identität ohne
Vertrauenseintrag von `-v` nicht gefunden wird. Das bleibt für Stufe 2 richtig,
und für Stufe 3 ist `-v` richtig. Der Unterschied liegt darin, wer wählt.

Stufe 2 prüft einen **genannten** Namen. Wer `KRK Entwicklung` angelegt hat, hat
sich für diese Identität entschieden, und der Bau hat sie nicht auszusortieren.
Stufe 3 **wählt** dagegen aus einer Menge aus, ohne dass jemand die Wahl
getroffen hätte. Automatisch gewählt werden darf dort nur, was auch signieren
kann. Ohne `-v` griffe die Stufe nach einem abgelaufenen Zertifikat oder einem,
dessen Kette sich nicht aufbaut, und der Bau scheiterte danach an
`errSecInternalComponent`: genau der Fehler, der diesen Defekt ausgelöst hat.
Ein Abbruch mit Anleitung ist besser als eine automatische Wahl, die hinterher
unverständlich scheitert.

Die Filterwirkung von `-v` ist nicht übernommen, sondern am 260802-2253 selbst
gemessen, und zwar **ohne den Anmeldeschlüsselbund des Nutzers anzufassen**: in
einem eigens im Scratchpad angelegten Schlüsselbund, der nie in die Suchliste
kam, zwei selbstsignierte Code-Signing-Identitäten importiert und
`find-identity` mit dem Schlüsselbund als Argument gerufen.

| Kommando | Ergebnis |
|---|---|
| `security find-identity -p codesigning <probe>` | 2 Einträge, beide `(CSSMERR_TP_NOT_TRUSTED)` |
| `security find-identity -v -p codesigning <probe>` | `0 valid identities found` |

Danach `security delete-keychain` und das Schlüsselmaterial gelöscht;
`security list-keychains -d user` und `security find-identity -p codesigning`
stehen nachweislich wieder auf dem Anfangsstand.

Nebenbei fiel dabei ein Unterschied auf, der für das Zählen zählt: **ohne `-v`
gibt `find-identity` zwei Abschnitte aus** (`Matching identities` und
`Valid identities only`), **mit `-v` nur einen**. Ein Zählen über die Ausgabe
ohne `-v` zählte jede Identität doppelt. Stufe 3 ruft deshalb eine eigene
Abfrage und liest nicht die Liste aus Stufe 2 weiter.

## Was am Abbruch sonst nötig war

Die alte Meldung lautete immer "Keine Signaturidentitaet gefunden". Bei mehr als
einer gültigen Identität wäre sie falsch, und eine falsche Meldung ist genau der
Defekt, den die dritte Stufe behebt. `anleitung` hat deshalb zwei Köpfe:

- ohne gültige Identität: "Keine gueltige Signaturidentitaet gefunden", plus ein
  Hinweis auf den neuen README-Abschnitt zur abgelaufenen Zertifikatskette;
- bei mehreren: "Mehrere gueltige Signaturidentitaeten gefunden [...] die Wahl
  waere nicht eindeutig", plus die Aufzählung der gefundenen Namen.

Der gemeinsame Rumpf (die Begründung gegen die Ad-hoc-Signatur und die zwei
Wege) bleibt unverändert.

**Gezählt werden Einträge, nicht verschiedene Namen.** Zwei Zertifikate können
denselben Namen tragen, und dann ist die Wahl gerade nicht eindeutig:
`codesign --sign` lehnt einen mehrdeutigen Namen ab. Ein Zusammenfassen nach
Namen versteckte die Mehrdeutigkeit, statt sie zu melden.

## Punkt 2: der README-Abschnitt

Neu: `### Abgelaufene Zertifikatskette (errSecInternalComponent)`, eingefügt
zwischen "Entwicklungsidentität anlegen" und "Prüfen, was signiert wurde".

Die Überschrift trägt die Fehlerkennung, und der Abschnitt beginnt mit dem
wörtlichen Meldungstext samt der Zeile `unable to build chain to self-signed
root`. Wer die Meldung sieht und im README danach sucht, landet dort. Das ist
der Zweck: die Meldung nennt das Zwischenzertifikat mit keinem Wort und deutet
auf die eigene Identität, die in Ordnung ist.

Der Abschnitt nennt das zusätzliche Symptom (`-p codesigning` zeigt die
Identität, `-v -p codesigning` meldet null), die Ursache (das
Apple-Zwischenzertifikat liegt im System-Schlüsselbund nur in der am 2023-02-07
abgelaufenen Fassung, während das Nutzerzertifikat von der G3-Instanz stammt),
die zwei am 260802-2045 ausgeführten Kommandos und den Nachtrag, dass das alte
Zwischenzertifikat nicht entfernt werden muss.

Der Abschnitt "Signierung" nennt jetzt drei Stufen statt zwei und erklärt, warum
Stufe 2 ohne und Stufe 3 mit `-v` fragt.

## Abnahme

Alle Kommandos am 260802 auf dem Referenzgerät ausgeführt.

| Prüfung | Ergebnis |
|---|---|
| `cargo build --workspace` | Rückgabewert 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | Rückgabewert 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | Rückgabewert 0 |
| `cargo test --workspace` | 80 Tests, 0 Fehler (23 davon in `xtask`, 7 neu) |
| `cargo clippy --workspace --all-targets` | keine Warnung, Rückgabewert 0 |
| `cargo fmt --all --check` | Rückgabewert 0 |
| `cargo xtask bundle` ohne Umgebungsvariable | (siehe unten) |

### Die drei Fälle der neuen Stufe, ehrlich getrennt

**Genau eine gültige Identität — der Normalfall.** Auf dem Gerät liegt genau
eine (`Apple Development: Kai Stalmann (FJ8U4B3QAC)`, in beiden Abfragen
sichtbar). Der Lauf von `cargo xtask bundle` ohne Umgebungsvariable blieb an
einem Schlüsselbund-Dialog von macOS hängen: `codesign` wartet auf die Freigabe
des Zugriffs auf den privaten Schlüssel. `SecurityAgent` lief nachweislich, der
`codesign`-Prozess stand. Das ist die Rückfrage, die das README unter
"Entwicklungsidentität anlegen" beschreibt, und sie braucht einen Klick auf
"Immer erlauben". Ergebnis nachgetragen, sobald der Nutzer sie beantwortet hat.

**Null gültige Identitäten.** Nicht end-to-end geprüft. Der Fall ließe sich auf
diesem Gerät nur herstellen, indem die vorhandene Identität aus dem
Anmeldeschlüsselbund verschwindet oder die Suchliste umgestellt wird, und beides
ist ausdrücklich ausgeschlossen. Geprüft ist die Ebene darunter: `gueltige_namen`
liefert gegen die real gemessene Ausgabe `0 valid identities found` eine leere
Liste, und `anleitung(&[])` erzeugt den Kopf ohne Identität samt Verweis auf den
Zertifikatskettenabschnitt. Der Pfad vom leeren `Vec` zum Abbruch ist eine
`if let [einzige]`-Zeile. **Ungeprüft bleibt ungeprüft**, und das ist hiermit
gesagt statt behauptet.

**Mehr als eine gültige Identität.** Ebenfalls nicht end-to-end geprüft, aus
demselben Grund: eine zweite gültige Apple-Identität ließe sich nicht ohne
Eingriff in den Schlüsselbund des Nutzers herstellen. Nachgeprüft ist die
Zeilenform, gegen die geparst wird: die Eintragszeile aus der echten Ausgabe des
Geräts, und zusätzlich die Form mit dem Zusatz `(CSSMERR_TP_NOT_TRUSTED)` aus
dem Probe-Schlüsselbund. Die Tests decken das Lesen zweier Einträge, das
Aussortieren der Zählzeile und der Überschriften und beide Abbruchköpfe ab. Die
zweizeilige Testvorlage `GUELTIGE_ZWEI` ist aus der echten Einzeilenform
fortgezählt, und ihr Kommentar sagt das.

## Ein Punkt, der außerhalb der Grenzen liegt

`xtask/src/main.rs` trägt in der Konstante `HILFE` die alte Beschreibung der
Suche: "Die Signaturidentitaet kommt aus der Umgebungsvariablen
KRK_SIGN_IDENTITY. Fehlt sie, wird im Schluesselbund die lokale Identitaet
\"KRK Entwicklung\" gesucht. Fehlt auch die, bricht der Bau [...] ab." Die
dritte Stufe fehlt dort. `main.rs` liegt außerhalb der für diese Aufgabe
gesetzten Grenzen (`sign.rs` und `README.md`), deshalb unverändert und hier
festgehalten statt nebenbei mitgeändert.

Ebenfalls außerhalb: `CLAUDE.md` beschreibt unter "Projektstand" den Stand vom
260802-1130 und behauptet, es gebe keinen Quellcode, kein Build- und kein
Testkommando. Inzwischen gibt es einen Rust-Workspace mit vier Mitgliedern und
80 Tests.

## Nicht gemacht

Kein Commit. Keine Markeränderung am Defekt. Keine Änderung am
Anmeldeschlüsselbund des Nutzers und keine an seiner Suchliste. Kein Eingriff in
`crates/`, `resources/`, `spikes/`. Keine Effort-Schätzung.
