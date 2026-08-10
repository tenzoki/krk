# Spec und Plan der Runde 2 tragen die Sternform, acht gekürzte Verweise sind gemeldet (T2)

**Agent:** ontocoder
**Status:** Complete
**Quellen:**
- `shared/issues/260810-1746_*_spec-und-plan-der-runde-2-tragen-sechs-verweise-mit-ausgeschriebenem-zustandsmarker.md`
- `shared/history/260810-1730-ontocoder-sternform-in-den-circle-datensaetzen.md`, Abschnitt `## Die Festlegung zu Aufzeichnungen eines Standes`

**Zum Stilprofil:** `fusion-rules ontocoder` gab wie in Turn 1 allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`. Für diesen Bericht gilt deshalb kein Langform-Schreibprofil; das Fehlen ist hier vermerkt, wie `rules/agent-setup.md` es verlangt.

---

## Der neu erhobene Bestand

Beide Suchen des Auftrags, vom Projektwurzelverzeichnis aus, dazu eine dritte, breitere zur Gegenprobe. **Der Bestand deckt sich mit den sechs gemeldeten Stellen**, Datei für Datei und Zeile für Zeile:

| Datei | Zeile | Zeichenkette |
|---|---|---|
| `260807-2147_*_spec-…` | 6 | `_t_circle.md` |
| `260807-2147_*_spec-…` | 667 | `260802-1036_c_spec-navigator-geruest.md` |
| `260808-0140_*_plan-…` | 21 | `_t_circle.md` |
| `260808-0140_*_plan-…` | 162 | `260810-0822_i_wie-die-formatansicht-…` |
| `260808-0140_*_plan-…` | 1615 | `_t_circle.md` |
| `260808-0140_*_plan-…` | 1667 | `260810-0822_i_wie-die-formatansicht-…` |

Drei der sechs waren zum Zeitpunkt der Erhebung **schon falsch**: der Circle-Datensatz heißt seit dem Rundenabschluss `_b_circle.md` und nicht `_t_circle.md`. Die anderen drei trafen. Der Unterschied ändert an der Behebung nichts, wie schon in Turn 1: ein zufällig richtiger ausgeschriebener Marker ist derselbe Defekt einen Tag vor seinem Eintreten.

## Was geändert ist

**Fünf der sechs Stellen tragen die Sternform**, zwei im Spec und drei im Plan. Dazu zwei Nachträge im Plan, die ohne die Umstellung falsch stehengeblieben wären.

- `planning/260807-2147_*_spec-…:6` — die Zeile `**Quelle:**` verweist auf den Circle-Datensatz. Der Verweis zeigt auf die Datei, in der der Wortlaut der Directive steht, und trifft seit dem Rundenabschluss nicht mehr.
- `planning/260807-2147_*_spec-…:667` — der Pfad auf den Spec der Runde 1. Der Satz behauptet daneben in Prosa, jener Spec trage `_c_`; diese Behauptung steht im Text und nicht im Pfad und bleibt unangetastet. Der Pfad ist ein Zeiger und geht auf die Sternform.
- `planning/260808-0140_*_plan-…:21` — im Abschnitt `## Directive`, wieder ein Zeiger auf den Circle-Datensatz. Der Plan nimmt seinen Kopf (Zeilen 3 bis 7) von der eigenen Sternstellen-Regel aus; Zeile 21 liegt hinter dem Kopf, die Ausnahme greift dort nicht.
- `planning/260808-0140_*_plan-…:162` — der Zeiger auf den Entscheidungsdatensatz zur Formatansicht. Drei weitere Zeiger auf dieselbe Datei (Zeilen 1572, 1576, 1645) trugen die Sternform bereits.
- `planning/260808-0140_*_plan-…:1615` — der Zeiger `_t_circle.md:7` im Abgleich vom 260810-0805.

**Zwei Prosastellen nachgezogen**, beide im `## Reconciliation Log` des Plans:

- Bei Zeile 1615 sagte der Absatz, die Zeile `**Active spec/plan:**` im Circle-Datensatz nenne beide Dateien mit festem Marker und liefe bei einer Umbenennung ins Leere. Beides ist überholt: der Plan trägt seit `511e362` (260810-0837) den Marker `_c_`, und jene Zeile ist am 260810-1730 selbst auf die Sternform gezogen worden. Der Nachtrag hält das fest, damit kein Leser eine erledigte Umbenennung nachholt.
- Bei Zeile 1667 zählte der Abgleich vom 260810-1404 „39 mit Sternstelle und einer mit festem Marker". Der eine ist jetzt umgestellt, die Zählung stimmte nach der Umstellung nicht mehr. Der Nachtrag nennt den neuen Stand und dazu einen Fehler, der schon damals drinstand: die genannte Zeile 1013 trägt den Verweis nicht, er stand und steht in Zeile 162.

## Was ausdrücklich stehenbleibt, und warum

**Eine der sechs Stellen behält ihren ausgeschriebenen Marker**, `planning/260808-0140_*_plan-…:1667`. Der Satz führt den festen Marker als **Befund**: „40 Verweise … geprüft, davon 39 mit Sternstelle und einer mit festem Marker (`decisions/260810-0822_i_…`)". Eine Sternform in dieser Klammer löschte die Aussage, statt sie haltbar zu machen — sie machte den Satz sogar selbstwidersprüchlich. Das ist derselbe Grund, aus dem in Turn 1 dreizehn von 76 Stellen stehengeblieben sind.

Nicht angefasst und außerhalb des Bestands, weil Prosa über einen Stand und kein Zeiger auf eine Datei: die Aussagen über den Marker `_a_` der Entscheidungsdatensätze in `spec:10`, `spec:583` und `plan:6`, die Aufzählung der Markerwerte in `spec:663` und `plan:1669`, und die Sätze „gehört diese Datei auf `_c_`" und „Der Defekt trägt den Marker `_c_`" an mehreren Stellen des Plans. Der Spec führt zwei davon selbst unter `**Zwei Stellen dieses Specs behaupten einen Marker, den der Bestand nicht mehr trägt**` und weist sie dem Nutzer zu.

## Ein Bestand daneben, den beide Suchen des Auftrags nicht erfassen

**Acht Verweise in derselben Form stehen in der Kurzform mit Auslassungspunkten** und fallen deshalb durch beide vorgegebenen Suchen, weil ihnen das `.md` am Ende fehlt:

```
planning/260807-2147_*_spec-…:556   260807-0010_o_kann-der-auffrischungsaufschub-entfallen...
planning/260807-2147_*_spec-…:556   260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben...
planning/260808-0140_*_plan-…:492   issues/260809-2148_c_...
planning/260808-0140_*_plan-…:690   issues/260808-0931_c_...
planning/260808-0140_*_plan-…:701   issues/260808-0931_c_...
planning/260808-0140_*_plan-…:716   issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...
planning/260808-0140_*_plan-…:853   issues/260809-2148_c_...
planning/260808-0140_*_plan-…:884   issues/260809-2148_c_...
```

Erhoben mit `grep -rnoE '[A-Za-z0-9./-]+_[aoicdspbt]_[A-Za-z0-9.-]*\.\.\.'`.

**Einer von ihnen ist bereits falsch:** Zeile 716 zitiert `260808-1413_o_vier-platzhalter-…`, die Datei trägt `_c_`. Der Plan führt sechs der acht selbst als offenen Befund (`#### Weitere Befunde`, „Sieben Verweise im Plan tragen weiter einen festen Marker", Zeilen 478, 670, 681, 696, 833, 864, 1330); die dort genannten Zeilennummern sind inzwischen um 14 bis 20 verschoben, und die siebte, Zeile 1330, ist am 260810-1404 behoben worden — sie steht jetzt in Zeile 1352 mit vollem Namen und Sternform und ist damit das Vorbild für die übrigen sechs. Die zwei im Spec führt niemand.

**Nicht behoben, sondern gemeldet.** Der Auftrag bestimmt den Bestand über zwei Suchen, und beide liefern die sechs. Die Kurzform ist eine andere Gestalt, ihre Behebung verlangt das Ausschreiben des vollen Namens und falsifiziert zwei weitere Absätze im `## Reconciliation Log`. Das gehört in eine eigene Aufgabe mit eigenem Auftrag, nicht in diese.

## Prüfung

Ein Skript über beide Planungsdateien, das jeden Verweis auf eine Datensatzdatei einsammelt, ihn workbench- und circle-relativ auflöst und den festen Marker nur an der einen zugelassenen Stelle durchgehen lässt. **Rückgabewert 0:**

```
90 Zitate geprueft, 89 mit Sternstelle, 1 mit festem Marker (zugelassen: 1), 0 Fehler.
```

- **Kein unbeabsichtigt ausgeschriebener Marker mehr.** Eine zweite Stelle hätte den Lauf scheitern lassen.
- **Alle 90 Zitate lösen auf**, jedes auf genau eine Datei. Keines zeigt ins Leere, keines ist mehrdeutig.
- Die Nacherhebung mit beiden Suchen des Auftrags liefert für die Circle-Form **null** Treffer und für die Datumsform genau die eine zugelassene Stelle.
- Der Zeiger `_*_circle.md:7` trifft weiter: Zeile 7 von `_b_circle.md` ist die Zeile `**Active spec/plan:**`.

`git diff --numstat`: Spec 2 Zeilen ersetzt, Plan 3 ersetzt und 4 hinzugefügt (die zwei Nachträge mit je einer Leerzeile). Keine Zeile gelöscht.

## Nicht angefasst

`portfolio.md` (der Zusammenhang steht als eigener Defekt), `CLAUDE.md`, die Circle-Datensätze, jede Datei außerhalb von `planning/` dieses Circles, kein Code. Die Dateimarker der beiden Planungsdateien sind unverändert: der Spec trägt weiter `_o_`, der Plan weiter `_c_`. Nicht committet, nicht gestaged. Der Defektdatensatz bleibt auf `_o_`; den `Resolved:`-Vermerk setzt der Orchestrator.
