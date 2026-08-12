Das Merkzeichen liegt im Bereich des ersten Kindes eines Punktes und wird mit ihm fett, kursiv, fest oder eingefärbt

---

Beginnt ein Listenpunkt unmittelbar mit einer Auszeichnung — `- **fett**`,
`- *kursiv*`, `` - `code` ``, `- [Verweis](…)`, `- # Titel` —, dann deckt die
Auszeichnung des Kindes das Merkzeichen mit ab. Der Aufzählungspunkt `•` und
die Nummer `1.` werden dadurch fett, kursiv, in fester Schrift, in
Überschriftsgröße oder in Verweisfarbe samt Unterstreichung gesetzt.

**Es ist eine Verschlechterung durch `c35f8b1` und kein Altbestand.**

---

**Gemessen** (`markdown::rendern` aus `crates/krk-ui/src/markdown.rs:182`,
beide Fassungen unverändert in dasselbe Prüfprogramm kopiert und
gegeneinander gefahren, `pulldown-cmark 0.13.4`, Tafel Hell; die Bereiche in
UTF-16 aufgelöst und als Textstück ausgegeben):

```
Quelle : "- **fett**\n"
f401dcc: Listenzeile{1} -> "• fett"   StarkeBetonung -> "fett"
c35f8b1: Listenzeile{1} -> "• fett"   StarkeBetonung -> "• fett"

Quelle : "- *kursiv*\n"
f401dcc: Betonung      -> "kursiv"
c35f8b1: Betonung      -> "• kursiv"

Quelle : "- `code`\n"
f401dcc: FesteSchrift  -> "code"
c35f8b1: FesteSchrift  -> "• code"

Quelle : "- [Link](http://a.example)\n"
f401dcc: Einfaerbung   -> "Link"
c35f8b1: Einfaerbung   -> "• Link"

Quelle : "- # Titel\n"
f401dcc: Ueberschrift{1} -> "Titel"
c35f8b1: Ueberschrift{1} -> "• Titel"

Quelle : "- ```\n  code\n  ```\n"
f401dcc: FesteSchrift  -> "code\n"
c35f8b1: FesteSchrift  -> "• code\n"

Quelle : "1. **fett**\n"
c35f8b1: StarkeBetonung -> "1. fett"

Quelle : "- - **fett**\n"
c35f8b1: StarkeBetonung -> "• • fett"   (beide Merkzeichen)
```

**Die Wirkung in AppKit ist die volle.** `crate::appkit::textmerkmale`
(`textmerkmale.rs:204-217`) setzt für `StarkeBetonung`, `Betonung`,
`FesteSchrift` und `Ueberschrift` je eine **Schrift** über
`NSRange(anfang, laenge)`, und für einen Verweis Farbe und Unterstreichung.
Das Merkzeichen liegt in diesem Bereich und bekommt sie mit. Nur
`Listenzeile` ist ein Absatzmerkmal und deshalb unbetroffen.

**Die Ursache.** `Zerlegung::oeffnen` (`markdown.rs:685-705`) setzt
`Offen::anfang` auf `self.stelle`. Für ein Kind, das als erstes im Punkt
geöffnet wird, ist der Merkzeichen-Wunsch zu diesem Zeitpunkt noch
uneingelöst, `self.stelle` steht also **vor** dem Merkzeichen.
`Zerlegung::merkzeichen_einloesen` (`markdown.rs:600-616`) schreibt es später
in `schreiben`, erhöht `self.stelle`, **zieht aber keinen `Offen::anfang`
nach**. `Zerlegung::absetzen` daneben tut genau das für die Umbrüche
(`markdown.rs:572-576`): `for eintrag in &mut self.offen { if eintrag.anfang
== vorher { eintrag.anfang = self.stelle; } }`. Beim Merkzeichen fehlt dieser
Schritt — und er ist auch nicht dieselbe Regel, denn beim Merkzeichen soll der
**Punkt** mitnehmen und das **Kind** nachrücken, während beim Absetzen alle
nachrücken.

Die Reihenfolge der Ereignisse für `- **fett**`:

```
Start(List)      oeffnen
Start(Item)      punkt_oeffnen   -> merkzeichen "• " vorgemerkt, nichts geschrieben
Start(Paragraph) oeffnen         -> anfang = 0        (Merkzeichen noch nicht da)
Start(Strong)    oeffnen         -> anfang = 0        (Merkzeichen noch nicht da)
Text("fett")     schreiben       -> absetzen, merkzeichen_einloesen ("• "), "fett"
End(Strong)      schliessen      -> laenge = 6 - 0 = 6, deckt "• fett"
```

**Keine Probe fängt es.** Von den 38 Proben in `markdown.rs` prüft keine den
Bereich einer Auszeichnung, die **als erstes** in einem Listenpunkt steht.
`betonung_und_starke_betonung_verlieren_ihre_sternchen` (`markdown.rs:926`)
steht auf Dokumentebene. `jede_stelle_liegt_innerhalb_der_laenge`
(`markdown.rs:1303`) führt zwar `- ein *Punkt* mit [Verweis](…)` in ihrer
Quelle, dort steht aber `ein ` vor der Betonung, also ist der Wunsch beim
Öffnen schon eingelöst; und sie prüft ohnehin nur die Schranke, nicht die
Lage. `ein_listenpunkt_traegt_den_einzug_und_behaelt_sein_zeichen`
(`markdown.rs:995`) prüft die `Listenzeile`, die als Absatzmerkmal
unbetroffen ist.

**Gewicht: hoch.** Der Punkt, der mit einer Auszeichnung beginnt, ist eine der
häufigsten Zeilen überhaupt. Im Baum dieses Projekts selbst:

```
grep -rEc '^[[:space:]]*([-*+]|[0-9]+\.)[[:space:]]+(\*\*|\*|`|\[)' --include='*.md' .
-> 3721 Treffer in 507 Dateien
```

Jede dieser Zeilen zeigt in KRKs Vorschau ein fettes, kursives oder
festbreites Aufzählungszeichen. C4.2 sagt Listen ausdrücklich zu, und die
Runde hat den Listen in Turn 3 und Turn 4 zweimal nachgebessert.

**Ein Zuschnitt** (nicht gewählt, hier nur als Ausgangspunkt):
`merkzeichen_einloesen` könnte nach dem Schreiben denselben Nachzug fahren
wie `absetzen`, aber nur für die Einträge **hinter** dem Punkt, dessen
Merkzeichen eingelöst wurde — die Einträge davor und der Punkt selbst sollen
es behalten. Der Punkt kennt seine Lage im Stapel beim Einlösen, denn die
Schleife läuft ohnehin über `self.offen`.

**Herkunft:** Circle der Runde 6, Turn 4, `c35f8b1` (Behebung von
`260812-1920_c_in-einer-losen-liste-steht-das-merkzeichen-allein-auf-seiner-zeile.md`,
Zuschnitt 2).
