# Schuldet das Paket einen Abnahmelauf gegen die zehn Zusagen, wenn der Hauptfaden auf den Verbindungswegen drei Zugriffe statt eines stellt?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md, 261016-0532-der-anhaengeweg-schreibt-bookmarks-toml-auf-dem-hauptfaden-und-die-zusage-nennt-nur-den-einen-stat.md, 260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md

---

## Question

Der Plan des Pakets verzichtet unter „Where this work stops“ auf einen Abnahmelauf gegen die zehn Zusagen aus C8 nur, solange der Hauptfaden auf dem Verbindungs- und dem Anhängeweg allein das eine `stat(2)` aus Entscheidung 14 stellt; sonst geht die Frage an den Nutzer. Gebaut sind drei Zugriffe: das `stat(2)`, das Anlegen einer fehlenden `secrets.txt` über den Weg von F2 (`notizordner_bereitstellen`, allein in `verbindungsschluessel_holen`) und das Schreiben von `bookmarks.toml` über `lesezeichen_aendern` (allein in `anhaengen_einziehen`). Die Auslieferung 3.4.0 ist vor der Antwort gefahren.

## Options

1. **Kein Abnahmelauf; die drei Zugriffe sind hinzunehmen.**
   - Pros: die zwei zusätzlichen Zugriffe liegen auf seltenen Wegen (erstes Anlegen, Anlegen einer Verbindung) und folgen vorhandenen Mustern (F2, jeder Lesezeichenbefehl).
   - Cons: ungemessen.
2. **Abnahmelauf am Bündel durch den Nutzer.**
   - Pros: gemessen statt geschätzt.
   - Cons: Nutzerarbeit im Vordergrund für Wege, die keine der zehn Zusagen misst.

---
Answered: 261016-1429-schuldet-das-paket-einen-abnahmelauf-wenn-der-hauptfaden-drei-zugriffe-statt-eines-stellt.md — Möglichkeit 1: „3 fach zugriff ist ok, Reaktionszeiten dabei nicht so relevant“; kein Abnahmelauf geschuldet; ruled by user, Kai Stalmann <kai@stalmann.org>
