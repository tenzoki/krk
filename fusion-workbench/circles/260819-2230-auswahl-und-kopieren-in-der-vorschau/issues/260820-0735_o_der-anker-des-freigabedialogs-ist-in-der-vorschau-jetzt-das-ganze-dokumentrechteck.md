Der Anker des Freigabedialogs ist in der Vorschau jetzt das ganze Dokumentrechteck und nicht mehr die sichtbare Fläche

---

Schritt 5 hat `Vorschaufenster::fokusansicht` (`crates/krk-ui/src/appkit/vorschau.rs:832-838`)
so geändert, dass es die Textanzeige liefert, solange die Bildlaufansicht steht. Diese eine
Zuordnung hat zwei Abnehmer, und der zweite ist nicht der Ersthelfer:

```
Anwendungsdelegierter::teilen  (anwendung.rs:3538-3556)
  -> teilen::anbieten(&pfade, flaeche, flaeche.bounds())
```

`flaeche` ist ab jetzt der `Vorschautext`. Der wird in `textanzeige`
(`vorschau.rs:1435-1444`) mit `setVerticallyResizable(true)` und
`setMaxSize(NSSize::new(f64::MAX, f64::MAX))` aufgesetzt, wächst also mit dem Inhalt. Sein
`bounds()` ist damit das **ganze Dokumentrechteck** und nicht der sichtbare Ausschnitt; bei
einer langen Datei liegt dessen Mitte weit unterhalb des Fensters. Bis zur Runde 14 war der
Anker die `Inhaltsflaeche`, deren `bounds()` genau die sichtbare Fläche ist.

---

**Was der Plan dazu sagt und was er nicht sagt.** Der Doc-Kommentar an `fokusansicht`
nennt den Ankerfall ausdrücklich und begründet die Verzweigung damit, dass „eine
ausgeblendete Ansicht für keines von beidem taugt". Die Frage nach der Ausblendung ist
damit beantwortet; die nach der **Größe** des Rechtecks ist nicht gestellt worden. Der
Doc-Kommentar an `teilen` sagt seinerseits, wie der Anker aussieht, sei „am Bündel zu
beurteilen" — nur ist das ein Satz aus der Runde 6, gemünzt auf eine Ansicht, deren
`bounds()` damals sichtbar war.

**Gemessen ist der Code, nicht das Bild.** Am Baum gelesen sind die drei Stellen oben. Wie
`NSPopover` auf ein Ankerrechteck außerhalb des sichtbaren Bereichs reagiert, ist am
laufenden Bündel zu sehen und gehört damit zur Bündelabnahme. Der Befund steht trotzdem
hier, weil der Zusammenhang aus dem Code allein zu lesen ist und die Abnahmeliste der Runde
(`## Nutzerarbeit` im Plan) ihn nicht führt: C1 der Runde 6 ist dort kein Gegenstand, also
wird niemand mit dem Fokus in der Vorschau auf „Teilen" drücken, wenn es nicht dasteht.

**Nachzusehen ist:** eine lange Textdatei in der Vorschau zeigen, hineinklicken, weit nach
unten blättern, den Teilen-Befehl drücken. Erscheint der Dialog an der erwarteten Stelle,
kostet der Befund nichts und wird geschlossen.

**Richtung, falls nicht:** den sichtbaren Ausschnitt als Rechteck nehmen — die
Bildlaufansicht kennt ihn — statt eine zweite Zuordnung von Fokuswert auf Ansicht
danebenzustellen. Die eine Zuordnung aus C1.8 bleibt davon unberührt: geändert würde das
Rechteck, nicht die Ansicht.

**Schwere:** mittel. Es trifft einen Befehl aus einer früheren Runde, den diese Runde nicht
anfassen wollte.
**Baumstand:** `b28cdd6`.
