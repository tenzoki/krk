# Planner: Nachtrag C9, C10 und C11 am bestehenden Plan

**Datum:** 2026-08-09, 21:12
**Agent:** `planner`
**Auftrag:** den bestehenden Plan `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` um Schritte für die drei Anzeigefähigkeiten erweitern, die der Spec seit dem 260809-2043 trägt. Kein neuer Plan, keine Umnummerierung.

## Was entstanden ist

Sechs Schritte, S43 bis S48, in drei Bündeln, dazu ein neuer Abschnitt `## Nachtrag vom 260809` im bestehenden Plan mit drei Befunden aus der Bestandsaufnahme, den Antworten auf die sechs Punkte, die der Spec dem Planner neu überlässt, zwei Mermaid-Bildern und der Reihenfolge gegen die achtzehn offenen Schritte.

| Schritt | Titel | Fähigkeit |
|---|---|---|
| S43 | Die Fokusabfrage fragt nach Enthaltensein statt nach Nämlichkeit | C9 |
| S44 | Fünf Kästen, eine Regel, drei Zustände | C9 |
| S45 | Der eine Auslösepunkt für jeden Wechsel des Ersthelfers | C9 |
| S46 | `appkit/nummernspalte`: die eine Spalte, im Editor eingehängt | C10 |
| S47 | Dieselbe Spalte in der Vorschau, und die Regel, wann sie steht | C10 |
| S48 | Der Fenstertitel folgt dem Fokus | C11 |

## Die tragende Frage und ihre Entscheidbarkeit

**Woher erfährt KRK, dass sich der Ersthelfer geändert hat?** Über einen Fokusbefehl weiß es KRK selbst; ein Mausklick ändert den Rang an KRK vorbei, und `Anwendungsdelegierter::fokus` ist eine Abfrage und keine Benachrichtigung. Entscheidbar ist die Frage, weil AppKit genau einen Durchgang hält: `NSWindow::makeFirstResponder:`. Eine Unterklasse, die ihn überschreibt, beobachtet eine entschiedene Größe statt einer vorhergesagten. Verworfen und begründet: eine Benachrichtigung gibt es nicht, Schlüsselwertbeobachtung auf `firstResponder` ist nicht zugesagt, ein Takt wäre die ausgeschlossene Vorhersage, und fünf meldende Ansichten wären fünf Unterklassen statt einer.

## Drei Befunde am Code

1. **Der Wechsel des Ersthelfers hat genau einen Durchgang.** `fokus_setzen` (`anwendung.rs:1199-1216`) und AppKit rufen dieselbe Methode.
2. **Die Enthaltensfrage kostet an der Stelle nichts, an der der Defekt `260809-1738_o_` sie teuer nennt.** Für den Feldeditor eines Textfeldes im Dateifenster antwortet der heutige Rückfall bereits `Dateifenster`; die Enthaltensfrage antwortet dasselbe. Was sich ändert, ist ausschließlich der Fall des Defekts.
3. **AppKit hält den Platz für die Nummernspalte schon frei.** `NSScrollView` führt `setVerticalRulerView`, und `NSRulerView` steht in `objc2-app-kit 0.3.2` samt `drawHashMarksAndLabelsInRect:`, `setClientView:` und `setRuleThickness:`. Am Bibliotheksbestand geprüft.

## Was am bestehenden Plan geändert wurde

- Kopfzeile `**Spec:**` um die drei Fähigkeiten erweitert, dazu ein dritter Absatz in der Zeile `**Entscheidbarkeit:**`.
- S33 bekommt eine Zeile: der Aufruf von `Nummernspalte::neu_zeichnen` nach dem Umschalten der Ansicht.
- S42 bekommt drei Abhängigkeiten (S45, S47, S48), drei Nachträge an Spec und `CLAUDE.md` und die Zahl 48 statt 42.
- Datenstrukturen um vier Zeilen, Risiken um vier Zeilen, Teststrategie um einen Absatz, "supersimpel" um vier Stellen, Offene Fragen um drei Punkte erweitert.

## Kein neuer Entscheidungsdatensatz

Die eine offene Frage der drei Fähigkeiten steht bereits als `decisions/260809-2043_o_bedeutet-der-akzentrahmen-kuenftig-den-fokus-oder-das-aktive-dateifenster.md`. Sie bindet S44 und hält ihn nicht auf: `### Frage 14` rechnet für alle drei Möglichkeiten aus, was sie kosten, und in jeder ist es ein Funktionsrumpf und höchstens ein zusätzlicher Schritt. Zwei Punkte, die eine eigene Frage hätten werden können, sind stattdessen beantwortet: die Lesart des zweiten Abnahmekriteriums von C9 folgt aus jenem Datensatz, und der Schnitt der Fokusabfrage folgt aus dem vierten Abnahmekriterium von C9.

## Vorbehalte

- `speculation:` Der Neuaufbau des Zeilenindex bei einer Datei nahe 16 MB ist ungemessen. Der Ausweg über die Fortschreibung aus `NSTextStorage` ist in der Risikotabelle benannt und wird nicht auf Verdacht gebaut.
- `inference:` Ob zwei Abstufungen der Akzentfarbe auf dem Referenzgerät gut genug zu unterscheiden sind, ist ungemessen; der Datensatz vom 260809-2043 sagt es selbst.
