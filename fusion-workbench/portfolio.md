# Portfolio

**Generated:** 260827-2101 (by playmaker session 260827-2101-playmaker-user-fusion-next)
**Domain bias:** code

Bestand: 1 vorgesehen, 0 aktiv, 6 kohärent geschlossen, 12 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 21 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach dem Abschluss der Runde 19 am 260827-1920.

## Anticipated (_a_) — ranked

Recommended next: 260827-2028-vorschau-rendert-pdf-als-betrachter — der einzige vorgesehene Circle, alle fünf Abhängigkeiten terminal, zwei offene Fragen in der Grundlage, von denen keine einen Planschritt aufhält.

**1. `260827-2028-vorschau-rendert-pdf-als-betrachter`** — die Vorschau rendert
PDF als Betrachter mit Zoom, Seitensprung und Seitenzähler; Text auf der Seite
lässt sich markieren und über die eine Zwischenablage-Hülle kopieren, die
Größengrenze ist `BILDGRENZE` von 64 MB. Der Shaper hat den Circle am
260827-2028 aus dem Ablageeintrag
`shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md` erzeugt und
den Gegenstand dabei auf PDF verengt, weil JPG und PNG seit der Runde 1
gerendert werden. Die Grundlage nennt zwei offene Entscheidungsdatensätze:
`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/decisions/260827-2028_*_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md`
ist die Frage nach der Tastenbelegung des Betrachters und gehört in den Spec;
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`
bindet jede Runde an der Vorschau, seit die Messstrecke ihre Arbeit nicht mehr
sieht. Zwei beantwortete Datensätze vom 260819-2216 (Quelltextzusage beim
Ziehen einer Auswahl, Abnahmelauf gegen L7) gelten weiter. Abhängigkeiten: fünf,
alle terminal. `260819-2230-auswahl-und-kopieren-in-der-vorschau` ist kohärent
geschlossen; `260802-0842-krk-mac-dateimanager-editor-git`,
`260811-1304-statusleiste-mit-bereichsschaltern`,
`260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` und
`260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` sind
beschränkt geschlossen. Die Heuristik "alle Abhängigkeiten kohärent" ist damit
formal nicht erfüllt, und das trägt hier nichts: in diesem Projekt misst der
beschränkte Abschluss, dass der Nutzer den Abnahmelauf im Vordergrund nicht
gefahren hat, und nicht, dass Arbeit offen wäre (CLAUDE.md, Absatz zur
Rangheuristik). Drei offene Defekte an der Vorschau erbt der Circle und führt
sie selbst auf: `shared/issues/260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md`,
`shared/issues/260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md`
und `shared/issues/260826-1423_*_zwei-zaehlangaben-zu-inhalt-in-vorschaumodell-rs-sind-seit-der-runde-16-um-eins-falsch.md`;
der dritte wird mit dem neuen `Inhalt`-Wert erneut falsch und ist im Plan
nachzuziehen. Wir empfehlen die Aktivierung; der Vorschlag steht als
`## Activation proposal` am Datensatz
`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/_*_circle.md`.

## Backlog — ranked

Recommended to shape: (keiner) — beide lebenden Einträge sind gebaut, und für beide steht unten eine Schließung zur Bestätigung; ein gebauter Gegenstand wird nicht ausgearbeitet.

**1. `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`**
(`_o_`) — verlangt eine zweite Kombination neben `f4` für den Editor-Einstieg.
Gebaut: `editor_rundweg` auf `cmd+e` öffnet seit dem 260823 im Dateifenster
denselben ausgewählten Eintrag wie `f4` (`resources/default-keymap.toml`,
Kommentar bei `bearbeiten`, Zeilen 174 bis 177, Eintrag Zeile 803).
  `close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e (editor_rundweg) öffnet seit dem 260823 im Dateifenster denselben ausgewählten Eintrag wie f4`

**2. `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`**
(`_o_`) — verlangt Leseprofile in einer Definitionsdatei unter
`~/Library/Application Support/KRK/`. Gebaut von der Runde 16 als `readers.toml`
mit zwölf Auslieferungsprofilen (`resources/default-readers.toml`), seit der
Runde 19 dazu das eingebaute Default-Profil.
  `close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Runde 16 hat die Leseprofile als readers.toml gebaut, mit zwölf ausgelieferten Profilen für die Werkbank`

Der dritte Eintrag, `shared/backlog/260827-1925_*_vorschau-rendert-pdf-und-bilder.md`,
ist seit 260827-2028 geschlossen: der Shaper hat ihn zum Circle
`260827-2028-vorschau-rendert-pdf-als-betrachter` gemacht.

Die zwei Schließungen sind Vorschläge, zum dritten Mal nach den Läufen
260827-0403 und 260827-1927. Dieser Lauf hält für keine der beiden eine
Bestätigung; `/fusion:next` legt die zwei Zeilen zur Bestätigung vor, und der
zweite Lauf des Relais führt aus, was bestätigt ist. Umbenannt hat dieser Lauf
nichts: keinen der zwei Einträge empfehlen wir zum Ausarbeiten.

## Recently closed (_c_ / _b_)

1. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` (`_c_`,
   260827-1920) — die Vorschau zählt den Inhalt eines Ordners in einem
   eingebauten Default-Profil. Ein Turn, acht Planschritte, Abnahmelauf vom
   Nutzer gefahren, Abgleich `coherent`. Zwei Low-Befunde der Durchsicht bleiben
   als offene Defekte für eine Folgerunde.
2. `260825-0711-kontextmenue-traegt-zip-unzip-finder` (`_b_`, 260825-1422) —
   das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen. Der Datensatz
   trägt keine Schließungsnotiz; die Begründung steht in `git:2a77012`. Siehe
   `## Warnings`.
3. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (`_b_`,
   260824-1810) — `readers.toml` als siebte Ablagedatei, die Erkennung in zwei
   Durchgängen, die vier Bausteine mit ihrem Haushalt. Beschränkt, weil sieben
   Abnahmekriterien KRK im Vordergrund verlangen.
4. `260821-1644-veroeffentlichen-als-achte-station` (`_c_`, 260821-2110) —
   Veröffentlichen als achte Station der Auslieferungskette, mit gefahrenem
   Abnahmelauf des Nutzers über fünfzehn Kriterien.
5. `260819-2230-auswahl-und-kopieren-in-der-vorschau` (`_c_`, 260820-1045) —
   die Vorschaufläche wird auswählbar, kopiert wird der Quelltext.

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
- Die Runde 19 lässt zwei Low-Befunde der Durchsicht als offene Defekte zurück:
  `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/issues/260827-1911_*_drei-saetze-im-kommentarteil-der-auslieferungsfassung-beschreiben-den-stand-vor-der-runde-19.md`
  (Ontocoder) und
  `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/issues/260827-1911_*_erkennung-rs-sagt-none-heisse-die-heutige-metadatenanzeige-und-das-ist-seit-der-runde-19-der-rueckfallzweig.md`
  (Coder). Beide sind Aufräumarbeit und keine Vorbedingung; der zweite liegt
  im Rückfallzweig der Vorschau, den der PDF-Betrachter für zu große und
  unlesbare Dateien mitbenutzt.
- Der vorgesehene Circle erbt drei offene Defekte an der Vorschau (siehe seinen
  Eintrag unter `## Anticipated`). Sie sind Gegenstand seines Plans und halten
  die Aktivierung nicht auf.
- Kein Zeigerfehler: `.active-circle` fehlt, und kein Datensatz ist aktiv.
- Kein Abhängigkeitszyklus: der eine nicht-terminale Circle hängt allein an
  terminalen Circles.
- Keine veraltete Grundlage: seit dem letzten Lauf hat kein Circle nach `_b_`
  gewechselt, und die vier beschränkt geschlossenen Runden in der Grundlage des
  neuen Circles waren beim Schreiben der Grundlage am 260827-2028 längst
  geschlossen.
