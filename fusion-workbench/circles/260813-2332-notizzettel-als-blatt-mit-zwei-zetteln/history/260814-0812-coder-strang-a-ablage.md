# Coder — Strang A: zwei Zetteldateien, ein Leseweg, ein Schreibweg

**Date:** 260814-0812
**Agent:** coder
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Plan:** `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, Schritte 1 bis 5
**Status:** Complete

---

## Auftrag

Strang A des Plans, und nur er. Nicht angefasst: `crates/krk-ui/` und
`resources/` — dort hat gleichzeitig ein zweiter coder Strang B gefahren.

**Eine Ausnahme von dieser Grenze war unvermeidlich und ist unten unter
„Die eine Zeile in `krk-ui`" ausgeschrieben.**

## Was gebaut ist

### Schritt 1 — `crates/krk-core/src/text/datei.rs`

Neu `pub enum Textstand { Text, Unlesbar { datei, grund }, KeinGueltigesZiel {
grund, fehlt } }` und `pub enum Unlesbarkeit { ZuGross(u64), KeinText }`, beide
vollständig und ohne Auffangzweig. Neu `pub fn lesen(pfad) -> Textstand` mit dem
bisherigen Rumpf von `oeffnen`: `ohne_warten_oeffnen`, `metadata()` am offenen
Deskriptor, Typprüfung, `EDITORGRENZE`, `read_to_end` hinter
`take(EDITORGRENZE + 1)`, `einlesen`. `EDITORGRENZE` steht weiterhin an genau
einer Stelle.

`oeffnen` ist die Übersetzung des Befundes in eine `Abweisung` und hat Signatur
wie Rückgabewerte behalten; der Editor sieht von der Zerlegung nichts. Die
bestehenden Proben zu `oeffnen` in `tests/text.rs` sind unverändert grün.

Das Zurückspulen entsteht an **einer** Stelle, der Hilfsfunktion `unlesbar`; ein
zweiter Bauplatz für `Textstand::Unlesbar` könnte es vergessen.

**Zwei Abweichungen vom Wortlaut des Plans, beide begründet:**

- **`KeinGueltigesZiel` trägt ein Feld `fehlt` und nicht nur einen `String`.**
  Der Plan verlangt an zwei Stellen Unvereinbares, wenn man ihn wörtlich nimmt:
  „der fehlende Fall kommt als `KeinGueltigesZiel` herein, und `text_laden`
  macht daraus einen leeren Zettel **ohne Meldung**" — aus einer Zeichenkette
  ist „nicht da" aber nicht verlässlich zu erkennen. Ein eigener vierter Wert
  hätte aus den zugesagten vier Ausgängen fünf gemacht und den Editor zu einer
  Unterscheidung gezwungen, die er nicht trifft. Ein Feld an einem der vier
  Ausgänge hält beides: die Zahl der Ausgänge und das Abnahmekriterium „Fehlt
  eine Zetteldatei, ist der Zettel leer, und KRK meldet keinen Fehler".
- **Ein gescheitertes Zurückspulen wird gemeldet und nicht verschwiegen.** Es
  kommt dann `KeinGueltigesZiel` zurück statt `Unlesbar`. Ein Deskriptor an
  unbekannter Stelle ergäbe eine abgeschnittene Sicherung, die aussieht wie eine
  vollständige. Auf einer gewöhnlichen Datei — und eine andere kommt bis dorthin
  nicht — ist der Fall nicht zu erreichen.

### Schritt 2 — `atomar` schreibt aus einem Leser

`vorbereiten(ziel, &mut impl Read)` und `schreiben(ziel, &mut impl Read)`; im
Rumpf steht `io::copy` statt `write_all`, `sync_all` und das zweistufige
Umbenennen sind unangetastet. Der Modulkopf trägt den Absatz, warum ein Leser
und keine Zeichenkette.

Die fünf Aufrufstellen schreiben `&mut text.as_bytes()`:
`Zugang::sichern`, `Zugang::beiseite_legen`, `einstellungen::anlegen_falls_fehlt`,
`text::datei::sichern` und `belegungsausgabe::in_ordner_schreiben`. Dazu vier
Aufrufstellen in `tests/ablage.rs`.

### Schritt 3 — `crates/krk-core/src/ablage/pfade.rs`

Neu `pub enum Zettel { Erster, Zweiter }` mit `ALLE`, `index()`, `andere()` und
denselben Ableitungen wie `Fensterseite`, dessen Bauform und Begründung es
übernimmt. `Datei` bekommt `Zettel(Zettel)`, `Datei::ALLE` ist `[Datei; 6]`,
`dateiname()` liefert `note-1.txt` und `note-2.txt`. Neu `pub enum Format {
Toml, Text }` und `pub const fn Datei::format()`, vollständig ohne
Auffangzweig. Der Modulkopf sagt „sechs Dateien in zwei Formaten" und schreibt
aus, warum die Zettel kein TOML tragen.

### Schritt 4 — `crates/krk-core/src/ablage/mod.rs`

`beiseite_legen(&self, datei, &mut impl Read) -> Beiseite`; die drei Regeln
stehen Wort für Wort, wo sie standen. `Grund` hat die vierte Variante
`ZuGross { groesse }`; `beschreibung()` und `einzelheit()` sind vollständige
Fallunterscheidungen.

Neu `Zugang::text_laden(Datei) -> Geladen<String>` mit der Übersetzung der vier
Ausgänge und `Zugang::text_sichern(Datei, &str)`. `laden`, `sichern`,
`text_laden` und `text_sichern` tragen je einen `debug_assert_eq!` auf das
erwartete `Format`.

**Eine Abweichung:** `Grund::einzelheit()` gibt jetzt `Cow<'_, str>` zurück und
nicht mehr `&str`. `ZuGross` trägt eine Zahl und keinen Satz, und der Satz
entsteht beim Lesen statt beim Erzeugen — sonst stünde `EDITORGRENZE` ein
zweites Mal im Baum, an der Stelle, die den Wert baut. Die vier bestehenden
Gründe reichen ihren Text weiterhin ohne Kopie durch; die fünf Aufrufstellen in
den Proben brauchten keine Änderung.

### Schritt 5 — die Proben

`tests/ablage.rs`: die vier TOML-Rundläufe laufen über die neue Hilfsfunktion
`toml_dateien()`, also über `Datei::format()` und nicht über eine zweite Liste.
Die drei Fragen nach Pfad, Name und Nichtanlage bleiben auf `Datei::ALLE` und
decken die zwei Zettel mit ab; die erwartete Namensliste ist um zwei gewachsen.
`vier_ersetzungen` heißt jetzt `ersetzungen_der_toml_dateien`.

Fünf neue Proben zum Zettel: fehlende Datei ohne Meldung, Rundlauf mit
unverändertem Text, ungültige Bytefolge mit Sicherung und Meldung, zweite
ungültige Fassung ohne Antasten der ersten, Datei über `EDITORGRENZE` nicht
geladen und beiseitegelegt.

`tests/text.rs`: eine Probe `der_befund_deckt_alle_vier_ausgaenge_und_spult_zurueck`
fährt `lesen` gegen alle vier Ausgänge und prüft in beiden `Unlesbar`-Fällen,
dass der Deskriptor am Anfang steht und die ganze Datei trägt.

`tests/baum.rs`: der Kommentar nennt „Vier Schreiber hinter einem `Zugang`"; die
Liste der fünf Dateien ist unangetastet, und
`nur_benannte_dateien_erreichen_das_atomare_schreiben` ist grün geblieben, ohne
angefasst zu werden.

## Die eine Zeile in `krk-ui`

`crates/krk-ui/src/belegungsausgabe.rs` ist die fünfte Aufrufstelle von
`atomar::schreiben` und steht als solche im Dateienverzeichnis von Schritt 2 des
Plans. Ohne sie übersetzt der Arbeitsbereich nicht, und kein Schritt darf den
Baum rot zurücklassen. Geändert sind zwei Zeilen in `in_ordner_schreiben`:

```rust
let text = markdown(belegung);
match atomar::schreiben(&ziel, &mut text.as_bytes()) {
```

Die Datei gehört zu keinem Schritt von Strang B; eine Kollision mit dem zweiten
coder war damit nicht zu erwarten und ist nicht eingetreten.

## Was ausdrücklich **nicht** gebaut ist

Die Zetteldateien werden beim Start **nicht** gelesen. `text_laden` hat in
diesem Strang keinen Aufrufer im Programm — er entsteht mit Schritt 12 in
Strang C, beim ersten Öffnen des Zettels. Der Spec sagt das unter dem Verhältnis
zu L4 ausdrücklich zu.

## Prüfung

`make check` — Bau, Proben, `clippy -D warnings` und `fmt --check` in einem Zug,
Rückgabewert 0. Kein `make bundle`, kein `cargo xtask`.
