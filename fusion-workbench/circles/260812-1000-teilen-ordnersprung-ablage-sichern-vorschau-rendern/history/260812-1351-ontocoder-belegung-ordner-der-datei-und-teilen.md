# Schritte 2 und 4: Die Auslieferungsbelegung kennt `ordner_der_datei` und `teilen`

**Date:** 2026-08-12
**Agent:** ontocoder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritte 2 und 4
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test -p krk-core` — exit 0; `cargo test -p krk-ui` — exit 101, planmäßig rot

---

## Was geändert wurde

Eine Datei: `resources/default-keymap.toml`. Beide Schritte fassen dieselbe
Datei an und hängen nicht voneinander ab, also sind sie in einem Zug gefahren.

**`ordner_der_datei` (Schritt 2)** steht im Block `C2: Navigation in der Liste`,
unmittelbar hinter `ordner_aufwaerts` und vor `pfadeingabe`. `tasten =
["opt+cmd+o"]`. Der Kommentar nennt die Reihenordnung — die `opt+cmd`-Reihe
trägt, was einen Ordner herstellt oder liefert — und den Nachbarn `opt+cmd+c`,
der den Pfad desselben Ordners kopiert.

**`teilen` (Schritt 4)** steht im Block `Pfade kopieren und mit dem
Standardprogramm öffnen`, unmittelbar hinter `eintragspfad_kopieren`
(`shift+cmd+c`) und vor `mit_standardprogramm_oeffnen`. `tasten =
["shift+cmd+s"]`. Der Kommentar nennt die Reihenordnung — die `shift+cmd`-Reihe
trägt, was auf die betroffenen Einträge wirkt — **und den angenommenen
Konflikt** zu „Sichern unter" samt dem verworfenen Ausweichweg `shift+cmd+f`.

Beide Kommentare zitieren den Entscheidungsdatensatz
`circles/260812-1000-…/decisions/260812-1000_*_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`,
Möglichkeit 1, Nutzerentscheid vom 260812-1105.

`reserviert_fuer` ist bei keinem der beiden gesetzt: die Schritte 3 und 5 bauen
die Kommandos in derselben Runde, das Feld hieße „einer späteren Runde
vorbehalten" und wäre damit falsch.

**Die Zählzeile im Dateikopf** geht in einem Zug von 79 Funktionen und 85
Kombinationen auf **81 und 87**. Nicht fortgeschrieben, sondern nachgezählt:
`grep -c '^\[\[funktion\]\]'` liefert 81, die Summe der Einträge in allen
`tasten`-Listen 87.

## Dass die zwei Kombinationen frei waren

Vor dem Schreiben nachgezählt, nicht aus dem Plan übernommen. Die
Schreibweise dieser Datei ist festgelegt (`[ctrl+][opt+][shift+][cmd+]<taste>`,
in genau dieser Reihenfolge), es gibt also je Kombination genau eine
Schreibung. `grep` über alle Kombinationen, die auf `o` oder `s` enden, fand
vor der Änderung genau zwei Treffer: `ctrl+o` und `cmd+s`. Weder `opt+cmd+o`
noch `shift+cmd+s` stand in irgendeiner Tastenliste.

Nach der Änderung trägt die Datei genau eine doppelt vergebene Kombination,
`cmd+a` bei `alle_markieren` und `text_alles_auswaehlen`. Die ist von vorher
und im Dateikopf als Fokusvorbehalt begründet — kein Zuwachs aus diesen zwei
Schritten. Doppelte Kennungen gibt es keine.

## Abnahme

Nicht `make check`, wie der Plan es für diese zwei Schritte vorschreibt.
`cargo` liegt unter `$HOME/.cargo/bin` und stand jedem Aufruf voran.

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test -p krk-core` | 0 (12 Läufe, 339 Proben, 0 Fehlschläge) |
| `cargo test -p krk-ui` | 101 — planmäßig rot |

**Der rote Lauf ist der vom Plan angekündigte.** 366 Proben halten, 28 fallen,
alle 28 in `belegungsmodell::tests` und `belegungsausgabe::tests`. Es gibt genau
drei Panikstellen, und alle drei sind Schleifen über
`Belegung::auslieferung().funktionen()`:

- `belegungsmodell.rs:547` (`nach_bereichen`) — 26 Fehlschläge,
- `belegungsmodell.rs:679` (`jede_kennung_hat_einen_funktionsbereich`) — 1,
- `belegungsausgabe.rs:820` (`jede_kennung_ohne_kommando_wird_vom_menue_zugestellt`) — 1.

**Jede der 28 Meldungen nennt `ordner_der_datei`; keine nennt `teilen`**, und
das ist kein Befund, sondern die Folge des Abbruchs bei der ersten
unzugeordneten Kennung: alle drei Stellen brechen im ersten Durchlauf ab, der
keine Zuordnung findet, und `ordner_der_datei` steht in der Datei rund 340
Zeilen vor `teilen`. `teilen` fällt auf, sobald Schritt 3 die erste Kennung
einordnet, und Schritt 5 fängt es. Nichts außerhalb dieser drei Stellen ist rot.

## Nachwirkungen

- **Schritte 3 und 5 stehen aus.** Bis dahin ist `cargo test -p krk-ui` rot und
  `make check` damit auch. Ein Bündelbau in dieser Zwischenlage ist nicht
  geprüft worden und war nicht Teil des Auftrags.
- Beide Kennungen brauchen je eine Zeile in `Kommando::KENNUNGEN`,
  `Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs`) und
  `belegungsmodell::bereich_des_kommandos` — Sache des `coder`, Schritte 3 und 5.
- Kein Commit. Der Entscheidungsdatensatz bleibt auf `_a_`, weil er erst mit den
  Kommandos aus den Schritten 3 und 5 umgesetzt ist und nicht mit der Belegung
  allein.
