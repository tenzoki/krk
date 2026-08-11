# Fragt KRK nach, bevor `return` eine große Zahl von Einträgen an das System übergibt?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:**
`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/decisions/260811-1612_a_oeffnet-return-alle-betroffenen-eintraege-oder-nur-den-unter-der-auswahl.md` (Abschnitt "Was mit dieser Antwort nicht entschieden ist"),
`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` (C3),
`crates/krk-ui/src/kommandos/operationen.rs:157` (`betroffene()`),
`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs` (die eine bestehende Rückfrage vor einer Mengenwirkung)

---

## Question

Der Nutzer hat am 260811-1610 entschieden, dass `return` **alle** betroffenen Einträge an das Standardprogramm des Systems gibt. Der Datensatz `260811-1612_*` hält im selben Zug fest, was damit **nicht** entschieden ist: ob KRK bei einer großen Zahl betroffener Einträge nachfragt, bevor es übergibt. Der Spec sagt dazu nichts zu, und der Umsetzungsplan vom 260811-1648 baut deshalb keine Schwelle.

Die Frage ist erheblich und sie wird es mit dem Gebrauch. Dreißig markierte Dateien heißen dreißig Aufrufe von `NSWorkspace::openURL:`, und je nach Bestand starten daraufhin mehrere Programme gleichzeitig. Rückgängig gibt es dafür nicht, und der Tastendruck ist eine einzelne Eingabetaste, also die Taste, die auf dem Mac sonst eine Vorgabeschaltfläche auslöst.

**Der Öffner ist zugleich der zweite Abnehmer von `betroffene()` ohne Rückfrage.** Bis zu dieser Runde hatten alle vier Abnehmer der Regel eine Rückfrage oder eine Umkehrbarkeit vor sich: Kopieren und Verschieben sind umkehrbar, der Papierkorb ist es, und das endgültige Löschen fragt nach und nennt dabei die Zahl. Der Pfadkopierer aus C2 wirkt ohne Rückfrage, richtet aber nichts an. Der Öffner wirkt ohne Rückfrage und startet Programme.

## Options

1. **Keine Schwelle, dauerhaft.** `return` übergibt, was betroffen ist, und die Statuszeile nennt danach die Zahl.
   - Pro: eine Regel im ganzen Haus, kein Zahlenwert, den jemand setzen müsste. Finder und ForkLift verhalten sich so.
   - Contra: ein Tastendruck kann viele Programme starten; die Rückmeldung kommt, nachdem es geschehen ist.
2. **Eine Rückfrage ab einer festen Zahl**, nach dem Vorbild der Rückfrage vor dem endgültigen Löschen.
   - Pro: die teure Handlung wird sichtbar, bevor sie eintritt; das Blatt und sein Weg stehen im Baum.
   - Contra: eine gesetzte Zahl ohne Messung. Dieses Projekt hat damit schlechte Erfahrungen gemacht, und der Datensatz `260811-1612_*` sagt es ausdrücklich. Eine zweite Regel neben `betroffene()` entstünde damit nicht, wohl aber eine zweite Regel neben "der Befehl wirkt sofort".
3. **Eine Rückfrage ab einer Zahl, die der Nutzer in `settings.toml` setzt**, mit einer Vorbelegung.
   - Pro: die Zahl setzt der, den sie betrifft; die Einstellungsdatei trägt bereits die Bündelkennung des Terminals und ist der vorhandene Ort dafür.
   - Contra: eine Einstellung mehr für einen Fall, den vielleicht niemand trifft. Eine Vorbelegung ist wieder eine gesetzte Zahl, nur mit einem Ausweg.

## Constraints

- Die Antwort darf `betroffene()` nicht antasten. Die Regel, worauf ein Befehl wirkt, steht einmal im Baum und bleibt dort.
- Eine Rückfrage ist ein Blatt, und ein stehendes Blatt hält jeden Befehl außer dem Abbruch an (`crates/krk-ui/src/kommandos/operationen.rs:208`). Wer sie baut, baut den vierten oder fünften Anlass eines Blattes und erbt dessen Bedienregeln.
- Ohne eine gemessene Größe ist jede Zahl gesetzt. Eine Messung, die trüge, müsste sagen, ab wie vielen gleichzeitig gestarteten Programmen das Referenzgerät spürbar einbricht; diese Runde fährt keine Messstrecke.

## Recommendation

Keine Empfehlung. Die Frage ist eine Verhaltensfrage und gehört dem Nutzer; der Planner legt sie ab, damit sie im Bestand auffindbar ist, statt allein im Rumpf eines beantworteten Datensatzes zu stehen.

Wer sie später aufgreift, sollte zwei Dinge dazunehmen: dass der Öffner der erste Abnehmer von `betroffene()` ist, der ohne Rückfrage etwas außerhalb von KRK auslöst, und dass die Flüchtigkeit der Markierung (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md`) den Fall in beide Richtungen berührt: eine gefallene Markierung öffnet zu wenig, eine stehende womöglich zu viel.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Keine Nachfrage.** Nutzerantwort am 260811-1710. `return` gibt die betroffenen
Eintraege ohne Zwischenschritt an das System, wie Finder und ForkLift es tun. Eine Regel, keine
Schwelle.

**Der Preis ist benannt und angenommen:** fuenfzig markierte Dateien starten fuenfzig Programme,
und Rueckgaengig gibt es dafuer nicht. Was den Fall entschaerft, steht ohnehin im Spec: die
Markierung faellt mit jedem Lesevorgang, eine grosse Markierung ist also selten und kurzlebig,
und die Rueckmeldung in der Statuszeile nennt die Zahl.

**Warum keine Schwelle.** Eine Schwelle waere eine Zahl, die jemand setzen muesste, ohne sie
gemessen zu haben. Dieses Projekt hat damit schlechte Erfahrungen: die Zusage L9 ist am 260807
zweimal an einem Tag nachgezogen worden, beide Male aus je einem Lauf abgeleitet, und die Frage,
ob sie wieder angehoben wird, liegt bis heute zurueckgestellt
(`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`).

**Der Weg zurueck steht offen und braucht dann keine geratene Zahl.** Faellt im Betrieb auf, dass
ein versehentliches `return` stoert, ist die Schwelle danach eine gemessene Groesse: man weiss
dann, bei welcher Zahl es zum ersten Mal weh tat.
