# Shaper, anticipated-circle: die Vorschau rendert PDF als Betrachter

**Datum:** 2026-08-27, 19:37 bis 20:28
**Modus:** anticipated-circle, dispatched über `/fusion:direct`, Domain code
**Entwurf:** `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` (ein Absatz: die Vorschau soll jpg, png und vor allem pdf rendern)

## Klärungsrunde 1 (260827-1937) und Antworten (260827-2028)

1. Bilder: Gegenstand oder Defekt? Antwort a: JPG und PNG rendert die Vorschau schon (`BILDENDUNGEN`, Runde 1), die Runde wird auf PDF verengt.
2. Was heißt „PDF rendern"? Antwort c: ein Betrachter mit Zoom, Seitensprung und Seitenzähler in der Statuszeile.
3. Größengrenze: Antwort a: dieselben 64 MB wie bei Bildern (`BILDGRENZE`).
4. Auswahl und Kopieren: Antwort b: Text auf der Seite lässt sich markieren und mit Cmd+C kopieren.

Keine zweite Runde nötig. Vorgaben nach bestehendem Muster, im Grounding festgehalten: Rückfall auf Metadaten über der Grenze und bei unlesbarem PDF wie beim Bild; Zwischenablage-PDF kein Gegenstand.

## Offen gelassen

Die Tastenbelegung für Zoom und Seitensprung war nicht Teil der Runde und ist als Datensatz gefilet: `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/decisions/260827-2028_*_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md`.

## Ergebnis

- Circle `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/` mit `_a_circle.md` und den sechs Unterordnern.
- Backlogeintrag auf `_c_` gesetzt, `Promoted:`-Zeile angehängt.
- Kein Spec, keine Aktivierung: die ist `/fusion:next`.
