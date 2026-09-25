# Coder — Schritt 5: Die acht Prosazahlen nachziehen

**Datum:** 260818-2112
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 5
**Baumstand beim Beginn:** `ebfab4f`, Arbeitsbaum sauber bis auf das Ereignisprotokoll

## Was der Auftrag war

Der letzte Schritt von Bündel A und reine Buchführung: acht Prosazahlen
nachziehen, die weder der Übersetzer noch eine Probe hält. Die Runde hat der
Belegung eine Funktion und der Aufzählung `Kommando` eine Variante
hinzugefügt; die Zahlen `84` und `78` in zwei Dateien reden noch vom Stand
davor.

## Gegen den Baum gezählt, nicht gegen den Plan

Der Plan hat seine Zahlen und Zeilennummern bei `b47355e` erhoben, zwei
Commits vor diesem Lauf. Jede Zahl ist deshalb neu erhoben worden:

| Was gezählt wird | Wie gezählt | Ergebnis |
|---|---|---|
| Funktionen der Auslieferungsbelegung | `grep -c '^\[\[funktion\]\]' resources/default-keymap.toml` | 85 |
| Kombinationen darin | `grep -oE '^tasten = \[.*\]' … \| grep -oE '"[^"]+"' \| wc -l` | 90 |
| Funktionen mit `Kommando` | `Kommando::KENNUNGEN` (`belegung.rs:661`), gleichlautend die Zahl der Varianten | 79 |
| Vom Menü zugestellte Funktionen ohne `Kommando` | `grep -c 'gehalten_von = "menue"' resources/default-keymap.toml` | 6 |

Die Zerlegung geht auf: 85 = 79 + 6. Die Sechs bleibt unverändert, und alle
acht Stellen, die von „den sechs Textbefehlen" sprechen, bleiben deshalb
stehen. Die Probe `belegungsausgabe.rs:757` hält die 79 selbst fest
(`mit_kommando == Kommando::KENNUNGEN.len()`); die Zahl in der Prosa ist ihr
Echo und wird von ihr nicht gehalten.

Die Zeilennummern des Plans stimmen alle acht unverändert: die beiden Dateien
sind seit `b47355e` nicht angefasst worden.

## Was geändert wurde

**`crates/krk-ui/src/belegungsausgabe.rs`** — sechs Zahlen an fünf Stellen:

| Zeile | vorher | nachher | was die Zahl zählt |
|---|---|---|---|
| 45 | `alle 84 Funktionen` | 85 | alle Funktionen der Belegung |
| 48 | `die 78 Funktionen mit [`Kommando`]` | 79 | Funktionen mit Kommando |
| 56 | `die 78 mit [`Kommando`]` | 79 | dieselben, in der Tabelle der vier Begründungslagen |
| 256 | `78 der 84` | `79 der 85` | beide, im Kommentar am ersten Zweig von `wirkung` |
| 730 | `alle 84` | 85 | alle Funktionen, im Doc-Kommentar der Probe |
| 731 | `die 78 mit Kommando` | 79 | Funktionen mit Kommando, ebenda |

**`crates/krk-ui/src/appkit/menue.rs`** — vier Zahlen an vier Stellen:

| Zeile | vorher | nachher | was die Zahl zählt |
|---|---|---|---|
| 128 | `der 84 Funktionen` | 85 | alle Funktionen, im Modulkopf |
| 799 | `einzige der 84 Funktionen` | 85 | alle Funktionen, an `die_sechs_zugestellten` |
| 867 | `78 der 84 Funktionen` | `79 der 85` | beide, an der Messprobe aus S1 der Runde 3 |
| 1132 | `Tafel aus 140 Faellen` | 280 | **nicht im Plan**, siehe unten |

## Eine neunte Stelle, die der Plan nicht führt

`menue.rs:1132` verweist auf „die Tafel aus 140 Faellen" in
`crate::kommandos::zulaessigkeit`. Diese Tafel trägt heute **280** Fälle: die
Probe heißt `die_tafel_aus_zweihundertachtzig_faellen_geht_auf` und schließt
mit `assert_eq!(geprueft, 280, …)` (`zulaessigkeit.rs:436`). Die Zahl steht
seit `16c0924` in `menue.rs`; `c3ada4d` („die Zulaessigkeitsregel fragt jetzt
auch nach dem Schluesselfenster") hat die Tafel um eine Dimension erweitert
und damit verdoppelt, ohne den Verweis nachzuziehen. Es ist dieselbe Sorte
Zahl, die dieser Schritt behebt, und sie steht in einer der zwei Dateien, die
der Schritt anfassen darf; deshalb ist sie mitkorrigiert.

## Was ausdrücklich nicht angefasst wurde

- `belegungsansicht.rs:665` und `:738` — Maße in Punkten, keine Zählungen. Der
  Plan nennt sie als falsche Freunde, und sie sind es.
- `belegungsausgabe.rs:533` und `:572` („fünf" unbelegte Funktionen ab Werk) —
  am Baum nachgezählt und weiterhin richtig: fünf `tasten = []`-Blöcke, und
  `ordner_angleichen` trägt `opt+cmd+s`, fällt also nicht darunter.
- `belegungsausgabe.rs:230` („die sieben Beschriftungen von
  `Wirkungsbereich`") — nachgezählt, sieben Werte, richtig.
- `menue.rs:9` („neun Obermenues") — `Funktionsbereich::ALLE` trägt neun,
  richtig.
- `menue.rs:1224` („Heute sind es acht") — von der Probe daneben mit
  `assert_eq!(alle, 8, …)` gehalten, also keine reine Prosazahl.

## Abweichungen zwischen Plan und Baum

Keine bei den acht genannten Stellen: Zeilennummern und Ausgangszahlen stimmen
sämtlich. Die einzige Abweichung ist die neunte Stelle oben, die der Plan
nicht kennt.

## Abnahme

`make check` — Exit 0, alle vier Kommandos grün (Bau, Proben, Clippy unter
`-D warnings`, `fmt --check`). Vor dem Lauf geprüft, dass weder `/tmp` noch
`$TMPDIR` eine `krk-messplan-*.toml` führt; es lief kein Messlauf.

**Nicht committet** — der Nutzer committet selbst.

## Was Schritt 5 nicht behebt

Die Ursache. Diese Sorte Zahl ist in diesem Baum wiederholt veraltet, und die
neunte Stelle oben ist der frische Beleg dafür: sie stand über mehrere Runden
falsch da, ohne dass eine der Erhebungen sie gesehen hätte. Der Plan sagt
ausdrücklich, dass der Schritt den heutigen Stand behebt und nicht die
Ursache; `CLAUDE.md` führt für dieselbe Sorte eigene Befunde.
