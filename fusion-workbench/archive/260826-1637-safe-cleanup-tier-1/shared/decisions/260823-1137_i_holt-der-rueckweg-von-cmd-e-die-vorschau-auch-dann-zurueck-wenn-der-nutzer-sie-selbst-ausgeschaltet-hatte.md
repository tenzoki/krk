# Holt der Rückweg von `cmd+e` die Vorschau auch dann zurück, wenn der Nutzer sie selbst ausgeschaltet hatte?

---
**Domain:** code
**Filed by:** coder
**Cross-references:**
`shared/issues/260823-1035_*_der-rueckweg-blendet-die-vorschau-auch-dann-ein-wenn-der-hinweg-sie-nicht-verdraengt-hat.md`
— der Befund, der diese Frage aufgeworfen hat.
`shared/decisions/260820-1034_i_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`
— der Entscheid vom 260823-0942, dessen Tafel der Code buchstabengetreu umsetzt.

---

## Question

`cmd+e` mit dem Fokus im Editor schließt ihn und blendet die Vorschau ein
(`Anwendungsdelegierter::anlass_ausfuehren`, Zweig `Anlass::EditorSchliessen`).
Die Begründung, die der Code für diese Zeile angibt, lautet: der Rückweg ist die
Umkehrung eines Hinwegs, der die Vorschau verdrängt hat. Hatte der Nutzer die
Vorschau vorher mit `f3` oder `cmd+y` selbst ausgeschaltet, trägt diese
Begründung nicht — der Hinweg hat dann nichts verdrängt, und der Rückweg
schaltet dem Nutzer eine Vorschau an, die er ausgeschaltet hatte.

Die Frage ist jetzt zu stellen, weil die Prosa an zwei Stellen im Baum eine
Begründung führt, die nur für einen Teil der Fälle gilt. Sie ist entscheidbar
und nicht anzunähern: die Auskunft „war die Vorschau vor dem Hinweg sichtbar"
hat das Programm heute nicht, sie ist aber erhebbar. **Aus der Lage beim Drücken
des Rückwegs ist sie nicht abzuleiten**: der gegenseitige Ausschluss aus C1 hält
die Vorschau ausgeblendet, solange der Editor die Fläche hat, gleich aus welchem
Grund. Wer die Zeile bedingt machen will, braucht deshalb einen gemerkten
Zustand.

## Options

1. **Es bleibt, wie es ist** (empfohlen) — der Rückweg endet immer in derselben
   Lage, gleich wo er begonnen hat. Die Prosa im Code sagt das dann als Regel und
   nicht mehr als Umkehrung.
   - Pros: kein neuer Zustand; die Regel ist in einem Satz erklärt und in einer
     Probe zu halten; der Entscheid vom 260823-0942 bleibt unangetastet.
   - Cons: nach `f3`, `cmd+e` hinein und `cmd+e` heraus steht die Vorschau
     wieder da. Der Nutzer nimmt sie mit einem Tastendruck (`f3`) wieder weg.
2. **Der Hinweg merkt sich die Sichtbarkeit der Vorschau, der Rückweg stellt sie
   her.** Das ist die einzige Möglichkeit, unter der die heute im Code stehende
   Begründung zutrifft.
   - Pros: der Rückweg wird wirklich die Umkehrung des Hinwegs.
   - Cons: ein gemerkter Zustand mehr, und die Frage, wann er verfällt, ist
     nicht klein. **„Der Hinweg" ist gar nicht wohldefiniert:** der Fokus kommt
     auch über `f4`, über `opt+cmd+b` und über die Wiederherstellung aus der
     Sitzung in den Editor, und aus jeder dieser Lagen ist der Rückweg
     erreichbar. Entweder merken sich alle Wege in den Editor die Sichtbarkeit,
     oder der Rückweg findet keinen oder einen veralteten Wert vor. Dazu kommt
     eine Probe je Weg.
3. **Der Rückweg blendet nie ein und lässt die Fläche leer**, wie `opt+cmd+e`.
   - Pros: der billigste Code, eine Zeile weniger.
   - Cons: dreht die Zeile um, die der Nutzer am 260823-0942 ausdrücklich
     verlangt hat („die Vorschau zeigt die Datei wieder"), und nimmt `cmd+e`
     seinen Unterschied zu `opt+cmd+e`.

## Constraints

- Der Entscheid vom 260823-0942 sagt für `cmd+e` mit dem Fokus im Editor: „die
  Vorschau zeigt die Datei wieder", ohne Vorbehalt. Möglichkeit 3 dreht ihn um.
- `opt+cmd+e` behält seine Bedeutung seit der Editor-Runde und lässt die Fläche
  leer; keine der drei Möglichkeiten fasst es an.
- Kein Datenverlust und nichts Unumkehrbares hängt daran: `f3` schaltet die
  Vorschau in jeder Lage wieder aus.

## Recommendation

Möglichkeit 1. Der Preis von Möglichkeit 2 ist nicht die eine gemerkte Größe,
sondern dass jeder Weg in den Editor sie setzen muss, damit der Rückweg sie
lesen darf — genau die Gestalt einer Zusage, die auf mehrere Aufrufstellen
verteilt ist und deren erste vergessene keine Prüfung findet. Der Baum vermeidet
sie an dieser Stelle schon einmal ausdrücklich, im Doc-Kommentar von
`editor_oeffnen_lassen`. Dagegen steht ein Fall, den ein Tastendruck aufräumt.

Umgesetzt ist mit dem 260823-1137 Möglichkeit 1, und zwar ohne Codeänderung: die
zwei Prosastellen sagen jetzt die Regel („der Rückweg endet immer in derselben
Lage") statt der Begründung („Umkehrung eines Hinwegs, der sie verdrängt hat"),
die nur für einen Teil der Fälle trägt. Entscheidet der Nutzer anders, ist die
Änderung eine Zeile in `anlass_ausfuehren` (Möglichkeit 3) oder der gemerkte
Zustand samt seiner Setzer (Möglichkeit 2).

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Der Nutzer hat am 260823-1235 Möglichkeit 1 gewählt: es bleibt, wie es ist. Der Rückweg
endet immer in derselben Lage, gleich wo er begonnen hat, und die Prosa im Code sagt das als Regel
statt als Umkehrung eines Hinwegs. Die Frage war ihm mit dem Preis vorgelegt: nach `f3` steht die
Vorschau nach dem Rundweg wieder da und geht mit einem weiteren `f3` wieder weg.

Der Einwand aus Möglichkeit 2 hat die Wahl getragen und gehört zum Ergebnis: „der Hinweg" ist
nicht wohldefiniert, weil der Fokus auch über `f4`, `opt+cmd+b` und die Sitzungswiederherstellung
in den Editor kommt. Ein gemerkter Zustand müsste an allen vier Wegen gepflegt werden, sonst
findet der Rückweg keinen oder einen veralteten Wert.

Implemented: `52fba42` — kein Verhalten geändert, weil das gewählte Verhalten das gebaute ist.
Umgesetzt ist die Begründung: die überzogene Prosa („Umkehrung eines Hinwegs, der die Vorschau
verdrängt hat") ist durch die Regel ersetzt, die wirklich gilt.
