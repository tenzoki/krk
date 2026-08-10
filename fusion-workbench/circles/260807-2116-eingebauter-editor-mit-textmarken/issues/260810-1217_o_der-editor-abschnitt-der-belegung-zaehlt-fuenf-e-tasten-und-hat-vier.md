Der Editor-Abschnitt der Belegung zählt fünf e-Tasten und hat vier

---

`resources/default-keymap.toml:479` behauptet über die elf Editor-Funktionen:

```
# Fuenf von ihnen teilen sich den Buchstaben e und unterscheiden sich in der
# Zusatztaste. Das ist die Systematik, die diese Belegung schon bei l und d
# faehrt: die blanke Cmd-Ebene traegt die Handlung selbst, shift+cmd den
# Fokusbefehl, opt+cmd das Ein- und Ausblenden, ctrl+cmd die Zweitform.
```

Es sind **vier**, nicht fünf. Der Bestand der Datei, geprüft am 260810-1217:

| Zeile | Kombination | Funktion | Rolle in der Systematik |
|-------|-------------|----------|-------------------------|
| 504 | `cmd+e` | `editor_aus_vorschau` | die Handlung selbst |
| 513 | `shift+cmd+e` | `fokus_editor` | der Fokusbefehl |
| 521 | `opt+cmd+e` | `editor_schliessen` | das Ein- und Ausblenden |
| 530 | `ctrl+cmd+e` | `editor_ansicht_umschalten` | die Zweitform |

Eine fünfte Kombination auf `e` gibt es in der ganzen Datei nicht:

```sh
grep -n 'tasten = .*+e"\|tasten = .*"e"' resources/default-keymap.toml
# 504, 513, 521, 530 — vier Treffer
```

Die verbleibenden sieben der elf tragen `s`, `j`, `f`, `g` (zweimal) und `r` (zweimal). Der Einstieg über F4 (`bearbeiten`, Zeile 133) steht in der Norton-Reihe und trägt kein `e`; er kann die fünfte nicht sein.

## Die Zahl war nie richtig

`git log -L 479,482:resources/default-keymap.toml` liefert genau einen Treffer: `e1acc68` hat den Absatz angelegt. Schon in diesem Stand tragen exakt vier Kombinationen den Buchstaben `e` (`git show e1acc68:resources/default-keymap.toml`, Zeilen 496, 505, 513, 522). Es ist also kein nachträglich veralteter Zählstand, sondern ein Zählfehler von Anfang an.

## Warum das mehr ist als eine Zahl

Der Absatz nennt vier Ebenen der Systematik und zählt fünf Träger. Wer die Belegung erweitert, liest daraus, es gäbe eine fünfte Ebene, die er nicht findet, oder eine fünfte Kombination, die er einfügen dürfte. Die vier Ebenen sind vollständig: blank, `shift+cmd`, `opt+cmd`, `ctrl+cmd`. Eine fünfte müsste eine Zusatztastenkombination erfinden, die diese Datei nirgends führt.

## Vorgeschlagene Behebung

`Fuenf` durch `Vier` ersetzen. Die drei Folgesätze stimmen unverändert; die Zuordnung der vier Ebenen auf die vier Funktionen geht in der Tabelle oben auf.

Der Vergleich mit `l` und `d` bleibt tragfähig: `cmd+d` trägt `lesezeichen_anlegen` (Handlung), `shift+cmd+l` und `shift+cmd+d` die Fokusbefehle, `opt+cmd+l` und `opt+cmd+d` das Ein- und Ausblenden. Die Zweitform auf `ctrl+cmd` gibt es dort nicht, und der Satz behauptet sie dort auch nicht.

## Zuständigkeit

`ontocoder`. Es ist eine Kommentarzeile in einer TOML-Datei, kein Programmteil. Keine Belegungszeile ist betroffen, und der Bau bleibt unberührt.

---

**Gefunden von:** ontorev, Durchsicht der Belegungsdatei 260810-1217
**Domain:** data
**Schwere:** Medium
**Betroffen:** `resources/default-keymap.toml:479`
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/reviews/260810-1217-ontorev-belegungsdatei-nach-den-drei-kommentarstellen.md` (der Bericht, aus dem dieser Defekt stammt)
