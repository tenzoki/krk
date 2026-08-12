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

---

**Resolved 260812** — `Zerlegung::merkzeichen_einloesen` zieht den `Offen::anfang`
jetzt nach, und zwar für die **inneren** Einträge und nur für sie.

**Die Behebung an einer Stelle.** Das Einlösen läuft nicht mehr über eine
gesammelte Zeichenkette, sondern Punkt für Punkt: wird das Merkzeichen des
Eintrags auf Stufe `n` geschrieben, so rücken alle Einträge hinter `n` nach,
deren `anfang` noch auf der Stelle davor steht. Der Punkt selbst und alles
außerhalb von ihm behalten ihren Anfang, denn das Merkzeichen gehört ihm und
der Einzug seiner Listenzeile soll es mitnehmen. Genau darin unterscheidet
sich der Nachzug von dem in `absetzen`, wo der Abstand keinem der offenen
Elemente gehört und deshalb **alle** nachrücken — der Datensatz hat das
vorhergesagt, und es hat sich beim Bauen bestätigt.

**Alle acht gemessenen Fälle, am Baum nachgemessen** (dieselbe Auflösung in
UTF-16 wie in der Messung oben):

```
"- **fett**\n"                  StarkeBetonung  -> "fett"     Listenzeile{1} -> "• fett"
"- *kursiv*\n"                  Betonung        -> "kursiv"   Listenzeile{1} -> "• kursiv"
"- `code`\n"                    FesteSchrift    -> "code"     Listenzeile{1} -> "• code"
"- [Link](http://a.example)\n"  Einfaerbung     -> "Link"     Listenzeile{1} -> "• Link"
"- # Titel\n"                   Ueberschrift{1} -> "Titel"    Listenzeile{1} -> "• Titel"
"- ```\n  code\n  ```\n"        FesteSchrift    -> "code\n"   Listenzeile{1} -> "• code\n"
"1. **fett**\n"                 StarkeBetonung  -> "fett"     Listenzeile{1} -> "1. fett"
"- - **fett**\n"                StarkeBetonung  -> "fett"     Listenzeile{1} -> "• • fett"
                                                              Listenzeile{2} -> "• fett"
```

Der Ausgabetext ist in allen acht unverändert; falsch war allein der Bereich.
Das ist der Grund, aus dem der Defekt durch 38 Proben hindurchging, und der
Grund, aus dem die neuen Proben **Bereiche** festschreiben und nicht Text.

**Sechs neue Proben in `crates/krk-ui/src/markdown.rs`**, alle gegen den
Zustand vor der Behebung gegengeprüft — der Nachzug wurde probeweise wieder
herausgenommen, und genau diese sieben Proben schlugen fehl:

- `eine_auszeichnung_am_anfang_eines_punktes_deckt_das_merkzeichen_nicht`
  (fünf Auszeichnungsarten in einer Tabelle, je Bereich **und** Listenzeile),
- `ein_verweis_am_anfang_eines_punktes_faerbt_das_merkzeichen_nicht`
  (die Einfärbung, die als einzige keine `Auszeichnung` ist),
- `eine_nummer_am_anfang_eines_punktes_wird_nicht_mit_ausgezeichnet`,
- `zwei_merkzeichen_liegen_gestaffelt_ausserhalb_der_auszeichnung`
  (`- - **fett**`: der äußere Punkt nimmt beide mit, der innere seines, die
  starke Betonung keines),
- `eine_lose_liste_haelt_ihre_auszeichnung_hinter_dem_merkzeichen`
  (die lose Liste ist die Form, die Turn 3 übersehen hat, die Auszeichnung am
  Anfang die, die Turn 4 übersehen hat — beide zusammen in einer Probe),
- `kein_merkzeichen_liegt_im_bereich_eines_stueckes`.

**Die letzte ist der Gurt um die ganze Klasse und nicht ein weiterer Fall.**
Sie läuft über dreizehn Quellen und verlangt, dass **kein** Bereich, der kein
Absatzmerkmal ist, mit einem gerenderten Merkzeichen beginnt — weder mit dem
`• ` noch mit einer Nummer samt Punkt und Leerzeichen. Nur
`Auszeichnung::Listenzeile` darf es, denn ihr gilt der Einzug. Diese Probe
hätte den Defekt gefangen, ohne dass jemand den einzelnen Fall hätte nennen
müssen; das ist die Lehre aus zwei Turns, die je eine Verschlechterung
eingeschleppt haben, weil keine Probe den betroffenen Fall maß.

**Der Modulkopf sagt die Regel jetzt in Worten** („Das Merkzeichen gehört
seinem Punkt und nicht seinem ersten Kind"), samt dem Unterschied zum Nachzug
in `absetzen`; derselbe Text steht am Doc-Kommentar von
`merkzeichen_einloesen`.

Abnahme: `cargo build --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
— alle vier Exit 0. Das Binärziel `krk` steht bei 478 Proben statt 466.
