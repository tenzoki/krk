krk-bench trägt ein einziges `#[must_use]`, und der Sitzungswächter ist nicht das eine

---

`grep -rn '#\[must_use' crates/*/src` zählt am 260826: `krk-core` 66, `krk-ui` 95,
**`krk-bench` 1**. Das eine steht auf `Messplanwaechter` (`crates/krk-bench/src/messen.rs:1533`).
Der Wächter daneben, der aus derselben Überlegung entstanden ist und dessen stilles Fallenlassen
teurer wäre, trägt keines: `Sitzungswaechter` (`messen.rs:1316`).

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `crates/krk-bench/src/messen.rs`, `crates/krk-bench/src/wegwerfordner.rs`
**Cross-references:** `CLAUDE.md`, Abschnitt „Was man nicht sieht" (die `#[must_use]`-Regel, entschieden 260811-2140); `shared/issues/260826-1302_*_eine-vierte-pruefordner-fassung-steht-in-xtask-und-die-zaehlprobe-c4-6-kann-sie-nicht-sehen.md` (dieselbe Dreiergruppe, andere Frage)

## Was ein zu früh fallender Sitzungswächter kostet

`Sitzungssicherung::anlegen` (`messen.rs:1379`) liefert `io::Result<Sitzungswaechter>`. Solange
der Wächter lebt, steht die Prüfsitzung aus C8 in der **echten Ablage des Nutzers**; sein `Drop`
(`messen.rs:1318-1322`, über `sitzung_zurueckspielen`) spielt den vorigen Stand zurück. Der eine
Rufer bindet ihn richtig (`let _sitzung = Sitzungssicherung::anlegen()?;`, `messen.rs:1034`).

`let _ = Sitzungssicherung::anlegen()?;` übersetzt aber ebenso — und lässt den Wächter sofort
fallen. Die Prüfsitzung wird dann zurückgenommen, **bevor** die erste Runde läuft, und die
zwanzig L4-Starts danach messen das Wiederherstellen der Sitzung des Nutzers statt der
Prüfsitzung aus C8. Der Lauf bricht dabei nicht ab: er liefert zwanzig Zahlen, ein Gate-Urteil
und einen Bericht, dessen Kopf `PRUEFSITZUNG` (`bericht.rs:504-508`) als hergestellt ausweist.
Das ist genau der Fall, den die Regel aus `CLAUDE.md` meint: ein Rückgabewert, dessen stilles
Fallenlassen unbemerkt bliebe.

Der Nachbar in derselben Datei zeigt, dass die Regel hier schon einmal angewandt wurde:
`Messplanwaechter` (`messen.rs:1533-1537`, die Zeile `#[must_use]` steht auf 1533) trägt `#[must_use]`, obwohl sein Fallenlassen nur eine
Datei im Temporärverzeichnis kostet. Der teurere der beiden Wächter ist der ungeschützte.

## Der Wegwerfordner ist derselbe Fall, mit einer Einschränkung

`Wegwerfordner` (`wegwerfordner.rs:33-35`) ist ein RAII-Wächter derselben Bauform, und
`Wegwerfordner::neu("zweck");` als nackte Anweisung übersetzt und legt einen Ordner an, den
niemand mehr hält. **Die Einschränkung gehört dazu:** seine beiden Schwesterfassungen,
`krk_ui::pruefordner::Pruefordner` (`crates/krk-ui/src/pruefordner.rs:47-49`) und der
`Pruefordner` in `crates/krk-core/tests/gemeinsam/mod.rs:63-65`, tragen ebenfalls keines. Wer
hier eines setzt, setzt es an allen dreien oder erzeugt eine vierte Abweichung zwischen den drei
Fassungen, die es ausdrücklich nur dreimal geben soll.

## Was ich nicht behaupte

Ich habe **keinen** Rufer gefunden, der einen dieser Werte fallen lässt. Der Befund ist die
fehlende Absicherung, nicht ein eingetretener Fehler. Ob über `Sitzungswaechter` und
`Wegwerfordner` hinaus weitere Rückgabewerte dieser Kiste die Regel brauchen — etwa
`Gesamtergebnis::bestanden` (`messen.rs:1009`) und `Durchstichergebnis::bestanden`
(`messen.rs:734`), an denen das Gate hängt —, ist eine Ermessensfrage; die Zahl 1 gegen 66 und 95
legt nahe, dass die Kiste bei ihrer Einführung schlicht übergangen worden ist.

## Denkbarer Weg

`#[must_use]` auf `Sitzungswaechter` setzen; das ist eine Zeile und hat keinen Rufer, der bricht.
Für `Wegwerfordner` zuerst entscheiden, ob alle drei Prüfordner-Fassungen es bekommen. Für die
beiden `bestanden` eine eigene Betrachtung, weil sie kein Wächter, sondern ein Urteil sind.
