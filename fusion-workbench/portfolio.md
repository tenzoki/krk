# Portfolio

**Generated:** 260821-2204 (by playmaker session 260821-2204-playmaker-direct-dispatch)
**Domain bias:** code

---

**Was ansteht.** Das Projekt trägt zum ersten Mal seit dem 260804 keine vorgesehene Arbeit. Der
Nutzer hat am 260821-2202 entschieden, dass das Abgeben an den Systembrowser genügt, und der
eingebaute Web-Betrachter, fünfzehn Runden lang der einzige vorgesehene Circle, ist damit
zurückgestellt. Fünfzehn Runden sind gefahren, kein Circle ist aktiv, und keiner steht bereit.
Die sechzehnte Runde hat heute keinen Träger.

**Die Rangliste der vorgesehenen Circles ist deshalb leer, und die Leere ist die Auskunft.** Ein
zurückgestellter oder geschlossener Circle kommt als Empfehlung nicht in Betracht: beide Zustände
sind Endzustände, und eine Fortsetzung wäre ein neuer Circle, der den alten über
`## Dependencies` zitiert. Was an die Stelle der Aktivierungsempfehlung rückt, ist der Rückstand.
Er führt einen einzigen lebenden Eintrag, das zweite Kürzel für den Editor-Einstieg, und ist
damit die einzige benannte Quelle künftiger Arbeit.

**Dieser Lauf hat seine Empfehlung von gestern zurückgenommen.** Am 260821-2115 stand unter dem
Rückstandseintrag der Vorschlag, ihn zurückzustellen, bis die offene Frage nach der Umschalttaste
beantwortet ist. Der Vorschlag entfällt. Solange daneben ein vorgesehener Circle stand, kostete
eine Zurückstellung nichts; jetzt wäre sie der Griff, der beide Flächen zugleich leert, und ein
zurückgestellter Eintrag kommt allein durch die Hand des Nutzers zurück. Die Spannung zwischen
Eintrag und offener Frage besteht unverändert, und sie ist besser durch eine Antwort auf die
Frage aufzulösen als durch das Parken des Eintrags.

**Kein Circle-Datensatz ist in diesem Lauf angefasst worden**, und das folgt aus dem Bestand.
Aktivierungsvorschlag, Zyklusvermerk und Vermerk zur veralteten Grundlage setzen je einen
nicht-terminalen Circle voraus. Seit dem 260821-2202 gibt es keinen. Geschrieben sind allein
diese Datei und das Protokoll dieses Laufs.

---

## Active (_t_)

(keiner)

Kein Circle trägt den Marker `_t_`, und `fusion-workbench/.active-circle` fehlt. Beides zusammen
ist der reguläre Zustand nach einem Abschluss und keine Warnung.

## Anticipated (_a_) — ranked

(keiner)

**Es gibt keine Empfehlung, welcher Circle als nächster aktiviert werden soll, weil es keinen
Kandidaten gibt.** `ls fusion-workbench/circles/*/_a_circle.md` liefert nichts. Der Graph der
nicht-terminalen Circles hat null Knoten, die Vorbedingungsprüfung hat keinen Gegenstand, und die
Rangheuristik der Domäne `code` läuft ins Leere. Zwölf Läufe in Folge haben hier einen einzigen
Kandidaten gerankt; dieser rankt keinen.

**Was den einen Kandidaten weggenommen hat, ist eine Absage und keine Vertagung.** Der Datensatz
`shared/decisions/260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`
hält die Wahl fest: der Systembrowser kann alles, was ein eingebauter Betrachter könnte, und
`Opt+Cmd+G` bleibt, wie es ist. Die Schließungsnotiz des Circles sagt ausdrücklich, dass der
Marker `_d_` weniger meint als die Entscheidung, weil das Circle-Vokabular für „nicht mehr
gewollt" keinen Buchstaben führt. Wer den Circle später liest, hält sich an die Notiz und nicht
an den Dateinamen.

**Zwei Wege führen zu einer sechzehnten Runde, und beide beginnen beim Nutzer.** Entweder wird
der Rückstandseintrag unten zu einer Runde geformt, oder der Nutzer beauftragt eine neue Runde
unmittelbar über `/fusion:direct <Entwurf>`. Der Playmaker legt keinen Circle an und filt keinen
Rückstandseintrag.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— der einzige lebende Eintrag, eine Idee, Datensätze auf der Platte, und seit heute die einzige
benannte Quelle künftiger Arbeit.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

**Rang 1 von 1: Der Editor-Einstieg braucht ein erreichbares Kürzel neben F4.**
`F4` öffnet den ausgewählten Eintrag des Dateifensters im eingebauten Editor und ist in der
Bedienung hakelig; gebraucht wird eine zweite, besser erreichbare Kombination. Der Eintrag steht
seit dem 260817 auf empfohlen (`_p_`) und bleibt dort. Er trägt eine Idee, hat kein Gegenstück im
Speicher, und seine Idee ist lebendig.

- Vorgeschlagen, nicht durchgeführt: nichts. Der Vorschlag vom 260821-2115, den Eintrag
  zurückzustellen, ist zurückgenommen; die Begründung steht im Kopf dieser Datei.

**Nichts durchgeführt.** Der Speicher trägt einen einzigen lebenden Eintrag. Nichts zu teilen,
nichts zusammenzuführen, nichts zu schließen. Der Marker bleibt auf `_p_`.

**Die Empfehlung trägt eine Einschränkung, und sie ist dieselbe wie gestern.** Der Eintrag und
die offene Frage
`shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`
greifen auf denselben knappen Vorrat, die Tastenkombinationen des fünften Bereichs. Die Frage
führt drei ausgearbeitete Möglichkeiten; Möglichkeit 2 legt einen neuen Befehl auf eine freie
Kombination, die erst gefunden werden muss, und Möglichkeit 3 macht `f3` zur Umschalttaste und
nennt die Nachbarschaft zu `f4` ausdrücklich als Argument. Wer den Eintrag vorher shapt, wählt
eine Kombination, während über die Kombinationen desselben Bereichs noch entschieden wird.

**Die saubere Reihenfolge kostet einen Nutzerakt und ist billiger als eine Zurückstellung.** Die
Frage ist entscheidbar und liegt seit dem 260820 vor; sie zu beantworten dauert ein Gate. Danach
shapt der Eintrag ohne Vorbehalt. Die Alternative wäre ein Eintrag über den ganzen Bereich, den
der Nutzer über `/fusion:memo` filt und der die Frage, die zwei Defekte und dieses Kürzel in
einer Runde zusammenfasst; die vier Datensätze stehen unter `## Warnings`, Punkt 4. Beide Wege
sind Nutzerarbeit, und beide sind besser als ein `/fusion:direct` auf den Eintrag allein.

**Ein Hinweis für den Fall, dass der Eintrag trotzdem sofort geformt wird.** Seine
Ursachenvermutung ist zur Hälfte widerlegt. Der Eintrag hält ungeprüft fest, `F4` sei ab Werk mit
Spotlight belegt und KRK sehe die Taste womöglich gar nicht; der Abnahmelauf vom 260820-1030
zeigt für den gemeldeten Fall das Gegenteil, die Taste erreicht KRK und die Datei öffnet sich.
Was fehlschlägt, ist das Setzen des Fokus
(`shared/issues/260820-1034_*_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`).

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
| `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` | `_d_` | Zurückgestellt am 260821-2202. Fallengelassen, nicht aufgeschoben: der Nutzer hat entschieden, dass das Abgeben an den Systembrowser genügt. Nie gefahren, siebzehn Tage vorgesehen. Die Schließungsnotiz sagt, dass `_d_` hier die nächstliegende und nicht die genaue Entsprechung ist. |
| `260816-2255-befehle-absetzen-und-makros-speichern` | `_d_` | Zurückgestellt am 260817-0445 zugunsten der zwölften Runde. Nichts gebaut. Spec mit 54 Abnahmekriterien und Plan mit 22 Schritten liegen vollständig vor. Eine Aufnahme wäre ein neuer Circle, der diesen über `## Dependencies` zitiert. |

Kein Circle trägt den Marker `_s_` (überholt).

Der Datensatz des Web-Betrachters trägt 23 Playmaker-Abschnitte aus dreizehn Läufen auf 1587
Zeilen. Mit dem terminalen Marker hört er auf zu wachsen: künftige Läufe schreiben keinen
Abschnitt mehr hinein. Wer den letzten Stand braucht, liest die zwei Abschnitte vom 260821-2115
und die Schließungsnotiz, nicht alle 23.

## Warnings

**1. Kein Abhängigkeitszyklus, und der Graph hat null Knoten.** Über die nicht-terminalen Circles
gebildet ist der Graph leer, weil kein Circle den Marker `_a_` oder `_t_` trägt. Aus demselben
Bestand folgt, dass die Fortpflanzungsprüfung für beschränkte Abschlüsse keinen Gegenstand hat:
sie braucht einen nicht-terminalen Eltern-Circle, dessen Grundlagenschnappschuss ein beschränkt
geschlossenes Kind zitiert. In keinen Circle-Datensatz ist in diesem Lauf geschrieben worden.

**2. Das Projekt hat zum ersten Mal keine vorgesehene Arbeit, und der Zustand ist neu, nicht
kaputt.** Fünfzehn Runden gefahren, keine aktiv, keine vorgesehen, ein lebender
Rückstandseintrag. Bis zum 260821-2202 stand hinter jedem Abschluss ein Kandidat; seit heute
steht dort nichts. Der Zustand meldet keinen Fehler: `.active-circle` fehlt korrekt, kein Zeiger
ist verwaist, kein Marker steht falsch. Was er meldet, ist eine Entscheidung, die ansteht. Ohne
Nutzerakt gibt es keine sechzehnte Runde, und die zwei Wege dorthin stehen oben unter
`## Anticipated`.

**3. Die Netzrichtlinie des Bündels ist ungemessen, und der Befund steht jetzt für sich.** Am
260821 an zwei Stellen nachgesehen: `resources/Info.plist` führt keinen Schlüssel
`NSAppTransportSecurity`, eine Berechtigungsdatei gibt es im Baum nicht, und signiert wird mit
`--options runtime`. Notiert war der Befund als Randbedingung des Web-Betrachters. Mit dessen
Wegfall verliert er seinen Anlass und nicht seine Geltung: wer künftig irgendeinen Netzzugriff
aus KRK heraus baut, trifft dieselbe ungemessene Frage an. Festgehalten ist er außerhalb des
zurückgestellten Circles in dessen Schließungsnotiz und hier.

**4. Ein Bereich ist beschrieben und steht als Runde nirgends: die Bewegung zwischen Editor und
Vorschau.** Vier Datensätze tragen ihn, und keiner davon ist ein Circle. Mit dem Wegfall des
Web-Betrachters ist er der einzige beschriebene Bereich, der auf eine Runde wartet:

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

**5. Der empfohlene Rückstandseintrag beschreibt in der Hälfte seines Rumpfes einen möglichen
Defekt, und der Playmaker filt keinen.** Der Nutzerentscheid vom 260802-1409 sagt zu, dass jede
Funktion der Norton-Reihe zusätzlich ein Cmd-Kürzel trägt, und nennt „F4 Bearbeiten" unter seinen
sechs. Der Kommentar an `bearbeiten` in `resources/default-keymap.toml` begründet die Abweichung
damit, die Zwei-Wege-Regel gelte den fünf Funktionen der Norton-Reihe ganz oben. Beide Aussagen
gehen nicht zusammen. Entweder ist der Kommentar eine unbelegte Umdeutung eines umgesetzten
Nutzerentscheids, dann ist es ein Defekt, oder der Nutzer hat `bearbeiten` bewusst herausgenommen,
dann fehlt der Datensatz dazu. Die Entscheidung liegt beim Nutzer.

**6. Die Hälfte einer bindenden Grundlage steht hinter einem Überholt-Marker, und ihr Adressat
hat gewechselt.** Der Datensatz
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`
trägt seit der Runde 14 den Marker für überholt. Überholt ist allein seine zweite Hälfte, die
Unauswählbarkeit der Vorschau. Die erste Hälfte gilt unverändert: ein Verweis im gerenderten
Markdown bekommt Farbe, keine Klickwirkung. Bis gestern band sie den Web-Betrachter; seit dessen
Wegfall beschreibt sie kein künftiges Vorhaben mehr, sondern das Verhalten der ausgelieferten
Anwendung. Der Preis ist derselbe geblieben: wer nach aktiver Grundlage sucht, sucht nach offen,
beantwortet und umgesetzt und bekommt diese Datei nicht zu sehen.

**7. Der Marker `_c_` trägt in diesem Projekt fünf Lesarten, und die Frage dazu ist weiter
offen.** Die Runden 8, 13 und 14 schlossen kohärent nach einem Abnahmelauf des Nutzers. Die Runde
12 schloss kohärent ganz ohne, weil ihre Directive über die zehn Zeitzusagen nichts sagt. Die
Runde 15 kommt als fünfte hinzu: kohärent nach einer gefahrenen Abnahme, bei der eines der
fünfzehn Kriterien nicht prüfbar ist und als Indiz statt als Abnahme dasteht, weil ein zweiter
Mac ohne Netz fehlt. Was der Marker an einem **Spec** bedeutet, bleibt offen
(`shared/decisions/260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md`).

Der Bestand von zehn `_b_` gegen fünf `_c_` ist kein Befund über Qualität. `CLAUDE.md` weist eine
Rangheuristik zurück, die allein `_c_` als erfüllte Vorbedingung zählt: der Marker misst hier die
Verfügbarkeit des Nutzers und nicht die Reife einer Runde, weil der Abnahmelauf KRK im
Vordergrund verlangt. In diesem Lauf hatte die Heuristik ohnehin keinen Gegenstand.

**8. `CLAUDE.md` steht an zwei Stellen neben dem Bestand, und eine davon ist heute entstanden.**
Die Rundentabelle führt vierzehn Zeilen; die fünfzehnte fehlt, und mit ihr die achte Station der
Auslieferungskette, das öffentliche Release und die Betriebsregel gegen den Datenverlust beim
Austausch der App. Neu hinzu kommt der Abschnitt zur bindenden Grundlage: er sagt, `ls
fusion-workbench/circles/*/_a_circle.md` liefere am 260815 einen Circle, den Web-Betrachter. Das
Kommando liefert seit dem 260821-2202 nichts. Ein Durchgang von `/fusion:curate` schließt beides;
er ist dem Nutzer vorbehalten.

**9. Das Auslieferungstor steht wieder offen.** Am 260821 geprüft: `Cargo.toml` trägt `0.5.6`,
der Tag `v0.5.6` existiert, aber `git tag --points-at HEAD` gibt nichts zurück, und sechs Commits
liegen zwischen `v0.5.6` und HEAD, alle Dokumentation. Station 1 von `cargo xtask release`
vergleicht Tag und Version und hält den Lauf an. Der Zustand kehrt nach jeder Runde wieder, die
Commits hinzufügt und keinen Tag setzt; die Zahl wählt der Nutzer im Argument von
`./release.sh <version>`. `cargo xtask bundle` und `make check` hängen nicht daran.

**10. Der Abnahmelauf der zehn Zeitzusagen ist seit dem 260810-1918 nicht mehr gefahren.** Er
liegt damit vor den Runden 5 bis 15. Jener Lauf war der erste vollständig saubere, alle zehn
Zusagen in allen fünf Durchgängen. Der zurückgestellte Datensatz
`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`
wartet weiter auf Läufe an verschiedenen Tagen und fällt als zurückgestellter aus jeder Suche
nach aktiver Grundlage heraus. Die Zusage L7 steht seit dem 260819-2242 wieder auf den
Gegenständen der späteren Messrunde.

**11. Drei Abnahmeläufe stehen weiterhin aus, und alle drei sind Nutzerarbeit.** Die Runde 11 hat
ihre Liste fertig hinterlassen (`messungen/260816-abnahme-inhaltsfilter.md`, 28 Beobachtungen an
vier Orten). Die Runde 10 hat zehn ihrer 77 Abnahmekriterien mit Bündelanteil offen, vier davon
sicherheitsrelevant
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`).
Die Runde 9 hat 21 Kriterien ohne vollständigen Beleg. Kein Marker bewegt sich davon: beschränkt
ist ein Endzustand. Was die Läufe einbringen, ist der Beleg und nicht der Buchstabe.

**12. 152 Defektdatensätze sind offen**, 44 im gemeinsamen Speicher und 108 in den Circles. Der
Bestand ist gegenüber dem Lauf vom 260821-2115 unverändert, weil seither allein Werkstattdaten
angefasst worden sind. Der Bestand:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**13. 36 Entscheidungsdatensätze sind offen und 7 beantwortet, und der Zuwachs bei den
beantworteten ist erklärt.** Der Lauf vom 260821-2115 zählte 36 und 6. Der siebte beantwortete
ist der heutige Absagedatensatz zum Web-Betrachter. Ungeklärt bleibt der Sprung von 14 auf 6
zwischen dem 260820-1044 und dem 260821-2115, den jener Lauf offengelassen hat; dieser Lauf
erhebt ihn nicht neu. Nachzuzählen mit
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_a_*.md'`
