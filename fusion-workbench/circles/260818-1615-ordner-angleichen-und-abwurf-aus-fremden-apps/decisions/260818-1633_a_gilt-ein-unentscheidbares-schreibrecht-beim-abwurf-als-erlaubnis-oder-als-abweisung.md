# Gilt ein unentscheidbares Schreibrecht beim Abwurf als Erlaubnis oder als Abweisung?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `shared/planning/260818-1510_*_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md` §C6 (die drei Lagen der frühen Abweisung); `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md` (die Gegenzusage „Unentschieden gilt als laut"); `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` (Modulkopf, Abschnitt „Warum es die dritte Antwort gibt"); `circles/260817-0833-.../decisions/260818-0249_*_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md` (dieselbe Familie, andere Frage); `/Applications/Xcode.app/…/MacOSX.sdk/System/Library/Frameworks/Foundation.framework/Headers/NSURL.h:247` (`NSURLIsWritableKey`)

---

## Question

Die Runde 13 bringt die erste Schreibrechtsprüfung dieses Baumes. Der Abwurf fragt vor dem Loslassen, ob der Zielordner beschreibbar ist, und weist ab, wenn er es nicht ist (C6, Lage 2). Die Frage hat drei Ausgänge und nicht zwei: der Ressourcenwert `NSURLIsWritableKey` antwortet mit `true`, mit `false`, oder er antwortet gar nicht — weil der Pfad kein gültiges UTF-8 trägt, weil die Abfrage einen Fehler liefert oder weil der Wert fehlt.

**Was mit dem dritten Ausgang geschieht, ist eine Festlegung und keine Ableitung.** Zwei Antworten sind vertretbar, und dieser Baum hat die entgegengesetzte schon einmal gegeben:

- Die Runde 12 hat für den Löschweg zugesagt: **„Unentschieden gilt als laut."** Ein Ziel, das sich nicht einordnen ließ, bekam die laute Rückfrage. Der Grund war die Richtung des Schadens: ein Löschvorgang nimmt weg, und wer nicht weiß, worauf er zielt, fragt lieber einmal zu viel.
- Der Abwurf zeigt in die andere Richtung. Sein Vorgabevorgang ist das Kopieren, also der nicht zerstörerische, und eine Abweisung während des Ziehens ist stumm: der Zeiger zeigt das Verbotszeichen, und der Nutzer bekommt keinen Satz dazu, warum. Ein Ordner, den KRK nicht einordnen kann, wäre damit ohne Erklärung unbenutzbar.

Der Plan dieser Runde geht auf **Möglichkeit 1** und schreibt sie in die Tafel seiner Abwurfregel aus. Der Datensatz steht hier, weil die Wahl über diese Runde hinausreicht: nach ihr trägt der Baum zwei dreiwertige Prüfungen mit entgegengesetzter Behandlung des dritten Wertes, und die nächste Runde, die eine dritte hinzufügt, braucht die Regel, nach der sie sich richtet.

## Options

1. **Unbekannt lässt durch, und der Rest wird nachträglich entschieden.** Nur ein gemessenes `false` weist ab. Ein unbeantwortbares Schreibrecht nimmt den Abwurf an; scheitert der Vorgang danach, erscheint jeder betroffene Eintrag mit seinem Grund in der Abschlussliste, auf demselben Weg, den F5 und F6 heute gehen.
   - Pro: kein Ordner wird ohne Erklärung unbenutzbar. Die Antwort auf „unbekannt" ist dieselbe, die der Spec für seine vierte Lage schon gewählt hat — die verschwundene Quelle wird nachträglich entschieden und nicht vorhergesagt. Ein Weg, kein zweiter.
   - Kontra: der Nutzer sieht in diesem Fall das Kopierzeichen und bekommt danach eine Abschlussliste voller übersprungener Einträge. Der Zeiger hat nichts Falsches behauptet, aber auch nichts Nützliches gesagt.
   - Was sie verbaut: nichts dauerhaft. Der Übergang auf Möglichkeit 2 kostet eine Zeile in der Tafel.

2. **Unbekannt weist ab, wie „Unentschieden gilt als laut".** Der Baum trägt dann eine Regel und nicht zwei.
   - Pro: eine Zusage über alle dreiwertigen Prüfungen, an einer Stelle nachzulesen. Kein Abwurf beginnt, den KRK nicht beurteilen konnte.
   - Kontra: die Abweisung während des Ziehens ist stumm, und die Lage, die sie auslöst, ist für den Nutzer nicht sichtbar. Ein Ordner, dessen Pfad kein gültiges UTF-8 trägt, nimmt dann gar nichts mehr an, ohne dass irgendetwas sagt warum. Die Zusage der Runde 12 hing daran, dass die Abweisung **laut** war; hier wäre sie leise, und damit trägt die Übertragung nicht.
   - Was sie verbaut: den Abwurf in Ordner, die KRK nicht einordnen kann, bis eine spätere Runde eine Meldung dafür baut.

3. **Unbekannt lässt durch und meldet es.** Wie 1, dazu eine Zeile in der Statuszeile beim Loslassen: KRK konnte das Schreibrecht nicht feststellen und hat es trotzdem versucht.
   - Pro: die Ehrlichkeit von 2 mit der Benutzbarkeit von 1.
   - Kontra: eine Meldung für eine Lage, die im Gebrauch praktisch nie eintritt, und ein vierter Ausgang in der Tafel, den keine Probe an echten Daten je erreicht. Der Spec verlangt genau eine Meldung während des Ziehens (C7) und begründet, warum nicht mehr.
   - Was sie verbaut: nichts.

## Constraints

- **Der Zeiger und die Wirkung müssen übereinstimmen.** Das ist die Zusage aus C5 und dem tragenden Datensatz der Runde. Alle drei Möglichkeiten halten sie: keine zeigt an, was nicht geschieht.
- **Die Abweisung während des Ziehens ist stumm**, bis auf die eine Meldung, die C7 für die Zusagedatei verlangt. Das ist keine Wahl dieses Plans, sondern die Form, die AppKit dem Ziehvorgang gibt.
- **Der Abwurf mündet in dieselbe Operationsmaschine wie F5 und F6.** Sie führt eine Abschlussliste mit Gründen je Eintrag; ein zweiter Weg für gescheiterte Einträge entsteht nicht.
- **`NSURLIsWritableKey` steht seit macOS 10.7** und antwortet nach dem EUID, also nach der Kennung, die gleich schreiben wird. Am SDK gelesen, `NSURL.h:247`.

## Recommendation

Möglichkeit 1. Der tragende Grund ist nicht die Bequemlichkeit, sondern dass die Übertragung von „Unentschieden gilt als laut" an ihrer eigenen Bedingung scheitert: jene Zusage kaufte Sicherheit gegen eine **sichtbare** Rückfrage, und hier gäbe es dafür nur ein stummes Verbotszeichen. Eine Regel, die den Nutzer nicht erreicht, schützt ihn nicht, sie hindert ihn bloß. Möglichkeit 3 ist die ehrliche Alternative für den, dem das zu still ist; ihr Preis ist ein vierter Ausgang in der Tafel für eine Lage, die keine Probe an echten Daten erreicht.

---
Answered: Nutzerentscheid am Plan-Gate der Runde 13, 260818 — Moeglichkeit 1: nur ein gemessenes `false` weist ab, ein unbeantwortbares Schreibrecht laesst den Abwurf zu, und gescheiterte Eintraege erscheinen mit Grund in der Abschlussliste des Vorgangs. Ausgeschrieben in der Tafel der Abwurfregel, Schritt 7 von `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_*_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`.
Implemented:
Deferred:
Superseded by:
