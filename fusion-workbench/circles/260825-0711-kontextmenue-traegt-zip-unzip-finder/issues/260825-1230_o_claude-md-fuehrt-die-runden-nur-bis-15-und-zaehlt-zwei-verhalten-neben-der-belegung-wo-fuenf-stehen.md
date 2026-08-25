`CLAUDE.md` führt die Runden nur bis 15 und zählt zwei Verhalten neben der Belegung, wo fünf stehen

---

Zwei Aussagen in `CLAUDE.md` sind mit der Runde 17 falsch geworden. Das Verweisregister der
gefahrenen Runden endet bei der 15, und der Absatz zur Tastenbelegung sagt, genau zwei Verhalten
stünden neben der Belegung; mit den drei Kontextmenü-Befehlen dieser Runde sind es fünf.

---

**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1230, beim Abgleich zum Abschluss der Runde 17.**

## Was der Baum trägt

**Erstens, das Verweisregister.** `CLAUDE.md:15–29` führt die Runden 1 bis 15. Der Dateibestand
führt zwei weitere gefahrene:

```
$ ls fusion-workbench/circles/*/_[bct]_circle.md | wc -l
17
```

Die fehlenden sind `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (Runde 16,
beschränkt geschlossen mit `a39aa95`) und `260825-0711-kontextmenue-traegt-zip-unzip-finder`
(Runde 17, diese). **Die Runde 16 fehlte schon vor dieser Sitzung**; die Runde 17 ist die
Abweichung, die diese Sitzung erzeugt hat. Der Absatz darunter („Was die Runden ab der zweiten
hinzugefügt haben, steht in der Tabelle oben und wird hier nicht wiederholt") verweist damit auf
ein Register, das den Stand nicht mehr trägt. Die Zeile über der Tabelle ist davon **nicht**
betroffen: sie nennt schon heute den Dateibestand als verbindlich und die Tabelle ausdrücklich als
Verweisregister für die Pfadregel.

**Zweitens, die zwei Verhalten neben der Belegung.** `CLAUDE.md:74` sagt:

> Zwei Verhalten stehen daneben und nicht in der Belegung: der Doppelklick auf eine Zeile […] und
> das App-Symbol […]

Die Runde 17 legt drei weitere daneben. `Kontextbefehl` (`crates/krk-ui/src/kommandos/kontextmenue.rs`)
trägt Zippen, Entpacken und ImFinderZeigen; keiner der drei ist ein `Kommando`, keiner steht in
`resources/default-keymap.toml`, keiner im Hauptmenü. Gegengeprüft: `git diff 428fbc4..HEAD --
crates/krk-core/src/tasten/belegung.rs resources/default-keymap.toml` gibt nichts aus, die
Aufzählung `Kommando` und die Belegung sind Zeile für Zeile unverändert. Der bisherige einzige
Kontextmenü-Eintrag, das Teilen, war kein Gegenbeispiel: er ist ein `Kommando` und trägt in
`resources/default-keymap.toml:733` den Eintrag `id = "teilen"`.

## Warum das trägt

Beide Stellen sind Nachschlagestellen und keine Prosa. Wer das Register liest, um einen Pfad der
Form `planning/…` aufzulösen, bekommt für die zwei jüngsten Runden keine Auskunft. Wer den
Belegungsabsatz liest, um zu wissen, was ein Befehl auslösen kann, hält die Belegung samt der zwei
genannten Ausnahmen für vollständig — und übersieht damit genau die Falle, die der Plan der Runde
17 selbst benennt: ein Kontextmenü-Eintrag hängt an keiner Prüfung, die der Übersetzer hält.

## Vorschlag

Für die Tabelle: die zwei Zeilen nachtragen. Die Form der Tabelle trägt das, sie ist ein
Verweisregister und keine Zählung.

Für den Belegungsabsatz: nicht auf „fünf" hochzählen, sondern dieselbe Bewegung wie bei `Kommando`
und den `#[must_use]`-Stellen. Die Zahl wächst mit jeder Runde, die eine Mausgeste baut. Der Satz
kann stattdessen die zwei Klassen nennen — was an einer Taste hängt, sagt die Belegung; was an
einer Mausgeste oder an der Bauzeit hängt, steht nicht darin — und für die zweite Klasse den
Modulkopf von `crates/krk-ui/src/kommandos/mod.rs` als Fundort nennen, der die Module ohne
Tastenbefehl schon heute ausdrücklich abgrenzt (`mod.rs:45`).

Daneben, im selben Zug zu prüfen und hier nicht als eigener Befund geführt: der Absatz zu `syntect`
und `two-face` (`CLAUDE.md:80`) nennt zwei Kisten, die ohne Vorgabemerkmale eingebunden sind; mit
`zip` sind es drei. Die tragende Aussage des Absatzes hält gemessen weiter — `cargo tree
--workspace -e normal,build` nennt am Stand `ddd41ff` weder `cc` noch einen `-sys`-Namen, und
`Cargo.lock` führt außer `windows-sys` keinen —, allein die Aufzählung ist zu kurz.

**Schwere:** niedrig. Keine Fehlfunktion, zwei falsche Auskünfte an Stellen, an denen
nachgeschlagen wird.

**Betroffen:** `CLAUDE.md`, Zeilen 15–29 (Verweisregister der Runden) und Zeile 74
(Tastenbelegung); nachrangig Zeile 80 (`syntect`/`two-face`)
