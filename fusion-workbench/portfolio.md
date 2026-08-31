# Portfolio

**Generated:** 260831-2211 (by playmaker session 260831-2211-playmaker-direct-dispatch)
**Domain bias:** code

Bestand: 0 vorgesehen, 0 aktiv, 9 kohärent geschlossen, 13 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 24 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach dem beschränkten Abschluss der Runde 23 am 260831-2024 (Commit
`b3e2688`).

## Anticipated (_a_) — ranked

(none)

Kein Circle ist vorgesehen, also hat diese Rangfolge nichts zu ordnen und dieser
Lauf keinen Aktivierungsvorschlag geschrieben.

**Was als Nächstes ansteht, ist angekündigt und noch nicht festgehalten.** Der
Nutzer hat in der Sitzung vom 260831 eine Directive genannt: KRK soll melden,
dass seine Auslieferungsfassung von `readers.toml` und `settings.toml`
weitergegangen ist, und den Unterschied zeigen. Der Anlass ist gemessen und steht
in `shared/history/260831-1353-ontocoder-leseprofile-archive-und-shared.md`: die
Auslieferungsfassung führt heute zwölf Leseprofile, die Nutzerdatei nach Auskunft
des Nutzers fünf, und nichts im laufenden Programm sagt ihm das. Ein Circle ist
daraus nicht angelegt, weder von diesem Lauf noch sonst; `/fusion:direct
<Entwurf>` fasst ihn, sobald der Nutzer es will.

Daneben liegt die offene Frage
`shared/decisions/260831-1353_*_bekommt-das-ablageprofil-eine-zweite-umfangszeile-obwohl-die-dateizahl-nicht-zaehlbar-ist.md`
aus derselben Arbeit. Sie berührt dieselbe Datei und hält nichts auf.

## Backlog — ranked

Recommended to shape: (keiner) — beide lebenden Einträge sind gebaut, und für
beide steht unten eine Schließung zur Bestätigung; ein gebauter Gegenstand wird
nicht ausgearbeitet.

**1. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`) — verlangt eine zweite Kombination neben `f4` für den Editor-Einstieg.
Gebaut: `editor_rundweg` auf `cmd+e` öffnet seit dem 260823 im Dateifenster
denselben ausgewählten Eintrag wie `f4` (`resources/default-keymap.toml`,
Kommentar bei `bearbeiten`, Eintrag `editor_rundweg`).
  `close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e (editor_rundweg) oeffnet seit dem 260823 im Dateifenster denselben ausgewaehlten Eintrag wie f4`

**2. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`**
(`_o_`) — verlangt Leseprofile in einer Definitionsdatei unter
`~/Library/Application Support/KRK/`. Gebaut von der Runde 16 als `readers.toml`
mit den Auslieferungsprofilen in `resources/default-readers.toml`, seit der Runde
19 dazu das eingebaute Default-Profil; die Auslieferungsfassung führt am 260831
zwölf Profile, darunter die zwei für `archive/` und `shared/`
(`shared/history/260831-1353-ontocoder-leseprofile-archive-und-shared.md`).
  `close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Runde 16 hat die Leseprofile als readers.toml gebaut, die Auslieferungsfassung fuehrt heute zwoelf Profile`

Der angekündigte Gegenstand aus `## Anticipated` ist nicht derselbe wie dieser
zweite Eintrag und hebt seine Schließung nicht auf. Der Eintrag verlangt die
Profile; sie stehen. Die Directive verlangt eine Meldung über den Abstand
zwischen Auslieferungs- und Nutzerfassung, und darüber sagt der Eintrag nichts.

Vier weitere Einträge sind geschlossen, weil der Shaper sie zu Circles gemacht
hat: `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` (Runde 20),
`shared/backlog/260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md`
(Runde 21),
`shared/backlog/260828-2345_*_cmd-c-und-cmd-x-kopieren-dateien-fuer-andere-apps.md`
(Runde 22) und
`shared/backlog/260829-0842_*_dateilistenfilter-versteht-stern-als-platzhalter.md`
(als zweite Fähigkeit in den Spec der Runde 21 aufgenommen).

Die zwei Schließungen sind Vorschläge, zum siebten Mal nach den Läufen
260827-0403, 260827-1927, 260827-2101, 260828-1053, 260829-0738 und 260829-1227.
Dieser Lauf hält für keine der beiden eine Bestätigung, und er hat keinen Kanal,
den Nutzer selbst zu fragen; der nächste `/fusion:next` legt die zwei Zeilen vor.
Umbenannt hat dieser Lauf nichts: keinen der zwei Einträge empfehlen wir zum
Ausarbeiten.

## Recently closed (_c_ / _b_)

1. `260830-1045-git-bereich-liest-status-branch-verlauf` (`_b_`, 260831-2024) —
   der Git-Bereich der Stufe A steht: ein sechster Bereich der Fensterzeile mit
   Branch, Statuszusammenfassung, Verlaufsliste und den Einzelheiten des
   ausgewählten Commits, `Fokus::Git` als sechster Fokuswert auf `shift+cmd+b`,
   der Umschalter auf `opt+cmd+r`, eine fünfte Spalte mit fünf Markenzuständen in
   beiden Dateifenstern, gelesen mit `gix` 0.87.1 und ohne jeden Schreibweg.
   Beschränkt und nicht kohärent, weil der Abnahmelauf am laufenden Bündel nicht
   gefahren ist: 25 der 90 Abnahmekriterien verlangen KRK im Vordergrund und sind
   damit Nutzerarbeit. Sechzehn der siebzehn Planschritte stehen auf `[DONE]`,
   vierzehn der fünfzehn Endbedingungen halten; der offene Schritt und die offene
   Endbedingung sind derselbe Lauf. Die Durchsicht ist gefahren
   (`reviews/260831-1444-coderev-git-bereich-runde-23.md`, 50 von 51 Dateien
   geöffnet), hat dreizehn Defekte gefunden, und alle dreizehn sind behoben.
   Zwei Auslieferungen sind aus der Runde hervorgegangen, 1.5.0 und 1.6.0. Elf
   offene Defekte und drei offene Entscheidungen bleiben zurück, siehe
   `## Warnings`.
2. `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` (`_c_`, 260829-1226) —
   `cmd+v` im Dateifenster hängt den Ablageinhalt an den Filtertext an, und der
   Filter versteht `*` als Platzhalter. Ein Turn mit zwölf Schritten, Commits
   `79d507a..8d64859`; Abnahmelauf vom Nutzer am Bündel auf `415ef6f` gefahren,
   alle zwölf Punkte bestätigt, Abgleich `coherent`. Ausgeliefert als 1.4.0.
3. `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab` (`_c_`, 260829-0737) —
   `cmd+c` und `cmd+x` im Dateifenster legen die betroffenen Einträge als
   Dateiverweise auf die Zwischenablage, für den Finder und andere Anwendungen.
   Ein Turn mit neun Schritten, Commits `4bd0084..35b95b3`; Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`. Ausgeliefert als 1.3.0.
4. `260827-2028-vorschau-rendert-pdf-als-betrachter` (`_c_`, 260828-1055) — die
   Vorschau rendert PDF als Betrachter mit Zoom, Seitensprung und Seitenzähler.
   Ein Turn, elf Planschritte, Commits `2033626..48cd818`; Abnahmelauf vom Nutzer
   in zwei Läufen gefahren, Abgleich `coherent`.
5. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`,
   260827-1920) — die Vorschau zählt den Inhalt eines Ordners in einem
   eingebauten Default-Profil. Ein Turn, acht Planschritte, Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`, 260821-2202)
  — KRK zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt, nicht verschoben:
  der Nutzer hat das Abgeben an den Systembrowser gewählt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
  Das Vokabular kennt für eine Absage keinen eigenen Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK
  setzt Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier
  heißt der Marker „später": die Runde war aktiv und ist der Löschabsicherung
  gewichen.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- Die Runde 23 lässt elf offene Defekte und drei offene Entscheidungen unter
  `circles/260830-1045-git-bereich-liest-status-branch-verlauf/` zurück. Die
  Schließungsnotiz sagt, keiner halte etwas auf, und zwei bestanden schon vor der
  Runde. Zwei der elf betreffen den Code: `issues/260831-0855_*_…` (der Zweig für
  `NeedsUpdate` in `posten_deuten` ist unerreichbar, `gix` fängt den Posten
  vorher ab) und `issues/260831-1652_*_…` (`gix` zieht ein unlesbares
  `.git`-Verzeichnis und einen toten `gitdir:`-Verweis selbst zu „kein
  Repository" zusammen). Der Rest sind Prosa- und Spec-Widersprüche, darunter
  vier gegen den Spec der Runde selbst.
- **Einer dieser elf ist überholt.**
  `circles/260830-1045-git-bereich-liest-status-branch-verlauf/issues/260831-1417_*_die-runde-23-schliesst-ohne-durchsicht-und-vierundzwanzig-commits-sind-ungedeckt.md`
  steht offen und trifft nicht mehr zu: die Durchsicht ist am 260831-1444 gefahren
  und deckt `d1fbaac..0a25ee0`. Der Datensatz gehört auf `_c_` mit einer Zeile
  darüber, und diesen Schritt fährt kein Playmaker; er gehört dem Reconciler oder
  dem Nutzer.
- **Zwölf Commits sind am HEAD ungedeckt**, gemessen mit
  `bin/fusion-review-coverage` gegen den Anker `workbench-root` seit `d1fbaac`.
  Darunter ist ein Codecommit ohne Durchsicht, `206718f` (die verschiebbare
  Grenze im Git-Bereich), dazu die zwei Auslieferungscommits für 1.5.0 und 1.6.0.
  Die Schließungsnotiz nennt fünf ungedeckte Commits; sie stimmt für den Stand des
  Abschlusses, und die sieben weiteren sind danach dazugekommen.
- **Arbeit nach dem Abschluss steht ohne Circle da.** Commit `206718f` und die
  Aufzeichnung `shared/history/260831-2141-coder-verschiebbare-grenze-im-git-bereich.md`
  bauen die Fläche des Git-Bereichs um: eine waagerechte `NSSplitView` teilt sie ab
  Werk hälftig, die Trennlinie lässt sich ziehen, und der Anteil übersteht das
  Beenden in `session.toml`. Nach der Herkunftsregel liegt das im gemeinsamen
  Speicher, und das ist richtig. `CLAUDE.md` sagt darüber nichts, und der
  Abnahmelauf für diese Anzeige ist ebenfalls Nutzerarbeit. Für den Kurator.
- Der Datensatz der Runde 23 trägt ein leeres Turn-Protokoll. Drei weitere
  terminale Datensätze tragen ebenfalls keines:
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`,
  `260819-2230-auswahl-und-kopieren-in-der-vorschau` und, mit einer
  Platzhalterzeile statt Einträgen, die zwei zurückgestellten Runden.
- Der Datensatz der Runde 17,
  `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_*_circle.md`, trägt
  keinen Abschnitt `## Closure note` und ein leeres Turn-Protokoll. Der Abschluss
  am 260825 war eine reine Umbenennung (`git:2a77012`); was die Runde erreicht hat
  und warum sie beschränkt geschlossen ist, steht allein in der Commit-Nachricht.
  Unverändert seit dem Lauf 260827-0403.
- Die Runden 19 bis 22 lassen zusammen ihre offenen Datensätze zurück,
  unverändert seit dem Lauf 260829-1227: fünf unter der Runde 21 (darunter die
  Nutzerfrage nach einer Höchstlänge des eingefügten Filtertexts und die
  Entscheidung zur Dateizwischenablage), vier unter der Runde 22, sieben unter der
  Runde 20 und drei unter der Runde 19. Aufräumarbeit, keine Vorbedingung.
- Zwei Ablageeinträge sind gebaut und stehen offen, weil außer der Promotion durch
  den Shaper kein Weg einen Eintrag ohne Bestätigung schließt. Die zwei
  Schließungen stehen in `## Backlog — ranked` als Zeilen zur Bestätigung.
- Kein Zeigerfehler: `.active-circle` fehlt, und kein Datensatz ist aktiv.
- Kein Abhängigkeitszyklus: es gibt keinen nicht-terminalen Circle.
- Keine veraltete Grundlage im Sinne der Propagation: die Runde 23 ist nach `_b_`
  gewechselt, und kein vorgesehener oder aktiver Circle zitiert sie, weil es
  keinen gibt.
