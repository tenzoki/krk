# Spec: Auf Werkseinstellungen zurücksetzen und neu einlesen

**Date:** 2026-09-29
**Status:** Draft
**Source:** Nutzerwunsch, wörtlich: „Wir brauchen einen Befehl unter Krk, der automatisch auf Werkseinstellungen zurücksetzt und die Werte neu einliest.“ Anlass ist die Auslieferung 2.1.1 mit geänderten fusion-Leseprofilen. Ein Nutzer mit vorhandener `readers.toml` bekommt sie bisher nur, wenn er KRK beendet, die Datei beiseitelegt und neu startet, weil KRK seine Ablagedateien allein beim Start liest. Antworten des Nutzers auf die erste Fragerunde: „1 b 2 a 3 a 4 a“.

## Directive

Ein Befehl im Menü „KRK“ setzt die drei von Hand gepflegten Ablagedateien (`readers.toml`, `settings.toml`, `keymap.toml`) nach einer Rückfrage in einem Zug auf den Zustand zurück, den ein erster Start herstellt. Die alten Dateien legt er mit Zeitstempel beiseite und löscht keine davon. Danach arbeitet KRK sofort mit den Werkswerten, ohne Neustart: Vorschau, Tastenbelegung, Hauptmenü, Terminal und Notizordner folgen dem neuen Stand.

## Grundlage aus dem Bestand

Wir halten fest, was der Code heute tut, weil mehrere Festlegungen unten darauf ruhen.

- KRK liest `settings.toml` und `readers.toml` einmal beim Start, im selben Durchgang wie die Sitzung und die Erhebung der Neuerungen (`Anwendungsdelegierter::sitzung_laden`, `crates/krk-ui/src/appkit/anwendung.rs`). `keymap.toml` liest es davor, ebenfalls einmal (`belegung::fuer_den_betrieb`).
- `settings.toml` und `readers.toml` legt KRK beim ersten Start wörtlich aus der einkompilierten Auslieferungsfassung an. `keymap.toml` legt KRK nie an; ohne diese Datei gilt die Auslieferungsbelegung vollständig.
- Für die Belegung gibt es einen Neuaufbau im Betrieb: Beim Verlassen der F1-Ansicht mit einer Änderung baut `belegungsansicht_verlassen` Hauptmenü und Tastenabgriff neu auf.
- Den Notizordner wechselt „Ort wählen…“ im Betrieb (`ort_uebernehmen`). Es weist den Wechsel unter anderem ab, solange der Editor eine Datei des Notizordners hält (`gehaltene_notizdatei`).
- Das Terminal liest KRK bei jedem Aufruf aus dem gehaltenen Einstellungsstand.
- Die Neuerungen-Meldung vergleicht allein Namen. Ein Leseprofil, das unter gleichem Namen einen neuen Inhalt bekommt, meldet sie nicht.

## Capabilities

### C1: Der Befehl und seine Rückfrage

**Description:** Im Menü „KRK“ steht ein Eintrag „Auf Werkseinstellungen zurücksetzen…“. Er öffnet eine Rückfrage, die sagt, was geschehen wird. Erst die ausdrückliche Bestätigung löst den Vorgang aus.

**Acceptance criteria:**
- [ ] C1.1 Das Menü „KRK“ führt den Eintrag „Auf Werkseinstellungen zurücksetzen…“ im selben Block wie „Neuerungen anzeigen“.
- [ ] C1.2 Ab Werk trägt der Befehl keine Tastenkombination. In der F1-Ansicht steht er unter dem Bereich „Anwendung“ und lässt sich dort wie jede andere Funktion belegen.
- [ ] C1.3 Die Wahl des Eintrags öffnet eine Rückfrage als Blatt am Hauptfenster. Ohne Bestätigung ändert sich keine Datei.
- [ ] C1.4 Die Rückfrage nennt die drei Dateien mit Namen und sagt, dass die alten Fassungen mit Zeitstempel im Ablageordner beiseitegelegt und nicht gelöscht werden.
- [ ] C1.5 Die Rückfrage nennt jede Folge, die tatsächlich eintritt. Weicht der geltende Notizordner von `~/krkhome` ab, nennt sie den bisherigen Ort und sagt, dass Notizen, Aufgaben, Termine und Geheimnisse danach unter `~/krkhome` gesucht werden und die Dateien am bisherigen Ort liegen bleiben. Steht eine eigene `keymap.toml`, sagt sie, dass alle eigenen Tastenzuweisungen aus dem Betrieb gehen.
- [ ] C1.6 Vorbelegt ist „Abbrechen“: Return und Esc brechen ab, bestätigt wird mit Cmd+Return. Das entspricht der Rückfrage vor dem Räumen in den Papierkorb.
- [ ] C1.7 Solange ein anderes Blatt steht, ist der Befehl nicht auslösbar. Er folgt damit derselben Regel wie jeder andere Befehl.

**Decisions made:**
- Ort im Menü „KRK“, neben „Neuerungen anzeigen“: auf Wunsch des Nutzers („unter Krk“). Der Block „Anwendung“ ist das Menü „KRK“.
- Keine Tastenkombination ab Werk (Vorgabe). So halten es schon „Ort wählen…“ und „Tastaturdefinition öffnen“. Für einen Befehl, der alle eigenen Zuweisungen aus dem Betrieb nimmt, ist ein versehentlicher Anschlag der teuerste Fehler.
- Rückfrage mit „Abbrechen“ vorbelegt (Vorgabe). Im Projekt fragt jeder Weg nach, der Bestand des Nutzers verdrängt.

### C2: Beiseitelegen und Zurücksetzen der drei Dateien

**Description:** Nach der Bestätigung legt KRK jede vorhandene der drei Dateien unter ihrem Namen mit angehängtem Zeitstempel beiseite. Danach stellt es den Zustand eines ersten Starts her. `readers.toml` und `settings.toml` stehen danach wörtlich als Auslieferungsfassung da. `keymap.toml` fehlt danach, und damit gilt die Auslieferungsbelegung.

**Acceptance criteria:**
- [ ] C2.1 Nach dem Vorgang ist `readers.toml` Byte für Byte gleich der Auslieferungsfassung, die das laufende KRK einkompiliert trägt. Dasselbe gilt für `settings.toml`.
- [ ] C2.2 Nach dem Vorgang steht im Ablageordner keine `keymap.toml`. Stand vorher eine, liegt sie unter ihrem Zeitstempelnamen.
- [ ] C2.3 Jede vorher vorhandene der drei Dateien liegt danach unverändert im Ablageordner unter `<name>.<JJMMTT-HHMM>`, etwa `readers.toml.260929-0815`. Ihr Inhalt ist Byte für Byte der alte.
- [ ] C2.4 Eine Datei, die vorher nicht stand, erzeugt keine Sicherung. Die Meldung sagt, dass für sie nichts beiseitegelegt wurde.
- [ ] C2.5 Ein zweiter Vorgang überschreibt keine frühere Sicherung, auch nicht in derselben Minute. Nach zwei Vorgängen liegen beide alten Fassungen einer Datei im Ablageordner.
- [ ] C2.6 `bookmarks.toml`, `session.toml` und `reported.toml` sind nach dem Vorgang Byte für Byte unverändert. Dasselbe gilt für jede Datei außerhalb des Ablageordners, insbesondere für den Inhalt des bisherigen und des neuen Notizordners.
- [ ] C2.7 Scheitert das Beiseitelegen einer der drei Dateien, wird keine der drei ersetzt, und keine wird entfernt. Die Statuszeile nennt die Datei und den Grund.
- [ ] C2.8 Nach dem Vorgang nennt die Statuszeile den vollen Pfad jeder angelegten Sicherung.
- [ ] C2.9 Ohne Ablageordner, etwa im Messmodus, geschieht nichts, und die Statuszeile sagt, warum.

**Decisions made:**
- Umfang: alle drei von Hand gepflegten Dateien in einem Zug (Nutzer, Antwort 1B). Lesezeichen, Sitzung und die Merkdatei der Neuerungen bleiben unberührt, vorgelegt und nicht widersprochen.
- Sicherungsname mit Zeitstempel, die Meldung nennt die Pfade (Nutzer, Antwort 3A). Die Reihe von Sicherungen wächst mit jedem Vorgang. Das nimmt der Nutzer für einen seltenen, bewussten Schritt in Kauf. Die Regel für beschädigte Dateien (fester Name `.beschaedigt`, erste Kopie bleibt) ist davon unberührt.
- Nichts wird zusammengeführt, die neuen Dateien sind die Auslieferungsfassung (Nutzer, Antwort 4A). Eigene Profile, Zuweisungen und Werte liegen in den Sicherungen und lassen sich von Hand zurückholen.
- **`keymap.toml` wird entfernt und nicht als Auslieferungstext geschrieben** (aus dem Bestand abgeleitet). „Werkseinstellungen“ heißt hier der Zustand nach einem ersten Start, und nach einem ersten Start gibt es keine `keymap.toml`. Eine geschriebene Kopie der Auslieferungsbelegung hätte eine zweite Folge. Jede künftige Fassung von KRK brächte neue Funktionen dann ohne Tastenkombination an, genau der offene Defekt `260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`. Ohne Datei kommen sie mit ihren Kombinationen an, bis der Nutzer in F1 wieder etwas zuweist.
- Alles oder nichts beim Beiseitelegen (Vorgabe): „in einem Zug“ verträgt keinen halb zurückgesetzten Stand, bei dem eine Datei ersetzt ist und eine andere nicht.

### C3: Neu einlesen im laufenden Betrieb

**Description:** Unmittelbar nach dem Zurücksetzen arbeitet KRK mit dem neuen Stand, ohne Neustart. Jede der drei Dateien wirkt über den Weg, den KRK für sie schon kennt oder den diese Arbeit schafft.

**Acceptance criteria:**
- [ ] C3.1 Die Vorschau baut ihre Zusammenfassung sofort aus den neuen Leseprofilen. Zeigt sie beim Vorgang einen erkannten Ort, zeigt sie danach ohne weiteres Zutun die Zusammenfassung nach der Auslieferungsfassung. Beispiel aus 2.1.1: Ein Arbeitspaketordner einer fusion-12-Werkbank zeigt das Profil „fusion-Werkbank: ein Arbeitspaket“.
- [ ] C3.2 Hauptmenü und Tastenabgriff folgen sofort der Auslieferungsbelegung. Eine Kombination, die nur in der alten `keymap.toml` stand, löst nichts mehr aus. Die Kombinationen der Auslieferungsbelegung wirken und stehen im Hauptmenü.
- [ ] C3.3 Die F1-Ansicht zeigt danach die Auslieferungsbelegung.
- [ ] C3.4 Der nächste Aufruf „Terminal öffnen“ nutzt die Terminal-Kennung der Auslieferungsfassung.
- [ ] C3.5 Weicht der geltende Notizordner von dem der Auslieferungsfassung ab, gilt danach `~/krkhome`, mit derselben Wirkung wie eine Wahl über „Ort wählen…“. F2 öffnet danach den Heimordner unter `~/krkhome`. Der Schutz von `secrets.txt` gilt für die `secrets.txt` dort.
- [ ] C3.6 Hält der Editor eine Datei des geltenden Notizordners, bricht der Befehl ab, bevor die Rückfrage aufgeht. Die Statuszeile nennt die Datei, wie es „Ort wählen…“ in derselben Lage tut, und keine der drei Dateien ändert sich.
- [ ] C3.7 Fällt eine andere Prüfung von „Ort wählen…“ gegen `~/krkhome` negativ aus, ändert sich ebenfalls keine der drei Dateien, und die Statuszeile nennt den Grund.
- [ ] C3.8 Stimmt der Notizordner schon mit der Auslieferungsfassung überein, findet kein Ortswechsel statt, und C3.6 greift nicht.
- [ ] C3.9 Nach dem Vorgang zeigt „Neuerungen anzeigen“ den Stand nach dem Zurücksetzen, also keine Unterschiede zwischen Auslieferungsfassung und Nutzerdateien. Der nächste Start meldet keine Neuerungen für die drei Dateien.

**Decisions made:**
- Neu einlesen im Betrieb statt Neustart (Nutzer, Antwort 2A). Die Zusage C4.5 aus der Anforderung zur Profil-Zusammenfassung fällt damit bewusst: „Ändert der Nutzer die `readers.toml`, während KRK läuft, zeigt die Vorschau weiter die Profile des Startzeitpunkts“ (`260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`). Neu gilt: KRK liest die Profile beim Start und beim Zurücksetzen, und bei keinem anderen Anlass. Ein Beobachter auf der Datei entsteht weiterhin nicht.
- Notizordner über den Weg von „Ort wählen…“ samt dessen Prüfungen (Nutzer, Antwort 2A). Diese Prüfungen laufen vor der Rückfrage und nicht erst dahinter (Vorgabe). Sonst stünde auf der Platte ein zurückgesetztes `settings.toml`, während KRK am alten Ort weiterarbeitet.
- Die Merkdatei der Neuerungen bleibt unberührt (C2.6). Die Neuerungen-Meldung ergibt sich aus dem Vergleich und braucht keinen Eingriff in die Merkdatei.

### C4: Anleitung und Auskunft

**Description:** Die Stellen, die dem Nutzer heute den Handgriff „beenden, beiseitelegen, neu starten“ nennen, nennen den Befehl als ersten Weg.

**Acceptance criteria:**
- [ ] C4.1 `README.md` nennt den Befehl im Abschnitt `## Neuerungen an den eigenen Dateien übernehmen` mit seinem Menüort und seinen Folgen. Genannt werden: eigene Zuweisungen gehen aus dem Betrieb, der Notizordner springt auf `~/krkhome`, die alten Dateien liegen mit Zeitstempel im Ablageordner.
- [ ] C4.2 `HowTo.md` nennt den Befehl an der Stelle, die heute den Handgriff für eine vorhandene `readers.toml` nennt.
- [ ] C4.3 Der Hinweis, eine einzelne neue Funktion über „Zuweisen“ (`cmd+t`) in F1 zu belegen statt über ein Zurücksetzen, bleibt in beiden Dateien stehen.
- [ ] C4.4 Die Betriebsregel „die neue Fassung über die alte kopieren und die alte nicht vorher löschen“ bleibt an allen Stellen unverändert.

## Stops when

- Zeigt die Prüfung am Code, dass die Vorschau einen neuen Profilstand nicht ohne Neuaufbau ihrer Tabs übernehmen kann, geht die Arbeit nicht auf einen Neustart über. Sie hält an und legt die Lage dem Nutzer vor, denn Antwort 2A schließt den Neustart aus.
- Zeigt die Prüfung, dass der Weg von „Ort wählen…“ sich nicht mit einem vorgegebenen Ziel statt eines gewählten Ordners gehen lässt, ohne seine Prüfungen zu verdoppeln, hält die Arbeit an und legt die Lage vor. Eine zweite, eigene Prüffolge für den Notizordner entsteht nicht stillschweigend.

## Constraints

- Keine Datei wird gelöscht, bevor ihre alte Fassung beiseitegelegt ist. Die Betriebsregel des Projekts gegen Datenverlust im Ablageordner gilt auch für KRKs eigenen Schreibweg.
- Eine Ablagedatei, die ein symbolischer Verweis ist, wird nicht über den Verweis ersetzt. Ist eine der drei ein Verweis, bricht der Befehl vor jeder Änderung ab und nennt die Datei. Das hält die Regel ein, die schon für den Schreibweg des Notizordners in `settings.toml` gilt.
- `settings.toml` bekommt mit diesem Befehl einen zweiten Schreibweg neben dem des Notizordners. `readers.toml` bekommt einen zweiten Lesezeitpunkt neben dem Start. Beides ist gewollt. Die Proben, die heute genau einen Weg halten, werden bewusst angepasst und nicht gelockert: `die_leseprofile_werden_im_baum_genau_einmal_geladen` (`appkit/anwendung.rs`), `die_profile_haben_genau_einen_schreiber_und_einen_rufer` (`appkit/vorschau.rs`), `die_ortswahl_hat_genau_eine_rufkette`, soweit der Befehl sie berührt. Jede führt danach die neuen Rufer namentlich.
- Der neue Befehl braucht alle Pflichtstellen eines neuen Kommandos, auch `Kommando::KENNUNGEN` und einen eigenen Ausführungszweig beim Anwendungsdelegierten. Ein Befehl ohne eigenen Zweig stünde im Menü und täte nichts.
- Nutzersichtbarer Text trägt Umlaute, Kommentare und Bezeichner die Umschrift.
- Eine zweite laufende Instanz von KRK bleibt bis zu ihrem Neustart auf ihrem alten Stand. Sichert sie danach eine geänderte Belegung aus F1, schreibt sie ihre alte Arbeitskopie zurück. Diesen Preis trägt schon die Entscheidung `260901-0734_*_haelt-krk-die-belegungsdatei-gegen-ihren-zweiten-schreiber-oder-bleibt-es-beim-hinweis.md` (Möglichkeit 1). Diese Arbeit fügt keine Abstimmung zwischen Instanzen hinzu, und durch die Sicherung geht nichts verloren.
- Die zehn Zeitzusagen aus C8 der ersten Runde sind unberührt: Der Befehl läuft nur auf Anforderung und fügt dem Start keine Arbeit hinzu.

## Out of Scope

- Ein Zurücksetzen einzelner Dateien oder eine Auswahl in der Rückfrage. Der Nutzer hat den Zug über alle drei gewählt.
- Ein Zusammenführen eigener Profile, Zuweisungen oder Werte mit der Auslieferungsfassung.
- Lesezeichen, Sitzung und die Merkdatei der Neuerungen.
- Ein automatischer Neustart von KRK.
- Ein Beobachter, der eine von Hand geänderte Ablagedatei im Betrieb bemerkt und nachlädt.
- Eine Schaltfläche im Neuerungen-Blatt, die den Befehl auslöst (Vorgabe). Das Blatt ist eine Auskunft, die nichts schreibt, und die Arbeit, die es gebaut hat, sagt das ausdrücklich zu. Der Befehl steht im selben Menü unmittelbar daneben.
- Eine Neuerungen-Meldung für Profile, die sich unter gleichem Namen geändert haben. Das wäre eine eigene Arbeit.
- Die Behebung des Defekts 260814-0656. Das Zurücksetzen der Belegung beseitigt seine Folgen für den Nutzer bis zur nächsten eigenen Zuweisung, behebt aber nicht die Ursache.
- Das Aufräumen alter Sicherungen im Ablageordner.
- Jede Änderung am bisherigen Notizordner und an seinem Inhalt.

## Open for Planner

- Wie die Vorschau den neuen Profilstand übernimmt und die sichtbare Zusammenfassung neu baut, insbesondere für Tabs, die beim Vorgang nicht aktiv sind.
- Wie der Wechsel des Notizordners den bestehenden Weg von „Ort wählen…“ mit einem vorgegebenen Ziel geht, und wie seine Prüfungen vor der Rückfrage laufen.
- In welcher Reihenfolge und unter welcher Sperre beiseitegelegt und geschrieben wird, damit C2.7 hält. Was bei einem Fehler nach erfolgreichem Beiseitelegen geschieht, entscheidet der Planer, ohne C2.3 und die Constraints zu verletzen.
- Wie ein zweiter Vorgang in derselben Minute einen eindeutigen Namen bekommt (C2.5).
- Wie der gehaltene Stand der Neuerungen und die Urteile der drei Leser nach dem Vorgang nachgezogen werden (C3.9).
- Der genaue Wortlaut der Rückfrage und der Statuszeile, innerhalb von C1.4, C1.5 und C2.8.
- Welche Stellen in `CLAUDE.md`, in den Modulköpfen von `ablage/leseprofile.rs`, `ablage/einstellungen.rs` und `ablage/neuerungen.rs` sowie in den Kommentaren von `resources/default-keymap.toml` nachgezogen werden, weil sie „nur beim Start“ oder „ein Schreibweg“ behaupten.

## User Decisions Pending

- [ ] Keine offene Entscheidung. Die Vorgaben dieses Dokuments sind: keine Tastenkombination, Rückfrage mit „Abbrechen“ vorbelegt, `keymap.toml` wird entfernt statt als Auslieferungstext geschrieben, alles oder nichts beim Beiseitelegen, Prüfung des Notizordners vor der Rückfrage, keine Schaltfläche im Neuerungen-Blatt, keine Abstimmung mit einer zweiten Instanz. Jede davon kann der Nutzer bei der Durchsicht ändern.
