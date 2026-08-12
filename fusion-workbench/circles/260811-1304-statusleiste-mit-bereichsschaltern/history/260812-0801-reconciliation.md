# Abgleich der fünften Runde gegen den Baum

**Datum:** 260812-0801
**Agent:** reconciler
**Domäne:** code
**Circle:** `260811-1304-statusleiste-mit-bereichsschaltern` (aktiv)
**Spanne:** `6b6ea3c..caeaa18`, zehn Commits
**Abnahme:** `make check`, Exit 0. KRK ist nicht gestartet worden.

---

## Ergebnis in einem Satz

Die Runde hält, was ihr Plan behauptet: acht Schritte, 27 mit **(Probe)** gekennzeichnete
Kriterien, vierzehn Entscheidungsdatensätze auf umgesetzt und zwölf geschlossene Defektdatensätze
sind einzeln gegen den Baum gelesen, und **eine** Zusage reicht weiter als der Code (C4.9). Ein
neuer Befund ist entstanden, vier offene Defekte sind zu Recht offen, und acht Aussagen in
CLAUDE.md sind veraltet.

---

## Was geprüft wurde und wie

`make check` mit `PATH="$HOME/.cargo/bin:$PATH"`: vierzehn Prüfziele, 0 Fehlschläge. Die beiden
größten sind 392 Proben im Binärziel `krk` und 142 im Kern; dazu `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings`, beide ohne Befund. Ein Bündelbau war für
den Abgleich nicht nötig — das Protokoll zu Schritt 8 weist `make bundle` mit Exit 0 aus, und keine
Änderung danach hat die Bündelbeschreibung angefasst.

Die (Bündel)-Kriterien sind **nicht** geprüft. Sie verlangen KRK im Vordergrund, und das ist in
diesem Projekt Nutzerarbeit (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`).
Sie sind kein Befund dieses Abgleichs, sondern der Grund, aus dem diese Runde wie die vier davor
als beschränkter Abschluss schließt.

## Die acht Planschritte

Alle acht tragen `[DONE]` zu Recht. Die Belegtabelle steht im Plan selbst, unter
`## Reconciliation Log`, damit sie neben dem Schritt liegt, den sie belegt. Zusammengefasst:

- **Schritte 1 bis 3** (proportionale Regel, Abweisung an den Mindestbreiten, ausblendbares linkes
  Dateifenster) stehen vollständig in `crates/krk-ui/src/fenstermodell.rs` und
  `crates/krk-core/src/ablage/sitzung.rs`, mit allen im Plan benannten Bauteilen: `Zeilenmass`,
  die zweizweigige `bereichsbreiten`, `mindestbreiten_passen`, `Sichtbarkeit::erstes_dateifenster`.
- **Schritte 4 bis 7** (fünf Belegungseinträge, zwei Umschaltbefehle, `Spalte` als reine
  Aufzählung, Spaltensichtbarkeit) stehen ebenso. `crates/krk-ui/src/spalten.rs` führt keine
  `use objc2`-Zeile; das war die Zusage des Schrittes 6.
- **Schritt 8** (Bereichsleiste) steht mit 616 Zeilen in
  `crates/krk-ui/src/appkit/bereichsleiste.rs`, samt der einen Höhenkonstante
  (`HOEHE = statuszeile::HOEHE`), `setRefusesFirstResponder(true)` an jedem Schalter und einem
  einzigen Aufrufer von `bereichsleiste_nachziehen`.

Die drei im Protokoll zu Schritt 8 selbst benannten Abweichungen von der Änderungsliste
(`controlSize` als eigene Untergrenze, `NSColor` nicht im Modulkopf, zwei Fallunterscheidungen
statt einer Feldtabelle) und die eine zu Schritt 6 (`titel` liefert `Retained<NSString>`) stehen am
Baum, wie sie dort beschrieben sind. Kein Protokoll behauptet etwas, das der Baum nicht trägt.

## Die 27 (Probe)-Kriterien

26 treffen zu. Eines trifft eingeschränkt zu:

**C4.9 sagt mehr zu, als der Baum hält.** Der Wortlaut lautet „verschieben die Trennlinie weiterhin
um genau einen Schritt von 40 Punkten", ohne Bedingung. Hängt ein anderer sichtbarer Bereich an
seinem Mindestmaß, kommt weniger an — bei 800 Punkten Fensterbreite gemessene 20,36 statt 40. Der
Sachverhalt ist bekannt und abgelegt (`issues/260812-0700_o_…`), und der Doc-Kommentar an
`massstab` (`crates/krk-ui/src/fenstermodell.rs:850`) trägt die Grenze ausdrücklich. Was fehlt, ist
die Grenze **am Kriterium**: die Probe, die es abnimmt, misst drei Fensterbreiten ohne gedeckelten
Bereich. Weg 1 jenes Datensatzes ist damit nicht nur eine Ergänzung, sondern eine Berichtigung an
einem geltenden, als **(Probe)** gekennzeichneten Kriterium. Der Befund ist im Datensatz und im
Plan vermerkt.

## Die vierzehn Entscheidungsdatensätze mit `_i_`

Jede Zeile `Implemented:` trifft zu. Die Commits tragen, was sie behaupten:

| Datensatz | Commit | Am Baum |
|---|---|---|
| was-heisst-proportional | `5e17c9e`, `026c665` | `bereichsbreiten` verteilt Anteile; die Vorrangordnung vom 260808 ist samt ihrer Zusage im Kommentar entfallen |
| welchen-anteil-nie-sichtbar | `5e17c9e` | `breite_oder_anfang` (`fenstermodell.rs:935`) |
| was-geschieht-wenn-die-mindestbreiten | `a2ea876` | `mindestbreiten_passen` (`:685`), nur beim Einschalten |
| traegt-das-linke-dateifenster-einen-schalter | `8ffaac2`, `0342445` | fünftes Feld in `Sichtbarkeit`, fünfter Schalter in der Leiste |
| ist-die-neue-leiste-die-statuszeile | `0342445` | eigene Fläche; `appkit/statuszeile.rs` ist in dieser ganzen Runde nicht angefasst worden |
| wie-zeigen-zwei-schalter-eine-flaeche | `0342445` | zwei Ankreuzfelder, Ausschluss allein in `Bereich::teilt_flaeche_mit` |
| wird-der-vorschaubreiten-defekt-behoben | `1ea5a3d` (Runde 4) | trifft zu, und der Datensatz sagt es von sich aus: „Dieser Circle hat die Behebung vorgefunden und nicht gebaut" |
| bekommen-die-spaltenschalter-tastenbefehle | `90b02d4` | drei Kommandos, `tasten = []`, nicht in der Markdown-Ausgabe; die Einschränkung steht in der Zeile selbst |
| gelten-die-spaltenschalter-gemeinsam | `90b02d4` | ein `Spaltensichtbarkeit`, `spaltenanzeige_nachziehen` schreibt beide Tabellen |
| ueberstehen-die-spaltenschalter-einen-neustart | `90b02d4` | `Sitzung::spalten`, `#[serde(default)]`, dreimal `true` |
| was-geschieht-mit-der-sortierung | `90b02d4` | `setHidden` an der Spalte; Probe `das_wegschalten_der_sortierspalte_laesst_die_sortierung_stehen` |
| was-geschieht-unter-der-mindestsumme | `5e17c9e`, `0342445` | zweiter Zweig der Regel; `MINDESTGROESSE` in der Breite unverändert 780 |
| was-tut-der-editorschalter-ohne-datei | `90b02d4` | `editor_umschalten` verwirft still über `editor_ist_ansprechbar` |
| welche-kombinationen | `90b02d4` | `opt+cmd+left`, `opt+cmd+b`, `opt+cmd+right`; `editor_schliessen` bleibt auf `opt+cmd+e` |

Eine Genauigkeit zum vorletzten Eintrag: die Zeile nennt `0342445` für eine **Nicht**-Änderung
(die Breite bleibt bei 780). Das ist zulässig — jener Commit fasst `MINDESTGROESSE` an und lässt
die Breite dabei stehen —, aber es ist die einzige Zeile der vierzehn, die einen Commit für etwas
zitiert, das nicht geschehen ist. Wer sie später liest, findet den Grund im Abschnitt `## Antwort
260812-0430` desselben Datensatzes.

## Die zwölf geschlossenen Defektdatensätze

Der Auftrag nannte elf; im Verzeichnis stehen zwölf mit `_c_`. Der zwölfte ist
`260811-1732_c_die-leiste-soll-auch-die-spalten-groesse-datum-und-typ-wegschalten.md`, und das ist
kein Defekt, sondern der Nachtrag des Nutzers. Elf Defekte plus ein Nachtrag geht auf.

**Keiner ist vorschnell geschlossen.** Am Baum nachgelesen, in der Reihenfolge des Verzeichnisses:

1. **Nachtrag Spaltenschalter** — drei Schalter in der Leiste, drei Kommandos, ein Feld je Spalte
   in der Ablage. Vier Fragen als eigene Datensätze beantwortet.
2. **Breitenschritt skaliert** — `massstab` (`fenstermodell.rs:850`) und der dritte Parameter an
   `breite_aendern` (`:788`).
3. **Drei Proben ohne Planschritt** — `OHNE_KOMBINATION_AB_WERK` (`krk-core/tests/belegung.rs:75`),
   von zwei Proben gelesen.
4. **Breitenbefehle in die falsche Richtung** — Ober- und Untergrenze einzeln, mit vorgezogener
   Feststellung (`fenstermodell.rs:812-818`).
5. **Doppelte Zuordnung Bereich auf Sichtbarkeit** — `aufteilung::sichtbar_im` entfallen.
   **Ein Rest ist geblieben, siehe den neuen Befund unten.**
6. **Zusammengezogenes Fenster** — `traegt_eine_ziehbewegung` vor der Rückrechnung
   (`fenstermodell.rs:909`).
7. **Probenname nach der abgeschafften Zweiteilung** —
   `ein_bereich_ohne_fensterseite_aendert_nur_seine_eigene_breite` (`:1922`).
8. **`make check` rot seit Schritt 4** — grün, Exit 0, in diesem Abgleich erneut gefahren.
9. **Modulkopf von `spalten.rs`** — `beschreibbar` ist ein ausgeschriebenes `match`
   (`spalten.rs:106`).
10. **Nachzug zweimal** — `Leistenquelle::geklickt` (`bereichsleiste.rs:282`) nimmt die
    Selbstkippung zurück; `bereichsleiste_nachziehen` hat genau einen Aufrufer
    (`anwendung.rs:2780`).
11. **Spaltenbefehle in der Markdown-Ausgabe** — berichtigt sind die drei Zusagen, nicht der Code;
    `belegungsausgabe::markdown` filtert unverändert auf belegte Funktionen
    (`belegungsausgabe.rs:178`).
12. **Doppelte Erreichbarkeitsprüfung** — `editor_ist_ansprechbar` (`anwendung.rs:1488`), von
    beiden Aufrufern gefragt.

## Die vier offenen Defekte

Alle vier sind zu Recht offen; jeder trägt jetzt einen Abschnitt `## Abgleich 260812-0801` mit dem
Beleg.

- **`260812-0415_o`** (Spalte und Schlüssel als zwei Aufzählungen): beide stehen weiter
  nebeneinander. **Ein Ort im Datensatz ist veraltet** — `Spalte` liegt seit Schritt 6 nicht mehr
  in `appkit/tabelle.rs:179`, sondern in `spalten.rs:33`. Der Umzug macht den vorgeschlagenen
  billigen Weg leichter, nicht schwerer.
- **`260812-0512_o`** (F4 am schmalen Fenster): `im_editor_oeffnen` (`anwendung.rs:3747`) fragt das
  Fenstermodell weiterhin nicht. **Weg 2 ist inzwischen versperrt**: der Nutzerentscheid vom
  260812-0430 lässt `MINDESTGROESSE` in der Breite bei 780. Weg 1 ist der einzige tragbare und seit
  dieser Runde billiger, weil `mindestbreiten_passen` die Frage schon beantwortet, nur privat.
- **`260812-0700_o`** (Breitenschritt neben einem gedeckelten Bereich): unverändert; siehe C4.9
  oben.
- **`260812-0810_o`** (die Zahl 39): `default-keymap.toml:319` steht unverändert im Präsens. Die
  Zeile darüber ist nachgezählt und stimmt.

## Neuer Befund

**`issues/260812-0801_o_zwei-modulkoepfe-nennen-aufteilung-sichtbar-im-das-es-nicht-mehr-gibt.md`**
(Schwere niedrig). `crates/krk-ui/src/spalten.rs:12` und `crates/krk-ui/src/appkit/tabelle.rs:185`
führen im Präsens „Dasselbe Muster tragen `aufteilung::sichtbar_im` und `aufteilung::rahmenfarbe`".
Die zweite Funktion steht (`aufteilung.rs:414`), die erste ist mit `026c665` entfallen. Der
Übersetzer hat es nicht gefunden, weil beide Namen in einfachen Backticks stehen und nicht als
Doc-Verweis; dieselben Sätze nennen `Bereich` und `tabelle` sehr wohl als eckige Verweise. Die
dritte Fundstelle (`fenstermodell.rs:301`) steht im Rückblick und bleibt richtig.

## Was diese Runde an CLAUDE.md veraltet hat

**Nur festgestellt, nichts geändert.** Die Datei ist geschützt und gehört dem Nutzer.

| Zeile | Was jetzt nicht mehr stimmt |
|---|---|
| 7 | Die Aufzählung der Bestandteile nennt keine Leiste am Fensterfuß und keine schaltbaren Spalten. |
| 11 | „Vier Runden sind gefahren" samt der vierzeiligen Tabelle. Es sind fünf. |
| 28 | „Keine der Runden 2 bis 4 hat eine elfte Zahl gesetzt oder eine der zehn angefasst." Für die Runde 5 gilt dasselbe — der Plan sagt es ausdrücklich —, aber die Spanne im Satz endet bei 4. |
| 32 | „Die Anwendung trägt: …" — die Bereichsleiste mit acht Schaltern fehlt in der Liste. Der Satz „Geprüft am 260811-2230" ist der Stand vor dieser Runde. |
| 54 | „Alle vier Runden sind als beschränkter Abschluss geschlossen" — es werden fünf. |
| 66 | Die vier Zählwerte: `Kommando` trägt **73** Varianten statt 68. Die anderen drei stimmen weiter, am 260812-0801 nachgezählt: `Wirkungsbereich` sieben, `Bereich` fünf, `Fokus` fünf. |
| 136 | „steht am 260811 in **31 von 33** Dateien" — es sind **32 von 34**. `bereichsleiste.rs` ist dazugekommen und trägt den Abschnitt; ohne ihn sind weiterhin nur `koordinaten.rs` und `mod.rs`. |
| 156, 158 | „Außerhalb der vier gefahrenen Runden …" und „**Zwei Circles sind vorgesehen und nicht gefahren**: die Statusleiste … und der Web-Betrachter". Die Statusleiste ist gefahren; vorgesehen bleibt einer. |

**Zwei der im Auftrag genannten Kandidaten gibt es in CLAUDE.md nicht**, und das ist kein
Versehen des Abgleichs, sondern der Bestand:

- **Ein Absatz über das linke Dateifenster** steht dort nicht. Die Aussage, die diese Runde
  überholt hat, stand im Code — im Kommentar an `Sichtbarkeit`
  (`crates/krk-core/src/ablage/sitzung.rs`) und im Modulkopf von
  `crates/krk-ui/src/fenstermodell.rs`, Abschnitt „Was das linke Dateifenster von den anderen
  unterscheidet". Beide sind mit Schritt 3 (`8ffaac2`) umgeschrieben; am Baum nachgelesen, sie
  sagen jetzt „eines bleibt".
- **Eine Aussage über `MINDESTGROESSE`** steht dort ebenfalls nicht. Sie steht im Circle-Datensatz
  `_t_circle.md`, Abschnitt `## Parent grounding stale`, Punkt 3 („steht auf 780 Punkten", gemeint
  780 mal 300) und in der Ausgangslage dieses Plans. Am Baum sind es jetzt 780 mal 318
  (`crates/krk-ui/src/appkit/fenster.rs:134`), und die Zeilennummer im Plan (`fenster.rs:116`) ist
  ebenfalls gewandert. Beides sind Aufzeichnungen eines Standes und behalten ihn nach der Ortsregel
  in CLAUDE.md; nachzuziehen ist nichts.

**Die Fensterzeile trägt weiterhin fünf Bereiche.** Der Satz in Zeile 58 stimmt unverändert: die
Bereichsleiste ist keine Unteransicht der `NSSplitView`, sondern ihre Schwester unter einer neuen
Inhaltsfläche (`fenster::fensterinhalt`, `fenster.rs:289`). Wer sie als sechsten Bereich in
CLAUDE.md schriebe, schriebe etwas Falsches.

## Geänderte Dateien dieses Abgleichs

- Plan umbenannt `_p_` → `_c_` und um `## Reconciliation Log` ergänzt
- vier offene Defektdatensätze um `## Abgleich 260812-0801` ergänzt
- zwei Durchsichtsdokumente um `## Abgleich 260812-0801 (reconciler)` ergänzt
- ein neuer Defektdatensatz
- dieses Protokoll
- `## Coherence` an `shared/history/260812-0306-orchestrator-session.md`

Kein Code, keine Daten, keine Beschreibung eines Plans oder Datensatzes ist geändert worden.
