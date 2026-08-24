Vierzehn Prosastellen der Ablage sagen weiter „vier", und ein offener Datensatz schützt drei davon ausdrücklich

---

Schritt 2 der Runde 16 hat `Datei::ALLE` von sechs auf sieben und die Zahl der TOML-Dateien
von vier auf fünf gebracht. Nachgezogen sind allein `pfade.rs` und zwei Doc-Kommentare in
`tests/ablage.rs`. Vierzehn weitere Stellen in `crates/krk-core/src/ablage/mod.rs` und
`crates/krk-core/tests/ablage.rs` nennen weiter „vier", eine nennt „sechs", und der Plan
deckt davon genau eine ab. Der Übersetzer hält keine dieser Stellen.

**Der eigentliche Befund ist nicht die Zahl, sondern eine Anweisung im Bestand.** Der offene
Datensatz `shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`
schreibt aus: „**Drei benachbarte Stellen sind richtig und dürfen nicht mitgezogen werden:**
`mod.rs:59`, `:549` (`laden`) und `:645` (`sichern`) sprechen über die vier TOML-Dateien."
Zum Zeitpunkt seiner Erhebung stimmte das. Seit Schritt 2 stimmt es nicht mehr: es sind fünf.
Wer jenen Datensatz nach seinem Wortlaut abarbeitet, lässt genau die drei Stellen stehen, die
diese Runde falsch gemacht hat. Die am 260824-0940 angehängte `Also seen`-Zeile nennt diese
Umkehrung nicht; sie spricht nur von den fünf Stellen, die schon vorher falsch waren.

---

**Gemessen am Baumstand `b76800b`.** `cargo build`, `cargo test`, `cargo clippy` und
`cargo fmt --check` laufen grün; kein Verhalten ist betroffen.

## Was der Baum trägt

`Datei::ALLE` führt sieben Werte (`crates/krk-core/src/ablage/pfade.rs:177-185`), davon fünf
mit `Format::Toml` (`:212-218`): `keymap.toml`, `bookmarks.toml`, `session.toml`,
`settings.toml`, `readers.toml`. Zwei tragen Text. `Ablage::pfad`, `Zugang::pfad` und
`Ablageort::datei` nehmen ein beliebiges `Datei` entgegen, also alle sieben.

## Die Stellen, nach Aussage getrennt

**Zahl aller Ablagedateien — sagt sechs, sind sieben.** Vom Plan **nicht** abgedeckt.

| Stelle | Wortlaut | Wahr ist |
|---|---|---|
| `mod.rs:1` | „Die Ablage: sechs Dateien in zwei Formaten" | sieben |

**Zahl der TOML-Dateien — sagt vier, sind fünf.**

| Stelle | Wortlaut | Anmerkung |
|---|---|---|
| `mod.rs:4` | „Vier tragen TOML und gehen ueber `Zugang::laden`" | die **einzige** Stelle, die Schritt 8 des Plans nennt |
| `mod.rs:59` | „# Eine der vier Dateien entsteht einmal und wird nie wieder geschrieben" | es sind zwei von fünf; von `260821-1023` als „richtig" geschützt |
| `mod.rs:117-118` | „Alle vier TOML-Dateien gehen durch `Zugang::laden`, und die vier Regeln gelten dort fuer alle vier gleich" | dreimal „vier" in einem Satz |
| `mod.rs:143` | „drei der vier TOML-Dateien tragen es" | drei von fünf |
| `mod.rs:549` | „Liest eine der vier Dateien." (`Zugang::laden`) | von `260821-1023` als „richtig" geschützt |
| `mod.rs:645` | „Schreibt eine der vier Dateien" (`Zugang::sichern`) | von `260821-1023` als „richtig" geschützt |
| `mod.rs:769` | „Die vier TOML-Dateien reichen ihren gelesenen Text als `&mut text.as_bytes()` herein" | |
| `tests/ablage.rs:53` | „Laedt eine der vier Dateien so, wie der Betrieb es tut" | |
| `tests/ablage.rs:70` | „Schreibt eine der vier Dateien unter der Schreibsperre" | |
| `tests/ablage.rs:126` | „Damit die Zusage \"alle vier Dateien\" trotzdem an vier Dateien geprueft wird" | |
| `tests/ablage.rs:1049` | „Der Pfad, unter dem die Sicherung einer der vier Dateien zu erwarten ist" | |
| `tests/ablage.rs:1079` | „Alle vier Dateien werden gesichert" | |
| `tests/ablage.rs:1558` | „wie bei den vier TOML-Dateien" | |
| `tests/ablage.rs:1577` | „Dieselbe Zusage wie fuer die vier TOML-Dateien" | |

**Eine Stelle ist nicht nur eine Zahl, sondern eine Begründung, die nicht mehr trägt.**
`mod.rs:241` begründet `Grund::ZuGross` mit: „Nur eine Zetteldatei kann ihn tragen: die vier
TOML-Dateien schreibt KRK selbst, und ihr Leseweg kennt keine Grenze." `readers.toml` schreibt
KRK gerade **nicht** selbst — `pfade.rs:148-152` sagt es an derselben Aufzählung ausdrücklich:
„Eine der beiden, die KRK im Betrieb nicht schreibt; die andere ist `Datei::Leser`." Der Schluss
(nur eine Zetteldatei kann `ZuGross` tragen) bleibt richtig, weil ihn `Zugang::text_laden` trägt
und nicht die Herkunft der Datei. Die genannte Begründung ist es nicht mehr.

**Zwei Probennamen tragen die Zahl im Bezeichner**, und keiner ist mitgezogen worden:
`alle_vier_dateien_ueberstehen_schreiben_und_wiedereinlesen` prüft seit Schritt 2 fünf, und
`jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt` prüft vier von fünf (siehe
`issues/260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`).

## Was der Plan abdeckt und was nicht

Schritt 8 (`planning/260824-0640_o_plan-…`, Bündel C) nennt als einzige Prosaarbeit an dieser
Datei: „sein Modulkopf zieht von vier TOML-Dateien auf fünf". Das trifft `mod.rs:4` und keine
der übrigen vierzehn. Wer Schritt 8 nach seinem Wortlaut fährt, hinterlässt den Rest.

## Vorschlag

1. `mod.rs:1` auf sieben, `mod.rs:4` auf fünf.
2. Die zwölf verbleibenden „vier"-Stellen in `mod.rs` und `tests/ablage.rs` je einzeln lesen
   und auf fünf setzen; `mod.rs:241` neu begründen (der Träger der Aussage ist
   `Zugang::text_laden`, nicht die Herkunft der Datei).
3. In `shared/issues/260821-1023_o_…` die Schutzanweisung für `mod.rs:59`, `:549` und `:645`
   zurücknehmen: sie ist seit Schritt 2 der Runde 16 die falsche Anweisung.
4. Die zwei Probennamen mit umbenennen, sobald `readers.toml` ihren Ladeweg hat.

**Schwere:** mittel. Kein Fehlverhalten, aber dieses Projekt führt Prosa-gegen-Code-Abweichungen
als Defekte, und ein offener Datensatz weist heute ausdrücklich an, drei falsche Stellen stehen
zu lassen. Das ist teurer als eine falsche Zahl.

**Gefunden:** coderev, Durchsicht des Bereichs `278a008..b76800b` am 260824-1014.

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:1`, `:4`, `:59`, `:117`, `:143`, `:241`,
`:549`, `:645`, `:769`; `crates/krk-core/tests/ablage.rs:53`, `:70`, `:126`, `:1049`, `:1079`,
`:1558`, `:1577`; `shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`

**Verwandt:** `shared/issues/260814-0912_o_neun-stellen-sprechen-weiter-von-vier-ablagedateien-es-sind-sechs.md`,
`shared/issues/260816-2307_o_der-doc-kommentar-von-ablage-pfad-nennt-vier-dateien-die-aufzaehlung-fuehrt-sechs.md`

**Domain:** code

---
Resolved: Alle vier Punkte des Vorschlags, am 260824-1245.

**1. und 2.** `mod.rs:1` steht auf sieben, `mod.rs:4` auf fünf. Die zwölf verbleibenden
„vier"-Stellen sind je einzeln gelesen und gesetzt: die Überschrift bei `:59` nennt jetzt
**zwei** von fünf (`settings.toml` und `readers.toml` gehen beide nicht über `Ablage::sichern`),
`:117` sagt „Alle fünf TOML-Dateien … und die vier Regeln gelten dort für alle gleich" — die
zweite Vier bleibt, sie zählt die Regeln —, `:143` nennt drei von fünf und schreibt dazu, dass
`readers.toml` `deny_unknown_fields` an `leseprofil::datei::Profildatei` ebenfalls trägt, aber
noch nicht über diesen Ladeweg geht. In `tests/ablage.rs` sind die sieben Stellen gesetzt; zwei
davon (`:53`, `:70`) sprechen jetzt ausdrücklich von den fünf **TOML**-Dateien statt von „den vier
Dateien", und `:1049` (`beiseitepfad`) nennt gar keine Zahl mehr, weil die Funktion jedes `Datei`
annimmt.

**`mod.rs:241` ist neu begründet und nicht bloß umgezählt.** Der Träger der Aussage ist
`Zugang::text_laden` und nicht die Herkunft der Datei; der alte Wortlaut steht im Text als das,
was er war, damit der nächste Leser die Umkehrung sieht statt nur das Ergebnis.

**3.** Die Schutzanweisung in
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-…` ist zurückgenommen: dort steht ein
`## Nachtrag 260824-1245`, der sagt, warum die drei Stellen `mod.rs:59`, `:549`→`:558` und
`:645`→`:654` seit Schritt 2 der Runde 16 **nicht** mehr richtig sind, und der die fünf offenen
Stellen jenes Datensatzes mit ihren neuen Zeilennummern und der berichtigten Zahl (sieben, nicht
sechs) führt. Der Befund jenes Datensatzes selbst bleibt offen; er gehört ihm und nicht dieser
Räumung.

**4. Die zwei Probennamen tragen jetzt gar keine Zahl mehr**, statt auf den Ladeweg zu warten:
`alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` und
`jede_toml_datei_mit_ladeweg_wird_bei_beschaedigung_zur_seite_gelegt`. Eine Zahl im Bezeichner
geht mit jeder neuen Ablagedatei wieder schief, und beide Proben laufen ohnehin über
`toml_dateien()` und nicht über eine Zahl. Der zweite Name sagt zusätzlich, was die Probe
tatsächlich prüft; die Ausnahme dazu steht als `OHNE_LADEWEG` daneben (siehe
`260824-0940_c_readers-toml-faellt-beim-zip-…`).

Nicht angefasst: `mod.rs:25` („drei der vier Inhalte"), das über die Inhaltsmodule spricht und
nicht über die Dateien, und die fünf Stellen des gemeinsamen Datensatzes.
