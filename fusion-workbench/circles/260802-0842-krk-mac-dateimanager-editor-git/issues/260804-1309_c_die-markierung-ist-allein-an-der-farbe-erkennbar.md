Die Markierung aus C2 ist allein an der Farbe erkennbar

---

Schritt 13 macht die Mehrfachauswahl aus C2 sichtbar, indem er die Zellen eines markierten Eintrags orange einfärbt (`crates/krk-ui/src/appkit/tabelle.rs`, `zellenansicht`). Das ist das einzige Kennzeichen: Schrift, Hintergrund, Zeilenhöhe und Text bleiben gleich.

Ein Nutzer mit einer Rot-Grün-Schwäche unterscheidet Orange und die Beschriftungsfarbe auf dem dunklen Hintergrund schlecht; bei einer Blau-Gelb-Schwäche verschwindet der Unterschied fast ganz. Für ihn zeigt eine markierte Liste dasselbe Bild wie eine unmarkierte, und die vier Markierungsbefehle wirken folgenlos.

---

## Warum es zählt

Die Markierung ist kein Schmuck: die Dateioperationen aus C4 wirken auf sie. Wer nicht sieht, was markiert ist, kopiert oder löscht im Blindflug. Die Norton-Reihe, das Vorbild der Anwendung, färbt markierte Einträge ebenfalls, setzt aber zusätzlich die Zahl der markierten Einträge und ihre Gesamtgröße in eine Zeile am Fuß.

## Was zu tun ist

Nicht in diesem Schritt: die Statuszeile aus C1 trägt heute allein Fehlermeldungen, und was sie sonst noch zeigt, ist eine Festlegung und keine Nebenwirkung. Der Modulkopf von `crates/krk-ui/src/appkit/statuszeile.rs` hält ausdrücklich fest, dass Lesefortschritt und Eintragszahl in einer späteren Runde **in dieselbe Zeile** kommen und nicht in eine zweite daneben; die Zahl der markierten Einträge gehört in dieselbe Frage.

Vorgeschlagen: ein zweites Kennzeichen neben der Farbe. Zwei Kandidaten, beide klein:

- Die Zahl der markierten Einträge in der Statuszeile des Dateifensters, zusammen mit der Frage, was diese Zeile sonst noch trägt.
- Ein Zeichen in der Namensspalte vor dem Namen, so wie es Dateimanager mit Textoberfläche halten.

Welches, entscheidet der Nutzer; beide berühren das Aussehen der Liste, und keines folgt aus dem Spec.

---

Herkunft: gefunden bei der Umsetzung von Schritt 13 am 260804-1309, beim Nachweis des Abnahmekriteriums 7 aus C2 im laufenden Bündel.

---
Resolved: Nutzerentscheidung vom 260805-0000, **beide** vorgeschlagenen Kandidaten. Die Markierung bekommt ein zweites Kennzeichen neben der Farbe, und die Statuszeile trägt künftig Zahl und Gesamtgröße der markierten Einträge. Begründung des Nutzers: Farbe allein ist für Farbfehlsichtige kein Kennzeichen, und die Zahl ist in jedem Dateimanager der Norton-Linie Standard.

Zwei Punkte hat der Nutzer dem Planner überlassen, beide entschieden und im Plan bei S16c begründet.

**Die Form des zweiten Kennzeichens ist die fette Schrift.** Ein markierter Eintrag steht in allen vier Spalten fett und bleibt orange. Sie ist eine Form und keine Farbe, wirkt also bei jeder Farbfehlsichtigkeit; sie sprengt die vier Spalten aus C1 nicht, weil sie keine Fläche braucht; und sie geht denselben Weg wie die Farbe, die `zellenansicht` in `crates/krk-ui/src/appkit/tabelle.rs` ohnehin in jedem Durchgang setzt. Es kommt eine Eigenschaft dazu und kein Mechanismus. Verworfen: eine fünfte Spalte mit einem Markierungszeichen, weil sie die vier Spalten sprengt, und ein Zeichen vor dem Namen, weil es den angezeigten Namen vom wirklichen unterscheidbar machte und die Namen in einer Proportionalschrift gegeneinander verschöbe.

**Der Markierungsstand braucht einen fünften Rang und passt nicht in den vierten.** Rang 4, die Tabmeldung, trägt einen Ordner, der sich nicht lesen ließ, und muss stehen bleiben, während der Nutzer markiert und die Markierung wieder aufhebt. Beides in ein Feld zu legen gäbe diesem Feld zwei Löschregeln, und genau diesen Sonderfall hat S16b für die Trennung von Befehlsantwort und Fenstermeldung schon einmal ausgeschlossen. Der neue Rang steht **unter** der Tabmeldung, weil ein nicht lesbarer Ordner ein Fehler ist und eine Markierungszahl keiner; er ist der Ruhezustand der Zeile. Der Preis ist gering, weil er als einzige Quelle kein Feld braucht: die vier vorhandenen halten je einen Text, den jemand setzt und eine Regel löscht, während der Markierungsstand bei jedem Schreiben der Zeile aus dem Ordnermodell des sichtbaren Tabs errechnet wird. Ein Feld hätte vier Schreiber und vier Gelegenheiten, veraltet zu sein.

Eingearbeitet in C1 und C2 des Specs; im Plan als neuer Schritt **S16c**. Entscheidungsdatensatz `decisions/260805-0000_a_zweites-kennzeichen-der-markierung-und-ihr-platz-in-der-statuszeile.md`. Sitzungsbericht `history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`.
