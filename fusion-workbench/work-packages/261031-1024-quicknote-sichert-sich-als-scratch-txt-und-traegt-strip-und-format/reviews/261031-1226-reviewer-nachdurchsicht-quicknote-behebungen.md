# Nachdurchsicht: Behebung der drei Defekte aus der Abschlussdurchsicht des Quicknote-Pakets

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `129d8d71..c1c2b7f8`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Domain:** code
**Verdict:** revise
**Cross-references:** `261031-1156-reviewer-abschlussdurchsicht-quicknote-scratch-strip-format.md`, `261031-1156-die-laengste-zeile-gilt-immer-als-umgebrochen-und-strip-verschluckt-befehle-format-tabellen.md`, `261031-1156-nach-fehlendem-ordner-oder-gescheitertem-schreiben-sichern-verlassen-und-beenden-den-letzten-stand-nicht.md`, `261031-1156-strip-kuerzt-nachlaufenden-leerraum-im-heredoc-rumpf-und-liest-arithmetisches-als-heredoc.md`

Außerhalb des Bereichs geöffnet: `spikes/quicknote-textdidchange/mit_fenster.swift`, `spikes/quicknote-textdidchange/ohne_schluesselfenster.swift`, `spikes/quicknote-textdidchange/starten.sh` (in der ersten Durchsicht nicht geöffnet).

## Zusammenfassung

Alle drei Defekte sind in der Form behoben, die ihre Issues beschreiben. Am Stand `c1c2b7f8` nachgemessen, mit `glaetten.rs` unverändert per `#[path]` in einem Wegwerfprogramm unter dem Scratchpad: Die Fälle aus A1, A2 und A3 verhalten sich jetzt richtig. Das Beispiel des Nutzers bleibt Zeichen für Zeichen grün. Die neue `Vormerkung` ist richtig. Die Breitenregel schließt aber die Fehlerklasse aus A1 nicht, sie engt sie ein: Strip verbindet jetzt getrennte Befehle ähnlicher Länge, auch zerstörend. `HowTo.md` sagt das Gegenteil. Den hingenommenen Preis eines einmal umbrochenen Textes beschreibt `HowTo.md` milder, als er ist.

## Zählung

| Schwere | Befunde |
|---|---|
| Critical | keiner |
| High | keiner |
| Medium | D1, D2 |
| Low | D3, D4 |

## Die drei Issues der ersten Durchsicht

**A1 (Breite): behoben, wie verlangt.** `belegte_breite` (`glaetten.rs:571`) gibt die längste Zeile erst als Breite zurück, wenn `MINDESTZAHL_VOLLER_ZEILEN` (2) Zeilen sie bis auf ein Zehntel erreichen. Am Stand `c1c2b7f8` gemessen:
- `# Erst die Abhaengigkeiten … bauen` + `cargo build` + `cargo test`: drei Zeilen, kein Befehl im Kommentar. Zusätzlich gilt `Zeilenende::Kommentar` (`glaetten.rs:311`): auch bei belegter Breite verbindet Strip nicht über ein Kommentarende.
- Beide Zwei- und Dreibefehlsfälle des Issues: bleiben untereinander.
- Kastentabelle und Baumskizze: Zeile für Zeile erhalten, die Ausrichtung bleibt (`ist_geruest`, `glaetten.rs:599`).

Alle fünf Abnahmezeilen des Issues sind erfüllt. Die Probe des Nutzerbeispiels ist im Diff unverändert. `cargo test -p krk-core --test glaetten`: 23 bestanden, 0 fehlgeschlagen.

**B1 (Vormerkung): behoben.** Zu den Fragen des Auftrags:
- **Kein Verlust beim Beenden:** `Vormerkung::sofort` (`quicknotesicherung.rs`) gibt einen Auftrag, solange `ungesichert` steht. `ungesichert` fällt allein in `nach_dem_einziehen`, wenn `gesichert_bis >= offen` gilt. `gesichert_bis` wird nur bei `Gesichert` nachgezogen. Der Faden meldet `Gesichert` bei jedem gelungenen Schreiben und nicht nur je Übergang (`schreiben`, Zweig `Ausgang::Gesichert`). Die Zählung hält auch bei zusammengefassten Aufträgen: `erledigt` geht in `beantwortet` ein. Eine Änderung nach der Abgabe setzt `offen = None`, also deckt ein späteres `Gesichert` für einen älteren Auftrag sie nicht. `quicknote_abschliessen` ruft `quicknote_sichern` vor `abschliessen` (`editor.rs:3694`), und `wird_beendet` ruft `quicknote_abschliessen` (`anwendung.rs:1587`). Den Ablauf aus dem Issue, also `OrdnerFehlt`, Ordner anlegen und Beenden ohne Änderung, fährt `nach_fehlendem_ordner_sichert_der_abschluss_ohne_weitere_aenderung` an einer echten Datei. Für `Gescheitert` tut das `nach_gescheitertem_schreiben_…`.
- **Kein Schreiben vor dem Lesen:** `geaendert` merkt nichts vor, solange `nimmt_auftraege` nein sagt. `Sicherung::schreiben` weist unverändert ab, und `ungesichert` ist am Anfang `false`. Die Probe dazu ist `eine_gesperrte_sicherung_merkt_nichts_vor`.
- **Keine Plattenarbeit auf dem Hauptfaden:** `Vormerkung` hat keinen Systemaufruf, die Zeit kommt herein. Die neuen Rümpfe in `editor.rs` beschicken allein Zellen und Kanäle. `die_quicknote_stellt_auf_dem_hauptfaden_keinen_systemaufruf` ist grün.

`cargo test -p krk-ui -- quicknotesicherung quicknote_stellt vor_dem_einlesen vor_der_antwort`: 21 bestanden, 0 fehlgeschlagen. Ob `editor.rs` die Vormerkung an den richtigen Stellen beschickt, sieht keine Probe. Die Regeldatei sagt das selbst (`.claude/rules/coding-was-man-nicht-sieht.md`, Absatz zur Quicknote). Das bleibt wie C1 der ersten Durchsicht bis zur Abnahme am Bündel offen.

**A2/A3 (Heredoc-Leerraum, Arithmetik): behoben.** Der harte Umbruch `zeile mit hartem Umbruch  ` bleibt im Rumpf erhalten. Die Endzeile verliert ihren Leerraum (`glaetten.rs:295-297`), und das ist nötig, damit sie den Rumpf beendet. `$((1 << 4))`, `(( … << … ))` und eine verschachtelte Klammer öffnen keinen Heredoc. Hinter `$(( 1<<3 ))` wird ein Umbruch wieder verbunden (gemessen). Den Wortlaut des Issues (`… awk '{print $2}'` + `-n`) verbindet Strip trotzdem nicht. Der Grund ist jetzt die Breitenregel, denn es gibt nur eine volle Zeile. Die Abnahme des Issues hält das mit einer zweiten vollen Zeile fest, und das ist richtig.

`cargo fmt --all --check` und `cargo clippy --workspace --all-targets` laufen ohne Ausgabe.

## Neue Befunde

### D1 (Medium): Strip verbindet getrennte Befehle ähnlicher Länge, und `HowTo.md:1223` verspricht, dass Befehle untereinander bleiben

Zwei Zeilen ab 40 Zeichen, deren Längen um höchstens ein Zehntel auseinanderliegen, belegen die Breite selbst. Gemessen:

```
rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b
  ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978
-> rm -rf target/debug/incremental/krk_core-1a2b3c4d5e6f7a8b ls -la target/debug/incremental/krk_ui-0f1e2d3c4b5a6978
```

Dasselbe gilt für zwei `cargo test`-Zeilen und für drei `git`-Zeilen ähnlicher Länge. Das ist die Klasse aus A1, nur mit einem engeren Auslöser. Die Liste in `HowTo.md` nennt den Fall. Der Satz davor, „Befehle untereinander bleiben untereinander“, hieß vor `b21ca032` „kurze Befehle …“ und ist jetzt falsch. Der Modulkopf (`glaetten.rs:48-50`) begründet die Schätzung damit, dass sie „in der Richtung [irrt], die nichts verbindet“, und auch das stimmt für diese Fälle nicht. Medium und nicht High: Der Auslöser ist enger als bei A1, und `cmd+z` nimmt die Umformung zurück. Die Folge ist dieselbe Bedeutungsänderung. Issue: `261031-1226-strip-verbindet-getrennte-befehle-aehnlicher-laenge-und-howto-verspricht-das-gegenteil.md`. Wie Strip sich verhalten soll, ist eine Frage an den Nutzer und kein Defekt. Sie steht als Entscheidungsfrage da: `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`.

### D2 (Medium): Den Preis „genau einmal umbrochen“ beschreibt `HowTo.md` milder, als er ist

Zur Frage des Auftrags: **Nein, nicht korrekt.** `HowTo.md` sagt, ein einmal umbrochener Text „bleibt, wie er ist; ein so umgebrochener Befehl steht nach Strip weiter auf zwei Zeilen. Vor dem Einfügen ins Terminal von Hand verbinden.“ Gemessen:
- **Format** lässt den Text nicht stehen. Es nimmt der Fortsetzungszeile die Einrückung (`glaetten.rs:221`), und die Fortsetzung steht danach als eigener Absatz da, den auch ein zweites Format nicht mehr verbindet. Das trifft jede Auswahl eines einzelnen einmal umbrochenen Absatzes oder Punkts. Ein Beispiel ist der Punkt „Jede englische Textdatei …“ aus dem Beispiel des Nutzers, allein ausgewählt.
- **Strip** ändert die Einrückung, meldet also kein „nichts zu ändern“, und die Ausgabe wirkt bearbeitet. Eingefügt läuft die erste Zeile als vollständiger Befehl. Ein Beispiel: `… && git push --force-with-lease` + `origin feature/…` führt `git push --force-with-lease` ohne Ziel aus. Das sagt `HowTo.md` nicht.

Dass das alte Strip-Beispiel in `HowTo.md` genau ein solcher Befehl war und in `b21ca032` ersetzt wurde, zeigt, wie nah dieser Fall am Gebrauch liegt. inference: Befehle zwischen einer und zwei Terminalbreiten sind die häufigste umbrochene Klasse. Nicht gezählt. Issue: `261031-1226-ein-einmal-umbrochener-text-bleibt-nicht-wie-er-ist-format-loest-die-fortsetzung-ab.md`.

### D3 (Low, kein Issue): Zwei Fehllesungen auf der sicheren Seite

- `let x=1<<2` öffnet für Strip einen Heredoc mit der Endmarke `2`, denn die Arithmetik ohne `((` erkennt der Automat nicht. Danach wird nichts mehr verbunden (gemessen). Selten und auf der Seite, die nichts verbindet. Zur Kenntnis.
- Eine Prosazeile mit einem Zeichen aus `U+2500..U+259F` (etwa `─>` als Pfeil) ist für Format eine Gerüstzeile und wird nie verbunden (`ist_geruest`, `glaetten.rs:603`). Auf der sicheren Seite. Zur Kenntnis.

### D4 (Low, kein Issue): `starten.sh` legt seine Programme ungeschützt in den Baum

`spikes/quicknote-textdidchange/starten.sh` übersetzt mit `swiftc -o "$name"` in das Verzeichnis des Spikes. `.gitignore` deckt die zwei Programme nicht, und `git check-ignore` meldet für `mit_fenster` nichts. Wer den Spike erneut fährt, hat zwei unverfolgte Binärdateien im Arbeitsbaum. `cargo xtask release` schlägt an einem unsauberen Baum an (`stand_pruefen`). Sonst sind die zwei Swift-Programme, was `messungen/261031-1300-quicknote-textdidchange.txt` beschreibt. Der Ersetzungsweg in `mit_fenster.swift` folgt in derselben Reihenfolge `Quicknote::ersetzen` (`appkit/quicknote.rs:494-502`): `breakUndoCoalescing`, `shouldChangeText`, `replaceCharacters`, `didChangeText`. Sie schreiben nichts auf die Platte und tragen keine Sicherheitsfrage.

## Sicherheitszwänge

In `git diff 129d8d71..c1c2b7f8 -- crates` steht kein neues `unsafe`, kein `Command::new`, keine neue Kiste und keine Änderung an `Cargo.toml` oder `Cargo.lock` (Dateiliste des Diffs: `glaetten.rs`, `tests/glaetten.rs`, `editor.rs`, `quicknotesicherung.rs`). Rechte `0600`, exklusives Anlegen und der eine Schreiber von `scratch.txt` sind unberührt. Die Proben, die das festhalten, wurden in den Lauf oben mitgefahren.

## Querschnitt

- **Eine Schätzung, zwei Preise, und Strip zahlt beide.** Format verträgt einen Irrtum in jeder Richtung. Für Strip ändert jede Fehlentscheidung die Bedeutung: zu viel verbunden heißt ein Befehl mit fremden Argumenten, zu wenig verbunden heißt ein abgeschnittener Befehl. Der Modulkopf behandelt „verbindet nicht“ als die sichere Richtung. Für Strip gibt es keine sichere Richtung. Deshalb steht D1 als Entscheidungsfrage und nicht als Schwellenkorrektur da.
- **Nebenbefund zur Werkbank, außerhalb des Codebereichs:** Die drei Issues vom 261031-1156 sind in ihren Steuerdateien `closed`/`fixed`, uncommittet. Ihren Erzählungen fehlt die Zeile `Resolved:`, die `fusion-workbench-conventions.md` `### Issue files` vom Ausführenden verlangt.

## Reihenfolge

1. Vor einer Auslieferung: die Entscheidungsfrage zu Strip dem Nutzer vorlegen. Danach D1 und D2 umsetzen. Die Dokumentation in beiden ist unabhängig von der Antwort zu berichtigen.
2. Die Abnahme am Bündel, die der Plan vor einer Auslieferung verlangt, und mit ihr die Beschickung der `Vormerkung` durch `editor.rs`.
3. Aufräumen: D4 (`.gitignore` oder Ausgabe nach `target/`).
