# Code-Durchsicht: die Bereichsleiste, die Spaltensichtbarkeit und die neu geschnittene Naht

**Datum:** 260812-0727
**Sender:** coderev
**Reviewed-range:** `8ffaac2..0342445`
**Not-opened:** none
**Maßstab:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`,
Fähigkeiten C1 bis C7, **nur die als (Probe) gekennzeichneten Kriterien**;
Implementierungsschritte 4 bis 8 und die Behebung der vier Befunde vom 260812-0539
**Abnahme nachgefahren:** `cargo fmt --all --check` Exit 0,
`cargo clippy --workspace --all-targets -- -D warnings` Exit 0,
`cargo test --workspace` Exit 0

---

## Zusammenfassung

Der neue Schnitt an der Rückrechnung trägt. Die Frage, die der coder an die Stelle der
unentscheidbaren gesetzt hat — nicht „ist die Abbildung umkehrbar", sondern „steht auf dem
Schirm etwas, das die Regel nicht selbst ausgelegt hat" — ist aus den Eingaben beider
Aufrufer wirklich beantwortbar, und ich habe alle vier Wege durchgespielt, auf denen Modell
und Delegierter auseinanderlaufen könnten. Sie laufen wieder zusammen. Die Bereichsleiste
kann nicht in eine Schleife geraten, hält den Delegierten schwach, setzt
`setRefusesFirstResponder(true)` an allen acht Schaltern und schreibt keinen Zustand, den das
Modell nicht hat. Die drei Zahlen der Belegung sind nachgezählt und stimmen: 79 Funktionen,
85 Kombinationen, 73 Kommandos.

**Ein Befund von Gewicht, und er ist kein Programmierfehler, sondern ein Widerspruch zwischen
zwei Nutzerentscheiden**, den beim Beantworten des zweiten niemand gegen den ersten gelesen
hat: die drei Spaltenbefehle stehen nicht in der Markdown-Ausgabe, obwohl das Kriterium C3.5,
der Entscheidungsdatensatz und ein Kommentar in der ausgelieferten Belegungsdatei es zusagen.
Drei kleinere Befunde betreffen einen Modulkopf, eine abgeschriebene Bedingung und einen
Kommentar, der eine Doppelung bestreitet.

**Zählung:** kritisch 0, hoch 0, mittel 1, niedrig 3.

---

## Was ich nachgerechnet und bestätigt habe

### Der neue Schnitt an der Rückrechnung ist entscheidbar

Die Frage lautet in `traegt_eine_ziehbewegung` (`crates/krk-ui/src/fenstermodell.rs:1188`):
weicht eine der fünf gemessenen Breiten um mehr als einen Viertelpunkt von dem ab, was
`bereichsbreiten` aus den gehaltenen Wünschen und der genannten Geometrie rechnet? Alle vier
Eingaben liegen dem Aufrufer vor. Das ist der Unterschied zur alten Frage: „hängt ein Bereich
an seinem Mindestmaß, weil er gedeckelt wurde, oder weil der Nutzer ihn dorthin gezogen hat"
ist aus den Rahmen nicht zu beantworten, „stammt diese Zahl von der Regel" schon.

**Die Verschiebung ist keine Ausweichbewegung**, und der Grund steht im Code: die
Rückrechnung gibt es allein wegen der Ziehbewegung, und eine Ziehbewegung ist genau das, was
die Regel nicht geschrieben hat. Die beiden Fehlerrichtungen sind dabei nicht gleich schwer,
und die Wahl fällt auf die harmlose. Eine Ziehbewegung, die zufällig genau auf der Ausgabe
der Regel landet, wird übersehen — und richtet nichts an, weil dieselbe Zeile herauskommt.
Eine Abweichung, die fälschlich als Ziehbewegung gilt, wird übernommen — und das ist das
Verhalten von vor dem 260812, also der Defekt und nichts Schlimmeres.

**Die vier Wege, auf denen Modell und Delegierter auseinanderlaufen könnten, laufen wieder
zusammen.** Seit dem 260812 gibt es die Wünsche an zwei Stellen: `Fenstermodell::breiten` und
`AufteilungsIvars::wuensche` (`crates/krk-ui/src/appkit/aufteilung.rs:118`). Nachgegangen:

1. **Ziehbewegung, dann ein Befehl.** `bildschirmbreiten_uebernehmen` am Kopf von
   `kommando_ausfuehren` misst gegen die Wünsche des Modells, erkennt die Abweichung, das
   Modell übernimmt; `aufteilung_nachziehen` → `anwenden` → `wuensche_merken` trägt sie in
   den Delegierten. Beide gleich.
2. **Ziehbewegung, dann eine Fenstergrößenänderung ohne Befehl.** `neu_auslegen` erkennt sie
   gegen die Wünsche des Delegierten und übernimmt dort; das Modell weiß noch nichts. Der
   nächste Befehl misst gegen das **Modell**, findet erneut eine Abweichung und zieht nach.
   Kein dauerhafter Unterschied.
3. **Ziehbewegung, dann ein abgewiesener Befehl.** `bildschirmbreiten_uebernehmen` läuft vor
   der Abweisung, das Modell übernimmt, `aufteilung_nachziehen` läuft nicht. Der Delegierte
   bleibt zurück; die nächste Größenänderung erkennt die Abweichung und holt ihn nach.
4. **Reine Fenstergrößenänderung.** Gemessen ist genau, was `auslegen` geschrieben hat, also
   keine Abweichung, also keine Übernahme. Das ist C4.7, und
   `ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her` misst genau die Folge von
   Aufrufen, die die Aufteilung fährt.

**`alte_groesse` ist dabei wirklich tragend**, und der Parameter, den die Datei bis heute
ignoriert hat, ist der richtige: `splitView:resizeSubviewsWithOldSize:` wird gerufen,
nachdem AppKit die Breite der `NSSplitView` gesetzt und **bevor** irgendjemand die Rahmen der
Unteransichten angefasst hat. Die gemessenen Breiten sind unter der alten Breite entstanden,
und nur an ihr gemessen lässt sich sagen, ob sie von der Regel stammen. `auslegen` unmittelbar
danach liest dagegen die **neue** Breite über `zeilenmass(teiler)`. Beide Stellen nehmen die
Geometrie, die zu ihrer Frage gehört.

**Ein Ring entsteht nicht.** Der Delegierte hält einen Wert und keine Sicht auf das
Fenstermodell; das Modell erfährt von einer Ziehbewegung weiterhin nur, wenn jemand nachmisst.
Der Modulkopf sagt das, und der Code hält es.

### Was geschieht, wenn AppKit die Rahmen nach dem Auslegen doch verändert

Der coder nennt das als ungemessen, und es bleibt ungemessen; ich kann es ohne Fenster
ebensowenig messen. Zwei Beobachtungen, die den Rahmen der Unsicherheit verkleinern.

**Der Baum rundet nirgends.** `auslegen` (`aufteilung.rs:585`) setzt die Rahmen mit den
gebrochenen Zahlen aus `bereichsbreiten`, `gemessene_breiten` liest `frame().size.width`
zurück, und dazwischen liegt keine Zeile, die auf ganze Punkte oder Bildpunkte legt.
`NSView.frame` gibt zurück, was gesetzt wurde; die Ausrichtung auf den Sicherungsspeicher
betrifft das Zeichnen und nicht die Eigenschaft. Der Spielraum von einem Viertelpunkt fängt
also nichts ab, was heute anfiele.

**`inference:` Die Begründung des Spielraums trägt nur auf einem Schirm mit doppelter
Auflösung.** Der Kommentar an `ZIEHSPIELRAUM` (`fenstermodell.rs:1147`) sagt, ein Viertelpunkt
liege „über dem, was ein Runden der Rahmen auf ganze Bildpunkte hinterließe". Auf einem Schirm
mit einfacher Auflösung ist ein Bildpunkt ein Punkt, ein Runden hinterließe bis zu einem
halben, und der Vergleich `> 0.25` schlüge an. Der andere Halbsatz, der Spielraum liege unter
dem kleinsten Schritt einer Ziehbewegung, hält auf beiden Schirmen. Ich lege dazu keinen
Datensatz an: der Fall setzt ein Runden voraus, das dieser Baum nicht vornimmt, und die Folge
wäre der bekannte Defekt und kein neuer. Wer den Kommentar das nächste Mal anfasst, setzt „auf
einem Schirm mit doppelter Auflösung" auch in diesen Halbsatz.

**Der Zustand vor dem ersten `anwenden` ist unauffällig.** Feuert AppKit `neu_auslegen`
zwischen dem Setzen der Inhaltsansicht und `aufteilung_nachziehen`, dann stehen im Delegierten
noch `Breiten::default()`, die gemessenen Rahmen tragen die Aufbaugröße, die Abweichung gilt
als Ziehbewegung, und die Zeile wird einmal falsch ausgelegt. `aufteilung_nachziehen`
(`anwendung.rs:852`) überschreibt sie, bevor das Fenster gezeigt wird. Dasselbe galt vor dem
260812; kein Rückschritt.

### Die Bereichsleiste

**Keine Schleife.** `zustaende_setzen` schreibt ausschließlich `setState:`, und `setState:`
löst die Aktion eines `NSButton` nicht aus — nur eine Benutzerhandlung oder `performClick:`
tut das. Der Melder kann sich also nicht selbst wieder anstoßen. `Leistenquelle::melden`
leiht die Zelle dabei nur lesend aus; der einzige schreibende Zugriff ist `melder_setzen` beim
Aufbau, und der liegt vor jedem möglichen Klick. Auch die `RefCell` des Fenstermodells trägt:
`aufteilung_nachziehen` und `bereichsleiste_nachziehen` geben ihre Ausleihe je in einem
eigenen Block wieder her, bevor sie weiterrufen.

**Kein Zustand, den das Modell nicht hat.** `zustaende_setzen` liest allein `sichtbar_in` und
`spalte_sichtbar_in` über den Stand, den `bereichsleiste_nachziehen` gerade aus dem Modell
geholt hat. Der gegenseitige Ausschluss von Vorschau und Editor bleibt im Modell (C2.3), und
die Leiste zeigt sein Ergebnis. Die Aufteilung liest ihre Sichtbarkeit weiterhin aus den
Ansichten, die Leiste aus dem Modell — beide stehen unmittelbar hintereinander in
`aufteilung_nachziehen`, und `anwenden` schreibt das Modell vorher in die Ansichten, also
können sie nicht auseinanderlaufen.

**Der Melder hält den Delegierten schwach.** `objc2::rc::Weak::from_retained(&self.retain())`
(`anwendung.rs:776`), derselbe Zuschnitt wie die fünf anderen Melder. Die Gegenrichtung
schließt sich nicht: `Bereichsleiste` hält die `Leistenquelle` stark, die Quelle hält nur den
Rückruf und keine Ansicht, `NSControl` hält sein Ziel ohnehin nur schwach — der
SAFETY-Kommentar an `schalter_bauen` belegt das mit der Quellzeile der Bindung.

**`setRefusesFirstResponder(true)` steht an allen acht.** Es gibt genau einen Erzeuger,
`schalter_bauen` (`bereichsleiste.rs:407`), und alle acht Schalter gehen durch ihn; ein
Schalter ohne die Zeile ist nicht baubar, ohne sie herauszunehmen. Ob die Eigenschaft den Rang
bei eingeschalteter vollständiger Tastaturbedienung wirklich verhindert, ist weiterhin
ungemessen — das ist C1.4 und Nutzerarbeit, und der Plan sagt es selbst.

**Die Zuordnung Schalter zu Kommando trägt sich selbst.** Die `tag` eines Bereichsschalters
ist `Bereich::index()`, und `index()` gibt genau die Stelle in `Bereich::ALLE` zurück, aus dem
`bauen` die Schalter erzeugt und in das `zustaende_setzen` zurückgreift; nachgelesen an
`fenstermodell.rs:122` und `:131`. Die `tag` eines Spaltenschalters ist die Stelle in
`Spalte::ALLE`, und beide Sorten werden über verschiedene Selektoren zugestellt, können sich
also nicht verwechseln.

### Die Spaltensichtbarkeit

**Die geladene Sitzung kommt bei beiden Tabellen an.** `oberflaeche_aufbauen` setzt das Modell
aus der Sitzung (`anwendung.rs:705`), lange bevor `spaltenanzeige_nachziehen` bei `:854`
läuft; jene Funktion durchläuft `Fensterseite::ALLE` und `Spalte::ALLE`, schreibt also acht
Zustände in zwei Tabellen. Ohne diese eine Zeile erreichte die Sitzung die Anzeige nicht, weil
`spaltenkopf` seine vier Spalten immer sichtbar baut.

**Ein ausgeblendetes Dateifenster ändert nichts daran.** `spalte_verbergen` setzt
`NSTableColumn::setHidden` an einer Spalte der Tabelle, und die Tabelle liegt unabhängig von
der Sichtbarkeit ihres Bereichs vor; der Bereich wird über die Unteransicht der `NSSplitView`
ausgeblendet, nicht über die Tabelle. Der Zustand steht damit schon richtig, wenn das
Dateifenster wieder aufgeht. Die einzige Vorsichtszeile ist die Frage nach
`ivars().dateifenster`, und sie trägt genau den Aufbau ab, in dem es die beiden noch nicht
gibt.

**Die Sortierung bleibt unangetastet, und das ist keine Zusicherung, sondern eine
Unmöglichkeit.** Der Sortierschlüssel wohnt in `Tabliste`, `Fenstermodell::spalte_umschalten`
kommt an die Tabs nicht heran, und `spalte_verbergen` ruft `setHidden:` statt
`removeTableColumn:`, wodurch die Spalte in `tableColumns` und in `numberOfColumns` bleibt.
Die Probe `das_wegschalten_der_sortierspalte_laesst_die_sortierung_stehen`
(`fenstermodell.rs:2729`) misst am Weg durch `session.toml`, also dort, wo eine Änderung
sichtbar würde. C3.3 hält.

### Die drei Zahlen, selbst nachgezählt

Nicht übernommen, sondern gezählt:

| Zahl | Weg | Ergebnis |
|---|---|---|
| Funktionen der Auslieferungsbelegung | Zahl der `[[funktion]]`-Blöcke in `resources/default-keymap.toml` | **79** |
| Kombinationen | Summe der Zeichenketten über alle `tasten`-Listen | **85** |
| Kommandos | Varianten von `enum Kommando` und Einträge in `KENNUNGEN` | **73 und 73** |

Die Kopfzeile der Belegungsdatei nennt 79 und 85, die Feldbreite von `KENNUNGEN` nennt 73.
79 minus die sechs Einträge mit `gehalten_von` ergibt 73, und genau sechs tragen das Feld.
Genau drei Funktionen sind ohne Kombination, und es sind die drei Spaltenschalter. Eine
Kombination steht zweimal, `cmd+a`, und das ist der bekannte Fall der beiden Auswahlbefehle
mit verschiedenem Wirkungsbereich, nicht neu in diesem Bereich.

**Die gefallene Zusage ist an den Proben nachgezogen.** „Jede nicht reservierte Funktion trägt
mindestens eine Kombination" gilt nicht mehr, und beide Proben, die sie trugen, lesen jetzt
dieselbe Liste `OHNE_KOMBINATION_AB_WERK` am Kopf von `crates/krk-core/tests/belegung.rs:75`:
`jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` von der Seite der
Datei, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` von der Seite der
gebauten Kommandos. Eine Liste, zwei Leser, mit Verweis auf den Datensatz. Sauber gelöst.

**Die Prosastellen sind nachgezogen**, mit einer Ausnahme, die Befund 1 ist: über den ganzen
Baum findet sich keine Stelle mehr, die „74 Funktionen" oder „68 Kommandos" rechnet; die neun
in `belegungsausgabe.rs` und `menue.rs` tragen 79 und 73. Die Suchbegriffe „nie `false`",
„Punktzahl", „weicht", „das linke" und „reserviert" liefern keine Stelle mehr, die die alte
Welt behauptet — wo sie stehen, stehen sie als Rückblick oder in ihrer heutigen Bedeutung
(`Bereich::teilt_flaeche_mit` weicht wirklich, `reserviert_fuer` ist wirklich noch ein Feld
der Datei).

### Die fünf neuen vollständigen Fallunterscheidungen

| Funktion | Ort | Auffangzweig | Kommentar sagt warum | hält den Bau an |
|---|---|---|---|---|
| `Bereich::beschriftung` | `fenstermodell.rs:250` | keiner | ja | ja, bei einem sechsten Bereich |
| `Bereich::langname` | `fenstermodell.rs:272` | keiner | ja | ja |
| `spalte_sichtbar_in` | `fenstermodell.rs:349` | keiner | ja, samt Begründung für den Zweig `Spalte::Name` | ja, bei einer fünften Spalte |
| `kommando_des_bereichs` | `bereichsleiste.rs:127` | keiner | ja | ja |
| `kommando_der_spalte` | `bereichsleiste.rs:144` | keiner | ja | ja |

Alle fünf halten, was ihr Kommentar zusagt. Dazu kommt eine sechste Verbesserung außerhalb der
Liste: `appkit::tabelle::ausrichtung` hat mit diesem Bereich seinen Auffangzweig
`_ => NSTextAlignment::Left` verloren und nennt die drei linksbündigen Spalten einzeln.

**Zwei Stellen halten den Bau nicht an, und beide sind ehrlich benannt.** Die Feldbreite
`[Retained<NSButton>; 3]` und das `try_into().expect(…)` in `Bereichsleiste::bauen`
(`bereichsleiste.rs:348`) würden bei einer vierten schaltbaren Spalte erst zur Laufzeit
scheitern; der Kommentar sagt das und benennt die Probe
`genau_drei_spalten_sind_schaltbar` als das, was die beiden aneinanderhält. Die Feldbreite
`[Retained<NSButton>; 5]` dagegen kommt aus `Bereich::ALLE.map(…)` und hält den Bau wirklich
an. Die dritte Stelle ist Befund 2.

---

## Befunde

### Befund 1 (mittel): Die drei Spaltenbefehle stehen nicht in der Markdown-Ausgabe, obwohl drei Stellen es zusagen

**Datensatz:** `issues/260812-0727_o_die-drei-spaltenbefehle-stehen-nicht-in-der-markdown-ausgabe-obwohl-drei-stellen-es-zusagen.md`
**Ort:** `crates/krk-ui/src/belegungsausgabe.rs:170` (`markdown`), Zeilen 175 bis 179
**Betrifft:** `krk-ui`, Kriterium C3.5, Datensatz `decisions/260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md`, `resources/default-keymap.toml`

`markdown` nimmt eine Funktion nur auf, wenn sie mindestens eine Kombination trägt. Die drei
Spaltenbefehle tragen ab Werk keine, also fehlen sie in `~/Downloads/KRK-Tastenbelegung.md`.
Die Bildschirmansicht führt sie dagegen: `nach_bereichen` ordnet jede Funktion einem Abschnitt
zu, und `gliederung` baut für jede eine Zeile. **Die erste Hälfte von C3.5 hält, die zweite
nicht.**

Drei Stellen sagen das Gegenteil zu, alle drei aus dieser Runde:

1. **Kriterium C3.5** des Plans, Zeile 46: „stehen in der Belegungsansicht **und in der
   Markdown-Ausgabe der Runde 3**". Es trägt die Kennzeichnung **(Probe)**.
2. **Der Entscheidungsdatensatz** `260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md`,
   Abschnitt `## Antwort`, wörtlich derselbe Satz. Er trägt `Implemented: 90b02d4` und den
   Marker `_i_`.
3. **`resources/default-keymap.toml`**, Zeilen 306 bis 310: „stehen wie jede andere in der
   Belegungsansicht und in der Markdown-Ausgabe". Das ist eine ausgelieferte Datei, die der
   Nutzer liest.

**Zwei Proben halten inzwischen ausdrücklich das Gegenteil fest**, und beide sind mit Schritt
7 auf den Code nachgezogen worden statt den Widerspruch zu melden:
`jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
(`belegungsausgabe.rs:530`) prüft, dass genau die drei Spaltenkennungen unbelegt sind, und
zählt die Zeilen der Datei gegen die Zahl der belegten Funktionen;
`innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei` (`:621`) filtert die Erwartung
seit dem 260812 auf die belegten.

**Die Ursache ist ein Widerspruch zwischen zwei Nutzerentscheiden**, und keiner der beiden ist
ein Versehen. Die Runde 3 hat am 260811-0110 gewählt, nur belegte Funktionen aufzunehmen, den
Preis dabei ausdrücklich benannt und die Empfehlung des Datensatzes überstimmt; jener
Datensatz hält fest, dass die Wahl ab Werk nichts ändert, weil damals keine der 71 Funktionen
ohne Kombination war. Seit dem 260812 sind es drei. Die Antwort vom 260812-0306 ist offenbar
nicht gegen die von 260811-0110 gelesen worden.

**Nicht im Vorbeigehen zu beheben**, weil jede Behebung entweder einen Nutzerentscheid umkehrt
oder eine Zusage streicht. Die drei Wege stehen im Datensatz; der billigste berichtigt die
drei Textstellen, der erwartbarste nimmt unbelegte Funktionen in die Ausgabe auf. Unabhängig
davon zu erledigen: die Zeile `Implemented: 90b02d4` behauptet heute etwas, das der Commit
nicht enthält.

### Befund 2 (niedrig): Der Modulkopf von `spalten.rs` zählt `beschreibbar` unter die vollständigen Fallunterscheidungen

**Datensatz:** `issues/260812-0727_o_der-modulkopf-von-spalten-rs-zaehlt-beschreibbar-unter-die-vollstaendigen-fallunterscheidungen.md`
**Ort:** `crates/krk-ui/src/spalten.rs:16-24` gegen `:98-100`
**Betrifft:** `krk-ui`

Der Kopf der neuen Datei sagt zu, jede Fallunterscheidung über `Spalte` sei ausgeschrieben und
eine fünfte Spalte halte den Bau an, und zählt „die Frage, ob man in ihr schreiben darf"
ausdrücklich mit auf. `Spalte::beschreibbar` ist `matches!(self, Spalte::Name)`, also ein
`match` mit `_ => false` darunter. Die Probe `genau_die_namensspalte_ist_beschreibbar`
(`:118`) fängt es nicht ab: eine fünfte, still nicht beschreibbare Spalte lässt ihre Gleichheit
unberührt.

**Der Code ist nicht neu, die Zusage schon.** Die Fassung mit `matches!` ist mit Schritt 6
unverändert aus `appkit/tabelle.rs` umgezogen; der Modulkopf ist beim Umzug entstanden.
Dieselbe Runde hat an `ausrichtung` in derselben Bewegung die entgegengesetzte Wahl getroffen
und den Auffangzweig entfernt.

### Befund 3 (niedrig): `editor_umschalten` schreibt die Erreichbarkeitsprüfung von `fokus_editor_holen` wortgleich ab

**Datensatz:** `issues/260812-0727_o_editor-umschalten-schreibt-die-erreichbarkeitspruefung-von-fokus-editor-holen-wortgleich-ab.md`
**Ort:** `crates/krk-ui/src/appkit/anwendung.rs:4389` gegen `:1466`
**Betrifft:** `krk-ui`

Die neue Funktion besteht aus neun Zeilen, die Zeichen für Zeichen in `fokus_editor_holen`
stehen, und einer zehnten, die sich unterscheidet. Beide beantworten dieselbe Frage: **der
Editor ist ansprechbar, wenn er steht oder wenn er eine Datei hält.** Der Doc-Kommentar
benennt die Doppelung („Dieselbe Bedingung trägt `fokus_editor_holen`"), ohne sie aufzulösen.

Das wiegt in diesem Baum mehr als anderswo: derselbe Commitbereich legt vier Stellen
ausdrücklich an, um eine zweite Fassung zu vermeiden (`breite_in`, `spaltenfach`, die eine
`kennung`, `sichtbar_in` als einzige Zuordnung), und die erste Durchsicht dieser Runde hat mit
Befund 3 genau eine solche Doppelung geschlossen.

### Befund 4 (niedrig): Der Nachzug der Bereichsleiste läuft nach einem angenommenen Klick zweimal

**Datensatz:** `issues/260812-0727_o_der-nachzug-der-bereichsleiste-laeuft-nach-einem-angenommenen-klick-zweimal.md`
**Ort:** `crates/krk-ui/src/appkit/anwendung.rs:777` (Melder) und `:2761`
(`aufteilung_nachziehen`), gegen den Doc-Kommentar an `:2780`
**Betrifft:** `krk-ui`

Der Melder ruft `kommando_ausfuehren` und danach unbedingt `bereichsleiste_nachziehen`;
`kommando_ausfuehren` ruft bei Erfolg `aufteilung_nachziehen`, das seinerseits mit
`bereichsleiste_nachziehen` endet. Auf dem Klickweg werden nach einem angenommenen Befehl also
sechzehn `setState:` geschrieben statt acht. Der Doc-Kommentar sagt „Der zweite Anlass ist
keine Verdopplung des ersten" — für den abgewiesenen Klick stimmt das, für den angenommenen
nicht.

Kein falscher Zustand: die Schreibvorgänge sind idempotent und lesen denselben Modellstand.
Die Risikotafel des Plans wägt die Kosten dieses Nachzugs aber ausdrücklich gegen die
Zeitzusage L1 ab und setzt dafür acht an. Wichtiger als die Zeichenarbeit ist der Satz: dieses
Projekt hat dreimal eine Sitzung an einem Kommentar verloren, der etwas anderes sagte als der
Code.

---

## Was quer liegt

**Die eine Naht ist zu, und die andere ist aufgegangen.** Die erste Durchsicht fand beide
Befunde von Gewicht an derselben Stelle, der Grenze zwischen Schirm und gespeicherter Zahl;
diese findet ihren an der Grenze zwischen dem, was die Runde zusagt, und dem, was eine ältere
Runde entschieden hat. Beides sind Nahtstellen, aber es sind verschiedene, und die zweite ist
nicht durch schärferes Lesen von Code zu schließen. **Der Vorgang, der sie aufgemacht hat, ist
benennbar:** die Klärungsrunde vom 260812-0306 hat eine Zusage über die Markdown-Ausgabe
gegeben, ohne den Datensatz zu lesen, der über deren Umfang schon entschieden hatte. Die
Entscheidungsdatensätze binden über Runden hinweg, und CLAUDE.md sagt es an zwei Stellen; hier
hat eine neue Antwort eine alte überholt, ohne sie als überholt zu kennzeichnen.

**Die Zusage „jede Funktion trägt eine Kombination" ist an drei von vier Stellen sauber
gefallen und an einer nicht.** Die beiden Proben in `tests/belegung.rs` lesen jetzt eine
gemeinsame Liste, die Ausgabeprobe nennt die drei beim Namen, und die Zählprobe mit der 74 im
Namen ist gestrichen statt umgeschrieben. Was übersehen wurde, ist die **Wirkung** der
gefallenen Zusage auf die Ausgabe selbst: dass eine Funktion ohne Kombination aus der Datei
fällt, war bis zum 260812 eine Regel ohne Anwendungsfall, und mit dem ersten Anwendungsfall
hätte sie zur Frage werden müssen. Das Protokoll zu Schritt 4 hat den Zusammenhang sogar
gestreift („eine behauptet, ab Werk sei keine Funktion unbelegt"), und die Antwort war, die
Probe nachzuziehen.

**Die Aufzählungen und ihre Kommentare halten, mit einer Ausnahme.** Fünf neue vollständige
Fallunterscheidungen, alle ohne Auffangzweig, alle mit Begründung, alle den Bau anhaltend.
`ausrichtung` hat seinen Auffangzweig verloren. Übrig bleibt `beschreibbar` als die eine
Stelle, an der ein Modulkopf mehr verspricht als der Code hält — und sie steht ausgerechnet in
der Datei, deren Kopf die Regel aufschreibt.

**Die Doppelspeicherung der Wünsche ist ein bewusster Preis und keine Nachlässigkeit.** Zwei
Stellen halten dieselben fünf Zahlen, und `critical-stance.md` §2 würde das normalerweise als
zweite Wahrheit lesen. Hier trägt es: die eine ist der Wunsch des Nutzers, die andere der
Ausgangspunkt der letzten Auslegung, und ohne die zweite gäbe es nichts, wogegen `neu_auslegen`
die gemessenen Rahmen halten könnte. Sie laufen nachweislich wieder zusammen (die vier Wege
oben). Was fehlt, ist eine Probe, die das misst, und ohne Fenster ist sie nicht zu haben —
`ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her` bildet die Folge der Aufrufe
nach, nicht das Zusammenspiel der beiden Speicher.

---

## Für die Abnahme am laufenden Bündel

Kein Befund, sondern zwei Hinweise auf Stellen, an denen der Augenschein besonders lohnt. Sie
gehören zu der Liste, die der Plan unter `## Abnahme am laufenden Bündel` führt.

**Ein Klick nimmt einen Weg, den kein Tastendruck nehmen kann.** Der Fokusvorbehalt in
`appkit/ereignisse.rs` entscheidet, wem eine **Taste** gehört, und reicht sie an AppKit weiter,
sobald der Ersthelfer eine Textfläche ist. Ein Klick auf einen Schalter der Bereichsleiste geht
an diesem Vorbehalt vorbei, weil er keine Taste ist, und `setRefusesFirstResponder(true)` sorgt
dafür, dass der Ersthelfer dabei stehen bleibt. Das ist gewollt (C2.6), hat aber eine Lage, die
es vorher nicht gab: **ein Klick auf „Links", während in der linken Dateiliste ein Name direkt
in der Zeile umbenannt wird.** Die Fläche mit dem Feldeditor verschwindet, AppKit vergibt den
Ersthelferrang neu, und was aus der angefangenen Umbenennung wird, ist am Code nicht
abzulesen. Dieselbe Lage entsteht mit der Pfadeingabe.

**Die acht Schalter bei 780 Punkten.** Der Plan überschlägt rund 540 Punkte und nennt es
ausdrücklich `speculation:`. `einhaengen` (`bereichsleiste.rs:451`) setzt keine Autogröße und
lässt rechts frei, was frei wird; passen die acht nicht, werden die letzten abgeschnitten,
ohne dass etwas meldet. Betroffen wären zuerst die drei Spaltenschalter.

---

## Empfohlene Reihenfolge

1. **Vor dem Abschluss der Runde: Befund 1 vorlegen.** Er verlangt eine Nutzerentscheidung und
   keine Codeänderung, und er hängt an einer Zusage, die in einer ausgelieferten Datei steht.
   Solange er offen ist, trägt der Datensatz `260812-0306` ein `Implemented:` für etwas, das
   nicht umgesetzt ist, und ein als **(Probe)** gekennzeichnetes Kriterium gilt als erfüllt,
   ohne es zu sein.
2. **Zusammen mit der nächsten Änderung an `spalten.rs`: Befund 2.** Zwei Zeilen, und die
   Datei hält dann, was ihr Kopf verspricht.
3. **Zusammen mit der nächsten Änderung an `anwendung.rs`: Befund 3 und Befund 4.** Beide sind
   Aufräumarbeit; Befund 4 ist im Zweifel eine Kommentarzeile und keine Codeänderung.

**Kein Freigabehindernis** unter den vieren. Keiner ist ein Absturz, ein Datenverlust oder
eine Sicherheitslücke, und keiner hält die Abnahme am laufenden Bündel auf. Die acht
Planschritte liefern, was der Plan verlangt, mit der einen Ausnahme, die Befund 1 ist.

---

## Abgleich 260812-0801 (reconciler)

Alle vier Befunde sind erledigt und ihre Datensätze geschlossen; am Baum nachgelesen:

- **Befund 1** (Markdown-Ausgabe): berichtigt worden sind die drei Zusagen, nicht der Code.
  `belegungsausgabe::markdown` filtert unverändert auf Funktionen mit mindestens einer Kombination
  (`crates/krk-ui/src/belegungsausgabe.rs:178`); Kriterium C3.5 des Plans, der Kommentarblock in
  `resources/default-keymap.toml:307-317` und die Zeile `Implemented:` in
  `decisions/260812-0306_i_bekommen-die-spaltenschalter-tastenbefehle.md` sagen jetzt dasselbe wie
  der Code. **Die Empfehlung „Befund 1 vor dem Abschluss der Runde vorlegen" ist damit gegenstandslos:**
  der Datensatz trägt kein `Implemented:` mehr für etwas Nichtumgesetztes.
- **Befund 2** (`beschreibbar`): ausgeschriebenes `match` über alle vier Werte
  (`crates/krk-ui/src/spalten.rs:106`).
- **Befund 3** (doppelte Erreichbarkeitsprüfung): `editor_ist_ansprechbar`
  (`crates/krk-ui/src/appkit/anwendung.rs:1488`) ist die eine Fassung; beide Aufrufer fragen sie.
- **Befund 4** (doppelter Nachzug): `Leistenquelle::geklickt`
  (`crates/krk-ui/src/appkit/bereichsleiste.rs:282`) nimmt die Selbstkippung zurück, bevor es
  meldet; `bereichsleiste_nachziehen` hat genau einen Aufrufer (`anwendung.rs:2780`).

Die beiden Hinweise unter `## Für die Abnahme am laufenden Bündel` bleiben offen und sind
Nutzerarbeit: der Klick während einer laufenden Umbenennung in der Zeile, und ob die acht Schalter
bei 780 Punkten nebeneinander passen.
