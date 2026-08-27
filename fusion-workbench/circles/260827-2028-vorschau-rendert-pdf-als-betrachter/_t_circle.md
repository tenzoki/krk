# Die Vorschau rendert PDF als Betrachter mit Zoom, Seitensprung und Seitenzähler

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Claim:** Claimed 260828-0034: Kai Stalmann <kai@stalmann.org>, checkout 6c11b1f2.
**Active spec/plan:** (none yet)
**Active session history:** circles/260827-2028-vorschau-rendert-pdf-als-betrachter/history/260828-0035-orchestrator-session.md

---

## Directive

Wer im Dateifenster eine PDF-Datei anwählt, sieht sie im Vorschaufenster gerendert und nicht mehr als Metadatenzeilen: ein Betrachter zeigt die Seiten, lässt sich vergrößern und verkleinern, springt auf eine gewählte Seite, und die Statuszeile am Fensterfuß nennt die aktuelle Seite und die Seitenzahl. Text auf einer Seite lässt sich mit der Maus markieren und mit Cmd+C in die Zwischenablage kopieren, über dieselbe eine Hülle wie jedes andere Kopieren in KRK. Für die Größe gilt dieselbe Grenze wie für Bilder, 64 MB; eine größere PDF-Datei fällt wie ein zu großes Bild auf die Metadaten zurück, ohne gelesen worden zu sein. Die übrigen Wege der Vorschau (Text, Markdown, Bild, Metadaten, Zusammenfassung) bleiben unverändert; Bilder in JPG und PNG rendert die Vorschau bereits seit der Runde 1 und sind kein Gegenstand dieser Runde.

## Grounding snapshot

**Die Dreiteilung der Vorschau steht, und PDF fällt heute auf ihren dritten Weg.** `vorschaumodell.rs` (Modulkopf, `crates/krk-ui/src/vorschaumodell.rs:29-35`) zeigt Textdateien bis 1 MB als Text, die zehn Bildendungen aus `BILDENDUNGEN` (`ebd.:217-219`) bis `BILDGRENZE` = 64 MB (`ebd.:191`) als Bild und alles Übrige als Metadaten. `pdf` steht in keiner Endungsliste; eine PDF-Datei erreicht `laden` (`ebd.:711`) und kommt als `Inhalt::Metadaten` heraus. Die Verzweigung nach Dateiart geschieht an genau dieser Stelle über `ist_bildpfad` (`ebd.:807`); der neue Weg tritt daneben, als weiterer Wert von `Inhalt` (`ebd.:244`) und nicht als Sonderfall des Bildwegs.

**Der Nutzer hat am 260827-2028 die Runde auf PDF verengt.** Der Backlogeintrag nannte JPG und PNG mit; beide stehen seit der Runde 1 in `BILDENDUNGEN`, und das Rendern ist gebaut. Ein Defekt an der Bildanzeige ist nicht gemeldet; sollte der Nutzer einen sehen, ist er ein Defektdatensatz und kein Gegenstand dieser Runde.

**Der Bildweg ist die Vorlage für die Größengrenze und den Rückfall.** `laden` fragt vor der Grenze allein `stat(2)` und liest über der Grenze nichts (`ebd.:764`, `bis_zur_grenze_lesen`); `Inhalt::Bild` führt die Metadaten mit, damit die Ansicht bei einer nicht dekodierbaren Datei ohne zweites Lesen auf sie zurückfallen kann (`ebd.:268-284`). Der Nutzer hat für PDF dieselben 64 MB gewählt (Frage 3, Antwort a); der Rückfall über der Grenze und bei einer Datei, die kein lesbares PDF ist (beschädigt, verschlüsselt), ist derselbe wie beim Bild: Metadaten. Das ist eine Vorgabe des Shapers nach dem bestehenden Muster und keine Nutzerfrage gewesen; der Spec kann sie ausformulieren. Die Grenze gilt auf dem Dateiweg; ein PDF aus der Zwischenablage (C10) ist kein Gegenstand dieser Runde.

**Die Anzeigefläche kennt heute zwei Ansichten, und je eine ist sichtbar.** `appkit/vorschau.rs` hält einen `NSScrollView` mit `Vorschautext` und daneben einen `NSImageView` (`crates/krk-ui/src/appkit/vorschau.rs:4-13`, Feld `bild` `ebd.:538`); `anzeigen` (`ebd.:1066`) verzweigt über den `Inhalt`, und `bild_zeigen` (`ebd.:1350`) schaltet die beiden Schalter gegenläufig (`ebd.:854`). Ein Betrachter ist eine dritte Ansicht in derselben Fläche; welche Klasse ihn trägt, ist die Sache des Planers. Jede Datei unter `appkit/` trägt den Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` (`ebd.:167`), und die Untergrenze macOS 15 hält allein diese Gewohnheit (CLAUDE.md, „Technologiewahl"); eine neue Klasse muss dort eingetragen werden. Die `objc2`-Kisten stehen ohne Vorgabemerkmale in der Wurzel-`Cargo.toml` (`Cargo.toml:228-258`), jede Kiste mit Begründung; ob PDF-Rendering eine weitere Kiste braucht und ob sie C-Code hereinzöge, ist Planerfrage, die Regel „kein `cc` in `Cargo.lock`" bindet.

**Auswahl und Kopieren in der Vorschau sind seit der Runde 14 gebaut, und der PDF-Betrachter tritt in dieses Werk ein.** Der Nutzer hat gewählt, dass Text auf der Seite markierbar ist und Cmd+C ihn kopiert (Frage 4, Antwort b). Das Kopieren der Vorschau geht über `text_auf_ablage_schreiben` in `appkit/zwischenablage.rs`, der einen Hülle um `NSPasteboard` (CLAUDE.md, „Projektstand"); eine zweite entsteht nicht. Damit Cmd+C in der Fläche überhaupt bei KRK ankommt, muss die neue Fläche bei `Anwendungsdelegierter::ist_eigene_textflaeche` (`crates/krk-ui/src/appkit/anwendung.rs:2594-2606`) angemeldet werden, sofern sie ein Ersthelfer ist, der AppKit gehört; das ist die Fallunterscheidung aus CLAUDE.md („Der Ereignisabgriff fragt nach der Nämlichkeit"), und ein Bereich der Fensterzeile wird angemeldet. Die fünf Datensätze der Runde 14 sind umgesetzt (`_i_`) und liegen seit dem Archivlauf unter `archive/*/shared/decisions/260819-2216_*`; ihre Antworten gelten (kopiert wird bei Markdown der Quelltext, das Kontextmenü der Vorschau, Pfeiltasten). Für eine PDF-Seite gibt es keinen Quelltext; kopiert wird der Text der Seite, wie die Antwort b sagt.

**Zoom und Seitensprung brauchen Tasten, und die Belegung ist die eine Quelle.** `resources/default-keymap.toml` führt jedes Kommando mit Kennung und Kombination; ein neues Kommando braucht je eine Zeile in `Kommando::wirkungsbereich`, `Kommando::KENNUNGEN` und `bereich_des_kommandos` sowie einen eigenen Ausführungszweig, sonst steht es im Menü und tut nichts (CLAUDE.md, „Etliche Fallunterscheidungen"). Das Vorschaufenster ist ein eigener Fokusbereich (`Fokus::Vorschau`, `crates/krk-ui/src/kommandos/fokus.rs:80-87`), und die Befehle des Betrachters wirken sinnvoll nur dort. Welche Kombinationen Zoom größer, Zoom kleiner, Seite vor, Seite zurück und Seitensprung bekommen, ist als offener Datensatz dieser Runde abgelegt (`decisions/260827-2028_*_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md`), nach dem Vorbild der Runde 6.

**Die Statuszeile hat sechs Ränge und keinen Auffangzweig.** `Rang::ALLE` (`crates/krk-ui/src/appkit/statuszeile.rs:207-240`) zählt Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung, Filterstand und einen sechsten; ein siebter Rang hält den Bau an, bis jede Stelle nachgezogen ist (`ebd.:203`). Der Seitenzähler (Frage 2, Antwort c) ist entweder ein neuer Rang oder eine Meldung in einem bestehenden; das entscheidet der Planer nach der Rangordnung, die die Datei ausschreibt.

**Das Rendern läuft auf dem Arbeitsfaden der Vorschau, und L7 bleibt ungemessen.** `Vorschaumodell::datei_anzeigen` startet je Anfrage einen Faden (`vorschaumodell.rs:331`), und die Endbedingung von L7 hängt an `laedt_noch`. Diese Runde setzt keine elfte Zeitzusage und fasst keine der zehn an; die Frage, wie Arbeit an der Vorschau gegen L7 gemessen wird, ist offen (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`) und bindet auch diese Runde. Der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit; die Runde schließt voraussichtlich wie die meisten als `_b_`.

**Offene Defekte an der Vorschau, die der Betrachter erbt:** `shared/issues/260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md`, `shared/issues/260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md`, `shared/issues/260826-1423_*_zwei-zaehlangaben-zu-inhalt-in-vorschaumodell-rs-sind-seit-der-runde-16-um-eins-falsch.md` (ein weiterer `Inhalt`-Wert macht die Zählangaben erneut falsch; wer ihn einfügt, zieht sie nach).

## Dependencies

- `260819-2230-auswahl-und-kopieren-in-der-vorschau` — die auswählbare Vorschaufläche, ihr Kontextmenü und der Kopierweg über `text_auf_ablage_schreiben` stammen aus dieser Runde; der PDF-Betrachter setzt darauf auf.
- `260802-0842-krk-mac-dateimanager-editor-git` — die Dreiteilung der Vorschau (C6), die Bildgrenze und der Arbeitsfaden stammen aus C6 jener Runde; L1 und L7 aus C8 gelten unverändert.
- `260811-1304-statusleiste-mit-bereichsschaltern` und `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — die Statuszeile mit ihren Rängen, in die der Seitenzähler tritt.
- `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` — ihr offener Datensatz `decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` bindet auch diese Runde.

## Turn log


## Activation proposal

**Lauf:** 260827-2101-playmaker-user-fusion-next (`shared/history/260827-2101-playmaker-user-fusion-next.md`)
**Vorgeschlagene Aktivierung:** 260827-2101, über `/fusion:next` nach Bestätigung des Nutzers

Der Circle ist der einzige vorgesehene und wird als nächster empfohlen. Seine Grundlage nennt zwei offene Entscheidungsdatensätze, `decisions/260827-2028_*_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md` und `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`; keiner hält einen Planschritt auf, der erste ist im Spec zu beantworten. Alle fünf Abhängigkeiten sind terminal: `260819-2230-auswahl-und-kopieren-in-der-vorschau` kohärent, die vier übrigen beschränkt geschlossen, was in diesem Projekt den ungefahrenen Abnahmelauf des Nutzers und keine offene Arbeit bedeutet. Der Circle erbt drei offene Defekte an der Vorschau, die seine Grundlage selbst aufführt; sie sind Gegenstand seines Plans und keine Vorbedingung.
