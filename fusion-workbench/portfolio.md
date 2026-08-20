# Portfolio

**Generated:** 260820-1044 (by playmaker session 260820-1044-playmaker-direct-dispatch)
**Domain bias:** code

---

**Was ansteht.** Die Runde 14 ist kohärent geschlossen, kein Circle ist aktiv, und die Wahl der
nächsten Runde liegt beim Nutzer. Vorgesehen ist genau einer, der eingebaute Web-Betrachter im
Vorschaufenster, gefilt am 260804 und seither vierzehn Runden lang übergangen. Er ist weiterhin
aktivierbar, aber nicht ohne Vorlauf: eine Untersuchung des Darstellungsmittels und eine
Klärungsrunde über drei offene Fragen stehen davor.

**Der zweite Kandidat ist neu und steht nirgends als Runde.** Der Abnahmelauf des Nutzers vom
260820-1030 hat drei Datensätze hervorgebracht, die zusammen einen Bereich beschreiben: die
Bewegung zwischen Editor und Vorschau. Zwei Defekte und eine ausgearbeitete Nutzerfrage, dazu der
eine lebende Eintrag des Rückstandsspeichers, der ein viertes Stück desselben Bereichs trägt. Der
Bereich ist kleiner als der Web-Betrachter, er hat einen hohen Defekt darunter, und seine
Grundlage ist zwei Stunden alt statt zwei Wochen. Als Circle existiert er nicht, und der
Playmaker legt keinen an.

**Sprache.** Dieses Portfolio ist wieder deutsch. Das vorige vom 260819-0804 war englisch, weil
`CLAUDE.md` damals eine Artefaktsprache `en` deklarierte. Der Nutzer hat die Deklaration am
260819-2032 zurückgenommen; `**Language:** de` steuert seither beide Flächen.

---

## Active (_t_)

(keiner)

Kein Circle trägt den Marker `_t_`, und `fusion-workbench/.active-circle` ist gelöscht. Beides
zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung. Der Orchestrator hat
den Zeiger beim Schließen der Runde 14 entfernt.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
vorgesehene Circle des Projekts, Vorbedingungen sauber, aber erst nach einer Untersuchung des
Darstellungsmittels und einer Klärungsrunde über drei offene Fragen aktivierbar.

**Rang 1 von 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`**
Directive: KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters,
wird über die Tastatur bedient und bekommt Sprungmarken auf jeden sichtbaren Link. Kein Verlauf,
kein Adressfeld, kein Herunterladen, allein `http:` und `https:`.
Abhängigkeiten: eine Kante, auf `260802-0842-krk-mac-dateimanager-editor-git` (Runde 1),
terminal und am Baum gebaut.

Die Rangfolge ist zum zwölften Mal in Folge keine Leistung der Heuristik: es gibt nur diesen
einen Kandidaten. Was die Heuristik beiträgt, ist die Prüfung der Vorbedingungen, und sie fällt
sauber aus. Ein einziger offener Entscheidungsdatensatz bindet den Circle, die
Verfügbarkeitsprüfung für Schnittstellen ab macOS 26
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
und sie hängt an derselben Wahl, die der Circle bewusst offenlässt: womit KRK Web-Inhalt
darstellt. Der Abzug für beschränkten statt kohärenten Abschluss der Abhängigkeit ist wie in
jedem Lauf davor nicht angesetzt. Was diesen Lauf von den elf davor unterscheidet, ist die
Alterung: die Runde 14 hat die Vorschaufläche selbst verändert, und dieser Circle soll in einem
Tab genau dieses Fensters leben. Die Fläche nimmt jetzt den Fokus, sie ist auswählbar, der
Ereignisabgriff kennt zwei angemeldete Textflächen statt einer, und die Auswahl im gerenderten
Markdown liefert Quelltext über eine Kachelung, die es nur für Markdown gibt. Vier Punkte für die
Klärungsrunde, keiner davon hält die Aktivierung auf; sie stehen ausgeschrieben im Vermerk
`## Parent grounding stale` vom 260820-1044 im Datensatz des Circles.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— eine Idee, seit dem 260817 auf Empfehlung stehend, und der Abnahmelauf vom 260820 hat ihr drei
Datensätze zur Seite gestellt, die denselben Bereich betreffen.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

**Rang 1 von 1: Der Editor-Einstieg braucht ein erreichbares Kürzel neben F4.**
`F4` öffnet den ausgewählten Eintrag im eingebauten Editor und ist in der Bedienung hakelig;
gebraucht wird eine zweite, besser erreichbare Kombination. Der Eintrag trägt genau eine Idee,
zitiert Datensätze, die auf der Platte liegen, und ließe sich heute shapen.

**Zwei Dinge sollte der Nutzer wissen, bevor er ihn shapt.**

Erstens ist die Vermutung des Eintrags zur Ursache inzwischen zur Hälfte widerlegt. Er hält
ungeprüft fest, `F4` sei auf Apple-Tastaturen ab Werk mit Spotlight belegt und KRK sehe die Taste
womöglich gar nicht. Der Abnahmelauf vom 260820-1030 zeigt das Gegenteil für den gemeldeten Fall:
die Taste erreicht KRK, die Datei öffnet sich im Editor, und was fehlschlägt, ist das Setzen des
Fokus
(`shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`).
Ein Teil der gemeldeten Hakeligkeit ist damit ein Defekt und keine Tastenfrage.

Zweitens deckt der Eintrag nur ein Viertel des Bereichs ab, den die vier Datensätze zusammen
beschreiben. Wer ihn allein shapt, bekommt eine Runde über ein zweites Kürzel für den
Editor-Einstieg, während der hohe Defekt am Fokus, der wirkungslose `cmd+e` in der Vorschau und
die offene Frage nach einer Umschalttaste daneben stehen bleiben. Die drei sind keine
Rückstandseinträge und werden hier nicht als solche aufgeführt; sie stehen unter `## Warnings`,
Punkt 2. Das Filen eines Eintrags, der den ganzen Bereich trägt, ist Nutzerarbeit über
`/fusion:memo`.

**Nichts vorgeschlagen, nichts durchgeführt.** Der Speicher trägt einen einzigen lebenden
Eintrag. Er trägt eine Idee, also gibt es nichts zu teilen; er hat kein Gegenstück, also gibt es
nichts zusammenzuführen; seine Idee ist lebendig und fällig, also weder zu schließen noch
zurückzustellen. Sein Marker steht auf empfohlen (`_p_`) und bleibt dort.

## Recently closed (_c_ / _b_)

| Circle | Marker | Abschluss |
|---|---|---|
| `260819-2230-auswahl-und-kopieren-in-der-vorschau` | `_c_` | 260820-1045. Die Vorschaufläche ist auswählbar; bei gerendertem Markdown landet der Quelltext mit seinen Auszeichnungszeichen in der Ablage. Der Nutzer hat den Abnahmelauf gefahren. |
| `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` | `_c_` | 260819. `opt+cmd+s` gleicht die Ordner der zwei Dateifenster an; die Dateiliste nimmt Abwürfe aus fremden Anwendungen entgegen. Abnahmelauf gefahren. |
| `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` | `_c_` | 260818. Ein Löschweg, er führt in den Papierkorb, jeder Vorgang mit genau einer Rückfrage. Kohärent ohne Abnahmelauf, weil die Directive über die zehn Zeitzusagen nichts sagt. |
| `260816-1321-inhaltsfilter-mit-ankreuzfeld-content` | `_b_` | 260816-2030. Der Filter berücksichtigt den Inhalt, Ankreuzfeld „Content" neben „Deep". Die Abnahmeliste liegt fertig unter `messungen/260816-abnahme-inhaltsfilter.md`. |
| `260814-1551-tippen-filtert-dateiliste-flach-und-tief` | `_b_` | 260815. Tippen filtert die Dateiliste, „Deep" weitet den Filter auf den Unterbaum. Die Sprungmarke ist gefallen und ein abgenommenes Kriterium der Runde 1 ersetzt. |

Vierzehn Runden sind gefahren. Zehn davon tragen den beschränkten Abschluss (`_b_`), und immer
aus demselben Grund: der Abnahmelauf verlangt KRK im Vordergrund und ist Nutzerarbeit. Der
vollständige Bestand steht in `ls fusion-workbench/circles/*/_[bc]_circle.md`.

## Archived (_s_ / _d_)

| Circle | Marker | Stand |
|---|---|---|
| `260816-2255-befehle-absetzen-und-makros-speichern` | `_d_` | Zurückgestellt am 260817-0445 zugunsten der zwölften Runde. Nichts gebaut. Spec mit 54 Abnahmekriterien und Plan mit 22 Schritten liegen vollständig vor. Eine Aufnahme wäre ein neuer Circle, der diesen über `## Dependencies` zitiert. |

Kein Circle trägt den Marker `_s_` (überholt).

## Warnings

**1. Kein Abhängigkeitszyklus, und der Graph hat nur einen Knoten.** Über die nicht-terminalen
Circles gebildet, also über die vorgesehenen und den aktiven, enthält der Graph genau einen
Knoten und keine Kante innerhalb dieser Menge: der Web-Betrachter ist der einzige nicht-terminale
Circle, und seine eine Abhängigkeitskante führt auf die terminale Runde 1. In keinen
Circle-Datensatz wurde ein `## Dependency warning` geschrieben.

**2. Ein Bereich ist beschrieben und steht als Runde nirgends: die Bewegung zwischen Editor und
Vorschau.** Vier Datensätze tragen ihn, und keiner davon ist ein Circle:

- `shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`
  (Schwere hoch): die Datei öffnet sich, der Fokus landet in der Lesezeichenleiste oder nirgends.
  Als Ursache erschlossen, nicht gemessen, ist die Reihenfolge von Einblenden und Fokussetzen in
  `editor_oeffnen_lassen`.
- `shared/issues/260820-1034_*_cmd-e-bleibt-in-der-vorschau-wirkungslos-und-ist-in-der-dateiliste-gar-nicht-belegt.md`
  (Schwere mittel): in der Dateiliste ist die Wirkungslosigkeit die Belegung und kein Defekt, in
  der Vorschau ist sie ein Defekt. Die Ursache ist nicht erhoben.
- `shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`
  (offen): drei ausgearbeitete Möglichkeiten mit Empfehlung. Die erste widmet `cmd+e` um und
  behebt den Defekt dabei; die dritte legt das Umschalten auf `f3` und bricht eine bestehende
  Gewohnheit.
- Der Rückstandseintrag oben, das zweite Kürzel für den Editor-Einstieg.

Die drei ersten stammen aus dem Abnahmelauf der Runde 14 und liegen deshalb im gemeinsamen
Speicher: sie kommen nicht aus deren Directive. Ob daraus eine Runde wird, ist eine Entscheidung
des Nutzers. Der Playmaker filt weder Defekte noch Rückstandseinträge und legt keinen Circle an.

**3. Ein Vermerk `## Parent grounding stale` ist geschrieben worden, und die übliche
Auslösebedingung war nicht erfüllt.** Der Auftrag knüpft ihn an ein Kind, das beschränkt (`_b_`)
schließt; die Runde 14 hat kohärent geschlossen. Der Nutzer hat die Prüfung für diesen Lauf
ausdrücklich beauftragt, und sie fällt bejahend aus: die Runde hat die Vorschaufläche selbst
verändert, in der der Web-Betrachter leben soll. Der Lauf vom 260819-0804 hat für die Runde 13
anders entschieden, weil dort die Belegungstabelle betroffen war und nicht die Fläche. Der
Vermerk steht im Datensatz
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`.

**4. Die Hälfte einer bindenden Grundlage steht hinter einem Überholt-Marker.** Der Datensatz
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`
trägt seit der Runde 14 den Marker für überholt. Überholt ist allein seine zweite Hälfte, die
Unauswählbarkeit der Vorschau. Die erste Hälfte gilt unverändert und bindet den Web-Betrachter:
ein Verweis im gerenderten Markdown bekommt Farbe, keine Klickwirkung. Der Datensatz schreibt die
Trennung in seiner Zeile `Superseded by:` selbst aus, und die Schließungsnotiz der Runde 14
wiederholt sie. Der Preis bleibt: wer nach aktiver Grundlage sucht, sucht nach offen, beantwortet
und umgesetzt und bekommt diese Datei nicht zu sehen.

**5. Der Marker `_c_` trägt in diesem Projekt jetzt vier Lesarten, und eine offene Frage dazu
steht seit dem 260819.** Runde 8 und Runde 13 schlossen kohärent nach einem Abnahmelauf, den der
Nutzer gefahren hat. Runde 12 schloss kohärent ganz ohne Abnahmelauf, weil ihre Directive über
die zehn Zeitzusagen nichts sagt. Runde 14 schloss kohärent nach einem gefahrenen Abnahmelauf,
dessen Verdikt vorher `review-needed` lautete und allein durch den Durchgang des Nutzers
entfiel. Wer den Marker als „vom Nutzer abgenommen" liest, liegt bei dreien richtig und bei
Runde 12 falsch. Was er an einem **Spec** bedeutet, ist offen
(`shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`).
Der beschränkte Abschluss (`_b_`) ist in diesem Projekt weiterhin kein Mangel: er misst die
Verfügbarkeit des Nutzers und nicht die Reife einer Runde. Dieser Lauf hat keinen Abzug dafür
angesetzt.

**6. Der Abnahmelauf der Runde 14 hat keinen Datensatz auf der Platte hinterlassen, und das ist
das zweite Mal in Folge.** Die Schließungsnotiz hält fest, der Bündeldurchgang sei am 260820-1030
an `KRK.app` 0.5.4 aus `05cb614` gelaufen und die neuen Funktionen hielten. Unter
`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/history/` liegt kein Abnahmedokument,
und `messungen/` trägt als jüngsten Eintrag `260816-abnahme-inhaltsfilter.md`. Die Runde 8 hat
für denselben Vorgang eine Datei hinterlassen und sie in ihrer Schließungsnotiz benannt. Die
Aussage des Nutzers wird hier nicht bestritten; was fehlt, ist das Artefakt, an dem eine spätere
Runde nachlesen könnte, welche der Prüfungen gegen welches Bündel gehalten hat. Die
Schließungsnotiz nennt den Fall selbst: ob der Nutzer das Ziehen und die Dienste geprüft hat, ist
nicht aufgezeichnet, und der tragende Vorbehalt der Runde bleibt damit unbelegt.

**7. Zwei Datensätze der Runde 14 sind nicht committet.** `git status` zeigt die Umbenennung von
`_t_circle.md` auf `_c_circle.md` als Löschung plus unverfolgte Datei. Der Abschluss steht auf
der Platte und nicht in git. Der Orchestrator committet; dieser Lauf tut es nicht.

**8. `CLAUDE.md` trägt eine falsche Aussage über den Ereignisabgriff und kennt die Runde 14
nicht.** Unter „Was man nicht sieht" steht, die Textfläche des Editors sei die eine Ausnahme im
Ersthelfervorbehalt. Es sind zwei: `ist_eigene_textflaeche` prüft seit der Runde 14 auch die
Vorschaufläche. Die Schließungsnotiz der Runde 14 nennt die Stelle und drei weitere
unvollständige und hat keine angefasst. Daneben führt die Rundentabelle dreizehn Runden und nicht
vierzehn. Ein Durchgang von `/fusion:curate` schließt beides; er ist dem Nutzer vorbehalten.

**9. Das Auslieferungstor steht offen.** Am 260820 geprüft: `Cargo.toml` trägt `0.5.4`, der Tag
`v0.5.4` existiert, aber `git tag --points-at HEAD` gibt nichts zurück, und 22 Commits liegen
zwischen `v0.5.4` und HEAD. Station 1 von `cargo xtask release` vergleicht Tag und Version und
hält den Lauf an. Der Zustand kehrt nach jeder Runde wieder, die Commits hinzufügt und keinen Tag
setzt; die Zahl wählt der Nutzer im Argument von `./release.sh <version>`. `cargo xtask bundle`
und `make check` hängen nicht daran.

**10. Der Abnahmelauf der zehn Zeitzusagen ist seit dem 260810-1918 nicht mehr gefahren.** Er
liegt damit vor den Runden 5 bis 14. Jener Lauf war der erste vollständig saubere, alle zehn
Zusagen in allen fünf Durchgängen. Ein zweiter Punkt hängt daran: der zurückgestellte Datensatz
`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`
wartet auf weitere Läufe an verschiedenen Tagen und fällt als zurückgestellter aus jeder Suche
nach aktiver Grundlage heraus. Wird nie wieder gemessen, ist „bei 65 bleiben" der Sache nach
entschieden, ohne dass es jemand aufgeschrieben hätte.

**11. Drei Abnahmeläufe stehen weiterhin aus, und alle drei sind Nutzerarbeit.** Die Runde 11 hat
ihre Liste fertig hinterlassen (`messungen/260816-abnahme-inhaltsfilter.md`, 28 Beobachtungen an
vier Orten). Die Runde 10 hat zehn ihrer 77 Abnahmekriterien mit Bündelanteil offen, vier davon
sicherheitsrelevant
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`).
Die Runde 9 hat 21 Kriterien ohne vollständigen Beleg. Kein Marker bewegt sich davon: beschränkt
ist ein Endzustand. Was die Läufe einbringen, ist der Beleg und nicht der Buchstabe.

**12. 145 Defektdatensätze sind offen**, 37 davon im gemeinsamen Speicher und 108 in den Circles.
Beim Lauf vom 260819-0804 waren es 138. Die sieben neuen stammen sämtlich aus der Runde 14: fünf
in ihrem eigenen Speicher, darunter die vier, die der Nutzer am 260820-0750 ausdrücklich für
diesen Durchgang ausgeschlossen hat, und zwei im gemeinsamen aus dem Abnahmelauf. Der
gewichtigste ist
`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/issues/260820-0733_*_die-abfangstelle-verwirft-die-geforderten-sorten-und-leert-jede-gereichte-ablage.md`:
die Zusage „eine Stelle für alle Ausgabewege" ist für die Zwischenablage eingelöst, für das
Ziehen und die Dienste nicht. Der Bestand:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**13. 33 Entscheidungsdatensätze sind offen, 14 beantwortet und noch nicht umgesetzt.** Beim Lauf
vom 260819-0804 waren es 29 und 12. Die Runde 14 hat sieben Datensätze beantwortet, davon fünf
umgesetzt und zwei begründet auf beantwortet stehen lassen: die Quelltextzusage für das Ziehen
und die Dienste (`shared/decisions/260819-2216_*_gilt-die-quelltextzusage-auch-fuer-das-ziehen-einer-auswahl-und-die-dienste.md`,
gebunden an den Defekt aus Punkt 12) und die Frage nach einem Abnahmelauf gegen die Zusage L7
(`shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`,
deren Antwort auf zwei Ersatzkriterien ruht, von denen eines keinen Prüfer hat). Keine offene
Frage hält einen Planschritt auf; alle binden künftige Arbeit. Der Bestand:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**14. Der empfohlene Rückstandseintrag beschreibt in der Hälfte seines Rumpfes einen möglichen
Defekt, und der Playmaker filt keinen.** Der Nutzerentscheid vom 260802-1409 sagt zu, dass jede
Funktion der Norton-Reihe zusätzlich ein Cmd-Kürzel trägt, und nennt „F4 Bearbeiten" unter seinen
sechs. Der Kommentar an `bearbeiten` in `resources/default-keymap.toml:169-170` begründet die
Abweichung damit, die Zwei-Wege-Regel gelte den fünf Funktionen der Norton-Reihe ganz oben; am
260820 nachgelesen und unverändert. Beide Aussagen gehen nicht zusammen. Entweder ist der
Kommentar eine unbelegte Umdeutung eines umgesetzten Nutzerentscheids, dann ist es ein Defekt,
oder der Nutzer hat `bearbeiten` bewusst herausgenommen, dann fehlt der Datensatz dazu. Die
Entscheidung liegt beim Nutzer; der Playmaker schreibt weder in den Defekt- noch in den
Rückstandsspeicher.

**15. Der Datensatz des Web-Betrachters trägt jetzt 21 Playmaker-Abschnitte aus zwölf Läufen**,
zwölf Aktivierungsvorschläge und neun Vermerke zur gealterten Grundlage, auf 1437 Zeilen. Die
Länge wächst mit jedem Lauf, in dem der Circle vorgesehen bleibt, ohne bearbeitet zu werden. Die
zwei Abschnitte dieses Laufs tragen den Stand für sich; wer den aktuellen Stand braucht, liest
sie und nicht alle 21.
