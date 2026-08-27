# Orchestrator-Sitzung — 260827-0622

**Directive:** siehe `**Active spec/plan:**` im Circle-Datensatz, solange dort `(none yet)` steht: der Abschnitt `## Directive` von `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/_t_circle.md`.
**Mode:** custom (Circle-Lauf), Phase 0b folgt
**Status:** Laufend

## Herkunft

Die Sitzung ist um 260826-2245 mit `/fusion:setup` gestartet und lief bis zur Aktivierung
ohne Circle; ihr Aufsatzpunkt steht in
`shared/history/260826-2245-orchestrator-session.md`. Mit der Aktivierung um 260827-0622
wandert das Protokoll hierher, weil jeder Speicher dieser Sitzung ab jetzt im Circle liegt.

Vorgeschichte der Runde:

- Rückstandseintrag `shared/backlog/260826-1920_*_vorschau-default-profil-zaehlt-ordnerinhalt.md`,
  vom Nutzer als nächste Runde bestimmt.
- `/fusion:direct` auf diesen Eintrag; der Shaper hat in einer Fragerunde vier Festlegungen
  geholt und den Circle angelegt (`history/260827-0313-shaper-…`).
- `/fusion:next` mit ausdrücklichem Ziel: Portfolio aufgefrischt, Datensatz von `_a_` auf `_t_`,
  Zeiger `.active-circle` gesetzt, Anspruch eingetragen.

## Aufsatzpunkt

| Größe | Wert |
|---|---|
| git HEAD bei Aktivierung | `eced324` |
| Turn-Budget | 12 |
| Bereich (domain) | `code` |
| Kennung | Kai Stalmann <kai@stalmann.org>, checkout `6c11b1f2` |

Zwei offene Fragen liegen im Circle und binden die Planung:

- `decisions/260827-0311_*_bekommen-die-profile-…` — wird die Zählung nach Typ und versteckt
  eine allgemeine Fähigkeit der Profile, oder bleibt sie dem Default-Profil vorbehalten?
- `decisions/260827-0311_*_was-sagen-die-zaehlzeilen-…` — was sagen die drei Zeilen für einen
  Ordner oberhalb der Eintragsschranke?

## Verlauf

(wird während der Sitzung fortgeschrieben)

## Antworten des Nutzers, 260827-0629

Beide offenen Fragen des Circles sind beantwortet, jeweils Möglichkeit 1:

- Zählung nach Typ und versteckt: der Baustein `zaehlung` bekommt zwei weitere, freiwillige
  Kriterien, und das Default-Profil benutzt dieselbe Maschine wie jedes Profil aus
  `readers.toml`. Kein zweiter Zählweg im Baum.
- Ordner über der Eintragsschranke von zweitausend: es gilt dieselbe Regel wie beim Baustein
  `zaehlung`. Jede der drei Zeilen sagt „mindestens N (Lesung bei 2.000 Einträgen
  abgebrochen)", und die Klammer mit den versteckten entfällt in dieser Lage ganz. Die
  Schranke bleibt, wo sie steht; gemessen wird für diese Runde nichts.
