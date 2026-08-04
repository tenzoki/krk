# S11b — Acht Tastennamen in der Kombinationsschreibweise nachgetragen

---
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `#### 11b.`
**Dateien:** `crates/krk-core/src/tasten/parser.rs`, `crates/krk-core/tests/belegung.rs`
**Commit:** noch nicht committet (der Orchestrator committet)

---

## Was umgesetzt wurde

Die Tabelle `TASTEN` in `crates/krk-core/src/tasten/parser.rs` wächst von 53 auf
61 Einträge. Neu sind die beiden fehlenden Pfeile und die sechs fehlenden
Funktionstasten der Reihe F1 bis F12, alle als `dokumentiert(...)`:

| Name | Tastencode | Carbon-Name |
|---|---|---|
| `left` | 123 | `kVK_LeftArrow` |
| `right` | 124 | `kVK_RightArrow` |
| `f1` | 122 | `kVK_F1` |
| `f2` | 120 | `kVK_F2` |
| `f9` | 101 | `kVK_F9` |
| `f10` | 109 | `kVK_F10` |
| `f11` | 103 | `kVK_F11` |
| `f12` | 111 | `kVK_F12` |

`left` und `right` stehen im Pfeilblock hinter `up` und `down`; die sechs
Funktionstasten stehen in einem eigenen Block hinter der Norton-Reihe, mit dem
Grund im Kommentar, warum die Reihe bei F12 endet und nicht bei F19.

`resources/default-keymap.toml` ist unverändert. Ein Name ist keine Belegung.

## Die Codes sind selbst nachgelesen, nicht übernommen

Quelle: `kVK_*` in
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h`,
Zeilen 288 bis 314, gelesen am 260804-1200.

Gefunden: `kVK_LeftArrow = 0x7B` (123), `kVK_RightArrow = 0x7C` (124),
`kVK_F1 = 0x7A` (122), `kVK_F2 = 0x78` (120), `kVK_F9 = 0x65` (101),
`kVK_F10 = 0x6D` (109), `kVK_F11 = 0x67` (103), `kVK_F12 = 0x6F` (111).

**Alle acht stimmen mit der Tabelle des `planner` überein.** Keine Abweichung,
kein vertauschtes Paar. Die beiden verwechslungsanfälligen Stellen sind
kontrolliert: F1 bis F12 stehen im Header nicht in Nummernfolge (F9 ist 0x65,
F10 ist 0x6D, F11 ist 0x67, F12 ist 0x6F), und die Pfeile links und rechts
liegen mit 0x7B und 0x7C unmittelbar vor 0x7D und 0x7E, die schon als `down`
und `up` in der Tabelle standen.

## Das neue Beispiel der Prüfung für den unbekannten Tastennamen

`eine_fehlende_oder_unbekannte_taste_ist_ein_fehler` belegte den Fall bisher mit
`cmd+left`. Seit diesem Schritt ist `left` ein bekannter Name. Das Beispiel
lautet jetzt `cmd+arrowleft`, die Aussage der Prüfung bleibt.

**Warum dieser Name.** `arrowleft` ist die Schreibweise anderer Systeme für
dieselbe Taste. Die Tabelle darf ihn nie aufnehmen, weil er den Tastencode 123
bräuchte, den `left` bereits hält, und
`jeder_name_und_jeder_code_steht_genau_einmal` lässt keine zwei Einträge auf
denselben Code. Das Beispiel ist damit nicht bloß heute unbekannt, sondern
maschinell gegen eine spätere Erweiterung gesichert.

Verworfen wurden `f13` und ein Satzzeichen wie `bracketleft`. Beide gehören zu
Gruppen, die die Regel "die Schreibweise wächst um ganze Tastengruppen" später
aufnehmen kann; das Beispiel hielte nur bis zur nächsten Erweiterung.

## Abnahme

| Prüfung | Ergebnis |
|---|---|
| `cargo test -p krk-core` | 0, alle Prüfungen grün |
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 194 Prüfungen, 1 ignoriert (unverändert) |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |

Namentlich bestätigt:

- `jeder_name_und_jeder_code_steht_genau_einmal` ... ok — deckt maschinell ab,
  dass keiner der acht neuen Codes schon vergeben war.
- `genau_die_drei_funktionstasten_der_messung_sind_gemessen` ... ok — unverändert
  im Text, liefert weiter `["f3", "f5", "f8"]`. Keine der acht neuen Tasten ist
  aufgewertet worden.
- `die_tabelle_deckt_die_ganze_schreibweise_ab` ... ok — nennt jetzt `f1` bis
  `f12` sowie `left` und `right`.
- `die_tastencodes_stimmen_mit_der_carbon_tabelle_ueberein` ... ok — die acht
  neuen Zahlen stehen als Gegenprobe in Hex darin.
- `jede_taste_der_tabelle_ueberlebt_lesen_und_schreiben` ... ok — läuft über alle
  61 Einträge, also auch über die acht neuen.

## Was der Belegungsansicht aus S20 später begegnet

Zwei Beobachtungen aus dem Erweitern, beide ohne Defektcharakter, deshalb hier
und nicht unter `issues/`:

1. **Die Ansicht braucht eine Beschriftung, die die Tabelle nicht führt.** Ein
   Eintrag trägt `name`, `code` und `herkunft`. `Kombination`s `Display` schreibt
   `f10` und `left` — die Form der Belegungsdatei, nicht die Form, die auf einer
   Taste steht. Für die Belegungsansicht heißt das entweder eine zweite Spalte in
   `Taste` oder eine Übersetzung in der Ansicht; das Zweite wäre die zweite
   Namensliste, die der Modulkopf ausschließt.

2. **`f1` bis `f12` sind ab Werk vom System belegt.** Der Entscheidungsdatensatz
   `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`
   beantwortet das für F3 bis F8. Die Ansicht weist eine Kombination zu, indem
   der Nutzer sie drückt; drückt er F10 ohne die Systembelegung abgeschaltet zu
   haben, kommt bei KRK kein Tastendruck an, und die Ansicht kann nicht zwischen
   "Taste unbekannt" und "Tastendruck nie angekommen" unterscheiden. Der Nutzer
   sieht in beiden Fällen, dass nichts passiert. Das trifft S20, nicht diesen
   Schritt.
