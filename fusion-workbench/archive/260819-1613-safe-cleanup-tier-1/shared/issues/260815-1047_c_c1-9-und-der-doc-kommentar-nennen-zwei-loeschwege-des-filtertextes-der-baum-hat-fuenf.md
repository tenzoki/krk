# C1.9 und der Doc-Kommentar nennen zwei Löschwege des Filtertextes, der Baum hat fünf

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md:308` (C1.9); `crates/krk-ui/src/tabs.rs:543-548` (Doc-Kommentar von `Tabliste::ordner_setzen`); `crates/krk-ui/src/tabs.rs:440-450` (`verdeckten_tab_setzen`); `crates/krk-ui/src/tabs.rs:504-513` (`Tabliste::schliessen`); `crates/krk-core/src/ablage/sitzung.rs:82-113` (`Tab`, der Zustand in `session.toml`); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/issues/260815-0020_o_verdeckten-tab-setzen-baut-denselben-frischen-tabinhalt-und-traegt-zwei-von-vier-werten-hinueber.md`

---

Zwei Stellen schreiben seit `897605e` dieselbe Aussage als Regel aus, und beide zählen
die Löschwege abschließend auf:

| Stelle | Wortlaut |
|---|---|
| `spec …:308` (C1.9) | „Gelöscht wird er allein durch `Esc` und durch das Zurücknehmen seines letzten Zeichens." |
| `crates/krk-ui/src/tabs.rs:546` | „Geloescht wird er allein vom Nutzer, mit `Esc` oder Zeichen fuer Zeichen ueber die Rueckschritt-Taste." |

**Der Baum kennt drei weitere Wege**, und keiner von ihnen ist eine der beiden
genannten Tasten:

1. **Das Schließen des letzten Tabs** (`tabs.rs:504-513`). Steht nur noch ein Tab und
   zeigt er nicht schon den Standardordner, ersetzt `Tabliste::schliessen` ihn durch
   einen frischen `Tabinhalt::aus_zustand(&Tabzustand::default())`. Damit fallen
   Filtertext und Filter der Tiefe, und mit ihnen Sortierung und Verstecke. Der
   Auslöser ist `cmd+w`, also ein Tastenbefehl des Nutzers, aber keiner der beiden,
   die die Regel nennt.
2. **Der Auswurf eines Datenträgers unter einem verdeckten Tab**
   (`tabs.rs:440-450`). `verdeckten_tab_setzen` trägt Sortierung und Verstecke
   hinüber und lässt Filtertext und Filter der Tiefe fallen. Hier gibt es überhaupt
   keine Handlung des Nutzers am Filter. Der Weg ist als eigener Datensatz offen
   (`…/issues/260815-0020_o_verdeckten-tab-setzen-…`); dort ist er als
   **unentschieden** eingeordnet, weil C1.10 damals „wenn ‚Deep' an ist" sagte. Mit
   der einen unbedingten Regel aus C1.9 ist er nicht mehr unentschieden, sondern ein
   Widerspruch zum Wortlaut. Der Befundtabelle jenes Datensatzes fehlt außerdem die
   Nachführung: sie führt für `ordner_setzen` weiter „ja, wenn der Filter der Tiefe an
   ist".
3. **Der Neustart der Anwendung.** `krk_core::ablage::sitzung::Tab`
   (`sitzung.rs:82-113`) führt Ordner, Auswahl, Verstecke, Sortierung und Bildlauf und
   **weder Filtertext noch Filter der Tiefe**. Ein wiederhergestellter Tab kommt ohne
   Filter zurück. Der Zustand selbst ist stimmig, die Aufzählung der Löschwege ist es
   nicht.

## Warum das der Rede wert ist

Der Doc-Kommentar begründet die Regel damit, dass der Filtertext ein Zustand des
Nutzers ist, den nur der Nutzer wegnimmt. Genau diese Begründung trägt die drei Wege
nicht, und dieses Projekt hält seine Begründungen im Code: eine, die den Baum nicht
mehr beschreibt, kostet später eine Sitzung. Der Datensatz
`shared/issues/260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-…` ist der
gleiche Fehlertyp an anderer Stelle — eine abschließende Aufzählung, die stillschweigend
unvollständig wird.

## Zwei Auswege

1. **Die Aufzählung öffnen.** In C1.9 und im Doc-Kommentar „allein durch `Esc` und die
   Rückschritt-Taste" ersetzen durch die Aussage, die stimmt: kein Ordnerwechsel und
   keine Auffrischung löscht ihn; er fällt mit dem Tab, der ihn hält, und mit der
   Sitzung. Kostet zwei Sätze und ändert am Verhalten nichts.
2. **Die Aufzählung halten und die drei Wege angleichen.** Dann trägt
   `verdeckten_tab_setzen` alle vier Werte, `schliessen` baut den Standardtab mit dem
   Filter des alten, und `Tab` in `sitzung.rs` bekommt zwei Felder. Das ist eine
   Entwurfsfrage für den Nutzer und keine Berichtigung: ob ein Filtertext einen
   Neustart überleben soll, hat er nie beantwortet.

**Weg 1 ist der billigere und ändert kein Verhalten**; Weg 2 setzt eine
Nutzerentscheidung voraus. Zu wählen ist einer von beiden — die Aufzählung so stehen
zu lassen ist der einzige Ausgang, der falsch bleibt.

---
Resolved: Möglichkeit 1 gefahren, die Aufzählung geöffnet statt die drei Wege anzugleichen; Nutzerentscheid vom 260815-1055. C1.9 im Spec der Runde 10 nennt jetzt fünf Wege statt zwei und verweist auf diesen Datensatz; der Doc-Kommentar von `Tabliste::ordner_setzen` ist mit ihm in Übereinstimmung gebracht. `Tabliste::schliessen` und `verdeckten_tab_setzen` sind unangetastet — der zweite Weg bleibt als eigene Entwurfsfrage offen unter `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/issues/260815-0020_o_verdeckten-tab-setzen-baut-denselben-frischen-tabinhalt-und-traegt-zwei-von-vier-werten-hinueber.md`, dessen Befundtabelle und Einordnung dabei nachgezogen wurden.
