# Durchsicht: das Verweisziel fragt am Namen, nachgemessen

**Sender:** coderev
**Reviewed-range:** `e37a1e3..60a8ca5`
**Not-opened:** keine

## Zusammenfassung

`7fae5ba` hält, was es behauptet. Die drei am Referenzgerät gemessenen Fehlfälle des Befunds
`260815-1713` sind weg, selbst nachgemessen und nicht dem Bericht geglaubt: der Unix-Socket
und die Datei mit Modus `000` kommen als `KeinOrdner`, das Verzeichnis mit Modus `0111` als
`Ordner`, die Röhre ohne Schreiber bleibt unverändert richtig und hält die Frage nicht an.
Ein vierter Fehlfall dieser Art ist nicht zu finden. Zwei Befunde bleiben, beide niedrig bis
mittel und beide an der Beschreibung oder der Deckung, nicht am Verhalten: der erläuternde
Satz zu `Unerreichbar` zählt drei Gründe auf, wo `stat(2)` an mehr scheitert, und der Socket
ist der einzige der drei behobenen Fälle ohne Probe. Nichts davon hält eine Auslieferung auf.

## Zahlen

| Schwere | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 1 |
| Niedrig | 1 |

Dazu ein Nachtrag an einem bestehenden Datensatz statt eines dritten neuen.

## Was gemessen wurde und wie

`bestimmen` ist nach dem Wechsel ein sechszeiliger Rumpf über `std::fs::metadata` ohne
weitere Eingabe. Gemessen habe ich zweigleisig: einmal die abgelegten Proben am echten Code
(`cargo test -p krk-core --test verzeichnis`, 53 bestanden, 0 gescheitert, 2 als Kindproben
übersprungen, 0 herausgefiltert — also liefen auch die zwei neuen), und einmal eine
wortgleiche Nachbildung des Rumpfes gegen zwölf eigens angelegte Zustände, weil die Proben
den Socket und die Grenzfälle der Auflösung nicht abdecken. Der alte Weg,
`open(O_RDONLY|O_NONBLOCK)` mit `fstat`, ist an denselben Zuständen gegengemessen.

macOS 24.6.0, uid 502:

| Zustand | alter Weg | neuer Weg | Befund |
|---|---|---|---|
| Unix-Socket | `errno 102` Operation not supported on socket | `KeinOrdner` | behoben |
| Datei Modus `000` | `errno 13` Permission denied | `KeinOrdner` | behoben |
| Verzeichnis Modus `0111` | `errno 13` Permission denied | `Ordner` | behoben |
| Röhre ohne Schreiber | ok | `KeinOrdner`, ohne Halt | unverändert richtig |
| `/dev/null` | ok | `KeinOrdner` | unverändert richtig |
| Verknüpfung ins Leere | — | `Unerreichbar {os error 2}` | richtig |
| Ring aus zwei Verknüpfungen | — | `Unerreichbar {os error 62}` | richtig |
| Kette aus 40 Verknüpfungen ohne Ring | — | `Unerreichbar {os error 62}` | richtig, benannt aber falsch |
| Zwischenverzeichnis Modus `000`, Ziel Datei | — | `Unerreichbar {os error 13}` | richtig |
| Zwischenverzeichnis Modus `000`, Ziel Ordner | — | `Unerreichbar {os error 13}` | richtig |
| `datei.txt/unterpfad` | — | `Unerreichbar {os error 20}` | richtig |
| Namensteil mit 300 Zeichen | — | `Unerreichbar {os error 63}` | richtig, benannt aber falsch |

**Ein vierter Fehlfall der gesuchten Art ist nicht dabei.** Gesucht war ein Zustand, den
`stat(2)` falsch einordnet oder in dem es Wirkung am Ziel hätte. Beides tritt nicht ein: die
Typfrage beantwortet `stat` für jeden vorhandenen Eintrag, und angefasst wird kein Ziel. Die
beiden letzten Zeilen der Tabelle sind richtig eingeordnet und tragen nur den falschen Namen;
daraus wird der eine der zwei Datensätze.

## Befunde

### Die Zusicherung „drei Werte, überschneidungsfrei und vollständig" — gilt jetzt für beides

Ja, auf der Ebene, auf der sie steht. Der `Err`-Zweig ist ein Auffangzweig über jeden
Fehlschlag von `stat(2)`, und der Satz „hinter ihm steht nichts, was von hier aus erreichbar
wäre" trifft in allen zwölf gemessenen Zuständen zu. `KeinOrdner` und `Unerreichbar`
beschreiben nach dem Wechsel nicht mehr denselben Zustand: die Datei ohne Leserecht liegt
jetzt dort, wo ihr Doc-Kommentar sie hinstellt. Der Schnitt trägt.

**Der erläuternde Satz darunter trägt nicht** (`verweisziel.rs:129-133`). Er nennt drei
Gründe, „ins Leere, im Ring, oder eine Stufe des Pfades lässt sich nicht durchschreiten",
und liest sich als vollständige Liste. Der zu lange Name fällt unter keinen davon, und
`ELOOP` entsteht ab 32 aufgelösten Verknüpfungen ganz ohne Ring — beides gemessen. Schwere
niedrig, Datensatz
`shared/issues/260815-1845_o_der-doc-kommentar-von-unerreichbar-zaehlt-drei-gruende-auf-und-stat-scheitert-an-mehr.md`.

### Der neue Modulkopf — der Beleg über Editor und Vorschau stimmt

Gegengelesen: `crates/krk-core/src/text/datei.rs:414-424` öffnet über die Hülle, fragt
`metadata()` am selben `datei` und liest bei `:442-446` mit `datei.by_ref().take(…)` aus
genau diesem Deskriptor. `crates/krk-ui/src/vorschaumodell.rs:679-686` tut dasselbe in fünf
Zeilen. Beide kaufen also wirklich, was der Satz ihnen zuschreibt, und beide behalten den
Deskriptor. Ebenfalls gegengelesen und richtig: `tabelle::in_zeile_einsteigen` ruft
`ordner_lesen(&ziel, None)` mit dem **Pfad** (`crates/krk-ui/src/appkit/tabelle.rs:1447`),
das Fenster zwischen Prüfung und Benutzung besteht also fort; und
`kommandos::pfadeingabe::pruefen` fragt tatsächlich `std::fs::metadata`
(`crates/krk-ui/src/kommandos/pfadeingabe.rs:61`), samt der als gewollt beschriebenen
Zusatzprüfung über `read_dir` bei `:68-77`.

**Eine Behauptung im Kopf stimmt nach dem Wechsel nicht mehr:** „Dieses Modul benutzt seinen
Deskriptor nicht. Es gibt ihn am Ende der Funktion sofort wieder ab" (`:49-53`). Das Modul
hat keinen Deskriptor mehr. Der Absatz argumentiert gegen den alten Weg, steht aber im
Präsens. Zwei weitere Stellen derselben Art stehen in den Proben (`:1843-1845` „am
Deskriptor", `:1934-1937` „aus demselben `open(2)`"). Alle drei sind an
`shared/issues/260815-1752_*_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md`
angehängt statt als dritter Datensatz abgelegt; dessen Liste wächst damit von vier auf sieben
Stellen.

### Die zwei neuen Proben — die Begründung trägt, mit einer Einschränkung im Wortlaut

Beide laufen unter `root` unverändert durch, und beide bleiben dabei wahr: `stat(2)` liefert
für die Datei mit Modus `000` „kein Verzeichnis" und für das Verzeichnis mit Modus `0111`
„Verzeichnis", gleich unter welcher Kennung. Kein `#[ignore]` ist damit richtig, und die
Selbsteinschätzung im Doc-Kommentar — unter `root` verlöre die Probe allein ihre Fähigkeit,
einen Rückfall zu fangen — trifft zu. Auch die Entscheidung, nicht über das tatsächliche
Ergebnis zu schneiden, ist richtig begründet: ein solcher Schnitt bestätigte sich selbst.

Der Satz „`stat(2)` beantwortet die Typfrage für jede Kennung gleich" ist als allgemeiner
Satz eine Spur zu weit — das Suchrecht auf einem Zwischenverzeichnis hängt sehr wohl an der
Kennung, gemessen in Zeile neun und zehn der Tabelle oben. Für die beiden Proben stimmt er,
weil ihre Pfadstufen dem Prüfordner gehören. Kein Datensatz; die Schlussfolgerung, um die es
im Kommentar geht, bleibt richtig.

### Das Aufräumen der neuen Proben hält

`Pruefordner::drop` läuft über `abraeumen`, und dort fängt `entsperren_und_loeschen` genau
diesen Fall: `remove_dir_all` scheitert am Verzeichnis mit `0111`, die zweite Stufe setzt
`0755` zurück und steigt hinab (`crates/krk-core/tests/gemeinsam/mod.rs:170-198`). Die beiden
neuen Proben lassen also nichts im Temporärverzeichnis stehen. Nachgesehen, weil ein
liegengebliebener Ordner mit entzogenen Rechten die nächste Sitzung gekostet hätte.

### Die entfallene Hilfsfunktion und das geänderte `#[must_use]`

Beide Begründungen tragen. `unerreichbar` hatte genau einen Daseinsgrund, zwei Fehlschlagpfade
mit derselben Antwort; nach dem Wechsel gibt es einen einzigen `Err`-Zweig, und eine
Hilfsfunktion für eine Fundstelle wäre eine Stelle mehr zum Nachschlagen. Die neue
`#[must_use]`-Begründung ist die stärkere von beiden: die alte hing an der Nebenwirkung
(„öffnet sonst einen Deskriptor und schließt ihn wieder"), die neue an der Wirkungslosigkeit
(„tut nichts außer zu antworten"). Das ist genau die Regel aus `CLAUDE.md` — ein Rückgabewert,
dessen stilles Fallenlassen unbemerkt bliebe —, und sie greift beim reinen Frager
zwingender als beim Öffner.

### Der Socket hat keine Probe

Von den drei behobenen Fällen haben zwei eine Probe bekommen und einer nicht, und es ist der,
der als Rückfallwächter am meisten trüge: `EOPNOTSUPP` hängt an der Art des Eintrags und nicht
an Rechten, fällt also auch unter `root` an, wo die beiden neuen Proben ihre Wirkung verlieren.
Schwere mittel, Datensatz
`shared/issues/260815-1846_o_der-unix-socket-ist-der-einzige-der-drei-gemessenen-fehlfaelle-ohne-probe.md`,
mit dem konkreten Weg über `UnixListener::bind` und der Pfadlängenfalle von `AF_UNIX`.

## Nicht neu gesucht

Die zwei bereits aufgenommenen Befunde sind gegengelesen und bestehen unverändert: der tote
Verweis in `verweisziel.rs:95` (`260815-1812`) und die vier Beschreibungsstellen aus
`260815-1752`. Der zweite ist um drei Stellen ergänzt, siehe oben.

## Empfohlene Reihenfolge

Kein Befund dieser Durchsicht hält eine Auslieferung auf. Der Socket-Datensatz ist der
einzige, der sich mit der Zeit verteuert: er ist eine Probe, und Proben werden nachträglich
selten nachgereicht. Die vier Beschreibungsdatensätze — `260815-1752` mit jetzt sieben
Stellen, `260815-1812` mit einer — gehören sinnvoll in einen Zug, weil sie dieselbe Datei
betreffen und dieselbe Quelle nachziehen.
