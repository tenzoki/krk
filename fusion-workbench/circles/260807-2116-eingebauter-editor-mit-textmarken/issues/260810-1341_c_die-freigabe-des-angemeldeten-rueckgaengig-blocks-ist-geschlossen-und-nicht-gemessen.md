Die Freigabe des angemeldeten Rueckgaengig-Blocks ist geschlossen und nicht gemessen
---
Das Stapelbudget aus `260810-1314` zaehlt die gehaltenen Bytes ueber `Stapellast`, und `Stapellast` traegt in ihrem `Drop` ab. Dass der `NSUndoManager` den angemeldeten Block wieder freigibt — nach dem Ausfuehren, beim Raeumen des Wiederherstellungsstapels, bei `removeAllActions`, beim Fortfallen des Verwalters —, ist die Regel von Objective-C fuer einen aufbewahrten Block und in diesem Baum durch nichts gemessen.
---
**Schwere:** Niedrig
**Gefunden:** bei der Behebung von `260810-1314`, als benannte Annahme des dort gewaehlten Umbaus
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`
**Zusammenhang:** `issues/260810-1314_*_ein-wiederholtes-sammelersetzen-legt-je-ruf-einen-bereich-in-dateigroesse-in-den-stapel.md`, `decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`

## Belegstellen

`Stapellast` benennt die Annahme im Doc-Kommentar unter „Die Freigabe des Blocks
ist geschlossen und nicht gemessen" und sagt dort auch, was eine falsche Annahme
kostet.

`der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung` messt die Regel
des Budgets und die Bytes, die sie haelt — aber gegen ein `Vec<Stapellast>`, das
die Probe selbst fuehrt, nicht gegen einen `NSUndoManager`.

## Warum die Messung nicht mitgekommen ist

Ein `NSUndoManager` verlangt einen `MainThreadMarker`, und der Pruefstand von Rust
gibt keinen: die vier Proben, die ihn heute brauchen, behaupten ihn mit
`MainThreadMarker::new_unchecked`. Darueber steht eine offene Nutzerentscheidung
(`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`).
Eine fuenfte Probe derselben Bauart daneben zu stellen haette die Frage
vergroessert, statt sie zu beantworten.

## Fehlszenario

Gaebe der Verwalter den Block nicht frei, ginge der Zaehler nur hoch und nie
herunter. Das Budget griffe dann bei **jedem** Sammelersetzen, und der Stapel
hielte statt „Budget plus eine Handlung" genau eine Handlung: ein `cmd+z` nimmt
das letzte Ersetzen zurueck, ein zweites tut nichts, und zwar auch an einer
kleinen Datei, an der beide nebeneinander gepasst haetten.

**Die Schranke aus `260810-1314` faellt damit nicht aus**, sie wird nur strenger
als noetig. Der Fall ist deshalb keine Speichergefahr, sondern eine unbemerkte
Verkuerzung des Verlaufs, und er ist am laufenden Buendel sichtbar: zwei
Sammelersetzen an einer kleinen Datei, dann zweimal `cmd+z`. Nimmt das zweite
nichts zurueck, greift das Budget zu frueh.

## Was zu pruefen waere

Zwei Wege, und der erste haengt an der offenen Frage:

1. **Eine Probe am `NSUndoManager`**, in demselben Pruefziel, in das die vier
   Instanzproben umziehen, sobald die Frage `260810-1044` beantwortet ist: eine
   Handlung anmelden, den Zaehler lesen, `removeAllActions`, den Zaehler wieder
   lesen. Dasselbe fuer `undo`.
2. **Eine Probe allein am Block**, ohne AppKit: dass ein `RcBlock`, der eine
   `Stapellast` uebernimmt, sie beim Fortfallen der letzten Referenz fallen
   laesst. Sie misst die Rust-Haelfte der Kette und laesst die
   Objective-C-Haelfte offen; ob das den Aufwand lohnt, ist die eigentliche
   Frage an diesem Weg.

---
Resolved: als Lage angenommen, nicht behoben — der Orchestrator am 260810-1520.

Die Behebung hängt an `decisions/260810-1044_d_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`,
und die ist zurückgestellt: die Frage betrifft Prüfgerüst und nicht KRK, und die
einzige nebenwirkungsfreie Antwort wäre ein Umbau der ganzen Kiste `krk-ui`.

Der Befund selbst bleibt richtig und ist am Code sichtbar: der Doc-Kommentar von
`an_einer_flaeche` benennt die Behauptung. Bricht sie auf einem Gerät, fällt der
Prüflauf aus und nicht die Anwendung — das ist der Preis, der hier bewusst
gezahlt wird. Wer `krk-ui` aus einem anderen Grund umbaut, nimmt beides mit.
