Die abgeschnittene Zählung zeigt „über <Treffer>", und C6.5 wie Schritt 6 verlangen „über 2.000"

---

C6.5 sagt: „Eine Zählung, die die Grenze erreicht, zeigt „über 2.000" statt einer Zahl", und
Planschritt 6 schreibt dazu, die Sätze der Anzeige entstünden „aus der Konstante und nicht aus
einer zweiten Zahl im Text". Gebaut ist etwas anderes: `Wert::UeberGrenze` trägt die Zahl der
**Treffer** innerhalb der Teillesung, und `als_text` schreibt sie aus. Trägt die Zählung ein
`muster`, steht dort eine Zahl, die mit 2.000 nichts zu tun hat — gemessen „über 1" für einen
Ordner mit 2.101 Einträgen.

---

**Gemessen am 260824-1214 an diesem Baum**, Stand `abe1a31`, in einer Wegwerfprobe unter
`crates/krk-core/tests/`, die danach wieder entfernt wurde.

## Der Lauf

Ein Ordner `viele` mit 2.101 Einträgen, davon genau einer mit `_o_` im Namen:

```text
zaehlung = { ordner = "viele", muster = '_o_' }   ->  Wert::UeberGrenze(1)   ->  "Mit Muster: über 1"
zaehlung = { ordner = "viele" }                   ->  Wert::UeberGrenze(2000) -> "Ohne Muster: über 2000"
```

Der Code steht in `crates/krk-core/src/leseprofil/bausteine.rs:390-401` (`zaehlen` gibt die
gefilterte Zahl in `Wert::UeberGrenze` weiter) und `crates/krk-core/src/leseprofil/mod.rs:548`
(`Wert::UeberGrenze(gezaehlt) => format!("über {gezaehlt}")`).

## Warum die gebaute Fassung sachlich richtiger ist als die geplante

Aus der Konstanten gebildet hieße die Zeile „über 2.000 offene Defekte", und das wäre für einen
Speicher mit einem einzigen offenen Defekt und 2.100 geschlossenen schlicht falsch. Die gebaute
Fassung sagt eine wahre Aussage: es sind mehr als die gezählten. Die Zahl im Wert ist damit die
bessere Wahl, und der Befund ist nicht, dass sie falsch wäre.

## Warum der Satz trotzdem nicht trägt

**„über 1" sagt dem Nutzer nicht, dass etwas weggelassen wurde.** Es liest sich als „mindestens
zwei" und nicht als „die Liste war zu lang, um sie zu Ende zu zählen". Genau die Auskunft, die
`Lesestand::abgeschnitten` trägt und die den Wert von `Wert::Zahl` unterscheidet, verschwindet
in der Anzeige, sobald die Zahl klein ist. Bei „über 2000" errät sie der Nutzer noch, weil die
Zahl der Grenze gleicht; bei „über 1" nicht mehr.

Dazu kommt die Schreibweise: der Spec schreibt „über 2.000", `als_text` schreibt „über 2000".
Das ist Kleinkram und gehört nur der Vollständigkeit halber dazu.

## Was zu tun ist

Zwei Stellen sind zu berichtigen, und welche Richtung gilt, ist eine Entscheidung:

1. **Der Satz der Anzeige.** Entweder er nennt beides — die gezählten Treffer und dass die
   Lesung abgebrochen ist — oder er nennt die Grenze. Eine Form, die beides trägt, wäre
   `mindestens 1 (Lesung bei 2.000 Einträgen abgebrochen)`; die Zahl 2.000 käme dann aus
   `HOECHSTENS_EINTRAEGE` und stünde nicht ein zweites Mal im Text, wie Schritt 6 es verlangt.
2. **C6.5 und Schritt 6.** Ihr Wortlaut nennt „über 2.000" als den anzuzeigenden Satz. Bleibt
   die Zahl im Wert, ist der Wortlaut nachzuziehen, sonst wäre das Kriterium bei der Abnahme
   als Abweichung abzuhaken, obwohl die Anzeige die bessere Auskunft gibt.

Derselbe Zuschnitt wie beim Befund `260824-1124_o_c4-3-sagt-eine-zeile-je-profilzeile-…`: der
Bau ist entschieden, die Buchführung nicht.

**Schwere:** mittel. Kein Fehlverhalten der Rechnung, aber eine Anzeige, die ihre wichtigste
Auskunft — „hier fehlt etwas" — bei kleinen Zahlen nicht mehr transportiert, und zwei
Prosastellen, die etwas anderes zusagen als der Baum tut.

**Gefunden:** coderev, bei der Durchsicht von Bündel B am 260824-1215.

**Betroffen:** `crates/krk-core/src/leseprofil/mod.rs` (`Wert::als_text`, `Wert::UeberGrenze`),
`crates/krk-core/src/leseprofil/bausteine.rs` (`zaehlen`),
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (C6.5),
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Schritt 6)

**Domain:** code
