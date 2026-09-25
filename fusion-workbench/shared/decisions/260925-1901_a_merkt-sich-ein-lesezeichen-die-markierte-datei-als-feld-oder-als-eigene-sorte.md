# Merkt sich ein Lesezeichen die markierte Datei als Feld am Ordnerziel oder als eigene Sorte?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-1850_*_ein-lesezeichen-bei-markierter-datei-merkt-nur-den-ordner-und-der-sprung-markiert-nichts.md

---

## Question

Ein Lesezeichen aus dem Dateifenster soll beim Sprung die Datei wieder markieren, auf der die Auswahl beim Anlegen stand. `Ziel` (`crates/krk-core/src/ablage/lesezeichen.rs`) kennt dafür keine Angabe. Zu entscheiden ist, wo die Datei im Lesezeichen und damit in `bookmarks.toml` steht.

## Options

1. **Wahlfreies Feld `auswahl` an `Ziel::Ordner`** — die Ordnermarke merkt sich den Namen der ausgewählten Zeile.
   - Pros: eine Datei aus der Zeit davor wird unverändert gelesen und geschrieben; gültig bleibt die Marke, solange der Ordner steht; fehlt die Datei, öffnet der Sprung den Ordner wie bisher.
   - Cons: das Sinnbild in der Leiste unterscheidet eine Marke mit gemerkter Zeile nicht von einer ohne.
2. **Dritte Sorte `Ziel::Datei`** — eigenes Sinnbild, ungültig, sobald die Datei fehlt.
   - Pros: die Sorte sagt, worauf das Lesezeichen zeigt.
   - Cons: im Dateifenster steht die Auswahl fast immer auf irgendeiner Zeile, also würde fast jedes neue Lesezeichen zu dieser Sorte, auch wo nur der Ordner gemeint war; die unmarkierte Auswahl in `bookmarks.toml` müsste `Textstelle` vor `Datei` prüfen, weil beide ein Feld `datei` tragen.

## Constraints

- Eine bestehende `bookmarks.toml` bleibt gültig (C6, dreizehntes Abnahmekriterium).
- Die Datei bleibt ohne Sortenkennung von Hand lesbar (C6, zwölftes Abnahmekriterium).

## Recommendation

Möglichkeit 1: sie ändert die Bedeutung eines bestehenden Lesezeichens nicht, und eine Marke, deren Datei fehlt, bleibt brauchbar.

---
Answered: dieser Datensatz `## Recommendation` — Möglichkeit 1, das wahlfreie Feld an der Ordnermarke; ruled by user, Kai Stalmann <kai@stalmann.org>
