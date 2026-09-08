# Die Antwortzeile zu cmd+f begründet das Paar mit dem Wirkungsbereich, die Regel vom 260805 nennt den Zusteller

---

Die `Answered:`-Zeile von
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
schreibt: „cmd+f ist dabei nicht frei, dort liegt der Mac-Standard zum Suchen im
Text; es traegt aber nach der Regel vom 260805 fuer cmd+a, weil zwei Zusteller in
verschiedenen Wirkungsbereichen kein Konflikt sind."

Die Regel vom 260805 sagt das Gegenteil des Halbsatzes. Sie lautet: „Zwei
Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen und
denselben **Zusteller** haben", und sie trägt einen eigenen Abschnitt
„`Wirkungsbereich` ist kein zweiter Zusteller" mit dem Satz: „Zwei vom Abgriff
zugestellte Funktionen mit verschiedenem Wirkungsbereich bleiben ein Konflikt: der
Nachschlag findet nur eine von beiden, und die andere wäre still tot."
(`260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`.)

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Der Entscheid selbst ist davon nicht betroffen: `cmd+f` trägt das Einfügen in den
Filtertext, und es geht — aber nur über den anderen **Zusteller**, also über eine
Funktion mit `gehalten_von = "menue"` neben dem vom Ereignisabgriff zugestellten
`editor_suchen`. Die Begründung im Datensatz ist falsch, das Ergebnis nicht.

Der Preis der falschen Begründung ist nicht rhetorisch. Wer sie liest und
`filter_einfuegen` als gewöhnliche Funktion mit `Wirkungsbereich::Dateifenster`
und **ohne** `gehalten_von` einträgt, legt zwei vom Abgriff zugestellte Funktionen
auf `cmd+f`. Die eingebettete Auslieferungsbelegung bricht bei einem Konflikt beim
ersten Zugriff ab und nimmt jeden Test und jeden Programmstart mit; so ist der
Datensatz vom 260805 überhaupt entstanden. `Belegung::nachschlag`
(`crates/krk-core/src/tasten/belegung.rs:1486-1497`) liefert daneben nur den
ersten Treffer, und die zweite Funktion wäre still tot.

## Abnahme

Die `Answered:`-Zeile nennt den Zusteller als Grund und nicht den Wirkungsbereich,
oder ein Zusatz darunter berichtigt sie. Der Marker bleibt, wo er steht.

Also seen: 260907-2046 by ontocoder — die Zeile `filter_einfuegen` steht seit dem 260907-2046 mit `gehalten_von = "menue"` in `resources/default-keymap.toml`, also nach dem Zusteller und nicht nach dem Wirkungsbereich gebaut; die Frage, woran die Trennung des Anschlags dann hängt, steht als `260907-2046_*_traegt-das-cmd-f-paar-auf-demselben-grund-wie-cmd-a-oder-auf-einem-dritten.md`.

---
Resolved: Die zweite der zwei Abnahmeformen — ein Zusatz unter der `Answered:`-Zeile
berichtigt sie, statt sie umzuschreiben. Die Zeile hält fest, was am 260907-2009 gerulet
wurde, und das bleibt unangetastet; der Zusatz nennt den Wortlaut der Regel vom 260805, den
Abschnitt „`Wirkungsbereich` ist kein zweiter Zusteller", das unberührte Ergebnis und den
Preis der falschen Lesart. Der Marker des Entscheids bleibt bei `_a_`, wie dieser Datensatz
es vorsieht.
