# Durchsicht: Singlestrip-Regel, Format-Korrektur für D2 und Umbenennung Strip zu Singlestrip

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `3a0a9a07..780d6870`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Domain:** code
**Verdict:** revise
**Cross-references:** `261031-1226-reviewer-nachdurchsicht-quicknote-behebungen.md`, `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`, `261031-1226-ein-einmal-umbrochener-text-bleibt-nicht-wie-er-ist-format-loest-die-fortsetzung-ab.md`, `261031-1226-strip-verbindet-getrennte-befehle-aehnlicher-laenge-und-howto-verspricht-das-gegenteil.md`, `261031-1344-singlestrip-verlaesst-sich-auf-kommentare-die-zsh-ab-werk-nicht-kennt.md`, `261031-1344-singlestrip-klebt-nach-backslash-und-leerzeile-zwei-woerter-zusammen.md`

## Zusammenfassung

Singlestrip setzt die Antwort des Nutzers um. Inhalt in Anführungen und Heredoc-Rümpfen bleibt, bis auf die Kopiereinrückung, Zeichen für Zeichen stehen, `\`-Fortsetzungen lösen sich wie in der Shell. Die Format-Korrektur für D2 ist richtig und idempotent, das Beispiel des Nutzers bleibt grün, und die Umbenennung ist vollständig. `HowTo.md` sagt unmissverständlich, dass Singlestrip für mehrere Befehle nicht gedacht ist. Die neue Kommentarregel ist dokumentiert, aber ihre Sicherheitszusage gilt nur in einer Shell, die Kommentare in der interaktiven Eingabe kennt. Die zsh von macOS kennt sie ab Werk nicht (E1, Medium). Daneben steht ein kleiner Verbindungsfehler nach `\` und Leerzeile (E2, Low).

## Zählung

| Schwere | Befunde |
|---|---|
| Critical | keiner |
| High | keiner |
| Medium | E1 |
| Low | E2, E3, E4, E5 |

## Messung

Am Stand `780d6870` (Baum `933b4b05`, `crates/` ohne Abweichung im Arbeitsbaum) mit `glaetten.rs` unverändert per `#[path]` in einem Wegwerfprogramm unter dem Scratchpad, übersetzt mit `rustc` 1.97.1, Edition 2024. Jeder Fall lief zweimal, damit die Idempotenz mitgemessen wird. 38 Fälle, alle idempotent. Die Fälle, auf die sich die Aussagen unten stützen:

| Fall | Eingabe (gekürzt) | Ausgabe |
|---|---|---|
| S1 | `git commit -m "Titel␣␣` / `␣␣` / `␣␣Zweite Zeile␣␣` / `␣␣␣␣␣␣tiefer` / `␣␣Ende" && git push` / `␣␣origin main` | `git commit -m "Titel␣␣\n\nZweite Zeile␣␣\n␣␣␣␣tiefer\nEnde" && git push origin main` |
| S2 | `echo 'a` / `␣␣␣␣b␣␣␣` / `␣␣c'` / `␣␣\| wc -l` | `echo 'a\n␣␣b␣␣␣\nc' \| wc -l` |
| S3 | `printf $'x\n` / `␣␣y'` / `␣␣> out` | `printf $'x\n\ny' > out` |
| S5 | `cat <<'EOF' > x.txt` / Rumpf mit Leerraum, Leerzeile / `␣␣EOF` / `␣␣echo fertig` | Rumpf samt Leerraum und Leerzeile erhalten, `EOF\necho fertig` |
| S6–S8 | `<<-` mit Tabulator, zwei Heredocs auf einer Zeile, Heredoc in `$(` | Rümpfe und Endzeilen erhalten |
| S9 | `cargo test \` / `␣␣␣␣--workspace \` / `␣␣␣␣-- --skip appkit::` | `cargo test --workspace -- --skip appkit::` |
| S10 | `foo\` / `␣␣bar` | `foobar` (wie die Shell) |
| S11 | `foo\` / Leerzeile / `␣␣bar` | `foobar`, siehe E2 |
| S13 | `echo a\\` / `␣␣b` | `echo a\\ b` (maskiertes `\`, keine Fortsetzung) |
| S16 | `ls -la # zeigt alle` / `␣␣dateien an` | `ls -la # zeigt alle dateien an` |
| S17–S20 | `echo a;# don't`, `${#x}`, `$#`, `https://x/#anker` | Kommentar nur am Wortanfang erkannt |
| S21 | `# kommentar` / `␣␣cat <<EOF` / … | alles in einer Kommentarzeile |
| S25 | `rm -rf build/alt` / `␣␣ls -la build/neu` | `rm -rf build/alt ls -la build/neu` |
| S27 | einmal umbrochener `git push --force-with-lease` / `␣␣origin feature/x` | eine Zeile, Ziel dabei |
| F1 | einmal umbrochener Punkt allein | ` - Jede …` / `␣␣␣␣Zusatz …`, Fortsetzung eingerückt |
| F6 | F1 neben einem zweiten umbrochenen Punkt | beide verbunden |
| F3 | `# Erst die Abhaengigkeiten …` (längste) / `␣␣cargo build` / `␣␣cargo test` | `# Erst …\n␣␣cargo build\ncargo test`, siehe E3 |

Zusätzlich auf diesem Gerät: Die `HowTo.md`-Aussage, dass `rm -rf build/alt ls -la build/neu` auch `build/neu` löscht, stimmt unter `/bin/rm` von macOS (in einem Wegwerfordner nachgefahren: `build/` danach leer). Die zsh-Messungen stehen unter E1.

Proben am Baum: `cargo test -p krk-core --test glaetten --test belegung` (88 + 25 bestanden), `cargo test -p krk-core --lib sprache` (5), `cargo test -p krk-core --test baum` (20), alle ohne Fehlschlag. Die Läufe für `krk-ui`, `cargo fmt --all --check` und `cargo clippy --workspace --all-targets` stehen unter „Prüfläufe“ weiter unten.

## Zu den Fragen des Auftrags

**Bleibt Inhalt in Anführungen und Heredoc-Rümpfen unverändert?** Ja, bis auf die Kopiereinrückung, die nach der Regel des Modulkopfs auch dort fällt (S1, S2, S3, S5–S8). Nachlaufender Leerraum bleibt im Rumpf und am Zeilenende in offener Anführung (`leerraum_bleibt`, `befehl` in `glaetten.rs`). Eine Zeile, die in einer Anführung beginnt, verliert nur die Kopiereinrückung und nicht ihren ganzen Leerraum (`matches!(lage, Lage::Normal)` vor dem Kürzen). Unverändert von früher: Ist eine Zeile in Anführung die einzige ab der zweiten, gilt ihre ganze Einrückung als Kopiereinrückung (`echo "a` / `␣␣␣␣b"` ergibt `b` ohne Einrückung). Das ist nicht entscheidbar und war vor dem Bereich schon so. Kein Befund.

**Löst `\`-Fortsetzung korrekt auf?** Ja (S9, S10, S13, CRLF). Die Ausnahme ist E2: `\` ohne Leerraum, danach eine Leerzeile.

**Ist die neue Kommentarregel sicher und dokumentiert?** Dokumentiert ja: Modulkopf, `HowTo.md`, `.claude/rules/coding-was-man-nicht-sieht.md` und die Probe `singlestrip_haelt_einen_kommentar_bis_zum_ende_der_zeile`. Die Erkennung ist richtig (S17–S22). Ihre Zusage „läuft nicht“ hält aber nur in einer Shell mit Kommentaren in der interaktiven Eingabe. E1.

**Ist die Format-Korrektur für D2 richtig, und bleibt das Nutzerbeispiel grün?** Ja. Der neue Zweig in `fliesstext` greift nur bei `breite == 0`. Er misst mit `laengste` dieselbe Bedingung, die `umgebrochen` sonst mit der belegten Breite prüft. Die Fortsetzung behält ihre Einrückung, ein zweites Format ändert nichts, und ein größerer Text verbindet sie noch (F1, F6, Probe `format_laesst_einen_einmal_umbrochenen_punkt_und_absatz_als_fortsetzung_stehen`). `das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe` ist grün, und die Probe ist im Diff unverändert. Einen sichtbaren Nebeneffekt zeigt E3.

**Ist die Umbenennung vollständig?** Ja. `git grep -i -E 'quicknote_strip|QuicknoteStrip|\bStrip\b' 780d6870` außerhalb der Werkbank findet allein den historischen Hinweis in `crates/krk-core/src/tasten/belegung.rs` am Doc-Kommentar von `Kommando::QuicknoteSinglestrip`. Übrig sind nur Slugs von Datensätzen (`…-traegt-strip-und-format…`) und das Wort in `spikes/fn-tasten/README.md` („Control Strip“, ein anderes Ding). de, fr und en tragen `QuicknoteSinglestrip`, `QuicknoteSinglestripUnveraendert` und den Kommandonamen. Selektor `quicknoteSinglestrip:`, Methode `singlestrip_geklickt`, Knopfliste und Zweigprobe stimmen überein (`appkit/quicknote.rs`). Die alte Kennung `quicknote_strip` ist nie ausgeliefert worden: `git tag --contains 293dfb47` ist leer, jüngster Tag `v3.12.0`. Eine eigene `keymap.toml` mit der alten Kennung kann es außerhalb dieses Geräts also nicht geben. Die Knöpfe setzen ihre Breite über `sizeToFit`, also braucht der längere Name keinen Platz von Hand.

**Ist `HowTo.md` zu mehreren Befehlen unmissverständlich?** Ja. Die Schaltflächenliste sagt „Für mehrere Befehle untereinander ist es nicht gedacht“. Der Abschnitt `### Singlestrip und Format` sagt „**ein** Befehl“ und bringt das `rm`-Beispiel mit der Folge, und die Folge ist auf macOS nachgemessen. Der frühere Satz „Befehle untereinander bleiben untereinander“ ist weg. Damit ist die Abnahme von D1 erfüllt.

## Befunde

### E1 (Medium): Die Kommentarregel verspricht „läuft nicht“, und die zsh von macOS liest `#` ab Werk nicht als Kommentar

Seit `f39e14fb` hängt Singlestrip jede Folgezeile an eine Zeile, die in einem Kommentar endet. Der Modulkopf (`glaetten.rs`, Absatz „Ein Kommentar haelt die Zeile bis zu ihrem Ende“) sagt: „Verbunden laeuft schlimmstenfalls nichts“. `HowTo.md` sagt: „Alles, was danach steht, steht im Kommentar und läuft nicht.“ Gemessen, eingegeben über die Standardeingabe einer interaktiven Shell:

```
zsh -f -i:      ls -la # zeigt alle dateien an
                -> ls: #: No such file or directory  (ebenso zeigt, dateien, an; `alle` wird gelistet)
zsh -f -i:      # Erst bauen cargo build   -> zsh: command not found: #
bash --norc -i: ls -la # zeigt alle dateien an   -> nur ls -la
```

`zsh -f -i -c '[[ -o interactivecomments ]]'` meldet `off`, und die Konfiguration des Nutzers schaltet die Option ein. KRK wird öffentlich ausgeliefert, und die Anleitung setzt keine Shell voraus. Singlestrip macht den Fall breiter, denn alle angehängten Wörter werden Argumente des Befehls vor dem `#`. Medium und nicht High: Mit der Konfiguration des Nutzers tritt der Fall nicht auf. Der häufigste Fall, die Kommentarzeile über dem Befehl, endet in `command not found: #`. Der Rohtext hätte in derselben Shell eingefügt dieselbe Klasse Fehler. Issue: `261031-1344-singlestrip-verlaesst-sich-auf-kommentare-die-zsh-ab-werk-nicht-kennt.md`. Es nennt zwei Richtungen: die Bedingung dokumentieren oder den Kommentar herausnehmen. Unter der Annahme „ein Befehl“ ist das Herausnehmen entscheidbar und hängt an keiner Shell.

### E2 (Low): `foo\` + Leerzeile + `bar` wird `foobar`

Der Zweig für eine Leerzeile außerhalb in `befehl` überspringt sie mit dem Kommentar „der Trenner davor steht schon“. Nach `Trenner::Fortsetzung` steht aber nur dann ein Leerzeichen, wenn Leerraum am `\` stand oder die Folgezeile damit beginnt. `nachher_leer` liest dabei die Leerzeile (S11). Ein seltener Fall. Das Ergebnis ist trotzdem ein Wort, das niemand geschrieben hat. Issue: `261031-1344-singlestrip-klebt-nach-backslash-und-leerzeile-zwei-woerter-zusammen.md`.

### E3 (Low, kein Issue): Format lässt nach der längsten Zeile eine Nicht-Fortsetzung eingerückt stehen

F3: Unter einer langen Kommentar- oder Überschriftzeile, die als einzige voll ist, behält die nächste eingerückte Zeile ihre Einrückung, die übernächste nicht. Das ist die Regel, die der Modulkopf ausschreibt („jede Zeile, die mit der laengsten Zeile als Breite verbunden wuerde …“). Ob die Zeile eine Fortsetzung ist, ist nicht entscheidbar, und der Preis ist Kosmetik. Die Ausgabe ist idempotent, und `cmd+z` nimmt sie zurück. Zur Kenntnis.

### E4 (Low, kein Issue): Die Probe der Anleitungsbeispiele führt das neue `rm`-Beispiel nicht

`die_beispiele_der_anleitung_stimmen` (`crates/krk-core/tests/glaetten.rs`) sagt „beide Beispiele woertlich“. Der Abschnitt in `HowTo.md` trägt seit `f39e14fb` ein drittes, `rm -rf build/alt` / `␣␣ls -la build/neu`. Dessen Verhalten hält `singlestrip_haengt_getrennte_befehle_aneinander_denn_die_eingabe_ist_einer` nur mit anderem Wortlaut fest. Eine spätere Änderung am Beispiel bliebe ungeprüft. Aufräumen.

### E5 (Low, kein Issue): Eine Zeile der Meldungstafel in `appkit/editor.rs` ist aus der Spalte gerutscht

Im Doc-Kommentar über `Editormeldung` steht `///  Singlestrip und Format              crate::appkit::editor::umgeformt …` seit `46abb49a` sechs Zeichen rechts der Spalte. Das Ersetzen hat den Füllraum nicht nachgezogen. Kosmetik.

## Sicherheitszwänge

In `git diff 3a0a9a07..780d6870 -- crates resources xtask Cargo.toml Cargo.lock` steht kein neues `unsafe` außer dem umbenannten `#[unsafe(method(quicknoteSinglestrip:))]`, das an derselben Stelle mit demselben `SAFETY`-Kommentar wie vorher steht (`appkit/quicknote.rs`, innerhalb der Ausnahme von `appkit/mod.rs`). Es gibt kein `Command::new`, keine neue Kiste und keine Änderung an `Cargo.toml` oder `Cargo.lock`. `glaetten.rs` bleibt reine Rechnung auf einer Zeichenkette. Der Weg zu `scratch.txt` (`quicknotesicherung.rs`, `0600`, ein Schreiber) ist im Bereich nicht berührt. Alle Texte gehen weiter über die Sprachtabelle, und `baum.rs` ist grün. `spikes/quicknote-textdidchange/starten.sh` legt seine Programme jetzt in `mktemp -d` unter `$TMPDIR` ab und räumt sie per `trap` weg. Damit ist D4 der vorigen Durchsicht behoben, und `"$ausgabe"` ist überall in Anführung.

## Prüfläufe

- `cargo test -p krk-ui -- --skip appkit::editor::tests::die_flaeche quicknote umform zweigproben menuemodell belegungsmodell zulaessigkeit`: 156 bestanden, 0 fehlgeschlagen. Darin stehen die Zweigprobe in `appkit/anwendung.rs`, die Knopfprobe und `singlestrip_und_format_sind_eine_zuruecknehmbare_handlung` in `appkit/quicknote.rs`, `eine_umformung_ohne_aenderung_legt_keinen_rueckgaengigeintrag_an`, die Menü- und Belegungsproben sowie die Zulässigkeitsproben.
- Nicht gelaufen: `appkit::editor::tests::die_flaeche_der_quicknote_ist_reiner_text_mit_eigenem_verwalter`. Im gefilterten Lauf blieb sie über 60 Sekunden stehen. Sie baut eine Ansicht unter der Hauptfadensperre (`an_einer_flaeche`), stammt aus `d3824d96` und ist im Bereich nicht geändert. Allein gefahren (`--test-threads=1`) hing sie ebenso und wurde nach 200 Sekunden abgebrochen (Signal `SIGALRM`, Exitcode 142). Ob sie vor dem Bereich lief, ist nicht geprüft. inference: Sie verlangt eine Lage, die die Quicknote-Proben schon als „unter `libtest` ohne Schlüsselfenster“ beschreiben. Kein Befund dieses Bereichs, aber vor der Abnahme am Bündel zu klären.
- `cargo fmt --all --check`: ohne Ausgabe. `cargo clippy --workspace --all-targets`: keine Warnung, kein Fehler.

## Querschnitt

- **Die Annahme „ein Befehl“ hat die Breitenschätzung aus Singlestrip entfernt und eine Frage nach der Shell hineingebracht.** Bis `3a0a9a07` hielt der Umbruch nach einem Kommentar den Rest in einer eigenen Zeile. Jetzt hängt alles am `#`, und welche Shell den Text liest, entscheidet die Bedeutung. Das ist der eine Rest, den die Annahme nicht wegnimmt. E1 lässt sich deshalb über den Mechanismus entscheidbar machen und nicht über eine weitere Schwelle.
- **Werkbank, außerhalb des Codebereichs:** Die zwei Issues vom 261031-1226 stehen in ihren Steuerdateien auf `closed`/`fixed`, uncommittet. Ihre Erzählungen tragen die Abschlusszeile als `**Resolved:**` am Ende und nicht in der Form `---` + `Resolved:`, die `fusion-workbench-conventions.md` `### Issue files` vorgibt. Zur Kenntnis.

## Reihenfolge

1. Vor einer Auslieferung: E1. Die Richtung wählt der Planer oder der Nutzer. Die Dokumentation muss in jeder Richtung berichtigt werden.
2. E2 zusammen mit E1, im selben Abschnitt von `befehl`.
3. Aufräumen: E4, E5.
4. Unverändert offen aus den vorigen Durchsichten: die Abnahme am Bündel vor einer Auslieferung.
