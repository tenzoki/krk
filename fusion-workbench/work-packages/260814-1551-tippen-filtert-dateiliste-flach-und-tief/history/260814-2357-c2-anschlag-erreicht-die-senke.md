# C2 — Der Anschlag erreicht die Senke, und der eine Zweig ruft die Regel

**Status:** Complete
**Datum:** 260814-2357
**Executor:** coder
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang C, Schritt C2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.14 bis C1.20, C6.9, C6.11
**Verification:** `make check` — Exit 0

## Was gebaut ist

Der Tastendruck reicht jetzt bis in den Ausführungszweig, und dort entscheidet die
Regel aus C1 (`crates/krk-ui/src/kommandos/rueckschritt.rs`, Commit `2ff4b5a`), was ein
Druck auf `delete` bedeutet. Die Regel ist gerufen und nicht nachgebaut.

**Der Weg, in der Reihenfolge, in der er läuft:**

```text
NSEvent ─┬─ Tastendruck::aus_ereignis
         └─ isARepeat            ──> Anschlag { druck, wiederholung }
                                        │
              Belegung::nachschlag ─────┤
                                        ▼
                    Eingabe::Kommando { kommando, anschlag }
                                        │
                       eingabe_ausfuehren ── Merker zurücksetzen,
                                        │    außer bei der nackten Rücktaste
                                        ▼
          kommando_ausfuehren(kommando, Option<Anschlag>)
                                        │
                  zulaessigkeit::zulaessig (unverändert)
                                        ▼
                    Kommando::InPapierkorb
                                        │
                  papierkorb_oder_zeichen_zurueck(anschlag)
                       │            │              │
                  kein Anschlag  cmd+delete   nackte Rücktaste
                       ▼            ▼              ▼
                  Papierkorb    Papierkorb    rueckschritt(…)
```

### `crates/krk-ui/src/appkit/ereignisse.rs`

- `Anschlag { druck: Tastendruck, wiederholung: bool }` als kleine Struktur **neben**
  `Tastendruck`. Der Tastendruck ist unangetastet: er ist der Nachschlagschlüssel, trägt
  `Hash` und `Ord`, und ein Wiederholungsbit darin änderte, was „zwei Ereignisse ergeben
  denselben Tastendruck" heißt.
- `Anschlag::ist_nackter_rueckschritt` — die **eine** Erklärung der Frage, mit zwei
  Fragern: dem Ausführungszweig und der Rücksetzzeile des Merkers.
- `Eingabe::Kommando` ist von einem Tupelwert zu `{ kommando, anschlag }` geworden.
- `behandeln` liest `ereignis.isARepeat()`. **Das ist die erste Lesestelle dieses Werts
  im Baum**; der Modulkopf sagt es und nennt die Folge: der Messmodus schreibt in seine
  synthetischen Ereignisse `false` (`ereignis_senden`) und kann den Wiederholungszweig
  deshalb nicht fahren. Die Abnahme von C1.18 und C1.20 bleibt am Bündel und damit
  Nutzerarbeit. Ein Weg um diese Grenze herum ist nicht gebaut.
- Untergrenze nachgetragen: `NSEvent.isARepeat` steht seit macOS 10.0.
- Probe `nur_die_nackte_ruecktaste_gilt_als_rueckschritt`: sie fährt die beiden
  Kombinationen der Funktion `in_papierkorb` aus der Auslieferungsbelegung durch und
  hält fest, dass `delete` als nackte Rücktaste zählt und `cmd+delete` nicht (C1.17).

### `crates/krk-ui/src/appkit/anwendung.rs`

- `kommando_ausfuehren(kommando, anschlag: Option<Anschlag>)`. Drei Aufrufstellen: die
  Senke reicht `Some(anschlag)` durch, der Menüeintrag (`krkKommando:`) und der Melder
  der Bereichsleiste geben `None`. **`None` ist die Aussage „es gab keinen
  Tastendruck"** und damit die Antwort auf C1.19 und C6.11 in der Signatur statt in
  einem Zweig.
- Neuer Ivar `rueckschritt_merker: Cell<bool>` — ob die laufende Wiederholung bei
  stehendem Filtertext begann. Er wohnt beim Delegierten und nicht am Tab: eine
  Tastenwiederholung gehört keinem Tab, denn ein Tabwechsel braucht einen anderen
  Tastendruck, und der beendet die Wiederholung.
- Eine Zeile am Kopf von `eingabe_ausfuehren` setzt den Merker bei jeder Eingabe
  zurück, die nicht die nackte Rücktaste ist. Sie nimmt der Rechnung die Annahme, dass
  AppKit `isARepeat` nur für aufeinanderfolgende Drücke derselben Taste setzt.
- `papierkorb_oder_zeichen_zurueck` als der eine Aufrufer der Regel. `betroffene` wird
  für `ZeichenZurueck` und `Nichts` nicht befragt, weder Auswahl noch Markierung werden
  angefasst (C6.9).
- `Kommando::InPapierkorb` ruft ihn. `f8`, `opt+cmd+delete` und `ctrl+delete` haben
  keine Zeile bekommen: die ersten beiden tragen `Kommando::EndgueltigLoeschen`, der
  dritte geht durch `Leistenquelle::kommando_ausfuehren`.

### `crates/krk-ui/src/kommandos/rueckschritt.rs`

- **Das `#[cfg_attr(not(test), expect(dead_code, …))]` ist an beiden Stücken
  gefallen**, wie C1 es vorgesehen hatte: mit dem Aufrufer wurde die Erwartung
  unerfüllt, und der Bau hielt unter `-D warnings` an, bis die Zeilen weg waren. Der
  Modulkopf trägt an ihrer Stelle jetzt den Abschnitt `# Der eine Aufrufer`.
- Probe `die_regel_hat_genau_einen_aufrufer`, in der Form von
  `beide_frager_rufen_die_eine_regel` und über `crate::quellbaum::aufrufstellen`
  (C6.10).

### `crates/krk-ui/src/appkit/tabelle.rs` — die dritte Datei

Nicht in der Dateiliste von C2, und aus demselben Grund angefasst wie die vierte bei
E1: aus `anwendung.rs` ist das `Ordnermodell` des sichtbaren Tabs nicht erreichbar, der
Weg läuft über `DateifensterQuelle`. Zwei öffentliche Methoden neben
`tiefe_suche_umschalten`, in derselben Bauart: `filter_steht` liest, und
`letztes_filterzeichen_weg` nimmt das Zeichen zurück und ruft `umsortiert`. Datensatz:
`issues/260814-2357_o_c2-nennt-zwei-dateien-der-weg-an-den-filtertext-des-tabs-fuehrt-durch-eine-dritte.md`.

## Was nicht angefasst ist

`crates/krk-ui/src/kommandos/zulaessigkeit.rs` steht unverändert da, und seine Tafel aus
280 Fällen behält ihre Bedeutung. Der Menüeintrag „In den Papierkorb räumen" wird nicht
ausgegraut. `Tastendruck`, `Lage`, `fokus::wirkt`, `operationen::betroffene` und
`angezeigtedatei::welche` sind unverändert.

## Abnahme

`make check` — Exit 0. Alle vier Kommandos grün: `cargo build --workspace`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo fmt --all --check`.

**Was der Baum nicht abnimmt:** die Bündelanteile von C1.15, C1.17, C1.18 und C1.20. Sie
sind am laufenden `KRK.app` im Vordergrund zu beobachten und stehen in Schritt G2; zwei
davon prüfen die Richtung, in der ein Fehler Dateien wegräumt, und laufen in einem
Prüfordner mit entbehrlichen Dateien. C1.18 und C1.20 kann der Messmodus nicht fahren.

**Und noch nicht am Bündel zu sehen:** Schritt B1 ist nicht gefahren, also gibt es noch
keinen Weg, einen Filtertext zu tippen. Die Fallunterscheidung steht und ist geprüft,
aber `filter_steht` antwortet heute in jeder Lage `false` — die Rücktaste räumt also
weiter wie bisher, und das ist der erwartete Zwischenstand und kein Befund.

## Datensätze

- `decisions/260814-2102_i_gehoert-die-fallunterscheidung-der-rueckschritt-taste-in-die-zulaessigkeitsregel.md`
  — von beantwortet auf umgesetzt gezogen, mit Pfadzitat statt Commit-Hash: der Nutzer
  setzt den Commit.
- Zwei Datensätze bleiben bewusst auf beantwortet, weil ihre nutzersichtbare Wirkung an
  B1 hängt: `260814-1830_a_wie-nimmt-der-nutzer-ein-einzelnes-zeichen-des-filters-zurueck.md`
  und `260814-1852_a_raeumt-ein-gehaltener-rueckschritt-weiter-wenn-der-filtertext-leer-wird.md`.
