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

---

**Resolved 260812** — die falsche Zusage ist berichtigt, die Ausgabe ist von
einer Probe festgeschrieben, und zwei der drei gemessenen Formen kommen
nebenbei richtig heraus. **Das Verhalten des ersten Falles bleibt, und zwar
mit Absicht**; der Grund steht unten und jetzt auch im Doc-Kommentar.

**Was der Datensatz zu Recht beanstandet, war die Zusage und nicht die
Ausgabe.** Der Doc-Kommentar an `schliessen` sagte pauschal zu, `• - [ref]: …`
entstehe nicht. Er sagt jetzt, was gilt: die Zusage betrifft den Punkt, der
sich schließt, und nicht seine äußeren. Deren Merkzeichen stehen **vor** dem
ausgegebenen Quellbereich und nicht darin, kommen also mit ihm nicht mit.

**Das `• - ` ist die richtige Ausgabe und keine Doppelung.** Bei
`"- - [ZIEL]: …"` steht in der Quelle zweimal `- `. Das äußere gehört dem
äußeren Punkt und liegt im Vorspann, fällt also nach der Deckungsgrenze aus
der Ausgabe heraus — gerendert kommt es allein als `• ` heraus. Das innere
kommt mit dem Quellbereich des inneren Punktes mit. **Beide Merkzeichen der
Quelle stehen danach genau einmal da, und keines fehlt.** Würde man die
Einlösung im wörtlichen Zweig unterdrücken, wie der Zuschnitt des Datensatzes
es erwogen hat, so verschwände das Merkzeichen des äußeren Punktes ersatzlos:
die Ausgabe wäre `"- [ZIEL]: …"` und trüge ein Merkzeichen weniger als die
Quelle. Das ist schlechter, nicht besser.

**Zwei der drei gemessenen Formen sind mit dem Nachbardefekt weggefallen:**

```
"- -\n"                        vorher: "• -"    jetzt: "• • "
"- - [ZIEL]: http://z.example\n"  unverändert:  "• - [ZIEL]: http://z.example\n"
"- >\n"                        unverändert:  "• >\n"
```

Der erste geht über `traegt_nur_sein_merkzeichen`
(`260812-2019_c_ein-leerer-listenpunkt-zeigt-sein-rohes-bindestrich-zeichen-und-verliert-seinen-einzug.md`): der innere Punkt trägt nichts als
sein Merkzeichen, also löst er seinen Wunsch ein statt sein `-` roh
auszugeben. Der dritte kommt nicht von einem Punkt, sondern von einem
Zitatblock ohne Inhalt und fällt damit unter denselben dritten Satz der
Deckung wie `[](https://example.com)`.

**Neue Probe in `crates/krk-ui/src/markdown.rs`:**
`ein_innerer_punkt_ohne_zeichen_steht_neben_dem_merkzeichen_des_aeusseren`
schreibt die Ausgabe des verbliebenen Falles fest. Der Doc-Kommentar an
`schliessen` nennt sie beim Namen, damit die berichtigte Zusage und ihre
Messung nicht auseinanderlaufen.

**Was nicht getan ist, und warum es nichts zu tun gibt:** eine Ausgabe, in der
gerendertes und rohes Merkzeichen **nicht** nebeneinander stünden, verlangte
eine Regel, die das Merkzeichen eines Containers vom Quelltext dahinter
trennt. Für den leeren Punkt ist sie jetzt da — dort ist hinter dem
Merkzeichen nichts. Für einen Punkt mit Inhalt ist sie es nicht, und
`260812-1920_c_die-deckungszusage-gilt-nicht-innerhalb-eines-elements-das-zeichen-geliefert-hat.md` hält am selben Punkt fest, dass sie nicht
mechanisch zu haben ist. Wer sie doch will, ändert damit die dort getroffene
Entscheidung und nicht diesen Defekt.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0.
