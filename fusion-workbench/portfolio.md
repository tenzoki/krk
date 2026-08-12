# Portfolio

**Generated:** 260812-0816 (by playmaker session 260812-0816-playmaker-direct-dispatch)
**Domain bias:** code

Sechs Circles liegen unter `circles/`: keiner aktiv, einer vorgesehen, fünf beschränkt
abgeschlossen. Kein Circle ist überholt oder zurückgestellt. Die Runde 5, die Bereichsleiste mit
den acht Schaltern, ist am 260812-0820 geschlossen; das Feld der vorgesehenen Circles ist damit
von zwei auf einen geschrumpft.

**Zur Zitierform in dieser Datei.** Jedes Pfadzitat trägt an der Stelle des Zustandsmarkers eine
Sternstelle (`_*_`), weil `portfolio.md` bei jedem Lauf neu entsteht und seine Zitate zwischen
zwei Läufen altern. Ausgenommen sind die Stellen, an denen der Marker selbst die Aussage ist.

## Active (_t_)

**(keiner)**

Kein Circle-Datensatz trägt `_t_`, und `fusion-workbench/.active-circle` ist nicht vorhanden.
Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Warnung: der Orchestrator
löscht den Zeiger bei der Umbenennung auf `_b_`.

Der nächste Schritt liegt beim Nutzer. Er entscheidet über `/fusion:next`, ob der eine vorgesehene
Circle aktiv wird.

## Anticipated (_a_) — ranked

Recommended next: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige
verbliebene Kandidat; seine geerbten Bauteile stehen, aber vor dem Plan liegen eine Klärungsrunde
und eine Untersuchung, welches Mittel Web-Inhalt darstellt.

Diese Empfehlung hat keinen Vergleichswert. Eine Rangfolge mit einem Element sagt nichts über
relative Reife, und der Vorschlag stützt sich deshalb auf die absoluten Signale des einen
Datensatzes. Wer ein Feld mit Auswahl will, legt über `/fusion:direct` einen weiteren vorgesehenen
Circle an; Kandidaten dafür liegen bereit, etwa der Abnahmelauf der zehn Zeitzusagen oder die vier
offenen Defekte, die die Runde 5 hinterlässt.

### Rang 1: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**KRK zeigt Web-Seiten in einem eigenen Betrachter.** Angelegt am 260804-0933, Domain `code`.

Eine Web-Adresse aus der Zwischenablage öffnet KRK künftig selbst, in einem gewöhnlichen Tab des
Vorschaufensters, statt sie an den Systembrowser abzugeben. Bedient wird der Betrachter über die
Tastatur, mit Sprungmarken auf jedem sichtbaren Link. Er speichert keinen Verlauf, trägt kein
dauerhaftes Adressfeld und lädt nichts herunter.

Für den Circle spricht, dass alles, was er erbt, auf der Platte liegt und seit dem letzten
Vorschlag nicht angefasst worden ist: die Auswertung der Zwischenablage in
`crates/krk-core/src/zwischenablage.rs`, das Vorschaufenster in
`crates/krk-ui/src/appkit/vorschau.rs` mit der Tableiste daneben, die Statuszeile in
`crates/krk-ui/src/appkit/statuszeile.rs` und der Befehl `zwischenablage_springen` auf
`opt+cmd+g`. Die beiden zeitlichen Bindungen des Datensatzes, die Schritte S13 und S19 der Runde
1, stehen seit dem 260807. Neu hinzugekommen ist die `Bereichsleiste` der Runde 5, die der
Vorschau einen Schalter am Fensterfuß gibt.

Gegen eine sofortige Aktivierung spricht der Zuschnitt, und daran hat sich seit dem 260804 nichts
gebessert. Der Datensatz hält selbst fest, dass das Mittel der Darstellung von Web-Inhalt offen
ist und in eine eigene Untersuchung vor dem Plan gehört. Daneben liegt die ungemessene
Verfügbarkeitsfrage für macOS-26-Schnittstellen
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`),
die der Datensatz selbst als `inference:` einordnet, und die projektweit offene Frage, ob die
Angabe der macOS-Untergrenze prüfbar gemacht wird
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`).

Der Abschluss der Runde 5 hat eine vierte Frage in die Klärungsrunde gelegt, und sie ist die
inhaltlich schwerste Folge dieses Laufs. Die Mindestbreite der Vorschau von 160 Punkten
(`crates/krk-ui/src/fenstermodell.rs:213`) war bis zur Runde 5 eine Zahl, die nur beim Ziehen der
Trennlinie galt. Seither entscheidet sie zweierlei: ob die Vorschau überhaupt aufgeht, denn
`Fenstermodell::umschalten` (`:639`) weist einen Einschaltbefehl stumm ab, dessen Bereichssatz
nicht mehr in die Fensterzeile passt, und wer beim Schrumpfen weicht, denn `bereichsbreiten`
(`:1044`) nimmt einen Bereich, der unter sein Mindestmaß fiele, aus der Verteilung heraus und
lässt die übrigen den kleineren Rest teilen. Beides ist an der Fensterbreite von 780 Punkten
gedeckelt, die `MINDESTGROESSE` (`crates/krk-ui/src/appkit/fenster.rs:134`) hält. Für die
Vorschau bleibt dort eine Obergrenze von rund 177 Punkten, gerechnet und nicht gemessen; über ihr
geht die Vorschau am schmalsten zulässigen Fenster gar nicht mehr auf. Der Betrachter hat oberhalb
der heutigen 160 also etwa 17 Punkte Luft, und die Zahl gehört dem Bereich und nicht dem Tab: sie
gilt für jeden Vorschau-Tab mit. Die Einzelheiten stehen im Abschnitt `## Parent grounding stale`
des Circle-Datensatzes, angefügt in diesem Lauf.

## Recently closed (_c_ / _b_)

Fünf abgeschlossene Circles, alle beschränkt (`_b_`), keiner kohärent (`_c_`). Neueste zuerst.

**`260811-1304-statusleiste-mit-bereichsschaltern`** — `_b_`, 260812-0820. Die Bereichsleiste am
Fensterfuß steht mit acht Ankreuzfeldern, fünf für die Bereiche und drei für die Spalten Größe,
Datum und Typ; die Breitenregel verteilt Anteile statt Punktzahlen. Beschränkt, weil dreizehn
Abnahmekriterien nur am laufenden Bündel im Vordergrund zu sehen sind.

**`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`** — `_b_`, 260811-2210. Pfad des Ordners
und des Eintrags kopieren, Einträge an das Standardprogramm geben, `cmd+w` aus jedem Fokus.
Beschränkt aus demselben Grund; 23 der 62 Kriterien hat der Abgleich am Baum belegt.

**`260809-2040-tastenbelegung-als-markdown-in-downloads`** — `_b_`, 260811-1415. KRK schreibt die
geltende Tastenbelegung als Markdown nach `~/Downloads/KRK-Tastenbelegung.md`. Beschränkt, weil
der Nutzer den Abnahmeschritt am 260811-1215 gestrichen hat.

**`260807-2116-eingebauter-editor-mit-textmarken`** — `_b_`, 260810-1445. Der Editor als fünfter
Bereich, mit Roh- und Formatansicht, Suchen, Ersetzen und Textmarken. Beschränkt aus zwei Gründen,
die beide beim Nutzer liegen: der Abnahmelauf über 110 Kriterien und die zurückgestellte Frage
nach einem Prüfziel ohne `libtest`-Harness.

**`260802-0842-krk-mac-dateimanager-editor-git`** — `_b_`, 260807-1035. Das Navigator-Gerüst mit
Lesezeichenleiste, zwei Dateifenstern, Vorschau, Dateioperationen und Messmodus. Beschränkt wegen
des Belegstands der zehn Zeitzusagen, nicht wegen der Arbeit.

## Archived (_s_ / _d_)

**(keiner)**

Kein Circle-Datensatz trägt `_s_` (überholt) oder `_d_` (zurückgestellt).

## Warnings

Keine Zeigerlage: kein `STALE-POINTER`, kein `POINTER-MISMATCH`, kein `MULTIPLE-ACTIVE`, kein
`MISSING-POINTER`.

Kein `dependency-cycle-detected`. Der gerichtete Graph über die nicht-terminalen Circles hat einen
einzigen Knoten, den Web-Betrachter, und dessen einzige Kante endet auf der beschränkt
abgeschlossenen Runde 1. Ein Zyklus ist damit ausgeschlossen. Kein Abschnitt
`## Dependency warning` angefügt.

**1. Die Rangheuristik hat bei den Vorbedingungen keine Trennschärfe mehr, und in diesem Lauf
auch keinen Gegenstand.** Fünf von fünf gefahrenen Runden sind beschränkt abgeschlossen (`_b_`),
jedes Mal weil der Abnahmelauf KRK im Vordergrund verlangt und damit Nutzerarbeit ist
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Da allein ein kohärenter Abschluss (`_c_`) als erfüllte Vorbedingung zählt, trägt jeder denkbare
Kandidat dieses Projekts das Kennzeichen der unerfüllten Vorbedingung. Das ist eine Eigenschaft
des Projekts und keine Häufung von Fehlschlägen; als Befund gegen einen Kandidaten gelesen wäre es
falsch. Empfohlen ist, die Vorbedingung an der Beschränkung selbst zu prüfen statt am Marker: bei
diesem Kandidaten bindet sie inhaltlich, weil der offene Beleg der Zeitzusagen über seine dritte
offene Frage in ihn hineinreicht. Der Playmaker ändert die Heuristik nicht; sie sitzt in der
installierten Kopie des Plugins.

**2. Die Kante zwischen der Runde 5 und dem Web-Betrachter läuft nur in eine Richtung.** Der
Abschnitt `## Dependencies` der Runde 5 nennt den Web-Betrachter beim Namen und benennt die
Berührung an der Mindestbreite der Vorschau. Der Datensatz des Web-Betrachters nennt die Runde 5
an keiner Stelle; sein `## Dependencies` führt allein die Runde 1. Ein Graph, der nur aus dem
`## Dependencies` des Web-Betrachters gebaut wird, sieht die Beziehung deshalb nicht. Wer sie
festhalten will, ergänzt den Abschnitt von Hand; der Playmaker schreibt nicht in
`## Dependencies`.

**3. Der Datensatz des Web-Betrachters trägt jetzt je zwei Abschnitte `## Parent grounding stale`
und `## Activation proposal`.** Die älteren stammen vom 260807-1042 und beziehen sich auf die
Runde 1, die neueren aus diesem Lauf auf die Runde 5. Die Abschnitte werden angefügt und nie
umgeschrieben, damit die Vorgeschichte lesbar bleibt. Wer den Datensatz liest, nimmt den jüngeren
Vorschlag; der vom 260807-1042 nennt den Circle ebenfalls den empfohlenen Kandidaten, stützt das
aber auf einen Bestand von damals.

**4. Der Datensatz der Runde 3 trägt im Kopf `**Status:** anticipated` bei Dateiname
`_b_circle.md`.** Unverändert seit dem Lauf vom 260811-1415. Der Marker am Dateinamen ist die
maßgebliche Aussage, die Kopfzeile widerspricht ihm. Der Playmaker schreibt keine Kopfzeilen.

**5. Vier von fünf Runden lassen ihre Abnahmekriterien unabgehakt zurück, und das ist kein
Versehen.** Die Spec-Dateien der Runden 2, 3 und 4 stehen auf `_o_`; die Runde 5 hat gar keinen
Spec, ihre Fähigkeiten und Kriterien stehen im Plan
(`circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_*_bereichsleiste-und-proportionale-breitenregel.md`),
der auf `_c_` steht, während dreizehn seiner Kriterien nur am laufenden Bündel abzunehmen sind.
Wer die offene Arbeit dieses Projekts zählt, zählt an diesen Dateien und nicht an den Markern.

**6. Der Plan der Runde 5 führt drei Wahlpunkte als unabgehakte Kästchen, deren Datensätze
sämtlich auf umgesetzt stehen.** Betroffen sind die Kombinationen der beiden neuen Umschalter, das
Verhalten des Editorschalters ohne Datei und das Verhalten unter der Summe der Mindestbreiten. Die
drei Datensätze in `decisions/` jenes Circles tragen `_i_` samt Commit; die Kästchen im Plan sind
gegenüber ihnen gealtert. Geringes Gewicht, aber ein Leser des Plans hält sie für offen.

**7. Die `## Closure note` der Runde 5 datiert den Abschluss auf 260812-0820, dieser Lauf läuft um
260812-0816.** Der Abschluss steht damit vier Minuten in der Zukunft dieses Laufs. Ohne Folge für
die Reihenfolge in `## Recently closed`, aber die Zahl stimmt an einer der beiden Stellen nicht.

**8. Die Sternform in den Pfadzitaten dieser Datei hält kein Mechanismus.** Der Lauf hat sie von
Hand durchgehalten. Bei jedem künftigen Lauf ist sie erneut von Hand zu prüfen, und eine
Handkorrektur an dieser Datei überlebt den nächsten Lauf nicht.
