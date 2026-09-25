# Drei kleine Defekte: zwei Doku-Aussagen und die Pruefordner des Vorschaumodells

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1330
**Circle:** circles/260807-2116-eingebauter-editor-mit-textmarken

## Auftrag

Drei unabhaengige offene Defekte des Circles beheben, jeder in einer eigenen
Datei. Dateigrenze: `crates/krk-core/src/verzeichnis/sys.rs`,
`crates/krk-core/src/verzeichnis/mod.rs`, die Nummernspalte und
`crates/krk-ui/src/vorschaumodell.rs`. Gesperrt: `crates/krk-ui/src/appkit/editor.rs`.

## Was getan ist

### 1. `260810-1300` — `ohne_warten_oeffnen` hat zwei Aufrufer, nicht einen

Alle drei Stellen nennen jetzt zwei. Die tragende (`sys.rs`, Doku der Funktion)
begruendet die Zielpruefung neu, statt sie zu verschieben, und die Begruendung ist
am Bestand entschieden: beide Aufrufer fragen `is_file()` am Deskriptor, aber der
Editor antwortet mit `Abweisung::KeinGueltigesZiel` samt Grund und haelt
`EDITORGRENZE`, die Vorschau faellt still in ihre Metadatenanzeige und haelt
`TEXTGRENZE` oder `BILDGRENZE`. Eine Typpruefung in der Huelle muesste eine der
beiden Antworten waehlen und kennt keine der drei Grenzen — also bleibt sie beim
Aufrufer. Das Datenflussbild im Modulkopf traegt einen zweiten Pfeil, und
`verzeichnis/mod.rs:14` spricht vom "gemeinsamen Eingang".

### 2. `260810-1314` — der Rueckverweis in der Nummernspalte

Nachgeprueft an drei Stellen in `editor.rs` (die Zeile in `textflaeche_bauen`, die
Probe `die_gebaute_flaeche_steht_auf_textkit_1`, der Modulkopf), dass der Rueckfall
auf TextKit 1 nicht mehr an der Nummernspalte haengt. Der Verweis ist damit nicht
mehr tragend, und eingetragen ist genau das: ein Absatz, der sagt, dass das
Rueckgaengig des Editors bis zum 260810-1243 mit an diesem Zugriff hing und
seither nicht mehr, und dass beim Nachziehen dieser Datei allein die Einfaerbung
der Formatansicht zu klaeren bleibt. Die Messung bleibt im Modulkopf von
`editor.rs`, wie der Datensatz es verlangt. Kein Verweis auf eine Voraussetzung,
die keine mehr ist.

### 3. `260810-1256` — sieben Proben mit festen Ordnernamen

Alle sieben nehmen den `Pruefordner`, der seit dem 260810-1247 in derselben Datei
steht. Dazugekommen ist an ihm eine Zeile, `pfad(&self) -> &Path`, denselben
Lesezugriff traegt `Pruefordner` in `krk-core/tests/verzeichnis.rs` auch. Kein
zweiter Mechanismus. Der von `krk-core` ist nicht erreichbar, weil er in einem
Testziel steht und ein Testziel eine eigene Kiste ist.

## Neu gefundene Defekte

- `issues/260810-1330_o_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`
  — zehn `Pruefordner` und zwei `Wegwerfordner`, vier davon in einer Kiste, wo
  eine Fassung genuegte. Im Circle, weil der Befund aus der Arbeit an
  `260810-1256` stammt.
- `shared/issues/260810-1330_o_der-messplan-bleibt-liegen-wenn-eine-runde-abbricht.md`
  — `krk-bench` loescht `krk-messplan-<pid>.toml` nur hinter der letzten Runde;
  neun Dateien liegen auf dem Geraet. Geteilt, weil `krk-bench` nicht zu dieser
  Runde gehoert und der Befund beim Nachsehen im Temporaerverzeichnis anfiel.

## Abnahme

Alle fuenf Kommandos gruen:

```text
cargo build --workspace                  exit 0
cargo test --workspace                   exit 0
cargo clippy --workspace --all-targets   exit 0
cargo fmt -p krk-core -- --check         exit 0
cargo fmt -p krk-ui -- --check           exit 0
```

Ein Zwischenlauf des Clippy meldete `exit 101` mit
`E0425: cannot find function verlauf_fuer_umbau` in
`crates/krk-ui/src/appkit/editor.rs:2014` — die gesperrte Datei, mitten in der
Aenderung des parallel arbeitenden Agenten und ausserhalb dieser Dateigrenze.
`cargo clippy -p krk-core --all-targets` war in derselben Minute `exit 0`. Der
Wiederholungslauf nach dem Landen jener Aenderung ist `exit 0`; die Zahlen oben
sind der letzte, vollstaendige Durchgang.

Zu Punkt 3 dazu: vor dem Lauf lagen die sieben Ordner mit festem Namen unter
`$TMPDIR` als Rueckstand aelterer Laeufe und sind geloescht worden; nach
`cargo test --workspace` und nach einem zweiten Lauf der 19 Proben des
Vorschaumodells (alle gruen) findet `ls -d "$TMPDIR"krk-vorschau-probe-*` keinen
Eintrag.

## Geaenderte Dateien

- `crates/krk-core/src/verzeichnis/sys.rs`
- `crates/krk-core/src/verzeichnis/mod.rs`
- `crates/krk-ui/src/appkit/nummernspalte.rs`
- `crates/krk-ui/src/vorschaumodell.rs`

`crates/krk-ui/src/appkit/editor.rs` ist nur gelesen, nicht angefasst.

## Anmerkung zur Dateigrenze

Die Aufgabe nannte `crates/krk-ui/src/nummernspalte.rs`; eine Datei dieses Namens
gibt es unter `src/` nicht. Gemeint und bearbeitet ist
`crates/krk-ui/src/appkit/nummernspalte.rs`, die Datei, die der Datensatz
`260810-1314` unter `Betroffen:` fuehrt.
