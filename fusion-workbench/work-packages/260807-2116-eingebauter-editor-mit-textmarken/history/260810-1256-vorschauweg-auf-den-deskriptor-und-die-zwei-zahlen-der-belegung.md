# Der Vorschauweg auf den Deskriptor, und die zwei Zahlen im Kopf der Belegung

**Agent:** coder
**Status:** Complete
**Circle:** `260807-2116-eingebauter-editor-mit-textmarken`
**Behandelt:** `issues/260810-1247_*_die-typpruefung-am-pfad-ist-im-vorschauweg-geblieben-und-dort-blockiert-sie.md`, `issues/260810-1219_*_die-zwei-zahlen-im-kopf-der-belegungsdatei-wachsen-nicht-mit-ihr.md`
**Dateigrenze:** `crates/krk-ui/src/vorschaumodell.rs`, `crates/krk-core/src/tasten/belegung.rs`

## Was getan wurde

### 1. Der Leseweg der Vorschau geht ueber den Deskriptor (`260810-1247`)

Die beiden `std::fs::read(pfad)` in `laden` sind durch **eine** neue Stelle ersetzt, `bis_zur_grenze_lesen(pfad, grenze)`. Sie oeffnet ueber `krk_core::verzeichnis::sys::ohne_warten_oeffnen`, also ueber das Stueck, das die Behebung des Defekts `260809-1652` fuer den Editor gebaut hat; ein zweiter Oeffnungsweg im Vorschaumodell entsteht nicht. Drei Eigenschaften kommen damit mit:

- `O_NONBLOCK` beim Oeffnen, das die Huelle in `krk-core` selbst wieder abnimmt. Eine benannte Roehre ohne Schreiber haelt den Faden `krk-vorschau` nicht mehr an.
- `fstat(2)` am Deskriptor statt `stat(2)` am Pfad. Roehre, Zeichengeraet, Blockgeraet und Socket fallen an `!angaben.is_file()` heraus, statt als `Typ::Datei` durch die Groessenschranke zu kommen, die bei `st_size == 0` nie greift.
- `take(grenze + 1)` beim Lesen. Eine Datei, die zwischen `fstat` und `read` wachst, kann die Grenze nicht mehr ueberschreiten; bisher war "die Vorschau liest nie mehr als ihre Grenze" eine Vorhersage aus einer alten Auskunft.

`typ_von` ist unangetastet geblieben, wie der Datensatz es verlangt: es beantwortet, was die Vorschau **anzeigt**, und dafuer sind drei Zweige richtig. Der Modulkopf fuehrt die beiden Fragen jetzt getrennt: `lstat(2)` fuer die Anzeige, `fstat(2)` am Deskriptor fuer die Lesbarkeit.

**Der Umbau hat `laden` kuerzer gemacht, nicht laenger.** Alle vier Ablehnungsgruende (zu gross, keine gewoehnliche Datei, nicht lesbar, kein UTF-8) enden in den Metadaten, und die letzten drei taten es schon vorher; die beiden eigenen Groessenzweige sind damit entfallen.

### 2. Eine Probe haelt die zwei Zahlen im Kopf der Belegungsdatei (`260810-1219`)

`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` steht neben `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` in `crates/krk-core/src/tasten/belegung.rs`. `resources/default-keymap.toml` ist unangetastet, die Zahlen 71 und 79 ebenso.

**Die Probe traegt keine eigene Zahl.** Der Helfer `zahlen_aus_dem_dateikopf` liest beide aus der Kommentarzeile, die mit `# Ausgeliefert sind ` beginnt, und die Probe zaehlt die Datei dagegen. Damit stehen die Zaehlstaende weiterhin an genau einer Stelle, und ein Nachtrag hat genau eine Stelle nachzuziehen: die Kommentarzeile. Der Fehlschlag nennt sie im Klartext.

Das ist auch die Antwort auf eine Spannung im Baum, die dieser Datensatz nicht erwaehnt: die Nachbarprobe `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren` hatte eine Vorgaengerin mit genau diesen beiden Zahlen als Literal, und ihr Kommentar sagt, warum sie weichen musste. Eine Probe mit Literal haette diesen Grund ein zweites Mal eingesammelt.

Gesucht wird die Zeile an ihrem Anfang und nicht an ihrer Nummer. Das hat sich noch in derselben Sitzung bewaehrt: der parallel arbeitende Agent hat den Dateikopf um drei Zeilen verlaengert, und die Zeile stand danach auf 33 statt auf 30.

## Gemessen

| Gegenstand | Ohne die Behebung | Mit ihr |
|---|---|---|
| `laden` auf einer Roehre ohne Schreiber | Fehlschlag nach 5 s: "laden ist nach 5s nicht zurueckgekommen; das Oeffnen haengt" | `Inhalt::Metadaten` mit dem Pfad der Roehre |
| Zahlen im Dateikopf, Datei um einen Eintrag laenger | — | Fehlschlag: "der Kopf von resources/default-keymap.toml nennt 71 Funktionen, die Datei traegt 72; die Zeile \"# Ausgeliefert sind ...\" gehoert nachgezogen" |

Beide Gegenrichtungen sind am 260810 je einmal gefahren, indem die behobene Stelle im Arbeitsbaum voruebergehend zurueckgestellt wurde (`std::fs::File::open` statt `ohne_warten_oeffnen`; ein angehaengter `[[funktion]]`-Block im Pruefcode). Danach ist der Stand byteweise gegen die Abschrift von vorher verglichen worden.

**Fuer `/dev/zero` steht bewusst keine zweite Probe.** Es faellt an derselben Zeile heraus wie die Roehre (`!angaben.is_file()`), und vor der Behebung waere es kein Befund, sondern ein volllaufender Arbeitsspeicher auf dem Geraet dessen, der die Behebung zuruecknimmt. Der Grund steht im Kommentar der bestehenden Probe.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt -p krk-core -- --check` | exit 0 |
| `cargo fmt -p krk-ui -- --check` | exit 1, allein wegen `crates/krk-ui/src/appkit/editor.rs:1825`; das ist die Datei des parallel arbeitenden Agenten und lag ausserhalb der Dateigrenze |

Die beiden bearbeiteten Dateien fuer sich: `rustfmt --edition 2024 --check crates/krk-ui/src/vorschaumodell.rs crates/krk-core/src/tasten/belegung.rs` → exit 0. Die einzige Clippy-Warnung des Laufs (`getragene_bytes is never used`) steht ebenfalls in `appkit/editor.rs`.

Ein erster Lauf von `cargo test --workspace` scheiterte mit exit 101 an einem Uebersetzungsfehler in `crates/krk-ui/src/hervorhebung.rs:1050`, mitten in der Arbeit des parallelen Agenten. Der Lauf danach war gruen; an den hier geaenderten Dateien lag es nicht.

## Neu gefundene Defekte

- `issues/260810-1300_o_die-doku-von-ohne-warten-oeffnen-nennt-einen-einzigen-aufrufer-und-es-sind-zwei.md` — drei Stellen in `krk-core` sagen weiter, `ohne_warten_oeffnen` habe einen einzigen Aufrufer. Ausserhalb der Dateigrenze; die tragende ist `crates/krk-core/src/verzeichnis/sys.rs:736`, weil sie mit der Einzigkeit begruendet, dass die Zielpruefung beim Aufrufer liegt.
- `issues/260810-1256_o_die-proben-des-vorschaumodells-legen-ihre-ordner-unter-festen-namen-an.md` — sieben aeltere Proben derselben Datei legen ihren Pruefordner unter einem festen Namen an und raeumen ihn nicht ab. Der neue `Pruefordner` dieser Datei ist das Mittel; die Umstellung ist bewusst nicht in diesen Diff gelegt worden.
