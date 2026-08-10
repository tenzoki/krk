Die Typpruefung am Pfad ist im Vorschauweg geblieben, und dort blockiert sie
---
Der Defekt `260809-1652` ist im Editor behoben: die Typpruefung steht am Deskriptor, geoeffnet wird mit `O_NONBLOCK`. Die Vorschau geht denselben Weg unveraendert weiter — `symlink_metadata`, drei Zweige, die eine Roehre und ein Zeichengeraet als `Typ::Datei` einordnen, dann `std::fs::read`. Auf einer Roehre ohne Schreiber bleibt der Arbeitsfaden fuer immer stehen; auf `/dev/zero` waechst der Puffer ohne Grenze.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/vorschaumodell.rs`
**Herkunft:** Das Modul gehoert dem Navigator der Runde 1 und ist in diesem Diff nur um eine Zusicherung erweitert. Der Datensatz liegt hier, weil er das Gegenstueck zum Defekt `260809-1652` dieses Circles ist; ein Abgleich darf ihn nach `shared/issues/` verschieben.
**Zusammenhang:** `issues/260809-1652_c_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`

## Belegstellen

`crates/krk-ui/src/vorschaumodell.rs:580-589` — drei Zweige, und der dritte faengt alles:

```rust
fn typ_von(roh: &std::fs::Metadata) -> Typ {
    let art = roh.file_type();
    if art.is_symlink() {
        Typ::Verknuepfung
    } else if art.is_dir() {
        Typ::Ordner
    } else {
        Typ::Datei
    }
}
```

Eine benannte Roehre, ein Zeichengeraet, ein Blockgeraet und ein Socket werden `Typ::Datei`. Danach greift die Groessenschranke nicht, weil beide `st_size == 0` melden, und dann steht dort:

`crates/krk-ui/src/vorschaumodell.rs:555`:

```rust
match std::fs::read(pfad) {
```

`std::fs::read` ist `File::open` plus `read_to_end`, also genau das blockierende Oeffnen ohne Groessenschranke, das `krk_core::text::datei::oeffnen` seit diesem Diff nicht mehr tut. Dieselbe Zeile steht bei den Bildendungen an `:542`.

## Gemessen

Ein Rust-Programm, das nachbaut, was `laden` tut, an einer frisch angelegten Roehre ohne Schreiber:

```text
is_symlink=false is_dir=false is_file=false len=0
=> typ_von liefert Typ::Datei: true
=> groesse > TEXTGRENZE(1MB)? false
jetzt std::fs::read ...
BLOCKIERT nach 3 s
```

Und die Kennzahlen der beiden Geraetearten:

```text
/dev/zero      type=Character Device  size=0
/dev/urandom   type=Character Device  size=0
<roehre>       type=Fifo File         size=0
```

Das `/dev/zero`-Szenario habe ich **nicht** ausgefuehrt; es steht aus dem Code gelesen, und die beiden Tatsachen, auf denen es ruht, sind oben belegt.

## Fehlszenarien

1. **Die Schreibmarke wandert im Dateifenster auf eine benannte Roehre.** Unter `/tmp` und `/private/var/run` liegen welche, und die Proben dieses Projekts legen selbst welche an. Der Faden `krk-vorschau` bleibt fuer die Lebensdauer des Programms im `open` stehen. `tab.ladevorgang` bleibt `Some`, also meldet `laedt_noch()` auf Dauer `true`. Die Anzeige erholt sich mit der naechsten Auswahl (`datei_anzeigen` ersetzt den Vorgang), der Faden nicht — einer je beruehrter Roehre.
2. **Die Schreibmarke wandert auf `/dev/zero`, `/dev/random` oder `/dev/urandom`.** `read_to_end` auf einem endlosen Zeichengeraet laesst den Puffer unbegrenzt wachsen. Eine `take()`-Schranke steht hier nicht, anders als in `datei.rs:346`. Unbegrenzter Speicherverbrauch, bis das Geraet auslagert oder der Prozess stirbt.

Der Editor ist gegen beides seit diesem Diff dicht: `ohne_warten_oeffnen` oeffnet mit `O_NONBLOCK` und nimmt es danach wieder ab, und `is_file()` am `fstat`-Ergebnis weist Roehre und Geraet ab (`datei.rs:328`).

## Vorschlag

Das Mittel liegt im Baum und ist genau das, was `datei::oeffnen` jetzt tut:

1. `sys::ohne_warten_oeffnen` benutzen statt `std::fs::read`. Es ist `pub` in `krk-core` und fuer diesen Zweck entstanden.
2. Typ und Groesse am Deskriptor erheben (`fstat`) statt am Pfad, damit die Frage „ist das eine gewoehnliche Datei" und das Lesen dieselbe Datei betreffen.
3. Eine `take()`-Schranke ueber `TEXTGRENZE` beziehungsweise `BILDGRENZE`, damit eine Datei, die zwischen `stat` und `read` waechst, den Speicher nicht sprengt.

Das gilt fuer beide `std::fs::read` in `laden` — den Bildweg und den Textweg. `typ_von` bleibt, wie es ist: es beschreibt, was die Vorschau **anzeigt**, und dafuer sind drei Zweige richtig. Was fehlt, ist die Frage „ist das eine gewoehnliche Datei", und die gehoert an den Deskriptor, nicht in `typ_von`.
