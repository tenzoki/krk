# S12: Die Suche in der Nähe einer Textmarke

---
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` `#### 12.`
**Bindender Datensatz:** `decisions/260807-2147_a_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md` (Möglichkeit 1, entschieden am 260808-0017)

---

## Was gebaut wurde

`krk-core::text::marke` beantwortet die eine Frage, die beim Sprung auf eine
Textmarke offen ist: wohin die Schreibmarke gehört, wenn die Datei sich seit dem
Merken geändert hat.

```rust
pub const NAHFENSTER: u32 = 50;
pub enum Fund { Getroffen, Verschoben, NichtGefunden }
pub struct Markensprung { pub zeile: u32, pub sprung: Zeilensprung, pub fund: Fund }
pub fn wiederfinden(text: &str, zeile: u32, zeileninhalt: &str) -> Markensprung
```

Die Reihenfolge ist die des Datensatzes. Steht der gemerkte Inhalt auf der
gemerkten Nummer, trifft die Marke sofort, und keine Suche läuft an. Sonst wird
von der gemerkten Nummer aus nach außen gesucht, Abstand für Abstand, bis
`NAHFENSTER`. Bleibt die Suche ohne Treffer, führt die Marke trotzdem an die
gemerkte Nummer und trägt `Fund::NichtGefunden`; melden ist Sache des Aufrufers
und kommt in Phase G.

Die Funktion ist rein: sie bekommt den gehaltenen Stand als Zeichenkette und
fasst kein Dateisystem an. Sie nimmt die beiden gemerkten Felder als einzelne
Werte entgegen und nicht als `Lesezeichen`, damit die Textrechnung die Ablage
nicht kennen muss.

## Die Grenze der Fähigkeit steht im Code, nicht nur im Datensatz

Der Modulkopf von `marke.rs` schreibt sie aus: **der gemerkte Zeileninhalt ist
keine eindeutige Kennung.** Eine Marke auf einer Zeile, die in der Datei mehrfach
steht, kann nach einer Änderung von außen nicht zuverlässig wiedergefunden
werden. Die Suche liefert dann den nächstliegenden Treffer, und das ist eine
nachvollziehbare Antwort, aber nicht notwendig die gemeinte. Keine Reichweite
ändert daran etwas.

Die Probe `kommt_der_inhalt_im_fenster_zweimal_vor_gewinnt_der_naechstliegende`
hält den Fall fest, statt ihn zu verdecken: zwei Zeilen `}` im Fenster, und die
weiter entfernte verliert.

`NAHFENSTER` trägt den `inference:`-Vermerk aus dem Datensatz, dass fünfzig ein
Vorschlag und keine gemessene Größe ist, und den Satz, dass eine Änderung daran
eine Konstante ändert und keine Regel. Sie steht deshalb als Konstante da und
nicht als Literal in `wiederfinden`.

## Kein zweiter Weg neben der Regel aus S8

`Markensprung::sprung` kommt aus `Zeilenindex::anfang_der_zeile` und aus keiner
eigenen Rechnung. Eine gemerkte Nummer über der Zeilenzahl landet damit am
Dateiende, genau wie der Zeilensprung aus C5, und die Regel dafür steht weiterhin
allein in `text::zeilen`. Die Probe
`eine_nummer_ueber_der_zeilenzahl_landet_am_dateiende_wie_der_zeilensprung`
prüft beide Wege gegeneinander:

```rust
assert_eq!(ergebnis.sprung, Zeilenindex::neu(&text).anfang_der_zeile(500));
```

`Fund` und `Zeilenlage` sind dabei zwei verschiedene Auskünfte und keine zwei
Zweige derselben: die erste sagt, ob der gemerkte Inhalt wiedergefunden wurde,
die zweite, ob die angesteuerte Nummer im Text überhaupt vorkommt. Eine Marke auf
Zeile 500 einer inzwischen auf 100 Zeilen gekürzten Datei trägt beide, und der
Aufrufer hat beides zu melden. Der Kommentar an `Markensprung` sagt es.

## Vier Abweichungen von der Schrittbeschreibung

**Die Proben stehen in `marke.rs` und nicht in `tests/text.rs`.** Die
Integrationsdatei war für den parallel laufenden S9 reserviert. Zehn Proben als
`#[cfg(test)] mod tests` im Modul, dieselbe Bauart, die `zeilen.rs`, `suche.rs`
und `lesezeichen.rs` schon tragen.

**`crates/krk-core/src/text/mod.rs` ist trotz Reservierung angefasst, um zwei
Zeilen.** Ohne `pub mod marke;` wird die neue Datei nicht mitübersetzt, keine
Probe läuft an, und das Abnahmekriterium des Schrittes wäre nicht zu erfüllen —
ein nicht eingebundener Rust-Quelltext ist für `cargo build`, `clippy` und
`fmt --check` gleichermaßen unsichtbar und damit ungeprüft. Die beiden Zeilen
sind rein additiv:

```rust
pub mod marke;                          // im Block der drei `pub mod`
pub use marke::{Fund, Markensprung};    // im Block der Wiederausfuhren
```

Der ASCII-Überblick und der Modulkopf jener Datei sind **nicht** nachgezogen; sie
nennen `marke` noch nicht. S9 schreibt denselben Kopf ohnehin um, weil `datei`
dort ebenfalls fehlt. Die Stelle ist damit offen und benannt.

**`Zeilenindex::inhalt_der_zeile` ist in `text/zeilen.rs` hinzugekommen.** Die
Marke vergleicht ganze Zeilen und braucht dafür den Inhalt einer Zeilennummer.
Ihn in `marke.rs` über `str::lines` zu bilden wäre die zweite Meinung darüber,
was eine Zeile beendet, und der Modulkopf von `zeilen.rs` begründet seit S8, warum
es davon nur eine gibt. `str::lines` trägt zudem eine andere Zählung: es kennt die
leere letzte Zeile nach einem abschließenden `\n` nicht, die derselbe Modulkopf
ausdrücklich führt. Die Methode geht über `str::get` statt über eine
Bereichsangabe und liefert `None` statt in Panik zu enden, falls ihr ein anderer
Text gereicht wird als der, über den der Index lief.

**Die Regel für gleich weit entfernte Treffer stand im Schritt nicht.** Bei
gleichem Abstand nach oben und nach unten gewinnt die kleinere Nummer, also der
Treffer, der in der Datei zuerst steht. Die Wahl ist willkürlich: bei gleichem
Abstand gibt es keine bessere Antwort, und das ist dieselbe Mehrdeutigkeit, die
der Datensatz als Grenze der Fähigkeit benennt. Festgelegt ist sie allein, damit
sie wiederholbar ist; die Probe `bei_gleichem_abstand_gewinnt_die_kleinere_nummer`
hält sie fest.

## Die Gültigkeitsprüfung liest keine Datei — jetzt gemessen, nicht nur zugesagt

Der tragende Grund der Antwort vom 260808-0017 ist nicht die Trefferquote,
sondern die Gültigkeitsprüfung der Leiste: **ungültig heißt allein, dass die
Datei fehlt**, weil die Leiste die Frage bei jedem Neuaufbau ihrer Liste für jede
Marke stellt. S11 hat die Zusage in den Modulkopf geschrieben und über den
Zeileninhalt geprüft; dass gar kein Lesevorgang stattfindet, war bis jetzt nicht
belegt.

Zwei Belege, beide in diesem Schritt entstanden:

Der erste ist am Quelltext. `crates/krk-core/src/ablage/lesezeichen.rs` nennt
genau zwei Berührungen des Dateisystems und keine einzige lesende Schnittstelle:

```
$ grep -nE "fs::|File|read|open|BufRead|is_file|is_dir" crates/krk-core/src/ablage/lesezeichen.rs
197:            Ziel::Ordner { ordner } => ordner.is_dir(),
198:            Ziel::Textstelle { datei, .. } => datei.is_file(),
```

Beide sind `stat` und kein `open`. Kein `fs::read`, kein `File`, kein Puffer.

Der zweite ist am Verhalten. Die Probe
`die_gueltigkeitspruefung_kommt_ohne_lesen_der_datei_aus` in
`crates/krk-core/tests/ablage.rs` entzieht der gemerkten Datei mit `chmod 000`
jedes Leserecht, prüft mit `fs::read`, dass sie damit tatsächlich unlesbar ist,
und stellt dann fest, dass die Marke gültig bleibt. Wer die Datei öffnen wollte,
bekäme `PermissionDenied`; die Prüfung bekommt ihre Antwort trotzdem. Das
Gegenstück steht in derselben Probe: verschwindet die Datei, wird die Marke
ungültig, und das ist der einzige Grund, aus dem sie es wird.

**Unter root belegt die Verhaltensprobe nichts**, weil Zugriffsrechte dann nicht
greifen. Sie bricht in diesem Fall erkennbar ab, statt still durchzugehen und
eine Zusage vorzutäuschen; dieselbe Einschränkung steht seit der Runde 1 bei
`eine_nicht_lesbare_datei_fuehrt_ebenso_zum_auslieferungszustand`.

## Abnahme

Die vier Abnahmekommandos, alle grün:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | übersetzt |
| `cargo test --workspace` | 15 Testprogramme, alle `ok`, 0 Fehlschläge |
| `cargo clippy --workspace --all-targets` | 0 Warnungen, 0 Fehler |
| `cargo fmt --all --check` | sauber |

`cargo fmt --all` hat dabei ausschließlich `marke.rs` umgeschrieben; die im
Arbeitsbaum liegenden Dateien der parallel laufenden Schritte waren bereits
formgerecht und sind unberührt geblieben.

Die fünf geforderten Fälle, jeder an seiner Probe:

| Zusage | Probe |
|---|---|
| Unveränderte Datei trifft sofort | `eine_unveraenderte_datei_trifft_sofort` |
| Um zehn Zeilen verschobene Stelle wird gefunden | `eine_um_zehn_zeilen_verschobene_stelle_wird_gefunden` |
| Um sechzig Zeilen verschoben: nicht gefunden, gemerkte Nummer mit Kennzeichen | `eine_um_sechzig_zeilen_verschobene_stelle_wird_nicht_gefunden` |
| Zweimal im Fenster: der nächstliegende gewinnt | `kommt_der_inhalt_im_fenster_zweimal_vor_gewinnt_der_naechstliegende` |
| Nummer über der Zeilenzahl: Dateiende über dieselbe Funktion wie C5 | `eine_nummer_ueber_der_zeilenzahl_landet_am_dateiende_wie_der_zeilensprung` |

Fünf Proben darüber hinaus: der Rand des Fensters an beiden Enden
(`das_fenster_reicht_genau_fuenfzig_zeilen_weit`), der Vorrang des Treffers auf
der gemerkten Nummer (`ein_treffer_auf_der_gemerkten_nummer_schlaegt_jeden_nachbarn`),
die gleich weit entfernten Treffer, eine gemerkte Nummer 0 aus einer von Hand
geänderten `bookmarks.toml`
(`eine_gemerkte_nummer_null_fuehrt_an_den_textanfang_und_sucht_trotzdem`) und die
Zeichengrenze bei Umlauten und Emojis
(`der_versatz_liegt_auch_bei_mehrbytezeichen_auf_einer_zeichengrenze`). Dazu zwei
Proben zu `inhalt_der_zeile` in `zeilen.rs`, von denen eine den Unterschied zu
`str::lines` festhält.

## Der Datensatz bleibt auf `_a_`

`decisions/260807-2147_a_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`
ist mit diesem Schritt **nicht** vollständig in Code umgesetzt und behält seinen
Marker. Die Antwort hat zwei Hälften: die Suche in der Nähe steht jetzt, die
Meldung in der Statuszeile bei `Fund::NichtGefunden` kommt in Phase G. Der
Marker gehört auf `_i_`, wenn diese zweite Hälfte gelandet ist.

## Geänderte Dateien

- `crates/krk-core/src/text/marke.rs` — neu: `NAHFENSTER`, `Fund`,
  `Markensprung`, `wiederfinden`, zehn Proben
- `crates/krk-core/src/text/zeilen.rs` — `Zeilenindex::inhalt_der_zeile`, zwei
  Proben
- `crates/krk-core/src/text/mod.rs` — zwei additive Zeilen, entgegen der
  Reservierung für S9, Begründung oben
- `crates/krk-core/tests/ablage.rs` — Probe
  `die_gueltigkeitspruefung_kommt_ohne_lesen_der_datei_aus`
- Dieser Plan — Schritt 12 auf `[DONE]`, Umsetzungsvermerk mit den vier
  Abweichungen
