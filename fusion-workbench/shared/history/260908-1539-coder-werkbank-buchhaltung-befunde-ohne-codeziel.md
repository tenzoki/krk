# Werkbank-Buchhaltung: die offenen Befunde ohne Codeziel durchgesehen

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baumstand bei Beginn:** `2f4b7b2`

## Was gefragt war

Die offenen Defektdatensätze durchgehen, deren Behebung keine Codedatei unter `crates/`,
`xtask/` oder `resources/` anfasst, sondern die Werkbank selbst: Marker, Abschlussvermerke,
tote Zeiger, Abweichungen zwischen einer Directive und dem, was daraus wurde. Keine Codedatei
anfassen, kein Anforderungsdokument und keinen Plan innerhalb einer geschlossenen Runde
anfassen, nicht committen.

## Die Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues \
  -maxdepth 1 -name '*_o_*.md'
```

124 offene Datensätze. Davon nennen 14 keinen Pfad unter `crates/`, `xtask/` oder `resources/`
(``grep -cE 'crates/|xtask/|resources/'`` je Datei, Treffer null). Diese mechanische Auswahl
ist weder vollständig noch scharf, und beide Abweichungen sind einzeln geprüft:

- **Einer der 14 gehört nicht dazu:**
  `260826-1419_*_die-zwei-mauswege-fragen-die-vorstufe-…` zitiert seine Codestellen ohne
  Kistenvorsatz (`kommandos/mod.rs`, `abwurfregel.rs`) und ist ein Codebefund.
- **Fünf weitere gehören dazu, obwohl sie einen Codepfad nennen** — sie nennen ihn als Beleg
  und nicht als Behebungsziel: `260818-0807_*`, `260818-0752_*`, `260823-1336_*`,
  `260820-2056_*`, `260906-0509_*`.

Bahn also 18 Datensätze.

## Was behoben ist

| Datensatz | Behebung |
|---|---|
| `260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md` | toter Zeiger im Circle-Datensatz des Web-Betrachters berichtigt, Speichervorsatz dabei entfallen; aufgelöst auf genau einen Träger |
| `260819-2206_*_die-commit-nachricht-liegt-in-einem-geteilten-tmp-namensraum-…md` | besteht nicht mehr: fusion trägt die Sitzungskennung heute im Dateinamen |
| `260813-0642_*_zwei-hingenommene-verluste-stehen-auf-keiner-abnahmeliste.md` | Nachsatz an `## Randbedingungen` des Spec der Runde 7 im gemeinsamen Planungsspeicher |
| `260908-1552_*_drei-kopffelder-active-spec-plan-zeigten-nach-zwei-aufraeumlaeufen-ins-leere.md` | neu gefunden und behoben: drei Kopffelder auf die speicherlose Kurzform |

## Was teilweise erledigt ist

- `260906-0212_*_sechzehn-plan-und-specdateien-…`: der Plan der Runde 18 ist auf `_c_` gezogen,
  `**Status:** Complete`, neue Kopfzeile `**Abnahme:** offen`, ursprünglicher Wortlaut als
  Nachsatz. Er ist der einzige der sechzehn außerhalb einer geschlossenen Runde.
- `260906-0509_*_fuenf-datensaetze-der-klasse-s-…`: zwei der fünf Ziele erledigt, drei hängen
  an der Zuständigkeit für den Abschnitt `## Directive`.

## Was offen bleibt

Zehn Datensätze, jeder mit benanntem Grund; die Gründe stehen im Bericht an den Auftraggeber
und, wo sie sich seit der Ablage bewegt haben, als datierter Absatz am Datensatz selbst
(`260819-1440_*`, `260826-1933_*`, `260906-0212_*`, `260906-0509_*`).

## Neue Datensätze

- `260908-1544_*_das-wort-offen-hinter-einem-berichtigten-zitat-…` (offen)
- `260908-1552_*_drei-kopffelder-active-spec-plan-zeigten-…` (geschlossen)
- `260908-1601_*_fast-jedes-kopffeld-eines-circle-datensatzes-nennt-seine-datei-mit-speicherpfad-…` (offen)
- `260908-1608_*_gilt-die-nachsatzregel-vom-260906-auch-unter-dem-abschnitt-directive-…` (Nutzerfrage)

## Verifikation

`cargo test -p xtask` — Rückgabewert 0, 171 Proben grün, darunter
`jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk` und
`kein_offener_defektdatensatz_traegt_eine_leere_abschlusszeile`.

Jeder reparierte Zeiger ist einzeln aufgelöst: fünf Dateinamen aus den drei Kopffeldern und der
eine aus dem Circle-Datensatz des Web-Betrachters, je über
`find fusion-workbench -name '<basename mit _*_>'`, je genau ein Träger.

Nicht committet, wie beauftragt.
