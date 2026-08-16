# KRK setzt Befehle ab und führt gespeicherte Makros aus

---
**Domain:** code
**Status:** active
**Filed by:** orchestrator
**Active spec/plan:** shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md
**Active session history:** shared/history/260816-2113-orchestrator-session.md

---

## Directive

Wer in KRK einen Befehl absetzen will, öffnet ein Blatt, tippt ihn und sieht seine Ausgabe fortlaufend in einem angehefteten Vorschau-Tab, während die Statuszeile den laufenden Vorgang trägt und `Esc` ihn abbricht. Häufig gebrauchte Befehle stehen als benannte Vorlagen in einer von Hand gepflegten Makrodatei, mit Platzhaltern für den angezeigten Ordner, den Ordner der anderen Seite, die ausgewählten Einträge und den Eintrag unter dem Cursor; gestartet werden sie aus einer Liste oder über einen von neun Plätzen der Tastenbelegung, und freie Argumente fragt KRK vorher nach. Ein eingebautes Terminal entsteht dabei nicht.

## Grounding snapshot

Erhoben am 260816 gegen den Baumstand `627b5f4`, Version 0.5.0. Die zwölfte Runde des Projekts; elf sind gefahren, zehn davon beschränkt geschlossen.

### Woher das Vorhaben kommt

Der Nutzer hat zwei Wünsche genannt: Bash-Befehle absetzen und Makros speichern und ausführen. Seine Beispiele sind alle Dateien nach einem Muster auflisten, ein Replace-Skript im Baum mit Argumenten rufen, git-Befehle, und eine Kommandozeilenanwendung starten, etwa `fusion`. Die Vorprüfung im Chat vom 260815 hat den Gegenstand umrissen und der Nutzer ihn festgelegt: KRK setzt einen Befehl ab und zeigt dessen Ausgabe. Ein Werkzeug, das selbst die Tastatur führt und dafür ein eingebautes Terminal verlangte, gehört nicht dazu.

Die Beratung `shared/consult/260815-1354-befehlslauf-und-makros-in-krk.md` hat den Baum daraufhin geprüft und eine Runde mit vier Fähigkeiten empfohlen. Der Nutzer hat am 260816 alle vier bestellt.

### Was der Baum schon trägt und diese Runde erbt

Der Befehlslauf besteht aus zwei Teilen, und für jeden steht eine ausgereifte Vorlage im Baum; zusammengeführt sind sie an keiner Stelle, und darin liegt die Bauarbeit.

**Der Lauf** hat seine Vorlage in den Dateioperationen (`crates/krk-ui/src/kommandos/operationen.rs`): ein Arbeitsfaden meldet über einen Kanal an einen Vermittlerfaden, der einen Weckruf über die Hauptschlange absetzt, worauf der Hauptfaden den Stand liest und zeichnet. Die Kette trägt fortlaufende Meldungen, eine Bündelung ohne Zeitgeber, einen Abbruchgriff und eine Vorgangszeile in der Statuszeile, die den Abbruch im eigenen Text nennt.

**Die Anzeige** hat ihre Vorlage in `Vorschaumodell::zwischenablage_anzeigen` (`crates/krk-ui/src/vorschaumodell.rs`): die Vorschau zeigt schon heute etwas an, das keine Datei ist, ohne dass eine eigene Tab-Sorte entstünde. Die Befehlsausgabe ist die dritte Quelle nach der Datei und der Zwischenablage.

Wo die beiden Vorlagen sich nicht berühren, liegt der Zuschnitt: `Ladevorgang` schickt genau eine Meldung und kennt weder Fortschritt noch Abbruch, ein laufender Befehl liefert fortlaufend und muss beim Abbruch getötet werden statt vergessen.

**Der erste Unterprozess dieses Vorhabens.** KRK startet heute keinen einzigen im Produktivcode; `std::process::Command` steht allein in Prüfdateien. Der Modulkopf von `crates/krk-ui/src/appkit/terminal.rs` nennt genau das als einen der drei Gründe gegen `open -a`: ein Unterprozess wäre der erste, "mit den Fragen, wer ihn abholt und was der Hauptfaden solange tut". Dieselbe Runde, die ihn einführt, bringt die Antwort mit, deren Fehlen 260805 das Argument gegen ihn war.

### Die elf Festlegungen des Nutzers vom 260816

Sie stehen ausformuliert im Spec unter `## Was der Nutzer am 260816 entschieden hat` und werden hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderlaufen. In Stichworten: PATH einmal beim Start aus der Anmeldeshell, nebenher erfragt; beide Ausgabeströme zusammen in die Vorschau und der Rückgabewert in die Statuszeile; Makros von Hand gepflegt, KRK schreibt die Datei nie; genau ein laufender Vorgang; ein Vorschau-Tab kann angeheftet sein; 1 MB Anzeige bei vollständigem Lauf; Abbruch trifft die Prozessgruppe; neun Makroplätze in einem zehnten Funktionsbereich; eine fehlerhafte Datei kostet beim Neu-Einlesen nichts; `NO_COLOR` und `TERM` plus Filter.

### Was diese Runde ausdrücklich nicht tut

Kein eingebautes Terminal, kein sechster Bereich der Fensterzeile, keine Oberfläche zum Anlegen oder Ändern von Makros, keine Prüfung oder Deutung des Makrotexts, keine Git-Anbindung, keine elfte Zeitzusage, keine Verlaufsliste abgesetzter Befehle, keine mehreren gleichzeitigen Läufe, kein zehnter Makroplatz.

**Die Abgrenzung „KRK als Kommandozentrale für Fusion" aus der Runde 1 bleibt bestehen.** Sie bindet diese Runde nicht und wird von ihr nicht aufgehoben. Die Runde bewegt sich sichtbar in ihre Richtung, denn ein Makro kann jedes Kommandozeilenwerkzeug rufen, `fusion` eingeschlossen. Was entsteht, ist ein Weg, einen Befehl abzusetzen, und keine Kenntnis irgendeines Werkzeugs: KRK kennt kein Fusion, keine Circles und keine Marker.

## Dependencies

- `circles/260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1. Sie liefert die Dateioperationen mit Vermittlerfaden und Vorgangszeile, das Vorschaufenster, die Tastenbelegung und die zehn Zeitzusagen aus C8. Ihr Abschnitt `## Ausdrücklich außerhalb dieses Circles` führt „KRK als Kommandozentrale für Fusion"; diese Runde nennt die Abgrenzung ausdrücklich und hebt sie nicht auf.
- `circles/260807-2116-eingebauter-editor-mit-textmarken` — die Runde 2. Ihr Editor ist das Ziel des Befehls „Makrodatei im Editor öffnen".
- `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` — die Runde 9. Sie hat die Ablage von vier auf sechs Dateien gebracht; die Makrodatei ist die siebte.
- `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content` — die Runde 11. Ihr Ankreuzfeld wird mit leerer Tastenliste ausgeliefert statt mit `reserviert_fuer`; die neun Makroplätze folgen diesem Vorbild.
- `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — vorgesehen, nicht gefahren. Er greift dieselbe Fläche an, die Tabs des Vorschaufensters. Der Nutzer hat am 260816 entschieden, diese Runde zuerst zu fahren: sie legt mit der Anheftung die Regel fest, nach der eine fremde Quelle in einen Vorschau-Tab schreibt, und der Web-Betrachter wäre die vierte nach derselben Regel.

**Bindende Datensätze, die diese Runde nicht aufhält:**

- `shared/decisions/260813-0053_o_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md` — offen. Die Runde baut ein zehntes Obermenü und folgt damit der Empfehlung des Datensatzes, ohne ihn zu schließen.
- `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_o_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md` — offen. Die Runde fügt `Esc` keine Bedeutung hinzu.
- `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` — offen. Alle dreizehn Funktionen dieser Runde werden ohnehin unbelegt ausgeliefert, also geht nichts verloren.

## Turn log

(noch kein Turn gefahren)

## Closure note

(offen)
