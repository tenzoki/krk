`### Frage 2` des Plans nennt den unwirksamen Mechanismus, nicht den wirksamen

---

Der Programmtext ist am 260803-2025 nachgezogen: die Generationsprüfung je Stapel ist aus `crates/krk-ui/src/appkit/tabelle.rs` entfernt, und der Modulkopf nennt jetzt, was einen Ordnerwechsel mitten im Lesen wirklich trägt. Der Plan sagt an dieser Stelle weiter das Alte. Plan und Programmtext beschreiben damit verschiedene Mechanismen, nur andersherum als vorher.

---

## Die Stelle

`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `### Frage 2`:

> Wer schnell durch Ordner navigiert, hat mehrere Lesevorgänge gleichzeitig unterwegs. Ohne die Nummer bräuchte jeder davon eine eigene Abbruchbehandlung; mit ihr verwirft der Hauptfaden jeden Stapel, dessen Generation nicht mehr die aktuelle ist.

Die Umsetzung hält nie mehr als einen Lesevorgang und liest allein aus dessen Kanal. Der Satz beschreibt eine Prüfung, die es nicht mehr gibt und die, solange es sie gab, nie einen Stapel verworfen hat.

## Was stattdessen dasteht

Der neue Modulkopf von `tabelle.rs` und die Dokumentation von `Ordnermodell::generation` sagen beide dasselbe: `ordner_lesen` lässt den alten `Lesevorgang` fallen, damit fällt sein Empfänger, `Lesevorgang::drop` setzt das Abbruchkennzeichen, und spätestens das nächste `send` scheitert am verschwundenen Empfänger. Der Lesefaden prüft das Kennzeichen vor jedem Systemaufruf und zwischen zwei Stapeln; der Abbruch greift innerhalb von zwei Stapeln. Der alte Lauf wird also beendet und nicht ignoriert.

Die Generationsnummer bleibt und trägt weiterhin zwei Dinge: sie benennt den Lesefaden, und sie sagt dem Modell beim Leeren, zu welchem Lauf sein Inhalt gehört.

## Warum das getrennt gemeldet ist

Der `coder` hat den Programmtext am 260803-2025 im Auftrag geändert und dabei ausdrücklich die Anweisung gehabt, den Plan nicht anzufassen, weil der `planner` zur selben Zeit in derselben Datei arbeitet. Der Nachzug in `### Frage 2` gehört ihm.

**Aufgefallen bei:** der Behebung von `issues/260803-1536_c_die-generationspruefung-kann-nicht-greifen-und-verdeckt-den-wirksamen-mechanismus.md`, gemeldet in der Prüfung von Schritt 6 und 7, `reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
