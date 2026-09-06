# Analyst-Sitzung — 260906-0239: zweite Standanalyse nach der dritten Behebungsschleife

**Status:** Complete
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Auftrag:** Bestand neu erheben, dem Vorgängerbericht `260905-2307` gegenüberstellen, zwei bestrittene Nachmessungen prüfen, den Rest der fünf Empfehlungsgruppen beziffern, drei bis fünf Gruppen für die sieben verbleibenden Schleifen nennen samt dem, was keine Schleife abträgt, die offenen Nutzerfragen bündeln und das Untertreibungsmuster prüfen.
**Grenze:** lesend. Kein Code, keine Datensätze, kein Marker geändert. Geschrieben wurden allein der Analysebericht und diese Datei.

## Erzeugnis

`fusion-workbench/shared/analyses/260906-0239-der-boden-des-bestands.md`

## Baumstand

Alle Zahlen an `26dac51` gemessen, 2026-09-06T02:15:38+02:00, `main`, 15 Commits vor `origin/main`. Vergleichszahlen aus `28c4a47`, `8779a25`, `2fa1d0e` und `56e5c2d`, mit `git ls-tree` und `git grep` aus dem Commit gelesen.

## Verifikation

- `make check` — Exit 0, alle fünf Kommandos grün; der Baum trägt 1872 `#[test]`-Marken gegen 1841 bei Sitzungsbeginn, darunter die drei neuen in `xtask/src/werkbank.rs`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — Exit 0
- `cargo doc -p krk-core --no-deps` mit `--force-warn rustdoc::private_intra_doc_links` — 49 Meldungen, in einem eigenen `CARGO_TARGET_DIR`, danach entfernt
- Klassifikation aller 209 offenen Datensätze in zwei unabhängigen Läufen, je Datensatz vollständig gelesen; beide Summen gehen auf (110 und 99)

## Die Kernzahlen

| Größe | `28c4a47` | `8779a25` | `26dac51` |
|---|---|---|---|
| offene Defekte | 347 | 286 | 209 |
| geschlossene ohne Archiv | 479 | 543 | 623 |
| offene Fragen | 51 | 53 | 56 |
| Schließquote | 58 % | 65 % | 75 % |

144 Schließungen, sechs Neuablagen, davon drei in derselben Sitzung wieder geschlossen.

## Was die Analyse festhält

- **Die Klassen sind neu erhoben, nicht fortgerechnet.** V 15, E 53, S 33, W 10, B 32, C 38, L 28. Die erreichbaren Klassen sind von 192 auf 108 gefallen, die gesperrten von 94 auf 101 gestiegen.
- **Der Boden liegt bei 78 bis 101 von 209.** Die Spanne steckt ganz in Klasse E und ist dieselbe, die der Vorgängerbericht offengelassen hat.
- **Zur Fettform der Abschlussvermerke hat der Werkbanklauf sachlich recht und in den Zahlen nicht.** Nachgerechnet an seinem eigenen Commit: 691 geschlossen, 632 Konvention, 19 verschobener Doppelpunkt, 30 Fettform, 10 ohne Zeile. Der Modulkopf von `xtask/src/werkbank.rs`, den derselbe Lauf geschrieben hat, nennt mit „59 in drei Schreibweisen" genau diese Aufteilung; sein Bericht nennt eine andere.
- **Das Untertreibungsmuster trägt für Fundstellenzahlen und kehrt sich für Restmengen um.** Zwanzig Nachmessungen, davon zwölf untertreibend, fünf übertreibend durch Zeitablauf, drei exakt. Kein Fall von Übertreibung bei der Ablage.
- **Die Fläche wächst, während sie geräumt wird:** +1 570 Doc-Zeilen in drei Schleifen gegen +23 Zeilen, die ein Zählkommando statt einer Zahl führen.

## Abgelegte Datensätze

Keine. Fünf offene Fragen stehen im Berichtsabschnitt `## Offene Fragen`; drei davon sind auch schon im Vorgängerbericht als nicht abgelegt vermerkt und bleiben es.
