# Ontoreview: Dritte Nachdurchsicht der Profildatei vor der Auslieferung

**Reviewed-range:** `75ba8e2..96e32cb`
**Not-opened:** none

**Sender:** ontorev
**Gegenstand:** `resources/default-readers.toml` nach `96e32cb`, gegen die drei
`Resolved:`-Vermerke unter `shared/issues/260825-2233_c_*.md` und gegen den Quelltext in
`crates/krk-core/src/leseprofil/{datei,erkennung,bausteine,mod}.rs`. Von
`crates/krk-core/tests/leseprofil.rs` ist die geänderte Probe zu C6.7 gelesen, weil N3 sie zur
Bedingung hat; die Probendatei als Ganzes gehört der Durchsicht durch `coderev` und ist hier
nicht bewertet.

## Summary

Alle drei niedrigen Befunde der zweiten Durchsicht sind erledigt, und jeder Vermerk hält der
Nachmessung stand: die zwei Lagen von N1 sind einzeln gemessen und stehen beide im Satz, der
falsche Halbsatz von N2 ist weg und der Preis daneben unverändert, und die Herleitung von N3 ist
unverändert richtig — die Probe, die `96e32cb` dafür gebaut hat, misst genau sie und nicht eine
Nachbaraussage. Ein neuer Befund, niedrig, an derselben Zeile, die N1 erzeugt hat: die
Fallunterscheidung ist wahr, trägt ihre Bedingung aber nur an einem der zwei Zweige.
**Die Datei kann ausgeliefert werden.**

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 1 |

Der neue steht unter
`shared/issues/260826-0139_o_die-fallunterscheidung-des-kennzeichnen-satzes-traegt-ihre-bedingung-nur-an-einem-zweig.md`.
Die drei niedrigen Befunde der **ersten** Durchsicht (`260825-2126_o_*`: `.DS_Store`,
flight-Doppelungshinweis, drei Felder) sind unverändert offen und hier nicht gedoppelt; keine
Änderung dieser Runde berührt einen davon. Ebenso nicht gedoppelt: die Lücke im Modulkopf von
`datei.rs`, die der Orchestrator inzwischen selbst als `260826-0128_o_*` abgelegt hat.

## Wie geprüft wurde

Nicht durch Hinsehen. Die Messhilfe der vorigen Durchsichten lag im damaligen
Sitzungsverzeichnis und ist neu gebaut: eine eigene Kiste im Sitzungsverzeichnis mit einer
Pfadabhängigkeit auf `krk-core`, die eine beliebige Profildatei über `toml::from_str` und
`leseprofil::datei::pruefen` lädt und danach `leseprofil::bausteine::zusammenfassen_gezaehlt` an
genannten Orten fährt. Der Baum ist unverändert; die Profildatei ist nicht angefasst, und kein
Git-Kommando ist über den ganzen Baum gelaufen.

Gefahren gegen die ausgelieferte Fassung an vier wirklichen Orten und vier künstlichen
Projektwurzeln sowie gegen fünf abgewandelte Fassungen der Datei.

`cargo test -p krk-core --test leseprofil`: 47 grün, 1 übersprungen.
`cargo test -p krk-core --lib leseprofil`: 10 grün.

## Die drei Behebungen, einzeln nachgemessen

### 1. N1 — der Vorbehalt für `pfad` (`:61-67`): behoben, beide Lagen stimmen

Der Satz sagt jetzt: „`kennzeichnen` statt `kennzeichen` nimmt dem Profil sein Erkennungsmuster,
und es fällt in die zweite Reichweite oder greift, steht ein `pfad` daneben, still über diesen
allein". Beide Lagen gemessen:

| Fassung | Profile | Meldungen | gemessener Ort | Ergebnis |
|---|---|---|---|---|
| `kennzeichnen = 'x'` **neben** dem `pfad` der zwei Speicherprofile (`:352`, `:375`) | 12 | keine | `fusion-workbench/shared/history` | Profil greift, 1 Leselauf, 10 Öffnungen |
| `kennzeichnen` **statt** `kennzeichen` am Wurzelprofil (`:305`), ohne `pfad` | 11 | „es nennt weder ein Pfadmuster noch eine Kennzeichendatei" | `fusion-workbench` | kein Profil |

Das ist Zeile für Zeile, was der `Resolved:`-Vermerk behauptet. Der Mechanismus ist nicht
angefasst: `Profilblock` trägt weiter kein `deny_unknown_fields`, und `pruefen` meldet weiter
allein den Fall `pfad.is_none() && kennzeichen.is_none()`.

**Der Satz ist in der Sache genauer geworden, und zwar über den Befund hinaus.** Vor `255ad7a`
hing der Schluss „und das ohne jede Meldung" an einer Aufzählung, deren erstes Glied der
`kennzeichnen`-Fall war — und der erzeugt eine Meldung, siehe zweite Zeile der Tabelle. Das neue
Semikolon trennt ihn ab. Diese zweite Ungenauigkeit hat die vorige Durchsicht nicht gesehen; sie
ist mitbehoben.

**Lesbar ist der Satz danach schlechter, und darum geht der neue Befund.** Er trägt jetzt eine
Fallunterscheidung mit zwei Ausgängen, und die Bedingung, die zwischen ihnen entscheidet, steht
allein im zweiten: „es fällt in die zweite Reichweite" liest sich unbedingt, und „steht ein
`pfad` daneben" ist zwischen das Verb `greift` und dessen Nachsatz `still über diesen allein`
geschoben. Beide Zweige sind gemessen disjunkt und vollständig — der Satz sagt es nur nicht. Ein
`dann` im ersten Zweig genügt. Datensatz `260826-0139_o_*`, Schwere **niedrig**, kein
Auslieferungshindernis.

Ein zweiter Vorbehalt zum selben Absatz, ausdrücklich **kein** Befund: der spiegelbildliche Fall
ist nicht ausgeschrieben. Gemessen an `pfd` statt `pfad` am Defektspeicherprofil: mit einem
`kennzeichen` daneben → 12 Profile, keine Meldung, das Profil greift über das `kennzeichen`
allein; ohne `kennzeichen` daneben → 11 Profile und dieselbe Meldung. Der Mechanismus ist also
symmetrisch, und die Symmetrie trägt in der Datei allein der Satz `:61-63` „Was er kostet, ist
das, was der Schreibfehler weggenommen hat". Siehe die Antwort auf die zweite Frage unten.

### 2. N2 — der gestrichene Halbsatz (`:627-634`): behoben, der Preis steht vollständig

„also der Zustand vor `/fusion:setup`" ist weg. Der Absatz nennt jetzt zwei Zustände, und beide
kommen vor. Nachgemessen an vier künstlichen Projektwurzeln:

| Projektwurzel enthält | Profil | Leseläufe | Öffnungen | Zeilen |
|---|---|---|---|---|
| leeres Verzeichnis `fusion-workbench` | Projektwurzel mit fusion-Werkbank | 2 | 0 | 7 × `--` |
| Datei `fusion-workbench` | Projektwurzel mit fusion-Werkbank | 2 | 0 | 7 × `--` |
| leeres Verzeichnis `flight-workbench` | Projektwurzel mit flight-Werkbank | 2 | 0 | 7 × `--` |
| `fusion-workbench` mit Unterordnern, ohne `.fusion-setup` | Projektwurzel mit fusion-Werkbank | 3 | 0 | 6 × `--`, „Runden: 0" |

Die zwei verbliebenen Beispiele sind damit nicht bloß richtig, sondern **genau**: sie sind die
zwei Gestalten, die die vollen sieben Platzhalterzeilen liefern. Die vierte Zeile der Tabelle
zeigt, dass ein halb gefülltes Verzeichnis weniger liefert — sie stünde als drittes Beispiel
falsch da und steht zu Recht nicht da.

Der Absatz steht nach dem Eingriff nicht schief. Der gestrichene Einschub war eine Begründung,
kein Satzglied: „Ein leeres `fusion-workbench` oder eine Datei dieses Namens genügt, und die
Vorschau zeigt statt der Metadaten sieben Zeilen Platzhalter" trägt Subjekt, Preis und Folge
vollständig, und der Satz danach („Der Preis ist in Kauf genommen, weil …; wer ihn nicht zahlen
will, streicht dieses Profil") hängt an keinem der gestrichenen Wörter. Was der Absatz verloren
hat, ist nicht Genauigkeit, sondern Dringlichkeit — er nennt den Preis jetzt, ohne ihn
wahrscheinlich zu machen. Das war die bewusst gewählte Möglichkeit 1 des Datensatzes, und sie ist
die richtige: die Alternative hätte einen zweiten Zustand behauptet, um den ersten zu ersetzen.

Der Absatz verlor dabei eine Zeile, N1 gewann eine. Das ist die Rechnung, die die Datei bei
801 Zeilen hält.

### 3. N3 — die Beispielzahl vier (`:236-240`): unverändert, und die Begründung stimmt

Die Frage der Dispatch ist, ob die Herleitung im Kommentar mit dem übereinstimmt, was die neue
Probe misst. **Sie stimmt, und zwar an beiden Hälften.**

Der Kommentar leitet her: die Zahl der Leseläufe ist die Zahl der **verschiedenen** genannten
Orte, plus einen Lauf für die Erkennung, wenn das Profil über sein `kennzeichen` erkannt wurde
und keine seiner Zeilen den erkannten Ordner selbst nennt; das Projektwurzelprofil kostet
deshalb vier, „dort trägt jede Zeile eine Ortsangabe, und den erkannten Ordner liest allein die
Erkennung".

Die Probe hält vier Aussagen, und sie sind die Glieder genau dieser Herleitung:

| Zusicherung der Probe | Glied der Herleitung |
|---|---|
| `projektorte == ["fusion-workbench", "fusion-workbench/circles", "fusion-workbench/shared/issues"]` | die drei verschiedenen Orte |
| `!projektorte.iter().any(String::is_empty)` | „keine seiner Zeilen nennt den erkannten Ordner selbst" |
| `(leselaeufe, oeffnungen) == (4, 5)` | die Zahl selbst |
| `leselaeufe == projektorte.len() + 1` | „plus einen Lauf für die Erkennung" |

Die dritte Zusicherung zählt, die vierte leitet her — das ist der Unterschied, den N3 verlangt
hat, und die Probe hält beide nebeneinander statt eine von der anderen abzuleiten.

Selbst nachgemessen, unabhängig von der Probe:

| Ort | Profil | Leseläufe | Öffnungen |
|---|---|---|---|
| `krk` | Projektwurzel mit fusion-Werkbank | 4 | 4 |
| `krk/fusion-workbench` | fusion-Werkbank: die Wurzel | 3 | 4 |

Und eine Gegenprobe am Mechanismus, die die Probe nicht fährt: gibt man dem
Projektwurzelprofil eine achte Zeile **ohne** `ordner`, also eine, die den erkannten Ordner
selbst nennt, so steigt die Zahl der verschiedenen Orte auf vier, der Erkennungslauf teilt sich
mit ihr — und die Zahl bleibt **vier**. Die Regel hält also in beiden Zusammensetzungen, und
nicht nur an der einen, aus der die Beispielzahl stammt.

Die Drei des Wurzelprofils hält die Probe weiter genau (`(3, 5)`). Eine eigene
Herleitungszusicherung hat sie nicht, und sie braucht keine: gäbe man den fünf `feld`-Zeilen des
Wurzelprofils eine Ortsangabe, ginge die Drei auf Vier und die Probe würde rot.

**Die Fünf neben der Vier ist keine Aussage der Profildatei.** Die Probe hält `(4, 5)`, die
Kostenmessung an der wirklichen Werkbank zählt `(4, 4)`; der Unterschied ist das fehlende
`.active-circle`, und der Doc-Kommentar der Probe schreibt ihn aus. Die Profildatei spricht an
dieser Stelle allein von Läufen. Kein Widerspruch.

## Die zwei Fragen darüber hinaus

### Der Kommentaranteil und die Lesbarkeit

Nachgezählt an vier Ständen, jeweils über `git show`:

| Stand | Zeilen | Kommentarzeilen | Anteil |
|---|---|---|---|
| `75ba8e2` (vor dieser Runde) | 801 | 433 | 54,1 % |
| `1ac5dde` | 801 | 433 | 54,1 % |
| `255ad7a` (die Behebung) | 801 | 433 | 54,1 % |
| `96e32cb` | 801 | 433 | 54,1 % |

**Der Ontocoder hat recht, und meine Beobachtung von gestern bleibt trotzdem stehen** — sie war
nie eine Aussage über Zeilen. Was ich gemessen hatte, war der Sprung von 52 auf 54 Prozent über
die **vorige** Runde (58 neue Zeilen), und was ich als wachsend bezeichnet hatte, war nicht die
Länge, sondern die Zahl der Vorbehalte je Satz. Diese Runde ändert an der Länge nichts und an
den Vorbehalten eines: `:63-65` trägt einen dazu, `:629-631` gibt eine Begründung ab, die keiner
war.

**Der Kopf ist durch diese Runde besser geworden, nicht schlechter,** und zwar an drei Stellen
gegen eine:

- N2 hat einen Satz entfernt, der etwas behauptete, das nicht vorkommt. Das ist die reinste Form
  von Verbesserung, die ein Handbuchteil kennt.
- N1 hat nebenbei die falsche Anbindung von „und das ohne jede Meldung" gelöst.
- N3 hat einer Zahl im Kommentar eine Probe untergeschoben, ohne den Kommentar anzufassen. Der
  Text ist derselbe und trägt jetzt mehr.
- Dagegen steht die eine Zeile, an der die Fallunterscheidung schwerer zu lesen ist als vorher —
  und die ist der Preis dafür, dass sie überhaupt vollständig dasteht. Ein unvollständiger Satz
  liest sich immer flüssiger als ein vollständiger. Der Befund `260826-0139_o_*` sagt, wie man
  beides bekommt; er kostet ein Wort.

Die Stelle, an der ich diesen Kopf weiterhin als grenzwertig ansehe, ist unverändert die
Leselaufregel `:228-240` und nicht der Abschnitt über die Schreibfehler. Dazu die zweite Frage.

### Die zwei vorgeschlagenen und nicht ausgeführten Kürzungen

**(a) „Was er kostet, ist das, was der Schreibfehler weggenommen hat" (`:61-62`) — nicht kürzen,
und nicht später.** Das ist keine Frage von Reihenfolge, sondern von Inhalt: der Satz ist die
einzige Stelle der ganzen Datei, die den spiegelbildlichen Fall abdeckt. Gemessen (siehe oben):
ein verschriebenes `pfad` neben einem gültigen `kennzeichen` wird ebenso still übergangen wie
ein verschriebenes `kennzeichen` neben einem gültigen `pfad`. Der Satz davor („ein unbekannter
Schlüssel wird NICHT gemeldet, sondern übergangen") sagt den **Mechanismus**, die zwei Beispiele
dahinter sagen **eine Richtung** — die Kostenregel dazwischen ist das Glied, aus dem ein Leser
die andere Richtung selbst ableiten kann. Ohne sie stünden zwei Beispiele ohne Regel da, und ein
Nutzer, der beide Schlüssel führt und sich beim `pfad` vertippt, fände in der Datei nichts, das
seinen Fall trifft. Der Vorschlag ist abzulehnen, nicht zu verschieben.

**(b) Die Leselaufregel `:228-240` als zwei Sätze statt eines — Aufräumarbeit, und für diese
Runde nicht nötig.** Die Regel ist richtig, in beiden Zusammensetzungen gemessen, und beide
Beispielzahlen halten seit `96e32cb` eine Probe. Es steht nichts Falsches da, und ein
Umformulieren wäre der vierte Eingriff in zwei Tagen in denselben Kopf. Der Vorschlag ist eine
Gestaltungsfrage und gehört in die nächste Runde, die diese Datei ohnehin anfasst.

**Ein Vorbehalt gehört zu dieser Antwort, und er gilt für beide Vorschläge und für den neuen
Befund gleichermaßen.** „Später aufräumen" ist bei dieser Datei nicht dasselbe wie sonst: sie
wird beim ersten Start wörtlich kopiert und danach nie wieder angefasst
(`ablage::leseprofile::anlegen_falls_fehlt`, C1.2 der Runde 16), und wie eine neue
Auslieferungsfassung einen bestehenden Nutzer erreicht, ist mit
`shared/decisions/260825-1725_a_wie-erreichen-neue-auslieferungsprofile-…` als **Handgriff des
Nutzers** beantwortet (`README.md:62-63`). Wer heute installiert, behält den heutigen Wortlaut,
bis er ihn von Hand austauscht. Das macht aus keiner der drei Sachen ein
Auslieferungshindernis — es macht nur den Unterschied zwischen „heute" und „nächste Runde
größer, als er bei einer gewöhnlichen Datei wäre. Wenn der Nutzer eine der drei ohnehin will,
ist jetzt der billige Zeitpunkt; wenn nicht, verliert kein Leser eine Auskunft.

## Cross-cutting

**Zwei Runden hintereinander hat die Behebung eines Prosabefunds einen kleineren Prosabefund an
derselben Zeile erzeugt.** N1 aus der zweiten Durchsicht war ein fehlender Vorbehalt; sein Fix
brachte den Vorbehalt und mit ihm eine Fallunterscheidung, deren Bedingung schief sitzt. Das ist
kein Anzeichen von Schlamperei, sondern die natürliche Kurve: die Befunde werden kleiner, und
der jetzige ist der erste dieser Reihe, bei dem in der Sache nichts falsch ist. Das ist die
Stelle, an der man aufhört.

**Die Prüfschärfe der Datei liegt nicht mehr allein in der Prosa.** Von den drei Zahlen, die die
Datei behauptet, halten jetzt zwei eine Probe genau (`(3, 5)` und `(4, 5)`), und die dritte, die
24, ist eine Konstante. Der Zustand, den die erste Durchsicht vorfand — eine Prosa, die neben
dem Mechanismus stand, ohne dass etwas sie daran gebunden hätte —, besteht an keiner der
geprüften Stellen mehr.

**Von den offenen Befunden dieser Datei ist keiner am Mechanismus.** Die drei aus der ersten
Durchsicht betreffen den flight-Teil und eine Zählprobe; der neue betrifft eine Satzstellung.

## Kann die Datei ausgeliefert werden?

**Ja.** Nichts hält die Auslieferung auf, und das ist keine Zurückhaltung, sondern das Ergebnis
der Messung:

- Die zwölf Profile laden ohne Meldung, und jedes greift an dem Ort, für den es geschrieben ist.
- Die drei Behebungen dieser Runde halten einzeln der Nachmessung stand, jede an den Lagen, die
  ihr Vermerk behauptet.
- Keine Aussage der Datei steht heute im Widerspruch zum Quelltext. Die fünf mittleren Befunde
  der ersten Durchsicht sind seit `1ac5dde` erledigt, die drei niedrigen der zweiten seit
  `96e32cb`.
- Die vier offenen Befunde (drei aus der ersten Durchsicht, einer aus dieser) sind sämtlich
  niedrig, keiner betrifft einen Wert oder ein Muster, und keiner kann einen Nutzer zu einer
  falschen Änderung an seiner Datei verleiten.
- `cargo test -p krk-core --test leseprofil` 47 grün, `--lib leseprofil` 10 grün, darunter
  `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`, die die ausgelieferte Fassung durch
  denselben Prüfschritt schickt, den sie beim Nutzer nimmt.

Was ich der Auskunft beilege, ohne dass es an dieser Datei liegt: die Fassung, die jetzt
ausgeliefert wird, erreicht **keinen** Nutzer, der KRK schon einmal gestartet hat, ohne dessen
Handgriff. Das ist mit `shared/decisions/260825-1725_a_…` so entschieden und in `README.md:62-63`
beschrieben; es betrifft die berichtigten Muster dieser Runde ebenso wie die Prosa. Wer die
Auslieferung als „die Berichtigungen sind jetzt bei den Nutzern" liest, liest sie falsch.

## Empfohlene Reihenfolge

1. **Ausliefern.** Kein Befund steht dem entgegen.
2. `260826-0139_o_*` — ein Wort, bei nächster Gelegenheit an dieser Datei.
3. Die drei offenen `260825-2126_o_*` — unverändert, alle drei am flight-Teil und an einer
   Zählprobe.
4. `260826-0128_o_*` — der Modulkopf von `datei.rs`, `coder`.
5. Die Leselaufregel als zwei Sätze — offener Gestaltungsvorschlag, kein Datensatz.

## Was ich nicht geprüft habe

- **Die Anzeige.** Was das Vorschaufenster aus `Zusammenfassung::als_text` macht, ist an einer
  laufenden Anwendung zu sehen und bleibt Nutzerarbeit.
- **`crates/krk-core/tests/leseprofil.rs` als Ganzes.** Gelesen ist die Probe zu C6.7 samt ihrer
  neuen Hilfen, weil N3 an ihr hängt; die Bewertung der Probendatei gehört `coderev`.
- **Ob eine bestehende `readers.toml` die berichtigte Fassung je erreicht.** Das ist die
  beantwortete Frage `260825-1725_a_…` und keine Eigenschaft dieser Datei.

---

**Vermerk des reconciler, 260826-0149.** Die Freigabe ist gegen den Baumstand `e5ec81a`
nachgehalten und bestätigt: `resources/default-readers.toml` führt zwölf Profile, die Zahl
steht an drei Prüfstellen (`crates/krk-core/src/ablage/leseprofile.rs:181`,
`crates/krk-core/tests/ablage.rs:2048`, `crates/krk-core/tests/leseprofil.rs:2273`), und
`make check` läuft grün. Die drei niedrigen Befunde der ersten Durchsicht (`260825-2126_o_*`)
und der neue (`260826-0139_o_die-fallunterscheidung-des-kennzeichnen-satzes-…`) sind einzeln
gegen die Datei nachgelesen und unverändert offen; der Satz „dieselben drei Felder wie
`.fusion-setup`" steht weiter da und ist im Datensatz mit `:646-647` zitiert, während er
inzwischen auf `:683` gewandert ist.
