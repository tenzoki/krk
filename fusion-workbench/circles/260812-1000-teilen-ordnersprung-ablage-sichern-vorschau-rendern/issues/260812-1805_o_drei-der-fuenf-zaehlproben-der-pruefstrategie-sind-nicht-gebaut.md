Drei der fünf Zählproben aus der Prüfstrategie der Runde 6 sind nicht gebaut

---

Der Plan zählt unter `## Prüfstrategie` fünf Zählproben auf den Baum auf:
„genau ein Aufrufer von `NSSharingServicePicker`, genau ein Menübauer, keine
Web-Ansicht, genau eine `NSPasteboard`-Hülle, genau drei
Prüfordner-Fassungen". Gebaut sind die ersten beiden, in
`crates/krk-ui/src/appkit/teilen.rs`. Die drei übrigen gibt es nicht, obwohl
die zugehörigen Kriterien C4.5, C1.8 und C6.6 jeweils **(Probe)** tragen.

---

**Nachgezählt am Baum:**

| Zusage | Kriterium | Probe |
|---|---|---|
| genau ein Aufrufer von `NSSharingServicePicker` | C1.7 | `teilen.rs:429` `allein_diese_datei_baut_den_freigabewaehler` |
| genau ein Menübauer | C1.7 | in derselben Datei |
| **keine Web-Ansicht** | **C4.5** | **fehlt** |
| **genau eine `NSPasteboard`-Hülle** | **C1.8** | **fehlt** |
| **genau drei Prüfordner-Fassungen** | **C6.6** | **fehlt** |

Die drei Eigenschaften **halten heute**: `grep -rn "WKWebView\|WebKit" crates/`
findet nichts, `NSPasteboard` steht allein in
`crates/krk-ui/src/appkit/zwischenablage.rs`, und die drei Prüfordner-Fassungen
stehen dort, wo `CLAUDE.md` sie nennt. Es fehlt nicht die Eigenschaft, sondern
ihre Abnahme.

**Warum das mehr ist als eine fehlende Probe.** C4.5 ist der einzige der drei,
dessen Kriterium die Prüfform selbst vorschreibt: „Keine Web-Ansicht. Weder
`WKWebView` noch `NSAttributedString`-Auszeichnung aus Markdown noch ein
anderer Weg über eine Darstellungsschicht des Systems. **Die Prüfung zählt den
Klassennamen im Baum.** (Probe)". Als abgenommen gilt das Kriterium damit
heute nicht — es ist wahr, aber nicht gemessen. Und es ist genau die Sorte
Zusage, die still bricht: der Web-Betrachter-Circle steht als eigener Kandidat
im Portfolio, und wer ihn fährt, fügt eine `WKWebView` hinzu, ohne dass an der
Vorschau etwas anhält.

**Der Bauplan liegt fertig daneben.** `teilen.rs:375-412` trägt
`quelldateien()` und `einsammeln()`, die den Quellbaum unter
`CARGO_MANIFEST_DIR/src` einlesen und dabei ausdrücklich fehlschlagen, wenn der
Baum leer ist. Die drei fehlenden Proben sind je ein Filter darüber. Der
naheliegende Ort für die Web-Ansicht ist `appkit/vorschau.rs`, weil dort die
Zusage wohnt; die beiden anderen gehören zu `zwischenablage.rs` und zu einer
Stelle, die alle drei Kisten sieht.

Beim Bauen ist zu beachten, was `teilen.rs` an seiner eigenen Probe schon
gelöst hat: die Nadel muss zwischen Nennen und Benutzen trennen, sonst schlägt
die Probe an ihrem eigenen Prosakommentar an.

**Gewicht:** niedrig für den Zustand des Baums, mittel für die Abnahme —
C4.5 ist als **(Probe)** ausgewiesen und hat keine.

**Herkunft:** Circle der Runde 6, Prüfstrategie des Plans. C4.5 stammt aus
Turn 2; C1.8 und C6.6 aus Turn 1 und sind dort nicht aufgefallen. Ein Datensatz
für alle drei, weil es eine Lücke mit einem Handgriff ist.
