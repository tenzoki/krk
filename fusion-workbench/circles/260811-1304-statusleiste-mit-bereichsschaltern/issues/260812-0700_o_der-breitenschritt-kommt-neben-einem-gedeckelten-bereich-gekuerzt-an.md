Der Breitenschritt kommt neben einem gedeckelten Bereich gekürzt an

---

Hängt ein **anderer** sichtbarer Bereich an seinem Mindestmaß, dann verschiebt `opt+cmd+rechts` die
Trennlinie zwischen den beiden Dateifenstern um weniger als die 40 Punkte, die C4.9 zusagt.
Gemessen: bei 800 Punkten Fensterbreite und den ausgelieferten Breiten sind es 20,36 Punkte, also
gut die Hälfte. Die Richtung stimmt, die Zahl nicht.

---

**Schwere:** niedrig (der Befehl wirkt und wirkt in die richtige Richtung; er wirkt nur schwächer,
als sein Schritt sagt, und ein zweiter Anschlag holt die Differenz nach)
**Gefunden:** coder, bei der Behebung von
`260812-0539_c_die-breitenbefehle-aus-c7-wirken-unter-der-mindestsumme-in-die-falsche-richtung.md`
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs`, `massstab` zusammen mit `breite_aendern`
**Domain:** code

## Nachgerechnet

Vier sichtbare Bereiche der Runde 1, keine Trennlinien, 800 Punkte. Auf dem Schirm steht
`[120; 259,64; 259,64; 160,73]` — die Lesezeichenleiste ist auf ihr Mindestmaß gedeckelt, die
übrigen drei nicht. Der Maßstab ist 1280/800 = 1,6, der Schritt also 40 × 1,6 = 64 gespeicherte
Punkte; gedeckelt wird er auf 36, weil das rechte Dateifenster sein skaliertes Mindestmaß von 384
erreicht. Gespeichert stehen danach 456 zu 384, auf dem Schirm 280 zu 240.

Das linke Dateifenster gewinnt damit 20,36 Punkte statt 40. Die Probe
`ein_gedeckelter_dritter_bereich_sperrt_den_breitenbefehl_nicht` schreibt beide Zahlen aus.

## Die Ursache

`massstab` ist **ein** Faktor für die ganze Zeile: `gespeicherte Summe der sichtbaren / verfügbare
Breite`. Er trifft, solange kein sichtbarer Bereich an seinem Mindestmaß hängt. Hängt einer, nimmt
er mehr als seinen Anteil, und die übrigen teilen einen kleineren Rest: ihr wirklicher Faktor ist
ein anderer als der gemeinsame. Der Kommentar an `massstab` benennt das seit dem 260812-0512 und
lässt es ausdrücklich stehen; dieser Datensatz hält fest, was es an C4.9 kostet, damit die Kosten
nicht allein in einem Kommentar stehen.

## Abgrenzung

**Nicht dieser Datensatz:** dass die Deckelung das Vorzeichen des Betrags verschluckte. Das war der
Befund vom 260812-0539 und ist behoben; unter der Mindestsumme bleibt der Befehl jetzt ohne
Wirkung. Gekürzt ist nicht umgekehrt.

## Zwei Wege

1. **So lassen und die Grenze schreiben.** Der Kommentar an `massstab` trägt sie bereits, C4.9
   nicht. Dann gehört ein Satz in den Spec: der Schritt gilt, solange kein Bereich an seinem
   Mindestmaß hängt.
2. **Den Faktor für die betroffenen zwei Bereiche ausrechnen**, statt den gemeinsamen zu nehmen.
   Das verlangt zu wissen, welche Bereiche die Wasserstandsrechnung herausgenommen hat, also einen
   Rückgabewert mehr an `bereichsbreiten` — und es ist eine Änderung an der einen Regel und keine
   Zeile daneben. Ob sich das für einen Fall lohnt, den nur ein sehr schmales Fenster erreicht, ist
   die Frage.

Der Weg entscheidet sich am Spec und nicht am Code; der Datensatz gehört deshalb vor die nächste
Abnahme von C4.9 und nicht in einen Behebungsschritt.
