CLAUDE.md sagt, den Tag setze der Nutzer; das Werkzeug setzt ihn seit dem 260813
---
`CLAUDE.md:106` schreibt über die Auslieferungskette: „Seit der Runde 8 bricht
`cargo xtask release` ab, wenn HEAD keinen Tag `v<version>` trägt, der zur
`Cargo.toml` passt — **den Tag setzt der Nutzer, nicht das Werkzeug**." Der zweite
Halbsatz stimmt seit dem 260813-1534 nicht mehr. `cargo xtask version <zahl>` setzt
den Tag selbst.
---
**Gemessen am 260819 gegen den Baumstand `76ceb68`**, aufgefallen bei der Frage, ob eine
Auslieferung auf 0.5.3 oder auf 0.5.4 gehen soll.

**Was tatsächlich gilt.** `xtask/src/version.rs:1` sagt im Modulkopf „Die Version setzen,
eintragen und **taggen**", und die Vorhabens-Aufzählung darunter führt drei Ausgänge, von
denen zwei taggen: `NurTaggen` und `SetzenEintragenTaggen`. Die Hilfe in
`xtask/src/main.rs:48-58` schreibt es aus: das Kommando „traegt Cargo.toml und Cargo.lock
als eine Aenderung ein **und setzt den Tag v<zahl> auf HEAD**".

`README.md` unter `### Versionsstufen` sagt dasselbe und nennt den Grund: „**Jede
Auslieferung bekommt einen Tag `v<version>`, und den setzt das Werkzeug.** Bis zum
260813-1534 galt das Gegenteil … Der Nutzer hat diese Festlegung am selben Tag
zurückgenommen, weil sie einen Auslieferungsweg in einem Kommando unmöglich macht."

**Die Datensätze sind in Ordnung, nur `CLAUDE.md` ist es nicht.**

- `shared/decisions/260813-1534_i_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`
  trägt `_i_`, ist also beantwortet **und** umgesetzt.
- Der überholte Datensatz
  `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/decisions/260813-0939_s_wer-setzt-den-ersten-tag-v0-1-0-und-wann.md`
  trägt korrekt `_s_`.

Die Markerkette stimmt also durchgehend. Was nicht nachgezogen wurde, ist der eine Halbsatz
in `CLAUDE.md`, und er steht ausgerechnet in dem Absatz, den jemand liest, der ausliefern
will.

**Warum das mehr kostet als eine falsche Prosazeile.** Wer den Satz glaubt, sucht vor einer
Auslieferung nach dem Handgriff, der den Tag setzt, findet keinen, und hält das Werkzeug für
kaputt. Der erste Satz desselben Absatzes ist dabei richtig — `cargo xtask release` bricht
ohne passenden Tag auf HEAD ab —, was den falschen zweiten Halbsatz besonders glaubwürdig
macht: die Abbruchmeldung bestätigt scheinbar, dass man selbst hätte taggen müssen.

**Zum Muster.** Es ist der dritte Befund dieser Sorte an `CLAUDE.md` binnen zweier Tage, alle
drei aus derselben Ursache: eine Aussage wurde von einem späteren Entscheid oder einer
späteren Runde überholt, und die Datei ist nicht mitgezogen. Die zwei anderen sind
`shared/issues/260816-2138_*_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md` (am 260819
sind es dreizehn) und
`shared/issues/260817-1610_*_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md`.
Alle drei gehören in denselben Kuratorenlauf.

**Die Berichtigung ist ein Halbsatz.** „den Tag setzt der Nutzer, nicht das Werkzeug" wird zu
einer Aussage, die sagt, dass `cargo xtask version` ihn setzt, mit Verweis auf
`shared/decisions/260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`.
Der Rest des Absatzes bleibt, wie er ist.

**Verweise:**
- `CLAUDE.md:106` — die Stelle
- `xtask/src/main.rs:48-58` — die Hilfe des Kommandos
- `xtask/src/version.rs:78-88` — die drei Vorhaben, zwei davon taggen
- `README.md`, `### Versionsstufen` — die richtige Aussage samt Begründung

---
Resolved: Doppelt gefilt und deshalb zurueckgezogen. Der Sachverhalt steht seit dem 260816 als `shared/issues/260816-1232_*_claude-md-sagt-den-tag-setze-der-nutzer-seit-dem-260813-setzt-ihn-das-werkzeug.md` im Speicher, samt der Folge, die hier als neu beschrieben ist. Was dieser Datensatz an Belegstellen mehr trug, ist dort als `Also seen:` angehaengt. Ursache des Doppels: die Konvention verlangt vor dem Schreiben ein Auflisten der offenen Datensaetze des Zielspeichers, und der Orchestrator hat es unterlassen.
