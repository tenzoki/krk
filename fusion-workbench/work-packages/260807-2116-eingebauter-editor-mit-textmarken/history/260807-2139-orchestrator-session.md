# Orchestrator Session — 260807-2139

**Directive:** Der eingebaute Editor mit Roh- und Formatansicht und Textmarken (aus dem
Circle-Datensatz `circles/260807-2116-eingebauter-editor-mit-textmarken/_t_circle.md`,
Abschnitt `## Directive`)
**Mode:** custom — Shaping und Planung stehen an, es gibt weder Spec noch Plan
**Status:** laufend

## Ausgangslage

**Git HEAD bei Sitzungsbeginn:** `eef4188`
**Aktiver Circle:** `260807-2116-eingebauter-editor-mit-textmarken`, aktiviert am 260807-2132
**Vorgänger:** `260802-0842-krk-mac-dateimanager-editor-git`, beschränkt abgeschlossen (`_b_`)

Der Circle ist am 260807-2116 vom `shaper` als vorgesehen angelegt und am 260807-2132 über
`/fusion:next` aktiviert worden. Sein Grounding-Abschnitt trägt sechs am Code belegte
Bauteile der Runde 1, die der Editor erbt, und benennt den Preis der ausgeklammerten
Messreihen.

## Die vier Festlegungen der Aktivierungsrunde

Vom Nutzer am 260807-2139 beantwortet. Sie bestimmen den Zuschnitt und gehen als Eingabe in
den Spec.

### 1. Formatansicht: eine Ansicht pro Dateityp, jeweils eigens besetzt

Markdown wird gerendert. Code bekommt Syntaxhervorhebung mit einklappbaren Blöcken.
Einfacher Text bekommt Zeilenumbruch am Fensterrand und eine lesbare Schriftgröße. Die
Rohansicht zeigt in allen drei Fällen die Zeichen so, wie sie in der Datei stehen.

**Der Nutzer ist der Empfehlung des Datensatzes nicht gefolgt.**
`shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` empfahl die dritte
Möglichkeit, die Formatansicht als durchweg schreibgeschützte Leseansicht. Gewählt ist die
erste. Der im Datensatz benannte Preis gilt damit: der Unterschied zwischen Roh- und
Formatansicht ist bei einfachem Text schwach. Der Gegenwert: in der Formatansicht bleibt
das Bearbeiten möglich, was die dritte Möglichkeit ausgeschlossen hätte.

### 2. Einstieg: F4 aus dem Dateifenster und ein Übergang aus der Vorschau

Zwei Einstiegswege statt einem. **Der Übergang aus der Vorschau muss die Datei aktiv
mitnehmen**, weil sich die Vorschau beim Öffnen des Editors schließt (Festlegung der
Circle-Anlage). Ein Übergang, der die Datei nur stehen ließe, verlöre sie.

### 3. Ungespeicherte Änderungen: Nachfragen beim Schließen

Sichern, verwerfen oder abbrechen. **Die Nachfrage muss an allen drei Anlässen greifen:**
beim Schließen des Editors, beim Beenden der Anwendung und bei der Sitzungssicherung in
`session.toml`. Der Editor ist der erste Bereich in KRK, der einen verlierbaren Zustand
hält.

### 4. Textmarke: Zeilennummer plus Textinhalt als Prüfung

Eine Marke merkt sich beide. Der Sprung geht zur gemerkten Zeile, prüft den dort gemerkten
Text und sucht bei Abweichung in der Nähe. Damit trifft eine unveränderte Datei sofort, und
eine von außen verschobene Stelle wird wiedergefunden. Die gemeinsame Gültigkeitsprüfung
der Lesezeichenleiste hängt daran.

## Bestand bei Sitzungsbeginn

Mit aktivem Circle deckt jeder `SCAN_*` zwei Speicher ab, den des Circles und den
gemeinsamen.

| Größe | Zahl |
|---|---|
| Offene Defekte (Circle + gemeinsam) | 1 — `shared/issues/260807-2112_o_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md` |
| Offene Planschritte | 0 — es gibt noch keinen Plan |
| Offene Fragen (Circle + gemeinsam) | 3 gemeinsam nach der Beantwortung der Formatansicht-Frage |
| Analysen | 0 im Circle |

Die fünf offenen Defekte und fünf offenen Fragen im terminalen Vorgänger-Circle bleiben
außerhalb der Reichweite. Das Portfolio führt sie als Warnung 2; sie haben keinen
Bearbeiter, solange sie dort liegen.

**Der Defekt zu `cmd+y` trifft diesen Circle unmittelbar.** `shift+cmd+y` ist der einzige
Tastenweg in das Vorschaufenster, und der Editor erbt dasselbe Fokusmuster. Ein Fokusbefehl
für den Editor mit Zusatztaste liefe in denselben Fehler. Er gehört in den Plan.

## Die sechs Festlegungen der Spec-Runde

Der `shaper` hat beim Schreiben des Spec fünf Fragen als Datensätze angelegt, weil sie sich
aus der Grundlage nicht ableiten ließen. Eine sechste ergab sich aus der Antwort auf die
erste. Vom Nutzer am 260808-0017 beantwortet.

### 5. Sprachen der Syntaxhervorhebung: eine fertige Kiste einbinden

Eine Rust-Kiste übernimmt Erkennung und Einfärbung für einige Dutzend Sprachen; das Projekt
schreibt keine Sprachregel selbst. Damit wird sie die **fünfte fremde Kiste mit Wirkung auf
die Anwendung** und braucht wie die vier bestehenden eine geschriebene Begründung in
`Cargo.toml`.

Zwei Preise sind angenommen. `speculation:` Ob eine solche Kiste die Maxime "superschnell"
auf dem Referenzgerät von 2018 hält, ist ungemessen, und der Abnahmelauf, an dem man es
messen würde, ist aus dieser Runde ausgeklammert. Und die Kiste bringt die einklappbaren
Blöcke nicht mit, siehe Festlegung 6.

### 6. Einklappbare Blöcke: entfallen in dieser Runde

Hervorhebung braucht Wortarten, Einklappen braucht Blockgrenzen; das sind zwei
Kenntnisse, nicht eine. Die Kiste liefert die erste. Die einklappbaren Blöcke kommen als
eigenes Vorhaben später.

**Damit ist die Festlegung 1 vom 260807-2139 zur Hälfte zurückgenommen.** Sie nannte für
Code "Syntaxhervorhebung mit einklappbaren Blöcken". Die Hervorhebung bleibt, das Einklappen
geht. Der Spec und der Datensatz zur Formatansicht tragen das nach.

### 7. Welche Dateien der Editor öffnet: eigene höhere Grenze, nur Text

Textdateien bis etwa 16 MB. Alles Nichttextliche und alles Größere wird abgewiesen, mit
Grund in der Statuszeile. Der Übergang aus der Vorschau legt dieselbe Prüfung an wie F4,
sonst gäbe es zwei Wege mit zwei Regeln.

Zwei Zahlen für dieselbe Frage sind damit angenommen: die Vorschau steht bei 1 MB, der
Editor bei 16 MB. Beide tragen dieselbe Regel, nämlich eine Obergrenze für das vollständige
Einlesen in den Arbeitsspeicher; verschieden ist, wie viel die jeweilige Handlung
rechtfertigt. `speculation:` Die 16 MB sind ein Vorschlag und keine gemessene Größe.

**Bindend, unabhängig von der Zahl:** kein Weg darf eine Datei beim Sichern verändern, die
der Editor nicht vollständig und verlustfrei als Text gelesen hat.

### 8. Textmarke: nur eine Stelle, die Directive wird nachgezogen

Eine Marke ist eine Zeile. Die Formulierung "und Textbereiche" in der Directive dieses
Circles gilt als überholt und ist im Circle-Datensatz zu streichen.

Der Grund ist nicht der Aufwand, sondern eine unbeantwortete Folgefrage: ein Bereich hat
zwei Anker, und was gilt, wenn nach einer Änderung von außen nur einer wiedergefunden wird,
ist zu entscheiden und nicht abzuleiten.

### 9. Nachfrage bei der Sitzungssicherung: fällt mit dem Beenden zusammen

Die Sitzung wird beim Beenden ein letztes Mal geschrieben, und dort steht die Nachfrage
ohnehin. Die getakteten Zwischenschreibvorgänge (höchstens einer je zwei Sekunden) fragen
nichts und tragen den ungesicherten Stand nicht mit; sie halten allein fest, welche Datei
offen ist.

**Der Preis ist angenommen:** bei einem Absturz oder einem erzwungenen Beenden ist der
ungesicherte Stand verloren, ohne dass jemand gefragt hätte. Eine Absturzsicherung, die
den Pufferinhalt mitsichert, ist ein eigenes späteres Vorhaben.

**Zu beachten beim Beenden:** `crates/krk-ui/src/appkit/anwendung.rs:1162` hält fest, dass
heute kein `applicationShouldTerminate:` im Weg steht und die Aufrufer nicht mit einer
Rückkehr rechnen. Die Nachfrage beim Beenden ändert das.

### 10. Suche in der Nähe: festes Fenster, Fehlschlag springt trotzdem

Gesucht wird der gemerkte Text in einem festen Fenster um die gemerkte Zeile, etwa fünfzig
Zeilen in beide Richtungen. Wird er nicht gefunden, springt die Marke an die gemerkte
Zeilennummer und meldet in der Statuszeile, dass die Stelle sich geändert hat. **Ungültig
heißt allein: die Datei fehlt.**

Der tragende Grund ist die gemeinsame Gültigkeitsprüfung der Leiste: sie wird bei jedem
Neuaufbau der Liste gestellt, und diese Trennung hält sie bei einer Frage an das
Dateisystem statt bei einem Lesevorgang je Marke.

`inference:` Fünfzig Zeilen ist ein Vorschlag, keine gemessene Größe. Wer sie ändert, ändert
eine Konstante und keine Regel.

**Als Grenze der Fähigkeit festzuhalten, unabhängig von der Wahl:** der gemerkte
Zeileninhalt ist keine eindeutige Kennung. Eine Marke auf einer mehrfach vorkommenden Zeile
kann nach einer Änderung von außen nicht zuverlässig wiedergefunden werden.

## Diagrammprüfung des Spec

`conceptrev` am 260807-2202, Urteil **acceptable**. Bericht:
`circles/260807-2116-eingebauter-editor-mit-textmarken/reviews/260807-2202-conceptrev-spec-eingebauter-editor-mit-textmarken.md`.
Alle drei Diagramme parsen, kein Zyklus, kein God-Node, höchste Ausgangsverzweigung zwei.

Drei Befunde für die Spec-Überarbeitung:

1. Diagramm 2 (Lesezeichen) mischt Datenmodell, Bauteil und Schritt in einem Bild. Der
   obere Teil wäre ein `erDiagram`.
2. C4 trägt kein Diagramm, obwohl es der zustandsreichste Teil ist: fünf Anlässe für die
   Nachfrage, drei Antworten, dazu die von außen geänderte Datei.
3. Die zwei Kanten aus `Editor` heraus in Diagramm 1 sind unbedingt gezeichnet, obwohl C4
   genau an sie die Nachfrage hängt.

## Die elfte Festlegung, nach der Spec-Überarbeitung

### 11. Sicherungsform: KRK schreibt immer die Unix-Form

Immer Unix-Zeilenenden, immer ein abschließender Umbruch, nie eine Bytefolgenmarke,
unabhängig von der Form, die die Datei mitbrachte. Vom Nutzer am 260808-0043 entschieden.

**Der Nutzer ist der Empfehlung des Datensatzes nicht gefolgt.** Empfohlen war, die Form der
Datei beim Lesen zu merken und beim Sichern zurückzuschreiben. Der benannte Preis ist
angenommen: das Sichern ändert Zeilen, die der Nutzer nicht angefasst hat, und eine fremde
Datei aus einem Windows-Projekt kommt verändert zurück. In einem versionierten Verzeichnis
heißt das eine Änderung in jeder Zeile statt in der einen.

**Diese Antwort steht noch nicht im Spec.** Sie fiel nach dessen Überarbeitung. Der Spec
trägt für das Sichern kein Abnahmekriterium zur Zeilenendenform. Verbindlich ist der
Datensatz `decisions/260808-0021_a_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`;
der Planner bekommt ihn ausdrücklich als bindende Grundlage mit und leitet daraus das
fehlende Kriterium ab.

## Spec-Abnahme

Am 260808-0043 vom Nutzer abgenommen.
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`,
79 Abnahmekriterien in acht Fähigkeiten C1 bis C8, fünf Diagramme.

Gegenüber der ersten Fassung geändert: C3 trägt die fremde Kiste samt Begründungspflicht in
`Cargo.toml` und hat die einklappbaren Blöcke verloren; C2 heißt "die beiden Einstiege und
die eine Prüfung davor" und trägt die 16-MB-Grenze; C4 hat vier feste Anlässe statt fünf mit
einem offenen; C6 trägt die Stelle statt des Bereichs, das Fenster von fünfzig Zeilen und
die Regel, dass ungültig allein das Fehlen der Datei heißt.

**Der schärfste Befund des Spec ist C7.** KRK reicht heute jeden Tastendruck unverändert an
AppKit weiter, sobald der Fokus in einer `NSTextView` steht
(`crates/krk-ui/src/appkit/ereignisse.rs:386`). Ein Editor auf dieser Klasse hätte mit dem
Fokus in sich selbst keine einzige Tastenbedienung mehr. Das ist keine Vermutung, sondern am
Code belegt.

**Die macOS-26-Frage ist geprüft und bindet diese Runde nicht.** `NSTextView`, TextKit 2 und
die Markdown-Auswertung stehen alle unter dem Zielsystem 15.0. Damit ist
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`
für diese Runde beantwortet, ohne dass der Datensatz selbst angefasst wurde: er gehört einem
terminalen Circle.

Von den drei vollständigen Fallunterscheidungen ohne Auffangzweig sind zwei berührt,
`schiebt_auffrischung_auf` nicht.

## Die zwei Festlegungen der Planungsrunde

### 12. Die y-Tasten auf der deutschen Tastatur: Buchstaben über das Zeichen nachschlagen

Buchstaben und Ziffern werden künftig über das gemeldete **Zeichen** nachgeschlagen,
Funktionstasten weiter über den **Tastencode**. Vom Nutzer am 260808-0155 entschieden.

**Der `planner` hat die Ursache des Defekts gefunden, und beide Verdächtigen des
Defektdatensatzes sind am Code widerlegt.** Weder greift das Menü `cmd+y` ab, noch trifft
die Normalisierung der Zusatztasten daneben. Die Stelle, die KRK als `kVK_ANSI_Y` führt
(Code 16, `crates/krk-core/src/tasten/parser.rs:209`), trägt auf einer deutschen Tastatur
ein Z. `cmd+y` liegt damit unter der Taste mit der Aufschrift Z, und Code 6, wo auf der
deutschen Tastatur das Y sitzt, ist in der ganzen Belegung unbelegt.

Derselbe Befund war am 260804 schon einmal geschlossen worden, mit der Begründung, `f3` sei
der Hauptweg. Seit dem 260807 gilt sie nicht mehr: `shift+cmd+y` hat keinen zweiten Weg.

Tragend für die Wahl war ein Punkt, den der Datensatz von 260803 noch nicht hatte: **das
Hauptmenü schlägt bereits heute über das Zeichen nach.** `NSMenuItem.keyEquivalent` nimmt
eine Zeichenkette (`crates/krk-ui/src/appkit/menue.rs:322-342`), und genau deshalb wirken
`cmd+c` und `cmd+v` auf jeder Tastaturbelegung an der beschrifteten Stelle. Die
zeichenbasierte Nachschlagart ist im Projekt keine fremde Mechanik, sondern die, die vier
Funktionen schon tragen. Der Zuschnitt beendet eine bestehende Asymmetrie, statt eine neue
zu schaffen.

Betroffen sind genau zwei der 58 ausgelieferten Kombinationen; alle übrigen Buchstaben
liegen auf deutscher und amerikanischer Tastatur gleich.

### 13. Gerendert bei Markdown: Auszeichnung bleibt stehen, Wirkung wird gezeigt

Die Quelltextzeichen bleiben sichtbar, und die ausgezeichneten Stellen bekommen ihre
Wirkung: Überschriften größer und fett, Listen eingerückt mit abgesetztem
Aufzählungszeichen, Links unterstrichen und eingefärbt, Quelltextblöcke in fester Schrift.
Vom Nutzer am 260808-0155 entschieden.

Der Grund, warum die Frage überhaupt entstand: "gerendert" und "in beiden Ansichten
bearbeitbar" lassen sich bei Markdown nicht beide in ihrer stärksten Lesart einlösen. Die
gewählte Auslegung geht mit beiden zusammen, weil der Stand in der Ansicht Zeichen für
Zeichen der Stand der Datei bleibt. Damit halten die Zusagen zu Suchen und Ersetzen, die
sich ausdrücklich auf den Text der Datei und nicht auf seine Darstellung beziehen.

**Der Preis ist angenommen:** wer "gerendert" als "wie im Browser" gemeint hat, bekommt das
nicht.

Die dritte Möglichkeit, volles Rendern mit Rückrechnung der Bearbeitung, ist nicht am
Aufwand gescheitert, sondern an der Sache: aus fettem Text folgt nicht, ob im Quelltext
zwei Sternchen oder zwei Unterstriche standen. Jede Wahl der Rückrechnung schriebe Zeilen
um, die der Nutzer nicht angefasst hat, also derselbe Schaden wie bei der Sicherungsform,
nur größer.

## Plan-Abnahme

Am 260808-0155 vom Nutzer abgenommen.
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`

**42 Schritte in acht Phasen:** A Tastenweg und Fokus (7), B Kernrechnung ohne Fenster (8),
C die Fläche (6), D die beiden Einstiege (3), E Sichern und Nachfrage (7), F die beiden
Ansichten (3), G Springen, Suchen, Marken (6), H Abnahme (2). Jeder Schritt geht an `coder`.

**Nutzerarbeit:** 23 der 42 Schritte brauchen den Nutzer für einen Teil ihrer Abnahme, weil
KRK dafür im Vordergrund stehen muss. Vollständig von einem Agenten abnehmbar sind 19:
S1, S3 bis S16, S18, S21, S27 und S32.

**Die tragende Entwurfsantwort auf C7** steht in der Zeile `**Entscheidbarkeit:**` des
Plans: der Ereignisabgriff fragt künftig nach der Nämlichkeit des Ersthelfers statt nach
seiner Klasse. Danach fällt fast alles Übrige von C7 ohne Sonderfall an, weil ein nicht
ausgeführtes Kommando das Ereignis nicht verbraucht.

**Alle Fragen des Circles sind beantwortet.** Der Circle-Speicher führt acht
Entscheidungsdatensätze, alle mit Marker `_a_`, keinen mit `_o_`.

## Turn-Protokoll

(Arbeitsschlange wird gebaut, Turn 1 steht an)
