Das Tastenprotokoll ist über `open target/KRK.app --args --tasten-protokoll` nicht lesbar

---

Das Abnahmekriterium von S7 verlangt, dass `open target/KRK.app --args --tasten-protokoll`
beim Drücken von F3, F5 und F8 die Codes 99, 96 und 100 protokolliert. Der
Protokollmodus schreibt sie, wie der Plan es vorschreibt, auf die Standardausgabe.
Eine über `open` gestartete Anwendung hat aber keine: LaunchServices hängt
Standardausgabe und Standardfehler eines so gestarteten Prozesses ins Leere. Der
Nutzer sieht damit nichts, obwohl das Programm richtig arbeitet.

---

**Drei Wege, und der erste kostet nichts.**

1. **Das Bündel unmittelbar starten statt über `open`:**
   `target/KRK.app/Contents/MacOS/krk --tasten-protokoll`. Das Terminal bleibt die
   Standardausgabe, die Zeilen erscheinen dort, und die Anwendung ist dieselbe. Der
   Preis: der Prozess erbt die TCC-Freigaben des Terminals statt eigene zu erfragen.
   Für diese Messung ist das folgenlos, weil sie keinen geschützten Ordner anfasst,
   sondern Tastencodes zählt. Das Kriterium wäre entsprechend umzuschreiben.
2. Das Protokoll zusätzlich über `os_log` schreiben und mit `log stream --process krk`
   mitlesen. Das trägt auch bei einem Start über den Finder, kostet aber eine weitere
   Abhängigkeit und einen zweiten Ausgabeweg neben der Standardausgabe.
3. Das Protokoll in eine Datei unter `messungen/` schreiben. Damit wäre es
   versioniert wie die Messberichte, aber der Modus wäre kein Protokoll mehr, sondern
   eine Messstrecke, und S8 hat dafür bereits eine.

**Empfehlung:** Weg 1. Er ändert eine Zeile im Plan und keine im Code, und die
Abnahme wird dadurch nicht schwächer: gemessen wird, welchen Tastencode AppKit
liefert, und daran ändert der Startweg nichts.

**Stand des Codes.** `--tasten-protokoll` ist umgesetzt und arbeitet. Belegt am
260803-1309 über einen Lauf aus dem Terminal mit synthetischen Tastenereignissen:
jede empfangene Taste erzeugt eine Zeile der Form
`tastencode=125 maske=keine kommando=AuswahlRunter`. Der Modulkommentar von
`crates/krk-ui/src/appkit/ereignisse.rs` hält den Punkt an der Stelle fest, an der
das Protokoll geschrieben wird.

---
Resolved: Das Kriterium von S7 startet das Bündel jetzt unmittelbar über `target/KRK.app/Contents/MacOS/krk --tasten-protokoll`, also Weg 1 des Datensatzes. Der Code bleibt unberührt. Der Absatz zum Kriterium hält den Grund fest, damit der Startweg nicht bei der nächsten Durchsicht auf `open` zurückgezogen wird: gemessen wird, welchen Tastencode AppKit liefert, und daran ändert der Startweg nichts; der einzige Unterschied ist, dass der Prozess die TCC-Freigaben des Terminals erbt, was für eine Messung ohne geschützten Ordner folgenlos bleibt. Die Prüfung von S6, die ebenfalls `open target/KRK.app` verwendet, ist nicht betroffen: sie liest keine Standardausgabe, sondern sieht ein Fenster an. Nachgezogen am 260803-2007.
