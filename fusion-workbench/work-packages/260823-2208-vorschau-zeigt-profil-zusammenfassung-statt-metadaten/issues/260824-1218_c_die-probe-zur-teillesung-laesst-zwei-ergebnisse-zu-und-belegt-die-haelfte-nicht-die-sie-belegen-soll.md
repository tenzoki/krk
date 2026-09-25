Die Probe zur Teillesung lässt zwei Ergebnisse zu und belegt die Hälfte nicht, die sie belegen soll

---

`eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet` prüft die zweite der drei Anwendungen
der Teillesungsregel mit `matches!(werte[1].1, Wert::Vorhanden(true) | Wert::Nicht)`
(`crates/krk-core/tests/leseprofil.rs:1238-1242`). Beide zugelassenen Werte sind genau die
beiden möglichen Antworten des Bausteins. Die Zusicherung kann damit nicht rot werden und belegt
nichts. Planschritt 6 verlangt an dieser Stelle ausdrücklich, dass ein Vorhandensein „mit Treffer
`ja`" liefert.

---

**Gemessen am 260824-1218 am Baumstand `abe1a31`.**

## Warum die Probe so geschrieben ist

Der Prüfordner trägt 2.001 durchnummerierte Dateien und daneben genau eine
`der-eine-treffer.txt` (`tests/leseprofil.rs:1202-1207`). Ob dieser eine Eintrag innerhalb der
ersten 2.000 gelesenen liegt, entscheidet die Reihenfolge, in der das Dateisystem die Einträge
liefert, und die ist nicht zugesagt. Die Probe konnte den Fall deshalb nicht festnageln und hat
stattdessen beide Ausgänge zugelassen. Das ist ehrlich und trägt trotzdem nichts: eine
Zusicherung, die jeden möglichen Wert annimmt, ist keine.

Die anderen zwei Anwendungen der Regel sind sauber belegt: `werte[2]` (Nichtfund in der
Teilliste) und `werte[3]` (die jüngsten N) prüfen je auf genau einen Wert.

## Wie sie zu retten ist

Der Treffer muss unabhängig von der Lesereihenfolge in der Teilliste liegen. Das geht, ohne die
Zusage abzuschwächen: ein Muster wählen, das auf **so viele** Einträge passt, dass jede
Zweitausend-Teilmenge einen davon enthält. Bei 2.001 Dateien der Form `00000.md` bis `02000.md`
leistet das etwa `muster = '\.md$'` oder `muster = '^0'` — jede Auswahl von 2.000 aus 2.001
Einträgen enthält mindestens 1.999 davon.

Der heutige Einzeltreffer `der-eine-treffer.txt` kann daneben stehen bleiben, aber dann als das,
was er ist: ein Fall, über den die Probe nichts sagt, und dann gehört er nicht in eine
Zusicherung.

## Warum das jetzt zählt und nicht erst bei Schritt 12

Die Regel über die Teillesung ist die eine Regel, die der Plan dreimal anwendet, und die Probe
ist ihr einziger Beleg im Kern. Schritt 12 baut die Zählproben zu C6, nicht diese; wer die
Zeile später liest, hält sie für abgenommen.

**Schwere:** mittel. Keine Auswirkung auf das Laufzeitverhalten. Der Befund ist eine Probe, die
grün ist, weil sie nichts fordert, und in einem Baum, dessen Proben sonst einzelne Werte
festnageln, fällt das erst auf, wenn jemand nachliest.

**Gefunden:** coderev, bei der Durchsicht von Bündel B am 260824-1218.

**Betroffen:** `crates/krk-core/tests/leseprofil.rs:1199-1252`

**Domain:** code

---
Resolved: Der vorgeschlagene Weg, gegangen. Das Vorhandensein in
`eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet` sucht jetzt `muster = '\.md$'` statt
`der-eine-treffer`, und die Zusicherung lautet `assert_eq!(werte[1].1, &Wert::Vorhanden(true))`
statt eines `matches!` über beide möglichen Werte. Bei 2.001 Dateien `00000.md` bis `02000.md`
enthält jede Auswahl von 2.000 mindestens 1.999 Treffer; der Ausgang hängt an keiner
Lesereihenfolge mehr.

`der-eine-treffer.txt` ist entfallen und nicht danebenstehen geblieben: die Datei diente allein
dieser einen Zusicherung, und ein Prüfstück ohne Aussage lädt den nächsten Leser dazu ein, eine
zweite darauf zu schreiben. Der Doc-Kommentar der Probe trägt einen Abschnitt, der sagt, warum
das Muster auf fast jeden Eintrag passen muss.

Die zwei sauber belegten Anwendungen (`werte[2]`, `werte[3]`) sind unverändert; die erste prüft
weiter `UeberGrenze(HOECHSTENS_EINTRAEGE)` und sagt seit `260824-1215` „mindestens" statt „über".
