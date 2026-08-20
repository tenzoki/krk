# Planner-Sitzung 260819-2245: Umsetzungsplan zur Auswahl und zum Kopieren in der Vorschau

**Agent:** planner
**Circle:** `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau` (aktiv)
**Baumstand:** `fce0b6f`
**Ergebnis:** `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`

## Auftrag

Umsetzungsplan zum abgenommenen Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Der Nutzer hat den Spec am 260819-2228 abgenommen. Sieben Entscheidungsdatensätze vom
260819-2242 binden die Runde, alle beantwortet, alle unter `shared/decisions/260819-2216_a_*.md`.

## Was gelesen wurde

Spec vollständig, Circle-Datensatz `_t_circle.md`, alle sieben Entscheidungsdatensätze
(zwei davon im Wortlaut ihrer Optionen: die Randfrage und die Frage nach den Ausgabewegen),
`CLAUDE.md` am Baumstand, dazu am Code: `markdown.rs` (Modulkopf, `rendern`, `Zerlegung`
vollständig), `vorschaumodell.rs` (`Inhalt`, `laden`), `appkit/vorschau.rs` (Modulkopf,
`Inhaltsflaeche`, `anzeigen`, `text_zeigen`, `textanzeige`, `fokusansicht`,
`textView:menu:forEvent:atIndex:`), `appkit/ereignisse.rs` (Modulkopf,
`ersthelfer_gehoert_appkit`, die Zählprobe), `appkit/anwendung.rs` (`lage`,
`ist_editorflaeche`, `fokusansicht`, `bereich_des_ersthelfers`),
`appkit/zwischenablage.rs`, `appkit/menue.rs` (die Antwortkettenprobe),
`quellbaum.rs`, `krk-core/tests/baum.rs`, `resources/default-keymap.toml`,
sowie der Abschnitt „Was der Übersetzer einfordert" aus dem Plan der Runde 13.

## Was der Plan entscheidet

Die sieben Punkte, die der Spec dem Planner überlässt:

1. **Wo Quelltext und Abbildung wohnen.** Als drittes Feld `quellbezug: Arc<Quellbezug>` an
   `Gerendert`, damit Text und Abbildung strukturell nicht auseinanderlaufen können und der
   Klon je Neuzeichnen ein Zählerschritt bleibt.
2. **Die Gestalt der Abbildung.** Eine Kachelung, die **beide** Seiten lückenlos deckt, mit
   drei Abschnittsarten (`Woertlich`, `Ersetzt`, `Erzeugt`). Die Totalität aus C2.6 wird damit
   zu zwei maschinell nachmessbaren Zusagen statt zu einer Aufzählung von Fällen.
3. **Die Randregel.** Ein Fixpunkt über die Elemente, die eine Klammer tragen. Die Bedingung
   „trägt eine Klammer" ist der Unterschied zwischen der gewählten Möglichkeit b und der
   verworfenen blockweisen Möglichkeit; ohne sie blähte ein Absatz jede Auswahl darin auf.
4. **Die Abfangstelle.** `writeSelectionToPasteboard:types:` in einer Unterklasse von
   `NSTextView`. Sie ist die eine Stelle, an der AppKit `cmd+c`, Menü, Kontextmenü, Dienste
   und Ziehen zusammenführt — Erschließung, am Bündel abzunehmen.
5. **Die Anmeldung im Ereignisabgriff.** Ein Abschluss bleibt einer; die zwei Vergleiche
   stehen beim Delegierten, der beide Flächen ohnehin hält. `appkit/ereignisse.rs` lernt
   weder Editor noch Vorschau kennen.
6. **`fokusansicht`.** Liefert die Textanzeige, solange die Bildlaufansicht steht, sonst die
   Inhaltsfläche. Der Zweig ist nötig, weil dieselbe Zuordnung seit der Runde 6 auch den
   Anker des Freigabedialogs liefert und eine ausgeblendete Ansicht für beides untauglich ist.
7. **Reihenfolge.** Vier Bündel, acht Schritte, alle für den `coder`; der Abhängigkeitsgraph
   steht im Plan.

## Zwei Berichtigungen am eigenen Entwurf

- **Der erste Entwurf des Ablaufdiagramms trug einen Kreis** (Quellbezug → anzeigen → Fläche →
  Überschreibung → Quellbezug). Die Architektur hat ihn nicht; das Diagramm ist neu gezeichnet,
  statt den Kreis in Prosa wegzuerklären.
- **Der erste Entwurf berief sich für „der Plan der Runde 6 wird nicht angefasst" auf die
  Ortsregel aus `CLAUDE.md`.** Falsch: jene Regel zählt `planning/` gerade **nicht** zu den
  ausgenommenen Orten. Der tragende Grund ist ein anderer und steht jetzt da: der Spec ist das
  ersetzende Werkzeug, und die Kostenliste des Specs ist nur lesbar, solange die abgelöste
  Zusage im Wortlaut noch irgendwo steht.

## Was der Plan nicht tut

- **Kein Schritt für `ontocoder`.** C1.14 und C4.2 sagen zu, dass die Belegungsdatei keinen
  Eintrag bekommt; C4.6 schließt eine neue fremde Kiste aus.
- **Kein Schritt für `analyst`.** Die Runde bringt keinen strategischen Datensatz hervor; die
  sieben Fragen waren vor dem Plan beantwortet.
- **Kein neuer Entscheidungsdatensatz und kein neuer Defektdatensatz.** Keine der sieben
  Planentscheidungen bindet Arbeit über diese Runde hinaus, und am Baum ist nichts gefunden
  worden, was falsch oder widersprüchlich wäre.
- **Nichts committet.** Der Orchestrator committet und geht mit dem Plan zum Nutzer.

## Erhobene Zahlen

- 39 Abnahmekriterien im Spec (C1: 15, C2: 13, C3: 4, C4: 7).
- **15 davon tragen einen Bündelanteil**, nicht 14, wie die Anweisung an den Planner sagte;
  gezählt an den Vorkommen von `(Bündel` in den Kriterienlisten. Die Nutzerarbeitstabelle des
  Plans führt sie einzeln auf und nennt daneben C4.4, das Augenschein und keinen Bündellauf
  verlangt.
- Der Übersetzer hält 5 Stellen, eine Probe 6, nichts hält 4 — die vier sind Prosastellen in
  Modulköpfen, kein Ausführungszweig. Die gefährlichste Fläche des Projekts, der Auffangzweig
  in `kommando_ausfuehren`, fällt für diese Runde weg, weil sie kein Kommando anlegt.

## Voice-Profile

`chat-voice-de.yaml` und `default-voice-de.yaml`. Die zweite Sprachzeile ist am Baum nicht mehr
vorhanden; `CLAUDE.md` erklärt die fehlende Zeile ausdrücklich, damit steuert `**Language:** de`
Chat- und Artefaktsprache. Der Plan ist deshalb deutsch geschrieben, die Kopfmarke
`**Decidability:**` englisch, weil sie aus der ausgelieferten Vorlage stammt.
