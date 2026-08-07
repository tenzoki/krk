# Portfolio

**Generated:** 260807-1129 (by playmaker session 260807-1129-playmaker-direct-dispatch)
**Domain bias:** code

## Active (_t_)

(keiner)

Kein Circle-Datensatz trägt die Marke `_t_`, und `fusion-workbench/.active-circle` fehlt. Beides zusammen ist der reguläre Zustand nach einem Abschluss und keine Störung. Die Runde 1 wurde am 260807-1035 mit beschränktem Abschluss geschlossen, und der Zeiger wurde dabei gelöscht.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — der einzige nicht abgeschlossene Circle; seine Voraussetzungen aus der Runde 1 stehen und sind am Code belegt, und nur ein offener Entscheidungsdatensatz bindet ihn.

**Die Rangfolge hat ein Element.** Nach dem Abschluss der Runde 1 ist dieser Circle der einzige vorgesehene im Portfolio. Eine Reihenfolge mit einem Eintrag sagt nichts über relative Reife, und sie behauptet es hier auch nicht. Die Empfehlung steht auf den absoluten Signalen des Circles.

### 1. `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`

**Directive:** KRK zeigt eine Web-Adresse in einem eigenen Betrachter im Vorschaufenster an, statt sie an den Systembrowser abzugeben. Bedient wird er über die Tastatur, mit Sprungmarken auf jedem sichtbaren Link.
**Dependencies:** ein Vorgänger, `260802-0842-krk-mac-dateimanager-editor-git`, beschränkt abgeschlossen (`_b_`).
**Offene Entscheidungen im Grounding:** eine bindende, vom Circle selbst als Schlussfolgerung eingeordnet.
**Offene Fragen im eigenen Datensatz:** drei, Eingabe für die Klärungsrunde bei der Aktivierung.

Die Voraussetzungen aus der Runde 1 sind erfüllt, am Code geprüft und nicht am Marker. Der Datensatz nennt zwei zeitliche Bindungen, die Schritte S13 und S19, und schreibt: "Es kann erst geplant werden, wenn beide stehen." Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_*_plan-navigator-geruest-runde-1.md` trägt `**Status:** Complete` bei 38 von 38 Schritten, und der Abgleich vom 260807-1022 hat sie am Code belegt. Die vier Bauteile, die dieser Circle laut seinem Grounding erbt, liegen auf der Platte und wurden für diesen Lauf einzeln geprüft: die Auswertung der Zwischenablage in `crates/krk-core/src/zwischenablage.rs`, das Vorschaufenster in `crates/krk-ui/src/appkit/vorschau.rs` mit der Tableiste in `crates/krk-ui/src/appkit/tableiste.rs`, die Statuszeile in `crates/krk-ui/src/appkit/statuszeile.rs` und der Befehl `zwischenablage_springen` auf `opt+cmd+g` in `resources/default-keymap.toml`.

Ein einziger offener Entscheidungsdatensatz bindet den Circle. `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` fragt, wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt. Der Circle ordnet diese Bindung selbst als Schlussfolgerung ein und nicht als geprüfte Tatsache. Für die Gewichtung `code`, die Circles mit wenigen unbeantworteten Fragen bevorzugt, ist ein einzelner Datensatz ein guter Wert. Die drei projektweit offenen Fragen zu Git, Editor-Formatansicht und Code-SDK berühren diesen Circle nicht, und von den vier weiteren offenen Fragen im Circle-Speicher der Runde 1 bindet ihn keine.

Gegen eine sofortige Aktivierung steht die Art des Abschlusses, an dem der Circle hängt. Seine einzige Abhängigkeit ist beschränkt geschlossen und nicht kohärent, und die Beschränkung reicht über die dritte offene Frage des Circles in seine Grundlage hinein. Der Abschnitt `## Warnings` unten führt den Befund aus.

## Recently closed (_c_ / _b_)

### `260802-0842-krk-mac-dateimanager-editor-git` — beschränkter Abschluss (`_b_`), 260807-1035

Die Runde 1 hat das Navigator-Gerüst gebaut, und alle 38 Planschritte sind am Code belegt. Beschränkt ist der Beleg der Zeitzusagen: sieben der zehn stehen auf der Abnahmereihe `messungen/260805-2207-MacBookPro15-1-abnahme.txt` vom 260805-2207, und drei spätere Commits haben Wege berührt, die genau diese Zusagen messen. Der Artefakt der Beschränkung ist die Einsicht, dass eine Messreihe an jedem Commit altert, der einen gemessenen Pfad berührt, ohne es selbst zu sagen. Die Nachholarbeit ist ein Abnahmelauf am gebauten Bündel, der KRK im Vordergrund und damit den Nutzer verlangt.

## Archived (_s_ / _d_)

(keine)

Kein Circle-Datensatz trägt `_s_` oder `_d_`.

## Warnings

- `parent-grounding-stale: parent=260804-0933-eingebauter-web-betrachter-im-vorschaufenster child=260802-0842-krk-mac-dateimanager-editor-git` — die dritte offene Frage des vorgesehenen Circles, "Bekommt der Betrachter eine eigene Zeitzusage?", leitet eine mögliche elfte Zusage aus den zehn bestehenden ab. Zwei davon sind für einen Betrachter im Vorschaufenster die naheliegenden Bezugsgrößen, und beide gehören zum ungemessenen Teil: L5, der Tabwechsel mit 50 ms, und L7, die Vorschau mit 100 ms. Die alternden Commits treffen diese Wege nachweislich, `9a47c4a` vom 260807-0933 über `crates/krk-ui/src/kommandos/fokus.rs` und `crates/krk-ui/src/fenstermodell.rs`, `5d7e299` vom 260807-0819 über `crates/krk-ui/src/tabs.rs`. Der Abschnitt `## Parent grounding stale` steht seit dem Lauf 260807-1042 im Datensatz des vorgesehenen Circles.

- `parent-grounding-stale: der Artefakt der Beschränkung ist an den vorgesehenen Circle adressiert` — die `## Closure note` der Runde 1 schließt mit dem Satz: "Eine spätere Runde, die Zeitzusagen führt, braucht dafür eine Regel statt einer Nachfrage." Ob dieser Circle eine Zeitzusage führt, entscheidet seine eigene dritte Frage. Fällt die Antwort auf ja, ist der Artefakt eine bindende Eingabe für den Aktivierungs-Spec.

- `dependency-not-coherent: 260804-0933-eingebauter-web-betrachter-im-vorschaufenster → 260802-0842-krk-mac-dateimanager-editor-git` — die einzige Abhängigkeit des vorgesehenen Circles ist beschränkt abgeschlossen (`_b_`) und nicht kohärent (`_c_`). Nach der Rangheuristik zählt allein `_c_` als erfüllte Vorbedingung. Inhaltlich trägt das Kennzeichen hier, siehe die beiden Befunde oben.

- `stale-path-citations: 260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — vier Zitate im Abschnitt `## Dependencies` zeigen auf Pfade oder Marker, die es so nicht mehr gibt: Zeile 100 auf `planning/260802-1036_o_spec-navigator-geruest.md` (ist `_c_`), Zeile 101 auf `decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md` (ist `_i_`), Zeile 102 auf `260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md` (ist `_b_circle.md`), Zeile 106 auf `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (ist `_c_`). Der Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md` führt allein Zeile 101. Die drei übrigen sind ihm entgangen, weil sein Abgleich um 260807-1022 lief und damit vor der Umbenennung von 260807-1035, und weil sein Suchmuster `YYMMDD-HHMM_x_name.md` die Form `_t_circle.md` ohnehin nicht erfasst. Der Playmaker berichtigt keine Zitate; ob der Defekt erweitert wird, entscheidet der Nutzer.

- `stale-path-citations: CLAUDE.md` — **neu in diesem Lauf.** `CLAUDE.md:9` verweist auf `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`. Diesen Pfad gibt es seit dem 260807-1035 nicht mehr; der Datensatz heißt `_b_circle.md`. `CLAUDE.md:41` beschreibt die Runde 1 im selben Zug als laufend ("Für den Rundenabschluss fehlt allein ein frischer Abnahmelauf am gebauten Bündel"), und die Runde ist genau deswegen beschränkt abgeschlossen. Beide Stellen zusammen stellen einen abgeschlossenen Circle als aktiv dar, und `CLAUDE.md` ist die Datei, die jede Sitzung als erstes liest. Der Defekt zu den überholten Zustandsmarkern deckt die erste Stelle nicht ab, weil sein Suchmuster die Form `_t_circle.md` nicht erfasst; er führt aus `CLAUDE.md` allein Zeile 17.

- `issue-count-overstated: 260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker` — der Defekt zählt 22 Stellen, davon sechs in `portfolio.md`. Diese sechs sind seit der Neuerzeugung vom 260807-1042 erledigt, und dieser Lauf hält sie erledigt; der Defekt sagt das selbst voraus. Offen sind noch 16 Stellen, davon zehn im Datensatz der Runde 1. Der Playmaker ändert keine Defektdatensätze, deshalb steht die Zahl im Datensatz weiter auf 22.

**Keine Abhängigkeitszyklen.** Geprüft wurde der gerichtete Graph über die nicht-terminalen Circles, und das ist nach dem Abschluss der Runde 1 genau einer. Sein einziger ausgehender Verweis zeigt auf einen terminalen Circle, der als Knoten nicht mitzählt. Ein Zyklus braucht mindestens zwei Knoten mit gegenläufigen Kanten; hier gibt es einen Knoten und keine Kante innerhalb des Graphen.

**Keine Zeigerfehler.** `fusion-workbench/.active-circle` fehlt, und kein Datensatz trägt `_t_`. Diese Kombination ist der reguläre Zustand nach einem Abschluss und löst keine Meldung aus.

---

## Details

| Marke | Bedeutung | Anzahl |
|---|---|---|
| `_a_` | vorgesehen | 1 |
| `_t_` | aktiv | 0 |
| `_c_` | geschlossen-kohärent | 0 |
| `_b_` | beschränkt abgeschlossen | 1 |
| `_s_` | überholt | 0 |
| `_d_` | zurückgestellt | 0 |

Offene Entscheidungen: 8 insgesamt, davon 5 im Circle-Speicher der Runde 1 und 3 im geteilten Speicher. Offene Defekte: 5, alle im Circle-Speicher der Runde 1, dazu einer zurückgestellt (`_d_`). Pläne: 2, beide geschlossen (`_c_`); der geteilte Planungsspeicher ist leer. `tasklist.md` existiert nicht.

Dieser Lauf hat keinen Abschnitt in einen Circle-Datensatz geschrieben. Der Zustand des Portfolios ist seit dem Lauf 260807-1042 unverändert, und die Abschnitte `## Parent grounding stale` und `## Activation proposal` in `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md` stammen von jenem Lauf und gelten weiter. Ein zweiter, inhaltsgleicher Anbau hätte den Datensatz nur verdoppelt. Sitzungshistorie dieses Laufs: `shared/history/260807-1129-playmaker-direct-dispatch.md`.
