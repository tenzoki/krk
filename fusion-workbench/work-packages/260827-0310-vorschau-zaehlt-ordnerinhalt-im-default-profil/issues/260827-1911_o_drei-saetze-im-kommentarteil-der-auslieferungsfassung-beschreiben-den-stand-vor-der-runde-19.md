Drei Sätze im Kommentarteil der Auslieferungsfassung beschreiben den Stand vor der Runde 19

---

`resources/default-readers.toml` trägt seit `c072de7` einen Abschnitt „Das eingebaute Default-Profil", und drei ältere Sätze derselben Datei sagen weiterhin, ohne Profiltreffer bleibe die Metadatenanzeige, wie sie war, oder nennen den unbekannten `typ`-Wert nicht unter dem, was die ganze Datei kostet. C3.9 und C3.10 sagen zu, dass ein Nutzer, der allein diese Datei liest, die richtige Auskunft bekommt; drei Sätze darin geben ihm die alte.

---

**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Domain:** code
**Tree state:** `d444879`
**Affected:** `resources/default-readers.toml:18-19`, `:43-47`, `:229`
**Cross-references:** `planning/260827-0646_*_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (C3.6, C3.9, C3.10); `planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` (Schritt 6); `history/260827-1754-ontocoder-schritt-6-der-kommentarteil-der-auslieferungsfassung.md` (Nebenbefund des Ontocoders, nennt die ersten zwei Stellen); `reviews/260827-1911-coderev-durchsicht-runde-19-default-profil-zaehlzeilen.md` (F1); `crates/krk-core/src/leseprofil/datei.rs:49-57` (der Modulkopf, der dieselbe Reichweite für `typ` und `versteckt` schon nachgezogen hat)

## Der Befund

Drei Stellen, alle im Kommentarteil, keine in einem `[[profil]]`- oder `[[profil.zeile]]`-Block:

1. `:18-19`, Einleitung: „Trifft keines zu, bleibt die Metadatenanzeige, wie sie war." Seit `5e506e6` treten unter die sechs Metadatenangaben die drei Zählzeilen des Default-Profils (`crates/krk-ui/src/appkit/vorschau.rs:1414`).
2. `:229`, „Welches Profil gewinnt", Schritt 3: „Hat auch das nicht getroffen: die gewohnte Metadatenanzeige." Der Gewinner ist seit `bf3a91d` das eingebaute Default-Profil (`crates/krk-core/src/leseprofil/bausteine.rs:300-305`), und der Abschnitt sieben Zeilen darunter (`:236-252`) sagt genau das.
3. `:43-47`, „Was ein Schreibfehler kostet", erste Reichweite: nennt „ein Wert für `zeigt`, den es nicht gibt", nicht aber den unbekannten Wert für `typ` und den Nicht-Wahrheitswert für `versteckt`, die seit `9f91f92` in dieselbe Reichweite fallen (C3.6; belegt von `ein_unbekannter_typ_oder_ein_nicht_wahrheitswert_fuer_versteckt_kostet_die_ganze_datei` in `crates/krk-core/tests/leseprofil.rs`). Der Modulkopf von `datei.rs:49-57` hat die Aufzählung nachgezogen, die Auslieferungsfassung nicht.

Der Nebenbefund des Ontocoders aus Schritt 6 nennt die Stellen 1 und 2; die dritte kommt mit dieser Durchsicht dazu.

## Warum ein Datensatz und keine Notiz

C3.10 verlangt, dass ein Nutzer, der die drei Zählzeilen sieht und ihren Block sucht, „an der Stelle die Auskunft statt des Blocks" findet. Er findet sie — und zweihundert Zeilen davor das Gegenteil. Für einen Nutzer, der KRK schon gestartet hat, ist die Datei ohnehin eingefroren (der Plan nennt das unter Risiken); für jeden, der sie neu bekommt, ist sie die einzige Auskunft, die er ohne den Quellbaum hat.

## Was zu tun ist

Executor `ontocoder`, Kommentarzeilen und nichts sonst. Stelle 1 und 2 auf den neuen Stand bringen und dabei auf den Abschnitt „Das eingebaute Default-Profil" verweisen statt ihn zu wiederholen; Stelle 3 um `typ` und `versteckt` ergänzen, in derselben Form wie `zeigt`. Kein Block wird angefasst, und die Probe `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` (`crates/krk-core/src/ablage/leseprofile.rs`) bleibt grün, weil sie hinter dem ersten `#` abschneidet. Die Probe `die_auslieferungsfassung_besteht_die_eigene_pruefung` in derselben Datei hält daneben, dass die Datei TOML bleibt.

## Schließbedingung

`grep -n 'bleibt die Metadatenanzeige\|gewohnte Metadatenanzeige' resources/default-readers.toml` liefert nichts, die erste Reichweite unter „Was ein Schreibfehler kostet" nennt `typ` und `versteckt`, und `make check` ist grün.
