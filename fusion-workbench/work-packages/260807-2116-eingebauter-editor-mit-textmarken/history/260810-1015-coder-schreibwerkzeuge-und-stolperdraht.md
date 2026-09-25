# Coder: die acht Datensätze zum Stolperdraht der Textflächen-Einstellungen

**Status:** Complete
**Agent:** coder
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Datum:** 260810-0930 bis 260810-1015
**Dateigrenze:** ausschließlich `crates/krk-ui/src/appkit/editor.rs`

---

## Auftrag

Acht offene Defekte mit einem Gegenstand: die textverändernden Automatiken von
macOS an der Textfläche des Editors, der Stolperdraht, der sie aufzählt, und die
Behauptungen im Modulkopf und in der Commit-Nachricht `d9fc2c8` darüber.

1. `260810-0512` Schreibwerkzeuge nicht abgewählt
2. `260810-0745` Stolperdraht sieht drei der vier Schreibwerkzeug-Einstellungen nicht
3. `260810-0746` dritte Tür außerhalb aller drei Namensformen
4. `260810-0747` Hinweis der Gegenrichtung von `libtest` verschluckt
5. `260810-0748` Kopplung der zehn Paare im Baum durch nichts gehalten
6. `260810-0749` Begründung „`unsafe` verbiete den sachlichen Schnitt" ist falsch
7. `260810-0750` „derselbe Speicher" stärker als die Messung
8. `260810-0751` Aufzählung sieht nur die Klasse selbst

## Der tragende Schnitt

Vier der acht (2, 3, 5, 8) sagen dasselbe aus verschiedenen Richtungen. Der eine
Schnitt, der sie zusammen erledigt, hat zwei Teile.

**Erstens: die Aufzählung bekommt zwei Quellen statt einer.** `FORMEN` allein war
eine Heuristik über Selektornamen. Daneben steht jetzt der sachliche Schnitt über
das Protokoll `NSTextInputTraits`, das ohne jede Namensform entscheidet — wer
Mitglied ist, ist eine Texteingabe-Einstellung. Der Schnitt ist erreichbar, was
Defekt 6 behauptet und die Commit-Nachricht bestritt: `objc2::ffi` führt
`protocol_copyPropertyList`, und `unsafe` ist in `src/appkit/` erlaubt. Dazu
läuft die Namensform-Quelle jetzt über die **ganze** Vererbungskette bis
`NSObject` statt nur über `NSTextView`, und `FORMEN` trägt sechs Formen statt
drei. Ergebnis: 36 eingeordnete Einstellungen statt 26.

**Zweitens: die Aufstellung wird die Vorlage der Messungen statt ihres
Nachtrags.** Vier neue Proben bauen eine `NSTextView` und lesen ihre Namen aus
`EINSTELLUNGEN`. Eine Zeile, die etwas Falsches behauptet, hält damit den Bau an,
statt eine Behauptung zu bleiben. Das erledigt 5 (Kopplung), 7 (`Default`), den
Messteil von 1 (Vorgabewert der Schreibwerkzeuge) und die Zusage über die sieben
Zeilen in `textflaeche_bauen`, die vorher als Nutzerarbeit geführt war.

**Die Hürde für Teil zwei war zu prüfen und ist geprüft.** Defekt 5 nennt
ausdrücklich offen, ob eine AppKit-Ansicht im Testlauf zu bauen ist. Sie ist es:
eine `NSTextView` lässt sich in `cargo test` erzeugen und über
`valueForKey:` / `setValue:forKey:` befragen. Der Preis ist die Behauptung des
Hauptfadens über `MainThreadMarker::new_unchecked`, sechs saubere vollständige
Läufe, und ein eigener Datensatz dafür (`260810-1001`).

## Was am Code geändert ist

Eine Datei: `crates/krk-ui/src/appkit/editor.rs`. `textflaeche_bauen` ist
**unverändert** — es waren keine neuen Zeilen im Auslieferungscode nötig, weil
die Kopplung jetzt gemessen statt vorausgesetzt wird.

Im Modulkopf:

- „zwei Türen" → drei Türsorten, mit der Sammeltür als eigener.
- „derselbe Speicher" → die gemessene, schwächere Aussage.
- Der Satz „sonst wäre `NSTextInputTraits` der sachliche **statt** des
  namensbasierten Schnitts" ist fort; der Protokollschnitt läuft **neben** dem
  namensbasierten, und der Kopf benennt die Gegenbehauptung aus `d9fc2c8` als
  falsch.
- Die Liste der Grenzen: die vierte (Oberklassen) ist geschlossen statt
  aufgenommen; die Richtungsgrenze sagt jetzt, wie der Hinweis wirklich
  ausgegeben wird; die Namensformgrenze gilt nur noch für die zweite Quelle, und
  die erste Quelle nennt ihre eigene.
- Vier Schreibwerkzeug-Einstellungen statt einer, mit gemessenen Werten und
  Verweis auf den Entscheidungsdatensatz.
- Ein Absatz zu den Proben in der Verfügbarkeitssektion: sie fragen die Laufzeit
  nach Namen und stellen deshalb keine Verfügbarkeitsfrage.

Unter `mod tests`:

- `FORMEN`: 3 → 6 Namensformen.
- `MERKMALSPROTOKOLL`, `setzer_des_protokolls`, `setzername`, `merkmalsname`,
  `getragene_einstellungen` — die zwei Quellen und die Kette.
- `Einordnung`: fünfte Antwort `SammeltuerZu(&[…])` neben `ZweiteTuerZu(&str)`.
- `EINSTELLUNGEN`: 26 → 36 Einträge; die Sammeltür, drei weitere
  Schreibwerkzeug-Einstellungen, sechs aus `NSView` und `NSResponder`.
- `SCHREIBWERKZEUGE`: der Pfad auf den Entscheidungsdatensatz, an einer Stelle.
- `an_einer_flaeche`, `probenrahmen`, `merkmal`, `merkmal_setzen`,
  `aus_bedeutet` — die Werkzeuge der Instanzproben, mit der Notlüge an einem Ort.
- `keine_unbekannte_einstellung_steht_an_der_textflaeche`: beide Quellen, und der
  Hinweis über `std::io::stderr` statt `eprintln!`.
- `jede_zweite_tuer_zeigt_auf_eine_beantwortete_einstellung` →
  `jede_tuer_zeigt_auf_beantwortete_einstellungen`, für beide Türsorten.
- Vier neue Proben: `die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`,
  `jede_zweite_tuer_und_ihre_erste_legen_einander_um`,
  `die_erste_tuer_kann_default_weder_herstellen_noch_anzeigen`,
  `die_sammeltuer_ist_eine_sicht_auf_dieselben_bits`,
  `der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl`.

## Messungen dieses Durchgangs

Alle auf macOS 15.7.7 (Build 24G720), aus `cargo test` heraus.

| Gegenstand | Ergebnis |
|---|---|
| `NSTextView` selbst, sechs Formen | 30 Selektoren (14 davon Protokollmitglieder) |
| `NSText` / `NSObject` | keine |
| `NSView` / `NSResponder` | 3 + 3 |
| `NSTextInputTraits` | 14 Pflichtmerkmale, keines mit eigenem Setzernamen |
| Kopplung, zehn Paare, beide Richtungen | hält an allen zehn |
| `Default` über die erste Tür | zeigt je Einstellung anders (8× YES, 2× NO), stellt sich nicht wieder her |
| Maske frisch / an KRKs Fläche | `0x23c1` / `0x2001`, nur Bits gefallen |
| Werkswert der Maske an KRKs Fläche | schaltet 4 Automatiken an, Grammatikprüfung aus |
| `writingToolsBehavior` | `Default` (0), frisch und an KRKs Fläche |
| `allowsWritingToolsAffordance` | ab Werk **an** |
| `eprintln!` in einem grünen Test | wird verschluckt |
| `std::io::stderr()` in einem grünen Test | erscheint |

## Zwei Datensätze irren, und die `Resolved:`-Zeile sagt es

- **`260810-0746`** sagt, die Sammeltür mache „fünf der sieben abgeschalteten
  Automatiken wieder an". Gemessen sind es vier; die fünfte Zeile seiner eigenen
  Ausgabe ist dieselbe Automatik durch ihre zweite Tür. Dafür fehlt eine Wirkung:
  der Werkswert schaltet die Grammatikprüfung aus.
- **`260810-0749`** Punkt (c) ist unscharf: `xtask`s `AUSNAHME` ist die Grenze für
  das Nennen einer `objc2`-Kiste, nicht für `unsafe`. Der Befund hält auf (a) und
  (b) allein.
- **`260810-0751`** zählt fünf Selektoren an den Oberklassen; es sind sechs, weil
  die neue Form `Types:` `setAllowedTouchTypes:` mitbringt.

## Abnahme

```
cargo build --workspace                  -> exit 0
cargo test --workspace                   -> exit 0   (313 Proben in krk-ui)
cargo clippy --workspace --all-targets   -> exit 0
cargo fmt --all --check                  -> exit 0
```

Dazu sechs vollständige Läufe von `cargo test --workspace` nach dem Umbau, alle
exit 0, wegen der AppKit-Instanzen in den Proben.

**Ein Fehlschlag außerhalb der Dateigrenze während der Arbeit**, inzwischen von
selbst grün: `krk-core/tests/text.rs::eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden`
schlug in fünf Läufen um 09:40 mit `Permission denied` fehl, während
`crates/krk-core/src/text/datei.rs` von einem parallelen Agenten geändert wurde
(`260809-1652`, Typprüfung auf dem Deskriptor). Nicht angefasst, und am Ende des
Durchgangs grün.

## Was offen bleibt

- **Die Lesart von C4** zu den vier Schreibwerkzeug-Einstellungen:
  `decisions/260810-0959_o_schliesst-c4-die-schreibwerkzeuge-aus.md`, drei
  Optionen und eine Empfehlung. Nutzerarbeit.
- **Der Hauptfaden der Instanzproben**:
  `issues/260810-1001_o_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`,
  drei Wege.
- **Eine Zweitschrift der Messungen** unter `spikes/` und `messungen/`, wie
  `260810-0748` sie vorschlug: nicht mehr die Sicherung, aber möglich. Beide
  Verzeichnisse lagen außerhalb der Dateigrenze dieses Durchgangs.
- **Die Marker** der acht Datensätze benennt der Nutzer um.
