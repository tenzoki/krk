# Schritt 1: Eine beschädigte Ablagedatei wird zur Seite gelegt

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 1
**Verification:** `make check` — exit 0

---

## Was gebaut wurde

`Ablage::laden` legt den gelesenen Text einer beschädigten Ablagedatei unter
festem Namen daneben, bevor der Auslieferungszustand einspringt. Die Regel gilt
für alle vier Dateien unter `~/Library/Application Support/KRK/`, weil alle vier
durch dieselbe Funktion gehen und dort keinen eigenen Zweig haben.

**`crates/krk-core/src/ablage/atomar.rs`**

- `BESCHAEDIGTENDUNG = "beschaedigt"` und `beiseitepfad(ziel)` neben
  `NACHBARENDUNG` und `nachbarpfad`. Beide leiten den Namen fest ab und tragen
  keine Laufnummer, aus entgegengesetzten Gründen; beide Doc-Kommentare sagen
  das und verweisen aufeinander, damit niemand die eine Begründung für die
  andere hält.
- Die vier Zeilen der Ableitung stehen einmal, in der privaten `mit_endung`.
  Zwei Kopien wären zwei Stellen, an denen der Umgang mit einem Pfad ohne
  Dateinamen auseinanderlaufen könnte.

**`crates/krk-core/src/ablage/mod.rs`**

- Neue Aufzählung `Beiseite` mit vier Werten ohne Auffangzweig: `Nicht`,
  `Gesichert(PathBuf)`, `SchonVorhanden(PathBuf)`, `Gescheitert(String)`.
  `Ersetzung` trägt sie als Feld `beiseite`.
- `beiseite_legen` in der Reihenfolge Pfad bilden, `try_exists`, `atomar::schreiben`.
  Trägt `#[must_use]`. Ein `Err` aus `try_exists` fällt in `Gescheitert`; die
  Fallunterscheidung ist damit über den Rückgabewert total.
- `Display for Ersetzung` verzweigt über alle vier Werte. Für `Nicht` steht der
  Satz Wort für Wort wie vor dieser Runde; die beiden Sicherungsfälle beginnen
  mit dem, was der Nutzer tun kann, und nennen beide Pfade; `Gescheitert` nennt
  keinen Pfad, weil unter ihm nichts liegt.
- Der Modulkopf beschrieb den Rückfall auf den Auslieferungszustand ohne die
  Sicherung. Er hat einen neuen Abschnitt bekommen, der die vier Regeln des
  Vorgangs trägt: nur bei `Beschaedigt`, kopieren statt verschieben, eine
  dastehende Sicherung bleibt, und der Weg ist `atomar::schreiben`.

**`crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/src/ablage/einstellungen.rs`**

Das neue Feld hält den Bau an jeder Stelle an, die `Ersetzung` baut. Die drei
Stellen außerhalb von `Ablage::laden` tragen `Beiseite::Nicht`, jede mit einer
Zeile, warum dort nichts zu sichern ist. Der Plan führt diese beiden Dateien
nicht in seiner Dateiliste; der Übersetzer verlangt sie.

**`crates/krk-core/tests/ablage.rs`**

Sechs neue Proben, dazu eine Zusage an einer bestehenden:

| Probe | Kriterium |
|---|---|
| `jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt` | C3.1, C3.3, C3.4 |
| `der_name_der_sicherung_haengt_die_endung_an_und_ist_keine_ablagedatei` | C3.1 |
| `eine_zweite_beschaedigung_laesst_die_erste_sicherung_unangetastet` | C3.2 |
| `eine_fehlende_und_eine_nicht_lesbare_datei_werden_nicht_zur_seite_gelegt` | C3.5 |
| `ein_gescheitertes_zur_seite_legen_wird_gemeldet_und_verspricht_keine_datei` | C3.6, C3.8 |
| `die_meldung_unterscheidet_die_vier_lagen_und_bleibt_einzeilig` | C3.7, C3.8 |
| `eine_bookmarks_toml_aus_der_zeit_vor_den_textmarken_bleibt_lesbar` (erweitert) | C3.9 |

## Zwei Entscheidungen, die im Bericht stehen sollten

**Die Probe zur alten `bookmarks.toml` ist erweitert und nicht ein zweites Mal
geschrieben worden.** Die Verträglichkeitsprobe der Editor-Runde prüft bereits,
dass eine Datei allein aus `name` und `ordner` gelesen wird, nicht als
beschädigt gilt und alle drei Lesezeichen liefert — das ist Wort für Wort, was
Festlegung D verlangt. Neu ist allein die Zusage der Runde 6, dass daneben nichts
zur Seite gelegt wird; sie hängt an derselben Datei und demselben Lesevorgang.
Eine zweite Probe daneben wäre dasselbe Ereignis zweimal. Der Doc-Kommentar der
Probe sagt jetzt beides und nennt die Stelle, an der der Gegenbeweis steht.

**Das Scheitern wird an der Nachbardatei des atomaren Schreibens ausgelöst.**
Ein Ordner unter `bookmarks.toml.beschaedigt.neu` lässt `atomar::vorbereiten`
scheitern. Das ist zugleich der Nachweis für C3.6: gäbe es einen zweiten
Schreibweg, käme die Sicherung trotzdem zustande. Der Weg kommt ohne entzogene
Rechte aus und läuft deshalb auch unter `root`.

## Abnahme

`make check` — Exit 0. Vier Kommandos: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`.

Neun der zehn Kriterien von C3 sind damit ohne laufendes Bündel nachgewiesen:
C3.1 bis C3.9. Offen bleibt C3.10, die Meldung in der Statuszeile beim Start;
sie ist am laufenden `KRK.app` im Vordergrund zu sehen und damit Nutzerarbeit.

## Datensätze

- `decisions/260812-1000_i_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`
  — von beantwortet auf umgesetzt gezogen, mit Verweis auf die beiden Stellen im
  Code. Der Commit steht noch aus; die Zeile sagt es.
- `decisions/260812-1000_a_wie-erfaehrt-der-nutzer-dass-eine-ablagedatei-zur-seite-gelegt-wurde.md`
  — bleibt beantwortet. Der Satz steht, seine Zustellung hängt an C3.10 und an
  der Statuszeile aus den Schritten 10 und 11.
- `issues/260812-1204_o_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`
  — bei der Umsetzung gefunden, nicht behoben. `belegung::laden` baut seine
  `Ersetzung` eine Ebene über `Ablage::laden` und sieht das Zur-Seite-Legen
  nicht; eine `keymap.toml`, die gültiges TOML ist und eine unbekannte
  Kommandokennung nennt, bleibt damit ungesichert.
