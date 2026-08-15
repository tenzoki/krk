# Consultation: Befehle absetzen und Makros speichern, Weg A

**Date:** 2026-08-15 13:54
**Status:** Complete
**Requested by:** k1, Chat vom 260815

## Question

Was braucht KRK, um Terminalfunktionalität aufzunehmen? Der Nutzer nennt zwei Wünsche: Bash-Befehle absetzen und Makros speichern und ausführen, mit Beispielen wie „alle Dateien nach einem Muster auflisten", „ein Replace-Skript im Baum mit Argumenten rufen", „git-Befehle" und „eine CLI starten, etwa fusion".

Die Vorprüfung im Chat hat die Wünsche in zwei Vorhaben getrennt. Weg A setzt einen Befehl ab und zeigt seine Ausgabe. Weg B startet ein Programm, das selbst die Tastatur führt, und verlangt ein Terminal-Emulat mit Pseudoterminal, ANSI-Zerleger und Bildschirmmodell. **Der Nutzer hat sich am 260815 auf Weg A beschränkt.** Dieser Bericht behandelt allein Weg A. Weg B bleibt außerhalb und wird hier nur dort erwähnt, wo eine Entscheidung ihn später verbaut oder offen hält.

## Context

KRK steht nach zehn gefahrenen Runden auf Version 0.4.1. Der Baum trägt einen Navigator mit zwei Dateifenstern, ein Vorschaufenster, einen eingebauten Editor, Dateioperationen mit Fortschritt und Abbruch, eine frei konfigurierbare Tastenbelegung mit 78 Kommandos und zehn Blätter am Hauptfenster. Die Anwendung läuft außerhalb der App-Sandbox (Technologiewahl vom 260802-1150), ein Unterprozess braucht also keine Berechtigung und keinen Eintrag in der Bündelbeschreibung.

Berührt ist eine Abgrenzung der ersten Runde. Ihr Circle-Datensatz führt unter `## Ausdrücklich außerhalb dieses Circles` den Punkt „KRK als Kommandozentrale für Fusion". Die Abgrenzung galt jener Runde und bindet keine spätere, verdient in einem neuen Circle-Datensatz aber eine ausdrückliche Nennung: eine Runde, die Makros für den Aufruf beliebiger Kommandozeilenwerkzeuge baut, bewegt sich sichtbar in ihre Richtung.

## Analysis

### Der Baum trägt beide Hälften des Vorhabens, aber keine Stelle trägt es ganz

Ein Befehlslauf besteht aus zwei Teilen, und für jeden gibt es im Baum eine ausgereifte Vorlage. Zusammengeführt sind sie an keiner Stelle, und genau darin liegt die eigentliche Bauarbeit.

**Der Lauf** hat seine Vorlage in den Dateioperationen. `crates/krk-ui/src/kommandos/operationen.rs:24-33` zeichnet die Kette: ein Arbeitsfaden meldet über einen Kanal an einen Vermittlerfaden, der einen Weckruf über die Hauptschlange von Grand Central Dispatch absetzt, worauf der Hauptfaden den Stand liest und zeichnet. Gestartet wird der Vermittler in `crates/krk-ui/src/appkit/anwendung.rs:4977-5005`. Die Kette trägt bereits alles, was ein laufender Befehl braucht: fortlaufende Meldungen, eine Bündelung ohne Zeitgeber, die einen Weckruf verwirft solange der vorige nicht gezeichnet ist, einen Abbruchgriff und eine Vorgangszeile in der Statuszeile, die den Abbruch im eigenen Text nennt.

**Die Anzeige** hat ihre Vorlage im Vorschaufenster, und der tragende Befund ist, dass die Vorschau schon heute etwas anzeigt, das keine Datei ist. `Vorschaumodell::zwischenablage_anzeigen` (`crates/krk-ui/src/vorschaumodell.rs:429-445`) setzt Titel, Inhalt und einen leeren Pfad im aktiven Tab, ohne Arbeitsfaden und ohne dass eine eigene Tab-Sorte entstünde. Der Modulkopf sagt die Regel dahinter ausdrücklich: jede Quelle schreibt in den aktiven Tab und in keinen anderen, und eine Tab-Sorte mit eigener Regel entsteht nicht, auch nicht für die Zwischenablage. Eine Befehlsausgabe wäre die dritte Quelle nach der Datei und der Zwischenablage und fügt sich in dieselbe Regel.

Wo die beiden Vorlagen sich nicht berühren, liegt der Zuschnitt der Arbeit. `Ladevorgang` (`crates/krk-ui/src/vorschaumodell.rs:246-280`) ist ein Faden, der genau eine Meldung schickt und danach fertig ist; sein Kanal hat die Tiefe 1, er kennt keinen Fortschritt und keinen Abbruch, und eine neuere Quelle verwirft ihn, indem sie das Feld auf `None` setzt. Ein laufender Befehl liefert fortlaufend und muss beim Abbruch getötet werden, nicht vergessen. Die Vorschau liefert also die Fläche und die Tabs, die Dateioperationen liefern den Lauf, und die Verbindung der beiden ist neu.

### Der erste Unterprozess des Vorhabens

KRK startet heute keinen einzigen Unterprozess im Produktivcode. Nachgesehen über den ganzen Baum findet sich `std::process::Command` allein in Prüfdateien, nirgends unter `crates/krk-ui/src` oder `crates/krk-core/src`. Der Modulkopf von `crates/krk-ui/src/appkit/terminal.rs:22-27` nennt das ausdrücklich als einen der drei Gründe, aus denen der Terminalaufruf aus C11 nicht über `open -a` läuft: er „wäre der erste Unterprozess dieses Vorhabens, mit den Fragen, wer ihn abholt und was der Hauptfaden solange tut".

Weg A stellt diese Fragen und beantwortet sie. Der Vermittlerfaden ist die Antwort auf beide, und die Symmetrie ist bemerkenswert genug, sie zu nennen: dieselbe Runde, die den Unterprozess einführt, bringt die Maschinerie mit, deren Fehlen 260805 das Argument gegen ihn war.

Zwei Eigenschaften des Laufs sind aus den Beispielen des Nutzers ableitbar und sollten früh festgeschrieben werden. Der Lauf geht durch eine Shell, denn „Dateien nach Muster auflisten" verlangt Namensausdehnung und die genannten Skriptaufrufe verlangen Röhren und Verkettungen; ein direktes `execve` ohne Shell könnte davon nichts. Und die Ausgabe wird fortlaufend gelesen und nicht am Ende eingesammelt, weil ein Befehl über einem großen Baum sonst minutenlang eine leere Fläche zeigt.

### Die Einsetzung von Argumenten ist entscheidbar, die Wirkung des Befehls nicht

Ein Makro, das den ausgewählten Eintrag in eine Befehlszeile einsetzt, wirft die Frage nach Anführung und Sonderzeichen auf. Zwei Fragen liegen hier dicht beieinander und sind verschieden, und die Trennung entscheidet, ob die Runde eine lösbare Aufgabe bearbeitet.

Was ein Befehl anfassen wird, lässt sich aus seinem Text nicht bestimmen. Ein Pfad kann zur Laufzeit entstehen, über eine Variable kommen oder durch `eval` laufen. Wer versucht, aus der Makrovorlage vorherzusagen, welche Dateien der Lauf schreibt, bearbeitet eine unentscheidbare Frage. Fusions eigene Regelsammlung führt genau diesen Fall als ausgeschriebenes Beispiel (`rules/critical-stance.md`, Abschnitt 4).

KRK braucht diese Vorhersage nicht. Die Frage, die es beantworten muss, lautet: wie wird ein beliebiger Dateiname so in eine Shell-Zeile gesetzt, dass die Shell ihn als genau einen Wert liest. Diese Frage ist entscheidbar und hat eine vollständige Antwort, die Einzelanführung mit Verdopplung des Anführungszeichens im Wert. Der Makrotext selbst bleibt dabei ungeprüft, und das ist richtig so: der Nutzer schreibt ihn, er ist sein Code, und eine Prüfung darüber wäre die unentscheidbare Frage durch die Hintertür.

### Die Makroablage passt in keine der vier vorhandenen Dateien

KRK führt heute vier Ablagedateien unter `~/Library/Application Support/KRK/`, und keine nimmt Makros auf. Die Herleitung steht in `crates/krk-core/src/ablage/einstellungen.rs`, dessen Modulkopf schon einmal dieselbe Prüfung für die Terminal-Bündelkennung gefahren hat.

`keymap.toml` scheidet aus, weil der Rücksetzbefehl aus C3 die gesamte Belegung ersetzt und die Makros mitnähme. `session.toml` scheidet aus, weil KRK sie alle zwei Sekunden überschreibt. `bookmarks.toml` hält Ordnerverweise. `settings.toml` scheidet aus dem interessantesten Grund aus: sie ist ausdrücklich die eine Datei, die KRK im Betrieb nie schreibt, und ihre Aufnahmeregel verlangt einen Wert, der in seiner Runde keine Oberfläche hat. Sobald eine Ansicht einen Wert ändern kann, kommt ein Schreibweg dazu, und ein Schreibweg löscht die Kommentare, die den Sinn der Datei ausmachen.

Daraus folgt die Frage, die den Zuschnitt der Runde spürbar verschiebt: bekommen Makros eine Oberfläche in KRK, oder werden sie von Hand in einer Datei gepflegt? Ohne Oberfläche kann eine fünfte Datei nach dem Muster von `settings.toml` entstehen, kommentiert, einmal angelegt und nie wieder geschrieben. Mit Oberfläche braucht sie den Schreibweg über `ablage::atomar` und verliert damit ihre Kommentare, was einen zweiten Ort für die Erklärung verlangt. Die zweite Möglichkeit ist die größere Runde und die bessere Bedienung.

### Von der Taste zum Makro führt heute kein Weg

Die Tastenbelegung ordnet Kombinationen einer geschlossenen Aufzählung zu. `Kommando` trägt am 260815 achtundsiebzig Varianten, und die Zuordnung von Kennung zu Variante steht als feste Paartabelle in `crates/krk-core/src/tasten/belegung.rs:662`. Die Fallunterscheidungen darüber haben absichtlich keinen Auffangzweig: jedes neue Kommando braucht eine Zeile in `Kommando::wirkungsbereich` (`belegung.rs:752`) und eine in `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs`), und der Übersetzer hält den Bau an, bis beide stehen.

Eine im Betrieb wachsende Zahl von Makros passt in diese Form nicht. Zwei Zuschnitte passen. Nummerierte Plätze führen eine feste Menge von Varianten ein, etwa `makro_1` bis `makro_9`, jede mit ihrer Zeile in beiden Tabellen; die Belegung bleibt unverändert in ihrer heutigen Form, und ein Makro ohne Platz ist über eine Liste erreichbar statt über eine eigene Taste. Ein parametrisiertes Kommando träfe die Sache genauer, kostet aber den Umbau der Paartabelle und jeder vollständigen Fallunterscheidung darüber, und es bricht die Eigenschaft, dass die Belegungsansicht aus C3 jede Funktion mit Namen zeigen kann.

Wir empfehlen die nummerierten Plätze. Der Preis ist eine willkürliche Obergrenze, der Gewinn ist, dass keine der gewachsenen Aufzählungen ihre Form verliert.

### Was Weg A auch dann nicht kann

Drei Grenzen gehören in den Spec, damit sie nicht später als Defekt erscheinen.

Ein Befehl, der nach einer Eingabe fragt, hängt. `sudo` ohne zwischengespeicherte Berechtigung, eine Passwortabfrage, `git commit` ohne `-m`, das den Editor aus `$EDITOR` startet: alle drei warten auf ein Terminal, das es nicht gibt, und KRK sieht den Unterschied zu einem langen Lauf nicht. Der Abbruch bleibt die Antwort darauf, und der Spec sollte es zusagen statt es offenzulassen.

Farbige und bildschirmsteuernde Ausgabe erscheint als Zeichenmüll. Viele Werkzeuge erkennen ein fehlendes Terminal und schalten die Steuerzeichen selbst ab, verlässlich ist das nicht. `inference:` Ein Filter, der die gängigen Farbfolgen aus der Ausgabe entfernt, ist die kleine Antwort darauf und keine eigene Runde wert; geprüft ist das an keiner Stelle des Baums.

Die Ausgabe braucht eine Größengrenze. Die Vorschau trägt für Text 1 MB (`crates/krk-ui/src/vorschaumodell.rs:121`), und ein rekursiver Aufruf über einen großen Baum überschreitet sie mühelos. Ob die Ausgabegrenze dieselbe Zahl trägt oder eine eigene, ist eine Frage an den Spec; dass sie eine trägt, ist keine.

### Der Rückstand bei den Zeitzusagen wächst um eine Runde

Der letzte vollständige Abnahmelauf der zehn Zeitzusagen stammt vom 260810 und liegt vor den Runden 5 bis 10; keine der sechs ist gegen die Zusagen gemessen. Eine elfte Runde ändert daran nichts und vergrößert den Abstand.

Ein belegter Präzedenzfall verdient dabei Beachtung. Die Closure Note der ersten Runde nennt `9a47c4a` als einen von drei Commits, die eine Messreihe altern ließen, und der Grund war eine Erweiterung der Kommando-Aufzählung, durch die jeder Tastendruck läuft. Weg A erweitert dieselbe Aufzählung. Der Effekt ist vermutlich klein, gemessen ist er nicht.

## Recommendations

**Wir empfehlen einen Circle mit vier Fähigkeiten, in dieser Reihenfolge gebaut.**

Die erste ist der Befehlslauf ohne Makros: ein Kommando, das ein Blatt mit einem Eingabefeld öffnet, den eingegebenen Text durch `/bin/sh -c` fährt und die Ausgabe fortlaufend in einen Vorschau-Tab schreibt, mit Abbruch über die Vorgangszeile der Statuszeile. Das Eingabeblatt hat seine Vorlage in `crates/krk-ui/src/appkit/blaetter/pfadeingabe.rs` und `namenseingabe.rs`, der Lauf die in den Dateioperationen, die Anzeige die in `zwischenablage_anzeigen`. Nach dieser Fähigkeit allein ist das Vorhaben schon brauchbar.

Die zweite ist die Makroablage: eine fünfte Datei unter `~/Library/Application Support/KRK/`, die eine Liste von Vorlagen mit Namen und Befehlstext führt, samt Platzhaltern für den angezeigten Ordner, den Ordner der anderen Seite, die ausgewählten Einträge und den Eintrag unter dem Cursor. Die Einsetzung zitiert jeden eingesetzten Wert vollständig.

Die dritte ist der Aufruf: eine Liste der Makros als Blatt, aus der ein Eintrag gewählt und gestartet wird, dazu eine Nachfrage nach freien Argumenten für Vorlagen, die welche verlangen. Die Vorlage für die Nachfrage ist wieder das Eingabeblatt.

Die vierte ist die Tastenbindung über nummerierte Plätze, mit je einer Zeile in `Kommando::wirkungsbereich` und `bereich_des_kommandos`.

**Zwei Dinge gehören ausdrücklich nicht hinein.** Ein sechster Bereich in der Fensterzeile lohnt nicht: er zöge `Bereich` (`crates/krk-ui/src/fenstermodell.rs:103-128`), `Fokus`, `Wirkungsbereich` (`belegung.rs:186`) und die proportionale Breitenregel der fünften Runde nach sich, und die Vorschau leistet dasselbe ohne eine dieser vier Änderungen. Und das Terminal-Emulat aus Weg B bleibt draußen; für interaktive Werkzeuge bleibt `Ctrl+O`, das den angezeigten Ordner an Ghostty oder Terminal übergibt.

**Ein Hinweis zur Reihenfolge im Portfolio.** Der einzige heute vorgesehene und nicht gefahrene Circle ist der Web-Betrachter im Vorschaufenster (`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster`). Er greift dieselbe Fläche an wie dieser Vorschlag, nämlich die Tabs des Vorschaufensters. Wer beide fährt, sollte die Reihenfolge bewusst wählen; wer den Befehlslauf zuerst baut, legt die Regel fest, nach der eine dritte fremde Quelle in einen Vorschau-Tab schreibt, und der Web-Betrachter wäre dann die vierte nach derselben Regel.

## Open Questions

- [ ] Bekommen Makros eine Oberfläche zum Anlegen und Ändern, oder werden sie von Hand in einer kommentierten Datei gepflegt? Die Antwort entscheidet, ob die fünfte Ablagedatei einen Schreibweg braucht, und verschiebt den Umfang der Runde spürbar.
- [ ] Welche Shell fährt den Lauf, `/bin/sh` oder die Anmeldeshell des Nutzers mit seiner Umgebung? Ein Makro, das `fusion` oder ein anderes Werkzeug aus `~/.local/bin` ruft, findet es unter `/bin/sh -c` ohne geladenes Profil nicht.
- [ ] Bleibt eine Befehlsausgabe stehen, wenn der Nutzer im Dateifenster die Auswahl wechselt, oder überschreibt die Datei sie wie heute die Zwischenablage? Die bestehende Regel des Vorschaumodells beantwortet das mit „überschreiben", und eine Ausnahme wäre die Tab-Sorte mit eigener Regel, die der Modulkopf ausdrücklich vermeidet.
- [ ] Wie viele nummerierte Makroplätze, und welche Kombinationen tragen sie? Die Belegung führt heute 78 Kommandos, und die freien Kombinationen sind knapp.
- [ ] Trägt die Ausgabe die Textgrenze der Vorschau von 1 MB, oder eine eigene Zahl?
- [ ] Wird der Verzicht auf ein Terminal-Emulat im Circle-Datensatz als Abgrenzung geführt, zusammen mit der Nennung der Abgrenzung „KRK als Kommandozentrale für Fusion" aus der ersten Runde?

## Sources

Gelesen und geprüft am 260815 zwischen 13:20 und 13:54.

- `crates/krk-ui/src/appkit/terminal.rs:1-95` — der heutige Terminalaufruf über `NSWorkspace`, und die Begründung gegen einen Unterprozess.
- `crates/krk-ui/src/kommandos/operationen.rs:1-120` — Vermittlerfaden, Bündelung ohne Takt, Vorgangszeile, Abbruch, die 150-ms-Regel.
- `crates/krk-ui/src/appkit/anwendung.rs:4977-5005` — Start des Vermittlerfadens.
- `crates/krk-ui/src/vorschaumodell.rs:1-90, 121, 186-230, 246-281, 419-457` — Halteverhalten der Tabs, `Inhalt`, `Ladevorgang`, `zwischenablage_anzeigen`, `TEXTGRENZE`.
- `crates/krk-ui/src/angezeigtedatei.rs:1-58` — welche Datei als angezeigt gilt; ein Tab ohne Pfad fällt heraus.
- `crates/krk-core/src/ablage/einstellungen.rs:1-45` — Aufnahmeregel und Schreibverhalten von `settings.toml`.
- `resources/default-settings.toml` — die Auslieferungsfassung mit ihrer Aufnahmeregel im Klartext.
- `crates/krk-core/src/tasten/belegung.rs:186-232, 662, 752` — `Wirkungsbereich`, die Paartabelle von Kennung zu Kommando, `wirkungsbereich`.
- `crates/krk-ui/src/fenstermodell.rs:103-140` — die fünf Bereiche der Fensterzeile.
- `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs:1-35` und `zettel.rs:1-70` — Blattvorlagen für Eingabe und für eine Textfläche.
- `resources/default-keymap.toml:596-620` — C11 und die Begründung von `Ctrl+O`.
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_b_circle.md` — Directive, Abschnitt `## Ausdrücklich außerhalb dieses Circles`, Closure Note mit dem Alterungsfall `9a47c4a`.
- `Cargo.toml` — die Abhängigkeiten des Arbeitsbereichs; keine davon startet Prozesse.
- `rules/critical-stance.md`, Abschnitt 4 — der Fall der unentscheidbaren Vorhersage über Shell-Befehle.

Nicht geprüft und deshalb nicht behauptet: ob ein Filter für Farbfolgen nötig ist, ob die Erweiterung der Kommando-Aufzählung eine der zehn Zeitzusagen messbar bewegt, und wie sich `/bin/sh -c` gegenüber der Anmeldeshell in der Praxis dieses Nutzers verhält.
