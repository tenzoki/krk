# Shaper, anticipated-circle: eingebauter Web-Betrachter

**Datum:** 2026-08-04, 09:33
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete
**Ergebnis:** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/`

## Auftrag

Der Nutzer hat am 260804 einen Entwurf für einen eigenen Browser in KRK eingereicht. Der Entwurf geht auf seine Ankündigung vom 260804-0830 zurück, "bei URL zur Zeit Systembrowser, später eigener", die der Plan der Runde 1 unter seinen offenen Punkten als nirgends festgehalten vermerkt.

Der Auftrag lief über zwei Nachrichten. Die erste hat der Shaper mit vier Klärungsfragen beantwortet. Die zweite trug die vier Antworten, zwei benannte Spannungen darin, eine Vorentscheidung zur Grenze aus C9 und den Auftrag, den Circle zu schreiben.

## Was der Shaper gelesen hat

- `CLAUDE.md` (Projektstand, Sprachdeklaration, Entscheidungsstand)
- `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` (Directive, Grounding, Abgrenzung)
- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C1, C2, C3, C6, C9, C10, Nicht in dieser Runde)
- `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S13, S19, offene Punkte)
- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md`
- die Datensatzlisten beider Entscheidungsspeicher, Circle und geteilt

## Die vier Antworten des Nutzers

1. **Umfang: Betrachter.** Der Betrachter zeigt die Adresse, die KRK ihm übergibt. Kein Adressfeld, kein Verlauf, kein Herunterladen. Blättern und Zurück.
2. **Ort: gewöhnlicher Tab im Vorschaufenster**, mit dem Halteverhalten aus C6. Keine neue Tab-Sorte.
3. **Der eingebaute Betrachter ersetzt den Systembrowser** als Ziel von Opt+Cmd+G.
4. **Sprungmarken auf jedem sichtbaren Link**, zusätzlich zu Blättern, Zurück, Vor und Adresseingabe. Begründet mit der ersten Maxime des Projekts, der Steuerung über die Tastatur.

## Die zwei Spannungen

**Kein Adressfeld, aber Adresseingabe.** Zur Hälfte aufgelöst, zur Hälfte als offene Frage in den Circle geschrieben. Aufgelöst ist das Bedienelement: ein dauerhaft sichtbares Adressfeld gibt es nicht, beide Antworten sagen dasselbe, und KRK trägt an keiner Stelle eine dauerhafte Eingabezeile. Offen bleibt die Fähigkeit: ein Blatt statt einer Zeile nimmt das Feld weg und nicht das Eintippen, und wer eine Adresse eintippen kann, erreicht jede Adresse. Genau daran hängt der Unterschied zwischen Betrachter und Browser. Der Gegeneinwand steht im Circle daneben: mit den Sprungmarken verlässt der Betrachter die übergebene Adresse ohnehin, sobald der Nutzer einem Verweis folgt.

Der Fragetext der ersten Runde steht nirgends auf der Platte, weshalb die Rückprüfung an der eigenen Fragestellung nicht möglich war. Die Vermutung, dass die Aufzählung in Antwort 4 aus dem Optionstext stammt und dort die übliche Browser-Ausstattung beschrieb, ist im Circle als `speculation:` markiert.

**Kein Verlauf, aber Zurück und Vor.** Ohne Rest aufgelöst. Der gespeicherte Verlauf ist eine Liste besuchter Seiten, die eine Sitzung überdauert; Antwort 1 schließt ihn aus. Der Navigationsstapel ist die Folge der Seiten eines Tabs, liegt im Arbeitsspeicher und erscheint nirgends als Liste; Zurück und Vor arbeiten allein auf ihm. KRK hält einen Navigationsstapel je Tab und keinen gespeicherten Verlauf. Daraus folgt aus dem Halteverhalten aus C6, ohne neue Regel: ersetzt die nächste Auswahl im Dateifenster den Inhalt des aktiven Tabs, stirbt dessen Navigationsstapel mit.

## Die angekündigte weitere Runde

Drei Fragen standen aus. Keine ist gestellt worden.

- **Herunterladen** ist mit Antwort 1 erledigt.
- **Die Grenze aus C9** hat der Nutzer vorentschieden, weil sie die Circle-Grenze berührt: `http:` und `https:` bleiben die einzigen Schemata, und ein eingebauter Betrachter derselben zwei verschiebt die Grenze nicht. So im Circle festgehalten.
- **Die Lesezeichenleiste** ist nicht als eigene Frage gestellt, sondern in die offene Frage nach den Adressquellen eingefaltet. Sie liegt auf derselben Achse: darf der Betrachter eine Adresse erreichen, die KRK ihm nicht übergeben hat. Zwei Fragen daraus zu machen hätte zwei Antworten erzeugt, die sich widersprechen können.

## Offene Fragen, die der Circle trägt

1. Welche Quellen dürfen die Adresse setzen? Drei Möglichkeiten: nur KRK und die angezeigte Seite; zusätzlich eine Adresseingabe über ein Blatt; zusätzlich gespeicherte Web-Adressen in der Lesezeichenleiste aus C5.
2. Zeigt der Betrachter auch lokale HTML-Dateien? Die Antwort ändert die Dreiteilung aus C6.
3. Bekommt der Betrachter eine eigene Zeitzusage neben den zehn aus C8?

## Getroffene Verfahrensentscheidung

Die drei offenen Fragen stehen im Circle-Datensatz und nicht als eigene Entscheidungsdatensätze. Grund: der Datensatz ist in diesem Modus das Artefakt, und die Grounding eines anticipated Circles entsteht erst bei der Aktivierung. Die Fragen sind Eingabe für genau diese Klärungsrunde. Als eigene Datensätze müssten sie dort erneut gestellt und dann beantwortet werden, womit die Frage an zwei Stellen stünde. Sieht der Nutzer es anders, sind es drei Datensätze in `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/decisions/`.

## Nicht getan

Kein Spec geschrieben, kein Turn eröffnet, kein Planner beauftragt, nicht committet. Der aktive Circle ist unverändert. Die Aktivierung ist der eigene Schritt des Nutzers über `/fusion:next`.
