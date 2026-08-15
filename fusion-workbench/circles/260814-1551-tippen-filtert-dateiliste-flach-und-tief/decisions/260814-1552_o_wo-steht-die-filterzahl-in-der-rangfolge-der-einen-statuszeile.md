# Wo steht die Filterzahl in der Rangfolge der einen Statuszeile?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/appkit/statuszeile.rs:197-240` (die fünf Ränge); `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` (die Runde, die die eine Zeile gebaut hat)

---

## Question

Die Directive sagt zwei Dinge zu, die beide in die Statuszeile gehören: die gezeigten gegen die vorhandenen Einträge bei stehendem Filter, und die mitzählende Anzeige während eines laufenden tiefen Durchlaufs. KRK hat genau eine Statuszeile, und sie zeigt zu jedem Zeitpunkt genau eine Meldung. Welche das ist, entscheidet die Aufzählung `Rang` mit ihren fünf Werten in fester Rangfolge: Befehlsantwort, Vorgangsanzeige, Fenstermeldung, Tabmeldung, Markierungsstand. Die Aufzählung ist eine vollständige Fallunterscheidung ohne Auffangzweig, und ihr Kommentar sagt ausdrücklich, dass ein sechster Rang den Bau anhält und die Antwort erzwingt, wo er einzuordnen ist. Die Frage ist damit unvermeidlich und fällt nicht nebenbei bei der Umsetzung.

Sie hat einen konkreten Kollisionsfall. Der unterste Rang ist heute der Markierungsstand, und genau der steht typischerweise da, wenn ein Nutzer filtert und markiert. Beide wollen dieselbe Stelle.

## Options

1. **Ein sechster Rang unter dem Markierungsstand.** Die Filterzahl erscheint nur, wenn nichts markiert ist.
   - Pro: kein bestehendes Verhalten ändert sich; der Markierungsstand behält seinen Platz.
   - Kontra: wer filtert und markiert, sieht die Filterzahl nie, und das ist der Fall, in dem sie am meisten trägt. Er sieht nicht, dass die Liste verkürzt ist.
2. **Ein sechster Rang über dem Markierungsstand.** Bei stehendem Filter verdrängt die Filterzahl den Markierungsstand.
   - Pro: dass die Liste unvollständig ist, wiegt schwerer als die Markierungszahl, weil es erklärt, warum ein Eintrag fehlt.
   - Kontra: der Markierungsstand verschwindet genau dann, wenn die Markierung heikel wird, nämlich unter einem Filter. Die zugehörige Frage nach der Markierung unter dem Filter liegt daneben als eigener Datensatz.
3. **Ein Rang, der beide Zahlen in einem Satz trägt**, etwa „38 von 412 gezeigt, 3 markiert".
   - Pro: der Nutzer bekommt beides, und die eine Zeile bleibt eine Zeile.
   - Kontra: verbindet zwei Zustände in einem Rang, die aus verschiedenen Quellen kommen und getrennt entstehen. Die Zeile hat eine feste Breite, und ein zusammengesetzter Satz reißt in einem schmalen Fenster ab.
4. **Die Filterzahl geht nicht in die Statuszeile, sondern in die Tableiste oder den Fenstertitel.**
   - Pro: sie ist ein stehender Zustand und keine Meldung, und die Statuszeile trägt Meldungen. Die Rangfolge bliebe unangetastet.
   - Kontra: bricht die Zusage der Runde 6, dass Lesefortschritt und Einträgezahl „in einer späteren Runde in dieselbe Zeile und nicht in eine zweite daneben" kommen. Der Fenstertitel trägt seit der Runde 8 Name und Version und ist damit belegt.

## Constraints

- Es bleibt bei **einer** Statuszeile. Eine zweite Anzeige daneben ist durch die Runde 6 ausgeschlossen und wäre der Fehler, den jene Runde ausdrücklich vermieden hat.
- `Rang::ALLE` ist die Rangfolge, und `zeile` läuft ohne eine zweite Vorschrift über dieses Feld. Wer die Reihenfolge ändert, ändert die Auswahl der Zeile.
- `Rang::art` rechnet die Farbe aus dem Rang und setzt sie nicht. Eine Filterzahl ist kein Fehler und darf nicht rot werden.
- Die mitzählende Anzeige während eines tiefen Durchlaufs und die stehende Zahl bei fertigem Filter können derselbe Rang sein oder zwei. Auch das gehört zur Antwort.

## Recommendation

Möglichkeit 2. Eine verkürzte Liste ist die Auskunft, ohne die der Nutzer das Fehlen eines Eintrags für einen Defekt hält, und sie wiegt deshalb schwerer als eine Zahl, die er durch Hinsehen ohnehin abschätzen kann. Möglichkeit 1 stellt die Filterzahl genau dort ab, wo sie nie erscheint. Möglichkeit 3 hat unter den vieren den größten Erklärungsbedarf bei schmalem Fenster.

---
Answered:
Implemented:
Deferred:
Superseded by:

---

## Abgleich 260815-1216 (reconciler, Stand `9a2d0e0`)

**Diese Frage ist weder beantwortet noch gegenstandslos geworden, und die Sitzung vom 260815-0912 hat ihr Gewicht erhöht.** Der Marker bleibt `_o_`.

**Der Baum fährt seit der Runde 10 auf der Empfehlung, ohne dass der Nutzer sie bestätigt hätte.** `Rang::ALLE` (`crates/krk-ui/src/appkit/statuszeile.rs:235-242`) trägt sechs Werte, und `Rang::Filterstand` steht auf Platz 5, unmittelbar **über** `Rang::Markierungsstand`. Das ist Möglichkeit 2 dieses Datensatzes. Der Kommentar darüber (`statuszeile.rs:230-233`) zitiert diese Frage und sagt zu, dass eine andere Antwort genau diese eine Zeile verschiebt und sonst nichts.

**Zwei Angaben im Rumpf sind damit überholt** und stehen bewusst unverändert, weil dieser Datensatz die Frage stellt und nicht den Stand beschreibt: der Abschnitt `## Question` spricht von der Aufzählung „mit ihren fünf Werten", der Baum hat sechs; die Kopfzeile `**Cross-references:**` nennt `statuszeile.rs:197-240` als „die fünf Ränge".

**Neu hinzugekommen ist ein zweiter Abhängiger.** Die Sitzung vom 260815 hat den Filtertext jeden Ordnerwechsel überstehen lassen (`decisions/260814-1830_i_bleibt-der-filtertext-…`, Möglichkeit 2, umgesetzt in `897605e`). Die Bedingung dieses Entscheids ist, dass der stehende Filtertext zu sehen ist — und sichtbar ist er nur, solange keiner der vier Ränge über ihm steht. Der Nutzer hat die Lage am 260815-1055 festgehalten statt sie zu beheben (`shared/issues/260815-1047_d_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`, Möglichkeit 3). Damit hängt an der Rangfolge jetzt nicht mehr nur C4.1 der Runde 10, sondern auch die Tragfähigkeit des Filters im gewöhnlichen Gebrauch: ein vergessener Filtertext, den die Zeile nicht nennt, war bis zum 260815 die Ausnahme und ist seither der Regelfall.
