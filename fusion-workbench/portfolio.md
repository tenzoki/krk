# Portfolio

**Generated:** 260812-2307 (by playmaker session 260812-2307-playmaker-direct-dispatch)
**Domain bias:** code

Sieben Circles liegen unter `circles/`: keiner aktiv, **einer** vorgesehen, sechs beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Seit dem Lauf vom 260812-1027 hat
die Runde 6 auf beschränkten Abschluss gewechselt, und damit ist das Feld der Kandidaten von
zwei auf einen geschrumpft. Ein Vergleich zwischen Kandidaten ist in diesem Lauf nicht möglich;
die Empfehlung steht auf absoluten Signalen.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen
zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden.
Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: der Orchestrator
löscht den Zeiger, wenn er einen Datensatz von `_t_circle.md` auf `_b_circle.md` umbenennt.

Der nächste Schritt liegt bei dir. Über `/fusion:next` entscheidest du, ob der eine vorgesehene
Circle aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige nicht
abgeschlossene Circle, und die Runde 6 hat ihm zwei Vorbedingungen günstig hinterlassen statt sie
zu verbrauchen.

### Rang 1 (von 1): `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter an, statt sie an den
Systembrowser abzugeben. Der Betrachter lebt in einem gewöhnlichen Tab des Vorschaufensters,
wird über die Tastatur bedient und trägt Sprungmarken auf jedem sichtbaren Link.

**Abhängigkeiten:** eine Circle-Kante, auf `260802-0842-krk-mac-dateimanager-editor-git` (Runde 1,
beschränkt abgeschlossen). Dazu eine eingehende Kante aus der Runde 6, die der Datensatz des
Betrachters selbst nicht führt; sie steht im Abschnitt `## Dependencies` von
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_*_circle.md`.

**Die Runde 6 hat diesem Circle zwei Vorbedingungen hinterlassen, und beide fallen zu seinen
Gunsten aus.** Die Mindestbreite der Vorschau bleibt bei 160 Punkten
(`crates/krk-ui/src/fenstermodell.rs:213`), womit die rund 17 Punkte Luft bis zur gerechneten
Obergrenze von 177 unverbraucht bleiben; der Datensatz dazu ist
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`
und steht bewusst auf beantwortet statt auf umgesetzt, weil die Antwort darin besteht, nichts zu
ändern. Und die zweite offene Frage des Betrachters, ob lokale HTML-Dateien gerendert erscheinen,
ist ihm ausdrücklich gelassen worden: HTML bleibt Quelltext, und der Datensatz
`.../decisions/260812-1000_*_zeigt-die-vorschau-lokale-html-dateien-gerendert.md` schreibt das
hin, statt es durch Schweigen zu entscheiden. Seine erste Frage, welche Quellen eine Adresse
setzen dürfen, hat die Runde 6 nicht berührt; sie hat keine Web-Ansicht gebaut und
`zwischenablage_springen` nicht angefasst. Der Zuschnitt dieses Circles ist damit vollständig
erhalten geblieben, und das ist das tragende Signal für Rang 1.

**Was gegen eine sofortige Aktivierung spricht, ist unverändert der Zuschnitt.** Der Datensatz
hält selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene
Untersuchung vor dem Plan gehört. Eine Untersuchung ist teurer als eine Klärungsrunde, und dieser
Circle braucht beides. Dazu die ungemessene Verfügbarkeitsfrage für macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
die der Datensatz selbst als erschlossen und nicht als geprüft einordnet.

**Die Klärungsrunde trägt jetzt fünf Fragen statt vier.** Zu den drei Fragen des Abschnitts
`## Grounding snapshot` und der Mindestbreite aus dem Lauf vom 260812-0816 kommt die Schriftgröße
der Vorschau
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-oder-waechst-sie-auf-die-des-editors.md`,
offen). Die Frage entscheidet, wie viele Zeichen in eine Vorschau von 160 Punkten passen, und
hängt damit an derselben Zahl wie die Mindestbreite.

**Zur Abhängigkeitslage, die in diesem Projekt nichts unterscheidet.** Die einzige
Circle-Abhängigkeit dieses Datensatzes ist beschränkt abgeschlossen (`_b_`) und nicht kohärent
(`_c_`), also trägt er nach der Rangheuristik das Kennzeichen der unerfüllten Vorbedingung. Alle
sechs gefahrenen Runden tragen `_b_`, und alle sechs aus demselben Grund: der Abnahmelauf
verlangt KRK im Vordergrund und ist Nutzerarbeit
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Das Kennzeichen steht damit an jedem denkbaren Kandidaten und ist keine Auskunft über diesen.
Inhaltlich bindet die Beschränkung hier dennoch, siehe Warnung 5 zur gealterten Messreihe.

## Recently closed (_c_ / _b_)

Die fünf jüngsten Abschlüsse, neueste zuerst. Alle sechs gefahrenen Runden sind beschränkt
abgeschlossen; kohärent abgeschlossen ist keine.

| Circle | Marker | Abschluss | Abschlussnotiz in einem Satz |
|---|---|---|---|
| `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` | `_b_` | 260812 | Elf Planschritte gebaut, fünf Fähigkeiten im Baum, 25 Commits und 478 Proben; nicht abgenommen, weil siebzehn Kriterien nur am laufenden Bündel zu sehen sind. |
| `260811-1304-statusleiste-mit-bereichsschaltern` | `_b_` | 260812-0820 | Die Breitenregel verteilt Anteile statt Punktzahlen, die Bereichsleiste trägt acht Ankreuzfelder; dreizehn Kriterien stehen am Bündel aus. |
| `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` | `_b_` | 260811-2210 | Vier Tastenbefehle für Pfade kopieren und Öffnen, dazu das App-Symbol; der Abnahmelauf blieb offen. |
| `260809-2040-tastenbelegung-als-markdown-in-downloads` | `_b_` | 260811-1415 | Die Tastenbelegung erscheint als Markdown in `~/Downloads`; der Abnahmelauf über 41 Kriterien wurde gestrichen. |
| `260807-2116-eingebauter-editor-mit-textmarken` | `_b_` | 260810-1445 | Der Editor steht als fünfter Bereich mit Roh- und Formatansicht und Textmarken; 110 Kriterien und zwei Restdefekte blieben beim Nutzer. |

Nicht mehr in dieser Tabelle, weil älter als die fünf jüngsten:
`260802-0842-krk-mac-dateimanager-editor-git` (Runde 1, `_b_` am 260807-1035).

## Archived (_s_ / _d_)

**(keiner)** — kein Circle-Datensatz trägt `_s_` oder `_d_`.

## Warnings

**1. `CLAUDE.md` ist zwei Runden alt und nennt einen Kandidaten, den es nicht mehr gibt.** Der
Abschnitt am Ende der Datei sagt: „Zwei Circles sind vorgesehen und nicht gefahren (Marker
`_a_`): die Statusleiste mit Bereichsschaltern (`260811-1304-…`) und der Web-Betrachter
(`260804-0933-…`). Die Statusleiste steht auf Rang 1." Die Statusleiste ist als Runde 5 gefahren
und am 260812-0820 beschränkt abgeschlossen worden; ihr Datensatz trägt `_b_`. Ebenso sagt die
Datei „Vier Runden sind gefahren", gefahren sind sechs. Wer die Rangfolge aus `CLAUDE.md` liest
statt aus dieser Datei, sucht einen Kandidaten, der keiner mehr ist. Der Playmaker ändert
`CLAUDE.md` nicht; `/fusion:revise-claude-md` ist der Weg.

**2. Die Zahl der offenen Defekte in der Abschlussnotiz der Runde 6 stimmt nicht mehr.** Sie
nennt 26 im Circle und 6 im gemeinsamen Speicher, zusammen 32. Am 260812-2307 gezählt sind es
39: 26 im Circle der Runde 6, 8 im gemeinsamen Speicher und 5 im Circle der Runde 5, die die
Notiz nicht mitrechnet. Die beiden zusätzlichen im gemeinsamen Speicher hat der Abgleich vom
260812-2253 nach der Notiz abgelegt. Verbindlich ist der Dateibestand:

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

**3. Der Abnahmelauf der Runde 6 steht aus, und er ist Nutzerarbeit.** Siebzehn Kriterien sind
nur am laufenden `KRK.app` im Vordergrund zu sehen; die Aufstellung steht im Plan
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_*_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`
unter `## Abnahme am laufenden Bündel`. Der schwerste ist C1.1: `NSSharingService.h:270` verlangt
für `showRelativeToRect:ofView:preferredEdge:` einen Mausdruck, und KRK ruft die Methode aus
einem Tastendruck. Der Ausweichweg über ein eigenes `NSMenu` ist gebaut. Ein Lauf, der nur den
Rechtsklickweg prüft, beantwortet die Frage nicht. Drei weitere Kriterien (C4.5, C1.8, C6.6) sind
im Baum wahr, aber ungemessen.

**4. KRK läuft auf keinem zweiten Mac.** Das Bündel ist mit einer Entwicklungsidentität signiert,
und Gatekeeper weist es auf jeder anderen Maschine ab; das Developer-ID-Zertifikat fehlt im
Schlüsselbund. `cargo xtask bundle` meldet die gelungene Signatur, ohne die Folge zu nennen, und
baut daneben nicht universell, sondern für die Architektur der Baumaschine. Der gebaute und
dokumentierte Weg zur Weitergabe ist `cargo xtask release`. Datensatz:
`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`.

**5. Die Messreihe, aus der der Web-Betrachter eine eigene Zeitzusage ableiten müsste, ist in der
Runde 6 schlechter geworden.** Seine dritte offene Frage leitet eine mögliche elfte Zusage aus
L5 und L7 ab. L7 wird bei tief verschachtelten Listen jetzt ab rund 12 kB verfehlt statt ab
19 kB
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/260812-2133_*_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-mehr-als-der-rest-der-zerlegung.md`).
Die Zahl der Zusage ist unverändert, der Abstand zu ihr nicht. Dazu steht L9 aus zwei Runden zum
Nachmessen an: die Bereichsleiste der Runde 5 nimmt der Fensterzeile 18 Punkte, die zusammengelegte
Statuszeile der Runde 6 gibt jedem Dateifenster 18 zurück, und gemessen ist keine der beiden
Rechnungen.

**6. Fünf Fragen binden die nächste Runde, und zwei binden jede Runde.** Zwölf Fragen stehen über
alle Speicher offen. Unmittelbar bindend sind drei aus der Runde 6: der Rechtsklick bei
Markierung anderswo
(`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1516_*_hebt-ein-rechtsklick-auf-eine-unmarkierte-zeile-die-markierung-anderswo-auf.md`),
die Schriftgröße der Vorschau (`.../260812-1707_*_bleibt-die-vorschau-bei-der-kleinen-systemschriftgroesse-…`)
und der Vorspann eines Containers als Lücke in der Deckungszusage von C4.3
(`.../260812-2002_*_bleibt-der-vorspann-eines-containers-die-eine-luecke-in-der-deckungszusage-von-c4-3.md`).
Jede Runde binden: wie KRK für den Abnahmelauf in den Vordergrund kommt
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`)
und ob die Angabe der macOS-Untergrenze prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`).

**7. Keine Zeigerwarnung, kein Zyklus.** `.active-circle` fehlt und kein Datensatz trägt `_t_`;
das ist der reguläre Zustand nach einem Abschluss. Der gerichtete Graph über die nicht-terminalen
Circles hat einen einzigen Knoten und keine Kante zu einem anderen nicht-terminalen Knoten. Kein
`dependency-cycle-detected`.

**8. Ein Vermerk `## Parent grounding stale` ist angefügt worden**, an
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`, wegen des
Abschlusses der Runde 6. Die wörtliche Auslösebedingung greift nicht: der Abschnitt
`## Grounding snapshot` jenes Datensatzes zitiert weder den Verzeichnisnamen der Runde 6 noch
ihren Abschluss-Artefakt. Die Kante läuft in die andere Richtung, aus dem Abschnitt
`## Dependencies` der Runde 6. Die Abweichung von der wörtlichen Regel steht hier und im Vermerk
selbst, damit sie nachlesbar ist.
