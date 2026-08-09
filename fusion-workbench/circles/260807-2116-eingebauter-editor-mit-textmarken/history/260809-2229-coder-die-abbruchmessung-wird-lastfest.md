# Die Abbruchmessung wird lastfest

**Agent:** coder
**Datum:** 260809-2229
**Status:** Complete
**Umfang:** `crates/krk-core/tests/operation.rs` (nur Testcode, kein Programmcode)

## Auftrag

`der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` brach beim
Nutzer den Lauf von `make frisch` ab: `der Abbruch kam nach 183.680912ms zurueck,
erlaubt sind 100 ms`. Der Test sollte lastfest werden, ohne die Zusage aus C8 zu
dehnen. Programmcode durfte nicht angefasst werden.

## Die gemessene Ursache

Der Abbruch wird nicht dort bemerkt, wo er gesetzt wird. `copyfile(3)` ruft seinen
Statusrueckruf am Ende jedes uebertragenen Blocks; erst dort sieht der
Arbeitsfaden das Kennzeichen und gibt `COPYFILE_QUIT` zurueck. Die gemessene
Spanne ist der Rest des laufenden Blocks plus KRKs eigener Anteil.

Gemessen mit einer Wegwerf-Instrumentierung direkt an
`krk_core::verzeichnis::sys::datei_kopieren`, die jeden Statusrueckruf mit einem
Zeitstempel versah:

| Groesse                             | ohne Last | unter Last |
|-------------------------------------|-----------|------------|
| Abstand zweier Statusrueckrufe, p50 |  0,76 ms  |    15,3 ms |
| Abstand zweier Statusrueckrufe, max |  1,15 ms  |     153 ms |
| Ruecklauf nach dem letzten Rueckruf |   1,4 ms  |     2,3 ms |
| `fs::remove_file` des Rests         |  0,01 ms  |    0,02 ms |

KRKs eigener Anteil bleibt unter Last bei gut 2 ms und ist damit weit von der
Frist entfernt. Was sich dehnt, ist der Block, den die Platte gerade schreibt.
Ein einzelner Versuch auf einer belasteten Maschine misst die Platte, nicht die
Anwendung. Genau diese Lage stellt `make frisch` her: es raeumt vorher alles weg
und uebersetzt neu, die Maschine ist beim Testlauf am staerksten belastet.

**Es ist keine Regression.** Der Abbruchweg ist in dieser Sitzung nicht angefasst
worden, und das Aufraeumen des Restes am Ziel kostet 20 Mikrosekunden.

## Der verworfene Weg: Ruhe vor der Messung

Der Verdacht, das Schreiben der 500 MB in `volle_datei` belege die Platte und die
Messung unmittelbar danach messe den Nachlauf mit, ist geprueft und **widerlegt**.

Ein erster Vergleich sah gut aus: `sync` plus 500 ms Pause hielt in 3 von 3
Faellen bei 5 bis 9 ms. Dieser Vergleich taugte aber nichts, weil der Weg mit
Pause immer nach dem Weg ohne Pause lief und dessen Vorarbeit erbte.

In acht **verschraenkten** Runden, in denen beide Wege abwechselnd zuerst liefen
und damit dieselben Lastphasen sahen, verschwand der Unterschied:

| Weg                      | bester   | schlechtester | ueber 100 ms |
|--------------------------|----------|---------------|--------------|
| sofort nach dem Schreiben| 0,64 ms  |      177,8 ms |      1 von 8 |
| nach `sync` + 500 ms     | 0,89 ms  |      151,1 ms |      1 von 8 |

Der Nachlauf des eigenen Schreibens ist nicht die Ursache; die Fremdlast ist es.
`volle_datei` ruft ohnehin `sync_all()`, die 500 MB liegen also bereits auf der
Platte, bevor die Messung beginnt.

Auch das `ZEITMESSUNG`-Schloss weiter zu spannen traegt nicht: es serialisiert
die vier Zeitmessungen der Datei gegeneinander, aber die Last, an der der Test
scheitert, kommt aus dem Uebersetzen vor dem Testlauf und von ausserhalb des
Prozesses. Kein Schloss innerhalb der Testdatei erreicht sie.

## Der gewaehlte Weg: bester von fuenf Versuchen

Die 100 ms bleiben als Zahl unangetastet. Weich wird allein die Messung.

Der Test faehrt bis zu fuenf Abbruchversuche und ist zufrieden, sobald einer die
Zusage haelt. Die 500-MB-Datei entsteht dabei **einmal** und wird von allen
Versuchen nur gelesen; ein Versuch kostet die 40 ms Vorlauf plus den Abbruch.
Der Zusatzaufwand ist damit klein gegen das Schreiben der Datei, das ohne Last
0,32 s und unter Last 1,7 s braucht.

**Warum fuenf:** unter kuenstlicher Platten- und Rechenlast ueberschritt ein
einzelner Versuch die Frist in 1 von 8 bis 2 von 7 Faellen, in der schlechtesten
Reihe also in knapp 30 Prozent. Fuenf Versuche lassen davon 0,3^5, etwa zwei von
tausend Laeufen.

**Was nicht gemittelt wird.** Vier Zusagen haengen nicht an der Last, sondern am
Verhalten des Kerns, und werden deshalb in **jedem einzelnen** Versuch geprueft:
der Abschluss ist `Abgebrochen`, die gemeldeten Bytes liegen unter der
Dateigroesse, kein Eintrag gilt als uebertragen, und der Rest am Ziel ist
weggeraeumt. Nur die Frist selbst und die Bedingung "der Abbruch lag mitten in
der Datei" duerfen einen Versuch verwerfen.

Die zweite Bedingung ist neu und schliesst eine zweite Lastempfindlichkeit
derselben Ursache: wird unter Last in den 40 ms Vorlauf kein einziger Block
fertig, sind null Bytes geflossen, und die gemessene Spanne waere die eines
Abbruchs **vor** der Uebertragung. So ein Versuch zaehlt nicht als Beleg, statt
als Fehlschlag zu gelten. Haelt kein Versuch, nennt die Fehlermeldung alle fuenf
mit Spanne und Bytezahl.

## Beleg

Kuenstliche Last: acht `dd`-Prozesse, die je 1,2 GB auf `$TMPDIR` schreiben und
`sync` rufen, dazu eine Rechenschleife je Kern. Alle Lastprozesse sind nach der
Messung wieder abgeraeumt.

**Vorher**, ganze Testdatei, 5 Laeufe unter Last: **1 Fehlschlag**
(`der Abbruch kam nach 120.984675ms zurueck`), Laufzeiten 15,5 bis 17,5 s.

**Einzelversuchsquote** unter der schweren Last, 20 Versuche in Folge: 1 von 20
ueber der Frist, der schlechteste bei **269,7 ms**. Die Last biss also weiterhin.

**Nachher**, ganze Testdatei, 18 Laeufe unter Last: **0 Fehlschlaege**.
Acht Laeufe bei Lastmittel 20, zehn Laeufe bei Lastmittel 98 bis 123 mit
Laufzeiten von 21 bis 46 s, also dem Vier- bis Neunfachen der 5,16 s, bei denen
der Test beim Nutzer gescheitert war.

`make check` laeuft durch: alle vier Abnahmekommandos gruen.

## Was offen bleibt

Die drei uebrigen Zeitmessungen der Datei messen ebenfalls mit einem einzigen
Versuch: das Verschieben der 200-MB-Datei unter 50 ms, der Klon der 500 MB unter
100 ms und der Abbruch im Stapel unter 100 ms. Sie haben in 23 Laeufen unter
derselben Last kein einziges Mal ausgeschlagen und sind deshalb unberuehrt
geblieben. Teilen sie die Ursache, taeten sie es schwaecher: das Verschieben und
der Klon ruehren keine Bytes an, und der Stapelabbruch greift zwischen zwei
Umbenennungen statt mitten in einem Plattenblock.
