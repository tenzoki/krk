# Playmaker-Lauf 260812-1027 (direct-dispatch)

**Status:** Complete
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Auftrags)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`
**Auslöser:** direkte Beauftragung. Der Auftrag trägt allein die Domänenzeile, ohne `/fusion:next`
und ohne die Ansage eines Phase-4-Pings, deshalb `direct-dispatch`.

**Schreibziel dieses Protokolls:** `shared/history/`, weil kein Circle aktiv ist.

## Bestand

Sieben Circle-Datensätze unter `circles/`, Marker aus dem Dateinamen gelesen.

| Marker | Zahl | Circles |
|---|---|---|
| `_t_` aktiv | 0 | — |
| `_a_` vorgesehen | 2 | `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_b_` beschränkt abgeschlossen | 5 | `260811-1304-statusleiste-mit-bereichsschaltern`, `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`, `260809-2040-tastenbelegung-als-markdown-in-downloads`, `260807-2116-eingebauter-editor-mit-textmarken`, `260802-0842-krk-mac-dateimanager-editor-git` |
| `_c_` kohärent abgeschlossen | 0 | — |
| `_s_` überholt | 0 | — |
| `_d_` zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Der
reguläre Zustand nach einem Abschluss; keine Zeigerwarnung.

Gegenüber dem Lauf vom 260812-0816 ist ein vorgesehener Circle hinzugekommen, angelegt vom Shaper
am 260812-1000. Kein Marker hat seither gewechselt.

Gelesene Eingaben: die drei offenen Fragen im gemeinsamen Speicher und die sechzehn in den
Circles, die acht offenen Defekte über alle Speicher, die beiden Circle-Datensätze der
vorgesehenen Circles vollständig, die `## Closure note` der fünf abgeschlossenen, das
Shaper-Protokoll `260812-1000-…`, das Bugfix-Protokoll `260812-0925-…`, das Orchestrator-Protokoll
`260812-0306-…`, das vorige Playmaker-Protokoll `260812-0816-…` sowie `CLAUDE.md`.

## Rangfolge

**Rang 1: `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`.** Vier
Nutzerfestlegungen vom 260812-0930 stehen im Wortlaut und entscheiden den Zuschnitt vor der
Aktivierung; das Baumaterial liegt auf der Platte; und zwei Vorrangkanten sagen, dass diese Runde
vor dem Web-Betrachter laufen sollte, weil sie zwei seiner offenen Fragen ohnehin entscheidet.

**Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.** Sein Datensatz verlangt
selbst eine Untersuchung vor dem Plan, und seine erste offene Frage ist die nach dem Zuschnitt.

**Die Rangfolge übergeht beide Zählwerte der Domänenheuristik, und die Begründung steht im
Portfolio als Warnung 1.** Der Zählwert der offenen Entscheidungsdatensätze gäbe dem
Web-Betrachter den Vorzug, drei gegen dreizehn. Die dreizehn liegen einzeln mit Möglichkeiten und
Folgen ab, die drei sind alles, was der Web-Betrachter abgelegt hat; seine vier offenen Punkte
stehen als Prosa im Grounding. Der Zählwert misst damit die Ablagedisziplin. Der zweite Zählwert,
durchweg kohärent abgeschlossene Abhängigkeiten, ist in diesem Projekt gegenstandslos, weil alle
fünf gefahrenen Runden aus demselben Grund beschränkt sind.

## Zyklen

Kein `dependency-cycle-detected`. Der gerichtete Graph über die nicht-terminalen Circles hat zwei
Knoten und zwischen ihnen eine einzige Kante, von der Runde 6 zum Web-Betrachter. Eine Gegenkante
besteht nicht: der Datensatz des Web-Betrachters führt in `## Dependencies` allein die Runde 1.
Alle übrigen Kanten enden auf terminalen Knoten. Kein Abschnitt `## Dependency warning` angefügt.

## Bounded-Closure-Propagation

Kein Abschnitt `## Parent grounding stale` angefügt, und kein `parent-grounding-stale`-Ereignis.

Seit dem Lauf vom 260812-0816 hat kein Circle auf `_b_` gewechselt. Die wörtliche
Auslösebedingung greift beim neuen Circle der Runde 6: sein `## Grounding snapshot` zitiert die
Verzeichnisnamen der Runden 1, 2 und 5. Der Vermerk bleibt aus, weil die Lage, die er anzeigen
soll, dort nicht besteht. Das Grounding ist am 260812-1000 am Baum erhoben worden, also Stunden
nach allen fünf Abschlüssen, und es benennt sie ausdrücklich in einem eigenen Abschnitt. Ein
Vermerk hätte eine Alterung behauptet, die es nicht gibt. Die Abweichung von der wörtlichen Regel
steht hier und im Portfolio, damit sie nachlesbar ist.

## Angefügter Abschnitt

Einer, in
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_a_circle.md`:
`## Activation proposal` vom 260812-1027. Er trägt die Rangbegründung, den Widerspruch zur
Zählung, die am Baum nachgelesenen Erbstücke, die zwei offenen Punkte und die Reihenfolgefrage.

An den Datensatz des Web-Betrachters ist nichts geschrieben worden. Er steht auf Rang 2, und der
Playmaker fügt einen Vorschlag nur an den empfohlenen Circle an. Sein Vorschlag vom 260812-0816
nennt ihn weiterhin den empfohlenen Kandidaten; der Widerspruch bleibt im Datensatz stehen und ist
im Portfolio als Warnung 3 benannt.

## Am Baum geprüft

Die tragenden Aussagen des angefügten Abschnitts sind am 260812-1027 im Baum gelesen und nicht aus
Datensätzen übernommen.

| Aussage | Fundstelle |
|---|---|
| Die eine Auswahlregel „Markierung hat Vorrang, sonst der Eintrag unter der Auswahl" | `crates/krk-ui/src/kommandos/operationen.rs:162` |
| Mindestbreite der Vorschau 160 Punkte | `crates/krk-ui/src/fenstermodell.rs:213` |
| Die Verteilung der Bereichsbreiten | `crates/krk-ui/src/fenstermodell.rs:1044` |
| `NSSharingServicePicker` und `menuForEvent:` kommen im Baum nicht vor | Suche über `crates/`, kein Treffer |
| Die `wunschauswahl` überlebt einen laufenden Lesevorgang | `crates/krk-ui/src/tabs.rs:55`, `:187-197` |

## Warnungen im Portfolio

1. Die Rangheuristik trägt an beiden Zählwerten nicht. Fortgeschrieben aus dem Lauf vom
   260812-0816 und um den zweiten Zählwert erweitert, der sich in diesem Feld gegen seinen Zweck
   kehrt.
2. Neu: die Kante zwischen der Runde 6 und dem Web-Betrachter ist eine Vorrangkante und keine
   Abhängigkeit. Löst die Warnung 2 des vorigen Laufs ab, die dieselbe Einbahnigkeit zwischen der
   Runde 5 und dem Web-Betrachter beschrieb.
3. Neu: der Datensatz des Web-Betrachters nennt sich selbst den empfohlenen Kandidaten, dieser
   Lauf setzt ihn auf Rang 2. Löst die Warnung 3 des vorigen Laufs über die doppelten Abschnitte
   ab und trägt deren Kern weiter.
4. Der Kopf des Datensatzes der Runde 3 trägt `**Status:** anticipated` bei Dateiname
   `_b_circle.md`. Unverändert seit dem 260811-1415.
5. Neu: `CLAUDE.md` ist an zwei Stellen gealtert, Zeile 11 mit vier statt fünf Runden und Zeile
   158 mit der Statusleiste als vorgesehenem Circle auf Rang 1.
6. Neu: fünf offene Defekte liegen im terminalen Circle der Runde 5, drei weitere im gemeinsamen
   Speicher, und kein vorgesehener Circle nimmt sie auf.
7. Die Spec-Dateien der Runden 2, 3 und 4 bleiben auf `_o_`; die Runde 5 hat keinen Spec.
   Fortgeschrieben.
8. Der Plan der Runde 5 führt drei Wahlpunkte als unabgehakte Kästchen, deren Datensätze auf `_i_`
   stehen. Fortgeschrieben.
9. Die Sternform in den Pfadzitaten des Portfolios hält kein Mechanismus. Fortgeschrieben.

Nicht fortgeschrieben: die Warnung 7 des Laufs vom 260812-0816 über den Abschluss der Runde 5, der
vier Minuten nach jenem Lauf datiert. Sie betraf das Verhältnis zwischen jenem Laufzeitpunkt und
dem Abschluss und ist gegenstandslos.

## Was dieser Lauf nicht getan hat

Keine Umbenennung eines Markers, kein Schreiben oder Löschen von `.active-circle`, kein Defekt
angelegt oder geschlossen, keine Entscheidung angefasst, kein Plan und keine Aufgabenliste
berührt, kein `## Dependencies` ergänzt, kein `CLAUDE.md` geändert, kein Commit.
