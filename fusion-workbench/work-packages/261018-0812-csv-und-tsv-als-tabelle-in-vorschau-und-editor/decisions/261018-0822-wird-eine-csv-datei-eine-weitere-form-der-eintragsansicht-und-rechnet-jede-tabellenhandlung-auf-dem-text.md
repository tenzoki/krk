# Wird eine CSV- oder TSV-Datei eine weitere Form der Eintragsansicht, und rechnet jede Tabellenhandlung auf dem Text des Editors?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md, 260926-2308_*_duerfen-zwei-funktionen-desselben-zustellers-eine-kombination-tragen-wenn-ihre-wirkungsbereiche-einander-ausschliessen.md

---

## Question

Der Editor soll CSV und TSV als bearbeitbare Tabelle zeigen. Im Baum steht schon eine Tabelle mit Zellenbearbeitung: die Eintragsansicht (`crates/krk-ui/src/appkit/eintragsansicht.rs`) für Aufgaben, Notizen, Termine und Geheimnisse. Sie hält kein eigenes Modell, sondern leitet ihre Zeilen aus dem Stand des Editors ab. Jede Handlung rechnet der Kern als reine Funktion vom alten Text in einen neuen (`Neustand`), und `Editorbereich::umbau_anwenden` schreibt ihn mit einem Rückgängigschritt ein. Die Frage ist, ob die CSV-Tabelle in dieses Gerüst kommt oder eine eigene Tabellenmaschine bekommt. Sie bindet über dieses Paket hinaus, weil jede spätere Tabellenform (etwa Markdown-Tabellen) dieselbe Antwort erbt. Damit verbunden ist die Frage, ob die Zeilenbefehle der CSV-Tabelle die vorhandenen Eintragsbefehle sind oder eigene.

## Options

1. **Eine weitere Form derselben Ansicht, jede Handlung ein Spleiß auf dem Text, die Zeilenbefehle sind die Eintragsbefehle.** `Editorform` bekommt den Wert `Tabelle`, `Zeilen` eine Variante mit Spalten aus der Datei. Ein neues Kernmodul zerlegt den Text nach RFC 4180 in Sätze und Felder mit Byte-Bereichen. Jede Handlung ersetzt genau die berührten Bytes und liefert den neuen Text. Rückgängig, Sichern, Rohansicht, `zelle_uebernehmen`, der eigene Feldeditor und die Frage nach der laufenden Zelle bleiben unverändert. „Eintrag hinzufügen“, „Eintrag bearbeiten“, „Eintrag löschen“ und das Verschieben wirken auf Zeilen der CSV-Tabelle; dazu kommen allein die Spaltenbefehle und „Zeile darüber einfügen“.
   - Pros: keine zweite Tabelle, deren Nämlichkeit `laufende_zelle`, `feldeditor_fuer`, das Fokusziel und der Flächentausch ein zweites Mal beantworten müssten (Modulkopf der Eintragsansicht, „Eine Ansicht fuer jede Art“). Unveränderte Sätze bleiben Byte für Byte stehen, weil keine Handlung sie anfasst. `cmd+n`, `delete`, `cmd+return` und `opt+cmd+up` tragen schon die Bedeutung „Zeile“; eigene Befehle könnten diese Tasten nicht teilen, weil beide auf der Seite des Editors stünden.
   - Cons: die Ansicht ist heute fest auf vier Arten verdrahtet (`spalten_einrichten`, `zellenansicht`, `abgeleiteter_text`); sie bekommt eine fünfte Art mit Spalten aus den Daten, einen waagerechten Rollbalken und die Wiederverwendung von Zellansichten. Jede Handlung schreibt den ganzen Stand in die verborgene Textfläche (`stand_einsetzen`), bei 16 MiB ein messbarer Preis. Die Eintragsbefehle stehen im Menü „Home“, die CSV-Tabelle hat mit dem Notizordner nichts zu tun.
2. **Eine eigene Tabellenansicht mit eigenem Modell der Zellen.** Eine zweite `NSTableView`-Klasse hält die Zellen als Werte und schreibt beim Sichern die Datei neu.
   - Pros: frei von den Bindungen der Eintragsansicht; Spalten von Anfang an aus den Daten.
   - Cons: jede Frage nach dem Ersthelfer, dem Feldeditor und dem Fokus doppelt; ein zweiter Rückgängigstapel neben dem des Textes, und `cmd+z` nähme Tabelle und Rohansicht in falscher Reihenfolge zurück; byte-treues Schreiben bräuchte eine eigene Buchführung der Rohform je Zelle; eigene Zeilenbefehle könnten `cmd+n` und `delete` nicht teilen.
3. **Wie Möglichkeit 1, aber eigene Zeilenbefehle statt der Eintragsbefehle.**
   - Pros: die Namen im Menü passen genauer, und das Menü „Home“ bleibt dem Notizordner.
   - Cons: dieselben Tasten wären ein Konflikt, weil beide Wirkungsbereiche auf `Seite::Editor` stünden; die CSV-Zeilenbefehle bekämen andere Tasten als jede andere Tabelle im Editor, oder die Konfliktregel bräuchte eine dritte Art des Teilens.

## Constraints

- Kein Lesen auf dem Hauptfaden; das Einlesen bleibt auf dem Faden `krk-editor`.
- Keine neue Kiste, wenn ein eigener Leser genügt; keine Kiste mit C-Code auf den Mac-Zielen.
- Jede Fallunterscheidung über `Editorform`, `Zeilen` und `Wirkungsbereich` bleibt ohne Auffangzweig.
- Ein Rückgängigstapel je Editorfläche (Modulkopf von `appkit/editor.rs`, `rueckgaengigstapel_leeren`).

## Recommendation

Möglichkeit 1. Sie ist die eine Bauweise, die schon zweimal erweitert wurde (Termine, Geheimnisse mit sechs Spalten), ohne dass eine zweite Tabelle entstand. Der Preis des ganzen Standes je Handlung wird im Plan an einer 16-MiB-Probe gemessen; liegt er zu hoch, ist das ein Defekt an `stand_einsetzen`, der jede Form zugleich betrifft, und kein Grund für eine zweite Maschine. Die Menüfrage steht als offene Frage im Plan.

---
Answered: 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md — Empfehlung des Plans angenommen („ja“); ruled by user, Kai Stalmann <kai@stalmann.org>
