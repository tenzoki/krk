# `CLAUDE.md` auf den Stand der Runde 17

**Status:** Complete
**Agent:** coder
**Datum:** 260825-1247
**Baumstand:** `7ba5a20`

## Auftrag

`CLAUDE.md` auf den Stand nachziehen, den die Runde 17 hergestellt hat. Ausdrücklich
allein diese Datei; eine zweite Sitzung liest den Baum gerade durch. Grundlage sind zwei
Defektdatensätze:
`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-1230_*_claude-md-fuehrt-die-runden-nur-bis-15-…`
und `shared/issues/260825-0727_*_claude-md-nennt-zwei-aufrufer-von-ohne-warten-oeffnen-…`.

## Geändert

Eine Datei: `CLAUDE.md`. Keine Codedatei, kein `Cargo.toml`, keine Belegung.

1. **Verweisregister** — die Runden 16 und 17 nachgetragen. Der Satz darüber nennt jetzt
   auch den Marker `_t_`, den er nicht kannte, solange keine Runde lief.
2. **Belegungsabsatz** — statt „zwei Verhalten stehen daneben" (mit den drei
   Kontextbefehlen wären es fünf) die zwei Klassen: was an einer Taste hängt, sagt die
   Belegung; was an einer Mausgeste oder an der Bauzeit hängt, steht nicht darin. Fundort
   der Gesten ist der Modulkopf von `crates/krk-ui/src/kommandos/mod.rs`, der ihre Zahl
   aus demselben Grund nicht nennt.
3. **Die gewachsenen Aufzählungen** — „Vier" gefallen, `Art`
   (`krk-core/src/operation/auftrag.rs`) mit seinem Zählkommando dazu. Die Rückverweisung
   im Abschnitt „Was man nicht sieht" nachgezogen.
4. **Kisten ohne Vorgabemerkmale** — `zip` als dritte neben `syntect` und `two-face`,
   mit `deflate-flate2` und dem Nebeneintrag `flate2`.
5. **`ohne_warten_oeffnen`** — Überschrift von „Textdatei" gelöst, die Aufzählung der
   Aufrufer durch das breite Zählkommando ersetzt, die Ortsangabe nach Klassen statt nach
   einer Datei, und die Begründung dazu, warum die Typfrage beim Aufrufer bleibt.
6. **Neuer Absatz in „Was man nicht sieht"** — der zweite Weg in die Anwendung, der keine
   Taste hat: die drei Kontextbefehle stehen weder in der Belegung noch im Hauptmenü, und
   `Kontextbefehl` mit seinem Ausführungszweig ohne Auffangzweig ist die Sperre gegen den
   wirkungslosen Menüeintrag.

## Nachgezählt statt übernommen

| Aussage | Kommando | Ergebnis |
|---|---|---|
| eine Hülle um `NSPasteboard` | `grep -rln 'NSPasteboard' crates/` | hält; die weiteren Treffer sind `NSPasteboardType` im Abwurf und der hereingereichte Parameter der Vorschau, beide aus früheren Runden |
| `Wirkungsbereich` sieben | `awk '/^pub enum Wirkungsbereich/,/^}/' …` | 7 |
| `Bereich` fünf | `awk '/^pub enum Bereich/,/^}/' …` | 5 |
| `Fokus` fünf | `awk '/^pub enum Fokus/,/^}/' …` | 5 |
| kein neues `Kommando` | `git diff 428fbc4..HEAD -- …/belegung.rs resources/default-keymap.toml` | leer |
| kein `cc`, kein `-sys` außer `windows-sys` | `grep '^name = ' Cargo.lock` | hält |
| keine Git-Variante in `Kommando` | `awk '/^pub enum Kommando/,/^}/' … \| grep -i git` | leer |
| Untergrenzen-Abschnitt, zwei Ausnahmen | `grep -L 'Ab welchem macOS' crates/krk-ui/src/appkit/*.rs` | `koordinaten.rs` und `mod.rs`, wie beschrieben |
| `allow(unsafe_code)` an zwei Stellen | `grep -rn 'allow(unsafe_code)' crates/*/src` | `verzeichnis/sys.rs:97`, `appkit/mod.rs:1` |
| Aufrufer von `ohne_warten_oeffnen` | `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` | fünf, zwei davon außerhalb `text/datei.rs` |

## Datensätze

- `circles/260825-0711-…/issues/260825-1230_c_claude-md-fuehrt-die-runden-nur-bis-15-…` —
  vollständig behoben, `Resolved:` angehängt, `_o_` → `_c_`.
- `shared/issues/260825-0727_o_claude-md-nennt-zwei-aufrufer-…` — **bleibt offen.** Die
  `CLAUDE.md`-Hälfte ist erledigt; der Modulkopf von
  `crates/krk-core/src/verzeichnis/sys.rs` zählt an zwei Stellen weiter daneben und ist
  eine Codedatei, die dieser Auftrag ausdrücklich nicht anfasst. Eine Fortschrittsnotiz
  ist angehängt.

## Verification

`make check` ist nicht gefahren und war nicht zu fahren: keine Codedatei angefasst,
`git status --short -- crates/ xtask/ resources/ Cargo.toml Makefile release.sh` ist leer.
Geprüft ist stattdessen jede geänderte Aussage einzeln mit dem Kommando, das sie zählt;
die Tabelle darüber führt sie auf.
