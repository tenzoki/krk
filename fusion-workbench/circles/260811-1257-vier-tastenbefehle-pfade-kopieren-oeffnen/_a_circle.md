# Vier Tastenbefehle: Pfade kopieren, mit dem Standardprogramm öffnen, Cmd+W überall

---
**Domain:** code
**Status:** anticipated
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** (none yet)
**Active session history:** circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/history/260811-1257-shaper-vier-tastenbefehle-pfade-kopieren-oeffnen.md

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
