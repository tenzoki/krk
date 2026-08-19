Die Commit-Nachricht liegt in einem geteilten /tmp-Namensraum und ist in dieser Sitzung überschrieben worden

---

Der Orchestrator schreibt seine Commit-Nachricht nach `/tmp/fusion-commit-msg-<task-id>.txt`
und committet mit `git commit -F <pfad>`. Die Aufgabenkennungen sind projektübergreifend
generisch — `T1`, `T2`, `P-3`, `rev`, `final` —, der Pfad trägt keinen Projektnamen und keine
Sitzungskennung, und `/tmp` ist auf diesem Gerät für alle gleichzeitig laufenden fusion-Sitzungen
dasselbe Verzeichnis. Zwei Sitzungen in verschiedenen Projekten, die zur selben Zeit ihre erste
Aufgabe committen, schreiben damit auf dieselbe Datei.

---

**Gefilt von:** orchestrator, Sitzung `260819-2026`
**Gefunden:** unmittelbar am eigenen Lauf, nicht durch Suche.
**Schwere:** hoch in der Wirkung, niedrig in der Häufigkeit. Nichts bricht laut. Der Schaden
wäre eine Commit-Nachricht, die zu ihrem Commit nicht gehört, und die fällt erst auf, wenn
jemand die Historie liest.
**Baumstand:** `6be1e81`.

## Was beobachtet wurde

Diese Sitzung hat für ihre Aufgabe T1 die Nachricht nach `/tmp/fusion-commit-msg-T1.txt`
geschrieben und den Commit `6be1e81` daraus gefahren. Der Commit trägt die richtige Nachricht.
Wenige Minuten später meldete die Sitzungsumgebung, dieselbe Datei habe sich auf der Platte
geändert, und ihr Inhalt war die Commit-Nachricht eines **anderen Projekts**: ein Text über
`hooks/dist`, `git archive` und den Circle `260819-1645-four-constraints-on-deep-change`, also
aus dem Quellbaum von fusion selbst.

Der Zusammenstoß ist damit nicht hypothetisch, sondern in dieser Sitzung eingetreten. Er ist
nur deshalb folgenlos geblieben, weil er **nach** dem `git commit -F` geschah. Zwischen dem
Schreiben der Datei und dem Lesen durch git liegt ein Fenster von wenigen Sekunden; fällt der
fremde Schreibvorgang hinein, trägt der Commit die fremde Nachricht, und `git commit` meldet
dabei keinen Fehler, weil die Datei lesbar und nicht leer ist.

## Wie groß der Namensraum ist

`ls /tmp/fusion-commit-msg-* | wc -l` zählt am 260819-2206 **178** Dateien, flach, ohne
Projekttrennung. Darunter `T1`, `T2`, `T3`, `T4`, `T5`, `T6`, `T7`, `T8`, `T9`, `T10`, `T14`
sowie `final`, `rev`, `recon`, `release`, `portfolio`, `housekeeping` — durchweg Kennungen, die
jedes fusion-Projekt vergibt und keines für sich hat.

Eine frühere Sitzung dieses Projekts hat das Problem offenbar gesehen und umgangen:
`/tmp/fusion-commit-msg-krk-T4-260816.txt` trägt Projektnamen und Datum im Dateinamen. Die
Umgehung steht in keiner Regel, wird von nichts eingefordert und ist in dieser Sitzung nicht
angewandt worden — der Orchestrator hat den Pfad genommen, den sein Prompt wörtlich nennt.

## Warum der Ort trotzdem richtig ist

`/tmp` ist nicht das Problem und darf nicht zur Lösung erklärt werden. Der Prompt des
Orchestrators begründet die Wahl ausdrücklich: `/tmp` wird vom System geräumt, der
Arbeitsplatz nicht, und eine Nachrichtendatei unterhalb von `fusion-workbench/` wäre ein
Überbleibsel im nächsten `git status` und im schlimmsten Fall Inhalt eines Commits. Genau
dieser Fall ist dort als gemessener Defekt vermerkt. Der Fehler liegt allein im **Namen**, nicht
im Verzeichnis.

## Wo das hingehört

Der Defekt liegt in fusion und nicht in KRK: die Pfadform steht in
`$FUSION_PLUGIN_ROOT/agents/orchestrator.md`, Schritt 3b Punkt 3, und in den Bodies der beiden
committenden Skills. KRK ist der Ort, an dem er aufgefallen ist, nicht der Ort seiner Ursache.
Nach der Herkunftsregel gehört er in den gemeinsamen Speicher dieses Projekts und ist von hier
aus an fusion zu melden.

Eine Behebung hätte den Dateinamen um etwas zu ergänzen, das je Sitzung eindeutig ist — der
Projektname, wie die Umgehung vom 260816 ihn schon trägt, oder die Prozesskennung, oder beides.
Welche Form, ist nicht hier zu entscheiden.
