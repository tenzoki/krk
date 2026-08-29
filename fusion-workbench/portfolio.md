# Portfolio

**Generated:** 260829-0738 (by playmaker session 260829-0738-playmaker-orchestrator-phase4)
**Domain bias:** code

Bestand: 1 vorgesehen, 0 aktiv, 8 kohärent geschlossen, 12 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 23 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach dem Abschluss der Runde 22 am 260829-0737 (Commit `d523d1e`).

## Anticipated (_a_) — ranked

Recommended next: 260828-1041-dateilistenfilter-nimmt-eingaben-per-paste — der einzige vorgesehene Circle, drei Abhängigkeiten terminal, zwei offene Fragen in der Grundlage, von denen keine einen Planschritt aufhält; die Runde 22 hat die Ablageseite geliefert, auf die seine offene Frage wartet.

**1. `260828-1041-dateilistenfilter-nimmt-eingaben-per-paste`** (Runde 21) —
`cmd+v` im Dateifenster hängt den Inhalt der Zwischenablage an den Filtertext
des sichtbaren Tabs an, bei einem Pfad oder Dateiverweis allein den Dateinamen,
und besetzt damit den reservierten Einhängepunkt `text_einfuegen` ohne eine
Dateizwischenablage zu bauen. Der Shaper hat den Circle am 260828-1041 aus dem
Ablageeintrag
`shared/backlog/260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md`
erzeugt. Die Grundlage nennt zwei offene Entscheidungsdatensätze:
`circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/decisions/260828-1041_*_was-tut-cmd-v-mit-einem-dateiverweis-sobald-die-dateizwischenablage-gebaut-ist.md`
fragt nach dem Verhalten bei einem Dateiverweis, sobald eine Dateizwischenablage
existiert, und gehört in den Spec;
`shared/decisions/260826-0859_*_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md`
bindet die Schwelle, die die eingefügten Zeichen mitzählt. Zwei umgesetzte
Datensätze (Einhängepunkt-Reservierung der Runde 1, Filtertext übersteht den
Ordnerwechsel, Runde 10) gelten weiter. Abhängigkeiten: drei, alle terminal und
alle beschränkt geschlossen:
`260814-1551-tippen-filtert-dateiliste-flach-und-tief`,
`260816-1321-inhaltsfilter-mit-ankreuzfeld-content` und
`260802-0842-krk-mac-dateimanager-editor-git`. Die Heuristik „alle
Abhängigkeiten kohärent" ist damit formal nicht erfüllt, und das trägt hier
nichts: in diesem Projekt misst der beschränkte Abschluss, dass der Nutzer den
Abnahmelauf im Vordergrund nicht gefahren hat, und nicht, dass Arbeit offen wäre
(CLAUDE.md, Absatz zur Rangheuristik). Seit dem letzten Lauf hat die Runde 22
(`260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab`) `copy:` und `cut:` am
Dateifenster beantwortet und der Hülle um `NSPasteboard` einen Ausgang für
Dateiverweise gegeben; zwei Sätze der Grundlage beschreiben den Baum davor,
siehe `## Warnings`. Der Spec liest die Grundlage deshalb gegen den Baum nach,
bevor er die Abgrenzung übernimmt. Wir empfehlen die Aktivierung; der Vorschlag
steht als zweiter `## Activation proposal` am Datensatz
`circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/_*_circle.md`.

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

Drei weitere Einträge sind geschlossen, weil der Shaper sie zu Circles gemacht
hat: `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` (Runde
20), `shared/backlog/260828-0909_*_dateilistenfilter-nimmt-eingaben-per-paste.md`
(der vorgesehene Circle oben) und
`shared/backlog/260828-2345_*_cmd-c-und-cmd-x-kopieren-dateien-fuer-andere-apps.md`
(Runde 22, seit dem 260829 kohärent geschlossen).

Die zwei Schließungen sind Vorschläge, zum fünften Mal nach den Läufen
260827-0403, 260827-1927, 260827-2101 und 260828-1053. Dieser Lauf ist ein
Phase-4-Dispatch ohne Nutzer im Gespräch und hält für keine der beiden eine
Bestätigung; der nächste `/fusion:next` legt die zwei Zeilen zur Bestätigung
vor. Umbenannt hat dieser Lauf nichts: keinen der zwei Einträge empfehlen wir
zum Ausarbeiten.

## Recently closed (_c_ / _b_)

1. `260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab` (`_c_`, 260829-0737) —
   `cmd+c` und `cmd+x` im Dateifenster legen die betroffenen Einträge als
   Dateiverweise auf die Zwischenablage, für den Finder und andere Anwendungen;
   `cmd+x` meldet, dass das Verschieben beim Ziel liegt. Autonome Runde ohne
   Tore, ein Turn mit neun Schritten, Commits `4bd0084..35b95b3`; Abnahmelauf
   vom Nutzer am Bündel auf `023ee64` gefahren, alle zehn Punkte bestätigt,
   Abgleich `coherent`. Vier offene Datensätze unter `issues/` bleiben zurück,
   siehe `## Warnings`. Auslieferung 1.3.0 folgt auf Wunsch des Nutzers.
2. `260827-2028-vorschau-rendert-pdf-als-betrachter` (`_c_`, 260828-1055) —
   die Vorschau rendert PDF als Betrachter mit Zoom, Seitensprung und
   Seitenzähler. Ein Turn, elf Planschritte, Commits `2033626..48cd818`;
   Abnahmelauf vom Nutzer in zwei Läufen gefahren, Abgleich `coherent`. Sieben
   offene Datensätze unter `issues/` bleiben zurück, siehe `## Warnings`.
3. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`,
   260827-1920) — die Vorschau zählt den Inhalt eines Ordners in einem
   eingebauten Default-Profil. Ein Turn, acht Planschritte, Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`. Drei Befunde bleiben als offene Defekte
   für eine Folgerunde.
4. `260825-0711-kontextmenue-traegt-zip-unzip-finder` (`_b_`, 260825-1422) —
   das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen. Der Datensatz
   trägt keine Schließungsnotiz; die Begründung steht in `git:2a77012`. Siehe
   `## Warnings`.
5. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (`_b_`,
   260824-1810) — `readers.toml` als siebte Ablagedatei, die Erkennung in zwei
   Durchgängen, die vier Bausteine mit ihrem Haushalt. Beschränkt, weil sieben
   Abnahmekriterien KRK im Vordergrund verlangen.

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

- Die Grundlage des vorgesehenen Circles
  `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/_*_circle.md`
  beschreibt in zwei Sätzen den Baum vor der Runde 22: „`copy:` bleibt dort
  unbeantwortet" und „kein Dateiverweis wird abgelegt". Seit `1644ada` und
  `3764fb6` beantwortet das Dateifenster `copy:` und `cut:`, und die Hülle um
  `NSPasteboard` schreibt Dateiverweise. Die Abgrenzung der Runde 21 (kein
  Einfügen von Dateien, `paste:` für den Filter) bleibt davon unberührt; der
  Spec übernimmt die Sätze nicht ungelesen. Kein Abschnitt `## Parent grounding
  stale`, weil der Abschluss kohärent (`_c_`) ist und die Propagation allein
  einem beschränkten Abschluss gilt.
- Die Runde 22 lässt vier offene Datensätze unter
  `circles/260828-2349-cmd-c-und-cmd-x-legen-dateiverweise-ab/issues/` zurück:
  zwei Low-Befunde der Durchsicht (`issues/260829-0051_*_…` `must_use` an den
  Geschwistern, `issues/260829-0052_*_…` Abweisungsmeldung), die Probenablagen
  der Hülle bei parallelen Testläufen (`issues/260829-0041_*_…`) und drei
  Spec-Aussagen gegen den Baum, eine davon überholt (`issues/260829-0006_*_…`).
  Aufräumarbeit, keine Vorbedingung.
- Vier Punkte für den Kurator an CLAUDE.md, aus den Schließungsnotizen der
  Runden 20 und 22: die Hülle um `NSPasteboard` schreibt seit der Runde 22 auch
  Dateiverweise und steht dort nur als Textschreiber; die Rundentabelle endet
  bei 18; `copy:` und `cut:` sind neben dem Kontextmenü ein weiterer Weg ohne
  Taste; `Wirkungsbereich` trägt acht Werte, nicht sieben
  (`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/260828-1046_*_claude-md-nennt-sieben-werte-fuer-wirkungsbereich-der-baum-traegt-acht.md`).
- Die Runde 20 lässt sieben offene Datensätze unter
  `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/` zurück,
  unverändert seit dem Lauf 260828-1053: der Kurator-Befund darüber, drei
  Low-Befunde der Durchsicht, die Frage an C6 der Runde 1 (Schließung gehört dem
  Nutzer), `make tasten` im Spec und fünf History-Dateien mit Zeitstempeln nach
  ihrem eigenen Commit.
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
- Der Datensatz des vorgesehenen Circles trägt keinen Abschnitt `## Closure
  note`; die Vorlage in `rules/circle-records.md` sieht ihn vor. Der
  Orchestrator legt ihn beim Abschluss an, die Aktivierung hält es nicht auf.
- Zwei Ablageeinträge sind gebaut und stehen offen, weil außer der Promotion
  durch den Shaper kein Weg einen Eintrag ohne Bestätigung schließt. Die zwei
  Schließungen stehen in `## Backlog — ranked` als Zeilen zur Bestätigung.
- Die Runde 19 lässt drei offene Defekte unter
  `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/issues/`
  zurück, unverändert. Aufräumarbeit, keine Vorbedingung.
- Kein Zeigerfehler: `.active-circle` fehlt, und kein Datensatz ist aktiv.
- Kein Abhängigkeitszyklus: der eine nicht-terminale Circle hängt allein an
  terminalen Circles.
- Keine veraltete Grundlage im Sinne der Propagation: der Abschluss dieses
  Laufs ist kohärent (`_c_`), und seit dem letzten Lauf hat kein Circle nach
  `_b_` gewechselt.
