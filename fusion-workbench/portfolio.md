# Portfolio

**Generated:** 260814-1513 (by playmaker session 260814-1513-playmaker-user-fusion-next)
**Domain bias:** code

---

**Was ansteht.** Zwei Dinge stehen bereit, und sie kosten sehr unterschiedlich viel. Das Billigere
ist keine Runde: eine zweite Abnahmeliste für die Runde 9, gebunden an die 21 Kriterien ohne
vollen Beleg, rund zwanzig Minuten am laufenden Bündel im Vordergrund. Die Runde bleibt
beschränkt geschlossen, gleich wie die Liste ausgeht, denn `_b_` ist ein Endzustand; was du
gewinnst, ist die Auskunft, ob der Notizzettel hält, was er zusagt. Das Teurere sind zwei
Kandidaten nebeneinander: die eine Idee im Speicher, ein zweites Kürzel für den Editor-Einstieg,
und der vorgesehene Web-Betrachter. Die Idee ist die kleinere Runde und trägt ihre Vorbedingungen
inzwischen beantwortet mit sich.

Seit dem Lauf vom 260814-1301 haben sich zwei Dinge bewegt, beide außerhalb der Circles: `v0.3.0`
ist getaggt und ausgeliefert, und `CLAUDE.md` ist kuratiert worden.

---

## Active (_t_)

(keiner)

`fusion-workbench/.active-circle` ist nicht vorhanden, und kein Circle-Datensatz trägt `_t_`.
Beides zusammen ist der reguläre Zustand nach einem Abschluss; es steht dazu keine Warnung.
Die Runde 9 ist am 260814-1300 beschränkt geschlossen worden.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — einziger
vorgesehener Circle, alle Vorbedingungen am Baum gebaut, und der Zuschnitt hat sich seit dem
260804 nicht bewegt.

### Rang 1 — der eingebaute Web-Betrachter

`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`

**Directive in einem Satz.** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, der in
einem gewöhnlichen Tab des Vorschaufensters lebt, über die Tastatur bedient wird und Sprungmarken
auf jedem sichtbaren Link trägt; `Opt+Cmd+G` öffnet die Adresse aus der Zwischenablage danach in
KRK statt im Systembrowser.

Der Circle steht als einziger Kandidat auf Rang 1, und die Reihenfolge ist damit keine Leistung
der Rangheuristik. Was die Heuristik beiträgt, ist die Prüfung der Vorbedingungen, und die fällt
sauber aus. Ein einziger offener Entscheidungsdatensatz bindet ihn, und der Grounding-Abschnitt
ordnet die Bindung selbst als Schluss und nicht als Feststellung ein:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`
fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt. Ein Betrachter
spricht eine Systemschnittstelle an, deren Umfang sich zwischen macOS 15 und macOS 26
unterscheidet; gemessen ist das nicht. Die sieben projektweit offenen Fragen zu Git, Code-SDK,
Untergrenzen-Prüfung, Ereignisabgriff, zweiter Instanz, Belegungssuche und Menüleiste binden ihn
nicht. Seine Abhängigkeit führt auf die Runde 1, und die trägt beschränkten Abschluss (`_b_`).
Für die Gewichtung `code` wäre das ein Abzug, wenn allein `_c_` als erfüllte Vorbedingung zählte.
In diesem Projekt zählt es nicht: `CLAUDE.md` hält seit der Kuratierung vom 260814-1405
ausdrücklich fest, dass `_b_` hier die Verfügbarkeit des Nutzers für den Abnahmelauf misst und
nicht die Reife einer Runde. Die Zwischenablage-Auswertung aus Schritt S13 und das Vorschaufenster
aus Schritt S19, auf denen der Betrachter aufsetzt, stehen beide am Baum.

Vor der Aktivierung stehen zwei Arbeiten, und keine davon ist eine Formalie. Erstens eine
Untersuchung des Darstellungsmittels: der Circle legt bewusst nicht fest, womit KRK Web-Inhalt
darstellt, weder eine Systemschnittstelle noch eine fremde Kiste. Zweitens eine Klärungsrunde über
die drei Fragen im Datensatz, von denen die erste den Zuschnitt entscheidet: Welche Quellen dürfen
die Adresse setzen? Bleibt es bei der Zwischenablage und den Sprungmarken der Seite, bekommt KRK
einen Betrachter; kommen Adresseingabe und gespeicherte Web-Adressen hinzu, bekommt es einen
Browser. Die beiden späteren Playmaker-Vermerke am Datensatz haben drei weitere Fragen
hinzugefügt, die aus den Runden 5 bis 9 stammen.

Eine Vorbedingung kommt aus der Runde 9 und betrifft die Abnahmeplanung, nicht die Klärung: die
Directive verlangt vier neue Befehle, Blättern, Zurück, Vor und das Schalten der Sprungmarken, und
jeder davon erreicht einen Nutzer mit eigener Belegungsdatei ab Werk unbelegt
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).
Dazu die Lehre aus dem Abschluss der Runde 9: die Abnahmeliste dieser Runde sollte je Kriterium
eine Kennzeichnung und je Bündelkriterium eine Beobachtung tragen, so wie die Runde 8 es gehalten
hat. Der Betrachter wird viele Kriterien am laufenden Bündel haben, weil Blättern, Zurück, Vor und
Sprungmarken sich an einer echten Seite zeigen und nicht an einer Probe.

Kein weiterer Circle trägt `_a_`.

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
`editor_ansicht_umschalten`. Dazu dieselbe Vorbedingung wie beim Web-Betrachter: eine neue
Kombination erreicht keinen Nutzer, der seit der Runde 7 einmal eine Taste zugewiesen hat
(`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

Der Eintrag ist keine Dublette zum offenen Defekt
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
Der Defekt betrifft dasselbe `F4` mit einem anderen Symptom und bliebe bestehen, gleich welche
zweite Kombination hinzukommt.

**Performed this run:**

- Umbenennung `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
  von offen (`_o_`) auf empfohlen (`_p_`). Der Lauf vom 260814-1301 hat den Eintrag empfohlen und
  die Umbenennung nicht mitgezogen; die Rangumbenennung ist autonom und wird hier nachgeholt.

Keine der vier bestätigungspflichtigen Operationen, also weder Split noch Zusammenlegung noch
Schließen noch Zurückstellen, ist vorgeschlagen oder ausgeführt. Ein einziger lebender Eintrag mit
genau einer Idee lässt für keine davon einen Anlass. Zwei Einträge stehen auf geschlossen und
nennen im Rumpf den Circle, der aus ihnen wurde:
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` (Runde 8) und
`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
(Runde 9).

Ein Teil des empfohlenen Eintrags ist defekt- und nicht ideenförmig. Er steht unter
`## Warnings`, Punkt 2; der Playmaker legt dafür keinen Datensatz an.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss in einem Satz |
|---|---|---|
| `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | `_b_` | Beschränkt am 260814-1300: Notizzettel als zehntes Blatt, zwei Zettel als Tabs, `f2` und `cmd+k` hin, `Esc` zurück, vier Sicherungsmomente; gebaut und teilabgenommen, aber 16 der 29 Kriterien mit Bündelanteil hat keine Beobachtung angefasst. |
| `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` | `_c_` | Kohärent am 260813-1415, als bisher einzige Runde dieses Projekts: Titelleiste mit Name und Version, Über-Dialog, semantische Versionstags und eine vierte Bedingung in der Zulässigkeitsregel. |
| `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` | `_b_` | Beschränkt am 260813: Suche in der Belegungsansicht, alle Funktionen im Menü, eine weitere Instanz mit zwei Sperren über `flock`; gebaut, nicht abgenommen. |
| `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | `_b_` | Beschränkt am 260812: Teilen, Ordnersprung, geschützte Ablage, gerendertes Markdown in der Vorschau und eine Statuszeile über die volle Breite; gebaut, nicht abgenommen. |
| `260811-1304-statusleiste-mit-bereichsschaltern` | `_b_` | Beschränkt am 260812-0820: Breitenregel über Anteile für alle fünf Bereiche und ausblendbares linkes Dateifenster; 13 Abnahmekriterien nur am laufenden Bündel zu sehen. |

Ältere Abschlüsse: `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (beschränkt am
260811-2210), `260809-2040-tastenbelegung-als-markdown-in-downloads` (beschränkt am 260811-1415),
`260807-2116-eingebauter-editor-mit-textmarken` (beschränkt am 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (beschränkt am 260807-1035).

Neun Runden sind gefahren: acht beschränkt, eine kohärent.

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
den Kriterien ohne vollen Beleg gehören die fünf Zwischenablagebefehle im Zettel, die sieben
Textautomatiken und alle drei Beenden-Kriterien von C4.

**2. Der empfohlene Ideeneintrag beschreibt zur Hälfte einen Defekt, und der Playmaker legt keinen
an.** Der Nutzerentscheid vom 260802-1409 sagt zu, jede Funktion der Norton-Reihe trage ab Werk
zusätzlich ein Cmd-Kürzel, und nennt unter seinen Constraints sechs davon, „F4 Bearbeiten"
darunter. In `resources/default-keymap.toml` tragen `f3`, `f5`, `f6`, `f7` und `f8` je zwei Wege,
`f4` als einzige einen. Der Kommentar an `bearbeiten` begründet die Abweichung ausdrücklich damit,
die Zwei-Wege-Regel gelte den sechs Funktionen „ganz oben" und `bearbeiten` gehöre zu den
späteren. Beide Aussagen zusammen gehen nicht auf. Entweder ist der Kommentar eine unbelegte
Umdeutung eines umgesetzten Nutzerentscheids, dann ist es ein Defekt, oder der Nutzer hat
`bearbeiten` bewusst herausgenommen, dann fehlt der Datensatz dazu. Die Entscheidung, was davon
gilt, gehört dir; der Playmaker schreibt weder in den Ideen- noch in den Defektspeicher.

**3. Der Defekt am doppelt belegten Ausgabeort besteht unverändert.** `cargo xtask bundle` und
`cargo xtask release` legen beide `target/KRK.app` an, und ein gewöhnliches `make run` überschreibt
damit ein beglaubigtes Bündel
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`,
drei Zuschnitte mit Kosten). Die Abnahme der Runde 9 hat den Fall zum ersten Mal praktisch
getroffen und ihn von Hand umgangen: das beglaubigte Bündel liegt gesichert unter
`~/Library/Caches/krk-beglaubigt-260814-1054/`. Die Auslieferung von `v0.3.0` am 260814 macht den
Fall nicht kleiner, sondern häufiger.

**4. 89 Defekte sind offen**, 11 davon im gemeinsamen Speicher. Die Zahl ist gegenüber dem
260814-1301 unverändert. Die Runde 9 hat 18 hinterlassen; ihre Abschlussnotiz ordnet sie ein: kein
Defekt betrifft das Verhalten des Zettels im gewöhnlichen Gebrauch, der Schwerpunkt liegt bei
Prosa, die dem Code hinterherläuft, und bei der Abnahmedeckung selbst. Die Liste liefert:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**5. 19 Entscheidungsdatensätze sind offen, einer ist beantwortet und nicht umgesetzt.** Auch
diese Zahl ist gegenüber dem 260814-1301 unverändert. Keine offene Frage hält einen Planschritt
auf; alle binden künftige Arbeit. Der eine beantwortete ist
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`.
Die Liste liefert:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**6. Kein Abhängigkeitszyklus.** Der gerichtete Graph über die nicht terminalen Circles hat einen
Knoten und keine Kante innerhalb dieser Menge. Die einzige Kante des Portfolios führt vom
Web-Betrachter auf die Runde 1, und die ist terminal. An keinen Circle-Datensatz ist eine
`## Dependency warning` angehängt worden.

**7. Kein neuer Vermerk zu gealterter Grundlage.** Seit dem Lauf vom 260814-1301 ist kein Circle
auf beschränkten Abschluss (`_b_`) übergegangen, also ist die Auslösebedingung nicht eingetreten.
Der Datensatz des Web-Betrachters trägt weiterhin die sechs Vermerke der früheren Läufe; der
jüngste vom 260814-1301 behandelt die Runde 9 und gilt unverändert.

**8. Der Datensatz des Web-Betrachters trägt 943 Zeilen und vierzehn Playmaker-Abschnitte aus acht
Läufen**, acht Aktivierungsvorschläge und sechs Vermerke zu gealterter Grundlage. Die Länge wächst
mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne dass an ihm gearbeitet würde. Der
Abschnitt dieses Laufs ist kurz gehalten und trägt allein die Änderungen nach. Wer den Stand lesen
will, liest den letzten Abschnitt, nicht alle vierzehn.

**9. Zwei Warnungen früherer Läufe sind erledigt und stehen deshalb nicht mehr hier.** Die
Auslieferungssperre ist geschlossen: `git tag --points-at HEAD` liefert `v0.3.0` und `Cargo.toml`
führt `0.3.0`, geprüft am 260814-1513. Und die Warnung, `CLAUDE.md` beschreibe ein Projekt mit vier
Runden, ist mit dem Kuratorenlauf vom 260814-1405 gegenstandslos geworden; die Datei führt jetzt
neun Runden und benennt zusätzlich, warum der Marker `_c_` in diesem Projekt keine Reifeauskunft
gibt.
