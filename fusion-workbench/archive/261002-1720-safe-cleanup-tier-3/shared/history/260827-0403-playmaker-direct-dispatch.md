# Playmaker-Lauf 260827-0403

**Status:** Complete
**Auslöser:** Direktaufruf durch den Nutzer
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Portfolio:** `fusion-workbench/portfolio.md`

## Bestand

Zwanzig Circle-Datensätze, gelesen als Marker am Dateinamen:

| Marker | Zahl | Bedeutung |
|---|---|---|
| `_a_` | 1 | vorgesehen |
| `_t_` | 0 | aktiv |
| `_c_` | 5 | kohärent geschlossen |
| `_b_` | 12 | beschränkt geschlossen |
| `_s_` | 0 | überholt |
| `_d_` | 2 | zurückgestellt |

`.active-circle` fehlt, und kein Datensatz trägt `_t_`. Das ist der reguläre
Zustand nach einem Rundenabschluss und keine Warnung.

## Rangfolge der vorgesehenen Runden

Ein Kandidat, und damit der Vorschlag:
`circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil`. Seine
Grundlage steht auf der Platte, seine zwei eigenen offenen Fragen hat der
Shaper bei der Anlage gestellt, und beide sind vom Nutzer in einem Zug zu
beantworten.

**Die Vorbedingungsprüfung ist gegen die Projektregel gefahren und nicht gegen
die Vorgabe.** Beide genannten Vorläufer tragen `_b_` und nicht `_c_`. Die
Standardheuristik zählte das als Mangel; `CLAUDE.md` hält ausdrücklich fest,
dass der Marker in diesem Projekt die Verfügbarkeit des Nutzers für den
Abnahmelauf misst und nicht die Reife der Runde. Gelesen sind deshalb die
Schließungsnotizen selbst: beide Vorläufer haben ihre Planschritte vollständig
belegt, und beide sind allein am nicht gefahrenen Abnahmelauf beschränkt
geblieben. Kein Abhängigkeitsvermerk.

**Offene Entscheidungen, die den Kandidaten binden:** vier.

- `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/decisions/260827-0311_*_bekommen-die-profile-aus-readers-toml-die-zaehlung-nach-typ-und-versteckt.md`
- `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/decisions/260827-0311_*_was-sagen-die-zaehlzeilen-fuer-einen-ordner-ueber-der-eintragsschranke.md`
- `shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`
- `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`

## Abhängigkeitszyklen

Keine. Der Graph über die nicht-terminalen Datensätze trägt einen Knoten, und
seine zwei Kanten laufen auf terminale Runden. Kein `## Dependency warning`
angehängt.

## Beschränkte Abschlüsse, die eine Grundlage veralten lassen

Keiner in diesem Lauf, und der Grund ist die Reihenfolge der Daten. Der einzige
nicht-terminale Datensatz nennt zwei beschränkt geschlossene Vorläufer, aber
seine Grundlage ist am 260827-0310 geschrieben worden, nach beiden Abschlüssen
(260807-1035 und 260824-1810), und sie führt die Ergebnisse dieser Abschlüsse
ausdrücklich. Ein Vermerk „Grundlage veraltet" wäre hier falsch. Kein
`## Parent grounding stale` angehängt.

## Ablage

Drei Einträge gelesen: zwei offene (`_o_`), einer geschlossen (`_c_`). Keine
Doppelung, kein Eintrag mit mehreren Ideen, nichts Defekt- oder Fragegestaltiges
darunter.

**Beide offenen Einträge sind gebaut, und keiner ist geschlossen worden.**

- `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  verlangt eine zweite, besser erreichbare Kombination neben `f4` für den
  Editor-Einstieg. Seit dem 260823 öffnet `cmd+e` im Dateifenster denselben
  ausgewählten Eintrag wie `f4` und läuft durch denselben Rumpf
  (`resources/default-keymap.toml`, Eintrag `editor_rundweg` und der Kommentar
  bei `bearbeiten`, Zeilen 174 bis 177). Der Eintrag ist am 260813-2033 gefüllt
  worden, zehn Tage davor.
- `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`
  verlangt Leseprofile in einer Definitionsdatei unter
  `~/Library/Application Support/KRK/`, die je Ort eine Zusammenfassung
  festlegen, mit der fusion-Werkbank als Beispielfall. Gebaut hat das die Runde
  16 als `readers.toml`
  (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`),
  angelegt zweiunddreißig Minuten nach dem Eintrag. `resources/default-readers.toml`
  trägt heute zwölf Profile, darunter die Wurzel der Werkbank, ein Speicher, ein
  Defektspeicher, der Ablagespeicher, alle Runden und eine einzelne Runde. Der
  Dateiname weicht ab, die Sache nicht.

**Geschrieben ist in der Ablage nichts.** Beide Schließungen sind
bestätigungspflichtig, und dieser Lauf hält für keine eine Bestätigung: der
Auftrag nennt keine bestätigten Vorgänge, und ein Kanal zum Nutzer steht diesem
Lauf nicht offen. Beide stehen als Vorschlag im Abschnitt `## Backlog — ranked`
des Portfolios.

**Keine Umbenennung zwischen `_o_` und `_p_`.** `_p_` hieße „zur Ausarbeitung
empfohlen"; empfohlen ist für beide Einträge die Schließung, nicht die
Ausarbeitung.

## Warnungen

- Der Datensatz der Runde 17
  (`circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/_b_circle.md`)
  trägt überhaupt keinen Abschnitt `## Closure note`, und sein Turn-Protokoll
  ist leer. Der Abschluss am 260825 war eine reine Umbenennung des Datensatzes
  (`git:2a77012`, `similarity index 100%`), die den Rumpf nicht angefasst hat.
  Er ist damit der einzige der siebzehn terminalen Datensätze ohne
  Schließungsnotiz; die Begründung des Abschlusses steht allein in der
  Commit-Nachricht.
- Drei weitere terminale Datensätze tragen ein leeres Turn-Protokoll:
  `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`,
  `260819-2230-auswahl-und-kopieren-in-der-vorschau` und, mit einer
  Platzhalterzeile, die zwei zurückgestellten Runden.
- Ein Ablageeintrag kann gebaut werden, ohne dass ihn etwas schließt. Der
  Shaper schließt einen Eintrag, den er zu einer Runde macht; die Runde 16 ist
  auf einem anderen Weg entstanden, und ihr Eintrag steht seither offen da.

## Vorgänge in der Ablage

Keine ausgeführt. Zwei vorgeschlagen und nicht ausgeführt, beide mangels
gehaltener Bestätigung:

- `close shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
- `close shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`

## Geschrieben

- `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/_a_circle.md`
  um einen Abschnitt `## Activation proposal` ergänzt.
- `portfolio.md` vollständig neu erzeugt.
