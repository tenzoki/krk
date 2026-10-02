Die Zeitstempel in Dateinamen laufen der Uhr voraus, bis zu dreieinhalb Stunden

---

Die Sitzung vom 260824 hat zehn Dateien angelegt, deren Namensstempel **später** liegt als die
Zeit des Commits, der sie trägt. Sieben davon um 25 bis 208 Minuten; drei um eine oder zwei
Minuten, was der normale Abstand zwischen Schreiben und Committen ist.

Gemessen am 260824-1757 über `278a008..HEAD`, Stempel gegen die Autorenzeit des tragenden
Commits:

| voraus | Stempel | Commit | Datei |
|---|---|---|---|
| +208 | 260824-2115 | 1747 | `shared/issues/…_o_ein-commit-des-orchestrators-nimmt-die-git-mv-…` |
| +195 | 260824-2100 | 1745 | `history/…-coder-der-modulkopf-zieht-auf-die-fassung-des-wertes-nach.md` |
| +150 | 260824-2010 | 1740 | `history/…-coder-sechs-befunde-der-durchsicht-…` |
| +146 | 260824-1902 | 1636 | `history/…-coder-die-zaehlproben-zu-c6.md` |
| +100 | 260824-1755 | 1615 | `history/…-coder-die-anwendung-laedt-die-profile-…` |
| +56 | 260824-1650 | 1554 | `history/…-ontocoder-vierte-zustandszeile-…` |
| +25 | 260824-1612 | 1547 | `history/…-coder-der-siebte-inhalt-…` |

Die Uhr des Geräts geht richtig: `date` und die Commitzeiten stimmen überein.

---

**Sechs der sieben stammen von Sub-Agenten, einer vom Orchestrator.** Der Orchestrator hat
`date` beim Setup einmal gelesen und die Stempel danach geschätzt; die betroffenen Agenten
haben es offenbar gar nicht gelesen. Zwei Agenten desselben Laufs haben richtig gestempelt
(`260824-1739-ontocoder-…`, `260824-1751-analyst-…`), also ist es keine Eigenschaft der
Umgebung, sondern eine Frage, ob der Schreiber die Uhr fragt.

**Warum es zählt.** Der Stempel im Dateinamen ist in dieser Werkbank kein Schmuck: er ordnet
die Datensätze, er entscheidet in `bin/fusion-review-coverage` und in der Auslieferungsfassung
der Leseprofile (`juengste = { anzahl = 10 }`), welche zehn Verläufe die jüngsten sind, und die
Zählung „filed this session" im Sitzungsbericht vergleicht ihn gegen `session.started`. Ein
Stempel, der drei Stunden vorausläuft, sortiert eine Datei vor Dateien, die später entstanden
sind, und dieser Fehler ist aus der Datei selbst nicht erkennbar — nur aus dem Vergleich mit
dem Commit, den kaum jemand fährt.

**Die Regel steht schon.** `rules/fusion-workbench-conventions.md` und die Agentenanweisungen
verlangen, den Zeitstempel aus `date +%y%m%d-%H%M` zu holen und nicht zu schätzen. Es fehlt
nicht die Regel, sondern etwas, das ihre Einhaltung misst.

## Was in dieser Sitzung schon berichtigt ist

- Der Datensatz mit +208 ist auf `260824-1745` umbenannt.
- Die zwei Entscheidungsdatensätze `260824-0634_i_…` und `260824-1313_i_…` trugen in ihren
  Zeilen `Answered:` und `Implemented:` geschätzte Zeiten (1505 und 1650). Sie stehen jetzt auf
  den nachprüfbaren Commitzeiten 1546 und 1554.

## Was offen ist

Die sechs Verlaufsdateien tragen ihre falschen Stempel weiter. Sie umzubenennen ist eine
Entscheidung und keine Selbstverständlichkeit: eine Aufzeichnung eines Laufs behält sonst,
was sie zum Zeitpunkt des Laufs trug, und der Verweis darauf steht in Commitnachrichten und
Berichten dieser Sitzung.

**Möglichkeiten:** (1) stehen lassen und in diesem Datensatz als bekannt führen; (2) umbenennen
und die Verweise nachziehen; (3) eine Prüfung bauen, die den Stempel gegen die Commitzeit hält
— sie fände den Fehler bei künftigen Sitzungen und nicht in dieser.

**Domain:** code
**Gefunden:** analyst, beim Nachziehen von Spec und Plan; nachgemessen vom Orchestrator

---
**Nachgeprüft beim Abgleich zum Abschluss der Runde 16, 260824-1852: die Beschreibung stimmt, und
der offene Teil ist unverändert offen.** Die sechs Verlaufsdateien tragen ihre vorauslaufenden
Stempel weiter (`260824-2100`, `-2010`, `-1902`, `-1755`, `-1650`, `-1612` unter `history/` der
Runde 16). Die zwei berichtigten Stellen halten: der Datensatz mit +208 heißt
`shared/issues/260824-1745_*_ein-commit-des-orchestrators-…`, und die zwei
Entscheidungsdatensätze tragen die Commitzeiten 1546 und 1554.

**Der Abgleich hat eine Folge des Fehlers gefunden und geräumt.** Der Spec der Runde 16 zitierte
den umbenannten Datensatz weiter unter seinem alten Namen `260824-2115_o_…`, also einen Pfad, der
ins Leere zeigte. Das Zitat steht jetzt in der Sternform auf `260824-1745`. Die Umbenennung nach
Möglichkeit 2 zieht solche Verweise nach sich; dieser eine ist der erste gemessene Fall.

Also seen: 260906-0206 by coder — die Frage „umbenennen oder vermerken", an der dieser Datensatz und drei weitere hängen, ist jetzt als Entscheidung abgelegt: `260906-0206_*_werden-dateinamen-mit-vorauslaufendem-zeitstempel-umbenannt-oder-vermerkt.md`. Vier Datensätze aus vier Runden beschreiben denselben Vorgang; keiner ist ohne diese Antwort zu schließen.

---
Resolved: Der Nutzer hat am 260907-1301 Moeglichkeit 2 des Datensatzes `260906-0206_*_werden-dateinamen-mit-vorauslaufendem-zeitstempel-umbenannt-oder-vermerkt.md` gewaehlt: **der Bestand wird nicht umbenannt.** Die geschaetzten Zeitstempel bleiben in den Dateinamen stehen, und dieser Vermerk ist die Berichtigung. Die hier aufgezaehlten Dateien behalten ihre Namen. Umbenennen haette neunzehn Aufzeichnungen und jeden Verweis auf sie getroffen, ueber ein Suchmuster, das in diesem Projekt schon fuenfmal zu eng war. **Der Befund selbst ist damit nicht widerlegt, sondern als Lage angenommen:** der Verlaufsspeicher wird nach dem Namen sortiert gelesen, und in der so entstehenden Reihenfolge steht ein spaeterer Schritt vor einem frueheren. Wer die Abfolge braucht, liest sie aus dem Commitverlauf und nicht aus der Namensliste. Die vierte Moeglichkeit des Datensatzes, die Ursache pruefbar zu machen, hat der Nutzer als eigene Frage bestellt.
