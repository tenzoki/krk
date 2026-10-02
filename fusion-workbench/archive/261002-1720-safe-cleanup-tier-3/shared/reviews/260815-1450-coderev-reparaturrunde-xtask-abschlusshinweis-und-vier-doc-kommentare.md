# Durchsicht: Reparaturrunde — Abschlusshinweis in xtask und vier Doc-Kommentare

**Sender:** coderev
**Reviewed-range:** `cd0b5b7..093a6f4`
**Not-opened:** keine

## Zusammenfassung

Drei Commits, acht Codedateien. Die vier Doc-Kommentar-Änderungen sind sachlich richtig und
am SDK beziehungsweise an der Belegung gegengeprüft; keine Zahl steht danach falsch da. Der
neue ausführbare Code in `xtask` erfüllt seinen Auftrag, hat aber zwei inhaltliche Lücken in
der Meldung selbst und zwei Proben, die weniger tragen als ihre Doc-Kommentare zusagen.
Dazu eine Ursache, die die Runde benennt und nicht weiterträgt.

## Zahlen

| Schwere | Anzahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 3 |
| Niedrig | 2 |

Fünf Datensätze abgelegt, alle unter `shared/issues/`, alle `_o_`.

## Was gegengeprüft ist und hält

**Die Zahlen 84 und 78** (`ea5f23e`). Dreifach nachgezählt und untereinander stimmig:
`grep -c '^id = ' resources/default-keymap.toml` → 84,
`grep -c '^gehalten_von = '` → 6, die Aufzählung `Kommando` → 78 Varianten. Die Ableitung
84 − 6 = 78 ist keine Rechnung auf gut Glück: die Belegung führt kein Feld `kommando`, und
keine Funktion trägt beides oder keines von beiden — nachgeprüft mit einem `awk` über alle
84 Blöcke. Die Probe `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`
(`belegungsausgabe.rs:751-754`) hält `mit_kommando == Kommando::KENNUNGEN.len()`.

Neun Stellen geändert, drei in `menue.rs`, sechs in `belegungsausgabe.rs`. Danach findet
`grep -n '\b79\b\|\b73\b'` in beiden Dateien keine Fundstelle dieser Bedeutung, und im
übrigen Baum steht keine.

**Die 10.7 für `NSLayoutManager`** (`a7253c2`). Am SDK dieses Geräts nachgelesen:
`NSLayoutManager.h:65` trägt `API_AVAILABLE(macos(10.7), ios(7.0), tvos(9.0))`. Die vier
Klassen, die in `nummernspalte.rs` als 10.0er stehen bleiben, sind mitgeprüft:
`NSRulerView`, `NSTextContainer` und `NSClipView` tragen im SDK gar keine Angabe,
`NSTextStorage` trägt ausdrücklich `macos(10.0)`. Die berichtigte Aufzählung stimmt
vollständig, nicht nur an der einen Stelle.

**Der Folgesatz in `textmerkmale.rs:64-68`.** Der alte Satz zählte im Präsens fremde
Modulköpfe und wäre durch die eigene Berichtigung falsch geworden. Der neue begründet die
10.7 aus der Lage der Klasse in der Aufzählung dieser Datei selbst. Er hat sich keine neue
Art zu veralten eingehandelt, die schlimmer wäre: die Aussage „zwischen lauter 10.0ern"
handelt von den fünf Klassen, die eine Zeile darüber stehen, und veraltet nur zugleich mit
der Zeile, die sie beschreibt — der billigste Fehlermodus, den ein solcher Satz haben kann.
Der zweite Halbsatz steht im Perfekt und veraltet nicht.

`vorschau.rs:137-139` sagt dasselbe im Präteritum („wie zwei aeltere Modulkoepfe … es
nannten") und ist mit Recht unangetastet geblieben; die Zahl zwei benennt den Bestand des
Datensatzes `260812-1558` und keinen Gegenwartszustand.

**Die Freihaltung von `release`, erste Hälfte.** `release::ausfuehren`
(`release.rs:155-191`) geht nachweislich nicht durch `bundle::bauen` — es fährt
`bundle::vorbereiten`, `bundle::uebersetzen` je Ziel, `zusammenfuegen` und
`vorlage.zusammensetzen` einzeln. `bundle::bauen` hat genau zwei Rufer, `main.rs:140` und
`messen.rs:45`. Der strukturelle Teil der Begründung stimmt.

**Die Randfälle von `lipo_name`.** Leere Zeichenkette, ganzes Ziel-Tripel, fremde
Architektur und ein Name, der schon der von `lipo` ist: alle vier reicht die Funktion durch,
und alle vier stehen in einer Probe. Die Bedingung `rest.starts_with('-')` (`release.rs:115`)
ist es, die das Tripel und `aarch` von `aarch64` trennt.

**`std::env::consts::ARCH` als Quelle der Architektur** (`main.rs:156`). Kein Befund. Es ist
die Architektur des `xtask`-Prozesses, nicht des gebauten Bündels; beide löst `cargo` aus
demselben Host auf, `bundle::bauen` übersetzt ohne Ziel-Tripel (`bundle.rs:145`), und auch
unter Rosetta oder mit gesetztem `CARGO_BUILD_TARGET` fallen die beiden zusammen.

**`Gebaut` trägt die `Identitaet`** (`bundle.rs:132-137`). Der richtige Ort. `Gebaut` ist
die Beschreibung dessen, was ein Bau hinterlassen hat, und womit signiert wurde, gehört
dazu — dieselbe Begründung, mit der der Binärpfad dort steht und nicht zusammengesetzt wird.
`messen.rs` bekommt das Feld mit und liest es nicht; das ist kein Zustand, der durch die
Schichten wandert, sondern ein Feld, das ein Rufer nicht braucht.

## Befunde

### Was die Meldung sagt (2 mittel)

**M1 — Der Auffangzweig erklärt jede Nicht-Developer-ID zur Entwicklungsidentität.**
`sign.rs:178-183`. Die Unterscheidung ist disjunkt, aber nicht vollständig: der `else`-Zweig
behauptet eine positive Einordnung für jeden Namen, der nicht mit `Developer ID Application`
beginnt. Drei Wege führen dort hinein, obwohl keine Entwicklungsidentität vorliegt — eine
über `KRK_SIGN_IDENTITY` als Teilzeichenfolge oder SHA-1-Abdruck gewählte Developer-ID
(`codesign` nimmt beide Formen an, `aus_umgebung` prüft nur die Nichtleere,
`sign.rs:237-244`) und eine `Apple Distribution:`- oder
`3rd Party Mac Developer Application:`-Identität, die die dritte Stufe von `bestimmen`
als „einzige gültige" aufliest. Das ist die Falle, die
`shared/issues/260812-1628` ausdrücklich benennt, nur durch eine Tür, die die Runde nicht
geprüft hat. Abschwächend: die Sachaussage über Gatekeeper trifft in allen Fällen zu, denn
`bundle` beglaubigt in keinem. Falsch sind die Einordnung und „bleibt auf dieser Maschine".
→ `shared/issues/260815-1444_o_…`

**M2 — Der Developer-ID-Zweig nennt nur die fehlende Beglaubigung.** `sign.rs:172-177`.
`bundle` signiert über `sign::signieren` ohne `--options runtime`; `signieren_gehaertet`
ruft nur `release`. Ein Nutzer, der dem Satz „Beglaubigt ist es nicht: …" folgt und dieses
Bündel selbst einreicht, bekommt von `notarytool` eine Ablehnung wegen der fehlenden
gehärteten Laufzeitumgebung, von der der Hinweis nichts gesagt hat. Auch der Schlusssatz
über `cargo xtask release` (`sign.rs:188-191`) nennt sie nicht, obwohl der Hilfetext in
`main.rs:73-76` es tut. Dieselbe Form wie der Ausgangsdefekt: eine wahre Aussage, deren
Folge sie nicht mitnennt, nur eine Station später.
→ `shared/issues/260815-1445_o_…`

### Was die Proben halten (1 mittel, 1 niedrig)

**M3 — Die Probe zum einen Rufer schreibt den Zweig nicht fest.** `sign.rs:625-643`. Ihr
Doc-Kommentar sagt zu, sie halte fest, „dass es genau einen gibt und wo er nicht liegt". Sie
liest drei der sechs Module unter `xtask/src/` (`version.rs` und `git.rs` fehlen) und zählt
in `main.rs` nur Vorkommen, nicht Stellung. Die Unterbefehlsverteilung steht in derselben
Datei, und `release` erreicht sie über `main.rs:161`; wer die drei Zeilen des Aufrufs vom
`"bundle"`- in den `"release"`-Zweig verschöbe, ließe die Probe grün. Die Freihaltung ist
für `bundle::bauen` strukturell und für den Ausgabeort eine Position in einem `match`.
→ `shared/issues/260815-1446_o_…`

**N1 — Die Paarungsprobe prüft Mitgliedschaft.** `release.rs:759-773`.
`ARCHITEKTUREN.contains(&lipo_name(rust_name))` geht auch dann durch, wenn `lipo_name` den
Namen durchgereicht hat, sofern der Rust-Name zufällig in `ARCHITEKTUREN` steht — für
`x86_64` ist das heute der Fall. Die Zusage des Doc-Kommentars („statt still durchgereicht
zu werden") trägt die Probe damit nicht. Für die zwei heutigen Ziele reicht das Vorhandene:
das `const`-assert auf die Länge trägt vollständig, und
`die_beiden_ziele_tragen_die_namen_die_lipo_dafuer_meldet` fängt jedes Vertauschen der zwei
Paare. Bei einem dritten Ziel öffnet sich die Lücke. Eine stellungsbezogene Schleife über
`ZIELE.zip(ARCHITEKTUREN)` schlösse sie in vier Zeilen.
→ `shared/issues/260815-1447_o_…`

### Was die Runde benennt und nicht weiterträgt (1 mittel)

**M4 — Die neun Zahlen stehen weiter unverankert.** Der Datensatz `260813-1345` schließt als
`_c_` mit dem Satz „Der Befund ist damit behoben und die Ursache nicht" — und die Ursache
bekommt keinen eigenen Datensatz, während der andere unerledigte Rest derselben Runde, der
Hilfetext, einen bekommt (`260815-1436_o_`). Ein geschlossener Datensatz fällt aus jeder
Suche nach offener Arbeit heraus. Die Familie ist belegt: viermal in vier Tagen, zuletzt
veralteten die Zielzahlen des Datensatzes zwischen Aufschreiben (82/76), Abgleich (83/77)
und Behebung (84/78). Der Ort für die Verankerung steht schon fest und rechnet die Zahl
bereits aus.
→ `shared/issues/260815-1448_o_…`

## Übergreifend

**Zweimal dasselbe Muster: die Prosa sagt mehr zu als der Code hält.** M3 und N1 sind
derselbe Befund an zwei Stellen — ein Doc-Kommentar, der die Reichweite einer Probe
beschreibt, und eine Probe, die weniger misst. In beiden Fällen ist der Kommentar präzise
genug, dass die Abweichung sichtbar wird; das spricht für die Schreibweise dieses Projekts
und gegen die beiden Rümpfe.

**Zweimal dieselbe Wurzel wie im Ausgangsdefekt: eine wahre Aussage ohne ihre Folge.** M1
und M2 sind nicht Fehler in der neuen Meldung, sondern Wiederholungen des Musters, gegen
das sie geschrieben wurde. M2 ist die deutlichere: der Hinweis nennt genau eine offene Sache
und macht damit den nächsten Schritt des Lesers falsch.

**Die Kopplung `ZIELE`/`ARCHITEKTUREN` ist die bessere Hälfte der Runde.** Statt eine dritte
Namensliste hinzustellen, liest `lipo_name` die zwei vorhandenen paarweise, und beide tragen
weiter ihre eigene Aufgabe. Das `const`-assert auf die Länge ist genau die Art Wächter, die
dieses Projekt sonst über nicht-erschöpfende Fallunterscheidungen setzt.

## Reihenfolge

Kein Auslieferungshindernis. Keiner der fünf Befunde ändert Verhalten, Bau oder Bündel.

1. **M2** zuerst — ein Halbsatz, und er verhindert einen fehlgeschlagenen Einreichversuch.
2. **M1** danach — ein Satz im Auffangzweig, drei Proben ziehen mit.
3. **M3** und **N1** zusammen — beides Proben, beides in derselben Sitzung.
4. **M4** ist eine Nutzerfrage vor einer Codeänderung: Zahlen verankern oder aus der Prosa
   nehmen, wie `CLAUDE.md` es getan hat.

---
Abgleich 260815-1812 (reconciler), nur Statusvermerk — keine Aussage dieser Durchsicht ist
geändert.

Von den fünf abgelegten Datensätzen sind zwei geschlossen und drei stehen offen:

| Datensatz | Stand am 260815-1812 | Beleg |
|---|---|---|
| `260815-1444` Weitergabehinweis erklärt jede Nicht-Developer-ID zur Entwicklungsidentität | `_c_` | `a46fd1f`; `xtask/src/sign.rs:194-198`, Proben `ein_name_ohne_developer_id_praefix_bekommt_keine_art_zugeschrieben` und `ein_sha1_abdruck_als_identitaet_bekommt_keine_art_zugeschrieben` |
| `260815-1445` Developer-ID-Zweig nennt die gehärtete Laufzeitumgebung nicht | `_c_` | `a46fd1f`; `xtask/src/sign.rs:199-208`, Probe `beide_faelle_nennen_die_fehlende_gehaertete_laufzeitumgebung` bindet über `include_str!("main.rs")` |
| `260815-1446` Probe zum einen Rufer liest drei von sechs Modulen | `_o_` | `xtask/src/sign.rs:701-707` liest weiter `release.rs`, `messen.rs`, `bundle.rs` |
| `260815-1447` Paarungsprobe prüft Mitgliedschaft statt Paarung | `_o_` | `xtask/src/release.rs:769` steht unverändert auf `ARCHITEKTUREN.contains(&lipo_name(rust_name))` |
| `260815-1448` die neun Zahlen stehen unverankert | `_o_` | keine Zusicherung auf 84/78 in `crates/krk-ui/src/belegungsausgabe.rs` |

Die Gegenprüfungen dieser Durchsicht halten: `xtask` fährt am 260815-1812 **98** Proben
(96 zur Zeit der Durchsicht plus die zwei, die `260815-1444` und `260815-1445` mitgebracht
haben), `cargo test --workspace` läuft grün, Exit 0. Die Zahlen 84 und 78 sind unabhängig
nachgezählt und stimmen.
