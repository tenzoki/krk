# Auswahl und Kopieren in der Vorschau

---
**Domain:** code
**Filed by:** orchestrator
**Active spec/plan:** shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md
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

