# Analyse: neunzehn offene Fragen, zur Vorlage im Chat verdichtet

**Datum:** 2026-09-06 21:15
**Typ:** Dokumentenstudie
**Status:** Complete
**Angefordert von:** Orchestrator

## Frage

Die neunzehn Datensätze 20 bis 38 der Vorlageliste sind so zu verdichten, dass der Nutzer sie im
Chat nacheinander beantworten kann: je eine Frage in Alltagssprache, die Möglichkeiten aus dem
Datensatz, eine Empfehlung und die Auskunft, ob die Frage gegen den heutigen Baum überhaupt noch
einen Gegenstand hat.

## Umfang

Neunzehn Entscheidungsdatensätze, gelesen im Volltext; dazu die im Text genannten Quelldateien,
soweit sie die Aktualität entscheiden. Es wurde nichts geändert und nichts umbenannt.

Baumstand: HEAD `8276170`, Zweig `main`, `git status -sb` meldet eine geänderte Datei
(`fusion-workbench/orchestrator-events.jsonl`). Jede Aussage im Präsens unten ist auf diesen
Stand datiert.

## Befund: fünf der neunzehn Fragen haben ihren Gegenstand verloren

| Nr. | Warum gegenstandslos | Beleg |
|---|---|---|
| 31 | Der empfohlene Weg ist gebaut | `crates/krk-core/src/ablage/sperre.rs:113,163` |
| 32 | Der empfohlene Weg ist gebaut | `crates/krk-ui/src/belegungsmodell.rs:169` |
| 33 | Die betroffene Kopfzeile ist im Rahmenwerk abgeschafft | `Answer located:`-Zeile des Datensatzes |
| 36 | Regel **und** Prüfwerkzeug stehen inzwischen | `$FUSION_PLUGIN_ROOT/bin/fusion-citation-check` |
| 26 | Gebaut, aber bewusst als Bestätigungsfrage offen gehalten | `crates/krk-ui/src/belegungsmodell.rs:139` |

Die übrigen vierzehn stehen unverändert.

## Die Vorlage

### 20. Soll die Löschabfrage sagen, worauf sich ihre zweite Zahl bezieht?

Räumt man 25 Dateien weg, fragt KRK „Diese 25 Einträge mit 25 Einträgen in den Papierkorb
räumen?“ — die erste Zahl zählt die markierten Zeilen, die zweite den Unterbau darunter. Beide
sind richtig, der Satz liest sich aber wie fünfzig.

a) Es bleibt, wie es ist — schließt jede Textänderung aus, der schlechte Satz bleibt stehen.
b) Die zweite Zahl bekommt das Wort „insgesamt“ — schließt aus, dass die Zeile im
   Anforderungsdokument noch wörtlich stimmt (das Abnahmekriterium hält weiter).
c) Die erste Zahl fällt weg, wenn der Umfang der Warngrund ist — schließt aus, dass der Nutzer
   in diesem Fall erfährt, wie viele Zeilen er markiert hat.
d) Der Umfang zählt gar nicht mehr als Warngrund — schließt aus, dass eine Auswahl von 4000
   Einträgen noch laut nachfragt; als einzige Möglichkeit ändert sie das Verhalten.

**Empfehlung:** b, weil sie ein Wort kostet, kein Verhalten ändert und beide Abnahmekriterien hält.
**Noch aktuell?** ja — `crates/krk-ui/src/kommandos/loeschwarnung.rs:606-607` trägt weiterhin
„mit 25 Einträgen“ und „mit mehr als 25 Einträgen“ ohne Zusatz.
**Datei:** `260818-0512_o_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`

---

### 21. Soll die Messstrecke einen Ordnersprung bekommen, damit die Vorschauarbeit überhaupt gemessen werden kann?

Die Vorschau zeigt seit der Runde 16 für erkannte Orte eine Zusammenfassung, und das kostet je
Ordner einen zusätzlichen Verzeichnisdurchlauf. Die Messstrecke wählt für die betroffene
Zeitzusage aber eine **Datei** und keinen Ordner, sieht diese Arbeit also nie.

a) Stehen lassen und richtig aufschreiben — schließt aus, dass die Arbeit dieser Runde je gemessen
   wird; die Zusage trägt dann eine Endbedingung, die keine Messung erreicht.
b) Die Messstrecke bekommt zusätzlich einen Ordnersprung — schließt aus, dass die Zahl ohne einen
   von Ihnen gefahrenen Abnahmelauf zu haben ist.
c) Der Messmodus lädt zusätzlich die Leseprofile — schließt aus, dass die Messung unabhängig vom
   Dateibestand des Geräts bleibt.

**Empfehlung:** b, weil sie genau den Preis misst, den jeder Ordner in jedem Projekt zahlt, und
weder eine Profildatei noch einen Eingriff in den Messmodus braucht.
**Noch aktuell?** ja — `crates/krk-ui/src/messmodus.rs:429` fragt weiter, welche **Datei** der
Vorschau-Tab zeigt; ein Ordnersprung ist nicht gebaut, und der jüngste vollständige Abnahmelauf
ist unverändert `messungen/260810-1918-alle-zusagen.txt`.
**Datei:** `260824-1900_o_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`

---

### 22. Darf die Messung dieselbe Anwendung vermessen, die ausgeliefert wird — auch wenn die Zahlen dadurch mit denen von früher nicht mehr vergleichbar sind?

Seit der Runde 19 wertet die Vorschau jeden Ordner aus, auch ohne Profildatei. Das fällt jetzt
auch im Messmodus an, und damit messen der Lauf vom 10. August und der nächste nicht mehr genau
dasselbe.

a) Es bleibt so, die Änderung wird im Kopf des nächsten Messberichts genannt — schließt aus, dass
   der nächste Lauf sich direkt mit dem vom 10. August vergleichen lässt.
b) Der Messmodus wird von der Auswertung ausgenommen — schließt aus, dass die Kosten der Vorschau
   je über die vorhandene Strecke gemessen werden können, und entfernt die gemessene Anwendung ein
   zweites Mal von der ausgelieferten.
c) Der Prüfordner der Messstrecke verliert seine Unterordner — schließt dauerhaft aus, dass die
   Messstrecke einen gewachsenen Arbeitsordner nachbildet, und macht auch drei weitere Zusagen
   unvergleichbar.

**Empfehlung:** a, weil eine Messung an einer Sonderfassung eine Zahl ausgibt, die besser ist als
die des Nutzers, ohne dass man ihr das ansieht.
**Noch aktuell?** ja, aber vorentschieden — gebaut ist a
(`crates/krk-ui/src/appkit/anwendung.rs:1706-1715` schreibt es aus). Fällig wird die Frage erst
beim nächsten vollständigen Abnahmelauf, und der steht seit dem 10. August aus.
**Datei:** `260827-1322_o_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md`

---

### 23. Soll das Vergrößern in der PDF-Ansicht auch auf einer US-Tastatur über die beschriftete Plus-Taste gehen?

KRK erkennt eine Taste an dem Zeichen, das sie ohne Zusatztaste liefert. Auf der deutschen
Tastatur ist das beim Plus ein `+`, auf der US-Tastatur ein `=` — dort trifft die Kombination
deshalb nicht.

a) Es bleibt bei der einen Regel; auf der US-Tastatur wirkt nur der Zehnerblock — schließt aus,
   dass ein US-Nutzer ohne Zehnerblock die Vergrößerung über die Tastatur erreicht (Menü und
   Trackpad bleiben ihm).
b) KRK liest bei gehaltener Umschalttaste ein zweites Zeichen — schließt aus, dass der
   Tastendruckpfad ungemessen bleibt, denn an ihm hängt eine Zeitzusage; die Belegungsansicht
   müsste dieselbe zweite Lesung tragen.
c) Das Gleichheitszeichen kommt als eigener Tastenname dazu — schließt aus, dass ein Satzzeichen
   mit wandernder Lage draußen bleibt; ein deutscher Nutzer sähe dann eine Kombination, die auf
   seiner Tastatur ganz anders heißt.

**Empfehlung:** a, weil sie keine Zeile am Tastenabgriff kostet und das Referenzgerät vollständig
trifft; das Anforderungsdokument sagt dann für die US-Tastatur weniger zu als heute.
**Noch aktuell?** ja — `crates/krk-core/src/tasten/parser.rs` entscheidet die Taste unverändert
über das Zeichen ohne Zusatztaste; die US-Hälfte ist ungebaut.
**Datei:** `260828-0712_o_wie-erreicht-eine-us-tastaturbelegung-cmd-plus-wenn-das-pluszeichen-dort-die-umschalttaste-braucht.md`

---

### 24. Was soll Cmd+V im Dateifenster tun, wenn später einmal Dateien über die Zwischenablage eingefügt werden können?

Heute füllt Cmd+V den Filtertext; liegt ein Dateiverweis in der Zwischenablage, wird sein Name
eingefügt. Für eine spätere Dateizwischenablage war genau diese Tastenkombination reserviert.

a) Der Dateiverweis wechselt später die Bedeutung und fügt dann die Datei ein — schließt aus,
   dass man sich auf eine Bedeutung der Taste verlassen kann; sie tut dann je nach unsichtbarem
   Ablageinhalt zwei verschiedene Dinge.
b) Der Filter behält Cmd+V, das Einfügen von Dateien bekommt eine andere Kombination — schließt
   aus, dass die auf dem Mac übliche Geste zum Einfügen von Dateien an ihrer üblichen Stelle liegt.
c) Es kommt gar keine Dateizwischenablage — schließt sie dauerhaft aus; Dateien bewegt KRK dann
   weiter über seine Vorgänge und den Abwurf aus fremden Anwendungen.

**Empfehlung:** b, weil eine Taste mit zwei Bedeutungen je nach unsichtbarem Ablageinhalt gegen
die Maxime „supersimpel“ steht und die heutige Regel unangetastet bleibt.
**Noch aktuell?** ja, aber nicht dringend — der Baum tut heute genau das, was die Frage zum
Gegenstand hat (`crates/krk-core/src/zwischenablage.rs:170`); eine Dateizwischenablage gibt es
nicht. Die Frage bindet erst die Runde, die sie baut.
**Datei:** `260828-1041_o_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`

---

### 25. Soll der Git-Bereich merken, wenn Sie außerhalb von KRK committen?

Der Git-Bereich frischt sich heute nur auf, wenn das Dateifenster seinen Ordner neu liest.
Committen Sie im Terminal, während KRK einen **Unterordner** des Repositorys zeigt, zeigt der
Bereich weiter den alten Stand.

a) So lassen; ein Auffrischungsweg und kein zweiter daneben — schließt aus, dass ein Commit von
   außen vor dem nächsten Ordnerwechsel sichtbar wird.
b) KRK beobachtet zusätzlich den `.git`-Ordner — schließt aus, dass die Menge der beobachteten
   Ordner stabil bleibt; `.git` meldet bei jedem Git-Befehl viele Ereignisse und bräuchte eine
   Dämpfung gegen das Aufschaukeln.
c) Ein eigener Befehl holt den Git-Stand von Hand — schließt aus, dass die Aktualität KRKs Sache
   bleibt; sie wird zur Aufgabe des Nutzers.

**Empfehlung:** a mit Wiedervorlage, nachdem Sie das gebaute Bündel benutzt haben — ob der
veraltete Stand stört, ist eine Beobachtung am laufenden Programm und keine Ableitung.
**Noch aktuell?** ja — `crates/krk-ui/src/auffrischung.rs:180` kennt weiterhin keine `.git`-Quelle;
gebaut ist a.
**Datei:** `260830-1251_o_haengt-der-gitbefund-zusaetzlich-an-einem-beobachter-auf-git.md`

---

### 26. Bekommt Git ein eigenes Obermenü in der Menüleiste?

Die Menüleiste gliedert die Befehle nach Bereichen der Anwendung; Vorschau und Editor haben je
ein eigenes Obermenü. Für den Git-Bereich hieße dieselbe Regel ein zehntes Obermenü mit zunächst
zwei Einträgen.

a) Git bekommt einen eigenen Bereich und damit ein zehntes Obermenü — schließt aus, dass die
   Menüleiste kurz bleibt; das Menü ist bis zur nächsten Git-Runde dünn besetzt.
b) Die zwei Befehle reihen sich bei „Fenster“ und der Leiste ein — schließt aus, dass die Regel
   ohne Sonderfall bleibt; die nächste Git-Runde stellte die Frage mit vier weiteren Befehlen
   erneut, und die zwei müssten dann umziehen.
c) Git benutzt das Obermenü der Vorschau mit — schließt aus, dass der Name eines Obermenüs noch
   sagt, was darin steht.

**Empfehlung:** a, weil es die Regel ist, die dieses Projekt für den Editor schon einmal gegen
dieselbe Bequemlichkeit durchgehalten hat.
**Noch aktuell?** ja als Bestätigung, nicht als offene Wahl — a ist gebaut
(`crates/krk-ui/src/belegungsmodell.rs:139,169,454`), und das Obermenü steht an achter Stelle.
Bewusst offen gehalten, damit die Frage nicht durch vollendete Tatsache entschieden ist; sie
bindet die nächste Git-Runde.
**Datei:** `260830-1317_o_bekommt-der-git-bereich-einen-eigenen-funktionsbereich-und-damit-ein-zehntes-obermenue.md`

---

### 27. Soll KRK begrenzen, wie viele Rechenkerne der Git-Status benutzt?

Die Git-Bibliothek verteilt den Statuslauf auf so viele Fäden, wie das Gerät Kerne hat. Auf dem
Referenzgerät können zwei solche Läufe nebeneinander stehen, und ob sie dem Zeichnen der
Oberfläche Bilder wegnehmen, ist ungemessen.

a) Keine Grenze; die Stelle steht namentlich im Code als erster Hebel — schließt aus, dass die
   Wirkung auf die Bildrate vor Ihrem nächsten Abnahmelauf bekannt ist.
b) Eine feste Grenze, etwa die Hälfte der Kerne — schließt aus, dass der Statuslauf so schnell
   bleibt, wie er gemessen ist; die Zahl wäre geraten, auf vier Kernen zu niedrig, auf sechzehn
   zu hoch.
c) Eine Grenze, die von der Zahl gleichzeitiger Läufe abhängt — schließt aus, dass die Grenze in
   einer Schicht bleibt; der Wert müsste aus der Oberfläche in den Kern gereicht werden.

**Empfehlung:** a mit ausdrücklicher Wiedervorlage nach dem ersten Abnahmelauf — erst messen,
dann bauen; die Grenze ist danach eine Zeile und kein Umbau.
**Noch aktuell?** ja — `crates/krk-core/src/git/leser.rs:44` schreibt aus, dass keine Grenze
gesetzt ist; gemessen ist die Fadenzahl weiterhin nicht.
**Datei:** `260830-1317_o_wird-die-fadenzahl-von-gix-gedeckelt-und-woran-waere-die-zahl-zu-messen.md`

---

### 28. Womit soll KRK später einmal ein Sprachmodell ansprechen?

Der ursprüngliche Entwurf nennt als späte Ausbaustufe eine KI-Anbindung, ohne zu sagen, welches
Werkzeug gemeint ist. Der Datensatz hält die Frage nur fest; ausgearbeitete Möglichkeiten führt
er nicht.

a) *(aus dem Text erschlossen)* Die KI läuft in KRK selbst, über eine Bibliothek zum Sprachmodell
   — schließt aus, dass KRK ohne eigene Netzanbindung, eigene Schlüsselverwaltung und eigenes
   Fehlerverhalten auskommt.
b) *(aus dem Text erschlossen)* KRK steuert ein vorhandenes Werkzeug von außen an — schließt aus,
   dass die Anbindung ohne dieses Werkzeug auf dem Gerät funktioniert.

**Empfehlung:** noch keine — der Datensatz gibt selbst keine ab, und die Vorbedingung, die er sich
setzt, ist erst halb erfüllt. Sinnvoll ist heute allenfalls die Vorfrage: soll KRK selbst reden
oder nur steuern?
**Noch aktuell?** ja, aber verfrüht — der Baum enthält keine Zeile zu einer KI-Anbindung, und die
Bedingung des Datensatzes („erst wenn Navigator, Editor und Git-Anbindung stehen“) ist noch nicht
erreicht: von Git ist nur die lesende Hälfte gebaut.
**Datei:** `260802-0842_o_code-sdk-fuer-ki-integration.md`

---

### 29. Was soll der Git-Befehl „verwerfen“ tun: die Änderungen einer Datei wegwerfen oder einen alten Commit rückgängig machen?

Das Wort aus dem ursprünglichen Entwurf trägt in Git zwei Bedeutungen, die nichts miteinander zu
tun haben. Die Wahl entscheidet, was der Knopf tut und wie gefährlich er ist.

a) Nur die Änderungen der Datei verwerfen — schließt aus, dass man über KRK einen Commit
   zurücknimmt; wirft ungespeicherte Arbeit unwiederbringlich weg.
b) Nur einen Commit zurücknehmen — schließt aus, dass man über KRK die eigene ungespeicherte
   Änderung wegwirft, also den häufigsten Wunsch im Editor.
c) Beides, als zwei getrennte Befehle — schließt aus, dass die Oberfläche so schlank bleibt, wie
   der Entwurf sie vorsah.

**Empfehlung:** c, weil beide Bedeutungen dort sitzen, wo der Nutzer sie sucht, und der
Versions-Schieberegler die Commit-Auswahl ohnehin mitbringt.
**Noch aktuell?** ja, und sie wird jetzt fällig — die lesende Git-Hälfte steht, die schreibende
ist die nächste (`crates/krk-core/src/git/mod.rs:17`: kein Weg im Baum schreibt ins Repository).
**Datei:** `260802-0842_o_git-verwerfen-bedeutung.md`

---

### 30. Soll ein Prüflauf erzwingen, dass jede Oberflächendatei ihre macOS-Untergrenze im Kopf nennt?

Der verwendete Baukasten warnt nicht, wenn KRK eine Methode anspricht, die es unter macOS 15 noch
nicht gab — es stürzt ab. Die Gegenmaßnahme ist bislang eine Gewohnheit, und die war schon einmal
auf fünf von einunddreißig Dateien abgesunken.

a) Nur prüfen, dass der Abschnitt da ist — schließt aus, dass Fehler im Inhalt auffallen; fängt
   die neue Datei, sonst nichts. Rund ein Dutzend Zeilen.
b) Zusätzlich prüfen, dass jede benutzte Klasse darin genannt ist — schließt aus, dass die Regel
   ohne eine Ausnahmeliste auskommt, und solche Listen hat dieses Projekt anderswo abgeschafft.
   Rund achtzig Zeilen.
c) Auch die Zahlen selbst prüfen — schließt aus, dass der Prüflauf ohne installiertes Xcode läuft;
   der Entwickler rät ausdrücklich ab, es wäre ein halber Compiler.

**Empfehlung:** a und b zusammen, c nicht — sie decken den häufigen Fehler ab und kommen ohne
fremde Werkzeuge aus. Die Richtigkeit der Zahl bleibt in jedem Fall eine Zusage des Menschen, und
der Prüflauf darf nicht so heißen, als prüfe er mehr.
**Noch aktuell?** ja — im Baum steht weder ein Prüflauf noch ein `make`-Ziel dafür; heute tragen
30 von 32 Oberflächendateien den Abschnitt, also gerade der von Hand wiederhergestellte Stand.
**Datei:** `260811-2050_o_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`

---

### 31. Was passiert mit Lesezeichen und Sitzung, wenn KRK zweimal läuft?

Zwei gleichzeitig laufende KRK-Fenster schrieben ohne Absprache dieselben Dateien und konnten
sich gegenseitig Lesezeichen und Sitzung überschreiben.

a) Eine Schreibsperre über der Ablage und ein Sitzungsrecht daneben — schließt aus, dass die
   zweite Instanz sich ihre Fensteraufteilung merkt.
b) Der zuletzt Schreibende gewinnt, KRK sagt es beim Start — schließt aus, dass der Bestand
   sicher ist; ein Verlust bleibt möglich.
c) Die zweite Instanz darf nur lesen — schließt aus, dass man in ihr ein Lesezeichen anlegt oder
   die Belegung ändert.
d) Jede Instanz bekommt ihren eigenen Ablageordner — schließt aus, dass beide dieselben
   Lesezeichen und dieselbe Tastenbelegung sehen.

**Empfehlung:** a — sie ist gebaut, es geht nur noch um die Bestätigung.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: `crates/krk-core/src/ablage/sperre.rs:113`
(`Schreibgriff`) und `:163` (`Sitzungsrecht`) sind gebaut, `crates/krk-core/tests/baum.rs:454,703`
halten beide namentlich fest. Möglichkeit a ist umgesetzt; der Datensatz gehört bestätigt und
geschlossen.
**Datei:** `260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`

---

### 32. Wie viele Obermenüs soll die Menüleiste tragen, wenn jeder Tastenbefehl auch im Menü steht?

Damals trug das Hauptmenü drei Obermenüs mit zehn Befehlen; ins Menü sollten alle 81 Funktionen.
Die Gliederung nach neun Bereichen gab es schon.

a) Neun Obermenüs, eines je Bereich — schließt aus, dass die Menüleiste kurz bleibt; neun ist für
   ein Programm mit der Maxime „supersimpel“ viel.
b) Fünf Obermenüs mit Untermenüs für die kleineren Bereiche — schließt aus, dass es bei einer
   Gliederung bleibt; die Zuordnung wäre eine zweite, von Hand gepflegte.
c) Ein Sammelmenü „Befehle“ mit neun Untermenüs — schließt aus, dass ein Befehl mit einem Griff
   erreichbar ist; jeder liegt dann zwei Ebenen tief.

**Empfehlung:** a — sie ist gebaut, es geht nur noch um die Bestätigung.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: die Gliederung steht als
Aufzählung in `crates/krk-ui/src/belegungsmodell.rs:169` und trägt seit der Git-Runde zehn Werte,
also neun plus Git. Möglichkeit a ist gebaut und läuft seit Wochen.
**Datei:** `260813-0053_o_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`

---

### 33. Soll die Statuszeile im Kopf eines Datensatzes künftig automatisch mit dem Dateinamen mitgezogen werden?

Wer einen Datensatz beantwortete, benannte die Datei um, ließ die Zustandsangabe im Kopf aber
stehen. Zwischenzeitlich widersprachen sich Kopf und Dateiname in neunzehn von siebenundzwanzig
Defektdatensätzen.

a) *(aus dem Text erschlossen)* Die Berichtigung wird Teil desselben Arbeitsschritts wie die
   Umbenennung — schließt aus, dass der Schritt weiter einzeln danebensteht und wieder ausgelassen
   wird.
b) *(aus dem Text erschlossen)* Es bleibt eine bekannte Lage, die man von Hand nachzieht —
   schließt aus, dass der Widerspruch verschwindet; er kommt bei jedem Durchgang wieder.

**Empfehlung:** keine mehr nötig — die Frage ist gegenstandslos, siehe die nächste Zeile.
**Noch aktuell?** nein, weil das Rahmenwerk die Zeile inzwischen abgeschafft hat. Die Zeile
`Answer located:` des Datensatzes nennt `260826-0818-curator-run.md` `### K02` und
`rules/fusion-workbench-conventions.md` `## Decision Record Template`: das Kopffeld ist ersatzlos
gestrichen, ein vorhandenes bleibt bewusst unangetastet. Es gibt keine zweite Quelle mehr, die
falsch antworten könnte. **Der Datensatz gehört ohne Sachentscheidung geschlossen.**
**Datei:** `260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`

---

### 34. Soll KRK sagen, warum ein Ordner leer bleibt, in den Sie kein Leserecht haben?

Tippt man einen Pfad ein, meldet KRK das fehlende Leserecht in der Statuszeile. Klickt man
denselben Ordner in der Liste doppelt an, wechselt es wortlos in eine leere Liste.

a) Der Doppelklick meldet künftig auch — schließt aus, dass der Einstieg ohne einen zusätzlichen
   Systemaufruf auskommt, auch im häufigen Fall des lesbaren Ordners; zwei Zeitzusagen hängen
   daran und sind seit der Runde 4 nicht mehr gemessen.
b) Die Pfadeingabe schweigt künftig auch — schließt aus, dass eine Meldung bleibt, die ein
   Abnahmekriterium ausdrücklich verlangt.
c) Der Unterschied bleibt — schließt aus, dass jemand das Verhalten aus dem Code ablesen kann,
   ohne beide Wege zu vergleichen.

**Empfehlung:** a, weil das Abnahmekriterium eine Meldung für den nicht lesbaren Pfad verlangt und
diese Möglichkeit sie auf beiden Wegen erfüllt; der zusätzliche Systemaufruf gehört in den nächsten
Abnahmelauf.
**Noch aktuell?** ja, mit verschobener Ausgangslage — der Doppelklick auf einen gewöhnlichen Ordner
schweigt weiterhin (`crates/krk-ui/src/appkit/tabelle.rs:2472-2503`), die Pfadeingabe meldet
weiterhin (`crates/krk-ui/src/kommandos/pfadeingabe.rs:72`). Anders als im Datensatz beschrieben
**meldet der Doppelklick auf eine Verknüpfung heute wieder**: der Zweig `Verweisziel::Unerreichbar`
schreibt „Ins Leere, im Ring oder ohne Recht“ in die Statuszeile. Es sind also drei Wege mit zwei
Verhalten, nicht zwei mit zwei.
**Datei:** `260815-1749_o_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`

---

### 35. Soll das Werkzeug selbst aus den Dateien eines Commits ableiten, ob er eine Durchsicht braucht?

Das Deckungswerkzeug hat sieben Commits als „nur Werkbank-Text“ durchgelassen; einer davon änderte
ausgelieferten Code. Die Zuordnung geschieht heute von Hand.

a) *(aus dem Text erschlossen)* Die Zuordnung wird aus dem Dateibestand des Commits abgeleitet —
   schließt aus, dass ein Codewurf noch als reine Textarbeit durchgeht; `git show --name-only`
   beantwortet es in einer Zeile.
b) *(aus dem Text erschlossen)* Es bleibt bei der Zuordnung von Hand — schließt aus, dass der
   Fehler verschwindet; er ist an derselben Stelle mehrfach aufgetreten.

**Empfehlung:** a, weil die Frage maschinell in einer Zeile zu beantworten ist und die
Fehlzuordnung sonst wiederkommt.
**Noch aktuell?** ja, aber die Frage greift zu kurz — beide Möglichkeiten setzen voraus, dass es
überhaupt eine gemessene Spanne gibt. Arbeit außerhalb einer Sitzung ist für die Deckungsmessung
strukturell unsichtbar; das Werkzeug meldet dann „ungeprüft“, und das liest sich in einem Bericht
wie „nichts zu beanstanden“. Wer a wählt, deckt einen Teil der Commits ab, nicht alle.
**Datei:** `260815-1812_o_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md`

---

### 36. Sollen Querverweise zwischen Datensätzen den Zustandsbuchstaben durch einen Platzhalter ersetzen?

Ein Verweis nennt heute den Zustand, den die Zieldatei beim Schreiben trug. Sobald sie ihn
wechselt, zeigt der Verweis ins Leere — in einer einzigen Zeile waren alle drei Ziele tot.

a) Alles bleibt, wie es ist; ein Verweis behält den Buchstaben, mit dem er geschrieben wurde —
   schließt aus, dass die Verweise auflösbar bleiben; wer ihnen folgt, sucht.
b) Nur die Querverweiszeile im Kopf bekommt den Platzhalter — schließt aus, dass die Verweise im
   Fließtext, in den Antwort- und Erledigungsvermerken mitgezogen werden; von sieben gemessenen
   toten Verweisen stünden fünf weiter da.
c) Jeder Verweis, der ein Zeiger ist, bekommt den Platzhalter — schließt aus, dass die Regel
   allein am Ort der Datei zu entscheiden ist; man müsste je Absatz urteilen, und genau das hat
   das Projekt einmal abgelehnt.

**Empfehlung:** keine mehr nötig — die Frage ist gegenstandslos, siehe die nächste Zeile.
**Noch aktuell?** nein, weil das Rahmenwerk das inzwischen entschieden hat, und zwar breiter, als
die Frage gestellt war. Die Zeile `Answer located:` des Datensatzes nennt
`rules/fusion-workbench-conventions.md` `## Filename Patterns` und `## Marker globs`: der
Platzhalter gilt für **jedes** Zitat, ohne Ausnahme nach Speicher, und die Unentscheidbarkeit von
Möglichkeit c ist anders gelöst — eine Aussage **über** ein Zitat bekommt eine eigene, von außen
erkennbare Gestalt. **Auch die offene zweite Hälfte, die Prüfung, steht inzwischen:**
`$FUSION_PLUGIN_ROOT/bin/fusion-citation-check` löst jeden Verweis gegen den Dateibestand auf, und
`fusion-citation-sweep` schreibt die alte Form um. Der Datensatz gehört ohne Sachentscheidung
geschlossen.
**Datei:** `260818-0201_o_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`

---

### 37. Soll das „erledigt“ am Anforderungsdokument heißen „gebaut“ oder „von Ihnen abgenommen“?

Sieben Anforderungsdokumente längst geschlossener Runden stehen weiter auf „offen“, zwei auf
„erledigt“ — beide Male mit ausgeschriebener und einander widersprechender Begründung. Jeder
Durchgang hat bisher für sich entschieden.

a) Es folgt der Abnahme — schließt aus, dass der Zustand etwas über die Bauarbeit sagt; jede
   Zählung offener Planungsarbeit meldet dann sieben Posten, an denen niemand arbeitet.
b) Es folgt der belegten Bauarbeit — schließt aus, dass die Abnahme noch einen maschinellen Träger
   hat; sie stünde dann nur noch in Prosa. Nebenwirkung: die sieben Dateien werden beim nächsten
   Aufräumen archiviert, und Kurzverweise auf sie zeigen unbemerkt ins Leere.
c) Beides trennen: der Zustand folgt der Bauarbeit, die Abnahme bekommt eine eigene Kopfzeile —
   schließt nichts aus, ist aber die teuerste; dreizehn Köpfe bekommen eine Zeile, und eine
   Kopfzeile ohne Prüfung läuft erfahrungsgemäß auseinander.

**Empfehlung:** c, sonst b — der Streit entsteht daraus, dass ein Zustand mit vier Werten zwei
unabhängige Fragen beantworten soll („ist es gebaut?“ und „ist es abgenommen?“); solange beide an
ihm hängen, liefert jede Wahl eine richtige und eine falsche Auskunft.
**Noch aktuell?** ja, und der Bestand ist unübersichtlicher geworden — im Baum stehen heute
nebeneinander „offen“ (7 Anforderungsdokumente), „erledigt“ (5) und der Ausweichzustand „in
Arbeit“ (2, unter anderem am Anforderungsdokument der Git-Runde), obwohl an keinem davon ein Agent
arbeitet. Der Ausweichzustand behauptet also eine Tätigkeit, die es nicht gibt.
**Datei:** `260819-1440_o_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`

---

### 38. Soll KRK eine leergeräumte Sitzungsdatei als beschädigt behandeln?

Findet KRK eine Ablagedatei ohne einen einzigen Eintrag, hält es das bei den Lesezeichen für einen
Schaden und legt die Datei zur Seite; bei Sitzung und Tastenbelegung nimmt es die Vorgabe. Für die
Lesezeichen ist das gemessen, für die anderen beiden nicht.

a) Es bleibt bei der Nachsicht für alle drei — schließt aus, dass ein unbemerkter Verlust der
   Sitzung auffällt; Ordner, Tabs und die geöffnete Editordatei sind der Bestand, der am häufigsten
   und unbemerkt verlorenginge.
b) Die Sitzungsdatei wird streng, Tastenbelegung und Einstellungen bleiben nachsichtig — schließt
   aus, dass eine Sitzungsdatei aus einer **späteren** KRK-Fassung in einer älteren noch gelesen
   wird; beim Zurückspringen verlören Sie Ihre Sitzung, mit Meldung, aber Sie verlören sie.
c) Alle vier werden streng — schließt aus, dass Sie eine von Hand gepflegte Datei leerräumen
   können, ohne sie zu löschen; für die Einstellungen schützt es gegen gar nichts.

**Empfehlung:** b, aber erst nach der Messung, die der Datensatz verlangt (schreibt KRK je eine
Sitzungsdatei ohne obersten Eintrag?), und mit einer Probe für die Rückwärtsrichtung.
**Noch aktuell?** ja, mit einem abgeschwächten Argument — `crates/krk-core/src/ablage/pfade.rs:322-327`
behandelt unverändert allein die Lesezeichendatei streng, und die Sitzungsstruktur weist unbekannte
Felder weiterhin nicht ab. Seit der Änderung `d771ec6` wird eine leere Datei allerdings **nicht mehr
gesichert**, sondern nur gemeldet; der Hauptgrund für b („der Sitzungszustand ist der teuerste
Verlust“) trägt damit für diese Hälfte schwächer als bei der Ablage der Frage.
**Datei:** `260821-0142_o_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`

## Folgerungen

Die neunzehn zerfallen in drei Gruppen, und die Vorlage sollte in dieser Reihenfolge laufen.

**Vier sind reine Bestätigungen und in Minuten erledigt** (31, 32, 33, 36). Bei 31 und 32 ist der
empfohlene Weg gebaut und läuft; bei 33 und 36 hat das Rahmenwerk die Frage aufgehoben. Keine
verlangt eine Sachentscheidung.

**Fünf hängen an einer Messung oder an Ihrer Beobachtung am laufenden Programm** (21, 22, 25, 27,
38) und sind ohne sie nicht besser zu beantworten als heute. Für sie ist die richtige Antwort meist
„erst messen“ — was zugleich heißt, dass sie so lange offen bleiben.

**Zehn sind heute entscheidbar** (20, 23, 24, 26, 28, 29, 30, 34, 35, 37). Zwei davon binden
unmittelbar die nächste Arbeit: 29 entscheidet, was die schreibende Git-Hälfte baut, und 26
entscheidet, wohin ihre Befehle einsortiert werden.

## Empfehlungen

1. Erst die vier Bestätigungen (31, 32, 33, 36) — sie räumen die Liste um ein Fünftel.
2. Dann die zwei, die die nächste Runde binden (29, 26).
3. Dann die übrigen acht entscheidbaren.
4. Die fünf messungsabhängigen (21, 22, 25, 27, 38) zuletzt, mit der ausdrücklichen Wahl „jetzt
   festlegen“ oder „bis zum nächsten Abnahmelauf offen halten“.

## Eingereichte Defektdatensätze

Keine. Diese Analyse liest und verdichtet; jeder Befund unten ist Sachstand zu einer offenen Frage
und kein eigener Defekt.

## Quellen

- Die neunzehn Entscheidungsdatensätze der Vorlageliste, im Volltext gelesen.
- `crates/krk-ui/src/kommandos/loeschwarnung.rs:606-607`
- `crates/krk-ui/src/messmodus.rs:429,560`
- `crates/krk-ui/src/appkit/anwendung.rs:1698-1715,1830`
- `crates/krk-core/src/tasten/parser.rs`
- `crates/krk-core/src/zwischenablage.rs:167-177`
- `crates/krk-ui/src/auffrischung.rs:180`
- `crates/krk-ui/src/belegungsmodell.rs:125-139,169,202,454`
- `crates/krk-core/src/git/leser.rs:44,56`; `crates/krk-core/src/git/mod.rs:17`
- `crates/krk-core/src/ablage/sperre.rs:113,163`; `crates/krk-core/tests/baum.rs:454,703`
- `crates/krk-core/src/ablage/pfade.rs:322-327`; `crates/krk-core/src/ablage/sitzung.rs`
- `crates/krk-ui/src/appkit/tabelle.rs:2472-2503`; `crates/krk-ui/src/kommandos/pfadeingabe.rs:72`
- `crates/krk-core/src/verzeichnis/verweisziel.rs:148,168`
- `$FUSION_PLUGIN_ROOT/bin/fusion-citation-check`, `fusion-citation-sweep`, `fusion-review-coverage`
- `messungen/` (Bestand am 260906), `Makefile` (Zielliste)

## Offene Punkte

- [ ] Bei 26 und 22 ist der empfohlene Weg gebaut, die Frage aber bewusst offen gehalten. Ob eine
      solche Bestätigung als Antwort zählt oder als eigener Vorgang, entscheidet der Nutzer.
- [ ] Bei 35 greift die Frage, wie sie gestellt ist, zu kurz; ob sie neu gefasst wird, ist selbst
      eine Nutzerfrage.
