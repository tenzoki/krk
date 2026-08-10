Spec und Plan der Runde 2 tragen sechs Verweise mit ausgeschriebenem Zustandsmarker

---

Der Defekt über die veralteten Zustandsmarker
(`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md`)
stammt vom 260807-1022 und damit von vor der Editor-Runde. Seine beiden Hälften decken die
Circle-Datensätze samt `portfolio.md` und daneben `CLAUDE.md` und `spikes/` ab. Spec und Plan
der **Runde 2** kommen in keiner der beiden vor, und sie tragen sechs Stellen mit
ausgeschriebenem Marker. Plan und Spec der Runde 1 sind seit dem 260805-0000 sauber.

---

**Schwere:** Niedrig
**Gefunden:** ontocoder, bei der Abarbeitung der ersten Hälfte des Defekts vom 260807-1022
**Domain:** data

## Die sechs Stellen

Erhoben am 260810-1730, in
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/`:

| Datei | Zeilen |
|---|---|
| `260807-2147_*_spec-eingebauter-editor-mit-textmarken.md` | 6, 667 |
| `260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` | 21, 162, 1615, 1667 |

Vier davon führt `portfolio.md` bereits unter Warnung 6. Erhebung zum Nachziehen, vom
Projektwurzelverzeichnis aus:

```sh
grep -rnoE '26[0-9]{4}-[0-9]{4}_[aoicdspb]_[a-z0-9-]+\.md' \
  fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/ | sort -u
```

## Warum das ein eigener Datensatz ist

Der Befund fällt zwischen die zwei Hälften des Defekts vom 260807-1022, und beide sind mit
dieser Sitzung abgearbeitet. Ihn an eine davon anzuhängen hieße, einen geschlossenen Datensatz
um einen Bestand zu erweitern, den er nie geführt hat. Es ist derselbe Befund zum fünften Mal,
und die vier Vorgänger sind alle geschlossen, was für sich schon eine Aussage ist: die Ursache
liegt nicht im einzelnen Dokument, sondern darin, dass niemand die Sternform beim Schreiben
erzwingt.

## Zusammenhang

Der Befund an der Erzeugung von `portfolio.md`
(`shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`)
beschreibt dieselbe Ursache an einer anderen Stelle. Wer beide anfasst, prüft zuerst, ob eine
gemeinsame Antwort trägt, statt zweimal von Hand nachzuziehen.

## Festlegung, die hier schon gilt

Aufzeichnungen eines Standes behalten ihren damaligen Marker, und die Ausnahme gilt je Datei
nach ihrem Ort, nicht je Absatz: ausgenommen sind `history/`, `reviews/`, `analyses/`,
`issues/`, `decisions/`, `messungen/` und `spikes/`. Spec und Plan gehören nicht dazu, sie sind
lebende Dokumente. Die Festlegung ist am 260810-1730 getroffen und in
`shared/history/260810-1730-ontocoder-sternform-in-den-circle-datensaetzen.md` begründet.
