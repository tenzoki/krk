CLAUDE.md zählt die bindenden offenen Fragen zu niedrig und nennt eine Prüfzeit, die noch nicht war

---

**Domain:** code
**Filed by:** reconciler (Abgleich 260807-1022, Sitzung 260806-2257)
**Für:** `coder`
**Cross-references:** `CLAUDE.md` Zeilen 21 und 92, Commit `710ce84`,
`issues/260806-0904_c_claude-md-fuehrt-projektstand-und-entscheidungsstand-vom-260803.md`

---

Die Revision vom 260807 (`710ce84`) hat `CLAUDE.md` auf den heutigen Stand
gezogen. Zwei Angaben darin halten der Nachzählung nicht stand.

**Erstens: "Geprüft am 260807-1200" (Zeile 21).** Der Commit, der die Zeile
schreibt, trägt den Zeitstempel 260807-1011; zum Zeitpunkt dieses Abgleichs ist
es 260807-1022. Die genannte Prüfzeit lag beim Schreiben rund zwei Stunden in
der Zukunft und ist damit geschätzt statt von der Uhr gelesen. Der Wert taugt
so nicht als Beleg dafür, wie alt der Projektstand ist — und genau dafür steht
er da. Richtig wäre 260807-1011.

**Zweitens: "Zwei binden künftige Arbeit statt der laufenden" (Zeile 92).** Die
Zeile beziffert, wie viele offene Fragen künftige Arbeit binden, und nennt
namentlich die Verfügbarkeitsprüfung für Schnittstellen jenseits von macOS 15
und den Vordergrund des Abnahmelaufs. Offen sind im aktiven Circle aber fünf
Fragen, und drei davon binden ebenfalls künftige Arbeit:

- `decisions/260806-1730_*_welche-sprache-bestimmt-die-sortierordnung.md`
  empfiehlt selbst "Möglichkeit 1 für Runde 1, und die Frage bei einer Runde
  wieder aufrufen, die KRK über den deutschsprachigen Gebrauch hinaus trägt".
- `decisions/260807-0010_*_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`
  hängt an der Umstellung der Lesestelle aus dieser Sitzung (`5f2e45d`).
- `decisions/260807-0020_*_soll-die-markierung-eine-auffrischung-ueberleben.md`
  stellt eine Frage an C2 und C9, die der Spec heute nicht zusagt.

Der erste Satz der Zeile, "Keine offene Frage hält derzeit einen Planschritt
auf", stimmt und ist nachgeprüft: alle 38 Schritte tragen `[DONE]`, und keiner
der fünf Datensätze erklärt sich für einen Schritt bindend. Falsch ist allein
die Zahl im zweiten Satz.

**Warum das zählt.** Die Zeile steht unmittelbar vor der Entscheidung über den
Rundenabschluss und sagt dem Leser, wie viel Grounding noch aussteht. Eine zu
niedrige Zahl an dieser Stelle liest sich als "es ist weniger offen, als es
ist". Dass die Aufstellung der Fragen selbst mit derselben Revision aus
`CLAUDE.md` genommen wurde, weil sie zweimal in vier Tagen veraltete, macht die
verbliebene Zahl nicht haltbarer — sie veraltet auf demselben Weg.

**Denkbarer Weg.** Die Zahl durch die Aussage ersetzen, die nicht veraltet:
"Keine der offenen Fragen hält einen Planschritt auf; alle binden künftige
Arbeit." Wer die Namen braucht, hat drei Zeilen darüber das `find`-Kommando.

**Dringlichkeit.** Gering. Kein Abnahmekriterium hängt daran, und keine der
zehn Zeitzusagen ist berührt.

---
Resolved: Beide Angaben sind am 260807-1030 nachgezogen. Zeile 21 steht auf
260807-1011, dem Zeitstempel des Commits, der sie schreibt. Zeile 92 nennt keine
Zahl mehr, sondern die Aussage, die nicht veraltet: keine offene Frage hält einen
Planschritt auf, alle binden künftige Arbeit. Das ist der Weg, den dieser
Datensatz vorschlägt, samt seiner Begründung — eine Zahl an dieser Stelle
veraltet auf demselben Weg wie die Aufstellung, die dort bis zum 260807 stand.
