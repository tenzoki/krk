# Durchsicht: die Vorgangsmaschine und das Stapelumbenennen

**Reviewed-range:** `004ff72..004ff72` — kein Commit-Bereich, sondern eine Vollbaum-Durchsicht von crates/krk-core/src/{operation,stapelumbenennen}/ am Stand dieses einen Commits
**Not-opened:** none
**Sender:** coderev
**Gelesen:** alle 14 Dateien des Umfangs
**Umfang:** 14 Dateien, 3.993 Zeilen (`git ls-files 'crates/krk-core/src/operation/*' 'crates/krk-core/src/stapelumbenennen/*'`)

## Zusammenfassung

Beide Module sind auffallend sorgfaeltig gebaut: die Konfliktbehandlung ist an drei Stellen
dieselbe, der Abbruch wird innerhalb jeder grossen Datei geprueft und nicht nur zwischen
Eintraegen, jeder Abbruchpfad raeumt seinen halben Rest weg, und fast jede Entscheidung traegt
ihre Begruendung samt der verworfenen Alternative im Doc-Kommentar. Die Zusagen aus `CLAUDE.md`
halten mit **einer** Ausnahme: zwei Verzweigungen ueber `Art` tragen einen Auffangzweig.

Der schwerwiegende Befund liegt nicht bei Zip und Unzip, sondern im aeltesten Weg der Maschine:
das Verschieben ueber eine Datentraegergrenze loescht die Quelle auch dann, wenn das Kopieren
**gescheitert** ist. Der Kommentar an der Stelle behauptet das Gegenteil und ist an beiden
Haelften seiner Begruendung falsch.

## Zahlen

| Schwere | Zahl |
|---|---|
| Critical | 1 |
| High | 1 |
| Medium | 3 |
| Low | 2 |

Dazu **eine** Nutzerfrage als Entscheidungsdatensatz. Keine Doppelung zu den 194 offenen
Defekten: die vier, die denselben Code beruehren (`260825-1130`, `260825-1425`, `260825-1859`,
`260825-2127` in zwei Fassungen), sind gelesen und jeweils an der Stelle zitiert, an der sie an
einen neuen Befund grenzen.

## Befunde nach Thema

### Thema 1 — Ein Abbruchpfad, der etwas halb Getanes zurücklässt: die Quelle

**Critical.** `verschieben::ueber_datentraeger` (`crates/krk-core/src/operation/verschieben.rs:111-129`)
loescht die Quelle, sobald `kopieren_nach` nicht `Ablauf::Abgebrochen` liefert. Ein
**gescheitertes** Kopieren liefert aber `Ablauf::Weiter`, genau wie ein gegluecktes:

- Datei: `kopieren.rs:115-118` — `steuerung.ueberspringen(pfad, grund(&fehler)); Ablauf::Weiter`
- Ordner: `kopieren.rs:132-133` und `:138-140` — dasselbe

Danach laeuft `loeschen::baum_entfernen(quelle.pfad)` (`loeschen.rs:101-110`) unbedingt. Fuer eine
Datei ist das ein `fs::remove_file`, fuer einen Ordner ein rekursiver Abstieg, der jedes Kind
wegraeumt. Der Kommentar an `verschieben.rs:120-122` sagt: „Ist beim Kopieren etwas uebersprungen
worden, steht es noch in der Quelle, und `baum_entfernen` scheitert daran." Beides trifft nicht
zu — die Quelle steht da, weil sie nie angefasst wurde, und `baum_entfernen` scheitert an einem
vollen Ordner nicht, sondern leert ihn erst.

Erreichbar bei jedem Abwurf auf ein anderes Volume, bei `F6` in ein Dateifenster auf einem
anderen Datentraeger und bei jedem Abwurf aus einer fremden Anwendung. Die gewoehnlichsten
Ausloeser stehen im Uebersetzer `grund` (`mod.rs:476-484`): keine Rechte, kein Platz, Ziel
verschwunden. **Keine Probe im Baum beruehrt den Weg**: `crates/krk-core/tests/operation.rs`
kennt kein `EXDEV`.

Datensatz: `shared/issues/260826-1221_*_ein-gescheitertes-kopieren-ueber-die-datentraegergrenze-loescht-die-quelle-trotzdem.md`

*Gepruefet durch Lesen, Zeile fuer Zeile. Nicht am laufenden Geraet nachgestellt: dafuer braeuchte
es zwei Datentraeger.*

### Thema 2 — Die eine Stelle, an der die Vollstaendigkeit über `Art` nicht hält

**High.** `CLAUDE.md` haelt fest, dass Fallunterscheidungen ueber `Art` bewusst ohne
Auffangzweig stehen, damit eine neue Variante den Bau anhaelt. Nachgezaehlt gegen den Baum:
**neun** Verzweigungen ueber `Art` sind vollstaendig, **zwei** nicht.

Vollstaendig: `ausfuehren` (`mod.rs:189-196`), `einen_abarbeiten` (`mod.rs:240-283`, mit einem
eigens erlaeuterten toten Zweig `Art::Zippen`), `zielordner` (`auftrag.rs:226-234`),
`schiebt_auffrischung_auf` (`krk-ui/src/auffrischung.rs:332-341`), zwei Stellen in
`krk-ui/src/kommandos/operationen.rs:433-438` und `:484-489`, zwei in
`krk-ui/src/appkit/anwendung.rs:522-532` und `:6605-6647`.

Nicht vollstaendig: `Auftrag::neuer_name` (`auftrag.rs:178-183`) und `Auftrag::entpackziel`
(`auftrag.rs:186-191`), beide mit `_ => None`. Das sind gerade die zwei, die eine **Angabe je
Stelle** liefern — eine siebte Art derselben Bauart uebersetzte, liefe durch und meldete je
Eintrag „es fehlt der neue Name".

Datensatz: `shared/issues/260826-1221_*_zwei-verzweigungen-ueber-art-tragen-einen-auffangzweig-und-halten-den-bau-nicht-an.md`

### Thema 3 — Namensvergleiche, die das Dateisystem anders beantwortet

**Medium.** `kollision::pruefen` (`stapelumbenennen/kollision.rs:78-101`) vergleicht bytegenau,
ueber `HashSet<&str>` und `HashMap<&str, usize>`. Der Datentraeger, auf dem KRK laeuft, tut das
nicht. Am 260826 in einem Wegwerfordner gemessen:

- `renamex_np("a.txt", "B.TXT", RENAME_EXCL)` neben einem vorhandenen `b.txt` → `rc=-1,
  errno=17 (File exists)`. Die Vorschau hatte keine Kollision gemeldet.
- Ein als NFD angelegter Umlautname ist unter der NFC-Form auffindbar (`os.path.exists` →
  `True`, `os.listdir` liefert NFD).

**Es geht nichts verloren**: die Ausfuehrung nimmt `RENAME_EXCL` (`umbenennen.rs:95` →
`sys.rs:764-775`) und weist den Eintrag ab; im selben Lauf nachgesehen, dass der Inhalt der
getroffenen Datei unangetastet bleibt. Kaputt ist die Zusage der Vorschau, die der Modulkopf
`vorschau.rs:3-5` „das, was das Umbenennen im Stapel ungefaehrlich macht" nennt, und die Zahl in
`Vorschau::kollisionen`.

Dieselbe Wurzel steht als offener Datensatz an einer **anderen** Stelle:
`shared/issues/260825-1425_*_der-schnitt-sieht-einen-zerlegt-geschriebenen-umlaut-...`, dort am
Schnitt des Packziels in `krk-ui`. Wer eine Antwort baut, sieht beide Stellen zugleich an.

Datensatz: `shared/issues/260826-1221_*_die-kollisionspruefung-vergleicht-bytegenau-und-uebersieht-jede-kollision-in-schreibweise-und-normalform.md`

### Thema 4 — Zusagen, die weiter reichen als der Code

**Medium, zwei Befunde.**

`Uebertragungsart::ImmerBytes` (`sys.rs:699-702`) setzt allein `COPYFILE_ALL`. `COPYFILE_EXCL`
kommt ueber `COPYFILE_CLONE` herein (`sys.rs:473-476`), steht also in diesem Zweig nicht da. Der
Doc-Kommentar an `datei_kopieren` (`sys.rs:638-639`) sagt ohne Einschraenkung „ein vorhandenes
Ziel laesst den Aufruf scheitern". Die Wahl faellt im Umfang dieser Durchsicht:
`Auftrag::mit_uebertragung` (`auftrag.rs:210-214`) ist oeffentlich. Heute waehlt die Anwendung sie
nirgends — `grep` findet ausserhalb von `sys.rs` vier Stellen, zwei davon Proben. Latenter Fall.

`freier_name` (`umbenennen.rs:141-158`) liefert nach 1.000 Versuchen genau den Namen zurueck, den
es eben als belegt vorgefunden hat, und heisst „freier Name". Der Doc-Kommentar an
`HOECHSTE_KOPIE` begruendet die Grenze mit „falls das Dateisystem jeden Namen als vorhanden
meldet" — und in dieser Lage antwortet die Funktion mit dem einen Namen, von dem sie weiss, dass
er vorhanden ist. Der offene Datensatz `260825-1130` schreibt ausdruecklich, bei
`AutomatischUmbenennen` „haelt die Zusage"; dieser Befund berichtigt seine Voraussetzung.

Datensaetze: `shared/issues/260826-1221_*_die-zweite-uebertragungsart-verliert-copyfile-excl-...`
und `shared/issues/260826-1221_*_der-freie-name-gibt-nach-tausend-versuchen-einen-belegten-namen-heraus.md`

### Thema 5 — Der eine expect

**Medium.** `operation::starten` (`mod.rs:158-165`) bricht mit `expect` ab, wenn der Arbeitsfaden
nicht startet. `thread::Builder::spawn` liefert ein `io::Result` gerade fuer diesen Fall; das
System liefert `EAGAIN`, wenn die Fadengrenze erreicht ist. Gerufen wird vom Hauptfaden. Es ist
der einzige `expect` ausserhalb der Pruefmodule in beiden Modulen — dieselbe Datei uebersetzt
jeden anderen Systemfehler in eine Zeile der Abschlussliste.

Datensatz: `shared/issues/260826-1221_*_der-arbeitsfaden-einer-dateioperation-wird-mit-expect-gestartet-und-reisst-die-anwendung-mit.md`

### Thema 6 — Altlasten

**Low, zwei Befunde.**

`#[must_use]` steht in beiden Modulen an vierzehn Stellen, alle in drei Dateien (`zippen.rs`
sieben, `mod.rs` vier, `auftrag.rs` zwei). In den uebrigen **elf** Dateien steht sie nirgends —
auch nicht an `Vorschau::auszufuehren` (`vorschau.rs:64-66`), deren fallen gelassener Iterator
buchstaeblich nichts tut, und die zugleich die eine Auskunft darueber ist, was die Ausfuehrung
anfasst.

Fuenf oeffentliche Namen ohne Rufer in der Anwendung: `MELDEABSTAND` (`fortschritt.rs:50`,
weitergereicht in `mod.rs:81`), `HOECHSTE_STELLENZAHL` (`regel.rs:38`, weitergereicht in
`stapelumbenennen/mod.rs:59`), `Regel::ist_wirkungslos` (`regel.rs:97-99`, einziger Rufer die
eigene Probe), `Lauf::warten` (`fortschritt.rs:249-253`, zehn Rufer, alle in
`tests/operation.rs`) und `Abschluss::ist_abgebrochen` (`fortschritt.rs:64-66`), das im ganzen
Arbeitsbereich **keinen** Rufer hat, nicht einmal eine Probe. Der letzte traegt zugleich denselben
Namen wie eine Methode eines anderen Typs in `verzeichnis/leser.rs:69` — die Sorte Doppelname, die
`stapelumbenennen/mod.rs:41-52` als eigenen Umbenennungsgrund festhaelt.

Datensaetze: `shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-...` und
`shared/issues/260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-...`.
Die parallele Durchsicht von `verzeichnis/` hat denselben `#[must_use]`-Befund fuer ihren Umfang
gefunden und die andere Haelfte des Doppelnamens; beide Datensaetze tragen eine `Also seen`-Zeile
auf den jeweils anderen.

### Thema 7 — Eine Nutzerfrage, die seit dem 260825 unaufgeschrieben offen steht

`ziel_klaeren` (`mod.rs:431-441`) ruft im Zweig `Ueberschreiben` weiterhin
`loeschen::baum_entfernen`, also endgueltiges Loeschen. `zippen` und `entpacken` nehmen seit dem
260825 den Papierkorb. Dieselbe Schaltflaeche desselben Blattes bedeutet damit zweierlei. Der
geschlossene Datensatz `260825-0942` benennt die Frage selbst als Nutzerfrage und laesst sie
offen; ein Datensatz dafuer gab es nicht, und damit fiel sie aus jeder Suche nach aktiver
Grundlage heraus.

Datensatz: `shared/decisions/260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`

## Was gehalten hat

Ausdruecklich geprueft und **ohne** Befund:

- **Zip und Unzip sind keine zweite Maschine.** Beide rechnen ueber dieselbe `Steuerung`, rufen
  dasselbe `konflikt_loesen`, melden ueber `zwischenstand`/`eintrag_fertig`/`ueberspringen` und
  pruefen `abgebrochen` innerhalb der einzelnen Datei (`zippen.rs:458`, `entpacken.rs:517`).
  `Art::Zippen` und `Art::Entpacken` sind zwei Werte derselben Aufzaehlung, und `zippen::lauf`
  bekommt den `Papierkorb` gereicht wie jede andere Bahn.
- **Die Frage nach dem gueltigen Ziel steht beim Aufrufer, nicht in der Huelle.** Das Packen
  fragt `metadata()` am offenen Deskriptor (`zippen.rs:406-416`) und schreibt „keine
  gewoehnliche Datei" in die Abschlussliste; das Entpacken fragt gar nicht, sondern reicht den
  Deskriptor an `ZipArchive::new` (`entpacken.rs:144-154`) und uebernimmt den Wortlaut der Kiste.
  Beide Wege verhalten sich genau so, wie `CLAUDE.md` sie beschreibt.
- **`schiebt_auffrischung_auf`** (`krk-ui/src/auffrischung.rs:332-341`) fuehrt alle sechs Arten
  auf und hat keinen Auffangzweig.
- **Die Merkmalswahl der Kiste `zip` wird gebraucht.** `unreserved` traegt genau eine Zeile:
  `wahl.add_extra_data(FELD_INFOZIP_UNIX, …)` (`zippen.rs:682-689`); ohne das Merkmal weist die
  Kiste die Kennung `0x5855` ab. Das Zeitfeld kommt formgerecht an: `0x5455` traegt Kennzeichen
  `0b11` plus vier Byte Aenderung plus vier Byte Zugriff, `0x5855` traegt Zugriff vor Aenderung,
  je vier Byte — beides die Reihenfolge, die `entpacken::infozip_unix_zeit` (`entpacken.rs:387-401`)
  wieder herausliest. Die zwei Enden sind ein Zug.
- **Kein Weg am Papierkorb vorbei, der Nutzerdaten faenge.** Die drei `fs::remove_file`-Stellen
  in `zippen.rs:319`, `kopieren.rs:94` und `entpacken.rs:535` raeumen alle einen Rest weg, den
  KRK im selben Lauf selbst angelegt hat. `baum_entfernen` hat zwei Rufer, und beide sind in
  Thema 1 und Thema 7 einzeln behandelt.
- **`#![deny(unsafe_code)]` haelt**: keine Zeile `unsafe` und kein `allow` in den vierzehn
  Dateien.
- **Zwei Sperren gegen ein feindliches Archiv, beide vorhanden**: `enclosed_name`
  (`entpacken.rs:267`) und `kette_anlegen` (`entpacken.rs:431-460`), letztere mit `lstat(2)` je
  Ebene und ohne Auffangzweig ueber `Component`.
- **`namen_teilen` steht genau einmal** (`umbenennen.rs:177-182`), und die Probe
  `die_trennung_stimmt_mit_der_trennung_von_path_ueberein` haelt sie gegen die
  Standardbibliothek.

## Querschnitt

Drei Muster liegen quer zu den Themen:

1. **Ein Rueckgabetyp mit zwei Werten, wo drei gebraucht wuerden.** `Ablauf` kennt `Weiter` und
   `Abgebrochen`; der **Fehlschlag** verschwindet in `Weiter`. Genau daran haengt der Critical
   aus Thema 1. `zippen::Packschritt` (`zippen.rs:175-184`) hat dasselbe Problem gehabt und drei
   Werte bekommen, mit ausgeschriebener Begruendung. Die Loesung steht also schon im selben
   Modul; sie ist nur nicht auf `Ablauf` uebertragen worden.
2. **Zwei Ungleichheiten stammen aus derselben Runde.** Die Runde 17 hat Zip und Unzip auf den
   Papierkorb gestellt (Thema 7) und `Art` um zwei Varianten erweitert, die als einzige eine
   Angabe je Stelle fuehren (Thema 2). Beide Befunde sind Reste desselben Umbaus: er ist an der
   neuen Stelle sauber gefuehrt und an der alten daneben nicht nachgezogen.
3. **Die Sorgfalt haengt an der Datei, nicht an der Regel.** `zippen.rs` traegt sieben
   `#[must_use]`, `entpacken.rs` keines; `mod.rs` begruendet jeden `None`-Fall von `zielordner`
   einzeln und laesst zwei Zeilen darueber zwei Auffangzweige stehen. Beides sind Anzeichen
   dafuer, dass die Regeln beim Schreiben angewandt werden und nicht beim Bauen erzwungen.

## Reihenfolge

**Vor der naechsten Auslieferung:**

1. Thema 1 (Critical) — Datenverlust, und der Weg hat keine Probe. Der Schnitt ist derselbe wie
   bei `Packschritt`: `Ablauf` braucht einen dritten Wert.
2. Thema 2 (High) — zwei Zeilen, und danach haelt die Zusage aus `CLAUDE.md` wieder ohne
   Ausnahme.

**Danach, in dieser Reihenfolge:**

3. Thema 7 (Nutzerfrage) — sie blockiert nichts, bindet aber jede spaetere Arbeit am
   Konfliktzweig, und ihre Antwort entscheidet, ob `baum_entfernen` einen Rufer behaelt.
4. Thema 3 und Thema 5 (Medium).
5. Thema 4 (Medium, latent) — kein Fehlverhalten am laufenden Buendel, solange kein Rufer
   `ImmerBytes` waehlt.
6. Thema 6 (Low) — Aufraeumen, am besten in einem Zug mit der Behebung von Thema 1, weil beide
   dieselben Dateien anfassen.
