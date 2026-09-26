# Consultation: Zweitlesung des Plans „F2 öffnet ~/krkhome mit Notizen, Aufgaben und Geheimnissen“

**Date:** 2026-09-26 01:07
**Status:** Complete
**Requested by:** Kai Stalmann, über den Koordinator, vor der Freigabe des Plans

## Question

Trägt der Plan `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md` den überarbeiteten Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`? Im Einzelnen: hat der Spec die Befunde der ersten Zweitlesung (`260926-0017-zweitlesung-spec-f2-krkhome.md`) übernommen, deckt der Plan jedes Kriterium mit Schritt und Probe, stimmen drei Behauptungen des Planers am Baum, sind die vier riskantesten Schritte tragfähig, stoßen die Tasten irgendwo an, stimmen die Empfehlungen der zwei neuen Datensätze, und berührt die Arbeit die zehn Zeitzusagen, die Ausnahmen von `deny(unsafe_code)` oder die C-Freiheit.

## Context

Gelesen sind Spec, Plan, die zwei offenen Datensätze vom 260926-0050 und die eigene Zweitlesung vom 260926-0017. Am Baum geprüft sind die Stellen, auf die sich die Behauptungen stützen, Stand `3a81abd`. Die Kryptografie-Kisten sind gegen das Register (`cargo search`) und gegen das Wegwerfprojekt im Scratchpad nachgesehen. Die Tastenkonventionen von AppKit stammen aus `/System/Library/Frameworks/AppKit.framework/Resources/StandardKeyBinding.dict` auf diesem Gerät.

**Zwei Befunde dieser Zweitlesung berichtigen die erste.** Der Planer hat beide gefunden, und er hat in beiden recht:

- Die erste Zweitlesung schrieb, der Inhaltsfilter lese versteckte Dateien nicht (`modell.rs:985-989`). Das stimmt nur, solange die Verstecke ausgeblendet sind. Einen Auftrag bekommt jede Zeile, die unter Vorbehalt steht (`crates/krk-core/src/verzeichnis/modell.rs:1001-1013`). Das Kennzeichen `versteckt` wirkt allein im ersten Zweig und dort nur zusammen mit `verstecke_ausblenden` (`modell.rs:826`). Aus diesem Satz ist die falsche Begründung von C7.13 entstanden.
- Die erste Zweitlesung verlangte ein neues Salz bei jeder Sicherung. Das war zu weit gegriffen: ein neues Salz bedeutet eine neue Argon2-Ableitung, also eine halbe Sekunde je `cmd+s` auf dem Hauptfaden. Die Begründung dort (Parameter später anheben) braucht nur, dass jede Datei Salz und Parameter im Kopf trägt.

## Analysis

### 1. Hat der Spec die Befunde der ersten Zweitlesung übernommen?

**Ja, vollständig in der Sache.** Die fünfzehn Punkte der Frage 4 stehen alle im Spec:

| Punkt | Stelle im Spec |
|---|---|
| 1 Kennung `notizzettel` bleibt | C1.1, C1.2, Constraints |
| 2 Handgriff F1, `cmd+r` | Absatz vor Stufe 1 |
| 3 Ausführungszweige | C1.4, C6.5, C5.5, C7.15 |
| 4 Zulässigkeit, Ausgrauen | C6.6, C5.6, C7.15 |
| 5 Sitzung ohne `.secrets.txt` | C7.9 |
| 6 F4 führt zur PIN | C7.10 |
| 7 einmalige Übernahme | C3.2, C3.4 und das Nutzerkriterium dazu |
| 8 exklusives Anlegen | C2.1 |
| 9 Start legt nichts an | C2.3 |
| 10 Kosten der Ausnahme | C7.12 |
| 11 Erkennung über Verweise | C2.6, C4.1, C7.11 |
| 12 falsche PIN und Manipulation als ein Befund | C7.6 |
| 13 `#[must_use]` | Constraints |
| 14 `ALLE`-Listen | Open for Planner |
| 15 Wortlautprobe | C7.16 |

Ebenso übernommen sind der Tausch der Stufen 3 und 4, der Wegfall der Kante von Stufe 2 nach Stufe 5, die Bauform „Tabelle als Sicht auf den Text“, die vier Formatlücken, die Nullbyte-Datei, der Auslöser der Übernahme (das Anlegen des Ordners), die Verfahrenswahl mit versioniertem Binärkopf, der Abfragetext ohne Zeitangabe und der Verzicht auf eine Zusage zum Tilgen des Speichers.

**Verloren ist nichts Tragendes. Zwei kleine Folgen stehen nirgends:**

- Eine `.secrets.txt`, die auf null Bytes abgeschnitten wird, erscheint beim nächsten Öffnen als neue Datei und fragt „neue PIN festlegen“. Die erste Zweitlesung hat das als annehmbare Folge genannt; der Nutzer sollte sie in `HowTo.md` lesen können. Das ist eine Zeile in Schritt 5.7.
- Zwei Fehler hat der Spec aus der ersten Zweitlesung geerbt: die Begründung von C7.13 und das neue Salz in C7.3. Beide gehören mit der Antwort auf die zwei neuen Datensätze berichtigt (siehe Frage 6).

### 2. Plan gegen Spec

**Die Deckung ist vollständig.** Jedes der am Baum nachweisbaren Kriterien hat einen Schritt, der es schließt, und eine benannte Probe: C1.1 bis C1.4, C2.1 bis C2.6, C3.1 bis C3.6, C4.1 bis C4.3, C6.1 bis C6.7, C5.1 bis C5.7, C7.1 bis C7.19. Eine kleine Ungenauigkeit gibt es: C7.8 verlangt ausdrücklich „nicht beim Ändern der PIN“, geschlossen wird es aber in 5.4, bevor der PIN-Wechsel in 5.5 existiert. Die Probe „kein Klartext in der Datei“ in 5.5 deckt das in der Sache ab. C7.8 sollte in 5.5 mit aufgeführt werden.

**Eine Lücke ist kein fehlendes Kriterium, sondern ein fehlender Weg, und sie wiegt schwer.** Der Plan übernimmt eine laufende Zellenbearbeitung nur vor einer Tabellenhandlung (3.2: „Jede Handlung beendet eine laufende Zellenbearbeitung zuerst mit Übernahme“) und bei `EintragBearbeiten` (3.3). Nicht übernommen wird sie vor `cmd+s`, vor dem Schließen des Editors samt Rückfrage, vor dem Wechsel auf eine andere Datei, vor dem Wechsel in die Rohansicht und vor dem Beenden. Solange eine Zelle bearbeitet wird, steht der getippte Text allein im Feldeditor. `Editormodell::sichern` schreibt `self.stand` (`crates/krk-ui/src/editormodell.rs:1031`), und die Abweichungsmarke kennt nur diesen Stand. Wer tippt und dann `cmd+s` drückt, sichert den Stand ohne seine Eingabe. Wer tippt und dann schließt, bekommt keine Rückfrage, weil das Modell nichts Ungesichertes meldet. Das verletzt den Constraint „Nichts, was der Nutzer geschrieben hat, verschwindet ohne Meldung“. Dieselbe Frage stellt sich in Stufe 5 für den PIN-Wechsel und das Verschließen.

**Was der Plan über den Spec hinaus hinzufügt:**

- `Format` fällt ganz, samt `Datei::format`, dazu voraussichtlich `Grund::ZuGross` (1.3). Das ist Aufräumen, das der Spec nicht verlangt. Es ist vertretbar, weil `Format` ohne die Zettel nur noch einen Wert hätte. Es vergrößert aber den ohnehin größten Schritt.
- In Stufe 2 zeigt auch die Formatansicht des **Editors** `notes.txt` und `tasks.txt` als Markdown, weil `hervorhebung::art` den neuen Dateityp als `Darstellungsart::Markdown` einordnet (2.1). Der Spec nennt für Stufe 2 nur die Vorschau. Die Folge ist harmlos, gehört aber als Verhaltensänderung genannt.
- Die Textbefehle des Editors (Suchen, Ersetzen, Zeilensprung) bleiben in der Tabellenform zulässig, und ein Suchtreffer ist dort nicht zu sehen (Open Questions). Das ist eine Vorgabe, die der Nutzer bei der Freigabe sehen sollte, nicht versteckt.
- Die Zeile für einen unlesbaren Zettel (nicht übernommen, mit Meldung) und das Entfernen einer halb geschriebenen `notes.txt` in 1.2 sind vernünftige Ergänzungen.
- Drei Wirkungsbereiche (`Eintraege`, `Aufgaben`, `Geheimnisse`) und zwei Felder der `Lage` sind der Mechanismus hinter C5.6, C6.6 und C7.15 und keine Ausweitung.

Die Liste der Dateien in 1.3 ist nicht vollständig. Prosa über den Notizzettel steht auch in `crates/krk-core/tests/text.rs:1106-1131` und `xtask/src/veroeffentlichung.rs:1376`. Beides baut weiter, ist aber nach dem Schritt falsch.

### 3. Die drei Behauptungen des Planers

**(a) Umbauweg und ein Rückgängig-Schritt je Änderung: stimmt, mit einer Einschränkung, die der Plan kennt.** `treffer_ersetzen` nimmt die Abschrift vor der Änderung, bildet `Umkehrpunkt::zwischen` und geht über `verlauf_fuer_umbau` in `stand_erneuern` (`crates/krk-ui/src/appkit/editor.rs:2718-2731`). `stand_einsetzen` meldet genau eine Handlung am Verwalter der Textfläche an (`editor.rs:2140-2161`, `umkehrung_anmelden` in `editor.rs:2192-2208`). Dieser Verwalter gehört dem Fenster, und `undo:` beantwortet `NSWindow` über den Verwalter des Ersthelfers (Messung im Kommentar `editor.rs:3391-3398`). Eine `NSTableView` als Ersthelfer findet über die Antwortkette denselben Fensterverwalter. Ein `cmd+z` mit der Tabelle im Fokus nimmt also den Umbau zurück, und einen zweiten Stapel gibt es nicht. Die Einschränkung: sprengt ein Umbau das Budget, geht er als `Verlauf::TraegtNurDiese` hinein und leert den Stapel davor (`editor.rs:1053-1054`, `2147-2150`). Das gilt für das Ersetzen genauso und ist nach dem Spec in Ordnung („innerhalb des bestehenden Budgets“).

Ungeprüft bleibt ein Fall, den der Plan nur behauptet: Tippt der Nutzer in der Rohansicht, wechselt in die Formatansicht und drückt dort `cmd+z`, dann nimmt der Fensterverwalter eine Tipp-Handlung der **ausgeblendeten** Textfläche zurück. Nach dem Plan soll `text_zurueckschreiben` dann die Tabelle nachziehen. Das ist plausibel, weil die Rücknahme über die Textfläche und deren Delegierten läuft. Gemessen ist es nicht, und die Proben in 3.2 decken allein die Rücknahme eines Umbaus ab.

**(b) Der Inhaltsfilter fragt nicht nach `versteckt` und liest jede gelistete Datei: stimmt.** `zeilengrund_von` lässt einen Eintrag nur fallen, wenn er versteckt **und** ausgeblendet ist (`modell.rs:826`). Jede gewöhnliche Datei, die danach stehen bleibt, geht bei wirkendem Inhaltsfilter in `UnterVorbehalt(Auftragsart::Inhalt)` (`modell.rs:855-860`), und `auftraege` erteilt für jeden Vorbehalt einen Auftrag (`modell.rs:1001-1013`). Der Durchlauf über den Unterbaum fragt das Kennzeichen gar nicht; `crates/krk-core/src/verzeichnis/durchlauf.rs` kennt das Wort nicht. Alle drei Wege aus dem Datensatz bestehen.

**(c) Stufe 3 braucht die Typerkennung aus Stufe 2: stimmt, weil der Plan es so baut, und nicht aus der Sache heraus.** Die Formatansicht muss `tasks.txt` im erkannten Ordner erkennen. Der Plan legt diese Erkennung als `Dateityp::Eintraege` in 2.1 ab, wo `Dateityp::von_pfad` heute allein nach der Endung fragt (`crates/krk-ui/src/editormodell.rs:305-312`, gerufen in `editormodell.rs:805`, `vorschaumodell.rs:869`, `appkit/vorschau.rs:1556` und `1772`). Stufe 3 könnte den Wert auch selbst einführen. Die Begründung des Planers trägt trotzdem: eine zweite Erkennung für den Editor wäre die zweite Stelle, die C2.6 ausschließt, und die Stufen laufen ohnehin nacheinander. Die Abweichung vom Spec ist richtig benannt und schadet nicht.

### 4. Die riskantesten Schritte

#### 1.3 Das Notizblatt fällt, quer über beide Kisten

**Der Ansatz ist richtig. Für einen Auftrag ist der Schritt zu groß.** Die Bestandsaufnahme stimmt: `zettelproben::` wird an sieben Stellen importiert, alle in `anwendung.rs`. `Format` hat zwei Werte, `Toml` und `Text` (`crates/krk-core/src/ablage/pfade.rs:113-118`). `Grund::ZuGross` entsteht allein in `ablage/mod.rs:937` auf dem Weg von `text_laden`. Dateien mit „Zettel“ oder „zettel“ stehen im Baum dreißig.

Der Schritt lässt sich dort teilen, wo die Kistengrenze läuft, und beide Hälften bleiben grün:

- **1.3a, Oberfläche:** Befehl umbenennen, neuer Zweig `notizordner_oeffnen`, Blatt und `zettelmodell.rs` fallen, die Prüfhelfer ziehen um, `Fenstermodell::sitzung` übergibt keinen Zettel mehr. Die Kernschnittstellen `Datei::Zettel`, `Zugang::text_laden` und `Sitzung::zettel` bleiben vorerst stehen. Sie sind öffentlich, also meldet der Übersetzer sie nicht als unbenutzt.
- **1.3b, Kern:** `Datei::Zettel`, `Zettel`, die zwei `Zugang`-Wege, `Sitzung::zettel`, `Format` und `Grund::ZuGross` fallen. Dazu kommen die `baum.rs`-Anpassungen und die Proben in `tests/ablage.rs`.

So hat jeder Auftrag einen Übersetzerlauf, dessen Fehlerliste zu überblicken ist. Das Fallenlassen von `Format` und `Grund::ZuGross` sollte ausdrücklich freiwillig sein: fehlt dafür die Zeit, bleibt es stehen, ohne dass ein Kriterium fällt.

#### 3.2 Die Tabelle im Editor mit Zellenbearbeitung, Rückgängig und Anmeldung

**Die Grundform stimmt, drei Stellen sind so nicht baubar oder falsch.**

1. **Der Zeitpunkt der Anmeldung.** Der Plan merkt sich das bearbeitete Feld in `controlTextDidBeginEditing:` und vergleicht danach mit ihm. AppKit schickt diese Nachricht aber erst mit der ersten Änderung des Textes und nicht schon, wenn der Feldeditor Ersthelfer wird. Das ist bekanntes AppKit-Verhalten, in diesem Baum aber nicht gemessen. Zwischen dem Klick in die Zelle und dem ersten Buchstaben wäre die Zelle damit nicht angemeldet. `esc` fiele dann an AppKit oder auf den dritten Rang von `abbrechen` und leerte den Filtertext des Dateifensters. Genau diesen Zusammenstoß will der Plan vermeiden. **Änderung:** Die Nämlichkeit wird im Augenblick der Frage aus dem Ersthelfer selbst gelesen, ohne gemerkten Zustand. Der Ersthelfer ist ein Feldeditor (`isFieldEditor`), und sein Delegierter liegt unter der Eintragstabelle (`isDescendantOf`). Das ist weiter ein Vergleich nach Nämlichkeit und nicht nach Klasse. Es ist dieselbe Bauart wie `bereich_des_ersthelfers` (`anwendung.rs:7203-7215`) und braucht kein Vergessen in `controlTextDidEndEditing:`.
2. **Der Tausch von Textfläche und Tabelle.** `darstellung_nachziehen` blendet nach dem Plan die Textfläche aus und die Tabelle ein. Hält die Textfläche dabei den Ersthelfer, vergibt AppKit den Rang neu und löst den Melder am Hauptfenster ein zweites Mal aus. `CLAUDE.md` beschreibt genau diese Falle am Beispiel von `fokusanzeige_nachziehen`. Dazu kommt: `darstellung_nachziehen` läuft über `stand_erneuern` bei jedem Umbau und jedem `umkehren`, also auch aus dem Rückgängig-Block heraus. Ein `makeFirstResponder:` von dort kann mitten in einem anderen landen (`anwendung.rs:5787-5790` warnt davor). **Änderung:** Getauscht wird nur, wenn sich Ansicht oder Dateityp wirklich ändern, und nicht bei jedem `stand_erneuern`. Der Ersthelfer wird ausdrücklich auf die neue Fläche übergeben, **bevor** die alte ausgeblendet wird. Eine Probe hält fest, dass `umbau_anwenden` die Sichtbarkeit nicht anfasst, in der Bauart von `der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an`.
3. **Die Übernahme vor Sichern, Schließen und Wechsel.** Siehe Frage 2. **Änderung:** Die Übernahme einer laufenden Zellenbearbeitung steht an **einer** Stelle des Editorbereichs, etwa `zelle_uebernehmen() -> bool`. Jeder Weg, der den Stand liest oder aufgibt, ruft sie zuerst: `sichern`, der Weg zur Rückfrage beim Schließen und beim Beenden, `datei_oeffnen` und `ansicht_umschalten`. Wird die Übernahme abgewiesen (in Stufe 4 eine `## `-Zeile), unterbleibt der Weg mit einer Meldung. Eine Quelltextprobe hält die Rufer.

**Der Schritt ist für einen Auftrag zu groß.** Er baut eine neue `NSTableView`-Unterklasse mit Quelle und Delegierten, Ankreuzfeld, `copy:`, den Flächentausch, den Umbauweg, drei Abfragen am Editorbereich, drei Änderungen in `anwendung.rs` und Proben mit `MainThreadMarker::new_unchecked`. Er lässt sich so teilen:

- **3.2a:** `umbau_anwenden`, `Editorform`, der Flächentausch mit Übergabe des Ersthelfers und eine **nur anzeigende** Tabelle mit Zeilenauswahl. Proben: eine Rücknahme je Umbau, der Flächentausch, die `.md`-Datei.
- **3.2b:** Zellenbearbeitung, Anmeldung über den Ersthelfer, der Rang in `abbrechen`, das Ankreuzfeld, `copy:` und die eine Übernahmestelle mit ihren Rufern.

Nach 3.2a liegt eine Tabelle vor, in der sich noch nichts tippen lässt. Das ist kein auslieferbarer Stand; ausgeliefert wird aber auch erst nach 3.5.

#### 4.2 Mehrzeilige Zellen mit automatischer Zeilenhöhe

**Das ist der Schritt mit der größten technischen Ungewissheit, und der Plan behandelt ihn wie jeden anderen.** `usesAutomaticRowHeights` berechnet die Zeilenhöhe aus den Auflagen der Zellenansicht. Solange der Feldeditor tippt, wächst die Zeile nach unserer Kenntnis nicht von selbst mit. Üblich ist, bei jeder Textänderung die Höhe über `noteHeightOfRowsWithIndexesChanged:` neu anzufordern und die Umbruchbreite (`preferredMaxLayoutWidth`) des Feldes zu setzen. Das ist Inferenz und nicht gemessen. Dazu kommen zwei Fragen der Bedienung:

- **`return` gegen die Mac-Konvention.** Der Plan bildet in der Notizzelle `insertNewline:` auf einen Zeilenumbruch ab. AppKit hat dafür schon eine Taste: `opt+return` trägt in `StandardKeyBinding.dict` `insertNewlineIgnoringFieldEditor:` und schreibt in jedem Textfeld einen Umbruch, ohne dass ein Delegierter etwas umbiegt. `return` beendet die Bearbeitung. Beide Lesarten sind vertretbar. Die des Plans ist für langen Notiztext bequemer, die andere ist die Mac-Konvention und kostet keinen Code. Der Nutzer sollte das wählen, nicht der Planer.
- **`esc` verwirft mehrzeiligen Text.** In einer einzeiligen Aufgabe ist Verwerfen mit `esc` harmlos und Mac-üblich. In einer Notiz verliert der Nutzer damit womöglich Absätze, ohne Meldung. Der Verwalter des Feldeditors hilft danach nicht mehr, denn er wird mit der Bearbeitung beendet. Das berührt denselben Constraint wie in Frage 2. **Änderung:** `esc` in einer Notizzelle übernimmt, oder es verwirft nur, wenn der Text unverändert ist, und sagt es sonst in der Statuszeile. Für die Aufgabenzelle kann Verwerfen bleiben.

**Vorschlag:** 4.2 beginnt mit einer kurzen Messung an einer Wegwerf-Tabelle: Wächst die Zeile beim Tippen mit, und wohin geht der Ersthelfer beim Nachmessen der Höhe? Scheitert sie, hält die Stufe mit einer neuen Haltestelle an. Die Ausweichform wäre eine Liste der Themen mit einer Textfläche für den Notiztext daneben. Sie wiche von „Tabelle mit den Spalten Thema und Notiz“ ab und bräuchte deshalb den Nutzer.

#### 5.4 PIN-Blatt, entschlüsseltes Laden, verschlüsseltes Sichern

**Der Ansatz ist richtig.** Es gibt eine Eintrittsstelle: `editor_oeffnen_lassen` hat vier Rufer (`anwendung.rs:2380`, `8193`, `8242`, `8262`), und `Editorbereich::datei_oeffnen` hat genau einen Rufer (`anwendung.rs:8148`). Verschlüsselt wird vor dem Schreibweg. Das Modell weist ohne PIN ab. Die Ableitung läuft auf dem Ladefaden. **Drei Änderungen:**

1. **Die Sperre gehört an den Editorbereich oder das Modell, nicht an den Delegierten.** Die Herkunft eines Öffnens wird seit dem 260810 an `Editorbereich::datei_oeffnen` erzwungen und nicht mehr im Delegierten; der Kommentar über `editor_oeffnen_lassen` (`anwendung.rs:8121-8127`) erzählt genau diesen Umzug. Die Frage nach der Sonderdatei gehört an dieselbe Stelle. Der Plan hat ohnehin die zweite Sicherung im Modell („ohne PIN weist das Modell ab“), und die trägt. Die Quelltextprobe sollte deshalb das Modell halten und nicht `editor_oeffnen_lassen`.
2. **Die Sicherungsform gilt vor dem Verschlüsseln.** `datei::sichern` hängt über `sicherungsform` einen Schlussumbruch an (`crates/krk-core/src/text/datei.rs:969-992`). Der Plan umgeht `datei::sichern` für `.secrets.txt` und schreibt die Bytes selbst über `atomar::schreiben`. Damit der Rundlauf derselbe ist wie bei `notes.txt`, wird `sicherungsform` auf den Klartext angewandt, bevor verschlüsselt wird. Die Prüfung `fremd_geaendert` vor dem Schreiben (`editormodell.rs:1025`) bleibt auf diesem Weg ebenfalls stehen.
3. **Teilen.** **5.4a** baut das Modell: `Schutz`, Laden mit PIN, verschlüsseltes Sichern und alle Proben ohne Fenster, darunter die abgefangene Nachbardatei. **5.4b** baut die Oberfläche: das PIN-Blatt, den Weg über den Editorbereich, `Editorform::Geheimnisse` und das Feld `pin_aenderbar`. Der Teil, dessen Fehler Klartext auf die Platte bringt oder Inhalt verschließt, lässt sich dann für sich prüfen, bevor ein Blatt dazukommt.

Nebenbei: `Schutz::Verschluesselt` hält nach dem Plan Schlüssel **und** PIN. Die PIN braucht der Plan nur für den Vergleich in 5.5. Das ist nach dem Bedrohungsmodell unerheblich, sollte aber als Absicht dastehen.

### 5. Tasten und der Rang von `esc`

**Keine der sieben Kombinationen stößt an.** Keine steht in `resources/default-keymap.toml`, erhoben mit `grep` über `shift+cmd+return`, `cmd+return`, `opt+cmd+up`, `opt+cmd+down`, `shift+cmd+delete`, `shift+cmd+x` und `shift+cmd+p`. Die Zählung im Kopf stimmt: heute 94 Funktionen mit 97 Kombinationen (`default-keymap.toml:34`), nach 3.4 also 100 mit 103. Die Konfliktregel kennt keinen Bereich, nur den Zusteller (`default-keymap.toml:124-130`). `return`, `delete`, `space` und `opt+up`/`opt+down` (die Lesezeichen, `default-keymap.toml:622`, `627`) sind damit tatsächlich verbaut.

Gegen die Textbelegung von AppKit ist keine der sieben belegt. `StandardKeyBinding.dict` kennt `cmd+delete` (`deleteToBeginningOfLine:`), `cmd+up`/`cmd+down`, `opt+up`/`opt+down` und `opt+return`, aber keine der vorgeschlagenen. Zwei Punkte zu Konventionen außerhalb des Textsystems, beide weich:

- `shift+cmd+delete` heißt im Finder „Papierkorb entleeren“, und KRK ist ein Dateiverwalter. Die Wirkung bleibt auf die Tabelle im Editor beschränkt und ist mit `cmd+z` zurückzunehmen. Der Nachhall stört also nur das Gedächtnis der Hand. Der Kopf der Belegung hält `shift+delete` und `opt+cmd+delete` aus genau diesem Grund frei (`default-keymap.toml:64-68`). Der Blockkommentar in 3.4 sollte sagen, warum `shift+cmd+delete` trotzdem vergeben wird.
- `shift+cmd+p` heißt in vielen Mac-Programmen „Seite einrichten“. KRK druckt nicht, also ist die Kombination frei.

**Der Rang von `esc`:** „nach dem Blatt, vor der laufenden Operation“ ist die richtige Stelle. Wer in einer Zelle tippt, meint mit `esc` die Zelle. Der Rang greift aber nur, wenn die Zelle erkannt wird, und das hängt an der Änderung 1 in 3.2. Zum Verwerfen mehrzeiligen Textes siehe 4.2.

### 6. Die zwei offenen Datensätze

**Salz (`zieht-jede-sicherung-von-secrets-txt-ein-neues-salz-…`): Zustimmung zu Möglichkeit 3.** Die erste Zweitlesung hat das neue Salz je Sicherung gefordert, ohne den Preis zu nennen, und diese Forderung nehme ich zurück. Eine zufällige 24-Byte-Nonce je Sicherung unter einem Schlüssel ist genau der Betrieb, für den XChaCha20-Poly1305 gebaut ist. Salz und Parameter stehen weiter im Kopf, damit bleibt das Anheben der Parameter beim nächsten PIN-Wechsel möglich. Folgen, die in die Antwort gehören:

- C7.3 lautet dann: „Jede Sicherung zieht eine neue Nonce; das Salz entsteht beim Festlegen und beim Ändern der PIN.“ Die Probe „zwei Sicherungen desselben Klartexts ergeben verschiedene Bytes“ aus 5.1 gilt unverändert.
- Angehobene Parameter im Code greifen bei einer bestehenden Datei erst mit dem nächsten PIN-Wechsel und nicht mit dem nächsten Sichern. Das sollte in `README.md` bei der Beschreibung des Kopfes stehen.

Möglichkeit 2 kauft den Wortlaut mit einem zweiten Sicherungszustand, der die Rückfrage beim Schließen und beim Beenden berührt. Das ist genau die Stelle, an der der Plan schon ohne ihn eine Lücke hat (Frage 2).

**Inhaltsfilter (`wie-weit-reicht-der-inhaltsfilter-…`): Zustimmung zu Möglichkeit 1.** Unter dem Bedrohungsmodell ist entscheidend, dass kein Werkzeug Klartext sieht, und keiner der drei Wege liefert welchen: gelesen wird Chiffrat. Die Regel in 5.2 bewahrt den Nutzer vor einem sinnlosen Inhaltstreffer auf `.secrets.txt` in dem Ordner, in dem er sie sieht, und kostet dort nichts. Möglichkeit 2 griffe in den Leseweg des Durchlaufs ein, dessen Deskriptorregeln die Runden 10 und 11 mit Kindproben unter `ulimit -n 64` abgesichert haben, und kaufte damit nichts, was das Bedrohungsmodell verlangt. Möglichkeit 3 widerspräche der Antwort zu „immer gelistet“, die die Verallgemeinerung auf jeden Ordner abgelehnt hat. Folge: C7.13 wird auf den erkannten Ordner eingeschränkt und neu begründet, etwa so: „Im erkannten Ordner bekommt `.secrets.txt` nie einen Inhaltsauftrag; die Regel steht bei der Ausnahme ‚steht immer‘ und nicht am Kennzeichen ‚versteckt‘.“ Das Nutzerkriterium „Filter mit ‚Content‘ … `.secrets.txt` erscheint nicht als Treffer“ bleibt, wie es ist, denn es wird im Tab auf `~/krkhome` geprüft.

Ein Hinweis zur Bauform in 5.2: Die Sonderregel sollte innerhalb des Zweiges für versteckte Einträge stehen, also nur für Einträge, deren Name mit einem Punkt beginnt. Dann kostet sie in jedem anderen Ordner nicht einmal die Abfrage des `Option` je Eintrag, sondern nur je verstecktem Eintrag. Das trifft C7.12 wörtlicher als die Fassung im Plan.

### 7. Zeitzusagen, `unsafe_code`, C-Freiheit

**C-Freiheit: nicht berührt.** Beide Fassungen stehen im Register (`chacha20poly1305 = "0.11.0"`, `argon2 = "0.6.0"`, `cargo search` am 260926). Ihr Baum im Wegwerfprojekt `scratchpad/kp` zieht auf `aarch64-apple-darwin` weder `cc` noch ein Paket mit einem Namen auf `-sys`. Er holt `getrandom v0.4.3` über `rand_core v0.10.1` herein, dieselbe Fassung, die über `gix-utils` schon im Baum steht (`cargo tree --target aarch64-apple-darwin -e normal,build -i getrandom`). Eine zweite `getrandom`-Fassung entsteht damit nicht. Die Haltestelle in 5.1 prüft beide Ziele am Projektbaum selbst, und das ist richtig so.

**`unsafe_code`: nicht berührt.** Die neuen Dateien der Oberfläche liegen unter `crates/krk-ui/src/appkit/` (`eintragsansicht.rs`, `blaetter/pin.rs`), also unter der bestehenden Ausnahme in `appkit/mod.rs`. Der Kern bekommt kein `unsafe`, das sagt der Plan ausdrücklich. Das `unsafe` in den fremden Kisten fällt nicht unter die kistenlokale Regel. Die Probe `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` bleibt grün.

**Zeitzusagen: ein Befund, und er ist das Einzige, was ich an dieser Stelle ändern würde.** Der Plan setzt die Ausnahme in `Tabliste::lesen_starten` über `Heimordner::ist(&tab.ordner)`, und `ist` löst beide Pfade über `std::fs::canonicalize` auf (1.1, 5.2). `lesen_starten` läuft auf dem Hauptfaden und stellt heute keinen einzigen Dateisystemaufruf: es setzt Zustand und startet den Lesefaden (`crates/krk-ui/src/tabs.rs:1335-1365`). Nach 5.2 kämen zwei `realpath(3)` je Lesevorgang dazu, also bei jedem Ordnerwechsel und für jeden Tab, den die Sitzung beim Start wiederherstellt. Auf der eigenen Platte kostet das Mikrosekunden. Der Plan nennt das selbst Inferenz. Auf einem hängenden Netzlaufwerk blockiert `realpath` des Tab-Ordners dagegen den Hauptfaden, so lange das Laufwerk nicht antwortet. Getroffen wären L6 (Einstieg in den Unterordner), L4 (Start mit wiederhergestellten Tabs) und über die eingefrorene Ereignisschleife L1 und L9. Der ganze Leseweg des Projekts ist so gebaut, dass genau das nicht geschieht; `ohne_warten_oeffnen` ist das Gegenstück dazu im Kern.

**Änderung:** `Heimordner` löst `~/krkhome` einmal auf und hält beide Formen, die geschriebene und die aufgelöste. Neu aufgelöst wird bei jedem F2 und beim ersten Bedarf nach dem Start, und immer ist es ein Pfad im eigenen Benutzerverzeichnis. `ist(p)` vergleicht dann `p` mit beiden Formen als Pfadtext, ohne einen Aufruf am gefragten Pfad. Die Kriterien C2.6 und C7.11 („über den Verweis und über sein Ziel dieselbe Antwort“) halten damit. Eine dritte Schreibweise desselben Ordners, etwa über einen zweiten Verweis, erkennt diese Form nicht. Das ist eine Einschränkung, die in den Modulkopf gehört. Wer sie nicht will, erhebt Gerät und Inode des Ordners im Lesefaden, der ihn ohnehin öffnet, und gibt sie mit dem ersten Stapel zurück. Das ist genauer und teurer im Umbau. `Dateityp::von_pfad` und die Vorschau fragen dieselbe Form und kommen dann ebenfalls ohne Aufruf aus. Heute kosten sie einen Aufruf nur bei den drei Dateinamen, und das ist auch nach dem Plan schon tragbar.

Sonst ist nichts berührt. Das Anlegen hängt allein an F2, und die Probe in 1.3 hält, dass `bereitstellen` genau einen Rufer hat. Beim Start erscheint kein Blatt, weil die Sitzung `.secrets.txt` nicht nennt (5.3). Die Argon2-Ableitung läuft auf Arbeitsfäden. Mit Möglichkeit 3 des Salz-Datensatzes kostet `cmd+s` so viel wie bisher. Ein Abnahmelauf ist nicht geschuldet, sobald die Änderung oben eingebaut ist. Ohne sie würde ich der Aussage „kein Abnahmelauf geschuldet“ unter „Where this work stops“ nicht zustimmen.

## Recommendations

**Vor der Freigabe zu ändern (Aufgabe des `implementation-planner`):**

1. **Die Übernahme einer laufenden Zellenbearbeitung steht an einer Stelle, und jeder Weg, der den Stand liest oder aufgibt, ruft sie**: Sichern, Rückfrage beim Schließen und Beenden, Dateiwechsel, Ansichtswechsel, in Stufe 5 auch PIN-Wechsel und Verschließen. Dazu eine Quelltextprobe über die Rufer. (Frage 2, 3.2)
2. **Die Zelle wird im Augenblick der Frage angemeldet**, also Feldeditor mit einem Delegierten unter der Eintragstabelle, und nicht über ein Feld, das in `controlTextDidBeginEditing:` gemerkt wird. (3.2)
3. **Der Flächentausch übergibt den Ersthelfer vor dem Ausblenden** und läuft nur bei einem echten Wechsel von Ansicht oder Dateityp, nicht bei jedem `stand_erneuern`. (3.2)
4. **Kein `realpath` am Tab-Ordner auf dem Hauptfaden.** `Heimordner` hält die geschriebene und die aufgelöste Form und vergleicht nach Text, oder die Erkennung geschieht im Lesefaden. (Frage 7, 5.2)
5. **`esc` verwirft in einer Notizzelle keinen geänderten mehrzeiligen Text ohne Meldung.** Die Wahl `return` gegen `opt+return` für den Zeilenumbruch geht als Vorgabe an den Nutzer. (4.2)
6. **1.3, 3.2 und 5.4 werden je in zwei Aufträge geteilt**, wie oben beschrieben. (Frage 4)
7. **Die zwei Datensätze werden mit Möglichkeit 3 und Möglichkeit 1 beantwortet**, und C7.3 und C7.13 im Spec werden mit der Antwort umformuliert. (Frage 6; Aufgabe des Nutzers, danach des `requirements-designer`)

**Freiwillig:**

- 4.2 beginnt mit einer Messung an einer Wegwerf-Tabelle, ob die Zeile beim Tippen mitwächst, und erhält eine Haltestelle für den Fall, dass nicht.
- Die Sperre vor dem gewöhnlichen Textweg sitzt in `Editorbereich::datei_oeffnen` oder im Modell, und die Quelltextprobe hält sie dort und nicht in `editor_oeffnen_lassen`. (5.4)
- `sicherungsform` wird auf den Klartext angewandt, bevor verschlüsselt wird. (5.4)
- In 3.2 kommt eine Probe dazu: eine Tipp-Handlung aus der Rohansicht, zurückgenommen mit der Tabelle im Fokus, zieht die Tabelle nach.
- C7.8 steht auch in der Liste „Closes“ von 5.5.
- Das Fallenlassen von `Format` und `Grund::ZuGross` ist ausdrücklich freiwillig.
- 1.3 nimmt die Prosa in `crates/krk-core/tests/text.rs:1106-1131` und `xtask/src/veroeffentlichung.rs:1376` mit.
- Die Sonderregel in `zeilengrund_von` steht innerhalb des Zweiges für versteckte Einträge.
- Der Blockkommentar in 3.4 begründet `shift+cmd+delete` gegen den Finder.
- `HowTo.md` nennt in 5.7 die abgeschnittene `.secrets.txt` und die Wirkung angehobener Parameter erst beim PIN-Wechsel.
- 2.1 nennt, dass auch die Formatansicht des Editors die zwei Dateien als Markdown zeigt.

## Open Questions

- [ ] Beendet `return` in der Notizzelle die Bearbeitung (Mac-Konvention, Umbruch mit `opt+return`), oder schreibt es einen Umbruch (Vorschlag des Plans, Übernahme mit `cmd+return`)?
- [ ] Übernimmt `esc` in der Notizzelle, oder verwirft es mit Meldung?
- [ ] Genügt die Erkennung von `~/krkhome` über zwei Pfadformen (geschrieben und aufgelöst), oder soll jede Schreibweise des Ordners erkannt werden, dann über Gerät und Inode aus dem Lesefaden?
- [ ] Wächst eine Zeile mit `usesAutomaticRowHeights` beim Tippen im Feldeditor mit? Speculation: nicht ohne `noteHeightOfRowsWithIndexesChanged:` je Änderung; zu messen vor 4.2.

## Sources

- Plan `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`, Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`, die Datensätze `260926-0050_*_zieht-jede-sicherung-…` und `260926-0050_*_wie-weit-reicht-der-inhaltsfilter-…` im Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`
- `260926-0017-zweitlesung-spec-f2-krkhome.md`
- `crates/krk-core/src/verzeichnis/modell.rs:260-279, 820-875, 1001-1013`; `crates/krk-core/src/verzeichnis/durchlauf.rs`
- `crates/krk-ui/src/appkit/editor.rs:1053, 2038-2071, 2140-2347, 2706-2826, 2906-2932, 3340-3403`
- `crates/krk-ui/src/editormodell.rs:305-312, 778, 805, 1021-1059`; `crates/krk-core/src/text/datei.rs:969-992`
- `crates/krk-ui/src/appkit/anwendung.rs:3157-3170, 5783-5792, 6781-6857, 7203-7215, 8121-8150`
- `crates/krk-ui/src/tabs.rs:637, 689, 732, 1335-1365`; `crates/krk-ui/src/kommandos/zulaessigkeit.rs:189-214`
- `crates/krk-core/src/tasten/belegung.rs:1047-1167`; `crates/krk-core/src/ablage/pfade.rs:113-118`, `ablage/mod.rs:937`
- `resources/default-keymap.toml:34, 64-68, 124-130, 233-275, 361, 580, 592, 622-627, 934, 1241-1255`
- `/System/Library/Frameworks/AppKit.framework/Resources/StandardKeyBinding.dict` auf diesem Gerät
- `cargo search chacha20poly1305`, `cargo search argon2`; `cargo tree --target aarch64-apple-darwin -e normal,build` im Wegwerfprojekt `scratchpad/kp` und `-i getrandom` im Workspace
- `CLAUDE.md`, Abschnitte zum Ereignisabgriff, zum Melder am Hauptfenster und zur C-Freiheit
