# KRK — Bedienung

Diese Datei beschreibt die Arbeit **mit** KRK.

Sie ist kein Vollverzeichnis der Belegung. Die verbindliche Auskunft darüber,
welche Taste welchen Befehl auslöst, gibt `resources/default-keymap.toml`; in
der laufenden Anwendung gibt sie **F1**. Was hier steht, sind die Griffe, die
man sich schlecht merkt, und die Stellen, an denen die naheliegende Annahme
falsch ist.

---

## Vor dem Aktualisieren

Die neue Fassung über die alte kopieren und die alte **nicht vorher löschen**.
Werkzeuge, die eine App samt ihrer Stützdateien entfernen, nehmen KRKs
Ablageordner mit, und darin liegt alles, was sich das Programm merkt.

---

## Wo KRK seine eigenen Dateien ablegt

Alles liegt in `~/Library/Application Support/KRK/`, außerhalb des Bündels:

| Datei | Inhalt | Wer schreibt |
|---|---|---|
| `keymap.toml` | die eigene Tastenbelegung | KRK und der Nutzer |
| `bookmarks.toml` | die Lesezeichen | KRK |
| `session.toml` | Ordner, Tabs, Sortierung, Spalten, sichtbare Bereiche, Breiten | KRK |
| `settings.toml` | Einstellungen ohne Oberfläche, heute die Terminal-Anwendung | nur der Nutzer |
| `readers.toml` | die Leseprofile der Vorschau | nur der Nutzer |
| `note-1.txt`, `note-2.txt` | die zwei Notizzettel | KRK |

Wer die Liste am Baum nachlesen will, liest sie an ihrer Quelle und nicht hier:

```sh
awk '/pub const ALLE: \[Datei;/,/\];/' crates/krk-core/src/ablage/pfade.rs
```

**`settings.toml` und `readers.toml` legt KRK beim ersten Start an und schreibt
sie danach nie wieder.** Weder überschreibend noch ergänzend, gleich was
darinsteht. Für die Einstellungen ist das gewollt, denn ein Schreibpfad löschte
die Kommentare, die den Sinn der Datei ausmachen; ein Feld, das die eigene Datei
nicht nennt, kommt ohnehin aus der Auslieferungsfassung.

**Für die Leseprofile hat es einen Preis.**
Eine neue KRK-Fassung bringt neue Profile mit, und wer KRK schon einmal
gestartet hat, sieht nach der Installation trotzdem weiter genau die Profile von
vorher. Eine Meldung darüber gibt es nicht, und es wäre auch keine am Platz:
eine unveränderte Datei ist nicht beschädigt. Der einzige Weg an die neuen
Profile ist die vollständige Neuanlage, und dabei gehen die eigenen Zeilen
verloren. Wie man sie sich zurückholt, steht in `README.md` unter „Neue
Leseprofile übernehmen"; der wichtigste Handgriff daraus ist, die alte Datei
**beiseitezulegen** und nicht zu löschen.

Außerhalb dieses Ordners schreibt KRK an genau eine feste Stelle: die
Markdown-Ausgabe der Tastenbelegung geht nach `~/Downloads/KRK-Tastenbelegung.md`,
mit festem Namen und ohne Laufnummer.

---

## Der Dateilistenfilter

Jedes Zeichen, das im Dateifenster getippt wird und keiner Funktion gehört,
hängt an den Filtertext des **sichtbaren Tabs** an, und die Liste zeigt nur
noch, was passt. Der Filtertext gehört dem Tab: ein zweiter Tab hat seinen
eigenen, und ein Ordnerwechsel lässt ihn stehen.

Aufgenommen wird, was ein Dateiname tragen kann. Steuerzeichen fallen weg, die
Zeichen der Funktionstasten auch, und der Schrägstrich ebenfalls, denn die
Namensspalte hängt ihn an jeden Ordner als Anzeige an und kein Eintrag heißt so.
Ein abgewiesenes Zeichen lässt den Filtertext unverändert; eine begonnene Suche
übersteht damit einen Anschlag, der keine Suche sein kann.

**`*` ist das eine Sonderzeichen** und steht für eine beliebige, auch leere
Zeichenfolge. Mehrere sind erlaubt und zwei nebeneinander bedeuten dasselbe wie
eines. Kein `?`, keine Zeichenklassen, kein Entkommen. Der Vergleich bleibt an
beiden Enden ungebunden: `abc` trifft an jeder Stelle des Namens, und ein `*`
am Anfang oder am Ende verankert nichts. Groß- und Kleinschreibung sind egal,
Umlaute werden nicht gefaltet.

**`cmd+v` hängt die Zwischenablage an den Filtertext an**, aber nicht so, wie
sie dort steht. Vom Text bleibt das letzte Stück nach dem letzten Schrägstrich,
Prozentzeichen aufgelöst, und daraus fällt jedes Zeichen, das die Zeichenregel
abweist, samt dem Doppelpunkt. Mehrzeiliger Text und mehrere Dateiverweise
werden ganz abgewiesen statt halb übernommen; die Statuszeile sagt es. Der
Doppelpunkt fällt dabei **nur** beim Einfügen: wer ihn tippt, bekommt ihn, denn
ein POSIX-Name trägt ihn.

**`Esc` leert den Filtertext, aber erst im dritten Rang.** Steht ein Blatt,
schließt Esc das Blatt. Läuft eine Dateioperation, bricht Esc sie ab. Erst
danach fällt der Filter, und dann trifft er den sichtbaren Tab des **aktiven**
Dateifensters, gleich wo der Fokus gerade steht.

**Die Rückschritt-Taste bedeutet zweierlei, und die Unterscheidung ist keine
Bequemlichkeit.** Nackt liegt sie auf „In den Papierkorb räumen":

| Lage | Was die nackte Rückschritt-Taste tut |
|---|---|
| Ein Filtertext steht | das letzte Zeichen fällt weg |
| Kein Filtertext, frischer Druck | Rückfrage, dann in den Papierkorb |
| Kein Filtertext, gehaltene Taste, die den Filter eben geleert hat | gar nichts |
| Kein Filtertext, gehaltene Taste, die nie einen sah | Rückfrage, dann in den Papierkorb |

Die dritte Zeile ist der Grund für die ganze Regel: ein gehaltener Rückschritt
trägt nicht über die Grenze, an der der Filtertext leer wird. `cmd+delete` und
`f8` gehen an dieser Unterscheidung vorbei und räumen in jeder Lage.

**Die zwei Ankreuzfelder der Bereichsleiste am Fensterfuß gelten dem sichtbaren
Tab, nicht dem Fenster.** „Deep" dehnt die Suche auf den Unterbaum aus und
**steht ab Werk auf ein**; schon der erste Anschlag stößt damit den Durchlauf
an. „Content" liest zusätzlich den Text der Dateien und steht ab Werk auf aus.
Beide brauchen einen Mindest-Filtertext, bevor der Inhalt überhaupt gelesen
wird, und die Schwelle hängt an „Deep": drei Zeichen ohne tiefe Suche, fünf mit
ihr, wobei ein `*` nicht mitzählt. Keiner der beiden Schalter trägt ab Werk eine
Tastenkombination; sie stehen in der Bereichsleiste und im Hauptmenü, und wer
eine Taste will, weist sie in der Belegungsansicht zu.

---

## Die Tastaturbelegung

**F1** öffnet sie. Sie ist ein Blatt am Hauptfenster und kein eigenes Fenster.
Die Tabelle führt zwei Spalten, Funktion und Belegung, gruppiert nach
Funktionsbereichen; innerhalb einer Gruppe steht die Reihenfolge der
Auslieferungsdatei.

**Auch hier filtert das Tippen**, und ein Eingabefeld gibt es nicht: man tippt
direkt in die Tabelle. Gesucht wird über die zwei sichtbaren Spalten, also über
den Funktionsnamen und über den Tastentext, nicht über die interne Kennung.
Verglichen wird als Teilzeichenfolge, ohne Rücksicht auf Groß- und
Kleinschreibung, und es läuft keine Frist: der Suchtext setzt sich nicht von
selbst zurück. Die Eingabetaste springt zum nächsten Treffer.

**Gelöscht wird der Suchtext mit der Rücktaste, Zeichen für Zeichen.** `Esc`
bekommt hier ausdrücklich **keine** zweite Bedeutung: es schließt das Blatt,
auch wenn ein Suchtext steht. Wer die zweistufige Esc-Bedienung aus Suchfeldern
erwartet, greift daneben.

Drei Schaltflächen, jede mit der Befehlstaste, damit keine der Suche ein
Zeichen wegnimmt:

| Schaltfläche | Taste | Wirkung |
|---|---|---|
| Zuweisen | `cmd+t` | der nächste Tastendruck wird die Kombination der ausgewählten Funktion |
| Auslieferungszustand | `cmd+r` | setzt die ganze Belegung zurück |
| Fertig | `cmd+Eingabe` | schließt und sichert |

Während einer laufenden Aufnahme bricht `Esc` nur die Aufnahme ab und schließt
das Blatt nicht.

**Das Zurücksetzen fragt nicht nach, und es wirkt zunächst nur in der Ansicht.**
Die Meldungszeile bestätigt es sofort, aber auf der Platte ändert sich nichts:
geschrieben wird erst beim Verlassen des Blattes, und dann wird `keymap.toml`
mit dem Auslieferungsstand **überschrieben**, nicht gelöscht. Ohne Änderung
bleibt die Datei unberührt.

Zwei Befehle gehören dazu und liegen ab Werk auf keiner Taste; ihr Weg ist das
Hauptmenü:

- **Tastaturdefinition öffnen** stellt die eigene `keymap.toml` in die Vorschau
  und holt den Fokus dorthin, sodass `cmd+e` von dort in den Editor führt.
  Gezeigt wird die Datei des Nutzers; die Auslieferungsfassung liegt im Bündel
  überhaupt nicht als Datei, sondern ist einkompiliert.
- **Tastenbelegung als Markdown sichern** schreibt
  `~/Downloads/KRK-Tastenbelegung.md`. Funktionen ohne Kombination fallen aus
  der Ausgabe, die Ankreuzfelder der Bereichsleiste also auch.

**Die Belegungsdatei hat zwei Schreiber, und sie wissen nichts voneinander.**
KRK liest sie einmal beim Start und lädt sie im Betrieb nicht nach; eine
Handänderung wirkt deshalb erst beim nächsten Start. Und das Verlassen der
F1-Ansicht **mit einer Änderung** schreibt die ganze Arbeitskopie zurück, die
sie aus dem Stand des Starts gebaut hat, womit jede Handänderung seither fort
ist.

Eine fehlerhafte `keymap.toml` wird als Ganzes verworfen und nicht teilweise
übernommen. Unbekannte Kennung, doppelte Kennung, unlesbare Tastenschreibweise
oder ein Kombinationskonflikt: KRK meldet es beim Start und fährt mit der
Auslieferungsbelegung. Die Datei selbst bleibt dabei stehen.

---

## Editor und Vorschau

Beide sitzen am rechten Rand und teilen sich dort die Fläche mit dem
Git-Bereich. Nie sind zwei von ihnen zugleich zu sehen.

| Taste | Wirkung |
|---|---|
| `cmd+e` | der Rundweg, siehe unten |
| `f4` | den ausgewählten Eintrag im Editor öffnen |
| `opt+cmd+e` | Editor schließen, mit Nachfrage bei ungesichertem Stand |
| `opt+cmd+b` | Editor ein- und ausblenden, ohne Nachfrage, der Stand bleibt |
| `ctrl+cmd+e` | zwischen Roh- und Formatansicht wechseln |
| `shift+cmd+e` | Fokus in den Editor |
| `f3`, `cmd+y` | Vorschau ein- und ausblenden |
| `shift+cmd+y` | Fokus in die Vorschau |
| `cmd+s` | sichern |
| `cmd+plus`, `cmd+minus`, `cmd+0` | Vorschau vergrößern, verkleinern, Ausgangsgröße |

**`cmd+e` tut Verschiedenes, je nachdem, wo der Fokus steht:**

| Fokus | Wirkung |
|---|---|
| Dateifenster | der ausgewählte Eintrag geht in den Editor, wie bei `f4` |
| Vorschau | die dort angezeigte Datei geht in den Editor |
| Editor | der Editor wird geschlossen, die Vorschau kommt zurück, der Fokus geht in die Dateiliste |

**Der Rückweg schließt und blendet nicht aus.** Er gibt die Datei frei und löst
damit dieselbe Nachfrage aus wie `opt+cmd+e`. Wer den Editor nur kurz aus dem
Weg haben will und seinen Stand behalten, nimmt `opt+cmd+b`.

Der PDF-Betrachter der Vorschau kennt außer den drei Zoombefehlen keine eigenen
Tasten. Bild-auf, Bild-ab, Pos1 und Ende sind dort keine Befehle von KRK; sie
laufen an AppKit weiter und blättern.

---

## Der Git-Bereich

`opt+cmd+r` blendet ihn ein und aus, `shift+cmd+b` holt den Fokus hinein. **Ab
Werk ist er ausgeblendet**, denn ohne Repository hätte er nichts zu zeigen.

Er hat drei Flächen übereinander, deren Grenze sich ziehen lässt:

- **Der Kopf** trägt zwei Zeilen. Oben der Branchname, oder bei losgelöstem HEAD
  der Kurzhash mit dem Zusatz „(abgelöst)", oder der Satz, dass dieser Ordner in
  keinem Repository liegt. Darunter die Statuszusammenfassung, oder „noch kein
  Commit" in einem Repository ohne Commit.
- **Die Verlaufsliste** führt je Zeile Kurzbeschreibung, Autor, Datum und
  Kurzhash.
- **Die Einzelheiten** zeigen für den ausgewählten Commit die vollständige
  Nachricht, Autor mit E-Mail, Datum und den vollen Objektnamen. Der Text ist
  bewusst nicht markierbar und nicht kopierbar.

Solange keine Antwort da ist, steht nichts: kein Platzhaltertext und kein
Fortschrittsanzeiger.

**Bedient wird die Liste mit Auf- und Ab-Pfeil.** Am Ende angekommen, bewegt der
Ab-Pfeil nichts mehr, sondern lädt den nächsten Schwung Commits nach. Das
geschieht auf diesen Tastendruck hin und nicht von selbst beim bloßen Erreichen
des Endes; wie groß ein Schwung ist, sagt `VERLAUFSSCHRITT` in
`crates/krk-core/src/git/lauf.rs`.

**Der Verlauf gilt dem ganzen Repository, die Marken und die Zusammenfassung
gelten dem angezeigten Ordner.** Wer wissen will, ob im ganzen Repository alles
sauber ist, bekommt aus KRK keine Antwort darauf. Der Zuschnitt ist gemessen und
nicht geraten: eine repositoryweite Zahl kostet über einem großen Ordner ein
Vielfaches der ordnerweiten.

Die Spalte „Marke" der Dateiliste trägt die Git-Marke eines Eintrags, einen
Buchstaben: `M` geändert, `S` vorgemerkt, `N` neu, `K` in Konflikt, `U`
umbenannt. Ein unveränderter Eintrag trägt kein sechstes Zeichen, sondern eine
leere Zelle. **Diese Spalte steht ab Werk**, anders als der Bereich selbst, und
daran hängt eine Folge, die man sonst nicht sieht: nachgesehen wird, sobald
**entweder** der Bereich **oder** die Markenspalte sichtbar ist. Wer den Bereich
ausblendet und die Spalte stehen lässt, hat den Statuslauf weiterhin bei jedem
Ordnerwechsel.

**Was einen Wechsel übersteht und was nicht:**

| Anlass | Der Befund |
|---|---|
| Ordnerwechsel | fällt ganz, auch die Nachladehöhe |
| Tabwechsel | fällt ganz, die Auswahl im Verlauf eingeschlossen |
| Wechsel des aktiven Dateifensters | bleibt stehen, denn jede Tabliste hält ihren eigenen |

Vier Dinge, die ein Nutzer sonst schmerzhaft lernt:

- **KRK beobachtet `.git` nicht.** Wer in einem Terminal committet, während KRK
  denselben Ordner zeigt, sieht die Änderung erst, wenn dieser Ordner neu
  gelesen wird.
- **KRK schreibt nichts.** Kein Commit, kein Stage, kein Checkout, keine Sperre
  auf dem Repository. Gelesen wird über `gix` im eigenen Prozess, das
  `git`-Binary wird dafür nicht gerufen.
- **Submodule sind ausgenommen.** Ein geänderter Submodul-Zeiger erscheint nicht
  als Marke.
- **In einem Ordner ohne Repository bleibt es still.** Ein Satz im Kopf, eine
  leere Markenspalte, und keine Meldung in der Statuszeile. Der Bereich blendet
  sich nie selbst aus.

---

## Bereiche ein- und ausblenden

Die Fensterzeile trägt von links nach rechts die Lesezeichen- und Geräteleiste,
die zwei Dateifenster, und am rechten Rand die Vorschau, den Editor und den
Git-Bereich, die sich dort dieselbe Fläche teilen.

| Taste | Bereich |
|---|---|
| `opt+cmd+l` | Lesezeichen- und Geräteleiste |
| `opt+cmd+left` | linkes Dateifenster |
| `opt+cmd+d`, `opt+cmd+right` | zweites Dateifenster |
| `f3`, `cmd+y` | Vorschau |
| `opt+cmd+b` | Editor |
| `opt+cmd+r` | Git-Bereich |
| `ctrl+left`, `ctrl+right` | aktiven Bereich verschmälern und verbreitern |

Dieselben Schalter stehen als Ankreuzfelder in der Bereichsleiste am
Fensterfuß, zusammen mit denen für die schaltbaren Spalten der Dateilisten und
den zwei Suchschaltern „Deep" und „Content". Ein Klick dort nimmt denselben Weg
wie ein Tastendruck.

**Mindestens ein Dateifenster bleibt stehen.** Welches von beiden, ist offen:
abgewiesen wird der Befehl, der das letzte sichtbare ausblenden würde.

**Eine gespeicherte Breite ist ein Anteil und keine Zusage in Punkten.** Die
verfügbare Breite verteilt sich im Verhältnis der Wünsche aller sichtbaren
Bereiche, wobei die Mindestbreite gegen den Anteil gewinnt. Zwei Bereiche im
Verhältnis zwei zu eins stehen deshalb auch dann noch so zueinander, wenn ein
dritter dazukommt.

Die Fokusbefehle liegen auf `shift+cmd+<Buchstabe>`: `l` für die Leiste, `d`
für das Dateifenster, `y` für die Vorschau, `e` für den Editor, `b` für den
Git-Bereich. Der Rahmen um jeden Bereich sagt, wer die Tasten annimmt, und
daneben, welches Dateifenster das aktive ist.

---

## Lesezeichen

Die Leiste ganz links führt eine Liste mit zwei Überschriften. Oben die
Lesezeichen aus `bookmarks.toml`, in der Reihenfolge, in der sie dort stehen;
unten „Geräte und Orte", die bei jedem Aufbau frisch vom System kommen und nie
abgelegt werden.

| Taste | Wirkung |
|---|---|
| `cmd+d` | Lesezeichen anlegen |
| `ctrl+u` | umbenennen |
| `ctrl+delete` | löschen |
| `opt+up`, `opt+down` | in der Liste nach oben und unten verschieben |
| `shift+cmd+l` | Fokus in die Leiste |
| `shift+cmd+d` | Fokus zurück in das Dateifenster |

Löschen und Umbenennen tragen eigene Kombinationen, weil `delete` und
`shift+f6` nur im Dateifenster wirken.

**`cmd+d` legt zwei verschiedene Sorten an, je nach Fokus.** Im Dateifenster
merkt es den angezeigten Ordner. Im Editor merkt es die Zeile der Schreibmarke
als Textmarke. Beide Sorten stehen in derselben Liste, und das Sinnbild vor der
Beschriftung sagt, welche es ist.

Ein Lesezeichen, dessen Ziel fort ist, bleibt stehen und trägt den Zusatz
„(fehlt)". Die Zeile ist zusätzlich grau, aber das Wort ist das Kennzeichen,
nicht die Farbe.

---

## Leseprofile

Wählt man im Dateifenster einen **Ordner** aus, zeigt die Vorschau seine
Metadaten. Für Orte, die KRK erkennt, steht dort stattdessen eine
Zusammenfassung des Inhalts. Eine Datei und eine Verknüpfung erreichen diese
Anzeige nie.

Welche Orte erkannt werden und was in der Zusammenfassung steht, sagt
`~/Library/Application Support/KRK/readers.toml`. Die Datei erklärt sich in
ihren eigenen Kommentaren; das Folgende ist, was man wissen muss, bevor man sie
aufschlägt.

**Die Erkennung läuft in zwei Durchgängen, und der erste schlägt den zweiten.**
Zuerst wird bei jedem Profil das Muster `pfad` gegen den vollen Ordnerpfad
gehalten. Erst wenn keines trifft, wird bei jedem Profil das Muster
`kennzeichen` gegen die Namen der Einträge im Ordner gehalten. Innerhalb eines
Durchgangs gewinnt das erste Profil in Dateireihenfolge; die Reihenfolge der
Blöcke ist damit die Rangordnung.

Ein Profil trägt Zeilen, und jede Zeile trägt genau einen von vier Bausteinen:
`zaehlung` zählt Treffer über eine Ebene und lässt sich mit `muster`, `typ` und
`versteckt` einschränken, `juengste` nennt die N jüngsten Einträge, `feld` fängt
eine Gruppe aus dem Inhalt einer Datei, `vorhandensein` antwortet mit ja oder
nein. Jeder nimmt ein `ordner` entgegen, und darin darf **ein** `*` stehen, das
für ein ganzes Namensstück steht; `juengste` und `feld` nehmen keines, denn
beide lesen Dateien und brauchen deren Pfad.

**Trifft kein Profil, tritt ein eingebautes Default-Profil hinzu.** Es zählt den
Ordnerinhalt nach Dateien, Ordnern und Verknüpfungen, versteckte Einträge
mitgerechnet, und seine drei Zeilen treten **unter** die Metadaten, statt sie zu
ersetzen. Trifft dagegen ein Profil aus der Datei, ersetzt dessen Zusammenfassung
die Metadaten vollständig, und die drei Zählzeilen fallen weg. Abschalten oder
anpassen lässt sich das Default-Profil nicht; wer es anders will, schreibt ein
eigenes Profil mit denselben Zeilen.

**Ein Fehler kostet unterschiedlich viel.** Ein verschriebener Schlüssel in
einem Baustein kostet die **ganze Datei**: sie gilt als beschädigt, wird
beiseitegelegt, und KRK läuft ohne jedes Profil weiter. Ein kaputtes
Erkennungsmuster kostet das ganze Profil. Ein Fehler in einer Zeile kostet nur
diese Zeile, die dann ihre Beschriftung mit einem Platzhalter zeigt.

### Was die mitgelieferten Profile für eine fusion-Werkbank tun

Die Auslieferungsfassung bringt Profile für zwei Werkbank-Arten mit, fusion und
flight. Wie viele es insgesamt sind, zählt
`grep -c '^\[\[profil\]\]' resources/default-readers.toml`. Für eine
fusion-Werkbank sind es diese:

| Profil | erkennt | zeigt |
|---|---|---|
| die Wurzel | den Ordner mit `.fusion-setup` darin | Projekt, Einrichtungszeitpunkt, fusion-Fassung, aktive Runde, Sitzung, Zahl der Runden, offene gemeinsame Defekte |
| Projektwurzel mit fusion-Werkbank | den Ordner, der `fusion-workbench` enthält | dieselben Angaben, eine Ebene höher gelesen |
| eine Runde | einen Circle-Ordner an seiner `_._circle.md` | Zustand der Runde, den Text der Directive, ob Spec und Plan stehen, Zahl der Entscheidungen, die jüngsten Verläufe |
| alle Runden | `circles/` | Runden gesamt und je eine Zeile für vorgesehen, aktiv, kohärent geschlossen, beschränkt geschlossen, überholt und zurückgestellt, dazu die offenen Defekte aller Runden |
| der gemeinsame Speicher | `shared/` | je Unterspeicher die Zahl der Datensätze und das jüngste Datum |
| ein Speicher | einen der Unterspeicher | Zahl der Datensätze und die jüngsten davon |
| ein Defektspeicher | `issues/` | Datensätze gesamt, offen, geschlossen, zurückgestellt, und die jüngsten davon |
| der Ablagespeicher | `archive/` | Zahl der Läufe und das Datum der letzten Ablage |

Praktisch heißt das: ein Klick auf den Projektordner beantwortet „welche Runde
läuft und wie viele Defekte sind offen", ein Klick auf einen Circle-Ordner
beantwortet „was war die Directive und wie weit ist sie", und ein Klick auf
`issues/` beantwortet „wie viel liegt hier offen".

**Und die Einschränkung von oben gilt hier:** neue Profile einer neuen
KRK-Fassung kommen nicht von selbst. Der Weg dorthin steht in `README.md` unter
„Neue Leseprofile übernehmen".

---

## Der Notizzettel

`f2` oder `cmd+k` öffnet ihn. Er ist ein Blatt mit zwei Zetteln als
anklickbaren Tabs über einer bearbeitbaren Textfläche, ohne Zeilennummern, ohne
Hervorhebung und ohne Suche. `Esc` oder „Fertig" schließt ihn.

**Geschrieben wird an vier Momenten, und jeder ist ein Weg aus dem Zettel
heraus:** der Klick auf den anderen Tab, das Schließen des Blattes, das
Schließen des Fensters mit `shift+cmd+w`, und das Beenden von KRK. Geschrieben
wird dabei jeder abweichende Zettel und nicht nur der offene. Steht ein Zettel
so da wie seine Datei, wird nichts geschrieben.

Zwei Regeln, die zusammengehören:

- **Eine gescheiterte Sicherung wirft den Stand nicht weg.** Der Zettel bleibt
  abweichend, der nächste Moment versucht es erneut, und der Grund geht in die
  Statuszeile, damit niemand darauf baut, dass sein Text auf der Platte liegt.
- **Der getippte Stand gewinnt beim Öffnen.** Weicht ein Zettel von seiner Datei
  ab, bleibt sein gehaltener Text stehen, und das frisch Gelesene wird
  verworfen. Der Preis ist benannt und angenommen: wer einen abweichenden Zettel
  öffnet, sieht nicht, was eine zweite Instanz von KRK inzwischen in die Datei
  geschrieben hat.

Dass `Esc` den Zettel schließt und nicht etwa eine Dateioperation abbricht,
hängt daran, dass seine Textfläche bei KRKs Befehlsprüfung bewusst **nicht**
angemeldet ist. Ihre Tasten gehören AppKit, und genau das ist hier erwünscht.
