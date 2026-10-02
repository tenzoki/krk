# Vollbaum-Durchsicht: `crates/krk-core/src/verzeichnis/`

**Reviewed-range:** `004ff72..004ff72`
**Not-opened:** none
**Umfang im Klartext:** kein Commit-Bereich, sondern eine Vollbaum-Durchsicht von
`crates/krk-core/src/verzeichnis/` am Stand HEAD `004ff72`. Beide Enden der Spanne benennen
denselben Commit, weil kein Bereich gelesen wurde, sondern ein Baumstand; die zwei aufgeloesten
Kurzhashes stehen da, damit `bin/fusion-review-coverage` die Zeile lesen kann. Alle 14 Dateien
des Umfangs sind geoeffnet, deshalb `none`.
**Sender:** coderev
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Gelesen:** 14 Dateien, 6.733 Zeilen, jede vollstaendig
**Nicht gefahren:** kein `cargo build`, kein `cargo test`. Begruendung unter „Was nicht gemessen ist".

## Zusammenfassung

Der Modulbaum ist in ungewoehnlich gutem Zustand. Die Zusagen, gegen die die Durchsicht
ausdruecklich angesetzt war, halten am Baum: der eine Verzeichnisdeskriptor, der eine
Dateideskriptor waehrend eines Lesens, die dreiwertige Antwort bei Deskriptormangel, die
Pruefung am Deskriptor statt am Pfad, die variadische `fcntl`-Deklaration, die drei Filterregeln
an je einer Stelle samt zutreffender Zaehlprobe, die eine Vorbelegung der tiefen Suche, das
Ersetzen statt Vorableeren, der einmal gebaute Sortierschluessel und die Grenze
`deny(unsafe_code)` mit ihrer einen Ausnahme. Zehn von zehn geprueften Zusagen gelten.

Der schwerste Befund liegt daneben und nicht darin: `Schwungleser::oeffnen` ist der eine
Oeffner der Datei `sys.rs`, der die Huelle nicht nimmt, die dieselbe Datei sechshundert Zeilen
tiefer dafuer fuehrt. Er kann an einer benannten Roehre fuer immer haengen, und zwei seiner
Rufer tun das auf dem Hauptfaden. Daneben stehen acht Befunde geringeren Gewichts, davon vier
in derselben Klasse: eine Regel, die der Baum an einer Stelle gelernt und an der Nachbarstelle
nicht angewandt hat.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 4 |
| Niedrig | 4 |
| **Summe** | **9** |

Alle neun sind als eigener Defektdatensatz unter `shared/issues/` gefiltert, Stempel
`260826-1221`. Kein Circle ist aktiv; die Herkunftsregel fuehrt sie damit in den gemeinsamen
Speicher.

## Befunde nach Thema

### Thema 1: Eine Regel gelernt, an der Nachbarstelle nicht angewandt

Das ist das durchgehende Muster dieser Durchsicht. Vier Befunde teilen es, und jeder von ihnen
hat im Baum einen geschlossenen Datensatz als Vorlage.

---

**[Hoch] Der Schwungleser oeffnet mit `File::open` und haengt an einer benannten Roehre fuer
immer.**
`sys.rs:229-236`. Datensatz: `shared/issues/260826-1221_*_der-schwungleser-oeffnet-mit-file-open-*`.

```rust
// crates/krk-core/src/verzeichnis/sys.rs:229-232
pub fn oeffnen(pfad: &Path) -> io::Result<Self> {
    let verzeichnis = File::open(pfad)?;
    if !verzeichnis.metadata()?.is_dir() {
```

Die **Typpruefung** steht richtig am Deskriptor. Das **Oeffnen** ist ungeschuetzt. In derselben
Datei steht `ohne_warten_oeffnen` (`sys.rs:889`), dessen Doc-Kommentar den Schaden wortgleich
beschreibt. Der Griff, der hier fehlt, ist im Baum dreimal getan worden: `260809-1652` fuer den
Editor, `260810-1247` fuer die Vorschau, `260825-0942` fuer das Packen.

Fuenf Rufer, gestaffelt nach Reichweite:

| Rufer | Pruefung vor dem Oeffnen | Faden |
|---|---|---|
| `umfang::zaehlen:239` | `lstat` am Pfad, dann `oeffnen` am Pfad | **Hauptfaden** |
| `umfang::zaehlen:274` | `Typ::Ordner` aus dem Schwung | **Hauptfaden** |
| `durchlauf.rs:512` | `Typ::Ordner` bzw. Auftragstyp | Arbeitsfaden |
| `leser.rs:281` | keine | Arbeitsfaden |
| `leser.rs:235` (`lesen_hoechstens`) | **keine** | Arbeitsfaden der Vorschau |

Der letzte kommt ohne jedes Zeitfenster aus: `leseprofil/bausteine.rs:422` und `:467` reichen
einen aus `readers.toml` zusammengesetzten Pfad ohne Typfrage weiter.

**Warum das nicht der zurueckgestellte Netzpfad-Defekt ist.** Der Datensatz
`circles/260802-0842-*/issues/260805-0000_d_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`
beschreibt denselben Schaden aus anderer Ursache und ist zurueckgestellt, weil ohne Pruefserver
nichts zu messen sei. Hier legt `mkfifo(1)` den Fall lokal an, die Abhilfe liegt im Baum, und
`umfang` gab es damals nicht — jener Datensatz haelt fest, C9 halte, weil kein Aufrufer je auf
einen Lesefaden wartet, und genau das gilt fuer die Zaehlung auf dem Hauptfaden nicht.

---

**[Mittel] Zwei verschiedene Typen unter `verzeichnis/` heissen beide `Lesestand`.**
`leser.rs:169` (oeffentlich, Runde 16) und `durchlauf.rs:610` (privat, Runde 10). Kein Feld
gemeinsam. Datensatz: `shared/issues/260826-1221_*_zwei-verschiedene-typen-*-lesestand.md`.

Es ist buchstaeblich der Befund `260817-1419`, den der Baum fuer `Befund` aufgeloest hat, und
`mod.rs:109-122` schreibt die Lehre daraus in einem eigenen Absatz aus — der die
`Befund`-Familie vollstaendig aufzaehlt und die zweite Doppelung im selben Verzeichnis nicht
nennt. Sie ist die gefaehrlichere von beiden, weil der oeffentliche Typ das Modul verlaesst:
`leseprofil/bausteine.rs:191` fuehrt ihn unqualifiziert ein und nennt ihn danach in sechs
Zeilen.

---

**[Mittel] Zwei Fadenstarts brechen mit Panik ab, waehrend derselbe Mangel am Deskriptor
sorgfaeltig behandelt ist.**
`leser.rs:117`, `durchlauf.rs:277`. Datensatz:
`shared/issues/260826-1221_*_zwei-fadenstarts-des-verzeichnisbaums-*`.

`thread::Builder::spawn` liefert ein `io::Result` genau dafuer, dass der Aufrufer `EAGAIN`
behandeln kann; beide Stellen werfen es mit `.expect(...)` weg, auf dem Hauptfaden. Derselbe
Modulbaum baut fuenf Stellen und einen ganzen Typ (`Loeschzielbefund`) darum herum, dass die
**Deskriptor**tabelle von aussen leerlaufen kann und das keine Aussage ueber den Gegenstand ist.
Der Fadenvorrat ist die Schwestergroesse und wird gegenteilig behandelt.

Ein parallel laufender Pruefer hat dieselbe Form in `operation/mod.rs:165` gefunden; die zwei
Datensaetze verweisen aufeinander.

---

**[Mittel] `#[must_use]` traegt sieben Praedikate des Modulbaums und rund zwanzig gleichartige
daneben nicht.**
Datensatz: `shared/issues/260826-1221_*_must-use-traegt-sieben-praedikate-*`.

Die zwei Paare, an denen die Auslassung nicht als Abwaegung zu lesen ist:

- `Ordnermodell::tief()` (`modell.rs:982`, ohne) und `Ordnermodell::inhalt()` (`modell.rs:1005`,
  **mit**) — die beiden Ankreuzfelder desselben Filters, zwanzig Zeilen auseinander, gleiche
  Signatur, gleicher Rumpf.
- `filter.rs` fuehrt laut eigenem Modulkopf „die drei Regeln des Filters"; `inhaltsschwelle`
  traegt es, `traegt_ein_dateiname` und `traegt_die_folge` nicht. Die Begruendung bei der
  ersten (`filter.rs:154`) trifft die beiden anderen wortgleich.

CLAUDE.md fuehrt die Regel als Nutzerentscheid vom 260811-2140. Der Bau faengt die Auslassung
nicht: ohne das Attribut gibt es keine Warnung, die `-D warnings` verschaerfen koennte.

### Thema 2: Eine ausgelieferte Vorgabe mit einer nicht aufgeschriebenen Folge

**[Mittel] Die tiefe Suche ab Werk nimmt jede Verknuepfung beim ersten Anschlag aus der Liste.**
`modell.rs:374`, `:746-772`, `durchlauf.rs:490-501`. Datensatz:
`shared/issues/260826-1221_*_die-tiefe-suche-ab-werk-nimmt-jede-verknuepfung-*`.

Der Weg ist kurz: eine Verknuepfung faellt im Pruefschritt **nicht** aus dem Ordnerzweig
(`modell.rs:757`), der Zweig `!self.tief` ist ab Werk uebersprungen (`:766`), also steht sie
unter Vorbehalt — und `unterbaum_entscheiden` beantwortet den fuer jede Verknuepfung negativ
(`durchlauf.rs:499`). Der Befund faellt erst mit dem Filtertext, nicht mit einem weiteren Lauf.

Das Verhalten ist alt, gewollt und geprueft (C1.6/C2.13). **Neu ist, welche Haelfte jener Zusage
der Nutzer ab Werk bekommt.** Die Probe schreibt beide aus, und ihre zwei Aufbauhelfer
`gefiltert` (`tests/verzeichnis.rs:710`) und `handmodell` (`:1225`) setzen `tief_setzen(false)`
ausdruecklich — sie stellen also den Zustand **vor** der Vorgabenaenderung her, und keine Probe
misst die Vorgabe gegen eine Verknuepfung.

`shared/decisions/260826-0859_*` behandelt genau diese Sorte Folge und benennt sie als solche
(„eine zweite Groesse, die niemand angefordert hat") — behandelt aber allein die
Inhaltsschwelle. Diese hier ist eine dritte, und keine der drei dort angebotenen Moeglichkeiten
aendert etwas an ihr.

### Thema 3: Prosa, die von dem Code abweicht, ueber den sie steht

Drei Befunde, alle niedrig, alle mit derselben Wirkung: ein spaeterer Leser zieht aus dem
Kommentar einen Schluss, den der Code nicht traegt.

**[Niedrig] Der Modulkopf des Ordnermodells sagt, die Oberflaeche frage vor jedem
Zeichendurchgang nach der Auswahlzeile.** `modell.rs:14-16`. Die vier Ruferstellen von
`auswahl_zeile` sind samtlich ereignisgetrieben (`tabelle.rs:2112`, `:2211`, `:2223`,
`kommandos/operationen.rs:188`; der `cfg(test)`-Block von `tabelle.rs` beginnt erst bei 4956).
`auswahl_zeile` geht ueber `zeile_von`, eine lineare Suche; je Ereignis ist das nichts, je
Zeichendurchgang waere es der teuerste Posten gegen L3 und L10. Datensatz:
`shared/issues/260826-1221_*_der-modulkopf-des-ordnermodells-sagt-*`.

**[Niedrig] Ein Kommentar im Durchlauf sagt, `Typ::Ordner` sei auch eine Verknuepfung, und
widerspricht sich zwei Zeilen spaeter.** `durchlauf.rs:548-554`. `typ_aus_objtype`
(`sys.rs:421-427`) legt `VLNK` nach `Typ::Verknuepfung`, nie nach `Typ::Ordner`; der Satz
traegt eine Aussage der **Sichtbarkeit** (`modell.rs:757`, wo sie stimmt) an eine Stelle, an
der sie ueber einen anderen Wert gemacht wird. Datensatz:
`shared/issues/260826-1221_*_ein-kommentar-im-durchlauf-sagt-typ-ordner-*`.

**[Niedrig] `Abschluss::ist_abgebrochen` hat ausserhalb der Proben keinen Rufer.**
`leser.rs:68-71`. `krk-ui/src/tabs.rs:1118` verzweigt direkt ueber die Variante. Weil `krk-core`
eine Bibliothek ist, sieht `dead_code` es nicht — die zwei Nachbarmodule `umfang.rs:146-149` und
`arbeitsbaum.rs:173-176` schreiben genau diesen Umstand in ihrem Modulkopf aus, `leser.rs` sagt
dazu nichts. Datensatz: `shared/issues/260826-1221_*_abschluss-ist-abgebrochen-hat-*`.

### Thema 4: Zwei Fassungen derselben Frage mit zwei Antworten

**[Niedrig] Die zwei Abbruchwege des Lesefadens behandeln den angefangenen Stapel verschieden.**
`leser.rs:288-291` ruft `rest_senden`, `leser.rs:311-314` nicht — und dort steht `gesammelt` per
Schleifenbedingung bei mindestens 1.024 Eintraegen. Beide melden denselben
`Abschluss::Abgebrochen`, und ein Grund fuer den Unterschied steht an keiner der beiden Stellen.
Datensatz: `shared/issues/260826-1221_*_die-zwei-abbruchwege-des-lesefadens-*`.

## Die zehn Zusagen aus dem Auftrag, einzeln geprueft

| Zusage | Stand | Beleg |
|---|---|---|
| Genau ein Verzeichnisdeskriptor, gleich wie tief | **haelt** | `durchlauf.rs:506-578`, `Lesestand` faellt am Ende jeder Runde des `while let`; `umfang.rs:249-300` traegt dieselbe Bauform |
| Unterordner als **Pfad** auf dem Stapel | **haelt** | `durchlauf.rs:556`, `umfang.rs:274` — beide `push` einen `PathBuf`, keinen Leser |
| Genau ein Dateideskriptor, nur waehrend eines Lesens | **haelt** | `datei_entscheiden` → `traegt_der_inhalt` → `bis_zur_grenze_lesen`, ein Aufruf je Kandidat, keine Sammlung |
| Deskriptormangel laesst unentschieden | **haelt** | `durchlauf.rs:514` und `:569`, `umfang.rs:256` und `:282`, `inhalt.rs:151`; `ist_deskriptormangel` trennt `EMFILE`/`ENFILE` (`sys.rs:344`) und die Probe `nur_emfile_und_enfile_gelten_als_deskriptormangel` deckt auch den Fehler **ohne** OS-Nummer ab |
| Pruefung am Deskriptor, nicht am Pfad | **haelt** | `ohne_warten_oeffnen:889-896`, `blockierend_stellen:903-922` mit `F_GETFL`/`F_SETFL`; die Probe `ein_geoeffneter_deskriptor_traegt_o_nonblock_nicht_mehr` fragt den Deskriptor selbst |
| Kein `libc`, variadische `fcntl` | **haelt** | `sys.rs:790-799`, `fn fcntl(fd, befehl, ...)`; `Cargo.toml` von `krk-core` fuehrt kein `libc` |
| Drei Filterregeln je einmal | **haelt** | `filter.rs:90`, `:122`, `:157`; nachgezaehlt: Zeichenregel zwei Rufer, Vergleich drei, Schwelle einer |
| Zaehlprobe stimmt noch | **haelt** | `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` erwartet `[tabelle.rs, belegungsmodell.rs]` und `[durchlauf.rs, inhalt.rs, modell.rs]`; `grep` liefert genau diese |
| „Deep" ab Werk an genau einer Stelle | **haelt** | `modell.rs:374` ist die einzige Zuweisung an `tief` ausserhalb der Setzer; `inhalt_wirkt:1080` haengt an demselben Wert |
| Lesevorgang leert nicht vorab | **haelt** | `lesevorgang_beginnen:413-416` setzt nur die Marke, `ersatz_einloesen:440-455` wird von `anhaengen` und `abschliessen` gerufen; vier Proben halten es |
| Sortierschluessel einmal beim Lesen | **haelt** | `Eintrag::mit_versteckt:88-91` baut beide Schluessel; `Sortierung::vergleiche:125-144` vergleicht ausschliesslich vorberechnete Werte |
| `deny(unsafe_code)` mit einer Ausnahme | **haelt** | `#![allow(unsafe_code)]` steht in `sys.rs:130` und in keiner anderen Datei des Umfangs |
| `#[must_use]`, wo stiller Verlust unbemerkt bliebe | **haelt nicht durchgaengig** | siehe Befund oben |

## Was nicht gemessen ist

- **Kein Bau- und kein Probenlauf.** Drei weitere Pruefer arbeiten parallel im selben Baum, und
  `Messplanwaechter::neu` raeumt beim Anlegen jede fremde `krk-messplan-*.toml` im
  Temporaerverzeichnis ab; `cargo test -p krk-core` loest das mit aus
  (`shared/issues/260810-1925_*`). Jede Aussage dieser Durchsicht ist am Quelltext gelesen und
  keine an einem Lauf gemessen.
- **Der Roehrenfall ist nicht nachgestellt.** Die Wirkung von `File::open` auf eine benannte
  Roehre ohne Schreiber ist dokumentiertes POSIX-Verhalten und steht im Doc-Kommentar von
  `ohne_warten_oeffnen` als bereits gemessen; dass `Schwungleser::oeffnen` denselben Aufruf tut,
  ist am Quelltext abgelesen. Das Zusammentreffen selbst ist `inference:`.
- **`getattrlistbulk(2)` unter `O_NONBLOCK`** ist ungeprueft. Fuer den Fix des schwersten
  Befunds ist das die eine offene Frage; `blockierend_stellen` nimmt das Kennzeichen unmittelbar
  nach dem Oeffnen wieder ab, also `inference:` ohne Wirkung.
- **Die Namensvergleiche des Filters** (`traegt_die_folge` ueber `to_lowercase()`) legen je
  Vergleich eine Zeichenkette an, im Unterbaum also eine je gelesenem Namen. Der Doc-Kommentar
  benennt die Asymmetrie ausdruecklich als gewaehlt (`filter.rs:112-116`); nicht gemeldet,
  weil die Wahl dasteht und begruendet ist. Gemessen ist sie nicht.

## Empfohlene Reihenfolge

**Vor der naechsten Auslieferung:** der Roehrenfall (`Schwungleser::oeffnen`). Er ist der
einzige Befund, der die Anwendung dauerhaft anhalten kann, und der Griff ist ein Zeilentausch
gegen eine Huelle, die im Baum steht.

**Als Runde oder als Nutzerfrage:** die Verknuepfungsfolge der Deep-Vorgabe. Sie ist eine
Entscheidung und keine Behebung; vorgelegt gehoert sie mit ihren Folgen, nicht als Liste.

**Aufraeumen, in einem Zug:** die zwei Fadenstarts, die `Lesestand`-Doppelung, die
`#[must_use]`-Erhebung. Alle drei sind mechanisch, alle drei haben einen geschlossenen
Datensatz als Vorlage.

**Wenn jemand die Dateien ohnehin anfasst:** die vier niedrigen. Keiner rechtfertigt einen
eigenen Durchgang.
