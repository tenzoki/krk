# Planner: Umsetzungsplan für den Notizzettel als Blatt mit zwei Zetteln

**Date:** 2026-08-14 06:56
**Agent:** planner
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/` (aktiv)
**Status:** Complete

---

## Auftrag

Den Umsetzungsplan zum Spec vom 260813-2348 bauen, in der am 260814-0628 nachgezogenen Fassung. Ausführer sind `coder` und `ontocoder`. Kein Bau, kein `make bundle`.

## Was entstanden ist

- `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md` — sechzehn Schritte in sechs Strängen, drei Mermaid-Bilder, je Schritt Ausführer, Dateien, Änderungen, Abhängigkeiten und das Abnahmekriterium des Spec, das er erfüllt.
- `decisions/260814-0656_o_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md` — drei Möglichkeiten mit Kosten, Empfehlung Möglichkeit 2.
- `shared/issues/260814-0656_o_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` — im gemeinsamen Speicher, weil der Defekt jede Runde seit der siebten betrifft und nicht aus dieser Directive entstanden ist.

## Grundlage: elf Feststellungen am Baum

Zusätzlich zu den achtzehn des Spec. Die fünf tragenden:

1. **`Zugang::beiseite_legen` kann den Zettel in seiner heutigen Form nicht annehmen.** Es nimmt `&str`, und beide unlesbaren Fälle tragen keinen: eine ungültige UTF-8-Folge ist keiner, und eine Datei über `EDITORGRENZE` darf nicht in den Speicher. Der Plan weitet `beiseite_legen` und `atomar::schreiben` auf einen Leser.
2. **`text::datei::oeffnen` beantwortet die Frage des Zettels schon vollständig**, wirft aber Bytes und Deskriptor weg. Der Plan zerlegt es in einen Befund (`Textstand`) und dessen Übersetzung; der Editor sieht keine geänderte Schnittstelle.
3. **Die „elf Fundstellen" von `Datei::ALLE` sind sieben und zerfallen in zwei Sorten** — vier TOML-Rundläufe, drei Fragen nach Pfad und Name. Getrennt wird über eine abgeleitete Frage `Datei::format()` statt über eine zweite Liste.
4. **Das `match` in `kommando_ausfuehren` hat einen Auffangzweig.** Ein neues Kommando ohne eigenen Zweig fällt stillschweigend durch und tut nichts; der Übersetzer sagt kein Wort. Der Zweig steht als eigener Schritt.
5. **Eine Funktion, die die `keymap.toml` des Nutzers nicht nennt, tritt unbelegt hinzu** (`belegung.rs:1252-1267`). Für jeden Nutzer mit eigener Datei kommt der Notizzettel ohne `f2` und `cmd+k` an. Daraus der Defekt oben und ein Posten in der Nutzerliste.

## Vier Entwurfsentscheidungen, die der Spec dem Planer überlassen hat

- **`Zettel` wird eine eigene Aufzählung mit zwei Werten**, `Datei::Zettel(Zettel)` trägt sie. Damit ist „genau zwei Zettel" eine Aussage über einen Typ. Vorbild ist `Fensterseite` mit derselben ausgeschriebenen Erwägung.
- **Der Wächter des Zettels ist ein eigener Typ und kein Schalter am `Eingabewaechter`.** Es sind zwei verschiedene AppKit-Protokolle mit zwei Signaturen, und ein Schalter wären zwei Wahrheiten darüber, was die Eingabetaste in einem Blatt tut.
- **Das Sichern hängt am Abschlussblock des Blattes und nicht am Wächter.** Beide Wege heraus, die Escape-Taste und ein Abbruch über den `Blattgriff`, laufen ohnehin dort zusammen.
- **Der vierte Sicherungsmoment nimmt beim Beenden keinen eigenen Durchgang**, sondern den, den `applicationWillTerminate:` schon hält. Zwei Durchgänge dort waren der Defekt vom 260813-0540.

## Die Messung, die der Spec offengelassen hat

Was AppKit mit `performClose:` an einem Fenster mit anhängendem Blatt tut, ist im Baum nicht gemessen, und ich kann es nicht messen: es verlangt KRK im Vordergrund. Der Plan nimmt die Frage stattdessen aus dem tragenden Weg heraus — gesichert wird unbedingt und vor dem Aufruf, das Blatt wird nicht vorher abgeräumt, und damit hält die Zusage in beiden Ausgängen. Die Messung steht als Nutzerarbeit mit sechs Schritten und einem Ziel in `messungen/`; sie trägt nach, welche Kante das Bündel geht, und entscheidet nichts mehr.

## Bilder

Drei, alle drei am 260814-0703 mit `@mermaid-js/mermaid-cli` 11.16.0 nach SVG gerendert: der Leseweg einer Zetteldatei mit seinen vier Ausgängen, die vier Sicherungsmomente mit ihrer einen Erklärung, und die Reihenfolge der sechzehn Schritte als DAG. Kein Zyklus, kein Gott-Knoten, kein freistehender Knoten, jedes Bild geschichtet.

## Was der Plan nicht anfasst

`waehrend_blatt_erlaubt`, `immer_erreichbar`, `zulaessigkeit::zulaessig`, die Aufzählungen `Wirkungsbereich`, `Bereich`, `Fokus` und `Funktionsbereich`, die zehn Zeitzusagen, den Abschnitt `## Directive` des Circle-Datensatzes (der gehört dem Shaper) und die zwei Bestandsaufnahme-Defekte des Circles.

## Voice-Profile

`chat-voice-de.yaml` und `default-voice-de.yaml` geladen und angewandt.
