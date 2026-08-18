# Die bestätigende Seite des Eingabewächters liegt fest auf der ersten Schaltfläche

**Datum:** 260817-1242
**Gefunden von:** coder, T1
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/appkit/blaetter/mod.rs`, `Blatt::zeigen_mit_wahl`
**Baumstand:** der Stand nach T1

## Der Befund

T1 hat die Frage „welche Schaltfläche ist die ungefährliche" auf eine Antwort gebracht
(`blaetter::abbruchstelle`). Die Gegenfrage steht weiter fest im Code: der
`Eingabewaechter` schickt für `bestaetigt == true` unverändert `NSAlertFirstButtonReturn`,
also die **erste** Schaltfläche.

Heute ist das richtig, und der Grund ist nachgezählt: einen Wächter hält nur ein Blatt aus
`Blatt::neu`, dessen Reihenfolge die erste Schaltfläche als die bestätigende festlegt. Die
fünf Rufer von `waechter_anhaengen` und `textfeld_setzen` kommen alle von dort
(`pfadeingabe`, `namenseingabe`, `zeilennummer`, `suche`, `stapelumbenennen`).

Es ist trotzdem dieselbe Sorte Annahme, die der Befund `260817-1106` auf der abbrechenden
Seite gekostet hat: eine Stelle im Rumpf rechnet mit einer Reihenfolge, die ein einzelnes
Blatt anders legen darf. Für die Löschrückfrage wäre die erste Schaltfläche „Abbrechen" —
sie trägt heute keinen Wächter, weil sie kein Feld hat.

## Richtung

Die bestätigende Stelle ist ableitbar, und zwar ohne neue Angabe: der Wächter existiert,
weil der Feldeditor die Eingabetaste verbraucht, und die Eingabetaste gehört der
Schaltfläche mit `Taste::Eingabe`. Eine reine Funktion neben `abbruchstelle`, die die erste
Schaltfläche mit `Taste::Eingabe` liefert und ohne eine solche auf `abbruchstelle`
zurückfällt, macht aus der Annahme eine Ableitung. Zwei Stellen im Baum bekämen damit
Antworten, die zu ihrem Blatt passen statt zu einer Gewohnheit.

Angehängt daran hängt die Frage, die
`260817-1241_o_das-konfliktblatt-gibt-seinem-namensfeld-keinen-eingabewaechter.md` aufwirft:
was ein Wächter in einem Blatt mit vier Antworten bedeuten soll.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, unverändert.**
`Blatt::zeigen_mit_wahl` liegt an `crates/krk-ui/src/appkit/blaetter/mod.rs:667` und trägt an
`:711` weiter das feste `NSAlertFirstButtonReturn`, während die Gegenseite über
`blaetter::abbruchstelle` (`:416`) berechnet wird.

---
Resolved 260818 (coder, Bündel C/D-Nachzug): **die bestätigende Seite ist abgeleitet und
nicht mehr angenommen.**

Gebaut in der Richtung, die dieser Datensatz nennt: `blaetter::bestaetigungsstelle`
(`crates/krk-ui/src/appkit/blaetter/mod.rs`), eine reine Funktion neben `abbruchstelle`, die
die erste Schaltfläche mit `Taste::Eingabe` liefert und ohne eine solche auf `abbruchstelle`
zurückfällt. `Blatt::mit_schaltflaechen` legt das Ergebnis wie die Abbruchstelle einmal je
Blatt ab; der `Eingabewaechter` liest es in `zeigen_mit_wahl`, wo bis dahin ein festes
`NSAlertFirstButtonReturn` stand.

**Warum der Rückfall auf `abbruchstelle` und nicht auf die erste Stelle.** Ein Blatt ohne
Schaltfläche auf der Eingabetaste gibt der Taste keine Bedeutung, und die Rückfrage vor dem
Räumen ist der Gegenfall dazu: sie legt die Eingabetaste ausdrücklich auf „Abbrechen". Ratend
die erste Stelle zu nehmen hieße in einem Blatt mit ausführender erster Schaltfläche, die
Eingabetaste auf den zerstörenden Ausgang zu legen — genau der Fehler von `260817-1106`, nur
auf der anderen Seite. Es ist dieselbe Antwort, die der Abschlussblock schon für eine
unbekannte Antwort gibt: lieber nichts tun als raten.

**Zwei Proben, beide ohne AppKit** (`blaetter::tests`):
- `die_tafel_der_bestaetigenden_stelle` — die Tafel Zeile für Zeile, in der Bauform der
  Nachbarin `die_tafel_der_liegenlassenden_stelle`.
- `die_eingabetaste_im_feld_gehoert_ihrer_eigenen_schaltflaeche` — die eigentliche Zusage,
  gemessen an den drei Bauplänen, die im Baum auseinandergehen: `Blatt::neu` (Eingabetaste
  vorn), Konfliktblatt (in der Mitte) und Löschrückfrage (vorn, aber liegenlassend).

**Nachgewiesen, dass sie den Fehler fängt, gegen den sie gerichtet ist.** Probeweise wieder
auf `0` festgelegt: beide Proben werden rot, und die zweite meldet
`die Eingabetaste faellt auf "Überschreiben", und die traegt sie nicht` — also mit dem Schaden
benannt und nicht mit einer Zahl. Zurückgenommen.

Nachgezogen: der Modulkopf von `blaetter/mod.rs` (der Abschnitt nennt jetzt beide Fragen) und
der Doc-Kommentar von `Blatt::neu`, der sagte, auf seiner Reihenfolge ruhe der Wächter.

Abnahme: `make check` — Exit 0.
