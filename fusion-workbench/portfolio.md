# Portfolio

**Generated:** 260813-0958 (by playmaker session 260813-0958-playmaker-direct-dispatch)
**Domain bias:** code

Neun Circles liegen unter `circles/`: keiner aktiv, **zwei** vorgesehen, sieben beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Seit dem Lauf vom 260813-0859 ist ein
Circle hinzugekommen, `260813-0939-titelleiste-fuehrt-version-und-semantische-tags`, aus dem
Ideeneintrag, den jener Lauf zum Shapen empfohlen hat. Der Ideenspeicher ist damit leer.

**Die Rangfolge kehrt sich mit diesem Lauf um.** Zum ersten Mal seit dem Abschluss der Runde 1
stehen zwei vorgesehene Circles nebeneinander, und der neue nimmt Rang 1. Der Betrachter, in vier
Läufen der einzige Kandidat und deshalb ohne Vergleichswert empfohlen, steht auf Rang 2.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen
zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

**Zur Rangheuristik, und warum sie weiterhin ausgesetzt ist.** Die Standardheuristik der
Domain-Gewichtung `code` bevorzugt Circles, deren Abhängigkeiten sämtlich kohärent abgeschlossen
sind (`_c_`). In diesem Projekt trägt kein einziger abgeschlossener Circle diesen Marker: alle
sieben gefahrenen Runden stehen auf beschränktem Abschluss (`_b_`), und alle sieben aus demselben
Grund. Der Abnahmelauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`,
offen seit dem 260806). Die Heuristik vergäbe jedem denkbaren Kandidaten dasselbe Kennzeichen und
träfe keine Unterscheidung. Sie ist deshalb **nicht in die Rangfolge eingerechnet**. Was die
Rangfolge stattdessen trägt, sind absolute Signale am einzelnen Datensatz: der Preis der Vorarbeit
vor dem Plan, die Zahl und die Art der noch zu klärenden Fragen, das Alter der Grundlage, und die
Menge der Vorbedingungen.

Eine Nebenbemerkung dazu, weil sie in diesem Lauf zum ersten Mal greift: der Rang-1-Kandidat hat
**keine** Circle-Vorbedingung. Für ihn ist die Prüfung leer erfüllt statt unentscheidbar, und das
ist eine echte Auskunft und nicht dieselbe Aussetzung noch einmal.

## Active (_t_)

**(keiner)**

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Datensatz trägt `_t_`. Das ist der
reguläre Zustand nach einem Abschluss und keine Warnung.

## Anticipated (_a_) — ranked

Recommended next: `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` — die Vorarbeit
ist eine Klärungsrunde über drei schmale Fragen, keine Untersuchung, und der Circle hat keine
Vorbedingung an einem anderen Circle.

```
/fusion:next 260813-0939-titelleiste-fuehrt-version-und-semantische-tags
```

### Rang 1: `260813-0939-titelleiste-fuehrt-version-und-semantische-tags`

**Directive.** Die Titelleiste trägt links einen eigenen Bereich mit Namen und Version, geschrieben
als `KRK 0.1.0`; der absolute Pfad bleibt mittig und ungekürzt. Verbindlich wird die Zahl durch
semantische Versionstags: ein Git-Tag `v<version>` je Auslieferung, ein Abschnitt in `README.md`
über die Stufen, und ein Abbruch in `cargo xtask release`, wenn HEAD keinen passenden Tag trägt.
Den Tag setzt der Nutzer, nicht das Werkzeug.

**Abhängigkeiten.** Keine auf einen anderen Circle. Vier Stellen binden inhaltlich: die Fähigkeit
C11 der Runde 2, die einzige bestehende Zusage über die Titelleiste
(`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md`);
die Runde 7, deren Menü keinen Eintrag „Über KRK" führt; und die beiden offenen Defekte am
Auslieferungsweg
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`
und
`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`).

**Warum Rang 1.** Der Unterschied zum Betrachter ist der Preis der Vorarbeit vor dem Plan, und er
ist der einzige, der in diesem Projekt noch unterscheidet. Der Betrachter hält selbst fest, dass
das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene Untersuchung gehört. Für
diesen Circle ist das Mittel mit der Klärungsrunde vom 260813 bereits eingegrenzt: ein neues Modul
unter `crates/krk-ui/src/appkit/`, vom Nutzer ausdrücklich akzeptiert, dessen Klasse der Planner
am Baum entscheidet. Dazu kommt das Alter der Grundlage. Der Abschnitt `## Grounding snapshot`
dieses Datensatzes ist vier Stunden alt, und vier seiner Tatsachenaussagen sind bei diesem Lauf
gegen den Baum gelesen worden: `git tag -l` liefert null Tags, die Version steht einquellig in
`[workspace.package]` der Wurzel-`Cargo.toml` auf `0.1.0`, `NSTitlebarAccessoryViewController`
kommt unter `crates/` nicht vor, und `crates/krk-ui/src/appkit/mod.rs` führt 27 Modulnamen. Alle
vier halten. Die drei offenen Fragen sind schmal, tragen je einen Datensatz in `decisions/` des
Circles und verlangen keine Messung. Die zweite ist die dringlichste: nach dieser Runde bricht
`cargo xtask release` ab, solange HEAD keinen passenden Tag trägt, und das Werkzeug darf ihn nicht
selbst erzeugen. Ohne eine Festlegung, wer `v0.1.0` setzt, ist der Auslieferungsweg ab dem
Abschluss der Runde abweisend.

**Was dagegen spricht.** Die Tag-Hälfte sitzt am selben Weg wie der offene Defekt `260813-0026`:
`cargo xtask bundle` und `cargo xtask release` schreiben beide nach `target/KRK.app`, und ein
Entwicklungsbau löscht die Beglaubigung. Die Runde fasst diesen Weg ohnehin an. Der
Aktivierungs-Spec sollte sagen, ob der Defekt hereingeholt wird oder ausdrücklich draußen bleibt.
Kleiner, aber notiert: die dritte Antwort der Klärungsrunde schwächt die Begründung ab, mit der
der Ideeneintrag Anzeige und Tags aneinanderbindet, weil ohne Kennzeichnung des Arbeitsstands
jeder Bau aus einem geänderten Baum dieselbe Zahl zeigt wie das ausgelieferte Bündel. Die Kopplung
bleibt eine Nutzerfestlegung; der Restpunkt gehört in den Spec.

### Rang 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive.** KRK zeigt eine Web-Adresse in einem eigenen Betrachter statt sie an den
Systembrowser abzugeben, in einem gewöhnlichen Tab des Vorschaufensters, bedient über die Tastatur
mit Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten.** Eine auf einen Circle: die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`, beschränkt abgeschlossen). Der Circle erweitert
deren Grenze und ist keine spätere Runde davon.

**Warum Rang 2.** Vor der Aktivierung stehen sechs Fragen und eine Untersuchung des
Darstellungsmittels; die Untersuchung ist der teurere Posten und der Grund für den zweiten Rang.
Eine der sechs ist kein Punkt dieses Circles, sondern der offene projektweite Nutzerentscheid
`shared/decisions/260813-0053_*_schluckt-der-abgriff-den-zulaessigen-befehl-oder-den-ausgefuehrten.md`,
der bestimmt, welchen Weg ein Tastendruck in einer Web-Ansicht nimmt. Die geerbten Bauteile stehen
unverändert am Baum: die Auswertung der Zwischenablage, das Vorschaufenster mit seiner Tableiste,
die Statuszeile über die volle Fensterbreite, und der Befehl auf `opt+cmd+g`. Der Rangwechsel ist
kein Befund gegen diesen Circle. Es ist der erste Lauf, in dem ihm überhaupt etwas
gegenübersteht.

## Backlog — ranked

**(keiner)**

`shared/backlog/` trägt einen Eintrag, und der steht auf `_c_`: `260813-0822_*_titelleiste-fuehrt-name-und-version.md`
ist mit der Anlage des Rang-1-Circles geschlossen. Kein Eintrag trägt `_o_` oder `_p_`. Die
Empfehlung des Laufs vom 260813-0859 ist damit ausgeführt, und der Speicher ist leer.

Ideen legst du mit `/fusion:memo` ab; ein Titel und ein Absatz genügen. Kein Agent schreibt in
diesen Speicher, der Playmaker liest und ordnet ihn nur.

## Recently closed (_c_ / _b_)

Die letzten fünf, neueste zuerst. Alle tragen `_b_`, keiner trägt `_c_`.

1. **`260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`** (Runde 7, `_b_`,
   260813). Die Belegungsansicht wird durch Tippen durchsucht, alle Funktionen stehen im Menü in
   neun Obermenüs mit Kürzel und Ausgrauung, und `opt+cmd+n` startet eine weitere Instanz, die
   sich die Ablage über zwei `flock`-Sperren mit der ersten teilt. Fünfzehn Planschritte gebaut,
   18 Commits, 1003 Proben über 19 Ziele. Nicht abgenommen.

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
(`260802-0842-krk-mac-dateimanager-editor-git`, `_b_`, 260807-1035). Die Runde 2 hält C11, die
Zusage über den Fenstertitel, die der Rang-1-Circle fortschreiben würde. Die Runde 1 hält die zehn
Zeitzusagen aus C8 und die Frage nach dem Vordergrund, an der jeder Abschluss dieses Projekts
hängt.

## Archived (_s_ / _d_)

**(keiner)**

Kein Circle-Datensatz trägt `_s_` (überholt) oder `_d_` (zurückgestellt).

## Warnings

Keine Warnung hält etwas auf. Alle sieben unten sind Auskünfte für deine nächste Entscheidung.

**1. Der Abnahmelauf steht für zwei Runden aus, und die Frage darunter ist die älteste offene des
Projekts.** Weder die Runde 6 noch die Runde 7 ist abgenommen. Die eine Frage, an der das hängt,
wie KRK für den Abnahmelauf in den Vordergrund kommt, steht seit dem 260806 offen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Solange sie offen ist, endet jede weitere Runde beschränkt, unabhängig davon, wie gut sie gebaut
ist. Es ist die einzige Frage im Bestand, deren Beantwortung die Abschlussart künftiger Runden
ändert.

**2. Das beglaubigte Bündel unter `target/KRK.app` überlebt keinen gewöhnlichen
Entwicklungsbau, und der empfohlene Circle arbeitet an genau diesem Weg.** `cargo xtask bundle`
und `cargo xtask release` schreiben an dieselbe Stelle, und über `bundle` hängen `run`,
`run-terminal`, `tasten`, `menue`, `durchstich` und `frisch`. Ein `make run` nach dem Release-Lauf
löscht die Beglaubigung. Der Preisunterschied ist der Punkt: ein Entwicklungsbündel ist in
Sekunden wieder da, ein beglaubigtes verlangt zwei Übersetzungsläufe im Profil `release`, `lipo`,
eine Signatur mit gehärteter Laufzeitumgebung und einen Netzlauf zu Apple. Der Datensatz ist
offen:
`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`.
Die Tag-Prüfung des Rang-1-Circles kommt als weitere Station an denselben Weg. Der
Aktivierungs-Spec sollte entscheiden, ob der Defekt mitgenommen wird.

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

**4. `CLAUDE.md` ist an vier nachgezählten Stellen überholt, und eine davon ist neu.** Die Datei
nennt vier gefahrene Runden, gefahren sind sieben. Sie nennt 68 Varianten für `Kommando`, während
`Kommando::KENNUNGEN` sechsundsiebzig Einträge trägt
(`crates/krk-core/src/tasten/belegung.rs:566`, am 260813-0958 nachgezählt); der offene Datensatz
`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`
kennt den Fehler, nennt aber selbst 75 und ist mit der Runde 7 ebenfalls zurückgefallen. Neu ist
die dritte Stelle: der Abschnitt „Zwei Circles sind vorgesehen und nicht gefahren" nennt die
Statusleiste (`260811-1304-…`), die seit dem 260812-0820 beschränkt abgeschlossen ist, und den
Web-Betrachter. Die Zahl zwei stimmt seit heute wieder, die Namen nicht: vorgesehen sind der
Web-Betrachter und die Titelleiste. Vierte Stelle: derselbe Abschnitt setzt die Statusleiste auf
Rang 1. `/fusion:revise-claude-md` ist der vorgesehene Weg.

**5. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat zwei
Knoten und keine Kante innerhalb dieser Menge.

```
260813-0939-titelleiste-…            (keine Circle-Kante)
260804-0933-web-betrachter-…  ──>  260802-0842-krk-mac-…  (terminal, _b_)
```

Die einzige Circle-Kante des Betrachters verlässt die Menge der nicht terminalen Circles. Ein
Zyklus besteht nicht.

**6. Parent grounding stale: kein Vermerk in diesem Lauf, und der Grenzfall wird offen benannt.**
Seit dem 260813-0714 ist kein Circle auf beschränkten Abschluss gegangen. Der neue Circle
`260813-0939-titelleiste-fuehrt-version-und-semantische-tags` nennt in seinem Abschnitt
`## Dependencies` allerdings die Runde 7, die beschränkt abgeschlossen ist, und das sieht nach der
Auslösebedingung aus. Er ist nicht angehängt worden, aus zwei Gründen. Erstens zitiert der
Abschnitt `## Grounding snapshot` weder den Verzeichnisnamen der Runde 7 noch den Artefakt ihrer
Abschlussnotiz; die Kante steht unter `## Dependencies`, wo die Regel nicht greift. Zweitens, und
das trägt schwerer: der Circle ist am 260813-0939 angelegt worden, also nach dem Abschluss der
Runde 7, und seine Grundlage liest den Baum in dem Zustand, den jene Runde hinterlassen hat. Ein
Vermerk „deine Grundlage ist gealtert" wäre an dieser Stelle unwahr. Der Vermerk vom 260813-0714
im Datensatz des Betrachters gilt unverändert fort; er nennt vier Feststellungen, darunter den
gestiegenen Eintrittspreis pro Befehl seit der gerechneten Menüleiste und die `flock`-Sperre mit
elf benannten Löchern
(`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0716_*_die-bewachte-luecke-ist-nicht-die-luecke-elf-schreibwege-an-der-sperre-vorbei-bleiben.md`,
offen).

**7. Am Datensatz des Betrachters hängt dieser Lauf wieder nichts an, und jetzt aus einem zweiten
Grund.** `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md` trägt
674 Zeilen, davon acht Playmaker-Abschnitte aus vier Läufen. Der Lauf vom 260813-0859 hat aus
diesem Grund keinen fünften Vorschlagsblock angehängt. Jetzt kommt hinzu, dass der Circle nicht
mehr Rang 1 ist: ein Aktivierungsvorschlag gehört nach der Regel an den empfohlenen Kandidaten,
und der ist seit diesem Lauf ein anderer. Der neue Rang-1-Circle hat seinen ersten
`## Activation proposal` erhalten.
