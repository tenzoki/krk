# Zwei Kommentarblöcke der Belegungsdatei behaupten den Nachschlag über den Tastencode

---
**Domain:** data
**Schwere:** Medium
**Gefunden von:** coder, bei der Abnahme von S41
**Betroffen:** `resources/default-keymap.toml`, Zeilen 484–492 und 617–628
**Cross-references:** `crates/krk-core/src/tasten/parser.rs` (Modulkopf, `Tastenkennung`), `crates/krk-ui/src/appkit/ereignisse.rs:134-142` (dort schon gezogen), `issues/260809-1746_o_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md` (derselbe Anlass, anderer Ort), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

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
`issues/260809-1746_o_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md`
und
`issues/260809-1527_o_der-plan-verbietet-y-und-z-und-legt-rueckgaengig-selbst-auf-cmd-z.md`
erledigen.** Alle drei ziehen dieselbe gegenstandslos gewordene Begründung aus
einem anderen Dokument; einzeln erledigt bleibt sie an den beiden übrigen Orten
stehen. Ausführender ist `ontocoder`, weil `.toml` nicht dem `coder` gehört.
