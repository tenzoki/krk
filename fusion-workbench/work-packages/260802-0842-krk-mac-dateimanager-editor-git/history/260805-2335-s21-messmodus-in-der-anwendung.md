# S21: Messmodus in der Anwendung — alle zehn Zusagen in einem Bericht

**Agent:** coder
**Datum:** 260805-2335
**Status:** Complete

## Auftrag

Planschritt S21 (`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, `#### 21.`): der Messmodus, der L1, L5, L6, L7, L8 und L9 im laufenden Buendel auf der Pruefsitzung aus C8 misst, L4 ueber den aeusseren Aufrufer, und die Zusammenfuehrung mit der kopflosen Strecke aus S3 zu einem Bericht ueber alle zehn Zusagen. Die Abnahme von L8 liegt seit 260804-2318 in diesem Schritt.

## Was entstanden ist

**Anwendung (`krk-ui`):**
- `src/messmodus.rs`: zwei neue Aufgaben. `--messmodus <plan.toml>` (Sitzungsstrecke: Messplan einlesen, Pruefsitzung ueber `Sitzungsschreiber` als `session.toml` herstellen, Schrittliste mit je zwanzig Messungen fuer L1, L5-Tab, L5-Fenster, L6, L7, L8, L9) und `--messmodus sitzungsstart` (L4: Sitzung aus `session.toml` wiederherstellen, bedienbar melden, sobald beide sichtbaren Tabs ihre erste Bildschirmseite zeigen). `Zustand` um die `Sitzungslage` erweitert (aktives Fenster, Tabs, Auswahlpfad, Vorschau, Vorgangsanzeige). Der Messplan traegt `[sitzung]` in der Serialisierung von `session.toml` — kein zweites Format. Kopierziel-Pruefung (Verzeichnis, leer, derselbe Datentraeger wie A, sonst Abbruch ohne Zahl) und das Leeren zwischen den Wiederholungen liegen hier. Keine `use objc2`-Zeile; die S23-Grenzpruefung laeuft leer.
- `src/appkit/ereignisse.rs`: `funktion_senden` — die erste Kombination einer Funktion aus der **Belegung** als synthetisches `NSEvent` in die eigene Schlange (`postEvent:atStart:`); bricht ab, wenn die Funktion keine Kombination traegt.
- `src/appkit/anwendung.rs`: `sitzung_laden` je Aufgabe (S8-Strecken unveraendert; Sitzungsstrecke stellt her und schreibt; Sitzungsstart laedt ohne Schreiber und bricht ohne lesbare Pruefsitzung ab), `messzustand()` als der eine Zustandsbauer, `messhandlung()` fuer ungemessene Vorbereitungen, Bildtakt-Rueckruf auf schwache Referenz umgestellt.
- `src/appkit/tabelle.rs`: Ablesegetter `auswahl_pfad()` und `vorgang_sichtbar()`.
- `src/vorschaumodell.rs` + `src/appkit/vorschau.rs`: der angezeigte Pfad je Vorschau-Tab (wechselt erst mit der gelieferten Meldung) und `laedt_noch()` — die Endbedingung von L7.

**Messwerkzeug (`krk-bench`):**
- `src/messen.rs`: `Gesamtlauf`/`Gesamtergebnis` — Sitzungslauf, zwanzig `sitzungsstart`-Prozessstarts (L4), kopflose Strecke (L2, L3, L10 warm), Messplan-Schreiber (Pruefsitzung ueber die `Sitzung`-serde aus `krk-core`), `kopierziel_pruefen` (anderer Datentraeger → Rueckgabewert ungleich 0, keine Zahl), L6-Unterordner mit 1.000 Eintraegen neben A (Startwert 4, mit Steckbrief, wiederverwendet), `systemlast()` (`sysctl vm.loadavg`).
- `src/bericht.rs`: `gesamt_verfassen`/`gesamt_schreiben` — der Bedingungskopf mit den drei Pruefordnern samt Startwert, Kopierziel, Pruefsitzung, Bildwiederholrate als Zahl aus NSScreen und der Systemlast vor/nach dem Lauf als neunter Angabe; L1 ausdruecklich als Spanne bis zum Ende des Zeichendurchgangs gekennzeichnet.
- `src/main.rs`: Unterbefehl `alle` neben fixture, messen, durchstich.

**Einstiegspunkt:** `xtask/src/messen.rs` (neu) + `xtask/src/main.rs`: `cargo xtask messen --alle …` baut das Buendel und faehrt `krk-bench alle`; `--kopflos` reicht an die S3-Strecke durch — ein Einstiegspunkt fuer beide Strecken. `Makefile`: Ziel `alle`, `fixture` legt jetzt auch Pruefordner B (Startwert 2) an.

## Messlauf (Funktionsnachweis, eine Runde, Bericht `messungen/260805-2134-alle-zusagen.txt`)

Alle zehn Zusagen mit p95, Median, Minimum und vollstaendigem Kopf; Bildwiederholrate 60. **L8: p95 169,777 ms < 200 ms — gehalten** (die Abnahme dieses Schritts). Alle acht Dauerzusagen halten (L4 391 ms, L5 36/15 ms, L6 48 ms, L7 55 ms). **L1 (75 %) und L9 (90 %) verfehlen den Anteil** in diesem Lauf unter Fremdlast (loadavg ~3) — nicht gelockert, als Defekt gemeldet: `issues/260805-2335_o_l1-und-l9-verfehlen-den-anteil-im-ersten-gesamtlauf-unter-fremdlast.md`. Die volle Reihe ist S22.

Die L5-Quantisierungswarnung des Plans hat sich nicht bestaetigt: p95 36,2 ms (Tab) bei 50 ms Budget.

`make check`: alle vier gruen (Bau, 265 Pruefungen, fmt, clippy -D warnings). Die Sicherung der Nutzer-`session.toml` wurde nach dem Lauf zurueckgespielt; das Kopierziel blieb leer.

## Entscheidungen im Kleinen (im Code begruendet)

- Synthetische Tasten kommen aus der Belegung (Rueckwaertsschlag Funktion → Kombination), nicht aus festen Codes: eine umbelegte `keymap.toml` misst die richtige Funktion oder bricht ab.
- L5 misst mit vorgewaermten Zielordnern (ungemessener Wechsel davor); der Bericht weist genau diesen Fall aus.
- L6 misst am eigens erzeugten 1.000er-Unterordner neben A (die Unterordner der Pruefordner sind leer und truegen nichts).
- Je L8/L9-Wiederholung: F5, ein Pfeil ab waehrend der Kopie, Abbruch, Kopierziel leeren, Auffrischung abwarten — jede Wiederholung misst dasselbe.
