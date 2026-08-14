# E3 — Das neunte Ankreuzfeld

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang E, Schritt E3
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C2.1 bis C2.4
**Verification:** `make check` — exit 0

## Was die Leiste jetzt trägt

Neun Ankreuzfelder in drei Gruppen, von links nach rechts:

```text
[Lesezeichen] [Links] [Rechts] [Vorschau] [Editor]   [Groesse] [Datum] [Typ]   [Deep]
└─ Bereich::ALLE, tag = index ─────────────┘         └─ Spalte::ALLE ────┘     └ ohne tag
                                    GRUPPENABSTAND ──┘             GRUPPENABSTAND ──┘
```

**Der neunte ist ein einzelnes Feld `tiefenschalter: Retained<NSButton>` und keine dritte
Sammlung.** „Deep" ist weder ein Bereich noch eine Spalte, sondern die Sucheinstellung des
sichtbaren Tabs; eine Aufzählung mit einem Wert wäre eine Aufzählung zu viel. Sein Kommando
steht als Konstante `KOMMANDO_DER_TIEFE`, seine Aufschrift als `AUFSCHRIFT_DER_TIEFE`, und
der Selektor `tiefeGedrueckt:` wie die Probe lesen dieselben zwei Konstanten. Eine
Aufstellung daneben prüfte sich selbst.

**Er braucht keine `tag`.** `schalter_bauen` nimmt die Stelle deshalb als `Option<usize>`:
die beiden Gruppen geben `Some(...)`, weil ihre Schalter sich einem Selektor gegenüber
nennen müssen, den mehrere teilen; der neunte gibt `None`, weil er der einzige seines
Selektors ist. Eine `tag` an ihm wäre eine Nummer, die niemand liest — und die ein Leser
für die Stelle in einer Aufzählung hielte.

**`Fokus` bekommt keinen sechsten Wert** (C2.2). Der Schalter geht durch dieselbe eine
Bauzeile `setRefusesFirstResponder(true)` wie die acht vorhandenen; die Begründung im
Modulkopf unter `# Kein Schalter nimmt den Ersthelferrang an` gilt für ihn unverändert.

## Wer den angezeigten Stand schreibt

`Bereichsleiste::zustaende_setzen` bekommt ein drittes Argument `tief: bool` und bleibt der
eine Schreiber. Die Leiste hält weiter keinen Stand: `Leistenquelle::geklickt` nimmt die
Selbstkippung des Ankreuzfelds zurück, bevor das Kommando gemeldet wird, und zwar für den
neunten auf demselben Weg wie für die acht (C2.3).

**`tief` kommt aus einer anderen Quelle als die beiden anderen Argumente, und die Leiste
erfährt davon nichts.** Sichtbarkeit und Spaltensichtbarkeit stehen im `Fenstermodell`,
`tief` am `Ordnermodell` des sichtbaren Tabs im **aktiven** Dateifenster — dieselbe Adresse,
an die `tiefe_suche_umschalten` aus E1 schreibt. Zwei verschiedene Adressen für Schreiben
und Lesen zeigten einen Stand, den der Klick nicht gekippt hat.

Der Weg dorthin führt durch `DateifensterQuelle`, also durch eine dritte Datei; sie trägt
jetzt den Leser `tiefe_suche_steht()`, neben `filter_steht` und in derselben Bauart. Der
Befund steht im Datensatz `issues/260814-2357_o_c2-nennt-zwei-dateien-…` und nicht in einem
neuen.

## Die Anlässe: einer neu, einer schon da

E3 nennt drei neue Anlässe für `bereichsleiste_nachziehen`. Gebraucht wurde **eine** Zeile:

| Anlass | Weg | neue Zeile? |
|---|---|---|
| Tabwechsel | `tab_gewechselt` → `ordnerwechsel_melden` | ja, dieselbe |
| Ordnerwechsel | `ordner_lesen`, `sichtbaren_lesen` → `ordnerwechsel_melden` | ja, dieselbe |
| Wechsel des aktiven Dateifensters | `aktives_setzen` bzw. `Kommando::FensterWechseln` → `aufteilung_nachziehen` | nein |

Der Rückruf `ordnerwechsel_setzen` deckt die ersten beiden ab, mit der Maus wie mit der
Taste, und der Nachzug der Leiste tritt dort **neben** `dateisystemwache_nachziehen` und
`titel_nachziehen` und nicht in sie hinein: jede dieser Funktionen hat genau einen
Gegenstand. Der dritte Anlass läuft auf beiden Wegen ohnehin durch
`aufteilung_nachziehen`, das `bereichsleiste_nachziehen` schon enthält.

## Die offene Frage bleibt billig umzukehren

`decisions/260814-1830_o_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md` ist offen, und
der Bau fährt auf „je Tab". Fällt die Antwort auf „je Fenster", ist die Naht die
**Quelle des dritten Arguments** und sonst nichts: `bereichsleiste_nachziehen` läse aus dem
Fenstermodell statt aus dem Tab, die eine Zeile im Ordnerwechsel-Rückruf fiele weg, und
`bereichsleiste.rs` bliebe unberührt. Beide Stellen sagen das im Kommentar, damit die
Umkehr nicht gesucht werden muss.

## Was geprüft ist und was am Bündel zu sehen bleibt

| Kriterium | Probe | am Bündel |
|---|---|---|
| C2.1 Dasein und Aufschrift `Deep` | `der_neunte_schalter_heisst_deep_und_steht_rechts_von_typ`, `die_leiste_traegt_neun_schalter` | die Lage auf dem Schirm und die 18 Punkte Höhe |
| C2.2 kein sechster Fokuswert | `der_neunte_schalter_gibt_fokus_keinen_sechsten_wert` | dass `setRefusesFirstResponder` wirklich greift |
| C2.3 ein Kommando, kein eigener Stand | `jeder_schalter_nennt_genau_ein_eigenes_kommando` (deckt jetzt neun) | dass die Selbstkippung nicht aufblitzt |
| C2.4 Wirkungsbereich entscheidet | `jeder_schalter_wirkt_aus_jedem_fokus` (deckt jetzt neun) | — |
| C2.4 ohne Filtertext ändert sich nichts | `ohne_filtertext_aendert_die_tiefe_suche_nichts` (`krk-core`) | — |

**Drei Kriterien sind an einer Probe abgenommen, C2.1 und C2.2 nur zur Hälfte.** Die
Reihenfolge der Schalter prüft die Probe über die Reihenfolge, in der `bauen` einhängt, und
das ist dieselbe Reihenfolge von links nach rechts; die tatsächliche Lage in der 18 Punkte
hohen Leiste ist Nutzerarbeit. Für `setRefusesFirstResponder` gilt dasselbe: der neunte
Schalter geht durch dieselbe Bauzeile wie die acht vorhandenen, und ein Bau ohne AppKit
kann das nicht nachsehen.

## Dateien

- `crates/krk-ui/src/appkit/bereichsleiste.rs` — der neunte Schalter, `KOMMANDO_DER_TIEFE`,
  `AUFSCHRIFT_DER_TIEFE`, Selektor `tiefeGedrueckt:`, `schalter_bauen` mit `Option<usize>`,
  drittes Argument an `zustaende_setzen`, Modulkopf, drei neue Proben
- `crates/krk-ui/src/appkit/anwendung.rs` — `bereichsleiste_nachziehen` holt `tief`, zweiter
  Anlass im Ordnerwechsel-Rückruf
- `crates/krk-ui/src/appkit/tabelle.rs` — Leser `tiefe_suche_steht`
- `crates/krk-core/tests/verzeichnis.rs` — `ohne_filtertext_aendert_die_tiefe_suche_nichts`
- `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/spalten.rs`,
  `crates/krk-ui/src/fenstermodell.rs` — die Zahl „acht Schalter" in Prosa nachgezogen
