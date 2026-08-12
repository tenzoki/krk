Das Merkzeichen eines äußeren Punktes wird vom wörtlichen Quelltext eines inneren eingelöst, und beide stehen nebeneinander

---

`Zerlegung::schliessen` sagt in seinem Doc-Kommentar zu, dass ein Punkt sein
noch ausstehendes Merkzeichen mitnimmt, „sonst stuende `• - [ref]: …`
doppelt da" (`markdown.rs:785-787`). Für den Punkt, der gerade geschlossen
wird, stimmt das. Für seine **äußeren** Punkte stimmt es nicht: deren
Merkzeichen sind noch offen, und `Zerlegung::woertlich` löst sie über
`schreiben` mit ein. Genau die Form, die der Kommentar ausschließt, kommt
dabei heraus.

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:182`,
beide Fassungen unverändert in dasselbe Prüfprogramm kopiert,
`pulldown-cmark 0.13.4`):

```
Quelle : "- - [ZIEL]: http://z.example\n"
f401dcc: "• \n• "
c35f8b1: "• - [ZIEL]: http://z.example\n"
         Listenzeile{1} deckt den ganzen Text

Quelle : "- -\n"
f401dcc: "• \n• "
c35f8b1: "• -\n"

Quelle : "- >\n"
f401dcc: "• \n\n>\n"
c35f8b1: "• >\n"
```

Gerendertes `• ` und rohes `- ` stehen in derselben Zeile.

**Die Ursache.** `Zerlegung::schliessen` (`markdown.rs:794-831`) nimmt den
schließenden Eintrag zuerst vom Stapel (`self.offen.pop()`) und ruft dann
`self.woertlich(eintrag.quelle)`. `woertlich` ruft `schreiben`, und
`schreiben` ruft `merkzeichen_einloesen` (`markdown.rs:600-616`), das über
**alle** noch offenen Einträge läuft und jeden vorgemerkten Wunsch einlöst.
Der gepoppte Eintrag ist raus — das ist die Hälfte, die der Kommentar meint —,
seine Vorfahren sind es nicht.

**Warum die Reihenfolge sonst richtig ist.** `merkzeichen_einloesen` löst von
außen nach innen ein, weil die Merkzeichen in der Quelle so stehen; die Probe
`zwei_punkte_uebereinander_tragen_beide_ihr_merkzeichen` (`markdown.rs:1092`)
hält das für `- - tief` fest. Der Fehler ist nicht die Reihenfolge, sondern
dass ein wörtlich ausgegebener Quellbereich, der die Merkzeichen der Quelle
schon trägt, überhaupt als „erstes Zeichen" gilt, vor dem einzulösen ist.

**Keine Probe fängt es.** Keine der 38 Proben in `markdown.rs` setzt ein
Element ohne ein einziges Zeichen **in** einen Listenpunkt.

**Ein Zuschnitt** (nicht gewählt): `woertlich` könnte im Zweig aus
`schliessen` an `schreiben` vorbeigehen, weil ein wörtlicher Quellbereich die
Merkzeichen seiner Umgebung ohnehin mitbringt. Ob dabei der Abstandswunsch aus
`absetzen` verlorenginge, ist zu prüfen.

**Gewicht: niedrig bis mittel.** Selten — ein Punkt, dessen einziges Kind
nichts liefert —, aber sichtbar roh, und der Doc-Kommentar an `schliessen`
sagt das Gegenteil zu. Die falsche Zusage wiegt unabhängig von der Häufigkeit;
das ist derselbe Befundtyp, den diese Runde in Turn 2 und Turn 3 je einmal
abgelegt hat.

**Herkunft:** Circle der Runde 6, Turn 4, `c35f8b1`.
