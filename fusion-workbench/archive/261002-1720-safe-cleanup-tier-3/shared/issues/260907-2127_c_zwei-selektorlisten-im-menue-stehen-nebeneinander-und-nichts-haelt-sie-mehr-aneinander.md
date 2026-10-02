# Zwei Selektorlisten des Menüs stehen nebeneinander, und seit dem 260907 hält sie nichts mehr aneinander

---

`menuemodell::ZUSTELLER` (`crates/krk-ui/src/menuemodell.rs`) führt jede vom
Menü zugestellte Funktion mit ihrem Selektor; `appkit::menue::die_sechs_zugestellten`
(`crates/krk-ui/src/appkit/menue.rs`, im Prüfmodul) führt die Selektoren, deren
Ersthelferklasse die Messung `die_sechs_zugestellten_textbefehle_werden_von_diesen_klassen_beantwortet`
abfragt. Bis zum 260907 trugen beide dieselben sechs Einträge, und was sie
aneinanderhielt, war Prosa: der Doc-Kommentar von `die_sechs_zugestellten`
sagte „dieselben sechs, die `resources/default-keymap.toml` mit
`gehalten_von = "menue"` führt". Mit `filter_einfuegen` sind die Listen
absichtlich verschieden geworden — sieben gegen sechs —, und damit ist auch die
Prosa weg, die einen künftigen Auseinanderlauf als Auseinanderlauf erkennbar
machte.

Wer als Nächster einen **AppKit**-Selektor zu `ZUSTELLER` hinzufügt, bekommt
keinen Fehlschlag: `die_sechs_zugestellten` wächst nicht mit, die Messung sieht
den neuen Selektor nie, und die dritte Spalte der Markdown-Ausgabe aus C3
bekommt für ihn eine Auskunft, die niemand gemessen hat. Der Übersetzer hält
die Beziehung nicht — die Listen stehen in zwei Modulen, tragen verschiedene
Typen (`&CStr` gegen `Sel`) und werden nirgends gegeneinander gelesen.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

**Cross-references:**
`260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
(der Entscheid, der die Divergenz ausgelöst hat),
`260907-2020_*_das-einfuegen-in-den-filtertext-braucht-eine-siebte-vom-menue-zugestellte-funktion-in-der-belegung.md`
(der Befund zur Belegungszeile),
`260907-2127-k15b-der-code-zur-belegungszeile-cmd-f.md` (der Durchgang, der die
Divergenz gebaut hat)

## Was heute an ihrer Stelle steht

Prosa an drei Stellen, die die Divergenz **beschreibt** und keine, die sie
prüft: der Doc-Kommentar von `ZUSTELLER`, der von `die_sechs_zugestellten` und
der Abschnitt „Wer die sechs Textbefehle beantwortet" im Modulkopf von
`appkit/menue.rs`. Alle drei sagen dasselbe, nämlich dass die kürzere Liste
AppKit-Selektoren zählt und nicht zugestellte Funktionen. Keine davon wird rot.

Die Ungleichheit selbst ist richtig und soll bleiben: `filterEinfuegen:` ist
ein Name, den KRK vergibt, keine AppKit-Klasse kennt ihn, und eine Zeile in
`GEMESSEN` sähe wie eine Messung aus, die nichts gemessen hat.

## Warum die Prüfung nicht offensichtlich ist

Die Frage, die eine Probe stellen müsste, lautet „ist dieser Selektor einer von
AppKit". Aus dem Namen ist sie nicht entscheidbar. Entscheidbar ist sie über
das Laufzeitsystem: ein Selektor, den keine der sechs Klassen aus
`ersthelferklassen` beantwortet und den der Anwendungsdelegierte beantwortet,
ist einer von KRK. Genau das misst `wer_antwortet` schon, und ein leeres
Ergebnis ist dort heute kein Fehlschlag, sondern nur ein leerer Eintrag.

## Akzeptanzprobe

Eine Probe liest `menuemodell::ZUSTELLER` und `die_sechs_zugestellten`
gegeneinander und wird rot, sobald ein Selektor in der ersten steht, den keine
Klasse aus `ersthelferklassen` beantwortet und der trotzdem nicht namentlich
als KRK-eigener geführt ist — oder umgekehrt. Ein Eintrag, der zu
`ZUSTELLER` hinzukommt, ohne dass jemand über seine Sorte entschieden hat, hält
damit den Lauf an.

---
Resolved: Die Abnahmeprobe steht, in der Form, die der Datensatz beschreibt. `jeder_zugestellte_selektor_ist_gemessen_oder_eigen` (`crates/krk-ui/src/appkit/menue.rs`, Pruefmodul) liest `menuemodell::ZUSTELLER` **aus dem Quelltext** — die Konstante ist privat, und ein `pub(crate)` daran oeffnete eine Modulgrenze fuer eine Probe — und stellt je Selektor die eine entscheidbare Frage an das Laufzeitsystem: beantwortet ihn eine der sechs `ersthelferklassen`? Ist die Antwort ja, muss er in `die_sechs_zugestellten` stehen; ist sie nein, in der neuen Liste `KRK_EIGENE`, die heute genau `filterEinfuegen:` fuehrt. Zwei Gegenproben halten die andere Richtung: `jeder_gemessene_selektor_wird_zugestellt` faengt eine gemessene Zeile ohne Menueeintrag, `jeder_eigene_selektor_hat_seinen_fall` eine Ausnahme ohne Fall. Der Absatz im Modulkopf, der die Divergenz beschrieb, nennt jetzt die Probe.
