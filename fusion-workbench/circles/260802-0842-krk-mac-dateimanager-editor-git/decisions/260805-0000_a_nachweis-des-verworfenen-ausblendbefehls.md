# Wird das vierte Abnahmekriterium von C7 auf den Nachweis am Modell gezogen, oder bekommt der Ausblendbefehl einen Weg zum linken Dateifenster?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1040_c_der-verworfene-ausblendbefehl-aus-c7-hat-keinen-ausloeser.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C7), `crates/krk-ui/src/fenstermodell.rs`, `crates/krk-core/src/ablage/sitzung.rs`

---

## Question

Das vierte Abnahmekriterium von C7 lautet: "Mindestens ein Dateifenster bleibt immer sichtbar. Ein Befehl, der das letzte ausblenden würde, wird ohne Fehlermeldung ignoriert." Der erste Satz hält. Der zweite beschreibt einen Fall, den mit der ausgelieferten Belegung kein Tastendruck herbeiführt: die einzige Funktion, die ein Dateifenster ausblendet, ist `zweites_fenster_umschalten`, und sie trifft immer das rechte. Das Abnahmekriterium von S12 verlangt aber, jedes Kriterium aus C7 sei am laufenden Bündel einzeln nachweisbar.

## Options

1. **Den Nachweis am Modell führen.** Das Kriterium behält beide Sätze, aber der zweite wird an der Prüfung `das_letzte_dateifenster_laesst_sich_nicht_ausblenden` in `crates/krk-ui/src/fenstermodell.rs` nachgewiesen statt am laufenden Bündel.
   - Pros: keine Taste für einen Fehlerfall; die Abweisung bleibt, wo sie ist, und eine spätere Belegung findet sie vor.
   - Cons: ein Kriterium aus C7 fällt aus der Sichtprüfung heraus, und S12 braucht dafür eine Notiz.
2. **Beide Dateifenster ausblendbar machen.** Der Befehl trifft dann das **aktive** Dateifenster; trifft er das letzte sichtbare, wird er verworfen.
   - Pros: der zweite Satz wird am Bündel wahr.
   - Cons: kostet ein Feld in `krk_core::ablage::Sichtbarkeit`, dessen Modulkopf ausdrücklich begründet, warum es keines für das linke Dateifenster gibt; ändert die Bedeutung von `zweites_fenster_umschalten`, dessen Name danach nicht mehr passt; und verbraucht eine Taste für einen Fall, den der Nutzer nie absichtlich herbeiführt.

## Constraints

- Die Abweisung selbst gibt es und ist geprüft; sie steht mit Absicht im Modell und nicht in der Belegungsdatei, damit eine spätere Belegung keinen ungeprüften Weg dorthin öffnet.
- S12 trägt `[DONE]`; ein abgenommener Schritt wird nicht rückwirkend verschärft.

## Recommendation

Möglichkeit 1.

---
Answered: Nutzer am 260805-0000 — Möglichkeit 1. Begründung des Nutzers: die Lage ist über die ausgelieferte Belegung nicht herstellbar, und ein Kürzel dafür zu erfinden hieße, eine Taste für einen Fehlerfall zu verbrauchen.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C7, im vierten Abnahmekriterium und als eigene Festlegung mit der verworfenen Möglichkeit. Im Plan bekommt S12 eine Notiz und bleibt abgenommen; dieselbe Notiz nimmt die Zahlen "acht" und "sieben" aus seinem Abnahmekriterium, weil C1 schon vorher mehr Kriterien führte als dort standen.
Implemented: `planning/260802-1036_o_spec-navigator-geruest.md` C7 — der Nachweis steht bereits in `crates/krk-ui/src/fenstermodell.rs` als `das_letzte_dateifenster_laesst_sich_nicht_ausblenden`; ein Codeeingriff folgt aus dieser Antwort nicht.
