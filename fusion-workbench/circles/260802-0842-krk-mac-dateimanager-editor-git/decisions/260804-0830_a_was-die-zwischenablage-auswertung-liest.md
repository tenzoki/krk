# Liest die Zwischenablage-Auswertung nur Text, oder auch die Dateiverweise des Finders?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C10), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 13 und Schritt 19)

---

## Frage

Der Nutzer hat am 260804 zwei Funktionen beauftragt, die die Zwischenablage
lesen: sie ansehen und zu ihrem Inhalt springen. Beide stehen als Fähigkeit C10
im Spec. Offen ist, **welche Sorten von Inhalt die Auswertung überhaupt zu
Gesicht bekommt**.

Eine Zwischenablage des Mac hält ihren Inhalt in mehreren Sorten gleichzeitig,
die `NSPasteboard` als Typen führt. Zwei sind für C10 einschlägig:

- `NSPasteboardTypeString`, der Text. Ein von Hand kopierter Pfad, ein aus einer
  Adresszeile kopierter Link und der Auszug aus einer Textdatei liegen alle hier.
- `NSPasteboardTypeFileURL`, der Dateiverweis. Wer im Finder eine Datei mit Cmd+C
  kopiert, legt diesen Typ ab und daneben den Namen als Text, nicht aber den
  vollständigen Pfad als Text.

Der Plan setzt C10 heute auf den Text an, weil die Beauftragung des Nutzers von
einem Pfad in der Zwischenablage spricht und ein Pfad Text ist. Damit trifft
"im Finder kopieren, in KRK hinspringen" ins Leere: die Zwischenablage trägt
dann einen Dateiverweis und als Text nur den bloßen Dateinamen, der kein
absoluter Pfad ist und nach der Regel aus C10 in der Statuszeile als nicht
verwertbar endet.

Warum jetzt: die Antwort ändert eine Zeile in `crates/krk-ui/src/appkit/zwischenablage.rs`
aus Schritt 13 und ein Abnahmekriterium in C10. Nach Schritt 19 wäre sie ein
Nachtrag an zwei Stellen statt an einer.

## Optionen

1. **Nur Text lesen.** Die Auswertung fragt `NSPasteboardTypeString` ab und
   nichts sonst.
   - Dafür: eine Sorte, ein Weg, keine Reihenfolgeregel. Deckt den Fall ab, den
     der Nutzer wörtlich genannt hat, nämlich den Pfad in der Zwischenablage.
   - Dagegen: der häufigste Weg, wie ein Pfad auf einem Mac in die Zwischenablage
     kommt, ist Cmd+C im Finder, und genau der liefert keinen Text-Pfad. Der
     Nutzer erlebt eine Funktion, die bei der naheliegendsten Eingabe eine
     Fehlermeldung zeigt, und liest das als Defekt.

2. **Text und Dateiverweis lesen, Dateiverweis zuerst.** Die Auswertung fragt
   `NSPasteboardTypeFileURL` ab; liegt dort nichts, fragt sie den Text.
   - Dafür: "im Finder kopieren, in KRK hinspringen" arbeitet. Die Reihenfolge
     ist keine Sonderregel, sondern die genauere Sorte vor der ungenaueren: ein
     Dateiverweis ist bereits ein Pfad und braucht keine Deutung, ein Text muss
     erst als Pfad erkannt werden.
   - Dagegen: zwei Abfragen statt einer, und die Vorschau aus C10 muss
     entscheiden, was sie bei einem Dateiverweis zeigt. Naheliegend ist dieselbe
     Anzeige, die C6 für die ausgewählte Datei kennt, was C10 an C6 bindet.

3. **Nur Text in Runde 1, Dateiverweis als eigene Fähigkeit einer späteren
   Runde.**
   - Dafür: hält den Umfang dieser Runde klein, nachdem C10 ihn ohnehin schon
     erweitert.
   - Dagegen: die spätere Runde ändert dieselbe Datei und dasselbe
     Abnahmekriterium noch einmal. Der Aufwand ist heute eine Abfrage und später
     ein Nachzug an drei Stellen.

## Constraints

- Die Antwort darf keine der zehn Zahlen aus C8 berühren. Keine der drei Optionen
  tut das.
- Sie darf die Grenze des Circles nicht verschieben: KRK schreibt die
  Zwischenablage in keinem Fall, und Cmd+C und Cmd+V bleiben nach C3 unbelegt.
- Der Sprung selbst bleibt in jedem Fall die Prüfung und Navigation aus der
  Pfadeingabe von Schritt 13; ein zweiter Navigationsweg entsteht nicht.

## Empfehlung

Möglichkeit 2. Der Mehraufwand ist eine zweite Abfrage an demselben Objekt, und
er kauft den Fall ab, den ein Nutzer als ersten ausprobiert. Die Reihenfolge
"Dateiverweis vor Text" ist keine Fallunterscheidung mit eigenem Rückfallweg,
sondern eine Rangfolge zweier Eingaben in dieselbe Auswertung; der Weg dahinter
bleibt einer. Für die Vorschau gilt dann dieselbe Anzeige wie in C6, weil ein
Dateiverweis auf eine Datei zeigt und C6 bereits sagt, wie eine Datei aussieht.

---
Answered: Nutzerentscheid 260804 — **Text und Dateiverweis.** Die Auswertung liest beides: einen Pfad oder eine URL als Text, und den Dateiverweis, den der Finder bei `Cmd+C` auf einer Datei ablegt. Die Empfehlung dieses Datensatzes trägt: `Cmd+C` im Finder ist der naheliegendste Weg, einen Pfad in die Zwischenablage zu bringen, und genau dieser Weg legt keinen Text-Pfad ab. Ein reiner Textleser fiele damit im häufigsten Fall aus. Der Preis ist eine zweite Abfrage am Pasteboard. Bindet S13; die Umsetzung zieht diesen Datensatz auf `_i_`.
