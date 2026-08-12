# Die doppelt zählende Identitätssuche, `make release` und die Beglaubigungsprüfung

**Datum:** 2026-08-13 00:15
**Status:** Complete
**Auslöser:** Nutzerauftrag, drei Teile. Teil 1 mit dem Defektdatensatz
`shared/issues/260812-2357_*_die-identitaetssuche-zaehlt-jede-identitaet-doppelt-und-bricht-den-release-lauf-immer-ab.md`.
**Schreibziel dieses Protokolls:** `shared/history/`, weil kein Circle aktiv ist — die Runde 6 ist
geschlossen, das hier ist Nachlaufarbeit.

## 1. Die Identitätssuche zählt doppelt

**Ursache, wie im Datensatz beschrieben und am Gerät nachgesehen.**
`security find-identity -p codesigning` gibt zwei Abschnitte aus, `Matching identities` und darunter
`Valid identities only`. Eine gültige Identität steht in beiden. `gueltige_namen` liest über die
ganze Ausgabe und zählt sie deshalb zweimal; `bestimmen_fuer_release` trifft damit immer den Zweig
`mehrere` und bricht ab. Am 260813 auf dem Referenzgerät nachgesehen: zwei Identitäten im
Schlüsselbund, vier Einträge in der Ausgabe.

**Die Behebung sitzt in `auflisten` und nicht bei den Aufrufern.** Die neue reine Funktion
`abschnitt_der_treffer` schneidet die Ausgabe auf den ersten Abschnitt zu; `auflisten` gibt nur noch
diesen zurück. Damit kann kein Aufrufer die Beschränkung vergessen, und es gibt sie genau einmal.

**Warum der erste Abschnitt und nicht `-v`.** Der Kommentar an `bestimmen_fuer_release` begründet,
warum ohne `-v` gesucht wird: wer eine Developer-ID angelegt hat, hat sich für sie entschieden, und
die Suche hat sie nicht an der Vertrauensbewertung auszusortieren. Der Abschnitt `Matching
identities` führt auch die ungültigen, also bleibt diese Absicht erhalten. `-v` hätte den Defekt
ebenfalls behoben und dabei die Absicht geändert.

**Beide Suchen geprüft, wie verlangt.** `bestimmen` für den Entwicklungsbau liest dieselbe Ausgabe,
war aber nie betroffen: es fragt über `enthaelt_identitaet` nur, ob ein genannter Name vorkommt, und
ein doppeltes Vorkommen ändert daran nichts. Die dritte Stufe beider Suchen liest
`auflisten_gueltige` mit `-v`, und diese Ausgabe hat am Gerät nachgesehen nur eine Liste, weder eine
Zeile `Policy:` noch zwei Abschnitte. Die Beschränkung ändert also für `bestimmen` nichts und deckt
es trotzdem mit ab.

| Datei | Änderung |
|---|---|
| `xtask/src/sign.rs` | `auflisten` gibt nur noch den Abschnitt der Treffer zurück, mit Begründung im Kopf. |
| `xtask/src/sign.rs` | Neu: `abschnitt_der_treffer` samt den zwei Überschriften als Konstanten. |
| `xtask/src/sign.rs` | Der Kommentar an `bestimmen_fuer_release` verweist auf die Beschränkung. |
| `xtask/src/sign.rs` | Fünf neue Proben und zwei feste Ausgaben mit **beiden** Abschnitten. |

**Die Proben messen den Defekt, weil beide Abschnitte in der Eingabe stehen.** Eine davon hält die
Doppelzählung über die ganze Ausgabe ausdrücklich fest (vier Einträge bei zwei Identitäten) — nicht
als Wunschverhalten, sondern als Befund über `security`: schlägt sie eines Tages fehl, weil Apple
nur noch einen Abschnitt ausgibt, ist die Beschränkung entbehrlich geworden und nicht kaputt. Eine
zweite prüft, dass eine nur im ersten Abschnitt geführte, selbstsignierte Identität die
Beschränkung überlebt; sie ist die Probe auf die getroffene Wahl.

## 2. `make release`

Neues Ziel im Abschnitt `── Ausliefern ──`, gebaut wie die übrigen Ziele: eine Hülle um
`$(CARGO) xtask release`, kein zweites Bauwerkzeug.

`KRK_NOTARY_PROFILE` steht nicht fest im Rezept, sondern als `NOTARPROFIL := $(or
$(KRK_NOTARY_PROFILE),krk-notar)`. Damit gilt die Vorgabe des Geräts, ein bereits gesetztes
`KRK_NOTARY_PROFILE` schlägt sie, und `make release NOTARPROFIL=anderes` schlägt beides — dieselbe
`$(or …)`-Form, die die Datei bei `RUNDEN` und `ORDNER` schon verwendet.

**`KRK_SIGN_IDENTITY` ist nicht hineingekommen.** Es war die Umgehung des Defekts aus Teil 1; im
Makefile wäre daraus ein Dauerzustand geworden. Der Kommentar über der Variablen sagt das und nennt
den Datensatz.

## 3. `make signatur` erweitert

Das Ziel prüft jetzt auch die Beglaubigung: `xcrun stapler validate` und `spctl -a -vvv -t exec`.

**Der Zielkonflikt und wie er aufgelöst ist.** Ein mit `make bundle` gebautes Bündel trägt eine
Entwicklungsidentität und muss bei `spctl` durchfallen; das ist der richtige Befund. Beide neuen
Kommandos tragen deshalb `|| true` und lassen das Ziel nicht scheitern — auch `frisch` ruft
`signatur` unmittelbar nach `bundle`. Die erste Prüfung, `codesign --verify --deep --strict`, bleibt
hart: sie gilt für jedes Bündel gleich.

Damit der Leser die zwei Fälle auseinanderhält, steht vor den beiden Kommandos eine Legende:

```
Beglaubigung — zwei erwartbare Befunde, beide richtig:
  rejected / origin=Apple Development       aus 'make bundle', nur lokal
  accepted / source=Notarized Developer ID  aus 'make release', ausliefbar
```

Die beiden Zeilen sind die am 260813 am selben Bündel gemessenen Befunde und keine Vermutung.
`|| true` steht sichtbar im Rezept statt hinter einem `@`, damit am Protokoll ablesbar bleibt, dass
der Exit-Code absichtlich fallen gelassen wird.

## Prüfung

- [x] `cargo build --workspace` — Exit 0
- [x] `cargo fmt --all --check` — Exit 0
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — Exit 0
- [x] `cargo test --workspace` — Exit 0. Binärziel `krk` 478 Proben, unverändert; `xtask` 46 statt
      41, also die fünf neuen.
- [x] `make help` — Exit 0, zeigt `release` und `signatur` mit ihren Beschreibungen.
- [x] `make signatur` am beglaubigten Bündel unter `target/KRK.app` — Exit 0, `accepted /
      source=Notarized Developer ID`.
- [x] **Gegenprobe:** `cargo xtask release` **ohne** `KRK_SIGN_IDENTITY` kommt an der Signaturwahl
      vorbei. Der Lauf ging durch AppKit-Grenzprüfung, beide Übersetzungsziele, `lipo` und die
      Montage und wurde nach zwölf Sekunden beim Signieren abgebrochen; kein Abbruch an der
      Mehrdeutigkeit.

**Das beglaubigte Bündel steht unverändert.** Der Bauzwischenstand war warm, deshalb hat die
Gegenprobe `target/KRK.app` binnen Sekunden neu montiert. Es wurde vorher mit `ditto` gesichert und
danach zurückgespielt; `stapler validate` und `spctl` melden am zurückgespielten Bündel wieder
`accepted / source=Notarized Developer ID`, und die Prüfsumme von `Contents/CodeResources` stimmt
mit der Sicherung überein. Ein neuer Bündelbau hat nicht stattgefunden.

## Weitere gefundene Defekte oder Fragen

Keine. Eine Beobachtung ohne Handlungsbedarf: `sign::bestimmen` fände auf diesem Gerät heute keine
Identität — es gibt keine Identität namens `KRK Entwicklung`, und die dritte Stufe sieht zwei
gültige und bricht mit „Mehrere gueltige Signaturidentitaeten" ab. Das ist das zugesagte Verhalten
bei mehrdeutiger Lage und kein Defekt; `make bundle` braucht auf diesem Gerät also weiter eine
ausdrücklich gewählte Identität.
