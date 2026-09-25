# Was tut der Zettel mit einer Zetteldatei, die er nicht lesen kann?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (Spec der Runde 9)
**Cross-references:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` (C5); `crates/krk-core/src/ablage/mod.rs` (`Zugang::beiseite_legen`); `crates/krk-core/src/ablage/atomar.rs` (`beiseitepfad`); `crates/krk-core/src/text/datei.rs` (`EDITORGRENZE`, die Prüfung am Deskriptor)

---

## Question

Die zwei Zetteldateien liegen offen im Ablageordner, und C5 sagt ausdrücklich zu, dass der Nutzer sie in einem beliebigen Textprogramm öffnen und lesen kann. Damit kann er sie auch verändern, und ein anderes Programm kann es ebenso. KRK trifft dann beim Öffnen des Zettels eine Datei an, die es nicht als Text annehmen kann: sie ist keine gültige UTF-8-Folge, oder sie ist so groß, dass das Lesen und das spätere Schreiben die Oberfläche anhalten, oder sie ist nicht lesbar.

**Die Frage ist scharf, weil auf sie unmittelbar eine Sicherung folgt.** Der Zettel sichert beim Tabwechsel, beim Schließen und beim Beenden, und er fragt dabei nichts. Zeigt er eine unlesbare Datei als leeren Zettel an, dann schreibt der nächste dieser drei Momente den leeren Stand über den Inhalt, den er nicht lesen konnte. Der Nutzer hat die Datei dann nicht verändert und sie trotzdem verloren.

Die sieben beantworteten Klärungsfragen decken den Fall nicht ab. Antwort 7 nimmt das Überschreiben zwischen **zwei Instanzen von KRK** in Kauf; sie sagt nichts über eine Datei, die KRK selbst nicht lesen kann.

Zur selben Antwort gehört die Frage nach der oberen Grenze. Der Editor führt eine, `EDITORGRENZE` mit 16 MB, und er prüft die Art der Datei am offenen Deskriptor und nicht am Pfad. Der Zettel hat heute keine Grenze, und ohne eine ist auch die Zusage aus dem Abschnitt zu den zehn Zeitzusagen nicht einlösbar, dass er die Oberfläche nicht merklich anhält.

## Options

1. **Leer zeigen und beim nächsten Sichern überschreiben.**
   - Pro: nichts zu bauen. Der Zettel hat keinen weiteren Zustand und keine weitere Meldung.
   - Contra: der Inhalt der Datei ist weg, ohne dass der Nutzer etwas getan hätte, das nach Löschen aussieht. Er öffnet einen Zettel, sieht ihn leer, drückt `Esc`, und die Datei ist überschrieben. Das ist der einzige Weg in diesem Programm, auf dem ein bloßer Blick Daten vernichtet.
2. **Leer zeigen, und diesen Zettel bis zum Neustart nicht sichern.**
   - Pro: nichts geht verloren. Der Bau ist klein: ein Zustand je Zettel und eine Meldung.
   - Contra: der Zettel ist für diese Sitzung tot, und zwar auf eine Art, die der Nutzer beim Tippen nicht merkt. Er schreibt hinein, schließt, und der Text ist fort. Die Meldung müsste er gelesen haben, und ein Blatt hat keine Statuszeile. Es entstünde daneben ein zweiter Zustand „gesperrt" neben „offen", den C4 heute nicht kennt.
3. **Den unlesbaren Inhalt beiseitelegen und mit einem leeren Zettel weiterarbeiten.**
   - Pro: es ist die Antwort, die dieses Projekt für dieselbe Frage schon gegeben hat. `keymap.toml` und `settings.toml` sind von Hand änderbar, und ein Tippfehler darin nimmt dem Nutzer die Datei nicht weg: `Zugang::beiseite_legen` kopiert den Text an den Beiseitepfad und tastet die ältere Fassung nicht an, wenn dort schon etwas steht. Der Zettel bekäme keinen zweiten Zustand, keine dauerhafte Sperre und keine Ausnahme im Sicherungsweg.
   - Contra: ein sechster Aufrufer von `beiseite_legen`, und der Zettel läuft damit über `Zugang` statt an ihm vorbei. Der Nutzer findet eine Datei mehr im Ablageordner und erfährt von ihr nur über die Meldung.

## Constraints

- **Die drei Sicherungsmomente fragen nichts.** Antwort 5 schließt eine Rückfrage aus, und ein Blatt über einem Blatt geht in AppKit ohnehin nicht. Eine Möglichkeit, die eine Nachfrage braucht, scheidet aus, ohne dass sie hier aufgeführt wäre.
- **Es gibt genau einen Schreibweg in den Ablageordner**, `atomar::schreiben` unter dem `Schreibgriff`. Der Datensatz vom 260812-1105 schließt einen zweiten aus. Möglichkeit 3 fügt keinen hinzu; sie benutzt den bestehenden.
- **Der Beiseitepfad besteht und ist erprobt.** `atomar::beiseitepfad` und die Reihenfolge in `beiseite_legen` sind seit Schritt 10 der Runde 1 in Gebrauch, samt der Zusage, die ältere beiseitegelegte Fassung nicht zu ersetzen.
- **Eine Grenze für die Größe ist Teil derselben Antwort.** Wird eine gesetzt, ist die naheliegende Wahl `EDITORGRENZE`, damit im Baum keine zweite Zahl für dieselbe Sache entsteht. Wird keine gesetzt, ist das zweite Kriterium im Abschnitt zu den zehn Zeitzusagen ohne Deckung.

## Recommendation

**Möglichkeit 3, mit `EDITORGRENZE` als Grenze.** Sie ist die einzige der drei, bei der der Nutzer nichts verliert und der Zettel keinen zweiten Zustand bekommt, und sie ist keine neue Erfindung, sondern die bestehende Antwort dieses Projekts auf dieselbe Frage. Der Preis, ein sechster Aufrufer von `beiseite_legen` und ein Weg über `Zugang`, fällt gegen den Preis von Möglichkeit 1 nicht ins Gewicht: dort vernichtet ein Blick auf einen Zettel eine Datei.

Möglichkeit 2 sieht sicher aus und ist es nicht. Ein Zettel, der Tippen annimmt und beim Schließen nichts behält, verliert Arbeit, die der Nutzer gerade erst geleistet hat, während Möglichkeit 1 und 3 nur mit dem umgehen, was schon auf der Platte lag.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260813-2342-orchestrator-session.md, Abschnitt "Drei Antworten am Spec-Tor" — Möglichkeit 3: den unlesbaren Inhalt über den bestehenden beiseite_legen-Weg beiseitelegen und mit einem leeren Zettel weiterarbeiten.

---
Implemented: 9362034 — `Zugang::text_laden` (`crates/krk-core/src/ablage/mod.rs:564`) legt den unlesbaren Inhalt über `beiseite_legen` beiseite und liefert einen leeren Zettel samt `Ersetzung`; `EDITORGRENZE` ist die obere Schranke. Möglichkeit 3, wie am 260814-0005 am Spec-Tor gewählt.

**Die vier Ausgänge einzeln nachgelesen** (`mod.rs:572-611`): gültiger Text, fehlende Datei (leerer Zettel ohne Meldung), nicht zu öffnen (leerer Zettel mit Meldung, nichts beiseitezulegen), und `Textstand::Unlesbar` mit `Grund::ZuGross` beziehungsweise `Grund::Beschaedigt` über `beiseite_legen`. Die Aufzählung ist vollständig ohne Auffangzweig.

**Die drei Rahmenbedingungen halten.** Keine Rückfrage entsteht. Der Schreibweg ist der eine bestehende: `nur_benannte_dateien_erreichen_das_atomare_schreiben` (`crates/krk-core/tests/baum.rs:189`) bleibt bei denselben fünf Quelldateien. `beiseite_legen` hat jetzt zwei Aufrufer, `Zugang::laden` (`mod.rs:499`) und `Zugang::text_laden` (`mod.rs:596`), und tastet eine dort liegende ältere Fassung nicht an — `eine_zweite_ungueltige_zetteldatei_laesst_die_erste_sicherung_stehen` (`crates/krk-core/tests/ablage.rs:1530`) hält es.

**Die Fehlzählung im Contra von Möglichkeit 3 bleibt hier stehen und ist im Spec berichtigt.** Der Datensatz spricht von einem „sechsten Aufrufer"; es ist der zweite. Die Berichtigung steht in C5 des Spec, und dieser Datensatz behält seinen damaligen Wortlaut als Aufzeichnung seines Standes.

Abgleich 260814-1002 (reconciler, Runde 9), `make check` exit 0 am Stand `79dab20`.
