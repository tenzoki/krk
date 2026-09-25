# S10: Die eine Größen- und Typprüfung vor dem Öffnen

- Agent: `coder`
- Datum: 260809-1525
- Plan: `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Phase B, Schritt 10
- Bindender Datensatz: `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260807-2147_a_welche-dateien-oeffnet-der-editor-ueberhaupt.md`
- Status: Complete

## Was umgesetzt ist

`text::datei::oeffnen(&Path) -> Result<String, Abweisung>` in
`crates/krk-core/src/text/datei.rs` ist die eine Stelle, die eine Datei für den
Editor liest. F4 aus C2, der Menüweg und der Sprung auf eine Textmarke aus C6
rufen dieselbe Funktion; ein zweiter Leseweg entsteht nicht. Der Modulkopf sagt
das im Abschnitt „Der eine Weg von einem Pfad zu einem Stand" und nennt den
Grund, den `kommandos::pfadeingabe` für den Pfad schon trägt: die erste
Abweichung zwischen zwei Wegen fände keine Prüfung.

Die Reihenfolge steht als Fließtext an der Funktion und ist bindend:

```text
metadata (nicht symlink_metadata)  ──> die Verknüpfung gilt nach ihrem Ziel
        │
        v
!is_file()                         ──> KeinGueltigesZiel (Ordner voran)
        │
        v
len() > EDITORGRENZE               ──> ZuGross, ohne ein Byte zu lesen
        │
        v
File::open + take(GRENZE + 1)      ──> die Bytes
        │
        v
einlesen (String::from_utf8)       ──> NichtAlsTextLesbar oder der Stand
```

`EDITORGRENZE = 16 * 1024 * 1024` trägt die drei Aussagen, die der Schritt
verlangt: der Nutzer hat sie am 260808-0017 gewählt, sie ist die zweite Zahl
neben `TEXTGRENZE` der Vorschau und beide tragen dieselbe Regel, und
`speculation:` sie ist ein Vorschlag und keine gemessene Größe.

`Abweisung` hat drei Werte ohne Auffangzweig: `KeinGueltigesZiel { pfad, grund }`,
`ZuGross { pfad, groesse }`, `NichtAlsTextLesbar { pfad }`. `meldung()` ist eine
vollständige Fallunterscheidung darüber und liefert je einen Satz für die
Statuszeile aus C1; eine zweite Meldefläche entsteht nicht.

## Was die drei Gründe unterscheidet

- **`KeinGueltigesZiel`** — der Pfad ist nichts, was ein Texteditor öffnen
  könnte. Der Ordner ist der Fall, den der Datensatz namentlich als sicher
  abgewiesen nennt; er braucht keine eigene Regel, weil er keine gewöhnliche
  Datei ist und daran scheitert. Denselben Grund bekommen der fehlende Pfad, das
  fehlende Leserecht, die Verknüpfung ins Leere und alles, was keine gewöhnliche
  Datei ist. Der `grund` trägt entweder den Systemfehler oder die Art.
- **`ZuGross`** — über `EDITORGRENZE`, also gar nicht erst gelesen. Der Wert
  führt die Größe mit, wie `stat(2)` sie **vor** dem Lesen gemeldet hat.
- **`NichtAlsTextLesbar`** — gelesen, aber kein gültiges UTF-8. An diesem Wert
  hängt die bindende Zusage des Datensatzes: kein Weg darf eine Datei beim
  Sichern verändern, die der Editor nicht vollständig und verlustfrei als Text
  gelesen hat. Wer hier abweist, statt mit Ersatzzeichen zu öffnen, hält sie ein.

Das neunte Abnahmekriterium von C2 verlangt, „zu groß" von „nicht als Text
lesbar" zu unterscheiden; die eine Antwort lädt zum Teilen der Datei ein und die
andere nicht.

**Die Byteangaben in `meldung()` stehen roh und nicht in MB.** Der
menschenlesbare Größensatz des Programms ist `menge` in
`krk-ui/src/kommandos/operationen.rs` und liegt in der anderen Kiste; ihn hier
nachzubauen hieße, zwei Schreibweisen für dieselbe Größe zu haben. Die Ansicht
kann aus den Feldern des Wertes ihren eigenen Satz bauen.

## Wie belegt ist, dass die Größe vor dem Lesen geprüft wird

Der Nachweis steht an den **Rechten** und nicht an der Laufzeit, und er ist
damit deterministisch. Zwei gleich angelegte Löcher im Prüfordner, beide auf
Rechte `000` gesetzt, um genau ein Byte verschieden:

```text
  EDITORGRENZE + 1 Bytes, Rechte 000  ──> ZuGross
  EDITORGRENZE     Bytes, Rechte 000  ──> KeinGueltigesZiel (Lesefehler)
```

Die zweite Zeile ist die tragende: sie zeigt, dass unterhalb der Grenze
**wirklich** geöffnet und gelesen wird. Käme die Größenprüfung erst nach dem
Lesen, müsste die erste Zeile denselben Lesefehler melden wie die zweite.

**Gegengeprüft, nicht behauptet.** Am 260809 habe ich die Größenprüfung
versuchsweise hinter das Lesen verschoben und die Probe laufen lassen: genau
`eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden` fällt dann,
und keine andere der zwanzig. Danach zurückgesetzt; der Bau ist wieder grün.
Der Nachweis ist damit nachweislich load-bearing und keine Probe, die auch bei
falscher Reihenfolge durchliefe.

Die Probe tritt zur Seite, wenn die Rechtesperre auf der laufenden Kennung nicht
wirkt (root liest auch eine gesperrte Datei). Gefragt wird nicht die
Benutzerkennung, sondern die Wirkung.

Daneben steht eine zweite Probe: ein Loch von zwei Gigabyte — die Protokolldatei
aus dem Datensatz — wird in Mikrosekunden abgewiesen, mit einer großzügigen
Zeitschranke von 500 ms, die nicht messen, sondern nur den Unterschied zwischen
Mikrosekunden und Sekunden treffen soll. Die Löcher entstehen über `set_len` und
kosten auf APFS weder Platz noch Zeit.

## Drei Abweichungen von der Schrittbeschreibung

**1. Die Zusicherung zur Übersetzungszeit steht nur halb.** Der Schritt verlangt
`const _: () = assert!(EDITORGRENZE > vorschau-TEXTGRENZE)`. `TEXTGRENZE` liegt
in `krk-ui/src/vorschaumodell.rs:83`, `krk-ui` hängt von `krk-core` ab und nicht
umgekehrt, also kann `krk-core` die Zahl der Vorschau nicht benennen. In
`datei.rs` steht deshalb `assert!(EDITORGRENZE > 1024 * 1024)` mit der Zahl statt
dem Bezug. Sie fängt ein **Absenken** der Editorgrenze unter die Vorschaugrenze;
sie fängt **nicht**, dass jemand die Vorschaugrenze über 16 MB anhebt. Der
Kommentar an der Zusicherung sagt das ausdrücklich, damit niemand die halbe für
die ganze hält. Die vollständige gehört nach `krk-ui`, wo beide Zahlen sichtbar
sind, also an S23; festgehalten in
`issues/260809-1610_o_die-zusicherung-editorgrenze-groesser-textgrenze-laesst-sich-in-krk-core-nur-halb-schreiben.md`.

**2. Die Grenze wird eingehalten und nicht nur vorhergesagt.** Die Prüfung fragt
`stat(2)`, und zwischen `stat` und `read` kann eine Datei wachsen — eine
wachsende Protokolldatei ist genau der Fall, für den ein Nutzer den Editor
aufmacht. Gelesen werden deshalb über `Read::take` höchstens `EDITORGRENZE + 1`
Bytes, und kommt das eine Byte zuviel an, wird abgewiesen; gemeldet wird dann die
Größe von jetzt und nicht die von vorhin. Ohne diese Schranke wäre „die Datei
steht zu keinem Zeitpunkt vollständig im Arbeitsspeicher" eine Vorhersage aus
einer alten Auskunft, mit ihr ist es eine Eigenschaft der Bauart.

**3. `crates/krk-core/src/text/mod.rs` ist um eine Zeile erweitert**,
`pub use datei::Abweisung;`, rein additiv. Sie folgt der dort festgehaltenen
Regel, dass die Typen im Wiederausfuhrblock stehen und die Funktionen nicht;
`oeffnen` bleibt deshalb unter seinem Modulnamen.

## Was der Typ erzwingt und was nicht

Die Fallunterscheidung in `is_file()` fasst Ordner, benannte Röhren, Sockel und
Gerätedateien in **einem** Zweig zusammen und unterscheidet allein im Meldetext.
Das ist nicht Sparsamkeit, sondern die überschneidungsfreie und vollständige
Form: eine eigene Regel je Art wäre der Anfang des Sonderfallsaums, den die
Maxime „supersimpel" ausschließt. Ein Nebeneffekt trägt: `File::open` auf eine
benannte Röhre hängt, bis jemand hineinschreibt, und die Artprüfung steht vor dem
Öffnen und nicht erst vor dem Lesen.

## Abnahme

Alle vier Abnahmekommandos gefahren:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | Finished, ohne Warnung |
| `cargo test --workspace` | `tests/text.rs` 20 von 20; zwei Fehlschläge in `tests/belegung.rs`, siehe unten |
| `cargo clippy --workspace --all-targets -- -D warnings` | Finished, ohne Befund |
| `cargo fmt --all --check` | für die drei Dateien dieses Schritts sauber, siehe unten |

**Die zwei Fehlschläge gehören nicht zu diesem Schritt.**
`die_auswahl_und_der_fokuswechsel_wirken_in_beiden_bereichen` und
`die_y_kuerzel_liegen_auf_kvk_ansi_y_und_die_stelle_kvk_ansi_z_ist_unbelegt`
stehen in `crates/krk-core/tests/belegung.rs`. Diese Datei, dazu
`crates/krk-core/src/tasten/belegung.rs` und `resources/default-keymap.toml`,
sind für einen parallel laufenden Schritt reserviert und stehen laut
`git status` als geändert im Baum. S10 fasst keine von ihnen an.

**`cargo fmt --all --check` meldet einen Rest in `crates/krk-core/src/tasten/belegung.rs`**,
derselben reservierten Datei. Formatiert habe ich deshalb gezielt mit
`rustfmt --edition 2024` über die drei Dateien dieses Schritts, statt mit
`cargo fmt --all` in die Arbeit des anderen Schritts zu schreiben.

## Dateien

- `crates/krk-core/src/text/datei.rs` — erweitert: `EDITORGRENZE`, die
  Zusicherung, `Abweisung`, `Abweisung::meldung`, `oeffnen`; Modulkopf und
  ASCII-Überblick nachgezogen
- `crates/krk-core/src/text/mod.rs` — eine additive Zeile
- `crates/krk-core/tests/text.rs` — sieben neue Proben und drei Helfer am
  `Pruefordner` (`luecke`, `unterordner`, `verknuepfung`), Modulkopf nachgezogen
- `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-1610_o_die-zusicherung-editorgrenze-groesser-textgrenze-laesst-sich-in-krk-core-nur-halb-schreiben.md` — neu
- Plan: Schritt 10 auf `[DONE]`, mit Umsetzungsnotiz
