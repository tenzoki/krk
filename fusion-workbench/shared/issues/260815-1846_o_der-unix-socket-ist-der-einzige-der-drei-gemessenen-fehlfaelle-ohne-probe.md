Der Unix-Socket ist der einzige der drei gemessenen Fehlfälle, für den keine Probe steht

---

Der Befund `260815-1713` hat drei Zustände am Referenzgerät gemessen, die der alte
Deskriptorweg falsch einordnete: einen Unix-Socket, eine Datei mit Modus `000` und ein
Verzeichnis mit Modus `0111`. `7fae5ba` behebt alle drei und legt für zwei davon eine Probe
an (`crates/krk-core/tests/verzeichnis.rs:1868-1897` und `:1899-1915`). Für den Socket steht
keine.

Damit fehlt gerade die Probe, die einen Rückfall auf den Deskriptorweg unter **jeder**
Kennung fangen würde.

---

**Gefunden am:** 260815-1844, Stand `60a8ca5`
**Gefunden von:** coderev, Durchsicht des Bereichs `e37a1e3..60a8ca5`
**Schwere:** mittel. Das Verhalten ist richtig, nachgemessen. Ungedeckt ist die Rückkehr des
behobenen Befunds, und zwar an dem Zustand, an dem sie am sichersten auffiele.
**Betroffen:** `crates/krk-core/tests/verzeichnis.rs:1836-1998`,
`crates/krk-core/tests/gemeinsam/mod.rs:118-146`
**Domain:** code

## Warum gerade der Socket die tragfähigste Probe wäre

Die beiden neuen Proben hängen an entzogenen Rechten, und ihr eigener Doc-Kommentar hält
fest, was das unter `root` bedeutet (`:1878-1884`): die Behauptung bleibt richtig, aber die
Probe verliert ihre Fähigkeit, einen Rückfall zu fangen, „dort duerfte `root` die Datei
oeffnen". Das ist zutreffend und am Gerät nachvollzogen.

Der Socket hat diese Schwäche nicht. `open(O_RDONLY|O_NONBLOCK)` auf einen Unix-Socket
scheitert mit `EOPNOTSUPP`, und dieser Fehlschlag hängt an der Art des Eintrags und nicht an
Rechten. Nachgemessen, uid 502:

| Ziel | alter Weg, `open` mit `O_NONBLOCK` | neuer Weg, `stat` |
|---|---|---|
| Unix-Socket | `errno 102` Operation not supported on socket | `KeinOrdner` |
| Datei Modus `000` | `errno 13` Permission denied | `KeinOrdner` |
| Verzeichnis Modus `0111` | `errno 13` Permission denied | `Ordner` |
| Röhre ohne Schreiber | ok, kein Verzeichnis | `KeinOrdner` |
| `/dev/null` | ok, kein Verzeichnis | `KeinOrdner` |

`errno 102` fällt für `root` genauso an wie für uid 502. Eine Socket-Probe wäre damit die
einzige der drei, die unter jeder Kennung misst, was sie zu messen vorgibt.

## Was der Prüfordner dafür braucht

`Pruefordner` (`crates/krk-core/tests/gemeinsam/mod.rs`) kennt `roehre` über `/usr/bin/mkfifo`,
aber keinen Socket. Anders als bei der Röhre braucht es dafür keinen Fremdaufruf und keine
fünfte Bindung in `verzeichnis::sys`: `std::os::unix::net::UnixListener::bind` steht in der
Standardbibliothek, und der gebundene Eintrag bleibt im Dateisystem stehen, auch wenn der
Horcher fällt.

**Eine Falle gehört dazu**, weil sie beim Nachmessen dieser Durchsicht zugeschlagen hat:
`AF_UNIX` fasst auf macOS 104 Bytes Pfad. Der Prüfordner liegt unter
`/var/folders/…/T` (48 Zeichen), und mit dem Ordnernamen aus Zweck, Prozesskennung und
Laufnummer landet ein Socket namens `sock` bei 95 Zeichen. Das geht, aber der Spielraum
beträgt neun Zeichen; ein längerer Zweckname sprengt die Grenze, und der Fehlschlag heißt
dann nicht „der Socket ist falsch eingeordnet", sondern „AF_UNIX path too long".
`abraeumen` räumt den Eintrag ohne Zutun ab, `remove_dir_all` kommt an einem Socket vorbei.

## Vorschlag

`Pruefordner::socket(&self, name: &str) -> PathBuf` über `UnixListener::bind`, mit dem
Zwecknamen kurz gehalten, und eine Probe
`eine_verknuepfung_auf_einen_socket_ist_kein_ordner` neben den beiden neuen. Ihr
Doc-Kommentar hält den Unterschied fest, um dessentwillen sie dasteht: sie misst unter jeder
Kennung, die beiden anderen nur unterhalb von `root`.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und die Directive keiner Runde.
