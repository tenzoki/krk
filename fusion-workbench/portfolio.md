# Portfolio

**Generated:** 260815-0350 (by playmaker session 260815-0350-playmaker-user-fusion-next)
**Domain bias:** code

---

**Was ansteht.** Die zehnte Runde ist gefahren und beschränkt geschlossen, und damit stehen jetzt
zwei Abnahmelisten aus statt einer. Beide sind billig und beide sind Nutzerarbeit: rund zwanzig
Minuten für die Runde 9 und rund fünfundzwanzig für die Runde 10, je am laufenden Bündel im
Vordergrund. Keine bewegt einen Marker, denn beschränkt ist ein Endzustand; was sie einbringen,
ist die Auskunft, ob Notizzettel und Filter halten, was sie zusagen. Daneben stehen unverändert
zwei Kandidaten für die nächste Runde: die eine Idee im Speicher, ein zweites Kürzel für den
Editor-Einstieg, und der vorgesehene Web-Betrachter. Die Idee ist die kleinere Runde.

Seit dem Lauf vom 260814-1513 hat sich eine Sache bewegt, und sie ist die größte seit Tagen: die
Runde 10 hat das Tippen im Dateifenster von einer Sprungmarke zu einem Filter gemacht, die eine
Statuszeile um einen sechsten Rang erweitert und `Esc` eine dritte Bedeutung gegeben. `CLAUDE.md`
weiß davon noch nichts.

---

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt aktiv
(`_t_`). Beides zusammen ist der reguläre Zustand nach einem Abschluss; es steht dazu keine
Warnung. Die Runde 10 ist am 260815 beschränkt geschlossen worden.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — einziger
vorgesehener Circle, alle Vorbedingungen am Baum gebaut, und der Zuschnitt hat sich seit dem
260804 nicht bewegt.

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
fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt. Ein Betrachter
spricht eine Systemschnittstelle an, deren Umfang sich zwischen macOS 15 und macOS 26
unterscheidet; gemessen ist das nicht. Die übrigen offenen Fragen des Projekts binden ihn nicht.
Seine Abhängigkeit führt auf die Runde 1, und die trägt beschränkten Abschluss (`_b_`). Für die
Gewichtung `code` wäre das ein Abzug, wenn allein kohärent (`_c_`) als erfüllte Vorbedingung
zählte. In diesem Projekt zählt es nicht: `CLAUDE.md` hält seit der Kuratierung vom 260814-1405
ausdrücklich fest, dass `_b_` hier die Verfügbarkeit des Nutzers für den Abnahmelauf misst und
nicht die Reife einer Runde. Die Zwischenablage-Auswertung aus Schritt S13 und das Vorschaufenster
aus Schritt S19, auf denen der Betrachter aufsetzt, stehen beide am Baum.

Vor der Aktivierung stehen zwei Arbeiten, und keine davon ist eine Formalie. Erstens eine
Untersuchung des Darstellungsmittels: der Circle legt bewusst nicht fest, womit KRK Web-Inhalt
darstellt, weder eine Systemschnittstelle noch eine fremde Kiste. Zweitens eine Klärungsrunde über
die drei Fragen im Datensatz, von denen die erste den Zuschnitt entscheidet: Welche Quellen dürfen
die Adresse setzen? Bleibt es bei der Zwischenablage und den Verweisankern der Seite, bekommt KRK
einen Betrachter; kommen Adresseingabe und gespeicherte Web-Adressen hinzu, bekommt es einen
Browser.

**Was die Runde 10 daran verändert hat, und es ist mehr als bei den Runden davor.** Der Vermerk
vom 260815-0350 am Datensatz führt es aus; drei Punkte gehören in die Kurzfassung. Die eine
Statuszeile trägt jetzt sechs Ränge statt fünf, und der Betrachter braucht einen siebten für seine
Meldungen, dessen Einordnung der Bau erzwingt. Ein nackter Tastendruck im Dateifenster füllt seit
der Runde 10 einen Filter, und die Sprungmarke, deren Namen die Directive dieses Circles für seine
Verweisanker benutzt, gibt es dort nicht mehr. Und `Esc` trägt drei Bedeutungen, zu denen der
Betrachter eine vierte brächte.

Kein weiterer Circle trägt vorgesehen (`_a_`).

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
liegt. Genau eine Idee, kein Split vorzuschlagen.

Die Vorbedingung, die der Eintrag selbst mitbringt, ist beantwortet, und die Idee löst sich dabei
nicht auf. Der Eintrag vermutet, `F4` sei nur deshalb hakelig, weil die Systemeinstellung „F1, F2
usw. als Standard-Funktionstasten verwenden" ausgeschaltet ist. Gemessen am 260802-1137 auf dem
Abnahmegerät, mit der Einstellung ausgeschaltet, kommen `fn+F3`, `fn+F5` und `fn+F8` als
gewöhnliche `keyDown`-Ereignisse an, und KRK kann eine gehaltene `fn`-Taste gar nicht von einer
nackten Funktionstaste unterscheiden
(`shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md`, Nachtrag 260802-1409,
Beleg `spikes/fn-tasten/messung-A.txt`). Der Grund liegt beim Gerät: das Abnahmegerät trägt einen
Touch Bar, und dort heißt die Funktionstaste „fn halten und auf Glas tippen". Derselbe Nachtrag
sagt zu, jede Funktion der Norton-Reihe trage ab Werk zusätzlich ein Cmd-Kürzel, und die Runde 9
hat genau diesen Schritt zuletzt getan: `notizzettel` liegt auf `f2` **und** `cmd+k`
(`resources/default-keymap.toml`), mit demselben Nutzerentscheid als Begründung.

Was eine Klärungsrunde zu tragen hätte, ist die Wahl der Kombination, und die ist eng: alle vier
Cmd-Ebenen von `e` sind vergeben, `cmd+e` auf `editor_aus_vorschau`, `shift+cmd+e` auf
`fokus_editor`, `opt+cmd+e` auf `editor_schliessen`, `ctrl+cmd+e` auf
`editor_ansicht_umschalten`. Die Belegung ist mit der Runde 10 auf 84 Einträge und `Kommando` auf
78 Varianten gewachsen, der Spielraum also nicht größer geworden. Dazu dieselbe Vorbedingung wie
beim Web-Betrachter: eine neue Kombination erreicht keinen Nutzer, der seit der Runde 7 einmal
eine Taste zugewiesen hat
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

Der Eintrag ist keine Dublette zum offenen Defekt
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Defekt betrifft dasselbe `F4` mit einem anderen Symptom und bliebe bestehen, gleich welche
zweite Kombination hinzukommt.

**Performed this run:**

Keine. Der eine lebende Eintrag steht bereits auf empfohlen (`_p_`), die Rangumbenennung des
Laufs vom 260814-1513 gilt unverändert, und keine der vier bestätigungspflichtigen Operationen,
also weder Split noch Zusammenlegung noch Schließen noch Zurückstellen, ist vorgeschlagen oder
ausgeführt. Ein einziger lebender Eintrag mit genau einer Idee lässt für keine davon einen Anlass.
Zwei Einträge stehen auf geschlossen und nennen im Rumpf den Circle, der aus ihnen wurde:
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` (Runde 8) und
`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
(Runde 9).

Ein Teil des empfohlenen Eintrags ist defekt- und nicht ideenförmig. Er steht unter
`## Warnings`, Punkt 4; der Playmaker legt dafür keinen Datensatz an.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss in einem Satz |
|---|---|---|
| `260814-1551-tippen-filtert-dateiliste-flach-und-tief` | `_b_` | Beschränkt am 260815: Tippen filtert die Dateiliste an beliebiger Stelle des Namens, der Filtertext gehört dem Tab, ein neuntes Ankreuzfeld „Deep" weitet ihn auf den Unterbaum, die Statuszeile bekommt einen sechsten Rang; 14 Planschritte, 23 Commits, zehn der 77 Kriterien mit Bündelanteil unabgenommen. |
| `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | `_b_` | Beschränkt am 260814-1300: Notizzettel als zehntes Blatt, zwei Zettel als Tabs, `f2` und `cmd+k` hin, `Esc` zurück, vier Sicherungsmomente; gebaut und teilabgenommen, aber 16 der 29 Kriterien mit Bündelanteil hat keine Beobachtung angefasst. |
| `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | `_c_` | Kohärent am 260813-1415, als bisher einzige Runde dieses Projekts: Titelleiste mit Name und Version, Über-Dialog, semantische Versionstags und eine vierte Bedingung in der Zulässigkeitsregel. |
| `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` | `_b_` | Beschränkt am 260813: Suche in der Belegungsansicht, alle Funktionen im Menü, eine weitere Instanz mit zwei Sperren über `flock`; gebaut, nicht abgenommen. |
| `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | `_b_` | Beschränkt am 260812: Teilen, Ordnersprung, geschützte Ablage, gerendertes Markdown in der Vorschau und eine Statuszeile über die volle Breite; gebaut, nicht abgenommen. |

Ältere Abschlüsse: `260811-1304-statusleiste-mit-bereichsschaltern` (beschränkt am 260812-0820),
`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (beschränkt am 260811-2210),
`260809-2040-tastenbelegung-als-markdown-in-downloads` (beschränkt am 260811-1415),
`260807-2116-eingebauter-editor-mit-textmarken` (beschränkt am 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (beschränkt am 260807-1035).

Zehn Runden sind gefahren: neun beschränkt, eine kohärent.

## Archived (_s_ / _d_)

(keiner) — kein Circle-Datensatz trägt überholt (`_s_`) oder zurückgestellt (`_d_`).

## Warnings

**1. `CLAUDE.md` beschreibt ein Projekt mit neun Runden, und der Dateibestand trägt zehn.** Die
Zeile „Neun Runden sind gefahren" und die Tabelle darunter kennen die Runde 10 nicht, und der
Absatz `## Projektstand` ist auf den 260814-1430 datiert und nennt den Filter der Dateiliste
nicht. Die Datei sagt selbst, verbindlich sei der Dateibestand und nicht die Zeile, und dieser
Selbstschutz hält den Fehler klein; er nimmt ihn nicht weg. Betroffen ist zusätzlich der Absatz
über die Aufzählungen: `Kommando` trägt am 260815 78 Varianten und die Belegung 84 Einträge,
nachgezählt mit `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs` und
`grep -c '^\[\[' resources/default-keymap.toml`. Der Kuratorenlauf vom 260814-1405 liegt vor der
Runde 10; ein zweiter über `/fusion:curate` schließt die Lücke.

**2. Zwei Abnahmeläufe stehen aus, und beide sind Nutzerarbeit.** Die Runde 10 hat zehn ihrer 77
Abnahmekriterien mit Bündelanteil offen gelassen, vier davon sicherheitsrelevant; ihre Liste ist
`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`,
geschätzt 25 Minuten am laufenden Bündel im Vordergrund. Die Runde 9 hat 21 Kriterien ohne vollen
Beleg gelassen, rund zwanzig Minuten. Der Marker bewegt sich davon bei keiner der beiden:
beschränkt ist ein Endzustand, ein `mv` zurück auf aktiv ist unzulässig, und eine Fortsetzung ist
ein neuer Circle, der den beschränkten über `## Dependencies` zitiert (`rules/circle-records.md`,
`### Worked transitions`). Was die Läufe einbringen, ist der Beleg, nicht der Buchstabe.

**3. Die Auslieferungssperre steht wieder offen.** Am 260815-0350 geprüft: `git tag --points-at
HEAD` liefert nichts, 24 Commits liegen zwischen `v0.3.0` und `HEAD` (`2d2ce87`), `Cargo.toml`
führt weiter `0.3.0`. Station 1 von `cargo xtask release` vergleicht Tag und Version und hält den
Weg an. Der Zustand kehrt nach jeder Runde zurück, die Commits hinzufügt und keinen Tag setzt; der
Tag ist Nutzerarbeit. `cargo xtask bundle` und `make check` hängen nicht daran.

**4. Der empfohlene Ideeneintrag beschreibt zur Hälfte einen Defekt, und der Playmaker legt keinen
an.** Der Nutzerentscheid vom 260802-1409 sagt zu, jede Funktion der Norton-Reihe trage ab Werk
zusätzlich ein Cmd-Kürzel, und nennt unter seinen Constraints sechs davon, „F4 Bearbeiten"
darunter. In `resources/default-keymap.toml` tragen `f3`, `f5`, `f6`, `f7` und `f8` je zwei Wege,
`f4` als einzige einen. Der Kommentar an `bearbeiten` begründet die Abweichung ausdrücklich damit,
die Zwei-Wege-Regel gelte den sechs Funktionen „ganz oben" und `bearbeiten` gehöre zu den
späteren. Beide Aussagen zusammen gehen nicht auf. Entweder ist der Kommentar eine unbelegte
Umdeutung eines umgesetzten Nutzerentscheids, dann ist es ein Defekt, oder der Nutzer hat
`bearbeiten` bewusst herausgenommen, dann fehlt der Datensatz dazu. Die Entscheidung, was davon
gilt, gehört dir; der Playmaker schreibt weder in den Ideen- noch in den Defektspeicher.

**5. Der Defekt am doppelt belegten Ausgabeort besteht unverändert.** `cargo xtask bundle` und
`cargo xtask release` legen beide `target/KRK.app` an, und ein gewöhnliches `make run` überschreibt
damit ein beglaubigtes Bündel
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`,
drei Zuschnitte mit Kosten). Die Abnahme der Runde 9 hat den Fall zum ersten Mal praktisch
getroffen und ihn von Hand umgangen. Die zwei ausstehenden Abnahmeläufe aus Punkt 2 treffen ihn
erneut.

**6. 99 Defekte sind offen**, 13 davon im gemeinsamen Speicher. Gegenüber dem 260814-1513 sind es
zehn mehr: acht aus der Runde 10 und zwei im gemeinsamen Speicher. Die Abschlussnotiz der Runde 10
ordnet ihre acht ein: keiner betrifft das Verhalten des Filters im gewöhnlichen Gebrauch, der
Schwerpunkt liegt bei Proben ohne Ort und bei Prosa, die dem Code hinterherläuft. Die Liste
liefert:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**7. 24 Entscheidungsdatensätze sind offen, neun sind beantwortet und nicht umgesetzt.** Gegenüber
dem 260814-1513 sind das fünf offene und acht beantwortete mehr, alle dreizehn aus der Runde 10.
Keine offene Frage hält einen Planschritt auf; alle binden künftige Arbeit. Eine der fünf offenen
ist im Baum bereits beantwortet, ohne dass der Datensatz nachgezogen wäre: die Rangfolge von `Esc`
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_*_an-welcher-stelle-der-bedeutungen-von-esc-steht-der-filtertext.md`).
Derselbe Befund für sechs weitere Datensätze steht als eigener Defekt
(`shared/issues/260814-1955_*_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`).
Die Liste liefert:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**8. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge. Die vier Kanten des Web-Betrachters führen auf die
Runden 1, 5, 6 und 7, und alle vier sind terminal. An keinen Circle-Datensatz ist eine
`## Dependency warning` angehängt worden.

**9. Ein neuer Vermerk zu gealterter Grundlage.** Die Runde 10 ist seit dem letzten Lauf auf
beschränkten Abschluss übergegangen, und der Datensatz des Web-Betrachters hat dafür einen
`## Parent grounding stale` vom 260815-0350 bekommen. Die Auslösebedingung ist wie schon am
260814-1301 zur Hälfte erfüllt: der Grounding-Abschnitt des Betrachters stammt vom 260804 und
zitiert die Runde 10 nicht, wohl aber zwei Bauteile, die sie angefasst hat. Fünf Punkte stehen
darin, die drei wichtigsten sind der sechste Rang der Statuszeile, die gefallene Sprungmarke des
Dateifensters und die dritte Bedeutung von `Esc`.

**10. Der Datensatz des Web-Betrachters trägt 1069 Zeilen und sechzehn Playmaker-Abschnitte aus
neun Läufen**, neun Aktivierungsvorschläge und sieben Vermerke zu gealterter Grundlage. Die Länge
wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne dass an ihm gearbeitet würde. Die
zwei Abschnitte dieses Laufs tragen allein die Änderungen nach. Wer den Stand lesen will, liest
die letzten beiden, nicht alle sechzehn.
