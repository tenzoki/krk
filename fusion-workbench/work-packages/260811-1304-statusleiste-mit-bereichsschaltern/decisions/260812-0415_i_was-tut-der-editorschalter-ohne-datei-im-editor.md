# Was tut der Editorschalter, wenn der Editor keine Datei hält?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_o_bereichsleiste-und-proportionale-breitenregel.md` (Schritt 5, C2.4 und C6.3), `crates/krk-core/src/ablage/sitzung.rs` (`Sichtbarkeit::default`), `crates/krk-ui/src/kommandos/fokus.rs` (`holt_hervor`), `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_a_wie-zeigen-zwei-schalter-eine-flaeche-die-nur-einer-haben-kann.md`

---

## Question

Beim allerersten Start hält der Editor keine Datei, und er steht deshalb auf ausgeblendet. Die Begründung steht an `Sichtbarkeit::default`: ein sichtbarer leerer Editor nähme den Dateifenstern Platz für nichts. Dieselbe Bedingung trägt der Fokusbefehl: `fokus_editor` holt einen ausgeblendeten Editor nur hervor, sofern er eine Datei hält.

Der neue Editorschalter in der Bereichsleiste stellt die Frage ein drittes Mal, und die Klärungsrunde hat sie nicht gestellt. Ein Klick auf den Schalter, während der Editor keine Datei hält: was geschieht?

## Options

1. **Ohne Meldung verwerfen, der Schalter springt zurück.** Dieselbe Form wie die beiden anderen Abweisungen dieser Runde und dieselbe Bedingung, die `fokus_editor` schon trägt.
   - Pro: Es entsteht keine dritte Antwort auf eine unmögliche Sichtbarkeitsanforderung. Die Begründung an `Sichtbarkeit::default` bleibt gültig.
   - Contra: Ein Klick ohne sichtbare Wirkung, und der Nutzer erfährt nicht, warum. Beim ersten Start ist das der Normalfall und nicht die Ausnahme.
2. **Den leeren Editor zeigen.** Der Schalter tut immer, was er sagt.
   - Pro: Keine Abweisung, kein stummer Klick. Der Bereich verhält sich wie die vier anderen.
   - Contra: 320 bis 460 Punkte Fensterbreite für eine leere Fläche, gegen die Begründung an `Sichtbarkeit::default`. Daneben verschöbe es die Frage, was der leere Editor anzeigt und ob die Vorschau dafür weichen soll.
3. **Der Schalter füllt den Editor: er öffnet den ausgewählten Eintrag des aktiven Dateifensters, wie F4.**
   - Pro: Der Klick hat immer eine Wirkung, und sie ist die, die der Nutzer vermutlich will.
   - Contra: Ein Schalter, der zwei verschiedene Dinge tut, je nach Zustand. Es entstünde ein dritter Einstiegsweg in den Editor neben F4 und dem Übergang aus der Vorschau, und er hätte eine eigene Auswahlregel.

## Constraints

- Der Editorschalter geht durch dieselbe Modellfunktion wie der Tastenbefehl; ein zweiter Weg an den Prüfungen vorbei ist ausgeschlossen.
- Die Bedingung "hält eine Datei" kennt allein der Editorbereich. Das Fenstermodell weiß von Dateien nichts, also steht die Prüfung beim Anwendungsdelegierten, wie sie es für `fokus_editor` tut.
- C7 der Runde 1 verlangt für den bestehenden Fall einer unmöglichen Sichtbarkeitsanforderung ausdrücklich, dass keine Meldung erscheint.

## Recommendation

**Möglichkeit 1.** Sie ist die einzige, die keine vierte Regel neben die drei bestehenden stellt, und sie hält die Begründung ein, aus der der Editor beim ersten Start ausgeblendet ist. Der Preis ist benannt: beim allerersten Start ist der Editorschalter stumm, bis der Nutzer eine Datei öffnet.

Der Plan setzt Möglichkeit 1 um. Eine andere Antwort ändert den Rumpf von `editor_umschalten` in Schritt 5 und sonst nichts.


## Antwort 260812-0430

**Die Empfehlung des Plans wird uebernommen: ohne Meldung verwerfen.**

Der Editor haelt beim allerersten Start keine Datei, und ein sichtbarer leerer Editor naehme den
Dateifenstern Platz fuer nichts — das ist der Grund, aus dem `Sichtbarkeit::default()` ihn als
einzigen auf `false` setzt. Ein Umschalter, der ihn ohne Datei aufgehen liesse, machte diese
Festlegung ruecklings zunichte.

**Die Form ist die dritte Abweisung derselben Bauart**, neben der am letzten Dateifenster (C7 der
Runde 1) und der an den Mindestbreiten. Alle drei verwerfen ohne Meldung, und der Schalter in der
Leiste springt in seinen alten Zustand zurueck (Kriterium C2.4 und C2.5 des Plans). Drei
verschiedene Antworten auf drei unmoegliche Sichtbarkeitsanforderungen waeren eine
Fallunterscheidung ohne Grund.

**Die Bedingung steht beim Anwendungsdelegierten und nicht im Fenstermodell.** Dasselbe tut
`fokus_editor` schon heute, und der Grund traegt hier wie dort: das Fenstermodell weiss von
Dateien nichts.

Umsetzung: Schritt 5 des Plans `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0430` — beantwortet vom Orchestrator, autonom auf Weisung des Nutzers; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented: 90b02d4 — `editor_umschalten` beim Anwendungsdelegierten verwirft ohne Meldung, solange der Editor ausgeblendet ist und keine Datei haelt.
Deferred:
Superseded by:
