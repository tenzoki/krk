# Woran erkennt der Nutzer in der Dateiliste auf einen Blick einen Ordner?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1309_c_die-markierung-ist-allein-an-der-farbe-erkennbar.md, circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1723_c_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md, circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_a_zweites-kennzeichen-der-markierung-und-ihr-platz-in-der-statuszeile.md

---

## Frage

Die Zellen der Dateiliste sind Beschriftungen ohne Sorten-Kennzeichen
(`crates/krk-ui/src/appkit/tabelle.rs`, `zellenansicht` und `beschriften`). Ein
Ordner unterscheidet sich von einer Datei heute an drei schwachen Stellen: er
steht in jeder der acht Sortierungen vorn (`gruppe`,
`crates/krk-core/src/verzeichnis/sortierung.rs:126,148`), seine Spalte `Größe`
zeigt `--`, und seine Spalte `Typ` bleibt leer, weil `endung()` nichts liefert.
Ein Kennzeichen an der Zeile selbst fehlt.

## Was den Lösungsraum bereits einengt

Drei Festlegungen dieses Projekts, alle geprüft:

- **Farbe allein ist kein Kennzeichen** (Nutzerentscheid 260805-0000, Befund
  260804-1309): bei Farbfehlsichtigkeit verschwindet sie. Dieselbe Überlegung
  steht am `Sinnbild` der Leiste (`crates/krk-ui/src/leistenmodell.rs:127`).
- **Fett und Orange sind vergeben.** Eine markierte Zeile steht in allen vier
  Spalten fett und orange (`zellenansicht`). Blau gehört der Auswahl. Ein
  viertes Kennzeichen darf mit keinem der drei zusammenfallen.
- **Ein Zeichen vor dem Namen ist einmal verworfen worden** (260805-0000), weil
  es den angezeigten Namen vom wirklichen trennt und die Namen in der
  Proportionalschrift gegeneinander verschiebt.

Ausgeschlossen ist damit auch der naheliegende vierte Weg, in der Spalte `Typ`
das Wort "Ordner" zu zeigen: der Nutzerentscheid 260806-2300 legt fest, dass
diese Spalte die Dateiendung führt, damit Anzeige und Sortierung übereinstimmen.

## Optionen

1. **Sinnbild in der Namensspalte** — dasselbe Muster, das die Leiste seit C5
   fährt: `NSImageView` mit einem Systemsinnbild neben der Beschriftung,
   Einzug über eine Rahmenrechnung, `None` wenn das System das Bild nicht
   kennt (`crates/krk-ui/src/appkit/leiste.rs:436-546`). Der Aufzählungswert
   `Sinnbild::Ordner` liegt schon vor.
   - Für: Form statt Farbe, wirkt bei jeder Farbfehlsichtigkeit; dieselbe
     Sprache wie die Leiste nebenan; entspricht Finder und ForkLift; VoiceOver
     liest die Sorte vor.
   - Kosten: die Zelle der Namensspalte ist heute **selbst** das `NSTextField`,
     und daran hängen zwei Dinge, die keine anderen Spalten haben: das
     Umbenennen an Ort und Stelle aus C4 (`editColumn:row:withEvent:select:`
     über `NAMENSSPALTE`, Rückweg `umbenennung_beenden(&NSTextField)`) und die
     Wiederverwendung über `makeViewWithIdentifier:` samt `downcast::<NSTextField>`.
     Beides muss auf `NSTableCellView` umgestellt werden, die AppKit-eigene
     Zellenklasse mit `imageView` und `textField`, die das Bearbeiten trägt.
     Die drei anderen Spalten bleiben, wie sie sind.
   - Zu prüfen: ein einziges, einmal erzeugtes Systemsinnbild je Sorte, kein
     Symbol je Datei über `NSWorkspace`; sonst steht je Zeile ein
     Plattenzugriff zwischen L3 und L10.

2. **Ordnernamen kursiv** — ein dritter und vierter Schriftschnitt neben den
   beiden vorhandenen, gesetzt nur in der Namensspalte.
   - Für: keine Änderung an der Ansichtsart, kein Eingriff ins Umbenennen,
     keine Änderung am angezeigten Text. Eine Eigenschaft mehr in einer Methode,
     die Schrift und Farbe ohnehin in jedem Durchgang setzt.
   - Kosten: ein markierter Ordner muss fett **und** kursiv sein. Dass sich zwei
     Schriftschnitte in diesem Baum zusammenlegen, ist nicht gesichert: der
     zurückgestellte Befund
     `circles/260812-1000-.../issues/260812-1851_d_zwei-schriftschnitte-legen-sich-nicht-zusammen-fett-in-kursiv-bleibt-aufrecht.md`
     hat genau das an der Vorschau gemessen. Anderer Codeweg, gleiche Frage:
     vor einer Zusage zu messen.
   - Kosten: Kursiv ist keine Mac-Konvention für "Ordner" und liest sich eher
     als "abgeschwächt".

3. **Schrägstrich hinter dem Ordnernamen** — eine Zeile in `beschriften`.
   - Für: billigste Fassung, Tradition der Norton-Reihe.
   - Kosten: der angezeigte Name ist nicht mehr der wirkliche, und dasselbe Feld
     ist der Editor des Umbenennens. Es braucht eine Regel, die den Schrägstrich
     beim Beginn der Bearbeitung wegnimmt und am Ende nicht zurückliest. Genau
     diese Trennung von angezeigtem und wirklichem Namen hat der Nutzerentscheid
     260805-0000 schon einmal abgelehnt.

## Randbedingungen

- Es bleibt bei vier Spalten (C1, C2). Eine fünfte Spalte ist verworfen.
- Das Kennzeichen darf Orange, Fett und Blau nicht antasten.
- Die zehn Zeitzusagen aus C8 gelten weiter; L3 und L10 messen das Blättern und
  Sortieren großer Ordner.

## Empfehlung

Option 1. Sie ist die einzige der drei, die kein bestehendes Kennzeichen und
keinen bestehenden Nutzerentscheid berührt, und sie baut keinen neuen
Mechanismus: die Leiste fährt dasselbe Muster mit derselben Begründung seit C5.
Der Preis ist eine umschriebene Umstellung der Namensspalte auf `NSTableCellView`
in einer Datei.

---
Answered:
Implemented:
Deferred:
Superseded by:

## Nutzerentscheid vom 260815-2058: Option 3, der Schrägstrich

Der Nutzer hat Option 3 gewählt: ein Ordner trägt in der Namensspalte einen
Schrägstrich hinter dem Namen. Die Kosten waren dabei vorgelegt, auch die
Ablehnung derselben Trennung am 260805-0000; die Wahl fällt trotzdem hierauf,
weil sie ohne Umbau der Zellenansicht auskommt.

### Was daraus folgt, und was die Umsetzung leisten muss

**Der Schrägstrich ist Anzeige und nicht Name.** Er entsteht in
`DateifensterDelegierter::beschriften` für `Spalte::Name` und nirgends sonst.
Sortierung, Filter, Zwischenablage, Vorschau und jede Dateioperation lesen
weiterhin `eintrag.name`; keine von ihnen sieht ihn.

**Das Umbenennen an Ort und Stelle (C4) zeigt und liest den wirklichen Namen.**
Dasselbe Textfeld ist Zelle und Editor. Zugesagt ist damit:

1. Beginnt eine Bearbeitung, steht der Name ohne Schrägstrich im Feld, gleich
   auf welchem Weg sie begonnen hat.
2. Endet sie mit Return, wird der eingegebene Text ohne Zutun des Nutzers als
   Name gelesen.
3. Endet sie mit Escape, steht in der Zelle wieder die Anzeigeform mit
   Schrägstrich, und der Name ist unverändert.

Der Weg dahin ist nicht festgelegt und gehört dem `coder`. Zwei Kandidaten,
beide am Baum geprüft und keiner an AppKit gemessen: der Haken
`control:textShouldBeginEditing:` samt `controlTextDidEndEditing:` des schon
konformen `NSControlTextEditingDelegate` (`tabelle.rs:2461`), dessen
Zeilenauffrischung es bereits gibt (`reloadDataForRowIndexes:columnIndexes:`,
`tabelle.rs:1731`) und dessen Feld dafür einen Delegierten bekäme; oder das
Abschneiden eines abschließenden Schrägstrichs in `umbenennung_beenden`. Der
erste Kandidat deckt jeden Einstieg in die Bearbeitung ab, auch den Klick ins
Feld, der an `umbenennung_beginnen` vorbeigeht. Welcher es wird, entscheidet die
Messung am Bündel und nicht diese Zeile.

**Eine Verknüpfung auf einen Ordner bekommt keinen Schrägstrich.** Das
Kennzeichen hängt an `Eintrag::ist_ordner()`, also an `Typ::Ordner`, und damit an
derselben Bedingung wie das `--` der Spalte `Größe` und die Gruppe der
Sortierung. Die Alternative hieße, das Verweisziel je Zeile zu erfragen, also ein
`stat` je sichtbarem Eintrag, und genau diese Schleife messen L3 und L10.

---
Answered: fusion-workbench/shared/decisions/260815-2056_a_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md §Nutzerentscheid — Option 3, Schrägstrich hinter dem Ordnernamen, Anzeige nur in der Namensspalte, Umbenennen zeigt und liest den wirklichen Namen.
Implemented:
Deferred:
Superseded by:
