# Portfolio

**Generated:** 260818-1018 (by playmaker session 260818-1018-playmaker-orchestrator-phase4)
**Domain bias:** code

---

**Was ansteht.** Die zwölfte Runde ist gefahren und als erste dieses Projekts ohne Abnahmelauf
kohärent geschlossen. Kein Circle ist aktiv, und die Wahl der nächsten Runde steht zwischen zwei
Kandidaten, von denen nur einer vorgesehen ist. Vorgesehen ist der Web-Betrachter im
Vorschaufenster, seit dem 260804 und seither zwölf Runden lang nicht dran; er braucht vor der
Aktivierung eine Untersuchung des Darstellungsmittels und eine Klärungsrunde über drei Fragen.
Daneben liegt die zurückgestellte Runde `260816-2255-befehle-absetzen-und-makros-speichern` mit
fertigem Spec und fertigem Plan, an der nichts gebaut ist und die nur nicht dran war; sie
aufzunehmen heißt, einen neuen Circle anzulegen, denn zurückgestellt ist ein Endzustand. Im
Ideenspeicher steht unverändert eine lebende Idee, das zweite Kürzel für den Editor-Einstieg,
und sie ist die kleinste der drei Runden.

Seit dem Lauf vom 260815-0350 haben sich vier Dinge bewegt: die Runde 11 hat den Dateifilter auf
den Inhalt ausgeweitet, die Runde 12 hat jeden Löschweg mit einer Rückfrage versehen und das
endgültige Löschen abgeschafft, eine geplante Runde ist zurückgestellt worden, und die Zahl der
offenen Befunde ist von 99 auf 133 gestiegen. Drei Abnahmeläufe stehen jetzt aus statt zweien.

---

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt aktiv
(`_t_`). Beides zusammen ist der reguläre Zustand nach einem Abschluss; es steht dazu keine
Warnung. Die Runde 12 ist am 260818 kohärent geschlossen worden.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — einziger
vorgesehener Circle, alle Vorbedingungen am Baum gebaut, ein einziger offener
Entscheidungsdatensatz bindet ihn; vor der Aktivierung stehen eine Untersuchung und eine
Klärungsrunde, und eine Zeile seines Datensatzes ist zu berichtigen.

### Rang 1 — der eingebaute Web-Betrachter

`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`

**Directive in einem Satz.** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, der in
einem gewöhnlichen Tab des Vorschaufensters lebt, über die Tastatur bedient wird und Verweisanker
auf jedem sichtbaren Link trägt; `Opt+Cmd+G` öffnet die Adresse aus der Zwischenablage danach in
KRK statt im Systembrowser.

Der Circle steht als einziger Kandidat auf Rang 1, und die Reihenfolge ist damit keine Leistung
der Rangheuristik. Was die Heuristik beiträgt, ist die Prüfung der Vorbedingungen, und die fällt
weiterhin sauber aus. Ein einziger offener Entscheidungsdatensatz bindet ihn, und sein
Grounding-Abschnitt ordnet die Bindung selbst als Schluss und nicht als Feststellung ein:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`
fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt. Seine
Abhängigkeiten führen auf die Runden 1, 5, 6 und 7, alle terminal und alle am Baum gebaut; die
Zwischenablage-Auswertung aus Schritt S13 und das Vorschaufenster aus Schritt S19, auf denen er
aufsetzt, stehen. Für die Gewichtung `code` wäre der beschränkte Abschluss der Abhängigkeiten ein
Abzug, wenn allein kohärent (`_c_`) als erfüllte Vorbedingung zählte; in diesem Projekt ist er
nicht angesetzt, und die Begründung steht unten unter `## Warnings`, Punkt 1.

**Was die Runden 11 und 12 daran verändert haben.** Der Vermerk vom 260818-1018 am Datensatz
führt es aus; drei Punkte gehören in die Kurzfassung. Erstens gilt das Halteverhalten des Tabs
aus C6, auf das die Directive sich beruft, seit der Runde 11 nicht mehr für **laufende** Arbeit:
ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, und der Rückwechsel stößt ihn nicht
wieder an. Das Laden einer Web-Seite ist laufende Arbeit, und der Aktivierungs-Spec muss eine
Antwort wählen, statt sich auf C6 zu berufen. Zweitens trifft die zweite Möglichkeit der ersten
offenen Frage — eine Adresseingabe als Blatt am Fenster — seit der Runde 12 auf einen Blattbauer,
an dem eine Nutzerfrage offen steht und ein Befund liegt; ein Adressblatt hätte eine ausführende
erste Schaltfläche und wäre genau der Fall, den der Befund als noch nicht eingetreten beschreibt.
Drittens ist die Statuszeile bei sechs Rängen geblieben, nachgezählt am 260818: der siebte für
die Meldungen des Betrachters ist nicht teurer geworden.

**Was die Runde 12 aufgehoben hat, berührt ihn nicht.** Sie hat eine umgesetzte Nutzerfestlegung
der Runde 1 überholt und deren Datensätze nachgezogen. Alle drei Stellen der Runde 1, die dieser
Circle unter `## Dependencies` zitiert, sind unberührt und lösen auf, der Ausschluss
„Integrierter Browser zum Navigieren von Websites" eingeschlossen. Das steht hier, damit niemand
danach sucht.

**Eine Zeile seines Datensatzes ist zu berichtigen, bevor er aktiviert wird.** Zeile 438 zitiert
einen Namensteil, den es nie gegeben hat; abgelegt als
`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`.
Sie steht im Abschnitt `## Grounding snapshot`, der bei der Aktivierung als bindende Grundlage
gelesen wird, und begründet dort eine Zahl zur Zusage L7. Der Playmaker berichtigt sie nicht: er
schreibt an Circle-Datensätzen nur die drei Abschnitte, die sein Auftrag nennt.

Kein weiterer Circle trägt vorgesehen (`_a_`). Der zweite Kandidat für die nächste Runde steht
unter `## Archived` und ist zurückgestellt, nicht vorgesehen; er wird deshalb hier nicht mitgerankt.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— eine Idee, kein Split nötig, ihre selbstgestellte Vorbedingung ist beantwortet, und die Runde 9
hat für denselben Fall den Präzedenzfall gebaut.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

### Rang 1 — das zweite Kürzel für den Editor-Einstieg

`shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`

Eine zweite, besser erreichbare Tastenkombination für `bearbeiten`, das heute allein auf `F4`
liegt. Genau eine Idee, kein Split vorzuschlagen; der Eintrag steht seit dem 260814-1513 auf
empfohlen (`_p_`) und bleibt es.

Die Vorbedingung, die der Eintrag selbst mitbringt, ist beantwortet, und die Idee löst sich dabei
nicht auf. Der Eintrag vermutet, `F4` sei nur deshalb hakelig, weil die Systemeinstellung „F1, F2
usw. als Standard-Funktionstasten verwenden" ausgeschaltet ist. Gemessen am 260802-1137 auf dem
Abnahmegerät, mit der Einstellung ausgeschaltet, kommen `fn+F3`, `fn+F5` und `fn+F8` als
gewöhnliche `keyDown`-Ereignisse an, und KRK kann eine gehaltene `fn`-Taste gar nicht von einer
nackten Funktionstaste unterscheiden
(`shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md`, Nachtrag 260802-1409,
Beleg `spikes/fn-tasten/messung-A.txt`). Derselbe Nachtrag sagt zu, jede Funktion der Norton-Reihe
trage ab Werk zusätzlich ein Cmd-Kürzel, und die Runde 9 hat genau diesen Schritt zuletzt getan:
`notizzettel` liegt auf `f2` **und** `cmd+k`.

Was eine Klärungsrunde zu tragen hätte, ist die Wahl der Kombination, und die ist eng: alle vier
Cmd-Ebenen von `e` sind vergeben, `cmd+e` auf `editor_aus_vorschau`, `shift+cmd+e` auf
`fokus_editor`, `opt+cmd+e` auf `editor_schliessen`, `ctrl+cmd+e` auf `editor_ansicht_umschalten`.
Die Belegung trägt am 260818 nachgezählt 84 Einträge und `Kommando` 78 Varianten, in der Summe
unverändert seit dem 260815; die Zusammensetzung hat sich bewegt, und **eine Kombination ist neu
frei geworden**: `opt+cmd+delete`, seit dem Wegfall des endgültigen Löschens am 260817. Sie
gehört nicht zur `e`-Familie, ist also keine Antwort, aber sie zeigt, dass der Spielraum sich
bewegt. Dazu dieselbe Vorbedingung wie beim Web-Betrachter: eine neue Kombination erreicht keinen
Nutzer, der seit der Runde 7 einmal eine Taste zugewiesen hat
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

Der Eintrag ist keine Dublette zum offenen Defekt
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Defekt betrifft dasselbe `F4` mit einem anderen Symptom und bliebe bestehen, gleich welche
zweite Kombination hinzukommt.

**Performed this run:**

Keine. Dieser Lauf ist eine Phase-4-Zuteilung des Orchestrators, hält keine Bestätigung und führt
deshalb keine der vier bestätigungspflichtigen Operationen aus. Es ist auch keine
vorgeschlagen: der eine lebende Eintrag trägt genau eine Idee, es gibt keine zweite, mit der er
zusammenzulegen wäre, seine Idee ist weiter lebendig, und ein Zurückstellen wäre eine Verfügung
über sie, die dem Nutzer zusteht. Die einzige autonome Schreibweise, die Rangumbenennung zwischen
`_o_` und `_p_`, war nicht anzuwenden: der Eintrag steht bereits auf `_p_` und bleibt Rang 1.

Zwei Einträge stehen auf geschlossen und nennen im Rumpf den Circle, der aus ihnen wurde:
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` (Runde 8) und
`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
(Runde 9).

Ein Teil des empfohlenen Eintrags ist defekt- und nicht ideenförmig. Er steht unter
`## Warnings`, Punkt 5; der Playmaker legt dafür keinen Datensatz an.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss in einem Satz |
|---|---|---|
| `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` | `_c_` | Kohärent am 260818: KRK kennt genau einen Löschweg, jeder Vorgang fragt einmal nach mit „Abbrechen" vorbelegt, ungewöhnliche Ziele und große Umfänge tragen ein Warnzeichen, das endgültige Löschen ist aus Anwendung, Belegung und Menü gefallen; ausgelöst von einem Schadensfall am eigenen Projektverzeichnis. |
| `260816-1321-inhaltsfilter-mit-ankreuzfeld-content` | `_b_` | Beschränkt am 260816-2030: der Dateifilter berücksichtigt den Inhalt, eingeschaltet über das zehnte Ankreuzfeld „Content", gelesen nur Text und nur bis 1 MB; gebaut und am Baum nachgelesen, nicht am Bündel abgenommen. |
| `260814-1551-tippen-filtert-dateiliste-flach-und-tief` | `_b_` | Beschränkt am 260815: Tippen filtert die Dateiliste an beliebiger Stelle des Namens, der Filtertext gehört dem Tab, ein neuntes Ankreuzfeld „Deep" weitet ihn auf den Unterbaum; zehn der 77 Kriterien mit Bündelanteil unabgenommen. |
| `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | `_b_` | Beschränkt am 260814-1300: Notizzettel als zehntes Blatt, zwei Zettel als Tabs, `f2` und `cmd+k` hin, `Esc` zurück; 16 der 29 Kriterien mit Bündelanteil hat keine Beobachtung angefasst. |
| `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | `_c_` | Kohärent am 260813-1415: Titelleiste mit Name und Version, Über-Dialog, semantische Versionstags und eine vierte Bedingung in der Zulässigkeitsregel. Die einzige Runde, deren Abnahmelauf der Nutzer selbst gefahren hat. |

Ältere Abschlüsse: `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz`
(beschränkt am 260813), `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern`
(beschränkt am 260812), `260811-1304-statusleiste-mit-bereichsschaltern` (beschränkt am
260812-0820), `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (beschränkt am 260811-2210),
`260809-2040-tastenbelegung-als-markdown-in-downloads` (beschränkt am 260811-1415),
`260807-2116-eingebauter-editor-mit-textmarken` (beschränkt am 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (beschränkt am 260807-1035).

Zwölf Runden sind gefahren: zehn beschränkt, zwei kohärent. **Die beiden kohärenten tragen ihren
Marker aus verschiedenen Gründen**, und der Unterschied steht unter `## Warnings`, Punkt 1.

## Archived (_s_ / _d_)

| Circle | Marker | Stand |
|---|---|---|
| `260816-2255-befehle-absetzen-und-makros-speichern` | `_d_` | Zurückgestellt am 260817-0445 zugunsten der zwölften Runde. Nichts ist gebaut, und die Directive ist erreichbar; die Runde war nur nicht dran. Vollständig hinterlassen: Spec mit 54 Abnahmekriterien (`shared/planning/260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md`), Plan mit 22 Schritten in fünf Bündeln (`circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md`), zwei Entscheidungen und ein Befund mit Messung. |

Kein Circle-Datensatz trägt überholt (`_s_`).

**Zurückgestellt ist ein Endzustand.** Ein `mv` zurück auf vorgesehen ist unzulässig; wer diese
Runde aufnehmen will, legt einen neuen Circle an, der den zurückgestellten über `## Dependencies`
zitiert und dessen Spec und Plan übernimmt (`rules/circle-records.md`, `### Worked transitions`).
Das ist der einzige Kandidat für die nächste Runde, der nicht vorgesehen ist und deshalb im
Ranking oben nicht vorkommt — genannt, damit die Wahl dem Nutzer offensteht.

## Warnings

**1. Die Rangheuristik wertet `_b_` und `_c_` in diesem Projekt gleich, und seit dem 260818 ist
auch `_c_` nicht mehr eindeutig.** `CLAUDE.md` hält fest, dass zehn der zwölf Runden beschränkt
schließen und immer aus demselben Grund: der Abnahmelauf der zehn Zeitzusagen verlangt KRK im
Vordergrund und ist Nutzerarbeit, die kein Agent fahren kann. Der Marker misst dort die
Verfügbarkeit des Nutzers und nicht die Reife der Runde, und eine Heuristik, die allein `_c_` als
erfüllte Vorbedingung zählt, gäbe hier eine irreführende Auskunft. Dieser Lauf hat den Abzug
deshalb nicht angesetzt, wie alle Läufe davor. **Neu ist die andere Richtung:** die Runde 12
trägt `_c_`, ohne dass der Abnahmelauf gefahren wäre — ihre Abschlussnotiz sagt es selbst und
begründet es damit, dass ihre Directive über die zehn Zeitzusagen nichts aussagt. Wer `_c_` als
„vom Nutzer abgenommen" verrechnet, liest sie falsch; das trifft allein auf
`260813-0939-titelleiste-fuehrt-version-und-semantische-tags` zu.

**2. `CLAUDE.md` beschreibt ein Projekt mit zehn Runden, und der Dateibestand trägt zwölf.** Die
Zeile „Zehn Runden sind gefahren" und die Tabelle darunter kennen die Runden 11 und 12 nicht. Der
Absatz `## Projektstand` ist auf den 260815-0600 datiert und nennt die Auslieferung als `v0.4.1`,
während `Cargo.toml` `0.5.1` führt. Die Datei sagt selbst, verbindlich sei der Dateibestand und
nicht die Zeile, und dieser Selbstschutz hält den Fehler klein; er nimmt ihn nicht weg. Was die
Runde 12 an inhaltlichen Aussagen berührt hat, ist nachgezogen — der Absatz über die
Rückschritt-Taste nennt die Rückfrage seit dem 260817 richtig. Ein Lauf über `/fusion:curate`
schließt die Lücke im Verweisregister.

**3. Drei Abnahmeläufe stehen aus, und alle drei sind Nutzerarbeit.** Die Runde 11 hat ihre Liste
fertig hinterlassen (`messungen/260816-abnahme-inhaltsfilter.md`, 28 Beobachtungen an vier Orten
mit Handgriff und erwartetem Ergebnis). Die Runde 10 hat zehn ihrer 77 Abnahmekriterien mit
Bündelanteil offen gelassen, vier davon sicherheitsrelevant
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`).
Die Runde 9 hat 21 Kriterien ohne vollen Beleg gelassen. Der Marker bewegt sich davon bei keiner:
beschränkt ist ein Endzustand. Was die Läufe einbringen, ist der Beleg, nicht der Buchstabe.

**4. Der Abnahmelauf der zehn Zeitzusagen ist seit dem 260810-1918 nicht mehr gefahren.** Die
Abschlussnotiz der Runde 12 zählt sieben Runden ohne Messung. Der letzte Lauf ist der erste
vollständig saubere gewesen, alle zehn Zusagen in allen fünf Durchgängen. Daran hängt eine zweite
Sache: der zurückgestellte Datensatz
`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`
wartet auf weitere Läufe an verschiedenen Tagen und fällt als zurückgestellter aus jeder Suche
nach aktiver Grundlage heraus. Wird nie wieder gemessen, ist „bei 65 bleiben" der Sache nach
entschieden, ohne dass es jemand aufgeschrieben hätte.

**5. Der empfohlene Ideeneintrag beschreibt zur Hälfte einen Defekt, und der Playmaker legt
keinen an.** Der Nutzerentscheid vom 260802-1409 sagt zu, jede Funktion der Norton-Reihe trage ab
Werk zusätzlich ein Cmd-Kürzel, und nennt „F4 Bearbeiten" unter seinen sechs. Der Kommentar an
`bearbeiten` in `resources/default-keymap.toml` begründet die Abweichung damit, die Zwei-Wege-Regel
gelte den fünf Funktionen „ganz oben". Beide Aussagen zusammen gehen nicht auf. **Die Runde 12 hat
den Kopf dieses Blocks angefasst** und ihn von „sechs" auf „die ersten fünf" gezogen; der
Widerspruch zum Nutzerentscheid ist damit sauberer aufgeschrieben, aber nicht aufgelöst. Entweder
ist der Kommentar eine unbelegte Umdeutung eines umgesetzten Nutzerentscheids, dann ist es ein
Defekt, oder der Nutzer hat `bearbeiten` bewusst herausgenommen, dann fehlt der Datensatz dazu.
Die Entscheidung gehört dir; der Playmaker schreibt weder in den Ideen- noch in den Defektspeicher.

**6. Der Datensatz des empfohlenen Circles trägt einen toten Zeiger in seiner Grundlage.**
Zeile 438 zitiert einen Namensteil, den es nie gegeben hat
(`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`).
Das Zitat steht bereits in der Sternform; die Sternform hält gegen einen Markerwechsel und gegen
nichts sonst. Es ist der dritte Beleg desselben Fehlertyps in zwei Tagen und der erste außerhalb
der zwölften Runde. Die Berichtigung ist eine Zeile und gehört vor die Aktivierung.

**7. Die Auslieferungssperre steht wieder offen.** Am 260818 geprüft: `git tag --points-at HEAD`
liefert nichts, 21 Commits liegen zwischen `v0.5.1` und `HEAD` (`563c17b`), elf davon fassen
`crates/`, `resources/` oder `xtask/` an. `Cargo.toml` führt `0.5.1`. Station 1 von
`cargo xtask release` vergleicht Tag und Version und hält den Weg an. Der Zustand kehrt nach jeder
Runde zurück, die Commits hinzufügt und keinen Tag setzt; der Tag ist Nutzerarbeit.
`cargo xtask bundle` und `make check` hängen nicht daran.

**8. 133 Defekte sind offen**, 33 davon im gemeinsamen Speicher. Gegenüber dem 260815-0350 sind
das 34 mehr. Sieben stammen aus der Runde 12 und stehen in ihrem Speicher; ihre Abschlussnotiz
ordnet sie ein: keiner ist ein Release-Blocker, zwei sind mittel. Die Liste liefert:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**9. 29 Entscheidungsdatensätze sind offen, zwölf sind beantwortet und nicht umgesetzt.**
Gegenüber dem 260815-0350 sind das fünf offene und drei beantwortete mehr. Vier der offenen sind
in der Runde 12 entstanden, drei in ihrem Speicher und eine im gemeinsamen. Keine offene Frage
hält einen Planschritt auf; alle binden künftige Arbeit. Zwei davon binden den empfohlenen Circle
mittelbar, weil sie den Blattbauer betreffen, den eine Adresseingabe benutzen würde. Die Liste
liefert:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**10. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge: der Web-Betrachter ist der einzige nicht terminale
Circle, und seine vier Kanten führen auf die Runden 1, 5, 6 und 7, die alle terminal sind. An
keinen Circle-Datensatz ist eine `## Dependency warning` angehängt worden.

**11. Ein neuer Vermerk zu gealterter Grundlage, für zwei Runden zusammen.** Der Datensatz des
Web-Betrachters hat einen `## Parent grounding stale` vom 260818-1018 bekommen, der die Runde 11
(beschränkt) und die Runde 12 (kohärent) in einem Abschnitt behandelt statt in zweien. Die
Auslösebedingung ist wie bei allen Vermerken davor zur Hälfte erfüllt: sein Grounding-Abschnitt
stammt vom 260804 und zitiert keine der beiden, wohl aber Bauteile, die beide angefasst haben.
Fünf Punkte stehen darin; die zwei, die neue Arbeit bedeuten, sind das nicht mehr eindeutige
Halteverhalten des Tabs und der Blattbauer unter offenen Fragen.

**12. Der Datensatz des Web-Betrachters trägt 1208 Zeilen und achtzehn Playmaker-Abschnitte aus
zehn Läufen**, zehn Aktivierungsvorschläge und acht Vermerke zu gealterter Grundlage. Die Länge
wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne dass an ihm gearbeitet würde.
Die zwei Abschnitte dieses Laufs tragen allein die Änderungen nach. Wer den Stand lesen will,
liest die letzten beiden, nicht alle achtzehn.
