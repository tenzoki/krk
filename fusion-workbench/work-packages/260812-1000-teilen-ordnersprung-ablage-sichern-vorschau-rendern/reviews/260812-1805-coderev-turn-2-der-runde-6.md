# Codedurchsicht: Turn 2 der Runde 6 — gerenderte Vorschau, eine Statuszeile

**Sender:** coderev
**Reviewed-range:** `34ab5b5..05797d7`
**Not-opened:** none

Alle sechzehn Code- und Manifestdateien des Bereichs sind geöffnet:
`markdown.rs`, `textmerkmale.rs`, `statuszeile.rs` und `appkit/vorschau.rs`
vollständig, die übrigen als vollständiger Unterschied samt der Umgebung jeder
geänderten Stelle. Dazu die zwölf Werkbank-Dokumente des Bereichs: fünf
Sitzungsprotokolle, drei Circle-Defektdatensätze, zwei gemeinsame
Defektdatensätze, der Entscheidungsdatensatz zur Schriftgröße und der
umbenannte Plan.

---

## Zusammenfassung

Die fünf Planschritte sind sauber gebaut. `make check` läuft am Baum durch
(`fmt`, `clippy -D warnings`, 903 Proben, Exit 0), jede der neuen und
geänderten macOS-Untergrenzen stimmt gegen das SDK, `Cargo.lock` führt
weiterhin kein `cc` und außer `windows-sys` kein `-sys`-Paket, und der
Einfärbungsvorgang wohnt nachweislich in der Ansicht, so dass L7 nicht auf
`syntect` wartet. **Der Schwerpunkt der Befunde liegt an einer Stelle: die
Zerlegung von Markdown ist nicht so total, wie Plan und Modulkopf sie
behaupten.** Quelltext, zu dem der Zerleger kein Ereignis mit Zeichen liefert,
verschwindet aus der Anzeige, statt als sein Quelltext zu erscheinen. Daneben
steht ein Befund an der Statuszeile: die eine Zeile zeigt auch Meldungen eines
Dateifensters, das gar nicht dasteht.

Kein Befund hält den Abschluss der Runde auf. Zwei sollten vor dem
Abnahmelauf am Bündel behoben sein, weil sie sonst dort als Fehlbefund
erscheinen.

## Zahlen

| Gewicht | Zahl |
|---|---|
| Kritisch (Freigabeblocker, Sicherheit, Datenverlust) | 0 |
| Hoch (Korrektheitsfehler, gebrochener Ablauf) | 0 |
| Mittel (Korrektheitsrisiko, Wartbarkeit) | 4 |
| Niedrig (Kosmetik, Aufräumen) | 5 |

Alle neun sind als eigene Datensätze unter
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/`
abgelegt, sämtlich mit dem Zeitstempel `260812-1805`.

---

## Befunde nach Themen

### Thema 1: Die Zerlegung von Markdown ist nicht total

Drei Befunde, alle an derselben Naht, und alle gegen dasselbe
Abnahmekriterium: C4.3, „Alles außerhalb dieses Umfangs erscheint als der
Quelltext, der dasteht."

**Gemessen und nicht gelesen.** `crates/krk-ui/src/markdown.rs` ist unverändert
in ein Prüfprogramm neben `pulldown-cmark 0.13.4` kopiert und mit
Grenzfällen gefahren worden; die Ausgaben unten sind die des echten
`markdown::rendern`.

**1. Quelltext ohne Ereignis verschwindet spurlos.** *(mittel,
`260812-1805_o_quelltext-ohne-ereignis-verschwindet-spurlos-…`)*

```
Quelle : "[ref]: https://example.com\n[zwei]: https://b.example\n"
Ausgabe: ""
```

Eine Verweisdefinition erzeugt beim Zerleger **kein einziges Ereignis**; die
Auffangregel in `markdown.rs:118-123` hängt an einem Ereignis und greift
deshalb nicht. Eine Datei aus lauter Definitionen zeigt eine leere Fläche, und
in jeder README mit Verweisen in Kurzform fällt der Definitionsblock weg. Der
zweite Fall derselben Art: `[](https://example.com)` verschwindet vollständig,
weil `Zerlegung::schliessen` (`markdown.rs:632-660`) bei Länge null nichts
einträgt und der Verweis auch nichts geschrieben hat.

Die Totalitätszusage in `markdown.rs:41-47` und in der `Decidability`-Zeile des
Plans gilt über `Event` und `Tag`. Sie gilt **nicht** über die Zeichen der
Datei, und diese Lücke ist nirgends benannt. Das wiegt in diesem Projekt
besonders, weil die Wahl von `pulldown-cmark` gegen `syntect` ausdrücklich mit
dem Satz begründet ist, ein fälschlich ausgeblendetes Zeichen sei „eine falsche
Auskunft über den Inhalt einer Datei" (Wurzel-`Cargo.toml`).

**2. Listen verlieren Merkzeichen, Nummerierung und Verschachtelungstiefe.**
*(mittel, `260812-1805_o_listen-verlieren-merkzeichen-…`)*

```
Quelle : "1. eins\n2. zwei\n3. drei\n"
Ausgabe: "eins\nzwei\ndrei"

Quelle : "- eins\n- zwei\n  - drunter\n    - noch tiefer\n"
Ausgabe: "eins\nzwei\ndrunter\nnoch tiefer"
```

Vier Verschachtelungsstufen bekommen denselben Einzug, weil `einzugsmerkmal`
(`textmerkmale.rs:412-424`) einen festen Absatzstil von 20 Punkten setzt und
`addAttributes:range:` ihn ersetzt statt zu addieren. Der
Entscheidungsdatensatz zum Umfang nennt verschachtelte Listen ausdrücklich
unter dem, was Möglichkeit 1 **nicht** enthält, und sagt für alles Weitere „es
erscheint als der Text, der dasteht". Die Umsetzung tut ein Drittes: sie
rendert sie und macht sie dabei flach. Aufgeschrieben ist diese Wahl nirgends;
ihre einzige Spur ist der Name einer Probe.

**3. YAML-Front-Matter erscheint als Trennlinie plus Überschrift.** *(niedrig,
`260812-1805_o_yaml-front-matter-…`)*

```
Quelle : "---\ntitle: Sache\n---\n\nText\n"
Ausgabe: "---\n\ntitle: Sache\n\nText"   mit Ueberschrift{2} über "title: Sache"
```

CommonMark-treu und trotzdem falsch für diese Vorschau: der Block liegt
außerhalb des Umfangs und erscheint weder gerendert noch als Quelltext.
`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` lieferte `Tag::MetadataBlock`, und
die vorhandene Auffangregel gäbe ihn wörtlich aus — eine Entscheidung, keine
Zeile, weil `Options::empty()` im Plan gewählt ist.

### Thema 2: Eine Zusicherung im Code, die seit dieser Runde nicht mehr gilt

**Der Überschneidungssatz in `textmerkmale::anwenden`.** *(mittel,
`260812-1805_o_der-ueberschneidungssatz-…`)*

Der SAFETY-Kommentar (`textmerkmale.rs:210-227`) sagt zu: „`Ueberschrift` und
`FesteSchrift` setzen beide die Schrift und überlappen einander deshalb nie."
`markdown.rs` ist seit Schritt 8 ein zweiter Erzeuger von `Formatierung` und
liefert genau diese Überschneidung — die eigene Probe
`die_auszeichnungen_stehen_von_aussen_nach_innen` (`markdown.rs:722`) schreibt
sie fest. Vier der fünf `Auszeichnung`-Werte setzen jetzt `NSFontAttributeName`,
und bei gleichem Namen ersetzt `addAttributes:range:`, statt zusammenzulegen.

Die Ersatzregel — außen vor innen sortieren, damit innen gewinnt — bricht,
sobald außen und innen dieselbe Länge haben. Gemessen:

```
"# `Code` im Titel"   Ueberschrift{1}(0,13), FesteSchrift(0,4)   innen gewinnt, wie gewollt
"**`code`**"          FesteSchrift(0,4), StarkeBetonung(0,4)     die feste Schrift geht verloren
"*kursiv **fett** …*" Betonung(0,25), StarkeBetonung(7,4)        die Kursive geht verloren
```

Das Gewicht liegt weniger in der Anzeige als in der Zusicherung. `CLAUDE.md`
führt unter „Was man nicht sieht" den Fall, in dem genau eine solche behauptete
Unmöglichkeit einen Fehlbefund und eine Sitzung gekostet hat.

### Thema 3: Die eine Statuszeile und die Sichtbarkeit

**Die Zeile zeigt Meldungen eines ausgeblendeten Dateifensters.** *(mittel,
`260812-1805_o_die-eine-statuszeile-zeigt-meldungen-…`)*

`statuszeile_nachziehen` (`anwendung.rs:3073-3097`) holt die Quellen **beider**
Dateifenster, ohne nach Sichtbarkeit zu fragen. `Fenstermodell::umschalten`
lässt die vier Meldungsfelder beim Ausblenden unangetastet — richtig, denn C5.7
verlangt es. Zusammen ergibt das: eine Tabmeldung des ausgeblendeten rechten
Dateifensters gewinnt gegen einen Markierungsstand des sichtbaren linken, und
in der Zeile steht dauerhaft und rot „rechtes Dateifenster: Ordner nicht
lesbar" über einen Bereich, den der Nutzer nicht sieht.

C5.8 sagt: „Steht nur ein Dateifenster, ist es das aktive, und kein Satz trägt
einen Zusatz." In dieser Lage ist der Satz falsch. Bis zur Runde 6 konnte der
Fall nicht eintreten: die Zeile saß am Fuß ihres Dateifensters und wurde mit
ihm ausgeblendet. Die Probe dazu
(`statuszeile.rs:906-923`) übergibt für das ausgeblendete Dateifenster
`Quellen::default()` und setzt damit die Voraussetzung, die das Programm nicht
herstellt.

**Die Ordnung selbst hält.** Die zweistellige Ordnung — außen `Rang::ALLE`,
innen `[aktiv, aktiv.andere()]` (`statuszeile.rs:341-364`) — ist über die zehn
Paare vollständig und überschneidungsfrei, wie der Plan behauptet: jeder der
zehn Plätze wird genau einmal in fester Folge besucht, ein Gleichstand kann
nicht entstehen. Die elf neuen Proben treffen die vier Stellen, an denen die
Ordnung sich entscheidet.

### Thema 4: Abnahme und Prüfbarkeit

**Drei der fünf Zählproben aus der Prüfstrategie sind nicht gebaut.**
*(niedrig, `260812-1805_o_drei-der-fuenf-zaehlproben-…`)*

Gebaut: „genau ein Aufrufer von `NSSharingServicePicker`" und „genau ein
Menübauer", beide in `teilen.rs`. Nicht gebaut: „keine Web-Ansicht" (C4.5),
„genau eine `NSPasteboard`-Hülle" (C1.8), „genau drei Prüfordner-Fassungen"
(C6.6). Alle drei Eigenschaften halten heute, gemessen; es fehlt ihre Abnahme.
C4.5 schreibt die Prüfform selbst vor („Die Prüfung zählt den Klassennamen im
Baum") und gilt damit nicht als abgenommen. Das Gerüst dafür steht fertig in
`teilen.rs:375-412`.

**`textmerkmale.rs` trägt keine einzige Probe.** *(niedrig,
`260812-1805_o_textmerkmale-rs-traegt-keine-einzige-probe`)*

436 Zeilen, zwei Verbraucher seit dieser Runde, kein `#[cfg(test)]`. Der
Sitzungsbericht zu Schritt 7 benennt die Lücke selbst und führt sie auf den
Hauptfaden zurück, den `libtest` nicht hergibt. Das trifft auf `anwenden`,
`zuruecksetzen` und `tafel_der_erscheinung` zu, **nicht** auf `grundschrift`
(eine reine Fallunterscheidung über sechs Eingabepaare, mit seit dieser Runde
zwei Aufrufern) und nicht auf `UEBERSCHRIFTSFAKTOREN`.

### Thema 5: Zahlen in Datensätzen und Dateinamen

**Der Datensatz zur Verweisfarbe nennt für die dunkle Tafel eine Farbe, die sie
nicht liefert.** *(niedrig,
`260812-1805_o_der-datensatz-zur-verweisfarbe-…`)*

`260812-1701_o` schreibt „(208, 135, 112) in Hell und (235, 203, 139) in
Dunkel". Nachgemessen mit denselben Kisten und Merkmalen: beide Tafeln liefern
(208, 135, 112). Der beschriebene Defekt hat damit heute **keine sichtbare
Wirkung**, und die Abwägung, die der offene Datensatz dem Nutzer vorlegt, steht
auf einer falschen Zahl.

**Sechs Sitzungsprotokolle tragen einen Zeitstempel aus der Zukunft.**
*(niedrig, `260812-1805_o_sechs-sitzungsprotokolle-…`)*

Dateiname gegen Änderungszeit: die Abweichung wächst über die Runde von 24
Minuten auf 2 Stunden 12 Minuten, drei Dateien liegen jetzt noch in der
Zukunft. Die Reihenfolge im Speicher `history/` ist damit erfunden, und diese
Durchsicht sortiert vor Arbeit, die vor ihr stattgefunden hat.

---

## Was nachgeprüft wurde und hält

Fünf Fragen aus dem Auftrag, jede am Baum oder am SDK gegengelesen.

**Die Messung hinter der `linkfarbe`-Abweichung stimmt genau.** Der Plan
verlangt einen Nachschlag auf `markup.underline.link`; der `coder` hat
gemessen, dass der in beiden Tafeln die Grundfarbe liefert, und statt dessen
den vollen Wortartenstapel genommen. Nachgemessen mit `syntect 5.3.0` und
`two-face 0.5.2` in denselben Merkmalen:

```
base16-ocean.light   Grundfarbe 79/91/102     markup.underline.link 79/91/102
base16-ocean.dark    Grundfarbe 192/197/206   markup.underline.link 192/197/206
beide                VERWEISSTAPEL 208/135/112
```

Der `VERWEISSTAPEL` ist zudem **genau** der Stapel, den `rechnen` im Editor für
den Text eines Verweises sieht — nachgestellt an
`[die Seite](https://example.com)`. Die Ersatzlösung trifft, was der Plansatz
erreichen wollte, und die Probe `die_tafel_faerbt_einen_verweis` verriegelt sie
gegen eine umbenannte Wortart. Eine saubere, gemessene Abweichung.

**Der Einfärbungsvorgang trägt; kein Faden verwaist, keiner läuft doppelt.**
Das dritte Ivar `einfaerbung_erneut` ist begründet: der wörtliche Plansatz
(„lässt einen laufenden Vorgang fallen") ergäbe je Pfeiltastendruck einen
weiterrechnenden Faden. Gelesen wurde der ganze Weg
`anzeigen → einfaerbung_nachfuehren → einfaerbung_einziehen`
(`vorschau.rs:842-935`):

- **Höchstens einer:** gestartet wird nur bei `einfaerbung.is_none()`, und alle
  Aufrufer liegen auf dem Hauptfaden.
- **Keiner verwaist:** `takt_beenden` greift nur, wenn weder ein Tab lädt noch
  eine Einfärbung läuft; der Neustart aus dem `ueberholt`-Zweig setzt das Ivar,
  bevor `einziehen` die Bedingung prüft.
- **Kein verschluckter Auftrag,** außer bei `Abholung::Weggefallen`: dort wird
  ein vorgemerkter Neustart mit verworfen. Der Faden ist dann gefallen, und die
  Fläche steht ohne Farben statt mit falschen; erwähnenswert, kein Defekt.

**L7 ist freigehalten.** `Vorschaumodell::laedt_noch` weiß von der Einfärbung
nichts, und die Probe `das_vorschaumodell_weiss_von_der_einfaerbung_nichts`
misst es über die Modulgrenze samt Gegenriegel. Der Weg für Markdown liegt
vollständig vor der Endbedingung, der für `syntect` vollständig dahinter.

**`tafel_der_erscheinung` steht genau einmal im Baum.** `grep` findet
`bestMatchFromAppearancesWithNames` allein in `textmerkmale.rs`, und der Ort
ist im Modulkopf über eine Ausschlussrechnung begründet: nicht in
`hervorhebung.rs` (keine Zeile AppKit, von S16 gemessen), nicht privat in
`editor.rs` (zwei Verbraucher). Sauber.

**Die Ersthelfer-Regel ist eingehalten.** `fenster.rs` hat an
`makeFirstResponder:` nichts angebaut; der Unterschied umfasst dort allein den
Modulkopf, `MINDESTGROESSE` und `fensterinhalt`.
`fokusanzeige_nachziehen` (`anwendung.rs:3128-3141`) schreibt weiterhin
ausschließlich die fünf Rahmenfarben und den Fenstertitel und ruft weder
`anwenden` noch `setHidden`. Ein zweiter Beobachter ist nicht entstanden.

**Zu C5.11 wird nichts behauptet, was nicht gemessen ist**, und das ist
richtig so: die Frage bleibt am Bündel. Am Baum entscheidbar ist, dass
`setRefusesFirstResponder` genau einmal vorkommt (`bereichsleiste.rs:478`, für
die Schalter) und die Zeile es nicht trägt, und dass `NSScrollView`
`acceptsFirstResponder` von `NSView` erbt. Die `NSScroller` sind `NSControl`
und damit von derselben Art wie die Schalter, deren Frage die Runde 5 offen
gelassen hat — der Modulkopf sagt das aus und nicht mehr.

**Die Einbindung von `pulldown-cmark` entspricht dem Plan.** `0.13.4`, ohne
Vorgabemerkmale; `Cargo.lock` wächst um `pulldown-cmark` und `unicase 2.9.0`,
`bitflags 2.13.1` und `memchr 2.8.3` standen schon da. Kein `cc`, außer
`windows-sys` kein `-sys`-Paket. Das Tabellenmerkmal bleibt aus, und die Probe
`eine_tabelle_bleibt_ein_quelltextraster` hält das Raster fest.

**Die UTF-16-Stellen sind richtig gezählt.** Jede Stelle entsteht aus dem
mitlaufenden Zähler, der ausschließlich um `encode_utf16().count()`
fortschreitet; damit liegt jede Stelle innerhalb von `laenge` und `laenge` ist
die UTF-16-Länge des Ausgabetextes. Nachgestellt an
`"Grüße 😀 [Ziel](https://x) danach"`: Einfärbung bei 9, Länge 4, Gesamtlänge
20 — in Bytes und in Zeichen wären es andere Zahlen. Die Probe
`die_stellen_sind_utf16_einheiten` misst genau diesen Unterschied.

**Jede geprüfte macOS-Untergrenze stimmt.** Alle Angaben der neuen und
geänderten Modulköpfe — `textmerkmale.rs`, `vorschau.rs`, `statuszeile.rs`,
`fenster.rs`, `editor.rs` — sind gegen SDK 26.2 gegengelesen: keine falsche
Zahl, keine Zeilennummer um mehr als eine Zeile daneben, kein Symbol, das im
SDK nicht steht. Die Deckung des Abschnitts unter `crates/krk-ui/src/appkit/`
steht bei **34 von 36** Dateien, rekursiv gezählt; ohne ihn sind weiterhin
`koordinaten.rs` und `mod.rs`, beide begründet. Die Zahl im offenen Datensatz
`shared/issues/260812-1438_o` („33 von 35") ist damit ihrerseits überholt; das
ist dieselbe Veraltung und braucht keinen zweiten Datensatz.

**Der Umzug in Schritt 7 ändert kein Verhalten.** Die alte Fassung sprang bei
fehlendem Speicher, fehlendem Verwalter und Längenabweichung über `return`
heraus und ließ das Nachziehen der Nummernspalte damit aus; die neue erhält
denselben Ablauf über den `#[must_use]`-Rückgabewert. Der Ausdruck ist Zeile
für Zeile gelesen.

**Die vollständigen Fallunterscheidungen sind vollständig.** `Inhalt` wächst auf
sechs Werte, `Auszeichnung` auf fünf, beide ohne Auffangzweig; `Rang` kommt als
neue hinzu. `Wirkungsbereich`, `Kommando`, `Bereich` und `Fokus` wachsen nicht.
`einzufaerben` und `zeigt_dateitext` decken alle sechs `Inhalt`-Werte
namentlich ab.

---

## Übergreifende Beobachtungen

**Ein Erzeuger mehr an einer Schnittstelle, und zwei Zusicherungen halten
nicht mehr.** `Formatierung` hatte bis zu dieser Runde genau einen Erzeuger,
`hervorhebung::formatieren`. Seit Schritt 8 gibt es einen zweiten, und beide
Befunde in Thema 2 und in Thema 1 gehen darauf zurück: der
Überschneidungssatz in `textmerkmale.rs` beschreibt eine Eigenschaft des
**ersten** Erzeugers und wurde beim Hinzufügen des zweiten nicht nachgelesen.
Derselbe Handgriff steht noch aus für die Frage, welche Auszeichnungen
gemeinsam wirken sollen, wenn sie einander enthalten.

**Die Wahl, die Zerlegung von einem Ereignisstrom abhängig zu machen, hat einen
blinden Fleck, den keine der Proben treffen konnte.** Die sechzehn Proben in
`markdown.rs` prüfen jeweils, dass etwas Bestimmtes **erscheint**. Keine prüft,
dass **nichts** verschwindet. Eine einzige Probe der Form „jedes Nicht-Leerzeichen
der Quelle kommt im Ausgabetext vor, sofern es kein Auszeichnungszeichen des
gewählten Umfangs ist" hätte alle drei Befunde aus Thema 1 gefangen. Das ist
die lohnendste einzelne Probe, die diese Runde nachtragen kann.

**Die Sichtbarkeit ist bei der Zusammenlegung der Statuszeilen durchgefallen.**
Plan, Kriterien und Sitzungsbericht behandeln das Ausblenden eines
Dateifensters an keiner Stelle, und die Probe zu C5.8 setzt die Bedingung, die
das Programm nicht herstellt. Bemerkenswert daran ist, dass die Verkopplung
vorher **räumlich** gelöst war: die Zeile war eine Unteransicht, also ging sie
mit. Der Umzug hat eine Kopplung aufgelöst, die niemand als Regel
aufgeschrieben hatte, weil sie aus dem Aufbau folgte.

**Die Selbstberichte des `coder` sind belastbar.** Vier Abweichungen vom
Wortlaut des Plans sind in den Sitzungsprotokollen benannt und begründet, jede
mit ihrer Messung; zwei davon habe ich unabhängig nachgemessen und beide
bestätigt. Drei Befunde hat der `coder` selbst als Datensatz abgelegt, statt
sie stehen zu lassen. Kein Befund dieser Durchsicht widerspricht einem
Berichtssatz — die drei Stellen in Thema 1 sind nicht falsch berichtet, sondern
gar nicht bemerkt worden.

---

## Empfohlene Reihenfolge

**Vor dem Abnahmelauf am Bündel**, weil sie sonst dort als Fehlbefund
erscheinen und Nutzerzeit kosten:

1. `260812-1805_o_quelltext-ohne-ereignis-verschwindet-spurlos-…` — die
   Auffangregel auf die Quellbereiche stellen. Zusammen mit der Deckungsprobe
   aus den übergreifenden Beobachtungen.
2. `260812-1805_o_die-eine-statuszeile-zeigt-meldungen-…` — zwei Zeilen in
   `statuszeile_nachziehen`, oder die Sichtbarkeit in `statuszeile::zeile`
   hineinreichen und die Probe darauf umstellen.
3. `260812-1805_o_der-datensatz-zur-verweisfarbe-…` — die Zahl berichtigen,
   bevor der Nutzer über den offenen Datensatz `260812-1701_o` entscheidet.

**Danach, in einem Zug mit den Modulkopf-Befunden aus Turn 1**
(`260812-1702_o`, `260812-1731_o`):

4. `260812-1805_o_der-ueberschneidungssatz-…` — mindestens den Kommentar
   berichtigen; ob die Schriften zusammengelegt werden, ist eine eigene Frage.
5. `260812-1805_o_listen-verlieren-merkzeichen-…` — entweder Merkzeichen und
   Tiefe nachtragen oder die Wahl aufschreiben.
6. `260812-1805_o_sechs-sitzungsprotokolle-…` — die Dateien umbenennen,
   solange die Verweise darauf überschaubar sind.

**Aufräumen, ohne Eile:**

7. `260812-1805_o_drei-der-fuenf-zaehlproben-…`
8. `260812-1805_o_textmerkmale-rs-traegt-keine-einzige-probe`
9. `260812-1805_o_yaml-front-matter-…` — eine Entscheidung, kein Handgriff.

---

## Was diese Durchsicht nicht abnehmen konnte

Kein Vordergrundlauf, kein Bündelbau, keine Messung der zehn Zusagen. Damit
bleiben die Kriterien am Bündel offen, die der Plan als solche ausweist —
C4.1 (zweite Hälfte), C4.7, C4.11 (zweite Hälfte), C4.14, C5.1 bis C5.4, C5.10
und C5.11. C5.11 ist ausdrücklich **nicht** beantwortet worden; es ist eine
Nutzerfrage am laufenden Bündel, und der Baum gibt darüber nur her, was oben
unter „Was nachgeprüft wurde" steht.

---

## Ein Nachtrag außerhalb des Bereichs

Während dieser Durchsicht ist `4413d7a` gelandet, ein Commit hinter dem Ende
des geprüften Bereichs und deshalb nicht Gegenstand: er überholt den Datensatz
`260812-1105_*` und legt
`decisions/260812-1809_a_wie-wird-eine-meldung-lesbar-die-breiter-ist-als-das-fenster.md`
an. Der Nutzer hat darin die **Bildlaufansicht aus Schritt 11 gegen einen
Kurzhinweis über `setToolTip:` getauscht**.

Für diese Durchsicht heißt das dreierlei. Was hier über die Bildlaufansicht
steht, beschreibt den Stand bei `05797d7` und ist mit der Umsetzung jenes
Datensatzes überholt. Der Befund zur ausgeblendeten Seite (Thema 3) ist davon
**nicht** berührt: er hängt an `statuszeile_nachziehen` und an
`statuszeile::zeile`, die der Tausch nicht anfasst. Und die Reihenfolge oben
bleibt, wie sie ist — der Tausch macht keinen der neun Befunde dringlicher oder
entbehrlicher.
