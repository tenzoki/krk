# Consultation: Zweitlesung der gebauten Stufe 3 (Aufgabeneditor) vor Stufe 4

**Date:** 2026-09-26 08:11
**Status:** Complete
**Requested by:** Kai Stalmann, über den Koordinator, bevor Stufe 4 auf Stufe 3 aufsetzt

## Question

Trägt der gebaute Stand der Stufe 3 des Arbeitspakets `260925-2356-f2-oeffnet-krkhome-statt-notizfenster` das, was die Plan-Zweitlesung `260926-0107-zweitlesung-plan-f2-krkhome.md` verlangt hat? Im Einzelnen fünf Fragen:

1. Sind die verlangten Änderungen umgesetzt wie gemeint, und gibt es noch einen Weg, auf dem getippter Text verloren geht?
2. Ist die gelockerte Zählprobe in `crates/krk-ui/src/appkit/ereignisse.rs` tragfähig?
3. Ist der neue Wirkungsbereich `Editortext` tragfähig?
4. Ist das Restrisiko der zerlegten Proben hinnehmbar, und was muss der Nutzer am laufenden Bündel prüfen?
5. Was muss Stufe 4 wissen oder vorher beheben?

## Context

Gelesen sind die Schritte 3.1 bis 3.5 des Plans `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md` samt Erledigungsnotizen, die eigene Plan-Zweitlesung, der Datensatz `260926-0115_*_was-tut-esc-in-einer-geaenderten-zelle-der-eintragstabellen.md` (beantwortet mit Möglichkeit 2) und die drei Commits `e6cc96b` (3.2a), `e6f5b2e` (3.2b) und `970abb0` (3.3 und 3.4). Am Baum gelesen sind `crates/krk-ui/src/appkit/eintragsansicht.rs` ganz, in `crates/krk-ui/src/appkit/editor.rs` der Flächentausch, die Zellenwege, der Umbauweg, die Rückgängig-Anmeldung und die Proben ab Zeile 6546, in `crates/krk-ui/src/appkit/anwendung.rs` die eigenen Textflächen, `abbrechen` und die Standfrage, dazu `form_passt` in `crates/krk-ui/src/kommandos/zulaessigkeit.rs` und die Tastenblöcke in `resources/default-keymap.toml`. `make check` haben wir nicht gefahren, weil zwei andere Agenten gleichzeitig am Baum bauen; die Aussage „grün“ stammt aus den Erledigungsnotizen des Plans.

## Analysis

### 1. Die verlangten Änderungen: umgesetzt, mit einem offenen Weg über `cmd+z`

**Die vier Stufe-3-Änderungen stehen im Code so, wie die Plan-Zweitlesung sie gemeint hat.** Einzeln geprüft:

- **Eine Übernahmestelle mit ihren Rufern.** `Editorbereich::zelle_uebernehmen` (`editor.rs:2199-2215`) prüft die laufende Zelle und beendet sie über `bearbeitung_beenden`, also über denselben `makeFirstResponder:` wie ein Klick daneben (`eintragsansicht.rs:666-668`). Einen zweiten Übernahmeweg gibt es nicht. Die fünf Rufer rufen sie jeweils als erste Handlung: `sichern` (`editor.rs:2617`), `datei_oeffnen` (`editor.rs:2579`), `ansicht_umschalten` (`editor.rs:3717`), `handlung_ausfuehren` (`editor.rs:2339`) und `editor_stand_befragen` (`anwendung.rs:8474`). Die letzte bedient `anlass_beginnen` (`anwendung.rs:8507`) und `beenden_erlauben` (`anwendung.rs:8850`). Die Probe `die_zellenuebernahme_hat_genau_diese_rufer` (`editor.rs:7233-7279`) hält die Rufer, deren Reihenfolge vor `.modell`, `flaeche_waehlen` und `umbau_anwenden` und den Weg über `bearbeitung_beenden`. Eine abgewiesene Übernahme hält jeden der fünf Wege an und meldet den Grund. Die Ausnahme `beenden_ohne_nachfrage` (`anwendung.rs:8847`) geht an der Übernahme vorbei. Sie gilt allein dem kaputten Tastenabgriff und nimmt den Verlust dort ausdrücklich in Kauf (`anwendung.rs:3052-3058`). Das ist hinnehmbar.
- **Erkennung im Augenblick der Frage.** `laufende_zelle` (`eintragsansicht.rs:617-625`) liest den Ersthelfer, prüft `isFieldEditor` und verlangt, dass der Delegierte unter genau dieser `Eintragstabelle` liegt (`isDescendantOf`, `eintragsansicht.rs:629-637`). Ein Merkfeld gibt es nicht. Die Probe `die_laufende_zelle_wird_vor_dem_ersten_zeichen_erkannt_und_ein_fremdes_feld_nicht` (`editor.rs:6932`) deckt den Fall vor dem ersten Zeichen ab, dazu ein fremdes Feld und die Textfläche.
- **Übergabe vor dem Ausblenden, Tausch nur bei echtem Wechsel.** `tauschschritte` (`editor.rs:1514-1528`) liefert bei gleichen Flächen nichts und sonst die Folge Einblenden, Ersthelfer, Ausblenden. `flaeche_waehlen` (`editor.rs:2401-2419`) hat vier Rufer: die zwei Ladeausgänge, `schliessen` und `ansicht_umschalten`. `stand_erneuern` (`editor.rs:3181-3186`) ruft es nicht. `makeFirstResponder:` steht in `editor.rs` allein in `flaeche_waehlen`.
- **Der Rang von `esc`.** Der Zellenrang in `abbrechen` steht unmittelbar nach dem Blatt und vor Operation und Filtertext (`anwendung.rs:6600-6611`). Er fragt ebenfalls im Augenblick des Tastendrucks. `zelle_abbrechen` (`editor.rs:2227-2234`) verwirft in der Aufgabenzelle, wie Möglichkeit 2 des Datensatzes es verlangt.

Die übrigen drei Punkte der Liste gehören nicht zu Stufe 3, sind aber erledigt. Schritt 3.2 ist in 3.2a und 3.2b geteilt. Die Heimordner-Erkennung vergleicht zwei Pfadformen als Text (`crates/krk-core/src/heimordner/mod.rs:21-54`; Datensatz `260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-…`, Möglichkeit 1). `esc` in der Notizzelle ist entschieden und wird in 4.3 gebaut.

**Ein Weg ändert den Stand noch unter einer laufenden Zelle: das Rückgängig über das Menü.** Die Prüfung verläuft so:

- `cmd+z` ist kein Befehl von KRK, sondern ein Menükürzel (`default-keymap.toml`, `text_rueckgaengig`, `gehalten_von = "menue"`). Es läuft deshalb die Antwortkette hinauf und landet beim Verwalter des Ersthelfers.
- Der Umbau jeder Tabellenhandlung meldet seine Umkehrung am Verwalter der Textfläche an (`umkehrung_anmelden`, `editor.rs:3030-3046`). Das ist der Verwalter des Fensters (Probe `textflaeche_und_tabelle_finden_denselben_verwalter`, `editor.rs:6696`).
- **Inference:** Der Feldeditor einer Zelle findet über seine Antwortkette denselben Verwalter. Er hat keinen eigenen, und `undoManagerForTextView:` bedient im Baum niemand (`grep` über `crates/krk-ui/src` findet es nur in einem Kommentar, `editor.rs:4243`).

Doppelklickt der Nutzer also eine Zeile und drückt `cmd+z`, bevor er tippt oder nachdem er sein Getipptes zurückgenommen hat, dann nimmt das Menü den letzten Umbau zurück, während die Zelle offen ist. `umkehren` (`editor.rs:3116-3136`) geht über `stand_erneuern` und `tabelle_nachziehen` bis `Eintragsansicht::zeilen_zeigen` (`eintragsansicht.rs:551-564`). Dort lädt `reloadData` die Tabelle neu, ohne nach einer laufenden Zelle zu fragen.

Was dann geschieht, hängt an AppKit und ist nicht gemessen. Zwei Ausgänge sind möglich, und beide sind schlecht:

- **Fall A: Das Feld verliert seine Zeile.** `rowForView:` antwortet dann nicht mehr, und `zelle_geendet` schreibt nichts fest (`eintragsansicht.rs:711-713`). Getippter Text fiele ohne Meldung weg. Diesen Ausgang hat das Projekt am Dateifenster schon gemessen: nach einer Auffrischung „liefert `rowForView:` dem Feld gar keine Zeile mehr“ (`tabelle.rs:3182-3195`).
- **Fall B: Das Feld behält seine Zeile.** Dann rechnet `zelle_festschreiben` (`editor.rs:2270-2276`) den Zellentext gegen den **zurückgenommenen** Stand an derselben Zeilennummer. War der zurückgenommene Umbau ein Verschieben, Hinzufügen oder Löschen, steht an dieser Nummer jetzt eine andere Aufgabe. Ihr Text würde durch den Text der alten Zeile ersetzt. Der Umbau entstünde zudem mitten in einem Rückgängig und landete auf dem Wiederherstellungsstapel (`eine_anmeldung_waehrend_eines_rueckgaengig_landet_im_wiederherstellungsstapel`, `editor.rs:5340`).

Das Dateifenster kennt genau diese Falle und hält sie mit einer Sperre: solange eine Namenszelle offen ist, liest es nicht neu, „ein `reloadData` beendete die Bearbeitung, ohne die Aktion zu schicken, und der getippte Name wäre fort“ (`tabelle.rs:1061-1073`). Die Eintragsansicht hat keine solche Sperre.

**Speculation:** Denselben Weg kann das Eintreffen eines Ladeausgangs nehmen. Zwischen `datei_oeffnen` und `ladeausgang_einziehen` (`editor.rs:2729-2743`) bleibt die alte Tabelle bedienbar. Eine in dieser Spanne begonnene Zelle endet mit dem Neuladen. Die Spanne dauert meist Millisekunden, der Fall ist daher selten.

Die Probe `die_zellenuebernahme_hat_genau_diese_rufer` fängt keinen dieser Fälle. Sie hält die Rufer der Übernahme fest, aber nicht die Gegenrichtung: dass kein Weg den Stand ändert, während eine Zelle läuft, ohne sie vorher zu beenden.

**Zwei kleinere Befunde, beide ohne Verlust:**

- `Eintrag hinzufügen` legt eine leere Aufgabe `- [ ] ` an und öffnet ihre Zelle (`editor.rs:2357-2366`). Verwirft `esc` die Zelle, bleibt die leere Aufgabe stehen, und der Stand gilt als geändert. Mit `cmd+z` lässt sie sich zurücknehmen. Mac-üblich wäre, dass eine verworfene neue Zeile verschwindet.
- `zelle_darf_enden` antwortet ja, wenn `rowForView:` keine Zeile findet (`eintragsansicht.rs:693-695`). Das ist dieselbe Lage wie in Fall A: der Text kommt dann nicht an.

### 2. Die gelockerte Zählprobe: tragfähig

**Die Ausnahme öffnet nicht, was die Probe schützt.** `CLAUDE.md` verlangt, dass der Ereignisabgriff nach der Nämlichkeit des Ersthelfers fragt und nicht nach seiner Klasse. Grund: eine Klassenfrage kann die Textfläche des Editors nicht vom Feldeditor eines Textfeldes trennen. `laufende_zelle` braucht die Klasse nur als Zugang. Erst `downcast_ref::<NSTextView>` erreicht `isFieldEditor` und `delegate`. Die Entscheidung fällt danach an einer Nämlichkeit, nämlich an `isDescendantOf` dieser einen Tabelleninstanz. Ein Feldeditor eines Blattes oder eines anderen Feldes fällt heraus, weil sein Delegierter anderswo liegt. Die Antwort geht als dritter Vergleich in `ist_eigene_textflaeche` (`anwendung.rs:3155-3172`) und damit in dieselbe Menge, die `ereignisse::ersthelfer_gehoert_appkit` befragt. Eine zweite Frage an AppKit neben dieser entsteht nicht.

Die Probe ist eng genug. Genau eine Typprüfung ist in `eintragsansicht.rs` erlaubt, und eine zweite dort macht sie rot (`e6f5b2e`, `ereignisse.rs`, Prüfmodul). Ihre letzte Zusicherung ist lockerer. Sie verlangt nur, dass `.isFieldEditor()` irgendwo in der Datei steht, und nicht im Rumpf von `laufende_zelle`. Das zu straffen ist freiwillig.

Die Tasten wirken dabei wie beabsichtigt. Der Feldeditor liegt unter der Editoransicht, also meldet `bereich_des_ersthelfers` (`anwendung.rs:6978-6990`) den Editor. Die nackten Tasten `return`, `delete` und `space` tragen Befehle der Dateifenster und wirken dort nicht. `tab` trägt `fenster_wechseln` im Bereich `Navigator` (`belegung.rs:1236-1238`) und wirkt dort ebenfalls nicht. Alle vier gehen deshalb an den Feldeditor. `esc` trägt `Wirkungsbereich::Ueberall` und erreicht den Zellenrang.

### 3. `Editortext`: tragfähig, und der Spec stützt es

**Die Sperre ist richtig und keine Überdehnung.** In der Tabellenform ist die Textfläche ausgeblendet. Ein Suchtreffer, ein Zeilensprung oder eine markierte Ersetzung landete unsichtbar darin. `form_passt` (`zulaessigkeit.rs:405-428`) entscheidet vollständig über `Wirkungsbereich` und `Editorform`, ohne Auffangzweig. `Editorform::Notizen` hält deshalb in 4.3 den Bau an und erzwingt eine bewusste Einordnung. Der Spec nennt „Suche über Einträge“ ausdrücklich als nicht vorgesehen (Spec, Zeile 393). Die gesperrte Form nimmt dem Nutzer also nichts, was zugesagt ist. Der Weg zum Suchen führt über die Rohansicht mit `ctrl+cmd+e`.

Drei Folgepflichten:

- Unter den Open Questions des Plans steht noch die alte Vorgabe „bleiben in der Tabellenform zulässig“ (Plan, Zeile 647). Sie widerspricht dem gebauten Stand und sollte als überholt markiert werden.
- `HowTo.md` (Schritt 3.5) sollte sagen, dass Suchen und Zeilensprung in der Tabelle ausgegraut sind und wie der Nutzer sie erreicht.
- In Stufe 5 muss `Editorform::Geheimnisse` in `form_passt` für `Editortext` eingeordnet werden. Zeigt die Geheimnisansicht eine Tabelle, gilt dieselbe Sperre.

### 4. Zerlegte Proben: hinnehmbar, wenn der Nutzer die Liste fährt

**Das Restrisiko ist hinnehmbar, weil es dem Projektstand entspricht und nicht wächst.** Die Zerlegung ist sauber. Die Reihenfolge des Tauschs ist rein geprüft (`editor.rs:6546-6621`), die Delegiertenwege an echten Feldern mit aufzeichnenden Wegen (`editor.rs:6991`), die Rechnung jeder Handlung am Kern (`editor.rs:7118`), und die Rufer sind am Quelltext festgehalten. Ungeprüft bleibt das Zusammenspiel mit AppKit in einem echten Fenster: wann AppKit die Delegiertenmethoden ruft, was `reloadData` innerhalb von `controlTextDidEndEditing:` tut und ob der Melder beim Tausch nur einmal auslöst. Das ist dieselbe Lücke, die der zurückgestellte Datensatz `260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md` seit der Runde 2 als Lage annimmt.

Der Preis ist, dass der Befund aus Frage 1 nur am laufenden Bündel sichtbar wird. **Diese Punkte muss der Nutzer prüfen.** Die ersten drei ergänzen die Listen im Plan:

1. **Neu, vorrangig:** In `tasks.txt` zwei Aufgaben A und B anlegen und B mit `opt+cmd+up` nach oben schieben. Dann B doppelklicken und ohne zu tippen `cmd+z` drücken. Erwartet: die Zelle schließt, die Reihenfolge ist wieder A, B, und beide Texte sind unverändert. Ein Fehler zeigt sich als „A heißt jetzt B“, als Absturz oder als eine Zelle, die offen über einer neu geladenen Tabelle steht.
2. **Neu:** Eine Zelle öffnen, drei Zeichen tippen und `cmd+z` viermal drücken. Das vierte `cmd+z` darf höchstens den vorigen Tabellenumbau zurücknehmen und nichts anderes.
3. **Neu:** Während des Tippens `cmd+w` drücken, das Fenster wieder einblenden und dann `cmd+q`. Die Datei muss danach den Text tragen, oder die Rückfrage muss kommen.
4. Aus 3.2a: der Tausch mit `ctrl+cmd+e` ohne Flackern des Fokusrahmens, `cmd+z` einer Tipp-Handlung der Rohansicht mit der Tabelle im Fokus, dazu Pfeile, Klick und eine `.md`-Datei.
5. Aus 3.2b: Doppelklick, `return`, `tab` und ein Klick daneben übernehmen, danach nimmt `cmd+z` die Übernahme zurück. `esc` stellt den Text her und lässt den Filtertext stehen. Ein eingefügter Umbruch bleibt mit Meldung in der Zelle. `cmd+s`, `ctrl+cmd+e`, `opt+cmd+e` und `cmd+q` übernehmen zuerst. Das Ankreuzfeld wirkt samt `cmd+z`, und `cmd+c` legt den Aufgabentext ab.
6. Aus 3.3 und 3.4, nach F1 und `cmd+r`: die sechs Befehle wirken und stehen im Menü, sonst sind sie ausgegraut. Die sechs Textbefehle sind in der Tabelle ausgegraut und in der Rohansicht bedienbar. `cmd+return` übernimmt, ein zweites öffnet wieder.

### 5. Was Stufe 4 wissen muss

**Vorher zu beheben ist allein der Befund aus Frage 1.** Stufe 4 verschärft ihn. In einer mehrzeiligen Notiz tippt der Nutzer mehr, nimmt mehr mit `cmd+z` zurück und erreicht den Tabellenstapel entsprechend öfter. Ein Verlust träfe dann Absätze statt einer Zeile. Das Muster „`esc` übernimmt eine geänderte Notizzelle“ läuft über denselben `reloadData` innerhalb von `controlTextDidEndEditing:`.

Die übrigen Punkte sind Bauwissen für 4.3, keine Vorbedingungen:

- **Die Zellenwege kennen keine Spalte.** `Zellenpruefung` und `Zellenende` tragen allein Zeile und Text (`eintragsansicht.rs:226-229`). `zelle_darf_enden` und `zelle_geendet` lesen allein `rowForView:`. Die Notiztabelle hat zwei Spalten mit verschiedenen Regeln: kein Umbruch im Thema, keine `## `-Zeile im Text. Die Wege brauchen deshalb die `Zelle` samt Spalte, und `columnForView:` gehört in beide Delegiertenwege. `bearbeitung_beginnen` öffnet heute fest `TEXTSPALTE` (`eintragsansicht.rs:180`, `655`).
- **Die Tabelle ist auf Aufgaben zugeschnitten.** Zeilenhöhe fest bei 24 und `setUsesAutomaticRowHeights(false)` (`eintragsansicht.rs:471-472`). Das Textfeld schneidet am Ende ab (`ByTruncatingTail`, `eintragsansicht.rs:758`). `Eintragszeile` trägt `erledigt`, und `tabelle_nachziehen` liefert für Notizen die leere Liste (`editor.rs:2478-2484`). Die Zelle je Form und die Ableitung je Form sind neu zu bauen. Den Befund aus 4.2 zur Zeilenhöhe braucht es vorher.
- **`copy:` liest den Aufgabentext** (`eintragsansicht.rs:591-600`). Für Notizen gilt nach der Plan-Vorgabe „der Notiztext ohne Thema“.
- **Die Zählprobe in `ereignisse.rs` ist ein Stolperdraht.** Wer in `eintragsansicht.rs` eine zweite Typprüfung auf `NSTextView` schreibt, etwa für `control:textView:doCommandBySelector:`, macht sie rot. Der Parameter dieser Methode kommt schon getypt an, eine zweite Prüfung ist dort nicht nötig.
- **`return` erreicht den Feldeditor.** Nackt trägt es `mit_standardprogramm_oeffnen` im Bereich der Dateifenster, und das wirkt im Editor nicht. Die Umleitung von `insertNewline:` auf `insertNewlineIgnoringFieldEditor:` im Delegierten greift deshalb wie geplant.
- **`zelle_abbrechen` bekommt den Notizzweig** und ruft darin `zelle_uebernehmen`. Die Rufer-Probe muss den sechsten Rufer aufnehmen, das sieht der Plan in 4.3 schon vor. „Geändert“ ist dabei im Augenblick der Frage zu bestimmen, als Vergleich des Feldeditortexts mit der Ableitung der Zelle. Ein Merkfeld braucht es dafür nicht.

## Recommendations

**Vor Stufe 4 zu beheben (`implementation-planner` für den Zuschnitt, dann `code-implementer`):**

1. **Kein Standwechsel von außen, solange eine Zelle läuft.** Wir empfehlen eine Lösung an der Ursache statt einer Sperre je Rufer: der Feldeditor der Eintragszellen bekommt einen eigenen Rückgängigverwalter, der mit der Zelle endet. Der Weg dafür ist ein eigener Feldeditor über `windowWillReturnFieldEditor:toObject:` am Fensterdelegierten in `appkit/fenster.rs`. Er gilt allein für Felder unter der `Eintragstabelle` und gibt über `undoManager` einen eigenen Verwalter zurück. Damit endet `cmd+z` in einer Zelle am Anfang der Zelle, wie in jedem Textfeld auf dem Mac, und kann den Tabellenstapel nicht mehr erreichen. Das nützt Stufe 4 doppelt, weil in einer Notiz mehrzeiliges Rückgängig innerhalb der Zelle erwartet wird. Vorbehaltlich einer Messung, dass AppKit den eigenen Feldeditor für die Zellen einer view-basierten Tabelle wirklich nimmt. Die Alternative in der Bauart des Dateifensters ist schwächer: `zeilen_zeigen` lädt nicht neu, solange eine Zelle läuft, und holt es nach (`tabelle.rs:1061-1073`). Sie verhindert den Verlust, lässt aber ein `cmd+z` in der Zelle den unsichtbaren Stand ändern. Eine von beiden gehört zusätzlich als Gürtel in `zeilen_zeigen`: läuft eine Zelle, wird sie vor `reloadData` verworfen und nicht übernommen, weil der Stand darunter nicht mehr der ist, gegen den sie begonnen hat. Dazu gehört eine Quelltextprobe, die für `umkehren` und `ladeausgang_einziehen` festhält, dass sie keine offene Zelle stehen lassen. Das ist die Gegenrichtung zu `die_zellenuebernahme_hat_genau_diese_rufer`.
2. **Die drei neuen Nutzerprüfungen aus Frage 4 kommen in den Plan**, unter 3.2b oder in die Nutzerarbeit der Stufe 3. Die erste davon zeigt auch ohne Behebung, ob Fall A oder Fall B eintritt, und sollte vor dem Bau von Punkt 1 gefahren werden.

**Freiwillig:**

- Die Vorgabe zu Suchen und Zeilensprung unter den Open Questions des Plans (Zeile 647) als überholt markieren und auf `Editortext` verweisen.
- `HowTo.md` (3.5) nennt die in der Tabelle ausgegrauten Textbefehle und den Weg über die Rohansicht.
- Eine verworfene, eben hinzugefügte leere Aufgabe fällt wieder weg, oder `esc` nach `Eintrag hinzufügen` übernimmt die Rücknahme gleich mit.
- Die letzte Zusicherung der Zählprobe in `ereignisse.rs` liest `isFieldEditor` im Rumpf von `laufende_zelle` statt irgendwo in der Datei.
- Die Tabelle nimmt keine neue Zellbearbeitung an, solange der Editor lädt (`laedt_noch`). Das schließt den Fall des Ladeausgangs, auch wenn Punkt 1 als bloße Sperre gebaut wird.

## Open Questions

- [ ] Tritt bei `cmd+z` in einer offenen Zelle Fall A (Text fällt still) oder Fall B (Text landet in der falschen Aufgabe) ein? Beantwortet von der ersten neuen Nutzerprüfung.
- [ ] Nimmt AppKit einen eigenen Feldeditor aus `windowWillReturnFieldEditor:toObject:` für die Textfelder einer view-basierten `NSTableView` an? Zu messen vor dem Bau von Empfehlung 1. Speculation: ja, der Weg ist nicht auf zellbasierte Tabellen beschränkt.
- [ ] Räumt AppKit die Tipp-Handlungen des gemeinsamen Feldeditors beim Ende der Zelle aus dem Fensterverwalter? Nach Empfehlung 1 erledigt sich die Frage.

## Sources

- Plan `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md` im Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`, Stufe 3 und 4, Open Questions
- Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`, Zeile 393
- Datensätze `260926-0115_*_was-tut-esc-in-einer-geaenderten-zelle-der-eintragstabellen.md`, `260926-0115_*_erkennt-krk-den-heimordner-an-zwei-pfadformen-oder-an-jeder-schreibweise.md`, `260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md`
- Eigene Plan-Zweitlesung `260926-0107-zweitlesung-plan-f2-krkhome.md`
- Commits `e6cc96b`, `e6f5b2e`, `970abb0`
- `crates/krk-ui/src/appkit/eintragsansicht.rs`, `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/tabelle.rs:1061-1073` und `3182-3195`, `crates/krk-ui/src/appkit/ereignisse.rs` (Prüfmodul), `crates/krk-ui/src/kommandos/zulaessigkeit.rs:405-428`, `crates/krk-core/src/tasten/belegung.rs:1236-1300`, `crates/krk-core/src/heimordner/mod.rs`, `resources/default-keymap.toml`

— Consultant
