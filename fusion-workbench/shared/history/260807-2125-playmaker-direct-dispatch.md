# Playmaker — Portfoliolauf 260807-2125

**Status:** Complete
**Auslöser:** direct-dispatch
**Domain-Gewichtung:** code (aus der Zeile `**Domain:** code` des Dispatch-Prompts, nicht vom Standardwert)
**Erzeugtes Portfolio:** `fusion-workbench/portfolio.md`

---

## Bestandsaufnahme

| Marker | Bedeutung | Zahl | Verzeichnisse |
|---|---|---|---|
| `_t_` | aktiv | 0 | — |
| `_a_` | vorgesehen | 2 | `260807-2116-eingebauter-editor-mit-textmarken`, `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` |
| `_c_` | kohärent geschlossen | 0 | — |
| `_b_` | beschränkt geschlossen | 1 | `260802-0842-krk-mac-dateimanager-editor-git` |
| `_s_` | überholt | 0 | — |
| `_d_` | zurückgestellt | 0 | — |

`fusion-workbench/.active-circle` fehlt, kein Datensatz trägt `_t_`. Regulärer Zustand nach einem Abschluss, keine der vier Fehlbedingungen trifft zu.

## Rangfolge

**Platz 1: `260807-2116-eingebauter-editor-mit-textmarken`.** Der Nutzer hat den Editor am 260807-1930 in der Übergabe `shared/history/260807-1930-uebergabe-an-die-editor-runde.md` als nächste Runde gewählt und den Circle dafür am 260807-2116 über `/fusion:direct` anlegen lassen.

**Platz 2: `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`.**

**Die Heuristik der Gewichtung `code` hätte die Reihenfolge umgekehrt**, und der Lauf hat sie nicht befolgt. Sie bevorzugt vorgesehene Circles mit wenigen zitierten offenen Entscheidungsdatensätzen; der Editor zitiert vier, der Web-Betrachter einen. Beide tragen dasselbe Abhängigkeitskennzeichen, weil ihr einziger Vorgänger beschränkt und nicht kohärent geschlossen ist. Die Übergabe vom 260807-1930 sagt vom Web-Betrachter ausdrücklich, er sei "nicht der gewählte nächste Schritt". Eine festgehaltene Nutzerwahl beantwortet die Frage, welcher Circle gewollt ist; der Zählwert beantwortet nur, welcher weniger Klärung braucht. Der Unterschied steht im Portfolio und im Aktivierungsvorschlag benannt, statt in der Rangzahl zu verschwinden.

Gezählte offene Entscheidungsdatensätze, alle am 260807-2125 am Dateibestand geprüft:

- Editor: `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` (bindet vor dem ersten Planschritt), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`, `.../260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`, `.../260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md`.
- Web-Betrachter: `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`.

## Geschriebene Abschnitte

- `## Activation proposal` angehängt an `circles/260807-2116-eingebauter-editor-mit-textmarken/_a_circle.md`.

Keine `## Dependency warning` angehängt, kein Zyklus gefunden. Keine `## Parent grounding stale` angehängt; die Begründung steht unten.

## Zyklenprüfung

Der gerichtete Graph über die nicht terminalen Circles trägt zwei Knoten und keine Kante zwischen ihnen. Beide Kanten zeigen auf `260802-0842-krk-mac-dateimanager-editor-git` und damit auf einen terminalen Knoten außerhalb des Graphen. Der Editor-Circle schreibt in `## Dependencies` ausdrücklich, der Web-Betrachter sei keine Abhängigkeit. Kein Zyklus.

## Prüfung auf gealterte Grundlage nach beschränktem Abschluss

Ein Circle trägt `_b_`: `260802-0842-krk-mac-dateimanager-editor-git`, geschlossen am 260807-1035. Beide nicht terminalen Circles zitieren ihn im `## Grounding snapshot`. Kein `parent-grounding-stale`-Ereignis in diesem Lauf, aus zwei getrennten Gründen:

- `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` trägt die Kennzeichnung bereits, geschrieben vom Lauf 260807-1042. Eine zweite Anschrift derselben Feststellung wäre Rauschen.
- `260807-2116-eingebauter-editor-mit-textmarken` ist am 260807-2116 entstanden und damit nach dem Abschluss. Sein Grounding zitiert die Abschlussnotiz wörtlich und trägt einen eigenen Abschnitt `### Was die Ausklammerung der Messreihen kostet`. Seine Grundlage ist gegenüber dem Abschluss aktuell; die Kennzeichnung wäre sachlich falsch.

## Warnungen im Portfolio

- Beide vorgesehenen Circles hängen an einem beschränkt abgeschlossenen Vorgänger (`_b_` statt `_c_`); Kennzeichen gesetzt, keiner blockiert.
- Fünf offene Defekte liegen im terminalen Circle `260802-0842-krk-mac-dateimanager-editor-git` und haben damit keinen Bearbeiter: 260806-1304 (Sitzungslauf bei L6), 260807-0219 (drei Aufrufer von `eintrag_waehlen`), 260807-0930 (Meldung zur Bündelkennung), 260807-1022 (Messstrecken-Defekt im Plan), 260807-1022 (zweiundzwanzig Verweise mit überholtem Marker).
- `CLAUDE.md` Zeile 43 führt den L9-Defekt als offen und zitiert ihn als `_o_`; auf der Platte trägt er `_c_`, geschlossen am 260807-1935 durch Annahme der Einbuße.
- Der Aktivierungsvorschlag im Datensatz des Web-Betrachters vom 260807-1042 nennt ihn den einzigen nicht abgeschlossenen Circle und ist seit dem 260807-2116 überholt.
- `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` zitiert unter `**Cross-references:**` den Pfad `circles/260802-0842-krk-mac-dateimanager-editor-git/_a_circle.md`, der nicht mehr existiert.
- Kein Zeigerproblem, kein Zyklus, keine neue Kennzeichnung wegen gealterter Grundlage.

## Geprüfte Bauteile

Die sechs Bauteile, die der Editor-Circle laut seinem Grounding erbt, wurden am Code nachgesehen und nicht angenommen: `resources/default-keymap.toml:130-137` (Funktion `bearbeiten`, leere Tastenliste, `reserviert_fuer = "editor"`), `crates/krk-ui/src/fenstermodell.rs:48-70` (`Bereich` mit vier Varianten und `ALLE: [Bereich; 4]`), `crates/krk-core/src/ablage/lesezeichen.rs`, `crates/krk-core/src/ablage/pfade.rs`, `crates/krk-ui/src/appkit/aufteilung.rs`, `crates/krk-ui/src/appkit/statuszeile.rs`. Alle sechs liegen vor.
