# Vier Tastenbefehle: Pfade kopieren, mit dem Standardprogramm öffnen, Cmd+W überall

---
**Domain:** code
**Status:** bounded closure (260811-2210) — gebaut, nicht abgenommen
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md, planning/260811-1648_c_plan-vier-tastenbefehle-pfade-kopieren-oeffnen.md
**Active session history:** circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/history/260811-1454-orchestrator-session.md

---

## Directive

Nach dieser Runde legt KRK auf Tastendruck zwei Sorten von Pfaden in die Zwischenablage, den des angezeigten Ordners im aktiven Dateifenster und den des betroffenen Eintrags, gleich ob Datei oder Ordner. Eine Datei geht per Doppelklick und per Tastenkombination an das Standardprogramm des Systems, und Cmd+W schließt den aktiven Tab auch dann, wenn der Fokus nicht in einem Bereich mit Tabs steht. Die Zwischenablage ist damit zum ersten Mal auch Ziel und nicht mehr nur Quelle. Alle vier Befehle laufen über die vorhandene Kommando-Maschinerie und über keine zweite daneben: je eine Zeile in `resources/default-keymap.toml`, ein Wert in `Kommando`, je eine Zeile in `Kommando::wirkungsbereich` und in `bereich_des_kommandos`.

## Grounding snapshot

Der Bestand ist am 260811-1257 am Baum gelesen worden. Sieben Feststellungen tragen die Runde, und zwei davon widersprechen dem Entwurf, aus dem sie entstanden ist.

**Die Zwischenablage ist heute reine Quelle.** `crates/krk-ui/src/appkit/zwischenablage.rs` ist die eine Hülle um `NSPasteboard`, und ihr Modulkopf sagt in zwei Sätzen zu, dass KRK sie in keinem Fall schreibt; `setString:forType:` und `writeObjects:` kommen darin nicht vor. Die beiden Kopierbefehle brechen genau diese Zusicherung, also gehört der Modulkopf mit derselben Änderung umgeschrieben. Eine zweite Hülle daneben wäre der Fehler, den die Datei ausdrücklich vermeidet.

**"Der markierte Eintrag" hat im Baum bereits eine Regel.** `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:157`) beantwortet die Frage "worauf wirkt dieser Befehl" einmal für Kopieren, Verschieben, Papierkorb und endgültiges Löschen: die Markierung hat den Vorrang, sonst gilt der Eintrag unter der Auswahl, gezählt werden allein die sichtbaren Einträge in Sichtreihenfolge. Der Pfadkopierer erbt diese Regel oder begründet, warum nicht.

**Die Lücke bei Cmd+W hat zwei verschiedene Ursachen, nicht eine.** `Kommando::TabSchliessen` trägt `Wirkungsbereich::Tabbereich` (`crates/krk-core/src/tasten/belegung.rs`, der Zweig der vier Tabbefehle), wirkt also nur mit dem Fokus in einem Dateifenster oder in der Vorschau, und nicht in der Leiste und nicht im Editor. Die Belegungsansicht dagegen ist kein Fenster, sondern ein Blatt am Hauptfenster (`crates/krk-ui/src/appkit/belegungsansicht.rs:3`), und solange ein Blatt steht, lässt `waehrend_blatt_erlaubt` (`crates/krk-ui/src/kommandos/operationen.rs:208`) allein `Kommando::Abbrechen` durch, durchgesetzt vom Anwendungsdelegierten. Der zweite Fall ist eine bewusste Sperre und keine vergessene Zeile. Welche der beiden Lücken diese Runde schließt, ist Frage F1 in `decisions/`.

**Welchen Tab Cmd+W ohne Tabbereich-Fokus schlösse, ist bereits beantwortbar.** `Fenstermodell::aktiv()` (`crates/krk-ui/src/fenstermodell.rs:318`) nennt die aktive Fensterseite, und dieser Wert steht unabhängig davon, wo die Tastatur gerade ist.

**Für das Öffnen mit dem Standardprogramm gibt es weder einen Doppelklick noch einen Rückfallweg.** `auswahl_oeffnen` (`crates/krk-ui/src/appkit/tabelle.rs:955`) filtert auf `ist_ordner()`, eine Datei löst dort nichts aus, und in der `NSTableView` des Dateifensters steht keine Doppelklick-Behandlung. Der Befehl `oeffnen` liegt auf dem nackten Rechts-Pfeil (`resources/default-keymap.toml:213`); die Eingabetaste ist ab Werk unbelegt und vom Nutzer ausdrücklich freigegeben (Kopfkommentar derselben Datei, C2). `NSWorkspace` ist über `appkit/terminal.rs`, `appkit/volumes.rs` und `appkit/zwischenablage.rs:133` schon dreifach im Haus, es kommt also keine neue Systemabhängigkeit dazu.

**Cmd+C und Cmd+V sind nicht frei, aber auch nicht gesperrt.** Beide liegen als `text_kopieren` und `text_einfuegen` mit `gehalten_von = "menue"` in der Auslieferungsbelegung (`resources/default-keymap.toml:651` und `:658`). Der Vorgang von Cmd+A zeigt, dass eine Kombination mit zwei Zustellern kein Konflikt im Sinne von C3 ist, solange der Fokusvorbehalt die beiden nie zusammenbringt (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`).

**Die vier vollständigen Fallunterscheidungen halten den Bau an.** Nachgezählt am 260811: `resources/default-keymap.toml` führt 71 Funktionen, die Aufzählung `Kommando` 65 Varianten, `Wirkungsbereich` sieben Werte. Vier neue Funktionen heißen vier Zeilen in der Belegungsdatei, vier Werte in `Kommando` und je vier Zeilen in `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und `bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`). Keine dieser Stellen hat einen Auffangzweig, der Übersetzer nennt sie also von selbst.

Die bindende Entscheidung zu Menükürzeln ist `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`. Der Nutzer hat am 260811-1250 festgelegt, dass sie **nicht** umgekehrt wird: Cmd+W bleibt auf dem Tab, `fenster_schliessen` bleibt auf Shift+Cmd+W.

**Vier Fragen sind offen und liegen als Entscheidungsdatensätze in `decisions/` dieses Circles.** Sie betreffen die Reichweite von Cmd+W (F1), was der Pfadkopierer bei stehender Markierung nimmt (F2), was ein Doppelklick auf einen Ordner tut (F3) und welche vier Kombinationen ab Werk gelten (F4). Keine davon hindert die Aktivierung; alle vier gehören vor den Spec beantwortet.

## Dependencies

- `260802-0842-krk-mac-dateimanager-editor-git` — Runde 1, geschlossen als beschränkter Abschluss. Sie stellt die Kommando-Maschinerie, die Konflikterkennung aus C3, die Zwischenablage als Quelle aus C10 und den Wirkungsbereich, den diese Runde erweitert.
- `260809-2040-tastenbelegung-als-markdown-in-downloads` — die laufende Runde. Sie schreibt die Tastenbelegung als Markdown heraus, mit einer Spalte je Wirkungsbereich; jede der vier neuen Funktionen erscheint dort und in der Belegungsansicht, sobald sie existiert.

Außerhalb dieser Runde und ausdrücklich nicht Teil davon: die Statusleiste mit Bereichsschaltern und die proportionale Neuaufteilung. Dafür ist ein eigener Circle vorgesehen, der am 260811-1257 noch nicht angelegt war.

## Turn log

**Turn 1 — die Runde bauen** (260811-1454 bis 260811-1735). Vier Zuschnittfragen beantwortet, eine
fünfte vom `shaper` beim Schreiben des Specs gefunden und ausgewiesen statt als Zusage geführt.
Spec mit fünf Fähigkeiten und 62 Kriterien, Plan mit fünf Schritten, beide vom Nutzer abgenommen.
Alle fünf Schritte umgesetzt. Die Durchsicht lieferte sechs Befunde, sämtlich an derselben Kante —
Text, der mehr zusagt, als der Code trägt —, und alle sechs sind behoben. Zwei Tickets kamen
mitten im Turn dazu und fielen in denselben Turn: festbreite Ziffern samt Datum ohne Komma, und
das Iconset im Bündel. Commits `f9ebbdc` bis `814c8bc`.

**Turn 2 — die offenen Defekte** (260811-1735 bis 260811-2200). Fünf Defekte aus dem gemeinsamen
Speicher, keiner aus dieser Directive: das Symbol im Bündel, die festbreiten Ziffern, die
macOS-Untergrenzen in 26 Modulköpfen, die Vorschaubreite, die Konvention am `Auswahlversuch`.
Commits `8695b77` bis `b2a6c2e`. Coherence-Verdikt `bounded-closure-proposed`.

## Activation proposal

**Vorgeschlagen am:** 260811-1326
**Playmaker-Lauf:** 260811-1326-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** nach dem Abschluss der laufenden Runde 3
(`260809-2040-tastenbelegung-als-markdown-in-downloads`), nicht davor.

Dieser Circle ist der empfohlene nächste Kandidat, und er steht auf Rang 1 von drei. Die
beiden anderen vorgesehenen Circles sind
`260811-1304-statusleiste-mit-bereichsschaltern` auf Rang 2 und
`260804-0933-eingebauter-web-betrachter-im-vorschaufenster` auf Rang 3.

**Der Ausschlag ist, dass dieser Circle als einziger der drei keine unbeantwortete
technische Größe trägt.** Seine Grundlage ist am 260811-1257 am Baum erhoben, mit
Zeilenverweisen auf jede tragende Feststellung. Der Bau besteht aus vier Zeilen in
`resources/default-keymap.toml`, vier Werten in `Kommando` und je vier Zeilen in
`Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs`) und
`bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs`). Keine dieser vier
Fallunterscheidungen hat einen Auffangzweig; der Übersetzer nennt die Stellen von selbst.
Die vier Bauteile, die dieser Circle erbt, liegen auf der Platte: die eine Hülle um
`NSPasteboard` in `crates/krk-ui/src/appkit/zwischenablage.rs`, die Regel "worauf wirkt
dieser Befehl" in `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:157`), die
aktive Fensterseite in `Fenstermodell::aktiv()` (`crates/krk-ui/src/fenstermodell.rs:318`)
und `NSWorkspace`, das über drei Module schon im Haus ist.

**Die vier offenen Entscheidungsdatensätze sind Zuschnittfragen und keine Untersuchungen.**
Sie liegen in `decisions/` dieses Circles und tragen alle `_o_`: die Reichweite von Cmd+W,
was der Pfadkopierer bei stehender Markierung nimmt, was ein Doppelklick auf einen Ordner
tut, und welche vier Kombinationen ab Werk gelten. Jede führt ihre Möglichkeiten samt
Folgen. Für die Gewichtung `code` ist ein Zählwert von vier kein guter Wert, und der
Playmaker unterschlägt es nicht: der Web-Betrachter zitiert nur einen offenen Datensatz.
Der Zählwert misst hier die falsche Größe. Der eine Datensatz dort ist eine ungemessene
technische Frage zur Verfügbarkeitsprüfung für macOS-26-Schnittstellen, und derselbe Circle
hält fest, dass auch das Mittel der Darstellung von Web-Inhalt offen ist und "in eine eigene
Untersuchung vor dem Plan" gehört. Die vier Fragen hier sind in einer Klärungsrunde mit dem
Nutzer zu beantworten.

**Ein Befund dieses Circles nimmt der Runde Arbeit ab, bevor sie beginnt.** Dass Cmd+W bei
stehender Belegungsansicht nicht durchkommt, hat eine andere Ursache als die fehlende
Reichweite im Editor und in der Leiste: `waehrend_blatt_erlaubt`
(`crates/krk-ui/src/kommandos/operationen.rs:208`) lässt bei stehendem Blatt allein
`Kommando::Abbrechen` durch, und das ist eine bewusste Sperre. Die Runde schließt damit
möglicherweise nur eine der zwei Lücken; welche, ist Frage F1.

**Was gegen eine sofortige Aktivierung spricht, in absteigender Schärfe.**

Die Runde 3 ist nicht geschlossen. Ein zweiter aktiver Circle wäre die Lage
`MULTIPLE-ACTIVE`, und `.active-circle` trägt genau einen Namen. Nach dem Dateibestand am
260811-1326 tragen die Schritte S1 bis S3 ihres Plans `[DONE]`, S4 ist am 260811-1215 vom
Nutzer gestrichen; die Runde läuft damit auf einen beschränkten Abschluss zu, und der
Abschluss ist die Bedingung dieser Aktivierung.

Beide Abhängigkeiten dieses Circles sind nicht kohärent geschlossen. Nach der Rangheuristik
zählt allein `_c_` als erfüllte Vorbedingung, also trägt der Circle das Kennzeichen. Die
Bindung an die Runde 3 ist inhaltlich schwach: die Belegungsausgabe zählt, was die Belegung
führt, und vier neue Funktionen erscheinen dort ohne Zutun. Die Bindung an
`260802-0842-krk-mac-dateimanager-editor-git` (`_b_`) betrifft die Kommando-Maschinerie, die
Konflikterkennung aus C3 und den Wirkungsbereich; die Beschränkung jener Runde liegt bei
ihren Zeitzusagen, und diese vier Befehle führen keine.

Die Runde bricht eine schriftliche Zusicherung. Der Modulkopf von
`crates/krk-ui/src/appkit/zwischenablage.rs` sagt in zwei Sätzen zu, dass KRK die
Zwischenablage in keinem Fall schreibt. Die beiden Kopierbefehle brechen genau das, also
gehört der Modulkopf mit derselben Änderung umgeschrieben. Eine zweite Hülle daneben wäre
der Fehler, den die Datei ausdrücklich vermeidet.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes
von `_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim
Nutzer über `/fusion:next` oder beim Orchestrator.

## Parent grounding stale

**Festgestellt am:** 260811-1415
**Playmaker-Lauf:** 260811-1415-playmaker-direct-dispatch
**Beschränkt abgeschlossenes Kind:** `260809-2040-tastenbelegung-als-markdown-in-downloads`,
geschlossen am 260811-1415

Die Runde 3 ist geschlossen, und zwar beschränkt. Drei Stellen dieses Datensatzes stehen
seither auf einer Grundlage, die sich bewegt hat. Keine davon hält die Aktivierung auf; alle
drei gehören in die Klärungsrunde.

**Zur Auslösebedingung, offen benannt.** Die Regel verlangt eine Nennung des
abgeschlossenen Kindes im Abschnitt `## Grounding snapshot`. Hier steht der
Verzeichnisname stattdessen in `## Dependencies` (Zeile 41), und der Abschluss-Artefakt des
Kindes ist inhaltlich berührt statt namentlich zitiert. Der Vermerk steht trotzdem, weil die
Sache trägt; wer anders entscheidet, sieht an dieser Stelle, worauf.

### 1. Die Abhängigkeit nennt die Runde 3 als laufend

Zeile 41 schreibt: "`260809-2040-tastenbelegung-als-markdown-in-downloads` — die laufende
Runde." Seit dem 260811-1415 trifft das nicht mehr zu, und der Datensatz jener Runde trägt
`_b_`.

### 2. Die geerbte Zusage steht auf einer Ausgabe, die niemand hat laufen sehen

Dieselbe Zeile schließt: "jede der vier neuen Funktionen erscheint dort und in der
Belegungsansicht, sobald sie existiert." Die Ausgabe ist gebaut, ihre 41 Abnahmekriterien
stehen sämtlich auf `- [ ]`, und der Spec
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_*_spec-tastenbelegung-als-markdown-in-downloads.md`
bleibt auf `_o_`. Die `## Closure note` des Kindes sagt es unumwunden: gebaut ist die richtige
Aussage über jene Runde, abgenommen nicht. Vier neue Funktionen erscheinen in der erzeugten
Datei ohne Zutun, und ob die Datei stimmt, ist ungeprüft. Der Preis ist klein und er ist
benannt.

### 3. Der Abschluss-Artefakt des Kindes berührt eine tragende Feststellung dieser Grundlage

Der Abschnitt `## Grounding snapshot` stützt sich an einer Stelle auf einen Präzedenzfall:
"Der Vorgang von Cmd+A zeigt, dass eine Kombination mit zwei Zustellern kein Konflikt im
Sinne von C3 ist, solange der Fokusvorbehalt die beiden nie zusammenbringt." Genau dieser
Vorgang ist der Gegenstand des Abschluss-Artefakts der Runde 3. Ihr Schritt S1 hat am
Objective-C-Laufzeitsystem gemessen, dass `NSTableView` `selectAll:` aus einer eigenen
Methode beantwortet, und damit eine Annahme des Specs widerlegt, bevor sie als falsche
Zusicherung in die erzeugte Datei geriet.

`inference:` Für diesen Circle dreht der Befund den Präzedenzfall nicht um, er verschiebt
seine Art. Wer bei Cmd+A antwortet, ist gemessen und nicht mehr angenommen. Cmd+C und Cmd+V
liegen als `text_kopieren` und `text_einfuegen` mit `gehalten_von = "menue"` in der
Auslieferungsbelegung, und dieser Datensatz schließt bislang von Cmd+A auf sie. Ob dieselbe
Messung für die beiden Textbefehle dasselbe ergibt, ist nicht geprüft. Die Frage F4 (welche
vier Kombinationen ab Werk gelten,
`decisions/260811-1300_*_welche-vier-kombinationen-gelten-ab-werk.md`) ist der Ort, an dem
sie hingehört, und die Messung aus S1 ist die Vorlage dafür.

Der Playmaker ändert keine Zitate und keine Abhängigkeit. Zeile 41 bleibt, wie sie steht.

## Activation proposal

**Vorgeschlagen am:** 260811-1415
**Playmaker-Lauf:** 260811-1415-playmaker-direct-dispatch
**Domain-Gewichtung:** code
**Vorgeschlagener Aktivierungszeitpunkt:** sofort

Dieser Vorschlag ersetzt den vom 260811-1326 in einem Punkt und bestätigt ihn im Übrigen.
**Die Bedingung ist erfüllt.** Jener Vorschlag hat als schärfstes Argument gegen eine
sofortige Aktivierung genannt, dass die Runde 3 nicht geschlossen sei und ein zweiter aktiver
Circle die Lage `MULTIPLE-ACTIVE` wäre. Die Runde 3 ist am 260811-1415 als beschränkter
Abschluss geschlossen, `fusion-workbench/.active-circle` ist gelöscht, und kein Circle-Datensatz
trägt `_t_`. Der Weg ist frei.

Der Circle steht weiterhin auf Rang 1 von drei. Die Begründung des Vorschlags vom 260811-1326
gilt unverändert: seine Grundlage ist am 260811-1257 am Baum erhoben, mit Zeilenverweisen auf
jede tragende Feststellung, und er trägt als einziger der drei keine unbeantwortete technische
Größe. Der Bau besteht aus vier Zeilen in `resources/default-keymap.toml`, vier Werten in
`Kommando` und je vier Zeilen in `Kommando::wirkungsbereich` und `bereich_des_kommandos`; keine
dieser Fallunterscheidungen hat einen Auffangzweig, der Übersetzer nennt die Stellen von selbst.

**Was seit dem 260811-1326 hinzugekommen ist.** Eine offene Frage der Runde 1 bindet die
zweite Frage dieses Circles, und sie steht in keinem der beiden Abschnitte oben:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md`
fragt, ob die Markierung einen Lesevorgang übersteht. Heute fällt sie mit jedem, weil sie eine
Menge von Eintragsindizes ist, während die Auswahl über den Namen getragen wird. Die Frage F2
dieses Circles (was der Pfadkopierer bei stehender Markierung nimmt,
`decisions/260811-1258_*_was-kopiert-der-pfadkopierer-bei-stehender-markierung.md`) baut auf
derselben Markierung auf. Wer F2 beantwortet, sollte wissen, dass ihr Gegenstand heute
flüchtig ist.

**Was gegen eine sofortige Aktivierung spricht.** Beide Abhängigkeiten sind beschränkt und
nicht kohärent geschlossen; nach der Rangheuristik zählt allein `_c_` als erfüllte
Vorbedingung, also trägt der Circle das Kennzeichen. Inhaltlich ist die Bindung an die Runde 3
schwach und im Abschnitt `## Parent grounding stale` oben aufgeschlüsselt, die an die Runde 1
betrifft die Kommando-Maschinerie und den Wirkungsbereich, deren Beschränkung bei den
Zeitzusagen liegt; diese vier Befehle führen keine. Und die Runde bricht eine schriftliche
Zusicherung: der Modulkopf von `crates/krk-ui/src/appkit/zwischenablage.rs` sagt in zwei Sätzen
zu, dass KRK die Zwischenablage in keinem Fall schreibt. Die beiden Kopierbefehle brechen genau
das, also gehört der Modulkopf mit derselben Änderung umgeschrieben.

Der Playmaker benennt Kandidaten, er aktiviert sie nicht. Die Umbenennung des Datensatzes von
`_a_circle.md` auf `_t_circle.md` und das Schreiben von `.active-circle` bleiben beim Nutzer
über `/fusion:next` oder beim Orchestrator.

## Closure note

**Geschlossen am 260811-2210 als beschränkter Abschluss (`_b_`).** Wie die Runden 1, 2 und 3, und
aus demselben Grund: **die Runde ist gebaut, nicht abgenommen.** Alle 62 Abnahmekriterien des
Specs stehen auf `- [ ]`, weil der Abnahmelauf KRK im Vordergrund verlangt — aus dem Hintergrund
weist die Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab. Kein Agent kann ihn fahren.
Der Abgleich hat die 62 sortiert: **23 trägt der Baum heute schon** (Zahlen, Verbotsseite,
Programmtext, alle 23 halten), **32 kann nur ein Mensch am laufenden Bündel sehen**, **7 brauchen
einen Prüfaufbau**.

**Die Directive ist im Code erfüllt.** Alle fünf Planschritte tragen `[DONE]`, jeder vom
`reconciler` einzeln am Baum nachgelesen statt aus dem Plan übernommen; `make check` grün mit 795
Proben und null Warnungen unter `-D warnings`. Die vier Befehle laufen über die vorhandene
Kommando-Maschinerie und über keine zweite daneben: 74 Funktionen in der Belegungsdatei, 82
Kombinationen, 68 `Kommando`-Varianten, sieben Wirkungsbereiche, null Auffangzweige. Die
Zwischenablage ist zum ersten Mal auch Ziel, und der Modulkopf, der das Gegenteil zusagte, ist mit
derselben Änderung umgeschrieben.

**Die offene Frage des Plans ist beantwortet, nicht vertagt.** Ob `NSTableView` `target` schwach
führt, steht im `SAFETY`-Block bei `tabelle.rs` mit Zeilenbeleg aus der objc2-Bindung **und** aus
dem SDK-Kopf. Ein schwach haltendes Zwischenobjekt braucht der Weg nicht.

**Was der Abgleich gefunden hat.** Ein einziger Fund in zwölf geprüften Behebungen: der Datensatz
`issues/260811-1648_c_…` beschreibt seine Behebung als zwei Handgriffe, gelaufen war nur der
erste. Nachgezogen. Daneben fünf veraltete Stellen in `CLAUDE.md`, ungeändert gemeldet — der Datei
fehlen zwei ganze Runden.

**Ein Kriterium verspricht mehr, als der Mechanismus beantworten kann.** C5 verlangt
Konfliktfreiheit „auch gegen ein Menükürzel", aber `Belegung::konflikte` vergleicht nur innerhalb
desselben `gehalten_von` — so entschieden am 260805. Das ist keine Lücke dieser Runde, aber wer C5
abnimmt, sollte es wissen.

**Für die Nachfolger.** Der Defekt der Vorschaubreite ist in dieser Runde behoben (`1ea5a3d`);
damit ist die harte Vorbedingung des vorgesehenen Circles
`260811-1304-statusleiste-mit-bereichsschaltern` weg. Der zugehörige Datensatz jenes Circles steht
auf beantwortet; ob umgesetzt oder überholt, entscheidet sich bei seiner Aktivierung. Ein Befund
nebenbei gehört demselben Circle: `MINDESTGROESSE` deckt mit 780 Punkten die vier Bereiche der
Runde 1, der Editor braucht 320 statt 160, und zwischen 780 und 920 Punkten Fensterbreite wird er
unter sein Mindestmaß gedrückt.

**Belege.** Abgleich `history/260811-2157-reconciliation.md`, Sitzungshistorie
`history/260811-1454-orchestrator-session.md` mit dem Coherence-Verdikt, dem Turn-Protokoll und dem
Sitzungsverlauf. Die Abnahmeanleitung für die Markdown-Ausgabe der Tastenbelegung liegt beim
Circle der Runde 3.
