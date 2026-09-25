Der Sitzungslauf der Abnahmestrecke bricht bei L5-Tab ab und gibt keine Zahl mehr aus

---

`make alle RUNDEN=1` kommt auf dem heutigen Stand von `main` nicht mehr durch. Der
Sitzungslauf endet mit Rückgabewert 4 und dieser Meldung:

```
krk: die Messung l5-tab ist nach 10 s nicht am Ziel; seit dem Beginn sind 579
Bildgrenzen eingegangen. Es wird keine Zahl ausgegeben.
```

Damit gibt der Lauf **für keine der sechs Größen** der Sitzungsstrecke eine Zahl aus,
also auch nicht für L1, L7, L5-Fenster, L6, L8 und L9. Die Abnahme aus S22 ist auf
diesem Weg nicht mehr fahrbar.

Gemessen am 260806-1220 auf MacBookPro15,1, macOS 15.7.7, dreimal hintereinander mit
demselben Wortlaut, also nicht sporadisch. Zweimal mit den Änderungen der laufenden
Defektbearbeitung im Baum und einmal ohne sie (`git stash` auf `crates/`, unverändertes
`main`): **der Abbruch tritt in beiden Fällen an derselben Stelle auf.** Er hängt
also nicht an den Änderungen dieser Sitzung.

Die Endbedingung von L5-Tab ist `tab_aktiv != vorher.tab && zeilen_aktiv > 0`
(`crates/krk-ui/src/messmodus.rs`, `sitzungsmessung_fertig`). Der Bildtakt läuft
während der zehn Sekunden weiter, 579 Bildgrenzen sind rund 58 je Sekunde; es steht
also nicht die Oberfläche, sondern der sichtbare Tab wechselt nicht.

Die Reihen davor, L1 und L7, laufen durch. Beide werden mit `auswahl_runter`
ausgelöst, einer nackten Pfeiltaste im Wirkungsbereich `Dateifenster`. L5-Tab
benutzt `tab_naechster`, das die Auslieferungsbelegung auf `ctrl+tab` legt und dessen
Wirkungsbereich `Tabbereich` ist. **inference, nicht gemessen:** die Ursache liegt
entweder darin, dass das synthetische `ctrl+tab` den Ereignisabgriff nicht so
erreicht wie eine Taste ohne Zusatztaste, oder darin, dass der Befehl im
Wirkungsbereich `Tabbereich` nicht im Dateifenster ankommt. Der ungemessene Vorlauf
davor beweist das Gegenteil nicht: seine Wartebedingung `AktivGelesen` ist auch dann
sofort erfüllt, wenn der Tab gar nicht gewechselt hat.

---

**Warum das vorher nicht auffiel.** Die Abnahme-Messreihe
`messungen/260805-2207-MacBookPro15-1-abnahme.txt` trägt für beide L5-Fälle Zahlen aus
fünf Runden. Zwischen ihr und heute liegen mehrere Änderungen an der Oberfläche,
darunter S20 (Belegungsansicht), S21 (Messmodus), S23 (Auslieferungspaket), die
Gliederung der Belegungsansicht nach Funktionsbereichen und S6b. Welche davon den
Abbruch verursacht, ist mit einer Halbierung über diese Commits festzustellen; dieser
Defekt sagt es nicht.

**Was weiterhin misst.** Die Durchstichstrecke (`make durchstich`) läuft vollständig
durch und nimmt L2, L3, L4 und L10 ab; sie benutzt keinen Tabwechsel. Betroffen ist
allein der Sitzungslauf.

**Aufgefallen bei:** dem Versuch, die zehn Zeitzusagen aus C8 nach der Bearbeitung der
sechs offenen Oberflächendefekte am 260806 gegenzumessen. Adressat: `coder`.

---
Resolved: Kein Commit war die Ursache. Mit crates/krk-ui/src und xtask/src auf den S22-Stand e8626b6 zurückgenommen bricht der Lauf an derselben Stelle ab. Die Ursache ist der Vordergrund: steht KRK nicht vorn, weist kommando_ausfuehren jeden Befehl mit Wirkungsbereich ab, und nur auswahl_runter (Wirkungsbereich::Ueberall) löst noch aus. Die Strecke verweigert die erste Messung jetzt mit NICHT_IM_VORDERGRUND, statt bei L5-Tab zehn Sekunden zu warten. Die Entwurfsfrage steht als decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md. Nachweis: aus einem Terminalfenster im Vordergrund liefert make alle RUNDEN=1 für alle zwölf Berichtszeilen Zahlen (messungen/260806-1103-alle-zusagen.txt).
