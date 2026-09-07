# K15b: der Codeanteil zu `filter_einfuegen`, und die 52 Fehlschläge sind weg

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Den roten Stand auflösen, den K15a hinterlassen hat: die Belegungszeile
`filter_einfuegen` (`cmd+f`, `gehalten_von = "menue"`) stand in
`resources/default-keymap.toml`, der Code dazu fehlte, und `cargo test -p
krk-ui` kam mit 101 zurück — 52 Fehlschläge an drei Stellen. Aufzulösen war das
durch Nachziehen des Codes und nicht durch Zurücknehmen der Belegungszeile.
Grundlage: der Nutzerentscheid vom 260907-2009
(`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`),
die Erhebung `260907-2020-k14-cmd-f-fuegt-in-den-filtertext-ein.md` mit ihrer
Stellenliste und `260907-2046-k15a-die-belegungszeile-fuer-cmd-f.md`.

## Der Kern der Änderung: ein Selektor weicht einem anderen

`paste:` beim Anwendungsdelegierten fällt weg, `filterEinfuegen:` tritt an seine
Stelle — in der Methode und im Zweig von `validateMenuItem:`. Der Rumpf
darunter bleibt unberührt: derselbe Vorspann `bearbeiten_am_dateifenster`,
dasselbe Tor `zulaessigkeit::dateiablage_zulaessig`, derselbe `Anspruch`,
dieselbe Datenquellenmethode `aus_zwischenablage_einfuegen`. Die Zahl der
Selektoren bleibt drei, die Zählprobe
`die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` bleibt bei zwei Fragern,
und `krk_core::zwischenablage::filtertext_aus` ist nicht angefasst.

Neu ist die Sorte des Selektors: `filterEinfuegen:` führt KRK selbst, keine
AppKit-Klasse kennt ihn. Daran hängen zwei Folgen, die im Baum jetzt
ausgeschrieben stehen. Erstens misst `appkit::menue::GEMESSEN` ihn nicht und
soll ihn nicht messen — eine Zeile dort sähe wie eine Messung aus, die nichts
gemessen hat. Zweitens bekommt `belegungsausgabe::wirkung` für ihn eine
**fünfte** Begründungslage: die Zelle ist am Code entscheidbar, weil der
Selektor genau einen Rufer hat, und liest sich als
`Wirkungsbereich::Dateifenster.beschriftung()` ab statt als zweite Kopie der
Zeichenkette.

## Was geändert ist, je Datei

Funktionaler Anteil:

- `crates/krk-ui/src/menuemodell.rs:165` — siebter Eintrag in `ZUSTELLER`,
  Länge `6` → `7`, `("filter_einfuegen", c"filterEinfuegen:")`.
- `crates/krk-ui/src/belegungsmodell.rs:248` — `"filter_einfuegen"` →
  `Funktionsbereich::Dateilisting` in `bereich`.
- `crates/krk-ui/src/belegungsausgabe.rs:350` — eigener Zweig in `wirkung`.
- `crates/krk-ui/src/belegungsausgabe.rs:131` — `Wirkungsbereich` im
  Modulkopf importiert.
- `crates/krk-ui/src/appkit/anwendung.rs:976` — `#[unsafe(method(paste:))]` →
  `#[unsafe(method(filterEinfuegen:))]`.
- `crates/krk-ui/src/appkit/anwendung.rs:1048` — `sel!(paste:)` →
  `sel!(filterEinfuegen:)` im Zweig von `validateMenuItem:`.

Proben:

- `crates/krk-core/tests/belegung.rs:532` — neu:
  `cmd_f_steht_bei_zwei_funktionen_und_ist_kein_konflikt`.
- `crates/krk-ui/src/menuemodell.rs:785` — neu:
  `cmd_f_traegt_im_menue_das_filtereinfuegen_und_nicht_die_editorsuche`.
- `crates/krk-ui/src/appkit/anwendung.rs:10476` —
  `der_delegierte_beantwortet_copy_cut_und_paste` heißt jetzt
  `..._copy_cut_und_das_filtereinfuegen` und trägt eine vierte Zusicherung:
  `paste:` bleibt **un**beantwortet.
- `crates/krk-ui/src/appkit/betrachter.rs:797` — die Stellenprobe erwartet für
  `paste:` jetzt null Stellen und für `filterEinfuegen:` genau eine.
- `crates/krk-ui/src/menuemodell.rs:514` —
  `die_sechs_zugestellten_tragen_ihren_selektor_und_kein_kommando` heißt jetzt
  `die_zugestellten_...`; die Zahl steht nicht mehr im Namen.
- `crates/krk-ui/src/belegungsausgabe.rs:764` —
  `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` heißt jetzt
  `..._die_begruendungslagen_...` und prüft die fünfte Lage mit.
- `crates/krk-ui/src/belegungsausgabe.rs:875` —
  `jede_kennung_ohne_kommando_wird_vom_menue_zugestellt` nimmt
  `"filter_einfuegen"` auf.

Prosa, je Datei die Stellen, die eine falsch gewordene Aussage trugen:

- `crates/krk-ui/src/menuemodell.rs:56, 74, 134, 213, 335` — die Zahl sechs im
  Doc-Kommentar von `ZUSTELLER`, die zwei Sorten Selektor, die Doppelung bei
  `cmd+a` **und** `cmd+f`, und der Variantenname, der weniger sagt, als er
  verspricht.
- `crates/krk-ui/src/belegungsmodell.rs:42, 182, 210` — die zugestellten
  Funktionen sind nicht mehr deckungsgleich mit den Textbefehlen.
- `crates/krk-ui/src/belegungsausgabe.rs:43, 244, 268, 310, 371, 419, 752, 833`
  — vier Begründungslagen werden fünf, samt Tabelle im Modulkopf.
- `crates/krk-ui/src/appkit/anwendung.rs:78, 219, 3402, 3412` — Modulkopf
  „Drei Antworten ohne Kommando", die macOS-Untergrenzen, die zwei
  Doc-Kommentare am Vorspann und an `einfuegen_ausfuehren`.
- `crates/krk-ui/src/appkit/menue.rs:23, 105, 145, 186, 915, 959, 1013` —
  Antwortkette, `validateMenuItem:`, der Einhängepunkt, die Messtafel.
- `crates/krk-ui/src/appkit/betrachter.rs:66, 716` — Modulkopf und
  Probenkommentar.
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs:10, 34, 48, 70, 239, 250, 290,
  458, 493` — die drei Selektoren des zweiten Eingangs.
- `crates/krk-ui/src/kommandos/mod.rs:82` — dieselbe Aussage.
- `crates/krk-ui/src/kommandos/operationen.rs:18, 1403` — die
  Abweisungssätze lösen jetzt `cmd+f` aus.
- `crates/krk-ui/src/appkit/zwischenablage.rs:84, 361` — die `paste:`-Hälfte
  der Hülle ist wieder frei.
- `crates/krk-ui/src/appkit/tabelle.rs:25, 2052, 2087` — die Datenquelle nennt
  `cmd+f`.
- `crates/krk-core/src/zwischenablage.rs:44` — die zweite Deutung.
- `crates/krk-core/tests/belegung.rs:436, 471` — die zwei Prosastellen, die
  „genau einen Fall der Doppelung" sagten (Anforderung 4 des Auftrags).
- `HowTo.md:82` — der Satz, den der Nutzer im Releasepaket liest.

`CLAUDE.md:35` ist auftragsgemäß **nicht** angefasst.

## Die Stellenliste aus K14, gegen den heutigen Baum gehalten

Alle 23 Zeilen der Tabelle sind im Baum wiedergefunden und abgearbeitet. Drei
Abweichungen:

1. **`crates/krk-ui/src/appkit/betrachter.rs:68` fehlt in der Liste.** Der
   Modulkopf dort sagte „seit der Runde 21 auch auf `paste:`"; die Liste führt
   von dieser Datei allein die Zeilen 717-727 und 778-782. Nachgezogen.
2. **Die Zeile zu `belegungsausgabe.rs` nennt nur den Zweig in `wirkung` und
   die Probe `jede_kennung_ohne_kommando_wird_vom_menue_zugestellt`.** Nicht
   genannt sind der Abschnitt „Die vier Begründungslagen der dritten Spalte" im
   Modulkopf samt seiner Tabelle und die Probe
   `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`, die beide
   die Zahl vier im Text und im Namen tragen. Beide sind mitgezogen; ohne sie
   wäre der Zweig gebaut und die Zählung daneben falsch geblieben.
3. **Ein Folgefehler, den die Erhebung nicht sehen konnte.** Der Import von
   `Wirkungsbereich` in `belegungsausgabe.rs` macht den expliziten Linkpfad in
   `belegungsausgabe.rs:244` redundant, und `RUSTDOCFLAGS="-D warnings" cargo
   doc` bricht darauf ab. Der explizite Pfad ist entfernt; das Ziel des Links
   ist dasselbe geblieben.

Die Voraussage der Erhebung, `menue.rs:894` und `:929` blieben bei sechs, hat
gehalten: falsch geworden ist dort allein die Begründung, die „dieselben sechs"
mit den `gehalten_von`-Funktionen gleichsetzte. Sie sagt jetzt, dass die Liste
AppKit-Selektoren zählt und nicht zugestellte Funktionen.

## Die neuen Proben, und was sie bewusst nicht behaupten

`cmd_f_steht_bei_zwei_funktionen_und_ist_kein_konflikt`
(`crates/krk-core/tests/belegung.rs`) hält die Belegungshälfte: beide
Funktionen tragen `cmd+f`, `editor_suchen` ohne und `filter_einfuegen` mit
`gehalten_von`, die Belegung ist konfliktfrei, `filter_einfuegen` trägt kein
Kommando. Dazu die Trennung, soweit sie ohne Fenster zu messen ist: der
Nachschlag liefert für `cmd+f` `editor_suchen` — nie eine vom Menü gehaltene
Funktion —, und dessen Wirkungsbereich ist `Editor`. Genau daran fällt der
Anschlag im Dateifenster durch, und **nicht** am Fokusvorbehalt, der dort ja
steht; das ist der Unterschied zum `cmd+a`-Paar, und der Doc-Kommentar der
Probe schreibt ihn aus.

`cmd_f_traegt_im_menue_das_filtereinfuegen_und_nicht_die_editorsuche`
(`crates/krk-ui/src/menuemodell.rs`) hält die Menühälfte: der Eintrag zu
`filter_einfuegen` steht als zugestellter im Obermenü „Dateilisting", trägt
`cmd+f` und den Selektor, und `editor_suchen` trägt im Menü kein Kürzel mehr.

**Was keine Probe behauptet:** dass AppKit den Anschlag nach der Abweisung
durch den Wirkungsbereich tatsächlich an diesen Menüeintrag reicht, dass der
Delegierte daraufhin den Filtertext füllt, und dass der Eintrag „Einfügen" mit
dem Fokus in der Dateiliste grau ist. Das verlangt ein laufendes Bündel im
Vordergrund und ist Nutzerarbeit. Von „`cmd+v` schweigt im Dateifenster" ist
allein die eine Hälfte gemessen, die ohne Fenster messbar ist: der Delegierte
beantwortet `paste:` nicht mehr, geprüft über `responds_to` an seiner Klasse
und über die Stellenzählung im Quellbaum.

## Was geprüft ist

Fünf Kommandos, je exit 0, in dieser Reihenfolge gefahren:

- `cargo build --workspace`
- `cargo test --workspace` — die 52 Fehlschläge sind weg. Das Binärziel von
  `krk-ui` läuft mit 925 Proben durch (vorher 872 grün und 52 rot, also 924),
  `crates/krk-core/tests/belegung.rs` mit 55 (vorher 54). Die Differenz von je
  einer Probe sind die zwei neuen zum `cmd+f`-Paar.
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all --check`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`

`make tasten` und `make menue` sind **nicht** gefahren: sie verlangen ein
gebautes, signiertes Bündel und sind Nutzerarbeit. Nicht committet; der
Orchestrator legt beide Hälften in einen Commit.

## Abgelegte Datensätze

- `260907-2127_*_zwei-selektorlisten-im-menue-stehen-nebeneinander-und-nichts-haelt-sie-mehr-aneinander.md`
  (Defekt). `menuemodell::ZUSTELLER` und `appkit::menue::die_sechs_zugestellten`
  trugen bis heute dieselben sechs Einträge, gehalten von Prosa. Die Divergenz
  ist gewollt, und mit ihr ist auch die Prosa weg, die einen künftigen,
  ungewollten Auseinanderlauf erkennbar machte: ein achter AppKit-Selektor in
  `ZUSTELLER` bliebe ungemessen, und niemand würde rot.
- `260907-2127_*_heisst-die-variante-eintrag-textbefehl-weiter-so-jetzt-wo-eine-zugestellte-funktion-kein-textbefehl-ist.md`
  (Nutzerfrage). `Eintrag::Textbefehl` trägt jetzt eine Funktion, die weder Text
  anrührt noch im Menü „Bearbeiten" steht. Drei Möglichkeiten, Empfehlung:
  `Eintrag::Zugestellt`.
- `260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`
  geschlossen (`_o_` → `_c_`), mit dem Vermerk, dass `make tasten` und `make
  menue` seiner Abnahme nicht gefahren sind.
- `260907-2046_*_claude-md-und-howto-md-nennen-cmd-v-als-den-weg-in-den-filtertext-seit-dem-260907-traegt-ihn-cmd-f.md`
  bleibt offen, mit dem Vermerk, dass die `HowTo.md`-Hälfte erledigt ist und
  `CLAUDE.md:35` auftragsgemäß unberührt bleibt.
