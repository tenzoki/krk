# R1 — ein Leseprofil für den Forum-Speicher der fusion-Werkbank

**Agent:** ontocoder
**Datum:** 260908-1754
**Status:** Complete (Datenänderung fertig und gegen den echten Bestand
gemessen; die Abnahme ist rot, weil fünf Zahlenzusagen unter `crates/` nachzuziehen
sind, und `crates/` war dieser Bahn verschlossen)

## Auftrag

Die Vorschau soll für `fusion-workbench/shared/forum` eine Nachrichtenübersicht
zeigen statt der allgemeinen Ordnerauskunft, und dieselbe Auskunft verdichtet
dort, wo die fusion-Werkbank zusammengefasst wird. Geändert werden durfte
allein `resources/default-readers.toml`.

## Geändert

`resources/default-readers.toml`, an vier Stellen:

1. **Neues Profil** `fusion-Werkbank: der Forum-Speicher`,
   `pfad = 'fusion-workbench/(shared|circles/[^/]+)/forum$'`, drei Zeilen:
   `Nachrichten` (Zählung), `Zuletzt angekommen` (`zeigt = "datum"`),
   `Die jüngsten zehn` (Titel). Ein Leselauf, höchstens zehn Öffnungen.
2. **`fusion-Werkbank: die Wurzel`** bekommt als achte Zeile
   `Nachrichten` über `shared/forum`.
3. **`Projektwurzel mit fusion-Werkbank`** bekommt dieselbe Zeile über
   `fusion-workbench/shared/forum`. Die zwei Blöcke sind laut Datei gekoppelt;
   eine Zeile allein in einem von beiden wäre genau die Drift, vor der die
   Datei warnt.
4. **Der Kostenabsatz** im Kopf: die zwei Beispielzahlen stehen auf vier und
   fünf statt drei und vier, und ein neuer Absatz schreibt aus, dass ein Ort,
   den es nicht gibt, gar keinen Leselauf kostet.

## Der Befund, der die Auftragsbeschreibung berichtigt

Der Auftrag behauptete: „Jede Ortsangabe kostet einen Lesevorgang, auch wenn
der Ordner nicht da ist." **Das stimmt nicht.** `ort_aufloesen` →
`zielordner` → `innerhalb` löst den Pfad über `canonicalize` auf und gibt
`None` zurück, bevor `stand_am` überhaupt gerufen wird; gebucht wird nichts.
Die zitierte Stelle in `CLAUDE.md` sagt etwas anderes — sie handelt vom
Platzhalterlauf, der als **ein** Ort **einen** Lauf kostet, und nicht vom
fehlenden Ordner.

Gemessen, nicht geschlossen: an KRKs eigener Werkbank ohne Forum kostet das
Wurzelprofil unverändert drei Läufe und vier Öffnungen, das Projektwurzelprofil
unverändert vier und vier; die neue Zeile steht auf `--`. **Ein Nutzer ohne
Forum zahlt für die Zeile nichts.**

## Wie gemessen wurde

Ein Wegwerf-Arbeitsbaum (`git worktree` auf `85bcbad`, eigener
`CARGO_TARGET_DIR`, beides nach der Messung entfernt), darin die geänderte
Profildatei, der Nachzug der Proben und eine wieder gelöschte Probe, die KRKs
eigenen Auswerter (`zusammenfassen_gezaehlt`) gegen die **echte** fusion-
Werkbank unter `/Users/k1/Projects/productive/fusion` laufen lässt. Der
Arbeitsbaum von KRK hat dabei keine Datei unter `crates/` gesehen.

Ergebnis am echten Bestand:

| Ort | Läufe / Öffnungen | Auskunft |
|---|---|---|
| `…/fusion/fusion-workbench/shared/forum` | 1 / 1 | `Nachrichten: 1`, `Zuletzt angekommen: 2026-09-08 13:14`, Titel `Neuer Befehl: sehen, was angekommen ist, bevor du ziehst` |
| `…/fusion/fusion-workbench` | 4 / 5 | achte Zeile `Nachrichten: 1` |
| `…/fusion` (Projektwurzel) | 5 / 5 | achte Zeile `Nachrichten: 1` |
| `…/krk/fusion-workbench` (kein Forum) | 3 / 4 | `Nachrichten: --` |
| `…/krk` (kein Forum) | 4 / 4 | `Nachrichten: --` |
| `…/fusion/fusion-workbench/shared` | 10 / 0 | unverändert, Abstand zur Schranke weiter zwei |

Die Titelregel trägt: die nackte erste Zeile ohne Raute wird als Titel gelesen.

## Entscheidungen

**Zehn Titel** im Forum-Profil, wie jeder andere Speicher der Datei. Die
vierzehntägige Aufbewahrung macht die Zahl zur Obergrenze statt zu einem
Ausschnitt; eine kleinere Zahl wäre eine Sonderregel ohne Gegenstand.

**Die verdichtete Zeile geht an die zwei Wurzelprofile und nicht an
`fusion-Werkbank: der gemeinsame Speicher`.** Zwei Gründe: der gemeinsame
Speicher steht mit zehn von zwölf Leseläufen zwei vor der Schranke und hat
dafür eine eigene Gegenprobe, während die Wurzelprofile bei drei und vier
stehen; und eine Nachricht für die Gegenseite muss dort auffallen, wo man
ohnehin steht. Die Lücke, die das im Speicherprofil hinterlässt, ist als
Datensatz abgelegt und nicht stillschweigend geblieben.

**Eine Zahl und kein Titel** in der verdichteten Form: `zaehlung` öffnet keine
Datei, die sechs Nachbarwerte sind kurz, und eine Nachrichtenüberschrift ist
ein ganzer Satz. Nebenbei trennt die Zahl zwei Lagen, die eine Null zusammenzöge
— `--` heißt „keine Forum-Ablage", `0` heißt „eine, und sie ist leer".

## Abgelegt

- `260908-1754_*_das-forum-profil-macht-fuenf-zahlenzusagen-in-drei-dateien-unter-crates-falsch.md`
  (Defekt, für `coder`) — der vollständige, gemessene Nachzug, nach dem
  `cargo test -p krk-core` grün ist.
- `260908-1754_*_bekommt-das-profil-des-gemeinsamen-speichers-die-zwei-forum-zeilen-die-es-jedem-anderen-unterspeicher-gibt.md`
  (Entscheidung) — die Lücke im Speicherprofil mit ihren zwei Auslösern.

## Abnahme

`cargo test -p krk-core` — Exit 101. Zwei Proben im Lib-Ziel fallen, vier im
Ziel `leseprofil`, eine in `ablage`; **alle allein an fest verdrahteten
Zahlen**, keine an der Gültigkeit der Datei. `datei::pruefen` beanstandet
nichts (`meldungen.is_empty()` läuft durch). Nicht committet.
