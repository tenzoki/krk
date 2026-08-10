# Wie kommt der Messplan bei Strg+C weg, ohne die Zusage der Sitzungssicherung zu brechen?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (nach Vorlage durch coder, Turn 2 der Sitzung 260810-1647)
**Cross-references:** `shared/issues/260810-1745_*_der-messplanwaechter-greift-bei-strg-c-nicht-weil-process-exit-kein-drop-laeuft.md`,
`shared/issues/260810-1330_*_der-messplan-bleibt-liegen-wenn-eine-runde-abbricht.md`,
`crates/krk-bench/src/messen.rs` (`Messplanwaechter`, `Sitzungssicherung`, `signalwache_starten`),
`crates/krk-bench/src/wegwerfordner.rs`

---

## Frage

Der `Messplanwaechter` löscht den Messplan im `Drop` und deckt damit Erfolgsweg, jedes `?`, die
Panik-Abwicklung und seit Turn 2 auch das Schreiben selbst ab. Einen Ausgang deckt er nicht:
`signalwache_starten` endet in `std::process::exit`, und dabei läuft kein `Drop`. Bei Strg+C
bleibt die Datei liegen.

Die Sitzungssicherung löst dasselbe Problem für die `session.toml` des Nutzers über eine
statische `SICHERUNG`, die die Signalwache abarbeitet. `SICHERUNG` ist ein
`Mutex<Option<Sitzungssicherung>>`, also auf genau **eine** Nutzlast typisiert, und trägt die
dokumentierte Zusage: „genau einer von beiden spielt zurück: wer zuerst kommt". Den Messplan
dort einzutragen berührt diese Zusage. Deshalb ist es eine Entscheidung und keine Ergänzung.

## Optionen

1. **Zweiter statischer Platz derselben Bauform** — `SICHERUNG` bleibt unberührt, ein zweiter
   Platz kommt daneben, die Signalwache bekommt eine Zeile mehr.
   - Pro: Die Zusage der Sitzungssicherung bleibt wörtlich stehen. Kein zweiter Signalpfad.
   - Contra: Eine wörtliche Kopie des Vierergespanns aus statischem Platz, Abräumfunktion,
     Stapelwächter und Nutzlast, rund 35 Zeilen. `plan_schreiben` müsste den Pfad getrennt
     zurückgeben, weil `fahren` ihn in jeder Runde braucht. Das ist der doppelte Mechanismus,
     den `HYG-USE-ABSTRACTIONS` als Defekt führt.

2. **Ein Verzeichnis statt eines Platzes** — `SICHERUNG` trägt eine Liste von Wächtern, die
   einmal abgeräumt wird.
   - Pro: Ein Mechanismus für N Aufräumaufgaben. Der nächste Fall dieser Art kostet eine Zeile
     statt 35. Sauber im Sinne des Entwurfs.
   - Contra: Der dokumentierte Satz „wer zuerst kommt" muss für N Einträge neu geschrieben
     werden, und die Reihenfolge zwischen Sitzungsrückspielen und Messplan-Löschen zieht vom
     Stapel in die Liste um. Die Zusage wird also nicht gebrochen, aber neu gefasst.

3. **Ohne statischen Platz** — die Signalwache setzt eine Marke und lässt den Hauptfaden früh
   zurückkehren; dann laufen alle `Drop` von selbst.
   - Pro: Die Frage löst sich auf, statt beantwortet zu werden. Kein statischer Zustand mehr,
     kein Sonderweg für die nächste Aufräumaufgabe.
   - Contra: Ändert das Abbruchverhalten. Der Ausgangswert 128+Signal fällt weg oder muss neu
     gebaut werden, die Sofortigkeit des Abbruchs geht verloren, und der Hauptfaden hängt bei
     einem Messlauf oft in `warten_bis` am Kindprozess, kommt also nicht ohne Weiteres zurück.

## Randbedingungen

- Kein zweiter Signalpfad neben dem bestehenden. Das ist keine Option, sondern Ausschluss.
- Der Ausgangswert bei Abbruch soll bleiben, was er ist, solange nichts dagegen spricht.
- Die Änderung darf die Zeitzusagen aus C8 nicht berühren; die Messstrecke misst, sie soll
  nicht selbst zum Gegenstand werden.

## Ein Nebenbefund, der jede Option betrifft

`signalwache_starten` läuft erst mit `Sitzungssicherung::anlegen()` bei `messen.rs:1034`,
`plan_schreiben` schon bei `messen.rs:1029`. In dieser Spanne fängt heute **gar nichts** ein
Strg+C ab, gleich welche der drei Optionen gewählt wird. Wer die Frage beantwortet, zieht die
Reihenfolge dieser beiden Aufrufe mit — sonst bleibt ein Fenster offen, das die gewählte Lösung
nicht schließt.

## Empfehlung

Keine. Die drei Optionen unterscheiden sich in dem, was sie an Entwurf kosten, und diese Abwägung
gehört dem Nutzer. Anzumerken ist allein: Option 1 ist die einzige, die die vorhandene Zusage
wörtlich stehen lässt, und zugleich die einzige, die einen Mechanismus verdoppelt. Zwischen
diesen beiden Gütern ist zu wählen.

---
Answered:
Implemented:
Deferred:
Superseded by:
