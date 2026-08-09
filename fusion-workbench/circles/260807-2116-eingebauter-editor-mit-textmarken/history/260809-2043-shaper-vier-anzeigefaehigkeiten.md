# Shaper: die vier Anzeigefähigkeiten vom 260809 in Directive und Spec nachgezogen

**Datum:** 2026-08-09, 20:43
**Agent:** shaper (in-Circle-Klärung, ohne Rückfragewerkzeug)
**Circle:** `circles/260807-2116-eingebauter-editor-mit-textmarken`
**Anlass:** Der Nutzer hat am 260809-2035 den Umfang des aktiven Circles um vier Anzeigefähigkeiten erweitert, bei 24 von 42 erledigten Planschritten.

## Was der Nutzer verlangt hat

Vier Fähigkeiten, alle von ihm bestätigt: eine Fokusanzeige für alle fünf Bereiche, Zeilennummern im Editor, Zeilennummern in der Vorschau, und der absolute Pfad im Fenstertitel. Drei Punkte waren dabei ausdrücklich zu klären statt offenzulassen: die Regel für den Fenstertitel, die Form der Fokusanzeige, und ob Editor und Vorschau sich eine Zeilennummernanzeige teilen können.

## Was daraus geworden ist

**Aus vier Wünschen sind drei Fähigkeiten geworden.** C9 trägt die Fokusanzeige, C10 die Zeilennummern in beiden Flächen, C11 den Fenstertitel. Die beiden Zeilennummern-Wünsche sind zusammengelegt, weil Editor und Vorschau dieselbe Bauart tragen; die Prüfung dazu steht unten.

**Neue Fähigkeiten statt Erweiterungen der bestehenden**, und der Grund ist, dass jede der drei die Grenze des Editors überschreitet. Die Fokusanzeige gilt für vier Bereiche der Runde 1 und einen dieser Runde. Die Zeilennummern in der Vorschau berühren C6 der Runde 1. Der Fenstertitel gehört dem Fenster und keiner der acht bestehenden Fähigkeiten. Eine Unterbringung in C1 bis C6 hätte Arbeit, die mit dem Editor nichts zu tun hat, unter Editor-Fähigkeiten versteckt.

**Einunddreißig Abnahmekriterien sind dazugekommen**: acht in C9, zwölf in C10, elf in C11. Der Spec zählt damit 109 statt 78, davon 107 in den Fähigkeiten und zwei im Abschnitt über die Zeitzusagen.

## Die drei Klärungen im Einzelnen

### Der Fenstertitel: der Fokus entscheidet

Der Titel zeigt den Pfad dessen, was der Bereich mit dem Fokus hält. Fünf Fokuswerte, fünf genannte Antworten, kein Auffangzweig: Dateifenster den angezeigten Ordner, Editor seine Datei, Vorschau ihre Datei, Lesezeichenleiste den Ordner des aktiven Dateifensters, und ein offenes Blatt lässt den Titel stehen. Hält ein Bereich nichts mit einem Pfad, etwa ein leerer Editor, gilt der Ordner des aktiven Dateifensters.

Damit sind beide Fragen des Nutzers beantwortet. Hält der Editor eine andere Datei als das aktive Dateifenster anzeigt, entscheidet der Fokus: wer im Editor tippt, sieht dessen Datei. Ist kein Editor offen, kommt er im Titel nicht vor.

Der tragende Grund für die Bindung an den Fokus statt an das aktive Dateifenster: KRK führt genau einen Fokus, und C9 macht ihn im selben Zug sichtbar. Der Rahmen sagt dann, wo der Nutzer arbeitet, der Titel, woran. Die Gegenmöglichkeit bräuchte zwei Begriffe und zeigte den Ordner des Dateifensters, während der Nutzer im Editor schreibt.

Ein Nebenbefund, der als Abnahmekriterium steht: die Auswahl im Dateifenster ändert den Titel nicht. Ein Titel, der ihr folgte, schriebe bei jedem Druck auf eine Pfeiltaste neu, und genau diesen Weg misst L1 aus C8 der Runde 1.

### Die Fokusanzeige: der gebaute Rahmen, auf fünf Bereiche gezogen

Am Code geprüft, nicht angenommen: `Aufteilung::aktives_markieren` (`crates/krk-ui/src/appkit/aufteilung.rs:229-238`) setzt heute die Rahmenfarbe eines `NSBox` je Dateifenster, `controlAccentColor` für das aktive, `separatorColor` für das andere, bei zwei Punkten Breite. Die drei übrigen Bereiche tragen keinen Kasten (`rahmen: [Retained<NSBox>; 2]`, Zeile 134).

Die zweite macOS-Gewohnheit, die hervorgehobene gegen die zurückgetretene Auswahlfarbe, trägt hier nicht für alle fünf: die Textanzeige der Vorschau lehnt Auswahl ab (`vorschau.rs:513`), und der Editor hat eine Schreibmarke statt einer ausgewählten Zeile. Eine Anzeige für drei von fünf ist keine.

**Dabei ist eine offene Frage entstanden**, und sie ist der einzige neue Datensatz dieser Runde: der Akzentrahmen soll künftig zwei Aussagen tragen, den Fokus und das aktive Dateifenster. Der Vorschlag des Shapers sind drei Zustände, und er steht als Vorbelegung im Spec, damit kein Planschritt wartet. Der Datensatz ist `decisions/260809-2043_o_bedeutet-der-akzentrahmen-kuenftig-den-fokus-oder-das-aktive-dateifenster.md`.

**Ein bestehender Defekt bindet C9 und ist im Spec zitiert.** `issues/260809-1738_o_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md` hält fest, dass `Anwendungsdelegierter::fokus` für jeden Ersthelfer außerhalb der fünf genannten Ansichten `Dateifenster` antwortet. Heute zeigt sich das nur darin, dass der falsche Befehl wirkt; mit einer Fokusanzeige wird derselbe Fehler zu einem sichtbar falsch gesetzten Rahmen. Ein Abnahmekriterium von C9 verlangt die richtige Antwort für einen Klick in eine Unteransicht.

### Die Zeilennummern: eine Anzeige für beide Flächen

Am Code geprüft: Editor (`appkit/editor.rs:344-382`) und Vorschau (`appkit/vorschau.rs:501-525`) bauen beide eine `NSTextView` in eine `NSScrollView`, beide mit `setHorizontallyResizable(false)`, also mit Umlauf an der Behälterbreite. Die Unterschiede liegen woanders: die Vorschau lehnt Bearbeiten und Auswahl ab, der Editor nimmt beides an. Für eine Nummernspalte spielt das keine Rolle. Eine gemeinsame Anzeige ist damit möglich, und der Spec sagt sie zu; womit sie gebaut wird, bleibt beim Planner.

Die Zählung kommt aus `krk_core::text::zeilen::Zeilenindex` und wird nicht zweimal geschrieben. Was der Index nicht liefert, ist die Höhe auf dem Schirm; die weiß beim Umlauf allein der Layoutverwalter. Zwei Fragen an zwei Stellen, die je eine Hälfte kennen, und keine zweite Rechnung.

Der Umlauf hat ein eigenes Abnahmekriterium bekommen: eine Dateizeile, die über mehrere Bildschirmzeilen läuft, trägt genau eine Nummer neben ihrer ersten. Eine Spalte, die je Bildschirmzeile zählte, gäbe für den Sprung aus C5 und die Textmarke aus C6 falsche Nummern aus.

Bei Markdown decken sich Ansicht und Datei zeilenweise, und das ist entschieden: die Antwort vom 260808-0155 (`decisions/260808-0140_a_...`) lässt die Auszeichnungszeichen stehen. Wäre die Gegenmöglichkeit gewählt worden, trüge die Formatansicht keine Dateizeilen und C10 hätte für sie keine Zusage.

## Was geändert wurde

**Der Circle-Datensatz `_t_circle.md`, Abschnitt `## Directive`**, hat einen Satz bekommen, der die drei Fähigkeiten in seiner Sprache nennt. Er bleibt ein Absatz. Kein anderer Abschnitt ist angefasst.

Das ist eine Abweichung vom sonstigen Zuschnitt des Shapers: beim Nachtrag vom 260808 hat er den Wortlaut genannt und die Änderung dem Orchestrator überlassen. Der Auftrag vom 260809-2035 nennt den Datensatz und den Abschnitt ausdrücklich als Ziel. Der eingefügte Satz steht im Spec im Wortlaut, damit der Nutzer ihn prüfen kann.

**Der Spec** hat bekommen: die drei Fähigkeiten C9 bis C11, ein Diagramm unter `### Eine Quelle, zwei Anzeigen`, einen Abschnitt `## Die vier später hinzugekommenen Fähigkeiten`, einen Vermerk im Kopf, sieben neue Punkte unter `## Offen für den Planner`, vier neue unter `## Ausdrücklich außerhalb dieser Runde`, zwei neue Randbedingungen, einen Absatz über die berührten Zeitzusagen, einen Absatz darüber, dass die drei neuen Fähigkeiten keine der drei vollständigen Fallunterscheidungen berühren, und einen dritten Punkt unter `## Was die Abnahme mitentscheidet`.

Der Abschnitt `## Abgleich mit der Circle-Directive` ist nachgezogen: die Directive nennt jetzt zwölf Bestandteile statt zehn, und der Absatz über die zu streichenden Wörter " und Textbereiche" ist durch den geprüften Befund ersetzt, dass sie nicht mehr im Datensatz stehen.

**Der Plan ist nicht angefasst.** Er ist Sache des Planners.

**Der offene Defekt `260809-2029_o_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md` ist nicht angefasst**, wie beauftragt.

## Was der Nutzer entscheiden muss

Ein neuer Datensatz mit dem Marker offen: `decisions/260809-2043_o_bedeutet-der-akzentrahmen-kuenftig-den-fokus-oder-das-aktive-dateifenster.md`. Er hält keinen Planschritt auf.

Dazu die zweite Ableitung des Shapers, die der Nutzer am Gate umstoßen kann: der Fenstertitel folgt dem Fokus. Sie steht im Spec unter `## Was die Abnahme mitentscheidet`.
