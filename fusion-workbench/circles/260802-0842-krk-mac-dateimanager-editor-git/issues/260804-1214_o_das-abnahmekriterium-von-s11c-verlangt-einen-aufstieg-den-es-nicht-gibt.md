Das Abnahmekriterium von S11c verlangt einen Aufstieg, den es in dieser Runde noch nicht gibt

---

Der letzte Satz des Abnahmekriteriums von Schritt 11c lautet (`planning/260802-1428_o_plan-navigator-geruest-runde-1.md:677`):

> Im gebauten Bündel steigt `cmd+right` in den ausgewählten Ordner ein, `cmd+left` und `cmd+up` steigen auf, und `return` löst nichts aus.

Die zweite Hälfte kann dieser Schritt nicht erfüllen. **Das Kommando für den Aufstieg gibt es nicht.** Die Aufzählung `Kommando` in `crates/krk-core/src/tasten/belegung.rs:80-114` führt 16 Werte, und keiner davon ist der Aufstieg in den übergeordneten Ordner; `Kommando::KENNUNGEN` (ebenda, Zeile 119-139) kennt die Kennung `ordner_aufwaerts` folglich auch nicht. `Kommando::aus_kennung("ordner_aufwaerts")` liefert `None`, und `behandeln` reicht den Tastendruck dann weiter, statt ihn zu schlucken.

Nach S11c drücken `cmd+left` und `cmd+up` also ins Leere. Gebaut wird der Aufstieg erst mit S13, der die Kommandos aus C2 nachträgt.

---

## Was der Schritt sehr wohl erfüllt

- `cmd+right` steigt ein: `Kommando::Oeffnen` steht in der Aufzählung und ist in `crates/krk-ui/src/appkit/tabelle.rs:460` an `auswahl_oeffnen` verdrahtet.
- `return` löst nichts aus: es steht in keiner Tastenliste mehr.

Die beiden Bereichsbreiten, die derselbe Schritt auf die Pfeile gelegt hat, wirken ebenfalls: `Kommando::BereichVerbreitern` und `Kommando::BereichVerschmaelern` stehen in der Aufzählung und hängen in `crates/krk-ui/src/appkit/anwendung.rs:403-404` an `breite_aendern`. `ctrl+right` und `ctrl+left` sind damit ab sofort wirksam.

## Warum das kein Baufehler ist

Der Plan sagt die Lage an zwei anderen Stellen richtig an. S11c selbst hält bei `f1` fest, eine belegte Taste ohne Kommando sei folgenlos, weil `aus_kennung` dann `None` liefert (`planning/...:671`); dieselbe Regel deckt `ordner_aufwaerts` ab. Und S13 nennt den Aufstieg ausdrücklich als seinen Gegenstand. Falsch ist allein die Abnahmezeile von S11c: sie prüft am gebauten Bündel etwas ab, das erst zwei Schritte später entsteht.

## Was zu tun ist

Den letzten Satz des Abnahmekriteriums von S11c auf das ziehen, was der Schritt leisten kann, etwa: "Im gebauten Bündel steigt `cmd+right` in den ausgewählten Ordner ein, `ctrl+left` und `ctrl+right` ändern die Breite des aktiven Bereichs, und `return` löst nichts aus. `cmd+left` und `cmd+up` sind belegt und folgenlos, bis S13 das Kommando für den Aufstieg baut."

Die Änderung gehört dem `planner`: sie fasst eine Plandatei an, und der Auftrag zu S11c schließt den Eingriff dort aus.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11c am 260804-1214, beim Nachprüfen der Aufzählung `Kommando` gegen die drei neuen Belegungen.
