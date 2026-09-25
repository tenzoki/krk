# Wie zeigen zwei Schalter eine Fläche, die nur einer von beiden haben kann?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/fenstermodell.rs:151` (`Bereich::teilt_flaeche_mit`), `circles/260807-2116-eingebauter-editor-mit-textmarken/` (C1 der Editor-Runde)

---

## Question

Vorschau und Editor sitzen beide am rechten Rand und teilen sich die Fläche zeitlich statt räumlich. `Bereich::teilt_flaeche_mit` ist die eine Stelle dieses gegenseitigen Ausschlusses: wird einer sichtbar, weicht der andere. Zwei der fünf Schalter können damit nie zugleich an sein.

Fünf gleich aussehende Schalter suggerieren fünf unabhängige Ja-Nein-Entscheidungen. Die Frage entscheidet, ob die Leiste den Ausschluss zeigt oder ihn dem Nutzer überlässt, und ob dafür ein zweites Bedienelement gebraucht wird.

## Options

1. **Zwei gewöhnliche Schalter; das Einschalten des einen schaltet den anderen sichtbar aus.** Der Nutzer sieht den Wechsel in der Leiste, weil der andere Schalter im selben Moment umspringt.
   - Pros: Kein neues Bedienelement. Genau das tun die Tastenbefehle heute schon, und die Leiste zeigt es lediglich an. Der Ausschluss bleibt an seiner einen Stelle im Modell und wandert nicht in die Oberfläche.
   - Cons: Der Nutzer erfährt die Regel erst, wenn er sie auslöst.
   - **Folgen weiter unten:** Nichts an der Fähigkeit ist zu bauen außer dem Nachziehen beider Schalterzustände nach jedem Umschalten. Der Anwendungsdelegierte vergleicht die Sichtbarkeit ohnehin schon vor und nach einem Aufruf, um zu erfahren, welche Bereiche er bewegt hat.

2. **Ein dreiwertiger Schalter für den rechten Platz: Vorschau, Editor, keins von beidem.**
   - Pros: Der Ausschluss ist ablesbar, bevor der Nutzer ihn auslöst. Vier Bedienelemente für fünf Bereiche, und die Zuordnung stimmt.
   - Cons: Ein zweites Bedienelement mit einer eigenen Bedienlogik. Die bestehenden Tastenbefehle bilden es nicht ab: `vorschau_umschalten` ist zweiwertig, und der Editor hat gar keinen Umschalter, sondern zwei Einstiege und ein Schließen. Die Leiste und die Tastatur führten dann zwei verschiedene Modelle derselben Sache.
   - **Folgen weiter unten:** Entweder die Tastenbefehle werden auf den dreiwertigen Schalter umgestellt, was C3 und die Belegungsansicht berührt, oder es entstehen zwei Wahrheiten über den rechten Platz.

3. **Zwei Schalter, der jeweils andere ausgegraut.** Steht die Vorschau, nimmt der Editorschalter keinen Klick an, und umgekehrt.
   - Pros: Der Ausschluss ist sichtbar, ohne dass ein neues Bedienelement entsteht.
   - Cons: Der Nutzer muss erst ausschalten, bevor er einschalten kann. Zwei Klicks für einen Wechsel, den die Tastatur mit einem Anschlag macht, und das widerspricht der Maxime "supersimpel".
   - **Folgen weiter unten:** Der Weg über die Leiste wird umständlicher als der über die Tastatur, obwohl beide dieselbe Handlung auslösen sollen.

## Constraints

- Der gegenseitige Ausschluss steht an **einer** Stelle, `Bereich::teilt_flaeche_mit`, und die Beziehung ist symmetrisch; die Probe `der_ausschluss_ist_gegenseitig` hält das fest.
- Ein verdrängter Editor verliert nichts: der Wechsel der Sichtbarkeit fasst das `Editormodell` nicht an. Deshalb geht dem Einblenden der Vorschau seit dem Nutzerentscheid vom 260810-0250 keine Nachfrage voraus.
- Die Leiste und die Tastatur lösen dieselbe Handlung aus und dürfen kein zweites Modell davon führen.

## Recommendation

**Möglichkeit 1.** Sie braucht kein neues Bedienelement, hält den Ausschluss an seiner einen Stelle und zeigt ihn dort, wo der Nutzer ihn ohnehin bemerkt: im Moment des Umschaltens springen beide Schalter. Möglichkeit 2 ist die einzige, die die Regel vorab anzeigt, und kostet dafür ein zweites Bedienmodell neben der Tastatur.


## Antwort 260812-0306

**Moeglichkeit 1: zwei gewoehnliche Schalter; das Einschalten des einen schaltet den anderen
sichtbar aus.**

Der gegenseitige Ausschluss bleibt an seiner einen Stelle, `Bereich::teilt_flaeche_mit`, und die
Leiste zeigt ihn im Moment des Umschaltens: beide Schalter springen zugleich. Genau das tun die
Tastenbefehle heute schon; die Leiste bildet es ab und fuehrt kein zweites Modell daneben.

Moeglichkeit 2, ein dreiwertiger Schalter, ist abgelehnt: die Tastenbefehle bilden ihn nicht ab
(`vorschau_umschalten` ist zweiwertig, der Editor hat zwei Einstiege und ein Schliessen), und es
entstuenden zwei Wahrheiten ueber den rechten Platz. Moeglichkeit 3, den jeweils anderen
auszugrauen, kostet zwei Klicks fuer einen Wechsel, den die Tastatur mit einem Anschlag macht,
und widerspricht der Maxime "supersimpel".

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0306` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung des Circles; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented: 0342445 — zwei gewoehnliche Ankreuzfelder; der Ausschluss bleibt allein in `Bereich::teilt_flaeche_mit`, und `bereichsleiste_nachziehen` laesst beide Schalter zugleich umspringen.
Deferred:
Superseded by:
