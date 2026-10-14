# Spec: Auf Werkseinstellungen zurücksetzen und neu einlesen

**Date:** 2026-09-29
**Status:** Draft
**Geändert:** 2026-09-29, siehe `## Änderung 260929`
**Source:** Nutzerwunsch, wörtlich: „Wir brauchen einen Befehl unter Krk, der automatisch auf Werkseinstellungen zurücksetzt und die Werte neu einliest.“ Anlass ist die Auslieferung 2.1.1 mit geänderten fusion-Leseprofilen. Ein Nutzer mit vorhandener `readers.toml` bekommt sie bisher nur, wenn er KRK beendet, die Datei beiseitelegt und neu startet, weil KRK seine Ablagedateien allein beim Start liest. Antworten des Nutzers auf die erste Fragerunde: „1 b 2 a 3 a 4 a“.

## Directive

Ein Befehl im Menü „KRK“ setzt die drei von Hand gepflegten Ablagedateien (`readers.toml`, `settings.toml`, `keymap.toml`) nach einer Rückfrage in einem Zug auf den Zustand zurück, den ein erster Start herstellt, mit einer Ausnahme: der Notizordner bleibt, wie er ist. Die alten Dateien legt der Befehl mit Zeitstempel beiseite und löscht keine davon. Danach arbeitet KRK sofort mit den Werkswerten, ohne Neustart: Vorschau, Tastenbelegung, Hauptmenü und Terminal folgen dem neuen Stand, und im Notizordner ändert sich nichts.

## Änderung 260929

Der Nutzer hat die Anforderung nach der ersten Fassung geändert, wörtlich: „Notizen, Secrets und Termine, also die Inhalte in Home dürfen nicht gelöscht werden.“ Auf die Rückfrage hat er gewählt: „Der Notizordner bleibt immer, wie er ist. Der Rest von settings.toml geht auf Werkseinstellung.“

Der Anlass: Sein Notizordner liegt auf Google Drive. Ein Rückfall auf `~/krkhome` ließe Notizen, Aufgaben, Termine und Geheimnisse scheinbar verschwinden, obwohl sie am alten Ort lägen.

Was sich damit ändert:

- `settings.toml` entspricht danach der Auslieferungsfassung bis auf den Wert von `notizordner`, der der vorherige bleibt (C2.1, C2.10).
- Der Befehl wechselt den Notizordner nicht. Damit entfallen die Prüfungen von „Ort wählen…“ vor der Rückfrage, der Abbruch wegen einer gehaltenen Notizdatei, die Sätze der Rückfrage zum Ortswechsel und der Notizordner-Teil des Neu-Einlesens (C1.5, C3.5 bis C3.8, der zweite Punkt unter `## Stops when`).
- Der Befehl löscht, verschiebt und schreibt keine Datei im Notizordner (C2.11).
- Ist `settings.toml` beschädigt oder nicht lesbar, setzt der Befehl nichts zurück und meldet es (C2.12).

Unverändert bleiben alle übrigen Antworten: alle drei Dateien in einem Zug, sofort wirksam, Sicherung mit Zeitstempel, `keymap.toml` fehlt danach.

## Grundlage aus dem Bestand

Wir halten fest, was der Code heute tut, weil mehrere Festlegungen unten darauf ruhen.

- KRK liest `settings.toml` und `readers.toml` einmal beim Start, im selben Durchgang wie die Sitzung und die Erhebung der Neuerungen (`Anwendungsdelegierter::sitzung_laden`, `crates/krk-ui/src/appkit/anwendung.rs`). `keymap.toml` liest es davor, ebenfalls einmal (`belegung::fuer_den_betrieb`).
- `settings.toml` und `readers.toml` legt KRK beim ersten Start wörtlich aus der einkompilierten Auslieferungsfassung an. `keymap.toml` legt KRK nie an; ohne diese Datei gilt die Auslieferungsbelegung vollständig.
- Für die Belegung gibt es einen Neuaufbau im Betrieb: Beim Verlassen der F1-Ansicht mit einer Änderung baut `belegungsansicht_verlassen` Hauptmenü und Tastenabgriff neu auf.
- Das Terminal liest KRK bei jedem Aufruf aus dem gehaltenen Einstellungsstand.
- Die Neuerungen-Meldung vergleicht allein Namen. Ein Leseprofil, das unter gleichem Namen einen neuen Inhalt bekommt, meldet sie nicht.
- Ist `settings.toml` beim Start beschädigt oder nicht lesbar, gilt **kein** Notizordner, und ausdrücklich nicht `~/krkhome`. F2 nennt Grund und Ausweg und legt nichts an (`260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md` im Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`, umgesetzt). Der Schreibweg von „Ort wählen…“ schreibt eine beschädigte Datei nicht; der Nutzer berichtigt sie zuerst (`einstellungen::notizordner_schreiben`).
- Ein Wert von `notizordner`, der kein Text ist (etwa `notizordner = 5`), macht die Datei nicht beschädigt. KRK meldet den Wert und kennt dann keinen Notizordner, `terminal` gilt weiter (`Ortswert::KeinText`).

## Capabilities

### C1: Der Befehl und seine Rückfrage

**Description:** Im Menü „KRK“ steht ein Eintrag „Auf Werkseinstellungen zurücksetzen…“. Er öffnet eine Rückfrage, die sagt, was geschehen wird. Erst die ausdrückliche Bestätigung löst den Vorgang aus.

**Acceptance criteria:**
- [ ] C1.1 Das Menü „KRK“ führt den Eintrag „Auf Werkseinstellungen zurücksetzen…“ im selben Block wie „Neuerungen anzeigen“.
- [ ] C1.2 Ab Werk trägt der Befehl keine Tastenkombination. In der F1-Ansicht steht er unter dem Bereich „Anwendung“ und lässt sich dort wie jede andere Funktion belegen.
- [ ] C1.3 Die Wahl des Eintrags öffnet eine Rückfrage als Blatt am Hauptfenster. Ohne Bestätigung ändert sich keine Datei.
- [ ] C1.4 Die Rückfrage nennt die drei Dateien mit Namen und sagt, dass die alten Fassungen mit Zeitstempel im Ablageordner beiseitegelegt und nicht gelöscht werden.
- [ ] C1.5 Die Rückfrage nennt jede Folge, die tatsächlich eintritt. Steht eine eigene `keymap.toml`, sagt sie, dass alle eigenen Tastenzuweisungen aus dem Betrieb gehen. Sie sagt außerdem, dass der Notizordner und sein Inhalt bleiben, wie sie sind. Einen Satz über einen Ortswechsel des Notizordners enthält sie nicht.
- [ ] C1.6 Vorbelegt ist „Abbrechen“: Return und Esc brechen ab, bestätigt wird mit Cmd+Return. Das entspricht der Rückfrage vor dem Räumen in den Papierkorb.
- [ ] C1.7 Solange ein anderes Blatt steht, ist der Befehl nicht auslösbar. Er folgt damit derselben Regel wie jeder andere Befehl.

**Decisions made:**
- Ort im Menü „KRK“, neben „Neuerungen anzeigen“: auf Wunsch des Nutzers („unter Krk“). Der Block „Anwendung“ ist das Menü „KRK“.
- Keine Tastenkombination ab Werk (Vorgabe). So halten es schon „Ort wählen…“ und „Tastaturdefinition öffnen“. Für einen Befehl, der alle eigenen Zuweisungen aus dem Betrieb nimmt, ist ein versehentlicher Anschlag der teuerste Fehler.
- Rückfrage mit „Abbrechen“ vorbelegt (Vorgabe). Im Projekt fragt jeder Weg nach, der Bestand des Nutzers verdrängt.
- Die Rückfrage sagt, dass der Notizordner bleibt (Vorgabe, Änderung 260929). Wer „Werkseinstellungen“ liest, erwartet sonst den Vorgabeort `~/krkhome`. Die Zusage nimmt ihm die Sorge, die zur Änderung geführt hat, bevor er bestätigt.

### C2: Beiseitelegen und Zurücksetzen der drei Dateien

**Description:** Nach der Bestätigung legt KRK jede vorhandene der drei Dateien unter ihrem Namen mit angehängtem Zeitstempel beiseite. Danach stellt es den Zustand eines ersten Starts her, mit einer Ausnahme. `readers.toml` steht danach wörtlich als Auslieferungsfassung da. `settings.toml` steht als Auslieferungsfassung da, trägt aber den bisherigen Wert von `notizordner`. `keymap.toml` fehlt danach, und damit gilt die Auslieferungsbelegung.

**Acceptance criteria:**
- [ ] C2.1 Nach dem Vorgang ist `readers.toml` Byte für Byte gleich der Auslieferungsfassung, die das laufende KRK einkompiliert trägt. `settings.toml` ist Byte für Byte gleich derselben Auslieferungsfassung, bis auf den Wert von `notizordner`, den C2.10 regelt. Jeder andere Wert, etwa `terminal`, und jede Kommentarzeile sind die der Auslieferungsfassung.
- [ ] C2.2 Nach dem Vorgang steht im Ablageordner keine `keymap.toml`. Stand vorher eine, liegt sie unter ihrem Zeitstempelnamen.
- [ ] C2.3 Jede vorher vorhandene der drei Dateien liegt danach unverändert im Ablageordner unter `<name>.<JJMMTT-HHMM>`, etwa `readers.toml.260929-0815`. Ihr Inhalt ist Byte für Byte der alte. Das gilt auch für die Sicherung von `settings.toml`: sie liegt im Ablageordner neben den beiden anderen und nirgends sonst.
- [ ] C2.4 Eine Datei, die vorher nicht stand, erzeugt keine Sicherung. Die Meldung sagt, dass für sie nichts beiseitegelegt wurde.
- [ ] C2.5 Ein zweiter Vorgang überschreibt keine frühere Sicherung, auch nicht in derselben Minute. Nach zwei Vorgängen liegen beide alten Fassungen einer Datei im Ablageordner.
- [ ] C2.6 `bookmarks.toml`, `session.toml` und `reported.toml` sind nach dem Vorgang Byte für Byte unverändert. Dasselbe gilt für jede Datei außerhalb des Ablageordners.
- [ ] C2.7 Scheitert das Beiseitelegen einer der drei Dateien, wird keine der drei ersetzt, und keine wird entfernt. Die Statuszeile nennt die Datei und den Grund.
- [ ] C2.8 Nach dem Vorgang nennt die Statuszeile den vollen Pfad jeder angelegten Sicherung.
- [ ] C2.9 Ohne Ablageordner, etwa im Messmodus, geschieht nichts, und die Statuszeile sagt, warum.
- [ ] C2.10 Der Wert von `notizordner` in `settings.toml` ist nach dem Vorgang derselbe wie vorher, in derselben Schreibweise, auch wenn er kein Text ist (etwa `notizordner = 5`). Nannte die alte Datei den Schlüssel nicht oder stand keine `settings.toml`, trägt die neue den Wert der Auslieferungsfassung. Stimmt der alte Wert mit dem der Auslieferungsfassung überein, ist die neue `settings.toml` Byte für Byte die Auslieferungsfassung.
- [ ] C2.11 Der Befehl löscht, verschiebt und schreibt keine Datei im Notizordner, weder am geltenden Ort noch unter `~/krkhome`. Er legt dort auch keine Datei und keinen Ordner an. Notizen, Aufgaben, Termine und `secrets.txt` sind nach dem Vorgang Byte für Byte die alten und liegen am alten Ort.
- [ ] C2.12 Ist `settings.toml` beschädigt oder nicht lesbar, ändert sich keine der drei Dateien, und es entsteht keine Sicherung. Die Statuszeile nennt `settings.toml` und den Befund und sagt, dass die Datei zuerst zu berichtigen ist. Der Befehl prüft das, bevor die Rückfrage aufgeht, und die Rückfrage öffnet sich in diesem Fall nicht.

**Decisions made:**
- Umfang: alle drei von Hand gepflegten Dateien in einem Zug (Nutzer, Antwort 1B). Lesezeichen, Sitzung und die Merkdatei der Neuerungen bleiben unberührt, vorgelegt und nicht widersprochen.
- Sicherungsname mit Zeitstempel, die Meldung nennt die Pfade (Nutzer, Antwort 3A). Die Reihe von Sicherungen wächst mit jedem Vorgang. Das nimmt der Nutzer für einen seltenen, bewussten Schritt in Kauf. Die Regel für beschädigte Dateien (fester Name `.beschaedigt`, erste Kopie bleibt) ist davon unberührt.
- Nichts wird zusammengeführt, die neuen Dateien sind die Auslieferungsfassung (Nutzer, Antwort 4A). Eigene Profile, Zuweisungen und Werte liegen in den Sicherungen und lassen sich von Hand zurückholen. Die eine Ausnahme ist der Wert von `notizordner` (Nutzer, Änderung 260929): „Der Notizordner bleibt immer, wie er ist. Der Rest von settings.toml geht auf Werkseinstellung.“
- Der Wert von `notizordner` bleibt in seiner alten Schreibweise stehen, auch als Nicht-Text (Vorgabe). „Bleibt, wie er ist“ schließt eine Deutung des Werts aus. Ein Wert, den KRK heute als unzulässig meldet, bleibt unzulässig und wird weiter gemeldet, statt still gegen `~/krkhome` getauscht zu werden.
- **Bei beschädigter oder unlesbarer `settings.toml` setzt der Befehl nichts zurück** (Vorgabe, aus dem Bestand abgeleitet). Der Wert, den der Befehl erhalten soll, lässt sich dann nicht lesen. Ihn durch `~/krkhome` zu ersetzen, wäre genau der Rückfall, den der Nutzer ausgeschlossen hat, und den der Bestand schon beim Start verweigert: bei beschädigter Datei gilt kein Notizordner und ausdrücklich nicht `~/krkhome` (`260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`, Möglichkeit 1). „Ort wählen…“ verhält sich ebenso und schreibt eine beschädigte Datei nicht. Der Abbruch umfasst alle drei Dateien, weil „in einem Zug“ keinen halb zurückgesetzten Stand verträgt (siehe unten). Wer die beschädigte Datei wirklich verwerfen will, hat dafür weiter den Handgriff „beiseitelegen und neu starten“ als eigenen, bewussten Schritt.
- **`keymap.toml` wird entfernt und nicht als Auslieferungstext geschrieben** (aus dem Bestand abgeleitet). „Werkseinstellungen“ heißt hier der Zustand nach einem ersten Start, und nach einem ersten Start gibt es keine `keymap.toml`. Eine geschriebene Kopie der Auslieferungsbelegung hätte eine zweite Folge. Jede künftige Fassung von KRK brächte neue Funktionen dann ohne Tastenkombination an, genau der offene Defekt `260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`. Ohne Datei kommen sie mit ihren Kombinationen an, bis der Nutzer in F1 wieder etwas zuweist.
- Alles oder nichts beim Beiseitelegen (Vorgabe): „in einem Zug“ verträgt keinen halb zurückgesetzten Stand, bei dem eine Datei ersetzt ist und eine andere nicht.

### C3: Neu einlesen im laufenden Betrieb

**Description:** Unmittelbar nach dem Zurücksetzen arbeitet KRK mit dem neuen Stand, ohne Neustart. Jede der drei Dateien wirkt über den Weg, den KRK für sie schon kennt oder den diese Arbeit schafft. Der Notizordner bleibt der, mit dem KRK vorher gearbeitet hat.

**Acceptance criteria:**
- [ ] C3.1 Die Vorschau baut ihre Zusammenfassung sofort aus den neuen Leseprofilen. Zeigt sie beim Vorgang einen erkannten Ort, zeigt sie danach ohne weiteres Zutun die Zusammenfassung nach der Auslieferungsfassung. Beispiel aus 2.1.1: Ein Arbeitspaketordner einer fusion-12-Werkbank zeigt das Profil „fusion-Werkbank: ein Arbeitspaket“.
- [ ] C3.2 Hauptmenü und Tastenabgriff folgen sofort der Auslieferungsbelegung. Eine Kombination, die nur in der alten `keymap.toml` stand, löst nichts mehr aus. Die Kombinationen der Auslieferungsbelegung wirken und stehen im Hauptmenü.
- [ ] C3.3 Die F1-Ansicht zeigt danach die Auslieferungsbelegung.
- [ ] C3.4 Der nächste Aufruf „Terminal öffnen“ nutzt die Terminal-Kennung der Auslieferungsfassung.
- [ ] C3.5 Der Notizordner ist nach dem Vorgang derselbe wie vorher, im laufenden KRK und nach dem nächsten Start. F2 öffnet denselben Heimordner wie vorher. Der Schutz von `secrets.txt` gilt unverändert für dieselbe `secrets.txt`. Eine im Editor gehaltene Datei des Notizordners bleibt geöffnet und unverändert.
- C3.6 entfällt (Änderung 260929): kein Abbruch wegen einer gehaltenen Notizdatei, weil kein Ortswechsel stattfindet.
- C3.7 entfällt (Änderung 260929): die Prüfungen von „Ort wählen…“ laufen nicht.
- C3.8 entfällt (Änderung 260929): aufgegangen in C3.5.
- [ ] C3.9 Nach dem Vorgang zeigt „Neuerungen anzeigen“ den Stand nach dem Zurücksetzen, also keine Unterschiede zwischen Auslieferungsfassung und Nutzerdateien. Der nächste Start meldet keine Neuerungen für die drei Dateien.

**Decisions made:**
- Neu einlesen im Betrieb statt Neustart (Nutzer, Antwort 2A). Die Zusage C4.5 aus der Anforderung zur Profil-Zusammenfassung fällt damit bewusst: „Ändert der Nutzer die `readers.toml`, während KRK läuft, zeigt die Vorschau weiter die Profile des Startzeitpunkts“ (`260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`). Neu gilt: KRK liest die Profile beim Start und beim Zurücksetzen, und bei keinem anderen Anlass. Ein Beobachter auf der Datei entsteht weiterhin nicht.
- Kein Ortswechsel des Notizordners (Nutzer, Änderung 260929). Damit ersetzt diese Änderung die frühere Festlegung, den Notizordner über den Weg von „Ort wählen…“ samt dessen Prüfungen auf `~/krkhome` zu setzen. Die Nummern C3.6 bis C3.8 bleiben als Leerstellen stehen, damit Verweise des Plans auf C3.9 gültig bleiben.
- Die Merkdatei der Neuerungen bleibt unberührt (C2.6). Die Neuerungen-Meldung ergibt sich aus dem Vergleich und braucht keinen Eingriff in die Merkdatei.

### C4: Anleitung und Auskunft

**Description:** Die Stellen, die dem Nutzer heute den Handgriff „beenden, beiseitelegen, neu starten“ nennen, nennen den Befehl als ersten Weg.

**Acceptance criteria:**
- [ ] C4.1 `README.md` nennt den Befehl im Abschnitt `## Neuerungen an den eigenen Dateien übernehmen` mit seinem Menüort und seinen Folgen. Genannt werden: eigene Zuweisungen gehen aus dem Betrieb, der Notizordner und sein Inhalt bleiben, wie sie sind, die alten Dateien liegen mit Zeitstempel im Ablageordner, und eine beschädigte `settings.toml` ist vorher zu berichtigen.
- [ ] C4.2 `HowTo.md` nennt den Befehl an der Stelle, die heute den Handgriff für eine vorhandene `readers.toml` nennt.
- [ ] C4.3 Der Hinweis, eine einzelne neue Funktion über „Zuweisen“ (`cmd+t`) in F1 zu belegen statt über ein Zurücksetzen, bleibt in beiden Dateien stehen.
- [ ] C4.4 Die Betriebsregel „die neue Fassung über die alte kopieren und die alte nicht vorher löschen“ bleibt an allen Stellen unverändert.

## Stops when

- Zeigt die Prüfung am Code, dass die Vorschau einen neuen Profilstand nicht ohne Neuaufbau ihrer Tabs übernehmen kann, geht die Arbeit nicht auf einen Neustart über. Sie hält an und legt die Lage dem Nutzer vor, denn Antwort 2A schließt den Neustart aus.
- Der frühere zweite Punkt zum Weg von „Ort wählen…“ mit festem Ziel entfällt mit der Änderung 260929.

## Constraints

- Keine Datei wird gelöscht, bevor ihre alte Fassung beiseitegelegt ist. Die Betriebsregel des Projekts gegen Datenverlust im Ablageordner gilt auch für KRKs eigenen Schreibweg.
- Der Befehl berührt ausschließlich Dateien im Ablageordner. Kein Schreib-, Lösch- oder Verschiebeweg dieser Arbeit zielt auf den Notizordner.
- Eine Ablagedatei, die ein symbolischer Verweis ist, wird nicht über den Verweis ersetzt. Ist eine der drei ein Verweis, bricht der Befehl vor jeder Änderung ab und nennt die Datei. Das hält die Regel ein, die schon für den Schreibweg des Notizordners in `settings.toml` gilt.
- `settings.toml` bekommt mit diesem Befehl einen zweiten Schreibweg neben dem des Notizordners. `readers.toml` bekommt einen zweiten Lesezeitpunkt neben dem Start. Beides ist gewollt. Die Proben, die heute genau einen Weg halten, werden bewusst angepasst und nicht gelockert: `die_leseprofile_werden_im_baum_genau_einmal_geladen` (`appkit/anwendung.rs`) und `die_profile_haben_genau_einen_schreiber_und_einen_rufer` (`appkit/vorschau.rs`). Jede führt danach die neuen Rufer namentlich. `die_ortswahl_hat_genau_eine_rufkette` berührt der Befehl nicht mehr und bleibt, wie sie ist.
- Der neue Befehl braucht alle Pflichtstellen eines neuen Kommandos, auch `Kommando::KENNUNGEN` und einen eigenen Ausführungszweig beim Anwendungsdelegierten. Ein Befehl ohne eigenen Zweig stünde im Menü und täte nichts.
- Nutzersichtbarer Text trägt Umlaute, Kommentare und Bezeichner die Umschrift.
- Eine zweite laufende Instanz von KRK bleibt bis zu ihrem Neustart auf ihrem alten Stand. Sichert sie danach eine geänderte Belegung aus F1, schreibt sie ihre alte Arbeitskopie zurück. Diesen Preis trägt schon die Entscheidung `260901-0734_*_haelt-krk-die-belegungsdatei-gegen-ihren-zweiten-schreiber-oder-bleibt-es-beim-hinweis.md` (Möglichkeit 1). Diese Arbeit fügt keine Abstimmung zwischen Instanzen hinzu, und durch die Sicherung geht nichts verloren.
- Die zehn Zeitzusagen aus C8 der ersten Runde sind unberührt: Der Befehl läuft nur auf Anforderung und fügt dem Start keine Arbeit hinzu.

## Out of Scope

- Ein Zurücksetzen einzelner Dateien oder eine Auswahl in der Rückfrage. Der Nutzer hat den Zug über alle drei gewählt.
- Ein Zusammenführen eigener Profile, Zuweisungen oder Werte mit der Auslieferungsfassung, außer dem Erhalt des Werts von `notizordner`.
- Jeder Wechsel des Notizordners durch den Befehl und jede Änderung am Notizordner und an seinem Inhalt.
- Ein Heilen einer beschädigten `settings.toml` durch den Befehl.
- Lesezeichen, Sitzung und die Merkdatei der Neuerungen.
- Ein automatischer Neustart von KRK.
- Ein Beobachter, der eine von Hand geänderte Ablagedatei im Betrieb bemerkt und nachlädt.
- Eine Schaltfläche im Neuerungen-Blatt, die den Befehl auslöst (Vorgabe). Das Blatt ist eine Auskunft, die nichts schreibt, und die Arbeit, die es gebaut hat, sagt das ausdrücklich zu. Der Befehl steht im selben Menü unmittelbar daneben.
- Eine Neuerungen-Meldung für Profile, die sich unter gleichem Namen geändert haben. Das wäre eine eigene Arbeit.
- Die Behebung des Defekts 260814-0656. Das Zurücksetzen der Belegung beseitigt seine Folgen für den Nutzer bis zur nächsten eigenen Zuweisung, behebt aber nicht die Ursache.
- Das Aufräumen alter Sicherungen im Ablageordner.

## Open for Planner

- Wie die Vorschau den neuen Profilstand übernimmt und die sichtbare Zusammenfassung neu baut, insbesondere für Tabs, die beim Vorgang nicht aktiv sind.
- Wie der bisherige Wert von `notizordner` in die Auslieferungsfassung kommt, sodass C2.1 und C2.10 zugleich halten, und wie die Prüfung auf eine beschädigte oder unlesbare `settings.toml` vor der Rückfrage läuft (C2.12). Ob dafür der Schreibweg von `notizordner_schreiben` oder dessen Prüfung wiederverwendet wird, entscheidet der Planer; eine zweite, abweichende Regel für „beschädigt“ entsteht nicht.
- In welcher Reihenfolge und unter welcher Sperre beiseitegelegt und geschrieben wird, damit C2.7 hält. Was bei einem Fehler nach erfolgreichem Beiseitelegen geschieht, entscheidet der Planer, ohne C2.3 und die Constraints zu verletzen.
- Wie ein zweiter Vorgang in derselben Minute einen eindeutigen Namen bekommt (C2.5).
- Wie der gehaltene Stand der Neuerungen und die Urteile der drei Leser nach dem Vorgang nachgezogen werden (C3.9).
- Wie der gehaltene Einstellungsstand im Betrieb nachgezogen wird, ohne den geltenden Notizordner anzufassen (C3.4, C3.5).
- Der genaue Wortlaut der Rückfrage und der Statuszeile, innerhalb von C1.4, C1.5, C2.8 und C2.12.
- Welche Stellen in `CLAUDE.md`, in den Modulköpfen von `ablage/leseprofile.rs`, `ablage/einstellungen.rs` und `ablage/neuerungen.rs`, in den Kommentaren von `resources/default-settings.toml` sowie in den Kommentaren von `resources/default-keymap.toml` nachgezogen werden, weil sie „nur beim Start“, „ein Schreibweg“ oder „ersetzt die ganze Datei“ behaupten.

## User Decisions Pending

- [ ] Keine offene Entscheidung. Die Vorgaben dieses Dokuments sind: keine Tastenkombination, Rückfrage mit „Abbrechen“ vorbelegt, die Rückfrage sagt, dass der Notizordner bleibt, `keymap.toml` wird entfernt statt als Auslieferungstext geschrieben, der Wert von `notizordner` bleibt in alter Schreibweise, bei beschädigter oder unlesbarer `settings.toml` wird nichts zurückgesetzt, alles oder nichts beim Beiseitelegen, keine Schaltfläche im Neuerungen-Blatt, keine Abstimmung mit einer zweiten Instanz. Jede davon kann der Nutzer bei der Durchsicht ändern.
