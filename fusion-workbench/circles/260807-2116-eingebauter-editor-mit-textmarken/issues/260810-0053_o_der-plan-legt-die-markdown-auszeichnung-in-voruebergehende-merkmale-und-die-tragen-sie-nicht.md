Der Plan legt die Markdown-Auszeichnung in vorübergehende Merkmale, und die tragen sie nicht

---

Aufgefallen beim Bau von S33 am 260810-0053. Kein Fehlverhalten im Code: S33 ist
so gebaut, dass alle vier zugesagten Wirkungen sichtbar sind. Was hier steht, ist
ein Widerspruch zwischen zwei Planstellen und einer gemessenen Eigenschaft von
AppKit, und er gehört korrigiert, damit die nächste Runde nicht auf einer Zusage
weiterbaut, die so nicht gilt.

---

## Der Befund

`### Frage 7` des Plans und der Datensatz
`decisions/260808-0140_a_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md`
nennen beide **eine** Mechanik für die ganze Formatansicht: vorübergehende
Merkmale des Layoutverwalters, gesetzt über
`NSLayoutManager::setTemporaryAttributes:forCharacterRange:`. Die Empfehlung des
Datensatzes formuliert es ausdrücklich als Eigenschaft der Bauart:

> Sie ist außerdem die einzige, die Markdown nicht zum Sonderfall macht.
> Einfacher Text, Code und Markdown laufen danach durch dieselbe Mechanik […]

Unter dieselbe Mechanik zählt die gewählte Möglichkeit 1 aber vier Wirkungen:
„Überschriften größer und fett, Listen eingerückt mit abgesetztem
Aufzählungszeichen, Links unterstrichen und eingefärbt, Quelltextblöcke in fester
Schrift."

Drei der vier gehen über vorübergehende Merkmale **nicht**, und der Grund steht
im Kopf des Systems selbst:

> Temporary attributes provide a way to override attributes for drawing on a
> per-layout manager basis, without affecting the underlying stored text.
> Clients may set any attributes they wish, but the only attributes that the
> layout manager will recognize for drawing are those that do not affect layout
> (color, underline, etc.).
>
> `MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers/NSLayoutManager.h:351`

Schriftgröße, Schriftschnitt, feste Schrift und Absatzeinzug ändern die
Auslegung. Als vorübergehendes Merkmal gesetzt tun sie nichts — nicht etwas
Falsches, sondern gar nichts. Von den vier zugesagten Wirkungen trüge diese
Mechanik allein die letzte Hälfte der dritten, nämlich die Unterstreichung und
die Einfärbung der Links.

## Was S33 stattdessen gebaut hat

Die Fallunterscheidung heißt nicht „Farbe gegen Rest", sondern **„wirkt auf die
Auslegung oder nicht"**, und sie ist trennscharf und vollständig, weil der Kopf
oben sagt, welche Seite AppKit beachtet:

| Wirkung | Wohin sie geht |
|---|---|
| Farbe der Wortarten, Unterstreichung der Links | vorübergehende Merkmale, `NSLayoutManager` |
| Überschrift größer und fett, Listeneinzug, feste Schrift für Quelltext | Merkmale des `NSTextStorage` |

Die Zusage, an der der Plan hängt, hält dabei unverändert, und sie hängt nicht an
den vorübergehenden Merkmalen: **die Auszeichnung kann beim Sichern nicht in die
Datei geraten.** Gesichert wird `Editormodell::stand`, eine gewöhnliche
Zeichenkette, und die kommt aus `NSTextView::string` — den Zeichen der Fläche.
Kein Merkmal wird auf dem Sicherungsweg auch nur gelesen, gleich in welchem der
beiden Speicher es liegt. `setRichText(false)` steht daneben.

Die drei übrigen Folgerungen des Plans halten ebenso: das Umschalten ist ein
Zurücksetzen und Neusetzen und kein Umbau, es gibt keine zweite Textkopie, und
die Schreibmarke bleibt stehen, weil die Zeichen unberührt bleiben.

## Was zu korrigieren wäre

1. `### Frage 7` des Plans: der Absatz „Ein `NSTextStorage`, zwei Darstellungen,
   und die Einfärbung als vorübergehende Merkmale des Layoutverwalters" ist für
   die **Einfärbung** richtig und für die Markdown-Auszeichnung falsch. Er
   braucht den zweiten Satz und die Zeile aus dem SDK-Kopf.
2. S33, Abschnitt „Änderungen" und „Abnahmekriterium": die Zeile „Der Diff zeigt,
   dass die Einfärbung über `setTemporaryAttributes` und nicht über
   `addAttributes` läuft" gilt weiter und ist eingehalten. Was fehlt, ist die
   Aussage, dass `addAttributes` für die layoutwirksamen Auszeichnungen der
   richtige und einzige Weg ist, und warum das die Zusage nicht bricht.
3. Der Datensatz vom 260808-0140: die Empfehlung begründet Möglichkeit 1 unter
   anderem damit, dass sie „nicht durch Sorgfalt, sondern durch die Bauart" hält,
   weil „der Textspeicher unangetastet bleibt". Der erste Halbsatz stimmt, der
   zweite nicht mehr. Die **Wahl** des Nutzers ist davon nicht berührt: alle vier
   Wirkungen sind gebaut, die Auszeichnungszeichen bleiben stehen, und der Stand
   in der Ansicht ist Zeichen für Zeichen der Stand der Datei.

Nichts davon ändert, was der Nutzer sieht. Es ändert, was der Plan über den Weg
dorthin behauptet.

**Aufgefallen bei:** dem Bau von S33 am 260810-0053, beim Nachlesen des
SDK-Kopfes vor dem Setzen der ersten Merkmale.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (`### Frage 7`, Schritt 33),
`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_a_was-heisst-gerendert-bei-markdown-wenn-zugleich-bearbeitet-wird.md` (Möglichkeit 1, Empfehlung),
`crates/krk-ui/src/hervorhebung.rs` (Modulkopf, Abschnitt „Zwei Listen, und warum es zwei sein müssen"),
`crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::formatierung_anwenden`)
