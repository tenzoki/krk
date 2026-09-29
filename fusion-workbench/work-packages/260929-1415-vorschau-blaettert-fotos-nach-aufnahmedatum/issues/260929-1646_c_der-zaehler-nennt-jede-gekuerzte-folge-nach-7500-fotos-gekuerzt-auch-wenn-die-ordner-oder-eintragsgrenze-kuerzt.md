Der Zähler nennt jede gekürzte Folge „nach 7.500 Fotos gekürzt“, auch wenn die Ordner- oder die Eintragsgrenze gekürzt hat
---
`Bildverzeichnis::ist_gekuerzt` wird an fünf Stellen gesetzt, von denen nur zwei die Fotogrenze sind. `bildzaehler_text` hängt bei jedem `gekuerzt` denselben Zusatz an: „(Folge nach 7.500 Fotos gekürzt)“. Eine Folge von 900 Fotos, die an der Gruppengrenze oder an der Eintragsgrenze eines Leselaufs gekürzt ist, zeigt „Bild 1 von 900 (Folge nach 7.500 Fotos gekürzt)“. Der Satz ist dann falsch, und der Nutzer sucht die Ursache an der falschen Grenze.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (Entscheidungen 3, 4 und 13); `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (C5.1)

## Befund

`crates/krk-core/src/leseprofil/bildfolge.rs`, `verzeichnis_erheben`, die Setzer von `gekuerzt`:

| Zeile | Anlass | Grenze |
|---|---|---|
| 289 | Leselauf des Jahresordners abgeschnitten | `HOECHSTENS_EINTRAEGE_JE_BILDORDNER` |
| 319 | vor der nächsten Gruppe: `gruppen() >= HOECHSTENS_BILDGRUPPEN` oder `fotos() >= HOECHSTENS_FOTOS` | Gruppen oder Fotos |
| 326 | `leselauf_nehmen` sagt nein | Leseläufe (`HOECHSTENS_BILDGRUPPEN + 1`) |
| 333 | Leselauf einer Gruppe abgeschnitten | `HOECHSTENS_EINTRAEGE_JE_BILDORDNER` |
| 348, 351 | `gruppe_nehmen` ohne Platz oder mit kleinerem Beitrag | Fotos |

`crates/krk-ui/src/appkit/statuszeile.rs:585`, `bildzaehler_text`: bei `gekuerzt` immer `"(Folge nach {HOECHSTENS_FOTOS} Fotos gekürzt)"`.

Die Eintragsgrenze ist kein Randfall: ein Kameraordner mit RAW und JPEG je Aufnahme erreicht 10.000 Einträge bei 5.000 Aufnahmen. Der Leselauf bricht dann in Lesereihenfolge des Verzeichnisses ab und nicht in der Reihenfolge der Folge; welche Fotos fehlen, hängt vom Dateisystem ab. Das ist nach Entscheidung 4 des Plans so gewollt („es wird nur geordnet, was gelesen ist“), der Zusatz nennt aber eine Grenze, die nicht gegriffen hat.

## Richtung

Die Kürzung trägt ihren Grund (etwa `Kuerzung::{Fotos, Ordner, Eintraege}` statt `bool`), und der Zähler nennt ihn; oder der Zusatz sagt allein „gekürzt“ ohne Zahl. `HowTo.md` (Zeile 692) und der Kopf von `resources/default-readers.toml` ziehen mit.

## Abnahme

- Eine Kernprobe: eine Folge aus 61 Monatsordnern mit je einem Foto ist gekürzt, und ihr Grund ist die Ordnergrenze.
- Eine Probe am Zähler: eine an der Ordner- oder Eintragsgrenze gekürzte Folge unter 7.500 Fotos nennt keine 7.500.

---
Resolved: die Kuerzung traegt ihren Grund (`bildfolge::Kuerzung` je `Grenze` Fotos, Ordner, Eintraege), `bildzaehler_text` nennt die gegriffenen Grenzen, HowTo.md zieht mit; Proben `eine_an_der_ordnergrenze_gekuerzte_folge_nennt_die_ordnergrenze` und `der_bildzaehler_nennt_die_grenze_die_gekuerzt_hat`.
