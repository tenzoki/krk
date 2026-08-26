Ein Messbericht kann einen früheren still überschreiben: der Dateiname hat Minutengenauigkeit

---

Alle drei Berichtsschreiber bilden ihren Dateinamen aus `kurzstempel`, also `JJMMTT-HHMM`, und
schreiben mit `fs::write`. Zwei Läufe in derselben Minute erzeugen denselben Namen, und der
zweite überschreibt den ersten ohne Meldung. Der Bericht ist der einzige Beleg dafür, dass eine
Zusage gehalten hat.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-bench/src/bericht.rs`, `crates/krk-bench/src/messen.rs`

## Die drei Stellen

- `bericht::schreiben` → `{kennung}-kopflos-{ordnername}-{zustand}.txt` (`bericht.rs:188` und
  `198`). Ordnername und Cache-Zustand stehen mit im Namen, also kollidieren nur zwei Läufe auf
  **demselben** Ordner in **demselben** Zustand.
- `bericht::gesamt_schreiben` → `{stempel}-alle-zusagen.txt` (`bericht.rs:495-498`). Nur der
  Stempel.
- `messen::durchstich_schreiben` → `{stempel}-durchstich.txt` (`messen.rs:2008-2011`). Nur der
  Stempel.

`fs::write` legt an oder kürzt; keiner der drei prüft, ob der Pfad schon steht.

## Wie wahrscheinlich das je Weg ist

Für `alle` und `durchstich` ist es unwahrscheinlich: ein Gesamtlauf fährt Minuten bis
Viertelstunden. Für `messen --kopflos` ist es das nicht — auf einem kleinen Prüfordner sind
zwanzig Läufe in Sekunden durch, und der übliche Weg, eine Streuung anzusehen, ist, den Befehl
zweimal hintereinander zu geben. `messungen/260810-1912-alle-zusagen.txt` und
`messungen/260810-1918-alle-zusagen.txt` zeigen, dass zwei Läufe kurz hintereinander vorkommen;
sechs Minuten haben dort gereicht.

## Warum es nicht nur eine Schönheitsfrage ist

`CLAUDE.md` und die Berichte unter `messungen/` sind die einzige Grundlage der Aussage „alle zehn
Zusagen halten". Ein verlorener Bericht ist ein verlorener Beleg, und der Verlust ist an nichts
zu erkennen: die Datei steht da, sie trägt die richtige Zeit, sie enthält nur nicht mehr den
Lauf, den jemand meint.

## Denkbarer Weg

Vor dem Schreiben prüfen, ob der Pfad schon steht, und dann entweder abbrechen (dieselbe Haltung
wie `pruefen_dass_leer` in `fixture.rs:373-392`: „Der Erzeuger ueberschreibt nichts") oder eine
Laufnummer anhängen. Der Abbruch passt besser zur Haltung dieser Kiste; er kostet den Messenden
eine Minute Wartezeit und keinen Beleg.
