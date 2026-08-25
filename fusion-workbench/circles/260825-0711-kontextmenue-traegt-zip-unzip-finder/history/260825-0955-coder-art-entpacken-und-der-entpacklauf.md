# Coder: `Art::Entpacken` und der Entpacklauf, und drei Vorgaben, die anders halten

**Datum:** 2026-08-25 09:55
**Status:** Complete
**Agent:** coder
**Baumstand:** `b0841ba` plus die Änderungen dieses Schritts

## Auftrag

Schritt 3 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`:
`Art::Entpacken`, das neue Modul `operation/entpacken.rs`, der Zweig in `einen_abarbeiten`, die
vier Stellen in `krk-ui`, und Proben in `crates/krk-core/tests/operation.rs` über den
öffentlichen Weg `operation::starten`.

Zwei beantwortete Datensätze binden den Schritt: der Zielordner-Konflikt geht über
`steuerung.konflikt_loesen`, und „überschreiben" räumt den vorhandenen Ordner in den
**Papierkorb** (`decisions/260825-0711_*_was-tut-unzip-wenn-der-zielordner-schon-dasteht.md`,
Möglichkeit 2); Unzip wirkt auf die betroffenen Einträge und entpackt **jedes** Archiv darin
(`decisions/260825-0727_*_nimmt-unzip-die-betroffenen-eintraege-*`, Möglichkeit 3).

## Drei Vorgaben des Schritts halten nicht, wie sie geschrieben stehen

**Erstens: `Art::Entpacken` trägt eine Liste von Zielen und keinen einzelnen Pfad.** Der Plan
schreibt `Entpacken { ziel: PathBuf }` und dazu „`quellen` trägt genau ein Archiv". Die
Nutzerentscheidung vom 260824-2120 ist jünger und geht vor: drei markierte Archive ergeben drei
Zielordner in **einem** Vorgang. Ein einzelner Zielpfad reichte dafür nicht, und ein Auftrag je
Archiv wäre drei Vorgänge, drei Fortschrittsanzeigen und drei Abbruchtasten für eine Handlung —
und das Ankreuzfeld „für alle weiteren" der `Konfliktregel` wirkte über die Grenze eines Vorgangs
hinweg gar nicht, obwohl die Entscheidung es ausdrücklich vorsieht.

Gewählt ist die Form, die der Baum für genau diese Lage schon trägt, nämlich die des
Stapel-Umbenennens: `Entpacken { ziele: Vec<PathBuf> }`, Stelle für Stelle zu `quellen`;
`Auftrag::entpacken(paare)` nimmt Paare entgegen und trennt sie auf, damit ein Aufrufer die zwei
Listen gar nicht erst gegeneinander verschieben kann; `entpackziel(stelle)` liest sie zurück, wie
`neuer_name(stelle)` es tut. `einen_abarbeiten` bekommt die Stelle bereits übergeben — sie lief
bisher allein für das Stapel-Umbenennen mit —, es kam also keine Signatur und kein Mechanismus
hinzu. Wie ein Ziel je Archiv **heißt**, bleibt Schritt 4; der Kern bekommt die fertigen Pfade.

**Zweitens: das Archiv wird geöffnet, bevor der Zielordner geklärt wird.** Der Plan nennt die
umgekehrte Reihenfolge, und beim Packen ist sie richtig: dort schnitte `File::create` die
vorhandene Datei ab, also muss die Rückfrage davor stehen. Hier räumt die Antwort „überschreiben"
einen ganzen **Ordnerbaum** in den Papierkorb. Stünde sie vor dem Öffnen, hätte der Nutzer bei
einer Datei, die sich als Archiv nicht öffnen lässt, seinen Ordner für nichts hergegeben: erst
die Rückfrage, dann der Papierkorb, dann die Meldung „kein Archiv". Das Öffnen schreibt kein
Byte, und die Zusage des Datensatzes — die Rückfrage kommt, **bevor ein Eintrag geschrieben
wird** — hält unverändert.

**Drittens: `enclosed_name` allein versperrt nicht jeden Weg aus dem Zielordner.** Der Plan nennt
es als die Prüfung, und für den Namen eines einzelnen Eintrags ist es das auch. Über **zwei**
Einträge führt aber ein zweiter Weg hinaus, an dem es nichts auszusetzen hat: der erste heißt
`hinaus` und ist eine Verknüpfung auf `..`, der zweite heißt `hinaus/draussen.txt`. Beide Namen
liegen für sich genommen im Zielordner, `enclosed_name` sagt zu beiden ja, und geschrieben würde
neben den Zielordner.

Versperrt ist er von `kette_anlegen`: jeder Ordner auf dem Weg zu einem Eintrag muss ein
**wirklicher** Ordner sein, geprüft über `fs::symlink_metadata` und nicht über `fs::metadata`,
denn ein `stat(2)` sähe bei `hinaus -> ..` einen Ordner und ließe den Weg durch. Der Preis ist
ein `lstat(2)` je Ordnerebene und Eintrag; neben dem Auspacken der Bytes fällt er nicht ins
Gewicht. Die Verknüpfung selbst wird weiterhin angelegt, unverändert, wie `kopieren` und `zippen`
eine Verknüpfung unverändert weitergeben — sie schreibt nichts, und durch sie hindurch schreibt
der Lauf jetzt nicht mehr. Daneben steht eine zweite kleine Sperre: steht am Ziel eines
Dateieintrags schon eine Verknüpfung, wird er ausgelassen, statt dass `File::create` ihr folgt.

**Was `enclosed_name` mit einem führenden Schrägstrich tut, ist der Erwähnung wert**, weil es
nicht das ist, was man annimmt: `/absolut.txt` wird nicht abgewiesen, sondern **abgestreift**,
und der Eintrag landet als `absolut.txt` im Zielordner (`zip-8.6.0/src/path.rs:43`, `RootDir` bei
Tiefe 0 ist zulässig). Das ist sicher, aber eben kein Auslassen; die Probe hält beide Formen
auseinander, damit ein späterer Leser nicht das eine für das andere hält.

## Was entstanden ist

**`crates/krk-core/src/operation/entpacken.rs`, neu.** Öffnen über
`verzeichnis::sys::ohne_warten_oeffnen` — dieselbe Begründung wie beim Packen, eine benannte
Röhre hielte ein `File::open` bis in alle Ewigkeit an —, dann `ZipArchive::new`, dann der
Zielordner, dann Eintrag für Eintrag: `enclosed_name`, `kette_anlegen`, und je nach Typ ein
Ordner, eine Verknüpfung oder eine Datei Stück für Stück mit Zwischenstand und Abbruchprüfung
**innerhalb** der Datei.

`zielordner_klaeren` behandelt alle vier `Konfliktantwort`-Werte, wie `zielarchiv_klaeren` es
tut. „Überschreiben" ruft die hereingereichte `Papierkorb`-Schnittstelle und **nicht**
`loeschen::baum_entfernen`, das der Zip-Lauf für seine einzelne Zieldatei nimmt; das ist die
Bindung aus Runde 12, und die Probe belegt sie von der stärkeren Seite: die Attrappe schreibt
nur mit, und danach steht der alte Inhalt noch da — der Kern hat also nicht selbst gelöscht.

**Ein vorhandener Zielordner ist hier ein Konflikt**, anders als bei `ziel_klaeren`, wo ein
Ordner auf einem gleichnamigen Ordner in den vorhandenen hineinwandert. Das Verschmelzen war
Möglichkeit 3 des Datensatzes und ist nicht gewählt worden.

**Die Ordnerrechte kommen zuletzt.** Ein Ordner, dessen Rechte beim Anlegen gesetzt würden, ließe
sich unter Umständen nicht mehr befüllen; dieselbe Reihenfolge und derselbe Grund wie in
`kopieren::ordnerangaben_uebernehmen`. Gesammelt wird `(Pfad, Modus)`, nachgetragen wird in
umgekehrter Reihenfolge, damit ein Ordner ohne Schreibrecht nicht seine eigenen Unterordner
sperrt. Die Rechte einer **Datei** stehen unmittelbar nach ihrem Inhalt; ohne sie wäre ein
ausführbares Skript nach dem Rundweg keines mehr.

**Nach einem Abbruch bleibt stehen, was schon entpackt ist**, und weggeräumt wird allein die
halbe Datei, an der der Abbruch traf. Das ist die Umkehrung des Packens, und der Grund gehört
dazu: ein halbes Archiv trägt kein Verzeichnis am Ende und lässt sich von keinem Werkzeug öffnen,
ein halb entpackter Ordner dagegen ist benutzbar, und ihn wegzuräumen wäre ein Löschen ohne
Auftrag.

**`auftrag.rs`:** der Wert `Entpacken { ziele }`, der Erzeuger `Auftrag::entpacken(paare)`, der
Leser `entpackziel(stelle)`, und `zielordner()` liefert `None` mit ausgeschriebenem Grund —
jede Quelle hat ihren eigenen Zielordner, einer davon wäre eine willkürliche Wahl.

**`mod.rs`:** `Art::Entpacken` läuft über `quelle_fuer_quelle`, `einen_abarbeiten` bekommt seinen
fünften Zweig. Der `None`-Fall von `entpackziel` ist behandelt und meldet, wie der entsprechende
Fall des Stapel-Umbenennens; die beiden Listen entstehen aus denselben Paaren und sind gleich
lang, aber ein leiser Ausfall hieße, ein Archiv stillschweigend auszulassen.

**Die vier Stellen in `krk-ui`:** Überschrift „Entpacken", kein Auffrischungsaufschub, kein
zusätzlicher Ordner (jeder Zielordner entsteht **in** dem einen angezeigten Ordner, der schon als
`quellordner` dasteht), und keine Auswahl nach dem Abschluss. Beim Entpacken kommt zum Grund des
Packens ein zweiter dazu: ein Vorgang legt möglicherweise mehrere Ordner an, und welcher von ihnen
die Auswahl bekäme, wäre willkürlich.

## Was der Schritt nicht nennt und trotzdem dasteht

**`STUECK` steht jetzt an einer Stelle statt an zweien.** Die 64 KiB zwischen zwei
Abbruchprüfungen standen als private Konstante in `zippen.rs`; das Entpacken braucht dieselbe
Zahl. Sie ist nach `operation/mod.rs` gewandert, `zippen.rs` liest sie von dort. Eine zweite
Konstante mit derselben Begründung wäre zwei Wahrheiten über eine Zahl gewesen.

**`Ablauf` trägt `#[must_use]`.** Ein fallen gelassenes `Abgebrochen` bliebe unbemerkt: der Lauf
liefe über die abgebrochene Position hinaus weiter, und keine Warnung erschiene. Die Marke steht
am **Typ** und nicht an den vier Funktionen, die ihn liefern, und deckt damit auch jede spätere.
`cargo clippy --all-targets -- -D warnings` bleibt grün; jeder heutige Aufrufer wertet den Wert
ohnehin aus.

**Drei Doc-Stellen in `verzeichnis/sys.rs` waren falsch geworden** und sind nachgezogen. Die
Hülle `ohne_warten_oeffnen` hat seit Schritt 2 nicht mehr zwei Aufrufer, sondern mit diesem
Schritt vier; die Skizze im Modulkopf nennt jetzt alle vier, und die Zahl im Fließtext ist durch
den Zählbefehl ersetzt, der sie liefert, statt beim nächsten Aufrufer wieder falsch zu werden.
Der Abschnitt „Zwei Aufrufer, und die Zielprüfung bleibt bei beiden" heißt jetzt „Mehrere
Aufrufer …" und schreibt aus, dass die zwei Archivwege **anders** fragen als die zwei Textwege:
das Packen kennt den Typ seiner Quelle vor dem Öffnen, das Entpacken fragt gar nicht nach ihm und
lässt `ZipArchive::new` antworten. Das ist der beste Beleg dafür, dass die Typprüfung nicht in
die Hülle gehört. **`CLAUDE.md` trägt dieselbe veraltete Aussage** („Die Hülle hat zwei Aufrufer,
und beide liegen seit der Runde 11 in `krk-core/src/text/datei.rs`") und ist in diesem Schritt
nicht angefasst worden.

**Zwei Doc-Zählungen in `auffrischung.rs` waren mit Schritt 2 falsch geworden** und sind ohne neue
Zahl gefasst: „eine fünfte Operationsart" heißt jetzt „eine weitere", „Die übrigen drei schieben
nicht auf" heißt „Jede übrige Art schiebt nicht auf" und nennt sie.

## Das Änderungsdatum bleibt offen, und zwar absichtlich

Eine entpackte Datei trägt die Uhrzeit des Entpackens und nicht den Zeitstempel ihres
Archiveintrags. Der Defekt `issues/260825-0838_o_jeder-gepackte-eintrag-traegt-den-1-januar-1980-*`
nennt diese Gegenrichtung schon und ist um einen Nachtrag ergänzt. Sie ist **nicht** behoben, und
der Grund gehört dazu: solange der Packlauf jedem Eintrag den 1. Januar 1980 gibt, machte ein
Entpacken, das den Zeitstempel des Eintrags übernähme, aus jeder Datei eine von 1980 — aus einem
verlorenen Wert würde ein falscher. Wer den Datensatz abarbeitet, fängt am Packende an. Der
Modulkopf von `entpacken.rs` verweist darauf.

## Prüfung

`make check` läuft grün über den ganzen Workspace, **Exit-Code 0**: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings`.

Vierzehn neue Proben, zwölf in `crates/krk-core/tests/operation.rs` über den selbstabräumenden
Prüfordner aus `tests/gemeinsam/mod.rs` und den öffentlichen Weg `operation::starten`, zwei in
`auftrag.rs`:

| Probe | Was sie hält |
|---|---|
| `ein_archiv_wird_in_seinen_ordner_entpackt_und_jeder_eintrag_steht_da` | Ordner, Dateien, der leere Ordner, die gezählten Bytes |
| `was_krk_packt_kommt_beim_entpacken_unveraendert_wieder_heraus` | der Rundweg gegen den Packlauf aus Schritt 2 |
| `ein_eintrag_der_aus_dem_zielordner_herausfuehrt_entsteht_nirgends` | **die geforderte Probe**: `../draussen.txt` und `../../weiter/weg.txt` entstehen nirgends, `/absolut.txt` landet im Zielordner, `drin.txt` läuft durch, beide Gründe nennen den Ausbruch mit Namen |
| `ein_eintrag_hinter_einer_verknuepfung_schreibt_nicht_aus_dem_zielordner_heraus` | der zweite Ausbruchsweg, über zwei Einträge |
| `eine_verknuepfung_im_archiv_wird_wieder_eine_verknuepfung` | `symlink` statt gewöhnlicher Datei, Verweisziel unverändert |
| `ein_ausfuehrbarer_eintrag_bleibt_ausfuehrbar` | 0o755 überlebt |
| `ein_vorhandener_zielordner_wird_einmal_und_vor_dem_ersten_eintrag_erfragt` | genau eine Frage je Archiv, gefragt wird über das Archiv nach dem Zielordner, und der alte Inhalt steht danach unverändert da |
| `mehrere_archive_in_einem_vorgang_bekommen_je_ihren_eigenen_zielordner` | die Nutzerentscheidung Möglichkeit 3 |
| `ueberschreiben_raeumt_den_vorhandenen_zielordner_in_den_papierkorb` | die Bindung aus Runde 12: die Attrappe bekommt den Ordner, und der Kern hat nichts selbst gelöscht |
| `ueberspringen_laesst_den_vorhandenen_zielordner_stehen` | Antwort 2 von 4, und es wird nicht hineinentpackt |
| `umbenennen_legt_den_zielordner_daneben` | Antwort 3 von 4 |
| `eine_datei_die_kein_archiv_ist_wird_gemeldet_und_die_uebrigen_laufen_durch` | der Wortlaut der Kiste in der Abschlussliste, kein Ordner für die Datei, das zweite Archiv kommt heraus |
| `ein_abbruch_beim_entpacken_laesst_das_fertige_stehen_und_raeumt_die_halbe_datei_weg` | die fertige Datei bleibt, die halbe geht |
| `ein_fehlendes_archiv_wird_gemeldet_und_die_uebrigen_werden_entpackt` | C4: eine gescheiterte Position hält den Stapel nicht auf |
| `ein_entpackauftrag_traegt_die_ziele_stelle_fuer_stelle_zu_den_archiven` | die Paare, `entpackziel`, `zielordner() == None` |
| `eine_andere_art_kennt_kein_entpackziel` | `entpackziel` antwortet nur für die eine Art |

Die Antwort „abbrechen" ist von `ein_vorhandener_zielordner_…` mitgeprüft; damit sind alle vier
`Konfliktantwort`-Werte belegt.

**Die Abbruchprobe hängt nicht an einer Wartezeit.** Sie liest die Meldungen mit und bricht auf
die erste Fortschrittsmeldung über die große Datei hin ab. Damit steht fest, dass der Lauf
wirklich in ihr steht, und die Probe misst nicht die Geschwindigkeit des Geräts. Das Archiv dafür
entsteht über `Auftrag::zippen` aus einer `rauschdatei` von 16 MB — kaum verdichtbare Bytes,
sonst wäre der Eintrag ausgepackt, bevor der Abbruch ihn erreicht.

Der Hilfsbau `archiv_bauen` baut Archive **von Hand** über `ZipWriter` und nicht über
`Auftrag::zippen`. Der Grund steht an ihm: die Proben brauchen Namen, die KRK selbst nie schriebe.
Ein Archiv ist eine fremde Datei, und geprüft wird, was mit einer fremden Datei geschieht.

## Was nicht Gegenstand war

Die Namensbildung des Zielordners (`ordnername_zum_archiv` und `entpackziel` in
`kommandos/kontextmenue.rs`, Schritt 4), das Kontextmenü selbst (Schritte 4 und 6), die
Ausführung beim Delegierten (Schritt 7) und die Kürzung des Konfliktblatts (Schritt 8). Das
Ankreuzfeld „für alle weiteren" braucht nichts Neues: die bestehende `Konfliktregel` trägt es,
und die Probe `fuer_alle_weiteren_aendert_die_regel_des_laufs` in `fortschritt.rs` hält es
unverändert.

Kein Commit. `Cargo.toml` und `Cargo.lock` sind unberührt geblieben.

## Berührte Dateien

- `crates/krk-core/src/operation/entpacken.rs` (neu)
- `crates/krk-core/src/operation/auftrag.rs`
- `crates/krk-core/src/operation/mod.rs`
- `crates/krk-core/src/operation/zippen.rs` (nur `STUECK` wandert)
- `crates/krk-core/src/verzeichnis/sys.rs` (nur Doku: die Zahl der Aufrufer)
- `crates/krk-core/tests/operation.rs`
- `crates/krk-ui/src/kommandos/operationen.rs`
- `crates/krk-ui/src/auffrischung.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_o_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md` (Nachtrag)
- `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md` (Schritt 3 auf `[DONE]`, Nachtrag)
