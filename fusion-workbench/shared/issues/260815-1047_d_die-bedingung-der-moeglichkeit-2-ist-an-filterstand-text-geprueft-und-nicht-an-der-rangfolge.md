# Die Bedingung der Möglichkeit 2 ist an `filterstand_text` geprüft und nicht an der Rangfolge

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_i_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md` (`## Constraints`, dritter Punkt, und die Zeile `Answered:`); `shared/history/260815-0912-orchestrator-session.md`; `crates/krk-ui/src/appkit/statuszeile.rs:369-386` (`filterstand_text`); `crates/krk-ui/src/appkit/statuszeile.rs:235-242` (`Rang::ALLE`); `crates/krk-ui/src/appkit/statuszeile.rs:516-545` (`zeile`); `crates/krk-ui/src/tabs.rs:543-548` (der Doc-Kommentar, der dieselbe Bedingung ausschreibt)

---

Der Entscheidungsdatensatz knüpft Möglichkeit 2 an eine Bedingung: „Möglichkeit 2
hängt daran, dass der stehende Filtertext zu sehen ist." Vergeben wurde die Antwort
mit dem ausdrücklichen Vermerk, die Bedingung sei „vor der Antwort am Baum geprüft
und erfüllt", belegt mit `crates/krk-ui/src/appkit/statuszeile.rs:369-386`.

**Diese Prüfung hat eine Funktion zu früh aufgehört.** `filterstand_text` baut den
Satz; ob er in die Zeile kommt, entscheidet `zeile` (`statuszeile.rs:516`) über die
Rangfolge `Rang::ALLE` (`statuszeile.rs:235`). Der Filterstand ist dort Rang 5 von
6, und die Ordnung ist zweistellig: **erst der Rang, dann die aktive Seite**. Vier
Ränge stehen über ihm, und drei davon gehören ebenso dem anderen Dateifenster.

```text
Rang 1  Befehlsantwort    beide Seiten   ── vor jedem Kommando geräumt
Rang 2  Vorgangsanzeige   beide Seiten   ── überlebt den Ordnerwechsel mit Absicht
Rang 3  Fenstermeldung    beide Seiten   ── geräumt nur vom Ordner-/Tabwechsel DERSELBEN Seite
Rang 4  Tabmeldung        beide Seiten   ── mit dem neuen Tabinhalt None
Rang 5  Filterstand       <── hier
```

## Der Weg, auf dem der Filterstand dauerhaft verschwindet

`fenstermeldung_loeschen` (`crates/krk-ui/src/appkit/tabelle.rs:2088`) hat genau zwei
Aufrufer, `ordner_lesen` (`tabelle.rs:721`) und `tab_gewechselt` (`tabelle.rs:781`),
und beide räumen allein das Feld **ihres eigenen** Dateifensters. Eine Fenstermeldung
des inaktiven Dateifensters steht damit, bis dieses Dateifenster selbst den Ordner
oder den Tab wechselt — und Rang 3 schlägt Rang 5 unabhängig von der Seite.

Zu erreichen ist die Lage über die Auswurfmeldung aus C9:
`auffrischung::datentraeger_verloren` (`crates/krk-ui/src/auffrischung.rs:349`) setzt
sie über `melden(seite, …)` auf **jeder betroffenen** Seite, also auch auf der
inaktiven. Dieselbe Lage entsteht über jede Startmeldung (beschädigte `keymap.toml`
oder `session.toml`, fehlgeschlagene Dateisystembeobachtung,
`crates/krk-ui/src/appkit/anwendung.rs:1280`, `:2467`), sobald der Nutzer danach das
andere Dateifenster aktiv macht.

Schrittfolge:

1. Eine Fenstermeldung steht im linken Dateifenster.
2. Der Nutzer macht das rechte aktiv und tippt dort einen Filtertext.
3. Die Zeile zeigt „linkes Dateifenster: …" und **nicht** den Filterstand.
4. Jeder weitere Ordnerwechsel im rechten Dateifenster trägt den Filtertext jetzt mit
   (Nutzerentscheid vom 260815-0955), und die Zeile sagt weiterhin nichts über ihn.

Damit steht genau die Lage da, vor der der Datensatz warnt: „Wer filtert, in einen
Ordner steigt und den Filter vergessen hat, hält den neuen Ordner für fast leer."

## Warum es jetzt zählt und vorher weniger

Die Rangfolge ist unverändert seit der Runde 10 (`2d3d971`), der Weg also nicht neu.
Was `897605e` ändert, ist die Häufigkeit: bis dahin überlebte der Filtertext einen
Ordnerwechsel nur bei eingeschaltetem „Deep", seither immer. Ein vergessener Filter,
den die Zeile nicht nennt, war die Ausnahme und ist jetzt der Regelfall.

Der zweite Rang gehört daneben: eine laufende Dateioperation verdrängt den
Filterstand ebenfalls, und das ist ausdrücklich gewollter Entwurf (Modulkopf von
`statuszeile.rs`, Nutzerentscheid vom 260812-1105). Der Unterschied ist die
Lebensdauer — ein Vorgang endet, eine fremde Fenstermeldung nicht.

## Was zu entscheiden ist

Kein Fix drängt sich auf, und die Wahl ist eine Entwurfsfrage für den Nutzer:

1. **Die Fenstermeldung beider Seiten räumen**, sobald irgendeine Seite den Ordner
   oder den Tab wechselt. Billig, bricht aber die Zusage C5.7 („verloren geht dabei
   nichts") und die Begründung, aus der die Meldung stehen bleiben soll, bis sie
   gelesen ist.
2. **Den Filterstand aus der Rangfolge nehmen** und ihm eine eigene Anzeige geben
   (etwa im Fenstertitel oder in der Bereichsleiste neben dem Ankreuzfeld „Deep").
   Teuer, löst die Kollision aber vollständig — und die Zeile bliebe eine.
3. **Nichts ändern und die Lage festhalten.** Dann gehört in den `## Constraints`
   des Entscheidungsdatensatzes und in den Doc-Kommentar von `Tabliste::ordner_setzen`
   der Satz, dass die Sichtbarkeit vier Ränge über sich hat und nicht zugesagt ist.

**Möglichkeit 3 ist keine Verlegenheitslösung**, sondern die ehrliche Form des
heutigen Zustands: die Bedingung des Datensatzes ist als erfüllt vermerkt, und
erfüllt ist sie nur, solange keiner der vier oberen Ränge steht.

---
Deferred: Möglichkeit 3 des Abschnitts `## Was zu entscheiden ist`, Nutzerentscheid vom 260815-1055: die Lage wird festgehalten und nicht behoben. Kein Zieltermin, der Auslöser für ein Wiederaufgreifen ist Gebrauchserfahrung — ob ein unsichtbarer Filtertext im Alltag tatsächlich stört.

Die drei Stellen, an denen der Vorbehalt jetzt ausgeschrieben steht: der Abschnitt `## Constraints` des Entscheidungsdatensatzes `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_i_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`, der Doc-Kommentar von `Tabliste::ordner_setzen` in `crates/krk-ui/src/tabs.rs`, und die Closure-Notiz der Runde 10. Die beiden anderen Möglichkeiten, Fenstermeldungen beider Seiten zu räumen oder dem Filterstand eine eigene Anzeige zu geben, bleiben in diesem Datensatz stehen und sind unverändert erreichbar.
