Text aus `.secrets.txt`, auf den Finder gezogen, wird eine Klartextdatei, und die Entscheidung zur Zwischenablage sagt dazu nichts

---

Die Rohansicht des Editors ist eine `NSTextView`, und eine solche gibt markierten Text von Haus aus als Ziehquelle her. Zieht der Nutzer eine Markierung aus der Rohansicht von `.secrets.txt` auf den Schreibtisch oder in ein Finder-Fenster, legt der Finder eine Textausschnittsdatei (`.textClipping`) mit dem Klartext an; Spotlight indiziert sie wie jede andere Datei. Derselben Art ist der Eintrag „Dienste" im Kontextmenü markierten Texts, der den Text an eine fremde Anwendung reicht, die ihn ablegen kann. Im Baum steht nichts, was das Ziehen aus der Editorfläche abstellt (kein Überschreiben von `dragSelectionWithEvent:offset:slideBack:`, keine Regel in `appkit/textautomatik.rs`).

Beide Wege sind eine bewusste Handlung des Nutzers, wie das Kopieren. Die Entscheidung `260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md` erlaubt aber ausdrücklich die Zwischenablage, deren Inhalt im Speicher liegt, und C7.8 des Spec sagt „an keine Stelle auf der Platte"; ein Ziehen auf den Finder legt den Text auf die Platte, und das deckt keiner der beiden Sätze.

---
**Domain:** code
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Am Code gelesen und aus dem Verhalten von AppKit gefolgert, nicht am Bündel ausgelöst.** Ob die Tabelle der Formatansicht (Zellen mit Feldeditor) dasselbe Ziehen hergibt, ist nicht geprüft. Gefunden bei der Durchsicht aller Wege, auf denen Inhalt von `.secrets.txt` aus KRK hinausgelangt (Behebung von `260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md`).

**Warum nicht mitbehoben.** Das ist eine Frage an den Nutzer und kein Defekt, den eine Zeile schließt: ist Ziehen ein Kopieren (dann gilt `260926-0033_*_…`, und `HowTo.md` nennt den Ausschnitt neben der Zwischenablage), oder ist es ein Weg auf die Platte, den C7.8 verbietet (dann braucht die Editorfläche für `.secrets.txt` eine gesperrte Ziehquelle, also ein Überschreiben in der Unterklasse der Textfläche, und die Dienste einen Riegel)? Nutzerprüfung: `.secrets.txt` öffnen, in der Rohansicht eine Zeile markieren und auf den Schreibtisch ziehen; entsteht eine Datei, `cat` darauf.

---
Resolved: 260926-1130 — als Frage an den Nutzer beantwortet, nicht im Code behoben: `260926-1119_*_bekommt-secrets-txt-unter-einer-dritten-schreibweise-eine-zusatzpruefung-und-gilt-ziehen-als-kopieren.md`, Teil (b), Möglichkeit 3. Ziehen in den Finder gilt wie Kopieren und bleibt erlaubt; es ist eine bewusste Ausfuhr durch den Nutzer und fällt damit unter `260926-0033_*_darf-text-aus-secrets-txt-in-die-zwischenablage.md`, C7.8 ist eine Zusage über KRKs eigene Schreibwege. `HowTo.md` nennt die entstehende Klartextdatei unter „Geheimnisse in `.secrets.txt`“ im Absatz „Gezogener Text wird eine Klartextdatei“, gleich nach dem Absatz zur Zwischenablage. Der Eintrag „Dienste“ ist in der Antwort nicht genannt und in der Anleitung deshalb nicht beschrieben.
