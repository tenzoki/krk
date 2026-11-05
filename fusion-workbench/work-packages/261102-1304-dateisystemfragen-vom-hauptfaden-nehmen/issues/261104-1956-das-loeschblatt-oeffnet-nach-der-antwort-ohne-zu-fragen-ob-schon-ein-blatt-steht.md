Das Löschblatt geht nach der Antwort des Fadens auf, ohne zu fragen, ob inzwischen ein anderes Blatt steht
---
Schwere: Mittel. Seit `5cdf39e3`. Die Einzugsmethode fragt `vorgang_laeuft` neu und nicht `blatt_steht`. Vor dem Faden konnte die Lage nicht entstehen, weil zwischen Tastendruck und Blatt keine Ereignisse liefen.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/anwendung.rs:8365` `loeschbefund_einziehen` fragt allein `vorgang_laeuft` neu und ruft `loeschblatt_zeigen`, `:8488`. Dort steht keine Frage nach `blatt_steht` (`:4609`). Am Ende steht `self.blatt_oeffnet(griff)`.

`anwendung.rs:4663` `blatt_oeffnet`, Doc-Kommentar: „Der Schlitz hält genau einen Griff, und ein zweiter darüber nimmt dem sichtbaren Blatt seinen Abbruchweg.“ `anwendung.rs:12233`: „`beginSheet` reiht ein zweites Blatt hinter ein noch hängendes ein“. `blaetter/mod.rs:791` `Blattgriff::steht` vergleicht mit `attachedSheet`. Die übrigen Stellen, die ein Blatt nachholen, warten darauf: `anwendung.rs:4623` nennt Konfliktfrage und Abschlussbericht, die liegen bleiben, solange ein Blatt steht.

**Fehlerbild, aus dem Quelltext abgeleitet, nicht im Programm gefahren.** Auf einer langsamen Freigabe drückt der Nutzer Löschen. Kein Blatt erscheint, und die Ereignisschleife läuft jetzt weiter, also öffnet er „Verbinden…“ oder die Pfadeingabe. Die Antwort kommt.

1. Das Löschblatt reiht sich hinter das sichtbare Blatt, und `offenes_blatt` hält seinen Griff.
2. `esc` im sichtbaren Blatt: der erste Rang von `abbrechen` (`:8863`) findet einen Griff, dessen Blatt nicht steht, und lässt ihn liegen. Der Abbruchweg des sichtbaren Blattes ist weg.
3. Schließt der Nutzer das erste Blatt, leert `blatt_geschlossen` (`:4638`) den Schlitz, sofern dieses Blatt sein Schließen meldet. Das Löschblatt erscheint ohne Griff und ohne erkennbaren Zusammenhang mit dem, was der Nutzer gerade tat.

**Nicht betroffen, geprüft:** Ein versehentliches Bestätigen mit der Eingabetaste löscht nichts. `blaetter/loeschbestaetigung.rs` `schaltflaechen` legt `Taste::Eingabe` auf „Abbrechen“ und `Taste::EingabeMitBefehl` auf die ausführende Schaltfläche. Der Plan regelt `vorgang_laeuft` und den gereisten Gegenstand, und beides stimmt.

**Richtung.** Der Plan sagt im Abschnitt `## Approach`, die Einzugsmethode prüfe neu, was vom heutigen Stand abhängt. `blatt_steht` gehört dazu. Steht ein Blatt, zeigt sie statt des Löschblatts einen Satz in der Statuszeile der gereisten Seite und verwirft die Antwort. Ob stattdessen nachgeholt werden soll wie bei `vorgang_zeichnen`, entscheidet der Ausführer und nennt es.

**Abnahme.** Eine Rumpfprobe oder reine Tafel hält, dass `loeschbefund_einziehen` vor `loeschblatt_zeigen` nach `blatt_steht` fragt, und im Rumpf von `loeschblatt_zeigen` entsteht kein zweiter Griff über einem stehenden Blatt.

---
Resolved: `loeschbefund_einziehen` fragt `blatt_steht` in einem eigenen Zweig vor den beiden Blattstufen der Stufentafel. Steht ein Blatt, zeigt die gereiste Seite den vorhandenen Satz `Text::BlattSteht`, und die Antwort verfällt; die Meldungsstufen und die Sperre `loeschpruefung`, die weiter als erste Handlung fällt, bleiben unverändert. Verworfen statt nachgeholt, weil kein Arbeitsfaden auf die Antwort wartet, die Eingabeblätter ihr Schließen nicht melden und ein späteres Löschblatt ohne Zusammenhang aufginge; der Nutzer drückt die Taste neu. `das_loeschblatt_oeffnet_nie_ueber_einem_stehenden_blatt` hält das am Rumpf. Ungefahren bleibt das Fehlerbild im Programm: Löschen auf einer langsamen Freigabe, währenddessen ein anderes Blatt öffnen, die Meldung in der Statuszeile lesen. Das ist Nutzerarbeit.
