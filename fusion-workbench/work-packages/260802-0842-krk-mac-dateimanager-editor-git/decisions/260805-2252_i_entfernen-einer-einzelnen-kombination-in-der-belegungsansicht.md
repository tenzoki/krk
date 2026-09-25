# Braucht die Belegungsansicht einen Weg, eine einzelne Kombination zu entfernen?

---
**Domain:** code
**Status:** implemented
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

## Antwort des Nutzers vom 260807

**Möglichkeit 1, der Empfehlung folgend: Runde 1 schließt ohne Entfernen-Befehl.** Die Frage kommt bei der nächsten Rundenplanung erneut auf den Tisch.

**"So lassen" ist hier eine Entscheidung und keine Nicht-Entscheidung, und deshalb steht der Grund hier und nicht nur der Ausgang.** Gewählt ist der Zuschnitt der Runde gegen die Vollständigkeit der Pflege. Runde 1 steht in der Abnahmephase, C3 ist ohne den Befehl erfüllt, und ein Rückweg existiert. Ein Entfernen-Befehl brächte dagegen zwei Dinge mit, die keine Fähigkeit verlangt: eine neue Bedienfrage, nämlich welche Taste ihn trägt und welche Kombination er wählt, wenn eine Zeile mehrere führt, und einen neuen Prüfumfang in genau der Phase, in der die Runde nichts mehr dazubekommen soll.

**Was die Wahl kostet.** Wer eine von mehreren Kombinationen einer Funktion loswerden will, hat zwei Wege, und beide sind unbequem. Er setzt die gesamte Belegung auf den Auslieferungszustand zurück, und **dabei fallen alle übrigen Anpassungen mit**; die Ansicht kennt kein Zurücknehmen einer einzelnen Zeile. Oder er bearbeitet `~/Library/Application Support/KRK/keymap.toml` von Hand, also genau die Datei, die die Belegungsansicht dem Nutzer abnehmen soll. Der Preis wächst mit jeder Anpassung, die der Nutzer über die Zeit vornimmt, denn er verliert beim Zurücksetzen umso mehr, je länger er die Belegung gepflegt hat. Das ist der Grund, aus dem die Frage bei der nächsten Rundenplanung erneut vorgelegt wird und nicht auf Dauer erledigt ist.

**Kein Abnahmekriterium ändert sich, und kein Schritt des Plans.** S20 bleibt abgenommen und unverändert.

---
Answered: `planning/260802-1036_*_spec-navigator-geruest.md`:234 — Möglichkeit 1, Runde 1 schließt ohne Entfernen-Befehl; der Preis des Zurücksetzens steht dort ausgeschrieben.
Implemented: `crates/krk-ui/src/appkit/belegungsansicht.rs` — die Antwort lautet „so lassen", und der Programmstand erfüllt sie ohne Eingriff: die Ansicht weist zu, setzt zurück und speichert beim Verlassen, einen Entfernen-Befehl gibt es nicht. S20 bleibt unverändert. Der Marker steht auf umgesetzt, weil der Zustand auf der Platte die Entscheidung trägt, nicht weil etwas gebaut worden wäre. Die Frage kommt bei der nächsten Rundenplanung erneut auf den Tisch; sie ist damit beantwortet und nicht erledigt.
Deferred:
Superseded by:
