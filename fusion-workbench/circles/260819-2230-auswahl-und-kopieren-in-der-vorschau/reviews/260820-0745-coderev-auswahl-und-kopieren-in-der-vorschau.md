# Durchsicht der Runde 14: Auswahl und Kopieren in der Vorschau

**Reviewed-range:** `fce0b6f..b28cdd6`
**Not-opened:** `CLAUDE.md` (vom Auftrag ausdrücklich ausgenommen), `fusion-workbench/orchestrator-events.jsonl` (Sitzungsprotokoll, kein Gegenstand einer Codedurchsicht), `fusion-workbench/circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/_t_circle.md` (Rundenabschluss), die acht Sitzungsprotokolle unter `circles/260819-2230-…/history/` (Ausführungsprotokolle; der Stand ist am Baum erhoben und nicht am Protokoll), `fusion-workbench/circles/260812-1000-…/decisions/260812-1000_s_was-tut-ein-link-…` (nur der Marker ist gewandert, Markerwanderung ist vom Auftrag ausgenommen), `fusion-workbench/shared/decisions/260819-1500_i_gilt-die-artefaktsprache-…`, `fusion-workbench/shared/history/260819-2007-orchestrator-session.md`, `fusion-workbench/shared/history/260819-2040-artefaktsprache-deklaration-zurueckgenommen.md`, `fusion-workbench/shared/history/260819-2216-shaper-auswahl-und-kopieren-in-der-vorschau.md`, `fusion-workbench/shared/issues/260817-1610_c_the-language-paragraph-…`, `fusion-workbench/shared/issues/260819-2206_o_die-commit-nachricht-…`, `fusion-workbench/shared/issues/260820-0602_o_make-check-prueft-den-ganzen-arbeitsbereich-…` (alle sechs betreffen die Artefaktsprache, den Commit-Namensraum und den Prüflauf, nicht diese Runde).

**Sender:** coderev
**Datum:** 260820-0745
**Baumstand:** `b28cdd6`
**Gegenstand:** die zehn Codedateien der Runde 14 unter `crates/krk-ui/src/`, der Spec, der Plan und die sieben bindenden Entscheidungsdatensätze.

---

## Zusammenfassung

Die Runde ist sauber gebaut: die Kachelung ist total, die Anmeldung im Ereignisabgriff
fragt nach der Nämlichkeit, der Ereignisabgriff kennt weder Editor noch Vorschau, es
bleibt bei einer Abfangstelle und einer Hülle um `NSPasteboard`, und alle vier
Prüfkommandos laufen grün. **Die Klammerregel trifft die vom Nutzer gewählte Möglichkeit
aber nicht in jedem Fall**, und zwar in beide Richtungen: ein Absatz mit einer Entität oder
einem Backslash-Escape bläht jede Auswahl darin auf sich selbst auf (die verworfene
Möglichkeit 3), und eine Überschrift, die mit einer Betonung oder einem Verweis beginnt,
verliert ihr `#`. Beide sind an einer Kopie des Baums gemessen und nicht erschlossen.

**Sechs Befunde**, alle als Datensätze unter `issues/` dieses Circles abgelegt.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch (Auslieferungssperre) | 0 |
| Hoch | 2 |
| Mittel | 2 |
| Niedrig | 2 |

## Wie geprüft wurde

Der Baum auf dem Stand `b28cdd6` liegt als Kopie unter `/private/tmp/…/scratchpad/krkcopy`;
dort sind drei Proben hinzugefügt und gefahren worden, die den Quellbezug an Konstrukten
befragen, die der Beispielsatz der Runde nicht führt. **Am Projektbaum ist nichts geändert
worden.** Der volle Prüflauf in der Kopie — `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` — läuft
grün; `krk-ui` trägt 734 Proben.

---

## Befunde nach Thema

### Die Klammerregel: zweimal die falsche Antwort, eine Wurzel

**H1 — Ein Absatz mit Entität oder Escape trägt eine Klammer.**
`issues/260820-0728_o_ein-absatz-mit-entitaet-oder-escape-traegt-eine-klammer-und-blaeht-jede-auswahl-darin-auf.md`
`crates/krk-ui/src/markdown.rs:1044-1075`. Gemessen:

```
"Ein &amp; hier im Absatz mit vielen Woertern.\n"  Auswahl "vielen"
  -> "Ein &amp; hier im Absatz mit vielen Woertern.\n"
"Ein \* Stern im Absatz mit vielen Woertern.\n"    Auswahl "vielen"
  -> "Ein \* Stern im Absatz mit vielen Woertern.\n"
```

Das ist die Möglichkeit 3 des bindenden Datensatzes, die der Nutzer nicht gewählt hat.
`pulldown-cmark` meldet für `&amp;` ein `Event::Text("&")` mit dem Quellbereich der
ungelösten Entität; `Zerlegung::schreiben` legt daraufhin einen `Ersetzt`-Abschnitt an, und
`klammer_verbuchen` schreibt ihn dem innersten offenen Element zu, also dem Absatz. Der
Leerraum-Halbsatz, der den Zeilenumbruch am Absatzende abfängt, greift hier nicht.

**H2 — Eine Überschrift, die mit einem Kind beginnt, verliert ihre eigene Klammer.**
`issues/260820-0731_o_eine-ueberschrift-die-mit-einem-kind-beginnt-verliert-ihre-eigene-klammer.md`
Dieselbe Funktion, dieselben Zeilen. Gemessen:

```
"# **Titel** und noch ein Stueck Text\n"     Auswahl "noch ein" -> "noch ein"
"## `code` und noch ein Stueck Text\n"       Auswahl "noch ein" -> "noch ein"
"# [V](https://e.com) und noch ein Stueck\n" Auswahl "noch ein" -> "noch ein"
"# Titel\n"                                  Auswahl "itel"     -> "# Titel\n"
```

C2.2 sagt zu, dass eine kopierte Überschrift ihr Doppelkreuz mitbringt. Für die ersten drei
Fälle gilt das nicht. Der Grund ist die Beschneidung auf den Quellbereich des innersten
Elements: das `# ` liegt im ersten Abschnitt des Kindes, wird dort verbucht und dem Vater
nie zugeschrieben.

**Die gemeinsame Wurzel.** Der Begriff „Klammer" meint im Plan und im Modulkopf, dass ein
Element an seinen **Rändern** Zeichen trägt, die ein Schnitt unbalanciert zurückließe.
Umgesetzt ist „irgendwo verdeckte Bytes, die nicht Leerraum sind", verbucht beim innersten
Element. Beide Befunde verschwinden, wenn die Klammer an Vorspann und Nachspann des
Elements selbst hängt — an die Bytes zwischen seinem Anfang und dem ersten darin
geschriebenen Zeichen und zwischen dem letzten und seinem Ende. Diese Auskunft hat der
Durchgang: `Offen::quelle` steht beim Öffnen fest, der Lesestand beim Schließen.

### Die eine Abfangstelle

**M1 — Die Abfangstelle verwirft die geforderten Sorten und leert jede gereichte Ablage.**
`issues/260820-0733_o_die-abfangstelle-verwirft-die-geforderten-sorten-und-leert-jede-gereichte-ablage.md`
`crates/krk-ui/src/appkit/vorschau.rs:445-462`, `zwischenablage.rs:258-262`. Der Parameter
`sorten` wird im Markdown-Zweig nicht gelesen, und `clearContents()` steht ohne
Fallunterscheidung. Für die Zwischenablage des Nutzers ist beides richtig; für die Ablage
eines Ziehvorgangs oder eines Dienstes ist es nicht geprüft. Der bindende Datensatz zum
Ziehen hält für den Fehlschlag seine Möglichkeit 2 bereit.

**Was an dieser Stelle richtig ist und geprüft wurde:** die zwei Zweige trennen sauber am
Merkposten; der Zweig ohne Quellbezug reicht mit `msg_send![super(self), …]` unverändert
weiter; die Koordinaten stimmen, denn `selectedRange` liefert UTF-16-Einheiten und der
Quellbezug führt seine Textbereiche in denselben; eine verdrehte oder eine über das
Textende hinausreichende Auswahl bricht nicht ab, weil `ausschnitt` sie auf einen leeren
Bereich zurückführt und `beitrag` in die Abschnittsgrenzen klemmt. Die Zählprobe
`die_abfangstelle_steht_im_baum_genau_einmal` hält die Zusage aus C2.12, soweit eine Suche
im Quelltext sie halten kann, und schreibt ihre blinden Flecken aus.

### Eine Nebenwirkung der neuen Fokusansicht

**M2 — Der Anker des Freigabedialogs ist in der Vorschau jetzt das ganze Dokumentrechteck.**
`issues/260820-0735_o_der-anker-des-freigabedialogs-ist-in-der-vorschau-jetzt-das-ganze-dokumentrechteck.md`
`vorschau.rs:832-838` liefert ab jetzt die Textanzeige, `anwendung.rs:3538-3556` nimmt
`flaeche.bounds()` als Ankerrechteck, und `textanzeige` (`vorschau.rs:1435-1444`) setzt die
Fläche auf `setVerticallyResizable(true)` mit `maxSize` `f64::MAX`. Bei einer langen Datei
liegt die Mitte des Ankers weit unterhalb des Fensters. Der Doc-Kommentar hat den Ankerfall
gesehen und nach der Ausblendung gefragt, nicht nach der Größe.

### Zusagen ohne Probe, und eine Ungleichheit

**N1 — C2.3 und C2.4 tragen die Kennzeichnung (Probe) und haben keine.**
`issues/260820-0737_o_zwei-abnahmekriterien-mit-probenkennzeichnung-haben-keine-probe.md`
Die Sache selbst stimmt am Baum — `Parser::new_ext(…).into_offset_iter()` steht in
`markdown.rs:593` genau einmal —, aber an diesen beiden Kriterien hängt der Verzicht auf
einen Abnahmelauf gegen L7, und kein Kommando misst sie nach.

**N2 — `text_schreiben` hat sein `#[must_use]` bei der Aufteilung nicht mitbekommen.**
`issues/260820-0739_o_text-schreiben-hat-sein-must-use-bei-der-aufteilung-nicht-mitbekommen.md`
`zwischenablage.rs:258-272`. Beide Rufer werten den Wert heute aus; der Befund ist die
Ungleichheit zwischen den zwei Hälften derselben Antwort.

---

## Was geprüft wurde und trägt

### Die Totalität der Kachelung ist strukturell und nicht von den Beispielen abhängig

Die Frage des Auftrags war, ob acht Beispiele genügen und ob ein Markdown-Konstrukt die
Kachelung durchlöchern kann. **Kann es nicht**, und zwar aus der Bauart heraus:

- `self.gelesen` wird im ganzen Modul an genau einer Stelle geschrieben, in
  `Zerlegung::kacheln` (`markdown.rs:1024`).
- `self.text.push_str` und `self.stelle +=` stehen an genau zwei Stellen, in
  `Zerlegung::erzeugen` (`:1128`) und `Zerlegung::schreiben` (`:1159`), und beide rufen
  unmittelbar danach `kacheln`.

Damit kann kein Ereignisfall Quelle abtragen oder Text schreiben, ohne einen Abschnitt
anzulegen. Zur Gegenprobe sind zwölf Konstrukte gefahren worden, die der Beispielsatz nicht
führt — Entität, Backslash-Escape, beide Formen des harten Umbruchs, Bild im Absatz,
HTML-Block, Inline-HTML, Setext-Überschrift, Datei ohne Schlussumbruch, Aufgabenpunkt,
Liste im Zitat, Trennlinie. Alle zwölf decken beide Seiten lückenlos, und bei allen zwölf
liefert die Auswahl über alles die Quelle byteweise vollständig.

**Der Wert der zehn Beispiele liegt woanders**, und dort ist er hoch: sie sind der
Rückhalt gegen eine künftige Änderung, die eine Schreibstelle an `kacheln` vorbeiführt. Was
sie **nicht** decken, ist die zweite Auskunft desselben Durchgangs, die Klammer — und genau
dort sitzen H1 und H2.

### Die Anmeldung im Ereignisabgriff

`Anwendungsdelegierter::ist_eigene_textflaeche` (`anwendung.rs:2385-2402`) fragt zwei
`isEqual`-Vergleiche gegen `editor.textflaeche()` und `vorschau.textflaeche()`, also nach
der Nämlichkeit und nicht nach der Klasse. `ersthelfer_gehoert_appkit`
(`ereignisse.rs:702-716`) stellt die Nämlichkeitsfrage vor der Klassenprüfung. **In
`ereignisse.rs` steht der Name „Editor" oder „Vorschau" nur an einer einzigen Stelle, und
das ist ein Doc-Kommentar einer Probe** (`:995`), der genau sagt, dass das Modul beide nicht
kennenlernen soll. Die Zusage aus C1.7 hält, und die neue Zählprobe
`die_menge_der_eigenen_textflaechen_steht_an_genau_einer_stelle` misst sie nach.

### Die Umrechnung UTF-16 gegen Bytes steht an einer Stelle

`byte_zur_stelle` (`markdown.rs:493-501`) ist die einzige Umrechnung von einer Textstelle
auf ein Byte, und sie wird nur aus dem `Woertlich`-Zweig von `Quellbezug::beitrag` gerufen.
Die übrigen Vorkommen von `encode_utf16` sind Zählungen im Durchgang, keine Umrechnungen;
außerhalb von `markdown.rs` gibt es in `krk-ui` keine zweite. Eine Grenze mitten in einem
Ersatzpaar liefert das Byte **hinter** dem Zeichen statt einer ungültigen Zeichengrenze;
das ist ausgeschrieben und verhindert den Abbruch beim Zugriff auf die Quelle. Jede
Schnittstelle, an der die Quelle geschnitten wird, liegt damit auf einer Zeichengrenze.

### Die drei berichtigten Zählerwartungen gingen in die richtige Richtung

Der Befund `issues/260820-0646_o_…` nennt drei Erwartungen, die der Plan behauptet und der
Baum nicht getragen hat. Alle drei sind nachgeprüft, **keine legitime Fundstelle ist still
verschwunden**:

| Fall | Erwartung im Plan | Was im Baum steht | Was die Probe jetzt tut |
|---|---|---|---|
| Schritt 3 | `setSelectable(false)` kommt nicht mehr vor | `belegungsansicht.rs:677` setzt es an einem `NSTextField` — nachgelesen, die Zeile steht und gehört dorthin | zählt je Datei und erwartet genau diese eine Stelle, dazu `setEditable(false)` genau einmal in `vorschau.rs` |
| Schritt 5 | `fn fokusansicht` steht genau einmal | zweimal: `anwendung.rs` und `vorschau.rs`, zwei Hälften derselben Frage | erwartet beide Stellen namentlich |
| Schritt 6 | `text_auf_ablage_schreiben` hat bis Schritt 7 keinen Rufer | `text_schreiben` ruft sie sofort — die Verdrahtung verlangt derselbe Schritt | die `expect(dead_code)`-Zeile ist weggelassen, `-D warnings` läuft grün |

Die berichtigten Proben sind schärfer als die geplanten: sie zählen je Datei statt im Baum
und schreiben ihre blinden Flecken aus.

### Was der Bau erzwingt

- `#![deny(unsafe_code)]` steht unverändert an den drei Kistenwurzeln; `#![allow(unsafe_code)]`
  steht weiterhin nur in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`,
  unter dem die neuen `unsafe`-Stellen in `vorschau.rs` liegen (C4.3).
- Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` steht in **jeder**
  Datei unter `appkit/` außer `koordinaten.rs` und `mod.rs`, den zwei begründeten Ausnahmen.
  Der Abschnitt in `vorschau.rs` nennt `Vorschautext`, `initWithFrame:`, `setSelectable:`,
  `isHidden`, `writeSelectionToPasteboard:types:`, `selectedRange`, `NSPasteboard`,
  `NSPasteboardType` und `NSArray` je mit Kopfzeile und Zahl (C4.4). Die Richtigkeit der
  Zahlen ist Augenschein am SDK und bleibt Nutzerarbeit.
- `#[must_use]` trägt in `markdown.rs` jeder neue Typ und jede neue Antwort, die still
  fallengelassen werden könnte: `Quellbezug`, `Abschnitt`, `Quellelement`, `quelltext`,
  `beruehrt`, `verdeckt_quelle`, `byte_zur_stelle`, dazu `quellbezug()` und
  `text_auf_ablage_schreiben`. Die eine Lücke ist N2.
- Die Fallunterscheidungen der Runde sind vollständig und ohne Auffangzweig:
  `Abschnittsart::verdeckt_quelle` liest über `match`, `Quellbezug::beitrag` ebenso, und
  `Vorschaufenster::fokusansicht` hat zwei Zweige über einen Wahrheitswert.
- Die Prosa ist durchgehend deutsch, und die neun berichtigten Stellen sind einzeln
  nachgelesen: `ereignisse.rs`, `vorschau.rs` (Modulkopf und `textanzeige`), `menue.rs`
  (Modulkopf und Probenkommentar), `textautomatik.rs`, `editor.rs`, `zulaessigkeit.rs`,
  `fokus.rs`. Jede nennt die Runde und lässt den tragenden Teil der alten Aussage stehen.

---

## Beobachtungen ohne Befund

**Die Klammerregel wirkt für Blockelemente blockweise, und das ist vermutlich gewollt.**
Gemessen: eine Auswahl von zwei Wörtern in einem langen Listenpunkt liefert den ganzen
Punkt samt `- `, in einem Zitat das ganze Zitat samt `> `, in einem Quelltextblock den
ganzen Block samt Zäunen. Für Überschriften sagt der Datensatz das ausdrücklich zu; für
Listenpunkt, Zitat und Quelltextblock folgt es aus der Wohlgeformtheit, denn deren
Merkzeichen wiederholt sich auf jeder Zeile. Es steht hier, damit die Bündelabnahme nicht
überrascht wird, und nicht als Befund.

**Eine leere Auswahl an der Stelle 0 liefert nicht nichts.** `Abschnitt::beruehrt` liest für
Abschnitte ohne Textzeichen das **geschlossene** Intervall — das ist die Halbregel, an der
C2.8 hängt, und sie ist richtig. Als Nebenwirkung trägt eine leere Auswahl an einer solchen
Stelle den dortigen Vorspann bei und die zweite Stufe erweitert danach auf sein Element.
Erreichbar ist das nur, wenn ein Ausgabeweg ohne Auswahl kopiert; C2.11 sagt zu, dass der
Menüeintrag dann grau ist, und die Ausgrauung kommt aus der Antwortkette. Kein Befund,
solange die Bündelabnahme das bestätigt.

**Der Speicherzuwachs der Kachelung ist weiterhin ungemessen.** Der Plan sagt das an
derselben Stelle. Die Abschnittsliste wächst mit der Zahl der Ereignisse und liegt im
`Arc`, wird also beim Klon je Neuzeichnen nicht kopiert; gerechnet wird sie einmal je
Lesevorgang auf dem Arbeitsfaden. Der Befund gehört der späteren Messrunde und nicht dieser
Durchsicht.

---

## Reihenfolge

**Vor der Bündelabnahme:** H1 und H2. Beide sitzen in derselben Funktion, beide sind ohne
Fenster prüfbar, und beide machen die Bündelabnahme von C2.2 und C2.9 andernfalls
missverständlich — der Nutzer sähe an einer Überschrift ohne führendes Kind das richtige
Ergebnis und hielte die Runde für abgenommen.

**Mit der Bündelabnahme:** M1 und M2. M1 hängt an der Frage, ob die eine Abfangstelle alle
fünf Wege trägt; M2 ist in einem Handgriff nachzusehen und gehört in die Liste unter
`## Nutzerarbeit`, die ihn heute nicht führt.

**Danach:** N1 und N2.
