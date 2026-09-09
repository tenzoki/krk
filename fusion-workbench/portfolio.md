# Portfolio

**Generated:** 260909-2209 (by playmaker session 260909-2209-playmaker-user-fusion-next-confirmed)
**Domain bias:** code

Bestand: 0 vorgesehen, 0 aktiv, 9 kohärent geschlossen, 13 beschränkt geschlossen, 0
überholt, 2 zurückgestellt. Summe 24 Circle-Datensätze. Die Runde 18 ist ohne
Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

Dieser Lauf hat die zwei vom Nutzer bestätigten Schließungen im Ablagespeicher ausgeführt.
Die Abschnitte `## Active`, `## Anticipated`, `## Recently closed` und `## Archived` sind
aus dem Stand von 260909-1413 übernommen, gegen den die Bestätigung eingeholt wurde; neu
gelesen ist allein der Ablagespeicher.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Das ist der reguläre
Zustand nach dem beschränkten Abschluss der Runde 23 am 260831-2024, und er steht seither
unverändert.

## Anticipated (_a_) — ranked

(none)

Kein Circle ist vorgesehen. Die Rangfolge hat nichts zu ordnen, und dieser Lauf hat keinen
Aktivierungsvorschlag in einen Datensatz geschrieben.

**Der angekündigte Gegenstand ist seit neun Tagen angekündigt und weiter nicht
festgehalten.** Der Nutzer hat in der Sitzung vom 260831 eine Directive genannt: KRK soll
melden, dass seine Auslieferungsfassung von `readers.toml` und `settings.toml`
weitergegangen ist, und den Unterschied zeigen. Der Anlass steht gemessen in
`260831-1353-ontocoder-leseprofile-archive-und-shared.md` im Verlaufsspeicher, und der Code
bestätigt die Lücke bis heute: der Modulkopf von
`crates/krk-core/src/ablage/leseprofile.rs` schreibt aus, dass die Auslieferungsfassung
beim ersten Start wörtlich geschrieben und „danach von KRK nie wieder angefasst" wird. Ein
Circle ist daraus nicht angelegt. `/fusion:direct <Entwurf>` fasst ihn, sobald der Nutzer es
will.

Die Frage aus derselben Arbeit,
`260831-1353_*_bekommt-das-ablageprofil-eine-zweite-umfangszeile-obwohl-die-dateizahl-nicht-zaehlbar-ist.md`
im gemeinsamen Entscheidungsspeicher, ist inzwischen beantwortet und wartet auf die
Umsetzung.

## Backlog — ranked

Recommended to shape: (keiner) — der Ablagespeicher hält nach diesem Lauf keinen lebenden
Eintrag mehr.

Beide Einträge lagen im Ablagespeicher unter `shared` und sind mit diesem Lauf geschlossen.
Der Nutzer hat beide Schließungen über `/fusion:next` bestätigt; das ist der neunte Lauf,
der die zwei Zeilen betrifft, und der erste, der sie ausführt. Die acht davor (260827-0403,
260827-1927, 260827-2101, 260828-1053, 260829-0738, 260829-1227, 260831-2211 und
260909-1413) haben sie nur vorgelegt, weil kein Weg außer der Promotion durch den Shaper
einen Eintrag ohne Bestätigung schließt.

Performed this run:

```
close 260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e (editor_rundweg) oeffnet seit dem 260823 im Dateifenster denselben ausgewaehlten Eintrag wie f4
close 260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Leseprofile stehen als readers.toml, die Auslieferungsfassung fuehrt 13 Profile und deckt die Skizze des Eintrags
```

Jeder der zwei Einträge trägt seinen Grund jetzt als letzte Zeile in der eigenen Datei.

Vier weitere Einträge waren schon vorher geschlossen, weil der Shaper sie zu Circles gemacht
hat: `260827-1925_*_vorschau-rendert-pdf-und-bilder.md` (Runde 20),
`260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md` (Runde 21),
`260828-2345_*_cmd-c-und-cmd-x-kopieren-dateien-fuer-andere-apps.md` (Runde 22) und
`260829-0842_*_dateilistenfilter-versteht-stern-als-platzhalter.md`, das als zweite
Fähigkeit in den Spec der Runde 21 aufgenommen wurde. Mit den zwei Schließungen dieses Laufs
sind damit alle sechs je gefilterten Ideen erledigt.

Vorgeschlagen, nicht ausgeführt: keine. Der angekündigte Gegenstand aus `## Anticipated` ist
kein Ablageeintrag und wird über `/fusion:direct` gefasst, nicht über diesen Speicher.

## Recently closed (_c_ / _b_)

1. `260830-1045-git-bereich-liest-status-branch-verlauf` (`_b_`, 260831-2024) — der
   Git-Bereich der Stufe A steht: ein sechster Bereich der Fensterzeile mit Branch,
   Statuszusammenfassung, Verlaufsliste und den Einzelheiten des ausgewählten Commits,
   `Fokus::Git` als sechster Fokuswert auf `shift+cmd+b`, der Umschalter auf `opt+cmd+r`,
   eine fünfte Spalte mit fünf Markenzuständen in beiden Dateifenstern, gelesen mit `gix`
   0.87.1 und ohne jeden Schreibweg ins Repository. Beschränkt und nicht kohärent, weil der
   Abnahmelauf am laufenden Bündel Nutzerarbeit ist. Von den elf offenen Datensätzen, die
   die Runde damals zurückließ, stehen heute noch zwei offen.
2. `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` (`_c_`, 260829-1226) — `cmd+v`
   im Dateifenster hängt den Ablageinhalt an den Filtertext an, und der Filter versteht `*`
   als Platzhalter. Ein Turn mit zwölf Schritten, Commits `79d507a..8d64859`; der Nutzer hat
   den Abnahmelauf am Bündel auf `415ef6f` gefahren und alle zwölf Punkte bestätigt.
   Ausgeliefert als 1.4.0.
3. `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab` (`_c_`, 260829-0737) — `cmd+c` und
   `cmd+x` im Dateifenster legen die betroffenen Einträge als Dateiverweise auf die
   Zwischenablage, für den Finder und andere Anwendungen. Ein Turn mit neun Schritten,
   Commits `4bd0084..35b95b3`; Abnahmelauf vom Nutzer gefahren. Ausgeliefert als 1.3.0.
4. `260827-2028-vorschau-rendert-pdf-als-betrachter` (`_c_`, 260828-1055) — die Vorschau
   rendert PDF als Betrachter mit Zoom, Seitensprung und Seitenzähler. Ein Turn, elf
   Planschritte, Commits `2033626..48cd818`; der Nutzer hat den Abnahmelauf in zwei Läufen
   gefahren, der erste fand den Stapelüberlauf beim Zoom.
5. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`, 260827-1920) — die
   Vorschau zählt den Inhalt eines Ordners in einem eingebauten Default-Profil. Ein Turn,
   acht Planschritte, neun Commits `a5c7a46..d444879`; Abnahmelauf vom Nutzer gefahren.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`, 260821-2202) — KRK
  zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt und nicht verschoben: der Nutzer
  hat das Abgeben an den Systembrowser gewählt
  (`260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`
  im gemeinsamen Entscheidungsspeicher). Das Vokabular kennt für eine Absage keinen eigenen
  Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK setzt
  Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier heißt der Marker
  „später": die Runde war aktiv und ist der Löschabsicherung gewichen. Spec und Plan stehen
  als einzige Anforderungsdokumente des Bestands weiter auf offen, und das ist richtig, weil
  keine Bauarbeit dahintersteht.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- **Neun Tage Arbeit stehen ohne Runde da.** Seit dem beschränkten Abschluss der Runde 23
  sind 41 Commits gefallen, 27 davon an `crates/`, `xtask/`, `resources/`, `Cargo.toml` oder
  `Makefile`, und die Version ist in vier Auslieferungen von 1.5.0 auf 1.8.0 gestiegen.
  Darunter sind zwei Fähigkeiten am Programm, die kein Circle-Datensatz führt: `cmd+f` fügt
  in den Filtertext ein (`67c93d5`) und eine Zeichenschwelle gilt jetzt für Unterbaum und
  Inhalt zugleich (`26cbc93`). Nach der Herkunftsregel liegt die Arbeit im gemeinsamen
  Speicher, und das ist richtig; ohne Runde fehlt ihr trotzdem der Abnahmerahmen.
- **Keine Durchsicht deckt diese 41 Commits.** `bin/fusion-review-coverage` meldet gegen den
  Anker `workbench-root` seit `28c4a47` das Urteil `uncovered`, mit `reviews=0` und
  `uncovered=41`. Die letzte Durchsicht liegt am 260831-1444 und deckt `d1fbaac..0a25ee0`.
- **108 offene Defekte**, 66 im gemeinsamen Speicher und 42 verteilt über die Runden; 12
  davon sind seit dem 260901 dazugekommen. Dazu 12 offene und 22 beantwortete, noch nicht
  umgesetzte Nutzerfragen. Aufräumarbeit, keine Vorbedingung für eine nächste Runde.
- **Fünf Verweise zeigen ins Leere**, seit der Abgleich vom 260909-1021 vierzehn
  Anforderungsdokumente und Pläne umbenannt hat. Sie stehen außerhalb der vierzehn Dateien
  und nennen ihr Ziel mit ausgeschriebenem Marker; abgelegt als
  `260909-1030_*_fuenf-verweise-zeigen-nach-dem-zug-der-plandokumente-ins-leere.md` im
  gemeinsamen Defektspeicher.
- Der Datensatz der Runde 17, im Verzeichnis
  `260825-0711-kontextmenue-traegt-zip-unzip-finder`, trägt keinen Abschnitt
  `## Closure note` und ein leeres Turn-Protokoll. Der Abschluss am 260825 war
  eine reine Umbenennung; was die Runde erreicht hat und warum sie beschränkt geschlossen
  ist, steht allein in der Commit-Nachricht. Unverändert seit dem Lauf 260827-0403.
- Vier terminale Datensätze tragen ein leeres Turn-Protokoll: die Runden 23, 17, 16 und 14.
  Die zwei zurückgestellten tragen statt Einträgen eine Platzhalterzeile.
- Die Warnung über zwei gebaute und offen stehende Ablageeinträge ist mit diesem Lauf
  erledigt: der Nutzer hat beide Schließungen bestätigt, und der Speicher hält keinen
  lebenden Eintrag mehr.
- Kein Zeigerbefund: `.active-circle` fehlt, und kein Datensatz ist aktiv.
- Kein Abhängigkeitszyklus: es gibt keinen nicht-terminalen Circle.
- Keine veraltete Grundlage im Sinne der Propagation: dreizehn Runden stehen auf
  beschränktem Abschluss, und keine vorgesehene oder aktive Runde zitiert eine davon, weil
  es keine gibt.
