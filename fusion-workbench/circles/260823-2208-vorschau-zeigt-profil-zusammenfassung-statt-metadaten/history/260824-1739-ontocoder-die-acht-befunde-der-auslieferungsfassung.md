# Die acht Befunde der Auslieferungsfassung

**Agent:** ontocoder
**Status:** Complete
**Datei:** `resources/default-readers.toml`, und keine zweite
**Quelle:** `reviews/260824-1700-ontorev-auslieferungsfassung-der-leseprofile.md`, acht Befunde
**Werkzeug:** ein Wegwerfprogramm außerhalb des Baumes gegen `regex` 1.13.1 und `toml` 1.1.4,
dieselben Fassungen, die `Cargo.lock` führt. Es liest die Datei mit den Strukturen aus
`leseprofil::datei`, baut die Erkennung aus `leseprofil::erkennung` nach, rechnet den Haushalt nach
der Regel aus `leseprofil::bausteine` und wertet jede Zeile am echten Bestand dieser Werkbank aus.
**Das Modell ist gegen den Baum geeicht**: es liefert für die unveränderte Datei (5, 11) für das
Rundenprofil und (3, 5) für die Wurzel, dieselben Paare, die
`crates/krk-core/tests/leseprofil.rs:2131-2187` behauptet.

## Was geräumt ist

Sieben der acht Datensätze stehen auf `_c_` und tragen ihr `Resolved:`. Der achte bleibt offen.

| Datensatz | Sachstand |
|---|---|
| `260824-1649_c_der-defektspeicher-zaehlt-zwei-von-vier-markern-…` | fünf Zeilen statt drei, Kommentar auf vier Marker |
| `260824-1650_c_die-zeile-sitzung-liefert-bei-leerem-current-abschnitt-…` | neues Feldmuster, gegen neun Gestalten gemessen |
| `260824-1651_c_der-kopf-des-speicherprofils-nennt-achtzehn-orte-…` | Kopf steht auf den Namen statt auf den Orten |
| `260824-1652_c_ein-abschliessender-schraegstrich-in-ordner-…` | alle fünf abgewiesenen Formen aufgezählt |
| `260824-1653_c_zwei-bausteinbeschreibungen-sagen-weniger-…` | Verankerungssatz bei `feld`, dritter Ausgang bei `vorhandensein` |
| `260824-1654_c_die-verlaufszeile-des-rundenprofils-traegt-kein-muster-…` | `muster = '\.md$'` ergänzt |
| `260824-1656_c_der-kopf-der-auslieferungsfassung-braucht-das-wort-ablage-…` | „Ablage" steht im Kopf nicht mehr |
| `260824-1655_o_sechs-speicher-unter-archive-…` | **offen**, siehe unten |

## Die Zeile „Sitzung": der Vorschlag des Datensatzes ist nicht übernommen

Der Datensatz trug einen gemessenen Vorschlag, `## Current\n[^\S\n]*([^\n]+)`, gegen fünf Gestalten
von `orchestrator-live.md`. Vier weitere Gestalten dazugerechnet, ergibt sich ein **dritter**
falscher Ausgang: die Gestalt mit einer Leerzeile unter `## Current` und dem Wert darunter liefert
den Platzhalter, obwohl der Wert dasteht. Genau diese Gestalt schreibt `werkbankwurzel` in
`crates/krk-core/tests/leseprofil.rs:1529`.

Nachgewiesen und nicht abgeleitet: mit dem Vorschlag in der Datei fällt
`die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` an `leseprofil.rs:2206` mit
`left: … Nicht …` gegen `right: … Text("Schritt 12, die Zaehlproben") …`. Die Datei ist danach aus
der Sicherung zurückgeholt worden; die Probe gehört dem `coder` und ist unberührt.

Gebaut ist stattdessen `## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)`, eine Fanggruppe. Es
nimmt die erste Zeile unter `## Current`, die etwas trägt, überspringt Leerzeilen und hält vor der
nächsten Überschrift an.

| Gestalt | ausgeliefert | Vorschlag | gebaut |
|---|---|---|---|
| die echte `fusion-workbench/orchestrator-live.md` | Wert | Wert | Wert |
| Vorlage `skills/setup/SKILL.md:107-114`, mit Zeilenende | Wert | Wert | Wert |
| dieselbe ohne Zeilenende am Dateiende | **Platzhalter** | Wert | Wert |
| Vorlage `skills/next/SKILL.md:234-241` | Wert | Wert | Wert |
| `## Current` leer, `## This Turn` folgt | **`## This Turn`** | Platzhalter | Platzhalter |
| `## Current` leer, Leerzeile am Dateiende | Platzhalter | Platzhalter | Platzhalter |
| `## Current` leer, nichts dahinter | Platzhalter | Platzhalter | Platzhalter |
| Überschrift umbenannt | Platzhalter | Platzhalter | Platzhalter |
| Gestalt des Prüfordners (Leerzeile, dann Wert) | Wert | **Platzhalter** | Wert |

Alle vier Vorlagen, die fusion für diese Datei ausliefert, setzen den Wert unmittelbar unter die
Überschrift, zwei Leerzeichen eingerückt; nachgesehen in `agents/orchestrator.md:51` und `:1175`,
`skills/setup/SKILL.md:113` und `skills/next/SKILL.md:240`. Die Gestalt mit der Leerzeile schreibt
allein der Prüfordner. Das gebaute Muster nimmt beide an.

**Ein Rest bleibt.** Die Kiste `regex` kennt kein `(?!…)`, also trennt das erste Zeichen der
Fanggruppe (`[^#\n]`) die Überschrift vom Wert; eine Zeile, die selbst mit `#` beginnt, wird nicht
gelesen. Der Kommentar in der Datei sagt es dem Nutzer. `(?s)` ist gefallen, es trug nichts.

## Der Defektspeicher: beide Zeilen, wie der Nutzer entschieden hat

Der Nutzer hat am 260824-1935 beide Zeilen gewählt. Das Profil trägt jetzt „Datensätze",
„Offen", „Geschlossen", „Zurückgestellt" und „Die jüngsten zehn".

Am Bestand nachgerechnet über die neunzehn Defektspeicher: 622 Datensätze, davon 178 `_o_`, 440
`_c_`, 4 `_d_`, 0 `_p_`. 178 + 440 + 4 + 0 = 622, **die Summe geht auf**. `_p_` bekommt keine
eigene Zeile und wird von der Gesamtzahl abgefangen; der Kommentar über dem Profil schreibt beides
aus und nennt jetzt alle vier Marker statt zwei.

## Die vier Haushaltszahlen nach der Änderung

| Profil | Zeilen | Leseläufe | Öffnungen |
|---|---|---|---|
| die Wurzel | 7 | **3** | **5** |
| ein Speicher | 2 | 1 | 10 |
| ein Defektspeicher | **5** (vorher 3) | 1 | 10 |
| alle Runden | 1 | 1 | 0 |
| eine Runde | 9 | **5** | **11** |

**Die vier Zahlen der Probe sind unverändert: (5, 11) und (3, 5).** Keine Zeile ist im Rundenprofil
oder in der Wurzel hinzugekommen, und die zwei neuen Zeilen des Defektspeichers sind eine
`zaehlung` **ohne** `ordner`, benutzen also den einen ohnehin fälligen Leselauf: das Profil bleibt
bei einem Leselauf und zehn Öffnungen, gegen die Grenzen 12 und 24 aus C6.4. Die zusätzliche
`muster`-Angabe der Verlaufszeile kann Öffnungen nur senken, hier keine.

## Trefferzahlen gegen den echten Bestand

Gemessen gegen alle **154** Verzeichnisse unter `fusion-workbench/`, vor und nach der Änderung
gleich: die Wurzel 1, ein Speicher 99, ein Defektspeicher 19, alle Runden 1, eine Runde 18, ohne
Profil 16. **33 reguläre Ausdrücke, keiner beanstandet, jedes Feldmuster mit genau einer
Fanggruppe** (vorher 30; drei kommen mit den neuen Zeilen und der `muster`-Angabe dazu).

Je Zeile am Bestand: die sechs Feldbausteine treffen alle, `Directive` 18 von 18. Die vier
Zustandszeilen antworten 0 / 1 / 15 / 2 mit „ja", zusammen 18, jedes Verzeichnis bejaht genau eine.
„Spec" 8 zu 10, „Plan" 14 zu 4, kein Platzhalter. Die Verlaufszeile des Rundenprofils nimmt nach
der Änderung 150 Verläufe statt 151 auf; der eine wegfallende ist die `.gitkeep` in
`circles/260804-0933-…/history/`, und kein Eintrag ohne `.md` bleibt übrig.

## Warum `260824-1655` offen bleibt

Ob die sechs Speicher unter `archive/` ein Profil bekommen sollen, ist eine Entscheidung und kein
Mangel; das sagt der Datensatz selbst, und die Durchsicht empfiehlt es als Frage an den Nutzer.
Dem Datensatz ist ein drittes Stück Grundlage angefügt, gemessen und nicht gelesen: die zweite der
zwei Zeilen des Speicherprofils ist „Die jüngsten zehn", und in einem eingefrorenen Bestand ist das
Änderungsdatum das des Archivlaufs. Die 51 Dateien in
`archive/260819-1613-safe-cleanup-tier-1/shared/issues` tragen zusammen **fünf** verschiedene
Änderungszeiten, die 14 in `…/shared/decisions` ebenfalls fünf. Die Hälfte der Auskunft eines
Speicherprofils wäre dort keine.

## Was daneben liegen bleibt

Ein neuer Datensatz,
`issues/260824-1739_o_die-raeumung-der-acht-befunde-macht-vier-stellen-in-spec-und-plan-falsch.md`:
vier Stellen in Spec und Plan beschreiben den Stand vor dieser Räumung. Sie liegen in Dateien des
`analyst` und sind deshalb nicht mitgeräumt worden.

## Abnahme

`make check` → Exit 0, alle vier grün.
`cargo test -p krk-core --test leseprofil` → 35 bestanden, 0 gescheitert, 1 übersprungen.
