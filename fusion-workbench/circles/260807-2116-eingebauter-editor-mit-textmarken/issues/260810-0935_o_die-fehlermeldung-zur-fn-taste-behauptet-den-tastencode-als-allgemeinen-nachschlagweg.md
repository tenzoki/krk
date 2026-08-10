# Die Fehlermeldung zur fn-Taste behauptet den Tastencode als allgemeinen Nachschlagweg

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** ontocoder, bei der Nebenwirkungsprüfung zu `260810-0914_*_der-dateikopf-der-belegung-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`
**Betroffen:** `crates/krk-core/src/tasten/parser.rs:453-456` (`fmt::Display for Schreibfehler`, Zweig `FnAlsZusatztaste`)
**Cross-references:** `issues/260810-0914_*_der-dateikopf-der-belegung-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md` (dieselbe Prämisse im Dateikopf, dort behoben), `issues/260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md` (dieselbe Prämisse in zwei Kommentarblöcken, behoben), `crates/krk-core/src/tasten/parser.rs` (Modulkopf, `Taste::kennung`, `Kombination::aus_tastendruck`), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

Der Text, den ein Nutzer zu sehen bekommt, wenn er `fn+f3` in seine `keymap.toml` schreibt, führt dieselbe Prämisse, die im Dateikopf der Belegung und in zwei ihrer Kommentarblöcke gerade richtiggestellt worden ist:

```rust
Schreibfehler::FnAlsZusatztaste => ausgabe.write_str(
    "fn ist keine Zusatztaste einer Belegung; KRK belegt den Tastencode, \
     und F3 mit gehaltener fn erzeugt denselben wie ein nacktes F3",
),
```

Der erste Halbsatz ist seit S2 (`00719cb`) als allgemeine Aussage falsch. `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer ASCII-Ziffer auf `Tastenkennung::Zeichen`, und `Kombination::aus_tastendruck` (dort, Zeilen 569-576) filtert bei der Stellensuche über den Code jede Taste mit Zeichenkennung ausdrücklich aus. Über den Tastencode gehen nur noch Funktionstasten, Pfeilblock und Steuertasten.

**Die Aussage der Meldung bleibt richtig**, weil F3 eine Funktionstaste ist und weiter über den Code nachgeschlagen wird. Falsch ist allein die Reichweite der Prämisse. Der Modulkopf derselben Datei schreibt beide Nachschlagarten in seinem Abschnitt "Zwei Nachschlagarten, und warum es zwei sein muessen" korrekt aus; die Meldung 260 Zeilen darunter widerspricht ihm nun sichtbar.

## Warum das zählt

Das ist die einzige der vier Stellen mit dieser Prämisse, die ein Nutzer im Betrieb sieht: die drei übrigen standen in Kommentaren einer Datei, die er nur beim Umbelegen aufschlägt. Eine Fehlermeldung ist der Ort, an dem er die Regel lernt, und sie lehrt ihn hier die falsche.

`Low` und nicht `Medium`, weil keine Regel für eigene Belegungen daran hängt: die Meldung erscheint genau in dem Fall, für den ihre Aussage zutrifft, und weist ihn korrekt ab. Wer sie liest, wird nichts Falsches tun, sondern etwas Falsches glauben.

## Was zu tun ist

Eine Wortänderung, kein Umbau, in derselben Form, in der der Dateikopf sie am 260810 bekommen hat: den Halbsatz auf die Tastensorte einschränken, über die die Meldung tatsächlich spricht. Etwa "fn ist keine Zusatztaste einer Belegung; Funktionstasten schlaegt KRK ueber den Tastencode nach, und F3 mit gehaltener fn erzeugt denselben wie ein nacktes F3". Die Formulierung ist Sache des `coder`; der Dateikopf von `resources/default-keymap.toml` (Zeile 42) trägt jetzt die entsprechende und ist der Vergleichspunkt.

Keine Probe hängt am Wortlaut. Geprüft über eine Suche nach `FnAlsZusatztaste` und nach dem Meldungstext über `crates/`: die drei Fundstellen sind der Modulkopf (Zeile 71), die auslösende Stelle (Zeile 518) und eine Probe, die auf die Variante und nicht auf ihren Text vergleicht (Zeile 752). Der Wortlaut selbst wird nirgends behauptet.

Ausführender ist `coder`, weil `.rs` nicht dem `ontocoder` gehört. Nicht bei `260810-0914` mit erledigt, weil dessen Schreibgrenze ausdrücklich auf `resources/default-keymap.toml` lautete und an `crates/**` zur selben Zeit andere Agenten arbeiteten.

## Eine weitere Fundstelle, die kein Defekt ist

Der Spec der Runde 1 (`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md`, Zeilen 176 und 243) führt denselben Satz zweimal. Beide Stellen leiten daraus etwas über F1 ab, also über eine Funktionstaste, und beide Ableitungen tragen weiter; das Dokument ist geschlossen (`_c_`) und beschreibt den Stand seiner Runde, in dem die Aussage allgemein zutraf. Hier bewusst nicht als Defekt geführt, sondern genannt, damit die nächste Suche nach dieser Prämisse nicht auf einen ungeklärten Rest stößt.
