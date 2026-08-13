# Portfolio

**Generated:** 260813-1510 (by playmaker session 260813-1510-playmaker-direct-dispatch)
**Domain bias:** code

Neun Circles liegen unter `circles/`: keiner aktiv, **einer** vorgesehen, acht abgeschlossen. Von
den acht trägt einer `_c_` und sieben tragen `_b_`. Kein Circle ist überholt oder zurückgestellt.
Seit dem Lauf vom 260813-0958 ist die Runde 8
(`260813-0939-titelleiste-fuehrt-version-und-semantische-tags`) gefahren und geschlossen, und zwar
**kohärent, als erste dieses Projekts**. Der Ideenspeicher ist leer.

**Der wichtigste Satz dieses Laufs ist eine Berichtigung.** Fünf Portfolios in Folge haben
behauptet, jede weitere Runde ende beschränkt, solange die Frage nach dem Vordergrund offen sei.
Die Behauptung war falsch, und die Runde 8 hat sie widerlegt. Was sie gelöst hat, war nicht die
Frage, sondern der Weg um sie herum: der Nutzer hat die Abnahme von Hand gefahren. Die Einzelheiten
stehen unter `## Warnings`, Punkt 1.

**Die Rangfolge hat wieder ein Element.** Der Rang-1-Circle des vorigen Laufs ist gebaut und
geschlossen, übrig bleibt der Web-Betrachter, seit dem 260804 vorgesehen. Er steht auf Rang 1, weil
er der einzige ist, und nicht weil er sich gegen etwas durchgesetzt hätte.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen zwei
Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

**Zur Rangheuristik, und warum ihre bisherige Aussetzungsbegründung weggefallen ist.** Die
Standardheuristik der Domain-Gewichtung `code` bevorzugt Circles, deren Abhängigkeiten sämtlich
kohärent abgeschlossen sind. Vier Läufe lang war sie mit der Begründung ausgesetzt, kein Circle
dieses Projekts trage `_c_`, also unterscheide der Marker nichts. Diese Begründung ist mit der
Runde 8 hinfällig: es gibt jetzt einen kohärent abgeschlossenen Circle.

Für den einen Kandidaten ändert das nichts, und der Grund dafür ist schärfer als der alte. Seine
einzige Circle-Kante führt auf die Runde 1, die `_b_` trägt. `_b_` ist ein Endzustand, und ein
Endzustand wird nicht zurückgenommen (`rules/circle-records.md`, Abschnitt
`### Worked transitions`). Die Prüfung fällt für diesen Circle also für immer negativ aus, gleich
welche Arbeit noch geschieht. Ein Kriterium, dessen Wert keine künftige Arbeit ändern kann, ist für
ihn kein Rangsignal, sondern eine Konstante, und bleibt draußen. Was die Einordnung stattdessen
trägt, sind absolute Signale am Datensatz: der Preis der Vorarbeit vor dem Plan, die Zahl der noch
zu klärenden Fragen, und das Alter der Grundlage.

## Active (_t_)

**(keiner)**

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss und keine Warnung.

## Anticipated (_a_) — ranked

Recommended next: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
vorgesehene Circle; vor seiner Aktivierung stehen eine Untersuchung des Darstellungsmittels und
eine Klärungsrunde über sechs Fragen.

```
/fusion:next 260804-0933-eingebauter-web-betrachter-im-vorschaufenster
```

**Vor diesem Kommando liegen zwei billigere Schritte**, und beide gehören dir. Sie stehen unter
`## Warnings` als Punkt 2 und Punkt 3: der Tag `v0.1.0`, ohne den der Auslieferungsweg heute
abweist, und ein paar Ideen im leeren Ideenspeicher, damit der nächste Lauf wieder eine Auswahl
vorlegen kann statt einer Liste mit einem Eintrag.

### Rang 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive.** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters,
bedient wird er über die Tastatur, mit Sprungmarken auf jedem sichtbaren Link. Kein Verlauf, kein
dauerhaftes Adressfeld, kein Herunterladen; angezeigt werden `http:` und `https:` wie schon heute.

**Abhängigkeiten.** Eine auf einen Circle: die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`, beschränkt abgeschlossen). Der Circle erweitert
deren Grenze und ist keine spätere Runde davon: er holt einen Punkt herein, den der Datensatz der
Runde 1 unter `## Ausdrücklich außerhalb dieses Circles` ausschließt.

**Warum Rang 1.** Weil er der einzige ist. Diese Zeile ersetzt in diesem Lauf die Begründung, die
der Lauf vom 260813-0958 noch führen konnte, als zwei Kandidaten nebeneinander standen und dieser
auf Rang 2 fiel. Am Datensatz selbst hat sich seither nichts geändert.

**Was sich zu seinen Gunsten geändert hat, liegt am Projekt und nicht an ihm.** Eine Runde, die
ihn ausführt, kann jetzt kohärent enden, sofern der Nutzer dieselbe Handabnahme fährt wie in der
Runde 8. Sieben Runden lang war das keine Aussicht, sondern ausgeschlossen. Die Aussicht hängt an
deiner Bereitschaft und nicht an einer gelösten technischen Frage.

**Was dagegen spricht, ist unverändert und wiegt schwer.** Der Datensatz hält selbst fest, dass das
Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene Untersuchung vor dem Plan
gehört. Eine Untersuchung ist teurer als eine Klärungsrunde, und dieser Circle braucht beides.
Daneben steht die ungemessene Verfügbarkeitsfrage für macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`,
offen) und die projektweite Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`, offen).

**Die Klärungsrunde trägt sechs Fragen**, unverändert seit dem 260813-0714: welche Quellen die
Adresse setzen dürfen, ob der Betrachter auch lokale HTML-Dateien zeigt, ob er eine eigene
Zeitzusage bekommt, die Mindestbreite der Vorschau, ihre Schriftgröße, und der offene
Nutzerentscheid darüber, welchen Weg ein Tastendruck in einer Web-Ansicht nimmt
(`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`).
Die Runde 8 hat keine davon beantwortet und keine hinzugefügt.

**Was die Runde 8 an seiner Grundlage bewegt hat**, steht als Vermerk `## Parent grounding stale`
im Datensatz, angehängt bei diesem Lauf. Zwei Feststellungen: die Zulässigkeitsregel fragt jetzt
vier Dinge statt drei, was den Preis je Befehl des Betrachters um eine Frage erhöht; und der
Umgang mit einer Ansicht, die nicht über ihre Klasse zu erkennen ist, hat mit der hereingereichten
Prüffunktion für die Editorfläche einen gebauten Präzedenzfall bekommen. Die zweite ist die erste
Feststellung seit vier Läufen, die für diesen Circle spricht statt gegen ihn.

## Backlog — ranked

**(keiner)**

`shared/backlog/` trägt einen einzigen Eintrag, und der steht auf `_c_`:
`260813-0822_*_titelleiste-fuehrt-name-und-version.md` ist mit der Anlage der Runde 8 geschlossen.
Kein Eintrag trägt `_o_` (offen) oder `_p_` (empfohlen). Der Speicher ist leer, und leer heißt
hier: die letzte Empfehlung ist ausgeführt und nichts ist nachgerückt.

Ideen legst du mit `/fusion:memo` ab; ein Titel und ein Absatz genügen. Kein Agent schreibt in
diesen Speicher, der Playmaker liest und ordnet ihn nur. Warum das jetzt zählt, steht unter
`## Warnings`, Punkt 3.

## Recently closed (_c_ / _b_)

Die letzten fünf, neueste zuerst. Der erste trägt `_c_`, die vier darunter `_b_`.

1. **`260813-0939-titelleiste-fuehrt-version-und-semantische-tags`** (Runde 8, **`_c_`**,
   260813-1415). Die Titelleiste trägt links `KRK 0.1.0` über
   `NSTitlebarAccessoryViewController` im neuen Modul `crates/krk-ui/src/appkit/titelzusatz.rs`,
   der Pfad bleibt mittig und ungekürzt. Dieselbe Zahl steht im Über-Dialog. `cargo xtask release`
   bricht als erste Station ab, solange HEAD keinen Tag `v<version>` trägt oder eine verfolgte
   Datei geändert ist. Zwei Turns, elf Commits, 16 von 16 Planschritten auf `[DONE]`, `make check`
   exit 0 mit 1025 Proben. **Abgenommen:** alle elf Beobachtungen mit Bündelanteil, vom Nutzer am
   260813-1410 am laufenden Bündel gefahren. Offen bleibt ein einziges Abnahmekriterium, C3.15,
   der Tag `v0.1.0`.

2. **`260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`** (Runde 7, `_b_`,
   260813). Die Belegungsansicht wird durch Tippen durchsucht, alle 82 Funktionen stehen im Menü
   in neun Obermenüs mit Kürzel und Ausgrauung, und `opt+cmd+n` startet eine weitere Instanz, die
   sich die Ablage über zwei `flock`-Sperren mit der ersten teilt. Fünfzehn Planschritte, 18
   Commits, 1003 Proben über 19 Ziele. Nicht abgenommen.

3. **`260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`** (Runde 6, `_b_`,
   260812). Teilen über die Freigabedienste und ein Kontextmenü an fünf Ansichten, Sprung in den
   Ordner der angezeigten Datei, eine beschädigte Ablagedatei wird zur Seite gelegt statt
   überschrieben, gerendertes Markdown in der Vorschau, eine Statuszeile über die volle
   Fensterbreite. Elf Planschritte, 25 Commits, 478 Proben. Nicht abgenommen.

4. **`260811-1304-statusleiste-mit-bereichsschaltern`** (Runde 5, `_b_`, 260812-0820). Die
   Bereichsleiste mit Schaltern und die proportionale Neuaufteilung. Dreizehn Abnahmekriterien
   sind nur am laufenden Bündel im Vordergrund zu sehen.

5. **`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`** (Runde 4, `_b_`, 260811-2210).
   Pfad des Ordners und des Eintrags kopieren, Öffnen mit dem Standardprogramm, `cmd+w` aus jedem
   Fokus. Alle 62 Abnahmekriterien offen; der Abgleich hat sie sortiert: 23 trägt der Baum, 32
   kann nur ein Mensch sehen, 7 brauchen einen Prüfaufbau.

Nicht mehr unter den letzten fünf, aber weiterhin bindend: die Runde 3
(`260809-2040-tastenbelegung-als-markdown-in-downloads`, `_b_`, 260811-1415), die Runde 2
(`260807-2116-eingebauter-editor-mit-textmarken`, `_b_`, 260810-1445) und die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`, `_b_`, 260807-1035). Die Runde 1 hält die zehn
Zeitzusagen aus C8 und die Frage nach dem Vordergrund; sie ist zugleich die einzige
Circle-Abhängigkeit des Rang-1-Kandidaten.

## Archived (_s_ / _d_)

**(keiner)**

Kein Circle-Datensatz trägt `_s_` (überholt) oder `_d_` (zurückgestellt).

## Warnings

Keine Warnung hält etwas auf. Punkt 2 und Punkt 3 sind Schritte für dich, die übrigen sind
Auskünfte für deine nächste Entscheidung.

**1. Berichtigung: die Behauptung, jede weitere Runde ende beschränkt, war falsch.** Fünf
Portfolios in Folge, zuletzt das vom 260813-0958, haben geschrieben: solange die Frage nach dem
Vordergrund offen ist, endet jede weitere Runde beschränkt, unabhängig davon, wie gut sie gebaut
ist. Die Runde 8 ist kohärent geschlossen, und die Frage steht unverändert offen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
offen seit dem 260806).

Der Fehler war eine Verwechslung zweier Abnahmewege. Der Datensatz fragt, wie die **automatische
Messstrecke** aus `krk-bench` KRK in den Vordergrund bekommt, damit ihre synthetischen
Tastendrücke ankommen; er nennt drei Möglichkeiten, und keine ist gewählt. Die Runde 8 ist auf
einem anderen Weg abgenommen worden: einer **Beobachtungsliste, die der Nutzer von Hand am
laufenden Bündel abarbeitet**
(`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1405-abnahmeliste-e2.md`,
elf Beobachtungen, alle bestanden). Dieser Weg braucht die offene Frage nicht.

Was daraus folgt und was nicht. Der Abschlusstyp einer Runde hängt an deiner Bereitschaft, die
Handabnahme zu fahren, und nicht am Stand jener Frage. Die zehn Zeitzusagen aus C8 bleiben davon
unberührt: sie sind gemessene Zahlen und keine Beobachtungen, sie brauchen die Messstrecke, und
für sie gilt der Datensatz unverändert. Die letzte vollständige Messreihe ist die vom 260810
(`messungen/260810-1918-alle-zusagen.txt`); seither liegen die Runden 5 bis 8 dazwischen.

**2. `cargo xtask release` weist heute ab, aus zwei Gründen, und der zweite trifft dich bei jedem
Lauf wieder.** Nachgeprüft am 260813-1510 gegen den Baum.

Der erste Grund ist der bekannte: `git tag -l` liefert in diesem Baum nichts, und Station 1 von
`cargo xtask release` verlangt auf HEAD einen Tag `v0.1.0` (`xtask/src/release.rs:113`, Prüfung in
`stand_pruefen` ab Zeile 226). Die Abschlussnotiz der Runde 8 nennt diesen Schritt als den einen
offenen, und er gehört dir: `git tag v0.1.0 <abschlusscommit>`. Danach ist auch C3.15 erfüllt.

Der zweite Grund ist neu und in keinem Datensatz benannt. Dieselbe Station fragt zusätzlich
`git status --porcelain --untracked-files=no` und bricht ab, sobald irgendeine verfolgte Datei
geändert ist, **ohne Pfadfilter**; die Wahl gegen einen Filter ist am Konstantenkopf `GIT_STAND`
ausdrücklich begründet. Verfolgt sind aber vier Dateien, die die Werkbank als flüchtigen
Sitzungszustand führt und bei jedem Agentenlauf neu schreibt: `fusion-workbench/monitor`,
`fusion-workbench/.fusion-setup`, `fusion-workbench/.guard-state/churn.json` und
`fusion-workbench/orchestrator-live.md`. Nach den Konventionen
(`rules/fusion-workbench-conventions.md`, Abschnitt `### Which of them a tracked workbench
tracks`) gehören genau diese vier nicht ins Repository; `.gitignore` hält bisher nur
`.commit-lock/` und `.session-marker` draußen. Im Augenblick meldet `git status` sechs geänderte
verfolgte Dateien, alle unter `fusion-workbench/`, und keine davon berührt den Code, der
ausgeliefert würde.

Die Folge: die Auslieferungsprüfung schlägt nach jeder Sitzung an, aus einem Grund, der mit dem
Bündel nichts zu tun hat. Zwei Wege stehen offen, und keiner ist meiner: die vier Dateien in
`.gitignore` aufnehmen und aus der Verfolgung nehmen, oder vor jedem Auslieferungslauf eintragen
beziehungsweise wegstellen, wie die Fehlermeldung es vorschlägt. Der Punkt ist defektförmig; ob er
einen Datensatz unter `shared/issues/` bekommt, entscheidest du. Der Playmaker legt keinen an.

**3. Der Ideenspeicher ist leer, und das begrenzt, was der nächste Lauf für dich tun kann.** Ein
Portfolio mit einem vorgesehenen Circle und null Ideen legt eine Rangfolge vor, die keine ist.
Wenn dir aus den acht Runden etwas im Kopf geblieben ist, das später eine Runde werden soll, kostet
es einen Titel und einen Absatz: `/fusion:memo`. Der nächste Lauf ordnet und empfiehlt daraus.

**4. Vier Nutzerfragen sind gebaut und trotzdem unbeantwortet.** Die Runde 7 ist auf den
Empfehlungen von vier offenen Fragen gefahren, der Baum hat je eine Möglichkeit umgesetzt, und
alle vier stehen weiter auf `_o_`:
`shared/decisions/260813-0053_*_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`,
`shared/decisions/260813-0053_*_wie-viele-obermenues-traegt-die-menueleiste-fuer-81-funktionen.md`,
`shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`
und
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`.
Nicht eine Antwort wartet auf ihre Umsetzung, sondern eine Umsetzung auf ihre Antwort. Die vierte
gehört zusätzlich vor die Aktivierung des Betrachters, weil sie den Weg eines Tastendrucks in
einer Web-Ansicht bestimmt. Insgesamt stehen 19 Entscheidungsdatensätze offen: 7 im
gemeinsamen Speicher, darunter diese vier, und 5 im Circle der Runde 1.

**5. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen
Entwicklungsbau.** `cargo xtask bundle` und `cargo xtask release` schreiben an dieselbe Stelle,
und über `bundle` hängen `run`, `run-terminal`, `tasten`, `menue`, `durchstich` und `frisch`. Ein
`make run` nach dem Auslieferungslauf löscht die Beglaubigung. Der Preisunterschied ist der Punkt:
ein Entwicklungsbündel ist in Sekunden wieder da, ein beglaubigtes verlangt zwei
Übersetzungsläufe im Profil `release`, `lipo`, eine Signatur mit gehärteter Laufzeitumgebung und
einen Netzlauf zu Apple. Der Datensatz ist offen:
`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`.
Die Runde 8 hat die Tag-Prüfung an denselben Weg gebaut, ohne den Defekt mitzunehmen. Er steht
also unverändert und jetzt an einem Weg, der eine Station mehr trägt.

**6. `CLAUDE.md` ist an fünf nachgezählten Stellen überholt, und eine davon ist inzwischen
sachlich falsch statt nur veraltet.** Die Datei nennt vier gefahrene Runden, gefahren sind acht.
Sie nennt 68 Varianten für `Kommando`, während `Kommando::KENNUNGEN` sechsundsiebzig Einträge
trägt (`crates/krk-core/src/tasten/belegung.rs:566`, am 260813-1510 nachgezählt). Sie führt zwei
vorgesehene Circles, die Statusleiste und den Web-Betrachter, und setzt die Statusleiste auf Rang
1; die Statusleiste ist seit dem 260812-0820 beschränkt abgeschlossen, vorgesehen ist allein der
Web-Betrachter.

Die fünfte Stelle ist die, die jetzt eine falsche Aussage trifft, und sie steht unter
„Projektstand": „Alle vier Runden sind als beschränkter Abschluss geschlossen … Das ist eine
Eigenschaft dieses Projekts und keine Häufung von Fehlschlägen. Wer eine Rangheuristik über die
Circles legt, die allein `_c_` als erfüllte Vorbedingung zählt, bekommt hier für jeden Kandidaten
dasselbe Kennzeichen und damit keine Auskunft." Der zweite Satz stimmt für den einen verbliebenen
Kandidaten weiterhin, aber aus einem anderen Grund (die Runde 1 ist terminal auf `_b_`), und der
erste stimmt seit dem 260813-1415 nicht mehr. `/fusion:revise-claude-md` ist der vorgesehene Weg.

**7. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge.

```
260804-0933-web-betrachter-…  ──>  260802-0842-krk-mac-…  (terminal, _b_)
```

Die einzige Circle-Kante verlässt die Menge der nicht terminalen Circles. Ein Zyklus besteht
nicht, und mit einem Knoten kann auch keiner entstehen.

**8. Parent grounding stale: ein Vermerk angehängt, obwohl die Auslösebedingung nicht erfüllt
ist.** Die Regel verlangt ein Kind auf beschränktem Abschluss (`_b_`) und ein Zitat dieses Kindes
im Abschnitt `## Grounding snapshot` des Elternteils. Die Runde 8 trägt `_c_`, und der Betrachter
kann sie nicht zitieren, weil sein Grounding vom 260804 stammt. Der Vermerk steht trotzdem am
Datensatz des Betrachters, weil die Runde 8 an der einen Regel gearbeitet hat, durch die jeder
seiner Befehle laufen wird: die Zulässigkeitsfrage fragt seit ihr vier Dinge statt drei
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs`, Struktur `Lage`, Tafel aus 280 Fällen).

Der Auslöser der Regel ist an dieser Stelle zu eng gefasst, und das gehört benannt: ein kohärenter
Abschluss bewegt den Baum mindestens so weit wie ein beschränkter. Der Markerbuchstabe sagt, ob
die Directive erreicht wurde, nicht ob sich der Boden unter einem vorgesehenen Circle verschoben
hat. Wer die Regel anders liest, sieht an dieser Stelle, worauf.

**9. Die Runde 8 lässt 16 offene Defekte in ihrem Circle zurück**, und ihre Abschlussnotiz nennt
zwei, die schwerer wiegen als die übrigen: die Aufruferzahl an `fokus` steht in Plan, Baum und
Durchsicht auf fünf und ist sechs, und die Diagrammbefunde an Spec und Plan sind nie behoben
worden, obwohl das Sitzungsprotokoll sie als erledigt mitführte. Neun Abnahmekriterien tragen die
Kennzeichnung `(Probe)` und haben keine. Projektweit stehen 70 Defekte offen, 10 davon im
gemeinsamen Speicher. Der Schwerpunkt liegt bei Prosa, die dem Code hinterherläuft.
