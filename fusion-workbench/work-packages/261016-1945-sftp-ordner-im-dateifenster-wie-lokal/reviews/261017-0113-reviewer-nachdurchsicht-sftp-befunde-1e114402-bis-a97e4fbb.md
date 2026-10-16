# Nachdurchsicht der elf sftp-Befunde, `1e114402..a97e4fbb`

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `1e114402..a97e4fbb`
**Not-opened:** none
**Domain:** code
**Work-item:** 261016-1945-sftp-ordner-im-dateifenster-wie-lokal
**Maßstab:** die elf Issues `261017-0004-*` dieses Pakets mit ihrer Abnahme, die Durchsicht `261017-0004-reviewer-sftp-paket-c461b3a1-bis-1e114402.md`
**Urteil:** revise

## Zusammenfassung

Alle elf Befunde sind am Code behoben. Jede Abnahme ist erfüllt oder ihre Abweichung im Issue begründet.

Die neue Ersatzlogik von „Überschreiben“ hat eine Lücke. Ein Zielordner, der sich nur zum Teil entfernen lässt, bleibt halb stehen, und der fertige Ersatz wird weggeräumt (Medium). Dazu kommen drei Befunde geringer Schwere.

Geprüft habe ich am Baum `a97e4fbb`:
- `cargo test -p krk-core --test fern --test fern_operation`: 29 und 14 Proben grün.
- Die Prüfmodule von `operation::fern` und `fern::gespraech`: 7 Proben grün.
- `krk-ui` mit dem Filter `loeschwarnung` und `pfadeingabe`: 47 Proben grün.
- `cargo clippy --workspace --all-targets`, `cargo fmt --all --check` und `cargo doc` mit `-D warnings`: ohne Befund.

Den ganzen `make check` und das Bündel habe ich nicht gefahren.

Die Dispatch verlangte außerdem die neun Dateien aus der `**Not-opened:**`-Liste der früheren Durchsicht. Gelesen habe ich ihre Änderungen in `c461b3a1..1e114402`; acht davon sind Code, die neunte ist `README.md`. Ein Befund ergab sich daraus nicht (Abschnitt „Nachgeholt“).

## Summen

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 3 |

## Die elf Befunde, je am Code

| Befund | Stand | Beleg |
|---|---|---|
| M1 `known_hosts` | behoben | `fern/adresse.rs` `rechnername_gueltig`; `rechnerschluessel.rs` `suchname_gueltig`, geprüft in `gescannt_aus` und vor dem ersten Byte in `anhaengen` |
| M2 `REALPATH` | behoben | `operation/fern.rs` `Stelle::derselbe_eintrag` → `Gleichheit`, `ersetzen_erlaubt`, `liegt_in` |
| M3 Konflikt, Zwischenname | behoben, mit neuem Befund N1 | `vorhandenes_ziel`, `kopie_ersetzt_ziel`, `umbenennen_ersetzt_ziel`, `tauschen` |
| M4 Längengrenze | behoben | `fern/gespraech.rs` `fehlerausgabe_lesen` über `fill_buf`/`consume`, 4096 Bytes je Zeile |
| M5 Papierkorb unbekannt | behoben | `kommandos/loeschwarnung.rs` `Vorstufe::PapierkorbUnbekannt`, Zweig in `appkit/anwendung.rs` |
| L1 `FAILURE` | behoben | `eintrag_verschieben` → `kann_eine_grenze_des_servers_heissen` → `ueber_die_grenze` |
| L2 Abbruch, Obergrenze | behoben | `fern_namen`, `fern_baum_entfernen`, `kinder_lesen` → `Err(Ablauf::Abgebrochen)` |
| L3 200 ms | behoben, ein Teil bewusst offen | `Fehlerfaden::abwarten`; der nicht lesende Server steht im Modulkopf als Grenze |
| L4 Namensregel | behoben | `fern/pfad.rs` `ist_einzelner_name`, gefragt in `verbinden`, `fern_eintrag`, `Stelle::liste` |
| L5 Zwischenablage | behoben | `appkit/tabelle.rs` `lokalen_pfad_anspringen`, `pfadeingabe::pruefen_fuer_fernen_tab` |
| L6 HowTo, Kommentare | behoben, ein Rest in N4 | `HowTo.md`, `belegung.rs`, `zulaessigkeit.rs`, `anwendung.rs` |

### Die Prüfpunkte der Dispatch im Einzelnen

**`known_hosts`.**
- `suchname` (`rechnerschluessel.rs:247`) ergibt `hostname`, `[hostname]:port` oder den `HostKeyAlias`.
- `suchname_gueltig` nimmt `[rechner]:port` mit Port 1 bis 65535 an. Eine IPv6-Adresse geht aus `ssh -G` ohne Klammern heraus und kommt bei Port 22 roh, sonst als `[::1]:2222` durch.
- Ein legitimer Server mit Port wird also nicht abgewiesen. Die Probe `ein_rechnername_mit_musterzeichen_kommt_nicht_nach_known_hosts` belegt `[::1]:2222` und `[example.test]:2222`.
- `anhaengen` hält jede Zeile vor dem Schreiben noch einmal dagegen.
- Am Rand: ein `HostKeyAlias` oder `Host`-Alias mit anderen Zeichen, etwa `+`, wird jetzt beim Annehmen abgewiesen. Das ist die sichere Seite.
- Am Rand: Rechnernamen mit Umlaut (IDN) gehen nur noch in Punycode. `HowTo.md` sagt das, die Fehlermeldung nicht (N4).

**`REALPATH`-Gleichheit.**
- Verglichen werden der Name und die aufgelösten Elternordner. Eine Verknüpfung an der Stelle des Ziels selbst ist ein eigener Eintrag. `tauschen` entfernt dann die Verknüpfung und nicht ihr Ziel, die Quelle bleibt.
- Ist der Befund unklar, wird nicht überschrieben (`ersetzen_erlaubt`).
- Harte Verweise und zwei Kennungen für einen Server nennt der Modulkopf als Grenze. Für das Verschieben über zwei Kennungen folgt daraus ein Datenverlust (N3).

**Ersatz unter Zwischennamen, Reste von `.krk-ersatz-*`.**
- Bei einem Abbruch räumt `datei_fuellen` die halbe Datei weg, danach `rest_wegraeumen` den Rest. Ein `NO_SUCH_FILE` dabei bleibt still.
- Bei einem unvollständigen Ersatz und bei `ZielBlieb` räumt ebenfalls `rest_wegraeumen` weg.
- Ein `.krk-ersatz-*` bleibt allein liegen, wenn das Wegräumen selbst scheitert, etwa bei einer Trennung, oder wenn der Prozess endet. Beides steht im Bericht, Ersteres mit dem falschen Text „nach dem Abbruch“ (N4).
- Die Probe `ueberschreiben_entfernt_das_ziel_erst_wenn_der_ersatz_vollstaendig_dasteht` prüft das Fehlen von Resten in beiden Richtungen.
- Neu und nicht gedeckt: das Entfernen eines Zielordners scheitert mittendrin (N1).

**`vorhandenes_ziel`.**
- Eine Regel mit vier Rufern. `namensgrund` und die Konfliktwege antworten jetzt gleich.
- Die Tafel `allein_ein_moeglicher_vergebener_name_ist_ein_konflikt` schreibt jeden Wert aus.
- Die Abnahme des Issues ist am Server des Systems nicht stellbar (`EEXIST` vor `EACCES`). Das Issue sagt das.

**`Vorstufe::PapierkorbUnbekannt`.**
- Die drei Werte stehen ohne Auffangzweig in `vor_der_rueckfrage`. Die Probe hält, dass die Meldung in de, fr und en keinen Ort ohne Papierkorb behauptet.
- Hinweis an den Nutzer: das Issue verlangte für die Wahl zwischen Möglichkeit 1 und 2 eine Antwort des Nutzers. Gewählt wurde selbständig Möglichkeit 1, die vorsichtige, die den Stand vor dem Paket wiederherstellt. Sie gehört bestätigt.

**Fehlerausgabe.**
- Gehalten werden höchstens acht Zeilen zu je 4096 Bytes. Der Überhang wird gelesen und verworfen, `ssh` hält nicht an.
- Probe mit 10 MiB ohne Zeilenende.

**200-ms-Frist.**
- `recv_timeout` an einem Kanal, dessen einziger Sender mit dem Faden fällt. Danach wird der Faden geerntet oder losgelassen.
- Die Probe mit `sleep 5 >&2 &` misst unter einer Sekunde.
- Je Trennung mit langlebigem Enkel bleibt ein Faden mit Pipe stehen, bis der Enkel endet. Der Doc-Kommentar sagt das.
- `compile_fail,E0277` prüft stabiles rustdoc nicht. Das Issue sagt das ehrlich.

**`ist_einzelner_name`.**
- Gefragt beim Lesen, beim Zusammensetzen und in den Vorgängen.
- `.` und `..` werden vorher ausgesondert und nicht mitgezählt.

**Ausweichweg bei `FAILURE`.** Kann er eine Quelle entfernen, deren Kopie unvollständig ist? Nein.
- `ueber_die_grenze` entfernt die Quelle allein ohne Übersprungenes seit dem Beginn.
- Jeder Weg ohne Ankunft ruft `ueberspringen`: Lesefehler, Namen, die sich nicht ansteuern lassen, `ZuVieleNamen`, Angaben, Konflikt mit „Überspringen“.
- Ein Abbruch kommt seit `kinder_lesen` als Abbruch zurück. Die Probe `ein_abbruch_beim_lesen_der_quelle_laesst_sie_beim_verschieben_stehen` belegt das.
- `inference:` aus OpenSSH `process_rename`: `FAILURE` bei freiem Ziel heißt neben `EXDEV` auch `EROFS`, `EBUSY` oder `EIO`.
  - Bei `EROFS` an der Quelle entsteht eine Kopie, und die Quelle bleibt mit dem Grund „kopiert, aber in der Quelle geblieben“.
  - Bei `EBUSY`, etwa an einem Einhängepunkt auf dem Server, wird der Inhalt verschoben, und der leere Ordner bleibt.
  - Daten gehen in keinem dieser Fälle verloren. Ohne Issue.

**Zwischenablagesprung.**
- In einem fernen Tab prüft `pruefen_fuer_fernen_tab` lokal, und `ordner_lesen` setzt `Tabort::Lokal`.
- Die Pfadeingabe von Hand bleibt beim Serverpfad.
- Hinweis an den Nutzer: auch hier wurde selbständig gewählt („bewusst zu entscheiden“ im Issue). `HowTo.md` beschreibt das gewählte Verhalten.

**`HowTo.md`.**
- Die Tasten `f5`/`f6`, `opt+cmd+g` und `cmd+e` stimmen mit `resources/default-keymap.toml` überein.
- Die Regel für den Rechnernamen, das unentschiedene Löschen und „Überschreiben“ stimmen mit dem Code, mit zwei Grenzen:
  - Der Satz „kommt der Ersatz nicht vollständig an, bleibt das Ziel, wie es war“ ist richtig. Ein Scheitern beim Entfernen des Ziels deckt er nicht ab (N1).
  - „Benennen Quelle und Ziel … über eine Verknüpfung denselben Eintrag, wird nichts überschrieben“ gilt für eine Kennung, nicht für zwei (N3).
- Ein verrutschter Satzbezug in N4.

## Neue Befunde nach Thema

### Überschreiben

**N1 (Medium). Ein Zielordner wird zur Hälfte entfernt, und der vollständige Ersatz wird verworfen.**
- `operation/fern.rs:820` `tauschen` entfernt das Ziel, bevor es umbenennt.
- `fern_baum_entfernen` (`:2251`) und lokal `loeschen::baum_entfernen` halten am ersten Fehler an. Das schon Entfernte bleibt entfernt.
- `kopie_ersetzt_ziel` (`:785`) räumt danach den Ersatz weg.
- Der Modulkopf (`:69`) und der Doc an `Tausch::ZielBlieb` (`:810`) behaupten, das Ziel bleibe unverändert.
- Richtung: das Ziel zuerst per `RENAME` beiseite benennen, dann den Ersatz einsetzen, zuletzt das Alte entfernen.
- Code gelesen, nicht ausgeführt.
- Issue `261017-0113-ueberschreiben-entfernt-einen-zielordner-zur-haelfte-und-wirft-dann-den-vollstaendigen-ersatz-weg.md`.

**N2 (Low). Gescheiterte Angaben verwerfen einen vollständigen Ersatz.**
- `datei_fuellen` (`:958`) und `ordner_fuellen` (`:1022`) überspringen, wenn nur `SETSTAT` scheitert. Für `kopie_ersetzt_ziel` heißt das „unvollständig“.
- An einem Server ohne `SETSTAT` gelingt „Überschreiben“ nie.
- Dazu verbucht der Bericht denselben Eintrag als fertig und als übersprungen, unter dem Pfad `.krk-ersatz-…`.
- Issue `261017-0113-ueberschreiben-verwirft-einen-vollstaendigen-ersatz-wenn-nur-rechte-oder-zeit-nicht-gesetzt-werden.md`.

**N3 (Low). Verschieben mit „Überschreiben“ über zwei Wege zum selben Speicher löscht die einzige Kopie.**
- Gemeint sind zwei Kennungen (`rechner`, `rechner:22`), ein lokal eingehängtes NAS neben demselben NAS per sftp oder sftp auf `localhost`.
- `derselbe_eintrag` sagt dort nein. `tauschen` entfernt die Quelle als Ziel, `ueber_die_grenze` (`:1339`) danach den Ersatz unter dem Pfad der Quelle.
- Das bestand vor der Behebung in derselben Lage ebenso. Das Kopieren ist seitdem sicher, das Verschieben nicht.
- Issue `261017-0113-verschieben-mit-ueberschreiben-loescht-die-einzige-kopie-wenn-quelle-und-ziel-ueber-zwei-wege-dieselbe-datei-sind.md`.

### Texte und Prosa

**N4 (Low). Vier Stellen weichen vom Code ab.**
- `Text::FernRechnerUngueltig` in drei Sprachen nennt die alte Regel.
- `rest_wegraeumen` meldet „nach dem Abbruch“ auch ohne Abbruch.
- `Erlaubnisbefund::erlaubt` hat keinen Rufer mehr, und sein `must_use`-Text ist falsch.
- `HowTo.md:1994`: der Bezug von „Dasselbe sagen“ ist verrutscht.
- Issue `261017-0113-texte-und-prosa-nach-den-behebungen-des-sftp-pakets-weichen-vom-code-ab.md`.

### Am Rand, ohne Issue

- `verschieben_ueber_eine_grenze_auf_dem_server_kopiert_und_entfernt_die_quelle` hängt in jedem `cargo test` ein HFS+-Abbild über `hdiutil` ein.
  - Ohne `hdiutil`, etwa in einer Sandbox, wird die Probe rot, statt sich zu überspringen.
  - Ein gescheitertes Aushängen lässt einen Rest stehen, den `hdiutil info` zeigt. Der Doc-Kommentar an `Einhaengung::drop` sagt das.
- `bereichsleiste.rs:144` (aus `c461b3a1..1e114402`) trägt eine nicht umbrochene Zeile im Untergrenzen-Abschnitt.

## Nachgeholt aus der früheren `**Not-opened:**`-Liste

Gelesen habe ich die Änderungen in `c461b3a1..1e114402`:
- `git/texte.rs`: `am_fernen_ort`.
- `gitmodell.rs`: Kennzeichen `am_fernen_ort`, fällt mit `zuruecksetzen`. Rufer in `tabs.rs:1693`.
- `heimordner/verbindung.rs`: `Weg::Sftp`, `Verbindungsziel::Sftp(Fernziel)`, `Zielfehler::Fernadresse` mit drei Zweigen ohne Auffangzweig.
- `blaetter/verbindung.rs`: `Sftp` kehrt vor `url_aus` zurück.
- `bereichsleiste.rs`: `suche_bedienbar` schaltet nur `setEnabled:`, der Zustand bleibt.
- `fernprobe.rs`: unter `#[cfg(test)]` (`main.rs:90`).
- `tests/heimordner.rs`: `Weg::Sftp`.
- `tests/operation.rs`: endgültiges Löschen ohne Papierkorb, die Attrappe bleibt leer.

Kein Befund außer der Zeile in `bereichsleiste.rs`. `README.md` ist kein Code. Ich habe sie nicht gegen den Code gehalten, weil die Dispatch auf Code eingegrenzt ist.

## Querschnitt

- **Entfernen vor Ersetzen.** N1 und N3 haben dieselbe Wurzel: `tauschen` zerstört zuerst und setzt dann ein.
  - Ein Beiseitebenennen des Ziels schließt N1 ganz.
  - Für N3 schafft es einen Prüfpunkt: fehlt danach die Quelle, waren beide derselbe Eintrag.
- **„Übersprungen“ ist der Zeuge für Vollständigkeit und zugleich der Kanal für Hinweise.**
  - N2 entsteht, weil ein Hinweis zum Fehler zählt.
  - Dieselbe Strenge gilt in `ueber_die_grenze`, dort bewusst vorsichtig.
  - Für „Überschreiben“ ist sie zu streng. Eine Trennung von „Inhalt fehlt“ und „Angabe fehlt“ beträfe beide Wege.

## Reihenfolge

1. Vor dem Nutzertest gegen `ssh.strato.de`: N1.
2. Die zwei selbständig getroffenen Wahlen (M5: Möglichkeit 1; L5: lokaler Sprung) vom Nutzer bestätigen lassen.
3. Aufräumen: N2, N3, N4.

Unverändert offen nach dem Plan sind der Nutzertest gegen `ssh.strato.de` und der Abnahmelauf L1 bis L10 aus C8 (E16).
