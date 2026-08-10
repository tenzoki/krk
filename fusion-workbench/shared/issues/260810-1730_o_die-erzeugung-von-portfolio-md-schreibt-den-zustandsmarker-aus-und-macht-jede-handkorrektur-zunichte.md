Die Erzeugung von `portfolio.md` schreibt den Zustandsmarker aus und macht jede Handkorrektur zunichte

---

`fusion-workbench/portfolio.md` wird vom `playmaker` bei jedem Lauf vollständig neu
geschrieben. Fünf Pfadzitate darin nennen ihr Ziel heute unter einem ausgeschriebenen
Zustandsmarker statt in der Sternform `_*_`, zwei davon nachweislich falsch. Eine
Berichtigung von Hand hält bis zum nächsten `playmaker`-Lauf und keinen Tag länger,
denn die Erzeugung selbst kennt die Sternform nicht.

## Was geprüft wurde, und woran

Geprüft am 260810-1730 gegen die installierte Fassung des Plugins unter
`$FUSION_PLUGIN_ROOT`, an den zwei Stellen, die die Erzeugung festlegen.

**Die Vorlage schweigt.** `rules/circle-records.md` führt ab „`$PORTFOLIO` is regenerated
by playmaker on every run. Template:" die fünf Abschnitte des Portfolios samt ihrer
Platzhalter. Kein Platzhalter und kein Satz daneben sagt etwas über die Form, in der ein
Pfad zitiert wird.

**Das erzeugende Verhalten schreibt den Marker aus, und zwar im eigenen Musterbeispiel.**
`agents/playmaker.md` beschreibt in Prozessschritt 3 die Begründung, die der
höchstgereihte Circle bekommt, und führt sie so vor:

```
(e.g. Circle `260511-1100-rebuild-auth` — three dependencies all `_c_`, one open
decision `260510-0930_o_token-format.md` cited)
```

Das ist die einzige Stelle in der Anweisung, die ein Pfadzitat des Portfolios ausformt,
und sie formt es mit ausgeschriebenem Marker. Ein Lauf, der dem Beispiel folgt, erzeugt
genau die Zitate, die dieser Defekt beanstandet.

**Die Regel, die die Sternform verlangt, erreicht den `playmaker` nicht.**
`rules/rule-file-provenance.md` begründet die Sternform ausführlich — „a citation carrying
a literal marker dies at the record's first transition" — und beruft sich auf den
Datensatz `circles/260805-2005-textschicht-gegen-code-nachziehen/decisions/260806-0015_*_zitierform-fuer-workbench-records.md`.
Zwei Dinge halten sie von der Erzeugung fern. Erstens ihr Gegenstand: die Regel bindet
den Herkunftskopf von Regeldateien, nicht die Ausgabe eines Agenten. Zweitens ihre
Zustellung: `fusion-rules playmaker` gibt sieben Pfade aus, und `rule-file-provenance.md`
ist keiner davon.

## Der Bestand in `portfolio.md`

Fünf Zitate mit ausgeschriebenem Marker, erhoben am 260810-1730 mit

```sh
grep -rnoE '26[0-9]{4}-[0-9]{4}_[aoicdspb]_[a-z0-9-]+\.md' fusion-workbench/portfolio.md
```

| Zeile | zitiert | ist |
|---|---|---|
| 87 | `260810-0805_o_ein-verweis-nennt-den-falschen-circle-und-die-zustellerregel-liegt-woanders.md` | `_p_` |
| 149 | `260806-1303_o_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md` | `_o_` |
| 177 | `260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` | `_o_` |
| 179 | `260808-0140_c_plan-eingebauter-editor-mit-textmarken.md` | `_c_` |
| 204 | `260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` | `_p_` |

Die zweite Namensform, `_x_circle.md`, steht in `portfolio.md` dreimal, in den Zeilen 200,
201 und 205. **Diese drei sind kein Befund.** Sie stehen in der Warnung 6, die genau den
Übergang `_t_circle.md` → `_b_circle.md` als Gegenstand hat; dort ist der Marker die
Aussage, und eine Sternform würde sie löschen. Wer die Erzeugung nachzieht, nimmt diesen
Unterschied mit: gestirnt wird ein Zeiger auf eine Datei, nicht die Nennung eines Markers.

Zwei der fünf zeigen schon heute ins Leere, beide auf denselben Übergang `_o_` → `_p_`.
Die drei übrigen sind heute richtig und veralten beim nächsten Markerwechsel ihres Ziels.
Das ist der Punkt: die Zahl der falschen Zitate ist kein Maß für den Defekt, weil sie
allein davon abhängt, wie viel sich seit dem letzten Portfolio-Lauf bewegt hat.

## Warum das hier steht und nicht berichtigt ist

Der Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md`
zieht seit dem 260807 dieselben Zitate in lebenden Dokumenten auf die Sternform und
schreibt dazu selbst: die Stellen in `portfolio.md` „sind nur dann zu berichtigen, wenn
die Erzeugung selbst die Sternform nicht setzt". Sie setzt sie nicht. Deshalb ist
`portfolio.md` bei der Behebung am 260810-1730 ausgelassen worden, und deshalb steht
dieser Datensatz an ihrer Stelle. Es ist derselbe Befund zum fünften Mal; eine fünfte
Handkorrektur wäre die vierte, die nicht hält.

## Wo der Fix liegt, und warum nicht in diesem Projekt

`$FUSION_PLUGIN_ROOT` gehört dem Plugin und ist aus diesem Projekt heraus nur lesbar. Zu
ändern sind Dateien in `rules/` und `agents/` des Plugins, nicht in KRK. Zwei Stellen
reichen aus:

1. **`rules/circle-records.md`**, Portfolio-Vorlage: ein Satz, der die Zitierform
   festlegt — jedes Pfadzitat in `portfolio.md` trägt an der Markerstelle `_*_`, weil das
   Portfolio bei jedem Lauf neu entsteht und seine Zitate zwischen zwei Läufen altern.
2. **`agents/playmaker.md`**, Prozessschritt 3: das Musterbeispiel auf `260510-0930_*_token-format.md`
   ziehen, damit die Anweisung nicht das Gegenteil vorführt.

`speculation:` Ob dieselbe Lücke auch die Circle-Datensätze trifft, ist hier nicht
entschieden. Der `playmaker` schreibt dort nur angehängte Abschnitte, und die sind von der
Handkorrektur am 260810-1730 mit erfasst worden; sie werden nicht neu erzeugt und halten
deshalb. Die Vorlage des Circle-Datensatzes schweigt zur Zitierform aber genauso, und
`shaper` wie `orchestrator` schreiben dort ebenfalls. Wer die zwei Stellen oben anfasst,
sieht sich das mit an.

---

**Gefunden von:** ontocoder, Aufgabe T2 der Sitzung 260810-1647
**Domain:** data
**Schwere:** Low
**Betroffen:** `fusion-workbench/portfolio.md` (Zeilen 87, 149, 177, 179, 204), erzeugt aus `$FUSION_PLUGIN_ROOT/agents/playmaker.md` und `$FUSION_PLUGIN_ROOT/rules/circle-records.md`
**Zuständig:** das Plugin, nicht KRK. Aus diesem Projekt heraus nicht behebbar.
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_*_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` (der Defekt, der diese Prüfung verlangt hat), `shared/history/260810-1730-ontocoder-sternform-in-den-circle-datensaetzen.md`

Warum im gemeinsamen Speicher und nicht im aktiven Circle: der Defekt sitzt im Werkzeug,
nicht im Gegenstand einer Runde, und `portfolio.md` gehört keinem Circle.

---
Resolved:
