Die eine Statuszeile zeigt Meldungen eines ausgeblendeten Dateifensters

---

`Anwendungsdelegierter::statuszeile_nachziehen`
(`crates/krk-ui/src/appkit/anwendung.rs:3073-3097`) holt die Meldungsquellen
**beider** Dateifenster, ohne zu fragen, ob beide sichtbar sind. Ist eines
ausgeblendet, bewerben sich seine fünf Quellen weiter um die eine Zeile, und
gewinnt eine von ihnen, steht in der Zeile eine Meldung über einen Bereich, den
der Nutzer nicht sieht — mit dem Namenszusatz „rechtes Dateifenster: …" davor.

---

**Der Weg dorthin, Schritt für Schritt am Baum:**

1. Im rechten Dateifenster steht ein Ordner, der sich nicht lesen lässt. Das
   setzt die Tabmeldung, Rang 4.
2. Der Nutzer blendet das rechte Dateifenster über die Bereichsleiste aus.
   `Fenstermodell::umschalten` (`crates/krk-ui/src/fenstermodell.rs:639-668`)
   setzt die Sichtbarkeit und schiebt das aktive Dateifenster nach links. Die
   vier Meldungsfelder der Quelle bleiben unangetastet — `QuelleIvars` wird
   dabei nicht angefasst, und das ist auch richtig, weil C5.7 verlangt, dass
   Verdrängtes nicht gelöscht wird.
3. `aufteilung_nachziehen` ruft `statuszeile_nachziehen`
   (`anwendung.rs:3006-3011`), das beide Quellensätze holt.
4. Hat das linke Dateifenster nichts über Rang 4 zu sagen, gewinnt die
   Tabmeldung des ausgeblendeten rechten. In der Zeile steht dann dauerhaft und
   rot: „rechtes Dateifenster: Ordner nicht lesbar".

Derselbe Weg über Rang 5: im rechten Dateifenster markieren, dann ausblenden —
die Zeile meldet den Markierungsstand eines Dateifensters, das nicht dasteht.

**Was daran gegen den Plan steht.** Das achte Abnahmekriterium von C5 lautet:
„Die Meldung nennt ihr Dateifenster genau dann im Text, wenn sie nicht vom
aktiven kommt. **Steht nur ein Dateifenster, ist es das aktive, und kein Satz
trägt einen Zusatz.**" Der zweite Satz ist in dieser Lage falsch: es steht nur
ein Dateifenster, und der Satz trägt trotzdem einen Zusatz. Der Doc-Kommentar
an `statuszeile::zeilentext`
(`crates/krk-ui/src/appkit/statuszeile.rs:384-390`) wiederholt die Zusage
wörtlich und begründet damit, warum der Fall „nur ein Dateifenster" keinen
eigenen Zweig braucht. Er braucht einen.

**Eine Verschlechterung gegenüber der Lage vor der Runde.** Bis zur Runde 6 saß
die Zeile am Fuß ihres Dateifensters und wurde mit ihm ausgeblendet; die
Meldung war weg, solange der Bereich weg war, und kam mit ihm zurück. Die
Zusammenlegung hat diese Kopplung gelöst, und der Plan hat den Fall nicht
behandelt: Schritt 10 nennt Sichtbarkeit an keiner Stelle.

**Die Probe dazu setzt die Voraussetzung, statt sie zu messen.**
`statuszeile.rs:906-923`, `steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz`,
übergibt für das ausgeblendete Dateifenster `Quellen::default()` und begründet
das im Doc-Kommentar mit „das ausgeblendete Dateifenster meldet nichts". Genau
diese Voraussetzung stellt `statuszeile_nachziehen` nicht her: es holt die
echten Quellen. Die Probe hält damit eine Regel fest, die im Programm nicht
gilt, und sie würde grün bleiben, wenn jemand den Fall behöbe oder verschlimmerte.

**Drei Zuschnitte, keiner ist hier gewählt:**

1. **Die unsichtbare Seite nicht befragen.** `statuszeile_nachziehen` übergibt
   für ein ausgeblendetes Dateifenster `Quellen::default()`. Zwei Zeilen, und
   C5.8 stimmt wieder wörtlich. Preis: eine Meldung, die während der
   Ausblendung eintrifft, ist unsichtbar — sie steht aber im Feld und erscheint
   beim Einblenden, was genau die Zusage von C5.7 ist.
2. **Die Sichtbarkeit in `statuszeile::zeile` hineinreichen.** Die Regel bleibt
   an einer Stelle und wird ohne Fenster prüfbar; die Signatur bekommt zwei
   `bool`. Etwas mehr Aufwand, dafür misst eine Probe die neue Regel.
3. **So lassen und C5.8 umformulieren.** Dann ist zu sagen, warum eine Meldung
   über einen unsichtbaren Bereich nützlich ist. Der Nutzer kann auf sie nicht
   reagieren, ohne den Bereich erst wieder einzublenden.

**Gewicht:** mittel. Kein Absturz, aber eine dauerhafte rote Zeile über etwas,
das nicht zu sehen ist, und ein Abnahmekriterium, das in dieser Lage nicht
hält.

**Herkunft:** Circle der Runde 6, Planschritt 10 (C5.8).

---
Resolved: Behoben am 260812, Zuschnitt 2 aus der Liste oben — die Sichtbarkeit reist in `statuszeile::zeile` hinein, statt beim Aufrufer eine Bedingung zu ziehen. Damit steht die Regel an einer Stelle und ist ohne Fenster prüfbar; Zuschnitt 1 wäre zwei Zeilen kürzer gewesen und von keiner Probe zu erreichen, also genau die Lage, an der dieser Defekt vorbeigelaufen ist.

Die Signatur lautet jetzt `zeile(links, rechts, aktiv, sichtbar: &Sichtbarkeit)`, und die Schleife über die beiden Seiten überspringt, wer nicht dasteht. Gefragt wird über `fenstermodell::sichtbar_in` und `Bereich::von_seite`, also über die eine Zuordnung von einem Bereich auf sein Feld; eine zweite Zuordnung von einer `Fensterseite` auf ein `bool` entsteht nicht. `Anwendungsdelegierter::statuszeile_nachziehen` holt `aktiv` und `sichtbarkeit()` in **einer** Ausleihe, wie `bereichsleiste_nachziehen` daneben es hält, und fragt weiterhin beide Dateifenster nach ihren Quellen — verworfen wird die Antwort des ausgeblendeten dort, wo die Regel steht.

**C5.8 stimmt damit wieder wörtlich**, und zwar der zweite Satz: steht nur ein Dateifenster, ist es das aktive, und kein Satz trägt einen Zusatz. Getragen wird das von zwei Zusagen zusammen — ein ausgeblendetes Dateifenster bewirbt sich nicht, und das aktive ist immer sichtbar. Die zweite stellt das Modell an beiden Wegen dorthin her: `Fenstermodell::umschalten` gibt die Aktivität ab, wenn das aktive ausgeblendet wird, und `Fenstermodell::aus_sitzung` zieht sie nach, wenn eine von Hand geschriebene `session.toml` sie auf ein ausgeblendetes zeigen lässt. Beide Stellen sind unverändert; hier wird auf sie verwiesen und keine dritte gebaut. Der Doc-Kommentar an `zeilentext`, der bis zum 260812 behauptete, der Fall folge schon aus der einen Bedingung, sagt jetzt, wo er wirklich eingelöst wird.

**C5.7 bleibt unangetastet.** Die vier Meldungsfelder der ausgeblendeten Seite werden nicht geräumt, und ihre Meldung steht wieder in der Zeile, sobald der Bereich zurückkommt.

**Die Probe misst die Voraussetzung jetzt, statt sie zu setzen.** `steht_nur_ein_dateifenster_traegt_kein_satz_einen_zusatz` übergibt dem ausgeblendeten Dateifenster einen Quellensatz, und zwar einen mit dem **höheren** Rang; grün ist sie nur, wenn er trotzdem stumm bleibt. Drei Proben kommen dazu: der Weg dieses Defekts Schritt für Schritt (Tabmeldung rechts, rechts ausgeblendet, Zeile leer), die Rückkehr derselben Meldung beim Einblenden (C5.7) und eine Runde über alle fünf Ränge in beide Richtungen. Der Helfer, der die Sichtbarkeit herstellt, prüft sich selbst gegen `sichtbar_in`, damit die Probe die Zuordnung nicht ein zweites Mal aufschreibt.

Abnahme am 260812: `cargo build --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace` je Exit 0. 457 Proben im Binärziel `krk` gegenüber 454 vorher, 22 in `statuszeile.rs` gegenüber 19; keine der neunzehn ist weggefallen. Noch nicht committet, der Nutzer committet nach der Aufgabe.
