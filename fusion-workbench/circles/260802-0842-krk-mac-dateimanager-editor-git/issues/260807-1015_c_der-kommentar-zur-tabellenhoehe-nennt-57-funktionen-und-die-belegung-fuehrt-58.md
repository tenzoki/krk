Der Kommentar zur Tabellenhöhe nennt 57 Funktionen und die Belegung führt 58

---

**Domain:** code
**Filed by:** ontocoder (beim Nachtrag von `fokus_vorschau`)
**Für:** coder
**Cross-references:** `crates/krk-ui/src/appkit/belegungsansicht.rs:76`,
`resources/default-keymap.toml`,
`issues/260807-0922_*_das-kommando-fokus-vorschau-steht-im-code-und-noch-nicht-in-der-auslieferungsbelegung.md`

---

Der Kopfkommentar der Konstanten `TABELLENHOEHE` sagt "57 Funktionen und neun
Bereichsueberschriften brauchen einen Rollbalken". Seit dem Nachtrag von
`fokus_vorschau` führt `resources/default-keymap.toml` 58 Funktionen mit
zusammen 65 Kombinationen; die Zahl neun stimmt weiter.

**Kein Rechenfehler und kein Verhaltensfehler.** Die Konstante steht auf
300,0 Punkten und hängt von der Zahl der Zeilen nicht ab; der Kommentar sagt
das selbst ("die Zahl hier bestimmt nur, wie viele Zeilen ohne Rollen sichtbar
sind"). Die Ansicht rollt vorher und rollt nachher. Falsch ist allein die
genannte Zahl.

**Der Nachtrag.** In `crates/krk-ui/src/appkit/belegungsansicht.rs:76` die 57
durch 58 ersetzen.

`crates/krk-ui/` gehört dem `coder`; der `ontocoder` hat die Zeile deshalb
stehen lassen. Eine dritte Stelle mit derselben Zahl gibt es im Baum nicht,
geprüft am 260807 über `grep -rn "57 Funktionen"` ohne `target/` und `.git/`.

## Dringlichkeit

Niedrig. Ein Kommentar, keine Zusage, keine der zehn Zeitzusagen aus C8 ist
berührt. Der Wert läuft mit jedem weiteren Belegungsnachtrag weiter
auseinander, und die Zahl in einem Kommentar zu führen ist selbst der Grund
dafür.

---
Resolved:
 Der Kommentar der Konstanten `TABELLENHOEHE`
(`crates/krk-ui/src/appkit/belegungsansicht.rs:76-86`) nennt gar keine Zahl der
Funktionen mehr, statt 57 durch 58 zu ersetzen. Der Nachtrag haette den Wert
richtiggestellt und die Ursache stehen lassen: die Konstante steht auf 300,0
Punkten und haengt an der Zahl der Zeilen nicht, wie der Kommentar selbst sagte.
Er sagt jetzt, was nachpruefbar ist und nicht altert — die Tabelle fasst fuenfzehn
Zeilen zu je `ZEILENHOEHE`, die Belegung ist laenger, also rollt die Ansicht —
und haelt in einem zweiten Absatz fest, warum die Zahl nicht wieder dort steht.
Kein Verhalten beruehrt. `make check` gruen, 525 Pruefungen.
