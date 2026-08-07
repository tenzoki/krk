Vier Anlässe prüfen die Lesezeichengültigkeit auf drei verschiedenen Wegen

---

Seit dem vierten Anlass (D5, Turn 25 der Sitzung 260806-2257) prüfen vier
Stellen die Gültigkeitsmarke eines Lesezeichens, und sie tun es auf drei
verschiedenen Wegen. Ein Weg gehört zu jedem Anlass, der ihn zuerst brauchte,
und keiner ist falsch; zusammen sind sie eine Vervielfachung, die beim nächsten
Eingriff auseinanderläuft.

---

**Die drei Wege**

1. Über `Leistenquelle::gueltigkeit_nachziehen` (`crates/krk-ui/src/appkit/leiste.rs:222`).
   Das ist der gemeinte Weg: er ruft `Leistenmodell::gueltigkeit_pruefen`, liest
   dessen Rückgabewert und zeichnet nur neu, wenn sich etwas geändert hat. Ihn
   gehen der Datenträgerwechsel, die Auswahlmeldung und seit D5 der Abschluss
   einer Dateioperation.
2. Über einen direkten Aufruf von `gueltigkeit_pruefen` mit weggeworfenem
   Rückgabewert: `Leistenquelle::orte_setzen` (`crates/krk-ui/src/appkit/leiste.rs:213`).
   Heute richtig, weil das anschließende `nachziehen()` die Tabelle ohnehin
   vollständig neu zeichnet und der Rückgabewert deshalb nichts entscheidet.
3. Gar nicht über `gueltigkeit_pruefen`: `Leistenmodell::lesezeichen_setzen`
   prüft die Gültigkeit implizit beim Aufbau der `Gemerkt`-Einträge.

**Warum es ein Befund ist und kein Fehlverhalten.** Alle drei Wege liefern heute
dasselbe Ergebnis, und keiner der vier Anlässe zeigt eine falsche Marke. Der
Schaden ist einer für später: wer die Prüfung ändert, etwa um einen fünften
Anlass aufzunehmen oder die Regel zu erweitern, muss drei Stellen finden und
wird die dritte übersehen, weil sie den Namen der Funktion nicht nennt. Das ist
dieselbe Form, die der Plan an anderer Stelle als zweite Wahrheit ausschließt.

**Denkbarer Weg.** `lesezeichen_setzen` baut seine `Gemerkt`-Einträge mit einer
gleichgültigen Vorbelegung auf und ruft danach `gueltigkeit_pruefen`; damit
gibt es genau eine Stelle, die weiß, was gültig heißt. Ob `orte_setzen`
zusätzlich auf `gueltigkeit_nachziehen` umgestellt wird, ist eine Frage des
Geschmacks und nicht der Richtigkeit; der Nutzen läge darin, dass alle vier
Anlässe gleich aussehen.

**Dringlichkeit.** Gering. Kein Nutzer sieht etwas davon, keine Zusage aus C5
ist berührt, und keine der zehn Zeitzusagen aus C8 ist betroffen. Der Wert liegt
darin, dass die nächste Änderung an der Prüfung eine Stelle hat statt drei.

**Betrifft:** `crates/krk-ui/src/appkit/leiste.rs`,
`crates/krk-ui/src/leistenmodell.rs`.

**Aufgefallen bei:** der Umsetzung von D5, Turn 25 der Sitzung 260806-2257,
`history/260807-0010-coder-vierter-anlass-lesezeichengueltigkeit.md`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-1730_c_die-gueltigkeit-eines-lesezeichens-veraltet-zwischen-zwei-anlaessen.md`

---
Resolved: Aus drei Wegen ist einer geworden, und er heisst `Gemerkt::nachpruefen`
(`crates/krk-ui/src/leistenmodell.rs:161`). Es ist jetzt die einzige Zeile der
Kiste, die `Lesezeichen::gueltig` ruft, und die einzige, die das Feld `gueltig`
schreibt.

**Der denkbare Weg aus dem Bericht, geprueft und abgewandelt.** Vorgeschlagen
war, `lesezeichen_setzen` mit einer gleichgueltigen Vorbelegung aufbauen und
danach `gueltigkeit_pruefen` rufen zu lassen. Das ergibt das richtige Ergebnis,
setzt aber eine Falle: `gueltigkeit_pruefen` liefert "hat sich etwas geaendert",
und nach einer Vorbelegung heisst diese Antwort "gemessen gegen die
Vorbelegung", also nichts. Heute wird sie dort verworfen; wer sie morgen liest,
liest Unsinn. Die Vorbelegung steht deshalb nicht in `lesezeichen_setzen`,
sondern in `Gemerkt::neu` (`:145`), eine Zeile ueber dem Aufruf, der sie
ueberschreibt. Ein `Gemerkt` mit einer Marke, die seinen Ordner nicht kennt,
verlaesst den `impl`-Block nicht, und `Leistenmodell::gueltigkeit_pruefen`
behaelt eine Antwort, die etwas bedeutet.

**Der vierte Weg, den der Bericht als Geschmacksfrage fuehrte, ist auch fort.**
`Leistenquelle::orte_setzen` (`crates/krk-ui/src/appkit/leiste.rs:216`) rief
`gueltigkeit_pruefen` selbst und warf den Rueckgabewert weg. Die Pruefung steht
jetzt in `Leistenmodell::orte_setzen` (`crates/krk-ui/src/leistenmodell.rs:211`):
die Ortsliste aendert sich genau dann, wenn ein Datentraeger gekommen oder
gegangen ist, und damit aendert sich, was ein Lesezeichen darauf wert ist. Sie
beim Aufrufer zu lassen hiesse, jedem kuenftigen Aufrufer eine Pflicht
mitzugeben, die er vergessen kann — dieselbe Form, die der Bericht anmahnt. Die
AppKit-Seite ist damit auf zwei Zeilen geschrumpft und braucht die
`RefCell`-Klammer nicht mehr.

Kein Verhalten geaendert: alle vier Anlaesse liefern dasselbe Ergebnis wie
vorher, an denselben Zeitpunkten.

**Zwei neue Proben ohne Fenster.**
`der_aufbau_und_das_nachziehen_kommen_zum_selben_ergebnis` (`:785`) haengt allein
am Rueckgabewert: findet `gueltigkeit_pruefen` unmittelbar nach
`lesezeichen_setzen` etwas zu aendern, dann haben die beiden Anlaesse wieder
verschiedene Vorstellungen davon, was gueltig heisst.
`eine_neue_ortsliste_zieht_die_gueltigkeit_nach` (`:809`) haelt den ersten Anlass
im Modell fest, wo er bisher nur ueber AppKit erreichbar war. `make check`
gruen, 525 Pruefungen.
