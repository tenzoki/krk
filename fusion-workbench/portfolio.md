# Portfolio

**Generated:** 260829-1227 (by playmaker session 260829-1227-playmaker-orchestrator-phase4)
**Domain bias:** code

Bestand: 0 vorgesehen, 0 aktiv, 9 kohärent geschlossen, 12 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 23 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach dem Abschluss der Runde 21 am 260829-1226 (Commit `439d66f`).

## Anticipated (_a_) — ranked

(none)

Kein Circle ist vorgesehen; der letzte, die Runde 21, ist soeben kohärent
geschlossen. Was als Nächstes kommt, ist eine Frage an den Nutzer und nicht an
diese Rangfolge: die Ablage unten trägt keine unausgearbeitete Idee, und die
offenen Datensätze unter `## Warnings` sind Aufräumarbeit, aus der sich keine
Runde von selbst ergibt. Wer eine neue Idee hat, legt sie mit `/fusion:memo` in
die Ablage oder fasst sie mit `/fusion:direct <Entwurf>` gleich als Circle.

## Backlog — ranked

Recommended to shape: (keiner) — beide lebenden Einträge sind gebaut, und für beide steht unten eine Schließung zur Bestätigung; ein gebauter Gegenstand wird nicht ausgearbeitet.

**1. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`) — verlangt eine zweite Kombination neben `f4` für den Editor-Einstieg.
Gebaut: `editor_rundweg` auf `cmd+e` öffnet seit dem 260823 im Dateifenster
denselben ausgewählten Eintrag wie `f4` (`resources/default-keymap.toml`,
Kommentar bei `bearbeiten`, Eintrag `editor_rundweg`).
  `close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e (editor_rundweg) öffnet seit dem 260823 im Dateifenster denselben ausgewählten Eintrag wie f4`

**2. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`**
(`_o_`) — verlangt Leseprofile in einer Definitionsdatei unter
`~/Library/Application Support/KRK/`. Gebaut von der Runde 16 als `readers.toml`
mit den Auslieferungsprofilen in `resources/default-readers.toml`, seit der
Runde 19 dazu das eingebaute Default-Profil.
  `close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Runde 16 hat die Leseprofile als readers.toml gebaut, mit ausgelieferten Profilen für die Werkbank`

Vier weitere Einträge sind geschlossen, weil der Shaper sie zu Circles gemacht
hat: `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` (Runde
20), `shared/backlog/260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md`
(Runde 21), `shared/backlog/260828-2345_*_cmd-c-und-cmd-x-kopieren-dateien-fuer-andere-apps.md`
(Runde 22) und, neu seit dem letzten Lauf,
`shared/backlog/260829-0842_*_dateilistenfilter-versteht-stern-als-platzhalter.md`,
das der Shaper am 260829-1052 als zweite Fähigkeit in den Spec der Runde 21
aufgenommen hat und das mit ihr gebaut ist.

Die zwei Schließungen sind Vorschläge, zum sechsten Mal nach den Läufen
260827-0403, 260827-1927, 260827-2101, 260828-1053 und 260829-0738. Dieser Lauf
ist ein Phase-4-Dispatch ohne Nutzer im Gespräch und hält für keine der beiden
eine Bestätigung; der nächste `/fusion:next` legt die zwei Zeilen zur
Bestätigung vor. Umbenannt hat dieser Lauf nichts: keinen der zwei Einträge
empfehlen wir zum Ausarbeiten.

## Recently closed (_c_ / _b_)

1. `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste` (`_c_`, 260829-1226) —
   `cmd+v` im Dateifenster hängt den Ablageinhalt an den Filtertext an, bei
   Pfad und Dateiverweis allein den Dateinamen, und der Filter versteht `*` als
   Platzhalter für eine beliebige Zeichenfolge (die zweite Fähigkeit kam aus
   der Ablage in den Spec). Autonome Runde ohne Tore, ein Turn mit zwölf
   Schritten, Commits `79d507a..8d64859`; Abnahmelauf vom Nutzer am Bündel auf
   `415ef6f` gefahren, alle zwölf Punkte bestätigt, Abgleich `coherent`. Fünf
   offene Defekte und eine offene Entscheidung bleiben zurück, siehe
   `## Warnings`. Auslieferung 1.4.0 folgt auf Wunsch des Nutzers.
2. `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab` (`_c_`, 260829-0737) —
   `cmd+c` und `cmd+x` im Dateifenster legen die betroffenen Einträge als
   Dateiverweise auf die Zwischenablage, für den Finder und andere Anwendungen;
   `cmd+x` meldet, dass das Verschieben beim Ziel liegt. Ein Turn mit neun
   Schritten, Commits `4bd0084..35b95b3`; Abnahmelauf vom Nutzer gefahren,
   Abgleich `coherent`. Vier offene Datensätze unter `issues/` bleiben zurück.
   Ausgeliefert als 1.3.0.
3. `260827-2028-vorschau-rendert-pdf-als-betrachter` (`_c_`, 260828-1055) —
   die Vorschau rendert PDF als Betrachter mit Zoom, Seitensprung und
   Seitenzähler. Ein Turn, elf Planschritte, Commits `2033626..48cd818`;
   Abnahmelauf vom Nutzer in zwei Läufen gefahren, Abgleich `coherent`. Acht
   offene Datensätze bleiben zurück, siehe `## Warnings`.
4. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`,
   260827-1920) — die Vorschau zählt den Inhalt eines Ordners in einem
   eingebauten Default-Profil. Ein Turn, acht Planschritte, Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`. Vier offene Datensätze bleiben zurück.
5. `260825-0711-kontextmenue-traegt-zip-unzip-finder` (`_b_`, 260825-1422) —
   das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen. Der Datensatz
   trägt keine Schließungsnotiz; die Begründung steht in `git:2a77012`. Siehe
   `## Warnings`.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`,
  260821-2202) — KRK zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt, nicht
  verschoben: der Nutzer hat das Abgeben an den Systembrowser gewählt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
  Das Vokabular kennt für eine Absage keinen eigenen Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK
  setzt Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier
  heißt der Marker „später": die Runde war aktiv und ist der Löschabsicherung
  gewichen.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- Die Runde 21 lässt fünf offene Defekte und eine offene Entscheidung unter
  `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/` zurück.
  Zwei davon brauchen den Nutzer: `issues/260829-1215_*_…` fragt, ob der
  eingefügte Filtertext eine Höchstlänge bekommt, weil eine lange Zeile jeden
  Rückschritt zum Gang über den Bestand macht, und
  `decisions/260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
  wartet auf eine Dateizwischenablage, die keine Runde plant. Die übrigen sind
  Aufräumarbeit: `issues/260829-1201_*_…` (C6.6-Prosa), `issues/260829-1216_*_…`
  (`\r` allein gilt der Reinigung nicht als Zeilenende), `issues/260829-1217_*_…`
  (CLAUDE.md nennt die Zählprobe mit ihrem alten Namen und den Vergleich als
  Teilzeichenfolge; Kurator) und `issues/260829-1223_*_…` (die Abschlussklausel
  des Plans verlangt ein leeres `grep` nach `regex` in `Cargo.lock`, das seit
  `syntect` nie leer ist).
- Fünf Punkte für den Kurator an CLAUDE.md: die Hülle um `NSPasteboard`
  schreibt seit der Runde 22 auch Dateiverweise und steht dort nur als
  Textschreiber; die Rundentabelle endet bei 18; `copy:` und `cut:` sind neben
  dem Kontextmenü ein weiterer Weg ohne Taste; `Wirkungsbereich` trägt acht
  Werte, nicht sieben
  (`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/260828-1046_*_claude-md-nennt-sieben-werte-fuer-wirkungsbereich-der-baum-traegt-acht.md`);
  und seit der Runde 21 vergleicht der Filter nicht mehr wörtlich als
  Teilzeichenfolge (`circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/issues/260829-1217_*_…`).
- Die Runde 22 lässt vier offene Datensätze unter
  `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab/issues/` zurück,
  unverändert seit dem Lauf 260829-0738: zwei Low-Befunde der Durchsicht
  (`issues/260829-0051_*_…` `must_use` an den Geschwistern,
  `issues/260829-0052_*_…` Abweisungsmeldung), die Probenablagen der Hülle bei
  parallelen Testläufen (`issues/260829-0041_*_…`) und drei Spec-Aussagen gegen
  den Baum (`issues/260829-0006_*_…`). Aufräumarbeit, keine Vorbedingung.
- Die Runde 20 lässt acht offene Datensätze unter
  `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/` zurück,
  unverändert seit dem Lauf 260828-1053: der Kurator-Befund darüber, drei
  Low-Befunde der Durchsicht, die Frage an C6 der Runde 1 (Schließung gehört dem
  Nutzer), `make tasten` im Spec, fünf History-Dateien mit Zeitstempeln nach
  ihrem eigenen Commit und die offene Entscheidung zu `cmd+plus` auf einer
  US-Belegung (`decisions/260828-0712_*_…`).
- Die Runde 19 lässt drei offene Defekte und eine offene Entscheidung unter
  `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/`
  zurück, unverändert. Aufräumarbeit, keine Vorbedingung.
- Der Datensatz der Runde 17,
  `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_*_circle.md`, trägt
  keinen Abschnitt `## Closure note` und ein leeres Turn-Protokoll. Der
  Abschluss am 260825 war eine reine Umbenennung (`git:2a77012`); was die Runde
  erreicht hat und warum sie beschränkt geschlossen ist, steht allein in der
  Commit-Nachricht. Unverändert seit dem Lauf 260827-0403.
- Drei weitere terminale Datensätze tragen ein leeres Turn-Protokoll:
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`,
  `260819-2230-auswahl-und-kopieren-in-der-vorschau` und, mit einer
  Platzhalterzeile statt Einträgen, die zwei zurückgestellten Runden.
- Zwei Ablageeinträge sind gebaut und stehen offen, weil außer der Promotion
  durch den Shaper kein Weg einen Eintrag ohne Bestätigung schließt. Die zwei
  Schließungen stehen in `## Backlog — ranked` als Zeilen zur Bestätigung.
- Kein Zeigerfehler: `.active-circle` fehlt, und kein Datensatz ist aktiv.
- Kein Abhängigkeitszyklus: es gibt keinen nicht-terminalen Circle.
- Keine veraltete Grundlage im Sinne der Propagation: der Abschluss dieses
  Laufs ist kohärent (`_c_`), und seit dem letzten Lauf hat kein Circle nach
  `_b_` gewechselt.
