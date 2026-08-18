Der Turn-Log nennt zehn weitere Zeiger, die zwei Commits haben sechzehn gestellt

---

Der Eintrag zu Turn 4 im Circle-Datensatz sagt: „Alle 22 stehen jetzt in der Sternform und
lösen auf, dazu zehn weitere, die zwei eigene Erhebungen mitbrachten." Gezählt an den beiden
Commits sind es sechzehn weitere, nicht zehn. Die Zahl im Turn-Log ist die einzige Stelle, an
der der Umfang der Reparatur für eine spätere Runde ablesbar ist; der geschlossene Datensatz
`issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`
führt fünfzehn der sechzehn einzeln auf und nennt selbst keine Gesamtzahl.

---

**Schwere:** gering
**Gefunden von:** reconciler, zweiter Abgleich der Sitzung 260817-2131 (260818-0807)
**Betroffen:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/_t_circle.md`, Abschnitt `## Turn log`, Eintrag „Turn 4 (Sitzung 260817-2131)"
**Domain:** code

## Gemessen, an `9ac41ea`

Gezählt sind die Zitat-Token der Form `YYMMDD-HHMM_x_` auf den entfernten Zeilen der beiden
Reparaturcommits. Jedes entfernte Token trägt einen ausgeschriebenen Marker oder einen falschen
Namensteil, jedes hinzugefügte trägt die Sternform; die Zahlen der beiden Spalten stimmen je
Datei überein.

| Commit | Datei | gestellt | davon gemeldet | davon weitere |
|---|---|---|---|---|
| `adf638b` | `crates/krk-core/src/verzeichnis/arbeitsbaum.rs` | 3 | 3 | 0 |
| `adf638b` | `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` | 1 | 1 | 0 |
| `adf638b` | `crates/krk-core/src/verzeichnis/umfang.rs` | 1 | 1 | 0 |
| `adf638b` | `crates/krk-core/tests/verzeichnis.rs` | 1 | 0 | 1 |
| `adf638b` | `crates/krk-ui/src/appkit/tabelle.rs` | 3 | 0 | 3 |
| `adf638b` | `crates/krk-ui/src/appkit/textautomatik.rs` | 1 | 0 | 1 |
| `0494604` | `shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md` | 10 | 9 | 1 |
| `0494604` | `planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md` | 13 | 4 | 9 |
| `0494604` | `_t_circle.md` | 5 | 4 | 1 |
| | **Summe** | **38** | **22** | **16** |

Kommando:

```sh
git show adf638b; git show 0494604
```

je Commit die entfernten Zeilen gefiltert und die Token `\d{6}-\d{4}_[a-z*]_` gezählt.

## Woher die Zehn kommen könnte

Der geschlossene Datensatz `260818-0710` führt die weiteren in zwei Blöcken: der Lauf um 0737
nennt fünf (vier ausgeschriebene Marker, ein falscher Namensteil), der Lauf um 0744 nennt neun
(sechs ausgeschriebene Marker, zwei tote Glob-Zitate, ein falscher Namensteil) und dazu den
Runde-1-Spec an `spec:218` als „mitgezogen". Das sind fünfzehn. Die sechzehnte Stelle ist die
eine im Circle-Datensatz selbst, die über die vier gemeldeten hinausgeht und in keinem der
beiden Blöcke steht: der Orchestrator hat sie beim Nachziehen der vier mitgenommen und nicht
gezählt.

Keine Lesart der beiden Blöcke ergibt zehn.

## Warum das zählt

Der Circle-Datensatz ist lebender Text und wird bei der Aktivierung einer Nachfolgerunde als
bindende Grundlage gelesen. Wer den Umfang der Zeigerreparatur später schätzt — etwa um zu
entscheiden, ob der breite Fix aus `260818-0710` `## Fix` seinen Preis wert ist —, liest hier
32 statt 38 gestellter Zitate und damit einen um ein Sechstel zu kleinen Befund.

## Fix

Eine Zahl im Turn-Log-Eintrag zu Turn 4. Der Datensatz gehört dem Orchestrator, der die Datei
besitzt. Ob die sechzehnte Stelle daneben im geschlossenen Datensatz `260818-0710` nachgetragen
wird, ist eine zweite Frage: der Datensatz ist geschlossen, und ein Abschlussvermerk zeichnet
einen Stand auf.

---
Resolved: Der Turn-Log des Circle-Datensatzes sagt jetzt „sechzehn weitere" statt „zehn". Der Orchestrator besitzt die Datei und hat die Zahl unmittelbar nach diesem Abgleich berichtigt; gezählt ist sie an den Token der Commits `adf638b` und `0494604`, wie der Abgleich sie gezählt hat.
