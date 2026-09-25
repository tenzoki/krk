# Bündelbeschreibung Info.plist (Schritt 4)

**Datum:** 260802-1829
**Agent:** ontocoder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `## Implementierungsschritte`, Schritt 4
**Geänderte Dateien:** `resources/Info.plist` (neu), `resources/` (Verzeichnis neu angelegt)

## Was geschrieben wurde

Eine XML-Property-Liste mit fünfzehn Schlüsseln, in drei Blöcken mit je einem
erklärenden Kommentar.

```
Kennung        CFBundleIdentifier, CFBundleName, CFBundleExecutable,
               CFBundlePackageType, CFBundleShortVersionString, CFBundleVersion
System         LSMinimumSystemVersion, LSApplicationCategoryType,
               NSPrincipalClass, NSHighResolutionCapable
Rückfragen     NSDesktopFolderUsageDescription, NSDocumentsFolderUsageDescription,
               NSDownloadsFolderUsageDescription, NSRemovableVolumesUsageDescription,
               NSNetworkVolumesUsageDescription
```

Die zehn Schlüssel der ersten beiden Blöcke und die fünf Rückfragetexte sind genau
die, die Schritt 4 aufzählt. Nichts darüber hinaus: kein `CFBundleInfoDictionaryVersion`,
kein `CFBundleSignature`, kein `CFBundleIconFile`. Schritt 4 nennt sie nicht, und ein
Schlüssel, den kein Schritt verlangt, wäre erfunden. `CFBundleSignature` fehlt bewusst,
obwohl Schritt 5 eine `PkgInfo` schreibt; ohne den Schlüssel gilt der Vorgabewert `????`,
und `APPL????` ist genau der Inhalt, den eine `PkgInfo` üblicherweise trägt.

## Werte, die der Plan nicht vorgibt

Schritt 4 nennt vier Schlüssel ohne Wert. So sind sie belegt:

| Schlüssel | Wert | Woher |
|---|---|---|
| `CFBundleName` | `KRK` | Schritt 5 baut `target/KRK.app` |
| `CFBundleExecutable` | `krk` | `crates/krk-ui/Cargo.toml`, `[[bin]] name = "krk"`; Schritt 5 nimmt an `Contents/MacOS/krk` ab |
| `CFBundleShortVersionString` | `0.1.0` | `Cargo.toml`, `[workspace.package] version = "0.1.0"` |
| `CFBundleVersion` | `1` | Baunummer, beginnt bei 1 |

**Der Programmname ist `krk`, nicht `krk-ui`.** Das Cargo-Paket heißt `krk-ui`, sein
Binärziel heißt `krk`; geprüft an `crates/krk-ui/Cargo.toml` und am gebauten
`target/debug/krk`. Ein `CFBundleExecutable` von `krk-ui` hätte ein Bündel ergeben, das
`plutil -lint` besteht und beim Start nicht findet, was es ausführen soll. Der Wert
trägt deshalb einen Kommentar in der Datei.

**Die Version steht an zwei Stellen.** `0.1.0` steht in `Cargo.toml` und noch einmal
in der `Info.plist`. Schritt 5 kopiert die Datei unverändert, es gibt keine Ersetzung
beim Bauen, also lässt sich die Doppelung ohne eine Änderung an Schritt 5 nicht
auflösen. Der Kommentar an der Stelle sagt, dass beide Stellen gemeinsam zu ziehen
sind. Aufgelöst ist nichts, entschieden auch nicht.

## Zwei Randbedingungen aus dem Entscheidungsdatensatz

Nachgelesen in `decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md`,
Abschnitt `## Constraints`, nicht aus einer Zusammenfassung übernommen.

**Auslieferung außerhalb der App-Sandbox.** C9 verlangt Zugriff auf jeden lokalen
Pfad einschließlich `/Volumes`; in der Sandbox gibt es dafür keinen gangbaren Weg. Der
Zugriff läuft stattdessen über den Systemmechanismus für Transparenz, Zustimmung und
Kontrolle, der am signierten Bündel angreift. In dieser Datei schlägt sich das in den
fünf Rückfragetexten nieder und in dem, was **nicht** darin steht: die
Sandbox-Berechtigungen sind keine `Info.plist`-Schlüssel, sondern eine
Entitlements-Datei, und die entsteht in Schritt 5 nicht.

**Mindest-Zielsystem macOS 15.** `LSMinimumSystemVersion` steht auf `15.0` und trägt
denselben Wert wie `MACOSX_DEPLOYMENT_TARGET` in `.cargo/config.toml`, Zeile 5,
nachgeprüft. Die beiden Zahlen müssen zusammenbleiben: sie beschreiben dasselbe
Mindestsystem, einmal für den Übersetzer und einmal für den Starter des Systems.
Schritt 5 weist die eine Seite über `vtool -show-build-version` nach, Schritt 4 die
andere über `plutil -extract`.

## Warum die Rückfragetexte deutsch sind und den Nutzer duzen

**Der Plan sagt zur Sprache etwas, also war nichts zu entscheiden.** Schritt 4:
"Dazu die fünf TCC-Rückfragetexte **auf Deutsch**". `### Frage 7` desselben Plans:
"genau das sind diese fünf Texte, und sie sind deutsch." Das deckt sich mit
`CLAUDE.md`, wo `**Language:** de` steht und Prosa als deutsch festgelegt ist. Der
dortige Zusatz, dass maschinenlesbare Artefakte englischen Konventionen folgen, trifft
die Schlüsselnamen, nicht die Texte: die Schlüsselnamen sind von Apple vergeben und
englisch, die Texte sind Oberfläche.

**Die Anrede war offen, sie ist begründet gewählt.** Der Plan sagt dazu nichts. Gewählt
ist die Du-Form, aus einem Grund: macOS zeigt den Verwendungszweck-Text nicht allein,
sondern eingebettet in einen eigenen Dialogsatz. `inference:` Die deutschsprachige
Oberfläche von macOS 15 verwendet dort die Du-Form; ein gesiezter Zusatztext stünde
sichtbar neben einem geduzten Rahmen. Nachgemessen am laufenden System ist das nicht,
es ist aus der Systemversion geschlossen. Die zweite, schwächere Stütze: das
Chat-Stilprofil des Projekts (`fusion-workbench/stilwerk/chat-voice-de.yaml`, C03)
schreibt für Nutzeransprache ohnehin die Du-Form vor.

Jeder der fünf Texte ist ein Satz und nennt den Zweck, wie C4 es verlangt ("erklärt in
einem Satz, wozu"). Der Netzlaufwerk-Text nimmt die Abgrenzung aus C9 mit auf, dass
KRK keine eigene Serververbindung aufbaut, weil der Nutzer sonst aus dem Dialog eine
Fähigkeit herausliest, die KRK nicht hat.

## Abnahme

Die drei Kommandos des Abnahmekriteriums, am 260802-1829 auf dem Referenzgerät
ausgeführt, aus dem Projektwurzelverzeichnis.

```
$ plutil -lint resources/Info.plist
resources/Info.plist: OK
Rueckgabewert: 0
```

```
$ plutil -extract LSMinimumSystemVersion raw resources/Info.plist
15.0
Rueckgabewert: 0
```

Die fünf Rückfragetexte, jeder mit Rückgabewert 0:

| Schlüssel | Ausgabe von `plutil -extract … raw` |
|---|---|
| `NSDesktopFolderUsageDescription` | KRK zeigt deinen Schreibtisch als Ordner an, damit du die Dateien darin öffnen, kopieren, verschieben und umbenennen kannst. |
| `NSDocumentsFolderUsageDescription` | KRK zeigt deinen Dokumentenordner an, damit du die Dateien darin öffnen, kopieren, verschieben und umbenennen kannst. |
| `NSDownloadsFolderUsageDescription` | KRK zeigt deinen Download-Ordner an, damit du geladene Dateien von dort öffnen, kopieren, verschieben und umbenennen kannst. |
| `NSRemovableVolumesUsageDescription` | KRK zeigt angeschlossene Wechselmedien wie USB-Sticks und externe Festplatten an, damit du Dateien zwischen ihnen und deinem Mac kopieren und verschieben kannst. |
| `NSNetworkVolumesUsageDescription` | KRK zeigt bereits verbundene Netzlaufwerke wie jeden anderen Ordner an, damit du Dateien darin öffnen und bearbeiten kannst; eine eigene Verbindung zu einem Server baut KRK nicht auf. |

Über das Kriterium hinaus geprüft, weil Schritt 5 daran abnimmt: alle zehn übrigen
Schlüssel liefern über `plutil -extract … raw` ihren Wert mit Rückgabewert 0,
namentlich `CFBundleExecutable` den Wert `krk`.

## Kein Widerspruch gefunden

Schritt 4 und Schritt 5 wurden gegeneinander gelesen. Sie stimmen überein:

- Schritt 5 kopiert `resources/Info.plist`, genau der Pfad, den Schritt 4 schreibt.
- Schritt 5 nimmt `Contents/MacOS/krk` ab, `CFBundleExecutable` trägt `krk`.
- Schritt 5 prüft `minos 15.0` am Binärformat, `LSMinimumSystemVersion` trägt `15.0`.
- Schritt 5 baut `target/KRK.app`, `CFBundleName` trägt `KRK`.

Die vier Werte, die Schritt 4 offenlässt, sind keine Widersprüche, sondern Lücken; sie
sind oben ausgewiesen und aus vorhandenen Dateien abgeleitet, nicht erfunden.

## Nicht gemacht

Kein Bündelbau, kein `codesign`, keine `PkgInfo`, kein `xtask`-Unterbefehl. Das ist
Schritt 5. Keine Entitlements-Datei. Keine Auslieferungsbelegung
(`resources/default-keymap.toml`) — das ist Schritt 9, und der Nutzer will vorher
gefragt werden.

`crates/`, `xtask/`, `spikes/` und `Cargo.toml` sind unberührt; in `crates/` und
`xtask/` arbeitet parallel ein anderer Agent an Schritt 3. Am Plandokument nichts
geändert, auch der Schrittstatus nicht. `CLAUDE.md` unberührt, obwohl der Abschnitt
`## Technologiewahl` dort noch "Offen" führt und damit veraltet ist. Kein Commit,
keine Aufwandsschätzung.
