Das Abnahmekriterium von S16b nennt `cargo xtask messen`, das es nicht gibt

---

Das Abnahmekriterium von Schritt 16b verlangt: "`cargo xtask messen` weist L8 mit dem 95. Perzentil unter 200 ms aus." Diesen Unterbefehl gibt es nicht. `xtask` kennt genau einen: `bundle`. L8 ist damit auf dem im Plan genannten Weg nicht messbar.

---

## Der Befund

`xtask/src/main.rs` verzweigt auf `"bundle"` und auf die Hilfe; jeder andere Unterbefehl ist ein Aufruffehler. Die kopflose Messstrecke heißt `krk-bench messen` und misst das Lesen eines Ordners, nicht eine Dateioperation. Die Frühmessung am laufenden Bündel heißt `krk-bench durchstich` und deckt L1, L2, L3, L4 und L10 ab, nicht L8.

Der Weg, den L8 nach der Umstellung vom 260804-1832 gehen soll, ist derselbe wie bei L1: vom Ereigniszeitstempel bis zum Ende des Zeichendurchgangs, der die Änderung trägt. Dafür braucht es einen Messmodus in der Anwendung, der eine Dateioperation auslöst und die Bildgrenzen mitschreibt. Der Messmodus entsteht in S21, `krk-bench` ist bei S16b ausdrücklich nicht anzufassen, und das Abnahmekriterium von S16b setzt beides voraus.

## Wie L8 in dieser Sitzung stattdessen gemessen wurde

Über eine vorübergehende Sonde in `crates/krk-ui/src/appkit/anwendung.rs`, gebaut wie die von S16 und hinterher vollständig zurückgenommen. Sie hängt denselben `CADisplayLink` ein, den `bildtakt::Zeichenende` für L1 einhängt, nimmt den Zeitstempel unmittelbar vor dem Einreihen des F5-Ereignisses und stoppt an der ersten Bildgrenze, an der der Fortschritt in der Statuszeile steht. 20 Läufe am laufenden Bündel, Prüfordner mit 5.000 Einträgen unter `/tmp`, Kopie auf denselben APFS-Datenträger:

| Kennzahl | Wert |
|---|---|
| kleinster Wert | 154,5 ms |
| Median | 164,7 ms |
| 95. Perzentil | 168,9 ms |
| größter Wert | 169,0 ms |

Die Zusage von 200 ms hält mit rund 31 ms Reserve. **Das Abnahmekriterium selbst gilt trotzdem als nicht erfüllt**, weil der Weg, den es nennt, nicht existiert und die Zahl damit nicht wiederholbar ist, ohne die Sonde erneut zu bauen.

## Was zu tun wäre

Entweder bekommt L8 in S21 einen Messschritt im Messmodus und das Abnahmekriterium von S16b wird dorthin verschoben, oder der Plan nennt den Weg, den es heute gibt. Ein dritter Weg wäre ein Unterbefehl `xtask messen`, der `krk-bench` ruft; er wäre eine zweite Benennung für dieselbe Sache und widerspräche der Maxime "supersimpel".

**Aufgefallen bei:** der Umsetzung von Schritt 16b am 260804-1915.
