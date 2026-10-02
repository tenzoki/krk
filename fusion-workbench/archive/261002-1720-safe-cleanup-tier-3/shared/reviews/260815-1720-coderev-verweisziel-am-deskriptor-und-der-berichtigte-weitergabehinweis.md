# Durchsicht: das Verweisziel am Deskriptor und der berichtigte Weitergabehinweis

**Sender:** coderev
**Reviewed-range:** `a2670db..8c06747`
**Not-opened:** keine

## Zusammenfassung

Zwei Commits, fünf Codedateien. `a46fd1f` schließt seine beiden Befunde sachlich: die
Verzweigung ordnet kein Zertifikat mehr ein, beide Zweige nennen die fehlende gehärtete
Laufzeitumgebung, und die drei gestrichenen Wendungen sind wirklich weg. `8c06747` behebt
den gemeldeten Defekt und hält seine Leistungszusage, wählt für die Zielfrage aber `open(2)`,
wo nur `stat(2)` sie entscheiden kann; drei Dateisystemzustände werden dadurch falsch
eingeordnet, und die Zusicherung „überschneidungsfrei und vollständig" trägt auf der Ebene
der Werte nicht. Vier Datensätze abgelegt, keiner steht einer Auslieferung im Weg.

## Zahlen

| Schwere | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 2 |
| Niedrig | 1 |

Vier Datensätze, alle unter `shared/issues/`, alle `_o_`.

## Was gegengeprüft ist und hält

**Die beiden Befunde von `a46fd1f` sind geschlossen.** Der Auffangzweig (`sign.rs:196-198`)
sagt „mit {name}, und dieser Name ist nicht der einer Developer-ID" und behauptet damit
nichts über die Art. Für alle vier Fälle aus der Tabelle in `260815-1444` ist das richtig,
den SHA-1-Abdruck und die Teilzeichenfolge eingeschlossen: der Abdruck ist der Name keiner
Developer-ID, und die Teilzeichenfolge ist nicht der Name der Identität, die sie auswählt.
Die drei Wendungen „und damit richtig", „bleibt auf dieser Maschine" und „auf jedem anderen
Mac" finden sich mit `grep` im ganzen `xtask/` nicht mehr, und drei Proben halten ihre
Abwesenheit fest. Die fehlende gehärtete Laufzeitumgebung steht im **gemeinsamen** Teil
(`sign.rs:200-203`), also in beiden Zweigen; der Schnitt ist der, den die Commit-Botschaft
beschreibt, und die Verzweigung trägt nur noch die Aussage über den Namen.

**`bundle` signiert wirklich ohne gehärtete Laufzeitumgebung**, also stimmt die neue Aussage:
`signieren` (`sign.rs:212`) ruft `signieren_mit(…, &[])`, `--options runtime` setzt allein
`signieren_gehaertet` (`sign.rs:222`), und dessen einziger Rufer ist `release`.

**Der Hilfetext trägt die Wendung, an die die neue Probe bindet.** `main.rs:71` führt
„Developer-ID-Identitaet und gehaerteter Laufzeitumgebung"; die Zusicherung gegen
`include_str!("main.rs")` läuft also nicht ins Leere. Was sie darüber hinaus verspricht,
steht unten unter N1.

**Die Zusage „kein zusätzlicher Systemaufruf beim Lesen" hält.** Nachgezählt mit `grep`:
`verweisziel::bestimmen` hat außerhalb der Proben genau einen Rufer, `tabelle.rs:1426`, und
der steht im Zweig `Typ::Verknuepfung`. Kein Modul unter `crates/krk-core/src/verzeichnis/`
ruft es — weder `leser`, noch `modell`, noch `durchlauf`. Die Rechnung, an der L3 und L10
hängen, ist unangetastet. `in_zeile_einsteigen` selbst hat zwei Rufer, den Doppelklick
(`tabelle.rs:1144`) und den Rechts-Pfeil (`tabelle.rs:1379`).

**Der Deskriptor wird in jedem Zweig geschlossen, und `O_NONBLOCK` bleibt nirgends stehen.**
In `bestimmen` (`verweisziel.rs:84-94`) ist `datei` eine `File`-Bindung; sie fällt in allen
drei Zweigen mit dem Funktionsende. Scheitert `blockierend_stellen` innerhalb von
`ohne_warten_oeffnen` (`sys.rs:812-819`), trägt das `?` den Fehler heraus und die lokale
`File` schließt beim Verlassen. Ein Pfad, auf dem ein offener Deskriptor mit gesetztem
`O_NONBLOCK` an den Aufrufer geht, existiert nicht.

**Der Einstieg mit dem Pfad der Verknüpfung führt den Aufstieg richtig zurück.**
`eintrag_in_zeile` (`tabelle.rs:1462-1467`) baut `tab.ordner().join(&eintrag.name)`, also den
Pfad der Verknüpfung; `ordner_setzen` (`tabs.rs:592-619`) legt ihn unverändert ab und
kanonisiert nicht; `aufwaerts` (`verzeichnis/mod.rs:86-90`) ist reine Pfadarithmetik über
`parent()`. Zeigt die Verknüpfung tiefer oder ganz woandershin, ändert das daran nichts —
der Aufstieg ist lexikalisch und landet im Ordner der Verknüpfung, mit der Auswahl auf ihr.

**Die Auffrischung überlebt den verknüpften Pfad.** Das war der naheliegende Folgeschaden und
tritt nicht ein: `auffrischung::gleicher_ordner` vergleicht erst die geschriebene und dann
die aufgelöste Form, ausdrücklich weil „FSEvents den Pfad in aufgeloester Form" meldet und
„`/tmp` eine Verknuepfung ist" (`auffrischung.rs:63-73`). Ein Ordner, der über eine
Verknüpfung angesteuert wurde, frischt also auf.

**Die Röhrenprobe misst, was ihr Name sagt.**
`eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an`
(`tests/verzeichnis.rs:1920-1946`) legt die Frage auf einen eigenen Faden und lässt den
Hauptfaden mit `recv_timeout(5s)` warten; hängt `bestimmen`, scheitert die Probe mit einem
benannten Satz statt die Suite anzuhalten. Das trägt. Ihr Doc-Kommentar stellt allerdings
`File::open` als Gegenstück auf, und das war nie die Alternative; dazu H1.

**Die Warnung des Quelldatensatzes zur Sortierung ist wirklich gegenstandslos.**
`sortierung.rs`, `fn gruppe`, liest `eintrag.typ == Typ::Ordner` unmittelbar. `ist_ordner`,
`Typ` und die Sortierung sind unberührt; nachgeprüft, nicht übernommen.

**Der Statuszeilensatz folgt der Hausform.** `"{} lässt sich nicht öffnen: {grund}"` mit dem
rohen `io::Error`-Text ist dieselbe Bauart wie `pfadeingabe.rs:63` und `:73`
(`"{} gibt es nicht: {fehler}"`). Kein Befund.

## Befunde

### Der Mechanismus der Auflösung (1 hoch)

**H1 — Die Ordnerfrage wird mit `open(2)` beantwortet, das sie nicht entscheiden kann.**
`verweisziel.rs:84-94`. Am Referenzgerät gemessen (macOS 24.6.0, uid 502): eine Verknüpfung
auf einen Unix-Socket scheitert mit `EOPNOTSUPP` — nicht mit `ENXIO`, wie die Abschlussnotiz
annimmt —, eine auf eine gewöhnliche Datei ohne Leserecht mit `EACCES`, eine auf ein
Verzeichnis mit Modus 0111 ebenfalls mit `EACCES`. Alle drei kommen als `Unerreichbar`
zurück; `stat` sagt für alle drei die Wahrheit. Der Socket ist also **nicht** die einzige
Fehleinordnung, und der praktisch häufigste Fall ist die zweite Zeile: eine Verknüpfung auf
eine Datei ohne Leserecht bekommt eine Fehlermeldung, statt wie jede andere Datei an das
Standardprogramm zu gehen.

Damit trägt die Zusicherung `verweisziel.rs:50-52` nicht auf der Ebene, auf der ihre Werte
benannt sind: `KeinOrdner` beschreibt sich als „eine gewoehnliche Datei, eine Geraetedatei,
eine benannte Roehre" (`:57-60`), und genau eine solche kommt als `Unerreichbar` zurück. Die
zwei Doc-Kommentare beschreiben denselben Zustand.

Die zwei Gründe, mit denen der Modulkopf (`:29-38`) den Deskriptorweg begründet, gelten hier
beide nicht. Das Fenster zwischen Prüfung und Öffnen verschwindet nur, wenn derselbe
Deskriptor weiterbenutzt wird — `bestimmen` gibt ihn ab, und `ordner_lesen(&ziel, None)`
öffnet den Pfad ein zweites Mal. Und `stat(2)` blockiert an einer Röhre nicht; das steht
wörtlich in der Abschlussnotiz des Bauenden selbst
(`shared/history/260815-1658-coder-…`, Abschnitt „Offen"). Dazu kommt eine Nebenwirkung, die
`stat` nicht hätte: `bestimmen` **öffnet** das Ziel, und bei einer Verknüpfung auf eine
Gerätedatei wirkt schon das Öffnen am Gerät.
→ `shared/issues/260815-1713_o_…`

### Was die Prosa sagt und der Baum trägt (2 mittel)

**M1 — Sechs Stellen nennen zwei Aufrufer der Hülle, es sind drei.** `sys.rs:15-16`, `:46-49`,
`:787`, `:789-792`, `:810-811` und `CLAUDE.md:135`. `verzeichnis/mod.rs` ist ordentlich
nachgezogen, die Datei mit der Hülle selbst nicht. Eine Aussage ist dadurch nicht nur
unvollständig, sondern falsch: `sys.rs:794-796` schreibt einen „gemeinsamen Ablauf" fest
— „alles abweisen, was `is_file()` nicht bejaht, die Groesse gegen eine Grenze halten, erst
danach lesen" —, und der dritte Rufer tut nichts davon. Die Familie ist teuer belegt:
`260812-1438`, `260812-2253`, `260813-1345` und `260815-1047` sind vier Erhebungen derselben
Art in vier Tagen.
→ `shared/issues/260815-1714_o_…`

**M2 — Der Aufrufkommentar in `main.rs` trägt den ersetzten Leitsatz weiter.**
`main.rs:145-146`: „Was er sagt, entscheidet die Art der Identitaet; siehe
[`sign::weitergabehinweis`]." Genau dieser Satz ist der, den `a46fd1f` aus dem Modulkopf
genommen hat, weil er die falsche Einordnung mittrug. Der Kommentar schickt den Leser mit dem
falschen Halbsatz in eine Funktion, deren Doc-Kommentar jetzt ausdrücklich das Gegenteil sagt
(`sign.rs:154-157`). Wer ihn liest und den Hinweis erweitert, greift zur Identitätsart, also
zu dem `security`-Aufruf, den `260815-1444` verworfen hat.
→ `shared/issues/260815-1715_o_…`

### Was die Proben halten (1 niedrig)

**N1 — Die `include_str!`-Bindung koppelt eine Wendung, der Kommentar verspricht eine
Beschreibung.** `sign.rs:660-665`. Der Wächter ist echt und mehr, als vorher dastand. Er
prüft aber die ganze Datei statt der Konstante `HILFE`, kennt also keine Stellung, und er
koppelt 31 Zeichen einer zweigliedrigen Beschreibung; die übrigen Formulierungen laufen
weiter frei nebeneinander. „Eine Beschreibung von `release`, nicht zwei" beschreibt das
nicht. Es ist derselbe Befund wie `260815-1446` und `260815-1447` aus der vorigen Durchsicht,
zum dritten Mal.
→ `shared/issues/260815-1716_o_…`

## Übergreifend

**Beide Commits importieren eine Bauform samt ihres Namens, aber ohne ihren Grund.** H1 nimmt
`ohne_warten_oeffnen`, weil es „die im Baum eingefuehrte Form" ist — und die Form ist für
Öffnen-dann-Lesen eingeführt, nicht für eine Frage, nach der nichts gelesen wird. M2 lässt
einen Satz stehen, dessen Grund im selben Commit entfallen ist. In beiden Fällen ist die
Formulierung stimmig und der Bezug leer.

**Die Regel „eine Stelle je Frage" hält in vier von fünf Fällen und bricht in einem.**
`in_zeile_einsteigen` bleibt der eine Einstiegsrumpf für Doppelklick und Rechts-Pfeil,
`mit_standardprogramm_oeffnen` bleibt die eine Öffnungsstelle, `unerreichbar` ist der eine
Bauplatz seines Wertes, `ist_ordner` behält seine anderen Rufer mit Begründung je Rufer.
Gebrochen ist sie bei der Frage „führt dieser Pfad auf ein Verzeichnis?": sie steht jetzt
zweimal da, in `pfadeingabe::pruefen` mit `std::fs::metadata` und in `verweisziel::bestimmen`
mit `open`, und die beiden antworten verschieden. Der Modulkopf von `pfadeingabe.rs` warnt
genau davor: „Ein zweiter Navigationsweg daneben waere die zweite Wahrheit darueber, was KRK
fuer einen gangbaren Pfad haelt, und die erste Abweichung zwischen beiden faende keine
Pruefung."

**Die Aufzählung statt des Wahrheitswerts ist die bessere Hälfte von `8c06747`.** „Gemeldet"
von „kein Ordner" zu trennen, statt die unerreichbare Verknüpfung zusätzlich an das
Standardprogramm zu reichen, ist die richtige Entscheidung und aus dem richtigen Grund
getroffen: die Antwort des Systems überschriebe die eben geschriebene Statuszeile. Beide neuen
Aufzählungen stehen ohne Auffangzweig, wie es dieses Projekt hält, und `Einstieg` trägt
`#[must_use]` mit einem `let _ =` beim einen Rufer, der die Antwort nicht braucht.

**Eine Fläche bleibt unbeantwortet.** Der Quelldatensatz nennt Dateioperation, Lesezeichen
und Vorschau als offen. Eine fünfte kommt hinzu: bei eingeschaltetem „Deep" und stehendem
Filtertext meldet der Durchlauf für eine Verknüpfung immer „kein Treffer darunter"
(`modell.rs:565-573`), die Zeile fällt also aus der Liste, und der Ordner, den man seit
diesem Commit betreten kann, ist in dieser Lage nicht anklickbar. Das folgt aus dem
Nutzerentscheid `260814-1552` („die tiefe Suche steigt nicht hinab") und ist kein Widerspruch
zu ihm, aber es ist eine Fläche, die der Datensatz nicht führt. Kein eigener Befund; gehört
in die Liste des offenen Datensatzes `260814-1612`.

## Reihenfolge

**Kein Auslieferungshindernis.** `cargo clippy --workspace --all-targets` läuft in diesem
Baum durch (Exit 0, einmal gefahren). Keiner der vier Befunde ändert den Bau oder das
Bündel, und der gemeldete Defekt ist behoben.

1. **H1** zuerst, und als einziger vor einer Auslieferung erwägenswert: der Wechsel auf
   `std::fs::metadata` ist klein, nimmt zwei Systemaufrufe weg und schließt drei
   Fehleinordnungen. Er trägt eine Nutzerfrage mit sich — ob `Verweisziel::Ordner` das
   Leserecht mitprüfen soll, wie `pfadeingabe::pruefen` es tut.
2. **M1** danach; fällt teilweise von selbst weg, wenn H1 so entschieden wird, denn dann hat
   die Hülle wieder zwei Rufer.
3. **M2** ist ein Halbsatz.
4. **N1** zusammen mit `260815-1446` und `260815-1447`: drei Proben derselben Art, ein
   Durchgang.

---
Abgleich 260815-1812 (reconciler), nur Statusvermerk — keine Aussage dieser Durchsicht ist
geändert.

Von den vier abgelegten Datensätzen sind zwei geschlossen und zwei stehen offen:

| Datensatz | Stand am 260815-1812 | Beleg |
|---|---|---|
| `260815-1713` Ordnerfrage über `open(2)` statt `stat(2)` (Schwere hoch) | `_c_` | `7fae5ba`; `crates/krk-core/src/verzeichnis/verweisziel.rs:164-165` fragt `std::fs::metadata` |
| `260815-1714` `sys.rs` und `CLAUDE.md` nennen zwei Rufer, es sind drei | `_c_` | mit `7fae5ba` von selbst erledigt: `grep -rn 'ohne_warten_oeffnen' crates/` findet wieder genau zwei Aufrufstellen |
| `260815-1715` Aufrufkommentar in `main.rs` | `_o_` | unverändert |
| `260815-1716` `include_str!`-Bindung koppelt eine Wendung | `_o_` | unverändert |

**Der Befund der Schwere hoch ist behoben, und die Behebung ist selbst nicht durchgesehen.**
`7fae5ba` liegt hinter dem Bereich dieser Durchsicht (`a2670db..8c06747`) und wird von keiner
zweiten gedeckt; `bin/fusion-review-coverage` führt ihn unter den sieben ungedeckten Commits.
Er ist der einzige davon, der Code anfasst. Aufgenommen als
`shared/issues/260815-1812_*_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md`.
Der Abgleich hat an `7fae5ba` zwei Abweichungen gefunden, beide auf der Beschreibungsebene:
`shared/issues/260815-1812_*_ein-verweis-im-modulkopf-des-verweisziels-zeigt-auf-einen-datensatz-der-nie-so-hiess.md`
und die Zählangabe im Rumpf von `260815-1752`.
