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

4. **Der nächste Lauf räumt auf** — `Messplanwaechter::neu` löscht beim Anlegen jede
   `krk-messplan-*.toml` im Temporärverzeichnis, die nicht seine eigene ist. Nachgetragen am
   260810-1905 von `coderev` bei der Durchsicht von Turn 2.
   - Pro: Keine neue Bauform, sondern genau die Zeile, die `Wegwerfordner::neu`
     (`wegwerfordner.rs:45`) schon trägt. `SICHERUNG` bleibt unberührt, die Signalwache bekommt
     keine Zeile, die dokumentierte Zusage wird weder gebrochen noch neu gefasst. Deckt
     zusätzlich `SIGKILL` und den Stromausfall ab, die keine der drei anderen Optionen erreicht.
   - Contra: Der Plan liegt bis zum nächsten Lauf da, statt sofort wegzukommen. Schließt zwei
     gleichzeitige Messläufe aus — was der Abnahmelauf ohnehin tut, weil er KRK im Vordergrund
     verlangt, aber es ist eine Zusage, die heute nirgends steht.

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

**Option 4**, nachgetragen am 260810-1905. Die ursprüngliche Fassung dieses Datensatzes gab keine
Empfehlung ab, weil die ersten drei Optionen sich allein darin unterschieden, was sie an Entwurf
kosten: Option 1 lässt die vorhandene Zusage wörtlich stehen und verdoppelt dafür einen
Mechanismus, Option 2 fasst die Zusage neu, Option 3 ändert das Abbruchverhalten. Zwischen diesen
drei war die Abwägung eine Geschmacksfrage und gehörte dem Nutzer.

Option 4 ändert die Lage, weil sie keinen dieser Preise zahlt. Sie berührt die Zusage nicht,
verdoppelt nichts, ändert das Abbruchverhalten nicht, und sie benutzt eine Zeile, die im
Nachbarmodul schon steht. Dazu deckt sie zwei Ausgänge ab, die keine der drei anderen erreicht:
`SIGKILL` und den Stromausfall. Ihr einziger Preis ist, dass der Plan bis zum nächsten Lauf
liegen bleibt — und das ist genau der Zustand, den dieser Defekt heute beschreibt, nur mit einer
Obergrenze von einer Datei statt neun.

Was dagegen spricht und mitentschieden gehört: Option 4 setzt voraus, dass nie zwei Messläufe
gleichzeitig laufen. Das trifft heute zu, weil der Abnahmelauf KRK im Vordergrund verlangt, aber
diese Zusage steht nirgends geschrieben. Wer Option 4 wählt, schreibt sie auf.

---
Answered:
Implemented:
Deferred:
Superseded by:
