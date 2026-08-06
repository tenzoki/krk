# Braucht die Belegungsansicht einen Weg, eine einzelne Kombination zu entfernen?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator (aufgeworfen vom coder bei der Umsetzung von S20)
**Cross-references:** `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` (Schritt S20), `planning/260802-1036_*_spec-navigator-geruest.md` `### C3`, `history/260805-2250-coder-s20-belegungsansicht.md`

---

## Question

Die Belegungsansicht aus S20 kann einer Funktion eine Kombination zuweisen, die ganze Belegung auf den Auslieferungszustand zurücksetzen und beim Verlassen speichern. Eine **einzelne** Kombination entfernen kann sie nicht. Der Planschritt nennt diesen Fall nicht; C3 im Spec verlangt ihn nicht ausdrücklich. Die heutigen Rückwege sind das vollständige Zurücksetzen oder das Handbearbeiten von `~/Library/Application Support/KRK/keymap.toml`. Soll das so bleiben, oder bekommt die Ansicht einen Entfernen-Befehl?

## Options

1. **So lassen (Runde 1 schließt ohne Entfernen-Befehl).**
   - Pros: kein neuer Umfang in der Abnahmephase; C3 ist ohne ihn erfüllt; der Rückweg über Zurücksetzen existiert.
   - Cons: wer eine von mehreren Kombinationen einer Funktion loswerden will, verliert beim Zurücksetzen alle übrigen Anpassungen.
2. **Entfernen-Befehl in der Ansicht nachrüsten (eigener kleiner Schritt einer späteren Runde).**
   - Pros: vollständige Pflege der Belegung ohne Handbearbeitung der Datei.
   - Cons: neue Bedienfrage (welche Taste, welche Auswahllogik bei mehreren Kombinationen je Zeile) und neuer Prüfumfang.

## Constraints

- Runde 1 ist in der Abnahmephase; neuer Umfang gehört nach der bestehenden Praxis in einen Nachzug oder eine spätere Runde.
- Die Ansicht führt keine eigene Tabelle; ein Entfernen-Befehl müsste über die Belegungsmaschine laufen.

## Recommendation

Möglichkeit 1 für Runde 1; die Frage bei der nächsten Rundenplanung erneut vorlegen.

---
Answered:
Implemented:
Deferred:
Superseded by:
