Zwei Feldmuster der geplanten Auslieferungsfassung verankern mit `^` und können nie treffen

---

Schritt 7 des Plans schreibt die Muster von `resources/default-readers.toml` wörtlich vor. Zwei
davon treffen in der Kiste `regex` nie: `(?s)^## Directive\s*\n+(.+?)\n\n` auf dem
Circle-Datensatz und `^(.+)$` auf `.active-circle`. Der Grund ist derselbe für beide: `regex`
verankert `^` und `$` ohne die Angabe `m` an **Anfang und Ende der ganzen Eingabe** und nicht
an denen einer Zeile. Damit fallen die Zusammenfassung des einzelnen Circles um ihre Directive
(C5.6, C3.9) und die der Werkbankwurzel um den Namen des aktiven Circles (C3.8).

---

**Gemessen am 260824-1124, an diesem Baum, in der Probe
`crates/krk-core/tests/leseprofil.rs::das_feld_zieht_die_erste_fanggruppe_des_ersten_treffers`.**

## Der Befund, wie er entstanden ist

Die Probe zu C3.9 hat das Feldmuster aus Schritt 7 wörtlich übernommen und gegen einen
Prüfordner in der Gestalt eines Circles gehalten. Sie lief rot: `Wert::Nicht` statt des Absatzes.
Mit `(?sm)` statt `(?s)` läuft sie grün. Die Probe trägt die Angabe seitdem mit einem Kommentar,
der auf diesen Datensatz verweist.

## Die vier Muster der Wurzel und des Circles im Einzelnen

| Muster aus Schritt 7 | Gegenstand | Trifft? |
|---|---|---|
| `"plugin_version":"([^"]*)"` | `.fusion-setup` | ja, keine Verankerung |
| `"setup_at":"([^"]*)"` | `.fusion-setup` | ja |
| `"setup_pwd":"[^"]*/([^"/]+)"` | `.fusion-setup` | ja |
| `^(.+)$` | `.active-circle` | **nein** |
| `(?s)## Current\n\s*(.+?)\n` | `orchestrator-live.md` | ja, keine Verankerung |
| `(?s)^## Directive\s*\n+(.+?)\n\n` | `_._circle.md` | **nein** |

`^(.+)$` scheitert an einer zweiten Stelle zusätzlich: `.active-circle` endet nach dem
Circle-Namen auf ein Zeilenende, `.` deckt ohne die Angabe `s` kein Zeilenende, und `$` verlangt
das Ende der Eingabe. `regex` kennt die Nachsicht von Python, `$` auch vor einem abschließenden
Zeilenende treffen zu lassen, ausdrücklich nicht.

**Die Kennzeichen- und Pfadmuster sind nicht betroffen.** Sie laufen gegen einen Eintragsnamen
oder gegen einen Pfad, und beides ist eine einzige Zeile ohne Zeilenende; dort heißt „Anfang der
Eingabe" dasselbe wie „Anfang der Zeile". Betroffen ist allein der Feldbaustein, weil er als
einziger gegen einen **Dateiinhalt** läuft.

## Was zu tun ist

In Schritt 7, also in `resources/default-readers.toml`, und im Plan an der Stelle, die die
Muster vorschreibt:

1. `(?s)^## Directive\s*\n+(.+?)\n\n` wird `(?sm)^## Directive\s*\n+(.+?)\n\n`.
2. `^(.+)$` wird `^([^\n]+)`, oder mit derselben Wirkung `(?m)^(.+)$`. Die erste Fassung kommt
   ohne die Angabe `m` aus und sagt zugleich, was gemeint ist: der Name steht in der ersten
   Zeile und hört vor dem Zeilenende auf.
3. Der Kommentarabschnitt der Datei, der nach C5.10 alle vier Bausteinnamen an einem Beispiel
   zeigt, sollte den Satz mittragen: ein Feldmuster läuft über den ganzen Dateiinhalt, und wer
   eine Zeile verankern will, schreibt `(?m)`.

## Nachgemessen, nicht hergeleitet

Beide Befunde sind in dieser Kiste gelaufen, in einer Wegwerfprobe unter
`crates/krk-core/tests/`, die danach wieder entfernt wurde:

```text
"^(.+)$"        gegen "260823-2208-vorschau\n"  ->  None
"^([^\n]+)"     gegen "260823-2208-vorschau\n"  ->  Some("260823-2208-vorschau")
"(?m)^(.+)$"    gegen "260823-2208-vorschau\n"  ->  Some("260823-2208-vorschau")
"^(.+)$"        gegen "260823-2208-vorschau"    ->  Some("260823-2208-vorschau")
```

Die letzte Zeile ist die aufschlussreiche: ohne das abschließende Zeilenende trifft auch das
ursprüngliche Muster. Eine Probe, die ihre Eingabe im Quelltext ohne Zeilenende schreibt,
bestätigte das falsche Muster also und sagte nichts über die Datei auf der Platte.

**Schwere:** hoch für Schritt 7, keine für den gebauten Code. Der Feldbaustein arbeitet richtig;
falsch wären zwei der sechs mitgelieferten Muster, und beide gehören zu den Zusammenfassungen,
die der Nutzer bei der Abnahme zuerst ansieht.

**Gefunden:** coder, bei der Umsetzung von Schritt 6 am 260824-1124.

**Betroffen:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Schritt 7),
`resources/default-readers.toml` (steht noch aus)

**Domain:** code

---
Resolved: Die Messung ist am 260824-1224 unabhaengig nachgepruft, in einem Wegwerfprogramm gegen `regex` 1.13.1 ausserhalb des Baumes und gegen die echten Dateien dieser Werkbank: `^(.+)$` gegen `.active-circle` liefert keinen Treffer, `^([^\n]+)` und `(?m)^(.+)$` liefern beide den Namen; das alte Directive-Muster trifft null der achtzehn Circle-Datensaetze, das berichtigte alle achtzehn. Die vier uebrigen Ausdruecke sind im selben Lauf gegen `.fusion-setup` und `orchestrator-live.md` gehalten worden und treffen; die Kennzeichen- und Pfadmuster ebenso. Berichtigt sind: C3.8 des Specs auf `^([^\n]+)`, C3.9 um den Satz ueber die Angabe `m`, und Schritt 7 des Plans um beide Ausdruecke und um den Satz fuer den Kommentarabschnitt der Auslieferungsfassung. Die Berichtigung an C3.8 aendert ein freigegebenes Abnahmekriterium inhaltlich und ist dem Nutzer vorzulegen. `resources/default-readers.toml` steht weiterhin aus; Schritt 7 schreibt jetzt die richtigen Muster vor.
