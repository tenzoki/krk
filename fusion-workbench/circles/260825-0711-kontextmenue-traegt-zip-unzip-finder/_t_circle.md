# Das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md
**Active session history:** shared/history/260824-2120-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

**Das Kontextmenü ist gebaut und hat genau eine Anschlussstelle.** Es entsteht leer in `crates/krk-ui/src/appkit/tabelle.rs` (um Zeile 4420, `setMenu:` mit dem Delegierten der Dateifensterquelle) und wird bei jedem Rechtsklick über `menuNeedsUpdate:` neu befüllt, weil die betroffenen Einträge sich zwischen zwei Klicks ändern. Seinen heutigen einzigen Eintrag liefert `eintrag_anfuegen` in `crates/krk-ui/src/appkit/teilen.rs`, dem einen Menübauer für drei Flächen. Zwei Zählproben in jener Datei halten den Bau an, sobald ein zweiter Bauer oder ein zweiter `NSSharingServicePicker` danebentritt; drei neue Einträge treten also neben den Freigabeeintrag und nicht an eine zweite Baustelle.

**Worauf ein Befehl wirkt, ist eine bestehende Regel und bleibt es.** `kommandos::operationen::betroffene` (`crates/krk-ui/src/kommandos/operationen.rs:167`) nimmt die markierten Einträge und ersatzweise die ausgewählte Zeile. Der Nutzer hat in Runde 1 ausdrücklich diese Regel gewählt und den Entwurfsvorschlag verworfen, der bei leerer Markierung den ganzen angezeigten Ordner gepackt hätte. Vor `betroffene` setzt der Rechtsklick die Auswahl, geregelt in `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`. Die Anschlussfrage daneben ist offen und bindet diese Runde, weil drei weitere Einträge künftig auf derselben Auswahl aufsetzen: `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1516_*_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md`.

**Die Vorgangsanzeige mit Fortschritt und Abbruch steht, und Zip fügt sich als weitere Art ein.** `crates/krk-core/src/operation/` trägt `Auftrag` mit der Aufzählung `Art` (Kopieren, Verschieben, InDenPapierkorb, UmbenennenImStapel), meldet Fortschritt über einen Kanal und bricht über ein `AtomicBool` ab; kein Auftrag läuft auf dem Hauptfaden, und daran hängt die Zeitzusage L9. Die Aufzählung hat keinen Auffangzweig, der Übersetzer nennt also beim Erweitern die nachzuziehenden Stellen. Ein Archivvorgang ist damit derselbe Bauplan wie ein Kopiervorgang und keine zweite Maschine daneben.

**Das Konfliktblatt gibt vier Antworten, die Antwort aus Runde 2 nennt drei.** `crates/krk-ui/src/appkit/blaetter/konflikt.rs` bietet Überschreiben, Überspringen, Umbenennen und Abbrechen, dazu die Wahl „für alle weiteren übernehmen“; die Eingabetaste liegt bewusst auf Überspringen und nicht auf der ersten Schaltfläche. Ein Zip erzeugt genau eine Zieldatei, weshalb „für alle weiteren“ ohne Gegenstand ist und Überspringen mit Abbrechen zusammenfällt. Die Frage, welche Antworten in dieser Lage stehen bleiben, ist als offener Entscheidungsdatensatz dieser Runde abgelegt.

**Für Finder gibt es die Vorlage im Baum.** `crates/krk-ui/src/appkit/terminal.rs` löst eine Bündelkennung über `NSWorkspace::URLForApplicationWithBundleIdentifier:` auf und öffnet damit den angezeigten Ordner; der Kopf begründet ausführlich, warum nicht über `open -a` und warum der Rückrufparameter leer bleibt. Die Hausregel ist ein Modul je Frage: `teilen.rs`, `terminal.rs`, `standardprogramm.rs`, `papierkorb.rs` und `volumes.rs` stehen aus diesem Grund nebeneinander, und jede Datei unter `appkit/` trägt im Modulkopf den Abschnitt zu den Systemuntergrenzen.

**Zip und Unzip haben im Baum keinen Vorläufer, und der Weg dorthin ist offen.** Weder `krk-core` noch `krk-ui` führt eine Archivkiste, und `terminal.rs` hält ausdrücklich fest, dass dieses Vorhaben bis heute keinen Unterprozess startet. Welchen Weg der Bau nimmt, fremde Kiste oder Systemwerkzeug, entscheidet der Plan; fällt er auf ein Systemwerkzeug, bindet ihn die offene Frage `shared/decisions/260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`. Jede fremde Kiste dieses Projekts trägt ihre Begründung in der Wurzel-`Cargo.toml`, und auf dem Bauziel baut heute keine davon C-Code.

**Ohne Tastenkombination und ohne Hauptmenüeintrag entsteht keine neue `Kommando`-Variante.** Die drei Befehle wirken allein aus dem Kontextmenü, hängen also weder an `Kommando::wirkungsbereich` noch an `bereich_des_kommandos`, und `shared/decisions/260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md` bleibt unberührt. Die Kehrseite ist die bekannte Falle: der Ausführungszweig in `Anwendungsdelegierter::kommando_ausfuehren` endet auf einen Auffangzweig, den der Übersetzer nicht hält, und ein Menüeintrag ohne Wirkung fiele keiner Probe auf.

**Meldungen laufen über die Statuszeile.** Der Nutzer hat am 260804-0830 Möglichkeit 1 aus `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` gewählt: laufende Fehler trägt `appkit/statuszeile.rs`, und `appkit/hinweis.rs` bleibt dem einen abbrechenden Fehler vorbehalten. „Unzip findet kein Archiv“ ist damit eine Zeile in der Statuszeile und kein Blatt.

**Diese Runde setzt keine elfte Zeitzusage und fasst keine der zehn an.** Der letzte vollständige Abnahmelauf ist `messungen/260810-1918-alle-zusagen.txt`; die Zusage L7 steht seit dem 260819-2242 auf den Gegenständen der späteren Messrunde (`shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`).

## Dependencies

- `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — das Kontextmenü der Dateiliste und der eine Menübauer `eintrag_anfuegen` stammen aus dieser Runde; die drei neuen Einträge treten daneben. Deren offener Datensatz `decisions/260812-1516_*_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md` bindet auch diese Runde.
- `260802-0842-krk-mac-dateimanager-editor-git` — Vorgangsanzeige, Abbruch, Konfliktblatt und die Fehleranzeige in der Statuszeile stammen aus C4 jener Runde und werden hier wiederverwendet.

## Turn log
