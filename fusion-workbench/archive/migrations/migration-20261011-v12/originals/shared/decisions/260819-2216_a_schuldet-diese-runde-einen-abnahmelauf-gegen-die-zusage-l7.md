# Schuldet diese Runde einen Abnahmelauf gegen die Zusage L7?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (Abschnitt „Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1"); `crates/krk-bench/src/messen.rs:1109-1114` (L7); `crates/krk-ui/src/appkit/vorschau.rs` (Modulkopf, Abschnitt „Der Einfärbungsvorgang wohnt hier und nicht im Modell"); `messungen/260810-1918-alle-zusagen.txt`; `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`

---

## Question

L7 sagt zu, dass die Vorschau des ausgewählten Eintrags binnen 100 ms sichtbar ist, gemessen im Perzentil. Ihre Endbedingung ist `Vorschaumodell::laedt_noch`, und die Runde 6 hat den Einfärbungsvorgang eigens aus dem Modell herausgehalten, damit L7 nicht auf `syntect` wartet.

**Diese Runde legt Arbeit innerhalb dieser Endbedingung nach.** Der Quelltext bleibt stehen, statt weggeworfen zu werden, und die Abbildung von der Auswahl auf die Quelle entsteht im Durchgang, der ohnehin rendert. Beides liegt vor dem Zeitpunkt, an dem `laedt_noch` falsch wird.

Zu entscheiden ist, ob die Runde deshalb einen Abnahmelauf am laufenden Bündel schuldet. **Kein Agent kann ihn fahren**: der Lauf verlangt KRK im Vordergrund, und aus dem Hintergrund meldet die Messstrecke `NICHT_IM_VORDERGRUND` statt Zahlen. Es ist Nutzerarbeit, und deshalb ist es eine Frage an den Nutzer und keine Ableitung.

**Die Größenordnungen, soweit gemessen:** der Durchgang des Renderns kostet heute 19 bis 30 ms für 1,05 MB, das Budget von L7 beträgt 100 ms, und die Vorschau liest höchstens 1 MB (`TEXTGRENZE`). Der Quelltext ist die Eingabe des Renderns und wird nicht ein zweites Mal gelesen. Was die Abbildung kostet, ist nicht gemessen, weil es sie noch nicht gibt.

**Der Abnahmelauf der zehn Zusagen liegt neun Tage zurück und vor acht gefahrenen Runden.** Zuletzt am 260810 gefahren, alle zehn hielten. Keine Runde seither ist gegen ihn gemessen; das ist ein bestehender Zustand des Projekts und keine Folge dieser Runde.

## Options

1. **Kein Lauf in dieser Runde.** An die Stelle treten zwei ohne Messstrecke prüfbare Kriterien: die Abbildung entsteht in dem Durchgang, der rendert (C2.4), und sie liegt auf der Seite des Textes und nicht auf der der Einfärbung (C2.13). L7 wird als Gegenstand der späteren Messrunde benannt.
   - Folge: dasselbe Verfahren, das die Runde 10 für L2 und L3 gewählt hat. Die Runde kommt ohne Nutzerarbeit aus.
   - Preis: die Zahl bleibt ungemessen, und die Zahl der ungemessenen Runden wächst um eine.

2. **Ein Lauf am Ende dieser Runde, allein gegen L7.** Der Nutzer fährt die Messstrecke im Vordergrund und liest die eine Zahl.
   - Folge: die Zusage, die diese Runde berührt, ist danach gemessen. Der Lauf ist kürzer als der volle Abnahmelauf.
   - Preis: Nutzerarbeit, und die Runde schließt ohne sie nicht kohärent. Eine einzelne Zusage misst die Messstrecke zudem nicht isoliert, sondern innerhalb der Sitzungsstrecke, die L1, L5, L6, L7, L8 und L9 zusammen fährt.

3. **Der volle Abnahmelauf über alle zehn Zusagen.** Er ist ohnehin seit acht Runden fällig.
   - Folge: der Rückstand ist danach abgetragen, und alle zehn stehen wieder auf einem gemessenen Stand.
   - Preis: die meiste Nutzerarbeit von den dreien, und der Rückstand ist nicht die Schuld dieser Runde. Wer ihn hier abträgt, hängt eine Aufgabe an eine Runde, die sie nicht verursacht hat.

## Constraints

- Eine Zusage ohne Messstrecke wäre ein Wunsch; dieses Projekt hat in dreizehn Runden keine elfte gesetzt.
- Die Endbedingung von L7 bleibt `Vorschaumodell::laedt_noch`, und dieses beantwortet weiter allein „wartet ein Tab auf seinen Text".

## Recommendation

**Wir empfehlen Möglichkeit 1.** Der Zuwachs liegt in einem Durchgang, dessen gemessene Kosten ein Drittel des Budgets ausmachen, und die beiden Kriterien halten die Bauart fest, an der die Zusage hängt. Wer den Rückstand abtragen will, tut das besser in einer eigenen Messrunde als am Rand dieser: dort ließe sich ein Ergebnis auch der Runde zuordnen, die es verursacht hat.

## Antwort 260819-2242

**Möglichkeit a.** Kein Abnahmelauf in dieser Runde.

An die Stelle der Zahl treten zwei ohne Messstrecke prüfbare Kriterien, C2.4 über die Zahl der Durchgänge und C2.13 über den Ort der Abbildung. L7 kommt auf die Gegenstände der späteren Messrunde, neben die Geschwindigkeit der Syntaxhervorhebung aus C3 der Runde 2. Der Rückstand der zehn Zusagen seit dem 260810 bleibt bestehen und ist nicht von dieser Runde verursacht.

## Abgleich 260820-0834 — der Marker bleibt auf beantwortet

**Die Antwort lautet „kein Lauf“ und ist durch keinen Commit einzuloesen.** Eingeloest wird
sie durch das, was an die Stelle des Laufs treten sollte, und davon steht am Baum eine
Haelfte:

- **C2.13, der Ort der Abbildung: eingeloest.** Der Quellbezug liegt als Feld an `Gerendert`
  (`crates/krk-ui/src/markdown.rs:271`) und damit auf der Seite des Textes; die Probe
  `das_vorschaumodell_weiss_von_der_einfaerbung_nichts`
  (`crates/krk-ui/src/appkit/vorschau.rs:1558`) haelt die Zusage aus C4.11 der Runde 6 fest.
- **C2.3 und C2.4, die Zahl der Durchgaenge: ohne Probe.** Die Sache stimmt — `into_offset_iter`
  steht in `markdown.rs:582` genau einmal, und `Quellbezug::quelle` kommt aus
  `self.quelle.to_owned()` in `Zerlegung::abschliessen` (`:1594`), also aus der Eingabe des
  Durchgangs. Ein Kommando prueft es nicht nach. Gemessen ist das im offenen Befund
  `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/issues/260820-0737_o_zwei-abnahmekriterien-mit-probenkennzeichnung-haben-keine-probe.md`.

Der Verzicht auf einen Abnahmelauf haengt an diesen zwei Kriterien. Solange eines davon keine
Probe hat, ist der Ersatz nur zur Haelfte gebaut, und `_i_` behauptete mehr, als der Baum
traegt. Die dritte Haelfte der Antwort — L7 kommt auf die Gegenstaende der spaeteren Messrunde
— hat im Baum ueberhaupt keinen Ort; sie steht bislang allein in diesem Datensatz und im Spec.


---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented:
Deferred:
Superseded by:
