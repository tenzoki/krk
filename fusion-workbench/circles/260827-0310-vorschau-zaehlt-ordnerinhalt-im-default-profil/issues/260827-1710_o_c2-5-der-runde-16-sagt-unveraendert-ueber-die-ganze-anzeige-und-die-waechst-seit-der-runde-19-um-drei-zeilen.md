C2.5 der Runde 16 sagt „unverändert" über die ganze Anzeige, und die wächst seit der Runde 19 um drei Zeilen

---

Das Kriterium C2.5 im Spec der Runde 16 trägt zwei Aussagen. Die erste, die Aufzählung der sechs Metadatenangaben für einen Ordner ohne Profiltreffer, gilt nach der Runde 19 unverändert. Die zweite, „unverändert gegenüber dem Stand vor dieser Runde", bezieht sich im Wortlaut auf die Anzeige als Ganzes, und die trägt seit HEAD `5e506e6` drei Zählzeilen unter den sechs Angaben. Der fremde Spec wird nicht angefasst; der Datensatz bleibt offen, bis der Nutzer den Abnahmelauf der Runde 16 gefahren hat.

---

**Filed by:** analyst, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Domain:** code
**Tree state:** `5e506e6`
**Affected:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md:170` (Kriterium C2.5); `crates/krk-ui/src/appkit/vorschau.rs:1381-1415` (`metadaten_text`)
**Cross-references:** `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-0646_*_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md:39-45` (Ursache, Abschnitt `## Die Zusage C2.5 der Runde 16 ist berührt und der Sache nach gewahrt`, und `:249`, sechste Anweisung unter `## Open for Planner`); `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md:172-176` (Abschnitt 6) und `:226-229` (Schritt 7); `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_b_circle.md:84` (der Abnahmelauf der Runde 16 ist Nutzerarbeit und steht aus)

## Der Befund

Der Wortlaut von C2.5 (Spec der Runde 16, `:170`):

> Ein Ordner, für den weder ein Pfadmuster noch eine Kennzeichendatei trifft, zeigt die Metadatenanzeige mit Name, vollem Pfad, Größe, Änderungsdatum, Rechten und Typ, unverändert gegenüber dem Stand vor dieser Runde.

Zwei Aussagen stecken darin.

1. **Die Aufzählung der sechs Angaben.** Sie gilt. `metadaten_text` (`crates/krk-ui/src/appkit/vorschau.rs:1406-1414`) formatiert weiterhin `Name`, `Pfad`, `Größe`, `Geändert`, `Rechte` und `Typ` in dieser Reihenfolge, und die Zählzeilen kommen als siebter Platzhalter **hinter** „Typ" über `zeilen_als_text(zaehlzeilen)` (`:1414`) dazu. Sie ersetzen nichts. Das ist die Festlegung 1 des Nutzers vom 260827, die der Spec der Runde 19 unter `:41` festhält: die Zählzeilen treten unter die sechs.

2. **„unverändert gegenüber dem Stand vor dieser Runde."** Das trifft für die Anzeige als Ganzes nicht mehr zu. Ein Ordner ohne Treffer aus `readers.toml` wird seit `5e506e6` mit `Inhalt::Metadaten { metadaten, zaehlzeilen }` gezeigt (`vorschau.rs:1111-1117`), und die drei Zeilen für Dateien, Ordner und Verknüpfungen wachsen unter die sechs Angaben (Spec der Runde 19, `:12`). Die Anzeige ist also um drei Zeilen länger als vor der Runde 16 und vor der Runde 19.

**Ursache** ist der Spec der Runde 19, `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-0646_*_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, dessen Abschnitt `:39-45` diese Berührung ausdrücklich als Zweck der Runde und nicht als Defekt ausschreibt. Dieser Datensatz ist ein Befund über einen Text und nicht über Code: der gebaute Stand tut, was die Runde 19 verlangt.

**Der Spec der Runde 16 wird nicht angefasst.** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_*_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` steht auf `_o_`, seine Kriterien warten auf den Abnahmelauf des Nutzers, und ein fremder freigegebener Wortlaut ist nicht der Ort, an dem die Runde 19 ihre Wirkung einträgt. Die Buchung geschieht durch dieses Zitat, nicht durch eine Änderung dort (Plan der Runde 19, `:172-174`).

## Schließbedingung

Der Datensatz bleibt offen, weil der Abnahmelauf der Runde 16 aussteht (`_b_circle.md:84`: sieben Kriterien verlangen KRK im Vordergrund) und die Schließung dem Nutzer gehört. Wer C2.5 abnimmt, liest die erste Aussage als das Kriterium und nimmt für die zweite diesen Datensatz als Erklärung: drei Zählzeilen unter den sechs Angaben sind kein Verstoß gegen C2.5, sondern die Runde 19. Geschlossen wird er mit jenem Abnahmelauf.
