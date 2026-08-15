# Durchsicht: der Filtertext übersteht jeden Ordnerwechsel

---
**Domain:** code
**Sender:** coderev
**Reviewed-range:** `c3fcdef..897605e`
**Not-opened:** none
**Cross-references:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_i_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`; `shared/history/260815-0912-orchestrator-session.md`; `shared/history/260815-1019-coder-filtertext-uebersteht-jeden-ordnerwechsel.md`

---

## Zusammenfassung

Die Änderung selbst ist richtig und knapp: `Tabliste::ordner_setzen`
(`crates/krk-ui/src/tabs.rs:564-590`) trägt den Filtertext jetzt unbedingt in das neue
`Ordnermodell`, `filtertext_ueberlebt` ist entfallen, und vier Übertragungen stehen in
derselben Bauart nebeneinander. Die drei Proben halten, `make check` läuft grün
(Exit 0, am 260815-1047 nachgefahren). **Kein Befund gilt der Codezeile.**

Vier Befunde gelten dem, was um sie herum steht: eine Bedingung, an der der
Nutzerentscheid hängt und die an der falschen Stelle geprüft wurde; zwei abschließende
Aufzählungen, die der Baum nicht einhält; zwei normative Texte, die das alte Verhalten
weiter festschreiben; und vier Verweise, die ins Leere zeigen.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 3 |
| Gering | 1 |

Kein Befund hält die Auslieferung auf.

## Die fünf Fragen des Auftrags, beantwortet

### 1. Deckt die Regel jeden Weg in `ordner_setzen` hinein? — Ja, und daneben liegen zwei Wege, die nicht hineinführen

Jeder Weg, auf dem der **sichtbare** Tab seinen Ordner wechselt, läuft über
`DateifensterQuelle::ordner_lesen` (`crates/krk-ui/src/appkit/tabelle.rs:720`) und von
dort über `ordner_setzen`. Nachgezählt sind es sieben Rufer: der Einstieg
(`tabelle.rs:1361`), der Aufstieg (`tabelle.rs:1389`), die Pfadeingabe und die
Zwischenablage (`tabelle.rs:1454`), der Auswurf, wenn er den sichtbaren Tab trifft
(`tabelle.rs:619`), das Lesezeichen (`anwendung.rs:1596`), der Ordnersprung
(`anwendung.rs:2593`, `:3207`) und die Geräteliste (`anwendung.rs:6267`, `:6346`,
`:6351`). **Die Behauptung des Doc-Kommentars, der Aufstieg brauche keine eigene Zeile,
hält am Baum.**

Zwei Stellen daneben tauschen einen `Tabinhalt` aus, ohne durch `ordner_setzen` zu
gehen:

```text
 Tabinhalt neu gebaut in …          Sortierung  Verstecke  tief  Filtertext
 ordner_setzen        (tabs.rs:564)     ja          ja       ja      ja
 verdeckten_tab_setzen(tabs.rs:440)     ja          ja      NEIN    NEIN
 schliessen, letzter  (tabs.rs:504)    NEIN        NEIN     NEIN    NEIN
 oeffnen, neuer Tab   (tabs.rs:487)    NEIN        NEIN     NEIN    NEIN
```

`oeffnen` ist unstrittig: ein neuer Tab ist ein neuer Tab. `schliessen` setzt beim
letzten Tab bewusst alles zurück, ist in sich stimmig und widerspricht allein der
Aufzählung der Löschwege (Befund 2). `verdeckten_tab_setzen` ist **der zweite Weg, den
der Auftrag gesucht hat** — und er ist bereits erfasst:
`circles/260814-1551-…/issues/260815-0020_o_verdeckten-tab-setzen-baut-denselben-frischen-tabinhalt-und-traegt-zwei-von-vier-werten-hinueber.md`.
Nicht neu gemeldet, aber zweifach nachzuziehen: die Befundtabelle jenes Datensatzes
führt für `ordner_setzen` weiter „ja, wenn der Filter der Tiefe an ist", und seine
Einordnung als **unentschieden** stützte sich auf den alten Wortlaut von C1.10
(„wenn ‚Deep' an ist"). Mit der unbedingten Regel aus C1.9 ist der Weg nicht mehr
unentschieden, sondern ein Widerspruch zum Wortlaut.

### 2. Die Sitzungswiederherstellung — der Kommentar stimmt, der Zustand ist stimmig

`krk_core::ablage::sitzung::Tab` (`crates/krk-core/src/ablage/sitzung.rs:82-113`) führt
Ordner, Auswahl, Verstecke, Sortierung und Bildlauf. **Weder `filtertext` noch `tief`**,
geprüft an der Strukturdefinition. Ein wiederhergestellter Tab kommt ohne Filter und
mit `tief = false` zurück, also in einem Zustand, den die Oberfläche vollständig
anzeigt. Kein Befund am Verhalten; der Neustart zählt allerdings als dritter Löschweg
gegen die Aufzählung in C1.9 (Befund 2).

### 3. Die drei Proben — sie halten, was ihre Namen sagen

Alle drei laufen durch (`cargo test -p krk-ui --bin krk`, nachgefahren). Zur
Aufstiegsprobe im Einzelnen: sie rechnet `krk_core::verzeichnis::aufwaerts` selbst und
ruft `ordner_setzen(&eltern, Some(verlassen))`. Das ist **genau** das, was
`DateifensterQuelle::ordner_aufwaerts` (`tabelle.rs:1386`) tut — dessen Rumpf sind
dieselben zwei Zeilen, und der Umweg über `ordner_lesen` fügt
`fenstermeldung_loeschen`, `nach_lesebeginn` und `ordnerwechsel_melden` hinzu, von
denen keines den Filtertext berührt. **Die Probe trägt dieselbe Aussage wie der echte
Aufstieg.**

Eine Deckungslücke bleibt und ist keine Nachlässigkeit: `krk-ui` hat kein
Bibliotheksziel, `ordner_aufwaerts` ist eine private Methode eines AppKit-Typs, und
eine Probe kann sie nicht rufen. Baute jemand `ordner_aufwaerts` künftig auf eine
eigene Übertragung um, statt `ordner_lesen` zu rufen, bliebe die Probe grün. Das ist
die bekannte Kiste-ohne-Bibliotheksziel-Grenze (`CLAUDE.md`, „Was man nicht sieht") und
kein neuer Defekt.

### 4. Ist eine Aussage im Baum falsch geworden? — Ja, vier Stellen

Die Behauptung des `coder`, außerhalb von `tabs.rs` schreibe keine **Probe** das alte
Leeren fest, hält: nachgeprüft über `Ordnerwechsel`, `Filtertext`, `geleert`, `leert`
im ganzen Code. Der Kommentar in `crates/krk-core/tests/verzeichnis.rs:1687` ist unter
der neuen Regel unverändert richtig.

Die Werkbank hat dieselbe Suche nicht bekommen. Zwei normative Texte sagen weiter das
Gegenteil (Befund 3), dazu kommen die Aufzählung der Löschwege (Befund 2) und vier
Verweise (Befund 4).

### 5. Sichtbarkeit des stehenden Filters — hier liegt der Hauptbefund

**Ja, eine höherrangige Meldung kann den Filterstand verdrängen, und in einem Fall
dauerhaft.** Die Bedingung, an der der Nutzerentscheid ausdrücklich hängt, ist an
`filterstand_text` geprüft worden und nicht an `zeile`, der Funktion, die entscheidet,
ob dessen Satz die Zeile überhaupt erreicht. Ausgeschrieben in Befund 1.

Was **nicht** verdrängt: nach einer gewöhnlichen Navigation stehen Rang 1
(`befehlsantwort`, vor jedem Kommando geräumt), Rang 3 (`fenstermeldung`, von
`ordner_lesen` geräumt) und Rang 4 (`tabmeldung`, mit dem neuen `Tabinhalt` `None`)
leer, und `nach_lesebeginn` zeichnet die Zeile neu. Der Filterstand erscheint
unmittelbar, auch während der Lesevorgang noch läuft: das frische Modell hat eine leere
Sichtreihenfolge, also liefert `ersetzt_beim_naechsten_stapel` `false`
(`crates/krk-core/src/verzeichnis/modell.rs:262`), und die Sperre aus C4.7 greift
richtigerweise nicht.

## Die Befunde

### Befund 1 — Die Bedingung der Möglichkeit 2 ist an der falschen Stelle geprüft (Mittel)

Datensatz:
`shared/issues/260815-1047_o_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`

Der Filterstand ist Rang 5 von 6. Eine Fenstermeldung des **inaktiven** Dateifensters
steht auf Rang 3 und wird allein vom Ordner- oder Tabwechsel **derselben** Seite
geräumt (`crates/krk-ui/src/appkit/tabelle.rs:2088`, zwei Aufrufer, beide seitenlokal).
Steht sie, sieht der Nutzer im aktiven Dateifenster seinen stehenden Filtertext
überhaupt nicht — auch nicht in dem Augenblick, in dem er ihn tippt, und über jeden
folgenden Ordnerwechsel hinweg. Erreichbar über die Auswurfmeldung aus C9
(`crates/krk-ui/src/auffrischung.rs:349`, meldet auf **jeder** betroffenen Seite) und
über jede Startmeldung, sobald der Nutzer danach das andere Dateifenster aktiv macht.

Der Weg ist unverändert seit der Runde 10. Was `897605e` ändert, ist die Häufigkeit:
ein vergessener Filter über einen Ordnerwechsel hinweg war die Ausnahme und ist jetzt
der Regelfall.

### Befund 2 — Zwei Löschwege genannt, fünf vorhanden (Mittel)

Datensatz:
`shared/issues/260815-1047_o_c1-9-und-der-doc-kommentar-nennen-zwei-loeschwege-des-filtertextes-der-baum-hat-fuenf.md`

C1.9 (`spec …:308`) und `crates/krk-ui/src/tabs.rs:546` zählen die Löschwege
abschließend auf: `Esc` und die Rückschritt-Taste. Der Baum kennt drei weitere: das
Schließen des letzten Tabs (`tabs.rs:504`), den Auswurf unter einem verdeckten Tab
(`tabs.rs:440`) und den Neustart. Zwei Auswege, und der billigere ändert kein Verhalten.

### Befund 3 — Directive und Planschritt schreiben das alte Leeren fest (Mittel)

Datensatz:
`shared/issues/260815-1047_o_die-directive-der-runde-10-und-ein-planschritt-schreiben-das-alte-leeren-weiter-fest.md`

`_b_circle.md:14` — die laut `CLAUDE.md` verbindliche Formulierung — sagt weiter „wird
beim Ordnerwechsel geleert". Der `Changes`-Block von Schritt B2
(`plan …:340-341`) widerspricht sich in zwei aufeinanderfolgenden Zeilen und nennt
außerdem `let filtertext_ueberlebt = tief;` als das, was „zu einem `true` wird" — die
Zeile ist entfallen.

### Befund 4 — Vier Verweise zeigen ins Leere, drei davon neu (Gering)

Datensatz:
`shared/issues/260815-1047_o_vier-verweise-im-code-nennen-einen-marker-den-ihr-ziel-nicht-mehr-traegt-drei-davon-sind-neu.md`

275 Zitate im Code stehen in der Sternform `_*_`, 15 nennen einen ausgeschriebenen
Marker, vier davon sind falsch. Drei sind mit `897605e` entstanden und binnen derselben
Sitzung falsch geworden: sie nennen `260814-1830_a_…`, während der Datensatz auf `_i_`
läuft. Dazu ein falscher Typname im Doc-Kommentar der neuen Probe
(`Dateifenster::ordner_aufwaerts`, die Methode gehört `DateifensterQuelle`).

## Was quer liegt

**Die eine Wurzel unter den Befunden 1, 3 und 4 ist dieselbe: die Codeänderung ist
sauber nachgezogen, ihr Umfeld nicht.** Der `coder` hat den Doc-Kommentar, den
Rumpfkommentar, die drei Proben, den Spec und den Plan angefasst und dabei jede Stelle
berichtigt, die er gesucht hat. Gesucht hat er in den Proben und in den beiden
Planungsdateien. Nicht gesucht hat er im Circle-Datensatz, im `Changes`-Block desselben
Planschritts und in den Verweisen, die er selbst neu schrieb. Das ist kein Muster
mangelnder Sorgfalt, sondern eines zu enger Suchmuster — und `CLAUDE.md` warnt für
denselben Fehlertyp bereits vor Suchmustern, die `\.md` verlangen.

**Der zweite Querschnitt ist die abschließende Aufzählung.** Befund 2 und Befund 4 sind
derselbe Fehlertyp an verschiedenen Stellen: eine Liste, die vollständig gemeint ist und
still unvollständig wird. Dieses Projekt hat den Typ schon dreimal erfasst — die
Variantenzahl von `Kommando` in `CLAUDE.md`, die Untergrenzen-Quote, die sieben Verweise
im Circle-Datensatz der Runde 5. Der Baum wehrt sich gegen die Bauart an anderer Stelle
mit vollständigen Fallunterscheidungen ohne Auffangzweig; in Prosa hat er dieses Mittel
nicht.

## Beobachtungen ohne Befundstatus

- **Der Markerlauf `_a_` → `_i_` des Entscheidungsdatensatzes liegt unverfolgt im
  Arbeitsbaum.** `git status` zeigt den `_a_`-Namen gelöscht und den `_i_`-Namen
  ungetrackt. Der Lauf ist inhaltlich richtig (die `Implemented:`-Zeile nennt `897605e`);
  er ist nur nicht eingecheckt.
- **Der Kopf des Entscheidungsdatensatzes trägt weiter `**Status:** open`**, während der
  Dateiname `_i_` sagt. Das ist der siebte Fall des bereits erfassten
  `shared/issues/260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`
  — nicht neu gemeldet.
- **C1.10 ist nach der Umschreibung kein Abnahmekriterium mehr**, sondern eine Auskunft
  über die Geschichte des Specs („ist dabei keine Ausnahme mehr, sondern ein Fall von
  C1.9"), und seine Probe-Angabe verweist auf die von C1.9. Die Probe
  `mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel` trägt trotzdem eine
  eigene Aussage und soll bleiben. Kein Befund, aber beim nächsten Anfassen des Specs
  eine Zeile wert.
- **`shared/history/260815-1019-coder-…` verweist auf
  `…260815-1019_o_die-wettrennprobe-des-oeffnens-faellt-im-profil-debug-…`; die Datei
  heißt heute `…reisst-die-15-sekunden-schranke-in-beiden-profilen.md`.** Der Verweis
  ist tot und der Inhalt widerspricht ihm (die Aufzeichnung sagt, im Profil `release`
  laufe die Probe in 4,66 s durch). `history/` behält seinen damaligen Stand, also kein
  Befund an dieser Datei — wohl aber ein Hinweis für den, der jenen Defekt anfasst.

## Reihenfolge

Nichts hält die Auslieferung auf. Sinnvoll ist:

1. **Befund 3** zuerst, weil eine verbindliche Formulierung, die das Gegenteil des
   Codes sagt, die nächste Planung in die falsche Richtung schickt. Reine Textarbeit.
2. **Befund 4** danach, weil er billig ist und drei der vier Verweise mit dem
   ausstehenden Markerlauf gerade erst falsch werden.
3. **Befund 2** braucht eine Wahl zwischen zwei Wegen; der billigere ist Textarbeit.
4. **Befund 1** ist die einzige Entwurfsfrage und gehört dem Nutzer. Drei Möglichkeiten
   stehen im Datensatz, und die dritte — die Lage festhalten und nichts bauen — ist
   eine ernstgemeinte.
