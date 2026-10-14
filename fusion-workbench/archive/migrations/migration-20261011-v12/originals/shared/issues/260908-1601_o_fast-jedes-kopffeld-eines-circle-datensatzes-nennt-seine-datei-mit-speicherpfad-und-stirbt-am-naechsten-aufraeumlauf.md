Fast jedes Kopffeld eines Circle-Datensatzes nennt seine Datei mit Speicherpfad und stirbt am nächsten Aufräumlauf

---

Die Circle-Datensätze führen 48 Kopffelder `**Active spec/plan:**` und
`**Active session history:**`. **40 davon nennen ihre Datei mit vollem, werkbank-relativem
Pfad** statt mit der speicherlosen Kurzform. Alle 40 lösen heute auf. Jede einzelne stirbt in
dem Augenblick, in dem ein Aufräumlauf die genannte Datei bewegt — dreimal ist das schon
geschehen.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** Werkbankführung
**Baumstand:** `2f4b7b2`
**Schwere:** heute null, morgen mittel. Kein Bau, kein Verhalten, kein Leser, der heute ins
Leere greift.

## Gemessen am 260908-1601

```sh
grep -h '^\*\*Active ' fusion-workbench/circles/*/[_]*circle.md | wc -l              # 48
grep -h '^\*\*Active ' fusion-workbench/circles/*/[_]*circle.md \
  | grep -cE 'circles/|shared/'                                                      # 40
```

Die acht ohne Speicherabschnitt sind die drei am 260908-1552 reparierten, die zwei Felder des
Circle-Datensatzes von `260830-1045-git-bereich-liest-status-branch-verlauf`, die die Form von
Anfang an führen, das Feld von `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`, das den
Speicher ohne den Circle nennt, und die zwei Felder eines Datensatzes ohne genannte Datei.

## Warum es kein Schönheitsfehler ist

`rules/fusion-workbench-conventions.md` `## Filename Patterns` sagt es mit der Ursache: der
Speicherabschnitt ist genau das, was ein Aufräumlauf bewegt, also stirbt ein Zitat, das ihn
ausschreibt, am Aufräumlauf. Der Beleg steht im Baum und ist nicht abgeleitet: drei dieser
Felder haben es getan, zweimal beim Lauf `260819-1613` und einmal beim Lauf `260826-1637`, und
zwischen dem Sterben und dem Bemerken lagen achtzehn beziehungsweise dreizehn Tage
(`260908-1552_*_drei-kopffelder-active-spec-plan-zeigten-nach-zwei-aufraeumlaeufen-ins-leere.md`).

**Der Fehler ist still.** Ein totes Kopffeld bricht keinen Bau und keine Probe. Wer ihm folgt,
findet nichts und kann nicht unterscheiden, ob die Datei gelöscht wurde oder der Zeiger falsch
ist — dieselbe Klasse, die `HYG-NO-SILENT-FAIL` verbietet.

## Was einer Behebung im Weg steht

**Die Form des Feldes ist nicht entschieden.** `rules/circle-records.md` definiert
`**Active spec/plan:**` als werkbank-relativen Pfad, und `skills/next/SKILL.md` liest ihn beim
Aktivieren wörtlich; die Kurzform wäre für einen **vorgesehenen** Circle deshalb möglicherweise
falsch. Für einen geschlossenen Circle wird das Feld nie wieder maschinell gelesen, und dort
ist die Kurzform unstrittig. Ob das die richtige Trennlinie ist, hängt an
`260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md`,
und der ist offen.

**Solange steht der billigere Zuschnitt offen:** die Felder der geschlossenen Circles auf die
Kurzform ziehen und die eines laufenden oder vorgesehenen so lassen. Heute trägt kein Circle
`_t_` oder `_a_`, die Trennung kostete also nichts und wäre morgen wieder zu treffen.

## Abnahme

Entweder jedes Kopffeld nennt seine Datei in einer Form, die einen Aufräumlauf übersteht, oder
der Datensatz nennt namentlich, welche Felder aus welchem Grund den vollen Pfad behalten.
