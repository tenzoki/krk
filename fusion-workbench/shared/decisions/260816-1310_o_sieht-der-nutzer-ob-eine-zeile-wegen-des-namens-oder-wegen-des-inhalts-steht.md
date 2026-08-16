# Sieht der Nutzer, ob eine Zeile wegen ihres Namens oder wegen ihres Inhalts in der Liste steht?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-core/src/verzeichnis/modell.rs:542-587` (der eine Prüfschritt); `crates/krk-ui/src/appkit/tabelle.rs` (die flache Tabelle mit ihren vier Spalten, `ORDNERZEICHEN`); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/_b_circle.md`, Abschnitt `## Directive`, „Was diese Runde fallen lässt" (kein Baum, keine zweite Tabellenklasse); `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md` (C1, C4)

---

## Question

Die Antwort des Nutzers vom 260816 macht die beiden Treffergründe **überschneidungsfrei**: der Inhalt einer Datei wird nur gelesen, wenn ihr Name den Filtertext nicht schon trägt. Eine Zeile steht damit entweder wegen ihres Namens oder wegen ihres Inhalts in der Liste, nie wegen beidem. Genau dadurch wird die Kennzeichnung überhaupt erst eine wohldefinierte Aussage; solange beide Gründe zugleich gelten konnten, hätte eine Markierung erklären müssen, welcher von beiden gemeint ist.

Ohne Kennzeichnung ist die Liste mehrdeutig zu lesen. Wer `budget` tippt und `steuer-2025.csv` in der Liste sieht, weiß nicht, ob KRK etwas Falsches anzeigt oder ob das Wort in der Tabelle steht. Beim Namensfilter stellte sich diese Frage nicht: der Grund stand im Namen, den der Nutzer vor sich sah.

## Options

1. **Keine Kennzeichnung.** Die Liste zeigt Treffer, ohne den Grund zu nennen.
   - Pro: nichts zu bauen, keine Spalte, kein Zeichen, keine Farbe. Die Tabelle behält ihre vier Spalten.
   - Kontra: der Nutzer kann eine Trefferliste nicht mehr lesen. Bei eingeschaltetem Inhaltsfilter enthält sie Zeilen, deren Zusammenhang mit dem Getippten unsichtbar ist, und das ist der Regelfall und nicht der Ausnahmefall.

2. **Ein Zeichen in der Namensspalte**, wie der Schrägstrich, den ein Ordner seit dem 260815 dort trägt.
   - Pro: derselbe Weg, den die Namensspalte für eine andere Zusatzaussage schon geht, und er ist gebaut. Keine neue Spalte, keine Änderung der Breitenrechnung.
   - Kontra: die Namensspalte trägt dann zwei Zusatzzeichen mit verschiedener Bedeutung. Der Schrägstrich ist ausdrücklich Anzeige und nie Name, und der Filter nimmt ihn deshalb nicht auf (`filter.rs`, `traegt_ein_dateiname`); ein zweites solches Zeichen erbt dieselbe Sonderbehandlung und dieselbe Falle.

3. **Die Zeile wird abgesetzt dargestellt**, etwa in gedämpfter Schrift oder mit anderer Textfarbe.
   - Pro: kein Zeichen im Namen, keine Spalte, keine Breitenrechnung. Die Unterscheidung ist auf einen Blick da.
   - Kontra: die Tabelle hat heute keine zeilenweise Einfärbung nach Bedeutung, und eine solche Regel müsste sich mit der Auswahl, der Markierung und beiden Farbtafeln vertragen. Das ist mehr, als die Aussage wiegt.

4. **Die Statuszeile nennt die Aufteilung**, etwa `Filter „budget": 12 nach Name, 5 nach Inhalt`.
   - Pro: keine Änderung an der Tabelle. Die Zahlen fallen beim Prüfschritt ohnehin an.
   - Kontra: sagt, **wie viele** aus welchem Grund stehen, aber nicht, **welche**. Die Frage des Nutzers vor einer einzelnen Zeile bleibt unbeantwortet. Und die Zeile ist schon Gegenstand von `260816-1310_*_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md`; beide Antworten müssten in denselben Satz passen.

## Constraints

- Kein hierarchisches Modell, keine `NSOutlineView` und keine zweite Tabellenklasse. Das hat die Runde 10 ausdrücklich fallen lassen, und der Circle-Datensatz schreibt aus, dass es später niemand als Versehen nachbauen soll.
- Die Tabelle hat vier Spalten, drei davon schaltbar über die Bereichsleiste. Eine fünfte wäre ein neuntes Ankreuzfeld und eine eigene Frage.
- Die beiden Gründe schließen einander aus. Eine Kennzeichnung, die einen dritten Zustand kennt, beschreibt etwas, das es nicht gibt.
- Für einen Ordner gilt die Frage nicht: ein Ordner steht wegen seines Namens oder, bei eingeschalteter tiefer Suche, wegen eines Befundes darunter. Welcher Art der Befund war, ist eine eigene Frage und wird von keiner Möglichkeit hier beantwortet.

## Recommendation

Möglichkeit 3. Sie trägt die Aussage dort, wo der Nutzer sie braucht, nämlich an der einzelnen Zeile, und sie belastet weder den Namen noch die Spaltenrechnung. Der Einwand gegen sie ist echt und benannt: die Tabelle kennt heute keine bedeutungstragende Einfärbung, und eine erste muss sich mit Auswahl, Markierung und beiden Farbtafeln vertragen. Möglichkeit 2 ist billiger zu bauen und teurer zu behalten, weil sie die Ausnahmebehandlung des Ordnerzeichens ein zweites Mal erzeugt. Möglichkeit 1 ist nur dann vertretbar, wenn der Nutzer die Mehrdeutigkeit ausdrücklich hinnimmt; sie wäre dann als hingenommener Verlust im Spec zu benennen und nicht stillschweigend zu wählen.

---
Answered:
Implemented:
Deferred:
Superseded by:
