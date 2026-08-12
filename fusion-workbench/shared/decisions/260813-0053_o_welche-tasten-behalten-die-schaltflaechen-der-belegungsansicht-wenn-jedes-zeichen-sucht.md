# Welche Tasten behalten die Schaltflächen der Belegungsansicht, wenn jedes Zeichen in die Suche geht?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` (C1), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md` (C3), `crates/krk-ui/src/appkit/belegungsansicht.rs:483-520`

---

## Frage

Der Nutzerwunsch vom 260813 verlangt, dass in der Belegungsansicht jede Zeicheneingabe an einen Suchtext angehängt wird und die Eingabetaste zum nächsten Treffer springt. Beide Tasten sind heute vergeben: die Leertaste löst die Schaltfläche „Zuweisen" aus, die Eingabetaste die Schaltfläche „Fertig" (`belegungsansicht.rs:483` und `:519`, gesetzt über `NSButton::setKeyEquivalent` beziehungsweise `Schaltflaeche::neu("Fertig", Taste::Eingabe)`). Die Leertaste ist ein Zeichen, das in einem Funktionsnamen vorkommt: wer „spalte datum" sucht, tippt sie. Beide Tasten müssen die Ansicht behalten oder abgeben, und ohne die Antwort ist C3 der Runde 1 in seiner Bedienung nicht zugeschnitten.

Cmd+R für „Auslieferungszustand" und `esc` für das Verlassen sind nicht betroffen: die eine trägt eine Zusatztaste, die andere ist kein Zeichen.

## Möglichkeiten

1. **Die Suche nimmt Leertaste und Eingabetaste; beide Schaltflächen ziehen auf Kombinationen mit Zusatztaste um.** „Zuweisen" bekommt Cmd+T, „Fertig" bekommt Cmd+Eingabe über den vorhandenen Wert `Taste::EingabeMitBefehl` (`appkit/blaetter/mod.rs:280`).
   - Dafür: Der Suchtext nimmt jedes Zeichen, das ein Funktionsname trägt, und die Regel hat keine Ausnahme. Beide Umzüge greifen auf Vorhandenes zu, `Taste::EingabeMitBefehl` steht seit der Editor-Runde und wird bisher von keinem Blatt genutzt.
   - Dagegen: Zwei eingeübte Tasten der Belegungsansicht wechseln ihre Bedeutung. Wer die Ansicht kennt, drückt die Leertaste und sucht danach nach einem Leerzeichen.
2. **Die Leertaste bleibt bei „Zuweisen"; die Suche nimmt sie nicht.** Die Eingabetaste geht an die Suche, „Fertig" zieht auf Cmd+Eingabe um.
   - Dafür: Die eingeübte Bedienung von C3 bleibt zur Hälfte stehen.
   - Dagegen: Der Suchtext kann kein Leerzeichen tragen, und damit ist jeder mehrwortige Funktionsname nur über sein erstes Wort erreichbar. Von den 81 Funktionen tragen die meisten Namen aus zwei bis vier Wörtern. Die Suche verfehlt genau die Fälle, für die der Nutzer sie bestellt hat.
3. **Die Suche ist ein Modus, in den ein eigener Befehl führt.** Vor dem ersten Zeichen drückt der Nutzer etwa Cmd+F; darin gehören alle Zeichen der Suche, davor allen Schaltflächen.
   - Dafür: Keine Taste wechselt ihre Bedeutung.
   - Dagegen: Der Nutzerwunsch sagt ausdrücklich „jede eingabe eines zeichens", also ohne Vorlauf. Ein Modus wäre eine zweite Bedienlage in einem Blatt, das schon eine hat (die Aufnahme über `nimmt_auf`), und die Maxime „supersimpel" spricht dagegen.

## Randbedingungen

- Die Schaltflächen der Belegungsansicht liegen außerhalb von `resources/default-keymap.toml`, und das bleibt so. Cmd+R trägt dort `sortierrichtung_umkehren`, die Leertaste `markierung_umschalten`; beide begegnen einander nie, weil ein stehendes Blatt jeden Befehl außer dem Abbruch anhält. Das ist die Lage, die der Nutzerentscheid vom 260805-0713 als Nicht-Konflikt bestimmt hat, und Cmd+T (heute `tab_neu`) fällt unter dieselbe Regel.
- `esc` behält seine zwei vorhandenen Bedeutungen: während der Aufnahme bricht es sie ab, sonst verlässt es die Ansicht und sichert. Eine dritte Bedeutung „Suchtext löschen" kommt nicht hinzu; gelöscht wird mit der Rücktaste.

## Empfehlung

Möglichkeit 1. Der Preis von Möglichkeit 2 fällt genau dort an, wo der Nutzen liegt: eine Suche über 81 Funktionsnamen, die kein Leerzeichen aufnimmt, findet die mehrwortigen Namen nicht, und das sind fast alle. Möglichkeit 3 widerspricht dem Wortlaut des Wunsches und legt eine zweite Bedienlage neben die Aufnahme.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:
