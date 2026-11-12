# Nachträgliche Durchsicht: die Zeilenende-Regel im Kern und die Fragenzahl der Probe zu `abwarten`

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `8ec00352..eb715110`
**Not-opened:** none
**Review domain:** code
**Urteil:** accept

## Summary

Beide Behebungen halten ihre Zusage. `12a48952` ändert kein Verhalten: die neue Funktion ist im Rumpf zeichengleich mit den zwei gefallenen, und `zeilenende_des_textes` in `handlung.rs` ist im Code unberührt. Die drei dort genannten Abweichungen stimmen und sind vollständig. Die Schranke `1..=6` in `e405d96d` folgt aus dem Betriebscode von `abwarten`. Die Durchsicht ist nachträglich; der Stand ist als `v4.1.1` ausgeliefert.

## Totals

Critical 0, High 0, Medium 0, Low 0. Kein Defektdatensatz. Eine Anmerkung zu einem Werkbankdatensatz, außerhalb des Auftrags.

## Was geöffnet und geprüft wurde

- `crates/krk-core/src/text/zeilenende.rs` ganz, `crates/krk-core/src/text/mod.rs` (Diff).
- `crates/krk-core/src/ablage/einstellungen.rs`: `angehaengt`, die gefallene `zeilenende` aus dem Diff.
- `crates/krk-ui/src/appkit/editor.rs`: `protokoll_umgeformt`, die gefallene `zeilenende_des_ausschnitts` aus dem Diff.
- `crates/krk-core/src/trenntext/handlung.rs`: `zeilenende_des_textes`, das neue Prüfmodul, alle Rufer.
- `crates/krk-core/src/trenntext/index.rs`: Modulkopf, `satz_abtasten_bis`, `indexieren`, `Satzindex::vorspann`, `Satzindex::zeilenende`.
- `crates/krk-ui/src/cloudladen.rs`: Modulkopf, `abwarten`, das Prüfmodul ab `ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf`.
- `Cargo.toml`, `Cargo.lock`: allein die Versionszahl der vier Mitglieder.
- `fusion-workbench/orchestrator-events.jsonl` und die zehn Werkbankdateien des Bereichs, im Diff gelesen. Die Zeilen des Protokolls sind beim Lesen auf 600 Zeichen gekürzt; hinzugekommen sind 17 Zeilen der Arten `commit`, `record_change`, `task_start` und `task_done`.
- Ausgeführt am Stand `eb715110`: `cargo test -p krk-core --lib -- zeilenende trenntext::handlung` (neun grün), `cargo test -p krk-core --test baum` (21 grün), `cargo test -p krk-ui --bin krk -- cloudladen die_umformung_am_protokoll` (sechs grün), `cargo fmt --all --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` und `cargo clippy --workspace --all-targets`, alle ohne Befund.
- Zusätzlich: `abwarten_bricht_ab_oder_wartet_die_dauer` 600-mal, je 32 Läufe zugleich, 600 grün.

## Findings by theme

### `12a48952`: kein Verhalten geändert

**Die drei Rümpfe sind zeichengleich.** `des_textes` (`crates/krk-core/src/text/zeilenende.rs:22`) trägt denselben `match` wie die gefallene `zeilenende` (`git show 8ec00352:crates/krk-core/src/ablage/einstellungen.rs`, Zeile 585) und die gefallene `zeilenende_des_ausschnitts` (`git show 8ec00352:crates/krk-ui/src/appkit/editor.rs`, Zeile 2415): `text.find('\n')`, dann `text[..stelle].ends_with('\r')`. Die Randfälle fallen damit von selbst gleich aus, und die neuen Proben halten sie: kein `\n` ergibt `\n`, `\n` an Position 0 ergibt `\n` (der leere Vorsatz endet nicht auf `\r`), einzelne `\r` ohne `\n` ergeben `\n`, `\r\n` an Position 0 ergibt `\r\n`.

**Die zwei Rufer sind unverändert bis auf den Namen.** `angehaengt` (`einstellungen.rs:569`) und `protokoll_umgeformt` (`editor.rs:2367`) rufen an derselben Stelle mit demselben Argument; der `match` auf `"\n"` in `editor.rs` vergleicht weiter den Wert und nicht die Herkunft.

**`handlung.rs` ist im Code unberührt.** Der Diff trägt dort allein den Doc-Kommentar und das Prüfmodul.

**Keine vierte Fassung im Baum.** `grep -rn "ends_with(.\\r.)" crates` trifft allein `zeilenende.rs:24`; der alte Name `zeilenende_des_ausschnitts` steht in keiner Quelldatei, in `CLAUDE.md` und in `.claude/rules/` nicht mehr.

### Die drei Abweichungen in `handlung.rs` stimmen und sind vollständig

Gelesen gegen `satz_abtasten_bis` und `Satzindex::zeilenende` (`crates/krk-core/src/trenntext/index.rs:85`, `:353`):

1. Ein `\n` in einer angeführten Zelle läuft in der inneren Schleife als Feldzeichen durch. Bei `"a\nb";c\r\n` ist das Satzende `\r\n`, das erste `\n` des Textes steht hinter `a`. Stimmt.
2. `indexieren` setzt `stelle` hinter die `sep=`-Zeile, bevor der erste Satz beginnt. Bei `sep=;\na;b\r\n` ist das Satzende `\r\n`, das erste `\n` gehört dem Vorspann. Ohne Satz liest der Zweig `index.zeilenzahl() == 0` das Ende des Vorspanns. Stimmt.
3. Der letzte Satz ohne Zeilenende hat den leeren Bereich `ende..laenge`, und der ist nicht `\r\n`. Bei `"a\r\nb";c` gilt hier `\n`, dort `\r\n`. Stimmt.

**Vollständig (hergeleitet, nicht aufgezählt geprüft):** das erste `\n` des Textes steht im Vorspann (Fall 2), im Inhalt des ersten Satzes und dann in einer angeführten Zelle (Fall 1, oder Fall 3, wenn der Satz kein Ende hat), oder es ist das Ende des ersten Satzes. Im letzten Fall antworten beide gleich, weil der Leser `\r` vor `\n` außerhalb von Anführungen immer als `\r\n` nimmt. Ohne jedes `\n` sagen beide `\n`. Ein nicht schließendes Anführungszeichen wäre ein vierter Text, fällt aber unter Fall 3, und nach dem Modulkopf von `index.rs` rechnet keine Handlung auf einem solchen Index.

Die Probe `der_satzindex_antwortet_anders_als_das_erste_lf_des_textes` hält je Fall einen Text, die Gegenprobe drei Texte ohne Abweichung.

### `e405d96d`: die Schranke `1..=6`

**Obere Grenze sechs, am Betriebscode hergeleitet** (`crates/krk-ui/src/cloudladen.rs:60`):

- Die Schleife fragt, liest `verstrichen`, kehrt bei `verstrichen >= dauer` zurück und schläft sonst `SCHRITT.min(dauer - verstrichen)`.
- Ein Schlaf über den Rest beginnt nach dem Lesen von `verstrichen` und dauert mindestens `dauer - verstrichen`. Danach ist die Dauer um, es folgt genau eine Frage und die Rückkehr.
- Vor dem Schlaf Nummer k+1 sind k volle Schritte vergangen, also mindestens k mal 10 ms, und zugleich weniger als 50 ms. Daraus folgt k höchstens vier, also höchstens fünf Schlafphasen und sechs Fragen.

Die Herleitung des Bearbeiters stimmt. **Sie ruht auf einer Voraussetzung außerhalb des Baums:** `thread::sleep` kehrt nie vor der verlangten Dauer zurück, gemessen an derselben Uhr wie `Instant`. Die Standardbibliothek sagt das zu; gemessen ist es hier allein durch die 600 Läufe unter Last, die keinen Gegenfall zeigen. Das ist ein Indiz und kein Beweis.

**Untere Grenze eins.** Die erste Frage steht vor jeder Rückkehr. Ein Faden, der zwischen `Instant::now()` und `beginn.elapsed()` 50 ms nicht zugeteilt wird, kehrt nach einer Frage richtig zurück. Die alte Zwei konnte an einem richtigen `abwarten` fallen.

**Was die Probe noch fängt, und was ihr Kommentar sagt, decken sich.**

- Kreisen ohne Schlaf: mehr als sechs Fragen, rot.
- Ja ohne Frage: null Fragen, rot.
- Rückkehr vor der Dauer: `beginn.elapsed() >= dauer`, rot.
- Verschlafen in einem Stück: hält diese Probe nicht mehr, wie ihr Kommentar sagt. `ein_nein_mitten_in_der_wartezeit_beendet_sie` verlangt drei Fragen innerhalb von `HAENGT` bei einer Dauer von zehnmal `HAENGT` und wird an einem solchen `abwarten` rot. Der Verweis im Kommentar trägt.

Nichts, was die alte Schranke verlässlich hielt, ist ohne Halter geblieben.

### `eb715110`

Allein die Zahl `4.1.1` in `Cargo.toml` und an den vier Mitgliedern in `Cargo.lock`. Der Tag `v4.1.1` steht.

## Cross-cutting observations

**Anmerkung, kein Datensatz, außerhalb des Auftrags (Werkbank).** `261112-0851-darf-format-oder-singlestrip-in-einer-crlf-csv-einen-zellumbruch-im-ausschnitt-zu-crlf-machen.md` nennt in seiner Zeile `Implemented:` die Funktion `zeilenende_des_ausschnitts`. Die Zeile kam mit `b41e3a11`, nach `12a48952`, das die Funktion entfernt hat. Sie beschreibt mit ihrem Zusatz „der Stand ist der vor der Antwort gebaute“ den Commit `f0a60eea` und ist als Aufzeichnung dieses Standes lesbar; wer den Namen heute im Baum sucht, findet `krk_core::text::zeilenende::des_textes`.

## Recommended sequencing

Nichts zu tun. Kein Befund hält eine Auslieferung auf oder verlangt eine Nacharbeit.
