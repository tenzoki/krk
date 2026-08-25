Die Summenprobe der sechs Zustandszeilen fällt schon an einem `.DS_Store`

---

`resources/default-readers.toml:381-382` gibt dem Nutzer eine Probe an die Hand: die sechs
Zustandszeilen des Profils „alle Runden" seien „überschneidungsfrei und vollständig; ihre
Summe geht gegen die Zeile ‚Runden‘ auf, und ein siebter Marker würde als Differenz sichtbar."
Die Zeile „Runden" (`:393`) zählt jedoch **alle** Einträge in `circles/`, gleich welchen Typs,
während die sechs Zustandszeilen über `ordner = "*"` allein Unterordner sehen. Jeder
Fremdeintrag in `circles/` erzeugt damit dieselbe Differenz wie ein siebter Marker.

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:368-421`;
`crates/krk-core/src/leseprofil/mod.rs` (`Ortsangabe`, „Was er greift und was nicht: allein
Eintraege vom Typ `Typ::Ordner`")

## Was gemessen ist

Gemessen am 260825-2126 über `leseprofil::zusammenfassen_gezaehlt`, Baum `8478753`.

An der Werkbank dieses Projekts geht die Summe auf:

```
Runden 19  =  Vorgesehen 0 + Aktiv 0 + Kohärent 5 + Beschränkt 12 + Überholt 0 + Zurückgestellt 2
```

An einem künstlichen `circles/` mit zwei Runden, einem `.DS_Store` und einer `NOTIZ.md`:

```
Runden 4   =  0 + 0 + 1 + 1 + 0 + 0   →  Differenz 2
```

Nach Entfernen der zwei Fremdeinträge steht dort `Runden 2` und die Summe geht auf.

## Warum das zählt

Der Fall ist auf dem Bauziel dieser Anwendung kein gedachter. `.DS_Store` legt der Finder in
jeden Ordner, den er anzeigt, und KRK trägt seit der Runde 17 den Kontextmenüeintrag „Im
Finder öffnen". Ein einziger Blick in den Ordner `circles/` genügt.

Die Folge ist keine falsche Zahl, sondern eine falsche Lesart: der Kommentar sagt dem Nutzer,
eine Differenz bedeute einen Marker, den keine Zeile führt. Sie kann auch bedeuten, dass eine
Datei im Verzeichnis liegt. Zwei Ursachen, eine Anzeige, und die Anleitung nennt nur eine.

Dasselbe gilt in kleinerem Maß für die Zeile „Läufe" des Ablagespeichers (`:439`), die
ebenfalls ohne Muster zählt; dort steht allerdings keine Summenprobe daneben.

## Möglichkeiten

1. **Der Kommentar nennt die zweite Ursache.** Ein Halbsatz, dass die Zeile „Runden" alle
   Einträge zählt und eine Differenz deshalb auch von einer Datei im Verzeichnis kommen kann.
   Kein Verhalten ändert sich.
2. **Die Zeile „Runden" bekommt ein Muster**, das Fremdeinträge ausschließt. Es gibt keines,
   das den Typ trifft: `zaehlung` sieht auf Namen, nicht auf Typen. Ein Muster wie
   `'^[0-9]{6}-[0-9]{4}-'` träfe die Namensform der Rundenverzeichnisse und wäre eine zweite
   Stelle, an der die Namenskonvention der Werkbank steht.
3. **Der Mechanismus bekommt eine Typangabe an `zaehlung`.** Die teuerste, und sie berührt die
   Festlegung A7 über die vier Bausteine.

**Schwere:** niedrig. Kein Bau hängt daran; die Zahlen bleiben richtig, ihre Auslegung nicht.
