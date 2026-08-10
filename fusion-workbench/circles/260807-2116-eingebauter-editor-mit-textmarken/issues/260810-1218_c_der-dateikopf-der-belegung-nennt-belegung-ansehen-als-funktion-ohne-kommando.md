Der Dateikopf der Belegung nennt "belegung_ansehen" als Funktion ohne Kommando

---

`resources/default-keymap.toml:19-26` erklärt das Feld `gehalten_von` und schließt mit einem Beispiel:

```
#   gehalten_von     optional; wer den Tastendruck zustellt, nicht was er tut.
#                    Einziger Wert "menue": ein NSMenuItem traegt die
#                    Kombination als Kuerzel, der Ereignisabgriff fuehrt sie
#                    nicht aus, und die Antwortkette entscheidet, wer sie
#                    beantwortet. Eine solche Funktion bekommt nie ein
#                    Kommando. Ohne das Feld waere sie von einer Funktion
#                    nicht zu unterscheiden, deren Kommando ein spaeterer
#                    Schritt erst baut, wie es "belegung_ansehen" unten ist.
```

`belegung_ansehen` ist keine solche Funktion mehr. Sie trägt ein Kommando:

- `crates/krk-core/src/tasten/belegung.rs:409` — die Variante `Kommando::BelegungAnsehen`
- `crates/krk-core/src/tasten/belegung.rs:493` — die Zeile `(Kommando::BelegungAnsehen, "belegung_ansehen")` in `Kommando::KENNUNGEN`

`Funktion::kommando` (`belegung.rs:712-717`) schlägt über genau diese Tabelle nach und liefert für `belegung_ansehen` deshalb `Some(Kommando::BelegungAnsehen)`.

## Die Sorte, die das Beispiel meint, gibt es in der Datei nicht mehr

Geprüft am 260810-1218, über alle 71 Einträge der Datei gegen `Kommando::KENNUNGEN`:

| Sorte | Anzahl | Welche |
|-------|--------|--------|
| trägt ein Kommando | 65 | alle außer den sechs unten |
| `gehalten_von = "menue"`, trägt nie ein Kommando | 6 | `text_ausschneiden`, `text_kopieren`, `text_einfuegen`, `text_alles_auswaehlen`, `text_rueckgaengig`, `text_wiederholen` |
| `reserviert_fuer` gesetzt | 0 | — |
| **benannt, nicht zugestellt, ohne Kommando** | **0** | — |

Die 65 Kennungen entsprechen genau den 65 Varianten der Aufzählung `Kommando`, in beide Richtungen ohne Rest. Die Probe `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (`belegung.rs`) sichert die eine Richtung; die andere ist hier nachgezählt.

## Warum das mehr ist als ein veraltetes Beispiel

Das Beispiel trägt die Begründung, warum das Feld `gehalten_von` überhaupt existiert: ohne es wären zwei Sorten nicht auseinanderzuhalten. Die zweite Sorte ist leer, und damit steht die Begründung auf einem Fall, den die Datei nicht mehr vorführt. Wer den Kopf liest und `belegung_ansehen` unten nachsieht, findet dort eine Funktion mit Kommando und muss annehmen, entweder der Kopf oder seine eigene Lesart sei falsch.

Die Begründung selbst bleibt richtig — eine Funktion ohne Kommando ist ein Zustand, den ein späterer Schritt wieder herstellen kann. Was fehlt, ist ein Beispiel, das es noch gibt.

## Vorgeschlagene Behebung

Den Halbsatz `wie es "belegung_ansehen" unten ist` streichen und den Satz auf die Begründung ohne Beispiel stellen. Ein Ersatzbeispiel aus der Datei gibt es nicht; ein erfundenes wäre schlechter als keines.

## Zuständigkeit

`ontocoder`. Eine Kommentarzeile in einer TOML-Datei.

---

**Gefunden von:** ontorev, Durchsicht der Belegungsdatei 260810-1217
**Domain:** data
**Schwere:** Low
**Betroffen:** `resources/default-keymap.toml:25-26`
**Cross-references:** `circles/260807-2116-eingebauter-editor-mit-textmarken/reviews/260810-1217-ontorev-belegungsdatei-nach-den-drei-kommentarstellen.md` (der Bericht, aus dem dieser Defekt stammt)

---
Resolved: Der Halbsatz `wie es "belegung_ansehen" unten ist` ist aus `resources/default-keymap.toml:26` gestrichen. Der Satz steht jetzt auf der Begründung allein und sagt dazu, dass die Datei derzeit keinen Eintrag dieser Sorte trägt; das alte Beispiel ist als historische Notiz mit Datum 260810 erhalten, in derselben Form, die der Editor-Block und der Rückgängig-Block für ihre überholten Stände führen. Ein Ersatzbeispiel ist nicht erfunden.

Die Leere der zweiten Sorte ist vor der Änderung selbst nachgezählt, nicht aus dem Datensatz übernommen: über alle 71 Einträge der Datei gegen `Kommando::KENNUNGEN` tragen 65 ein Kommando und 6 `gehalten_von = "menue"`; die Menge der Einträge ohne Kommando ist genau die Menge der sechs vom Menü zugestellten, `reserviert_fuer` ist nirgends gesetzt, und die 65 Kennungen entsprechen den 65 Einträgen in beide Richtungen ohne Rest. `belegung_ansehen` trägt sein Kommando unverändert an den beiden Stellen, die dieser Datensatz nennt (`crates/krk-core/src/tasten/belegung.rs:409` und `:493`).

Keine Belegungszeile geändert (290 Nutzzeilen, Byte für Byte dieselben), keine Zahl im Dateikopf angefasst. `cargo test -p krk-core --lib tasten` und `cargo test --workspace` laufen grün.

Offen bleibt eine Lücke, die dieser Datensatz nicht schließt: der neue Satz behauptet den Bestand der Datei, und keine Probe hält ihn. Das ist dieselbe Lücke wie in `260810-1219_o_die-zwei-zahlen-im-kopf-der-belegungsdatei-wachsen-nicht-mit-ihr.md`, und die dort vorgeschlagene Probe deckt sie mit, wenn sie die Entsprechung zwischen den Kennungen und den Einträgen in beide Richtungen prüft statt nur die Zählstände.
