# Archive Manifest

**Date:** 2026-08-20 21:15
**Mode:** tier-1
**Slug:** safe-cleanup-tier-1
**Invoked by:** /fusion:cleanup (Schritt 4), direkt vom Nutzer angestoßen

## Circles archived

keine — alle vierzehn terminalen Circles sind in der Rundentabelle von `CLAUDE.md`
mit ihrem Verzeichnisnamen zitiert und damit durch Sicherheitsfilter 3 ausgenommen.

## Files archived

shared/decisions/260816-1310_i_welche-vorhandene-groessengrenze-gilt-fuer-den-inhaltsfilter.md
shared/decisions/260819-1500_i_gilt-die-artefaktsprache-en-fuer-den-ganzen-bestand-oder-wird-die-deklaration-zurueckgenommen.md
shared/issues/260816-1232_c_claude-md-sagt-den-tag-setze-der-nutzer-seit-dem-260813-setzt-ihn-das-werkzeug.md
shared/issues/260816-2138_c_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md
shared/issues/260817-1610_c_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md
shared/issues/260818-0028_c_claude-md-says-the-bundle-ships-as-v0-4-1-and-four-tags-have-been-set-since.md
shared/issues/260818-1635_c_claude-md-nennt-zwei-nachzuziehende-stellen-je-kommando-die-dritte-haelt-kein-uebersetzer.md
shared/issues/260820-0834_c_das-sitzungsprotokoll-der-runde-14-traegt-weder-directive-noch-turn-log.md
shared/issues/260820-1119_c_fuenf-offene-defektdatensaetze-beschreiben-claude-md-aussagen-die-zwei-kuratorenlaeufe-berichtigt-haben.md

## Guard event log

rolled: .guard-state/events.jsonl -> .guard-state/events-260820-2115.jsonl, 124 Zeilen, 38352 Bytes

## Counts

- 0 Circles, 2 Entscheidungen (`_i_`), 7 Defekte (`_c_`), 0 Pläne, 0 Backlog-Einträge
- 1 gerolltes Wächterprotokoll
- **Total:** 0 Circles, 10 Dateien, 90418 Bytes

## Safety filters applied

- 14 terminale Circles behalten: Verzeichnisname in `CLAUDE.md` zitiert
- 10 terminale Datensätze im gemeinsamen Speicher behalten: in `CLAUDE.md` in Sternform
  (`<Zeitstempel>_*_<Slug>`) zitiert. Die wörtliche Prüfung des Skills findet diese Form nicht;
  das Muster ist um den Zeitstempel erweitert worden, wie es `CLAUDE.md` unter
  „Jedes Suchmuster dieses Projekts, das \`\.md\` verlangt, hat einen blinden Fleck" verlangt
- `_o_`, `_p_`, `_a_`, `_d_` durchweg ausgenommen (lebende Arbeit bzw. Sicherheitsfilter 2)

## Collisions

none
