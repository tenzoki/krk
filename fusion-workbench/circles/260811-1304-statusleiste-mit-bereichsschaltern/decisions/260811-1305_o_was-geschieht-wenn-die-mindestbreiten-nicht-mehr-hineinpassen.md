# Was geschieht, wenn die Mindestbreiten der sichtbaren Bereiche nicht mehr hineinpassen?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/fenstermodell.rs:169` (`Bereich::mindestbreite`), `crates/krk-ui/src/fenstermodell.rs:639` (die Deckelung der festen Bereiche), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` (C7)

---

## Question

Die fünf Mindestbreiten summieren sich auf 1080 Punkte, wenn alle fünf Bereiche zugleich stünden: Lesezeichen 120, zwei Dateifenster je 240, Vorschau 160, Editor 320. Vorschau und Editor schließen sich aus, also sind es tatsächlich höchstens 920 mit dem Editor und 760 mit der Vorschau. Ein schmales Fenster unterschreitet auch das.

Heute steht die Antwort im Code und nirgends als Regel. `bereichsbreiten` deckelt jeden festen Bereich auf `(rest − Mindestmaß der Dateifenster).max(0.0)`; ein fester Bereich kann damit die Breite `0.0` bekommen und gilt weiterhin als sichtbar. Der Nutzer sähe einen Bereich, den er eingeschaltet hat und der nicht da ist. Mit einer Leiste, deren Schalter jederzeit einen vierten Bereich hinzuschalten, wird der Fall häufiger, und der Schalter behauptet dann etwas, was die Fensterzeile nicht zeigt.

## Options

1. **Der Schalter, der die Grenze überschritte, wird ohne Meldung verworfen.** Der Bereich geht nicht auf, und der Schalter springt zurück.
   - Pros: Genau die Form, die C7 für den Ausblendbefehl am letzten Dateifenster schon festlegt: "wird ohne Fehlermeldung verworfen". Es entsteht keine zweite Art, mit einer unmöglichen Anforderung umzugehen. Die Mindestbreite behält ihre Bedeutung.
   - Cons: Ein Klick ohne sichtbare Wirkung. Der Nutzer erfährt nicht, warum.
   - **Folgen weiter unten:** `Fenstermodell::umschalten` bekommt eine zweite Abweisungsbedingung neben der für das letzte Dateifenster, und beide brauchen dieselbe Form. Der Aktivierungs-Spec entscheidet, ob der Grund in der Statuszeile steht; C7 verlangt für den bestehenden Fall ausdrücklich, dass **keine** Meldung erscheint.

2. **Alle sichtbaren Bereiche schrumpfen anteilig unter ihr Mindestmaß.** Die Mindestbreite ist dann eine Vorgabe für das Ziehen der Trennlinie und keine Schranke der Aufteilung.
   - Pros: Der Schalter tut immer, was er sagt. Keine zweite Abweisung.
   - Cons: Vier Spalten passen dann nicht mehr in ein Dateifenster, und der Editor fällt unter die Breite, bei der eine Zeile Text lesbar ist. Das vierte Abnahmekriterium von C1 der Editor-Runde ist damit unterlaufen.
   - **Folgen weiter unten:** Die Mindestbreite trägt zwei Bedeutungen, je nachdem wer sie anfasst. `critical-stance.md` §4 nennt eine Fallunterscheidung, deren Zweige sich überschneiden, einen Defekt.

3. **Die Bereiche weichen in der Reihenfolge von `Bereich::ALLE`, der letzte bekommt null.** Die heutige Deckelung, aber als Regel ausgesprochen.
   - Pros: Nichts ist zu bauen, und das Verhalten bleibt, wie es ist.
   - Cons: Ein eingeschalteter Bereich mit der Breite null ist eine Sichtbarkeit, die nichts sichtbar macht. Der Schalter steht auf an, die Fensterzeile zeigt nichts, und beide Aussagen widersprechen sich.
   - **Folgen weiter unten:** Der Aktivierungs-Spec müsste ein Abnahmekriterium formulieren, das diesen Widerspruch als gewollt festhält.

## Constraints

- C7 legt für den einen bestehenden Fall einer unmöglichen Sichtbarkeitsanforderung fest: ohne Fehlermeldung verwerfen, und die Abweisung steht im Modell und nicht in der Belegungsdatei.
- Das vierte Abnahmekriterium von C1 der Editor-Runde verlangt vom Editor "nicht schmaler, als eine Zeile Text noch lesbar ist"; daher seine 320 Punkte.
- Das Fenster trägt eine eigene Mindestgröße; wie groß sie ist und ob sie den Fall bereits ausschließt, ist ungeprüft (`inference:`, nicht gemessen).

## Recommendation

**Möglichkeit 1**, und vor der Antwort ist die Mindestgröße des Fensters zu messen. Trifft der Fall bei der heutigen Mindestgröße gar nicht ein, ist die Abweisung eine Vorsichtsmaßnahme und keine Fähigkeit, und der Aktivierungs-Spec kann sie klein halten. Trifft er ein, ist Möglichkeit 1 die einzige, in der die Mindestbreite eine Bedeutung behält und der Schalter nicht lügt.

---
Answered:
Implemented:
Deferred:
Superseded by:
