# Portfolio

**Generated:** 260827-0403 (by playmaker session 260827-0403-playmaker-direct-dispatch)
**Domain bias:** code

Bestand: 1 vorgesehen, 0 aktiv, 5 kohärent geschlossen, 12 beschränkt
geschlossen, 0 überholt, 2 zurückgestellt. Summe 20 Circle-Datensätze. Die Runde
18 ist ohne Circle-Datensatz gefahren und in keiner dieser Zahlen enthalten.

## Active (_t_)

(keiner)

`.active-circle` fehlt, und kein Datensatz trägt den Aktiv-Marker. Der reguläre
Zustand nach einem Rundenabschluss.

## Anticipated (_a_) — ranked

Recommended next: `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil` — der einzige vorgesehene Circle, seine Grundlage steht vollständig auf der Platte, und seine zwei offenen Fragen sind vom Nutzer in einem Zug zu beantworten.

**1. `260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil`**
Directive: Die Vorschau zählt den Inhalt eines Ordners in einem eingebauten
Default-Profil. Abhängigkeiten: zwei genannte Vorläufer, beide beschränkt
geschlossen, beide vollständig gebaut.

Die Runde setzt an einer Stelle an, die gebaut und als Nutzerwille festgehalten
ist: greift kein Profil aus `readers.toml`, bleibt es heute bei der
Metadatenanzeige (`crates/krk-core/src/leseprofil/erkennung.rs`). Dort tritt das
eingebaute Default-Profil hinzu und hängt drei Zählzeilen unter die sechs
Metadatenangaben. Die Bausteine dafür liegen bereit: `Typ` am `Eintrag` trägt
genau die drei Werte Ordner, Datei und Verknüpfung, `Eintrag::versteckt` trägt
das Kennzeichen für die Klammerzahlen, und `verzeichnis::leser::lesen_hoechstens`
liest einen fremden Ordner einmal, ohne das angezeigte Ordnermodell anzufassen.
Vier offene Entscheidungen binden die Runde, und keine hält sie auf. Zwei hat
der Shaper bei der Anlage gestellt und legt sie dem Nutzer zur Aktivierung vor:
ob die Zählung nach Typ und versteckt eine allgemeine Fähigkeit der Profile wird
(`decisions/260827-0311_*_bekommen-die-profile-aus-readers-toml-die-zaehlung-nach-typ-und-versteckt.md`)
und was die Zählzeilen für einen Ordner über der Eintragsschranke von
zweitausend sagen
(`decisions/260827-0311_*_was-sagen-die-zaehlzeilen-fuer-einen-ordner-ueber-der-eintragsschranke.md`).
Die zwei anderen kommen von außerhalb und binden die Runde, ohne sie zu
blockieren:
`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`
und
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`.
Beide Vorläufer sind beschränkt geschlossen und nicht kohärent. Für diese Runde
ist das kein Mangel: geprüft sind die Schließungsnotizen, und beide Vorläufer
haben ihre Planschritte vollständig belegt und sind allein am nicht gefahrenen
Abnahmelauf beschränkt geblieben, den kein Agent fahren kann.

## Backlog — ranked

Recommended to shape: (keiner)

Die Ablage trägt zwei offene Einträge, und beide sind gebaut. Ausgearbeitet
werden soll keiner; vorgeschlagen ist für beide die Schließung. Ausgeführt ist
in der Ablage nichts: eine Schließung ist bestätigungspflichtig, und dieser Lauf
hält für keine der beiden eine Bestätigung.

**Vorgeschlagen, nicht ausgeführt:**

`close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md — cmd+e öffnet seit dem 260823 im Dateifenster denselben ausgewählten Eintrag wie f4`

`close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md — die Runde 16 hat die Leseprofile als readers.toml gebaut, mit zwölf ausgelieferten Profilen für die Werkbank`

**Die zwei Einträge im Einzelnen:**

- `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  — verlangt eine zweite, besser erreichbare Kombination neben `f4` für den
  Editor-Einstieg. Erfüllt: `resources/default-keymap.toml` trägt den Eintrag
  `editor_rundweg` auf `cmd+e`, und der Kommentar bei `bearbeiten` hält
  ausdrücklich fest, dass beide seit dem 260823 im Dateifenster denselben
  ausgewählten Eintrag öffnen und durch denselben Rumpf laufen. Die
  Vermutung des Eintrags zur Ursache, die Werksbelegung von `F4` auf
  Apple-Tastaturen, ist damit gegenstandslos für den Bedarf und weiterhin
  ungemessen.
- `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`
  — verlangt Leseprofile in einer Definitionsdatei unter
  `~/Library/Application Support/KRK/`, die je Ort eine Zusammenfassung
  festlegen, mit der fusion-Werkbank als Beispielfall. Gebaut hat das die Runde
  16 (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`),
  angelegt zweiunddreißig Minuten nach dem Eintrag. `resources/default-readers.toml`
  trägt heute zwölf Profile, darunter die Wurzel der Werkbank, ein Speicher, ein
  Defektspeicher, der Ablagespeicher, alle Runden und eine einzelne Runde. Die
  Skizze des Eintrags ist damit abgedeckt; der Dateiname weicht ab
  (`readers.toml` statt `krk-rc.yaml`), die Sache nicht.

## Recently closed (_c_ / _b_)

1. `260825-0711-kontextmenue-traegt-zip-unzip-finder` (`_b_`, 260825-1422) —
   das Kontextmenü trägt Zip, Unzip und Finder neben dem Teilen. Der Datensatz
   trägt keine Schließungsnotiz; die Begründung steht in `git:2a77012`. Siehe
   `## Warnings`.
2. `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` (`_b_`,
   260824-1810) — gebaut ist, was die Directive verlangt: `readers.toml` als
   siebte Ablagedatei, die Erkennung in zwei Durchgängen, die vier Bausteine mit
   ihrem Haushalt. Beschränkt, weil sieben Abnahmekriterien KRK im Vordergrund
   verlangen.
3. `260821-1644-veroeffentlichen-als-achte-station` (`_c_`, 260821-2110) —
   Veröffentlichen als achte Station der Auslieferungskette, mit gefahrenem
   Abnahmelauf des Nutzers über fünfzehn Kriterien.
4. `260819-2230-auswahl-und-kopieren-in-der-vorschau` (`_c_`, 260820-1045) —
   die Vorschaufläche wird auswählbar, kopiert wird der Quelltext.
5. `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` (`_c_`, 260819) —
   `opt+cmd+s` gleicht die Ordner der zwei Dateifenster an, fremde Anwendungen
   dürfen abwerfen. Abnahmelauf des Nutzers gefahren.

## Archived (_s_ / _d_)

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (`_d_`,
  260821-2202) — KRK zeigt Web-Seiten in einem eigenen Betrachter. Abgesagt, nicht
  verschoben: der Nutzer hat das Abgeben an den Systembrowser gewählt
  (`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`).
  Der Marker ist die nächstliegende Entsprechung und nicht die genaue; das
  Vokabular kennt für eine Absage keinen eigenen Marker.
- `260816-2255-befehle-absetzen-und-makros-speichern` (`_d_`, 260817-0445) — KRK
  setzt Befehle ab und führt gespeicherte Makros aus. Nichts ist gebaut. Hier
  heißt der Marker „später": die Runde war aktiv und ist der Löschabsicherung
  gewichen.

Überholte Runden (`_s_`) gibt es nicht.

## Warnings

- Der Datensatz der Runde 17,
  `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_b_circle.md`, trägt
  keinen Abschnitt `## Closure note` und ein leeres Turn-Protokoll. Der
  Abschluss am 260825 war eine reine Umbenennung von `_t_circle.md` nach
  `_b_circle.md` (`git:2a77012`, `similarity index 100%`), die den Rumpf nicht
  angefasst hat. Er ist der einzige der siebzehn terminalen Datensätze ohne
  Schließungsnotiz, und was diese Runde erreicht hat und warum sie beschränkt
  geschlossen ist, steht allein in der Commit-Nachricht.
- Drei weitere terminale Datensätze tragen ein leeres Turn-Protokoll:
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`,
  `260819-2230-auswahl-und-kopieren-in-der-vorschau` und, mit einer
  Platzhalterzeile statt Einträgen, die zwei zurückgestellten Runden.
- Ein Ablageeintrag kann gebaut werden, ohne dass ihn etwas schließt. Der Shaper
  schließt einen Eintrag, den er zu einer Runde macht; die Runde 16 ist auf einem
  anderen Weg entstanden, und ihr Eintrag steht seit dem 260823 offen da, obwohl
  die Sache seit dem 260824 gebaut ist. Beide heute offenen Einträge sind so
  entstanden.
- Kein Abhängigkeitszyklus. Kein Fall einer veralteten Grundlage: der einzige
  nicht-terminale Datensatz nennt zwei beschränkt geschlossene Vorläufer, aber
  seine Grundlage ist nach beiden Abschlüssen geschrieben und führt deren
  Ergebnisse.
