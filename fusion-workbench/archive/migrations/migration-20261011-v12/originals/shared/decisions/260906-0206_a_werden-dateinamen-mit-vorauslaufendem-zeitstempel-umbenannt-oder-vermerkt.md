# Werden Dateinamen mit vorauslaufendem Zeitstempel umbenannt, oder bekommen sie einen Vermerk?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260812-1805_*_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`, `260828-1044_*_fuenf-history-dateien-der-runde-20-tragen-zeitstempel-die-nach-ihrem-eigenen-commit-liegen.md`, `260824-1758_*_die-zeitstempel-in-dateinamen-laufen-der-uhr-voraus-bis-zu-drei-stunden.md`, `260818-0343_*_zwei-dateien-dieser-sitzung-tragen-einen-zeitstempel-fast-zwei-stunden-in-der-zukunft.md`, `260810-0918_*_der-plan-zitiert-einen-defekt-mit-einem-zeitstempel-den-sechs-datensaetze-tragen.md`, `rules/fusion-workbench-conventions.md` `## Timestamps`

---

## Question

Vier offene Defektdatensätze aus vier Runden halten denselben Vorgang fest: ein Agent hat den Zeitstempel eines Dateinamens geschätzt statt `date +%y%m%d-%H%M` zu rufen, und die Datei trägt seither eine Uhrzeit, die zum Zeitpunkt des Schreibens nicht erreicht war — sechs Verlaufsdateien der Runde 6 mit einer Abweichung, die monoton von 24 Minuten auf 132 wächst, fünf der Runde 20 mit Stempeln nach ihrem eigenen Commit, sechs der Runde 16 mit bis zu drei Stunden, zwei aus einer Sitzung im August.

Keiner der vier ist behoben, und keiner kann behoben werden, ohne dieselbe Frage zu beantworten: **werden die Dateien umbenannt, oder bekommen sie einen Vermerk?** Zwei der Datensätze sagen ausdrücklich, dass die Antwort beim Nutzer liegt („ob die Namen umbenannt werden oder ein Vermerk genügt, entscheidet der Nutzer", `260828-1044_*`), einer schlägt das Umbenennen vor und nennt die Verweisprüfung gleich mit (`260812-1805_*`). Solange die Frage offen ist, wachsen die Datensätze nach und keiner schließt.

Der Schaden ist derselbe in allen vier Fällen und liegt nicht am Inhalt: `history/` wird nach dem Namen sortiert gelesen, und die Reihenfolge ist die einzige Auskunft darüber, was worauf folgte. Sie stimmt hier nicht, und zwar so, dass Schritt 2 nach Schritt 7 erscheint.

## Options

1. **Umbenennen auf die belegte Zeit.** Jede betroffene Datei bekommt den Stempel, den die Erhebung ihres Datensatzes nennt (Änderungszeit im Dateisystem beziehungsweise Commitzeit), und jeder Verweis darauf wird nachgezogen.
   - Pro: die Sortierung stimmt danach, und sie stimmt für jeden künftigen Leser ohne Zusatzwissen. Das ist die einzige Möglichkeit, die den eigentlichen Schaden behebt.
   - Contra: sie fasst Aufzeichnungen an, und der Zeitstempel im Namen ist zugleich die Kennung, über die zitiert wird. Jede Suche muss dabei das Muster ohne `.md` mitnehmen, sonst entgehen ihr die Kurzformen — genau die Falle, die `CLAUDE.md` als blinden Fleck jeder `\.md`-Suche führt und die fünf Erhebungen schon getroffen hat. Und die Änderungszeit im Dateisystem ist kein verlässlicher Beleg mehr: nach einem frischen Auszug des Repositorys trägt jede Datei die Zeit des Auszugs. Belegt bleibt allein, was der jeweilige Datensatz damals gemessen hat.
2. **Vermerk am Datensatz, Namen bleiben.** Der Defektdatensatz behält seine Tabelle mit Name gegen belegte Zeit, wird auf `_c_` gezogen und ist damit die eine Stelle, an der die richtige Reihenfolge steht.
   - Pro: kein Zitat bricht, keine Aufzeichnung wird angefasst, und die Arbeit ist erledigt in vier Vermerken. Die Belege sind schon erhoben und stehen in den Datensätzen.
   - Contra: der Leser, der `history/` sortiert überfliegt, liest die falsche Reihenfolge weiter und erfährt nie, dass es einen Vermerk gibt. Der Datensatzspeicher wird sauber, der Verlaufsspeicher bleibt falsch.
3. **Vermerk in der Datei selbst.** Die Namen bleiben, und jede betroffene Verlaufsdatei bekommt in ihrem Kopf eine Zeile „Geschrieben um HH:MM; der Stempel im Namen läuft vor."
   - Pro: die Auskunft steht dort, wo sie gebraucht wird, und kein Zitat bricht. Der Leser, der die Datei öffnet, sieht es sofort.
   - Contra: wer nur die Namensliste überfliegt — und das ist der Fall, für den die Sortierung da ist —, öffnet keine Datei. Und es sind neunzehn Dateien einzeln anzufassen.
4. **Die Ursache prüfbar machen und den Bestand liegen lassen.** Eine Prüfung hält den Stempel jedes neuen Datensatzes gegen die Zeit seines Commits; der Altbestand bleibt, wie er ist.
   - Pro: die einzige Möglichkeit, die den Zufluss beendet. Alle drei anderen räumen den Bestand und lassen den nächsten Lauf denselben Fehler machen — vier Runden haben ihn gemacht.
   - Contra: sie behebt nichts von dem, was dasteht, und lässt die vier Datensätze offen. Und sie ist die teuerste: ein Stempel darf legitim vor der Commitzeit liegen (eine Datei wird geschrieben, bevor sie eingetragen wird), also braucht die Prüfung eine Toleranz, und über deren Höhe ist wieder zu entscheiden.

## Constraints

- Die Ortsregel aus `CLAUDE.md` bindet: `history/` ist einer der sieben Speicher, deren Dateien ihren damaligen Stand behalten. Möglichkeit 1 und 3 fassen sie an, und die Antwort muss sagen, ob und warum das zulässig ist.
- Eine Umbenennung zieht jeden Verweis nach — in voller Form **und** in Kurzform ohne `.md`. Ohne diese Erweiterung des Suchmusters entstehen tote Zeiger, und der Speicher trägt für diesen Fehlertyp schon zwei eigene Datensätze.
- Die belegte Zeit steht in den Datensätzen und ist am heutigen Dateisystem nicht mehr nachzumessen. Wer Möglichkeit 1 wählt, nimmt die Zahlen von damals.
- Möglichkeit 4 schließt keine der anderen aus; sie ist die einzige, die daneben stehen kann.

## Recommendation

Möglichkeit 2 für den Bestand, und Möglichkeit 4 als eigene Frage daneben. Der Schaden ist die falsche Sortierung eines Speichers, den Agenten überfliegen; er ist real, aber er ist kleiner als der Preis, neunzehn Aufzeichnungen umzubenennen und dabei jeden Verweis über ein Suchmuster nachzuziehen, das in diesem Projekt fünfmal zu eng war. Für Möglichkeit 1 spräche, wenn die Reihenfolge einmal wirklich zu einem Fehlschluss geführt hätte; das ist in keinem der vier Datensätze belegt.

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-1301 beantwortet` — Moeglichkeit 2 fuer den Bestand: Vermerk am jeweiligen Datensatz, die Namen bleiben. Umbenennen kostet neunzehn Aufzeichnungen und jeden Verweis ueber ein Suchmuster, das hier schon fuenfmal zu eng war. Zugleich ist Moeglichkeit 4 als eigene Frage bestellt: die Ursache pruefbar machen; ruled by user, Kai Stalmann <kai@stalmann.org>.
