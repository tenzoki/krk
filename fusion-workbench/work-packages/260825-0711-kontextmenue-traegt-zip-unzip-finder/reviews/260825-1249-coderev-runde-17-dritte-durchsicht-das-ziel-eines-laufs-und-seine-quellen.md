# Codedurchsicht: Runde 17, dritte Durchsicht — das Ziel eines Laufs und seine Quellen

**Reviewed-range:** `6faaa91..ddd41ff`
**Not-opened:** none

**Geoeffnet:** beide Commits des Bereichs als Unterschied, dazu die vier Codedateien aus `dd74b0e`
am Baumstand `ddd41ff` in voller Laenge an den beruehrten Stellen —
`crates/krk-ui/src/kommandos/kontextmenue.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-core/src/operation/zippen.rs` — und die von ihnen
gerufenen Nachbarn `crates/krk-core/src/operation/auftrag.rs`,
`crates/krk-core/src/operation/umbenennen.rs`, `crates/krk-ui/src/kommandos/operationen.rs`,
`crates/krk-ui/src/quellbaum.rs`. `ddd41ff` fasst allein Werkbank-Prosa an und ist als solcher
gelesen. Gelesen sind ausserdem die vier geschlossenen Defektdatensaetze der zweiten Durchsicht samt
ihrer `Resolved:`-Notizen, der ausloesende Datensatz `260825-1230_*_der-groesste-codecommit-…` und
die zwei einschlaegigen Abschnitte von `shared/history/260824-2120-orchestrator-session.md`.

**Hier gefahren am 260825:** `cargo test --workspace` (Exit 0, kein Fehlschlag),
`cargo clippy --workspace --all-targets` (Exit 0, keine Warnung), `cargo fmt --all --check`
(Exit 0). Dazu eine Gegenprobe der Paarungsprobe ausserhalb des Baums, nachgerechnet am Wortlaut von
`rumpf` (siehe Frage 4).

Diese Durchsicht schliesst an `260825-1144-coderev-…` (`6ad9198..6faaa91`) und
`260825-0942-coderev-…` (`428fbc4..423d5f2`) an. Die drei tilen zusammen den ganzen Sitzungsbereich
`428fbc4..ddd41ff`.

---

## Zusammenfassung

Die dritte Nutzerzusage ist gebaut, und sie ist in beiden Gestalten gebaut. Sie haelt fuer die
Faelle, die die Runde vorfuehrt, und faellt in einem erreichbaren vierten: der Schnitt vergleicht
Pfade buchstabengetreu, waehrend das Bauziel die Schreibung faltet und dieselbe Datei ein Archiv
ausdruecklich ohne Ruecksicht auf die Schreibung erkennt. Der Schnitt ist daneben an einer Stelle zu
weit und an einer Stelle stumm. Der schwerste Befund ist keiner am Verhalten: die Zusage ist ueber
eine Kistengrenze an einen Rufer abgegeben worden, und keine Probe haelt, dass dieser Rufer sie
traegt.

Der Modulkopf von `zippen.rs` ist ehrlich geworden, die Paarungsprobe zaehlt, was sie zu zaehlen
behauptet, und die Reihenfolge der Namensrechnung stimmt. Drei der fuenf Fragen der Beauftragung
sind mit Ja beantwortet.

## Summen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 4 |
| Gering | 2 |

## Die fuenf Fragen, einzeln beantwortet

### 1. Haelt die Zusage, in beiden Gestalten? — Teilweise.

**Der Bau stimmt.** Die Regel steht genau einmal da (`kontextmenue.rs:598-600`) und hat genau zwei
Rufer: `packziel` (`:453`) fuer das Packen und `ohne_die_eigenen_ziele` (`:612`) fuer das Entpacken.
Nachgezaehlt mit `grep`, keine dritte Fassung im Baum.

**Packen, der Fall des zweiten Laufs.** Markiert `{a.txt, Projekte.zip, b.txt}` im Ordner
`Projekte`: `archivname` rechnet `Projekte/Projekte.zip`, der Filter nimmt den gleichnamigen Eintrag
heraus, uebrig bleiben `{a.txt, b.txt}`. Gehalten von
`das_archiv_des_vorigen_laufs_faellt_aus_den_quellen` (`:1095`). Nachgerechnet, stimmt.

**Entpacken, `a.zip` neben `a.zip.zip`.** `paar` rechnet fuer das zweite den Zielordner
`<ordner>/a.zip`; `ohne_die_eigenen_ziele` haelt jedes Archiv gegen **alle** Ziele des Laufs und
nimmt `a.zip` heraus. Gehalten von `ein_archiv_das_zielordner_eines_anderen_ist_faellt_aus_den_quellen`
(`:1147`). Nachgerechnet, stimmt.

**Wo sie faellt.** `ist_ziel_des_laufs` vergleicht mit `==` auf `Path`, also buchstabengetreu. Das
Bauziel ist macOS, dessen APFS in der Vorgabe die Schreibung faltet, und `ist_zipname`
(`kontextmenue.rs:304-307`) faltet sie ausdruecklich mit — die zweite Nutzerentscheidung dieser
Runde. Ein Eintrag `PROJEKTE.ZIP` und das gerechnete Ziel `Projekte.zip` sind fuer den Schnitt zwei
Pfade und auf der Platte einer. Der Konfliktweg laeuft dann genau wie vor `dd74b0e`. Datensatz:
`issues/260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-waehrend-das-dateisystem-und-die-endungsregel-die-schreibung-falten.md`.
Die naheliegende Antwort — `eq_ignore_ascii_case` — ist nicht die richtige: auf einem
schreibungsempfindlich formatierten Datentraeger schnitte sie zu weit. Das ist eine Nutzerfrage und
keine Zeile nebenbei; der Datensatz legt drei Wege vor.

**Was die Zusage ihrem Wortlaut nach nicht verspricht, und das ist richtig so.** Markiert der Nutzer
`{a.txt, Projekte.zip}`, so faellt `Projekte.zip` aus den Quellen — und geht danach als **Ziel**
ueber das Konfliktblatt in den Papierkorb, wenn der Nutzer „Ueberschreiben" waehlt. Am Ausgang
aendert der Schnitt in dieser Lage nichts; er aendert, unter welchem Titel der Eintrag dorthin
kommt. Der Nutzer hat das in seiner Antwort selbst ausgeschrieben („Danach greift die Rueckfrage wie
sonst, denn der Zieleintrag steht ja weiterhin auf der Platte"), und der Modulkopf sagt es ebenso.
Kein Befund, aber die Grenze der Zusage, und sie gehoert in den Abnahmelauf.

### 2. Ist der Schnitt zu weit? — Beim Packen nein, beim Entpacken in einem Fall ja.

**Packen.** `ein_einzelnes_archiv_bleibt_seine_eigene_quelle` (`:1124`) haelt den Fall: `sicherung.zip`
allein markiert ergibt das Ziel `sicherung.zip.zip`, also einen anderen Pfad, und die Quelle bleibt
stehen. Nachgerechnet ueber `archivname` (`:416-426`): bei genau einem Eintrag ist der Stamm sein
**voller** Name samt Endung, das Ziel traegt die Endung ein zweites Mal. Der Schnitt kann hier nicht
zu weit greifen.

**Streng geprueft, wie gefordert: die Probe traegt weniger, als ihr Name sagt.** Sie ruft `packziel`
und nur `packziel`. Der Entpackfall — ein einzelnes Archiv, Zielordner `a`, der haeufigste
Unzip-Fall — kommt in ihr nicht vor. Er ist gehalten, aber von aelterem Bestand:
`drei_betroffene_archive_ergeben_drei_zielordner` (`:980`) und
`ohne_betroffenes_archiv_gilt_das_eine_des_ordners` (`:1021`) wuerden rot, waere der Entpackschnitt
zu weit. Beide stammen aus der Zeit vor dem Schnitt und nennen ihn im Namen nicht. Die in der
Commit-Nachricht gefahrene Gegenprobe („die dritte bleibt gruen") misst deshalb nicht, was sie zu
messen meint: die dritte bleibt gruen, weil sie den Entpackschnitt nicht beruehrt. Datensatz:
`issues/260825-1249_*_die-probe-gegen-den-zu-weiten-schnitt-prueft-nur-den-packschneider-der-entpackschneider-hat-keine.md`.

**Und der Entpackschnitt ist an einer Stelle wirklich zu weit.** `ohne_die_eigenen_ziele` rechnet die
Zielliste **einmal** und filtert danach; ein Archiv faellt auch dann, wenn sein einziger
Beanspruchter selbst gefallen ist. Aus `{a.zip, a.zip.zip, a.zip.zip.zip}` bleibt ein Paar, wo zwei
kollisionsfrei liefen: `a.zip.zip.zip` → `a.zip.zip` und `a.zip` → `a` beruehren einander nicht. Der
Doc-Kommentar (`:611`) schreibt das Ergebnis als beabsichtigt aus. Die Kette entsteht aus der
anhaengenden Endungsregel dieser Runde von selbst. Datensatz:
`issues/260825-1249_*_der-entpackschnitt-ist-kein-festpunkt-ein-archiv-faellt-wegen-eines-beanspruchers-der-selbst-gefallen-ist.md`.

### 3. Wird der Name wirklich nicht nachgerechnet? — Ja, und die Begruendung traegt.

Nachgesehen, drei Stufen:

1. **Die Reihenfolge in `packziel`** (`:453-460`): `let ziel = archivname(betroffen, ordner);` steht
   vor dem Filter, und `betroffen` ist die ungefilterte Markierung. Der Filter erzeugt eine neue
   Liste und laesst `ziel` unberuehrt.
2. **Es gibt keinen zweiten Rechner.** `grep` ueber `crates/` findet fuer `archivname(` genau zwei
   Rufer: `packziel` und die Proben desselben Moduls. `packziel(` hat genau einen Rufer,
   `anwendung.rs:6125`. `zipauftrag_stellen` nimmt beide Werte von dort und rechnet nichts nach.
3. **Auch der Kern rechnet keinen Namen.** `zielarchiv_klaeren`
   (`crates/krk-core/src/operation/zippen.rs`) nimmt `ziel` entgegen; der Zweig
   `Konfliktantwort::UmbenennenIn(name)` bildet `ziel.with_file_name(name)` aus dem **getippten**
   Namen und nicht aus den Quellen.

Die Begruendung stimmt auch der Sache nach: ohne die Reihenfolge wuerde aus `{a.txt, Projekte.zip}`
nach dem Schnitt eine einelementige Liste, und `archivname` gaebe fuer sie `a.txt.zip` statt
`Projekte.zip`. Der zweite Lauf hiesse anders als der erste. Nachgerechnet an der Regel, nicht bloss
nachgelesen.

### 4. Wird die Paarungsprobe bei vertauschten Zweigen rot? — Ja, gegengerechnet.

Der Rumpf von `kontextbefehl_ausfuehren` (`anwendung.rs:6081-6087`) traegt drei Zeilen, je Befehl
eine. Die Probe (`anwendung.rs:9064`) zaehlt Zeilen, die Befehlsnamen **und** Zweignamen zugleich
tragen, und verlangt genau eine.

Ausserhalb des Baums nachgerechnet, an der Wortlautfassung von `rumpf` (`anwendung.rs:8285-8299`)
und am echten Dateiinhalt:

| Stand | Zippen | Entpacken | ImFinderZeigen |
|---|---|---|---|
| wie er dasteht | 1 | 1 | 1 |
| Zip und Unzip vertauscht | **0** | **0** | 1 |

Beide Zeilen werden rot, nicht nur eine. Die Zusage der Commit-Nachricht stimmt.

**Bleibt sie aus einem anderen Grund gruen?** Drei Wege geprueft, alle drei zu:

- **Doc-Kommentare im Rumpf.** `rumpf` wirft jede Zeile weg, die nach `trim_start` mit `//` beginnt
  (`anwendung.rs:8294-8297`). Eine Kommentarzeile, die Befehl und Zweig zusammen nennt, kann die
  Zaehlung darum nicht auf 1 heben, wenn der Zweig falsch ist.
- **Der falsche Rumpf.** `rumpf` sucht `fn {name}(`, also die Erklaerung und nicht die erste
  Fundstelle des Namens; der Aufruf bei `anwendung.rs:1391` wird nicht getroffen.
- **Ein fehlender Rumpf.** `rumpf` bricht mit `panic!` ab, wenn Kopf oder Ende fehlen. Kein stiller
  Ausgang.

**Ein Rest bleibt, und er ist zu schmal fuer einen Datensatz:** die Probe vergleicht Teilzeichenfolgen
und keine Bezeichner. Ein zusaetzlicher Zweig `zipauftrag_stellen_v2` neben dem bestehenden zaehlte
mit. Ein blosses Umbenennen faellt dagegen laut aus, weil das zweite Glied der Probe
`rumpf(&datei, zweig)` ruft und dann kein `fn` mehr findet. Genannt, damit es beim naechsten Zug an
dieser Datei mitgelesen wird.

### 5. Traegt die neue Begruendung im Modulkopf von `zippen.rs`? — Sie ist ehrlich und ungehalten.

**Was besser geworden ist.** Das alte Argument („keine Loeschstelle nennt `auftrag.quellen`") stand
als Beweis der Zusage da und war eine Aussage ueber den Quelltext. Der neue Kopf
(`zippen.rs:53-72`) nennt genau das, sagt, warum es die Frage nicht beantwortet, benennt den Fall des
zweiten Laufs und schreibt die Folge aus: „geraeumt wird der Zielpfad, wer immer ihn hereinreicht."
Das ist der Stand, den die zweite Durchsicht verlangt hat, und er ist ohne Beschoenigung
geschrieben.

**Was nicht traegt: das Wort „sichert".** „Dass dieser Eintrag keine Quelle desselben Laufs ist,
sichert der Rufer und nicht dieser Zweig." Der Rufer tut es heute. Gehalten ist es von nichts:

- Keine Probe prueft, dass der Rumpf von `zipauftrag_stellen` `packziel` nennt.
- Keine zaehlt die Rufer von `Auftrag::zippen`.
- Die Signatur laesst es zu: `Auftrag::zippen(quellen, ziel)` nimmt zwei unabhaengige Listen. Der
  Doc-Kommentar der Schwesterfunktion sagt, wie es anders geht — `Auftrag::entpacken` nimmt Paare,
  „damit die beiden Listen gar nicht erst getrennt uebergeben werden koennen"
  (`crates/krk-core/src/operation/auftrag.rs:164-169`).

Nachgerechnet: wer in `zipauftrag_stellen` `packziel` weiter ruft, davon aber nur das `ziel`
verwendet und `auswahl.pfade` als Quellen weiterreicht, bekommt den Defekt zurueck — ohne
`dead_code`, ohne Warnung, mit gruenem `make check`. Datensatz:
`issues/260825-1249_*_die-zusage-haengt-jetzt-am-rufer-in-einer-anderen-kiste-und-keine-probe-haelt-dass-er-sie-traegt.md`.

## Befunde nach Themen

### Thema 1: der Schnitt und die Wirklichkeit der Platte

**B10 — der Vergleich faltet nicht, das Dateisystem schon.** Mittel.
`kontextmenue.rs:598-600`. Ausgeschrieben unter Frage 1.
Datensatz: `issues/260825-1249_*_der-schnitt-vergleicht-pfade-buchstabengetreu-*`.

**B11 — der Entpackschnitt ist kein Festpunkt.** Mittel. `kontextmenue.rs:612-618`. Ausgeschrieben
unter Frage 2. Datensatz: `issues/260825-1249_*_der-entpackschnitt-ist-kein-festpunkt-*`.

### Thema 2: der Schnitt und der Nutzer

**B12 — geschnitten wird wortlos.** Mittel. Von drei markierten Archiven entsteht ein Ordner, und
kein Satz sagt, was mit den zwei anderen ist. Keine Statuszeile, keine Abschlussliste — der
geschnittene Eintrag steht nicht im Auftrag, und die Abschlussliste kennt nur, was darin steht. Beim
Packen zaehlt die Positionszahl seit `dd74b0e` die verbliebenen Quellen (`anwendung.rs:6130`), zeigt
also „1" statt „2", und beantwortet nicht, warum.

**Der Baum hat diese Frage fuer die Schwesterlage schon anders entschieden**, und zwar in derselben
Datei, fuenfzig Zeilen ueber dem Schnitt (`kontextmenue.rs:340-344`): „Und den Eintrag
stillschweigend aus `Entpackbefund::Archive` zu nehmen waere schlechter als beides: von drei
markierten Archiven bliebe eines ohne Ordner und ohne Wort." Der Satz entscheidet dort den Fall des
unbrauchbaren Namens gegen das stille Herausnehmen; `ohne_die_eigenen_ziele` nimmt seit `dd74b0e`
genau so heraus.

Die Nutzerantwort deckt das Herausfallen und sagt zur Meldung nichts. Datensatz:
`issues/260825-1249_*_der-schnitt-nimmt-markierte-eintraege-aus-dem-lauf-und-kein-wort-erreicht-den-nutzer.md`.

### Thema 3: wer die Zusage haelt

**B13 — die Zusage haengt am Rufer, und keine Probe haelt ihn.** Mittel. Ausgeschrieben unter
Frage 5.

### Thema 4: Proben und Prosa

**B14 — die Probe gegen den zu weiten Schnitt prueft nur einen der zwei Schneider.** Gering.
Ausgeschrieben unter Frage 2.

**B15 — drei neue Zitate sind mitten im Pfad umgebrochen.** Gering. `zippen.rs:65-66`,
`kontextmenue.rs:87-88`, `anwendung.rs:9054-9055`. Zwischen `...-in-den-` und `papierkorb-*` stehen
ein Zeilenende, ein Kommentarzeichen und Leerzeichen; weder `grep` noch ein Glob loest das auf.
Dieselbe Form steht schon zweimal im Baum, `zippen.rs:40-41` und `93-94`, aus fruehen Commits
derselben Runde, von beiden fruehen Durchsichten nicht benannt. Die **Kuerzung** auf `…-des-laufs-*`
ficht der Datensatz nicht an: sie ist verbreitete Uebung dieses Baums und haengt an den offenen
Datensaetzen `shared/issues/260810-1851_*` und `shared/issues/260817-1130_*`.
Datensatz: `issues/260825-1249_*_drei-neue-zitate-sind-mitten-im-pfad-*`.

## Die Berichtigung in `tabelle.rs`: geprueft, haelt

`crates/krk-ui/src/appkit/tabelle.rs:199` — das doppelte `//! //!` ist weg, der Absatz zu
`clickedRow` steht wieder im Abschnitt „Ab welchem macOS die angesprochenen Klassen stehen" und
nicht als Text darin. Eine Zeile, ein Zeichen, genau der Fix, den
`260825-1144_*_ein-doppeltes-kommentarzeichen-*` verlangt hat. Nichts daneben angefasst.

## Die Projektbindungen

Alle geprueft, alle halten.

- **`#[must_use]`**: `packziel` traegt es, wie jede oeffentliche Funktion dieses Moduls. Die zwei
  neuen privaten Helfer tragen es nicht, wie die uebrigen privaten Helfer derselben Datei (`paar`,
  `ist_archivpfad`, `brauchbarer_stamm`) es auch nicht tun. Konsequent, und keine Stelle, an der ein
  stilles Fallenlassen unbemerkt bliebe.
- **`#![deny(unsafe_code)]` in `krk-core`**: unberuehrt. `zippen.rs` hat allein Prosa bekommen, und
  der Unterschied zeigt keine Codezeile.
- **Jedes `unsafe` in `appkit/` mit Begruendung**: `dd74b0e` fuegt kein `unsafe` hinzu; nachgesehen
  am Unterschied.
- **Abschnitt „Ab welchem macOS die angesprochenen Klassen stehen"**: keine neue Datei unter
  `appkit/`. Die Berichtigung in `tabelle.rs` stellt einen Absatz **in** diesen Abschnitt zurueck.
- **Vollstaendige Fallunterscheidungen ohne Auffangzweig**: `kontextbefehl_ausfuehren` unveraendert
  dreizweigig ohne `_ =>`; die drei Zweige des `Entpackbefund` in `entpackauftrag_stellen` ebenso.
  `packziel` fuehrt keine neue Fallunterscheidung ein.
- **Prosa deutsch**: durchweg.
- **L9 (kein Dateisystemzugriff, der den Hauptfaden waehrend eines laufenden Vorgangs anhaelt)**:
  gehalten. `packziel`, `ist_ziel_des_laufs` und `ohne_die_eigenen_ziele` rechnen auf `PathBuf`-Werten
  und fassen kein Dateisystem an; der Doc-Kommentar von `ist_ziel_des_laufs` (`:591-597`) sagt es
  ausdruecklich zu. `zipauftrag_stellen` hat keinen Zugriff hinzubekommen, sondern einen verschobenen
  Vergleich. Der Befund B10 beruehrt diese Bindung: der genaue Vergleich braeuchte die Platte, und
  das ist der Grund, aus dem Weg 2 seines Datensatzes nicht der billige ist.

## Was quer liegt

**Die drei Durchsichten dieser Runde finden dreimal dieselbe Form, und sie wandert nach aussen.**
Erste Durchsicht: ein **Typ** sagt zu, was er nicht geprueft hat. Zweite: ein **Modulkopf** sagt zu,
was sein Beleg nicht traegt. Dritte: der Modulkopf ist berichtigt und gibt die Zusage an einen
**Rufer in einer anderen Kiste** ab, den nichts haelt. Der Beleg ist jedes Mal einen Schritt weiter
von der Stelle weggerueckt, an der die Zusage gilt. Die Antwort dieses Baums auf so etwas steht
schon da und heisst Zaehlprobe ueber den Quellbaum; hier ist sie nicht gezogen worden.

**Zwei Befunde sind derselbe Riss, von zwei Seiten.** B11 sagt, der Schnitt nehme mehr, als die
Zusage deckt; B12 sagt, er sage darueber nichts. Wer B12 baut, sieht B11 am Bildschirm, sobald ein
Nutzer die Kette von drei einmal markiert. Wer B11 allein baut, macht den Fall seltener, ohne ihn zu
schliessen.

**Die Fallunterscheidung ist nicht disjunkt zur Wirklichkeit, sondern zu einem Modell davon.**
`ist_ziel_des_laufs` teilt die Markierung in „ist das Ziel" und „ist es nicht", vollstaendig und
ueberschneidungsfrei — ueber Pfadzeichenfolgen. Die Frage, die der Nutzer gestellt hat, gilt
**Eintraegen auf der Platte**, und dort faellt die Trennung anders, sobald die Schreibung abweicht.
Das ist kein Schnitzer im Bau, sondern die Grenze des Wegs, den der Nutzer bewusst gewaehlt hat: die
Oberflaeche entscheidet ohne Dateizugriff. Sie gehoert ausgeschrieben, damit die naechste Runde nicht
denselben Weg noch einmal fuer vollstaendig haelt.

**Was diese Durchsicht ausdruecklich nicht als Befund fuehrt**, weil die Beauftragung es ausnimmt:
dass die drei Befehle nur ueber die Maus erreichbar sind; dass der Abnahmelauf am gebauten Buendel
aussteht; die zwei bekannten offenen Defekte `260825-0838` (Zeitstempel 1980) und
`shared/issues/260825-1130` (selbst getippter Name im Konfliktblatt).

## Reihenfolge

1. **B13** vor dem Rundenabschluss. Zehn Zeilen Probe, und ohne sie ist die Nutzerzusage dieser Runde
   von einer einzigen Zeile gehalten, die keine Probe liest.
2. **B10** als Nutzerfrage, ebenfalls vor dem Abschluss. Er entscheidet, wie genau die Zusage sein
   soll; der Datensatz legt drei Wege vor, und keiner ist umsonst.
3. **B12** mit dem naechsten Zug an derselben Datei. Ein Satz in der Statuszeile, an einer Stelle
   formuliert.
4. **B11** danach, zusammen mit B12 oder als Berichtigung des Doc-Kommentars.
5. **B14** und **B15** sind Aufraeumen.
