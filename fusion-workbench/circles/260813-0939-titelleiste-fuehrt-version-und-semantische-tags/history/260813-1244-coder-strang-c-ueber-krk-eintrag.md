# Coder: Strang C der Runde 8 — der Eintrag „Über KRK" im Anwendungsmenü

**Datum:** 260813-1244
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte C1, C2 und C3 aus
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
nicht mehr und nicht weniger. `kommandos/`, `fenster.rs`, `anwendung.rs`,
`xtask/`, `README.md` und `resources/` sind unberührt geblieben.
**Abnahme:** `make check` Exit 0 (build, test, clippy unter `-D warnings`, fmt).
Proben in `krk-ui` vorher 544 im Lauf, davon zwei neu; die Kiste stand nach
Strang B bei 542.
**Nicht gefahren:** kein `make bundle`, kein `cargo xtask bundle`. Unter
`target/KRK.app` liegt ein beglaubigtes Bündel.

## Was gebaut wurde

**C1 — der Sonderposten und sein Trenner im Menümodell**
(`crates/krk-ui/src/menuemodell.rs`).

Zwei Konstanten neben den beiden der Markdown-Ausgabe: `UEBER_BESCHRIFTUNG`
mit `"Über KRK"` und `UEBER_SELEKTOR` mit `c"orderFrontStandardAboutPanel:"`.
Dazu die Funktion `ueber_eintrag_einfuegen`, die den Sonderposten und einen
Trenner über `splice(0..0, …)` an den **Anfang** des Anwendungsmenüs stellt,
gespiegelt zu `markdownausgabe_einfuegen`, das seine beiden über das Beenden
schiebt. Gerufen wird sie in `aufbau` im bestehenden
`if bereich == Funktionsbereich::Anwendung`-Zweig, vor dem bestehenden Ruf.

**Nichts ist gewachsen ausser der Menüleiste selbst.** Kein `Kommando` (bleibt
bei 76), kein `Wirkungsbereich` (7), kein `Funktionsbereich` (9), keine Zeile in
`resources/default-keymap.toml`. Das ist die Folge davon, dass der Eintrag kein
Kürzel trägt, und die bestehenden Zählproben in `krk-core` halten die Zahlen
unverändert.

Fünf Prosastellen sind nachgezogen, weil sie das Gegenteil des Neuen
behaupteten:

- Modulkopf, Aufzählung der drei Sorten Eintrag: „Es gibt genau einen" ist zu
  „Es sind zwei" geworden, mit beiden Beschriftungen und dem Hinweis, dass die
  beiden Selektoren **nicht an derselben Stelle** beantwortet werden.
- Modulkopf, Absatz über das Menü „Bearbeiten": „das Modell führt genau einen
  Trenner" ist zu „allein die zwei Trenner des Anwendungsmenüs" geworden.
- Der Doc-Kommentar an `MARKDOWN_BESCHRIFTUNG` („des einen Sonderpostens") und
  der an `MARKDOWN_SELEKTOR` nennen den Eintrag jetzt beim Namen.
- Die Variante `Eintrag::Sonderposten` nennt beide Einträge.
- **Das Feld `selektor` hiess „Der Selektor am Anwendungsdelegierten" und heisst
  jetzt „Der Selektor, den die Antwortkette beantwortet".** Der alte Wortlaut
  war für den neuen Eintrag schlicht falsch: `orderFrontStandardAboutPanel:`
  steht an `NSApplication`, und die Kette erreicht `NSApplication` **vor** dem
  Delegierten. Der Kommentar sagt das jetzt ausdrücklich und trennt, was
  gerufen wird, von dem, wer antwortet. Zwei Lesarten desselben Feldes sind
  damit nicht entstanden — das ist der Punkt, den der Spec unter „Offen für den
  Planner" nennt.

Zwei Proben:

- **Neu:** `der_ueber_eintrag_steht_ganz_oben` prüft die Stelle **relativ**, wie
  `der_markdown_eintrag_steht_ueber_dem_beenden` es tut: der Eintrag wird
  gesucht, seine Umgebung gemessen. Sie hält dreierlei fest — vor ihm steht
  nichts, unter ihm steht ein Trenner, und er trägt den Selektor des
  Standard-Dialogs. Der Selektorname steht in der Zusicherung ausgeschrieben
  und nicht als `UEBER_SELEKTOR`; gegen die Konstante geprüft wäre sie eine
  Tautologie, und ein Tippfehler darin bliebe unbemerkt.
- **Umbenannt:** `die_leiste_traegt_genau_einen_zusatz` heisst jetzt
  `die_leiste_traegt_zwei_sonderposten_und_zwei_trenner`. Sie zählt die beiden
  Sorten **getrennt** statt eine Summe von vier: eine Summe bliebe auch dann
  stehen, wenn ein Sonderposten ohne seinen Trenner dazukäme und ein anderer
  seinen verlöre. Der eigene Doc-Kommentar der Probe hatte gesagt, was zu tun
  ist, wenn die Zahl wächst.

**C2 — kein Programmtext, nur Prosa** (`crates/krk-ui/src/appkit/menue.rs`).

Der Sonderposten-Zweig in `umsetzen` trägt über `Sel::register` jeden
Selektornamen, den das Modell führt, und `roher_befehl` setzt kein Ziel. Damit
läuft der neue Eintrag ohne eine Zeile Änderung; KRK baut keine eigene Fläche
und implementiert keine Methode dafür.

Drei Prosastellen ziehen nach: der Absatz über das Ziel `nil` nennt jetzt auch
`orderFrontStandardAboutPanel:` und die Station **vor** dem Delegierten; aus
„Ein Eintrag trägt bewusst gar keine Kennung, und er ist der einzige" sind
zwei geworden, mit beiden Entscheiddaten (260811-0110 für die Markdown-Ausgabe,
260813-1010 für den Über-Eintrag); und ein eigener Absatz hält fest, warum es
**keinen zweiten Zweig in `validateMenuItem:`** gibt.

Der letzte Absatz ist der, an dem ein späterer Leser sonst hängen bliebe.
`validateMenuItem:` (`appkit/anwendung.rs`) fragt zuerst nach der Aktion und
antwortet für jede fremde `true`; beide Sonderposten fallen in genau diesen
Zweig, wie der Markdown-Eintrag es heute schon tut. Ein eigener Zweig für den
Über-Eintrag wäre die **erste** Sonderbehandlung eines einzelnen Eintrags an
dieser Stelle gewesen. Der Absatz sagt daneben, was er **nicht** behauptet: ob
AppKit selbst am Menü etwas ändert, solange ein Blatt steht, entscheidet diese
Regel nicht und ist am laufenden Bündel nachzusehen.

**C3 — die Zählprobe** (`crates/krk-ui/src/appkit/titelzusatz.rs`, nur das
Prüfmodul).

`nur_eine_stelle_im_baum_setzt_namen_und_version_zusammen` liest den Baum über
`quellbaum::quelldateien()` und hält C5.4: genau eine Datei setzt Namen und
Version zusammen, und es ist diese. Die Nadel steht wie jede Zählprobe dieses
Baums mit `concat!` zusammengesetzt da, damit sie sich nicht selbst findet.

**Gezählt werden zuerst Dateien und dann Fundstellen, und das hat einen
Grund.** In dieser einen Datei steht die Zusammensetzung zweimal: in
`beschriftung` und in der Zusicherung von
`die_beschriftung_ist_name_leerzeichen_version`, die genau sie prüft. Eine
reine Fundstellenzählung über den Baum stünde deshalb bei zwei und sagte nichts
mehr. Die zweite Zusicherung der Probe hält die Zahl in dieser Datei
ausdrücklich auf zwei; eine dritte Fundstelle hier wäre eine zweite
Zusammensetzung und fände sonst keine Probe.

Der Doc-Kommentar hält fest, was der Dialog wirklich zeigt: `CFBundleName`
steht in `resources/Info.plist` auf `KRK`, `CFBundleShortVersionString` trägt
den Platzhalter, den `cargo xtask bundle` zur Bauzeit durch die Zahl aus der
`Cargo.toml` ersetzt, und `CFBundleVersion` steht auf `1`. AppKit schreibt
daraus **seine eigene** Zeile, und die lautet nicht Zeichen für Zeichen wie
`beschriftung()`. Gleich ist die Zahl und ihre Quelle; genau das sagt C5.4 zu,
die Schreibweise sagt es nicht zu. Die Blindheit der Nadel steht daneben: eine
Zusammensetzung über `format!` oder über zwei Variablen fiele ihr nicht auf.

## Beide neuen Proben sind gegen einen Fehler gefahren worden

Eine Zählprobe, die nie rot war, ist eine Behauptung. Beide sind deshalb
einmal absichtlich gebrochen worden und danach zurückgesetzt:

- `ueber_eintrag_einfuegen` auf das **Ende** der Liste umgestellt:
  `der_ueber_eintrag_steht_ganz_oben` fällt mit `left: 5, right: 0` und druckt
  die fünf Einträge, die dann davor stehen.
- Eine zweite `concat!("KRK ", env!(…))`-Stelle in `fenstertitel.rs`:
  `nur_eine_stelle_im_baum_setzt_namen_und_version_zusammen` fällt.

Beide Dateien standen danach wieder unverändert; `git status` führt allein die
drei Dateien dieses Strangs.

## Was aufgefallen ist und nicht in diesen Strang gehörte

Der Doc-Kommentar an `validateMenuItem:` (`crates/krk-ui/src/appkit/anwendung.rs`)
zählt auf, wer in den `true`-Zweig fällt: „die sechs Textbefehle (C2.8) und der
Eintrag der Markdown-Ausgabe (C2.9)". Nach dieser Runde sind es zwei
Sonderposten. Der Satz wird dadurch nicht falsch, er zählt nur unvollständig
auf. `anwendung.rs` steht ausdrücklich ausserhalb dieses Strangs, deshalb ist
die Stelle unberührt geblieben und hier vermerkt.

## Abnahme

```
make check   → Exit 0
```

build, `cargo test --workspace`, `cargo clippy --workspace --all-targets --
-D warnings` und `cargo fmt --all --check`, in dieser Reihenfolge, alle vier
grün. Clippy ist die eigentliche Prüfung, weil `unused_must_use` erst unter
`-D warnings` ein Fehler ist.

Die drei betroffenen Proben im Lauf:

```
test appkit::titelzusatz::tests::nur_eine_stelle_im_baum_setzt_namen_und_version_zusammen ... ok
test menuemodell::tests::der_ueber_eintrag_steht_ganz_oben ... ok
test menuemodell::tests::die_leiste_traegt_zwei_sonderposten_und_zwei_trenner ... ok
```

## Geänderte Dateien

- `crates/krk-ui/src/menuemodell.rs` (C1)
- `crates/krk-ui/src/appkit/menue.rs` (C2, nur Prosa)
- `crates/krk-ui/src/appkit/titelzusatz.rs` (C3, nur das Prüfmodul)
- `fusion-workbench/circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`
  — C1, C2 und C3 auf `[DONE]`

## Was offen bleibt

Der Über-Dialog selbst ist an keiner Probe zu sehen. C5.1 verlangt neben der
Probe das **Bild** aus dem Bündel, C5.3 das Fenster, C5.5 den Entwicklungslauf
ohne Bündel und C5.6 die Beobachtung, ob ein Tastenbefehl von KRK wirkt,
solange der Dialog steht. Alle vier gehören der Abnahme in Strang E und damit
dem Nutzer: der Lauf verlangt KRK im Vordergrund.
