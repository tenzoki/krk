# Wird die Untergrenzen-Angabe im Modulkopf prüfbar gemacht?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (nach der Einschätzung des coder beim Nachtragen der 26 Modulköpfe)
**Cross-references:** `shared/issues/260811-1648_*_die-untergrenzen-angabe-im-modulkopf-steht-in-sieben-von-32-appkit-modulen.md`,
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260811-1230_*_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`

---

## Frage

`CLAUDE.md` führt als Gewohnheit: jedes AppKit-Modul nennt im Modulkopf die Untergrenze jeder
Klasse, die es anspricht. Der Grund ist ernst — **`objc2` führt keine Verfügbarkeitsangaben mit
sich**, der Übersetzer hält die Untergrenze macOS 15 also nicht, und wer eine später
hinzugekommene Methode anspricht, bekommt keine Warnung, sondern einen Absturz auf dem
Referenzgerät.

**Die Gewohnheit war auf 5 von 31 Modulen abgesunken**, ehe sie am 260811 von Hand
wiederhergestellt wurde. Sie hält sich also nicht von selbst. Die Frage ist, ob und wie weit sie
prüfbar gemacht wird.

## Optionen

Der `coder` hat drei Stufen mit ihren Kosten vorgelegt. Sie bauen aufeinander auf.

1. **Abschnitt vorhanden.** Ein Ziel neben `make check`: jede Datei unter `appkit/` mit einem
   `use objc2_`-Block muss die Überschrift tragen.
   - Pro: fängt genau den Fall, der die Gewohnheit hat absinken lassen — das **neue** Modul.
     Rund ein Dutzend Zeilen, kein SDK nötig.
   - Contra: sagt nichts darüber, ob der Abschnitt stimmt oder vollständig ist.

2. **Jede importierte Klasse im Abschnitt genannt.** Mengenvergleich zwischen den
   `use objc2_*`-Blöcken und den Backtick-Namen im Abschnitt.
   - Pro: fängt die **vergessene** Klasse, den zweithäufigsten Fehler. Rund 80 Zeilen, kein SDK.
     Der `coder` hat den Vergleich zur Durchsicht einmal von Hand gefahren.
   - Contra: meldet Aufzählungstypen als fehlend, die die Prosa unter ihren Konstanten führt. Die
     Regel braucht eine Ausnahmeliste oder eine festere Schreibweise — und eine Ausnahmeliste ist
     genau die Sorte Aufzählung, die dieses Projekt an anderen Stellen abgeschafft hat.

3. **Die Zahlen prüfen.** Ein Parser über die SDK-Köpfe.
   - Contra: braucht die Erbfolge (`setTarget:` steht an `NSControl`, angesprochen wird es an
     `NSTableView`), Kategorien mit eigener Angabe und die Rückabbildung der `objc2`-Namen auf
     Selektoren. Das ist ein halber Clang, und es bindet den Prüflauf an ein installiertes Xcode.
   - **Der `coder` rät ausdrücklich ab**, und dieselbe Abwägung ist in
     `issues/260810-0417` schon einmal abgelehnt worden.

## Constraints

Zwei Fallen kosten je einen Fehlbefund und gehören in jede Umsetzung, die den SDK-Kopf liest: die
`API_AVAILABLE`-Zeile einer **Klasse** steht über dem `@interface`, die einer **Aufzählung** an
der schließenden Klammer — und weil manche Aufzählung mit bloßem `};` schließt, liefert ein
naives `grep` die Zahl der *nächsten* Aufzählung. Genau so wären `NSAutoresizingMaskOptions` und
`NSWindowStyleMask` als 10.6 beziehungsweise 10.5 durchgegangen; beide tragen nichts.

## Empfehlung

**Stufe 1 und 2, Stufe 3 nicht.** Zusammen decken sie den häufigen Fehler ab — das fehlende
Modul und die vergessene Klasse — und beide kommen ohne SDK aus.

**Was auch dann eine Zusage des Menschen bleibt, ist die Richtigkeit der Zahl.** Ein Ziel, das
Stufe 1 und 2 prüft, darf nicht so heißen, als prüfe es mehr. Der `coder` sagt es schärfer, als
ich es formulieren würde: „das sollte ein solches Ziel nicht anders behaupten."

**Der Zusammenhang mit der offenen Frage `260811-1230` gehört mitentschieden.** Dort geht es um
Zahlen in Kommentaren, die keine Prüfung hält; hier um dasselbe für die Untergrenzen. Stufe 1 und
2 sind der Beleg, dass es für diese eine Sorte Kommentar doch geht — für die andere (die Zahl der
Ränge, der Module, der Funktionen) gilt das Argument nicht, weil dort kein Import danebensteht,
an dem sich vergleichen ließe.
