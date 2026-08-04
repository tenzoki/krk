# S11c: Ordnernavigation und Belegungsansicht in der Auslieferungsbelegung belegt

**Status:** Complete
**Agent:** ontocoder
**Datum:** 260804-1214
**Schritt:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 11c.`
**Geänderte Datei:** `resources/default-keymap.toml` (einzige Datendatei des Schrittes)

---

## Was geändert wurde

### Fünf Tastenlisten

| `id` | vorher | nachher | Herkunft |
|---|---|---|---|
| `ordner_aufwaerts` | `["cmd+up"]` | `["cmd+left", "cmd+up"]` | Nutzerentscheid 260804, Plan S11c |
| `oeffnen` | `["return"]` | `["cmd+right"]` | Nutzerentscheid 260804, Plan S11c |
| `belegung_ansehen` | (neu) | `["f1"]` | Nutzerentscheid 260804, C3, Plan S11c |
| `bereich_verbreitern` | `["ctrl+b"]` | `["ctrl+right"]` | `decisions/260804-1122_a_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` |
| `bereich_verschmaelern` | `["ctrl+s"]` | `["ctrl+left"]` | dieselbe Entscheidung |

Die Datei wächst von 49 auf 50 Funktionen und von 55 auf 57 Kombinationen, wie der Plan es ansagt. `return`, `ctrl+b` und `ctrl+s` stehen in keiner Tastenliste mehr.

Der neue Block `belegung_ansehen` steht am Dateiende unter einer eigenen Überschrift `── C3: die Belegungsansicht ──`. Er gehört in keinen der vorhandenen Abschnitte: die Norton-Reihe oben ist C3, führt aber die sechs Funktionstasten-Kürzel, und die Belegungsansicht ist keine davon.

### Sechs Stellen im Kommentartext

Der Auftrag nennt fünf und der Plan vier; tatsächlich angefasst sind sechs, weil die zwei mitgenommenen Defekte je eine eigene Stelle betreffen und der Plan die Belegungsstelle bei `bereich_verschmaelern` schon als seine vierte führt.

1. **Zeile 8 bis 9, Herkunftsangabe.** "Faehigkeiten C1 bis C7" → "C1 bis C7 sowie C10". Behebt `issues/260804-0907_c_kopfkommentar-der-auslieferungsbelegung-nennt-c10-nicht.md`.
2. **Zeile 29 bis 30, Namensliste.** "f3 bis f8, delete, up, down, …" → "f1 bis f12, delete, up, down, left, right, …". Zieht den Kopf auf die Tastentabelle nach, die S11b auf 61 Einträge erweitert hat (`crates/krk-core/src/tasten/parser.rs:152`). Plan-Stelle 1.
3. **Zeile 32 bis 35, fn-Annahme.** Der Klammerzusatz trennt jetzt Messung und Ableitung. Behebt `issues/260803-2317_c_der-kopf-der-belegungsdatei-nennt-eine-annahme-als-gemessen.md`. Die zitierten Ereignisse sind nachgesehen, nicht übernommen: `spikes/fn-tasten/messung-A.txt:17-19` zeigt #03 code=99 (F3), #04 code=96 (F5), #05 code=100 (F8), alle mit `mod=function`.
4. **Zeile 37 bis 39, ab Werk freie Kombinationen.** Drei werden vier; die Eingabetaste kommt dazu, mit dem Grund. Plan-Stelle 2.
5. **Zeile 160, Kommentar an `ordner_aufwaerts`.** Nannte allein den Finder als Vorbild. Nennt jetzt beide Wege mit je eigenem Vorbild. Plan-Stelle 3.
6. **Zeile 322 bis 324, Kommentar an `bereich_verschmaelern`.** Entfernt, nicht umformuliert. Er erklärte die Behelfsbelegung `ctrl+b`/`ctrl+s` mit einer Lücke der Kombinationsschreibweise; die Lücke ist mit S11b weg und die Behelfsbelegung mit dem Nutzerentscheid vom 260804. Plan-Stelle 4, überholt durch die Entscheidung.

Der Kommentar an `bereich_verschmaelern` bekommt keinen Ersatz. Die Herkunft der neuen Belegung steht im Entscheidungsdatensatz und in dieser Historiendatei; eine Zeile in der Datendatei hätte der Auftrag ausdrücklich nicht gewollt.

**Warum `ctrl+b` und `ctrl+s` nicht in die Liste der ab Werk freien Kombinationen wandern.** Jene Liste führt Kombinationen, die ein Leser belegt erwartete und die ausdrücklich frei bleiben: Umschalt+Entf nach dem Löschentscheid, Cmd+C und Cmd+V für eine spätere Zwischenablage, seit heute die Eingabetaste gegen die Erwartung aus jedem Dateimanager. `ctrl+b` und `ctrl+s` waren ein Behelf ohne Vorbild; sie sind nicht freigehalten, sondern nicht mehr vergeben.

---

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| gültiges TOML | ja, über `Belegung::auslieferung()`; siehe unten |
| `grep -c '^\[\[funktion\]\]'` | **50** |
| `grep -F '"return"'` | nichts gefunden |
| `grep -F '"ctrl+b"'` / `'"ctrl+s"'` | nichts gefunden |
| Kombinationen gesamt | **57**, keine doppelt (exakter Vergleich am vollständigen Eintrag) |
| `cargo test -p krk-core --test belegung` | **FEHLGESCHLAGEN**, 25 von 26; Ursache und Behebung unten |

**Zum TOML.** Es gibt in dieser Umgebung kein `tomllib` (Python ist 3.9.6) und kein `yq`. Die Gültigkeit ist stattdessen über den eigentlichen Verbraucher belegt: `include_str!` kompiliert die Datei in `krk-core` ein, und 25 der 26 Prüfungen in `tests/belegung.rs` bauen `Belegung::auslieferung()`, darunter `die_auslieferungsbelegung_ist_konfliktfrei` und `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste`. Ein Syntaxfehler oder eine unbekannte Taste hätte alle 26 zu Fall gebracht.

**Zur Doppelprüfung.** Verglichen wurde der vollständige Eintrag, nicht die Teilzeichenkette. Der Auftrag warnt zu Recht davor: die Gegenprobe listet 22 Paare, bei denen ein Eintrag Teilzeichenkette eines anderen ist, und die heutige Änderung legt ein neues dazu, `cmd+r` in `cmd+right`. Eine Teilzeichenkettenprüfung hätte die Sortierrichtung gegen den Ordnereinstieg als Konflikt gemeldet.

**Warum die Prüfung fehlschlägt.** `crates/krk-core/tests/belegung.rs:347` bindet `return` noch an `Kommando::Oeffnen`. Die Zeile ist seit der Umbelegung falsch; die Prüfung bricht mit `return trifft keine Funktion` ab. Es ist eine Codeänderung, und `crates/` liegt außerhalb dieses Auftrags: gemeldet als `issues/260804-1214_o_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md` für den `coder`. Die Fehlerursache liegt in der Prüfung und nicht in den Daten: die inhaltlichen Prüfungen über die Auslieferungsbelegung laufen alle durch, und der Rest des Pakets ist grün (26 von 26 im Bibliotheksteil, 19 von 20 mit 1 ignoriert in `ablage`).

---

## Drei neue Defekte

| Datei | Für wen | Worum es geht |
|---|---|---|
| `issues/260804-1214_o_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md` | `coder` | Blockiert das Abnahmekriterium. Eine Zeile: `("return", …)` → `("cmd+right", …)`. |
| `issues/260804-1214_o_das-abnahmekriterium-von-s11c-verlangt-einen-aufstieg-den-es-nicht-gibt.md` | `planner` | Der letzte Satz des Abnahmekriteriums ist so nicht erfüllbar; Begründung unten. |
| `issues/260804-1214_o_die-pruefung-der-ab-werk-freien-kombinationen-kennt-die-vierte-nicht.md` | `coder` | `die_drei_ab_werk_freien_kombinationen_kommen_nicht_vor` deckt die Eingabetaste nicht ab. |

### Zum unerfüllbaren Abnahmekriterium

Der Satz verlangt, dass im gebauten Bündel "`cmd+left` und `cmd+up` aufsteigen". Das Kommando dafür gibt es nicht: `Kommando` in `crates/krk-core/src/tasten/belegung.rs:80-114` führt 16 Werte und keinen Aufstieg, `Kommando::KENNUNGEN` (Zeile 119-139) kennt die Kennung `ordner_aufwaerts` nicht. S13 baut ihn. Nach S11c sind beide Kombinationen belegt und folgenlos, nach derselben Regel, die der Plan bei `f1` selbst ansagt.

Die andere Hälfte des Satzes stimmt: `Kommando::Oeffnen` steht in der Aufzählung und hängt in `crates/krk-ui/src/appkit/tabelle.rs:460` an `auswahl_oeffnen`, also wirkt `cmd+right`. Gegengeprüft wie beauftragt: `Kommando::BereichVerbreitern` und `Kommando::BereichVerschmaelern` stehen ebenfalls in der Aufzählung und hängen in `crates/krk-ui/src/appkit/anwendung.rs:403-404` an `breite_aendern`; `ctrl+right` und `ctrl+left` wirken ab sofort.

Die Wirksamkeit im **gebauten Bündel** ist damit aus der Verdrahtung abgeleitet und nicht am laufenden KRK nachgesehen. Was maschinell geprüft ist, ist die Belegungsseite.

---

## Offen nach diesem Schritt

- **Der Entscheidungsdatensatz `decisions/260804-1122_a_wandern-die-bereichsbreiten-auf-die-links-und-rechts-pfeile.md` steht weiter auf `_a_`.** Sein Rumpf sagt zu, die Umsetzung ziehe ihn auf `_i_`. Der Auftrag begrenzt den Eingriff auf die Belegungsdatei und zwei Defektdateien und untersagt das Committen; ohne Commit-Hash lässt sich die Zeile `Implemented:` nicht belegen. Der Übergang gehört damit hinter den Commit dieses Schrittes.
- **Der `[DONE]`-Vermerk an S11c** wird laut Auftrag vom Auftraggeber gesetzt, nicht hier.
- **Der Commit** steht aus, wie beauftragt.
