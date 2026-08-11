# Playmaker-Lauf 260811-2223 (direct-dispatch)

**Status:** Complete
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`
**Auslöser:** direkte Beauftragung durch den Nutzer nach dem Abschluss der Runde 4. Der Auftrag
nennt den Abschluss, den geräumten Zeiger und zwei Sachverhalte, und er verlangt ausdrücklich
keinen Commit. Ohne `/fusion:next` und ohne die Ansage eines Phase-4-Pings, deshalb
`direct-dispatch`.

**Schreibziel dieses Protokolls:** `shared/history/`, weil kein Circle aktiv ist.

## Bestand

Sechs Circle-Datensätze unter `circles/`, Marker aus dem Dateinamen gelesen:

| Marker | Zahl | Circles |
|---|---|---|
| `_t_` aktiv | 0 | — |
| `_a_` vorgesehen | 2 | `260811-1304-statusleiste-mit-bereichsschaltern`, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_b_` beschränkt abgeschlossen | 4 | `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`, `260809-2040-tastenbelegung-als-markdown-in-downloads`, `260807-2116-eingebauter-editor-mit-textmarken`, `260802-0842-krk-mac-dateimanager-editor-git` |
| `_c_` kohärent abgeschlossen | 0 | — |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Der
reguläre Zustand nach einem Abschluss; keine Zeigerwarnung.

## Rangfolge

**Rang 1: `260811-1304-statusleiste-mit-bereichsschaltern`.** Seine harte Vorbedingung, der vom
Nutzer am 260811-1240 gemeldete Rückfall der Vorschaubreite, ist am 260811-2130 in der Runde 4
gemessen und behoben worden (Commit `1ea5a3d`, Bruchstelle 1). Seine tragende Stelle ist eine
einzige Funktion ohne AppKit, `bereichsbreiten` (`crates/krk-ui/src/fenstermodell.rs:609`), und
damit ohne Vordergrund prüfbar. Seine sechs offenen Entscheidungen sind Zuschnittfragen an den
Nutzer.

**Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Besserer Zählwert bei den
offenen Entscheidungen, schlechterer Rang. Vor seiner Planung steht eine Untersuchung, welches
Mittel Web-Inhalt darstellt, dazu die ungemessene Verfügbarkeitsfrage für
macOS-26-Schnittstellen. Seine Grundlage stammt vom 260804 und kennt drei zwischenzeitlich
gefahrene Runden nicht.

Die Rangfolge kehrt die beiden Läufe vom 260811-1326 und 260811-1415 um, in denen die Statusleiste
hinter der Runde 4 stand.

## Zyklen

Kein `dependency-cycle-detected`. Der gerichtete Graph über die beiden nicht-terminalen Circles
trägt keine Kante zwischen zwei nicht-terminalen Knoten; alle Kanten enden auf beschränkt
abgeschlossenen Runden. Die einzige Nennung eines nicht-terminalen Circles durch einen anderen
steht im Datensatz der Statusleiste und ist dort ausdrücklich als nicht bindend ausgewiesen. Kein
Abschnitt `## Dependency warning` angefügt.

## Angefügte Abschnitte

Beide in `circles/260811-1304-statusleiste-mit-bereichsschaltern/_a_circle.md`, angefügt und nicht
umgeschrieben:

- `## Parent grounding stale` mit drei Punkten: die Directive sagt eine Behebung zu, die anderswo
  gefallen ist; die siebte offene Frage ist gegenstandslos und ihr Datensatz steht auf beantwortet;
  der Beifund `MINDESTGROESSE` von 780 Punkten gegen 920 Punkte Bedarf trifft die sechste offene
  Frage.
- `## Activation proposal` mit dem Vorschlag, sofort nach einer Klärungsrunde über den Zuschnitt zu
  aktivieren.

`parent-grounding-stale: parent=260811-1304-statusleiste-mit-bereichsschaltern child=260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`

Zur Auslösebedingung, offen benannt: die Regel verlangt, dass der Abschnitt `## Grounding snapshot`
des Elterndatensatzes den Verzeichnisnamen des Kindes oder den in seiner `## Closure note`
genannten Artefakt zitiert. Der Datensatz nennt weder das eine noch das andere. Er zitiert den
Defekt `shared/issues/260811-1245_*_…`, den die Runde 4 behoben hat, und deren `## Closure note`
adressiert den Befund unter „Für die Nachfolger" ausdrücklich an diesen Circle. Der Vermerk steht
deshalb trotzdem.

Für `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` wurde nichts angefügt. Sein
Datensatz zitiert die Runde 4 an keiner Stelle, und ihr Abschluss-Artefakt berührt ihn nicht. Der
Befund, dass seine Grundlage drei Runden nicht kennt, steht als Warnung 4 im Portfolio.

## Warnungen im Portfolio

- Keine Zeigerlage: kein `STALE-POINTER`, kein `POINTER-MISMATCH`, kein `MULTIPLE-ACTIVE`, kein
  `MISSING-POINTER`.
- Kein `dependency-cycle-detected`.
- Warnung 1, neu: die Rangheuristik verliert bei den Vorbedingungen ihre Trennschärfe. Vier von vier
  gefahrenen Runden sind `_b_`, jedes Mal weil der Abnahmelauf KRK im Vordergrund verlangt. Das
  Kennzeichen der unerfüllten Vorbedingung steht damit bei jedem Kandidaten und unterscheidet keine
  zwei mehr. Empfohlen ist, die Vorbedingung an der Beschränkung selbst zu prüfen statt am Marker.
  Der Playmaker ändert die Heuristik nicht; sie sitzt in der installierten Kopie des Plugins.
- Warnung 2, neu: zwei Warnungen des Laufs vom 260811-1415 zitieren Defektdatensätze, die es nicht
  gibt (`shared/issues/260811-0932_…` und `shared/issues/260810-1730_…`). Weder im Baum noch in der
  Git-Historie. Ursache ist eine Grenze der eigenen Zuständigkeit: der Playmaker darf keine Defekte
  anlegen, also durfte der Lauf einen Befund nicht als aufgenommen beschreiben. Beide Befunde
  bestehen in der Sache fort und stehen jetzt ohne Datensatzverweis.
- Warnung 3: der Kopf des Datensatzes der Runde 3 trägt `**Status:** anticipated` bei Dateiname
  `_b_circle.md`. Unverändert seit dem letzten Lauf.
- Warnung 4: der Aktivierungsvorschlag vom 260807-1042 im Datensatz des Web-Betrachters nennt ihn
  den empfohlenen nächsten Kandidaten; drei spätere Läufe haben ihn auf den letzten Rang gesetzt.
  Seine Grundlage kennt drei zwischenzeitlich gefahrene Runden nicht.
- Warnung 5: vier Stellen im Datensatz der Statusleiste sind mit der Runde 4 gealtert. Aufgeschlüsselt
  im angefügten Abschnitt `## Parent grounding stale`.
- Warnung 6: die Erzeugung von `portfolio.md` setzt die Sternform in Pfadzitaten nicht von selbst;
  dieser Lauf hat sie von Hand durchgehalten.
- Warnung 7: die Spec-Dateien der Runden 2, 3 und 4 bleiben auf `_o_` mit zusammen 213
  Abnahmekriterien auf `- [ ]`. Kein Versehen, sondern der Grund der Beschränkung.

## Was dieser Lauf nicht getan hat

Keine Umbenennung eines Markers, kein Schreiben von `.active-circle`, kein Defekt angelegt oder
geschlossen, keine Entscheidung angefasst, kein Plan und keine Aufgabenliste berührt, kein Commit.
Der Auftrag schließt den Commit ausdrücklich aus.
