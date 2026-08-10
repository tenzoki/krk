Zweiundzwanzig Verweise in lebenden Dokumenten tragen einen überholten Zustandsmarker

---

**Domain:** code
**Filed by:** reconciler (Abgleich 260807-1022, Sitzung 260806-2257)
**Für:** `coder` (CLAUDE.md, spikes/), `ontocoder` (Circle-Datensätze, portfolio.md)
**Cross-references:** `issues/260805-0000_c_zehn-verweise-in-spec-und-plan-tragen-einen-ueberholten-marker.md`,
`issues/260806-1320_c_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`,
`issues/260807-0755_c_ein-zitat-in-der-info-plist-schreibt-den-zustandsmarker-noch-aus.md`

---

Derselbe Befund zum vierten Mal, diesmal über den ganzen Projektbaum erhoben
statt an einer Datei. Ein Verweis, der den Zustandsmarker ausschreibt statt die
Sternform `_*_` zu benutzen, veraltet mit dem nächsten Markerwechsel seines
Ziels. Plan und Spec sind seit dem 260805-0000 sauber; die übrigen lebenden
Dokumente sind es nicht.

**Wie erhoben.** Jeder Verweis der Form `YYMMDD-HHMM_x_name.md` in allen `.md`,
`.rs`, `.toml`, `.plist` und `.txt` des Baums gegen den Ist-Marker der
gleichnamigen Datei im Workbench geprüft, am 260807-1022. Ausgenommen sind
`history/`, `reviews/`, `analyses/`, `issues/` und `decisions/`: das sind
Aufzeichnungen eines Standes und dürfen den Marker von damals tragen.

**Die zweiundzwanzig Stellen.**

| Datei | Zeile | zitiert | ist |
|---|---|---|---|
| `CLAUDE.md` | 17 | `260802-1036_a_leistungszusagen-navigator.md` | `_i_` |
| `fusion-workbench/portfolio.md` | 24 | `260802-0842_o_f-tasten-unter-macos-systembelegung.md` | `_i_` |
| `fusion-workbench/portfolio.md` | 24 | `260802-0842_o_loeschen-papierkorb-oder-endgueltig.md` | `_i_` |
| `fusion-workbench/portfolio.md` | 40 | `260802-0842_o_f-tasten-unter-macos-systembelegung.md` | `_i_` |
| `fusion-workbench/portfolio.md` | 40 | `260802-0842_o_loeschen-papierkorb-oder-endgueltig.md` | `_i_` |
| `fusion-workbench/portfolio.md` | 41 | `260802-0842_o_projektsprache-nicht-deklariert.md` | `_c_` |
| `circles/260802-0842-…/_t_circle.md` | 24 | `260802-1134_a_sprache-und-ui-werkzeugkasten.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 62 | `260802-0842_a_loeschen-papierkorb-oder-endgueltig.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 88 | `260802-0842_a_f-tasten-unter-macos-systembelegung.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 89 | `260802-0842_a_loeschen-papierkorb-oder-endgueltig.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 93 | `260802-1036_a_umbenennen-im-stapel-umfang.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 94 | `260802-1036_a_leistungszusagen-navigator.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 95 | `260802-1134_a_sprache-und-ui-werkzeugkasten.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 96 | `260802-1428_a_was-l4-mit-wiederhergestellten-tabs-meint.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 129 | `260802-0842_o_f-tasten-unter-macos-systembelegung.md` | `_i_` |
| `circles/260802-0842-…/_t_circle.md` | 129 | `260802-0842_o_loeschen-papierkorb-oder-endgueltig.md` | `_i_` |
| `circles/260804-0933-…-web-betrachter…/_a_circle.md` | 101 | `260804-0830_a_was-die-zwischenablage-auswertung-liest.md` | `_i_` |
| `spikes/fn-tasten/README.md` | 25 | `260802-1134_a_sprache-und-ui-werkzeugkasten.md` | `_i_` |
| `spikes/fn-tasten/README.md` | 54 | `260802-1036_a_leistungszusagen-navigator.md` | `_i_` |
| `messungen/260805-2207-…-begleittext.md` | 88 | `260806-0014_o_l9-verfehlt-den-anteil…` | `_i_` |
| `messungen/260805-2207-…-begleittext.md` | 100 | `260806-0014_o_pruefordner-unter-tmp…` | `_c_` |
| `messungen/260803-1641-durchstich.txt` | 301 | `260803-1755_a_l1-verfehlt-die-16-ms-zusage…` | `_i_` |

**Nach Dringlichkeit getrennt.**

Zehn Stellen im Datensatz des **aktiven Circles** wiegen am schwersten: er ist
das Dokument, an dem der Nutzer den Rundenabschluss abliest, und er behauptet
dort für sieben umgesetzte Entscheidungen den Stand "beantwortet" oder "offen".
`CLAUDE.md:17` ist die zweite Stelle mit Gewicht, weil jede Sitzung sie liest.
Die sechs Stellen in `portfolio.md` und die eine im vorgesehenen Circle sind
maschinell erzeugt und werden beim nächsten `playmaker`-Lauf ohnehin neu
geschrieben; sie sind nur dann zu berichtigen, wenn die Erzeugung selbst die
Sternform nicht setzt. Die drei Stellen unter `messungen/` und die zwei unter
`spikes/` sind Aufzeichnungen eines Standes und dürfen so bleiben, wenn der
Fixer das ausdrücklich so entscheidet.

**Denkbarer Weg.** Jede Stelle auf die Sternform ziehen, so wie Plan und Spec
sie seit dem 260805-0000 führen. Kein neuer Mechanismus. Ob das Erzeugen von
`portfolio.md` und der Circle-Datensätze die Sternform künftig selbst setzt,
gehört mitentschieden, sonst kommt der Befund ein fünftes Mal.

**Dringlichkeit.** Gering bis mittel. Kein Abnahmekriterium hängt daran, keine
der zehn Zeitzusagen ist berührt, und kein Verweis zeigt ins Leere — es ist
allein die Zustandsangabe, die falsch ist.

---

## Nachtrag 260807-1045: der Satz „kein Verweis zeigt ins Leere" stimmt nicht mehr

Der Rundenabschluss um 260807-1035 hat drei Marker umbenannt, und damit ist der
Befund über eine bloße Zustandsangabe hinausgewachsen. Drei Zitate im Abschnitt
`## Dependencies` des Circle-Datensatzes
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`
(Zeilen 100, 102 und 106) zeigen jetzt auf Dateien, die es unter dem genannten
Namen nicht mehr gibt: der Plan steht auf `_c_`, der Spec auf `_c_`, und
`_t_circle.md` heißt seit dem Abschluss `_b_circle.md`.

Gefunden hat es der `playmaker` beim Auffrischen des Portfolios. Der Abgleich
oben konnte sie nicht sehen: er lief um 260807-1022 und damit vor der
Umbenennung, und sein Suchmuster `YYMMDD-HHMM_x_name.md` erfasst die Form
`_t_circle.md` ohnehin nicht.

**Zwei Dinge folgen daraus für die Behebung.** Erstens gehört der Circle-Datensatz
in denselben Durchgang wie die übrigen Verweise; die Sternform trägt dort
genauso. Zweitens muss das Suchmuster die Form `_x_circle.md` mit abdecken,
sonst findet der nächste Abgleich dieselbe Stelle wieder nicht.

Der Orchestrator hat die drei Zitate **nicht** berichtigt: ein Circle-Datensatz
gehört dem `playmaker` und dem `shaper`, und der Orchestrator schreibt dort
allein die Abschlussnotiz des eigenen Circles.

---
Resolved:

---
Resolved: Beide Haelften sind abgearbeitet.

**Erste Haelfte (Circle-Datensaetze und portfolio.md).** Der Bestand war erheblich groesser als
dieser Datensatz fuehrt: 76 Stellen statt 16, in allen vier Circles statt in zweien, weil die
Runde 2 seit der Aufnahme durchgelaufen ist. Geaendert sind 55 Stellen in den vier
Circle-Datensaetzen, dazu zwei Prosastellen, die durch die Umstellung falsch geworden waeren.
Stehen bleiben 13 Stellen, an denen der Marker die Aussage selbst ist: die Befundtabelle mit
ihren Spalten "zitiert" und "ist", der Satz ueber das Suchmuster, das die Form `_t_circle.md`
nicht erfasst, und die Umbenennung am Ende jedes Aktivierungsvorschlags. Eine Sternform loeschte
dort den Inhalt.

**`portfolio.md` ist nicht angefasst, und das ist die Antwort auf die Frage, die dieser
Datensatz selbst stellt.** Geprueft wurde, ob die Erzeugung die Sternform setzt: sie tut es
nicht. `agents/playmaker.md` fuehrt an der einzigen Stelle, die ein Pfadzitat des Portfolios
ausformt, selbst einen ausgeschriebenen Marker vor. Eine Handkorrektur waere damit nur ein
Aufschub, und der Befund kaeme ein fuenftes Mal wieder. Der Zusammenhang steht als eigener
Defekt `shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`.

**Zweite Haelfte (CLAUDE.md und spikes/): leer, und mit Begruendung geschlossen statt mit einer
Aenderung.** `CLAUDE.md` traegt null Treffer; die Datei ist seit Aufnahme dieses Datensatzes neu
geschrieben und fuehrt die Sternform bereits. Die drei Stellen in `spikes/fn-tasten/README.md`
(Zeilen 17, 25 und 54) fallen unter die Festlegung, die die erste Haelfte getroffen hat:
Aufzeichnungen eines Standes behalten ihren damaligen Marker, und die Ausnahme gilt je Datei
nach ihrem Ort, nicht je Absatz. Ausgenommen sind `history/`, `reviews/`, `analyses/`,
`issues/`, `decisions/`, `messungen/` und `spikes/`. Die Ortsregel ist entscheidbar, die
Absatzregel waere es nicht.

**Ein Bestand faellt zwischen die beiden Haelften und ist eigens erfasst:** Spec und Plan der
Runde 2 tragen sechs Stellen, die dieser Datensatz nie gefuehrt hat, weil er von vor jener Runde
stammt. Siehe `shared/issues/260810-1746_*_spec-und-plan-der-runde-2-tragen-sechs-verweise-mit-ausgeschriebenem-zustandsmarker.md`.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 1.
