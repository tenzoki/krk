# Dritte Nachdurchsicht vor der Auslieferung: die Beispielzahl vier hält, und die drei Begründungen tragen

**Reviewed-range:** `75ba8e2..96e32cb`
**Not-opened:** `resources/default-readers.toml`

**Durchgesehen von:** coderev, Kai Stalmann <kai@qantr.com>
**Am:** 260826-0139
**Gelesen gegen:** `shared/issues/260825-2233_c_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md` samt Nachtrag vom 260826-0210, die zwei Vorgängerberichte `shared/reviews/260825-2127-coderev-…` und `260825-2230-coderev-…`, die Kostenmessung `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md` und die zwanzig offenen Datensätze unter `shared/issues/`.
**Zum Bereich:** `255ad7a` ändert allein `resources/default-readers.toml`; diese Datei liest `ontorev` in einer eigenen Durchsicht und ist hier nur insoweit geöffnet, wie die Probe sie über `AUSLIEFERUNGSTEXT` misst (die Profilblöcke `:303-333` und `:640-670`). Die vier Datensätze und zwei Verlaufsdateien unter `fusion-workbench/` sind gelesen und nicht beurteilt.

---

## Zusammenfassung

Die drei Punkte, die der Coder zur Prüfung vorgelegt hat, tragen alle drei, und
jeden habe ich selbst nachgemessen statt ihn zu glauben: die fünf Öffnungen
gegen die vier der Kostenmessung sind an `.active-circle` begründet, der Ausweis
für das gegriffene Profil trennt die zwei Profile wirklich, und die
zusammengelegte Bestandsfunktion verwischt nichts. Zwei Befunde, beide gering,
beide in der Prüfdatei und keiner an einem ausgelieferten Byte. **Aus Sicht des
Codes kann dieser Stand ausgeliefert werden.**

---

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 0 |
| Gering | 2 |

`96e32cb` fasst allein `crates/krk-core/tests/leseprofil.rs` an, also eine
Prüfdatei. Kein Byte des Bündels ändert sich durch diesen Commit.

**Selbst gefahren am 260826-0139:** `cargo fmt --all --check` sauber,
`cargo clippy --workspace --all-targets` ohne eine Meldung,
`cargo test --workspace` mit 23 Prüfzielen und keiner gescheiterten Probe,
darunter `leseprofil` mit 47 Proben.

---

## Die drei vorgelegten Punkte

### 1. Fünf Öffnungen statt der vier der Kostenmessung — die Begründung trägt

**Nachgemessen und nicht nachgelesen.** Ich habe die Rechnung nicht aus dem
Quelltext abgeleitet, sondern ein eigenes Cargo-Paket im Wegwerfverzeichnis
dieser Sitzung gegen `krk-core` gebaut, das ausschließlich öffentliche
Schnittstellen ruft (`AUSLIEFERUNGSTEXT`, `leseprofil::datei::pruefen`,
`zusammenfassen_gezaehlt`) und den Prüfordner der Probe nachbaut. Der KRK-Baum
ist dabei nicht angefasst worden. Fünf Gestalten, fünf Messungen:

| Gestalt | Leseläufe | Öffnungen |
|---|---|---|
| Projektwurzel, voller Bestand (das ist der Prüfordner der Probe) | 4 | 5 |
| dieselbe ohne `.active-circle` | 4 | 4 |
| Projektwurzel mit leerem `fusion-workbench` | 2 | 0 |
| Werkbankwurzel, voller Bestand | 3 | 5 |
| die wirkliche Projektwurzel `/Users/k1/Projects/productive/krk` | 4 | 4 |

Alle drei Zahlenpaare, die der Coder in den Doc-Kommentar geschrieben hat,
stimmen an der Ziffer, und die vierte Zeile bestätigt die Kostenmessung vom
260825-2107, die für Profil 8 vier und vier zählt.

**Der Grund ist der, den der Coder nennt, und er steht im Code.**
`Lauf::feld` (`crates/krk-core/src/leseprofil/bausteine.rs:675-690`) sucht den
Eintrag erst und bucht die Öffnung danach:

```rust
let Some(eintrag) = stand.eintraege.iter().find(…) else {
    return Wert::Nicht;
};
if !self.buchen(|haushalt| haushalt.oeffnungen_nehmen(1)) {
```

Eine Zeile, deren Datei im Ordner nicht steht, kommt an `oeffnungen_nehmen` gar
nicht heran. `krk/fusion-workbench` führt kein `.active-circle`, also fällt dort
die vierte der fünf Feldzeilen weg. Die **Leseläufe** sind in beiden Gestalten
vier, und um sie geht der Datensatz.

**Zwei Anmerkungen, keine Befunde.** Erstens: der dritte Fall der Probe hält für
die Werkbankwurzel `(3, 5)`, während die Kostenmessung dort vier Öffnungen
zählt — dieselbe Abweichung aus demselben Grund, eine Fallhöhe darüber und ohne
Notiz. Sie ist gedeckt, denn der Modulkopf der Probendatei (`:28-34`) sagt schon,
dass die Proben eine **Gestalt** mit bekanntem Bestand bauen und nicht die
wirkliche Werkbank messen. Zweitens: der Prüfordner der Probe ist die
großzügigere Gestalt, also misst die Probe die obere und nicht die untere Kante
des Haushalts. Das ist die richtige Richtung für eine Schrankenprobe.

### 2. Der Ausweis für das gegriffene Profil — er trennt, und die Zahlen trennen früher als die Werte

**Der Coder beschreibt die Lage richtig.** Wurzel- und Projektwurzelprofil
führen dieselben sieben Beschriftungen (`resources/default-readers.toml:307-333`
und `:644-670`), also kann die Beschriftungsliste (`:3234-3245` der Probe) die
zwei nicht auseinanderhalten. Der Doc-Kommentar sagt das ausdrücklich und
schiebt den Ausweis auf die Werteliste.

**Gegenprobe gefahren.** Ich habe den Block des Wurzelprofils aus
`AUSLIEFERUNGSTEXT` herausgeschnitten, sein Kennzeichen auf `^fusion-workbench$`
gesetzt und es an einer Projektwurzel messen lassen, also genau die Lage
hergestellt, gegen die die Probe rot werden muss. Ergebnis: ein Leselauf, keine
Öffnung, und siebenmal `Wert::Nicht`. Die Probe würde damit **vor** der
Werteliste rot, nämlich schon an
`(haushalt.leselaeufe(), haushalt.oeffnungen()) == (4, 5)` (`:3246-3251`). Der
Ausweis liegt also doppelt: an den Zahlen und an den Werten.

**Der Fall kann in der Auslieferungsfassung gar nicht eintreten**, und auch das
sagt der Coder richtig: das Wurzelprofil erkennt über `^\.fusion-setup$`, und an
der Projektwurzel steht diese Datei nicht. Nachgezählt, welche Profile mit
Kennzeichendatei vor dem Projektwurzelprofil stehen: genau die zwei, die der
Doc-Kommentar von `projektwurzel` nennt, nämlich `^\.fusion-setup$` an Stelle 1
und `^_._circle\.md$` an Stelle 7 (`grep -n '^kennzeichen' resources/default-readers.toml`).
Die Aufzählung ist vollständig. Der erste Erkennungsdurchgang über Pfadmuster
kommt nicht in Frage: keines der sieben Pfadmuster trifft einen Pfad unter
`std::env::temp_dir()`.

**Die Begründung, warum nicht gegen `wurzelwerte` verglichen wird, trägt.** Ein
Vergleich der zwei Wertelisten wäre eine Zusage, dass die zwei Profile dieselben
Werte liefern; die Datei gibt sie ausdrücklich nicht (`:296-302` und `:636-639`,
beide Male: „nichts hält die beiden Blöcke aneinander"). Nebenbei: der dritte
Fall schreibt seine Werteliste ebenfalls aus (`:3132-3155`), die zwei Fälle sind
also gleich gebaut.

### 3. `werkbankbestand(&Path)` mit zwei Rufern — nichts verwischt

**Die Zusammenlegung ist deckungsgleich mit dem alten Stand.** Die zwei
Schreibwege, die dabei getauscht wurden, tun dasselbe: `Pruefordner::datei`
(`crates/krk-core/tests/gemeinsam/mod.rs:96-100`) ist `fs::write` auf
`unter(name)`, und `schreiben` (`crates/krk-core/tests/leseprofil.rs:1108-1112`)
ist `fs::write` auf `ordner.join(name)`; `Pruefordner::ordner` (`:127-131`) ist
`create_dir_all`, und der neue Rumpf ruft `create_dir_all` unmittelbar. Der
Bestand nach dem Lauf ist Byte für Byte derselbe. Das `create_dir_all(wurzel)`
am Anfang ist für `werkbankwurzel` ein Leerlauf und für `projektwurzel` nötig.

**Die dritte Gegenprobe belegt, was sie belegen soll, aber nur das.** Streicht
man `.active-circle` aus `werkbankbestand`, fällt die Öffnung an beiden Rufern
weg: die Werkbankwurzel liefert dann `(3, 4)` statt `(3, 5)`, die Projektwurzel
`(4, 4)` statt `(4, 5)`. Beide Behauptungen werden rot, und weil der Wurzelfall
im Rumpf zuerst steht, meldet `libtest` ihn zuerst. Das belegt, dass **beide**
Fälle an diesem einen Bestand hängen, also dass die Zusammenlegung wirklich
eine ist. Es belegt nicht, dass sonst nichts verwischt wäre; dafür steht der
Absatz darüber, der die zwei Schreibwege gegeneinander liest.

**Der Doc-Kommentar begründet die Zusammenlegung mit der Sache und nicht mit
der Ersparnis** („ein zweiter, von Hand gepflegter Bestand daneben liefe von
diesem weg, und die zwei Messungen verglichen dann nicht mehr dieselbe
Gestalt"). Das ist der Grund, der trägt: die Vier des Datensatzes ist die Drei
**derselben** Gestalt plus eins, und mit zwei Beständen wäre sie das nicht mehr.

### 4. Der `step_by(2)`-Punkt aus dem eigenen letzten Bericht

**Ja, der vierte Fall trägt dieselbe Kopplung, und ja, sie ist ebenso rot mit
irreführender Meldung.** Die Werteliste (`:3265-3282`) prüft sieben Werte über
ihre Stellung; die Stellung kommt aus der Reihenfolge der sieben Zeilen in
`resources/default-readers.toml`. Die Beschriftungsliste darüber fängt eine
Umstellung nicht ab, weil sie gegen `projektwurzelprofil.zeilen()` vergleicht,
also gegen eine Liste, die sich mitdreht. Bei einer Umstellung wird die Probe
rot und meldet, eine Zeile habe nichts gefunden — was dann nicht stimmt.

Beide Stellen stehen jetzt in einem Datensatz, statt weiter nur in einem
Bericht zu wohnen. Siehe Befund G2.

---

## Befunde

### G1 — `genannte_orte` hat zwei Rufer und nennt in Doc und Meldung nur das Speicherprofil

`shared/issues/260826-0139_o_genannte-orte-hat-seit-96e32cb-zwei-rufer-und-nennt-in-doc-und-meldung-nur-das-speicherprofil.md`

`crates/krk-core/tests/leseprofil.rs:2921-2950`. Die Funktion sammelt seit
`96e32cb` auch die Orte des Projektwurzelprofils (`:3212`), ihr Doc-Kommentar
spricht aber weiter von „Unterspeichern" und ihre Abbruchmeldung von „einem Ort
des **Speicherprofils**". Träte der Abbruch am zweiten Rufer ein, benennte er
das falsche Profil und schickte die Suche an die falsche Stelle. Reine
Prüfdatei; behebbar über `profil.name()` in der Meldung.

**Schwere: gering.**

### G2 — Zwei Behauptungen der C6.7-Probe hängen an der Zeilenreihenfolge und melden etwas anderes

`shared/issues/260826-0139_o_zwei-behauptungen-der-c6-7-probe-haengen-an-der-zeilenreihenfolge-der-profildatei-und-melden-etwas-anderes.md`

`crates/krk-core/tests/leseprofil.rs:3202-3208` (die `step_by(2)`-Behauptung am
Speicherprofil, im Bericht vom 260825-2230 ohne Datensatz vermerkt) und
`:3265-3282` (die Werteliste des Projektwurzelprofils, neu in `96e32cb`). Beide
werden bei einer Umstellung der Profildatei rot und melden einen anderen Fehler
als den, der vorliegt. Die Richtung ist die ungefährliche; die Auskunft ist es
nicht.

**Schwere: gering.**

---

## Beobachtungen ohne Datensatz

- **Der Probenname sagt „die drei größten" und die Probe misst vier Fälle.**
  `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`
  (`:3048`). Der Name ist nicht falsch geworden, nur unvollständig, und der
  Doc-Kommentar geht die Frage von sich aus an: „Der vierte Fall ist nicht der
  eines der größten Profile. Er steht hier, weil …" (`:3007-3017`). Das ist eine
  bewusste und ausgeschriebene Wahl; ein Datensatz dagegen wäre Lärm.
- **Der Prüfordner der Probe ist die großzügigere Gestalt.** Siehe Punkt 1; die
  Probe misst damit die obere Kante des Haushalts, und das ist für eine
  Schrankenprobe die richtige Seite.

---

## Bindungen, nachgesehen

- **`#[must_use]`:** `werkbankbestand` gibt nichts zurück, `projektwurzel` und
  `werkbankwurzel` geben den Prüfordner zurück und werden gebunden;
  `profil_der_auslieferung` liefert eine Referenz, die sofort weiterverwendet
  wird. Kein neues `let _ =` im Bereich.
- **Vollständige Fallunterscheidungen:** `genannte_orte` verzweigt weiter über
  alle vier `Baustein`-Werte ohne Auffangzweig; `96e32cb` fasst den Zweig nicht
  an.
- **Untergrenzen-Abschnitt:** kein Modulkopf unter
  `crates/krk-ui/src/appkit/` ist im Bereich angefasst.
- **Zahlen in Prosa:** die neuen Zahlen des Doc-Kommentars — vier, fünf, drei
  Orte, zwei Läufe am leeren `fusion-workbench` — sind einzeln nachgemessen
  (Tabelle unter Punkt 1). Die vier und die fünf hält die Probe selbst; die zwei
  Nebenmessungen hält keine Probe und sind im Doc-Kommentar ausdrücklich als
  Messung mit Datum ausgewiesen.
- **`Cargo.lock`:** im Bereich unverändert.
- **Kein Prüfordner-Erzeuger hinzugekommen:** `werkbankbestand` schreibt in
  einen `Pruefordner` der einen Fassung des Kerns und ist keine vierte Fassung.

---

## Auslieferung

**Aus Sicht des Codes: ja, dieser Stand kann ausgeliefert werden.**

Die Begründung in drei Sätzen. `96e32cb` ändert allein eine Prüfdatei, also
kein Byte, das ins Bündel geht. `cargo fmt`, `cargo clippy --all-targets` und
`cargo test --workspace` sind am 260826-0139 selbst gefahren und alle drei
sauber. Die zwei Befunde dieser Durchsicht sind gering, liegen beide in der
Prüfdatei und ändern an keiner Zusage der Anwendung etwas.

Was diese Auskunft **nicht** deckt, und das gehört dazu:

- `255ad7a` ändert `resources/default-readers.toml`, und die Datei geht ins
  Bündel und wird beim ersten Start wörtlich ins Heimatverzeichnis kopiert. Sie
  liegt bei `ontorev`; ohne deren Durchsicht ist über den Handbuchteil hier
  nichts gesagt.
- Die zwanzig offenen Datensätze vom 260824-1745 bis 260826-0128 stehen
  unverändert. Keiner davon ist in dieser Durchsicht neu bewertet worden, und
  ob einer von ihnen die Auslieferung aufhält, ist eine Frage an den Nutzer und
  nicht an diese Durchsicht.
- Nichts über das Gesehene. Der Abnahmelauf verlangt KRK im Vordergrund und ist
  Nutzerarbeit.

---

**Vermerk des reconciler, 260826-0149.** Die Freigabe ist gegen den Baumstand `e5ec81a`
nachgehalten und bestätigt: `make check` selbst gefahren, Ausstiegscode 0, „alle vier gruen".
Die zwei niedrigen Befunde dieser Durchsicht stehen unverändert offen
(`shared/issues/260826-0139_o_genannte-orte-hat-seit-96e32cb-zwei-rufer-…` und
`…_zwei-behauptungen-der-c6-7-probe-haengen-an-der-zeilenreihenfolge-…`); beide liegen in
`crates/krk-core/tests/leseprofil.rs` und berühren kein ausgeliefertes Byte. Über den ganzen
Sitzungsbereich `20eccd4..e5ec81a` liegt jeder Codecommit in einem Durchsichtsbereich; die
sieben ungedeckten Commits sind reine Werkbankcommits. Was für einen Auslieferungslauf trotzdem
fehlt, ist Nutzerarbeit und steht im Reconciliation Log des Plans
(`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`).
