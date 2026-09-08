`textmerkmale.rs` trägt keine einzige Probe, obwohl zwei Bereiche daran hängen

---

Das neue Modul `crates/krk-ui/src/appkit/textmerkmale.rs` hat 436 Zeilen, fünf
öffentliche Stücke und seit dieser Runde zwei Verbraucher, Editor und Vorschau.
Es enthält kein `#[cfg(test)]`-Modul. Der Umzug aus `editor.rs` hat daran
nichts verloren: auch dort war keine der umgezogenen Funktionen geprüft.

---

**Nachgezählt:** `grep -n "cfg(test)" crates/krk-ui/src/appkit/textmerkmale.rs`
findet nichts. Planschritt 7 sagt „Die Proben aus `editor.rs`, die die
Umsetzung messen, ziehen mit um oder bleiben"; es gab keine, der Satz ist also
erfüllt, ohne dass etwas geprüft wäre.

**Was ohne Fenster prüfbar wäre und heute nicht geprüft ist:**

- `grundschrift(ansicht, art)` (`:333-347`) ist eine reine Fallunterscheidung
  über sechs Eingabepaare. Sie entscheidet, ob eine Fläche feste oder
  proportionale Schrift bekommt und in welcher Größe, und sie hat seit dieser
  Runde einen zweiten Aufrufer, dessen Schriftgröße sich dadurch geändert hat
  (Datensatz `decisions/260812-1707_o_…`). Eine Tafelprobe über die sechs Paare
  kostet zwanzig Zeilen und hielte die Zusage „eine Regel und keine drei" fest,
  die der Doc-Kommentar behauptet.
- `UEBERSCHRIFTSFAKTOREN` (`:139`) trägt sechs Zahlen, die streng fallend sein
  sollen („Absteigend, weil `#` mehr wiegt als `######`"). Eine Zeile Probe
  hielte das fest.
- Die Zuordnung von `Auszeichnung` auf Merkmalsart (`:197-208`) ist eine
  vollständige Fallunterscheidung über jetzt fünf Werte. Welcher Wert welches
  **Merkmal** setzt, ist ohne Fenster nicht zu messen; welche Werte **dieselbe**
  Merkmalsart setzen, wäre es, und genau daran hängt der Befund im Datensatz
  `260812-1805_o_der-ueberschneidungssatz-…`.

**Was zu Recht ungeprüft bleibt.** `anwenden`, `zuruecksetzen` und
`tafel_der_erscheinung` fassen eine Instanz an. `krk-ui` hat kein
Bibliotheksziel, und diese Runde baut ausdrücklich keine neue Probe, die den
Hauptfaden über `MainThreadMarker::new_unchecked` behauptet. Diese drei gehören
an das laufende Bündel und nicht in eine Probe — das ist hier nicht der
Vorwurf.

**Gewicht:** niedrig. Kein Defekt am Verhalten, sondern eine Lücke an einer
Stelle, die diese Runde von einer auf zwei Flächen ausgeweitet hat: wer die
Umsetzung hier ändert, ändert sie für Editor und Vorschau zugleich, und nichts
im Baum widerspricht ihm.

**Herkunft:** Circle der Runde 6, Planschritt 7.

---
Resolved: `crates/krk-ui/src/appkit/textmerkmale.rs` traegt ein `mod proben` mit vier Proben, alle ohne Fenster und ohne behaupteten Hauptfaden. Die zwei, die der Datensatz namentlich verlangt: `die_tafel_der_grundmerkmale` faehrt die sechs Eingabepaare von `grundmerkmale` einzeln (die Regel hinter `grundschrift`) und misst die Fallunterscheidung statt der Zahl, und `die_ueberschriftsfaktoren_fallen_streng_und_bleiben_ueber_eins` haelt die strenge Ordnung samt der unteren Schranke. Dazu zwei, die aus demselben Lesen entstanden: `die_grundgroesse_ist_die_zweite_haelfte_der_grundmerkmale` haelt die Zusage, an der `anwenden` haengt (eine Ueberschrift setzt auf derselben Grundlage auf wie ihr Text), und `die_liste_traegt_eine_zahl_je_stufe` haelt die Laenge, in die `anwenden` mit `stufe.clamp(1, 6) - 1` greift. **Der dritte Punkt des Datensatzes ist nicht gebaut:** welche `Auszeichnung`-Werte dieselbe Merkmalsart setzen, waere ohne Fenster messbar, verlangte aber eine reine Funktion `Auszeichnung -> Merkmalsart`, die es heute nicht gibt; er haengt am eigenen Datensatz `260812-1805_*_der-ueberschneidungssatz-*`. `anwenden`, `zuruecksetzen` und `tafel_der_erscheinung` bleiben ungeprueft, wie der Datensatz es ausdruecklich einraeumt.
