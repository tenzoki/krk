Drei von Durchsichten verlangte Nachträge an offenen Datensätzen sind nie eingetragen worden

---

Eine Durchsicht kann einen Befund auf zwei Wegen ablegen: als eigenen Datensatz oder als Nachtrag
an einem offenen. Der erste Weg trägt, weil eine neue Datei entsteht und ein Abgleich sie findet.
Der zweite trägt nicht: die Durchsicht schreibt in ihren eigenen Bericht, **was** nachzutragen
wäre, und niemand ist damit beauftragt, es zu tun. Am 260821 sind drei solche Nachträge
liegengeblieben.

---

**Gemessen am Baumstand `4e810f9`.** Die Durchsicht
`shared/reviews/260821-1346-coderev-artefakt-und-release.md` führt einen Abschnitt
`## Nachträge zu offenen Datensätzen` mit dem Satz „Diese Befunde gehen über die zwei offenen
Fragen nicht hinaus und sind dort nachzutragen statt neu zu filen". Sie nennt drei Ziele:

| Ziel | Was nachzutragen war | Stand bis zum 260821-1532 |
|---|---|---|
| `shared/decisions/260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-…` | die Zahl der `gh`-Aufrufstellen über die Konstante `GH`, und dass eine Umstellung deshalb eine Zeile berührt | nicht eingetragen |
| `shared/decisions/260821-1115_*_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-…` | dass die `README.md` die Begründung für Option 1 inzwischen ausschreibt, samt dem `export PATH`-Handgriff | nicht eingetragen |
| `shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-…` | dass es seit dem 260821 zwei Abfangstellen sind, `beglaubigen` und `veroeffentlichen`, und dass beide hinter dem Schaden stehen | nicht eingetragen |

Alle drei sind im Abgleich vom 260821-1532 nachgeholt worden. **Der Datensatz steht hier
trotzdem, weil der Mechanismus und nicht der Einzelfall das Problem ist.**

## Warum es der Rede wert ist

**Ein Nachtrag ohne Träger ist eine Anweisung an niemanden.** Ein eigener Datensatz erscheint in
jedem `find … -name '*_o_*.md'`, ein Nachtrag erscheint in keinem. Die Auskunft steht dann allein
im Bericht der Durchsicht, und den liest, wer diese Durchsicht sucht — nicht, wer den offenen
Datensatz aufschlägt. Genau der aber entscheidet die Frage.

**Der Preis ist hier gemessen und nicht angenommen.** Der erste der drei Nachträge nennt drei
`gh`-Aufrufstellen; es waren schon an dem Stand vier, den die Durchsicht gelesen hat
(`465330b:166`, `:169`, `:579`, `:622` — die zweite fehlt in ihrer Aufzählung). Wäre er
eingetragen worden, stünde die falsche Zahl heute im Entscheidungsdatensatz. Ein Nachtrag, der
liegenbleibt, wird also nicht nur vergessen, sondern altert dabei auch.

## Abhilfe

Zu entscheiden, nicht abzuleiten:

1. **Die Durchsicht trägt selbst nach.** Sie hat den offenen Datensatz ohnehin gelesen, und ein
   Anhängen ändert keine Beschreibung. Kostet der Durchsicht eine Schreibberechtigung an
   fremden Datensätzen, die sie heute nicht hat.
2. **Jeder Nachtrag wird ein eigener Datensatz.** Der Satz „statt neu zu filen" fällt. Billig und
   mechanisch, erzeugt aber viele kleine Datensätze, die je einen Satz tragen.
3. **Der Abgleich trägt sie nach.** Der Reconciler liest die Durchsichten ohnehin, und der
   Abschnitt `## Nachträge zu offenen Datensätzen` wäre für ihn eine Arbeitsliste. Kostet nichts
   Neues, verzögert den Eintrag aber bis zum nächsten Abgleich — im vorliegenden Fall um sechs
   Stunden, was folgenlos war, aber nicht folgenlos sein muss.

Der Sache nach eine Frage an fusion und kein Defekt dieses Projekts; er steht als Defekt hier,
weil er in diesem Baum dreimal an einem Tag eingetreten ist.

**Schwere:** niedrig für das Verhalten, mittel für die Grundlage. Eine offene Entscheidung, der
ihre Nachträge fehlen, wird auf einem älteren Bestand beantwortet als dem, den der Baum trägt.

**Gefunden:** reconciler, Abgleich zum Sitzungsabschluss 260821-1532, Bereich `01d2365..4e810f9`

**Betroffen:** `shared/reviews/260821-1346-coderev-artefakt-und-release.md` (Abschnitt
`## Nachträge zu offenen Datensätzen`)

**Domain:** code

**Herkunft:** gemeinsamer Speicher. Kein Circle war in dieser Sitzung aktiv.
