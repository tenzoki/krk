# Behebungsschleife in den Rundenverzeichnissen, Kiste `krk-ui`

**Agent:** coder
**Status:** Complete
**Baumstand bei Beginn:** `2fa1d0e`
**Grenze:** ausschließlich `crates/krk-ui/`

## Verification

```
cargo test    -p krk-ui                              — exit 0  (903 + 5 Proben grün)
cargo clippy  -p krk-ui --all-targets -- -D warnings — exit 0
cargo fmt     -p krk-ui -- --check                   — exit 0
RUSTDOCFLAGS="-D warnings" cargo doc -p krk-ui --no-deps — exit 0
```

## Bestand

`find fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'` gibt 139 Datensätze. Davon zitieren 69 `crates/krk-ui` ohne `krk-core` daneben, 21 beide. Die 69 sind einzeln gelesen und in drei Gruppen sortiert: behebbar am lebenden Baum, Aussage über einen eingefrorenen Spec- oder Plantext, Nutzerentscheidung oder Nutzerarbeit.

23 sind behoben und in ihrem Rundenverzeichnis auf `_c_` gesetzt.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/ereignisse.rs`
- `crates/krk-ui/src/appkit/menue.rs`
- `crates/krk-ui/src/appkit/mod.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/titelzusatz.rs`
- `crates/krk-ui/src/appkit/zwischenablage.rs`
- `crates/krk-ui/src/appkit/blaetter/zettel.rs`
- `crates/krk-ui/src/kommandos/mod.rs`
- `crates/krk-ui/src/kommandos/operationen.rs`
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`
- `crates/krk-ui/src/angezeigtedatei.rs`
- `crates/krk-ui/src/markdown.rs`
- `crates/krk-ui/src/spalten.rs`

## Wo eine Zahl gefallen ist statt berichtigt worden zu sein

Die Regel des Auftrags war, eine Zahl im Kommentar durch das Kommando zu ersetzen, das sie zählt, oder durch den Zeiger auf die Stelle, die die Auskunft trägt. Angewandt an acht Stellen:

| Stelle | vorher | jetzt |
|---|---|---|
| `zulaessigkeit.rs`, vier Stellen | „Tafel aus 280 Faellen", „zweihundertachtzigmal" | „Tafel ueber alle Faelle"; die Zahl rechnet `die_tafel_aus_allen_faellen_geht_auf` aus dem Produkt der drei Aufzaehlungen |
| `menue.rs`, Doc von `validateMenuItem:` | „Tafel aus 280 Faellen" | dasselbe |
| `anwendung.rs`, Doc von `kommando_ausfuehren` | „ihre Tafel aus 280 Faellen" | dasselbe |
| `anwendung.rs`, Doc von `lage` | „Drei Abnehmer lesen sie" | die vier Abnehmer namentlich, dazu `grep -n 'self\.lage()' …` |
| `anwendung.rs`, Doc von `ordner_der_datei_zeigen` | „dritter Aufrufer" von `ordner_lesen` | die Zusage ohne Ordnungszahl, dazu `grep -rn '\.ordner_lesen(' crates/krk-ui/src` |
| `anwendung.rs`, Doc von `fokus_bei` | „die fuenf uebrigen Aufrufer" | „die uebrigen Aufrufer" |
| `kommandos/mod.rs` | „einer ihrer drei Bestandteile" | „einer ihrer Bestandteile", Zeiger auf den Abschnitt, der sie aufzählt |
| `menue.rs`, Zählprobe | zweite Zusicherung auf 9 Gesamtaufrufe | gestrichen; sie sagte kein Kriterium zu und war der Stellvertreter, den `quellbaum` ausschließt |

## Jedes eingesetzte Kommando ist am Baum geprüft

- `grep -nE '\b(self|selbst)\.fokus\(\)' crates/krk-ui/src/appkit/anwendung.rs` → 9 Zeilen, davon 3 Prosa, also 6 Aufrufe. Der Datensatz sagte sechs; die Zahl ist deshalb trotzdem nicht eingesetzt worden, weil sie an der Schreibweise des Empfängers hängt und zweimal an derselben Blindheit bestätigt wurde.
- `grep -rn '\.ordner_lesen(' crates/krk-ui/src` → 11 Aufrufstellen. Der Datensatz sagte zehn, der Kommentar drei.
- `grep -n 'self\.lage()' crates/krk-ui/src/appkit/anwendung.rs` → 5 Zeilen, vier Abnehmer (`eintrag_pruefen` fragt in zwei Zweigen).
- `awk` über `zulaessigkeit.rs`: die Tafel deckt acht Achtel × acht Stellvertreter × sechs Fokuswerte. „280" war zweimal überholt.
- `grep -rn 'textmerkmale::' crates/krk-ui/src` → Editor und Vorschau, letztere an dreizehn Stellen.

## Fünf Behauptungen von Datensätzen sind am Baum gemessen und teilweise widerlegt

Keine übertrieb; drei waren durch spätere Runden bereits ganz oder halb erledigt.

1. `260829-0051` nennt sieben Funktionen in `operationen.rs` ohne `#[must_use]`. Am `2fa1d0e` tragen **alle sieben** die Marke; es gibt in der Datei keine Funktion dieser Bauart mehr ohne sie. Kein Code geändert.
2. `260831-1334` nennt vier `kommando_ausfuehren` ohne Marke und drei nackte Aufrufstellen. Am `2fa1d0e` fehlte sie an **einer** (`anwendung.rs`), und **eine** Aufrufstelle war nackt (der Melder der Bereichsleiste).
3. `260813-1420` nennt in `menue.rs` „die Tafel aus 140 Faellen". Der Baum sagte 280 — die Zahl war zwischenzeitlich einmal nachgezogen und ein zweites Mal falsch geworden.
4. `260813-1258_o_die-versionszahlprobe…` nennt `xtask/src/release.rs` als Ort der wörtlichen Versionszahl, ihr Schwesterdatensatz `xtask/src/bundle.rs:587`. `grep -rn 1.7.2 xtask/src` trifft am `2fa1d0e` **keine** von beiden, sondern drei Stellen in `xtask/src/beglaubigung.rs`. Deshalb steht im Doc-Kommentar das Kommando und keine Dateiliste.
5. `260812-1529` nennt für `angezeigtedatei.rs` acht geprüfte von zwölf erreichbaren Lagen. Nachgezählt: acht geprüft, eine neunte in der Nachbarprobe, drei in gar keiner. Die Tafel prüft jetzt alle zwölf.

## Am SDK nachgelesen statt geglaubt

Drei Untergrenzen-Befunde verlangten den Blick ins SDK, und alle drei halten:

- `NSTitlebarAccessoryViewController.h:23`: „For applications linked on Mac OS 10.11 or later, NSLayoutAttributeLeft is also supported".
- `NSSegmentedControl.h`: Klassendeklaration `:53` ohne `API_AVAILABLE`; `indexOfSelectedItem` `:103` mit `macos(10.4)`, was 10.0 ausschließt; `segmentStyle` `:91` mit 10.5; `segmentedControlWithLabels:…` `:130` mit 10.12.
- `NSApplication.h:202` (`keyWindow`) und `usr/include/objc/NSObject.h:17` (`isEqual:`) tragen keine Angabe und fallen unter den Pauschalsatz.

## Was der Auftrag ausdrücklich nicht erlaubte

Zwei Datensätze verlangen eine Änderung außerhalb `crates/krk-ui/` und bleiben deshalb offen: `260812-1526` (Kommentar in `resources/default-keymap.toml`) und `260813-0420` (zwei `[[funktion]]`-Blöcke in derselben Datei, Domain `data`). Beide gehören dem `ontocoder`.

Ein dritter, `260812-0801`, sieht Weg 2 „den Namen als eckigen Verweis schreiben" vor. Das ist in dieser Kiste nicht gangbar: `krk-ui` hat kein Bibliotheksziel, rustdoc löst `crate::appkit::…` nicht auf, und `aufteilung::rahmenfarbe` ist außerdem privat. Gefahren ist Weg 1.

## Neuer Datensatz

`shared/decisions/260906-0202_o_werden-defektdatensaetze-ueber-eingefrorene-spec-und-plantexte-geschlossen-oder-bleiben-sie-offen.md` — neun Datensätze dieser Kiste sind richtig und nicht behebbar, weil sie einen freigegebenen Text einer geschlossenen Runde beschreiben. Vier Möglichkeiten, keine gewählt.
