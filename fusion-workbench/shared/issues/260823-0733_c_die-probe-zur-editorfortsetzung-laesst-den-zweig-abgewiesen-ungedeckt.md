Die Probe zur Editorfortsetzung lässt den Zweig `Abgewiesen` ungedeckt

---

`die_editorfortsetzung_misst_vor_dem_einblenden` (`anwendung.rs:8166-8191`) hält die
Reihenfolge in `editorausgang_behandeln` an zwei Nadeln fest: `bildschirmbreiten_uebernehmen(`
muss vor `fokus_holen(` stehen. Der Rumpf ändert die Sichtbarkeit aber an **zwei** Stellen —
über `fokus_holen` im Zweig `Geoeffnet | SchonOffen` und über `editor_ausblenden` im Zweig
`Abgewiesen` (`:6487`). Die Messung ließe sich in den ersten Zweig verschieben, ohne dass die
Probe rot würde; der zweite Zweig liefe dann ohne Messung, und der Defekt wäre in seiner
zweiten Gestalt zurück.

---

**Gemessen am Baumstand `df8163d`.** Die drei Proben aus `mod sichtbarkeitsproben` laufen und
sind grün (`cargo test -p krk-ui sichtbarkeitsproben`).

## Was die Probe prüft

```rust
// crates/krk-ui/src/appkit/anwendung.rs:8181
let messung = concat!("bildschirmbreiten_", "uebernehmen(");
let einblenden = concat!("fokus_", "holen(");
let rumpf = rumpf(&diese_datei(), "editorausgang_behandeln");
...
assert!(stelle_messung < stelle_einblenden, ...);
```

`rumpf` (`:7813-7827`) schneidet den Methodenrumpf heraus und zieht Kommentarzeilen ab. Die
Behauptung lautet also: die Messung steht im Quelltext vor dem ersten `fokus_holen(`.

## Was sie damit nicht prüft

`editorausgang_behandeln` ändert die Sichtbarkeit an zwei Stellen:

| Zeile | Zweig | Weg in `sichtbarkeit_aendern` |
|---|---|---|
| `:6416` | `Ladeausgang::Geoeffnet \| SchonOffen` | `fokus_holen` → `bereich_einblenden` |
| `:6487` | `Ladeausgang::Abgewiesen` (aus der Sitzung) | `editor_ausblenden` → `bereich_umschalten` |

Die zweite steht im Quelltext **hinter** der ersten. Verschöbe jemand
`self.bildschirmbreiten_uebernehmen();` von der ersten Zeile des Rumpfs (`:6384`) in den ersten
`match`-Zweig — etwa neben `dateisystemwache_nachziehen()` —, bliebe
`stelle_messung < stelle_einblenden` wahr, die Probe grün, und der Zweig `Abgewiesen` liefe
ohne Messung in `sichtbarkeit_aendern` und damit in `aufteilung_nachziehen`. Genau das ist die
Lage, gegen die der Doc-Kommentar der Probe geschrieben ist (`:8166-8174`).

Die Zusage, die der Doc-Kommentar von `editorausgang_behandeln` gibt (`:6371-6381`), ist
stärker als das, was die Probe hält: „**Jeder Zweig unten** aendert die Sichtbarkeit — der
erste ueber `fokus_holen`, der letzte ueber `editor_ausblenden`". Die Probe kennt nur den
ersten.

## Vorschlag

Die Nadel `fokus_holen(` durch beide Sichtbarkeitsänderer ersetzen und gegen das Maximum
prüfen, statt gegen die eine Stelle:

```rust
for aenderer in [concat!("fokus_", "holen("), concat!("editor_", "ausblenden(")] {
    let stelle = rumpf.find(aenderer).expect(...);
    assert!(stelle_messung < stelle, ...);
}
```

Das altert mit dem dritten Änderer allerdings genauso. Die haltbarere Fassung prüft, dass die
Messung die **erste** Anweisung des Rumpfs ist — das ist die Zusage, die der Doc-Kommentar
gibt, und sie deckt jeden künftigen Zweig mit. Etwa: die erste nichtleere Zeile des Rumpfs
nach der `fn`-Zeile enthält die Nadel.

Die beiden anderen Proben des Moduls sind davon nicht betroffen:
`die_geaenderte_sichtbarkeit_kommt_auf_den_schirm` und
`der_nachzug_steht_vor_den_bereichsnachzuegen` lesen `sichtbarkeit_aendern`, und dessen Rumpf
hat nur einen Nachzug und nur eine Schleife.

**Schwere:** niedrig. Kein Fehlverhalten heute. Die Probe hält die Verdrahtung, die `df8163d`
gelegt hat, für einen von zwei Zweigen; als Schutz gegen die Rücknahme, für die sie geschrieben
ist, deckt sie die Hälfte.

**Gefunden:** coderev, Durchsicht des Commits `df8163d` am 260823-0733, Bereich
`ab11eb8..df8163d`

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:8166-8191`

**Domain:** code

**Verwandt:**
`shared/issues/260815-1446_o_die-probe-zum-einen-rufer-des-weitergabehinweises-liest-drei-von-sechs-modulen-und-nicht-den-zweig.md`
und `shared/issues/260815-1447_o_die-probe-ueber-die-paarung-von-zielen-und-architekturen-prueft-mitgliedschaft-statt-paarung.md`
— dieselbe Klasse: eine Quelltextprobe, deren Nadel weniger hält als ihr Doc-Kommentar zusagt.

---

In Arbeit: 260823-1137 durch coder. Die Probe heisst jetzt
`die_editorfortsetzung_misst_als_erste_anweisung` und prueft nicht mehr eine Reihenfolge
gegen eine Nadel, sondern die Stellung: die erste Anweisung des Rumpfs von
`editorausgang_behandeln` enthaelt `bildschirmbreiten_uebernehmen(`. Das ist die
haltbarere der beiden im Datensatz vorgeschlagenen Fassungen; sie deckt auch den dritten
Sichtbarkeitsaenderer mit, den noch niemand geschrieben hat, und sie ist genau die
Zusage, die der Doc-Kommentar von `editorausgang_behandeln` gibt.

Ein kleiner Helfer `erste_anweisung` steht daneben; er ueberspringt die Signatur bis zu
der Klammer, die den Rumpf oeffnet, und dann die Leerzeilen.

**Gemessen und nicht behauptet**: die Messung wurde versuchsweise aus der ersten Zeile
des Rumpfs in den Zweig `Geoeffnet | SchonOffen` verschoben, neben
`dateisystemwache_nachziehen()`. Die neue Probe wird dabei rot. Die alte waere gruen
geblieben: an demselben Baumstand nachgerechnet liegt die Messung bei Zeichen 376 und
`fokus_holen(` bei 468, also `stelle_messung < stelle_einblenden`. Der Datensatz trifft
damit zu. Bleibt zum Schliessen mit dem Commit.

---
Resolved: `52fba42` — behoben, `make check` gibt 0 zurück. Durchsicht: die Befunde stammen aus
`shared/reviews/260823-0735-coderev-einblenden-erreicht-den-schirm.md` und
`shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`; was im Einzelnen getan ist, steht
im Protokoll `shared/history/260823-1137-coder-acht-befunde-aus-zwei-durchsichten.md`.
