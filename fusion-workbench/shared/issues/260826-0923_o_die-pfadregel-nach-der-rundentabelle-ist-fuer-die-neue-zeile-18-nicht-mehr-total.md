Die Pfadregel nach der Rundentabelle ist für die neue Zeile 18 nicht mehr total

---

Der Absatz unter der Rundentabelle sagt: „Pfade der Form `planning/…`, `decisions/…`,
`analyses/…` und `issues/…` sind relativ zum Verzeichnis des **jeweils genannten** Circles zu
lesen. **Ohne Nennung gilt die Runde 2**." Die mit `fb50fcd` eingefügte Zeile 18 nennt in der
Circle-Spalte keinen Circle, sondern den Text „— kein Circle-Datensatz, alles unter
`fusion-workbench/shared/`". Für sie liefert die Regel kein Verzeichnis, und der Auffangzweig
(„ohne Nennung gilt die Runde 2") führt auf den falschen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

**Domain:** code

**Betroffen:** `CLAUDE.md`, Rundentabelle und der Absatz unmittelbar darunter

## Was heute gilt

Die Regel geht heute nicht schief, und das ist Zufall der Zitierweise und keine Eigenschaft der
Regel. Selbst gefahren am 260826-0923:

```
grep -oE '`[^`]*(planning|decisions|issues|analyses)/[^`]*`' CLAUDE.md | sort -u
```

Jedes Zitat, das zur Runde 18 gehört, trägt den Vorsatz `shared/`
(`shared/issues/260826-0149_*_…`, `shared/planning/260816-2240_*_…`) und ist damit gar keiner
„der Form `issues/…`". Die vier Zitate in Kurzform (`issues/260810-1001_*`,
`issues/260810-1102_*`, `issues/260810-1341_*`, `decisions/260810-1044_*`) gehören der Runde 2
und lösen sich über den Auffangzweig richtig auf.

## Warum das trotzdem trägt

Die Tabelle ist nach ihrem eigenen Satz „ein Verweisregister für die Pfadregel im Absatz
danach". Ihre zweite Spalte trug bis `fb50fcd` in jeder Zeile ein Circle-Verzeichnis, weil
genau das ihr Zweck war. Zeile 18 trägt dort etwas anderes, und damit ist die Fallunterscheidung
weder disjunkt noch vollständig: eine Zeile nennt einen Circle, eine nennt keinen, und für die
zweite greift stillschweigend der Zweig, der für „gar keine Nennung" gedacht ist. Wer künftig
„Runde 18, `planning/…`" schreibt — und die nächste Runde ohne Circle wird dieselbe Lage haben
—, wird auf den Circle der Runde 2 verwiesen.

Der Absatz sagt außerdem nirgends, wogegen ein Pfad mit dem Vorsatz `shared/` aufzulösen ist.
Dass es `fusion-workbench/` ist, weiß der Leser aus der Praxis und nicht aus der Regel. Solange
alle Runden einen Circle hatten, fiel das nicht auf.

**Schwere:** gering. Kein heutiges Zitat läuft falsch; die Regel ist unvollständig, und die
Lücke wächst mit der nächsten Runde ohne Circle.

## Vorschlag

Einen Halbsatz an die Regel: Pfade mit dem Vorsatz `shared/` und `circles/` sind relativ zu
`fusion-workbench/` zu lesen; eine Tabellenzeile ohne Circle-Verzeichnis nennt keinen und
erlaubt die Kurzform nicht.

**Gefunden:** coderev, Durchsicht von `e5ec81a..20c9833` am 260826-0923
