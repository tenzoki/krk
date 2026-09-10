# Coder-Sitzung: Schritt 4, die Erhebung läuft beim Start

**Date:** 2026-09-10, 260910-1600
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 4
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

Die drei Stücke aus den Schritten 1, 2 und 3 sind zusammengeführt. Beim Start liest KRK den
Merker, hält ihn gegen `env!("CARGO_PKG_VERSION")`, erhebt bei Ungleichheit den Unterschied
an den drei von Hand gepflegten Ablagedateien, schreibt den Merker und hängt die Startzeile
an die Meldungen des Starts. Bei Gleichheit geschieht nichts weiter — keine der drei Dateien
wird geöffnet.

### `crates/krk-ui/src/appkit/anwendung.rs`

**Die freie Funktion `neuerungen_erheben(&Zugang) -> (Option<Bestand>, Vec<String>)`.** Sie
steht als freie Funktion und nicht als Methode am Delegierten, weil sie an einem `Zugang`
hängt und an nichts sonst: so ist sie ohne AppKit, ohne Fenster und ohne Hauptfaden
prüfbar, und die vier Proben dieses Schrittes rufen genau sie.

Ihr Rumpf ist die ganze Logik des Schrittes:

- `merker::laden(zugang).mit_meldung()` — die `Ersetzung` wird weitergereicht und nicht
  verschluckt; der Kopf von `krk_core::ablage::merker` trägt den Grund.
- `merker.meldung_steht_aus(LAUFENDE_FASSUNG)` — sagt er nein, kommt `(None, meldungen)`
  zurück, und keine Datei ist geöffnet worden.
- sonst `neuerungen::erheben(zugang)`, dann `merker::vermerken`, dann
  `neuerungen::startzeile(&bestand, pfade::benutzerverzeichnis().as_deref())`.

**`None` heißt „nicht erhoben" und nicht „kein Unterschied".** Ein erhobener Bestand ohne
Unterschied kommt als `Some` zurück und trägt die vollen Pfade, die das Blatt aus Schritt 6
in jedem Fall zeigt. Der `ivar` hält die zwei Auskünfte deshalb auseinander, und der
Doc-Kommentar am Feld schreibt es aus.

**Der Merker wird auch dann geschrieben, wenn nichts zu melden war.** Sonst erhöbe jeder
Start derselben Fassung von neuem, und „einmal je Fassung" wäre für den häufigsten Fall —
es gibt keinen Unterschied — gerade nicht eingelöst.

**Ein gescheitertes Vermerken geht als Meldung hinaus.** Die Folge ist nicht, dass die
Erhebung fehlschlüge, sondern dass sie sich bei jedem Start wiederholt. Still wäre das der
eine Fall, in dem der Nutzer die Wiederholung für einen Defekt der Meldung hielte.

**Der Aufruf steht im vorhandenen Durchgang von `sitzung_laden` und dort als Letztes.** Die
Reihenfolge ist tragend: `einstellungen::laden` und `leseprofile::laden` legen
`settings.toml` und `readers.toml` beim ersten Start an. Wer davor erhöbe, verglichen gegen
zwei Dateien, die es in dieser Sekunde noch nicht gibt.

**Der neue `ivar` `neuerungen: RefCell<Option<Bestand>>`** wird nach den Profilen gesetzt,
die Meldungen danach angehängt. Damit steht die Startzeile als letzte in `meldungen` und
läuft durch dieselbe Tür wie jede andere Startmeldung — `startmeldungen::auskunft` aus
Schritt 2 entscheidet Statuszeile oder Blatt.

**Zwei Prosastellen sind mitgezogen.** Der Kommentar über dem Durchgang sagte „Ein Durchgang
für alle drei Dateien"; das ist mit der Erhebung falsch geworden und nennt jetzt die Stücke
statt einer Zahl. Die Bindung `Ok(alle_drei)` heißt `Ok(alles)`.

**Der Doc-Kommentar von `sitzung_laden` bekommt einen Absatz über den Messmodus:** dort
fällt die Erhebung mit dem ganzen Durchgang weg, weil jede der vier Aufgaben vorher
zurückkehrt. Kein Messlauf sieht also, was sie kostet — die Zusage aus dem L4-Datensatz
hängt an der Zählprobe und nicht an einer Messstrecke, und das steht jetzt dort, wo es
jemand liest.

### Die Proben, und wie die Öffnungen gezählt werden

`mod neuerungsproben` in derselben Datei, vier Stück.

**Das Messmittel.** Eine gelungene Lesung hinterlässt nichts, an dem sie abzulesen wäre. Eine
**beschädigte** Datei dagegen legt `Zugang::laden` beim Öffnen zur Seite, und die
beiseitegelegte Fassung steht danach auf der Platte (`atomar::beiseitepfad`). Die drei
verglichenen Dateien stehen in den zwei Zählproben deshalb absichtlich als kaputtes TOML da:
jede Öffnung schreibt genau eine Nachbardatei, und die Zahl dieser Nachbardateien **ist** die
Zahl der Öffnungen. Der Inhalt braucht beides — kein gültiges TOML **und** einen obersten
Schlüssel —, denn ohne obersten Schlüssel nimmt `laden` den anderen Zweig und legt nichts
zur Seite; dann zählte das Messmittel eine Öffnung nicht mit, die stattgefunden hat.

- `bei_gleichem_merker_wird_keine_der_drei_dateien_geoeffnet` — erwartet null. Das ist die
  Bedingung aus dem L4-Datensatz, gemessen und nicht zugesichert.
- `bei_neuer_fassung_werden_die_drei_dateien_geoeffnet` — erwartet drei, mit derselben
  Zeile gezählt. **Sie ist die Eichung und keine Zugabe:** null Nachbardateien misst auch
  ein Zähler, der nichts sieht.
- `der_zweite_start_derselben_fassung_meldet_nichts` — zwei Läufe an einer Ablage. Der erste
  erhebt und vermerkt, der zweite findet den Vermerk vor.
- `die_erhebung_steht_im_durchgang_hinter_den_zwei_ladern` — eine Quelltextprobe: im Rumpf
  von `sitzung_laden` steht `neuerungen_erheben(zugang)` hinter beiden Ladern. Der Übersetzer
  hält eine Reihenfolge nicht, und für ihn sind die zwei Aufrufe unabhängig.

`reported.toml` wird in beiden Zählfällen geöffnet und ist keine der drei — ohne sie gäbe es
die Frage nicht, die dort entschieden wird.

### `crates/krk-core/tests/ablage.rs`

`auf_einer_frischen_installation_meldet_der_erste_start_keine_neuerung`. Ein frischer
Prüfordner, dann in **einem** Durchgang `einstellungen::laden`, `leseprofile::laden`,
`neuerungen::erheben` — die Reihenfolge des Starts. Geprüft: `keymap.toml` trägt
`Befund::Fehlt`, `settings.toml` und `readers.toml` tragen `Befund::Verglichen`, der Bestand
trägt keinen Unterschied, und `startzeile` liefert `None`.

## Abweichung von der Dateiliste des Schrittes

**`crates/krk-core/tests/ablage.rs` ist dazugekommen, und eine vorhandene Zählprobe in
`anwendung.rs` ist mitgezogen.**

Das vierte Abnahmekriterium verlangt, dass die Erhebung hinter den zwei Ladern steht. Eine
Probe, die das *bewirkt*, muss beide Lader rufen — und
`leseprofilproben::die_leseprofile_werden_im_baum_genau_einmal_geladen` zählt die Aufrufform
`leseprofile::laden(` und verlangt genau einen Rufer.

**Ein Irrtum unterwegs, und er hat einen roten Lauf gekostet.** Ich hielt die Probe für eine
über den Quellbaum von `krk-ui`, also für eine, der die Prüfziele des Kerns entgehen, und
habe das Kriterium deshalb nach `krk-core/tests/ablage.rs` gelegt. `quellbaum::quelldateien`
liest aber **jede** `.rs`-Datei unter `crates/`, Prüfziele eingeschlossen; der erste
`make check` endete mit `left: [("krk-core/tests/ablage.rs", 1), ("krk-ui/src/appkit/
anwendung.rs", 1)]`. Der Umzug hat das Problem verschoben und nicht gelöst.

Behoben ist die Wurzel und nicht die Zahl. Die Probe zählt seither den **Betriebscode**:
jede Datei unter einem `src/`, und dort alles vor dem ersten `#[cfg(test)]` — dieselbe
Schnittstelle, die `vorschaumodell::tests::zusammenfassen_hat_einen_rufer_…` in dieser Kiste
schon benutzt. Das ist keine Aufweichung, sondern die Einengung auf das, was die Zusage aus
C4.5 überhaupt behauptet: KRK liest `readers.toml` einmal **beim Start**. Ein Ruf aus einem
Prüfziel läuft in keinem Start. Ein zweiter Rufer im Betriebscode macht die Probe weiter rot,
gleich in welcher Datei er steht.

Der Ausweg, den Aufruf in eine Form zu schreiben, die die Probe nicht sieht — `let laden =
leseprofile::laden;` und dann `laden(zugang)` —, ist verworfen: das wäre die Zahl richtig
gemacht und nicht die Sache.

Dass die Prüfung dieses Kriteriums im Kern liegt, bleibt richtig und folgt der Testing
Strategy des Plans: „der Kern trägt die Last, weil er ohne AppKit prüfbar ist". Die
Reihenfolge im Code selbst hält daneben die Quelltextprobe in `anwendung.rs`.

## Was offen bleibt

**Der `ivar` steht bei fast jedem Start auf `None`, und Schritt 9 hat dafür keine Antwort.**
Der Schritt sagt „das Blatt mit dem gehaltenen `Bestand`"; gehalten ist beim zweiten und
jedem weiteren Start derselben Fassung keiner, weil dann nichts erhoben wird. Das ist kein
Defekt an diesem Schritt — die Sparsamkeit ist die Bedingung, unter der der L4-Datensatz
beantwortet ist —, sondern eine Frage an den Nutzer:
`260910-1600_*_was-zeigt-das-blatt-auf-abruf-wenn-der-start-nichts-erhoben-hat.md`.
Empfohlen ist dort, in Schritt 9 auf Verlangen zu erheben: der Nutzer hat gefragt, also
kostet das Lesen kein L4.

`blaetter::neuerungen::zeigen` trägt weiter sein befristetes `#[expect(dead_code, …)]` aus
Schritt 6; der Ausführungszweig ist Schritt 9 und war nicht Auftrag dieser Sitzung.

## Abnahme

- `cargo test -p krk-ui neuerungsproben` — 4 von 4 grün.
- `make check` — **endet mit 0**, alle fünf Kommandos, in seiner Reihenfolge.

**Zwei rote Läufe davor, beide aufgeschrieben und nicht weggeräumt.** Der erste war der
Zählproben-Irrtum oben (`make check` Rückgabewert 2, `make: *** [test] Error 101`). Der
zweite war die Reihenfolge der drei `use krk_core::ablage::…`-Zeilen, die `cargo fmt --all
--check` beanstandet hat (Rückgabewert 2, `make: *** [fmt-check] Error 1`); behoben durch
Umstellen auf die alphabetische Ordnung, die `rustfmt` verlangt.
