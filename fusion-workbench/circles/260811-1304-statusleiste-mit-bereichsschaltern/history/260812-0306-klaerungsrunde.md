# Klärungsrunde bei der Aktivierung — 260812-0306

**Circle:** `circles/260811-1304-statusleiste-mit-bereichsschaltern`
**Sitzung:** `shared/history/260812-0306-orchestrator-session.md`
**Geführt von:** Orchestrator, autonom auf Weisung des Nutzers ("mache autonom den
Statuszeilen-Circle und fixe auch den darin enthaltenen Bug")

## Was hier entschieden wurde

Zehn Fragen. Sechs lagen als offene Datensätze im Circle, vier sind aus dem Nachtrag zu den
Spaltenschaltern (`issues/260811-1732_*_…`) neu gestellt und im selben Zug beantwortet worden.
Jede Antwort steht in ihrem eigenen Datensatz unter `decisions/`; dieser Bericht führt nur
zusammen, was sie zusammen bedeuten.

| Frage | Antwort |
|---|---|
| Was heißt „proportional zur letzten Aufteilung"? | Alle sichtbaren Bereiche proportional. Die Festlegung vom 260808 fällt. |
| Trägt das linke Dateifenster einen Schalter? | Ja. Fünf Schalter, jedes Dateifenster ausblendbar, solange eines bleibt. |
| Wie zeigen zwei Schalter eine Fläche, die nur einer haben kann? | Zwei gewöhnliche Schalter; beide springen beim Umschalten. |
| Neue Leiste oder die Statuszeile aus C1? | Neue Fläche, nur Schalter, Name `Bereichsleiste`. C1 bleibt unberührt. |
| Welchen Anteil bekommt ein nie sichtbarer Bereich? | Die Anfangsbreite, beim Lesen in einen Anteil umgerechnet. |
| Was, wenn die Mindestbreiten nicht hineinpassen? | Der Schalter wird ohne Meldung verworfen. Der Fall tritt ein (gemessen). |
| Spaltenschalter je Seite oder gemeinsam? | Gemeinsam. Drei Schalter für beide Listen. |
| Überstehen sie einen Neustart? | Ja. `Sitzung` wächst um drei `bool`. |
| Was wird aus der Sortierung? | Sie bleibt. Wegschalten verbirgt die Anzeige, nicht die Ordnung. |
| Bekommen sie Tastenbefehle? | Kommandos ja, ausgelieferte Kombination nein. |

## Die eine Antwort, die eine frühere Nutzerentscheidung umstößt

Die Festlegung vom 260808 — die Lesezeichenleiste weicht nicht, wenn der Editor aufgeht — fällt
mit der Antwort auf die erste Frage. Sie stand nirgends als Datensatz, sondern allein im
Dokumentationskommentar an `bereichsbreiten` (`crates/krk-ui/src/fenstermodell.rs:596-602`).

Der Grund ist die Directive selbst: der Nutzer hat am 260811 „zwei Bereiche, die im Verhältnis
2:1 zueinander standen, stehen nach dem Einblenden eines dritten weiterhin in diesem Verhältnis"
diktiert, und das gilt für Bereiche und nicht für Dateifenster. Eine benannte Ausnahme für die
Lesezeichenleiste risse in genau dieses Beispiel ein Loch.

Die Frage vom 260808 lautete: wer weicht, wenn es eng wird? ~~Unter einer Anteilsregel weicht
niemand einzeln, sondern alle mit demselben Faktor — die Frage hat unter der neuen Regel keinen
Gegenstand mehr. Sie wird also nicht überstimmt, sie löst sich auf.~~

**Zurückgenommen am 260812-0815, nach dem Abgleich.** Der Satz war falsch. Die
Wasserstandsrechnung nimmt einen Bereich am Mindestmaß aus der Verteilung heraus, danach weichen
nur noch die übrigen — es weicht also jemand einzeln, und wer es ist, bestimmt die Mindestbreite
statt der Reihenfolge in `Bereich::ALLE`. Die Lesezeichenleiste schrumpft beim Aufgehen des
Editors. **Die Festlegung ist überstimmt worden, nicht aufgelöst.** Tragfähig bleibt der erste
Grund allein: die Directive vom 260811 spricht von Bereichen.

**Der Nutzer kann das umstoßen, und es ist ihm im Abschlussbericht vorgelegt.** Die Kosten der
Umkehr sind benannt: `bereichsbreiten` und seine Proben wären ein zweites Mal zu fassen.

## Was gemessen wurde

`MINDESTGROESSE` steht auf `NSSize::new(780.0, 300.0)` (`crates/krk-ui/src/appkit/fenster.rs:116`,
gesetzt bei `:289`). Der größte zugleich mögliche Satz an Mindestbreiten ist Lesezeichen 120 +
Links 240 + Rechts 240 + Editor 320 = 920, weil Vorschau und Editor sich ausschließen. Zwischen
780 und 920 Punkten Fensterbreite passt er nicht. Die sechste Frage verlangte diese Messung vor
ihrer Antwort; sie ist damit gefahren, und die Abweisung ist eine Fähigkeit statt einer
Vorsichtsmaßnahme.

## Was der Nachtrag für den Zuschnitt heißt

Die Runde trägt zwei Sorten Schalter: fünf für die Bereiche, die die Aufteilung ändern, und drei
für die Spalten, die nur den Inhalt beider Dateilisten ändern. Sie liegen in verschiedenen
Schichten und teilen sich allein die Leiste, in der sie sitzen. Das ist der Grund, warum sie in
eine Runde passen: die Leiste ist ohnehin zu bauen, und ein zweiter Circle für drei Schalter in
derselben Leiste wäre ein zweiter Bau derselben Fläche.
