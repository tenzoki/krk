# Tote Zeiger in Modulköpfen auf die Sternform gebracht

---
**Status:** Complete
**Agent:** coder
**Datum:** 260818-0737
**Auftrag:** die fünf toten Zeiger unter `crates/` aus
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`
reparieren, dazu eine eigene Erhebung über `crates/`, `xtask/` und `resources/`.

---

## Bindende Grundlage

`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`
(umgesetzt seit `e49412a`): ein Zitat im lebenden Text schreibt die Sternform
`YYMMDD-HHMM_*_<slug>.md`. Der Geltungsbereich nennt `crates/` als Erstes.

## Die Erhebung

Ein Skript über `crates/`, `xtask/` und `resources/` nimmt jedes Vorkommen von
`[0-9]{6}-[0-9]{4}_[a-z*]_<slug>`, zerlegt es in Zeitstempel, Marker und Namensteil und löst
den Zeitstempel samt Namensteil mit `find fusion-workbench -name "<ts>_?_<slug>.md"` gegen
den Dateibestand auf. Der gefundene Dateiname liefert den heutigen Marker; der Vergleich
mit dem zitierten trennt vier Klassen: Sternform, ausgeschriebener Marker der noch stimmt,
ausgeschriebener Marker der nicht mehr stimmt, und kein Treffer.

Abgekürzte Zitate — die mit `…`, mit `-*.md` oder über einen Zeilenumbruch hinweg
geschriebenen — fallen im ersten Durchgang in „kein Treffer" und sind in einem zweiten
Durchgang über den Namensteil als Präfix aufgelöst worden.

**Gefunden: 424 Zitate.** Davon 383 vollständig geschrieben, 41 abgekürzt. Jedes der 41
löst über sein Präfix auf genau eine Datei auf, keines mehrdeutig.

## Befund vor der Reparatur

| Klasse | Zahl |
|---|---|
| Sternform, löst auf | 375 (+ 40 abgekürzte) |
| ausgeschriebener Marker, Ziel trägt ihn noch | 4 |
| ausgeschriebener Marker, Ziel trägt ihn nicht mehr | 5 |
| Sternform, Namensteil trifft keine Datei | 1 |

## Die zehn geänderten Zeilen

**Die fünf aus dem Datensatz** (Marker `_a_` geschrieben, Ziel steht seit `24bbccc` auf `_i_`):

| Stelle | Ziel |
|---|---|
| `crates/krk-core/src/verzeichnis/arbeitsbaum.rs:32` | `260817-0536_*_sieht-die-git-pruefung-…` |
| `crates/krk-core/src/verzeichnis/arbeitsbaum.rs:179` | `260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-…` |
| `crates/krk-core/src/verzeichnis/arbeitsbaum.rs:181` | `260817-0536_*_sieht-die-git-pruefung-…` |
| `crates/krk-core/src/verzeichnis/loeschzielbefund.rs:147` | `260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-…` |
| `crates/krk-core/src/verzeichnis/umfang.rs:152` | `260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-…` |

**Vier weitere, die der Datensatz nicht führt.** Sie sind keine toten Zeiger: ihr Ziel
trägt den zitierten Marker heute noch. Sie verstoßen gegen die Festlegung vom 260815 und
werden mit dem nächsten Zustandswechsel ihres Ziels zu toten Zeigern. Jede der vier steht
als reiner Zeiger da, nicht als Aussage über einen Zustand, also greift die Ausnahme des
Datensatzes nicht.

| Stelle | zitiert | Ziel |
|---|---|---|
| `crates/krk-core/tests/verzeichnis.rs:2974` | `_o_` | `260816-1359_*_die-probe-gegen-zeitmessung-im-filter-…` |
| `crates/krk-ui/src/appkit/tabelle.rs:1808` | `_c_` | `260802-1036_*_spec-navigator-geruest.md:254` |
| `crates/krk-ui/src/appkit/tabelle.rs:2840` | `_c_` | `260815-2203_*_…` |
| `crates/krk-ui/src/appkit/tabelle.rs:3277` | `_c_` | `260815-2203_*_…` |

**Eine zehnte, und sie ist die Grenze der Sternform selbst.**
`crates/krk-ui/src/appkit/textautomatik.rs:98` schreibt die Sternform und zeigt trotzdem
ins Leere: der **Namensteil** stimmt nicht. Zitiert war
`260810-0416_*_zwei-weitere-textveraendernde-automatiken-ohne-enabled-schalter-bleiben-an.md`,
der Datensatz heißt
`260810-0416_c_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`.
Beide handeln von `inlinePredictionType` und `mathExpressionCompletionType`; der Zeitstempel
ist derselbe, also ist das Ziel eindeutig. Genau diesen Fall nennt der Datensatz vom 260815
in seiner Zeile „Zwei Grenzen der Antwort": die Sternform hält gegen einen Markerwechsel und
gegen nichts sonst.

## Was stehen bleibt

Zwei Zitate in Kurzform, ohne Marker und ohne Namensteil: `issues/260809-2322` in
`crates/krk-ui/src/hervorhebung.rs:136` und `issues/260810-1001` in
`crates/krk-ui/src/appkit/belegungsansicht.rs:778`. Beide lösen über ihren Zeitstempel auf
eine vorhandene Datei auf, tragen also keinen falschen Marker. Sie gehören zum Fehlertyp,
den `shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`
schon führt, und nicht zu diesem Auftrag.

## Prüfung

`make check` — Exit 0, alle vier Kommandos grün.

Der Lauf ist dreimal angesetzt worden. Zweimal fiel die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` an ihrer 15-Sekunden-Grenze aus
(`crates/krk-core/tests/text.rs:870`), allein aufgerufen läuft sie dreimal in Folge grün
(9,5 bis 10,3 s). Das ist der schon aufgenommene Ausfall aus
`shared/issues/260816-0055_*_die-wettrennprobe-ein-wechsel-der-art-unter-dem-oeffnen-faellt-gelegentlich-aus.md`;
dort steht jetzt eine `Also seen`-Zeile mit dieser Messung. Die Änderung dieser Sitzung
kann ihn nicht ausgelöst haben: der Diff umfasst zehn Zeilen, und jede davon ist eine
Dokumentationszeile.

## Geänderte Dateien

- `crates/krk-core/src/verzeichnis/arbeitsbaum.rs`
- `crates/krk-core/src/verzeichnis/loeschzielbefund.rs`
- `crates/krk-core/src/verzeichnis/umfang.rs`
- `crates/krk-core/tests/verzeichnis.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/textautomatik.rs`
- `fusion-workbench/shared/issues/260816-0055_*_die-wettrennprobe-…` (`Also seen`-Zeile)
- `fusion-workbench/circles/260817-0833-…/issues/260818-0710_*_step-16-killed-…` (Abschlusszeile)
