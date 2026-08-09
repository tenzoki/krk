S25 Sichern schriebe den Plattenstand, weil die Rückschreibung erst S26 baut

---

Gefunden am 260809-2148 bei der Ursachensuche zu
`issues/260809-2029_c_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`.
Nicht behoben, weil er außerhalb des Zuschnitts jener Behebung liegt.

## Der Befund

**`Editormodell::bearbeiten` hat im ganzen `crates/krk-ui/src/appkit/` keinen
Aufrufer.** Gemessen:

```sh
grep -rn "bearbeiten(\|textDidChange\|NSTextViewDelegate" crates/krk-ui/src/appkit/
```

findet nichts. Was der Nutzer in die Textfläche tippt, steht deshalb allein in
der `NSTextView` und niemals in `Editormodell::stand`. Der Weg von der Fläche
ins Modell ist der Delegierte `textDidChange:`, und den baut **S26**
(`planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritt 26:
„Die Textfläche meldet ihre Änderungen über den Delegierten `textDidChange:`").

## Warum das den Plan trifft

**S25 (Sichern) hängt an S24 und steht damit vor S26.** S25 schreibt nach
seiner eigenen Beschreibung „den Stand des Editors" in die Datei, also
`Editormodell::stand` über `krk_core::text::datei::sichern`
(`editormodell.rs::sichern`). Ohne die Rückschreibung aus S26 ist dieser Stand
der **Plattenstand**, den `uebernehmen` beim Öffnen eingesetzt hat.

Wird S25 in der geplanten Reihenfolge gebaut, tut `cmd+s` also Folgendes: es
schreibt den unveränderten Dateiinhalt zurück in die Datei, meldet
`Sicherungsausgang::Gesichert`, löscht die Abweichungsmarke und zieht den
Stempel nach. Der Nutzer bekommt eine gelungene Sicherung gemeldet, und seine
Änderung ist nicht in der Datei. **Das ist schlimmer als der Defekt von
260809-2029**, denn dort blieb das Getippte wenigstens sichtbar in der Fläche
stehen; hier bestätigt KRK ausdrücklich eine Sicherung, die nichts gesichert
hat.

`inference:` Der Schluss ist am Code gezogen und nicht am laufenden Bündel
gemessen — S25 ist noch nicht gebaut, es gibt also nichts zu messen.

## Was zu tun ist

Die Wahl gehört dem Planer oder dem Nutzer, nicht dem Bearbeiter dieses
Datensatzes. Zwei Wege liegen nahe:

1. **Die Rückschreibung aus S26 in S25 vorziehen.** Sie ist der kleinere Teil
   von S26 (der Delegierte und `bearbeiten`); die Anzeige des ungesicherten
   Standes, der zweite Teil von S26, kann bleiben, wo sie ist. S25 bekäme damit
   die Abhängigkeit „S24 **und** die Rückschreibung".
2. **S26 vor S25 ziehen.** Dann steht die Abweichungsmarke, bevor irgendetwas
   sie liest. S26 hängt heute an S25 allein wegen des Satzes „nach einem
   Sichern verschwindet das Kennzeichen"; das ist eine Abnahme und keine
   Bauabhängigkeit.

Ein dritter Weg — S25 den Stand beim Sichern selbst aus der Textfläche holen
lassen — legte eine zweite Rückschreibung neben die aus S26 und widerspräche
`editormodell.rs`, das genau zwei Eingänge für fremden Text führt.

## Was daran hängt

Auch die vier Anlässe aus C4 (S28, S29) fragen `hat_ungesicherten_stand`. Ohne
die Rückschreibung ist die Antwort immer `false`, und die Nachfrage käme nie —
die Kette S27/S28 wäre gebaut und stumm. Die Rückschreibung ist damit nicht ein
Stück von S26, sondern die Voraussetzung von S25 bis S29.

**Aufgefallen bei:** der Ursachensuche zum Defekt von 260809-2029, am
Übersetzungsstand `111c72e`.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (Schritte 25 bis 29),
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-2029_c_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`

---
Resolved: Weg 2 des Datensatzes gegangen — S26 steht vor S25. Am 260809-2322
sind S24 und S26 zusammen umgesetzt worden, weil beide dieselben zwei Dateien
anfassen. `Editormodell::bearbeiten` hat seinen Aufrufer bekommen: der
Delegierte `textDidChange:` in `crates/krk-ui/src/appkit/editor.rs`
(`Editorbereich::text_zurueckschreiben`) holt den Stand aus der `NSTextView`,
führt ihn durch `krk_core::text::datei::in_gehaltene_form` und setzt die
Abweichungsmarke. `hat_ungesicherten_stand` kann damit wahr werden, und S25
schreibt beim Sichern das Getippte statt des Plattenstandes.

Der dritte, verworfene Weg ist nicht gegangen worden: es gibt weiterhin genau
zwei Eingänge für fremden Text, und das Sichern holt nichts aus der Fläche.

Der ungemessene Preis dieser Bauart — der ganze Stand je Tastendruck — steht
jetzt als eigener Datensatz da:
`issues/260809-2322_o_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`.
