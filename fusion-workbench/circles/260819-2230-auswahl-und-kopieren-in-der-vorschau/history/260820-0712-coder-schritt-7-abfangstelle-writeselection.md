# Schritt 7: Die eine Abfangstelle `writeSelectionToPasteboard:types:`

**Agent:** coder
**Datum:** 2026-08-20
**Plan:** `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel C, Schritt 7
**Status:** Complete

---

## Was der Schritt einlöst

Die Runde. Seit diesem Schritt liefert das Kopieren aus gerendertem Markdown den **Quelltext** mit seinen Auszeichnungszeichen und nicht den gerenderten Text — die Entscheidung, die der Nutzer am 260819-2210 gegen die Empfehlung des Datensatzes getroffen hat.

Alles davor lag bereit und war unerreichbar: der Quellbezug entstand im Durchgang (Schritt 1), rechnete die Klammerregel (Schritt 2), lag im Merkposten der Textanzeige (Schritte 3 und 5), und die Hülle um `NSPasteboard` nahm eine fremde Ablage entgegen (Schritt 6). Es fehlte der eine Rufer. Zwei `#[expect(dead_code, …)]` hielten die Lücke offen und sind mit ihm gefallen.

## Was gebaut ist

Drei Dateien berührt.

**`crates/krk-ui/src/appkit/vorschau.rs`** trägt die Überschreibung. `Vorschautext::auswahl_ablegen` beantwortet `writeSelectionToPasteboard:types:` mit zwei Zweigen, und der Merkposten entscheidet:

- Kein Quellbezug: unverändert an die Oberklasse über `msg_send![super(self), …]`. Die fünf übrigen Inhalte — roher Text, eingefärbter Quelltext, Metadaten, Hinweis, leerer Tab, Text aus der Zwischenablage — legen Zeichen für Zeichen ab, was markiert war (C2.1).
- Ein Quellbezug liegt bei: `selectedRange()`, dessen Werte bereits UTF-16-Einheiten und damit die Koordinaten des Quellbezugs sind, geht als `location..end()` in `Quellbezug::quelltext`, und das Ergebnis geht über `zwischenablage::text_auf_ablage_schreiben` auf die Ablage, die AppKit hereinreicht (C2.2).

Gerechnet wird dabei nichts in der Oberfläche. Die Umrechnung zwischen UTF-16-Einheiten und Bytes steht in `markdown.rs` und hat mit diesem Schritt keinen zweiten Ort bekommen.

**`crates/krk-ui/src/markdown.rs`** verliert das `#[cfg_attr(not(test), expect(dead_code, …))]` an `Quellbezug::quelltext` und den Absatz des Modulkopfs, der es ankündigte; an seine Stelle tritt die Nennung des einen Rufers. Sonst keine Zeile — die Datei war für diesen einen Eingriff freigegeben.

**`crates/krk-ui/src/appkit/menue.rs`** berichtigt die Aufzählung im Modulkopf, wen `copy:` über die Antwortkette erreicht: die Textfläche des Editors, den Feldeditor eines Textfeldes und ab jetzt die Textanzeige der Vorschau.

## Warum es eine Unterklasse braucht, und was daran Erschließung ist

Der Doc-Kommentar der Überschreibung schreibt beides aus, getrennt voneinander, weil die Trennung sonst nirgends steht.

**Erschließung** ist, dass diese eine Methode der gemeinsame Ausgang aller fünf Ausgabewege ist: `copy:`, der Eintrag des Hauptmenüs, der Eintrag des Kontextmenüs, die Dienste des Systems und das Ziehen einer Auswahl mit der Maus. So steht es in Apples Beschreibung (`NSTextView.h:258-277`), und dieselbe Signatur trägt das Dienste-Protokoll `NSServicesMenuRequestor` (`NSApplication.h:539`). **An diesem Baum ist keiner der fünf Wege gemessen**, denn dafür braucht es KRK im Vordergrund, und das ist Nutzerarbeit.

**Gemessen** ist allein, dass es bei einer Abfangstelle bleibt.

Trägt am laufenden Bündel einer der Wege nicht, gehört der Befund in `shared/decisions/260819-2216_a_gilt-die-quelltextzusage-auch-fuer-das-ziehen-einer-auswahl-und-die-dienste.md`, der für diesen Fall seine Möglichkeit 2 bereithält. Ein zweiter Entwurf steht deshalb **nicht** vorsorglich daneben.

## Die Zählprobe erwartet zwei Fundstellen und nicht eine

`die_abfangstelle_steht_im_baum_genau_einmal` liest den Baum über `crate::quellbaum` mit drei zusammengesetzten Nadeln.

Der Plan sagt „`writeSelectionToPasteboard` kommt genau einmal vor". Am Baum steht die Zeichenfolge im Programmtext **zweimal**, und beide Male zu Recht: einmal als Bezeichnung der Überschreibung im `#[unsafe(method(…))]`, einmal in der Weitergabe an die Oberklasse. Ein `msg_send!` an `super` kann den Selektor nicht anders nennen. Eine Probe mit der Erwartung „einmal" wäre von Anfang an rot gewesen — derselbe Fehlgriff, den die Schritte 3 und 5 dreimal am Baum abgewehrt haben.

Die Probe schreibt die Lage deshalb aus: genau eine Datei mit genau zwei Codezeilen, und dann die beiden Hälften einzeln, `unsafe(method(…))` genau einmal und `super(self), …` genau einmal. Das misst die Zusage von C2.12 genauer als die bloße Zahl: zugesagt ist eine **Überschreibung**, nicht ein Vorkommen einer Zeichenfolge.

Was die Nadeln nicht sehen, steht am Doc-Kommentar: ein umbrechendes Attribut, ein Abfangen unter einem anderen Selektor (`writeSelectionToPasteboard:type:` im Singular, `writablePasteboardTypes`), und ob die eine Stelle die fünf Wege wirklich trägt.

## Die Angaben zur Untergrenze

Vier Berührungen sind neu und stehen im Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`, jede am SDK gelesen:

- `writeSelectionToPasteboard:types:` in der Kategorie `NSTextView (NSPasteboard)` (`NSTextView.h:258`), die Methode an `NSTextView.h:277`. Weder Kategorie noch Methode trägt eine Verfügbarkeitsangabe, beide stehen damit seit 10.0.
- `selectedRange`, Eigenschaft von `NSText` (`NSText.h:100`), ohne Angabe, seit 10.0.
- `NSPasteboard` (`NSPasteboard.h:157`) und der Typaliasname `NSPasteboardType` (`NSPasteboard.h:23`), beide ohne Angabe, seit 10.0; dazu `NSArray` (`NSArray.h:17`), ebenso.

Die **Sortennamen** wie `NSPasteboardTypeString` tragen dagegen `API_AVAILABLE(macos(10.6))`. Diese Datei nennt keinen davon: sie reicht die Ablage an `zwischenablage::text_auf_ablage_schreiben` weiter, und die Angabe steht im Kopf jener Datei. Der Modulkopf schreibt genau das aus, damit die Grenze nicht als Lücke gelesen wird.

## Was der Schritt nicht angefasst hat

- `textView:menu:forEvent:atIndex:` (C3.2). Sobald die Fläche auswählbar ist, trägt AppKits Menü seine eigenen Einträge, und der Teilen-Eintrag steht daneben, wie er es im Editor tut. `es_gibt_genau_einen_menuebauer` in `appkit/teilen.rs` bleibt grün (C3.4).
- `crates/krk-ui/src/appkit/textautomatik.rs`. Der offene Datensatz dazu gehört Schritt 8.
- Die Belegung. Kopieren ist kein Befehl von KRK; der Menüeintrag trägt `copy:` und Ziel `nil`, und die Antwortkette entscheidet, wer ihn beantwortet.
- Die vier gewachsenen Aufzählungen und jede fremde Kiste. Kein Zuwachs.
- Die Markerwanderung der Entscheidungsdatensätze. Sie gehört dem Abschluss der Runde.

## Prüfung

`make check` (`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`), Rückgabewert 0, alle vier grün. 731 Proben in `krk-ui` durchgelaufen, darunter die neue.

**Nicht committet** — das tut der Orchestrator.
