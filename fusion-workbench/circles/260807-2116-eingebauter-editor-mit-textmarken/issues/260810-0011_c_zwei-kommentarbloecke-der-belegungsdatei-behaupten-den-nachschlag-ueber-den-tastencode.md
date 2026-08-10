# Zwei Kommentarblöcke der Belegungsdatei behaupten den Nachschlag über den Tastencode

---
**Domain:** data
**Schwere:** Medium
**Gefunden von:** coder, bei der Abnahme von S41
**Betroffen:** `resources/default-keymap.toml`, Zeilen 484–492 und 617–628
**Cross-references:** `crates/krk-core/src/tasten/parser.rs` (Modulkopf, `Tastenkennung`), `crates/krk-ui/src/appkit/ereignisse.rs:134-142` (dort schon gezogen), `issues/260809-1746_*_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md` (derselbe Anlass, anderer Ort), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

`resources/default-keymap.toml` erklärt an zwei Stellen, warum die elf
Editor-Kombinationen `y` und `z` meiden, und begründet es mit dem Mechanismus:

Zeile 484–486: "KRK belegt nach C3 der Runde 1 den virtuellen Tastencode, also
die **Stelle** auf der Tastatur, und genau diese beiden Stellen tauschen zwischen
der deutschen und der amerikanischen Belegung den Platz."

Zeile 619–622: "Die Regel gilt dem, was **KRK selbst** zustellt: der
Ereignisabgriff schlägt über den virtuellen Tastencode nach, also über die Stelle
auf der Tastatur, und die tauscht zwischen der deutschen und der amerikanischen
Belegung den Platz."

Seit S2 (`00719cb`) trifft das nicht mehr zu. `Taste::kennung` in
`crates/krk-core/src/tasten/parser.rs` legt einbuchstabige Namen auf
`Tastenkennung::Zeichen`, und `Kombination::aus_tastendruck` schlägt Buchstaben
und Ziffern über das gemeldete Zeichen nach; über den Code gehen nur noch
Funktionstasten, Pfeilblock und Steuertasten. Der Ereignisabgriff hat seinen
Modulkopf dabei mitgezogen (`ereignisse.rs:134-142`), die Belegungsdatei nicht.

Zeile 490–492 nennt die zugehörige Entscheidung außerdem als "offene Frage"; sie
ist am 260808-0155 beantwortet und in S2 umgesetzt.

## Warum das zählt

Die Datei ist nach C7 und C11 der Runde 1 von Hand lesbar und änderbar, also
Nutzerdokumentation und nicht nur Programmeingabe. Ein Kommentar, der den
Nachschlagweg falsch beschreibt, führt jeden in die Irre, der sich eine eigene
Kombination legen will: er meidet Buchstaben, die er nicht meiden muss, und er
erwartet einen Stellentausch, den es nicht mehr gibt.

Der zweite Block trägt zusätzlich die Begründung, aus der `cmd+z` und
`shift+cmd+z` ausgeliefert werden. Sie ist im Ergebnis richtig und in der
Herleitung falsch — die beiden Kürzel wirken heute an der beschrifteten Stelle,
weil **jeder** Buchstabennachschlag das tut, und nicht, weil das Menü zustellt.

## Was zu tun ist

Beide Blöcke auf den Stand nach S2 ziehen. Die Sachaussage ist an einer Stelle
ausgeschrieben und gehört nicht ein drittes Mal ausformuliert: der Modulkopf von
`crates/krk-core/src/tasten/parser.rs`, Abschnitt "Zwei Nachschlagarten, und
warum es zwei sein müssen". Die Belegungsdatei verweist darauf und sagt selbst
nur noch, was für ihre Einträge folgt: ein Buchstabenname benennt die
**Aufschrift**, kein Eintrag wandert mit der Tastaturbelegung, und die
Einschränkung auf `y` und `z` ist gegenstandslos.

Die Zeilen 490–492 verlieren dabei die Nennung als offene Frage; die Entscheidung
ist beantwortet.

**Zusammen mit
`issues/260809-1746_*_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md`
und
`issues/260809-1527_*_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`
erledigen.** Alle drei ziehen dieselbe gegenstandslos gewordene Begründung aus
einem anderen Dokument; einzeln erledigt bleibt sie an den beiden übrigen Orten
stehen. Ausführender ist `ontocoder`, weil `.toml` nicht dem `coder` gehört.

---
Resolved: Am 260810-0914 auf dem hier vorgeschlagenen Weg geschlossen, als dritter und letzter der drei Orte; `260809-1527` und `260809-1746` standen beim Beginn dieser Arbeit schon auf `_c_`. Geändert ist ausschließlich `resources/default-keymap.toml`, und darin ausschließlich die beiden genannten Kommentarblöcke. Keine Belegungszeile ist angefasst, keine Taste umgelegt, keine Datei unter `crates/**` berührt.

Die Behauptung ist vor dem Umschreiben am Code geprüft und trifft nicht mehr zu: `Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt jeden einbuchstabigen Namen aus einem ASCII-Kleinbuchstaben oder einer Ziffer auf `Tastenkennung::Zeichen`, und `Kombination::aus_tastendruck` (dort, Zeilen 569-576) schlägt genau diese über das gemeldete Zeichen nach; die Stellensuche über den Code filtert Tasten mit Zeichenkennung ausdrücklich aus. Über den Code gehen nur noch Funktionstasten, Pfeilblock und Steuertasten.

Der erste Block (jetzt Zeilen 484–499) sagt statt der Meidung von `y` und `z`, was für die Einträge dieser Datei folgt: ein einbuchstabiger Tastenname benennt die **Aufschrift**, kein Eintrag wandert mit der Tastaturbelegung, keiner meidet einen Buchstaben. Die Sachaussage ist nicht ein drittes Mal ausformuliert, sondern verweist auf den Modulkopf von `parser.rs`, Abschnitt "Zwei Nachschlagarten, und warum es zwei sein müssen". Die Nennung als offene Frage ist weg; der Entscheidungsdatensatz steht nun als Nutzerentscheid vom 260808-0155 da und trägt im Dateibestand ohnehin `_i_`. Dass hier bis zum 260810 eine Meidung stand und woran ihr Grund hing, bleibt in einem Satz stehen — dieselbe Gewohnheit, mit der der Dateikopf das Ausscheiden von Cmd+C und Cmd+V vom 260805 festhält, und der Schutz davor, die Regel ein zweites Mal einzuziehen.

Der zweite Block (jetzt Zeilen 625–640) behält sein Ergebnis und verliert seine Herleitung: `cmd+z` und `shift+cmd+z` wirken an der beschrifteten Stelle, weil **beide** Zusteller Buchstaben über das Zeichen nachschlagen — das Menü über `NSMenuItem.keyEquivalent` (`crates/krk-ui/src/appkit/menue.rs`, `zeichen_der_taste`), der Ereignisabgriff seit S2 über das gemeldete Zeichen — und nicht, weil das Menü zustellt. "Auf der Stelle kVK_ANSI_Z" ist zu "auf dem Buchstaben z" geworden, weil ein Buchstabeneintrag keine Stelle mehr benennt. Der Schlusssatz über die Anzeige- und Konfliktseite dieser Kürzel bleibt unverändert, ebenso der ganze Absatz darüber, aus dem das Rückgängig des Editors hängt.

Verification: `cargo test --workspace` → exit 0, 15 Testziele, alle grün (55, 139, 36, 42, 15, 26, 7, 5, 22, 16, 9, 308, 5, 35 bestandene Proben und ein Doc-Test-Ziel ohne Proben, ein `ignored`). Die Belegungsdatei geht über `include_str!` in den Bau; ein Formfehler hätte den Lauf angehalten.

Ein neuer Defekt ist bei dieser Arbeit gefunden und nicht mitbehoben worden, weil er außerhalb der Schreibgrenze lag: der Dateikopf derselben Datei behauptet in Zeile 42 dieselbe Prämisse ("KRK belegt den Tastencode") als allgemeine Regel. Geführt als `issues/260810-0914_o_der-dateikopf-der-belegung-behauptet-den-tastencode-als-allgemeinen-nachschlagweg.md`, Schwere Low, weil die Schlussfolgerung des Absatzes über die fn-Taste weiter trägt.
