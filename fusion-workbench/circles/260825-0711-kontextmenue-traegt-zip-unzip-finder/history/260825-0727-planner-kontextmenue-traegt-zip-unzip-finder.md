# Planner: Kontextmenü trägt Zip, Unzip und Finder

**Datum:** 2026-08-25
**Agent:** planner (dispatch, Executors: coder, ontocoder, analyst)
**Status:** Complete

## Auftrag

Den Implementierungsplan für die Runde 17 schreiben, gegen die Directive und den Grounding-Schnappschuss des aktiven Circles `260825-0711-kontextmenue-traegt-zip-unzip-finder`. Kein Spec vorhanden; geplant gegen den Circle-Datensatz. Die sechs vom Nutzer entschiedenen Punkte waren gesetzt und nicht neu zu verhandeln.

## Erzeugter Plan

`planning/260825-0727_o_plan-kontextmenue-traegt-zip-unzip-finder.md`, acht Schritte, alle an `coder`.

Kein Schritt geht an `ontocoder` oder `analyst`. Die zwei berührten `.toml`-Dateien sind Baumanifeste (`Cargo.toml`, `crates/krk-core/Cargo.toml`) und damit nach der Zuordnungstabelle Sache von `coder`; `resources/default-keymap.toml` bleibt unberührt, weil kein Befehl dieser Runde eine Tastenkombination bekommt. Ein strategisches Erzeugnis fällt nicht an: die vier Entscheidungen liegen als Datensätze des Shapers vor, und die Gegenüberstellung der drei Packwege ist im Plan selbst gemessen abgelegt.

## Der tragende Entwurf

Zip und Unzip werden zwei weitere Werte der Aufzählung `Art` und keine zweite Maschine neben `krk_core::operation`. Damit erben sie Fortschritt, Abbruch, Konfliktrückfrage, Abschlussliste und die Zusage L9. Zip steht als einziger Wert **neben** der Quelle-für-Quelle-Schleife, weil es ein Ziel für den ganzen Lauf hat und nicht eines je Quelle; die Verzweigung in `ausfuehren` bleibt vollständig und ohne Auffangzweig.

Die Regel, was das Kontextmenü trägt und worauf jeder Eintrag wirkt, zieht in ein neues Modul `krk-ui/src/kommandos/kontextmenue.rs` ohne AppKit, nach dem Vorbild von `kommandos/rueckschritt.rs` und `verzeichnis/filter.rs`. Die drei Einträge tragen **einen** Selektor und unterscheiden sich in der Menümarke; die Ausführung verzweigt vollständig über eine dreiwertige Aufzählung. Damit hält der Übersetzer genau die Falle geschlossen, die `CLAUDE.md` für Tastenbefehle beschreibt: ein Eintrag, der dasteht und nichts tut.

Für Finder entsteht keine neue Hülle. `appkit/terminal::ordner_oeffnen` beantwortet die Frage bereits und nennt sich selbst „die eine Stelle des Programms, die eine Bündelkennung in einen Anwendungsort auflöst"; sie bekommt einen zweiten Aufrufer mit `com.apple.finder`.

## Die Wahl der Packkiste, gemessen

Drei Wege standen zur Wahl: ein Unterprozess auf `ditto(1)`, ein eigener Zip-Schreiber, eine fremde Kiste. Der Plan wählt `zip 8.6` mit `default-features = false, features = ["deflate-flate2"]` und `flate2` daneben. Vier Messungen sind am 260825 auf diesem Gerät erhoben und stehen im Plan:

- **Zwei neue Pakete** in `Cargo.lock`, `zip 8.6.0` und `typed-path 0.12.3`. Neun weitere, auf denen `zip` aufsetzt, stehen bereits in denselben Fassungen im Baum, über `syntect` und `toml`; `crc32fast` steigt um eine Fehlerbehebungsstufe.
- **Kein `cc`, kein `-sys`-Paket.** Die Zusage aus der Technologiewahl hält.
- **`flate2` muss unmittelbar danebenstehen.** `zip` schaltet es ohne seine Vorgabemerkmale ein; allein damit übersetzt es nicht („No compression backend selected"). Gemessen an einem Wegwerf-Workspace, der zuerst rot und nach dem Zusatz grün war.
- **Geschwindigkeit auf diesem Gerät**, nicht auf dem Referenzgerät: 8 MB kaum verdichtbarer Daten in 212 ms im Profil `release`, also rund 38 MB/s.

Der Unterprozess scheidet an drei Zusagen aus: Fortschritt je Eintrag, Abbruch zwischen zwei Einträgen, und eine Fehlermeldung, die nicht über die Standardfehlerausgabe kommt. Er hätte zudem `shared/decisions/260821-1221_*` gebunden; so bleibt jener Datensatz unberührt.

## Die vier offenen Entscheidungen

Alle vier gelesen; der Plan folgt in allen vier Fällen der Empfehlung und nennt je Datensatz den Schritt, an dem die Antwort gebraucht wird. Zwei Zusätze, die die Datensätze nicht nennen:

- **Zum Archivnamen:** allein Möglichkeit 1 macht das Paar Zip/Unzip umkehrbar. Unter Möglichkeit 2 geht der Ursprungsname verloren, und da beide Befehle im selben Menü stehen, sieht der Nutzer den Verlust unmittelbar.
- **Zum Zielordner beim zweiten Entpacken:** der Datensatz schreibt „`Projekte 2`", und dieses Vorhaben hat für freie Namen bereits `operation::freier_name`, die „Projekte Kopie" liefert. Der Plan schlägt vor, die vorhandene Regel zu nehmen statt eine zweite zu bauen.

Zur Empfehlung des Konfliktblatts ist geprüft, dass die Form im Baum schon einmal gebaut ist: die Löschbestätigung legt die Eingabetaste auf „Abbrechen" und erreicht die Escape-Taste über `Blattgriff::abbrechen`. Ein neuer Mechanismus entsteht damit nicht.

## Angelegt

- `decisions/260825-0727_o_nimmt-unzip-die-betroffenen-eintraege-oder-allein-die-ausgewaehlte-zeile.md` — die Directive beschreibt Unzips Wirkungsbereich anders als Zips, und bei stehender Markierung fallen die beiden auseinander. Drei Möglichkeiten mit Folgen, Empfehlung: dieselbe Regel `betroffene` wie bei Zip.
- `shared/issues/260825-0727_o_claude-md-nennt-zwei-aufrufer-von-ohne-warten-oeffnen-der-baum-traegt-drei.md` — `CLAUDE.md` und der Modulkopf von `verzeichnis/sys.rs` zählen zwei Aufrufer, der Baum trägt drei; der dritte ist `anlesen` aus der Runde 16. Nicht aus der Directive dieser Runde entstanden, also in den gemeinsamen Speicher.

## Nicht getan

Kein Code, keine Datei unter `crates/` angefasst, kein Ausführeragent gestartet. Die Ausführung entscheidet der Nutzer nach dem Abnahmegate; vorher sind die fünf offenen Entscheidungen zu beantworten, weil vier davon Rumpfe von Funktionen festlegen, die in den Schritten 3 und 4 entstehen.
