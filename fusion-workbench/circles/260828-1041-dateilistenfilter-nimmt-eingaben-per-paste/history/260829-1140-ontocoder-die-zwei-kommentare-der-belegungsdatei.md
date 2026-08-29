# Ontocoder-Sitzung: Schritt 4 der Runde 21, die zwei Kommentare der Belegungsdatei

**Date:** 2026-08-29, 260829-1140
**Filed by:** ontocoder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste`
**Plan:** `planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 4
**Datei:** `resources/default-keymap.toml`, allein Kommentarzeilen

## Was getan wurde

Drei Kommentarstellen auf den Stand nach den Runden 22 und 21 gezogen: der Absatz zu Cmd+C und Cmd+V im Kopfblock (vorher `:81-84`), der Schlussabsatz des Blocks `C10: die Zwischenablage als Quelle` (vorher `:659-663`) und der Einhaengepunkt-Absatz im Block `C2: die Textbefehle des Menues "Bearbeiten"` (vorher `:990-997`). Die dritte Stelle nennt den Plan nicht ausdruecklich; sie sprach `paste:` derselben spaeteren Dateizwischenablage zu und ist deshalb mitgezogen (Dispatch: 660-665).

Kein `[[funktion]]`-Block, keine `tasten`-Zeile, kein neuer Eintrag; `text_einfuegen` bleibt zeichengleich (C1.9, A13, Constraint 5). Nachweis: `git diff -I '^\s*#' HEAD -- resources/default-keymap.toml` liefert 0 Byte.

## Die Fassungen

Vorher, Kopfblock:

```
# Cmd+C und Cmd+V standen bis zum 260805 in dieser Aufzaehlung. Sie tragen
# seither die Textbefehle des Menues "Bearbeiten" und sonst nichts, und genau
# das haelt sie fuer die Dateizwischenablage einer spaeteren Runde frei: ohne
# jenes Menue erreicht Cmd+V ueberhaupt kein Textfeld. Die Reservierung aus C3
# ist damit eingeloest und nicht gebrochen; C3 schreibt aus, warum
```

Nachher:

```
# Cmd+C und Cmd+V standen bis zum 260805 in dieser Aufzaehlung. Sie tragen
# seither die Textbefehle des Menues "Bearbeiten" und sonst nichts: ohne
# jenes Menue erreicht Cmd+V ueberhaupt kein Textfeld. Bis zur Runde 22 hielt
# genau das die beiden fuer eine spaetere Runde frei; seither sind sie am
# Dateifenster besetzt, und zwar ohne eine weitere Zeile in dieser Datei.
# `copy:` und `cut:` beantwortet der Anwendungsdelegierte seit der Runde 22
# und legt Dateiverweise fuer andere Anwendungen ab, `paste:` seit der Runde
# 21 und haengt den Text oder den Dateinamen aus der Zwischenablage an den
# Filtertext an; Dateien fuegt KRK nicht ein. Die Reservierung aus C3 ist
# damit ganz eingeloest und nicht gebrochen; C3 schreibt aus, warum
```

Vorher, C10:

```
# ganz unten und sonst nichts, und genau das haelt sie fuer die
# Dateizwischenablage einer spaeteren Runde frei (C3, C10).
```

Nachher:

```
# ganz unten und sonst nichts (C3, C10). Wer sie am Dateifenster beantwortet,
# steht dort: `copy:` und `cut:` der Anwendungsdelegierte seit der Runde 22,
# `paste:` seit der Runde 21, als Einfuegen in den Filtertext.
```

Vorher, C2:

```
# Textfeld ist das der Feldeditor von AppKit, im Dateifenster heute niemand,
# weshalb der Eintrag dort grau ist. Genau dieser Punkt ist spaeter der
# Einhaengepunkt der Dateizwischenablage: wer `copy:` und `paste:` am
# Dateifenster beantwortet, hat sie, ohne einen zweiten Menueintrag und ohne
# eine zweite Zeile in dieser Datei.
```

Nachher:

```
# Textfeld ist das der Feldeditor von AppKit, im Dateifenster der
# Anwendungsdelegierte: `copy:` und `cut:` beantwortet er seit der Runde 22
# und legt Dateiverweise fuer andere Anwendungen ab, `paste:` seit der Runde
# 21 und haengt den Text oder den Dateinamen aus der Zwischenablage an den
# Filtertext an. Bis zur Runde 22 antwortete im Dateifenster niemand, und der
# Eintrag war dort grau; genau dieser Punkt war als Einhaengepunkt einer
# Dateizwischenablage vorgemerkt. Besetzt ist er jetzt, ohne einen zweiten
# Menueintrag und ohne eine zweite Zeile in dieser Datei, und Dateien fuegt
# KRK dabei nicht ein. Was Cmd+V mit einem Dateiverweis tut, sobald eine
# Dateizwischenablage gebaut wird, ist offen:
# `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/decisions/260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`.
```

## Pruefung

Der Baum traegt waehrend dieser Sitzung die unverbundenen Aenderungen dreier Coder unter `crates/`; die Ergebnisse gelten fuer den Baum, wie er stand.

- `cargo build --workspace`: exit 0
- `cargo clippy --workspace --all-targets`: exit 0, keine Warnung
- `cargo fmt --all --check`: exit 0
- `cargo test --workspace`: exit 101; rot ist allein `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`crates/krk-core/tests/verzeichnis.rs:3257`), die Zaehlprobe, die Schritt 2 auf drei Rufer umbenennt. Sonst nichts rot.
- `make check`: exit 2, an derselben Probe; die Kette bricht bei `test` ab.
- `git diff -I '^\s*#' HEAD -- resources/default-keymap.toml | wc -c`: 0

## Was nicht getan wurde

Nichts unter `crates/` angefasst, nichts committet, kein baumweites git-Kommando gefahren. Kein Defekt gefiled: die rote Probe ist die vom Plan erwartete.
