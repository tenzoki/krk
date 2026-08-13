# Portfolio

**Generated:** 260813-0714 (by playmaker session 260813-0714-playmaker-direct-dispatch)
**Domain bias:** code

Acht Circles liegen unter `circles/`: keiner aktiv, **einer** vorgesehen, sieben beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Seit dem Lauf vom 260812-2307 ist
die Runde 7 hinzugekommen und sofort auf beschränkten Abschluss gegangen. Das Feld der Kandidaten
ist unverändert eines; ein Vergleich zwischen Kandidaten ist auch in diesem Lauf nicht möglich,
und die Empfehlung steht auf absoluten Signalen.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen
zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

**Zur Rangheuristik, und warum sie hier ausgesetzt ist.** Die Standardheuristik der
Domain-Gewichtung `code` bevorzugt Circles, deren Abhängigkeiten sämtlich kohärent abgeschlossen
sind (`_c_`). In diesem Projekt trägt kein einziger abgeschlossener Circle diesen Marker: alle
sieben gefahrenen Runden stehen auf beschränktem Abschluss (`_b_`), und alle sieben aus demselben
Grund. Der Abnahmelauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
offen seit dem 260806). Die Heuristik vergäbe damit jedem denkbaren Kandidaten dasselbe
Kennzeichen und träfe keine Unterscheidung. Sie ist deshalb **nicht in die Rangfolge
eingerechnet**, sondern ausgesetzt und an dieser Stelle benannt. Was die Rangfolge stattdessen
trägt, sind absolute Signale am einzelnen Datensatz: der Zustand der geerbten Bauteile am Baum,
die Zahl und die Art der noch zu klärenden Fragen, und der Preis der Vorarbeit vor dem Plan.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden.
Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: der Orchestrator
löscht den Zeiger, wenn er einen Datensatz von `_t_circle.md` auf `_b_circle.md` umbenennt.

Der nächste Schritt liegt bei dir. Über `/fusion:next` entscheidest du, ob der eine vorgesehene
Circle aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige nicht
abgeschlossene Circle; seine geerbten Bauteile stehen unverändert, sein Eintrittspreis pro Befehl
ist seit der Runde 7 gestiegen.

### Rang 1 (von 1): `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters,
wird über die Tastatur bedient und trägt Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten:** eine Circle-Kante, auf `260802-0842-krk-mac-dateimanager-editor-git`
(Runde 1, beschränkt abgeschlossen). Dazu eine eingehende Kante aus der Runde 6
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_*_circle.md`,
Abschnitt `## Dependencies`), die der Datensatz des Betrachters selbst nicht führt. Zur Runde 7
besteht **keine** notierte Kante in beide Richtungen.

**Die Runde 7 hat den Zuschnitt dieses Circles nicht angetastet, und das ist diesmal eine
schwächere Auskunft als beim letzten Lauf.** Beim Abschluss der Runde 6 war die Unberührtheit ein
Signal: jene Runde baute an derselben Fläche, gerendertes Markdown in der Vorschau, und hat zwei
Fragen des Betrachters ausdrücklich zu seinen Gunsten entschieden, statt sie stillschweigend zu
verbrauchen. Die Runde 7 hat die Vorschau gar nicht berührt. Sie hat an der Belegungsansicht, am
Hauptmenü und an der Ablage gearbeitet. Der Zuschnitt ist unverändert, weil niemand in seine Nähe
gekommen ist, und nicht, weil jemand ihn geschont hätte.

**Was die Runde 7 diesem Circle tatsächlich hinterlässt, ist ein höherer Preis je Befehl.** Die
Menüleiste wird seit der Runde 7 aus der Belegung gerechnet und nicht mehr als Programmtext
gepflegt: `menuemodell::aufbau` liefert neun Obermenüs und einen Eintrag je Funktion, und
`crates/krk-ui/src/appkit/menue.rs` setzt das Ergebnis in AppKit um. Ein neuer Befehl des
Betrachters, etwa Blättern, Zurück, Vor oder das Ein- und Ausschalten der Sprungmarken, erzeugt
damit selbsttätig einen Menüeintrag, und der verlangt eine Ausgrauungsregel. Die Ausgrauung ist
nach dem Grounding der Runde 7 eine Korrektheitsbedingung und keine Politur: bis zu jener Runde
führte ein Menüeintrag mit Kürzel einen Befehl aus, den die Fokusprüfung gerade abgewiesen hatte.
Der Gewinn steht daneben und ist echt: jeder Befehl des Betrachters ist danach auf drei Wegen
erreichbar statt auf einem, ohne dass der Betrachter dafür etwas eigenes bauen müsste. Der
Aktivierungs-Spec sollte den Preis beziffern, statt ihn zu erben; die Größenordnung steht am Baum
mit sechsundsiebzig Einträgen in `Kommando::KENNUNGEN`
(`crates/krk-core/src/tasten/belegung.rs`).

**Die geerbten Bauteile stehen unverändert auf der Platte.** Die Auswertung der Zwischenablage,
das Vorschaufenster mit seiner Tableiste, die Statuszeile über die volle Fensterbreite seit der
Runde 6, und der Befehl auf `opt+cmd+g`. Die Mindestbreite der Vorschau von 160 Punkten ist
weiterhin nicht angetastet (`crates/krk-ui/src/fenstermodell.rs`), also bleiben die rund 17 Punkte
Luft bis zur gerechneten Obergrenze unverbraucht. Die zweite offene Frage des Betrachters, ob
lokale HTML-Dateien gerendert erscheinen, ist ihm von der Runde 6 ausdrücklich gelassen worden.

**Was gegen eine sofortige Aktivierung spricht, ist unverändert der Zuschnitt.** Der Datensatz
hält selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene
Untersuchung vor dem Plan gehört. Eine Untersuchung ist teurer als eine Klärungsrunde, und dieser
Circle braucht beides. Daneben stehen zwei ungemessene Fragen zur Verfügbarkeit von
Systemschnittstellen oberhalb von macOS 15
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`
und `shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`), die
gerade eine Web-Ansicht berühren.

**Die Klärungsrunde trägt fünf eigene Fragen, und eine sechste liegt daneben.** Die drei des
Abschnitts `## Grounding snapshot`, die Mindestbreite der Vorschau und ihre Schriftgröße. Dazu
gehört vor die Aktivierung der offene Nutzerentscheid
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`:
seine Antwort bestimmt, welchen Weg ein Tastendruck in einer Web-Ansicht nimmt. Der Fokusvorbehalt
`ersthelfer_gehoert_appkit` (`crates/krk-ui/src/appkit/ereignisse.rs`) fragt heute nach drei
Textklassen, und eine Web-Ansicht ist keine davon.

Die vier Feststellungen im Einzelnen stehen im Abschnitt `## Parent grounding stale` vom
260813-0714 im Datensatz
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`.

## Backlog — ranked

**(keiner)**

Ein Backlog-Speicher besteht in dieser Workbench nicht; `shared/backlog/` ist nicht angelegt.
Vorhaben entstehen in diesem Projekt über `/fusion:direct` unmittelbar als vorgesehener Circle.

## Recently closed (_c_ / _b_)

Die letzten fünf, neueste zuerst. Alle tragen `_b_`, keiner trägt `_c_`.

1. **`260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`** (Runde 7, `_b_`,
   260813). Die Belegungsansicht wird durch Tippen durchsucht, alle Funktionen stehen im Menü in
   neun Obermenüs mit Kürzel und Ausgrauung, und `opt+cmd+n` startet eine weitere Instanz, die
   sich die Ablage über zwei `flock`-Sperren mit der ersten teilt. Fünfzehn Planschritte gebaut,
   18 Commits, 1003 Proben über 19 Ziele. Nicht abgenommen. Der Nebengewinn wiegt nach eigener
   Einschätzung schwerer als eine der vier Fähigkeiten: ein Menüeintrag mit Kürzel führte bis
   dahin einen Befehl aus, den die Fokusprüfung abgewiesen hatte.

2. **`260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`** (Runde 6, `_b_`,
   260812). Teilen über die Freigabedienste und ein Kontextmenü an fünf Ansichten, Sprung in den
   Ordner der angezeigten Datei, eine beschädigte Ablagedatei wird zur Seite gelegt statt
   überschrieben, gerendertes Markdown in der Vorschau, eine Statuszeile über die volle
   Fensterbreite. Elf Planschritte, 25 Commits, 478 Proben. Nicht abgenommen.

3. **`260811-1304-statusleiste-mit-bereichsschaltern`** (Runde 5, `_b_`, 260812-0820). Die
   Bereichsleiste mit Schaltern und die proportionale Neuaufteilung. Dreizehn Abnahmekriterien
   sind nur am laufenden Bündel im Vordergrund zu sehen.

4. **`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`** (Runde 4, `_b_`, 260811-2210).
   Pfad des Ordners und des Eintrags kopieren, Öffnen mit dem Standardprogramm, `cmd+w` aus jedem
   Fokus. Alle 62 Abnahmekriterien offen; der Abgleich hat sie sortiert: 23 trägt der Baum, 32
   kann nur ein Mensch sehen, 7 brauchen einen Prüfaufbau.

5. **`260809-2040-tastenbelegung-als-markdown-in-downloads`** (Runde 3, `_b_`, 260811-1415). Die
   geltende Tastenbelegung als Markdown nach `~/Downloads/KRK-Tastenbelegung.md`, gegliedert nach
   den neun Funktionsbereichen. Der Abnahmelauf ist am 260811-1215 vom Nutzer gestrichen worden.

Nicht mehr unter den letzten fünf, aber weiterhin bindend: die Runde 2
(`260807-2116-eingebauter-editor-mit-textmarken`, `_b_`, 260810-1445) und die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`, `_b_`, 260807-1035). Die Runde 1 hält die zehn
Zeitzusagen aus C8 und die Frage nach dem Vordergrund, an der jeder Abschluss dieses Projekts
hängt.

## Archived (_s_ / _d_)

**(keiner)**

Kein Circle-Datensatz trägt `_s_` (überholt) oder `_d_` (zurückgestellt).

## Warnings

Keine Warnung hält etwas auf. Alle sieben unten sind Auskünfte für deine nächste Entscheidung.

**1. Der Abnahmelauf steht jetzt für zwei Runden aus, und die Frage darunter ist die älteste
offene des Projekts.** Weder die Runde 6 noch die Runde 7 ist abgenommen. Die eine Frage, an der
das hängt, wie KRK für den Abnahmelauf in den Vordergrund kommt, steht seit dem 260806 offen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Solange sie offen ist, endet jede weitere Runde beschränkt, unabhängig davon, wie gut sie gebaut
ist. Es ist die einzige Frage im Bestand, deren Beantwortung die Abschlussart künftiger Runden
ändert.

**2. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen
Entwicklungsbau.** Seit dem 260813 liegt dort ein beglaubigtes, universelles Bündel, das auf
jedem Mac startet. `cargo xtask bundle` und `cargo xtask release` schreiben an dieselbe Stelle,
und über `bundle` hängen `run`, `run-terminal`, `tasten`, `menue`, `durchstich` und `frisch`. Ein
`make run` nach dem Release-Lauf löscht die Beglaubigung. Der Preisunterschied ist der Punkt:
ein Entwicklungsbündel ist in Sekunden wieder da, ein beglaubigtes verlangt zwei Übersetzungsläufe
im Profil `release`, `lipo`, eine Signatur mit gehärteter Laufzeitumgebung und einen Netzlauf zu
Apple. Der Datensatz ist offen:
`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`.
Wenn du das Bündel weitergeben willst, sichere es, bevor du das nächste Mal baust.

**3. Vier Nutzerfragen sind gebaut und trotzdem unbeantwortet.** Die Runde 7 ist auf den
Empfehlungen von vier offenen Fragen gefahren, der Baum hat je eine Möglichkeit umgesetzt, und
alle vier stehen weiter auf `_o_`:
`shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`,
`shared/decisions/260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`,
`shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`
und
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`.
Die Lage ist die umgekehrte der gewohnten: nicht eine Antwort wartet auf ihre Umsetzung, sondern
eine Umsetzung wartet auf ihre Antwort. Bestätigst du eine Empfehlung nachträglich, geht der
Datensatz auf umgesetzt; entscheidest du anders, ist der Baum die Abweichung. Die vierte Frage
gehört zusätzlich vor die Aktivierung des Betrachters, weil sie den Weg eines Tastendrucks in
einer Web-Ansicht bestimmt.

**4. Die Abschlussnotiz der Runde 7 zählt acht verbliebene Datensätze, der Speicher trägt
vierzehn offene.** Die Notiz schreibt: „Die verbliebenen acht Datensaetze sind saemtlich von
derselben Art, eine Zusage ist weiter als ihre Wache, und keiner betrifft das Verhalten." Unter
`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/` liegen
vierzehn Datensätze mit `_o_`, und der Circle-Datensatz ist um 07:13 geschrieben worden,
nachdem der letzte davon um 07:08 entstanden war. Drei der vierzehn beschreiben sichtbares
Verhalten und nicht eine ungedeckte Zusage: ein Klick in die Bereichsleiste wirkt während einer
Umbenennung nicht (`260813-0311_*`, einer der zwei bewusst hingenommenen Verluste), zwei
Menüeinträge teilen sich `cmd+a` und AppKit nimmt dem späteren das Kürzel (`260813-0416_*`), und
das Menü Bearbeiten verliert seine mac-übliche Reihenfolge und seinen Trenner (`260813-0420_*`).
Die Aussage der Notiz ist damit zu eng. Die Zahl selbst hält keine Arbeit auf; wer sie
korrigieren will, tut es an der Notiz und nicht am Speicher.

**5. `CLAUDE.md` ist an mindestens drei nachgezählten Stellen überholt.** Die Datei nennt vier
gefahrene Runden, gefahren sind sieben. Sie nennt zwei vorgesehene Circles, vorgesehen ist einer.
Und sie nennt 68 Varianten für `Kommando`, während `Kommando::KENNUNGEN` in
`crates/krk-core/src/tasten/belegung.rs` sechsundsiebzig Einträge trägt. Der offene Datensatz
`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`
kennt den Fehler, nennt aber selbst 75 und ist mit der Runde 7 ebenfalls einen Schritt
zurückgefallen. Wer `CLAUDE.md` als Bestandsaufnahme liest statt als Kurzfassung, zählt in diesem
Projekt falsch; `/fusion:revise-claude-md` ist der vorgesehene Weg.

**6. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat genau
einen Knoten, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`, und keine Kante
innerhalb dieser Menge. Seine einzige Circle-Kante zeigt auf die Runde 1 und damit auf einen
terminalen Circle. Ein Zyklus besteht nicht.

**7. Parent grounding stale: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` nach dem
Abschluss der Runde 7.** Der Vermerk vom 260813-0714 steht im Datensatz des Betrachters und
nennt vier Feststellungen: der Eintrittspreis pro Befehl ist mit der gerechneten Menüleiste
gestiegen; die dritte Möglichkeit der ersten offenen Frage führt jetzt an eine `flock`-Sperre mit
elf benannten Löchern
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0716_*_die-bewachte-luecke-ist-nicht-die-luecke-elf-schreibwege-an-der-sperre-vorbei-bleiben.md`,
offen); der Fokusvorbehalt fragt nach drei Textklassen, und eine Web-Ansicht ist keine davon;
und die Messreihe hinter der dritten offenen Frage steht jetzt zwei Runden zurück.

Die wörtliche Auslösebedingung der Regel greift dabei nicht, und der Vermerk sagt das an seiner
Stelle selbst: der Abschnitt `## Grounding snapshot` des Betrachters zitiert die Runde 7 nicht,
und der Abschnitt `## Dependencies` der Runde 7 nennt keinen Circle. Zwischen beiden besteht
keine notierte Kante in irgendeine Richtung. Der Vermerk steht dort, weil die Runde 7 am Baum
drei Sätze jenes Grounding eingeholt hat.
