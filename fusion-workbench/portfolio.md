# Portfolio

**Generated:** 260812-1027 (by playmaker session 260812-1027-playmaker-direct-dispatch)
**Domain bias:** code

Sieben Circles liegen unter `circles/`: keiner aktiv, zwei vorgesehen, fünf beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Seit dem Lauf vom 260812-0816 ist
ein vorgesehener Circle hinzugekommen, die Runde 6 mit Teilen, Ordnersprung, Ablagesicherung und
gerenderter Vorschau. Das Feld trägt damit zum ersten Mal in diesem Projekt einen Vergleich
zwischen zwei Kandidaten statt einer Rangfolge mit einem Element.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen
zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden.
Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: der Orchestrator
löscht den Zeiger bei der Umbenennung auf `_b_`, zuletzt mit Commit `25a8429`.

Der nächste Schritt liegt beim Nutzer. Er entscheidet über `/fusion:next`, welcher der beiden
vorgesehenen Circles aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — vier
Nutzerfestlegungen stehen im Wortlaut, das meiste Baumaterial liegt auf der Platte, und die Runde
entscheidet zwei Fragen, die der zweite Kandidat sonst ohne ihren Zusammenhang entscheiden müsste.

Die Empfehlung widerspricht der wörtlichen Zählung der Domänenheuristik. Die Gewichtung `code`
bevorzugt Kandidaten mit wenigen offenen Entscheidungsdatensätzen, und nach dieser Zahl gewänne
der Web-Betrachter mit drei gegen dreizehn. Die Zahl trägt hier nicht: die dreizehn Fragen der
Runde 6 liegen einzeln als Datensatz vor, mit Möglichkeiten und Folgen, während die offenen Punkte
des Web-Betrachters als Prosa in seinem `## Grounding snapshot` stehen und nie abgelegt worden
sind. Gezählt wird damit die Ablagedisziplin und nicht die Reife. Die ausführliche Begründung
steht im Abschnitt `## Activation proposal` des empfohlenen Datensatzes, angefügt in diesem Lauf.

### Rang 1: `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`

**KRK teilt Dateien, springt zum Ordner der angezeigten Datei, sichert seine Ablage und zeigt
Markdown gerendert.** Angelegt am 260812-1000, Domain `code`. Abhängigkeiten: die Runden 1, 2, 4
und 5, alle beschränkt abgeschlossen, dazu eine Vorrangkante auf den Web-Betrachter.

Vier Wünsche des Nutzers vom 260812-0930 in einer Runde. Teilen über die Freigabedienste des
Systems, ausgelöst über Tastenbefehl und ein neues Kontextmenü. Ein Befehl, der das aktive
Dateifenster in den Ordner der Datei bringt, die Vorschau oder Editor gerade zeigen. Eine
beschädigte Ablagedatei wird zur Seite gelegt statt überschrieben, für alle vier Dateien unter
`~/Library/Application Support/KRK/`. Und die Vorschau zeigt Markdown vollständig gerendert sowie
Quelltext eingefärbt, wobei der Text sofort erscheint und die Farben kurz danach nachziehen, damit
die Zeitzusage L7 aus C8 der Runde 1 unangetastet bleibt.

Für den Circle spricht, dass sein Zuschnitt entschieden ist, bevor die Aktivierung beginnt. Der
Nutzer hat vier Fragen beantwortet und ausdrücklich angewiesen, ihn nicht erneut zu fragen; die
Antworten stehen im Datensatz als Festlegung und nicht als Möglichkeit
(`shared/history/260812-1000-shaper-teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`).
Das Baumaterial liegt auf der Platte und ist in diesem Lauf am Baum nachgelesen: die Auswahlregel
`betroffene` in `crates/krk-ui/src/kommandos/operationen.rs:162`, der Weg in den Zielordner über
`Dateifenster::ordner_lesen` und `Tabliste::ordner_setzen` mit bereits zwei Aufrufern, und
`crates/krk-ui/src/hervorhebung.rs`, das `syntect` einmal über den Text führt und keine Zeile
AppKit trägt.

Gegen eine sofortige Aktivierung stehen zwei Punkte, und beide sind Planarbeit statt einer
vorgelagerten Untersuchung. Womit die Vorschau Markdown zerlegt, legt der Circle nicht fest. Und
das Teilen hat keinen Anknüpfungspunkt: `NSSharingServicePicker` kommt im Baum nicht vor, ein
`menuForEvent:` steht an keiner Stelle unter `crates/krk-ui/src/appkit/`, KRK hat heute also kein
eigenes Kontextmenü. Beides ist am 260812-1027 über den ganzen Baum unter `crates/` geprüft.

Die dreizehn offenen Fragen in `decisions/` dieses Circles sind die erste Arbeit der
Klärungsrunde. Vier binden über den Circle hinaus, und zwei davon entscheiden zugleich über den
Web-Betrachter.

### Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**KRK zeigt Web-Seiten in einem eigenen Betrachter.** Angelegt am 260804-0933, Domain `code`.
Abhängigkeit: die Runde 1, beschränkt abgeschlossen.

Eine Web-Adresse aus der Zwischenablage öffnet KRK künftig selbst, in einem gewöhnlichen Tab des
Vorschaufensters, statt sie an den Systembrowser abzugeben. Bedient wird der Betrachter über die
Tastatur, mit Sprungmarken auf jedem sichtbaren Link.

Auf Rang 2 steht er aus drei Gründen, von denen zwei seit dem Lauf vom 260812-0816 unverändert
gelten. Sein Datensatz verlangt selbst „eine eigene Untersuchung vor dem Plan" für das Mittel, mit
dem Web-Inhalt dargestellt wird, und eine Untersuchung ist teurer als eine Klärungsrunde. Seine
erste offene Frage, welche Quellen eine Adresse setzen dürfen, ist die Frage nach dem Zuschnitt:
sie entscheidet, ob KRK einen Betrachter oder einen Browser bekommt. Neu hinzu kommt die
Reihenfolge: die Runde 6 zieht seine zweite offene Frage vor, ob lokale HTML-Dateien gerendert
erscheinen, und entscheidet über die rund 17 Punkte Breite, die ihm oberhalb der heutigen
Mindestbreite der Vorschau von 160 Punkten bleiben (`crates/krk-ui/src/fenstermodell.rs:213`,
Verteilung bei `:1044`). Läuft er zuerst, entscheidet er beide Fragen ohne den Zusammenhang, in
dem sie entstehen.

Die vier Fragen seiner Klärungsrunde und die Einzelheiten zur Mindestbreite stehen in den
Abschnitten `## Grounding snapshot`, `## Parent grounding stale` und `## Activation proposal`
seines Datensatzes. Der jüngste Vorschlag dort stammt vom 260812-0816 und nennt ihn den
empfohlenen Kandidaten; dieser Lauf setzt ihn auf Rang 2, siehe Warnung 3.

## Recently closed (_c_ / _b_)

Fünf abgeschlossene Circles, alle beschränkt (`_b_`), keiner kohärent (`_c_`). Neueste zuerst.

**`260811-1304-statusleiste-mit-bereichsschaltern`** — `_b_`, 260812-0820. Die Bereichsleiste am
Fensterfuß steht mit acht Ankreuzfeldern, fünf für die Bereiche und drei für die Spalten Größe,
Datum und Typ; die Breitenregel verteilt Anteile statt Punktzahlen. Beschränkt, weil dreizehn
Abnahmekriterien nur am laufenden Bündel im Vordergrund zu sehen sind.

**`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`** — `_b_`, 260811-2210. Pfad des Ordners
und des Eintrags kopieren, Einträge an das Standardprogramm geben, `cmd+w` aus jedem Fokus.
Beschränkt aus demselben Grund; 23 der 62 Kriterien hat der Abgleich am Baum belegt.

**`260809-2040-tastenbelegung-als-markdown-in-downloads`** — `_b_`, 260811-1415. KRK schreibt die
geltende Tastenbelegung als Markdown nach `~/Downloads/KRK-Tastenbelegung.md`. Beschränkt, weil
der Nutzer den Abnahmeschritt am 260811-1215 gestrichen hat.

**`260807-2116-eingebauter-editor-mit-textmarken`** — `_b_`, 260810-1445. Der Editor als fünfter
Bereich, mit Roh- und Formatansicht, Suchen, Ersetzen und Textmarken. Beschränkt aus zwei Gründen,
die beide beim Nutzer liegen: der Abnahmelauf über 110 Kriterien und die zurückgestellte Frage
nach einem Prüfziel ohne `libtest`-Harness.

**`260802-0842-krk-mac-dateimanager-editor-git`** — `_b_`, 260807-1035. Das Navigator-Gerüst mit
Lesezeichenleiste, zwei Dateifenstern, Vorschau, Dateioperationen und Messmodus. Beschränkt wegen
des Belegstands der zehn Zeitzusagen, nicht wegen der Arbeit.

## Archived (_s_ / _d_)

**(keiner)**

Kein Circle-Datensatz trägt `_s_` (überholt) oder `_d_` (zurückgestellt).

## Warnings

Keine Zeigerlage: kein `STALE-POINTER`, kein `POINTER-MISMATCH`, kein `MULTIPLE-ACTIVE`, kein
`MISSING-POINTER`.

Kein `dependency-cycle-detected`. Der gerichtete Graph über die beiden nicht-terminalen Circles
trägt eine einzige Kante zwischen ihnen, von der Runde 6 zum Web-Betrachter; eine Gegenkante
besteht nicht. Alle übrigen Kanten enden auf terminalen Knoten. Kein Abschnitt
`## Dependency warning` angefügt.

Kein Abschnitt `## Parent grounding stale` angefügt. Seit dem Lauf vom 260812-0816 hat kein Circle
auf `_b_` gewechselt. Die wörtliche Auslösebedingung greift beim neuen Circle der Runde 6, dessen
`## Grounding snapshot` die Verzeichnisnamen dreier beschränkt abgeschlossener Runden zitiert; die
Lage, die der Vermerk anzeigen soll, besteht dort nicht. Sein Grounding ist am 260812-1000 erhoben
worden, also Stunden nach allen fünf Abschlüssen, und benennt sie ausdrücklich. Ein Vermerk hätte
eine Alterung behauptet, die es nicht gibt.

**1. Die Rangheuristik trägt in diesem Projekt an beiden Zählwerten nicht.** Der erste Zählwert,
durchweg kohärent abgeschlossene Abhängigkeiten, ist gegenstandslos: fünf von fünf gefahrenen
Runden sind beschränkt abgeschlossen, jedes Mal weil der Abnahmelauf KRK im Vordergrund verlangt
und damit Nutzerarbeit ist
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Jeder denkbare Kandidat trägt deshalb das Kennzeichen der unerfüllten Vorbedingung. Der zweite
Zählwert, wenige offene Entscheidungsdatensätze, kehrt sich in diesem Lauf gegen seinen Zweck: er
gewichtet den Kandidaten hoch, der seine offenen Punkte als Prosa führt, und den niedrig, der sie
einzeln abgelegt hat. Beide Zählwerte sind in diesem Lauf zugunsten der Sachlage übergangen, und
die Begründung steht im Vorschlag am empfohlenen Datensatz. Der Playmaker ändert die Heuristik
nicht; sie sitzt in der installierten Kopie des Plugins.

**2. Die Kante zwischen der Runde 6 und dem Web-Betrachter ist eine Vorrangkante und keine
Abhängigkeit.** Der Abschnitt `## Dependencies` der Runde 6 nennt den Web-Betrachter beim Namen,
und ein Graph, der jede Nennung dort als „hängt ab von" liest, folgert daraus, die Runde 6 sei
durch einen nicht abgeschlossenen Circle blockiert. Die Richtung ist die umgekehrte: die Runde 6
nimmt dem Web-Betrachter zwei Entscheidungen vorweg und sollte deshalb vor ihm laufen. Der
Datensatz sagt es selbst, im Satz „Die beiden Kanten nach rechts sind der Grund, aus dem die
Reihenfolge der beiden Circles nicht beliebig ist."

**3. Der Datensatz des Web-Betrachters nennt sich selbst den empfohlenen Kandidaten, und dieser
Lauf setzt ihn auf Rang 2.** Sein jüngster Abschnitt `## Activation proposal` stammt vom
260812-0816, aus einem Feld mit einem einzigen Kandidaten. Der Playmaker fügt einen Vorschlag nur
an den empfohlenen Circle an und schreibt bestehende Abschnitte nie um, also bleibt der
Widerspruch im Datensatz stehen. Maßgeblich ist diese Datei: sie entsteht bei jedem Lauf neu und
trägt die aktuelle Rangfolge.

**4. Der Datensatz der Runde 3 trägt im Kopf `**Status:** anticipated` bei Dateiname
`_b_circle.md`.** Unverändert seit dem Lauf vom 260811-1415. Der Marker am Dateinamen ist die
maßgebliche Aussage, die Kopfzeile widerspricht ihm. Der Playmaker schreibt keine Kopfzeilen.

**5. `CLAUDE.md` ist gegenüber dem Circle-Bestand an zwei Stellen gealtert.** Zeile 11 sagt „Vier
Runden sind gefahren"; es sind fünf, seit die Bereichsleiste am 260812-0820 geschlossen hat. Zeile
158 führt die Statusleiste als vorgesehenen Circle auf Rang 1; sie ist gefahren und beschränkt
abgeschlossen, und vorgesehen sind heute der Web-Betrachter und die Runde 6. Der Playmaker
schreibt nicht in `CLAUDE.md`; `/fusion:revise-claude-md` ist der Weg.

**6. Fünf offene Defekte liegen im Circle der Runde 5, die terminal ist, und kein vorgesehener
Circle nimmt sie auf.** Dazu drei im gemeinsamen Speicher. Ein beschränkter Abschluss lässt seine
offenen Defekte stehen, und die Directive der Runde 6 nennt keinen davon. Wer sie fahren will,
braucht dafür einen eigenen Circle oder einen Nachtrag zur Runde 6. Verbindlich ist der
Dateibestand:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`.

**7. Vier von fünf Runden lassen ihre Abnahmekriterien unabgehakt zurück, und das ist kein
Versehen.** Die Spec-Dateien der Runden 2, 3 und 4 stehen auf `_o_`; die Runde 5 hat gar keinen
Spec, ihre Fähigkeiten und Kriterien stehen im Plan
(`circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_*_bereichsleiste-und-proportionale-breitenregel.md`),
der auf `_c_` steht, während dreizehn seiner Kriterien nur am laufenden Bündel abzunehmen sind.
Wer die offene Arbeit dieses Projekts zählt, zählt an diesen Dateien und nicht an den Markern.

**8. Der Plan der Runde 5 führt drei Wahlpunkte als unabgehakte Kästchen, deren Datensätze
sämtlich auf umgesetzt stehen.** Betroffen sind die Kombinationen der beiden neuen Umschalter, das
Verhalten des Editorschalters ohne Datei und das Verhalten unter der Summe der Mindestbreiten.
Geringes Gewicht, aber ein Leser des Plans hält sie für offen.

**9. Die Sternform in den Pfadzitaten dieser Datei hält kein Mechanismus.** Der Lauf hat sie von
Hand durchgehalten. Bei jedem künftigen Lauf ist sie erneut von Hand zu prüfen, und eine
Handkorrektur an dieser Datei überlebt den nächsten Lauf nicht.

Nicht fortgeschrieben aus dem Lauf vom 260812-0816: dessen Warnung 7, die `## Closure note` der
Runde 5 datiere den Abschluss vier Minuten nach jenem Lauf. Sie betraf das Verhältnis zwischen
jenem Laufzeitpunkt und dem Abschluss und ist mit diesem Lauf gegenstandslos.
