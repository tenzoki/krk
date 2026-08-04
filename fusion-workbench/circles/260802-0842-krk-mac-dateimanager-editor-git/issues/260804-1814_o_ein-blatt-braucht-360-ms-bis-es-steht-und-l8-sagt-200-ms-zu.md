Ein Blatt braucht 360 ms, bis es steht, und L8 sagt 200 ms zu

---

Die 150-ms-Regel hält genau: KRK legt das Fortschrittsblatt 152 bis 154 ms nach dem Start der Operation an. Bis macOS das Blatt angehängt hat, vergehen weitere rund 360 ms. Das Abnahmekriterium "eine Kopie von 5.000 Einträgen zeigt binnen 200 ms einen Fortschritt" ist damit an der Oberfläche nicht zu halten, solange der Fortschritt ein Sheet ist.

---

## Die Messung

Am laufenden Bündel am 260804, Prüfordner mit 5.000 Einträgen unter `/tmp`, Kopie auf denselben APFS-Datenträger, dreimal wiederholt:

| Was | Zahl |
|---|---|
| Blatt angelegt, nach dem Start des Vorgangs | 152, 152, 153, 154 ms |
| `NSWindow.attachedSheet` meldet das Blatt, nach dem Tastendruck | 465, 466, 467, 472 ms |

Der Anteil, den KRK verantwortet, ist die erste Zeile. Die zweite enthält zusätzlich die Zeit, die macOS braucht, um das Blatt herunterzufahren. Diese Spanne ist getrennt gemessen worden, an der Rückfrage vor dem endgültigen Löschen, die ohne jeden Verzug aufgeht: **354, 361 und 403 ms** vom Tastendruck bis `attachedSheet`. Ein Sheet kostet also rund 360 ms, gleich was es zeigt.

## Was daran zu entscheiden ist

Drei Wege, ohne Empfehlung:

1. **Die Zusage anders lesen.** L8 aus C8 sagt "Fortschritt sichtbar, 200 ms nach Start". Wenn "sichtbar" heißt, dass KRK das Blatt in Auftrag gegeben hat und die Einblendung läuft, hält die Zusage. Der erste Pixel des Blattes erscheint vor dem Ende der Einblendung; wann genau, ist mit den Mitteln dieses Projekts nicht messbar.
2. **Den Verzug verkürzen.** 150 ms auf, sagen wir, 40 ms herunterzunehmen brächte das angehängte Blatt auf rund 400 ms. Die 200 ms erreicht das nicht, und die Zusage "eine kleine Kopie lässt kein Fenster aufblitzen" wäre schwächer.
3. **Den Fortschritt nicht als Blatt zeigen.** Eine Zeile im Dateifenster, etwa in der Statuszeile aus C1, erscheint mit dem nächsten Zeichendurchgang und ohne Einblendung. Sie kostet die Modalität: der Abbruch bräuchte eine eigene Taste, und die Oberfläche bliebe während der Operation bedienbar, was C4 ausdrücklich zusagt ("Während eine länger laufende Operation arbeitet, bleibt die Oberfläche bedienbar"). Der Plan hat sich in S16 für ein Blatt entschieden.

Weg 3 hängt mit dem Defekt `260804-1814_o_ein-modales-blatt-widerspricht-der-zusage-dass-die-oberflaeche-bedienbar-bleibt.md` zusammen und ist wahrscheinlich zusammen mit ihm zu entscheiden.

**Aufgefallen bei:** der Umsetzung von Schritt 16 am 260804-1814.

---

## Stand 260804-1832: Weg 3 gewählt, L8 bleibt bei 200 ms

Der Nutzer hat Weg 3 gewählt, den Fortschritt nicht als Blatt zu zeigen. Datensatz: `decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`.

Die Rechnung dahinter: KRKs eigener Anteil von 152 bis 154 ms bleibt unverändert, weil die 150-ms-Regel unverändert bleibt. Weg fallen die rund 360 ms für das Anhängen des Blattes. Eine Zeile in der Statuszeile erscheint mit dem nächsten Zeichendurchgang, am Referenzgerät also rund 17 ms nach dem Setzen. L8 liegt damit bei rund 170 ms. **Eine andere Zahl war nicht nötig; die Zusage bleibt bei 200 ms, und die neun übrigen Zahlen aus C8 sind unberührt.**

Weg 2, den Verzug auf rund 40 ms herunterzunehmen, ist damit gegenstandslos und wird nicht gegangen: die 150 ms tragen weiterhin die Zusage, dass eine kleine Kopie nichts aufblitzen lässt.

Nebenbei löst die Wahl das Messproblem, das dieser Datensatz unter Weg 1 benennt. Wann der erste Bildpunkt eines Blattes erscheint, war mit den Mitteln dieses Projekts nicht messbar. Eine Zeile wird ab S16b auf demselben Weg gemessen wie L1, L5, L6 und L7, nämlich vom Ereigniszeitstempel bis zum Ende des Zeichendurchgangs.

**Geschlossen wird dieser Defekt mit der Umsetzung von S16b.**
