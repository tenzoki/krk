# Zeigt die Vorschau lokale HTML-Dateien künftig gerendert, oder bleibt die Frage beim Web-Betrachter?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`, `### Offene Fragen`, Frage 2; `crates/krk-ui/src/vorschaumodell.rs:29` und `:182` (die Dreiteilung aus C6 und der `Inhalt`); Festlegung A dieser Runde (keine Web-Ansicht)

---

## Question

Der vorgesehene Circle des Web-Betrachters führt als zweite offene Frage: „Zeigt der Betrachter auch lokale HTML-Dateien?" Sie steht dort, weil eine `.html`-Datei heute unter Text fällt und als Quelltext erscheint, und weil ein gerendertes HTML die Dreiteilung der Anzeige aus C6 ändern würde, also eine abgenommene Fähigkeit der Runde 1.

Diese Runde ändert die Dreiteilung ohnehin, für Markdown. Damit steht die Frage jetzt an, und zwar bevor der Web-Betrachter sie stellen kann. Wer sie übersieht, entscheidet sie stillschweigend: eine Runde, die Markdown gerendert zeigt und HTML als Quelltext, hat gesagt, dass HTML Quelltext bleibt, ohne dass jemand es aufgeschrieben hätte.

Die Lage ist dabei nicht symmetrisch. Markdown lässt sich ohne Web-Mittel zerlegen und in Textmerkmale übersetzen, HTML im Allgemeinen nicht: eine Seite mit Auszeichnungssprache, Formatvorlagen und Skript ist genau das, wofür eine Web-Ansicht da ist, und die ist in Festlegung A abgelehnt.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **HTML bleibt Quelltext, und die Runde schreibt das ausdrücklich fest.** Eine `.html`-Datei erscheint wie heute als Text, künftig mit Syntaxeinfärbung über `hervorhebung.rs`, die HTML kennt.
   - Folge: die Frage des Web-Betrachters bleibt offen und wird dort mit ihren eigenen Möglichkeiten entschieden. Nebenbei bekommt HTML in dieser Runde eine sichtbare Verbesserung, nämlich Farbe statt grauem Text, und zwar ohne jedes neue Mittel.
   - Preis: keiner, der über die bestehende Lage hinausginge. Wer eine HTML-Datei gerendert sehen will, gibt sie mit `return` an das Standardprogramm, wie seit der Runde 4.

2. **HTML wird in dieser Runde ebenfalls gerendert, ohne Web-Ansicht.** Die Zerlegung nimmt aus HTML dieselben Wirkungen wie aus Markdown: Überschriften, Listen, Links, Betonung.
   - Folge: die Vorschau zeigt eine einfache HTML-Datei lesbar, und die zweite offene Frage des Web-Betrachters ist beantwortet.
   - Preis: die Zusage hält nur für einfaches HTML. Sobald Formatvorlagen oder Skript im Spiel sind, zeigt die Vorschau etwas, das dem gerenderten Ergebnis ähnelt und nicht entspricht, und der Nutzer kann nicht sehen, wann. Eine Anzeige, die manchmal stimmt, ist schwerer zu erklären als eine, die immer Quelltext zeigt. Daneben nimmt die Antwort dem Web-Betrachter seine Frage weg, ohne dass dieser Circle dessen übrige Zuschnittfragen kennt.

3. **HTML wird gerendert, sobald der Web-Betrachter steht, und diese Runde legt es fest.** Der Spec schreibt die Zusage, die Umsetzung wartet auf jenen Circle.
   - Folge: die Frage ist beantwortet, ohne dass diese Runde das Mittel bauen muss.
   - Preis: eine Zusage, die diese Runde nicht einlösen kann, gehört nicht in ihren Spec. Sie wäre ein Abnahmekriterium ohne Abnahme, und dieses Projekt hat mit unabgehakten Kriterien bereits genug Erfahrung.

## Constraints

- Eine Web-Ansicht ist in Festlegung A ausdrücklich abgelehnt. Jede Antwort, die eine braucht, ist ausgeschlossen.
- Die Dreiteilung aus C6 ist eine abgenommene Fähigkeit der Runde 1. Diese Runde schreibt sie fort und muss dabei sagen, wo HTML landet, ob sie es ändert oder nicht.
- Der Circle des Web-Betrachters hält selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene Untersuchung vor dem Plan gehört. Eine Antwort hier darf jener Untersuchung nicht vorgreifen.

## Recommendation

**Wir empfehlen Möglichkeit 1**, und zwar ausdrücklich im Spec und nicht durch Schweigen. Der Grund ist die Ungleichheit der beiden Formate: Markdown ist ohne Web-Mittel vollständig zerlegbar, HTML ist es nicht, und eine Anzeige, die bei einfachem HTML stimmt und bei anderem nicht, gibt dem Nutzer keine Regel an die Hand, an der er sich orientieren könnte.

Der Nebengewinn von Möglichkeit 1 ist echt und kostet nichts: `hervorhebung.rs` und `syntect` färben HTML bereits ein, sobald die Vorschau Quelltext überhaupt einfärbt. Eine `.html`-Datei sieht nach dieser Runde besser aus als vorher, auch ohne gerendert zu werden.


## Antwort 260812-1105

**Moeglichkeit 1.**

Lokale HTML-Dateien bleiben Quelltext. Die Frage bleibt beim Web-Betrachter-Circle, und der Spec
sagt das **ausdruecklich** statt es durch Schweigen offenzulassen.

Der Grund ist die Ungleichheit der beiden Formate: Markdown ist ohne Web-Mittel vollstaendig
zerlegbar, HTML ist es nicht, und eine Anzeige, die bei einfachem HTML stimmt und bei anderem
nicht, gibt dem Nutzer keine Regel an die Hand.

**Der Nebengewinn ist echt und kostet nichts:** `hervorhebung.rs` und `syntect` faerben HTML
bereits ein, sobald die Vorschau Quelltext ueberhaupt einfaerbt. Eine `.html`-Datei sieht nach
dieser Runde besser aus als vorher, auch ohne gerendert zu werden.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented: `crates/krk-ui/src/hervorhebung.rs:422-431` (`art` gibt nur fuer `Dateityp::Markdown` `Darstellungsart::Markdown`; HTML faellt unter `Dateityp::Sonstiges` und bekommt ueber `sprache_fuer` `Darstellungsart::Code`, also Quelltext mit Einfaerbung) — Schritt 8 des Plans, Commit `b4d9de2`. Eine zweite Endungsliste ist nicht entstanden. Abgeglichen am 260812-2253.
Deferred:
Superseded by:
