# Ist die neue Leiste die Statuszeile aus C1 oder eine zweite Fläche darunter?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/appkit/statuszeile.rs`, `crates/krk-ui/src/appkit/aufteilung.rs:374`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` (C1), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`

---

## Question

Der Entwurf spricht von einer "Statusleiste am unteren Rand von KRK, über die ganze Fensterbreite". Eine Statuszeile gibt es bereits, und sie sitzt woanders: `statuszeile.rs` beschreibt sie als "die Statuszeile am Fuß eines Dateifensters", und `aufteilung.rs` legt sie **je Dateifenster innerhalb des Bereichs** an. Es gibt also zwei davon, jede so breit wie ihr Dateifenster, und keine über die Fensterbreite.

C1 der Runde 1 legt diese Zeile als den einen Weg fest, auf dem KRK dem Nutzer eine laufende Meldung zeigt; sie trägt fünf Ränge von der Befehlsantwort bis zum Markierungsstand. Eine zweite Meldefläche wäre ein Bruch damit. Eine Leiste, die nur Schalter trägt, ist keine Meldefläche und bricht nichts. Die Frage entscheidet, welche der beiden Sachen gebaut wird.

## Options

1. **Eine neue Leiste über die Fensterbreite, die ausschließlich Schalter trägt.** Die beiden bestehenden Statuszeilen bleiben, wo sie sind, und behalten alle fünf Ränge.
   - Pros: C1 bleibt unberührt, weil keine zweite Meldefläche entsteht. Der Umfang der Runde bleibt bei dem, was der Entwurf verlangt.
   - Cons: Am Fensterfuß stehen dann zwei Reihen übereinander, wenn man die Statuszeilen der Dateifenster mitzählt. Das Wort "Statusleiste" aus dem Entwurf trifft die neue Fläche nicht mehr, und ein anderer Name wäre klarer.
   - **Folgen weiter unten:** Die Fensterhöhe verliert die Höhe der neuen Leiste, und die Aufteilung rechnet mit einem kleineren Rechteck. Der Aktivierungs-Spec braucht einen Namen für die Fläche, der sie von der Statuszeile unterscheidet.

2. **Eine neue Leiste über die Fensterbreite, die Schalter **und** die Meldungen trägt.** Die beiden Statuszeilen an den Fußenden der Dateifenster entfallen.
   - Pros: Eine Meldefläche für das ganze Fenster statt zwei, und nur eine Reihe am Fensterfuß.
   - Cons: Ein Umbau einer abgenommenen Fähigkeit. Die fünf Ränge sind heute **je Dateifenster** gerechnet, drei ihrer Quellen sind an ein Dateifenster gebunden, und der Markierungsstand kommt aus dem Ordnermodell des sichtbaren Tabs. Bei einer gemeinsamen Zeile ist zu entscheiden, welches der beiden Dateifenster sie beschreibt.
   - **Folgen weiter unten:** Die Rangfolge in `statuszeile::zeile` wird neu gefasst, und der Fortschritt einer Dateioperation aus C4 verliert die Zuordnung zu seinem Dateifenster, die der Nutzer am 260804-1832 ausdrücklich gewählt hat. Die Runde wächst über den Entwurf hinaus.

3. **Keine neue Fläche: die Schalter kommen in die bestehenden Statuszeilen.**
   - Pros: Keine zusätzliche Zeile am Fensterfuß.
   - Cons: Es gibt zwei Statuszeilen. Die Schalter stünden entweder doppelt oder nur in einer von beiden, und beides ist willkürlich. Die Zeile ist 18 Punkte hoch und trägt bereits Text.
   - **Folgen weiter unten:** Bei ausgeblendetem rechtem Dateifenster stünde die zweite Zeile nicht mehr zur Verfügung, und der Ort der Schalter hinge an der Sichtbarkeit eines Bereichs, den sie selbst schalten.

## Constraints

- C1 sagt zu, dass die Statuszeile der eine Weg für laufende Meldungen ist, und dass KRK keine Meldung über die Standardfehlerausgabe gibt.
- L9 aus C8 misst, wie viel einer Dateiliste im ersten Bild steht; die Zusage steht seit dem 260807-1900 bei 65 Prozent. Eine zusätzliche Leiste nimmt der Fensterzeile Höhe und liegt damit auf dem gemessenen Weg.
- Die Höhe der bestehenden Statuszeile ist `statuszeile::HOEHE = 18.0`, eine Zeile in der kleinen Systemschrift.

## Recommendation

**Möglichkeit 1.** Sie liefert, was der Entwurf verlangt, und lässt eine abgenommene Fähigkeit in Ruhe. Möglichkeit 2 ist als eigene Runde denkbar und gehört nicht in diese: sie fasst die Rangfolge und die Zuordnung des Vorgangsfortschritts neu an, und beides ist am 260804 vom Nutzer entschieden worden.

Der Name der neuen Fläche sollte im Aktivierungs-Spec nicht "Statuszeile" lauten, damit zwei verschiedene Sachen nicht denselben Namen tragen.


## Antwort 260812-0306

**Moeglichkeit 1: eine neue Leiste ueber die Fensterbreite, die ausschliesslich Schalter
traegt.** Die beiden bestehenden Statuszeilen an den Fuessen der Dateifenster bleiben, wo sie
sind, und behalten alle fuenf Raenge. C1 der Runde 1 bleibt damit unberuehrt, weil keine zweite
Meldeflaeche entsteht.

**Die Flaeche heisst `Bereichsleiste` und nicht "Statusleiste".** Zwei verschiedene Sachen
duerfen nicht denselben Namen tragen: `statuszeile.rs` beschreibt die bestehende Zeile als "die
Statuszeile am Fuss eines Dateifensters", und die neue Flaeche traegt keine Meldung. Der Name
aus dem Entwurf des Nutzers wird deshalb im Plan ersetzt; die Sache bleibt dieselbe.

Moeglichkeit 2 ist abgelehnt und als eigene Runde denkbar: sie fasst die Rangfolge in
`statuszeile::zeile` und die Zuordnung des Vorgangsfortschritts zu seinem Dateifenster neu an,
und beides hat der Nutzer am 260804-1832 entschieden. Moeglichkeit 3 haengt den Ort der Schalter
an die Sichtbarkeit eines Bereichs, den sie selbst schalten.

**Zur Hoehe, gegen L9 gerechnet:** die neue Leiste nimmt der Fensterzeile Hoehe, und L9 aus C8
misst, wie viel einer Dateiliste im ersten Bild steht. Der Plan haelt die Leiste auf der Hoehe
der bestehenden Statuszeile (`statuszeile::HOEHE = 18.0`, eine Zeile in der kleinen
Systemschrift) und nennt L9 als das Kriterium, das der naechste Abnahmelauf nachmisst.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0306` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung des Circles; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
