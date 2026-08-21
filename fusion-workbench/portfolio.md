# Portfolio

**Generated:** 260821-2115 (by playmaker session 260821-2115-playmaker-orchestrator-phase4)
**Domain bias:** code

---

**Was ansteht.** Die Runde 15 ist kohärent geschlossen, kein Circle ist aktiv, und die Wahl der
nächsten Runde liegt beim Nutzer. Vorgesehen ist weiterhin genau einer, der eingebaute
Web-Betrachter im Vorschaufenster, gefilt am 260804 und seither fünfzehn Runden lang übergangen.
Er ist aktivierbar, aber nicht ohne Vorlauf: eine Untersuchung des Darstellungsmittels und eine
Klärungsrunde über drei offene Fragen stehen davor.

**Die Runde 15 hat den Grounding-Schnappschuss des Web-Betrachters nicht gealtert, und die
Prüfung hat trotzdem etwas gefunden.** Ihre fünf Commits fassen `xtask/`, `README.md` und
Werkstattdatensätze an und keine Zeile Anwendungscode; die Fläche, in der der Betrachter leben
soll, steht unverändert. Was der Schnappschuss dagegen seit dem 260804 nicht sagt, ist, was KRKs
eigenes Bündel braucht, um `http:` selbst anzuzeigen statt es abzugeben. Zwei Stellen sind am
260821 nachgesehen: `resources/Info.plist` führt keinen Schlüssel `NSAppTransportSecurity`, und
eine Berechtigungsdatei gibt es im Baum nicht, während signiert wird mit `--options runtime`.
Beides ist keine Antwort und beides ist auch kein Hindernis für die Aktivierung; es sind zwei
Randbedingungen mehr für die Untersuchung, die ohnehin vor dem Plan steht. Ausgeschrieben stehen
sie im Vermerk vom 260821-2115 im Datensatz des Circles, zusammen mit einem Vorschlag, welche
vier Stücke des Schnappschusses eine Neuschärfung nachzöge.

**Die Runde 15 ist nachgetragen und ihre Ablage ist richtig, nicht mangelhaft.** Spec, Plan, vier
Durchsichten und die Sitzungsprotokolle liegen im gemeinsamen Speicher, weil bei ihrer Entstehung
kein Circle aktiv war; die Herkunftsregel legt sie genau dorthin. Im Circle selbst liegen die
Abnahme-Durchsicht, zwei Defekte und zwei Protokolle. Der Datensatz zitiert den Rest an seinem
Ort und kopiert nichts.

---

## Active (_t_)

(keiner)

Kein Circle trägt den Marker `_t_`, und `fusion-workbench/.active-circle` ist gelöscht. Beides
zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung. Der Orchestrator hat
den Zeiger beim Schließen der Runde 15 entfernt.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
vorgesehene Circle des Projekts, Vorbedingungen sauber, aktivierbar nach einer Untersuchung des
Darstellungsmittels und einer Klärungsrunde über drei offene Fragen.

**Rang 1 von 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`**
Directive: KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters,
wird über die Tastatur bedient und bekommt Sprungmarken auf jeden sichtbaren Link. Kein Verlauf,
kein Adressfeld, kein Herunterladen, allein `http:` und `https:`.
Abhängigkeiten: eine Kante, auf `260802-0842-krk-mac-dateimanager-editor-git` (Runde 1),
terminal und am Baum gebaut.

Die Rangfolge ist zum dreizehnten Mal in Folge keine Leistung der Heuristik: es gibt nur diesen
einen Kandidaten. Was die Heuristik beiträgt, ist die Prüfung der Vorbedingungen, und sie fällt
sauber aus. Ein einziger offener Entscheidungsdatensatz bindet den Circle, die
Verfügbarkeitsprüfung für Schnittstellen ab macOS 26
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
und sie hängt an derselben Wahl, die der Circle bewusst offenlässt: womit KRK Web-Inhalt
darstellt.

**Die eine Abhängigkeitskante bekommt keinen Abzug, und die Abweichung ist ausgesprochen.**
Mechanisch verlangt die Vorbedingungsprüfung eine Kante auf einen kohärent (`_c_`) geschlossenen
Circle; die Runde 1 trägt den beschränkten Abschluss (`_b_`) und bekäme damit eine Marke.
`CLAUDE.md` weist diese Lesart für dieses Projekt zurück, und der Bestand stützt sie: zehn der
fünfzehn gefahrenen Runden tragen `_b_`, immer aus demselben Grund, weil der Abnahmelauf KRK im
Vordergrund verlangt und damit Nutzerarbeit ist. Der Marker misst hier die Verfügbarkeit des
Nutzers und nicht die Reife einer Runde. Die Runde 1 ist am Baum gebaut, und die zwei Bauteile,
auf denen der Betrachter aufsetzt, die Zwischenablage-Auswertung und das Vorschaufenster mit
Tabs, stehen und werden täglich benutzt. Eine Heuristik, die allein `_c_` zählt, gäbe hier eine
irreführende Auskunft.

**Was diesen Lauf von den zwölf davor unterscheidet, ist die Prüfung des Schnappschusses.** Sie
fällt anders aus als am 260820: die Runde 15 hat nichts gealtert, weil sie kein Anwendungscode
anfasst. Gefunden ist stattdessen eine Lücke, die keine Runde verursacht hat und die so alt ist
wie der Schnappschuss selbst, die Netzrichtlinie des eigenen Bündels. Die vier Punkte der Runde
14 gelten unverändert daneben. Alles zusammen steht in den zwei Abschnitten vom 260821-2115 im
Datensatz des Circles.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— der einzige lebende Eintrag, eine Idee, Datensätze auf der Platte, heute shapebar. Dieser Lauf
schlägt daneben vor, ihn stattdessen zurückzustellen, und der Grund steht unter dem Eintrag.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

**Rang 1 von 1: Der Editor-Einstieg braucht ein erreichbares Kürzel neben F4.**
`F4` öffnet den ausgewählten Eintrag des Dateifensters im eingebauten Editor und ist in der
Bedienung hakelig; gebraucht wird eine zweite, besser erreichbare Kombination. Der Eintrag steht
seit dem 260817 auf empfohlen (`_p_`) und bleibt dort: er ist die einzige lebende Idee des
Speichers, und die Rangfolge ändert sich dadurch nicht.

- Vorgeschlagen, nicht durchgeführt:
  `defer shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md until shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md beantwortet ist`

**Warum die Zurückstellung vorgeschlagen ist.** Der Eintrag und die offene Nutzerfrage greifen
auf denselben knappen Vorrat, die Tastenkombinationen dieses Bereichs. Die Frage führt drei
ausgearbeitete Möglichkeiten, und zwei davon fassen genau dorthin: Möglichkeit 2 legt einen neuen
Befehl auf eine freie Kombination, die erst gefunden werden muss, und Möglichkeit 3 macht `f3`
zur Umschalttaste und nennt die Nachbarschaft zu `f4` ausdrücklich als Argument. Wer den Eintrag
vorher shapt, wählt eine Kombination, während über die Kombinationen desselben Bereichs noch
entschieden wird. Die Zurückstellung ist eine der vier bestätigungspflichtigen Operationen;
dieser Lauf hält keine Bestätigung und führt sie nicht aus.

**Zwei Dinge sollte der Nutzer wissen, wenn er den Eintrag doch shapt.** Erstens ist die
Ursachenvermutung des Eintrags zur Hälfte widerlegt. Er hält ungeprüft fest, `F4` sei ab Werk mit
Spotlight belegt und KRK sehe die Taste womöglich gar nicht; der Abnahmelauf vom 260820-1030
zeigt für den gemeldeten Fall das Gegenteil, die Taste erreicht KRK und die Datei öffnet sich,
und was fehlschlägt, ist das Setzen des Fokus
(`shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`).
Zweitens deckt der Eintrag ein Viertel des Bereichs ab, den vier Datensätze zusammen beschreiben;
die anderen drei stehen unter `## Warnings`, Punkt 2. Das Filen eines Eintrags über den ganzen
Bereich ist Nutzerarbeit über `/fusion:memo`.

**Nichts durchgeführt.** Der Speicher trägt einen einzigen lebenden Eintrag. Er trägt eine Idee,
also gibt es nichts zu teilen; er hat kein Gegenstück, also nichts zusammenzuführen; seine Idee
ist lebendig, also nichts zu schließen. Der Marker bleibt auf `_p_`.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss |
|---|---|---|
| `260821-1644-veroeffentlichen-als-achte-station` | `_c_` | 260821-2110. `cargo xtask veroeffentlichen <zahl>` ist die achte Station der Auslieferungskette; `KRK 0.5.6` liegt öffentlich als Zip an einer Releaseseite. Der Nutzer hat die fünfzehn ihm zugewiesenen Kriterien gemessen, 14 halten, eines ist nicht prüfbar. |
| `260819-2230-auswahl-und-kopieren-in-der-vorschau` | `_c_` | 260820-1045. Die Vorschaufläche ist auswählbar; bei gerendertem Markdown landet der Quelltext mit seinen Auszeichnungszeichen in der Ablage. Abnahmelauf gefahren. |
| `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` | `_c_` | 260819. `opt+cmd+s` gleicht die Ordner der zwei Dateifenster an; die Dateiliste nimmt Abwürfe aus fremden Anwendungen entgegen. Abnahmelauf gefahren. |
| `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` | `_c_` | 260818. Ein Löschweg, er führt in den Papierkorb, jeder Vorgang mit genau einer Rückfrage. Kohärent ohne Abnahmelauf, weil die Directive über die zehn Zeitzusagen nichts sagt. |
| `260816-1321-inhaltsfilter-mit-ankreuzfeld-content` | `_b_` | 260816-2030. Der Filter berücksichtigt den Inhalt, Ankreuzfeld „Content" neben „Deep". Die Abnahmeliste liegt fertig unter `messungen/260816-abnahme-inhaltsfilter.md`. |

Fünfzehn Runden sind gefahren, zehn davon mit beschränktem Abschluss (`_b_`) und fünf mit
kohärentem (`_c_`). Der vollständige Bestand steht in `ls fusion-workbench/circles/*/_[bc]_circle.md`.

## Archived (_s_ / _d_)

| Circle | Marker | Stand |
|---|---|---|
| `260816-2255-befehle-absetzen-und-makros-speichern` | `_d_` | Zurückgestellt am 260817-0445 zugunsten der zwölften Runde. Nichts gebaut. Spec mit 54 Abnahmekriterien und Plan mit 22 Schritten liegen vollständig vor. Eine Aufnahme wäre ein neuer Circle, der diesen über `## Dependencies` zitiert. |

Kein Circle trägt den Marker `_s_` (überholt).

## Warnings

**1. Kein Abhängigkeitszyklus, und der Graph hat nur einen Knoten.** Über die nicht-terminalen
Circles gebildet enthält der Graph genau einen Knoten und keine Kante innerhalb dieser Menge: der
Web-Betrachter ist der einzige nicht-terminale Circle, und seine eine Kante führt auf die
terminale Runde 1. In keinen Circle-Datensatz wurde ein `## Dependency warning` geschrieben.

**2. Ein Bereich ist beschrieben und steht als Runde nirgends: die Bewegung zwischen Editor und
Vorschau.** Vier Datensätze tragen ihn, und keiner davon ist ein Circle. Der Bestand ist
unverändert gegenüber dem 260820, weil die Runde 15 in diesem Bereich nichts angefasst hat:

- `shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`
  (Schwere hoch): die Datei öffnet sich, der Fokus landet in der Lesezeichenleiste oder nirgends.
- `shared/issues/260820-1034_*_cmd-e-bleibt-in-der-vorschau-wirkungslos-und-ist-in-der-dateiliste-gar-nicht-belegt.md`
  (Schwere mittel): in der Dateiliste ist die Wirkungslosigkeit die Belegung und kein Defekt, in
  der Vorschau ist sie ein Defekt. Die Ursache ist nicht erhoben.
- `shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`
  (offen): drei ausgearbeitete Möglichkeiten. Die erste widmet `cmd+e` um und behebt den Defekt
  dabei; die dritte legt das Umschalten auf `f3`.
- Der Rückstandseintrag oben, das zweite Kürzel für den Editor-Einstieg.

Ob daraus eine Runde wird, entscheidet der Nutzer. Der Playmaker filt weder Defekte noch
Rückstandseinträge und legt keinen Circle an.

**3. Ein Vermerk `## Parent grounding stale` ist geschrieben worden, und die Auslösebedingung
fehlte zweifach.** Der Auftrag knüpft ihn an ein Kind, das beschränkt (`_b_`) schließt; die Runde
15 hat kohärent geschlossen. Und der Schnappschuss des Web-Betrachters zitiert die Runde 15
nirgends, denn sie ist zwei Wochen jünger als er. Der Nutzer hat die Prüfung für diesen Lauf
ausdrücklich beauftragt, und sie hat einen Befund geliefert, der nicht von der Runde 15 stammt.
Der Vermerk sagt das in seinem ersten Absatz.

**4. Die Hälfte einer bindenden Grundlage steht hinter einem Überholt-Marker.** Der Datensatz
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`
trägt seit der Runde 14 den Marker für überholt. Überholt ist allein seine zweite Hälfte, die
Unauswählbarkeit der Vorschau. Die erste Hälfte gilt unverändert und bindet den Web-Betrachter:
ein Verweis im gerenderten Markdown bekommt Farbe, keine Klickwirkung. Der Preis bleibt: wer nach
aktiver Grundlage sucht, sucht nach offen, beantwortet und umgesetzt und bekommt diese Datei
nicht zu sehen.

**5. Der Marker `_c_` trägt in diesem Projekt jetzt fünf Lesarten, und die Frage dazu ist weiter
offen.** Die Runden 8, 13 und 14 schlossen kohärent nach einem Abnahmelauf des Nutzers. Die Runde
12 schloss kohärent ganz ohne, weil ihre Directive über die zehn Zeitzusagen nichts sagt. Die
Runde 15 kommt als fünfte hinzu: kohärent nach einer gefahrenen Abnahme, bei der eines der
fünfzehn Kriterien nicht prüfbar ist und als Indiz statt als Abnahme dasteht, weil ein zweiter
Mac ohne Netz fehlt. Was der Marker an einem **Spec** bedeutet, bleibt offen
(`shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`),
und die Runde 15 hat ihn an ihrem eigenen Spec bewusst auf `_o_` stehen lassen, um die Frage
nicht durch vollendete Tatsache zu entscheiden.

**6. Die Aufzeichnungslücke der Runde 14 hat sich nicht wiederholt.** Die Runde 15 hat ihre
Abnahme als Durchsicht abgelegt
(`circles/260821-1644-veroeffentlichen-als-achte-station/reviews/260821-2105-coderev-abnahme-der-fuenfzehn-nutzerkriterien.md`),
mit den fünfzehn Kriterien einzeln. Der Vorlauf aus der Runde 14, deren Abnahmelauf keinen
Datensatz hinterließ, ist damit unterbrochen und nicht fortgesetzt. Eine Lücke bleibt und ist
nicht mehr zu schließen: die Tagzahl der Gegenseite vor dem Auslieferungslauf um 20:24 ist
nirgends festgehalten, weshalb ein Kriterium an einer Kontrollmessung abgenommen ist statt am
Lauf selbst.

**7. Zwei Datensätze der Runde 15 sind nicht committet.** `git status` zeigt die Umbenennung von
`_t_circle.md` auf `_c_circle.md` als Löschung plus unverfolgte Datei. Der Abschluss steht auf
der Platte und nicht in git. Der Orchestrator committet; dieser Lauf tut es nicht.

**8. `CLAUDE.md` kennt die Runde 15 nicht, und die falsche Aussage über den Ereignisabgriff ist
weg.** Die Rundentabelle führt vierzehn Zeilen; die fünfzehnte fehlt, und mit ihr die achte
Station, das öffentliche Release und die Betriebsregel gegen den Datenverlust beim Austausch der
App. Der Absatz zum Ersthelfervorbehalt dagegen nennt seit `7da3098` richtig zwei angemeldete
Textflächen. Ein Durchgang von `/fusion:curate` schließt den Rest; er ist dem Nutzer vorbehalten.

**9. Das Auslieferungstor steht wieder offen.** Am 260821 geprüft: `Cargo.toml` trägt `0.5.6`,
der Tag `v0.5.6` existiert, aber `git tag --points-at HEAD` gibt nichts zurück, und zwei Commits
liegen zwischen `v0.5.6` und HEAD, beide Dokumentation. Station 1 von `cargo xtask release`
vergleicht Tag und Version und hält den Lauf an. Der Zustand kehrt nach jeder Runde wieder, die
Commits hinzufügt und keinen Tag setzt; die Zahl wählt der Nutzer im Argument von
`./release.sh <version>`. `cargo xtask bundle` und `make check` hängen nicht daran.

**10. Der Abnahmelauf der zehn Zeitzusagen ist seit dem 260810-1918 nicht mehr gefahren.** Er
liegt damit vor den Runden 5 bis 15. Jener Lauf war der erste vollständig saubere, alle zehn
Zusagen in allen fünf Durchgängen. Die Runde 15 fasst keinen Anwendungscode an, also ist von ihr
keine Zusage berührt und keine elfte Zahl entstanden. Der zurückgestellte Datensatz
`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`
wartet weiter auf Läufe an verschiedenen Tagen und fällt als zurückgestellter aus jeder Suche
nach aktiver Grundlage heraus.

**11. Drei Abnahmeläufe stehen weiterhin aus, und alle drei sind Nutzerarbeit.** Die Runde 11 hat
ihre Liste fertig hinterlassen (`messungen/260816-abnahme-inhaltsfilter.md`, 28 Beobachtungen an
vier Orten). Die Runde 10 hat zehn ihrer 77 Abnahmekriterien mit Bündelanteil offen, vier davon
sicherheitsrelevant
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`).
Die Runde 9 hat 21 Kriterien ohne vollständigen Beleg. Kein Marker bewegt sich davon: beschränkt
ist ein Endzustand. Was die Läufe einbringen, ist der Beleg und nicht der Buchstabe.

**12. 152 Defektdatensätze sind offen**, 44 im gemeinsamen Speicher und 108 in den Circles. Beim
Lauf vom 260820-1044 waren es 145, 37 und 108. Die sieben neuen liegen sämtlich im gemeinsamen
Speicher und stammen aus der Runde 15 und aus der Untersuchung des Lesezeichenverlusts. Vier
Befunde der Runde 15 bleiben offen, darunter `gh_pruefen` fragt nach dem Konto und nicht nach dem
Vorhaben
(`circles/260821-1644-veroeffentlichen-als-achte-station/issues/260821-2105_*_ein-angemeldetes-gh-das-das-vorhaben-nicht-erreicht-schiebt-erst-und-nennt-dann-die-falsche-abhilfe.md`)
und das Abnahmekriterium C6.3, das die Zeichenfolge enthält, die es verbietet. Ein Defekt ist
gemildert und nicht behoben: `bundle` und `release` schreiben an denselben Ort
(`shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`).
Der Bestand:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**13. 36 Entscheidungsdatensätze sind offen, 6 beantwortet und noch nicht umgesetzt — und die
zweite Zahl passt nicht zum vorigen Lauf.** Der Lauf vom 260820-1044 meldete 33 offene und 14
beantwortete. Die offenen sind um drei gewachsen, was zu den zwei neuen Fragen der Runde 15 und
der Ablagefrage vom 260821-0142 passt. Die beantwortete Zahl ist von 14 auf 6 gefallen, und git
kennt seit dem 260820 nur zwei Umbenennungen von beantwortet auf umgesetzt. Acht Datensätze sind
damit nicht erklärt: entweder hat einer der beiden Läufe falsch gezählt, oder Umbenennungen sind
außerhalb von git geschehen. Der Bestand von heute ist gemessen und steht oben; welche Zahl von
gestern galt, entscheidet dieser Lauf nicht. Nachzuzählen mit
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_a_*.md'`

**14. Der empfohlene Rückstandseintrag beschreibt in der Hälfte seines Rumpfes einen möglichen
Defekt, und der Playmaker filt keinen.** Der Nutzerentscheid vom 260802-1409 sagt zu, dass jede
Funktion der Norton-Reihe zusätzlich ein Cmd-Kürzel trägt, und nennt „F4 Bearbeiten" unter seinen
sechs. Der Kommentar an `bearbeiten` in `resources/default-keymap.toml` begründet die Abweichung
damit, die Zwei-Wege-Regel gelte den fünf Funktionen der Norton-Reihe ganz oben. Beide Aussagen
gehen nicht zusammen. Entweder ist der Kommentar eine unbelegte Umdeutung eines umgesetzten
Nutzerentscheids, dann ist es ein Defekt, oder der Nutzer hat `bearbeiten` bewusst herausgenommen,
dann fehlt der Datensatz dazu. Die Entscheidung liegt beim Nutzer.

**15. Der Datensatz des Web-Betrachters trägt jetzt 23 Playmaker-Abschnitte aus dreizehn Läufen**,
dreizehn Aktivierungsvorschläge und zehn Vermerke zur Grundlage, auf 1558 Zeilen. Die Länge
wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne bearbeitet zu werden. Die zwei
Abschnitte dieses Laufs tragen den Stand für sich; wer den aktuellen Stand braucht, liest sie und
nicht alle 23.
