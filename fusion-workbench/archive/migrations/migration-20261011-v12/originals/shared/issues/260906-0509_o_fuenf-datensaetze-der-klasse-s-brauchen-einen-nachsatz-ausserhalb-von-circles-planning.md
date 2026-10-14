Fünf Datensätze der Klasse S brauchen einen Nachsatz außerhalb von `circles/*/planning/`

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** Werkbankführung
**Baumstand:** `5cb5110`

---

Die vierte Behebungsschleife vom 260906 hat die 36 offenen Defektdatensätze der Klasse S — „Aussage
über einen eingefrorenen Spec-, Plan- oder Circle-Text" — neu sortiert und 30 davon geschlossen: 6
durch Arbeit im lebenden Baum, 24 durch einen datierten Nachsatz an der Spec- oder Plandatei, nach
der Nutzerentscheidung zu
`shared/decisions/260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md`.

**Fünf sind allein daran hängen geblieben, dass ihr Ziel nicht unter `circles/*/planning/` liegt.**
Der Auftrag jener Schleife nennt als Schreibfläche `crates/`, `xtask/` und „die Spec- und
Plandateien unter `fusion-workbench/circles/*/planning/`". Das ist keine Aussage über die Sache,
sondern über den Pfad, und die Sache ist bei allen fünf dieselbe wie bei den 24 geschlossenen: eine
Angabe in einem freigegebenen Text stimmt nicht, sie ist am Baum nachgemessen, und die Form der
Berichtigung liegt fest.

| Datensatz | Ziel des Nachsatzes | Was der Nachsatz sagen müsste |
|---|---|---|
| `circles/260813-0100-…/issues/260813-0642_*_zwei-hingenommene-verluste-stehen-auf-keiner-abnahmeliste.md` | `shared/planning/260813-0053_*_spec-…`, `## Randbedingungen` | Die Zusage „Kein Verlust gegenüber heute" hält nicht: zwei Verluste sind bewusst hingenommen, und beide erfüllen keinen der zwei Auswege, die die Randbedingung lässt. **Die Planhälfte dieses Datensatzes ist erledigt** (Nachsatz am Plan der Runde 7, Punkt 1); er bleibt allein wegen der Spechälfte offen. |
| `shared/issues/260821-1221_*_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md` | `shared/planning/260821-1115_*_spec-artefakt-und-release.md`, C6.3 | Die Zusage ist auf den Quellbaum begrenzt, wie der Plan sie gefasst hat; gemessen mit ``grep -rn 'sieben Stationen' README.md Makefile xtask/ CLAUDE.md`` → null Treffer. Wörtlich genommen bleibt das Kriterium unerfüllbar, weil es sich selbst zitiert. |
| `circles/260813-2332-…/issues/260814-0637_*_die-directive-im-circle-datensatz-nennt-drei-sicherungsmomente-der-spec-vier.md` | `circles/260813-2332-…/_b_circle.md`, `## Directive` | Es sind vier Sicherungsmomente. Die berichtigte Angabe steht schon in derselben Datei, in der Schließungsnotiz. |
| `circles/260813-2332-…/issues/260814-1002_*_die-directive-abweichung-steht-an-drei-stellen-des-circle-datensatzes-und-nicht-an-zwei.md` | dieselbe Datei, drei Stellen | Ergänzt die Aufstellung des vorigen um eine dritte Stelle; ein Nachsatz erledigt beide. |
| `shared/issues/260815-1047_*_die-directive-der-runde-10-und-ein-planschritt-schreiben-das-alte-leeren-weiter-fest.md` | `circles/260814-1551-…/_b_circle.md`, `## Directive` | Der Filtertext übersteht jeden Ordnerwechsel, gleich wie „Deep" steht (Nutzerentscheid 260815-0955, C1.9 des Spec). **Die Planhälfte ist am 260815-1145 berichtigt**; offen ist allein die Directive. |

---

**Zwei Sorten, und sie hängen nicht zusammen.**

Die ersten zwei zeigen auf `shared/planning/`. Dort liegen Spec- und Plandateien geschlossener
Runden genau wie in den Circle-Verzeichnissen; welche Runde ihren Spec wo abgelegt hat, ist
historisch und keine Aussage über seinen Rang. Hier fehlt nichts als die Erlaubnis, denselben
Handgriff einen Ordner weiter zu tun.

Die letzten drei zeigen auf den Abschnitt `## Directive` eines Circle-Datensatzes, und der gehört
nach `agents/shaper.md` dem Shaper im Modus `portfolio-activation`. Das ist keine Pfadfrage,
sondern eine Zuständigkeitsfrage, und sie ist mit der Nutzerentscheidung vom 260906 nicht
beantwortet: jene spricht von Spec und Plan und nicht von der Directive. **Der Orchestrator hat
für `260815-1047` schon einmal ersatzweise in die Schließungsnotiz geschrieben, weil er den
Abschnitt nicht anfassen darf** — der Ausweg steht also im Bestand, und er ist keiner, den
irgendwer gewählt hätte.

**Was zu tun ist.** Für die ersten zwei genügt ein Lauf mit `shared/planning/` in der
Schreibfläche; die Form des Nachsatzes steht fest und ist in den 24 geschlossenen Fällen
gleichlautend angewandt. Für die letzten drei ist zu entscheiden, ob ein Nachsatz unter einer
Directive dieselbe Zutat ist wie einer unter einem Spec, oder ob der Shaper sie fahren muss.

---
Stand 260908-1552: **die zwei Ziele unter `shared/planning/` sind erledigt, die drei unter
`## Directive` nicht, und der Datensatz bleibt deshalb offen.**

**Zeile 1 der Tafel** (`260813-0642_*_zwei-hingenommene-verluste-…`) ist geschlossen. Der
Nachsatz steht am 260908-1539 im Spec der Runde 7,
`260813-0053_*_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md`, Abschnitt
`## Randbedingungen`, unter dem letzten Aufzaehlungspunkt: beide Verluste namentlich, mit der
Begruendung, warum keiner der zwei Auswege der Randbedingung greift.

**Zeile 2** (`260821-1221_*_das-abnahmekriterium-c6-3-…`) ist am 260908 auf einem anderen Weg
geschlossen worden, ohne Nachsatz am Spec: der Quellbaum ist nachgemessen, die Zusage des Plans
haelt, und der Datensatz selbst traegt die Begruendung, warum C6.3 und die drei Aufzeichnungen
ihren Wortlaut behalten.

**Die Zeilen 3 bis 5 haengen unveraendert an der Zustaendigkeitsfrage**, und die ist jetzt als
Nutzerfrage abgelegt statt in diesem Absatz zu stehen:
`260908-1608_*_gilt-die-nachsatzregel-vom-260906-auch-unter-dem-abschnitt-directive-eines-circle-datensatzes.md`.
Sie fuehrt drei Wege und empfiehlt keinen; was sie entscheidbar macht, ist die Auskunft, ob ein
Shaper-Lauf im Modus `portfolio-activation` an einem geschlossenen Circle-Datensatz ueberhaupt
moeglich ist.

Dieser Datensatz schliesst, sobald jene Frage beantwortet und ihre Antwort an den drei Stellen
ausgefuehrt ist.
