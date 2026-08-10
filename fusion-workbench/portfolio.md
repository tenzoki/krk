# Portfolio

**Generated:** 260810-1439 (by playmaker session 260810-1439-playmaker-direct-dispatch)
**Domain bias:** code

## Active (_t_)

(keiner)

Kein Circle-Datensatz trägt die Marke `_t_` für aktiv, und `fusion-workbench/.active-circle`
fehlt. Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Störung. Die
Editor-Runde wurde am 260810-1445 mit beschränktem Abschluss geschlossen, und der Zeiger
wurde dabei geräumt.

## Anticipated (_a_) — ranked

**Recommended next:** `260809-2040-tastenbelegung-als-markdown-in-downloads` — die Ausgabe der
Belegung baut eine zweite Ausgabeform an einer bestehenden Aufbereitung, ihre Grundlage ist
einen Tag alt und rechnet die Editor-Runde schon ein, und ihre fünf offenen Fragen sind
Nutzerwahlen und keine Untersuchungen.

**Die Rangfolge hat sich gegenüber dem Lauf vom 260807-2125 gedreht, und der Grund ist der
Abschluss der Editor-Runde.** Jener Lauf folgte einer festgehaltenen Nutzerwahl vom
260807-1930, die den Editor gegen den Web-Betrachter stellte. Der Editor hat sie gewonnen
und ist geschlossen; die Wahl ordnet damit kein Feld mehr, in dem er nicht mehr steht. Der
Circle für die Belegungsausgabe entstand erst am 260809-2040 und stand in jenem Vergleich
gar nicht zur Wahl. Eine Aussage über zwei Kandidaten ordnet keine drei, und eine Wahl über
die Reihenfolge der beiden verbleibenden liegt nicht vor. Die Rangfolge dieses Laufs steht
deshalb auf dem Dateibestand.

**Die Gewichtung `code` zählt in die andere Richtung, und dieser Lauf unterschlägt es nicht.**
Sie bevorzugt vorgesehene Circles mit wenigen unbeantworteten Fragen. Nach diesem Maß liegt
der Web-Betrachter vorn: sein Grounding zitiert einen offenen Entscheidungsdatensatz, das
Grounding der Belegungsausgabe fünf. Der Zählwert misst hier die falsche Größe, und der
Abschnitt zu Rang 1 sagt, warum.

### 1. `260809-2040-tastenbelegung-als-markdown-in-downloads`

**Directive:** KRK schreibt die Tastenbelegung, die im Augenblick des Aufrufs gilt, als
Markdown-Datei in den Downloads-Ordner des Nutzers. Sie entsteht aus derselben Belegung, aus
der die Belegungsansicht ihre Zeilen bezieht, und stellt keine zweite Aufbereitung daneben.
Ein fertiges Druckbild sagt KRK nicht zu.

**Abhängigkeiten:** eine, `260802-0842-krk-mac-dateimanager-editor-git`, beschränkt
abgeschlossen (`_b_`) am 260807-1035. Die Vorbedingung ist damit gekennzeichnet und nicht
erfüllt, denn als erfüllt zählt allein der kohärente Abschluss (`_c_`). Beide vorgesehenen
Circles tragen dieses Kennzeichen; siehe die erste Warnung unten.

**Warum die fünf offenen Fragen kein Nachteil sind.** Sie liegen alle in `decisions/` dieses
Circles, tragen `_o_` und sind die eigenen Aktivierungsfragen des Circles: wie die Ausgabe
ausgelöst wird, wie die Datei heißt und was bei einer vorhandenen geschieht, was in der
Ausgabe steht und wonach sie gegliedert ist, ob der Wirkungsbereich mitkommt, und welche
Belegung bei offener Belegungsansicht gilt. Jede führt ihre Möglichkeiten und eine Empfehlung
des Shapers, jede ist aus dem Dateibestand beantwortbar, und die fünfte wird gegenstandslos,
wenn die erste auf ein gewöhnliches Kommando fällt. Eine Klärungsrunde mit dem Nutzer räumt
sie ab. Der eine Datensatz des Web-Betrachters ist von anderer Art: die Verfügbarkeitsprüfung
für Schnittstellen ab macOS 26 ist eine ungemessene technische Frage, und derselbe Circle
hält fest, dass das Mittel der Darstellung von Web-Inhalt offen ist und in eine eigene
Untersuchung vor dem Plan gehört. Ein Zählwert von eins verdeckt dort mehr ungeöffnete Arbeit
als ein Zählwert von fünf hier.

**Was auf der Platte liegt, am 260810-1439 nachgeprüft.** Die Belegung führt jede Funktion
genau einmal mit allen ihren Kombinationen (`crates/krk-core/src/tasten/belegung.rs`,
`Belegung::funktionen()`). Die Gliederung nach neun Funktionsbereichen steht an einer Stelle
(`crates/krk-ui/src/belegungsmodell.rs:73`, `Funktionsbereich` samt dem Wert `Editor`). Die
Beschriftung einer Kombination hat eine einzige Quelle (`belegungsmodell.rs:517`,
`anzeige()`). `resources/default-keymap.toml` trägt 71 Blöcke `[[funktion]]`, also genau die
Zahl, die das Grounding nennt. Das Modul `belegungsmodell.rs` spricht keine
AppKit-Schnittstelle an; eine Ausgabefunktion daneben ist ohne Fenster prüfbar.

**Die Grundlage kennt den Stand nach dem Editor.** Sie wurde am 260809-2040 geschrieben,
während die Editor-Runde lief, und rechnet deren Änderungen ein: dreizehn neue Funktionen,
der neunte Funktionsbereich, und der Nachschlag für Buchstaben und Ziffern über das gemeldete
Zeichen statt über den Tastencode. Sie warnt davor, eine der drei bewegten Zahlen fest zu
verdrahten. Der Vermerk `## Parent grounding stale` im Datensatz hält fest, was der Abschluss
daran ändert: die Zahlen stehen jetzt still, und die Warnung bleibt für spätere Runden
richtig.

**Die einzige echte Unbekannte** ist der Zugriff auf `~/Downloads`. Ungemessen ist, ob macOS
bei einem Schreibvorgang, den KRK selbst anstößt, eine Rückfrage zeigt und wie ein
abgelehnter Zugriff aussieht. Der Grounding-Abschnitt führt den Punkt als `speculation:` und
verlangt einen Prüflauf am gebauten Bündel im Aktivierungs-Spec. Geprüft sind dagegen der
Schlüssel `NSDownloadsFolderUsageDescription` in `resources/Info.plist` und die Auslieferung
außerhalb der App-Sandbox.

**Eine kleine Vorarbeit.**
`shared/issues/260810-0805_o_ein-verweis-nennt-den-falschen-circle-und-die-zustellerregel-liegt-woanders.md`
sitzt in einem der fünf Entscheidungsdatensätze dieses Circles und sollte vor der
Klärungsrunde berichtigt sein, damit der Nutzer einem Verweis folgen kann, der trägt.

### 2. `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter im Vorschaufenster an,
statt sie an den Systembrowser abzugeben. Bedient wird er über die Tastatur, mit Sprungmarken
auf jedem sichtbaren Link.

**Abhängigkeiten:** dieselbe eine, `260802-0842-krk-mac-dateimanager-editor-git`, mit
demselben Kennzeichen wegen des beschränkten Abschlusses. Hier trägt es inhaltlich weiter als
bei Rang 1: der Vermerk `## Parent grounding stale` vom 260807-1042 zeigt, dass die
Beschränkung über die dritte offene Frage dieses Circles in ihn hineinreicht.

Nach der reinen Zählung offener Entscheidungsdatensätze ist dieser Circle der reifere. Drei
Befunde am Dateibestand sprechen dagegen. Erstens ist sein Zuschnitt der größere: er hebt
einen ausdrücklichen Ausschluss der Runde 1 auf und überholt ein abgenommenes
Abnahmekriterium der Fähigkeit C10. Zweitens legt er kein Mittel der Darstellung fest und
verweist selbst auf eine Untersuchung vor dem Plan. Drittens stammt seine Grundlage vom
260804 und beschreibt das Vorschaufenster so, wie die Runde 1 es hinterließ, während die
Editor-Runde genau diese Fläche verändert hat; siehe die dritte Warnung unten.

## Recently closed (_c_ / _b_)

Zwei Circles sind geschlossen, beide als beschränkter Abschluss (`_b_`); die Vorlage sieht bis
zu fünf vor. Keiner der beiden trägt den kohärenten Abschluss (`_c_`).

**1. `260807-2116-eingebauter-editor-mit-textmarken`** — Marke `_b_`, geschlossen am
260810-1445. Der Editor steht als fünfter Fokusbereich mit Roh- und Formatansicht,
Zeilensprung, Suchen, Ersetzen und Textmarken in derselben Leiste und derselben Ablagedatei
wie die Ordner-Lesezeichen; alle 48 Planschritte tragen `[DONE]`, und die 53 Defekte der
Runde sind abgearbeitet. Beschränkt ist der Abschluss, weil der Abnahmelauf über die 110
Abnahmekriterien des Specs KRK im Vordergrund verlangt und damit Nutzerarbeit ist, und weil
zwei Restdefekte an der Frage hängen, ob `krk-ui` ein Bibliotheksziel bekommt. Der Artefakt
der Beschränkung sind drei Erkenntnisse, die die Directive nicht verlangte, angeführt von
der Textfläche, die auf TextKit 1 stehen muss und das seit `bf0fe18` selbst herstellt.

**2. `260802-0842-krk-mac-dateimanager-editor-git`** — Marke `_b_`, geschlossen am
260807-1035. Das Navigator-Gerüst steht mit allen 38 Planschritten auf `[DONE]`, am Code
belegt. Beschränkt ist der Abschluss wegen des Belegs und nicht wegen der Arbeit: sieben der
zehn Zeitzusagen stehen unverändert auf der Abnahmereihe vom 260805-2207, und drei spätere
Commits haben die gemessenen Wege berührt. Der Artefakt: eine Messreihe altert an jedem
Commit, der einen gemessenen Pfad berührt, und sie sagt es nicht selbst.

## Archived (_s_ / _d_)

(keiner)

Kein Circle-Datensatz trägt `_s_` für überholt oder `_d_` für zurückgestellt.

## Warnings

**1. `beide-abschluesse-beschraenkt` — kein Circle dieses Projekts hat je den kohärenten
Abschluss erreicht.** Beide geschlossenen Runden tragen `_b_`, und terminale Zustände sind
nicht rückholbar. Daraus folgt eine Eigenschaft der Rangfolge, die keine Bewertung der
beiden Kandidaten ist: das Kriterium „alle Abhängigkeiten stehen auf `_c_`" kann in diesem
Projekt derzeit von keinem Circle erfüllt werden, weil jeder vorgesehene Circle auf einen
beschränkt abgeschlossenen aufsetzt. Das Kennzeichen ist damit für die Unterscheidung der
Kandidaten wertlos und muss inhaltlich gelesen werden, Circle für Circle.

Beide Beschränkungen haben denselben Kern, und er ist eine offene Frage:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`.
Der Abnahmelauf verlangt KRK im Vordergrund und ist damit keine Agentenarbeit. Solange diese
Frage offen ist, endet jede Runde, die Abnahmekriterien am laufenden Bündel führt, wieder
beschränkt. Wer die Kette brechen will, beantwortet diese Frage, nicht die Rangfolge.

**2. `parent-grounding-stale: parent=260809-2040-tastenbelegung-als-markdown-in-downloads
child=260807-2116-eingebauter-editor-mit-textmarken`.** Das Grounding der Belegungsausgabe
führt die Editor-Runde in zwei Zeilen als aktiv und ihre Änderungen als laufend. Der Vermerk
steht im Datensatz des vorgesehenen Circles unter `## Parent grounding stale` und hält
außerdem fest, dass die drei bewegten Zahlen jetzt stillstehen und am Code nachgeprüft sind.
Er hält die Aktivierung nicht auf.

**3. Die Grundlage des Web-Betrachters ist gealtert, ohne den Vermerk auszulösen.** Der
Datensatz `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` nennt die Editor-Runde
an keiner Stelle, denn er entstand drei Tage vor ihr. Der Fortpflanzungsvermerk greift
deshalb mechanisch nicht, und dieser Lauf hat dort nichts angefügt. Sachlich ist die
Grundlage dennoch überholt: sie beschreibt das Vorschaufenster als eine von drei Flächen mit
dem Halteverhalten aus C6, während die Editor-Runde daraus einen von fünf Fokusbereichen
gemacht hat, ihm Zeilennummern gegeben hat und den Editor ihn zeitlich verdrängen lässt. Der
Betrachter soll laut Directive in einem Tab genau dieses Fensters wohnen.

Ein Artefakt der Editor-Runde ist für ihn außerdem einschlägig, ohne dass er ihn zitieren
könnte: ein stehendes Blatt hält Tastenbefehle beim Anwendungsdelegierten an und nicht über
den Fokusvorbehalt. Die zweite Möglichkeit seiner ersten offenen Frage ist eine
Adresseingabe „als Blatt am Fenster, wie die Pfadeingabe aus C2". Wer diesen Circle
aktiviert, liest die `## Closure note` der Editor-Runde dazu.

**4. Der Spec der Editor-Runde trägt `_o_`, während ihr Plan auf `_c_` steht und der Circle
geschlossen ist.** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`
führt im Kopf `**Status:** Gebaut, wartet auf den Abnahmelauf des Nutzers`; der Plan
`260808-0140_c_plan-eingebauter-editor-mit-textmarken.md` trägt dagegen die Marke für
geschlossen. Die Runde 1 hat Spec und Plan beide auf `_c_` gezogen. Die Zeile
`**Active spec/plan:**` im Circle-Datensatz nennt den Spec mit festem Marker `_o_` und würde
mit einer Umbenennung ins Leere laufen. Ob der Spec bis zum Abnahmelauf offen bleiben soll
oder mit dem Circle schließt, ist eine Nutzerentscheidung; der Playmaker ändert keinen Plan
und keinen Spec.

**5. Elf offene Defekte liegen in terminalen Circles und haben keinen Träger.** Sechs stehen
in `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/`, fünf in
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/`, dazu drei im gemeinsamen
Speicher. Ein terminaler Circle nimmt keine Arbeit mehr auf, und keiner der beiden
vorgesehenen Circles deckt diese Defekte mit seiner Directive. Zwei der sechs hängen laut
`## Closure note` an der Frage nach dem Bibliotheksziel für `krk-ui`
(`decisions/260810-1044_o_…`), die jede Datei jener Kiste berührt. Genau diese Lage hat nach
der Runde 1 den Defekt über die zweiundzwanzig überholten Verweise erzeugt. Verbindlich ist
der Dateibestand:

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'
```

**6. Drei Verweise in lebenden Dokumenten der Editor-Runde zeigen auf `_t_circle.md`**, das
mit dem Abschluss zu `_b_circle.md` geworden ist: der Spec in Zeile 6, der Plan in den Zeilen
21 und 1615. Die Plan-Zeile 1615 nennt die Folgeänderung selbst und schreibt sie dem
Orchestrator zu. Es ist dieselbe Sorte, die der offene Defekt
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md`
für die Runde 1 führt, und dessen Suchmuster erfasst die Form `_t_circle.md` nicht. Der
Playmaker berichtigt keine Zitate.

**7. Keine Abhängigkeitsschleife.** Der gerichtete Graph über die Abschnitte
`## Dependencies` der beiden nicht terminalen Circles hat zwei Kanten, beide auf
`260802-0842-krk-mac-dateimanager-editor-git`, und dieser Knoten ist terminal und zeigt auf
nichts. Es gibt keinen Zyklus.

**8. Der Zeigerzustand ist regulär.** `fusion-workbench/.active-circle` fehlt, und kein
Datensatz trägt `_t_`. Das ist der Zustand nach einem Abschluss; keine der Bedingungen
`STALE-POINTER`, `POINTER-MISMATCH`, `MULTIPLE-ACTIVE` oder `MISSING-POINTER` liegt vor.
