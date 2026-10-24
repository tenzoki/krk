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

## Die Sprache

KRK spricht Deutsch, Französisch und Englisch, und welche Sprache gilt,
entscheidet macOS: aus der Sprachliste des Systems, oder aus einer eigenen Wahl
für KRK. Eine Einstellung in KRK selbst gibt es dafür nicht. Steht in der Liste
keine der drei Sprachen, spricht KRK Englisch.

Eine andere Sprache als die des Systems wählt man in den Systemeinstellungen
unter „Allgemein“ › „Sprache & Region“, im Abschnitt „Apps“: mit „+“ KRK
hinzufügen und die Sprache auswählen. Die Wahl wirkt beim nächsten Start; ein
laufendes KRK wechselt nicht, weil macOS die Sprache eines Programms beim
Start festlegt.

Mit derselben Wahl beschriftet macOS, was es selbst stellt: die Größenangaben
in der Dateiliste, in der Vorschau und in der Statuszeile, das Kontextmenü
eines Textfeldes, das Über-Fenster und die Rückfrage beim ersten Zugriff auf
einen geschützten Ordner. Menüs, Blätter, Statuszeile und Vorschau von KRK
folgen ihr, und diese beiden Hälften sind so immer einig.

Das Datumsformat in der Dateiliste und in der Vorschau folgt
dagegen voraussichtlich nicht der Sprachwahl, sondern der Region unter
„Allgemein“ › „Sprache & Region“: wer die Region Deutschland eingestellt und
KRK auf Französisch gestellt hat, sieht dann ein deutsches Datum neben
französischen Menüs. Am gebauten Programm geprüft ist das noch nicht.

Die französischen und die englischen Texte hat ein Agent übersetzt; wer eine
Stelle findet, die sich wie eine Übersetzung liest oder etwas anderes sagt als
die deutsche, meldet sie, denn durchgesehen ist bis dahin nur das Deutsche.

---

## Wo KRK seine eigenen Dateien ablegt

Alles liegt in `~/Library/Application Support/KRK/`, außerhalb des Bündels:

| Datei | Inhalt | Wer schreibt |
|---|---|---|
| `keymap.toml` | die eigene Tastenbelegung | KRK beim Verlassen der F1-Ansicht mit einer Änderung, und der Nutzer von Hand; von selbst legt KRK sie nie an, und „Auf Werkseinstellungen zurücksetzen…“ legt sie beiseite |
| `bookmarks.toml` | die Lesezeichen | KRK |
| `session.toml` | Ordner, Tabs, Sortierung, Spalten, sichtbare Bereiche, Breiten, die Richtung der Termintabelle | KRK |
| `settings.toml` | Einstellungen ohne Oberfläche, heute die Terminal-Anwendung und der Ort des Notizordners | der Nutzer; KRK beim ersten Start, beim Zurücksetzen auf Werkseinstellungen und dazwischen allein den Ort des Notizordners, über „Ort wählen…“ |
| `readers.toml` | die Leseprofile der Vorschau | der Nutzer; KRK beim ersten Start und beim Zurücksetzen auf Werkseinstellungen |
| `reported.toml` | für welche Fassung die Neuerungen an den eigenen Dateien gemeldet sind, und ob die alten Zettel schon nach `notes.txt` übernommen sind | KRK |

Wer die Liste am Baum nachlesen will, liest sie an ihrer Quelle und nicht hier:

```sh
awk '/pub const ALLE: \[Datei;/,/\];/' crates/krk-core/src/ablage/pfade.rs
```

**Die Notizen liegen nicht hier, sondern im Notizordner, ab Werk `~/krkhome/`**
(siehe „Der Notizordner“ weiter unten). Ein Löschwerkzeug, das den Ablageordner
mitnimmt, lässt den Notizordner stehen; im Ablageordner selbst lässt KRK ihn
deshalb gar nicht erst einstellen. Wer KRK aus der Zeit des Notizblatts kennt,
findet im Ablageordner vielleicht noch `note-1.txt` und `note-2.txt`. KRK hat
sie einmal nach `notes.txt` im Notizordner übernommen und liest und schreibt
sie seither nicht mehr. Sie liegen ohne Leser da und dürfen bleiben oder gehen.

**`readers.toml` legt KRK beim ersten Start an und schreibt sie danach allein
auf ausdrücklichen Befehl**, beim Zurücksetzen auf Werkseinstellungen (siehe
unten). Von selbst schreibt KRK sie nie, weder überschreibend noch ergänzend,
gleich was darinsteht.

**`settings.toml` legt KRK ebenso beim ersten Start an und schreibt sie danach
an zwei Stellen.** „Ort wählen…“ im Menü „Home“ ersetzt darin allein den
Wert von `notizordner`; fehlt der Schlüssel, hängt es ihn mit einer
Kommentarzeile ans Ende. Jede andere Zeile bleibt Zeichen für Zeichen stehen,
jeder Kommentar eingeschlossen. „Auf Werkseinstellungen zurücksetzen…“ schreibt
dagegen die ganze Datei neu, als Auslieferungsfassung samt ihren Kommentaren, in
die allein der bisherige Wert von `notizordner` eingesetzt ist; die alte Fassung
legt es vorher beiseite. Ist die Datei beschädigt oder ein symbolischer
Verweis, schreibt KRK sie auf keinem der beiden Wege (siehe „Der Ort des
Notizordners“). Ein Feld, das die eigene Datei nicht nennt, kommt ohnehin aus
der Auslieferungsfassung.

**Eine neue KRK-Fassung bringt Einträge mit, die in den eigenen Dateien
fehlen.** Betroffen sind die drei von Hand gepflegten `keymap.toml`,
`settings.toml` und `readers.toml`. KRK meldet das beim ersten Start einer neuen
Fassung in der Statuszeile: je betroffener Datei die Zahl der Einträge, die nur
die Auslieferungsfassung führt, dahinter der Ordner, in dem die eigenen Dateien
liegen. Genannt sind nur Dateien mit einem Unterschied, und die Zeile kommt
**einmal je Fassung** — der zweite Start derselben Fassung zeigt sie nicht mehr.

Das Einzelne zeigt **„Neuerungen anzeigen"**, im Hauptmenü unter „Anwendung"
und ab Werk auf `opt+cmd+i`. Das Blatt nennt je Datei ihren vollen Pfad, die
Namen der Einträge nur in der Auslieferungsfassung, die Namen der Einträge nur
in der eigenen Datei und einen Satz darüber, was der Unterschied an dieser Datei
kostet. Wer eine `keymap.toml` von vor dieser Fassung hat, findet den Befehl bei
sich ohne Kürzel am Fuß der Gruppe „Anwendung": das ist zugleich das
nächstliegende Beispiel für den Fall, den das Blatt beschreibt.

**Der Preis ist bei den drei Dateien verschieden:**

| Datei | Was ein fehlender Eintrag kostet |
|---|---|
| `readers.toml` | Alles. Das Profil gibt es für KRK nicht, und die Vorschau zeigt an dem Ort, den es erkannt hätte, weiter die Metadaten. |
| `settings.toml` | Nur den erklärenden Kommentarblock. Die Einstellung selbst wirkt bereits, mit dem Wert aus der Auslieferungsfassung. |
| `keymap.toml` | Die ausgelieferten Tastenkombinationen. KRK hängt die Funktion unbelegt an: in ihrer Gruppe hinter die Funktionen, die die eigene Datei dort nennt, und fehlen mehrere, untereinander in der Folge der Auslieferung. Über das Hauptmenü bleibt sie erreichbar. |

**Der erste Weg ist der Befehl „Auf Werkseinstellungen zurücksetzen…“** im
Menü „KRK“, unmittelbar unter „Neuerungen anzeigen“. Ab Werk trägt er keine
Tastenkombination; in der F1-Ansicht steht er in der Gruppe „Anwendung“ und
lässt sich dort belegen wie jede andere Funktion. Er setzt alle drei Dateien in
einem Zug zurück, nach einer Rückfrage, in der „Abbrechen“ vorbelegt ist:
`return` und `esc` brechen ab, `cmd+return` bestätigt, ebenso ein Klick auf
„Zurücksetzen“. Danach gilt:

- Jede der drei, die dastand, liegt unverändert im Ablageordner unter ihrem
  Namen mit angehängtem Zeitstempel, etwa `readers.toml.260929-0815`. Ein
  zweiter Lauf in derselben Minute hängt `-2` an und überschreibt keine frühere
  Sicherung. Gelöscht wird keine. Die Statuszeile nennt den vollen Pfad jeder
  Sicherung und jede Datei, die nicht dastand.
- `readers.toml` steht als Auslieferungsfassung da, samt allen Kommentaren.
- `settings.toml` steht als Auslieferungsfassung da, mit einer Ausnahme: der
  Wert von `notizordner` bleibt der bisherige. **Der Notizordner und alle
  Dateien darin bleiben unberührt**; Notizen, Aufgaben, Termine und
  `secrets.txt` liegen danach am selben Ort wie vorher.
- `keymap.toml` fehlt, und es gilt die mitgelieferte Tastenbelegung. **Alle
  eigenen Tastenzuweisungen gehen damit aus dem Betrieb**; sie liegen danach
  allein in der Sicherung.
- KRK liest den neuen Stand sofort ein, ohne Neustart: Vorschau,
  Tastenbelegung, Hauptmenü und Terminal folgen ihm.

Die eigenen Zeilen holt man sich aus der Sicherung zurück, und das Blatt aus
„Neuerungen anzeigen" sagt unter „Nur in Ihrer Datei", welche das sind.

**Ist `settings.toml` beschädigt oder nicht lesbar, setzt der Befehl nichts
zurück**, und die Rückfrage geht gar nicht erst auf. Bei einer beschädigten
Datei sagt die Statuszeile:

> settings.toml ist zuerst von Hand zu berichtigen, KRK schreibt sie so nicht:
> … Nichts ist zurückgesetzt.

Aus einer solchen Datei lässt sich der Ort des Notizordners nicht übernehmen,
und `~/krkhome` an seine Stelle zu setzen, wäre ein stiller Umzug. Also zuerst
die Datei berichtigen, dann den Befehl noch einmal wählen. Ist eine der drei ein
symbolischer Verweis, bricht der Befehl ebenso vor jeder Änderung ab und nennt
die Datei.

**Der zweite Weg geht von Hand und nimmt sich eine Datei einzeln vor.** Für
`readers.toml` und `settings.toml` ist er derselbe: KRK beenden, die betroffene
Datei **beiseitelegen** und nicht löschen, KRK starten. Sie entsteht neu aus der
Auslieferungsfassung, samt allen Kommentaren darin. **Eine so neu entstandene
`settings.toml` nennt wieder den Vorgabeort `~/krkhome`**, anders als nach dem
Befehl; davon handelt „Der Ort des Notizordners“ weiter unten.

**Bei `keymap.toml` greift dieser Handgriff nicht**, denn KRK legt sie beim
Start nicht an: beiseitegelegt bleibt sie fort, und KRK arbeitet mit der
Auslieferungsbelegung. Steht die Datei gar nicht erst da, ist nichts zu tun.
Steht sie da und soll allein die Belegung zurück, führt der Weg durch die
Anwendung: in der F1-Ansicht **Auslieferungszustand** (`cmd+r`), und das
Verlassen der Ansicht schreibt `keymap.toml` mit dem Auslieferungsstand; die
eigenen Zuweisungen sind danach fort, wer sie behalten will, legt die Datei
vorher beiseite. Eine einzelne neue Funktion belegt man ohne jedes Zurücksetzen:
in der F1-Ansicht wählen, **Zuweisen** (`cmd+t`), die Kombination drücken und
die Ansicht verlassen. Das Zurücksetzen auf Werkseinstellungen wäre dafür der
falsche Weg, denn es nimmt alle eigenen Zuweisungen aus dem Betrieb.

Im Einzelnen stehen beide Wege in `README.md` unter „Neuerungen an den eigenen
Dateien übernehmen".

Außerhalb dieses Ordners schreibt KRK an genau eine feste Stelle: die
Markdown-Ausgabe der Tastenbelegung geht nach `~/Downloads/KRK-Tastenbelegung.md`,
mit festem Namen und ohne Laufnummer.

---

## Ordner, die macOS sperrt

Manche Ordner gibt macOS nur Programmen mit **Festplattenvollzugriff** heraus,
etwa `~/Pictures/Photos Library.photoslibrary`. Ohne ihn bleibt die Dateiliste
leer, und die Statuszeile meldet „macOS sperrt den Zugriff auf …“. Abhilfe:
Systemeinstellungen › Datenschutz & Sicherheit › Festplattenvollzugriff, KRK
einschalten und KRK neu starten. Fehlt dagegen das gewöhnliche Leserecht des
Ordners, meldet die Statuszeile wie bisher, dass er sich nicht lesen ließ; der
Festplattenvollzugriff hilft dann nicht.

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

**`cmd+f` hängt die Zwischenablage an den Filtertext an**, aber nicht so, wie
sie dort steht. Vom Text bleibt das letzte Stück nach dem letzten Schrägstrich,
Prozentzeichen aufgelöst, und daraus fällt jedes Zeichen, das die Zeichenregel
abweist, samt dem Doppelpunkt. Mehrzeiliger Text und mehrere Dateiverweise
werden ganz abgewiesen statt halb übernommen; die Statuszeile sagt es. Der
Doppelpunkt fällt dabei **nur** beim Einfügen: wer ihn tippt, bekommt ihn, denn
ein POSIX-Name trägt ihn.

Dieselbe Taste sucht im Editor im Text, und das ist kein Widerspruch: welche
der beiden Bedeutungen greift, entscheidet der Fokus. `cmd+v` tut im
Dateifenster nichts; die Kombination ist für das Einfügen einer Datei
reserviert, das eine spätere Fassung bringt. Bis zur Fassung vom 260907 lag das
Einfügen in den Filtertext auf `cmd+v`.

**`Esc` leert den Filtertext, aber erst im letzten Rang.** Steht ein Blatt,
schließt Esc das Blatt. Ist im Editor eine Zelle der Aufgabentabelle offen,
verwirft Esc das dort Getippte (siehe „Der Notizordner“). Läuft eine
Dateioperation, bricht Esc sie ab. Erst danach fällt der Filter, und dann trifft er den sichtbaren Tab des **aktiven**
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

**An einem Ort ohne Papierkorb löscht derselbe Befehl endgültig, nach einer
eigenen Rückfrage.** Das betrifft jeden Ordner auf einem Server (siehe „Ordner
auf einem Server“ unter „Verbindungen“), jeden Ordner in einem Archiv (siehe
„Archive im Dateifenster“) und eine eingehängte Freigabe, die
keinen Papierkorb führt, etwa über `smb`. Die Rückfrage sagt dann „endgültig
löschen“ und dass sich nichts zurückholen lässt, die Schaltfläche heißt
„Endgültig löschen“, und vorbelegt ist „Abbrechen“: Return und Esc brechen ab,
`cmd+return` löscht. Bis zur Fassung vom 261016 meldete KRK an solchen Orten
nur, dass nichts gelöscht wurde. Ein Ort mit Papierkorb geht unverändert den
Weg in den Papierkorb. **Kann KRK nicht feststellen, ob ein Ort einen
Papierkorb führt**, etwa weil sich der angezeigte Ordner nicht auflösen lässt,
erscheint dieselbe endgültige Rückfrage, mit eigenem Text: sie sagt, dass KRK
es nicht feststellen konnte und deshalb endgültig gelöscht wird, und behauptet
keinen Ort ohne Papierkorb. Schaltfläche und Vorbelegung sind dieselben, und
`cmd+return` löscht endgültig. Bis zur Fassung vom 261019 löschte KRK in
diesem Fall nichts und sagte das in der Statuszeile.

**Die zwei Ankreuzfelder der Bereichsleiste am Fensterfuß gelten dem sichtbaren
Tab, nicht dem Fenster.** „Deep" dehnt die Suche auf den Unterbaum aus und
**steht ab Werk auf ein**. „Content" liest zusätzlich den Text der Dateien und
steht ab Werk auf aus. **Beide greifen ab drei getippten Zeichen**, ein `*`
zählt dabei nicht mit: unter drei Zeichen filtert KRK flach und allein über die
Namen des angezeigten Ordners, ab drei Zeichen steigt es in den Unterbaum ab und
liest — mit „Content" — den Text. Die Liste springt am dritten Zeichen deshalb
sichtbar um; das ist so gewollt und keine Störung. Keiner der beiden Schalter
trägt ab Werk eine Tastenkombination; sie stehen in der Bereichsleiste und im
Hauptmenü, und wer eine Taste will, weist sie in der Belegungsansicht zu.

---

## Eine Datei duplizieren

**Der Eintrag steht allein im Menü der rechten Maustaste über der Dateiliste**
und heißt „Duplizieren…“, zwischen „Unzip“ und „Auswerfen“. Eine Taste
trägt er nicht, und im Hauptmenü steht er nicht; in der Belegungsansicht (F1)
findet man ihn deshalb auch nicht, denn er ist keine Funktion, die sich einer
Taste zuweisen ließe.

**Er wirkt auf genau eine gewöhnliche Datei.** Betroffen ist, worauf auch `f5`
und `f6` wirken: die markierten Einträge, sonst die ausgewählte Zeile, und ein
Rechtsklick auf eine unmarkierte Zeile rückt die Auswahl vorher auf diese
Zeile. Ist das keine einzelne gewöhnliche Datei, geht kein Blatt auf, und die
Statuszeile sagt, warum:

| Lage | Statuszeile |
|---|---|
| nichts markiert und nichts ausgewählt | „nichts zu duplizieren: nichts markiert und nichts ausgewählt“ |
| mehrere Einträge markiert | „nichts zu duplizieren: es sind mehrere Einträge markiert, und dupliziert wird genau eine Datei“ |
| ein Ordner | „nichts zu duplizieren: „<name>“ ist ein Ordner, und dupliziert wird allein eine gewöhnliche Datei“ |
| eine Verknüpfung | „nichts zu duplizieren: „<name>“ ist eine Verknüpfung, und dupliziert wird allein eine gewöhnliche Datei“ |

Was die Liste als Datei führt und keine ist, etwa eine benannte Röhre, weist
erst der Vorgang selbst ab, mit dem Grund „keine gewöhnliche Datei“ in der
Liste der übersprungenen Einträge.

**Das Blatt fragt „Wie soll das Duplikat heißen?“** Im Feld steht der alte
Name, ganz ausgewählt: das erste getippte Zeichen ersetzt ihn samt Endung, und
wer nur den Stamm ändern will, setzt die Einfügemarke vorher mit einer
Pfeiltaste oder einem Klick ins Feld. Die Schaltflächen heißen „Duplizieren“
und „Abbrechen“; die Eingabetaste bestätigt, `Esc` schließt das Blatt, und
angelegt ist dann nichts.

**Was kein Name ist, lässt das Blatt nicht bestätigen.** Bei einem leeren
Namen, einem Namen mit Schrägstrich und bei `.` und `..` bleibt „Duplizieren“
abgeschaltet, die Eingabetaste tut nichts, und die Zeile unter dem Feld nennt
den Grund: „der Name ist leer“, „ein Name darf keinen Schrägstrich enthalten“,
„'.' und '..' sind keine Namen“. Leerzeichen an beiden Enden des Namens fallen
weg.

**Ein vergebener Name überschreibt nichts.** Ob der Name frei ist, prüft KRK
nicht vorher; das entscheidet der Datenträger beim Anlegen. Ist der Name
vergeben, geht dasselbe Blatt erneut auf, mit dem zuletzt versuchten Namen im
Feld und dem Grund „es gibt schon einen Eintrag namens „<name>““ darunter. Das
gilt auch für den unveränderten alten Namen, den häufigsten Fall, wenn die
Eingabetaste zu früh fällt, und auf einem Mac-Datenträger in der Vorgabe auch
für einen Namen, der sich vom vorhandenen nur in der Groß- und Kleinschreibung
unterscheidet. Das Blatt geht so oft wieder auf, wie ein vergebener Name
genannt wird; „Abbrechen“ oder `Esc` beenden den Vorgang, und die vorhandene
Datei ist danach unverändert.

**Das Duplikat entsteht im selben Ordner wie die Datei**, und danach steht die
Auswahl auf ihm. Blendet ein Filtertext das Duplikat aus, bleibt die Auswahl,
wo sie war.

**Das Duplizieren ist ein Vorgang wie das Kopieren.** Die Statuszeile zeigt
„Duplizieren“ mit dem Fortschritt, und `Esc` bricht ab, ohne eine halbe Datei
zu hinterlassen. Jeder andere Fehlschlag, etwa fehlende Rechte, ein voller
Datenträger oder eine inzwischen verschwundene Datei, öffnet das Blatt nicht
erneut, sondern steht mit seinem Grund in der Liste der übersprungenen
Einträge, die nach dem Vorgang aufgeht.

---

## Archive im Dateifenster

**Ein Archiv lässt sich im Dateifenster betreten wie ein Ordner.** Betreten
werden Dateien mit diesen Endungen, ohne Rücksicht auf Groß- und
Kleinschreibung:

| Endung | Was das Fenster zeigt | Schreiben |
|---|---|---|
| `.zip` | den Inhalt des Zip | ja |
| `.tar` | den Inhalt des tar | ja |
| `.tar.gz`, `.tgz` | den Inhalt des gepackten tar | ja |
| `.gz` (ohne `.tar` davor) | eine einzige Datei | nein, allein lesbar |

Andere Formate betritt KRK nicht, auch nicht Abkömmlinge des Zip wie `.jar`,
`.docx` oder `.epub`, und ebenso wenig `.tar.bz2`, `.tar.xz`, `.7z` oder
`.rar`. **Ob eine Datei betreten wird, entscheidet allein ihre Endung.** Trägt
eine Datei die Endung, ist aber kein solches Archiv, betritt KRK sie trotzdem
und sagt in der Statuszeile, dass sie kein lesbares Archiv im genannten Format
ist, und warum.

**Hinein mit Pfeil rechts oder einem Doppelklick, hinaus mit Pfeil links.**
Pfeil links (oder `cmd+up`) geht im Archiv einen Ordner höher und an der
obersten Ebene zurück in den Ordner, in dem das Archiv liegt; dort steht die
Auswahl dann auf der Archivdatei. Eine Verknüpfung auf ein Archiv wird ebenso
betreten, wenn ihr eigener Name oder der Name ihres Ziels die Endung trägt,
also auch `aktuell`, das auf `paket-1.2.zip` zeigt. **`return` öffnet ein Archiv weiter mit dem Standardprogramm**, also
mit dem Archivprogramm des Systems, so wie `return` einen Ordner im Finder
öffnet und nicht betritt. Der Fenstertitel nennt das Archiv und den Ordner
darin als `/pfad/zum/archiv.zip:/ordner/darin`.

Ein großes Archiv zeigt seinen Inhalt erst, wenn KRK sein Inhaltsverzeichnis
gelesen hat; bei einem `.tar.gz` heißt das, einmal durch das ganze Archiv. KRK
bleibt dabei bedienbar. Danach merkt es sich das Verzeichnis, solange sich die
Datei nicht ändert, und das Betreten eines Unterordners kostet nichts mehr.

**Eine `.gz`-Datei zeigt genau einen Eintrag**: ihren Namen ohne `.gz`
(`bericht.txt.gz` zeigt `bericht.txt`), mit der entpackten Größe. Diesen
Eintrag kann man herauskopieren, und er kommt dabei entpackt an. Mehr geht in
einer `.gz`-Datei nicht: Hineinkopieren, Verschieben, Umbenennen, Anlegen und
Löschen sind dort ausgegraut, ein Kopieren aus dem anderen Fenster in ein
solches Fenster meldet „Dieses Archiv ist allein lesbar.“, und einen Abwurf aus
dem Finder nimmt es nicht an.

**Was im Archiv geht:**

- **Herauskopieren** (`f5`) ins andere Dateifenster, lokal oder auf einen
  Server. Ordner kommen samt Inhalt heraus, mit Rechten und Änderungszeit;
  eine Verknüpfung im Archiv wird am Ziel wieder eine Verknüpfung. Für einen
  vorhandenen Namen am Ziel gilt dasselbe Blatt wie bei jedem Kopieren.
- **Hineinkopieren** (`f5`) aus dem anderen Dateifenster, lokal, von einem
  Server oder aus einem anderen Archiv, und durch Ziehen aus dem Finder auf
  das Archivfenster; auf eine Ordnerzeile gezogen, landet es in diesem Ordner
  im Archiv, wie in einem Ordner auf dem Mac. Eine benannte Röhre, ein Socket
  oder ein Gerät in einem Ordner auf dem Mac kommt nicht ins Archiv; die
  Abschlussliste nennt es, der Rest kommt an, und ein Verschieben lässt den
  Ordner dann stehen. Steht der Name im Archiv schon, fragt dasselbe Blatt wie
  lokal: „Überschreiben“ ersetzt den Eintrag, bei einem Ordner samt Inhalt,
  „Umbenennen“ nimmt einen freien Namen, „Überspringen“ lässt aus, und Ordner
  auf Ordner führt ohne Frage zusammen. **Alle diese Fragen kommen, bevor KRK
  das erste Byte schreibt.**
- **Verschieben** (`f6`) in jede dieser Richtungen. Innerhalb desselben
  Archivs hängt KRK nur die Namen um. **Die Quelle wird erst entfernt, wenn ihr
  Inhalt vollständig am Ziel steht**: beim Verschieben in ein Archiv also erst
  nachdem das Archiv fertig geschrieben ist. **Beim Verschieben aus einem
  Archiv entfernt KRK die Einträge dort am Ende, in einem Umbau je
  Quellarchiv**, und allein die, die vollständig angekommen sind. Scheitert
  dieser letzte Umbau, stehen die Einträge doppelt da, am Ziel und weiter im
  Archiv, und nicht nirgends; die Abschlussliste nennt sie. Dasselbe gilt,
  wenn ein anderes Programm das Archiv ersetzt hat, während KRK kopierte:
  dann entfernt KRK darin nichts.
  **Aus einem Ordner auf dem Mac, der ins Archiv verschoben wird, entfernt KRK
  allein, was beim Zusammenstellen darin stand**, Datei für Datei und nur,
  wenn sie seitdem unverändert ist. Was während einer Rückfrage oder des
  Schreibens in den Ordner kommt oder sich dort ändert, bleibt stehen, mit
  ihm der Ordner, und die Abschlussliste nennt beides.
  **Das Archiv selbst oder einen Ordner, in dem es liegt, nimmt das Archiv
  nicht auf**, weder beim Kopieren noch beim Verschieben; die Abschlussliste
  sagt „das Ziel liegt in der Quelle“.
- **Umbenennen** (`shift+f6`), **neuer Ordner** (`f7`) und **neue Datei**
  (`ctrl+cmd+n`). Ein vergebener Name wird nie überschrieben, und die
  Statuszeile meldet ihn wie in einem lokalen Ordner.
- **Löschen ist endgültig**, denn ein Archiv hat keinen Papierkorb. `delete`,
  `cmd+delete` und `f8` führen gleich zur Rückfrage „endgültig löschen“, die
  den Ort als `/pfad/zum/archiv.zip:/ordner/darin` nennt; vorbelegt ist
  „Abbrechen“.
- **Der Filtertext** filtert über die Namen des angezeigten Ordners im Archiv.

**Wie KRK in ein Archiv schreibt, und was das kostet.** KRK öffnet das Archiv
nie zum Schreiben. Jede Änderung schreibt ein vollständiges neues Archiv unter
einem Zwischennamen `.krk-umbau-…` neben das alte, in denselben Ordner, und
ersetzt das alte erst ganz am Ende in einem Schritt. Daraus folgt:

- **Bei einem Abbruch (`Esc`) oder einem Fehler bleibt das Archiv unverändert**,
  auch mit den Einträgen, die bis dahin schon geschrieben waren; der
  Zwischenname verschwindet wieder. Das ist anders als beim lokalen Kopieren,
  nach dem stehen bleibt, was fertig ist. Die Abschlussliste sagt „das Archiv
  ist unverändert geblieben“. Kommt der Abbruch erst, nachdem das neue Archiv
  an die Stelle des alten getreten ist, hält er nichts mehr auf: der Vorgang
  endet als fertig, und die Abschlussliste sagt „der Abbruch kam erst nach
  dem Ersetzen; das Archiv ist geändert“.
- **Jede Änderung schreibt das ganze Archiv neu**, auch wenn sie nur einen
  Namen ändert. Bei einem großen `.tar.gz` heißt das einmal ganz entpacken und
  ganz neu packen; ein Zip kopiert die unveränderten Einträge, ohne sie neu zu
  packen. Der Ordner des Archivs braucht dafür vorübergehend so viel freien
  Platz wie das neue Archiv, und KRK braucht dort Schreibrecht; das
  Schreibrecht an der Archivdatei allein genügt nicht. **Ohne Schreibrecht an
  der Archivdatei ändert KRK sie nicht**, auch wenn der Ordner beschreibbar
  ist: eine Datei, die mit `chmod a-w` oder über „Informationen“ gegen
  Änderungen gesperrt ist, bleibt, wie sie ist, und die Abschlussliste sagt
  „die Archivdatei ist schreibgeschützt“.
- **Beim Ersetzen geht verloren, was an der alten Datei hing**: Finder-Etiketten
  und andere erweiterte Attribute, Zugriffslisten und das Erstellungsdatum der
  Archivdatei. Die Rechte der Datei bleiben. Ein harter Verweis auf das Archiv
  zeigt danach weiter auf die alte Fassung. Eine Verknüpfung auf das Archiv
  bleibt eine Verknüpfung, geschrieben wird an ihrem Ziel.
- **Ändert ein anderes Programm das Archiv, während KRK es umbaut**, ersetzt
  KRK es nicht und meldet „das Archiv hat sich während der Änderung von außen
  geändert“. Ganz schließen lässt sich das nicht: was ein anderes Programm im
  allerletzten Augenblick vor dem Ersetzen schreibt, geht verloren.
- **Stürzt KRK mitten im Umbau ab**, bleibt der Zwischenname `.krk-umbau-…`
  als versteckte Datei im Ordner des Archivs liegen; KRK räumt ihn nicht auf,
  und man kann ihn löschen.
- **Ein Archiv mit Einträgen, die die Liste verdeckt, ändert KRK nicht**: der
  frühere von zwei Einträgen gleichen Namens, eine Datei, die einem
  gleichnamigen Ordner weichen muss, und ein Eintrag hinter einer Verknüpfung
  im selben Archiv. Jede Änderung endet dort mit „das Archiv trägt Einträge,
  die die Liste verdeckt, etwa zwei gleichen Namens, und wird nicht geändert“;
  herauskopieren lässt sich, was die Liste zeigt. Sonst ließe jede Änderung
  die verdeckten Einträge fallen, auch an ganz anderer Stelle, oder ein
  Löschen holte den früheren wieder hervor.
- **Ein tar mit harten Verweisen ändert KRK ebenso nicht** und meldet „das tar
  trägt harte Verweise und wird nicht geändert“: ein harter Verweis hat keine
  eigenen Daten, und ein Löschen oder Umbenennen seines Ziels ließe ihn ins
  Leere zeigen.
- **Eine Datei über 4 GiB** kommt in ein Zip in der ZIP64-Form, beim
  Hineinkopieren wie beim Zippen über das Kontextmenü.

**Was im Archiv nicht geht, oder nur zum Teil:**

- **Ausgegraut** sind dort „Terminal öffnen“, „Mit dem Standardprogramm
  öffnen“, beide Pfadkopien, „Bearbeiten“, das Stapelumbenennen und die
  Schalter „Deep“ und „Content“, dazu mit dem Fokus im Dateifenster „Teilen“,
  „Lesezeichen anlegen“ und der Rundweg (`cmd+e`). In einer `.gz`-Datei kommen
  Verschieben, Papierkorb, Umbenennen und die zwei Anlegebefehle dazu.
- **Das Kontextmenü graut nichts aus**, sondern meldet: „Öffnen mit“, Zippen,
  Entpacken, Duplizieren, „Im Finder öffnen“, „Im Finder anzeigen“ und
  „Auswerfen“ sagen in der Statuszeile „Das geht in einem Archiv nicht.“
  Dasselbe sagen `cmd+c` und `cmd+x`, die aus einem Archiv keine
  Dateiverweise ablegen, und ein Doppelklick auf eine Datei im Archiv. Aus
  einem Archivfenster lässt sich nichts in den Finder ziehen.
- **Die Vorschau** zeigt eine Datei im Archiv wie dieselbe Datei auf dem Mac:
  Text bis 1 MB, Markdown gerendert, Quelltext eingefärbt, ein Bild als Bild,
  und eine `.jsonl` Datensatz für Datensatz, über 1 MB mit ihrem Anfang. KRK
  liest dafür den Anfang des Eintrags aus dem Archiv und entpackt nichts auf
  die Platte; bei einem `.tar.gz` heißt das, bis zu diesem Eintrag durch das
  Archiv. Für einen verschlüsselten Eintrag, ein PDF, einen Text über 1 MB und
  alles, was kein Text ist, zeigt sie Name, Größe, Änderungszeit und Typ aus
  der Liste, ebenso für einen Ordner. Bearbeiten lässt sich eine Datei im
  Archiv nicht; dafür kopiert man sie heraus. Der Git-Bereich fragt einen
  Ordner in einem Archiv nicht.
- **Ein Archiv in einem Archiv** lässt sich nicht betreten; die Statuszeile
  sagt „erst herauskopieren“, und herauskopiert lässt es sich betreten.
- **Ein Archiv auf einem Server** lässt sich nicht betreten; die Statuszeile
  sagt „erst herunterkopieren“.
- **Verschlüsselte Zips** zeigt KRK mit allen Einträgen, kann aber keinen
  verschlüsselten Eintrag herauskopieren und schreibt in ein solches Archiv
  nicht hinein: jede Änderung endet mit „das Archiv trägt verschlüsselte
  Einträge und wird nicht geändert“. Ebenso nicht heraus kommen Einträge, die
  mit einem Verfahren gepackt sind, das KRK nicht entpackt (etwa bzip2, LZMA,
  zstd oder Deflate64), und aus einem tar harte Verweise und Gerätedateien; die
  Abschlussliste nennt jeden mit seinem Grund.
- **Unsichere Namen**, etwa mit `..`, zeigt KRK nicht; die Statuszeile nennt
  ihre Zahl. Ein Name mit `/` am Anfang erscheint ohne ihn. Ein solcher
  Eintrag hält eine Änderung nicht auf und bleibt dabei unverändert im Archiv.
- **Ein tar mit einem Namen oder einer Zusatzangabe über 16 MiB** gilt als
  kaputt und lässt sich nicht betreten; die Statuszeile nennt den Grund. So
  kann ein kleines `.tar.gz` KRK nicht dazu bringen, Gigabytes in den Speicher
  zu lesen.
- **Ein Zip ohne Zeitangaben**, wie es manche Programme unter Windows packen,
  zeigt seine Einträge mit dem Datum 1. Januar 1970.

**Änderungen von außen erscheinen von selbst**: ein Archivtab beobachtet den
Ordner, in dem die Archivdatei liegt, über eine Verknüpfung betreten auch den
Ordner ihres Ziels, und liest das Archiv neu, wenn sie sich ändert. Gibt es den angezeigten Ordner danach im Archiv nicht mehr, geht der
Tab auf den nächsten vorhandenen Ordner darüber, notfalls auf die oberste
Ebene; gibt es die Archivdatei nicht mehr, in den Ordner, in dem sie lag. Die
Statuszeile nennt beide Male, was fehlt und was jetzt gezeigt wird. **Nach
einem Neustart steht ein Tab, der ein Archiv zeigte, im Ordner
der Archivdatei** und nicht mehr im Archiv; hinein geht es wieder mit Pfeil
rechts. Die Pfadeingabe (`shift+cmd+g`) in einem Archivtab nimmt einen Pfad auf
dem Mac und verlässt damit das Archiv; vorbelegt ist der Pfad der Archivdatei.

---

## SQLite-Datenbanken im Dateifenster

**Eine SQLite-Datenbank lässt sich im Dateifenster betreten wie ein Archiv,
allein zum Durchsehen.** Betreten werden Dateien mit den Endungen `.sqlite`,
`.sqlite3`, `.db` und `.db3`, ohne Rücksicht auf Groß- und Kleinschreibung.
Auch hier entscheidet allein die Endung: eine `.db`, die keine
SQLite-Datenbank ist, etwa ein `Thumbs.db` von Windows, betritt KRK trotzdem,
und die Statuszeile sagt, dass die Datei kein lesbares Archiv ist, mit dem
Grund „keine SQLite-Datenbank“.

**Hinein mit Pfeil rechts oder einem Doppelklick, hinaus mit Pfeil links**,
wie bei einem Archiv. **`return` öffnet die Datenbank weiter mit dem
Standardprogramm.** Der Fenstertitel nennt sie als
`/pfad/zur/datei.db:/name`.

**Was das Fenster zeigt:**

- Jede **Tabelle** und jede **Sicht** der Datenbank steht als Eintrag an der
  obersten Ebene, auch die inneren Tabellen, deren Name mit `sqlite_` beginnt.
  Indizes und Trigger stehen nicht da, Unterordner gibt es keine. Ein Name
  mit `/` oder `..` erscheint nicht; die Statuszeile nennt die Zahl.
- **Die Spalte „Größe“ zählt Zeilen und nicht Bytes**: für eine Tabelle steht
  dort „3 Zeilen“, für eine Sicht `--`, ebenso für eine Tabelle, deren Zeilen
  KRK nicht zählen kann, etwa eine virtuelle Tabelle. Auch der
  Markierungsstand unten zählt in einer Datenbank Zeilen.
- **Die Vorschau zeigt eine Tabelle als Tabelle**, so wie eine TSV-Datei,
  mit einer Kopfzeile „Tabelle · 3 Zeilen · 2 Spalten“ und den Spaltennamen
  in der ersten Zeile. Ergibt die Tabelle mehr als 1 MB Text, zeigt sie die
  vollständigen Zeilen dieses ersten MB, und die Kopfzeile sagt, wie viele
  von wie vielen Zeilen gezeigt sind.
- **Eine Sicht zeigt die Vorschau als ihren SQL-Text**, eingefärbt. KRK führt
  eine Sicht nie aus, auch nicht für die Vorschau.
- Eine Datenbank-Datei, die nicht betreten ist, zeigt in der Vorschau wie
  bisher ihre Angaben.

**Herauskopieren** (`f5`) ins andere Dateifenster, auf den Mac, auf einen
Server oder in ein Zip oder tar, legt eine Datei an:

| Eintrag | Datei am Ziel | Inhalt |
|---|---|---|
| Tabelle `kunden` | `kunden.tsv` | eine Zeile mit den Spaltennamen, dann jede Zeile, getrennt mit Tabulator |
| Sicht `offen` | `offen.sql` | der SQL-Text der Sicht |

Die Zeilen kommen in der Reihenfolge, in der die Datenbank sie liefert.
**Die Ausfuhr ist nicht verlustfrei**: ein leerer Wert (`NULL`) steht als
`NULL` da und ist von einer Zelle mit dem Text `NULL` nicht zu unterscheiden,
und ein Binärwert (BLOB) steht als Platzhalter wie `[BLOB, 512 Bytes]`, ohne
seinen Inhalt. Beide Wörter folgen der Sprache der Oberfläche. Beim Kopieren
in ein Zip oder tar liest KRK eine Tabelle zweimal, einmal, um die Länge zu
zählen, die das Archiv vorab braucht, und einmal zum Schreiben; eine große
Tabelle dauert dort also doppelt so lange.

**KRK schreibt nichts in eine Datenbank.** Hineinkopieren, Verschieben,
Umbenennen, Anlegen und Löschen gehen dort nicht. Die Befehle sind
ausgegraut, das Kontextmenü sagt „Das geht in einer Datenbank nicht.“, und
ein Kopieren aus dem anderen Fenster in eine Datenbank weist KRK ab; die
Statuszeile sagt dabei zurzeit noch „Dieses Archiv ist allein lesbar.“.

**Was beim Lesen an der Datei geschieht.** KRK öffnet eine Datenbank allein
lesend. Die Datenbankdatei selbst und ein danebenstehendes `-wal` bleiben
dabei Byte für Byte, wie sie sind, und KRK legt neben der Datenbank keine
Datei an, weder `-wal` noch `-shm` noch `-journal`. **Die eine Ausnahme ist
ein schon vorhandenes `-shm`**: steht neben einer Datenbank im WAL-Betrieb
eine `-shm`-Datei, setzt SQLite beim Lesen darin eine Lesemarke oder baut,
wenn KRK als einziges Programm liest, ihren Index neu auf. Die Daten der
Datenbank berührt das nicht. Liest KRK, während ein anderes Programm in die
Datenbank schreibt, sieht es den neuesten übergebenen Stand; hält das andere
Programm die Datenbank länger gesperrt, sagt die Statuszeile „die Datenbank
wird gerade geschrieben“. Eine kopierte WAL-Datenbank ohne ihre
Begleitdateien liest KRK ebenso, ohne dabei etwas anzulegen.

**Änderungen von außen erscheinen von selbst**, wie bei einem Archiv: legt
ein anderes Programm eine Tabelle an oder schreibt Zeilen, liest KRK das
Verzeichnis neu. Nach einem Neustart steht ein Tab, der eine Datenbank
zeigte, im Ordner der Datenbankdatei.

---

## Einen Datenträger auswerfen

**Auch dieser Eintrag steht allein im Menü der rechten Maustaste über der
Dateiliste** und heißt „Auswerfen“, zwischen „Duplizieren…“ und „Im Finder
öffnen“. Eine Taste trägt er nicht, und im Hauptmenü steht er nicht.

**Er wirft eingehängte Datenträger aus**: ein geöffnetes Disk-Image (DMG), einen
USB-Stick, eine Netzfreigabe. Der gewöhnliche Weg dorthin ist `/Volumes`: dort
steht jeder eingehängte Datenträger als Ordner, und ein Rechtsklick darauf und
„Auswerfen“ wirft ihn aus. Betroffen ist, worauf auch `f5` und `f6` wirken; sind
mehrere Datenträger markiert, gehen alle. Ein Disk-Image wird dabei abgemeldet,
ein Laufwerk mit mehreren Partitionen verschwindet ganz, wie beim Auswerfen im
Finder.

**Bei einem gewöhnlichen Ordner oder einer Datei tut er nichts** und sagt es:
„nichts auszuwerfen: unter den gewählten Einträgen steht kein eingehängter
Datenträger“. Ein Ordner **auf** einem Datenträger ist kein Datenträger; wer
ihn auswerfen will, geht in den übergeordneten Ordner und klickt auf den
Datenträger selbst. Als Datenträger gilt, was auch die Leiste links unter
„Geräte und Orte“ zeigt; den Startdatenträger wirft KRK nie aus.

**Dasselbe geht in der Leiste links unter „Geräte und Orte“.** Rechts in der
Zeile jedes auswerfbaren Datenträgers steht, wie im Finder, das
Auswerfzeichen ⏏; ein Klick darauf wirft ihn aus. Ein Rechtsklick auf die
Zeile öffnet ein Menü mit dem einen Eintrag „Auswerfen“, der dasselbe tut. Die
Auswahl in der Leiste bewegt sich dabei nicht, der Ordner im Dateifenster
bleibt also, wo er war. Das Benutzerverzeichnis, der Startdatenträger und die
Lesezeichen tragen weder das Zeichen noch das Menü. Nach dem Auswurf
verschwindet die Zeile aus der Leiste; die Meldungen stehen in der Statuszeile
des aktiven Dateifensters und lauten wie beim Weg über die Dateiliste. Ein
zweiter Klick, solange der Auswurf noch läuft, gleich ob in der Leiste oder in
der Dateiliste, schickt keinen zweiten Auftrag, sondern meldet „<name> wird
schon ausgeworfen …“.

**Die Statuszeile begleitet den Auswurf.** Zuerst steht dort „<name> wird
ausgeworfen …“, danach „<name> wurde ausgeworfen“, und die Liste in `/Volumes`
verliert den Eintrag. Stand ein Tab auf dem Datenträger, wechselt er wie bei
jedem anderen Auswurf ins Benutzerverzeichnis, und die Statuszeile nennt das.
Weist das System den Auswurf ab, meist weil noch ein Programm eine Datei darauf
offen hält, steht „<name> ließ sich nicht auswerfen: …“ da, mit dem Grund in
den Worten des Systems; der Datenträger bleibt dann eingehängt.

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

**Ein Kombinationskonflikt ist nicht jede Kombination, die zweimal vorkommt.**
Eine Kombination darf auf zwei Funktionen liegen, wenn beide nie zugleich
wirken können. Das trifft auf zwei Fälle zu:

- Die eine Funktion bekommt den Tastendruck über KRK, die andere über AppKit,
  etwa in einem Textfeld. So liegt `cmd+a` auf „Alle Einträge markieren“ und auf
  „Alles auswählen“.
- Beide bekommen ihn über KRK, aber die eine wirkt allein im Editor und die
  andere allein außerhalb davon. So liegt `cmd+1` auf „Nach Name sortieren“ im
  Dateifenster und auf „Termine: Sortierrichtung umkehren“ in der Termintabelle
  des Editors. Welche der beiden ein Anschlag auslöst, entscheidet der Ort des
  Fokus. Ebenso liegt `delete` auf „In den Papierkorb räumen“ im Dateifenster
  und auf „Eintrag löschen“ in den Eintragstabellen des Editors.
- Beide bekommen ihn über KRK, und die eine wirkt nur in einem Teil der Lagen,
  in denen die andere wirkt. Dann geht die engere vor. So liegt `cmd+n` auf
  „Eintrag hinzufügen“, solange der Fokus im Editor vor einer der
  Eintragsdateien des Notizordners steht, und sonst auf „Fenster einblenden“;
  ebenso `cmd+up` auf „Voriges Bild“, solange die Vorschau eine Bildfolge
  zeigt, und sonst auf „In den übergeordneten Ordner“.

Eine Funktion, die im Editor und außerhalb wirkt, schließt keine andere aus.
Drei Funktionen, die den Tastendruck auf demselben Weg bekommen, ergeben auf
einer Kombination deshalb immer einen Konflikt; eine Funktion, die ihn auf dem
anderen Weg bekommt, zählt dabei nicht mit und darf neben zweien davon liegen.
Dieselbe Regel gilt beim Zuweisen mit `cmd+t`: eine Kombination, die danach keinen
Konflikt ergibt, nimmt die Ansicht an, und beide Funktionen behalten sie; ergibt
sie einen, bleibt die Belegung, wie sie war, und die Ansicht nennt beide
Funktionen.
**Im Hauptmenü zeigt dann nur einer der beiden Einträge das Kürzel**, nämlich
der, der in der Menüleiste weiter links steht. „Termine: Sortierrichtung
umkehren“ unter „Home“ zeigt `⌘1`, und „Nach Name sortieren“ steht ohne Kürzel
da, sortiert mit `cmd+1` im Dateifenster aber weiter.

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
| `f10` | die Quicknote öffnen und schließen, siehe „Die Quicknote“ |

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

**Dateien aus JSON-Zeilen (`.jsonl`, `.ndjson`) zeigt die Vorschau Datensatz
für Datensatz.** Jede nicht leere Zeile erscheint eingerückt über mehrere
Zeilen, mit zwei Leerzeichen je Ebene und als JSON eingefärbt, und über jedem
Datensatz steht eine Trennzeile mit seiner Nummer, etwa `// ── 3 ──`. Werte
bleiben, wie sie in der Datei stehen: eine Zahl wie `1.50` und ein Escape wie
`é` werden nicht umgeschrieben. Eine Zeile, die kein gültiges JSON ist,
erscheint unverändert hinter `//`, und ihre Trennzeile sagt es dazu. Eine
solche Datei darf größer sein als die 1 MB, bis zu denen die Vorschau andere
Textdateien zeigt: dann zeigt sie die vollständigen Zeilen des ersten MB und
sagt in der ersten Zeile, dass gekürzt ist. Der Editor zeigt dieselbe Datei
unverändert, eine Zeile je Datensatz. Dasselbe gilt für eine `.jsonl` in einem
Archiv, auch in einer `.jsonl.gz`.

**Zeigt die Vorschau nichts, ist sie vielleicht ausgeblendet.** Die Vorschau
und der Editor teilen sich den rechten Rand: steht dort der Editor, ist in der
Leiste am Fensterfuß „Vorschau“ nicht angekreuzt, und die Auswahl in der
Dateiliste erscheint nirgends. `f3` (oder `cmd+y`) blendet die Vorschau wieder
ein, und sie zeigt dann den gewählten Eintrag.

### CSV- und TSV-Dateien

**Eine Datei mit der Endung `.csv`, `.tsv` oder `.tab` erscheint in Vorschau
und Editor als Tabelle.** Groß- und Kleinschreibung der Endung zählen nicht.
Andere Dateien mit Tabulatoren, etwa eine `.txt`, bleiben Text.

**Welches Zeichen die Felder trennt, wählt KRK nach einer festen Regel.** Eine
`.tsv` und eine `.tab` trennen immer mit dem Tabulator. Bei einer `.csv` gilt:

1. Steht in der ersten Zeile genau `sep=;`, `sep=,` oder `sep=` mit einem
   Tabulator, gilt dieses Zeichen. Die Zeile bleibt in der Datei stehen und ist
   keine Zeile der Tabelle.
2. Sonst zerlegt KRK den Anfang der Datei einmal mit Komma, einmal mit
   Semikolon und einmal mit Tabulator und nimmt das Zeichen, mit dem die meisten
   Zeilen gleich viele Felder tragen. Eine deutsche Excel-Datei mit `Name;Preis`
   und `Äpfel;1,50` trennt also mit dem Semikolon, obwohl sie Kommas enthält.
3. Greift keine der beiden Regeln, gilt das Komma.

Die Vorschau nennt das Zeichen in ihrer ersten Zeile, der Editor beim Öffnen in
der Statuszeile, dort samt der Regel, die es gewählt hat. **Hat die Regel
falsch gewählt, hilft „Trennzeichen wechseln“** im Menü „Editor“. Der Befehl
geht reihum Komma, Semikolon, Tabulator und ändert dabei kein Byte der Datei,
nur ihre Deutung. KRK merkt sich die Wahl nicht: beim nächsten Öffnen wählt
die Regel neu. Lädt der Editor dieselbe Datei nach einer Änderung von außen
neu, bleibt das gewechselte Zeichen stehen. Ab Werk liegt der Befehl auf keiner Taste; wer eine will, weist
sie mit **F1** und `cmd+t` zu. Für eine `.tsv` ist er ausgegraut.

**Gelesen wird allein UTF-8.** Eine CSV, die ein älteres Excel als
„CSV (Trennzeichen-getrennt)“ in Windows-Kodierung geschrieben hat, öffnet der
Editor nicht, und die Vorschau zeigt nur ihre Angaben. Excel schreibt UTF-8
unter „CSV UTF-8“.

**Die erste Zeile ist eine gewöhnliche Zeile der Tabelle.** KRK hält sie nie für
eine Kopfzeile: man kann sie bearbeiten, verschieben und löschen wie jede
andere. Über den Spalten steht der Buchstabe der Spalte (A bis Z, dann AA), im
Editor daneben gekürzt der Text, den die erste Zeile in dieser Spalte trägt.
So bleiben die Spaltennamen beim Rollen sichtbar, und eine Datei ohne Kopfzeile
verliert keine Zeile.

**Zeilen dürfen verschieden viele Felder haben.** Die Tabelle hat so viele
Spalten, wie die längste Zeile Felder trägt; in einer kürzeren Zeile stehen
die fehlenden Zellen leer. In die Datei kommen sie erst, wenn man in eine
solche Zelle etwas schreibt.

#### In der Vorschau

Die Vorschau zeigt die Datei als ausgerichteten Text in Festbreitenschrift:
oben eine Zeile mit Trennzeichen, Zeilen- und Spaltenzahl, darunter die
Spaltenbuchstaben, eine Trennlinie und links vor jeder Zeile ihre Nummer.

```text
Semikolon · 3 Zeilen · 2 Spalten
 │ A      │ B
─┼────────┼──────
1│ Name   │ Preis
2│ Äpfel  │ 1,50
3│ Birnen │ 2,00
```

- **Sie bricht nicht um und rollt waagerecht wie senkrecht.** Eine breite
  Tabelle reicht über den rechten Rand hinaus.
- Eine Zelle mit mehr als 40 Zeichen endet in „…“. Ein Zeilenumbruch in einer
  Zelle erscheint als `⏎`, ein Tabulator als `⇥`.
- Ist die Datei größer als 1 MB, zeigt die Vorschau die vollständigen Zeilen
  ihres ersten MB, und die erste Zeile sagt, dass gekürzt ist.
- Kopiert wird der gezeigte Text. Zellen zum Einfügen in eine
  Tabellenkalkulation kopiert der Editor.
- Eine CSV in einem Archiv erscheint ebenso als Tabelle, auch eine `.csv.gz`.
  Auf einem Server zeigt die Vorschau wie bei jeder Datei nur die Angaben.
- Als gewöhnlicher Text erscheint die Datei, wenn sie leer ist, wenn ein
  Anführungszeichen bis zum Dateiende nicht schließt oder wenn sie mehr Spalten
  trägt, als die Tabelle zeigt.

#### Im Editor

**In der Formatansicht steht die Datei als Tabelle**, vorn eine schmale Spalte
mit der Nummer jeder Zeile. `ctrl+cmd+e` wechselt zur Rohansicht, die die Datei
zeigt, wie sie auf der Platte steht, und zurück. Beide Flächen rollen
waagerecht und senkrecht. Die Breite einer Spalte lässt sich an ihrer Kante im
Kopf ziehen. Eine Zelle zeigt einen Zeilenumbruch als `⏎` und einen Tabulator
als `⇥`; bearbeitet wird der wirkliche Text.

**Die Datei steht in zwei Fällen als Text statt als Tabelle**, und die
Statuszeile nennt den Grund: wenn ein Anführungszeichen bis zum Dateiende nicht
schließt, denn dann würde der ganze Rest der Datei eine einzige Zelle, und wenn
die Datei mehr Spalten trägt, als die Tabelle zeigt (derzeit 256; die Meldung
nennt beide Zahlen). Nach einer Berichtigung im Text versucht `ctrl+cmd+e` die
Tabelle erneut. Das gilt allein für diese Datei: die nächste Datei, etwa eine
Markdown-Datei, öffnet in der Ansicht, die man zuletzt gewählt hat.

**Die Auswahl ist immer ein Rechteck aus Zellen**, nach dem Öffnen die Zelle A1.

| Griff | Wirkung |
|---|---|
| Klick auf eine Zelle | wählt die Zelle |
| `shift`-Klick, Ziehen | erweitert die Auswahl bis zu dieser Zelle |
| Klick auf den Spaltenkopf | wählt die Spalte, mit `shift` bis zu ihr |
| Klick auf die Zeilennummer | wählt die Zeile, mit `shift` bis zu ihr; Ziehen dort wählt weitere Zeilen |
| Klick auf den Kopf über den Zeilennummern | alle Zellen |
| Pfeiltasten | eine Zelle weiter |
| `shift` mit Pfeiltaste | erweitert die Auswahl um eine Zelle |
| `cmd+a` | alle Zellen |

**Eine Zelle bearbeiten:** ein Doppelklick öffnet die angeklickte Zelle,
`return` und `cmd+return` öffnen die Zelle oben links in der Auswahl. Ein
Doppelklick unter die letzte Zeile hängt eine leere Zeile an. In der offenen
Zelle gilt:

- `return`, `cmd+return` oder ein Klick daneben übernehmen den Text.
- `opt+return` schreibt einen Zeilenumbruch in die Zelle.
- `tab` übernimmt und öffnet die Zelle rechts daneben, am Zeilenende die erste
  der nächsten Zeile; `shift+tab` geht denselben Weg zurück.
- `esc` verwirft das Getippte, und die Zelle zeigt wieder den alten Text.
- `delete` nimmt ein Zeichen zurück und löscht keine Zeile.
- `cmd+c` und `cmd+v` kopieren und fügen Text innerhalb der Zelle ein.

Jeder Text ist erlaubt, auch einer mit Trennzeichen, Anführungszeichen oder
Zeilenumbruch: KRK setzt den Wert dann beim Schreiben in Anführungszeichen,
wie Excel und Numbers es lesen.

**Zeilen und Spalten:**

| Taste | Wirkung |
|---|---|
| `cmd+n`, `shift+cmd+return` | eine leere Zeile ans Ende, ihre erste Zelle gleich offen |
| `opt+cmd+return` | so viele leere Zeilen über der Auswahl, wie sie Zeilen umfasst |
| `opt+cmd+up`, `opt+cmd+down` | die Zeilen der Auswahl als Block eine Stelle nach oben oder unten |
| `delete`, `shift+cmd+delete` | die Zeilen der Auswahl löschen, ohne Rückfrage |
| `ctrl+cmd+return` | eine leere Spalte links der Auswahl |
| `ctrl+shift+cmd+return` | eine leere Spalte hinter der letzten |
| `ctrl+shift+cmd+delete` | die Spalten der Auswahl löschen, ohne Rückfrage |
| `cmd+c` | die gewählten Zellen kopieren |
| `cmd+v` | Zellen aus der Zwischenablage ab der Zelle oben links einfügen |
| `cmd+z` | jede dieser Handlungen zurücknehmen |

**`delete` löscht ganze Zeilen und nicht den Inhalt der gewählten Zellen.**
Wer eine einzelne Zelle leeren will, öffnet sie, löscht den Text und übernimmt.
Eine Tabelle behält immer mindestens eine Spalte. „Eintrag hinzufügen“ wirkt
auch in der Rohansicht: dort hängt es eine leere Zeile an, und die Schreibmarke
steht an ihrem Anfang.

„Zeile darüber einfügen“, die Spaltenbefehle und „Trennzeichen wechseln“
stehen im Menü „Editor“. Die Befehle für eine Zeile am Ende, das Verschieben
und das Löschen von Zeilen teilt die Tabelle mit den Notizen, und sie stehen
deshalb im Menü „Home“ bei den Eintragsbefehlen. Sie wirken nur,
solange der Fokus im Editor steht und dieser die Tabelle einer CSV- oder
TSV-Datei zeigt; sonst sind sie ausgegraut. „Aufgabe abhaken oder öffnen“
bleibt in dieser Tabelle ausgegraut, und Suchen, Ersetzen und der Zeilensprung
wirken allein in der Rohansicht. **Mit einer eigenen `keymap.toml` tragen die
neuen Befehle keine Tasten**; sie sind über das Menü erreichbar und lassen sich
mit **F1**, `cmd+t` einzeln belegen, ohne `cmd+r`.

**Kopieren und Einfügen sprechen dieselbe Form wie Numbers und Excel.** `cmd+c`
legt die gewählten Zellen als Text ab, die Zeilen durch Zeilenumbrüche und die
Zellen durch Tabulatoren getrennt. So eingefügt, landen sie in Numbers und
Excel Zelle für Zelle. Eine einzelne Zelle ist dabei ihr bloßer Text. `cmd+v`
liest einen Bereich aus Numbers oder Excel auf demselben Weg und setzt ihn ab
der Zelle oben links in der Auswahl ein. Reicht er über das Ende der Tabelle,
hängt KRK Zeilen an oder füllt Zeilen auf. Danach ist der eingefügte Bereich
gewählt, und ein einziges `cmd+z` nimmt ihn ganz zurück. Gelesen wird allein
Text: ein im Finder kopierter Eintrag kommt mit seinem Namen und nicht als
Pfad. Trägt die Zwischenablage gar keinen Text, etwa nur ein Bild, ändert
`cmd+v` nichts, und die Statuszeile sagt es.

**Rückgängig ist ein Verlauf für Tabelle und Rohansicht zusammen.** `cmd+z`
nimmt Tabellenhandlungen und Getipptes der Rohansicht in der Reihenfolge
zurück, in der sie geschahen. Eine Spaltenhandlung ändert fast jede Zeile der
Datei. In einer großen Datei kann sie deshalb so viel Platz im Verlauf
belegen, dass ältere Schritte herausfallen und sich nicht mehr zurücknehmen
lassen.

**Jede Handlung ändert zuerst nur den Stand im Editor.** Auf die Platte kommt
sie mit `cmd+s`. Dabei ändert KRK allein die Bytes der Zellen, Zeilen und
Spalten, die man angefasst hat; jede andere Zeile kommt Byte für Byte zurück,
mit ihren Anführungszeichen und ihren Zeilenenden. Der Editor öffnet Dateien
bis 16 MB.

**Eine CSV- oder TSV-Datei behält beim Sichern ihre Bytefolgenmarke und jedes
ihrer Zeilenenden.** Eine Datei, die Excel als „CSV UTF-8“ geschrieben hat,
beginnt mit einer unsichtbaren Marke, an der Excel die Kodierung erkennt, und
trennt ihre Zeilen mit `\r\n`; nach dem Sichern in KRK tut sie beides weiter,
und Umlaute erscheinen in Excel richtig. Ein Zeilenumbruch innerhalb einer
Zelle, den Excel als `\n` schreibt, bleibt `\n`; eine Datei mit gemischten
Zeilenenden behält jedes, wie es war, und eine Datei ohne Zeilenende am Schluss
bleibt ohne. Neu entsteht ein Zeilenende allein dort, wo man etwas hinzufügt:
eine neue Zeile endet wie die erste Zeile der Datei, und ein Umbruch, den man
mit `opt+return` in eine Zelle schreibt, ist ein `\n` wie bei Excel. Die
Rohansicht zeigt die Zeilenenden, wie sie in der Datei stehen; eine Zeile, die
man dort mit `return` beginnt, endet auf `\n`. **Eine Datei, deren Zeilen
allein mit `\r` enden**, wie Excel sie als „CSV (Macintosh)“ schreibt, zeigen
Vorschau und Editor ebenso als Tabelle, und beim Sichern behält sie ihre `\r`.
**Für jede andere Datei bleibt es bei der bisherigen Regel**: KRK sichert sie
ohne Marke und mit `\n`.

---

## Die Quicknote

`f10` öffnet im Editorbereich eine Quicknote: eine leere Schreibfläche für eine
Notiz, die keine Datei braucht. Steht der Editor nicht am rechten Rand, kommt
er hervor. Was dort vorher stand, die Vorschau oder der Git-Bereich, kommt beim
Schließen zurück, und der Fokus geht dorthin, wo er vor `f10` stand. Solange
die Quicknote offen ist, heißt der Kopf des Editors „Quicknote“, und mit dem
Fokus darin auch der Fenstertitel.

| Taste | Wirkung |
|---|---|
| `f10` | öffnen; steht der Fokus woanders, holt `f10` ihn in die offene Quicknote, steht er darin, schließt `f10` sie |
| `shift+f10` | kopieren und schließen, wie die Schaltfläche „Kopieren“ |
| `esc`, `cmd+e` | schließen, wie die Schaltfläche „Schließen“ |

Oben in der Fläche stehen drei Schaltflächen:

- **Leeren** löscht den ganzen Text. `cmd+z` nimmt das zurück, solange die
  Quicknote offen ist.
- **Schließen** blendet sie aus. Der Text bleibt stehen und ist beim nächsten
  `f10` wieder da.
- **Kopieren** legt den **ganzen** Text in die Zwischenablage, nicht die
  Auswahl, leert die Quicknote danach und schließt sie. Die Statuszeile sagt
  „Die Quicknote ist in der Zwischenablage“ und nennt die Zahl der Zeichen. Eine
  leere Quicknote kopiert nichts: sie schließt, und die Zwischenablage bleibt,
  wie sie war. Lässt sich der Text nicht kopieren, bleibt er stehen und die
  Quicknote offen.

Im Hauptmenü stehen die drei als „Quicknote öffnen und schließen“, „Quicknote
kopieren und schließen“ und „Quicknote leeren“ unter „Editor“; „Quicknote
leeren“ hat ab Werk keine Taste.

**Der Text liegt allein im Arbeitsspeicher.** KRK schreibt ihn nirgends hin,
auch nicht in die Sitzung. Mit dem Beenden von KRK oder mit einem Absturz ist er
verloren, und KRK fragt vorher nicht nach. Wer ihn behalten will, kopiert ihn.

**Eine im Editor offene Datei liegt unberührt darunter.** Ihr Stand, ihre
Schreibmarke und ihr Rückgängig bleiben, wie sie waren, und nach dem Schließen
ist sie wieder zu sehen. Die Befehle, die dieser Datei gelten, sind in der
Quicknote ausgegraut: „Sichern“, „Editor schließen“ (`opt+cmd+e`), „Zwischen
Roh- und Formatansicht wechseln“, Suchen und Ersetzen, „Zu Zeile springen“ und
„PIN ändern“. „Lesezeichen anlegen“ (`cmd+d`) ist nicht ausgegraut, legt aber
nichts an; die Statuszeile sagt „In der Quicknote gibt es keine Textmarken.“
„Teilen“ und „Ordner der angezeigten Datei zeigen“ sehen die Datei darunter
nicht und antworten, als zeige der Editor keine.

**Die Quicknote schließt auch ohne eigenen Befehl**: wenn `opt+cmd+b` den
Editor ausblendet oder die Vorschau oder der Git-Bereich seinen Platz
einnimmt, wenn eine Datei in den Editor kommt, etwa mit `f4`, wenn das
Hauptfenster geschlossen wird und bevor KRK nach einem ungesicherten Stand der
Datei darunter fragt. Der Text bleibt in jedem dieser Fälle stehen.

**Die Quicknote nimmt so viel Text auf, wie der Editor aus einer Datei
annimmt.** Eine Änderung, die darüber hinausginge, ob getippt, eingefügt oder
hineingezogen, unterbleibt ganz, und die Statuszeile sagt „Der Text ist zu groß
für die Quicknote; eingefügt wurde nichts.“ Ist das Fenster zu schmal, um den
Editor einzublenden, sagt sie „Für die Quicknote ist das Fenster zu schmal.“

**Text aus `secrets.txt` darf in die Quicknote.** Wer ihn dorthin einfügt, hat
ihn danach im Arbeitsspeicher stehen, bis er kopiert, geleert oder KRK beendet
wird, und „Kopieren“ legt ihn ohne weitere Frage in die Zwischenablage, wie es
ein Kopieren aus `secrets.txt` selbst täte.

**Mit einer eigenen `keymap.toml` trägt keine der drei Funktionen eine
Taste.** KRK hängt sie beim Start unbelegt an; über das Hauptmenü „Editor“ und
die Schaltflächen wirken sie trotzdem. `f10` und `shift+f10` weist man einzeln
zu: **F1**, „Quicknote öffnen und schließen“ wählen, **Zuweisen** (`cmd+t`),
`f10` drücken; ebenso „Quicknote kopieren und schließen“ mit `shift+f10`; die
Ansicht mit **Fertig** verlassen. Auf Apple-Tastaturen ist die nackte `f10` ab
Werk „Ton aus“, und KRK bekommt sie dann nur mit gehaltener fn-Taste, beim
Zuweisen wie beim Benutzen. **Nicht `cmd+r`**: es brächte die Tasten zwar mit,
setzt aber die ganze eigene Belegung auf die Auslieferungsfassung zurück, also
auch jede eigene Tastenzuweisung.

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

Die Leiste ganz links führt eine Liste mit bis zu drei Überschriften. Oben die
Lesezeichen aus `bookmarks.toml`, in der Reihenfolge, in der sie dort stehen;
darunter „Verbindungen“, ebenfalls aus `bookmarks.toml`, aber nur, sobald dort
mindestens eine Verbindung steht; unten „Geräte und Orte", die bei jedem Aufbau
frisch vom System kommen und nie abgelegt werden. Wie eine Verbindung entsteht
und was ein Klick auf sie tut, steht unter „Verbindungen“ im Abschnitt über den
Notizordner.

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
Beschriftung sagt, welche es ist. Die dritte Sorte, die Verbindung, legt
`cmd+d` allein in der Tabelle von `secrets.txt` an, für den gewählten Eintrag
(unter „Verbindungen“, „Eine Verbindung anlegen“); sonst entsteht sie über
„Verbinden…“ im Kontextmenü der Dateiliste.

**Umbenennen und Verschieben wirken auf Verbindungen wie auf Lesezeichen.**
`ctrl+u` ändert allein die Beschriftung in der Leiste, und die Verbindung findet
ihren Eintrag in `secrets.txt` weiter unter dem alten Namen. `opt+up` und
`opt+down` verschieben innerhalb der eigenen Gruppe und nie über eine
Überschrift hinweg. `ctrl+delete` nimmt allein das Lesezeichen; der Eintrag in
`secrets.txt` bleibt stehen.

Ein Lesezeichen, dessen Ziel fort ist, bleibt stehen und trägt den Zusatz
„(fehlt)". Die Zeile ist zusätzlich grau, aber das Wort ist das Kennzeichen,
nicht die Farbe. Eine Verbindung trägt den Zusatz nie: ob ihr Eintrag in
`secrets.txt` noch steht, weiß KRK erst nach dem Entschlüsseln, und die Leiste
entschlüsselt nichts.

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
| die Wurzel | den Ordner mit `.fusion-setup` darin | Projekt, Einrichtungszeitpunkt, fusion-Fassung, Zahl der Arbeitspakete, offene gemeinsame Defekte, Nachrichten im Forum |
| Projektwurzel mit fusion-Werkbank | den Ordner, der `fusion-workbench` enthält | dieselben Angaben, eine Ebene höher gelesen |
| ein Arbeitspaket | einen Ordner unmittelbar unter `work-packages/`, an seinem Pfad | Titel des Arbeitspakets, seinen Status, die Querverweise, wer es eingetragen hat, und den Text der Directive |
| alle Arbeitspakete | `work-packages/` | Arbeitspakete gesamt und die offenen Defekte aller Arbeitspakete |
| der gemeinsame Speicher | `shared/` | je Unterspeicher die Zahl der Datensätze und das jüngste Datum, bei Verläufen und Untersuchungen nur die Zahl |
| ein Speicher | einen der Unterspeicher | Zahl der Datensätze und die jüngsten davon |
| ein Defektspeicher | `issues/` | Datensätze gesamt, offen, geschlossen, zurückgestellt, und die jüngsten davon |
| der Ablagespeicher | `archive/` | Zahl der Läufe und das Datum der letzten Ablage |

Praktisch heißt das: ein Klick auf den Projektordner beantwortet „wie viele
Arbeitspakete gibt es, wie viele Defekte sind offen, und liegt eine Nachricht",
ein Klick auf den Ordner eines Arbeitspakets beantwortet „was war die Directive
und wie weit ist sie", und ein Klick auf `issues/` beantwortet „wie viel liegt
hier offen".

Die Profile folgen den Ordnernamen ab fusion 12: `work-packages/`, `plans/` und
`consultations/`. Eine Werkbank, die ihre Speicher noch unter den Namen von
vor fusion 12 führt, erkennen sie an diesen Stellen nicht, und die Vorschau
zeigt dort die Metadaten; `/fusion:migrate` benennt die Ordner um.

**Und die Einschränkung von oben gilt hier:** neue Profile einer neuen
KRK-Fassung kommen nicht von selbst. Dass es welche gibt, meldet KRK beim ersten
Start der neuen Fassung, und „Neuerungen anzeigen" nennt sie beim Namen. Wer
schon eine eigene `readers.toml` hat, sieht die umbenannten Profile erst nach
einem Zurücksetzen. Der erste Weg ist „Auf Werkseinstellungen zurücksetzen…“ im
Menü „KRK“: die Vorschau zeigt die mitgelieferten Profile sofort, ohne
Neustart, und die eigene Datei liegt mit Zeitstempel im Ablageordner. Der
Befehl setzt dabei auch `settings.toml` und `keymap.toml` zurück, mit allen
eigenen Tastenzuweisungen. Wer allein die Profile zurückholen will, beendet
KRK, legt `readers.toml` **beiseite** und startet KRK neu; sie entsteht dann
neu aus der Auslieferungsfassung. Eigene Zeilen holt man sich auf beiden Wegen
aus der beiseitegelegten Datei zurück. Im Einzelnen stehen die Wege oben unter
den Neuerungen und in `README.md` unter „Neuerungen an den eigenen Dateien
übernehmen".

### Die Bildfolge: durch die Fotos eines Jahres oder Monats blättern

Ein Profil kann statt Zeilen eine **Bildfolge** tragen. Trifft es den
ausgewählten Ordner und liegen darunter Fotos, zeigt die Vorschau das erste
Foto statt einer Zusammenfassung. Die Auslieferungsfassung bringt dafür zwei
Profile mit: „Fotos: ein Jahr“ für einen Ordner `…/Fotos/2008` und „Fotos: ein
Monat“ für einen Ordner `…/Fotos/2008/08`, gleich wo `Fotos` liegt. Ein Ordner
`Fotos/Urlaub` oder `Fotos/2008/August` trifft keines der zwei; wer seine
Fotos unter anderen Namen ablegt, schreibt das Muster in `readers.toml` um,
und die Datei zeigt dazu ein Beispiel unter „Die Bildfolge“.

**Sie gilt für den ausgewählten Ordner**, wie jede Zusammenfassung. Die
Jahresfolge sieht man, wenn in `Fotos` die Zeile `2008` ausgewählt ist, die
Monatsfolge, wenn in `Fotos/2008` die Zeile `08` ausgewählt ist.

**Ohne ausgewählte Zeile gilt der angezeigte Ordner.** Wer in `Fotos/2008`
oder `Fotos/2008/08` hineingeht, hat dort keine Zeile ausgewählt, und die
Vorschau zeigt die Folge dieses Ordners. Cmd+Pfeil hoch blättert dann in
dieser Folge und steigt nicht nach oben; auf dem ersten Foto geschieht nichts.
Aufsteigen geht dort mit Pfeil links.

- **Welche Fotos:** Dateien mit den Endungen png, jpg, jpeg, gif, tif, tiff,
  heic, heif, bmp und icns, gleich wie geschrieben. Versteckte Dateien, deren
  Name mit einem Punkt beginnt, zählen nicht. Beim Jahr zählen allein die
  Fotos, die unmittelbar in einem Monatsordner liegen.
- **In welcher Reihenfolge:** die Monatsordner nach ihrem Namen, also alle
  Fotos aus `01` vor allen aus `02`. Innerhalb eines Monats nach dem
  Aufnahmedatum aus den Bilddaten; wo die Bilddaten keines hergeben, gilt das
  Änderungsdatum, bei gleichem Zeitpunkt der Name. Ein Monatsordner ohne Fotos
  wird übersprungen.
- **Blättern:** Cmd+Pfeil runter zeigt das nächste Foto, Cmd+Pfeil hoch das
  vorige. Am ersten und am letzten Foto tut die Taste nichts, und die Folge
  beginnt nicht von vorn. Die Auswahl in der Dateiliste bleibt dabei stehen.
- **Zum Foto springen:** Return führt die Dateiliste in den Ordner des
  angezeigten Fotos und wählt es dort aus; die Liste behält ihre Sortierung,
  und ein Filtertext bleibt nach derselben Regel stehen wie bei jedem
  Ordnerwechsel. Verdeckt er das Foto, sagt die Statuszeile „IMG_0970.jpg ist
  ausgefiltert.“, ist das Foto inzwischen fort, „IMG_0970.jpg ist nicht mehr
  da.“. Solange das angezeigte Foto noch vorbereitet wird, springt Return
  nicht und meldet „Die Bildfolge wird noch vorbereitet.“.
- **Nur mit dem Fokus in der Dateiliste.** Mit dem Fokus in der Vorschau
  gelten Cmd+Pfeil wie bisher.
- **Die Statuszeile** zeigt „Bild 3 von 41“, solange eine Bildfolge steht.
  Eine Folge nimmt höchstens 7.500 Fotos aus höchstens 60 Ordnern auf und
  liest je Ordner höchstens 10.000 Einträge; greift eine dieser Grenzen, enthält
  sie die ersten Fotos, und die Statuszeile nennt die Grenze, etwa „(Folge nach
  7.500 Fotos gekürzt)“, „(Folge nach 60 Ordnern gekürzt)“ oder „(Folge an
  10.000 Einträgen eines Ordners gekürzt)“. Solange die Aufnahmedaten gelesen werden, steht in der Vorschau
  „Die Bildfolge wird vorbereitet: 1.000 Fotos.“, und die Dateiliste nimmt
  jeden Tastendruck an.
- **Ein neuer Ordner, eine neue Folge:** wechselt die Auswahl und kommt sie
  zurück, beginnt die Folge wieder beim ersten Foto. Liegt in einem getroffenen
  Ordner kein einziges Foto, zeigt die Vorschau die gewohnten Metadaten mit den
  drei Zählzeilen.

**An diesen Orten wechseln Cmd+Pfeil hoch und Return ihren Sinn.** Cmd+Pfeil
hoch heißt dort „Voriges Bild“ und nicht „In den übergeordneten Ordner“, Return
heißt „Zum angezeigten Bild springen“ und nicht „Mit dem Standardprogramm
öffnen“, auch im angezeigten Jahres- oder Monatsordner ohne ausgewählte Zeile.
Überall sonst gilt die gewohnte Bedeutung, und **Pfeil links führt in
jeder Lage in den übergeordneten Ordner**. Die drei Befehle „Nächstes Bild“,
„Voriges Bild“ und „Zum angezeigten Bild springen“ stehen im Menü „Vorschau“
und in der F1-Ansicht in der Gruppe „Vorschau“; dass zwei davon eine
Kombination mit einem Befehl des Dateifensters teilen, ist kein Konflikt,
denn sie wirken allein, solange eine Bildfolge steht.

**Die zwei Profile übernehmen, wer KRK schon gestartet hat.** Eine vorhandene
`readers.toml` bekommt sie nicht von selbst; „Neuerungen anzeigen“ nennt sie
beim Namen. Zwei Wege gibt es:

1. **„Auf Werkseinstellungen zurücksetzen…“** im Menü „KRK“. Die zwei Profile
   wirken sofort, ohne Neustart. **Der Befehl setzt zugleich die Belegung und
   die Einstellungen zurück:** `keymap.toml` mit allen eigenen
   Tastenzuweisungen und `settings.toml` stehen danach im Auslieferungszustand,
   die alten Dateien liegen mit Zeitstempel im Ablageordner. **Der Notizordner
   bleibt**, samt seinem gewählten Ort und seinem Inhalt.
2. **Von Hand:** die zwei `[[profil]]`-Blöcke vom Ende der Auslieferungsfassung
   (`resources/default-readers.toml`, Abschnitt „Die Profile für Fotoordner“)
   bei ausgeschaltetem KRK in die eigene `readers.toml` kopieren. Die übrige
   Datei bleibt, wie sie ist.

**Wer eine eigene `keymap.toml` führt, findet die drei Befehle unbelegt.** Sie
stehen im Menü „Vorschau“ und wirken von dort aus, nur ohne Kombination. Belegt werden sie in
der F1-Ansicht: den Befehl wählen, **Zuweisen** (`cmd+t`), die Kombination
drücken, die Ansicht verlassen. `cmd+up` und `return` lassen sich dabei
zuweisen, obwohl „In den übergeordneten Ordner“ und „Mit dem Standardprogramm
öffnen“ sie tragen. Die Auslieferungsbelegung zurückzuholen (`cmd+r` in der
F1-Ansicht) ist dafür der falsche Weg, denn es nimmt alle eigenen
Zuweisungen mit.

---

## Der Notizordner

`f2` oder `cmd+k` führt in den Notizordner, ab Werk `~/krkhome/`, gleich wo der
Fokus gerade steht. Das aktive Dateifenster zeigt den Ordner, und der Fokus geht
dorthin. Steht in diesem Dateifenster schon ein Tab auf den Notizordner, wird er
sichtbar; sonst entsteht ein neuer. Ein Blatt geht dabei nicht auf. Der Tab ist
ein gewöhnlicher Tab der Dateiliste und steht nach einem Neustart wieder da. Wo
der Notizordner liegt, wählt man über „Ort wählen…“ oder trägt es in
`settings.toml` ein; davon handelt „Der Ort des Notizordners“ weiter unten.

**Alle Befehle zum Notizordner stehen im Hauptmenü „Home“**, gleich hinter
dem Anwendungsmenü: „Notizordner öffnen“, „Ort wählen…“, die sechs Befehle der
Tabellen im Editor, „PIN ändern“ und „Termine: Sortierrichtung umkehren“. Unter „Anwendung“ und „Editor“ stehen sie
nicht mehr. Tasten und Wirkung der älteren Befehle sind geblieben, nur der Platz
in der Menüleiste hat sich geändert. „Ort wählen…“ hat ab Werk keine Taste; wer
eine will, weist sie in der Belegungsansicht (F1) zu. Mit einer eigenen
`keymap.toml` steht der Befehl ohne Kürzel unter „Home“, hinter den Befehlen,
die die eigene Datei dort nennt, und dafür ist an der eigenen Belegung nichts zu
tun.

**Beim ersten `f2` legt KRK den Ordner an**, und darin `notes.txt`,
`tasks.txt`, `secrets.txt` und `appointments.md`. Fehlt später eine dieser
Dateien, legt der nächste `f2` sie leer wieder an, mit null Bytes. Wer den
Ordner schon aus einer früheren Fassung hat, bekommt `appointments.md` beim
nächsten `f2` dazu; die übrigen Dateien bleiben, wie sie sind. Eine vorhandene Datei
überschreibt KRK nie. Wer aus einer früheren Fassung noch eine `.secrets.txt`
mit Punkt hat, findet sie nach dem nächsten `f2` als `secrets.txt` wieder;
was dabei im Einzelnen geschieht, steht unter „Geheimnisse in `secrets.txt`“.
Fehlt `secrets.txt`, legt „Verbinden…“ im Kontextmenü der Dateiliste Ordner
und Dateien auf demselben Weg an wie `f2`, nur ohne einen Tab zu öffnen. Beim Start legt KRK nichts an, auch nicht für einen
wiederhergestellten Tab auf den Ordner. Steht an der Stelle des Notizordners
eine gewöhnliche Datei, oder fehlt der Ordner darüber, etwa weil ein Laufwerk
nicht eingehängt ist, nennt die Statuszeile den Grund, und kein Tab geht auf.

**Bearbeitet wird im Editor**, mit `f4` auf der ausgewählten Datei. `notes.txt`,
`tasks.txt` und `appointments.md` sind reiner Text und lassen sich genauso in
jedem anderen Textprogramm pflegen. `secrets.txt` ist verschlüsselt und öffnet sich allein in
KRK mit einer PIN; davon handelt der letzte Teil dieses Abschnitts.

**Eine Notiz in `notes.txt`** beginnt mit einer Zeile `## <Thema>`. Alles
darunter bis zur nächsten solchen Zeile ist ihr Text:

```text
## Einkauf
Brot, Milch
und Kaffee für das Büro

## Idee
### Zwischenüberschrift im Text
Eine Zeile mit # oder ### eröffnet keine neue Notiz.
```

`## ` und nicht `# `, damit eine Zeile `# …` als Überschrift der ganzen Datei
frei bleibt.

**Eine Aufgabe in `tasks.txt`** ist eine Zeile `- [ ] <Text>`, erledigt
`- [x] <Text>`. Die Reihenfolge der Zeilen ist die Reihenfolge der Aufgaben:

```text
- [ ] Steuererklärung abgeben
- [x] Reifen wechseln
  Termin in der Werkstatt stand am Dienstag
- [ ] Zahnarzt anrufen
```

KRK liest großzügig: `- [X]`, `* [ ]`, `* [x]` und eingerückte Aufgaben gelten
ebenso. Eine Zeile, die keiner dieser Formen folgt, bleibt erhalten. In
`tasks.txt` gehört sie zur Aufgabe über ihr, wie die Werkstattzeile im Beispiel.

**Die Vorschau zeigt beide Dateien gerendert.** In `notes.txt` steht jedes Thema
als Überschrift, sein Text darunter. In `tasks.txt` trägt jede Aufgabe ein
Kästchen: ☐ für offen, ☑ für erledigt, in der Reihenfolge der Datei. Das
Aufzählungszeichen davor fällt weg, eine Nummer wie in `1. [x] …` bleibt
stehen. Die Vorschau liest nur: ein Klick auf das Kästchen hakt nichts ab, und
keine der beiden Dateien wird dabei geschrieben. Abgehakt wird im Editor, in
der Aufgabentabelle (siehe unten).

Eine Zeile ohne Eintragsform erscheint in der Vorschau als gewöhnlicher Text.
Wo sie steht, entscheidet, wie: unmittelbar unter einer Aufgabe zeigt die
Vorschau sie als Fortsetzungszeile dieser Aufgabe, so wie die Werkstattzeile im
Beispiel. Mit einer Leerzeile davor steht sie als eigener Absatz. KRK folgt
dabei den Regeln von CommonMark, nach denen jede Markdown-Datei gelesen wird.

**Die Formatansicht des Editors zeigt beide Dateien als Tabelle**
(`ctrl+cmd+e`): `tasks.txt` mit einer Zeile je Aufgabe, `notes.txt` mit den
Spalten Thema und Notiz. Wie man die zwei Tabellen bedient, steht am Ende
dieses Abschnitts. `appointments.md` folgt der Form von `notes.txt` mit einem
Datum als Thema; wie sie in Vorschau und Editor erscheint, steht unter „Termine
in `appointments.md`“.

**Die Kästchen gibt es nur für diese zwei Dateien im Notizordner.** Eine
`.md`-Datei mit `- [ ]` an einem anderen Ort zeigt die Vorschau wie bisher,
ohne Kästchen. Eine `notes.txt` oder `tasks.txt` in einem anderen Ordner bleibt
gewöhnlicher Text.

**Die Zettel des früheren Notizblatts stehen als Notizen in `notes.txt`.**
Übernommen wird genau einmal, nämlich in dem Augenblick, in dem ein `f2` den
Vorgabeort `~/krkhome/` zum ersten Mal selbst anlegt. KRK merkt sich das in
`reported.toml`: wer `~/krkhome/` später löscht, bekommt beim nächsten `f2`
eine leere `notes.txt` und die alten Zettel kein weiteres Mal. Wer schon mit
der Fassung 2.0.0 übernommen hat, dem setzt das nächste `f2` auf den Vorgabeort
denselben Vermerk, ohne ein zweites Mal zu übernehmen. An jedem anderen Ort entsteht
`notes.txt` leer, auch wenn `f2` den Ordner dort neu anlegt. Jeder nicht leere Zettel wird dabei zu einer
Notiz mit dem Thema „Zettel 1“ beziehungsweise „Zettel 2“, und sein Text bleibt
unverändert. Ein Zettel, der selbst eine Zeile mit `## ` trägt, wird nicht
übernommen, weil diese Zeile eine eigene Notiz eröffnete; die Statuszeile nennt
ihn. Gab es den Ordner schon, übernimmt KRK nichts, auch wenn `notes.txt` darin
fehlt und neu entsteht. `note-1.txt` und `note-2.txt` bleiben in jedem Fall
unverändert im Ablageordner liegen.

**Wer die Notizen auf mehreren Geräten haben will, stellt den Notizordner auf
einen Ordner in einem synchronisierten Speicher** (siehe „Der Ort des
Notizordners“). Der ältere, zweite Weg bleibt: `~/krkhome` als symbolischen
Verweis anlegen. Nötig ist er allein für den Vorgabeort, denn einen anderen Ort
wählt man über „Ort wählen…“ oder nennt ihn in `settings.toml` gleich selbst:

```sh
ln -s ~/Library/Mobile\ Documents/com~apple~CloudDocs/krkhome ~/krkhome
```

`ln -s` legt den Verweis nur an, wenn unter `~/krkhome` noch nichts steht; ein
schon angelegter Ordner zieht vorher mit seinem Inhalt an das Ziel um. KRK
behandelt dann das Ziel des Verweises als `~/krkhome/` und legt fehlende
Dateien dort an. Erkannt wird der Ordner über den Verweis `~/krkhome` und über
sein Ziel. Ein weiterer Verweis, den man selbst anderswo auf denselben Ordner
setzt, wird nicht erkannt: dort erscheinen `notes.txt` und `tasks.txt` wie
gewöhnliche Textdateien, und verloren geht nichts.

**Auch mit einer eigenen `keymap.toml` heißt der Menüeintrag „Notizordner
öffnen“.** Der Name eines Befehls kommt aus KRK selbst, in der Sprache, die
macOS für KRK gewählt hat, und nicht aus der eigenen Datei: ein Feld `name` in
`keymap.toml` wird gelesen und geduldet, aber nie angezeigt, und beim nächsten
Sichern der Belegung schreibt KRK es in der geltenden Sprache neu, damit die
Datei lesbar bleibt, wenn man sie über „Tastaturdefinition öffnen“ ansieht.
Eine `keymap.toml` aus einer früheren Fassung, die noch „Notizzettel anzeigen“
trägt, lädt deshalb ohne Fehler und ohne Meldung; die Meldung beim Start
vergleicht allein die Kennungen der Befehle, und die Kennung ist geblieben.
`f2` und `cmd+k` führen in den Notizordner, und der Eintrag steht unter dem
heutigen Namen im Menü „Home“. **Einen Handgriff an der eigenen Belegung
braucht das nicht**, und `cmd+r` in der Belegungsansicht ist dafür nicht nötig:
es setzt die ganze eigene Belegung auf die Auslieferungsfassung zurück, also
auch jede eigene Tastenzuweisung, und am Namen änderte es nichts.

### Der Ort des Notizordners

**Wo der Notizordner liegt, steht in `settings.toml` unter `notizordner`**, ab
Werk:

```toml
notizordner = "~/krkhome"
```

Gesetzt wird der Wert auf zwei Wegen: über „Ort wählen…“ im Menü „Home“, das
sofort gilt, oder von Hand in der Datei, was ab dem nächsten Start gilt. Beide
Wege prüfen den Ort nach denselben Regeln.

Der Wert beginnt mit `~/` für das eigene Benutzerverzeichnis oder mit `/`, etwa
`"~/Dropbox/Notizen"` oder `"/Volumes/Daten/Notizen"`. Ein leerer oder
relativer Wert, `~` allein, `~name/…` für ein fremdes Benutzerverzeichnis, ein
Wert, der kein Text ist, und ein Ort im Ablageordner von KRK gelten nicht.
**Einen Ersatzort setzt KRK dann nicht ein**: die Statuszeile nennt den Wert
beim Start, `f2` nennt ihn noch einmal, legt nichts an und öffnet keinen Tab.
Fehlt der Schlüssel in der eigenen Datei, gilt `~/krkhome`.

**„Ort wählen…“ öffnet einen Ordnerdialog am Hauptfenster**, der beim geltenden
Ort beginnt und fragt „Wo soll der Notizordner liegen?“. Wählen lässt sich
genau ein Ordner und keine Datei, übernommen wird er mit „Wählen“. Ein
Finder-Alias auf einen Ordner führt zu diesem Ordner, und einen neuen Ordner
legt man im Dialog selbst an. `esc` oder „Abbrechen“ schließt den Dialog, und
nichts ändert sich.

**Der gewählte Ort gilt sofort.** KRK trägt ihn als `notizordner` in
`settings.toml` ein: einen Ordner im eigenen Benutzerverzeichnis in der Form
`~/…`, jeden anderen mit vollem Pfad. Die Statuszeile nennt beide Orte:

> Der Notizordner ist jetzt „~/Dropbox/Notizen“; am alten Ort „~/krkhome“ bleibt
> alles liegen, und F2 führt zum neuen

Jeder Tab auf dem alten oder dem neuen Ort liest seinen Ordner neu, auch ein
verdeckter. Die Vorschau zeigt `notes.txt` und `tasks.txt` ohne neue Auswahl so,
wie es zum neuen Ort passt: dort gerendert, am alten Ort als Text. **Angelegt
wird dabei nichts, und kein Tab geht auf.** Was am neuen Ort fehlt, legt erst
der nächste `f2` an. Ist der gewählte Ordner schon der Notizordner, bleibt alles,
wie es ist:

> „~/krkhome“ ist schon der Notizordner; settings.toml bleibt, wie sie ist

Nannte `settings.toml` seit dem Start von Hand einen anderen Ort, und man wählt
den geltenden, schreibt KRK ihn zurück und meldet: „settings.toml nannte seit
dem Start einen anderen Ort; jetzt steht dort wieder „~/krkhome“, und der
Notizordner bleibt, wo er ist“.

**Solange der Editor eine Datei des Notizordners hält, wählt KRK keinen anderen
Ort.** Gemeint sind `notes.txt`, `tasks.txt`, `secrets.txt` und
`appointments.md`, am geltenden
Ort wie am gewählten; das zweite trifft etwa eine `notes.txt`, die am gewählten
Ort noch als gewöhnlicher Text offen ist. Die Statuszeile nennt die Datei:

> Zuerst notes.txt im Editor schließen; solange der Editor eine Datei des
> Notizordners hält, wählt KRK keinen anderen Ort

Ein gewählter Ordner, der nach den Regeln oben nicht gilt, wird mit derselben
Meldung abgewiesen wie ein Wert von Hand. Für den Ablageordner von KRK prüft
„Ort wählen…“ zusätzlich, ob der Ordner erst über einen Verweis dorthin führt.
In jedem dieser Fälle bleibt der geltende Ort.

**In eine beschädigte `settings.toml` schreibt KRK nicht**, und der Ort bleibt,
wie er ist. Die Statuszeile nennt den Befund:

> settings.toml ist zuerst von Hand zu berichtigen, KRK schreibt sie so nicht: …

**Ist `settings.toml` ein symbolischer Verweis**, etwa weil sie selbst in einem
synchronisierten Speicher liegt, ersetzt KRK den Verweis nicht durch eine Datei.
Die Statuszeile gibt stattdessen die Zeile mit, die man von Hand in die
Zieldatei einträgt:

> settings.toml ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch
> eine Datei; der Ort bleibt, wie er ist. Von Hand in die Zieldatei eintragen:
> notizordner = "~/Dropbox/Notizen"

Die eingetragene Zeile gilt dann wie jeder Wechsel von Hand ab dem nächsten
Start. Ist die Datei nicht lesbar oder lässt sie sich nicht schreiben, sagt die
Statuszeile das ebenso, und die Datei bleibt, wie sie war. Fehlt `settings.toml`
ganz, legt „Ort wählen…“ sie aus der Auslieferungsfassung an, mit dem gewählten
Ort.

**Ein Wechsel verschiebt nichts**, gleich auf welchem Weg er kommt. Die Dateien
am alten Ort bleiben liegen, und was am neuen Ort fehlt, legt erst der nächste
`f2` dort leer an; beim Start legt KRK nie etwas an. Ein Wechsel von Hand in
`settings.toml` gilt ab dem nächsten Start, und die Statuszeile nennt dann einmal
den neuen und den alten Ort. Die Regeln des Notizordners (die gerenderte
Vorschau, die Tabellen im Editor, die PIN für `secrets.txt`, der Inhaltsfilter)
gelten danach am neuen Ort und am alten nicht mehr. Eine `secrets.txt` am alten
Ort ist dann eine gewöhnliche Datei; ihr Chiffrat öffnet der Editor nicht als
Text.

**Zwei Grenzen hat der sofortige Wechsel.** Eine zweite laufende KRK-Instanz,
etwa aus „Weitere Instanz starten“, kennt den neuen Ort erst nach ihrem
Neustart. Bis dahin behandelt sie den alten Ort als Notizordner, **und ihr `f2`
legt am alten Ort an, was dort fehlt.** Außerdem kann ein Textprogramm, das
`settings.toml` offen hält, den geschriebenen Wert beim eigenen Sichern wieder
überschreiben; ab dem nächsten Start gilt dann dessen Stand. Wer die Datei von
Hand bearbeitet, schließt sie deshalb vor „Ort wählen…“ oder öffnet sie danach
neu.

**Ist `settings.toml` beschädigt oder nicht lesbar, gilt kein Notizordner.**
Das gilt auch, wenn der Fehler an einer ganz anderen Zeile steht, etwa ein
Tippfehler an `terminal`: KRK weiß dann nicht, ob ein anderer Ort eingestellt
ist, und legt lieber nichts an als am falschen Ort. Die Statuszeile nennt den
Schaden beim Start, `f2` nennt ihn noch einmal samt zwei Wegen hinaus:
`settings.toml` berichtigen und KRK neu starten, oder nach dem Berichtigen den
Ort über „Home“ → „Ort wählen…“ setzen. Der zweite Weg braucht keinen Neustart.
„Ort wählen…“ liest die Datei beim Schreiben neu; ist sie berichtigt, gilt der
gewählte Ort sofort, und ist sie es nicht, nennt die Statuszeile den Befund.
Konnte KRK die Datei beim Start gar nicht lesen, weil sich der Ablageordner
nicht öffnen oder seine Sperre nicht nehmen ließ, gilt ebenfalls kein
Notizordner, und die Meldung nennt den Neustart als Weg. Fehlte die Datei nur
und ließ sich nicht anlegen, gilt `~/krkhome`, denn dann hat niemand einen
anderen Ort eingestellt. **`secrets.txt` am zuletzt geltenden Ort bleibt in
dieser Lage geschützt**: Vorschau, Editor und Inhaltsfilter behandeln den
Ordner, der beim letzten Start galt (ohne einen solchen `~/krkhome`), weiter als
Notizordner, nur `f2` legt dort nichts an.

**Das Zurücksetzen auf Werkseinstellungen und das Beiseitelegen von Hand
wirken am Notizordner verschieden.** „Auf Werkseinstellungen zurücksetzen…“ im
Menü „KRK“ behält den eingestellten Notizordner: die neue `settings.toml` trägt
den bisherigen Wert von `notizordner` in derselben Schreibweise, `f2` öffnet
danach denselben Ordner, und keine Datei darin wird angefasst. Eine beschädigte
`settings.toml` setzt der Befehl deshalb gar nicht zurück, denn aus ihr ließe
sich der Wert nicht übernehmen. **Wer `settings.toml` dagegen von Hand
beiseitelegt**, wie es „Wo KRK seine eigenen Dateien ablegt“ als zweiten Weg
beschreibt, bekommt beim nächsten Start eine neue Datei mit `~/krkhome`, und
der Notizordner ist dann wieder `~/krkhome`. Die Dateien am alten Ort bleiben
dort liegen; die Zeile `notizordner = …` holt man sich wie jede eigene Zeile aus
der beiseitegelegten zurück, oder man wählt den Ort noch einmal.

**Eine ältere KRK-Fassung kennt den Schlüssel nicht.** Sie weist eine
`settings.toml`, die ihn führt, als beschädigt ab, lässt die Datei liegen und
arbeitet mit ihren eigenen Vorgaben weiter.

### Aufgaben in der Tabelle bearbeiten

**In der Formatansicht des Editors steht `tasks.txt` als Tabelle**, eine Zeile
je Aufgabe, vorn das Kästchen, dahinter der Text. `ctrl+cmd+e` wechselt
zwischen Tabelle und Rohtext, und der Fokus bleibt dabei im Editor. Die
Rohansicht zeigt die Datei so, wie sie auf der Platte steht.

**Die Tabelle zeigt allein die Aufgaben.** Eine Zeile ohne Aufgabenform, wie die
Werkstattzeile im Beispiel oben, sieht man nur in der Rohansicht. Verloren geht
sie trotzdem nicht. Beim Verschieben wandert sie mit ihrer Aufgabe. Beim Löschen
bleibt sie stehen und gehört danach zur Aufgabe darüber, bei der ersten Aufgabe
zum Anfang der Datei.

| Griff | Wirkung |
|---|---|
| Doppelklick auf den Text | die Zelle öffnet sich, ihr Text ist ausgewählt |
| Doppelklick unter die letzte Aufgabe, auch in die leere Tabelle | eine leere Aufgabe ans Ende, ihre Zelle gleich offen |
| Klick auf das Kästchen | die Aufgabe abhaken oder wieder öffnen |
| `shift+cmd+x` | die gewählte Aufgabe abhaken oder wieder öffnen |
| `cmd+return` | die gewählte Zelle öffnen; in einer offenen Zelle den Text übernehmen |
| `cmd+n`, `shift+cmd+return` | eine leere Aufgabe ans Ende, ihre Zelle gleich offen |
| `opt+cmd+up`, `opt+cmd+down` | die gewählte Aufgabe eine Stelle nach oben oder unten |
| `delete`, `shift+cmd+delete` | die gewählte Aufgabe löschen |
| `cmd+c` | den Text der gewählten Aufgabe kopieren |
| `cmd+z` | jede dieser Handlungen zurücknehmen |

Die sechs Befehle mit Tasten stehen auch im Hauptmenü „Home“. Wirken können
sie nur, solange der Fokus im Editor steht und dieser eine der Tabellen
zeigt; das Abhaken wirkt allein in der Aufgabentabelle, und das Verschieben
wirkt in der Termintabelle nicht. Allein das Anlegen wirkt auch in der
Rohansicht einer Eintragsdatei. Sonst sind sie
ausgegraut, und ihre Tasten wirken so, als wären sie nicht belegt.
Nach dem Öffnen ist die erste Aufgabe gewählt, damit die Tasten sofort eine
Zeile vor sich haben, und ein Klick auf das Kästchen holt den Fokus in die
Tabelle wie ein Klick auf den Text.

**Jede Handlung ändert zuerst nur den Stand im Editor.** Auf die Platte kommt
sie mit `cmd+s`, wie jede andere Änderung im Editor. Abhaken schreibt allein
die Zeile der Aufgabe neu, in der Grundform `- [ ] ` oder `- [x] `. Eine
Aufgabe, die als `* [X]` dastand, kommt deshalb als `- [ ] ` zurück, wenn man
sie wieder öffnet; ihre Nachbarn bleiben Zeichen für Zeichen, wie sie waren.

**`delete` und `shift+cmd+delete` fragen nicht nach.** Beide löschen den
gewählten Eintrag, in jeder der vier Tabellen, und `cmd+z` holt ihn zurück;
`shift+cmd+z` löscht ihn wieder. Die Statuszeile sagt es dazu. `delete` ist die
Rückschritt-Taste (⌫). Im Dateifenster räumt sie weiter in den Papierkorb, mit
Rückfrage; welche der zwei Bedeutungen gilt, entscheidet der Fokus.
`shift+cmd+delete` heißt im Finder „Papierkorb entleeren“ und wirkt hier
allein auf den gewählten Eintrag der Tabelle.

**In einer offenen Zelle bleibt `delete` der Rückschritt.** Solange eine Zelle
bearbeitet wird, nimmt die Taste dort ein Zeichen zurück und löscht keinen
Eintrag; dasselbe gilt in der Rohansicht und in der Quicknote. Wer aus einer
offenen Zelle heraus den ganzen Eintrag löschen will, nimmt
`shift+cmd+delete`: es übernimmt die Zelle und löscht dann.

**`cmd+n` legt einen Eintrag an, gleich wo der Fokus im Editor steht.** In der
Tabelle entsteht die leere Zeile am Ende, und ihre erste Zelle ist offen; aus
einer offenen Zelle heraus wird diese zuerst übernommen, und lässt sie sich
nicht übernehmen, entsteht nichts. **In der Rohansicht einer der vier
Eintragsdateien wirkt derselbe Befehl ebenso**, auch über `shift+cmd+return`:
KRK hängt den leeren Eintrag als Text ans Ende, also `- [ ] ` in `tasks.txt`,
`## ` in `notes.txt`, `## ` mit den vier leeren Feldzeilen `protocol:`,
`address:`, `account:` und `password:` darunter in `secrets.txt` und `## ` mit
dem heutigen Datum in `appointments.md`, und die Schreibmarke steht am Ende der
ersten Zeile des neuen Eintrags. Ein
`cmd+z` nimmt ihn zurück. In jeder anderen Datei, in der Quicknote und
außerhalb des Editors holt `cmd+n` wie bisher das Fenster zurück („Fenster
einblenden“); vor einer Eintragsdatei bleibt dieser Befehl über das Menü
erreichbar.

**Mit einer eigenen `keymap.toml` tragen „Eintrag löschen“ und „Eintrag
hinzufügen“ die zwei neuen Tasten nicht von selbst.** Beide behalten, was die
eigene Datei ihnen gibt. Wer `delete` und `cmd+n` dazu will, weist sie einzeln
zu: **F1**, den Befehl wählen, `cmd+t`, die Taste drücken, die Ansicht mit
„Fertig“ verlassen. Dass `delete` schon auf „In den Papierkorb räumen“ und
`cmd+n` schon auf „Fenster einblenden“ liegt, ist dabei kein Konflikt, und
beide behalten ihre Taste. **Nicht `cmd+r`**: es brächte die Tasten zwar mit,
setzt aber die ganze eigene Belegung auf die Auslieferungsfassung zurück.

**In einer offenen Zelle gelten diese Regeln:**

- `return`, `tab` oder ein Klick daneben übernehmen den Text. Der Fokus bleibt
  danach in der Tabelle, und `cmd+z` nimmt die Übernahme zurück.
- `esc` verwirft das Getippte, und die Zelle zeigt wieder den alten Text. Ein
  Filtertext im Dateifenster bleibt dabei stehen.
- Eine Aufgabe ist eine Zeile. Wer Text mit einem Zeilenumbruch einfügt, dem
  bleibt die Zelle offen, und die Statuszeile nennt den Grund.
- `cmd+s`, `ctrl+cmd+e`, das Schließen des Editors und `cmd+q` übernehmen eine
  offene Zelle zuerst; die Datei trägt danach den getippten Text. Lässt sich
  die Zelle nicht übernehmen, unterbleibt der Befehl.
- Die übrigen Befehle von KRK wirken auch während des Tippens, `f2` etwa führt
  weiter in den Notizordner.

Wer mit `shift+cmd+return` eine Aufgabe anlegt und dann `esc` drückt, behält
eine leere Aufgabe in der Datei. Das Verwerfen gilt dem Getippten und nicht dem
Anlegen; die leere Zeile nimmt ein `cmd+z` zurück.

**Suchen, Ersetzen und der Zeilensprung sind in der Tabelle ausgegraut.** Sie
arbeiten auf dem Rohtext, und der ist in der Tabellenform ausgeblendet; ein
Treffer landete unsichtbar darin. In der Rohansicht derselben Datei stehen sie
wie gewohnt bereit.

**Mit einer eigenen `keymap.toml` tragen die sechs Befehle keine Tasten.** KRK
hängt sie beim Start unbelegt an, und über das Hauptmenü „Home“ sind sie
trotzdem erreichbar, ohne jeden Handgriff an der eigenen Belegung.
Doppelklick, Kästchen, `cmd+c` und `cmd+z` wirken ohnehin. Wer eine Taste
will, weist sie in der Belegungsansicht einzeln zu: **F1**, den Befehl wählen,
`cmd+t`, die Taste drücken, „Fertig“. `cmd+r` ist dafür nicht nötig; es setzt
die ganze eigene Belegung zurück, also auch jede eigene Tastenzuweisung.

### Notizen in der Tabelle bearbeiten

**In der Formatansicht des Editors steht `notes.txt` als Tabelle mit zwei
Spalten**, links das Thema, rechts der Text der Notiz. Die Kopfzeile trägt die
Namen „Thema“ und „Notiz“. Eine Zeile ist so hoch wie ihr Text: eine Notiz mit
drei Zeilen belegt drei Zeilen, und beim Tippen wächst sie mit. Eine lange
Zeile bricht an der Spaltenbreite um. Die Themenspalte lässt sich an ihrer
Kante in der Kopfzeile breiter oder schmaler ziehen.

**Die Tabelle zeigt allein die Notizen.** Text vor dem ersten `## ` sieht man
nur in der Rohansicht, und er bleibt beim Verschieben und Löschen oben stehen.
Löschen nimmt eine Notiz ganz, also Thema und Text.

| Griff | Wirkung |
|---|---|
| Doppelklick auf eine Zelle | diese Zelle öffnet sich, ihr Text ist ausgewählt |
| Doppelklick unter die letzte Notiz, auch in die leere Tabelle | eine leere Notiz ans Ende, ihr Thema gleich offen |
| `cmd+return` | das Thema der gewählten Notiz öffnen; in einer offenen Zelle den Text übernehmen |
| `cmd+n`, `shift+cmd+return` | eine leere Notiz ans Ende, ihr Thema gleich offen |
| `opt+cmd+up`, `opt+cmd+down` | die gewählte Notiz eine Stelle nach oben oder unten |
| `delete`, `shift+cmd+delete` | die gewählte Notiz löschen, ohne Rückfrage |
| `cmd+c` | den Text der gewählten Notiz kopieren; ist er leer, das Thema |
| `cmd+z` | jede dieser Handlungen zurücknehmen |

Es sind dieselben Befehle wie in der Aufgabentabelle. „Aufgabe abhaken oder
öffnen“ bleibt hier ausgegraut, denn eine Notiz hat kein Kästchen. Wie dort
kommt jede Handlung erst mit `cmd+s` auf die Platte, und Suchen, Ersetzen und
der Zeilensprung sind in der Tabelle ausgegraut. Nach dem Öffnen ist auch hier
die erste Notiz gewählt, sodass die Befehle für die gewählte Notiz sofort
wirken.

**In einer offenen Zelle gelten andere Regeln als in der Aufgabentabelle:**

- `return` schreibt im Text der Notiz einen Zeilenumbruch, und die Zelle bleibt
  offen. Im Thema übernimmt `return` die Zelle, denn ein Thema ist eine Zeile.
- `tab` übernimmt die Zelle und öffnet die nächste: vom Thema in den Text
  derselben Notiz, vom Text in das Thema der nächsten. `shift+tab` geht
  denselben Weg zurück. Nach der letzten Zelle bleibt der Fokus in der Tabelle,
  und keine Zelle ist mehr offen.
- `cmd+return` oder ein Klick daneben übernehmen den Text.
- `esc` übernimmt eine geänderte Zelle und verwirft sie nicht. Die Statuszeile
  sagt dazu, dass `cmd+z` die Übernahme zurücknimmt. Eine unveränderte Zelle
  schließt `esc` einfach. Der Grund für diese Regel: eine Notiz kann mehrere
  Absätze tragen, und ein Verwerfen nähme sie ohne Warnung mit.
- Eine Zeile im Text, die mit `## ` beginnt, weist KRK ab, denn so beginnt die
  nächste Notiz. Die Zelle bleibt offen, und die Statuszeile nennt den Grund.
  `#` und `###` am Zeilenanfang sind erlaubt.
- Ein Thema mit Zeilenumbruch, etwa aus der Zwischenablage eingefügt, weist
  KRK ebenso ab.
- Das Übernehmen vor `cmd+s`, `ctrl+cmd+e`, dem Schließen des Editors und
  `cmd+q` gilt wie in der Aufgabentabelle, und `cmd+z` nimmt in einer offenen
  Zelle nur das Getippte zurück.

Wer mit `shift+cmd+return` eine Notiz anlegt und die Zelle ohne Eingabe
schließt, behält eine leere Notiz in der Datei. Ein `cmd+z` nimmt sie zurück.

### Termine in `appointments.md`

**`appointments.md` hat dieselbe Form wie `notes.txt`, nur steht in der
Kopfzeile statt eines Themas ein Datum.** Ein Termin beginnt mit einer Zeile
`## YYMMDD` oder `## YYMMDD HH:MM`, und alles darunter bis zur nächsten solchen
Zeile ist sein Text:

```text
## 261002 09:30
Zahnarzt, Praxis am Markt

## 261002
Geburtstag Anna
Geschenk nicht vergessen

## 261015
Steuererklärung abgeben
```

**Ein Datum gilt, wenn es genau so dasteht:** sechs Ziffern für Jahr, Monat
und Tag, wahlweise ein Leerzeichen und die Uhrzeit mit je zwei Ziffern für
Stunde und Minute. Der Tag muss im Kalender stehen, also kein 30. Februar, der
29. Februar nur im Schaltjahr; die Jahre reichen von 2000 bis 2099, die
Uhrzeiten von 00:00 bis 23:59. `261002`
und `261002 09:30` gelten, `26102`, `261002 9:30`, `261002 09:30 Uhr` und
`261302` nicht. **Ein ungültiges Datum bleibt als Termin stehen**, geht nicht
verloren und steht in der Tabelle am Ende, in beiden Richtungen. Text vor dem
ersten `## ` gehört zu keinem Termin; er bleibt oben in der Datei stehen und
erscheint allein in der Rohansicht.

**In der Formatansicht des Editors steht `appointments.md` als Tabelle mit zwei
Spalten**, „Datum“ und „Termin“, und bedient wird sie wie die Notiztabelle.
Sie ist nach dem Datum sortiert, ab Werk aufsteigend, also der früheste Termin
oben. Ein Termin ohne Uhrzeit steht vor einem am selben Tag mit Uhrzeit. Die
Datei selbst bleibt dabei in ihrer eigenen Reihenfolge; die Rohansicht zeigt
sie so, wie sie auf der Platte steht, und ein neuer Termin kommt ans Ende der
Datei, auch wenn er in der Tabelle weiter oben steht.

| Griff | Wirkung |
|---|---|
| Doppelklick auf eine Zelle | diese Zelle öffnet sich, ihr Text ist ausgewählt |
| Doppelklick unter den letzten Termin, auch in die leere Tabelle | ein Termin mit dem heutigen Datum, sein Datum gleich offen |
| `cmd+return` | das Datum des gewählten Termins öffnen; in einer offenen Zelle den Text übernehmen |
| `cmd+n`, `shift+cmd+return` | ein Termin mit dem heutigen Datum, sein Datum gleich offen |
| `delete`, `shift+cmd+delete` | den gewählten Termin löschen, ohne Rückfrage |
| `cmd+1`, Klick auf den Kopf „Datum“ | die Richtung umkehren, aufsteigend oder absteigend |
| `cmd+c` | den Text des gewählten Termins kopieren; ist er leer, das Datum |
| `cmd+z` | jede dieser Handlungen zurücknehmen |

**Ein neuer Termin trägt das heutige Datum** und keine Uhrzeit, damit er dort
steht, wo man weiterschreibt, und nicht als ungültiger am Ende. „Eintrag nach
oben“ und „Eintrag nach unten“ sind in der Termintabelle ausgegraut, denn ihre
Reihenfolge ist das Datum; wer einen Termin verlegen will, ändert sein Datum,
und die Zeile wandert an ihren neuen Platz und bleibt gewählt. „Aufgabe
abhaken oder öffnen“ und „PIN ändern“ bleiben hier ebenso ausgegraut.

**In einer offenen Zelle unterscheiden sich die zwei Spalten:**

- Das Datum ist eine Zeile. `return` übernimmt es, und `esc` verwirft das
  Getippte, wie in der Aufgabentabelle.
- Der Text eines Termins trägt Absätze. `return` schreibt einen
  Zeilenumbruch, und `esc` übernimmt eine geänderte Zelle, wie in der
  Notiztabelle; die Statuszeile sagt dazu, dass `cmd+z` es zurücknimmt.
- `tab` und `shift+tab` wechseln wie in der Notiztabelle, in der Folge, in der
  die Tabelle die Termine zeigt.
- Ein geändertes Datum, das nicht gilt, weist KRK ab; die Zelle bleibt offen,
  und die Statuszeile nennt die zwei gültigen Formen. Leerzeichen am Anfang und
  am Ende zählen nicht mit. Ein ungültiges Datum, das man unverändert lässt,
  prüft KRK nicht, sodass sich auch der Text eines solchen Termins bearbeiten
  lässt.
- Eine Zeile im Text, die mit `## ` beginnt, weist KRK ab, denn so beginnt der
  nächste Termin.

**`cmd+1` oder ein Klick auf den Kopf „Datum“ kehrt die Richtung um.** Ein
Pfeil im Spaltenkopf zeigt, in welche Richtung die Tabelle gerade ordnet, und
die Statuszeile meldet „Termine absteigend sortiert“ oder „Termine aufsteigend
sortiert“. Der gewählte Termin bleibt gewählt. Die Datei ändert sich dabei
nicht, und `cmd+s` ist nicht nötig. Die Richtung übersteht einen Neustart:
KRK merkt sie sich in `session.toml` als `terminrichtung`. **Dieselbe
Kombination sortiert im Dateifenster weiter nach Namen**; welche der beiden
Funktionen wirkt, entscheidet der Fokus (siehe „Die Tastaturbelegung“). Im
Hauptmenü trägt deshalb nur „Termine: Sortierrichtung umkehren“ unter „Home“
das Kürzel `⌘1`, und „Nach Name sortieren“ steht ohne Kürzel da.

**Jeder Termin des heutigen Tages ist in der Tabelle gelb hinterlegt**, mit
und ohne Uhrzeit. Eine gewählte Zeile zeigt die Farbe der Auswahl; sobald die
Auswahl weiterwandert, kommt die Hervorhebung wieder. Wer KRK über Mitternacht
offen lässt, sieht den neuen Tag, sobald das Fenster wieder in den Vordergrund
kommt oder der Fokus wechselt. Solange eine Zelle offen ist, bleibt die Tabelle
dabei, wie sie ist, damit nichts Getipptes verloren geht; nachgeholt wird beim
nächsten solchen Wechsel.

**Die Vorschau zeigt `appointments.md` gerendert und immer aufsteigend**,
gleich in welche Richtung die Tabelle im Editor gerade ordnet: den Text vor dem
ersten Termin oben, jedes Datum als Überschrift, ungültige Termine am Ende.
Hervorgehoben ist dort nichts, und geschrieben wird dabei nichts. Wer aus der
Vorschau kopiert, bekommt den Text in dieser Ordnung und nicht in der der
Datei. Eine `appointments.md` in einem anderen Ordner ist gewöhnliches Markdown,
ohne Tabelle und in der Reihenfolge der Datei.

**Mit einer eigenen `keymap.toml` trägt „Termine: Sortierrichtung umkehren“
keine Taste.** KRK hängt den Befehl beim Start unbelegt an, und über das Menü
„Home“ und den Klick auf den Kopf „Datum“ wirkt er trotzdem. Wer `cmd+1` dafür
will, weist es vor der ersten Benutzung einzeln zu: **F1**, „Termine:
Sortierrichtung umkehren“ wählen, `cmd+t`, `cmd+1` drücken, die Ansicht mit
„Fertig“ verlassen. Dass `cmd+1` schon auf „Nach Name sortieren“ liegt, ist
dabei kein Konflikt, und die Sortierung im Dateifenster behält die Taste.
**Nicht `cmd+r`**: es brächte die Taste zwar mit, setzt aber die ganze eigene
Belegung auf die Auslieferungsfassung zurück, also auch jede eigene
Tastenzuweisung.

### Geheimnisse in `secrets.txt`

**`secrets.txt` hält Einträge, die kein Programm nebenbei mitlesen soll.** Jeder
Eintrag trägt sechs Felder: Name, Protokoll, Adresse, Konto, Passwort und
Notizen. Auf der Platte steht die Datei nur verschlüsselt, und geöffnet wird
sie mit einer vierstelligen PIN. Die Felder Protokoll, Adresse, Konto und
Passwort braucht man nur für eine Verbindung (unter „Verbindungen“); ein
Eintrag, der allein einen Namen und Notizen trägt, ist ebenso gültig.

Im entschlüsselten Text ist ein Eintrag eine Notiz wie in `notes.txt`, deren
Text mit bis zu vier Feldzeilen beginnt:

```text
## NAS Büro
protocol: smb
address: nas.local/daten
account: kai
password: geheim
Zugang nur im Büronetz.
```

Die Zeile `## <Name>` beginnt den Eintrag. Danach liest KRK eine
ununterbrochene Folge von Zeilen als Felder, solange jede mit `protocol:`,
`address:`, `account:` oder `password:` beginnt (Groß- und Kleinschreibung
gleich) und ihr Schlüssel in dieser Folge noch nicht stand. Die erste andere
Zeile beendet die Felder, und alles ab ihr sind die Notizen.

**Einträge aus der Zeit vor den sechs Feldern bleiben, wie sie sind.** Ihr
Thema erscheint als Name, ihr Text als Notizen, die vier mittleren Spalten
sind leer. KRK schreibt dafür nichts um: erst wer eine Zelle eines solchen
Eintrags ändert, gibt ihm die volle Form mit allen vier Feldzeilen, leere als
`protocol:` ohne Wert. Jeder andere Eintrag bleibt dabei Byte für Byte. Beginnt
der Text eines alten Eintrags mit einer Zeile wie `password: …`, steht diese
Zeile danach in der Spalte, die ihr Wortlaut nennt; verloren geht nichts.

**Die Datei steht in der Liste wie jede andere.** Ihr Name beginnt nicht mit
einem Punkt, der Umschalter für versteckte Einträge (`shift+cmd+h`) betrifft sie
also nicht. Ein Filtertext wirkt auf ihren Namen wie auf jeden anderen. Den
Inhalt liest der Filter im Notizordner nie; mehr dazu unter „Was die PIN
schützt und was nicht“. In jedem anderen Ordner ist eine `secrets.txt` eine
gewöhnliche Datei.

**Eine `.secrets.txt` aus einer früheren Fassung benennt `f2` um.** Früher hieß
die Datei `.secrets.txt`, mit Punkt, und war ein versteckter Eintrag. Steht eine
solche im Notizordner und noch keine `secrets.txt`, gibt ihr der nächste `f2`
den neuen Namen, und die Statuszeile meldet „.secrets.txt heißt jetzt
secrets.txt“. Der Inhalt bleibt Byte für Byte, wie er war, und dieselbe PIN
öffnet ihn. Drei Fälle weichen davon ab:

- **Stehen beide Dateien schon da**, benennt KRK keine um und überschreibt
  keine. Es gilt `secrets.txt`, und die Statuszeile sagt es. `.secrets.txt` ist
  für KRK dann eine gewöhnliche versteckte Datei und öffnet sich nicht mehr mit
  der PIN. Wer ihren Inhalt statt des neuen haben will, legt `secrets.txt`
  beiseite; der nächste `f2` benennt dann `.secrets.txt` um.
- **Scheitert das Umbenennen**, bleibt `.secrets.txt` unverändert, und KRK legt
  daneben **keine** leere `secrets.txt` an. Die Statuszeile nennt den Grund, und
  der nächste `f2` versucht es wieder.
- **Lässt sich nur der alte Name nicht entfernen**, tragen beide Namen dieselbe
  Datei, und die Statuszeile nennt den Grund. Es gilt `secrets.txt`; den alten
  Namen legt man selbst weg, der Inhalt bleibt dabei unter dem neuen.

**Die Vorschau zeigt ihren Inhalt nie.** Sie liest die Datei nicht und zeigt
stattdessen den Satz „Diese Datei ist verschlüsselt und öffnet sich mit F4 und
der PIN im Editor.“ Die Sitzung merkt sich `secrets.txt` ebenfalls nie: wer
KRK mit der geöffneten Datei beendet, findet den Editor nach dem Neustart ohne
sie, und kein Blatt fragt nach einer PIN.

#### Die PIN festlegen und eingeben

`f4` oder `cmd+e` auf `secrets.txt` öffnet zuerst ein Blatt. Es fragt nach der
PIN, und zwar auf zwei Arten:

| Die Datei … | Das Blatt verlangt |
|---|---|
| hat null Bytes, ist also neu | eine neue PIN, zweimal einzugeben; beide Eingaben müssen gleich sein |
| hat einen Inhalt | die PIN, einmal |

Eine PIN besteht aus genau vier Ziffern. Solange die Eingabe keine vier Ziffern
sind oder die Wiederholung abweicht, bleibt die bestätigende Schaltfläche
abgeschaltet, und eine Zeile unter den Feldern nennt den Grund. `Esc` schließt
das Blatt, und die Datei bleibt zu.

Danach steht der Inhalt im Editor, in der Formatansicht als Tabelle mit sechs
Spalten (unter „Die Tabelle der Geheimnisse“). `ctrl+cmd+e` zeigt den Rohtext.
`cmd+s` sichert verschlüsselt und dauert so lange wie bei jeder anderen Datei;
die Wartezeit von rund einer halben Sekunde fällt allein beim Öffnen, beim
Festlegen und beim Ändern der PIN an.

**Eine falsche PIN öffnet nichts**, und das Blatt geht nicht von selbst wieder
auf. Die Statuszeile meldet „PIN falsch oder Datei verändert“. Dieselbe Meldung
kommt, wenn jemand die Datei außerhalb von KRK verändert hat, denn beides lässt
sich nicht unterscheiden. Für einen neuen Versuch wieder `f4`. Ist schon der
Anfang der Datei unlesbar, sagt die Statuszeile „Der Kopf der Datei ist
beschädigt“ und nennt den Grund. KRK sperrt nach Fehlversuchen nicht.

**Die PIN gilt, solange die Datei im Editor offen ist, und darüber hinaus
zehn Minuten.** Wer den Editor schließt, darin eine andere Datei öffnet oder
KRK beendet, verschließt den Inhalt wieder. Ein neues Öffnen innerhalb von zehn
Minuten nach der letzten PIN-Eingabe fragt aber nicht erneut; die Einzelheiten
stehen unter „Zehn Minuten ohne erneute PIN“. Den Editor nur auszublenden
(`opt+cmd+b`) verschließt nichts.

**Eine neue PIN steht erst mit dem ersten `cmd+s` in der Datei.** Wer nach dem
Festlegen schließt, ohne zu sichern, behält eine Datei mit null Bytes, und das
nächste Öffnen fragt wieder nach einer neuen PIN.

**Eine auf null Bytes abgeschnittene `secrets.txt` gilt beim nächsten Öffnen
als neue Datei.** Das Blatt fragt dann nach einer neuen PIN und nicht nach der
alten. Der frühere Inhalt ist in diesem Fall schon mit dem Abschneiden fort,
nicht erst mit der neuen PIN.

#### Die Tabelle der Geheimnisse

**Die Formatansicht zeigt sechs Spalten**: Name, Protokoll, Adresse, Konto,
Passwort und Notizen, die Einträge in der Reihenfolge der Datei. Die Griffe
sind die der Notiztabelle (unter „Notizen in der Tabelle bearbeiten“):
Doppelklick öffnet eine Zelle, `cmd+n` und `shift+cmd+return` legen einen
leeren Eintrag ans Ende, `opt+cmd+up` und `opt+cmd+down` verschieben, `delete`
und `shift+cmd+delete` löschen den ganzen Eintrag, `cmd+z` nimmt jede Handlung
zurück, und erst `cmd+s` schreibt.

**Fünf Spalten sind einzeilig, die Notizen nicht.** In Name, Protokoll,
Adresse, Konto und Passwort beendet `return` die Zelle, ein Zeilenumbruch,
etwa aus der Zwischenablage, wird abgewiesen, und **`esc` verwirft eine
geänderte Zelle**, wie die Datumszelle der Termine. In den Notizen schreibt
`return` einen Umbruch, und `esc` übernimmt die geänderte Zelle wie in der
Notiztabelle. Eine Zeile in den Notizen, die mit `## ` beginnt, weist KRK ab,
denn so beginnt der nächste Eintrag. `tab` und `shift+tab` gehen von Zelle zu
Zelle.

**Die Passwortspalte zeigt das Passwort im Klartext.** Die Rohansicht derselben
Datei zeigt es ohnehin, und eine Verdeckung allein in der Tabelle verspräche
einen Schutz, den die zweite Ansicht nicht hält. Wer neben sich jemanden
mitlesen lässt, öffnet die Datei nicht.

#### Zehn Minuten ohne erneute PIN

**Nach einer PIN-Eingabe merkt sich KRK den daraus abgeleiteten Schlüssel zehn
Minuten lang im Arbeitsspeicher.** Gemerkt wird nach dem Öffnen mit der PIN,
nach dem Festlegen einer neuen PIN, nach „PIN ändern“ und nach der PIN, die ein
Klick auf eine Verbindung oder „Verbinden…“ abfragt. In diesen zehn
Minuten öffnet `f4` die Datei ohne Blatt, und ein Klick auf eine Verbindung
und „Verbinden…“ fragen ebenso nicht. Gespeichert wird der Schlüssel nirgends
auf der Platte, und die PIN selbst merkt sich KRK nicht.

- **Die Frist zählt ab der Eingabe und verlängert sich nicht.** Ein Öffnen mit
  dem gemerkten Schlüssel setzt sie nicht neu; zehn Minuten nach der letzten
  Eingabe fragt das nächste Öffnen wieder nach der PIN. „PIN ändern“ ersetzt
  den gemerkten Schlüssel durch den neuen, und die Frist beginnt dann neu.
- **Der Ruhezustand zählt mit.** Wer den Mac zuklappt und nach einer Stunde
  wieder öffnet, wird nach der PIN gefragt. Eine verstellte Uhrzeit berührt
  die Frist nicht.
- **Eine offene Datei bleibt offen.** Läuft die Frist ab, während
  `secrets.txt` im Editor steht, bleibt sie entsperrt, bis der Editor sie
  schließt; die Frist betrifft allein das nächste Öffnen.
- **Ein Schlüssel, der nicht passt, fällt.** Hat jemand die PIN in einer
  zweiten KRK-Instanz geändert, den Notizordner gewechselt oder die Datei
  verändert, öffnet der gemerkte Schlüssel nicht. KRK vergisst ihn dann und
  zeigt das PIN-Blatt.
- **Eine Datei mit null Bytes fragt immer nach einer neuen PIN**, auch
  innerhalb der Frist, denn eine neue Datei bekommt ein neues Salz und damit
  einen neuen Schlüssel.
- **Vorzeitig vergessen lässt er sich allein durch Beenden.** Einen eigenen
  Befehl dafür gibt es nicht, und auch das Sperren des Bildschirms lässt den
  Schlüssel stehen.

**Was getilgt wird und was nicht.** Getilgt heißt: KRK überschreibt die Bytes
mit Nullen, bevor es den Speicher freigibt.

- Getilgt werden die 32 Bytes des Schlüssels, sobald eine Kopie davon fällt:
  beim Ablauf der Frist, beim Ersetzen durch „PIN ändern“, wenn er nicht passt,
  beim Beenden von KRK und wenn der Editor die Datei schließt.
- Getilgt wird das Passwort einer Verbindung, sobald es nicht mehr gebraucht
  wird: die Abschrift, die KRK bei jedem Tastendruck und beim Anlegen aus den
  Feldern des Blattes „Verbinden“ liest, und der Wert, den ein Klick aus
  `secrets.txt` liest, nach dem Einhängen; bei einer `sftp`-Verbindung, sobald
  KRK es an `ssh` gereicht hat oder der Aufbau endet.
- Getilgt wird der entschlüsselte Text von `secrets.txt`, den ein Klick auf
  eine Verbindung und „Verbinden…“ lesen, sobald ihr Gang endet, mit allen
  Passwörtern darin; ebenso der neue Text, den „Verbinden…“ verschlüsselt und
  schreibt.
- **Nicht** getilgt werden die PIN; der entschlüsselte Text, solange er im
  Editor oder in der Tabelle steht; was macOS für die Felder des Blattes
  „Verbinden“ hält, das Passwortfeld eingeschlossen; Name, Protokoll, Adresse,
  Konto und Notizen, die KRK beim Verbinden und Anlegen neben dem Text hält;
  die Kopie des Passworts, die KRK an das System zum Einhängen übergibt; bei
  `sftp` die Bytes im Puffer der Verbindung zu `ssh`, die Abschrift im kurzen
  Hilfsprozess, der das Passwort an `ssh` weitergibt und danach endet, und
  was `ssh` selbst damit tut; die
  inneren Arbeitsdaten der Verschlüsselungsverfahren; und Reste, die das
  Verschieben oder Wachsen eines Werts im Speicher hinterlässt.

Die Tilgung schützt also gegen einen Schlüssel, der nach Ablauf als Rest im
Speicher steht. Gegen ein Programm, das den Speicher von KRK liest, solange die
Datei offen ist, schützt sie nicht.

#### Eine vergessene PIN

**Eine vergessene PIN verschließt den Inhalt endgültig.** KRK hält keine
Hintertür und keine zweite Kopie. Die PIN steht nirgends außer im Gedächtnis
dessen, der sie festgelegt hat. Das Blatt sagt das bei jeder Abfrage.

#### Was die PIN schützt und was nicht

Die PIN soll verhindern, dass der Inhalt versehentlich gelesen wird. Mehr soll
sie nicht.

**Sie schützt vor:**

- Agenten und Kommandozeilenwerkzeugen, die über den Ordner lesen, etwa `cat`
  oder `grep`; sie sehen nur Binärdaten.
- Indexern wie Spotlight und Sicherungen wie Time Machine, die die Datei als
  Ganzes mitnehmen; auch sie bekommen nur das Chiffrat.
- dem Inhaltsfilter von KRK: im Notizordner liest „Content“ die Datei
  nie. Die tiefe Suche
  mit „Content“ aus einem übergeordneten Ordner liest sie wie jede andere Datei
  darunter, findet dort aber nur Chiffrat und keinen Eintrag.
- dem beiläufigen Weitergeben, wenn der Ordner mitkopiert oder geteilt wird.

**Sie schützt nicht vor:**

- jemandem mit einer Kopie der Datei, der gezielt alle zehntausend PINs
  durchprobiert. Jeder Versuch kostet auf dem Referenz-Mac rund eine halbe
  Sekunde, alle zusammen also rund anderthalb Stunden, auf einem schnelleren
  Rechner oder mit mehreren Kernen zugleich weniger. Wer den Notizordner in
  einen synchronisierten Speicher legt, legt dort auch eine solche Kopie ab.
- einem Programm, das mitliest, solange die Datei im Editor offen ist; dann
  steht der Inhalt im Klartext im Speicher. Ebenso während der zehn Minuten,
  in denen KRK den Schlüssel gemerkt hält: wer dann den Speicher liest, findet
  den Schlüssel.
- Schadsoftware, die unter dem eigenen Benutzerkonto läuft.

Was KRK im Speicher tilgt und was nicht, steht unter „Zehn Minuten ohne
erneute PIN“.

**Das Passwort einer Verbindung geht an den Rechner, an dem man sich
anmeldet.** Ein Klick auf eine Verbindung, die einhängt, reicht Konto und
Passwort an das System weiter, und das System schickt sie an den Server. Eine
`sftp`-Verbindung reicht das Passwort an `ssh`, und `ssh` schickt es an den
Server. Der Eintrag in `secrets.txt` bestimmt, wohin. Eine Verbindung, die KRK
an ein anderes Programm übergibt, bekommt nie ein Passwort.

**Kopierter Text liegt im Klartext in der Zwischenablage.** `cmd+c` aus der
Tabelle oder aus dem Rohtext ist erlaubt. Danach kann jedes Programm des
Benutzerkontos den Text lesen, ein Agent im Terminal etwa mit `pbpaste`. Ein
Verlauf der Zwischenablage behält ihn, und bei eingeschalteter Übergabe an
andere Geräte (Handoff) erreicht er auch diese. Wer ein Geheimnis kopiert hat,
kopiert danach etwas Harmloses darüber.

**Gezogener Text wird eine Klartextdatei.** Markierten Text aus dem Editor auf
den Schreibtisch oder in ein Finder-Fenster zu ziehen ist erlaubt wie das
Kopieren. Der Finder legt daraus aber eine Datei an, einen Textausschnitt mit
dem Klartext. Spotlight findet sie, und sie bleibt liegen, bis man sie löscht.
Wer ein Geheimnis gezogen hat, legt den Ausschnitt danach in den Papierkorb und
leert ihn.

**Eine Textmarke gibt es in `secrets.txt` nicht.** `cmd+d` legt in der
Rohansicht keine an, und die Statuszeile sagt, warum: die Textmarke schriebe
eine Zeile der Geheimnisse im Klartext in die Lesezeichen. In der Tabelle legt
`cmd+d` stattdessen ein Lesezeichen unter „Verbindungen“ auf den gewählten
Eintrag an, und das trägt allein dessen Namen (unter „Verbindungen“, „Eine
Verbindung anlegen“).

#### Die PIN ändern

**„PIN ändern“ liegt auf `shift+cmd+p`** und steht im Hauptmenü „Home“. Der
Befehl wirkt nur, wenn der Editor `secrets.txt` entsperrt hält, der Fokus im
Editor steht und die Datei **schon einmal gesichert** ist. Eine Datei, deren PIN
eben erst festgelegt und nie gesichert wurde, trägt noch keine PIN, die sich
ändern ließe; sonst ist der Eintrag ausgegraut.

Das Blatt fragt die alte PIN einmal und die neue zweimal. Ob die alte stimmt,
prüft KRK erst nach dem Blatt; stimmt sie nicht, meldet die Statuszeile „die
alte PIN stimmt nicht; die PIN bleibt, wie sie war“. Hat sich die Datei außerhalb
von KRK geändert, weist der Befehl ab wie das Sichern.

**Ungesicherte Änderungen bleiben dabei ungesichert.** Der Befehl verschlüsselt
den Stand, der auf der Platte steht, neu mit der neuen PIN. Was im Editor seit
dem letzten `cmd+s` dazugekommen ist, bleibt im Editor und geht erst mit dem
nächsten `cmd+s` auf die Platte, dann unter der neuen PIN. Ab dem Ende des
Befehls öffnet allein die neue PIN die Datei, und für die nächsten zehn
Minuten merkt sich KRK den Schlüssel der neuen PIN statt des alten.

**Mit einer eigenen `keymap.toml` trägt „PIN ändern“ keine Taste.** KRK hängt
den Befehl beim Start unbelegt an, und über das Hauptmenü „Home“ ist er
trotzdem erreichbar, ohne jeden Handgriff an der eigenen Belegung. Eine Taste
bekommt er wie die Tabellenbefehle, einzeln in der Belegungsansicht: **F1**,
den Befehl wählen, `cmd+t`, die Taste drücken, „Fertig“. `cmd+r` ist dafür nicht
nötig und setzt die ganze eigene Belegung zurück.

Wie die Datei aufgebaut ist und wie man sie ohne KRK entschlüsselt, steht in
`README.md` unter „Das Dateiformat von `secrets.txt`“.

### Verbindungen

**Eine Verbindung ist ein Eintrag in `secrets.txt` und ein Lesezeichen in der
Leiste, das ihn beim Namen nennt.** Ein Klick auf das Lesezeichen liest den
Eintrag und stellt die Verbindung her, etwa zu einer Freigabe auf einem
Netzlaufwerk oder, über `sftp`, zu einem Ordner auf einem Server, der dann im
Dateifenster steht wie ein lokaler.

#### Eine Verbindung anlegen

**„Verbinden…“ steht im Kontextmenü der Dateiliste**, gleich was angeklickt
ist. Es öffnet ein Blatt mit sechs Feldern, denselben wie in der Tabelle; das
Passwortfeld zeigt Punkte statt Zeichen, und die Notizen sind hier eine Zeile
(Absätze schreibt man danach in der Tabelle). „Anlegen“ bleibt abgeschaltet,
bis der Name nicht leer ist, das Protokoll eines aus der Tabelle unten ist, die
Adresse nicht leer ist und kein `://` enthält, Protokoll und Adresse zusammen
eine gültige Adresse ergeben (kein Leerzeichen im Rechnernamen, ein Port nur
aus Ziffern) und kein Feld einen Zeilenumbruch trägt. Leerzeichen und Umlaute
im Pfad sind erlaubt, ebenso `#` und `?`: beim Einhängen gehören sie zum
Ordnernamen. Die Zeile unter den Feldern nennt den Grund für einen
Zeilenumbruch und für eine ungültige Adresse schon beim Tippen, denn
Weitertippen heilt beide nicht; für einen leeren Namen, eine leere Adresse und
ein unbekanntes Protokoll erst, wenn man die Eingabetaste drückt.

Nach „Anlegen“ fragt KRK nach der PIN, außer der Schlüssel ist noch gemerkt.
Dann hängt es den Eintrag in der vollen Form ans Ende von `secrets.txt` und legt
das Lesezeichen in der Gruppe „Verbindungen“ an. Was dabei schiefgehen kann:

| Lage | Was geschieht |
|---|---|
| Der Name steht in `secrets.txt` schon | Das Blatt geht mit allen Feldern wieder auf und nennt den Grund; geschrieben ist nichts. |
| Die PIN ist falsch | Das PIN-Blatt geht wieder auf; die Felder bleiben erhalten. |
| `secrets.txt` fehlt | KRK legt sie an wie `f2`, ohne sie zu öffnen, und fragt nach einer neuen PIN, zweimal. |
| `secrets.txt` hat im Editor ungesicherte Änderungen | „Verbinden…“ weist ab, bevor das Blatt aufgeht: erst sichern. |
| `secrets.txt` ist im Editor mit einer eben festgelegten PIN offen und noch nie gesichert | „Verbinden…“ weist ab, bevor das Blatt aufgeht: erst sichern. Sonst legte es eine zweite PIN fest, und der Editor öffnete die Datei danach nicht mehr. |
| `secrets.txt` wird im Editor gerade geöffnet | „Verbinden…“ weist ab: erst warten, bis sie offen ist. |
| Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen | Die Statuszeile sagt genau das. |

Die drei Lagen des Editors aus der Tabelle prüft KRK zweimal: bevor das Blatt
aufgeht und noch einmal nach „Anlegen“ und der PIN, unmittelbar vor dem
Schreiben. Ist der Editor in der Zwischenzeit in eine davon geraten, schreibt
KRK nichts und nennt den Grund in der Statuszeile; die Felder des Blattes sind
dann verloren.

Hält der Editor `secrets.txt` ohne ungesicherte Änderungen, lädt er sie nach
dem Anlegen neu und zeigt den neuen Eintrag. Umgekehrt fragt der Editor nicht
nach der PIN, solange „Verbinden…“ schreibt oder eine Verbindung `secrets.txt`
liest: die Statuszeile sagt, dass man erst warten soll.

**Ein Eintrag, der schon in `secrets.txt` steht, bekommt sein Lesezeichen mit
`cmd+d` in der Tabelle.** Man öffnet `secrets.txt` im Editor in der
Formatansicht, wählt die Zeile des Eintrags und drückt `cmd+d` (im Menü
„Lesezeichen anlegen“). Ein Blatt geht nicht auf: das Lesezeichen heißt wie der
Eintrag und steht danach unten in der Gruppe „Verbindungen“; umbenennen lässt
es sich wie jedes andere mit `ctrl+u`. Eine laufende Zelle übernimmt `cmd+d`
vorher. In der Rohansicht legt `cmd+d` nichts an. Kein Lesezeichen entsteht,
und die Statuszeile nennt den Grund, in diesen Lagen:

| Lage | Was geschieht |
|---|---|
| Keine Zeile gewählt | Die Statuszeile bittet um eine Auswahl. |
| `secrets.txt` hat im Editor ungesicherte Änderungen oder eine noch nie gesicherte PIN | Erst sichern: ein Klick auf das Lesezeichen liest die Datei von der Platte und fände den Eintrag dort noch nicht. |
| `secrets.txt` wird im Editor gerade geöffnet | Erst warten, bis sie offen ist. |
| Der Eintrag hat keinen Namen | Ein Lesezeichen findet seinen Eintrag allein über den Namen. |
| Der Eintrag nennt kein Protokoll oder eines, das KRK nicht kennt | Ein Klick verbände nicht; die Statuszeile nennt die bekannten Protokolle. |
| Mehrere Einträge tragen denselben Namen | Ein Klick fände keinen davon; erst einen umbenennen. |
| Unter „Verbindungen“ zeigt schon ein Lesezeichen auf diesen Namen | Ein zweites entsteht nicht, auch wenn das erste umbenannt ist. |

**In `bookmarks.toml` steht von der Verbindung allein ihr Name.** Protokoll,
Adresse, Konto, Passwort und Notizen stehen nur verschlüsselt in
`secrets.txt`, gleich ob das Lesezeichen über „Verbinden…“ oder über `cmd+d`
entstanden ist.

#### Eine Verbindung herstellen

**Ein Mausklick auf die Zeile verbindet**, auch auf eine schon gewählte. Von
der Tastatur verbindet **„Verbindung herstellen“** auf `opt+return`, mit dem
Fokus in der Leiste. **Die Pfeiltasten wählen nur**: eine Auswahl, die mit
`up` und `down` über eine Verbindung läuft, stellt keine her. Ordner und
Textmarken öffnen dagegen weiter schon bei der Auswahl.

KRK liest `secrets.txt` dafür mit dem gemerkten Schlüssel oder fragt nach der
PIN, sucht den Eintrag mit dem Namen der Verbindung und verbindet je nach
Protokoll auf einem von drei Wegen. Gelesen wird der gesicherte Stand auf der
Platte, nicht ein ungesicherter im Editor. Groß- und Kleinschreibung des
Protokolls zählen nicht.

| Protokoll | Weg | Was geschieht |
|---|---|---|
| `smb`, `afp`, `nfs`, `ftp` | einhängen | KRK hängt `<protokoll>://<adresse>` ein, mit Konto und Passwort |
| `webdav`, `webdavs` | einhängen | ebenso, über `http://` beziehungsweise `https://` |
| `http`, `https` | an das System | der Browser öffnet `<protokoll>://<adresse>`, ohne Konto und ohne Passwort |
| `sftp` | im Dateifenster | KRK verbindet selbst über `ssh` und zeigt den Ordner auf dem Server im aktiven Dateifenster; siehe „Ordner auf einem Server“ |
| `ssh`, `vnc` | an das System | das zuständige Programm öffnet `<protokoll>://<konto>@<adresse>`, ohne Passwort; ohne Konto fehlt `<konto>@` |

**Das Passwort verlässt KRK allein beim Einhängen und bei `sftp`**: beim
Einhängen als eigener Wert an das System, bei `sftp` an `ssh`, und nie in einer
Adresse. Die Programme, an die KRK übergibt, fragen selbst danach, wenn sie es
brauchen. Die Adresse trägt kein Schema: `nas.local/daten`
und nicht `smb://nas.local/daten`. **Beim Einhängen gehören `#` und `?` zum
Pfad**: `nas.local/Daten#1` hängt den Ordner `Daten#1` ein und nicht `Daten`.
An das System übergeben bleiben sie, was sie in einer Webadresse sind, also
beginnt mit `?` die Anfrage und mit `#` die Sprungmarke.

Nach einem gelungenen Einhängen zeigt das aktive Dateifenster den
Einhängepunkt. Ist die Freigabe schon eingehängt, zeigt KRK den vorhandenen
Einhängepunkt, oder meldet „schon verbunden“, wenn es ihn nicht findet. Alle
anderen Ausgänge meldet die Statuszeile:

- kein Eintrag mit diesem Namen, auch wenn `secrets.txt` fehlt oder leer ist
  (dann fragt KRK gar nicht erst nach der PIN);
- **mehrere Einträge mit diesem Namen**: KRK verbindet dann nicht, denn es
  wüsste nicht, mit welchem; Abhilfe ist ein eindeutiger Name in
  `secrets.txt`;
- ein Protokoll, das KRK nicht kennt;
- eine abgewiesene Anmeldung, ein nicht erreichbarer Rechner, sonst die
  Fehlernummer des Systems.

Ein zweiter Klick, während eine Verbindung noch hergestellt wird, meldet das
und startet keine zweite. Das Einhängen läuft im Hintergrund; KRK bleibt dabei
bedienbar, abbrechen lässt es sich aber nicht.

**Mit einer eigenen `keymap.toml` trägt „Verbindung herstellen“ keine Taste.**
Der Klick wirkt trotzdem. `opt+return` weist man einzeln zu: **F1**, den Befehl
in der Gruppe „Leiste und Fokus“ wählen, `cmd+t`, `opt+return` drücken,
„Fertig“.

#### Ordner auf einem Server

**Eine `sftp`-Verbindung zeigt den Ordner auf dem Server im aktiven
Dateifenster**, und Kopieren, Verschieben, Umbenennen, Anlegen und Löschen
wirken dort wie in einem lokalen Ordner. KRK spricht dafür selbst mit dem
Server, über das `ssh` des Systems (`/usr/bin/ssh`); ein weiteres Programm
braucht es nicht, und was in `~/.ssh/config` steht, gilt weiter.

**Die Adresse lautet `rechner[:port][/pfad]`**, das Konto steht in seinem
eigenen Feld und nicht als `konto@` vor dem Rechner. Eine IPv6-Adresse mit Port
schreibt man `[adresse]:port`.

| Adresse | Was das Fenster zeigt |
|---|---|
| `ssh.example.org` oder `ssh.example.org:22` | das Anmeldeverzeichnis |
| `ssh.example.org/~/htdocs` | `htdocs` unter dem Anmeldeverzeichnis |
| `ssh.example.org/htdocs` | `/htdocs`, vom obersten Ordner des Servers aus |

Das Blatt „Verbinden…“ weist schon beim Anlegen ab, was `ssh` anders läse als
gemeint: einen leeren Rechner, einen Rechner mit `-` am Anfang, einen Port
außerhalb von 1 bis 65535 und jeden Rechner mit einem Zeichen, das in keinem
Rechnernamen und keiner IP-Adresse steht, etwa ein Leerzeichen, `@`, `,`, `*`,
`?` oder `!`. Erlaubt sind Buchstaben ohne Umlaut, Ziffern, `.`, `-` und `_`,
eine IPv6-Adresse in eckigen Klammern. Ein Name mit `,` oder `*` würde beim
Annehmen seines Schlüssels sonst in `~/.ssh/known_hosts` für fremde Rechner
gelten. Steht am angegebenen
Pfad kein Ordner, zeigt das Fenster das Anmeldeverzeichnis, und die
Statuszeile sagt, welcher Pfad gefehlt hat.

**Angemeldet wird mit dem Passwort aus dem Eintrag oder, bei leerem
Passwortfeld, mit Schlüssel.** Steht ein Passwort im Eintrag, meldet sich KRK
damit an und versucht keinen Schlüssel. Ist das Feld leer, meldet sich `ssh`
mit einem Schlüssel oder über den Agenten an, wie im Terminal, und fragt dabei
nach nichts: ein Schlüssel mit Passphrase muss dafür im Agenten liegen. Fragt
der Server nach dem Passwort ein zweites Mal, etwa nach einem zweiten Faktor,
scheitert die Anmeldung; für einen solchen Server ist der Schlüssel im Agenten
mit leerem Passwortfeld der Weg.

**Das Passwort steht dabei in keinem Befehl, keiner Datei und keiner
Umgebungsvariable.** KRK startet sich selbst als Anmeldehilfe für `ssh`, und
die Hilfe holt das Passwort genau einmal über eine Verbindung in einem Ordner
ab, den allein das eigene Benutzerkonto öffnen kann. **Wer dort fragt, prüft
KRK nicht**: ein Programm, das unter demselben Benutzerkonto läuft und während
des Aufbaus zuerst fragt, bekommt das Passwort. Danach bekommt niemand mehr
etwas, auch `ssh` nicht, und die Anmeldung scheitert sichtbar. Ein solches
Programm könnte ebenso `secrets.txt` lesen, solange KRK den Schlüssel gemerkt
hält; was getilgt wird und was nicht, steht unter „Zehn Minuten ohne erneute
PIN“.

**Beim ersten Verbinden mit einem Rechner fragt KRK, ob es seinen Schlüssel
annehmen soll.** Kennt `ssh` den Rechner noch nicht, holt KRK dessen Schlüssel
und zeigt ein Blatt mit Rechner, Port und je Schlüssel dem Fingerabdruck
`SHA256:…`. Diese Fingerabdrücke vergleicht man mit denen, die der Betreiber des
Servers nennt. **Return und Esc brechen ab, `cmd+return` nimmt an**: KRK trägt
genau die gezeigten Schlüssel in die erste Datei ein, die `ssh` für bekannte
Rechner nennt (`UserKnownHostsFile`, ab Werk `~/.ssh/known_hosts`), und
verbindet. Eingetragen wird angehängt, die Datei wird nie neu geschrieben;
fehlen Ordner oder Datei, legt KRK sie mit den Rechten an, die `ssh` erwartet.
Nach „Abbrechen“ sagt die Statuszeile, dass der Schlüssel unbekannt ist, und
verbunden wird nicht.

- **Ein Rechner hinter `ProxyJump` oder `ProxyCommand`** aus `~/.ssh/config`
  lässt sich so nicht abfragen; die Statuszeile nennt den Grund. Den Schlüssel
  trägt man dann selbst ein, etwa mit einem ersten `ssh` im Terminal.
- **Mit `UserKnownHostsFile none`** gibt es kein Blatt, denn KRK wüsste nicht,
  wohin es eintragen soll; die Statuszeile sagt, dass der Schlüssel unbekannt
  ist.
- **Hat sich der Schlüssel eines bekannten Rechners geändert, weist KRK die
  Verbindung ab** und zeigt die Zeile, mit der `ssh` abgelehnt hat. Ein
  Überschreiben bietet KRK nicht an. Wer sicher ist, dass der neue Schlüssel
  stimmt, entfernt den alten im Terminal mit `ssh-keygen -R <rechner>` und
  verbindet neu.

**Was im Ordner auf dem Server geht:**

- **Bewegen** wie lokal: hineingehen, aufwärts bis `/`, ein neuer Tab zeigt
  denselben Ordner, „Ordner angleichen“ (`opt+cmd+s`) stellt das andere Fenster
  auf ihn. Die Pfadeingabe (`shift+cmd+g`) nimmt einen Pfad auf demselben
  Server, absolut oder mit `~/` vom Anmeldeverzeichnis aus. Zurück zu einem
  lokalen Ordner geht es über ein Lesezeichen, die Geräteleiste oder „Ordner
  angleichen“ aus einem lokalen Fenster. **„Zur Zwischenablage springen“
  (`opt+cmd+g`) ist im Ordner auf dem Server ausgegraut**: der Pfad in der
  Zwischenablage ist ein Pfad auf dem Mac und nie einer auf dem Server, und
  KRK liest ihn dort deshalb gar nicht.
- **Der Fenstertitel** nennt `konto@rechner:/pfad`, mit abweichendem Port
  `konto@rechner:port:/pfad`.
- **Kopieren und Verschieben** in jede Richtung: vom Mac auf den Server, vom
  Server auf den Mac, zwischen zwei Ordnern desselben Servers und zwischen
  zwei Servern, über „Kopieren“ (`f5`) und „Verschieben“ (`f6`) ins andere
  Dateifenster oder durch Ziehen aus dem Finder auf das Fenster. Aus einem
  Dateifenster heraus lässt sich nichts ziehen. Fortschritt, Abbruch und das
  Blatt bei einem vorhandenen Namen sind dieselben wie lokal; eine
  abgebrochene Datei wird am Ziel entfernt. **„Überschreiben“ legt den Ersatz
  erst vollständig neben das Ziel, benennt dann das Ziel um, gibt dem Ersatz
  seinen Namen und entfernt das alte Ziel zuletzt**: kommt der Ersatz nicht
  vollständig an, bleibt das Ziel, wie es war, und die Abschlussliste sagt es.
  Lässt sich vom alten Ziel etwas nicht entfernen, etwa ein Ordner ohne
  Schreibrecht darin, steht der Ersatz trotzdem am Ziel, und die
  Abschlussliste nennt den Namen `.krk-alt-…`, unter dem der Rest liegt.
  Lehnt der Server nur Rechte und Änderungszeit ab, wird trotzdem ersetzt,
  mit einem Hinweis. **Beim Verschieben zwischen Mac und Server oder zwischen
  zwei Servern wird die Quelle erst entfernt, wenn alles angekommen ist**;
  ein solcher Hinweis oder ein Rest `.krk-alt-…` hält sie nicht auf, denn
  der Inhalt steht dann vollständig am Ziel, und die Abschlussliste nennt
  den Hinweis unter dem Namen am Ziel. Fehlt dagegen ein Teil, bleibt die
  Quelle mit dem Grund „nicht vollständig kopiert, in der Quelle geblieben“.
  Bricht die Verbindung ab, während KRK beim Überschreiben einen Eintrag
  umbenennt, weiß KRK nicht, ob der Server das Umbenennen noch ausgeführt
  hat; die Abschlussliste nennt dann beide Namen, unter denen der Eintrag
  liegen kann. Benennen Quelle und Ziel auf dem Server über eine Verknüpfung
  denselben Eintrag, wird nichts überschrieben. Erreichen beide Seiten
  denselben Speicher auf zwei Wegen, etwa ein NAS, das eingehängt und
  zugleich über sftp geöffnet ist, fragt KRK zwar nach, verliert aber
  nichts: beim Kopieren bleibt der Inhalt am Ziel erhalten, beim Verschieben
  bleibt die Quelle, und die Abschlussliste sagt es. **Liegt die Seite auf
  dem Mac auf einem Netzlaufwerk, ersetzt „Verschieben“ ein vorhandenes Ziel
  nie**, auch wenn es ein anderer Eintrag ist: dort kann der Mac nicht
  sicher sagen, ob die Quelle noch unter ihrem Namen steht. Die
  Abschlussliste nennt den Eintrag; dann erst kopieren und danach die Quelle
  löschen. **Mit gehen Rechte und Änderungszeit, nicht aber erweiterte
  Attribute**: Finder-Etiketten,
  Kommentare und andere Zusatzdaten einer Datei gehen auf dem Weg zum Server
  verloren. Verknüpfungen bleiben Verknüpfungen; auf einen Server, der sich
  nicht als OpenSSH ausweist, legt KRK keine an und nennt sie in der
  Abschlussliste. Die Abschlussliste nennt Einträge auf einem Server als
  `konto@rechner:/pfad`.
- **Umbenennen, neuer Ordner und neue Datei** laufen als Vorgang wie ein
  Kopieren, also einer zur Zeit. Ein vergebener Name wird nie überschrieben,
  und die Statuszeile meldet ihn wie lokal. Auf einem Server, dessen
  Dateisystem Groß- und Kleinschreibung nicht unterscheidet, scheitert deshalb
  auch das Umbenennen von `a` in `A`.
- **Löschen ist endgültig**, nach der eigenen Rückfrage aus „Der
  Dateilistenfilter“: ein Ordner auf einem Server hat keinen Papierkorb.
- **Der Filtertext** filtert über die Namen des angezeigten Ordners.

**Was im Ordner auf dem Server nicht geht:**

- **Ausgegraut** sind dort „Terminal öffnen“, „Mit dem Standardprogramm
  öffnen“, beide Pfadkopien, „Bearbeiten“ und das Stapelumbenennen, dazu mit
  dem Fokus im Dateifenster „Teilen“, „Lesezeichen anlegen“ und der Rundweg
  (`cmd+e`), außerdem „Zur Zwischenablage springen“. Die Ankreuzfelder „Deep“ und „Content“ lassen sich nicht
  bedienen.
- **Das Kontextmenü graut nichts aus**, sondern meldet: „Öffnen mit“, Zippen,
  Entpacken, Duplizieren, „Im Finder öffnen“, „Im Finder anzeigen“ und
  „Auswerfen“ sagen in der Statuszeile „Das geht an einem Ort auf einem Server
  nicht.“ Dasselbe sagen `cmd+c` und `cmd+x`, die von dort keine
  Dateiverweise ablegen, und ein Doppelklick auf eine Datei. Ein Doppelklick
  auf eine Verknüpfung tritt ein, wenn sie auf einen Ordner zeigt. „Teilen“
  steht dort gar nicht im Kontextmenü.
- **Die Vorschau** zeigt Name, Größe, Änderungszeit und Typ aus der Liste und
  liest keinen Inhalt. Der Git-Bereich fragt einen Ordner auf einem Server
  nicht.
- **Änderungen, die jemand anders auf dem Server macht, erscheinen nicht von
  selbst.** KRK liest den Ordner nach jedem eigenen Vorgang neu und sonst
  beim nächsten Betreten.
- **Große Ordner und große Dateien brauchen länger als lokal**, weil jede
  Antwort über die Leitung kommt. KRK bleibt dabei bedienbar; die
  Geschwindigkeitszusagen gelten für lokale Ordner.

**Eine Verbindung je Server.** Ein zweiter Klick auf dieselbe Verbindung, ein
neuer Tab und das andere Fenster nutzen die bestehende. **Reißt sie ab, baut
KRK sie nicht von selbst neu auf**, denn dafür bräuchte es unter Umständen die
PIN: der Tab sagt, dass die Verbindung getrennt ist, und ein Klick auf die
Verbindung verbindet neu; danach liest jeder Tab auf diesem Server neu.

**Nach einem Neustart steht ein Tab, der einen Ordner auf einem Server zeigte,
wieder an dem lokalen Ordner, den er vor dem Verbinden zeigte.** KRK verbindet
beim Start nicht und fragt dort nicht nach der PIN.

#### Zurück auf eine ältere Fassung

**Eine Fassung von KRK von vor den Verbindungen versteht eine `bookmarks.toml`
mit einer Verbindung nicht.** Sie hält die Datei für beschädigt, legt ihren
Inhalt als `bookmarks.toml.beschaedigt` daneben und startet ohne Lesezeichen;
was sie danach in `bookmarks.toml` schreibt, kennt die früheren Lesezeichen
nicht mehr. Steht schon eine `bookmarks.toml.beschaedigt` aus einem früheren
Anlass da, bleibt die ältere und die neue Kopie entfällt. Wer zur neuen Fassung
zurückkehrt, beendet KRK, legt `bookmarks.toml` beiseite und benennt
`bookmarks.toml.beschaedigt` in `bookmarks.toml` um. Wer vorher weiß, dass er
zurückgeht, löscht die Verbindungen in der Leiste zuerst; die Einträge in
`secrets.txt` bleiben dabei stehen.
