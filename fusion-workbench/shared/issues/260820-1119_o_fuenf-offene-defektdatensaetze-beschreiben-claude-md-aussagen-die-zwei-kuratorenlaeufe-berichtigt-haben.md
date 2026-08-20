Fünf offene Defektdatensätze beschreiben CLAUDE.md-Aussagen, die zwei Kuratorenläufe berichtigt haben

---

Fünf Datensätze tragen den Marker `_o_` und beschreiben je eine Aussage in `CLAUDE.md`, die es
dort nicht mehr gibt. Der Kuratorenlauf `260819-1500` hat vier davon mit seinen Einträgen L01,
L03, L07, L09, L10 und L11 berichtigt; der fünfte ist mit denselben Einträgen gefallen. Wer den
offenen Bestand als Arbeitsvorrat liest, bekommt fünf Aufgaben angeboten, die keine mehr sind.

---

**Schwere:** gering für den Baum, mittel für die Verlässlichkeit des offenen Bestands
**Gefunden von:** curator, Erhebung `shared/history/260820-1119-curator-run.md`, Abschnitt 10
**Betroffen:** die fünf unten genannten Dateien
**Domain:** code

## Gemessen, an `2beb1de`

Jede der fünf Behauptungen ist einzeln mit einem Kommando gegen den heutigen Stand von
`CLAUDE.md` gehalten.

| Datensatz | Behauptet | Kommando und Ausgabe |
|---|---|---|
| `shared/issues/260816-2138_o_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md` | die Datei nennt eine Rundenzahl als Zahl | `grep -c 'Wie viele Runden gefahren sind und wie jede geschlossen hat, sagt der Dateibestand' CLAUDE.md` → 1. Die Zahl ist durch `ls fusion-workbench/circles/*/_*_circle.md` ersetzt |
| `shared/issues/260816-1232_o_claude-md-sagt-den-tag-setze-der-nutzer-seit-dem-260813-setzt-ihn-das-werkzeug.md` | die Datei sagt „den Tag setzt der Nutzer" | `grep -c 'den Tag setzt der Nutzer, nicht das Werkzeug' CLAUDE.md` → 0 |
| `shared/issues/260818-0028_o_claude-md-says-the-bundle-ships-as-v0-4-1-and-four-tags-have-been-set-since.md` | die Datei nennt `v0.4.1` | `grep -c 'v0\.4\.1' CLAUDE.md` → 0 |
| `shared/issues/260818-1635_o_claude-md-nennt-zwei-nachzuziehende-stellen-je-kommando-die-dritte-haelt-kein-uebersetzer.md` | die Datei nennt die dritte Stelle nicht | `CLAUDE.md:122` trägt seit `5886d04` den Satz „Der Ausführungszweig hält er nicht" samt beiden Auffangzweigen |
| `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/issues/260816-1935_o_claude-md-nennt-zwei-filterregeln-und-eine-huelle-in-krk-ui-beides-hat-die-elfte-runde-abgeloest.md` | die Datei nennt zwei Filterregeln und eine Hülle in `krk-ui` | `CLAUDE.md:130` nennt drei Regeln; `grep -c 'krk-ui/src/vorschau' CLAUDE.md` → 0 |

## Warum der Kurator sie nicht selbst schließt

Einen Marker auf eine Prüfung gegen den Baum hin weiterzustellen gehört dem `reconciler`
(`agents/curator.md`, `### Explicitly not in your remit`, Ausschluss 1). Der Kurator ändert eine
Aussage auf einer seiner drei Flächen; ein Defektdatensatz ist keine davon.

## Fix

Der `reconciler` liest die fünf Behauptungen gegen `CLAUDE.md`, hängt jedem Datensatz seine Zeile
`Resolved:` an — mit dem Kuratorenlauf und dem Eintrag, der die Aussage berichtigt hat — und
benennt `_o_` nach `_c_` um.

**Eine Nebenfrage, die dabei auffällt und hier nicht mitentschieden wird:** vier der fünf sind in
dem Augenblick still falsch geworden, in dem der Kuratorenlauf seine Änderung anwandte, und kein
Schritt jenes Laufs sucht die Defektdatensätze, die eine Änderung erledigt. Ob ein
Kuratorendurchgang den offenen Defektbestand nach Treffern absucht, ist eine Frage an fusion und
nicht an dieses Projekt.
