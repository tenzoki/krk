# Drei Nutzerantworten vom 260907-0823 in Code umgesetzt: Zehnerblock-Prosa, Rechtesperre-Abbruch, `#[must_use]` an den Prüfordnern

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Aufgabe K5 der Sitzung. Drei beantwortete Entscheidungsdatensätze werden in Code
umgesetzt; keiner ändert Verhalten der Anwendung. HEAD beim Beginn war `90f352d`.

---

## Auftrag 1: die falsche Erklärung zum Zehnerblock

Datensatz `260826-1223_*_loesen-die-zifferntasten-des-zehnerblocks-dieselbe-funktion-aus-wie-die-obere-reihe.md`,
Antwort: es bleibt beim halben Anschluss, nur die Prosa wird richtiggestellt.

**Erhoben.** Der Baum nennt den Zehnerblock an einer Reihe von Stellen; geprüft
wurde jede gegen die Kette `Tastendruck::aus_ereignis` → `zeichen_als_kennung` →
`Tastendruck::kennung` → `Belegung::nachschlag`. Vier Stellen waren falsch oder
irreführend, die übrigen tragen:

- richtig und unangetastet: die Löschung des Bits `NSEventModifierFlagNumericPad`
  in `krk-core/src/tasten/normalisierung.rs` samt ihren zwei Proben in
  `tests/tasten.rs`, `maske_aus_appkit` in `krk-ui/src/appkit/menue.rs`, die drei
  Stellen um die synthetischen Ereignisse in `krk-ui/src/appkit/ereignisse.rs`
  und der Absatz „Zwei Ausnahmen mit Grund" über `plus` und `minus`.
- falsch: die zwei Stellen, die der Defektdatensatz nennt, und **zwei weitere in
  `krk-ui`**, die die Aussage „Satzzeichen, Zehnerblock" als Klammer
  weitertragen. Für die Ziffern des Blocks, für `+` und für `-` findet
  `Kombination::aus_tastendruck` sehr wohl einen Namen; sie erreichen
  `Zuweisung::OhneNamen` nie.

**Geändert.** Die Auskunft steht jetzt an genau einer Stelle, im Modulkopf von
`parser.rs` unter „Was der Zehnerblock auslöst, und was nicht"; die drei anderen
Stellen verweisen darauf, statt die Regel zu wiederholen. Keine Zahl: die Regel
lautet „eine Taste des Blocks löst genau dann aus, wenn das Zeichen, das sie
meldet, ein Name der Tabelle trägt", und welche Zeichen das sind, sagt
`zeichen_des_namens`.

Der zugehörige Defekt
`260826-1223_*_der-grund-fuer-den-ausschluss-des-zehnerblocks-traegt-seit-der-zeichenkennung-nicht-mehr.md`
ist damit geschlossen: sein Punkt 1 ist umgesetzt, sein Punkt 2 vom Nutzer
entschieden.

## Auftrag 2: Proben, die unter `root` nichts messen können

Datensatz `260826-1302_*_schweigt-eine-probe-die-unter-root-nichts-messen-kann-oder-faellt-sie-aus.md`,
Antwort: Abbruch mit klarem Text statt stillem Überspringen; die vier roten
Proben unter `root` sind gewollt.

**Erhoben, und der Bestand ist größer als der Datensatz sagt.** Der Datensatz
nennt zwei schweigende und zwei ungeprüfte Proben. Erhoben über
`grep -rn 'chmod\|set_permissions\|from_mode' crates/` und Lesen jeder Fundstelle:

- **still übersprungen: drei**, nicht zwei. Neben den zwei in
  `tests/text.rs` steht dieselbe Zeile in `tests/verzeichnis.rs`,
  `eine_datei_ohne_leserecht_traegt_nichts`.
- **Voraussetzung gar nicht geprüft: drei**, nicht zwei. Neben den zwei in
  `tests/operation.rs` steht `ein_nicht_lesbarer_ordner_gilt_als_kein_treffer`
  in `tests/verzeichnis.rs`; unter `root` liest sich der gesperrte Ordner, sein
  Eintrag trägt die gesuchte Folge im Namen, und die Probe fällt mit einer
  Meldung aus, die den Grund nicht nennt.
- **schon in der entschiedenen Form: eine.** `tests/ablage.rs`,
  `die_gueltigkeitspruefung_kommt_ohne_lesen_der_datei_aus`, prüft die
  Voraussetzung und bricht mit einem Text ab, der `root` nennt.
- **nur ein Hinweis im Meldetext: eine.** `tests/leseprofil.rs`,
  `ein_ordner_ohne_leserecht_zeigt_drei_platzhalter_unter_ihren_beschriftungen`,
  prüft nichts und hängt die Frage „läuft die Probe als root?" an eine
  fachliche Zusicherung.
- **absichtlich unter jeder Kennung gleich: zwei.** Die zwei
  Verweisziel-Proben am Ende von `tests/verzeichnis.rs` fragen `stat(2)` und
  nicht `open(2)`; ihre Köpfe schreiben aus, warum kein `#[ignore]` daransteht.
- **grün unter `root`, und dabei ohne Beweis: zwei.**
  `jeder_auftrag_bekommt_genau_einen_befund` und
  `ein_namenstreffer_im_unterbaum_bleibt_ungelesen` in `tests/verzeichnis.rs`.
  Beide bleiben grün, weil ihr Erwartungswert auf einem zweiten Weg zustande
  kommt; ihr Beleg fällt weg, ohne dass etwas rot würde. Das ist eine dritte
  Klasse, die der Datensatz nicht kennt, und sie ist nicht mitentschieden:
  Defekt `260907-0858_*_zwei-proben-bleiben-unter-root-gruen-und-verlieren-dabei-ihren-beleg.md`.

**Geändert.** Die Regel steht als eine Funktion in
`tests/gemeinsam/mod.rs`, `rechtesperre_haelt_oder_abbruch(zusage, sperre_haelt)`;
ihr Doc-Kommentar trägt die Begründung, den Nutzerentscheid und den Satz, dass
dieser Baum nie unter `root` geprüft wird — einmal und nicht siebenmal. Acht
Aufrufstellen in fünf Prüfdateien, jede mit dem Zugriff, den sie eben verboten
hat: `fs::read`, `fs::read_dir` oder ein Schreibversuch im Ordner mit `0o500`.
Die zwei bestehenden Eigenformen in `ablage.rs` und `leseprofil.rs` gehen auf
dieselbe Funktion, statt daneben stehen zu bleiben.

`krk-ui` fährt dieselbe Frage an fünf eigenen Stellen und erreicht
`tests/gemeinsam/` nicht: Defekt
`260907-0858_*_fuenf-proben-in-krk-ui-fahren-die-rechtesperre-frage-ohne-die-eine-antwort-des-kerns.md`.

## Auftrag 3: `#[must_use]` an den drei Prüfordner-Fassungen

Datensatz `260905-2155_*_bekommen-die-drei-pruefordner-fassungen-must-use-oder-keine.md`,
Antwort: alle drei, in einem Durchgang über die drei Kisten.

Die Marke steht an der **Struktur** und nicht an `neu`, damit sie auch für
`Pruefordner::nur_name` in `krk-ui` gilt; die Begründung ist an allen dreien
zeichengleich. `genau_drei_pruefordner_fassungen_stehen_im_baum` sucht
`struct Pruefordner` beziehungsweise `struct Wegwerfordner` und ist von der
Attributzeile darüber nicht betroffen; die Probe bleibt grün.

Die vierte Fassung `Wegwerfwurzel` in `xtask/src/release.rs` ist nicht Teil des
Auftrags und weicht seit diesem Durchgang um ein weiteres Merkmal ab. Vermerkt
als `Also seen:` an
`260826-1302_*_eine-vierte-pruefordner-fassung-steht-in-xtask-und-die-zaehlprobe-c4-6-kann-sie-nicht-sehen.md`.

---

## Geprüft

Abschlusslauf, alle fünf grün:

```
cargo build --workspace                                   exit 0
cargo test --workspace                                    exit 0
cargo clippy --workspace --all-targets -- -D warnings      exit 0
cargo fmt --all --check                                   exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  exit 0
```

Zwei Zwischenläufe waren rot, beide an `crates/krk-core/tests/baum.rs` und beide
aus der zweiten gleichzeitig laufenden Bahn (dort +290 Zeilen, keine davon aus
diesem Auftrag): einmal `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`
(`Bereich::ALLE` führte `Editor` doppelt), einmal ein Formatunterschied in
derselben Datei. Beide waren beim Abschlusslauf weg, ohne Zutun dieses Auftrags;
`baum.rs` ist hier nicht angefasst worden.

Nicht angefasst, wie beauftragt: `crates/krk-core/tests/belegung.rs` und
`crates/krk-core/tests/git.rs`.
