# Ziehen die vier Instanzproben in ein Prüfziel ohne libtest-Harness um?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `issues/260810-1001_o_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`, `issues/260810-0748_c_die-kopplung-der-zehn-paare-traegt-den-commit-und-ist-im-baum-durch-nichts-gehalten.md`, `issues/260810-0750_c_derselbe-speicher-ist-eine-stufe-staerker-als-die-messung-hergibt.md`, `issues/260810-0512_c_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`, `issues/260810-0212_o_drei-stuecke-des-editormodells-haben-keinen-aufrufer-und-der-plan-nennt-keinen.md`

---

## Frage

Vier Proben in `crates/krk-ui/src/appkit/editor.rs` bauen eine `NSTextView`, um
Eigenschaften von AppKit zu messen statt sie zu behaupten: die Kopplung der zehn
Paare, die Nicht-Darstellbarkeit von `Default`, die Sammeltür und die sieben
abgeschalteten Automatiken. Alle vier gehen durch `an_einer_flaeche`, und dort
steht `unsafe { MainThreadMarker::new_unchecked() }` — eine Behauptung über den
Faden, die `libtest` nicht deckt, während Apple für eine `NSView` den Hauptfaden
zusagt. Der Defekt ist `260810-1001`.

Der Weg heraus ist gemessen und liegt fest; zu entscheiden ist, ob er gebaut wird
und wie die vier Proben an ihre Messstücke kommen. Jetzt, weil die Notlüge
sonst stehenbleibt und der nächste Durchgang sie als gegeben nimmt.

## Was gemessen ist

Am 260810-1044 auf macOS 15.7.7 (Build 24G720), Rust 1.97.1:

```
  cargo test                          MainThreadMarker::new() ─> None
  cargo test -- --test-threads=1      MainThreadMarker::new() ─> None
  [[test]] mit harness = false        MainThreadMarker::new() ─> Some
```

Zwei Folgerungen. **`libtest` gibt den Hauptfaden nicht her**, auch nicht bei
einem einzigen Prüffaden: es legt jede Probe auf einen eigenen Faden. Und **ein
Prüfziel ohne libtest-Harness bekommt ihn** — es hält sein `main` selbst — und
wird von `cargo test` mitgefahren, also auch von `make check`. Die Annahme in
`260810-1001`, Weg 2 koste ein zweites Prüfkommando, ist damit widerlegt.

## Optionen

1. **Prüfziel ohne Harness, Messstücke öffentlich machen** — ein
   `[[test]] name = "textflaeche", harness = false` in
   `crates/krk-ui/Cargo.toml`, die Prüflaufdatei darunter, und
   `textflaeche_bauen`, `EINSTELLUNGEN`, `merkmal`, `merkmal_setzen`,
   `merkmalsname`, `probenrahmen` werden `pub`.
   - Pro: die Proben messen auf dem Faden, den Apple zusagt; die Notlüge fällt;
     `make check` bleibt unverändert.
   - Contra: sechs öffentliche Stücke ohne Aufrufer im Programm. Genau das Muster,
     das `260810-0212` in diesem Circle als Befund führt.
2. **Prüfziel ohne Harness, Messstücke bleiben modulintern** — die vier Proben
   bleiben in `editor.rs`, und das Prüfziel ruft eine **eine** öffentliche
   Funktion, die sie der Reihe nach fährt und ihre Fehlschläge als Text
   zurückgibt.
   - Pro: eine öffentliche Stelle statt sechs, und sie hat einen Aufrufer.
   - Contra: die Proben verlieren `#[test]`, also ihre Namen in der Prüfreihe und
     die Einzelauswahl über `cargo test <name>`. Der Fehlschlag kommt als Text
     und nicht als Prüfergebnis, und der Prüflauf muss ihn selbst zu einem
     Fehlschlag machen.
3. **Stehen lassen** — Weg 1 aus `260810-1001`: beobachten, und beim ersten
   Bruch auf einem anderen Gerät zurückbauen.
   - Pro: kostet nichts.
   - Contra: der Bruch fällt dem Nutzer auf und nicht dem Bau, und die Behauptung
     bleibt eine.

## Constraints

- `make check` darf kein zweites Kommando brauchen; die vier Abnahmekommandos aus
  `CLAUDE.md` bleiben, wie sie sind.
- Die vier Messungen dürfen nicht aus dem Baum fallen: sie tragen die
  Entscheidung, `textflaeche_bauen` **nicht** um zehn Zeilen zu ergänzen
  (`260810-0748`), den gemessenen Vorgabewert der Schreibwerkzeuge
  (`260810-0512`) und die Aussage über `Default` (`260810-0750`).
- Weg 3 aus `260810-1001` (zurücknehmen und unter `spikes/` ablegen) ist
  ausgeschlossen: er nimmt die Messungen aus dem Baum.
- `krk-ui` trägt `#![deny(unsafe_code)]`; die Ausnahme in `appkit/mod.rs` deckt
  `unsafe`, aber sie deckt nicht die Zusage, die `new_unchecked` verlangt.

## Recommendation

Option 2. Sie löst die Fadenfrage vollständig und legt genau eine öffentliche
Stelle frei, die auch einen Aufrufer hat; der Verlust der vier Prüfnamen ist der
kleinere Preis gegenüber sechs öffentlichen Stücken, die das Programm selbst
nicht ruft. Wer die Prüfnamen für wichtiger hält als die Zahl der freigelegten
Stücke, nimmt Option 1 — beide sind gegenüber Option 3 ein Fortschritt.

---
Answered:
Implemented:
Deferred:
Superseded by:

---

## Nachgeprüft am 260810-1139: die Messung hält, beide Optionen tragen nicht

**Die Messung des Vorgängers ist bestätigt.** Ein `[[test]]`-Ziel mit
`harness = false` unter `crates/krk-ui/tests/` bekommt den Hauptfaden, und
`cargo test` fährt es mit, ohne zweites Prüfkommando. Nachgemessen am 260810-1057
auf macOS 15.7.7, Rust 1.97.1, an einem Prüfziel in diesem Projekt:

```
  [[test]] mit harness = false        MainThreadMarker::new() ─> Some
```

Dieselbe Strecke hat die Messung zu `260809-2322` gefahren; sie stand und lieferte
Zahlen.

**Beide Optionen dieses Datensatzes ruhen trotzdem auf einer falschen Annahme:
`krk-ui` hat kein Bibliotheksziel.** Die Kiste führt allein `[[bin]] name = "krk"`
mit `src/main.rs`; ein `src/lib.rs` gibt es nicht. Eine Prüflaufdatei unter
`tests/` ist eine eigene Kiste und kann deshalb **nichts** aus `krk-ui`
ansprechen, ob `pub` oder nicht. Gemessen, nicht geschlossen:

```
  error[E0433]: cannot find module or crate `krk_ui` in this scope
    --> crates/krk-ui/tests/probe_pruefziel.rs:3:13
```

Damit fällt Option 1 (sechs Stücke `pub` machen) genauso wie Option 2 (eine
öffentliche Stelle, die die vier Proben fährt): beide setzen voraus, dass
Sichtbarkeit reicht, und das tut sie ohne Bibliotheksziel nicht.
`crates/krk-ui/tests/syntaxkiste.rs` läuft nur deshalb, weil es allein `syntect`
und `two-face` anspricht und keine Zeile von `krk-ui`.

## Was daraus folgt: die Frage hat eine vierte und eine fünfte Option

Zwei Wege bleiben, und beide sind größer als das, was dieser Datensatz zur
Entscheidung stellte:

4. **`krk-ui` bekommt ein Bibliotheksziel.** `src/lib.rs` trägt den Modulbaum,
   `src/main.rs` wird zur Hülle darüber. Das ist ein Umbau der ganzen Kiste, weil
   jeder `crate::`-Pfad in zwölf Modulen und zweiundzwanzig AppKit-Untermodulen
   dann durch die Bibliothek läuft. Er löst die Fadenfrage endgültig und
   nebenbei jede künftige Prüfung von außen.
   - Contra: der größte Eingriff, den diese Runde vorgeschlagen bekommt, und er
     berührt jede Datei der Kiste.
5. **Ein zweiter Kistenkopf unter `src/`.** `[[test]] path = "src/<name>.rs"` mit
   `harness = false` und derselben `mod`-Liste wie `main.rs` übersetzt die Kiste
   ein zweites Mal als Prüfziel. Ein neues Modul in `main.rs`, das `editor.rs`
   braucht, hält den Bau an — das ist laut und nicht still.
   - Contra: die ganze Oberfläche wird zweimal übersetzt, und `harness = false`
     schaltet `cfg(test)` ab, also müssten die vier Proben samt `EINSTELLUNGEN`
     aus `#[cfg(test)] mod tests` heraus in gewöhnlichen Programmtext. Damit
     stünden sie im Bündel.

**Empfehlung, geändert gegenüber der oben:** Option 4, und nicht in einem
Bugfix-Durchgang. Sie ist die einzige, die die Notlüge ohne Nebenwirkung entfernt;
Option 5 tauscht sie gegen zwei Übersetzungen und Prüfcode im Bündel, und das ist
kein Fortschritt. Bis dahin gilt Weg 1 aus `260810-1001` als Lage und nicht als
Wahl, so wie der Doc-Kommentar von `an_einer_flaeche` es schon sagt.

**Der Datensatz bleibt offen**, und er stellt jetzt eine andere Frage als am
260810-1044: nicht mehr „wie kommen die vier Proben an ihre Messstücke", sondern
„bekommt `krk-ui` ein Bibliotheksziel". Wer sie beantwortet, hat damit auch
`issues/260810-0212` in der Hand, das öffentliche Stücke ohne Aufrufer führt: mit
einem Bibliotheksziel hätte „öffentlich" wieder eine Bedeutung.

Die Dateigrenze dieses Durchgangs lief um `appkit/editor.rs`, `hervorhebung.rs`
und `krk-ui/Cargo.toml`; ein `src/lib.rs` und ein Umbau von `main.rs` lagen
außerhalb.
