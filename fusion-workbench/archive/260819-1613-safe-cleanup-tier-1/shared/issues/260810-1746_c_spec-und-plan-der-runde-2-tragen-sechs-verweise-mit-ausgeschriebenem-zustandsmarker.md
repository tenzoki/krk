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

---
Resolved: Fuenf der sechs Stellen sind auf die Sternform gezogen: `spec:6`, `spec:667`,
`plan:21`, `plan:162`, `plan:1615`. Der neu erhobene Bestand deckt sich Zeile fuer Zeile mit den
sechs gemeldeten; drei davon waren bereits falsch, weil `_t_circle.md` seit dem Rundenabschluss
`_b_circle.md` heisst.

**Stehengelassen: `plan:1667`.** Der Satz fuehrt den festen Marker als Befund ("39 mit
Sternstelle und einer mit festem Marker"). Eine Sternform machte den Satz selbstwidersprueflich.
Das ist dieselbe Ausnahme, die Turn 1 fuer 13 von 76 Stellen angewandt hat: wo der Marker die
Aussage selbst ist, bleibt er stehen. Bei `spec:667` ist nur der Pfad gezogen und das `_c_`
daneben stehengeblieben, weil die Prosa dort behauptet, der Spec der Runde 1 trage `_c_`.

Zwei Nachtraege im `## Reconciliation Log` des Plans halten fest, was durch die Umstellung
falsch geworden waere, und berichtigen dabei einen Fehler, der schon vorher darin stand: die
genannte Zeile 1013 traegt den Verweis nicht, er stand und steht in Zeile 162.

Abgenommen mit einem Pruefskript ueber beide Dateien, exit 0: 90 Zitate geprueft, 89 mit
Sternstelle, eines mit festem Marker an der einen zugelassenen Stelle, 0 Fehler. Die Dateimarker
der beiden Planungsdateien sind unveraendert; berichtigt ist der Inhalt, nicht der Zustand.

**Ein Rest ist eigens erfasst und nicht behoben:** acht Verweise derselben Art stehen in
Kurzform ohne `.md` und entgehen jedem bisherigen Suchmuster dieses Projekts. Siehe
`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 2.
