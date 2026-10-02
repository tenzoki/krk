# Shaper: vorgesehener Circle für die Editor-Runde

**Status:** Complete
**Geschrieben:** 260807-2116 vom `shaper` im Modus anticipated-circle, dispatcht über `/fusion:direct`.
**Ergebnis:** `circles/260807-2116-eingebauter-editor-mit-textmarken/_a_circle.md`

---

## Der Entwurf

Der Nutzer hat die Directive-Formulierung der Runde 1 als Entwurf übergeben:

> Der eingebaute Editor öffnet Text, Code und Markdown in einer Rohansicht und einer Formatansicht, springt zu einer Zeilennummer, sucht und ersetzt innerhalb der geöffneten Datei und speichert Marken auf Textstellen und Textbereiche als Lesezeichen. Suchen und Ersetzen über mehrere Dateien gehört nicht dazu.

Alle Klärungsfragen waren beim Dispatch bereits beantwortet. Der Shaper hat keine Runde gestellt, sondern die vier Antworten samt der fünf Randbedingungen in den Datensatz eingearbeitet.

## Die vier Festlegungen des Nutzers

1. **Ort.** Der Editor ist ein vierter Fokusbereich neben Lesezeichenleiste, Dateifenstern und Vorschaufenster. Er verdrängt die Vorschau zeitlich, nimmt rund ein Drittel der Fensterbreite, und bei geöffneter Lesezeichenleiste rücken die Dateifenster zusammen, statt dass die Leiste weicht.
2. **Textmarken.** Eine Liste, eine Leiste, eine Datei. Ein Lesezeichen zeigt künftig auf einen Ordner oder auf eine Stelle in einer Datei. Angenommener Preis: die Gültigkeitsprüfung und die Auswahl in der Leiste bekommen eine Fallunterscheidung.
3. **Ablageort.** `~/Library/Application Support/KRK/`, also `bookmarks.toml` neben den drei anderen Ablagedateien. Die Entwurfsformulierung "im Home-Verzeichnis" ist überholt und in der Directive ersetzt.
4. **Restarbeit der Runde 1.** Vollständig draußen, wörtlich: "Die Messreihen interessieren mich gerade nicht, komplett auf später verlagern."

## Was der Datensatz zusätzlich trägt

**Der Preis der vierten Festlegung ist im Grounding benannt.** Eigene Zeitzusagen des Editors würden auf einem Sockel gemessen, dessen sieben Zusagen seit dem 260805 unbelegt sind. Der Datensatz führt dafür einen eigenen Abschnitt, statt die Ausklammerung unkommentiert zu lassen.

**Drei Artefakte sind als bindend zitiert, keines davon geöffnet.** Die offene Frage zur Formatansicht je Dateityp gehört vor den ersten Planschritt, nicht vor den Circle. Die Verfügbarkeitsprüfung für macOS-26-Schnittstellen ist als zu prüfen und nicht anzunehmen markiert. Der Defekt zu `cmd+y` und `shift+cmd+y` ist zitiert, weil der Editor sich in dasselbe Fokusmuster einfügt wie das Vorschaufenster.

**Sechs Bauteile der Runde 1 sind am Code geprüft und im Grounding mit Fundstelle genannt:** die F4-Reservierung in `resources/default-keymap.toml:131`, die Bereichsaufzählung in `crates/krk-ui/src/fenstermodell.rs:50`, der Fokusvorbehalt in `crates/krk-ui/src/kommandos/fokus.rs`, die Fensterzeile mit ihrer einen Breitenregel in `crates/krk-ui/src/appkit/aufteilung.rs`, die Lesezeichenliste in `crates/krk-core/src/ablage/lesezeichen.rs` und die Ablage in `crates/krk-core/src/ablage/pfade.rs:17`.

**Vier offene Fragen stehen als Eingabe für die Aktivierung im Datensatz.** Formatansicht je Dateityp, Herkunft der geöffneten Datei, Umgang mit ungespeicherten Änderungen, und woran eine Textmarke gebunden ist. Keine davon ist als neuer Entscheidungsdatensatz gefilt: die erste existiert bereits im geteilten Speicher, die übrigen drei hat der Nutzer nicht zurückgestellt, sondern sie sind bei der Klärung nicht gestellt worden und gehören in die Aktivierungsrunde.

## Was nicht geschrieben wurde

Kein Spec. Im Modus anticipated-circle ist der Circle-Datensatz das Artefakt. Kein neuer Entscheidungs- oder Defektdatensatz; der Bestand in `shared/decisions`, `shared/issues` und den beiden Circle-Speichern wurde vorher gelesen, um nichts doppelt zu filen. Kein Eingriff an einem bestehenden Circle.

## Nächster Schritt

Der Circle ist vorgesehen (`_a_`), nicht aktiv. Die Aktivierung ist ein eigener Schritt des Nutzers über `/fusion:next`.
