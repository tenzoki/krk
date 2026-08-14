# Portfolio

**Generated:** 260814-1301 (by playmaker session 260814-1301-playmaker-direct-dispatch)
**Domain bias:** code

---

**Was ansteht.** Die billigste wertvolle Arbeit dieses Projekts ist keine Runde, sondern zwanzig
Minuten am laufenden Bündel: eine zweite Abnahmeliste für die Runde 9, gebunden an die 21
Kriterien, die der erste Lauf nicht angefasst hat. Die Runde selbst bleibt beschränkt geschlossen,
gleich wie die Liste ausgeht, denn `_b_` ist ein Endzustand. Was du gewinnst, ist die Auskunft,
ob der Notizzettel hält, was er zusagt. Danach stehen zwei Kandidaten nebeneinander: die eine
offene Idee im Speicher, das zweite Kürzel für den Editor-Einstieg, und der vorgesehene
Web-Betrachter. Die Idee ist billiger, als sie am 260813 aussah, und der Grund kommt aus der
Runde 9.

---

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt den Marker
für aktiv (`_t_`). Das ist der reguläre Zustand nach einem Abschluss und kein Befund.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
vorgesehene Circle; vor der Aktivierung stehen unverändert eine Untersuchung des
Darstellungsmittels und eine Klärungsrunde über sechs Fragen.

### Rang 1 — `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters und
wird über die Tastatur bedient, mit Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten:** eine Kante, auf die Runde 1
(`260802-0842-krk-mac-dateimanager-editor-git`), die den beschränkten Abschluss trägt.

Der Rang ist wie in den sechs Läufen davor keine Auswahl: dieser Circle ist der einzige
vorgesehene, und eine Rangfolge mit einem Element sagt nichts über relative Reife. Die Empfehlung
stützt sich deshalb auf absolute Signale, und zwei davon haben sich seit dem 260813-2203 bewegt.

**Die Bedingung, die der vorige Lauf als erledigt gemeldet hat, steht wieder offen.** Am
260814-1301 nachgeprüft: `git tag --points-at HEAD` liefert nichts, zwölf Commits liegen zwischen
`v0.2.1` und `HEAD` (`4907cd4`), und `Cargo.toml` führt weiter `0.2.1`. Station 1 von
`cargo xtask release` vergleicht Tag und Version (`xtask/src/release.rs`, `stand_pruefen` ab Zeile
208) und hält den Auslieferungsweg damit erneut an. Der Arbeitsbaum ist sauber. Das ist keine
Eigenschaft des Circles, sondern der wiederkehrende Zustand nach jeder Runde.

**Die Aussicht auf einen kohärenten Abschluss ist schwächer geworden, und die Runde 9 sagt,
woran es lag.** Der Lauf vom 260813-1510 hatte aus der Runde 8 geschlossen, eine Runde könne
kohärent enden, sofern der Nutzer die Handabnahme fährt. Die Runde 9 hat die Handabnahme gefahren
und ist dennoch beschränkt geschlossen. Der Unterschied liegt am Zuschnitt der Abnahmeliste, nicht
am Baum und nicht am Nutzer: die Runde 8 kennzeichnete jedes Abnahmekriterium einzeln als `(Probe)`
oder `(Bündel)` und stellte je Bündelkriterium eine Beobachtung, die Runde 9 führte zwei Listen je
Fähigkeit und verlor die Bindung. Für diesen Circle folgt daraus eine Auflage an sein Plan-Gate
und keine Verschiebung: seine Abnahmeliste trägt je Kriterium eine Kennzeichnung. Er wird viele
Kriterien am laufenden Bündel haben, weil Blättern, Zurück, Vor und Sprungmarken sich an einer
echten Seite zeigen und nicht an einer Probe.

Unverändert offen bleibt, was am Circle selbst hängt: das Mittel der Darstellung von Web-Inhalt
gehört in eine eigene Untersuchung vor den Plan, und die Klärungsrunde trägt sechs Fragen. Dazu
binden die ungemessene Verfügbarkeitsprüfung für Schnittstellen ab macOS 26
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`)
und die Frage, ob die Untergrenzen-Angabe prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`). Neu
hinzugekommen ist eine Vorbedingung für die Abnahme: seine vier neuen Befehle kommen bei jedem
Nutzer mit eigener Belegungsdatei unbelegt an
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

Die Prüfung „alle Abhängigkeiten kohärent abgeschlossen" ist für diesen Circle nicht eingerechnet:
seine einzige Kante führt auf einen beschränkten Abschluss, und der ist ein Endzustand, den keine
künftige Arbeit zurücknimmt (`rules/circle-records.md`, `### Worked transitions`). Ein Kriterium,
dessen Wert sich nie ändern kann, trägt kein Rangsignal.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— eine Idee, kein Split nötig, und ihre selbstgestellte Vorbedingung ist inzwischen beantwortet:
gemessen ist die Sache seit dem 260802, und die Runde 9 hat für denselben Fall den Präzedenzfall
gebaut.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

### Rang 1 — das zweite Kürzel für den Editor-Einstieg

`shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`

Eine zweite, besser erreichbare Tastenkombination für `bearbeiten`, das heute allein auf `F4`
liegt. Genau eine Idee, kein Split.

Der Lauf vom 260813-2203 hat diesen Eintrag auf Rang 2 gestellt, weil er seine eigene Vorbedingung
mitbrachte und sie ungeprüft war: er vermutet, `F4` sei nur deshalb hakelig, weil die
Systemeinstellung „F1, F2 usw. als Standard-Funktionstasten verwenden" ausgeschaltet ist, und
schloss daraus, die Idee könnte sich auflösen. **Die Vorbedingung ist beantwortet, und die Idee
löst sich nicht auf.** Drei Feststellungen, alle am Baum und an den Datensätzen geprüft:

- **Die Systemeinstellung ist nicht der Grund.** Gemessen am 260802-1137 auf dem Abnahmegerät,
  mit der Einstellung ausgeschaltet: `fn+F3`, `fn+F5` und `fn+F8` kommen als gewöhnliche
  `keyDown`-Ereignisse an. KRK belegt den Tastencode und kann eine gehaltene `fn`-Taste gar nicht
  von einer nackten Funktionstaste unterscheiden
  (`shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md`, Nachtrag 260802-1409,
  Beleg `spikes/fn-tasten/messung-A.txt`). Der Grund steht im selben Nachtrag: das Abnahmegerät
  trägt einen Touch Bar, und dort heißt die Funktionstaste „fn halten und auf Glas tippen".
- **Der Nutzer hat den Fall schon entschieden.** Derselbe Nachtrag hält fest: die Norton-Reihe
  bleibt auf den Funktionstasten, und jede dieser Funktionen trägt ab Werk zusätzlich ein
  Cmd-Kürzel, beide Wege in einer Zeile der Belegungsansicht. Der Datensatz trägt den Marker für
  umgesetzt.
- **Die Runde 9 hat denselben Schritt gerade getan.** `notizzettel` liegt auf `f2` **und** `cmd+k`
  (`resources/default-keymap.toml:916`), und der Kommentar darüber begründet die zwei Wege wörtlich
  mit demselben Nutzerentscheid vom 260802-1409. Eine Runde für `bearbeiten` täte nichts Neues,
  sondern schlösse die Lücke, die als einzige offen geblieben ist.

Was eine Klärungsrunde noch zu tragen hat, ist die Wahl der Kombination, und die ist eng: alle
vier Cmd-Ebenen von `e` sind vergeben, `cmd+e` auf `editor_aus_vorschau`, `shift+cmd+e` auf
`fokus_editor`, `opt+cmd+e` auf `editor_schliessen`, `ctrl+cmd+e` auf
`editor_ansicht_umschalten`. Dazu kommt eine Vorbedingung, die es am 260813 noch nicht gab: eine
neue Kombination erreicht keinen Nutzer, der seit der Runde 7 einmal eine Taste zugewiesen hat, denn
`Belegung::bauen` nimmt dessen Datei als Quelle der Tasten
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

Der Eintrag ist keine Dublette zum offenen Defekt
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Defekt betrifft dasselbe `F4` mit einem anderen Symptom und bliebe bestehen, gleich welche
zweite Kombination hinzukommt.

### Zur Konsolidierung

Ein Eintrag auf offen (`_o_`), keiner auf empfohlen (`_p_`). Der Eintrag trägt genau eine Idee,
also ist kein Split vorzuschlagen, und mit einem einzigen offenen Eintrag ist eine Dublettenprüfung
gegenstandslos. Zwei Einträge stehen auf geschlossen und nennen in ihrem Rumpf den Circle, der aus
ihnen wurde: `shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` (Runde 8) und
`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
(Runde 9, am 260813-2334 geschlossen).

Ein Teil des offenen Eintrags ist defekt- und nicht ideenförmig. Er steht unter `## Warnings`,
Punkt 2, und der Playmaker legt dafür keinen Datensatz an.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss in einem Satz |
|---|---|---|
| `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | `_b_` | Beschränkt am 260814-1300: Notizzettel als zehntes Blatt, zwei Zettel als Tabs, `f2` und `cmd+k` hin, `Esc` zurück, vier Sicherungsmomente; gebaut und teilabgenommen, aber 16 der 29 Kriterien mit Bündelanteil hat keine Beobachtung angefasst. |
| `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | `_c_` | Kohärent am 260813-1415, als bisher einzige Runde dieses Projekts: Titelleiste mit Name und Version, Über-Dialog, semantische Versionstags und eine vierte Bedingung in der Zulässigkeitsregel. |
| `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` | `_b_` | Beschränkt am 260813: Suche in der Belegungsansicht, alle 82 Funktionen im Menü, eine weitere Instanz mit zwei Sperren über `flock`; gebaut, nicht abgenommen. |
| `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | `_b_` | Beschränkt am 260812: Teilen, Ordnersprung, geschützte Ablage, gerendertes Markdown in der Vorschau und eine Statuszeile über die volle Breite; gebaut, nicht abgenommen. |
| `260811-1304-statusleiste-mit-bereichsschaltern` | `_b_` | Beschränkt am 260812-0820: Breitenregel über Anteile für alle fünf Bereiche und ausblendbares linkes Dateifenster; 13 Abnahmekriterien nur am laufenden Bündel zu sehen. |

Ältere Abschlüsse: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (beschränkt am
260811-2210), `260809-2040-tastenbelegung-als-markdown-in-downloads` (beschränkt am 260811-1415),
`260807-2116-eingebauter-editor-mit-textmarken` (beschränkt am 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (beschränkt am 260807-1035).

Damit sind neun Runden gefahren: acht beschränkt, eine kohärent.

## Archived (_s_ / _d_)

(keiner) — kein Circle-Datensatz trägt überholt (`_s_`) oder zurückgestellt (`_d_`).

## Warnings

**1. Die Runde 9 nennt einen Weg zu einem kohärenten Abschluss, und der Marker geht diesen Weg
nicht mit.** Ihre `## Closure note` schließt mit: „eine zweite Abnahmeliste, gebunden an die 21
unbelegten Kriterien, rund zwanzig Minuten im Vordergrund". Die Arbeit lohnt und ist die billigste
Auskunft, die dieses Projekt gerade zu kaufen hat. Der Marker bewegt sich davon nicht: `_b_` ist
ein Endzustand, ein `mv` zurück auf `_t_` ist unzulässig, und eine Fortsetzung ist ein neuer
Circle, der den beschränkten über `## Dependencies` zitiert (`rules/circle-records.md`,
`### Worked transitions`). Was die zweite Liste einbringt, ist der Beleg, nicht der Buchstabe. Zu
den unbelegten Kriterien gehören die fünf Zwischenablagebefehle im Zettel, die sieben
Textautomatiken und alle drei Beenden-Kriterien von C4.

**2. Der offene Ideeneintrag beschreibt zur Hälfte einen Defekt, und der Playmaker legt keinen
an.** Der Nutzerentscheid vom 260802-1409 sagt zu, jede Funktion der Norton-Reihe trage ab Werk
zusätzlich ein Cmd-Kürzel, und nennt unter seinen Constraints sechs davon, „F4 Bearbeiten"
darunter. In `resources/default-keymap.toml` tragen `f3`, `f5`, `f6`, `f7` und `f8` je zwei Wege,
`f4` als einzige einen. Der Kommentar an `bearbeiten` (Zeilen 164 bis 174) begründet die Abweichung
ausdrücklich damit, die Zwei-Wege-Regel gelte den sechs Funktionen „ganz oben" und `bearbeiten`
gehöre zu den späteren. Beide Aussagen zusammen gehen nicht auf. Entweder ist der Kommentar eine
unbelegte Umdeutung eines umgesetzten Nutzerentscheids, dann ist es ein Defekt, oder der Nutzer
hat `bearbeiten` bewusst herausgenommen, dann fehlt der Datensatz dazu. Die Entscheidung, was
davon gilt, gehört dir; der Playmaker schreibt weder in den Ideen- noch in den Defektspeicher.

**3. `CLAUDE.md` beschreibt ein Projekt mit vier Runden, und es sind neun.** Die Warnung steht zum
zweiten Lauf in Folge und ist die folgenreichste, weil jeder Agent diese Datei zuerst liest.
Falsch sind unter anderem die Tabelle „Vier Runden sind gefahren", der Satz „Alle vier Runden sind
als beschränkter Abschluss geschlossen" samt der daran hängenden Bemerkung zur Rangheuristik, und
der Absatz „Zwei Circles sind vorgesehen und nicht gefahren" mit der Zeile „Die Statusleiste steht
auf Rang 1" — die Statusleiste ist als Runde 5 gefahren und beschränkt geschlossen, vorgesehen ist
allein der Web-Betrachter. Der Projektstand trägt „Geprüft am 260811-2230" und ist drei Tage alt.
Drei der falschen Zahlen haben Defektdatensätze:
`shared/issues/260812-2253_*_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md` (der
Baum trägt inzwischen 77, am 260814-1301 nachgezählt in
`crates/krk-core/src/tasten/belegung.rs:579`),
`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`
und `shared/issues/260813-1345_*_fuenf-stellen-nennen-79-funktionen-und-73-mit-kommando-die-belegung-fuehrt-82-und-76.md`.
Für die Rundenzahl selbst gibt es keinen. `/fusion:revise-claude-md` ist der Weg.

**4. Der Defekt am doppelt belegten Ausgabeort besteht unverändert.** `cargo xtask bundle` und
`cargo xtask release` legen beide `target/KRK.app` an, und ein gewöhnliches `make run` überschreibt
damit ein beglaubigtes Bündel
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`,
drei Zuschnitte mit Kosten). Die Abnahme der Runde 9 hat den Fall zum ersten Mal praktisch
getroffen und ihn von Hand umgangen: das beglaubigte Bündel liegt gesichert unter
`~/Library/Caches/krk-beglaubigt-260814-1054/`
(`circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260814-1100-abnahmeliste-notizzettel.md`).

**5. 89 Defekte sind offen**, 11 davon im gemeinsamen Speicher. Die Runde 9 hat 18 hinterlassen und
liegt damit knapp hinter der Runde 6 mit 25. Ihre Abschlussnotiz ordnet die eigenen ein: keiner
betrifft das Verhalten des Zettels im gewöhnlichen Gebrauch, der Schwerpunkt liegt bei Prosa, die
dem Code hinterherläuft, und bei der Abnahmedeckung selbst. Vier der 18 hat erst der
Abschlussabgleich vom 260814-1247 gefunden, und sie betreffen genau die Zählung, an der die Runde
beschränkt geschlossen hat.

**6. 19 Entscheidungsdatensätze sind offen, einer ist beantwortet und nicht umgesetzt.** Die Zahl
ist gegenüber dem 260813-2203 unverändert: die Runde 9 hat zwei eigene Fragen gestellt und beide
umgesetzt geschlossen. Keine offene Frage hält einen Planschritt auf; alle binden künftige Arbeit.
Die Liste liefert:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**7. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge. Die einzige Kante des Portfolios führt vom
Web-Betrachter auf die Runde 1, und die ist terminal. An keinen Circle-Datensatz ist eine
`## Dependency warning` angehängt worden.

**8. Ein Vermerk zu gealterter Grundlage ist angehängt worden, an
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`.** Kind ist die
Runde 9. Die Auslösebedingung ist zur Hälfte erfüllt: das Kind trägt `_b_`, aber der Abschnitt
`## Grounding snapshot` des Elternteils stammt vom 260804 und zitiert es nicht. Der Vermerk steht
trotzdem, weil die Runde 9 an zwei Stellen gearbeitet hat, durch die jeder Befehl des Betrachters
laufen wird: die Ausnahme im Ersthelfervorbehalt hat jetzt einen gebauten Präzedenzfall in beide
Richtungen, und ein neuer Befehl kommt beim Nutzer mit eigener Belegungsdatei unbelegt an.

**9. Der Datensatz des Web-Betrachters trägt 914 Zeilen und dreizehn Playmaker-Abschnitte aus
sieben Läufen.** Die Länge wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne dass an
ihm gearbeitet würde. Die beiden Abschnitte dieses Laufs sind wieder kurz gehalten und tragen
allein die Änderungen nach. Wer den Stand lesen will, liest die letzten beiden Abschnitte, nicht
alle dreizehn.
