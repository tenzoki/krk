# In welcher Reihenfolge stehen die Satzteile des Filterstands, und was fällt im schmalen Fenster weg?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `shared/decisions/260816-1310_a_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md` (der Nutzerentscheid, der diese Bauentscheidung hinterlässt); `crates/krk-ui/src/appkit/statuszeile.rs:369-386` (`filterstand_text`), `:314-335` (`Filterstand`), `:668-671` und `:689-695` (Kurzhinweis bei Kürzung), `:126-136` (die benannte Lücke der Messung); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_o_wo-steht-die-filterzahl-in-der-rangfolge-der-einen-statuszeile.md`

---

## Question

Der Nutzerentscheid vom 260816-1330 wählt den Satzteil am Filterstand und schließt einen siebten Rang aus. Er hinterlässt eine Bauentscheidung: die Reihenfolge und die Kürzung der Satzteile im schmalen Fenster.

Der Satz trägt heute zwei Teile, den Kern und einen Zusatz über die ausgeblendeten Markierungen (`statuszeile.rs:369-386`). Er bekommt zwei weitere: einen, solange gelesen wird, und einen für die Dateien, die wegen der 1-MB-Grenze ungelesen blieben. Vier Teile in einer Zeile fester Höhe und veränderlicher Breite.

**Die feste Breite ist als Gegengrund benannt, und der Baum hat dafür schon eine Antwort.** Das Feld der Statuszeile ist einzeilig und bricht nicht um; AppKit kürzt am rechten Rand. `Statuszeile::kurzhinweis_nachziehen` misst nach jedem Setzen des Texts über `sizeToFit`, ob gekürzt wurde, und hängt bei Kürzung den vollen Satz als `NSToolTip` an (`statuszeile.rs:668-671`, `:689-695`). Der volle Satz ist damit erreichbar, auch wenn die Zeile ihn nicht zeigt. Neue Vorrichtung braucht es keine; zu entscheiden bleibt allein, welcher Teil zuerst hinter den Rand rutscht, und das entscheidet die Reihenfolge.

## Options

1. **Kern, Lesehinweis, Größenhinweis, Markierungshinweis.** Gekürzt wird von hinten, also verschwindet der Markierungshinweis zuerst und der Lesehinweis zuletzt.
   - Pro: der Lesehinweis steht unmittelbar hinter der Zahl, die er einschränkt. „38 von 4.812 angezeigt" ist eine Momentaufnahme, und „Inhalt wird gelesen" sagt, dass die 38 noch wachsen; die beiden zu trennen hieße, die Einschränkung von der Aussage zu lösen. Kein heute erzeugbarer Satz ändert sich dadurch: die zwei neuen Teile entstehen allein bei gesetztem „Content", und ohne sie steht der Markierungshinweis wie bisher direkt hinter dem Kern.
   - Kontra: bei gesetztem „Content" rutscht der Markierungshinweis von der zweiten auf die vierte Stelle und fällt im schmalen Fenster als Erster weg.
2. **Kern, Markierungshinweis, Lesehinweis, Größenhinweis.** Die bestehenden zwei Teile bleiben, wo sie sind, die neuen hängen sich an.
   - Pro: die Runde 10 wird an keiner Stelle angefasst.
   - Kontra: der Lesehinweis ist der flüchtigste und zugleich der dringlichste Teil, und er fällt hier als Erster weg. Er stünde außerdem hinter einer Aussage, die mit ihm nichts zu tun hat.
3. **Die Zeile misst und wählt zwischen einer langen und einer kurzen Fassung**, statt sich kürzen zu lassen.
   - Pro: nichts verschwindet unbemerkt.
   - Kontra: eine zweite Fassung jedes Satzteils und eine Auswahlregel darüber, also eine Vorrichtung für eine Frage, die der Kurzhinweis schon beantwortet. `filterstand_text` ist heute eine reine Funktion ohne Fenster; eine Messung zöge die Breite in sie hinein.
4. **Die neuen Teile werden abgekürzt**, etwa `liest` und `12 zu groß`.
   - Pro: der Satz bleibt kurz.
   - Kontra: Kürzel ohne Bezug, gegen `rules/user-facing-output.md`. Der Platzgewinn ist klein und der Verständnisverlust dauerhaft.

## Constraints

- Es bleibt bei **einer** Statuszeile; eine zweite Anzeige daneben ist durch die Runde 6 ausgeschlossen.
- Kein siebter Rang. `Rang::ALLE` und `Rang::art` bleiben unverändert, und der Filterstand bleibt `Art::Vorgang`, also nicht rot (C4.10).
- `filterstand_text` bleibt eine reine Funktion und ohne Fenster prüfbar.
- Der Kurzhinweis wird nur beim Setzen des Texts gemessen und nicht bei einer Fensteränderung. Diese Lücke besteht seit der Runde 6, ist dort benannt und wird von dieser Runde weder behoben noch verschlimmert.

## Recommendation

Möglichkeit 1. Der ausschlaggebende Punkt ist, dass kein heute erzeugbarer Satz sich ändert: die Umstellung wirkt genau dann, wenn einer der beiden neuen Teile dasteht, und die entstehen nur bei gesetztem „Content". Damit ist es eine Regel und keine Fallunterscheidung, und die Runde 10 bleibt in ihrem eigenen Zustand unberührt.

Der Singular bekommt einen eigenen Zweig, wie ihn der Markierungshinweis schon hat: `, eine Datei zu groß` gegen `, 12 Dateien zu groß`.

---
Answered: `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt F2 — Möglichkeit 1: Kern, Lesehinweis, Größenhinweis, Markierungshinweis; gekürzt wird von AppKit am rechten Rand, und der vorhandene Kurzhinweis trägt den vollen Satz.
Implemented: c8fd829 — der Satz traegt vier Teile in der Reihenfolge Kern, Lesehinweis, Groessenhinweis, Markierungshinweis; kein siebter Rang, keine zweite Zeile, keine eigene Kuerzungsvorrichtung. Sechs Proben in `statuszeile.rs` decken alle acht Kombinationen, darunter den Fall null zu grosser Dateien, in dem der Hinweis ausbleiben muss.
Deferred:
Superseded by:
