Drei Prosastellen der Ablage nennen die Zahl der Dateien falsch, und jedes bisherige Suchmuster musste sie uebersehen

---

`Datei::ALLE` fuehrt sieben Werte, zwei davon von Hand gepflegt. Drei Stellen in
`crates/krk-core/src/ablage/` sagen etwas anderes: `sperre.rs:3-4` („dieselben vier Dateien"),
`sperre.rs:40` („nicht auf den vier Nutzdateien") und `einstellungen.rs:1` („`settings.toml`:
die eine Ablagedatei, die der Nutzer von Hand pflegt").

**Der eigentliche Befund ist, dass keine der vier bisherigen Zaehlerhebungen sie finden
konnte**, und zwar aus drei verschiedenen Gruenden: die erste Stelle traegt ihr Wortpaar ueber
einen Zeilenumbruch verteilt und entgeht damit jeder zeilenweisen Suche, die zweite sagt
„Nutzdateien" statt „Dateien", die dritte sagt „eine" statt „vier".

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/ablage/sperre.rs:3-4`, `:40`,
`crates/krk-core/src/ablage/einstellungen.rs:1`
**Tree state:** `004ff72`
**Domain:** code

## Die drei Stellen

| Stelle | Wortlaut | Wahr ist |
|---|---|---|
| `sperre.rs:3-4` | „greifen zwei Prozesse auf dieselben **vier**\n Dateien … zu" | sieben |
| `sperre.rs:40` | „… und **nicht** auf den **vier Nutzdateien**" | sieben |
| `einstellungen.rs:1` | „`settings.toml`: die **eine** Ablagedatei, die der Nutzer von Hand pflegt (C11)" | zwei, seit der Runde 16 |

Die dritte widerspricht der Nachbardatei woertlich: `leseprofile.rs:1-2` sagt „`readers.toml`:
die **zweite** Ablagedatei, die der Nutzer von Hand pflegt", und `ablage/mod.rs:59-71` traegt
die Ueberschrift „Zwei der fuenf TOML-Dateien entstehen einmal". Sie steht in der ersten Zeile
ihres Moduls, also dort, wo ein Leser nach der Zustaendigkeit sucht.

Der Abschnitt `# Warum eine vierte Datei` darunter (`einstellungen.rs:11-13`) ist dagegen eine
Aussage ueber den damaligen Stand und traegt als solche weiter; er gehoert nicht mitgezogen.

## Warum keine der vier Erhebungen sie sehen konnte

- `circles/260813-2332-…/issues/260814-0912_*_neun-stellen-sprechen-weiter-von-vier-ablagedateien-es-sind-sechs.md`: neun Stellen, alle in `ablage/mod.rs`
- `circles/260813-2332-…/issues/260814-1002_*_die-erhebung-zu-vier-ablagedateien-nennt-neun-stellen-die-suche-liefert-mehr.md`: nennt sein Muster ausdruecklich, `vier Dateien|vier Ablagedateien|vier Lade- und Schreibmethoden` ueber `crates/`, siebzehn Zeilen
- `shared/issues/260816-2307_*_der-doc-kommentar-von-ablage-pfad-nennt-vier-dateien-die-aufzaehlung-fuehrt-sechs.md`: drei Stellen, alle in `ablage/mod.rs`
- `shared/issues/260821-1023_*_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`: sieben Stellen, `ablage/mod.rs` und `ablage/pfade.rs`
- `circles/260823-2208-…/issues/260824-1014_c_vierzehn-prosastellen-der-ablage-sagen-weiter-vier-…`: vierzehn Stellen, `ablage/mod.rs` und `tests/ablage.rs`

Nachgemessen am Baumstand `79dab20`, den die zweite Erhebung selbst nennt:

```sh
git grep -nE "vier Dateien|vier Ablagedateien|vier Lade- und Schreibmethoden" 79dab20 -- crates | wc -l
# 17 — dieselbe Zahl, die der Datensatz nennt

git grep -nE "vier Dateien|vier Ablagedateien" 79dab20 -- crates/krk-core/src/ablage/sperre.rs
# keine Zeile
```

Am heutigen Baum ebenso:

```sh
grep -rn "vier Dateien\|vier Ablagedateien" crates/krk-core/src/ablage/sperre.rs
# keine Zeile

tr '\n' ' ' < crates/krk-core/src/ablage/sperre.rs | grep -o "dieselben vier .\{0,20\}Dateien"
# dieselben vier //! Dateien
```

Das Wortpaar steht also da, und der Zeilenumbruch des Doc-Kommentars zwischen „vier" und
„Dateien" macht es fuer jede zeilenweise Suche unsichtbar. Die zweite und die dritte Stelle
tragen ein anderes Wort und waeren auch von einer zeilenuebergreifenden Suche nach diesem
Muster nicht erfasst worden.

## Was daraus folgt

CLAUDE.md fuehrt diese Lage schon einmal: „Wer eine Erhebung fährt, erweitert das Muster, bevor
er zählt" — dort ging es um Verweise in Kurzform, die fuenf Erhebungen entgangen sind
(`shared/issues/260810-1851_*`). Hier ist der blinde Fleck ein zweiter derselben Art. Wer den
naechsten Durchgang faehrt, sucht ueber `crates/krk-core/src/ablage/` als Ganzes, mit
zusammengezogenen Zeilen und ohne sich auf das Wort „Dateien" zu verlassen.

**Gefunden:** coderev, Vollbaum-Durchsicht von `crates/krk-core/src/{ablage,leseprofil}/` am
260826-1225.
