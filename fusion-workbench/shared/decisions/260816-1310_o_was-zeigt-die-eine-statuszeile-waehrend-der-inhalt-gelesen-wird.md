# Was zeigt die eine Statuszeile, während der Inhaltsfilter liest?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/appkit/statuszeile.rs:201-241` (die sechs Ränge, vollständige Fallunterscheidung ohne Auffangzweig) und `:376-386` (der Satz des Filterstands); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_*_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md` (die Runde-10-Fassung derselben Frage, weiter offen); `shared/issues/260815-1047_*_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`; `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md` (C4)

---

## Question

Die Statuszeile sagt heute bei stehendem Filter genau einen Satz: `Filter „rs": 38 von 4.812 angezeigt`, dazu bei Bedarf `, 3 Markierungen ausgeblendet`. Der Satz nennt **nicht**, ob noch etwas läuft. Für den Namensfilter über den Unterbaum ist das hinnehmbar, weil ein Durchlauf über Verzeichnismetadaten in aller Regel in Millisekunden durch ist und die Liste sichtbar mitwächst.

Der Inhaltsfilter ändert diese Lage. Er öffnet und liest Dateien, und über einen Unterbaum kann das Sekunden bis Minuten dauern. In dieser Zeit sieht der Nutzer eine Liste, die weiter wächst, und hat keine Auskunft darüber, ob sie fertig ist. Eine kurze Liste bedeutet dann entweder „es gibt nur diese Treffer" oder „das Lesen hat gerade erst begonnen", und die beiden sind für ihn nicht unterscheidbar. Genau diese Unterscheidung ist die Zusage `Befundmeldung`-seitig schon gebaut: `treffer: false` heißt „gelesen, nichts darunter", das Ausbleiben einer Meldung heißt „noch nicht entschieden" (`durchlauf.rs:120-127`). Die Statuszeile gibt diesen Unterschied heute nicht weiter.

Die Frage ist unvermeidlich, weil `Rang` eine vollständige Fallunterscheidung ohne Auffangzweig ist: ein siebter Rang hält den Bau an und erzwingt die Antwort darauf, wo er einzuordnen ist.

## Options

1. **Der Satz des Filterstands bekommt einen Zusatz, solange gelesen wird**, etwa `Filter „rs": 38 von 4.812 angezeigt, Inhalt wird gelesen`. Kein siebter Rang.
   - Pro: hält die Zusage der Runde 6 wörtlich ein, dass Lesefortschritt und Einträgezahl „in dieselbe Zeile und nicht in eine zweite daneben" gehören. Ein Rang bleibt ein Rang, `Rang::art` bleibt unverändert, keine Rangfolge wandert.
   - Kontra: der Satz wird länger und reißt im schmalen Fenster ab. Er erbt daneben die Schwäche, die für die Runde 10 schon festgehalten ist: der Filterstand steht auf Rang 5 von 6, und vier Ränge über ihm verdrängen ihn. Wer filtert, während eine Fenstermeldung steht, sieht auch den Lesehinweis nicht.

2. **Ein siebter Rang „Inhaltsdurchlauf" über dem Filterstand.** Solange gelesen wird, verdrängt er den Filterstand.
   - Pro: ein laufender Vorgang ist eine Meldung und kein Zustand, und die Rangfolge ordnet Meldungen über Zustände. Er stünde damit an derselben Stelle wie die Vorgangsanzeige einer Dateioperation, eine Stufe tiefer.
   - Kontra: die Zahl der gezeigten Einträge verschwindet genau dann, wenn sie am meisten schwankt. Ein siebter Rang ist zugleich der siebte Fall in einer Fallunterscheidung, die vier andere Aufzählungen dieses Baums nachziehen.

3. **Der Fortschritt geht in die Vorgangsanzeige (Rang 2)**, denselben Rang, den eine laufende Kopie benutzt.
   - Pro: kein neuer Rang. Ein Inhaltsdurchlauf ist derselben Art wie eine Dateioperation, nämlich eine Arbeit mit Dauer, die der Nutzer angestoßen hat.
   - Kontra: Rang 2 verdrängt die Fenstermeldung und die Tabmeldung; ein Lesevorgang, der beim Tippen ohnehin ständig neu beginnt, übernähme damit die Zeile fast dauerhaft. Und die Vorgangsanzeige gehört heute der Operationsmaschine allein, mit ihrem eigenen Abbruch und ihrem eigenen Fortschritt.

4. **Es wird nichts angezeigt.** Der Nutzer sieht die wachsende Liste und sonst nichts.
   - Pro: nichts zu bauen, und der Satz bleibt kurz.
   - Kontra: die Unterscheidung „fertig" gegen „läuft noch" ist für den Nutzer nicht herstellbar, und beim Inhaltsfilter ist sie der Unterschied zwischen einer Antwort und einem Zwischenstand. Bei einem Durchlauf über Metadaten war das hinnehmbar, hier nicht.

## Constraints

- Es bleibt bei **einer** Statuszeile. Eine zweite Anzeige daneben ist durch die Runde 6 ausgeschlossen.
- `Rang::ALLE` ist die Rangfolge, und `zeile` läuft ohne zweite Vorschrift über dieses Feld.
- `Rang::art` rechnet die Farbe aus dem Rang. Ein Lesefortschritt ist kein Fehler und darf nicht rot werden.
- Die Zeile hat eine feste Breite. Ein zusammengesetzter Satz reißt im schmalen Fenster ab; das steht in der Runde-10-Fassung dieser Frage unter Möglichkeit 3 als Gegengrund.
- **Ein zweiter Teil der Frage ist von der Größengrenze hierher gewandert.** Der Nutzer hat am 260816 die 1 MB der Vorschau gewählt und damit angenommen, dass eine Protokolldatei von 3 MB für die Suche unsichtbar ist. Möglichkeit 3 jenes Datensatzes hätte den Verlust sichtbar gemacht, indem die Zeile ungelesene Dateien ausweist; die Antwort nennt die Zahl und nicht diesen Zusatz. Ob die Zeile also zusätzlich sagt, wie viele Dateien wegen ihrer Größe ungelesen blieben, gehört zu dieser Antwort. Ohne einen solchen Hinweis hält der Nutzer eine nicht gefundene große Datei für nicht vorhanden.

## Recommendation

Möglichkeit 1, und zwar erweitert um den Größenhinweis aus der Nebenbedingung darüber: ein Satzteil für „es wird gelesen" und einer für „N Dateien sind zu groß". Beide beschreiben denselben Zustand des Filters und gehören in denselben Satz.

Möglichkeit 1. Die Zusage der Runde 6 ist ausdrücklich und wörtlich, und sie hat schon einmal eine Möglichkeit ausgeschlossen (Möglichkeit 4 der Runde-10-Fassung dieser Frage). Der Zusatz ist ein Satzteil und kein Rang, er entsteht und vergeht mit dem Durchlauf, und er braucht keine neue Farbregel. Der Einwand gegen ihn, die feste Breite, trifft Möglichkeit 3 der Runde-10-Fassung genauso und ist dort schon abgewogen worden. Möglichkeit 2 kauft die Sichtbarkeit mit einem siebten Fall in einer Aufzählung, die vier andere nachzieht, und nimmt dem Nutzer dafür die Zahl weg, die sich gerade bewegt.

---
Answered:
Implemented:
Deferred:
Superseded by:
