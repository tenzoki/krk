# Shaper: Kontextmenü trägt Zip, Unzip und Finder

**Datum:** 2026-08-25
**Agent:** shaper (anticipated-circle mode, dispatch über `/fusion:direct`)
**Status:** Complete

## Der Entwurf

Der Nutzer hat als Entwurf gegeben: das Kontextmenü der rechten Maustaste trägt heute genau einen Eintrag, das Teilen über die Freigabedienste des Systems. Drei weitere sollen dazukommen. Zip packt den markierten Ordner beziehungsweise die ausgewählten Dateien, und wenn nichts markiert ist den gesamten angezeigten Ordner, in ein Zip benannt nach Ordner oder Datei. Unzip entpackt ein Zip, nämlich die markierte Datei, wenn sie ein Zip ist, andernfalls das eine Zip im angezeigten Ordner, wenn dort genau eines liegt. Finder öffnet den Finder im angezeigten Ordner. Als Domäne war `code` angegeben.

## Was geklärt wurde

Zwei Runden mit je drei Fragen, alle sechs vom Nutzer beantwortet.

Runde 1 hat den Wirkungsbereich, den Ablageort und die Erreichbarkeit festgelegt. Zip folgt der bestehenden Regel aus `kommandos::operationen::betroffene`, also markierte Einträge und ersatzweise die ausgewählte Zeile; der Entwurfsvorschlag, bei leerer Markierung den ganzen angezeigten Ordner zu packen, ist damit verworfen. Das Archiv entsteht immer im angezeigten Ordner. Die drei Befehle bleiben allein im Kontextmenü und bekommen weder eine Tastenkombination noch einen Hauptmenüeintrag, womit keine neue `Kommando`-Variante entsteht.

Runde 2 hat die drei Folgefragen geschlossen. Ein Namenskonflikt beim Zip führt zu derselben Rückfrage wie beim Kopieren. Unzip legt seinen Inhalt immer in einen neuen Ordner im angezeigten Ordner, benannt nach dem Archiv. Sind mehrere Einträge markiert, trägt das Archiv den Namen des angezeigten Ordners.

Zwei Vorgaben des Shapers standen in beiden Runden unwidersprochen und sind in die Directive eingegangen: Zip und Unzip laufen über die bestehende Vorgangsanzeige mit Fortschritt und Abbruch, und wo ein Befehl nichts vorfindet, meldet er es in der Statuszeile statt in einem Blatt.

## Erkundung des Baums

Das Kontextmenü der Dateiliste entsteht leer in `crates/krk-ui/src/appkit/tabelle.rs` und wird bei jedem Rechtsklick über `menuNeedsUpdate:` neu befüllt; seinen einen Eintrag liefert `eintrag_anfuegen` in `crates/krk-ui/src/appkit/teilen.rs`, dem einen Menübauer für drei Flächen. Die Vorgangsmaschine unter `crates/krk-core/src/operation/` trägt `Auftrag` mit vier Arten ohne Auffangzweig, meldet Fortschritt über einen Kanal und bricht über ein `AtomicBool` ab. Das Konfliktblatt `crates/krk-ui/src/appkit/blaetter/konflikt.rs` bietet vier Antworten und ein Ankreuzfeld, also mehr als die Antwort aus Runde 2 nennt. Für Finder liegt die Vorlage in `crates/krk-ui/src/appkit/terminal.rs`, das eine Bündelkennung über `NSWorkspace` auflöst. Für Zip und Unzip gibt es im Baum keinen Vorläufer, und `terminal.rs` hält fest, dass dieses Vorhaben bis heute keinen Unterprozess startet.

## Erzeugter Circle

`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/`, Datensatz `_a_circle.md`, Marker `_a_` (vorgesehen). Die sechs Artefaktverzeichnisse stehen. Als Abhängigkeiten sind die Runde 6 (Kontextmenü und Menübauer) und die Runde 1 (Vorgangsanzeige, Konfliktblatt, Statuszeile) eingetragen.

## Offene Entscheidungsdatensätze dieser Runde

Vier Fragen sind bei der Erkundung aufgetaucht und als `_o_` in `decisions/` dieses Circles abgelegt:

- `260825-0711_o_wie-heisst-das-archiv-einer-einzelnen-datei-mit-endung.md`
- `260825-0711_o_woran-erkennt-unzip-dass-eine-datei-ein-zip-ist.md`
- `260825-0711_o_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
- `260825-0711_o_was-tut-unzip-wenn-der-zielordner-schon-dasteht.md`

## Nicht getan

Kein Spec, kein Plan, kein Turn. Die Aktivierung ist ein eigener Schritt des Nutzers über `/fusion:next`.
