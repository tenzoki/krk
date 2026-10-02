Drei Kopffelder `**Active spec/plan:**` zeigten nach zwei Aufräumläufen ins Leere

---

Die Kopffelder von drei geschlossenen Runden nannten ihren Spec beziehungsweise ihren Plan mit
vollem, werkbank-relativem Pfad unter `shared/planning/`. Zwei Aufräumläufe haben diese Dateien
nach `archive/` bewegt; unter dem genannten Pfad lag danach nichts mehr.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** Werkbankführung
**Baumstand:** `2f4b7b2`
**Schwere:** gering im Schaden, mittel als Gestalt. Kein Bau, kein Verhalten. Ein Leser, der
dem Feld folgt, findet nichts und kann nicht unterscheiden, ob die Datei gelöscht wurde oder
der Zeiger falsch ist.

## Die drei, am 260908-1552 aufgelöst

Jedes der drei Felder nannte den gemeinsamen Planungsspeicher als Vorsatz vor dem Dateinamen;
der Vorsatz ist hier absichtlich nicht ausgeschrieben, weil er in einem Zitat genau der Fehler
ist, den dieser Datensatz beschreibt.

| Datensatz | genannte Datei | steht seit |
|---|---|---|
| Circle-Datensatz von `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`, Zeile 7 | `260817-0536_*_spec-absicherung-jedes-loeschwegs.md` | Aufräumlauf `260819-1613` unter `archive/` |
| Circle-Datensatz von `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps`, Zeile 7 | `260818-1510_*_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md` | derselbe Lauf |
| Circle-Datensatz von `260821-1644-veroeffentlichen-als-achte-station`, Zeile 6 | `260821-1221_*_plan-artefakt-und-release.md` | Aufräumlauf `260826-1637` |

Erhoben über alle Kopffelder `**Active spec/plan:**` und `**Active session history:**` sämtlicher
Circle-Datensätze; jeder genannte Pfad wurde zuerst wörtlich und dann über
`find fusion-workbench -name '<basename mit _*_>'` aufgelöst. Andere tote Zeiger dieser Klasse
gibt es nicht — die übrigen Felder nennen entweder einen Pfad, der steht, oder bereits die
speicherlose Kurzform.

## Was daran die Gestalt ist und nicht der Einzelfall

**Ein Aufräumlauf bewegt eine Datei, und jeder Zeiger mit Speicherabschnitt stirbt dabei.**
Genau das nennt `260907-2340_*_wie-weit-reicht-die-neue-regel-fuer-den-zustand-eines-anforderungsdokuments-in-den-bestand-zurueck.md`
als Nebenwirkung seiner Möglichkeit 3, dort für Umbenennungen. Hier ist es keine Nebenwirkung
einer noch offenen Wahl, sondern zweimal eingetreten, und niemand hat es bemerkt.

`rules/fusion-workbench-conventions.md` `## Filename Patterns` hat die Antwort und begründet sie
mit derselben Ursache: „A citation carrying a store segment is a violation the gates report: the
segment is what a sweep moves, so a citation spelling it dies at the sweep." Die drei Felder
haben den Abschnitt buchstabiert.

## Warum die Reparatur die Kurzform nimmt und nicht den Archivpfad

Die speicherlose Kurzform überlebt den nächsten Aufräumlauf; ein Pfad unter `archive/<lauf>/`
täte es nicht besser als der unter `shared/planning/`, denn ein späterer Lauf kann ihn erneut
bewegen. Der Baum hat die Form schon: der Circle-Datensatz von
`260830-1045-git-bereich-liest-status-branch-verlauf` führt sein Kopffeld seit dem Abschluss als
`260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, also ohne Speicherabschnitt.

**Für einen geschlossenen Circle ist das Feld Beleg und kein maschinell gelesener Pfad.**
`skills/next/SKILL.md` liest es beim Aktivieren, und aktiviert wird allein ein vorgesehener
Circle. Das ist die Grenze, an der dieser Befund und
`260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md`
sich trennen: jener fragt nach der Form des Feldes an einem **laufenden** Circle, dieser
repariert es an drei geschlossenen. Die Frage jenes Datensatzes bleibt unberührt offen.

## Abnahme

Jeder Pfad in einem Kopffeld `**Active spec/plan:**` und `**Active session history:**` löst
über eine werkbankweite Suche auf genau einen Träger auf.

---
Resolved: Die drei Kopffelder sind am 260908-1552 auf die speicherlose Kurzform gezogen, mit
dem Zusatz in Klammern, unter welchem Aufraeumlauf die Datei heute liegt. Nachgeprueft: alle
fuenf genannten Dateien — die zwei Plaene und die drei Specs — loesen ueber
`find fusion-workbench -name '<basename mit _*_>'` auf **genau einen** Traeger auf.

**Was damit nicht behoben ist, und es ist der groessere Teil.** Fast jedes uebrige Kopffeld
`**Active spec/plan:**` und `**Active session history:**` nennt seine Datei weiter mit vollem
Speicherpfad; erhoben mit ``grep -h '^\*\*Active ' fusion-workbench/circles/*/[_]*circle.md |
grep -cE 'circles/|shared/'``. Diese Pfade loesen heute alle auf — die Dateien stehen noch, wo
sie stehen —, und sie sterben genauso, sobald ein Aufraeumlauf eine davon bewegt. Das ist
dieselbe Gestalt einen Schritt frueher, es ist ein eigener Befund und als solcher abgelegt
(`260908-1601_*_fast-jedes-kopffeld-eines-circle-datensatzes-nennt-seine-datei-mit-speicherpfad-und-stirbt-am-naechsten-aufraeumlauf.md`).
Die Zeile darueber hat in ihrer ersten Fassung das Gegenteil behauptet; sie ist am 260908-1601
nachgemessen und berichtigt worden, bevor sie irgendwo zitiert war.

Die Form des Feldes an einem **laufenden** Circle ist damit nicht entschieden; jene Frage
haengt unveraendert an
`260818-0753_*_die-ausnahme-fuer-maschinell-gelesene-kopffelder-steht-nur-in-einem-geschlossenen-datensatz.md`.
