# Darf ein Agent den Spec oder den Plan einer geschlossenen Runde berichtigen?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260813-0642_*_zwei-hingenommene-verluste-stehen-auf-keiner-abnahmeliste.md`, `260814-1002_*_c1-verlangt-dass-keine-kombination-zweimal-steht-cmd-a-steht-zweimal.md`, `260814-1247_*_die-abnahmeliste-rechnet-gegen-72-kriterien-der-spec-fuehrt-nach-dem-c5-nachtrag-75.md`, `260813-1345_*_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-und-das-stationsbild-zeigt-jetzt-sechs-wo-der-baum-sieben-traegt.md`, `260813-1345_*_die-eine-messung-die-der-plan-als-gegenmassnahme-nennt-ist-nicht-gefahren.md`, `260830-1317_*_die-25er-liste-der-nutzerarbeit-fuehrt-c3-1-und-c3-3-und-der-satz-darunter-sagt-sie-seien-nicht-gefuehrt.md`, `260830-1317_*_c1-1-nennt-vier-feldbreiten-die-den-bau-anhalten-gemessen-haelt-genau-eine.md`, `260830-1613_*_c8-3-nennt-98-zusaetzliche-pakete-am-projektbaum-gemessen-sind-es-101.md`, `260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`

---

## Question

Sieben offene Defektdatensätze aus sechs Runden verlangen dasselbe: eine Zahl, ein Satz oder eine Zeile im **Spec oder im Plan einer bereits geschlossenen Runde** ist falsch und soll berichtigt werden. Alle sieben sind am Baum nachgemessen, keiner ist strittig in der Sache, und keiner wird abgearbeitet — weil niemand weiß, ob eine solche Datei noch angefasst werden darf.

Die Ortsregel in `CLAUDE.md` beantwortet die Frage nicht. Sie zählt sieben Speicher auf, deren Dateien ihren damaligen Stand behalten — `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/`, `spikes/` —, und `planning/` steht **nicht** darunter. Nach dem Buchstaben ist ein Spec also ein lebendes Dokument. Nach seiner Rolle ist er das Gegenteil: er ist das, wogegen die Abnahme jener Runde gelaufen ist, und was er sagte, hat den Bau geleitet. Der Entscheid `260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md` behandelt Spec- und Plandateien ausdrücklich als lebend und lässt Zitate darin umstellen; das ist eine Antwort für die Zitate und keine für den Sachtext.

Die Frage ist jetzt fällig, weil sie sonst je Datensatz einzeln und jedes Mal anders beantwortet wird. Zwei der sieben sind schon so behandelt worden: die Messung zu `bundle::VERSION` ist am 260906 gefahren und ihr Ergebnis in den **Defektdatensatz** geschrieben statt in die Risikotafel des Plans, und der Zutrag des ontorev an die Runde 6 ist an den Defektdatensatz gehängt worden und nicht an den Spec. Beides ist Weg 3 unten, ohne dass jemand ihn gewählt hätte.

## Options

1. **Nein: ein Spec und ein Plan sind mit dem Rundenabschluss eingefroren.** `planning/` kommt zu den sieben Speichern der Ortsregel dazu. Eine falsche Aussage darin wird über den Defektdatensatz getragen, der sie festhält, und der Datensatz bleibt offen, bis jemand ihn liest.
   - Pro: eine Regel, entscheidbar nach dem Ort, keine Ausnahme. Der Spec bleibt der Text, gegen den die Runde abgenommen wurde; wer ihn nachträglich berichtigt, macht die Abnahmenotiz jener Runde unlesbar, weil sie gegen einen Text zählt, den es nicht mehr gibt. Genau diese Falle steckt in `260814-1247_*_die-abnahmeliste-rechnet-gegen-72-kriterien-…`: die Abnahme rechnet gegen 72, der Spec führt 75, und die Differenz ist eine Aussage über zwei Zeitpunkte.
   - Contra: sieben Datensätze bleiben dauerhaft offen, und ihr Gegenstand ist mit dem Einfrieren unbehebbar. Der Speicher trägt dann offene Befunde, die niemand je schließen kann — dieselbe Sorte Rauschen, gegen die dieses Projekt sonst arbeitet. Und ein Leser, der den Spec der Runde 23 aufschlägt, liest weiter, C1.1 nenne vier bauanhaltende Feldbreiten, wo genau eine hält.
2. **Ja, mit Nachtrag statt Überschreiben.** Der Sachtext bleibt Zeichen für Zeichen stehen, und die Berichtigung kommt als datierter Absatz an die Stelle oder an das Ende des Dokuments, wie es die Analysen dieses Projekts schon halten („Die Analyse behält ihre 98 nach der Ortsregel: sie ist die Aufzeichnung eines Standes, und ein Nachtrag berichtigt sie, kein Überschreiben", `260830-1613_*_…`).
   - Pro: die sieben Datensätze werden schließbar, ohne dass eine Abnahmenotiz ihren Bezug verliert. Die Form ist im Baum erprobt und von einem Defektdatensatz ausdrücklich empfohlen. Ein Leser sieht beides: was dastand und was stimmt.
   - Contra: ein Spec mit sieben Nachträgen liest sich schlechter als einer mit sieben berichtigten Sätzen, und die Nachträge stehen nicht dort, wo der Fehler steht. Bei einer Zahl mitten in einer Kriterienliste — C8.3, C1.1 — muss der Leser den Nachtrag erst finden.
3. **Ja, aber nur im Defektdatensatz.** Der Spec bleibt unberührt, und die richtige Aussage steht im Defektdatensatz, der ihn zitiert; dieser wird geschlossen, sobald sie dort steht und nachgemessen ist.
   - Pro: kein fremdes Dokument wird angefasst, die Datensätze werden schließbar, und die Arbeit ist die kleinste der drei. Es ist das, was zwei Läufe schon getan haben.
   - Contra: die Auskunft steht dann an einem Ort, an dem sie niemand sucht. Wer den Spec liest, liest ihn nicht mit dem Defektspeicher daneben — und der Spec ist die Prüfliste des Abnahmelaufs, den der Nutzer fährt. Genau daran hängt `260830-1317_*_die-25er-liste-…`: ein Leser, der dem falschen Satz folgt, streicht zwei Kriterien aus dem Abnahmelauf.

## Constraints

- Die Antwort muss nach dem **Ort** entscheidbar bleiben, nicht nach dem Absatz. Das ist die Eigenschaft, die `CLAUDE.md` mit der Ortsregel gekauft hat, und eine Regel je Absatz lieferte bei jedem Durchgang einen anderen Bestand.
- Sie muss die zwei Rollen eines Spec trennen können: er ist Grundlage für den Bau **und** Prüfliste für den Abnahmelauf am Bündel. Die zweite Rolle ist die, die eine falsche Zahl teuer macht.
- Sie darf keine Runde nachträglich anders abgenommen aussehen lassen, als sie abgenommen wurde.
- Was für den `## Directive`-Abschnitt eines Circle-Datensatzes gilt, ist davon unberührt: der gehört dem Shaper im Modus `portfolio-activation`, und zwei offene Datensätze der Runde 9 hängen daran.

## Recommendation

Möglichkeit 2, mit einer Zusatzbestimmung: der Nachtrag steht **an der Stelle** und nicht am Dateiende, wenn der Fehler eine Zahl oder ein Satz innerhalb einer Kriterienliste ist, und am Ende, wenn er das Dokument im Ganzen betrifft. Das hält beide Rollen zusammen — die Abnahme jener Runde bleibt lesbar, weil der ursprüngliche Wortlaut dasteht, und der Nutzer, der die Prüfliste abarbeitet, stolpert über die Berichtigung an der Stelle, an der sie ihn angeht. Möglichkeit 3 ist die billigste und trägt die Auskunft an den einen Ort, an dem der Abnahmelauf sie nicht findet; Möglichkeit 1 ist die sauberste Regel und kauft ihre Sauberkeit mit sieben unbehebbaren Befunden.

**Nicht Bestandteil dieser Empfehlung ist, wer den Nachtrag schreiben darf.** Auch bei Möglichkeit 2 kann die Antwort lauten, dass ein ausführender Agent es nicht darf und ein Abgleichslauf oder der Nutzer es tut.

---
Answered: `260905-2008-orchestrator-session.md` `### Nutzerentscheidung zu den Datensaetzen ueber eingefrorene Spec- und Plantexte` — ja, ein Agent darf eine falsche Angabe im Spec oder Plan einer geschlossenen Runde berichtigen, und zwar **als Nachsatz** und nicht im Text. Der urspruengliche Wortlaut bleibt unangetastet und lesbar, die Berichtigung steht darunter als erkennbar spaetere Zutat; ruled by user, Kai Stalmann <kai@stalmann.org>
