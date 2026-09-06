# Analysis: neunzehn offene Entscheidungsfragen, zur Vorlage im Chat aufbereitet

**Date:** 2026-09-06 21:02
**Type:** Document Study
**Status:** Complete
**Requested by:** user

## Question

Neunzehn offene Entscheidungsdatensätze aus zehn Runden sollen so verdichtet werden, dass der
Nutzer sie im Chat nacheinander beantworten kann: Frage in einem Satz, Möglichkeiten aus dem
Datensatz, Empfehlung, und die Auskunft, ob der heutige Baum die Frage inzwischen selbst
beantwortet hat.

## Scope

Die neunzehn im Auftrag genannten Datensätze, vollständig gelesen. Dazu je Frage die im
Datensatz genannten Quelldateien und der heutige Baum.

Baumstand: `8276170`, 2026-09-06, Zweig `main`, `## main...origin/main [voraus 17]`. Jede
Gegenwartsaussage unten ist auf diesen Stand datiert.

**Keiner der neunzehn Datensätze trägt eine Zeile `Answer located:`.** Nachgeprüft über alle
Entscheidungsspeicher; die zwei Datensätze im Baum, die eine solche Zeile führen, sind andere.

## Findings

Der vollständige Vorlageblock, so wie er dem Nutzer vorgelegt wird:

---

### 1. Wie spricht KRK eine Systemfunktion an, die es erst in einer neueren macOS-Version gibt?

KRK setzt auf macOS 15 auf und soll bis macOS 26 laufen. Rust warnt nicht, wenn Code eine
Funktion anspricht, die es auf dem älteren System nicht gibt; dort stürzt die Anwendung ab.

a) Zur Laufzeit fragen, ob es die Funktion gibt, und je Stelle einen Ersatzweg danebenstellen — schließt eine Prüfung durch den Übersetzer aus; ein vergessener Schutz fällt erst auf dem alten Gerät auf, und dort als Absturz.
b) Zusätzlich die betroffenen Systembibliotheken schwach binden — nötig allein für C-Funktionen; die Einstellung steht im Bauwerkzeug und ist im Quelltext unsichtbar.
c) Warten, bis die Rust-Bibliothek `objc2` das selbst kann — bindet das Projekt an einen fremden Zeitplan ohne Datum.
d) Eine eigene kleine Abfragefunktion im Projekt, durch die jede solche Stelle geht — erzwingt trotzdem nichts und löst das Bindungsproblem aus b nicht mit.

**Empfehlung:** a zusammen mit b, sobald die erste solche Funktion wirklich gebraucht wird, und d als Zugabe, weil die Laufzeitabfrage und das Binden zwei verschiedene Fragen beantworten und keine Alternativen sind.
**Noch aktuell?** ja, mit einer Verschiebung seit der Fragestellung: die Laufzeitabfrage ist inzwischen Auslieferungscode und keine Prüfhilfe mehr (`crates/krk-ui/src/appkit/textautomatik.rs:161` und `:217`, für eine erst ab macOS 15.4 zugesagte Einstellung). Schwaches Binden gibt es weiterhin nirgends im Baum.
**Datei:** `260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`

### 2. Wie kommt KRK für den Geschwindigkeits-Abnahmelauf in den Vordergrund?

Die zehn Zeitzusagen lassen sich nur messen, wenn KRK die vorderste Anwendung ist; aus dem
Hintergrund gestartet weist es jeden fokusgebundenen Befehl ab. Heute bricht der Lauf mit einer
Meldung ab, statt falsche Zahlen zu liefern.

a) Es bleibt beim Abbruch, und die Messanleitung schreibt die Bedingung hin — schließt jeden unbeaufsichtigten Lauf aus, etwa über einen Auftragsplaner.
b) Der Lauf startet das fertige Programm über `open`, das System holt es nach vorn — schließt die heutige Ausgabe über die Standardausgabe aus; die Messzeilen bräuchten einen zweiten Weg, etwa eine Datei.
c) Der Lauf wartet auf den Bediener statt abzubrechen — schließt den unbeaufsichtigten Lauf ebenso aus und schiebt eine Wartezeit mitten in eine Messreihe.

**Empfehlung:** a, weil der Abbruch gebaut ist und b sich erst lohnt, wenn die Messreihe je ohne dich laufen soll.
**Noch aktuell?** ja. Der Abbruch steht unverändert (`crates/krk-ui/src/messmodus.rs:110` und `:752`), und der letzte vollständige Messlauf stammt vom 260810; jede seither geschlossene Runde ist gegen die zehn Zusagen ungemessen.
**Datei:** `260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`

### 3. Nach welcher Sprache sortiert KRK die Dateinamen?

KRK sortiert nach der sprachneutralen Grundordnung von Unicode. Für Deutsch ist das genau die
erwartete Reihenfolge; im Schwedischen etwa stünde `ä` hinter `z` statt davor.

a) Bei der neutralen Grundordnung bleiben — schließt aus, dass ein Nutzer mit skandinavischer Systemsprache seine gewohnte Ordnung sieht.
b) Der Systemsprache folgen — schließt den heutigen Schichtenschnitt aus: die Ordnung käme aus der Oberfläche in den Kern, und der Sortierschlüssel müsste bei jedem Sprachwechsel neu gebaut werden. Die zwei gemessenen Zeitzusagen für das Lesen hängen daran.
c) Einstellbar machen — eine Einstellung mehr für einen Fall, der bei deutscher Oberfläche nie eintritt, mit derselben Schichtenfrage wie b.

**Empfehlung:** a, weil die Änderung die zwei gemessenen Zusagen berührt und es bisher keinen Nutzer gibt, der die andere Ordnung braucht.
**Noch aktuell?** ja. `crates/krk-core/src/verzeichnis/kollation.rs` fährt im Modulkopf unter „Welche Ordnung" unverändert auf der Grundordnung und verweist die Frage ausdrücklich an einen eigenen Entscheid.
**Datei:** `260806-1730_o_welche-sprache-bestimmt-die-sortierordnung.md`

### 4. Soll die Dateiliste während eines Stapel-Umbenennens weiterhin stehenbleiben statt sich laufend zu erneuern?

Das Umbenennen im Stapel löst je Datei eine Änderungsmeldung des Systems aus. KRK hält deshalb
die Auffrischung während dieses einen Vorgangs an. Der Defekt, für den die Regel eingezogen
wurde, ist inzwischen an anderer Stelle behoben.

a) Der Aufschub bleibt, seine Begründung wird nachgezogen — schließt aus, dass eine Fallunterscheidung über alle Vorgangsarten wegfällt; sie greift auch dort, wo sie nichts mehr abfängt.
b) Der Aufschub fällt ganz weg — schließt den Schutz für sehr große Ordner aus: ab rund 60.000 Einträgen zeigte die Liste während des ganzen Vorgangs nur den unsortierten Anfang.
c) Der Aufschub hängt künftig an der Ordnergröße statt an der Vorgangsart — schließt die einfache Regel aus; eine Schwelle muss gemessen, begründet und gepflegt werden und hängt am Gerät.

**Empfehlung:** a, weil der Aufschub weiter eine echte Fehlfunktion abfängt und b sie gegen eine gesparte Zeile eintauscht.
**Noch aktuell?** ja, mit gewachsenem Preis. Die Fallunterscheidung deckt inzwischen sechs Vorgangsarten statt fünf, weil Zippen und Entpacken dazugekommen sind (`crates/krk-ui/src/auffrischung.rs:332-344`), und jede weitere Art muss dort wieder eingeordnet werden. Die Begründung im Code ist nachgezogen, aber anders formuliert als der Datensatz vorschlägt.
**Datei:** `260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`

### 5. Soll eine Markierung bestehen bleiben, wenn sich der angezeigte Ordner von außen ändert?

Die Auswahl übersteht ein Neulesen des Ordners, weil sie am Dateinamen hängt. Die Markierung
hängt an Zeilennummern und fällt deshalb bei jedem Neulesen weg. Wer acht Dateien markiert und
dabei eine fremde Änderung im Ordner erlebt, fängt von vorn an.

a) So lassen — schließt das ruhige Markieren in einem Ordner aus, in dem sich etwas rührt.
b) Die Markierung wie die Auswahl über die Namen tragen — schließt die einfache Kostenrechnung aus: bei 100.000 markierten Einträgen sind das 100.000 Zeichenketten zu kopieren und nachzuschlagen, und das fällt in die Spanne, die zwei Zeitzusagen messen.
c) Nur beim Neulesen tragen, nicht beim Ordnerwechsel — zwei Wege statt einem, und wenn der teure Fall eintritt, kostet er dasselbe.

**Empfehlung:** keine. Der Datensatz gibt bewusst keine: der Fall ist nirgends beobachtet, und ohne deine Erfahrung damit baut man b für niemanden. Wenn dir das nie passiert, ist a die Antwort.
**Noch aktuell?** ja. `ersatz_einloesen` (`crates/krk-core/src/verzeichnis/modell.rs:515-531`) leert die Markierung bei jedem Lesevorgang unverändert, und einen Träger über die Namen gibt es nicht.
**Datei:** `260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md`

### 6. Sollen Kommentare im Quelltext weiterhin die Nummer eines Statuszeilen-Rangs ausschreiben?

Die Statuszeile zeigt zu jedem Zeitpunkt eine von mehreren möglichen Meldungen, in fester
Rangfolge. Kommentare, die diese Rangnummern nennen, sind viermal falsch geworden, als ein Rang
dazukam. Kein Prüflauf liest eine Zahl in einem Kommentar.

a) So lassen — schließt aus, dass der nächste neue Rang die Zahlen nicht wieder falsch macht; bemerkt wird es erst, wenn jemand den Kommentar liest.
b) Die Zahl weglassen und stattdessen auf die eine Stelle verweisen, an der die Ordnung steht — schließt die kürzere Ausdrucksweise aus: die Zahl sagt mit, dass eine Meldung *unter* einer anderen steht, und dafür braucht es dann einen Satz statt einer Ziffer. Rund neun Absätze in drei Dateien wären umzuschreiben.
c) Die Rangfolge als eigene Aufzählung im Code führen — schließt nichts von dem aus, worum es geht: der Kommentar wird dadurch trotzdem nicht geprüft, und der Umbau kostet Code und Proben.

**Empfehlung:** b, aber erst wenn jemand die Dateien ohnehin anfasst; c löst das Problem nicht.
**Noch aktuell?** ja, und der Fall ist seither zweimal wieder eingetreten. Die Statuszeile hat inzwischen sieben Ränge (`crates/krk-ui/src/appkit/statuszeile.rs:280-288`), und `crates/krk-ui/src/tabs.rs:819` sagt weiterhin „Rang 5 von 6". Zwei gleichartige Befunde vom 260826 sind behoben (`260826-1327_*_…` und `260826-1420_*_…`). An der Aufzählung selbst ist b schon gebaut: ihr Kommentar nennt bewusst keine Zahl mehr (`statuszeile.rs:219-221`).
**Datei:** `260811-1230_o_soll-ein-kommentar-den-rang-der-statuszeile-als-zahl-nennen.md`

### 7. Soll ein Rechtsklick auf eine unmarkierte Zeile die Markierung anderswo aufheben?

Wenn irgendwo in der Liste etwas markiert ist und du rechts auf eine andere, unmarkierte Zeile
klickst, springt die Auswahl auf die angeklickte Zeile, das Kontextmenü wirkt aber weiterhin
auf die Markierung. Es zeigt auf A und wirkt auf B.

a) So lassen — schließt aus, dass der Klick und seine Wirkung je zusammenfallen; die Lücke wächst mit jedem weiteren Menüeintrag.
b) Der Rechtsklick hebt die Markierung auf — schließt aus, dass eine Markierung einen Klick übersteht, der nur ein Menü öffnen sollte. So verhält sich der Finder.
c) Der Menütext nennt, worauf er wirkt („3 markierte Einträge teilen") — schließt den Systemeintrag für das Teilen aus, dessen Text von macOS kommt, und der Text muss zur Laufzeit gebaut werden.

**Empfehlung:** keine. Der Datensatz legt die Wahl bewusst dir vor: die drei treffen verschiedene Erwartungen.
**Noch aktuell?** ja, mit gewachsenem Einsatz. Das Kontextmenü trägt seit der Runde 17 vier Einträge statt einem (Teilen, Zip, Unzip, Im Finder öffnen); zerstörend ist noch keiner. Die Regel ist unverändert (`crates/krk-ui/src/kommandos/operationen.rs:183-208` und `:250-257`).
**Datei:** `260812-1516_o_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md`

### 8. Soll die Vorschau kleiner schreiben als der Editor?

Beim Zusammenlegen der Schriftwahl ist die Vorschau von 11 auf 13 Punkt gewachsen, ohne dass
das jemand entschieden hätte. Die Schriftart ist unverändert.

a) So lassen, Vorschau und Editor schreiben gleich groß — schließt aus, dass du in der schmalen Vorschaufläche so viele Zeilen auf einmal siehst wie früher.
b) Die kleine Größe für die Vorschau zurückholen — schließt die eine Antwort auf „welche Größe gilt wo" aus; die Schriftwahl bekäme eine Unterscheidung, die sie heute nicht hat.
c) Beide Flächen auf die kleine Größe — schließt die Größe aus, die der Editor seit seiner Runde bewusst trägt.

**Empfehlung:** keine. Das ist eine Geschmacksfrage über eine sichtbare Fläche und am laufenden Programm zu beurteilen, nicht am Code.
**Noch aktuell?** ja. `grundschrift` (`crates/krk-ui/src/appkit/textmerkmale.rs:355-368`) gibt Vorschau und Editor in der Rohansicht unverändert dieselbe Systemgröße; eine Unterscheidung für die Vorschau gibt es nicht.
**Datei:** `260812-1707_o_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-oder-waechst-sie-auf-die-des-editors.md`

### 9. Wird die Zusage „alles Nicht-Gerenderte erscheint als Quelltext" um ihre Ausnahme ergänzt?

Die gerenderte Vorschau zeigt heute jedes Byte der Quelle mit einer benannten Ausnahme: den
Anfang eines Listenpunkts oder ähnlichen Behälters, dort wo sein Aufzählungszeichen steht. Eine
Verweisdefinition, die sich dorthin verirrt, fällt heraus.

a) Die Zusage um die Ausnahme ergänzen — schließt aus, dass das Abnahmekriterium wörtlich gilt; es wird nachträglich enger. Der Fall ist selten und schreibt niemand absichtlich.
b) Die Lücke im Code schließen — schließt die einfache, mechanische Regel aus: um im Anfang das Aufzählungszeichen von allem anderen zu trennen, bräuchte es wieder Sonderregeln über Markdown-Syntax, also genau das, was diese Runde abgeschafft hat.
c) So lassen und nichts nachziehen — schließt die Übereinstimmung von Zusage und Code aus; eine Zusage, die weiter reicht als der Code, wird geglaubt.

**Empfehlung:** a, weil der Code hier sauberer ist als die Zusage und die Ausnahme schmal, gemessen und durch eine Probe festgehalten ist.
**Noch aktuell?** teilweise überholt, und zwar in der Voraussetzung: „die eine Lücke" stimmt nicht mehr. Das Abnahmekriterium steht unverändert (`fusion-workbench/circles/260812-1000-…/planning/260812-1145_c_…md`, Zeile 63), und daneben steht ein zweiter offener Befund derselben Zusage: YAML-Kopfzeilen erscheinen als Trennlinie und Überschrift statt als Quelltext (`…/issues/260812-1805_o_yaml-front-matter-…md`). Wer die Zusage nachzieht, zieht sie für beide Fälle nach oder entscheidet den zweiten zuerst.
**Datei:** `260812-2002_o_bleibt-der-vorspann-eines-containers-die-eine-luecke-in-der-deckungszusage-von-c4-3.md`

### 10. Darf das Hauptmenü die eine Gliederung der Funktionen umsortieren und einen Bereich umbenennen?

macOS verlangt den Anwendungsnamen im ersten Menü und hängt seine Textzusätze an ein Menü
namens „Bearbeiten". Die Gliederung, aus der KRK Menü, Belegungsansicht und Markdown-Ausgabe
zugleich baut, führte weder das eine noch das andere.

a) Die eine Gliederung anpassen, das Menü benutzt sie unverändert — schließt aus, dass Belegungsansicht und Markdown-Ausgabe unverändert bleiben: sie zeigen ihre Abschnitte danach in anderer Reihenfolge und einen unter anderem Namen.
b) Das Menü bekommt eigene Reihenfolge und eigene Namen — schließt die eine Gliederung aus: zwei Ordnungen über dieselben Bereiche, von Hand gepflegt, ohne Zwang vollständig zu bleiben.
c) Nur umsortieren, nicht umbenennen — schließt die Zusage aus, dass macOS keine fremden Einträge ins Menü hängt; ob das ohne ein Menü namens „Bearbeiten" trägt, ist ungeprüft.

**Empfehlung:** a, weil die eine Gliederung der Grund ist, aus dem das Menü überhaupt aus ihr gebaut wird.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: „Anwendung" steht an erster, „Fenster" an letzter Stelle, und der Bereich der Textbefehle heißt „Bearbeiten" (`crates/krk-ui/src/belegungsmodell.rs:178-205`, gesetzt in `16c0924` und `a949ff1`). Die Liste ist seither auf zehn Bereiche gewachsen. Es fehlt allein dein Ja zu einer Änderung, die seit der Runde 7 wirkt.
**Datei:** `260813-0159_o_darf-das-menue-die-eine-gliederung-umsortieren-und-umbenennen.md`

### 11. Darf `Esc` im Editor wirkungslos verschluckt werden?

`Esc` bricht in KRK ab, was gerade läuft. Steht im Editor nichts zum Abbrechen an, tut die Taste
nichts, wird aber trotzdem verbraucht. In Japanisch, Chinesisch oder Koreanisch bräche `Esc` an
einer Textfläche sonst eine laufende Zeichenzusammensetzung ab, ebenso nach einer Akzenttaste.

a) So lassen — schließt den Abbruch einer laufenden Zeichenzusammensetzung im Editor aus, für Nutzer, die in solchen Sprachen schreiben.
b) `Esc` behält seine alte Sonderstellung — schließt die eine einheitliche Regel aus und bringt den Sonderfall zurück, wegen dessen der Menüeintrag „Abbrechen" doppelt liefe.
c) Der Editor meldet, ob er gerade zusammensetzt, und `Esc` ist dann unzulässig — schließt die schmale Zustandsbeschreibung aus: ein vierter Wert für einen einzigen Befehl, und die Prüftafel verdoppelt sich.

**Empfehlung:** a, weil der Verlust eng und ungemessen ist; c ist der saubere Weg, falls dich der Verlust am laufenden Programm stört, und lässt sich ohne Umbau nachziehen.
**Noch aktuell?** ja, und die Taste tut inzwischen mehr. `Esc` gilt weiterhin überall (`crates/krk-core/src/tasten/belegung.rs:1060`), und seit der Runde 10 räumt es als dritte Wirkung den Filtertext des aktiven Dateifensters (`crates/krk-ui/src/appkit/anwendung.rs:6253-6278`, `14718c2`). Aus dem Editor gedrückt trifft es damit heute den Filter im Dateifenster. Möglichkeit c ist nicht gebaut: `hasMarkedText` steht nirgends im Baum.
**Datei:** `260813-0320_o_esc-im-editor-erreicht-heute-die-textflaeche-und-wird-nach-s3-geschluckt.md`

### 12. Wer zeigt `Cmd+A` im Menü, wenn zwei Funktionen sich die Kombination teilen?

`Cmd+A` markiert in der Dateiliste alle Einträge und wählt in einem Textfeld den Text aus. Eine
Menüleiste verträgt dieselbe Tastenentsprechung nicht zweimal; macOS nimmt sie dem später
stehenden Eintrag still weg.

a) Das Kürzel bekommt die Textfeld-Funktion, der KRK-Befehl zeigt keines — schließt aus, dass das Menü als vollständiges Nachschlagewerk taugt: „Alle Einträge markieren" steht dann ohne `Cmd+A` da, obwohl `Cmd+A` es auslöst. Wirkung geht keine verloren, denn KRK sieht jeden Tastendruck vor dem Menü.
b) macOS entscheiden lassen — schließt `Cmd+A` in jedem Textfeld aus, weil die Dateiliste im Menü vorn steht.
c) Die Doppelbelegung aus der Tastendatei nehmen — nimmt dir eines von beiden weg.
d) Der KRK-Befehl zeigt `Cmd+A` als bloßen Beschriftungszusatz — schließt die eine Wahrheit über Tastenkürzel aus: eine Beschriftung, die ein Kürzel nur behauptet.

**Empfehlung:** a, weil sie als einzige niemandem etwas nimmt und ihre Regel auf der Sache steht und nicht auf einer Reihenfolge.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: die Regel ist gebaut und wird bei jedem Menüaufbau gefragt (`crates/krk-ui/src/menuemodell.rs:339` und `:236`), die Doppelbelegung steht unverändert in `resources/default-keymap.toml:352` und `:1174`. Es fehlt allein dein Ja.
**Datei:** `260813-0430_o_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`

### 13. An welcher Stelle der Statuszeile steht die Filterzahl?

KRK hat eine Statuszeile und zeigt darin je eine Meldung. Wenn du filterst und zugleich etwas
markiert hast, wollen beide Zahlen dieselbe Stelle.

a) Die Filterzahl unter den Markierungsstand — schließt aus, dass du die Filterzahl in dem Fall siehst, in dem sie am meisten trägt; du siehst nicht, dass die Liste verkürzt ist.
b) Die Filterzahl über den Markierungsstand — schließt aus, dass der Markierungsstand sichtbar bleibt, gerade wenn das Markieren unter einem Filter heikel wird.
c) Beide Zahlen in einem Satz — schließt die feste Zeilenbreite aus: ein zusammengesetzter Satz reißt im schmalen Fenster ab.
d) Die Filterzahl in Tableiste oder Fenstertitel — schließt die Zusage aus, dass Lesefortschritt und Einträgezahl in dieselbe Zeile kommen und nicht in eine zweite daneben.

**Empfehlung:** b, weil eine verkürzte Liste die Auskunft ist, ohne die du das Fehlen eines Eintrags für einen Defekt hältst.
**Noch aktuell?** ja, als Bestätigung. Der Filterstand steht auf Platz 5 von sieben, unmittelbar über dem Markierungsstand (`crates/krk-ui/src/appkit/statuszeile.rs:280-288`), also nach Möglichkeit b, ohne dass du sie bestätigt hättest. Daneben liegt ein zurückgestellter Befund: vier Ränge stehen über dem Filterstand, und ein vergessener Filtertext bleibt deshalb oft unsichtbar (`260815-1047_*_…`).
**Datei:** `260814-1552_o_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`

### 14. Räumt `Esc` den Filtertext vor oder nach einem laufenden Kopiervorgang weg?

`Esc` schließt heute ein offenes Blatt und bricht sonst einen laufenden Vorgang ab. Der
Filtertext kommt als dritte Bedeutung dazu. Der Fall: du filterst, markierst, kopierst mit F5,
und drückst `Esc`, um das Kopieren anzuhalten.

a) Filtertext zuerst — schließt aus, dass `Esc` beim ersten Druck das Kopieren anhält; stattdessen wird die Liste wieder lang. Ein offenes Blatt wäre über die Tastatur gar nicht mehr zu schließen.
b) Filtertext zuletzt — schließt aus, dass du den Filter während eines laufenden Kopiervorgangs in einem Druck loswirst; du wartest oder drückst zweimal.
c) Filtertext in der Mitte — trägt den Nachteil von a für den laufenden Vorgang und gewinnt nichts gegenüber b.

**Empfehlung:** b, weil `Esc` in KRK „halte an, was läuft" heißt und ein Filtertext nicht läuft.
**Noch aktuell?** ja, als Bestätigung. `abbrechen` prüft in genau dieser Reihenfolge: Blatt, laufender Vorgang, Filtertext (`crates/krk-ui/src/appkit/anwendung.rs:6253-6278`), ohne dass du sie bestätigt hättest. Seit der Filtertext jeden Ordnerwechsel übersteht, ist `Esc` der einzige Griff, der ihn in einem Zug wegnimmt.
**Datei:** `260814-1830_o_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`

### 15. Gilt das Ankreuzfeld „Deep" je Tab oder für das ganze Fenster?

Der Filtertext gehört dem Tab. Für den Schalter, der die Suche in den Unterbaum ausdehnt, war
das nicht festgelegt; die anderen Felder der Leiste gelten alle fensterweit.

a) Je Tab — schließt die einheitliche Bauart der Leiste aus: ihr erstes Feld hätte je nach sichtbarem Tab eine andere Bedeutung.
b) Je Fenster, wie die Spaltenschalter — schließt aus, dass ein Tabwechsel dich unüberrascht lässt: ein Tab mit stehendem Filtertext zeigt sich dann unversehens tief gefiltert und stößt einen Durchlauf an.
c) Je Dateifenster — löst die Überraschung nicht, denn sie tritt innerhalb einer Seite auf, und führt einen dritten Geltungsbereich ein.

**Empfehlung:** a, weil der Schalter nicht das Fenster beschreibt, sondern die Suche, und die gehört dem Tab; dazu: der Stand soll die Sitzung nicht überleben.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: der Stand sitzt im Ordnermodell (`crates/krk-core/src/verzeichnis/modell.rs:339`), und jeder Tab hält sein eigenes. In die Sitzungsdatei geht er nicht. Das ist a samt der Zugabe. Seit dem 260826 steht der Schalter ab Werk auf ein (`modell.rs:438`). Es fehlt allein dein Ja.
**Datei:** `260814-1830_o_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`

### 16. Soll die Liste der ab Werk tastenlosen Funktionen an einer Stelle statt an zweien stehen?

Welche Funktionen ohne Tastenkombination ausgeliefert werden, steht zweimal im Prüfcode. Beim
vierten Eintrag sind die beiden auseinandergelaufen und haben drei Proben zugleich rot gemacht.
Die zwei Listen prüfen dabei Verschiedenes: die eine nur, dass keine andere Funktion tastenlos
ist, die andere zusätzlich, dass diese vier auch wirklich keine Taste haben.

a) Es bleibt bei zwei Listen mit gegenseitigem Verweis — schließt eine Sicherung durch den Übersetzer aus: die Verweise sind Prosa, und der nächste Eintrag läuft wieder auseinander, wenn niemand sie liest.
b) Die Liste zieht in den ausgelieferten Code — schließt aus, dass Prüfwissen im Prüfcode bleibt; das Programm trüge eine Aufzählung mit, die niemand liest, und beide Seiten des Vergleichs gehörten demselben Autor.
c) Die Liste bleibt an einer Stelle im Prüfcode, und die zweite Prüfrichtung zieht mit um — schließt aus, dass ein Leser der zweiten Probe an Ort und Stelle sieht, *welche* Funktion aus der Ausgabe fällt; drei Proben sind anzufassen.

**Empfehlung:** c, aber erst bei der nächsten Gelegenheit; sie stellt als einzige eine Stelle her, ohne eine Zusage aufzugeben.
**Noch aktuell?** ja. Beide Listen stehen unverändert und führen heute je sieben Einträge (`crates/krk-core/tests/belegung.rs:149-157` und `crates/krk-ui/src/belegungsausgabe.rs:594-612`). Seit der Frage sind drei Einträge dazugekommen, ohne dass die Listen wieder auseinandergelaufen wären.
**Datei:** `260814-2326_o_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`

### 17. In welcher Befehlssprache schreibst du deine Makros?

Die geplante Befehlsausführung braucht eine Shell. Die Wahl entscheidet, in welcher Sprache du
jedes Makro schreibst, das du je anlegst; eine spätere Umstellung entwertete deine Makrodatei.

a) `/bin/sh` führt aus, deine Anmeldeshell wird einmal beim Start nach dem Suchpfad gefragt — schließt zsh-Schreibweise in Makros aus: `**/*.rs` und `[[ … ]]` gibt es dort nicht. Dafür verhält sich jedes Makro auf jedem Gerät gleich.
b) Deine eigene Shell führt aus — schließt die Vorhersagbarkeit aus: zsh liest bei jedem Lauf `~/.zshenv`, eine Datei, die KRK nicht kennt und die den beim Start erfragten Suchpfad still überschreiben kann; eine Makrodatei ist zwischen zwei Geräten nicht mehr austauschbar.
c) Deine Shell mit vollem Anmeldevorgang bei jedem Lauf — durch deine eigene Festlegung vom 260816 bereits ausgeschlossen und mehrere hundert Millisekunden je Lauf teuer.

**Empfehlung:** a, weil die zwei Anforderungen zwei verschiedene Fragen stellen und jede die Antwort bekommt, die nur sie geben kann.
**Noch aktuell?** ja, aber ohne Eile: die Runde ist zurückgestellt, und im Baum steht von der Befehlsausführung nichts.
**Datei:** `260816-2307_o_welche-shell-faehrt-den-lauf-und-woher-kommt-ihr-pfad.md`

### 18. Bekommen die zwei gegenläufigen Löschprüfungen zwei getrennte Typen?

Ein Prüfergebnis mit den drei Werten Ja, Nein und Unentschieden beantwortet zwei entgegengesetzte
Fragen: bei „führt das Ziel einen Papierkorb" ist Ja die Erlaubnis, bei „liegt es auf einem
Netzlaufwerk" ist Ja der Warngrund. Der Übersetzer sieht eine Verwechslung nicht. Genau die ist
am 260817 eingetreten, und gefunden hast du sie, nicht eine Probe.

a) Es bleibt bei einem Typ — schließt die Sicherung durch den Übersetzer aus: jede neue Frage dieser Art muss ihre Richtung von Hand richtig treffen, und die Sicherung dagegen ist eine Zählprobe je Datei, die jemand erst schreiben muss.
b) Zwei Typen für zwei Fragen — die Verdrehung wird unübersetzbar; Preis: ein Typ mehr und eine Umrechnung an vier Prüfstellen, und der günstige Zeitpunkt ist verstrichen.
c) Eine Umkehrfunktion am Typ — von dir am 260817 ausdrücklich verworfen, steht nur der Vollständigkeit halber da.

**Empfehlung:** keine. Der Datensatz stellt es an die Frage, wie viele Prüfungen dieser Art noch dazukommen, und das entscheidet die Planung der nächsten Runden.
**Noch aktuell?** ja. Der Typ ist unverändert einer und liegt in einem eigenen Modul, dessen Kopf beide Richtungen ausschreibt (`crates/krk-core/src/verzeichnis/loeschzielbefund.rs:101-120`). Antworten liefern inzwischen vier Prüfungen statt dreier, die vierte über einen eigenen Zwischentyp.
**Datei:** `260818-0249_o_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md`

### 19. Soll der Übersetzer erzwingen, dass jedes Rückfrage-Blatt einen ungefährlichen Ausgang hat?

Wenn KRK eine Rückfrage stellt, muss eine der Schaltflächen alles liegen lassen. Welche das ist,
wird heute gesucht; findet die Suche keine, fällt sie auf die erste Schaltfläche zurück, und in
einem Blatt mit ausführender erster Schaltfläche wäre das der zerstörende Ausgang. Seit dem
260818 hält dagegen eine Prüfung zur Laufzeit.

a) Es bleibt bei der Laufzeitprüfung — schließt die Zusage beim Übersetzen aus; im Code bleibt eine Zeile für einen Fall, den es nicht geben soll.
b) Ein eigener Typ für den Bauplan, ohne ungefährlichen Ausgang nicht baubar — die Verdrehung wird unübersetzbar; Preis: ein Typ und elf Aufrufstellen.
c) Der halbe Schritt: jedes Blatt bekommt eine reine Bauplanfunktion — schließt die Zusage beim Übersetzen weiterhin aus, macht aber jeden Bauplan ohne Oberfläche prüfbar, drei Zeilen je Blatt.

**Empfehlung:** c, falls die nächste Runde ohnehin an den Rückfragen arbeitet; b lohnt erst, wenn ein Blatt dazukommt, dessen erste Schaltfläche ausführt.
**Noch aktuell?** ja. Die Laufzeitprüfung steht (`crates/krk-ui/src/appkit/blaetter/mod.rs:657-662`), und der Rückfall auf die erste Schaltfläche steht ebenso (`:452-457`). Kein Blatt im Baum hat heute eine ausführende erste Schaltfläche.
**Datei:** `260818-0250_o_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ.md`

---

## Implications

Die neunzehn zerfallen in drei Gruppen, und die Gruppe entscheidet, wie viel Aufwand die
Antwort auslöst.

**Vier sind vom Baum längst entschieden und brauchen nur noch die Zustimmung** (10, 12, 15 und
mit Einschränkung 13 und 14). Der Code fährt dort seit Runden auf der Empfehlung, weil der
Nutzer die Runden als autonom beauftragt hatte. Ein Ja kostet keine Zeile; ein Nein kostet je
nach Frage eine Zeile bis einen Umbau.

**Zwei haben ihre Voraussetzung verloren und sind vor der Antwort nachzuführen.** Bei 9 ist die
Aussage „die eine Lücke" durch einen zweiten offenen Befund derselben Zusage überholt. Bei 6
ist der beschriebene Fehler seit der Fragestellung zweimal wieder eingetreten, und eine Stelle
im Baum trägt ihn heute.

**Die übrigen sind unverändert offen**, und drei davon (5, 7, 18) tragen im Datensatz bewusst
keine Empfehlung: sie hängen an einer Erfahrung oder an einer Planung, die kein Agent hat.

## Recommendations

- Die vier faktisch entschiedenen Fragen zuerst vorlegen: sie sind in einem Satz zu beantworten.
- Vor der Antwort auf 9 den offenen Befund zu den YAML-Kopfzeilen entscheiden; die Zusage lässt
  sich sonst zweimal hintereinander nachziehen.
- Frage 6 gewinnt durch den neuen Befall an Gewicht; wer sie mit b beantwortet, räumt bei der
  Gelegenheit `crates/krk-ui/src/tabs.rs:819` mit.

## Filed Issues

Keine. Die zwei Befunde, die diese Durchsicht gefunden hat (die überholte Rangzahl in
`tabs.rs:819` und die überholte Voraussetzung von Frage 9), sind Bestandteil der Vorlage und
werden mit der jeweiligen Antwort mitentschieden statt getrennt geführt.

## Sources

Die neunzehn Entscheidungsdatensätze im Auftrag, vollständig. Dazu am Baumstand `8276170`:
`crates/krk-ui/src/appkit/textautomatik.rs:161,217`; `crates/krk-ui/src/messmodus.rs:110,752`;
`crates/krk-core/src/verzeichnis/kollation.rs` (Modulkopf);
`crates/krk-ui/src/auffrischung.rs:296-344`; `crates/krk-core/src/verzeichnis/modell.rs:339,438,515-531`;
`crates/krk-ui/src/appkit/statuszeile.rs:219-221,280-288,341-353`; `crates/krk-ui/src/tabs.rs:819`;
`crates/krk-ui/src/kommandos/operationen.rs:183-208,250-257`;
`crates/krk-ui/src/kommandos/kontextmenue.rs` (`Kontextbefehl`);
`crates/krk-ui/src/appkit/textmerkmale.rs:355-368`;
`crates/krk-ui/src/belegungsmodell.rs:178-205`; `crates/krk-core/src/tasten/belegung.rs:1060`;
`crates/krk-ui/src/appkit/anwendung.rs:6253-6278`; `crates/krk-ui/src/menuemodell.rs:236,339`;
`resources/default-keymap.toml:352,1174`; `crates/krk-core/tests/belegung.rs:149-157`;
`crates/krk-ui/src/belegungsausgabe.rs:540-612`;
`crates/krk-core/src/verzeichnis/loeschzielbefund.rs:101-140`;
`crates/krk-ui/src/appkit/blaetter/mod.rs:452-457,652-662`.
Commits `16c0924`, `a949ff1`, `14718c2`, `5ff1ee4`.
Befunde: `260826-1327_*_…`, `260826-1420_*_…`, `260815-1047_*_…`, `260812-1805_*_…`.

## Open Questions

- [ ] Alle neunzehn warten auf die Antwort des Nutzers; diese Analyse entscheidet keine davon.
