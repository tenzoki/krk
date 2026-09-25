# Auswahl und Kopieren in der Vorschau

---
**Domain:** code
**Filed by:** orchestrator
**Active spec/plan:** `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/planning/260819-2245_*_plan-auswahl-und-kopieren-in-der-vorschau.md` — der Plan in Kraft, vom Nutzer am 260819-2252 abgenommen. Der Spec dazu, vor diesem Circle entstanden und deshalb im gemeinsamen Speicher: `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
**Active session history:** shared/history/260819-2026-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

Die Grundlage ist am 260819-2216 vom Shaper am Baumstand `6be1e81` erhoben und steht
ausformuliert im Spec, Abschnitt `## Ausgangslage, am 260819-2216 am Baum erhoben`. Sie wird
hier nicht zweitgeschrieben; was diese Runde trägt, sind die vier Punkte, ohne die sie nicht
zu verstehen ist:

- **Die Vorschaufläche ist längst eine `NSTextView`.** Zwei Zeilen schalten die Auswahl ab,
  `crates/krk-ui/src/appkit/vorschau.rs:1120-1121`. Der Eingriff nimmt einen der beiden
  Schalter zurück und fügt keine Ansicht hinzu.
- **Die Unauswählbarkeit war eine abgenommene Zusage, kein Versehen.** Das achte
  Abnahmekriterium von C4 der Runde 6 und die Zeile 417 ihres Plans sagen sie zu; der Nutzer
  hat sie am 260812-1105 selbst gewählt und die jetzt gefahrene Möglichkeit damals abgelehnt.
  Der Spec ersetzt beide Stellen ausdrücklich, im Abschnitt
  `## Was diese Runde an der Runde 6 ändert`.
- **Der Weg, Auswahl und KRK-Tasten zugleich zu behalten, ist gebaut.** Der Editor der Runde 2
  ist eine bedienbare `NSTextView` und in `ersthelfer_gehoert_appkit`
  (`crates/krk-ui/src/appkit/ereignisse.rs:685`) über die **Nämlichkeit** angemeldet, nicht
  über die Klasse. Die Vorschaufläche wird dort ein zweites Mal angemeldet, an derselben
  Stelle.
- **Kopieren ist in KRK kein eigener Befehl.** `text_kopieren` trägt
  `gehalten_von = "menue"`, der Menüeintrag hat Ziel `nil` und Selektor `copy:` und läuft die
  Antwortkette hinunter. Die Runde braucht weder einen Belegungseintrag noch eine
  `Kommando`-Variante.

Sieben Entscheidungsdatensätze vom 260819-2216 liegen beantwortet im gemeinsamen Speicher und
binden diese Runde; sie sind im Spec je Kriterium zitiert.

## Dependencies

- `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` — diese Runde ersetzt zwei
  ihrer Zusagen und überholt die zweite Hälfte ihres Datensatzes
  `decisions/260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`.
  Die erste Hälfte, was ein Verweis im gerenderten Markdown tut, gilt unverändert weiter.
- `260807-2116-eingebauter-editor-mit-textmarken` — liefert das Muster der Anmeldung über die
  Nämlichkeit, das diese Runde ein zweites Mal anwendet. Nicht geändert, nur nachgeahmt.
- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` (vorgesehen) — dieser Runde ist
  ausdrücklich untersagt, seine erste offene Frage vorwegzunehmen, nämlich welche Quellen eine
  Adresse setzen dürfen. Anklickbare Verweise sind nicht Gegenstand.

## Turn log


## Closure note

**Kohärent geschlossen am 260820-1045.** Sitzungsprotokoll:
`shared/history/260819-2026-orchestrator-session.md`. Abgleich:
`circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/history/260820-0834-reconciliation.md`.
Durchsicht: `.../reviews/260820-0745-coderev-auswahl-und-kopieren-in-der-vorschau.md`.

**Die Directive ist erreicht, und der Nutzer hat sie am laufenden Bündel abgenommen.** Der
Bündeldurchgang lief am 260820-1030 an `KRK.app` 0.5.4 aus `05cb614`; sein Befund lautet, die
neuen Funktionen halten. **Das ist in diesem Projekt die Ausnahme:** zehn der dreizehn Runden
davor sind beschränkt geschlossen, weil der Abnahmelauf den Nutzer im Vordergrund verlangt und
kein Agent ihn fahren kann.

**Der Marker `_c_` steht auf einer gefahrenen Abnahme und nicht auf ihrem Ausbleiben.** Der
Abgleich vom 260820-0834 hatte `review-needed` verdiktiert, und zwar allein deshalb, weil 15 der
39 Abnahmekriterien ungefahren waren. Dieser Grund ist mit dem Durchgang des Nutzers entfallen.

## Was die Runde gebracht hat

Die Vorschaufläche ist auswählbar; alles, was die Textfläche zeigt, lässt sich markieren, ein
Bild nicht. Bei gerendertem Markdown landet **der Quelltext mit seinen Auszeichnungszeichen** in
der Ablage und nicht der gerenderte Text — die Entscheidung, die der Nutzer am 260819-2210 gegen
die Empfehlung des Shapers getroffen hat. Dafür trägt der Renderdurchgang seit `13be459` eine
zweite Auskunft: eine Kachelung, die gerenderten Text und Quelle beidseitig lückenlos aufeinander
abbildet. Die Tastenbedienung der Vorschau ist unangetastet geblieben, weil die Textanzeige in
`ersthelfer_gehoert_appkit` über die Nämlichkeit angemeldet ist — dasselbe Muster, das der Editor
seit der Runde 2 nutzt, ein zweites Mal angewandt und nicht ein zweites Mal erfunden.

**Zwei abgenommene Zusagen der Runde 6 sind ersetzt und nicht ergänzt:** das achte
Abnahmekriterium von C4 und die Umsetzungszusage in Zeile 417 ihres Plans. Der Datensatz, der
beide trug, ist zur **Hälfte** überholt: `circles/260812-1000-…/decisions/260812-1000_s_…`. Seine
erste Hälfte, was ein Verweis im gerenderten Markdown tut, gilt unverändert weiter und gehört
weiterhin dem vorgesehenen Circle des Web-Betrachters.

## Was offen bleibt, und warum

**Vier Befunde der Durchsicht, auf ausdrücklichen Nutzerentscheid vom 260820-0750.** Der
gewichtigste ist `260820-0733_o_`: die Abfangstelle liest die geforderten Sorten nicht und leert
jede gereichte Ablage. Damit ist die Zusage „eine Stelle für alle Ausgabewege" für die
Zwischenablage eingelöst, für das Ziehen und die Dienste **nicht**. Der zugehörige
Entscheidungsdatensatz `shared/decisions/260819-2216_a_gilt-die-quelltextzusage-…` steht deshalb
auf beantwortet und nicht auf umgesetzt.

**Ob der Nutzer das Ziehen und die Dienste im Durchgang geprüft hat, ist nicht aufgezeichnet.**
Sein Bericht nennt sie nicht, weder als haltend noch als fehlschlagend. Der tragende Vorbehalt
der Runde — dass AppKit alle fünf Ausgabewege durch dieselbe Methode führt — bleibt damit
unbelegt, und `260820-0733_o_` misst am Baum, dass er in dieser Form nicht trägt.

**Der zweite offene Entscheidungsdatensatz** ist der zu L7: die Antwort „kein Abnahmelauf" ruht
auf zwei Ersatzkriterien, von denen eines keinen Prüfer hat (`260820-0737_o_`).

**Drei Befunde aus dem Abnahmelauf gehören nicht dieser Runde** und liegen im gemeinsamen
Speicher: `cmd+e` bleibt in der Vorschau wirkungslos, `f4` setzt den Fokus nur in einen bereits
sichtbaren Editor, und eine Taste zum Umschalten zwischen Editor und Vorschau fehlt. Geprüft und
nicht vermutet: keiner der drei stammt aus dieser Runde.

**`CLAUDE.md` trägt seit `6531f38` eine falsche Aussage** — die Textfläche des Editors sei die
eine Ausnahme im Ereignisabgriff; es sind zwei — dazu drei unvollständige. Nicht angefasst; ein
Kuratorendurchgang ist dem Nutzer vorbehalten.

## Was die Runde über das Verfahren gelernt hat

Viermal hat der Plan eine Zählerwartung ausgeschrieben, die am Baum nicht zutraf, und viermal hat
der ausführende Coder die Erwartung an den Baum angepasst statt umgekehrt
(`issues/260820-0646_o_`). Neun Prosastellen waren falsch geworden, wo der Plan vier führte, und
der Grund ist benannt: eine Erhebung, die von den geänderten Dateien ausgeht, findet die übrigen
nicht (`issues/260820-0731_c_`). Beide Befunde binden künftige Runden, nicht diese.
