# Portfolio

**Generated:** 260910-0735 (by playmaker session 260910-0735-playmaker-direct-dispatch)
**Domain bias:** code

Bestand: 1 vorgesehen, 0 aktiv, 9 kohärent geschlossen, 13 beschränkt geschlossen, 0
überholt, 2 zurückgestellt. Summe 25 Circle-Datensätze. Die Runde 18 ist ohne
Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

**Die Rangfolge hat erstmals seit dem 260821-2202 wieder einen Gegenstand.** Der Shaper hat
heute früh über `/fusion:direct` die vorgesehene Runde
`260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap` angelegt. Sie ist die
empfohlene nächste, und sie ist die einzige.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Das ist der reguläre
Zustand nach dem beschränkten Abschluss der Runde 23 am 260831-2024. Kein Datensatz eines
anderen Checkouts steht hier: der Bestand kennt keinen aktiven Anspruch.

## Anticipated (_a_) — ranked

Recommended next: `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap` — einziger
vorgesehener Circle, Grundlage heute geschrieben, kein Hindernis ohne benannten Weg daran
vorbei.

**1. `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`**
Directive: KRK meldet beim Start, was eine neue Fassung an `readers.toml`, `settings.toml`
und `keymap.toml` mitbringt, und zeigt den Unterschied auf Abruf.

Die Runde schliesst die Lücke, die `CLAUDE.md` unter „Was man nicht sieht" als zweite Regel
derselben Bauart führt: KRK legt die drei von Hand gepflegten Ablagedateien beim ersten
Start an und fasst sie danach nie wieder an, also sieht niemand die Profile und Tasten, die
eine neue Fassung mitbringt. Der Vergleich braucht dafür weder Bündel noch Netz, weil alle
drei Auslieferungsfassungen über `include_str!` einkompiliert daneben stehen; das steht in
der Grundlage des Datensatzes mit den Fundstellen. Zwei Datensätze liegen auf dem Weg, und
die Grundlage nennt beide beim Namen. Die Nutzerfrage
`260907-1407_*_bekommt-session-toml-eine-fassungsangabe-damit-auch-die-zweite-haelfte-der-bestandsregel-greifen-kann.md`
steht offen und ist keine Sperre, sondern eine Überschneidung: wer die gemeldete
Versionsnummer nach `session.toml` schreibt, beantwortet jene Frage mit. Der Defekt
`260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md`
steht ebenfalls offen, und die Startzeile dieser Runde träte genau in seine Kollision; die
Grundlage schreibt vor, ihn zu beheben oder die Zeile ausdrücklich gegen ihn zu stellen.
Beide Abhängigkeiten auf andere Runden, `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
und `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln`, sind beschränkt geschlossen. Die
übliche Rangregel liest daraus eine unerfüllte Vorbedingung; in diesem Projekt trifft das
nicht zu, weil der beschränkte Abschluss hier die Verfügbarkeit des Nutzers für den
Abnahmelauf misst und nicht die Reife der Runde (`CLAUDE.md`, „Projektstand"). Gebaut ist
beides, und beide Runden binden diese hier mit dem, was sie gebaut haben.

Aktiviert wird über `/fusion:next`. Dieser Lauf hat den Datensatz nicht umbenannt und
`.active-circle` nicht geschrieben; angehängt ist allein der Abschnitt
`## Activation proposal`.

## Backlog — ranked

Recommended to shape: (keiner) — der Ablagespeicher hält keinen lebenden Eintrag.

Beide Einträge stehen auf geschlossen, seit der Nutzer ihre Schliessung am 260909-2209 über
`/fusion:next` bestätigt hat. Dieser Lauf hat im Ablagespeicher nichts gelesen, was eine
Rangfolge trüge, nichts umbenannt und nichts vorzuschlagen. Neue Ideen kommen über
`/fusion:memo` herein; ein Agent legt keinen Eintrag an.

Performed this run: keine Ablageoperation.

## Recently closed (_c_ / _b_)

1. `260830-1045-git-bereich-liest-status-branch-verlauf` (`_b_`, 260831-2024) — der
   Git-Bereich der Stufe A steht: ein sechster Bereich der Fensterzeile mit Branch,
   Statuszusammenfassung, Verlaufsliste und den Einzelheiten des ausgewählten Commits,
   `Fokus::Git` auf `shift+cmd+b`, eine fünfte Spalte mit fünf Markenzuständen in beiden
   Dateifenstern, gelesen mit `gix` 0.87.1 und ohne jeden Schreibweg ins Repository.
   Beschränkt, weil der Abnahmelauf am laufenden Bündel Nutzerarbeit ist.
2. `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` (`_c_`, 260829-1226) — `cmd+v`
   im Dateifenster hängt den Ablageinhalt an den Filtertext an, und der Filter versteht `*`
   als Platzhalter. Ein Turn mit zwölf Schritten, Commits `79d507a..8d64859`; Abnahmelauf
   vom Nutzer auf `415ef6f` gefahren. Ausgeliefert als 1.4.0.
3. `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab` (`_c_`, 260829-0737) — `cmd+c` und
   `cmd+x` legen die betroffenen Einträge als Dateiverweise auf die Zwischenablage, für den
   Finder und andere Anwendungen. Ein Turn mit neun Schritten, Commits `4bd0084..35b95b3`.
   Ausgeliefert als 1.3.0.
4. `260827-2028-vorschau-rendert-pdf-als-betrachter` (`_c_`, 260828-1055) — die Vorschau
   rendert PDF als Betrachter mit Zoom, Seitensprung und Seitenzähler. Elf Planschritte,
   Commits `2033626..48cd818`; der erste Abnahmelauf fand den Stapelüberlauf beim Zoom.
5. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`, 260827-1920) — die
   Vorschau zählt den Inhalt eines Ordners in einem eingebauten Default-Profil. Acht
   Planschritte, neun Commits `a5c7a46..d444879`.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`, 260821-2202) — KRK
  zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt und nicht verschoben: der Nutzer
  hat das Abgeben an den Systembrowser gewählt
  (`260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`
  im gemeinsamen Entscheidungsspeicher). Das Vokabular kennt für eine Absage keinen eigenen
  Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK setzt
  Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier heisst der Marker
  „später": die Runde war aktiv und ist der Löschabsicherung gewichen.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- **Veraltete Grundlage nach Zählung, in der Sache nicht.** `stale-grounding:
  260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap: 2 von 4 zitierten
  Datensätzen terminal oder archiviert; HEAD unbekannt viele Commits hinter dem Stand der
  Grundlage, weil sie keinen Commit festhält.` Die Schwelle greift bei der Hälfte, und die
  Hälfte ist erreicht. Empfohlen wäre danach ein erneutes Schärfen über den Shaper vor der
  Aktivierung. Dieser Lauf empfiehlt es nicht: die Grundlage ist heute um 0707 geschrieben,
  und beide terminalen Zitate binden mit Absicht fertige Arbeit, nämlich eine umgesetzte
  Nutzerfrage und die Runde, deren Regel `Ersatz::Nichts` die neue Runde ausdrücklich nicht
  antastet. Der Rang bleibt unverändert.
- **Propagationsbefund geprüft, kein Vermerk gesetzt.** Die Grundlage der vorgesehenen Runde
  zitiert die beschränkt geschlossene Runde
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`, womit die
  Propagationsregel mechanisch greift. Auf die Sache trifft sie nicht zu: der Abschluss
  liegt am 260824, die Grundlage ist siebzehn Tage später geschrieben und verarbeitet ihn
  ausdrücklich. Ein Anhang an einen Datensatz ist endgültig, also hat dieser Lauf keinen
  Abschnitt `## Parent grounding stale` angehängt und legt den Befund stattdessen hier vor.
  Wer ihn anders liest, sagt es, und der nächste Lauf hängt an.
- **62 Commits seit dem beschränkten Abschluss der Runde 23**, davon 38 an `crates/`,
  `xtask/`, `resources/`, `Cargo.toml` oder `Makefile`; die Version steht auf 1.8.0. Nach
  der Herkunftsregel liegt diese Arbeit im gemeinsamen Speicher, und das ist richtig; ohne
  Runde fehlt ihr trotzdem der Abnahmerahmen.
- **Keine Durchsicht deckt 42 dieser Commits.** `bin/fusion-review-coverage` meldet gegen den
  Anker `workbench-root` seit `28c4a47` das Urteil `uncovered`, mit `reviews=0` und
  `uncovered=42`.
- **108 offene Defekte**, 66 im gemeinsamen Speicher und 42 verteilt über die Runden. Dazu
  12 offene und 22 beantwortete, noch nicht umgesetzte Nutzerfragen. Aufräumarbeit, keine
  Vorbedingung für die nächste Runde.
- **Fünf Verweise zeigen ins Leere**, seit der Abgleich vom 260909-1021 vierzehn
  Anforderungsdokumente und Pläne umbenannt hat; abgelegt als
  `260909-1030_*_fuenf-verweise-zeigen-nach-dem-zug-der-plandokumente-ins-leere.md` im
  gemeinsamen Defektspeicher.
- Der Datensatz der Runde 17, im Verzeichnis
  `260825-0711-kontextmenue-traegt-zip-unzip-finder`, trägt keinen Abschnitt
  `## Closure note` und ein leeres Turn-Protokoll. Unverändert seit dem Lauf 260827-0403.
- Vier terminale Datensätze tragen ein leeres Turn-Protokoll: die Runden 23, 17, 16 und 14.
  Die zwei zurückgestellten tragen statt Einträgen eine Platzhalterzeile.
- Der neue Datensatz endet nach `## Turn log` und führt den Abschnitt `## Closure note`
  nicht, den die Vorlage vorsieht. Folgenlos bis zum Abschluss, dann zu ergänzen.
- **`CLAUDE.md` widerspricht dem Bestand.** Unter „Bindende Grundlage" steht, `ls
  fusion-workbench/circles/*/_a_circle.md` gebe seit dem 260821-2202 nichts aus. Seit heute
  gibt es einen Treffer. Nichts für diesen Lauf: die Datei gehört dem Kurator.
- Kein Zeigerbefund: `.active-circle` fehlt, kein Datensatz ist aktiv, und das ist der
  reguläre Zustand.
- Kein Abhängigkeitszyklus. Der eine nicht-terminale Circle zeigt allein auf terminale, also
  hat der Graph keine Kante zurück.
