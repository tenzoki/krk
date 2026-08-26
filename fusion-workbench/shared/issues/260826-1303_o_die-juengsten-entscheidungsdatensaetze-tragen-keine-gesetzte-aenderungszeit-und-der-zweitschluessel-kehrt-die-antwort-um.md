# Die jüngsten Entscheidungsdatensätze tragen keine gesetzte Änderungszeit, und der Zweitschlüssel kehrt die Antwort um

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Medium
**Affected:** `crates/krk-core/tests/leseprofil.rs:1265-1269` (die Zusicherung), `:1067-1074` (der Bestand ohne gesetzte Zeit), `:1114-1129` (`geaendert_setzen` und sein Doc-Kommentar)
**Tree state:** `4a57028`
**Cross-references:** `crates/krk-core/src/leseprofil/bausteine.rs:635-644` (der Zweitschlüssel)

---

## Was ist

`werkbankgestalt` legt vier Verlaufsdateien an und setzt jeder ausdrücklich eine
Änderungszeit:

```rust
// tests/leseprofil.rs:1089-1101
let history = ordner.ordner("history");
for (nummer, name) in [ … ].into_iter().enumerate() {
    let pfad = schreiben(&history, name, &format!("# Verlauf {nummer}\n"));
    geaendert_setzen(&pfad, 1_700_000_000 + nummer as u64 * 60);
}
```

Der Doc-Kommentar von `geaendert_setzen` (`:1114-1119`) schreibt aus, warum:

> Ohne ihn hängt die Reihenfolge der jüngsten N daran, wie schnell die Probe
> läuft: vier Dateien nacheinander geschrieben tragen auf einem schnellen
> Dateisystem denselben Zeitpunkt, und die Probe prüfte dann den Zweitschlüssel
> statt der Sortierung.

Derselbe Helfer legt zwölf Zeilen darüber drei Entscheidungsdatensätze an, **ohne**
diesen Aufruf:

```rust
// tests/leseprofil.rs:1067-1074
let decisions = ordner.ordner("decisions");
for (name, inhalt) in [
    ("260823-2208_a_erste-frage.md",  "#   Erste Frage?\n\nText\n"),
    ("260824-0541_a_zweite-frage.md", "# Zweite Frage?\n"),
    ("260824-0600_a_dritte-frage.md", "# Dritte Frage?\n"),
] {
    schreiben(&decisions, name, inhalt);
}
```

Und genau auf deren Reihenfolge liegt eine Zusicherung:

```rust
// tests/leseprofil.rs:1265-1269 — juengste { ordner = "decisions", anzahl = 1 }
assert_eq!(
    werte[2].1,
    &Wert::Titel(vec!["Dritte Frage?".to_owned()]),
    "das Doppelkreuz und die Leerzeichen dahinter fallen weg"
);
```

## Warum der Zweitschlüssel die Antwort umkehrt

`bausteine.rs:639-644` sortiert absteigend nach Änderungszeit und bricht den
Gleichstand **aufsteigend nach Namen**:

```rust
kandidaten.sort_by(|links, rechts| {
    rechts.geaendert.cmp(&links.geaendert)
        .then_with(|| links.name.cmp(&rechts.name))
});
```

Tragen die drei Dateien denselben Zeitpunkt, gewinnt der **kleinste** Name, also
`260823-2208_a_erste-frage.md`. Der Wert wäre dann `Titel(["Erste Frage?"])` und
die Probe rot — nicht wegen eines Fehlers im Code, sondern weil das Dateisystem
schnell war. Der Zweitschlüssel dreht die erwartete Antwort um, statt sie
zufällig zu treffen.

Die zwei anderen Zusicherungen desselben Laufs hängen nicht daran: `werte[1]`
fragt mit `contains` (`:1256-1263`), `werte[3]` prüft den Platzhalter am leeren
Ordner (`:1270-1274`).

## Warum das nicht auf „läuft heute durch" hinausläuft

APFS führt Änderungszeiten in Nanosekunden, und drei `write`-Aufrufe treffen
selten dieselbe. Der Baum ist deshalb heute grün. Der Punkt ist die
Ungleichbehandlung: derselbe Helfer sichert die Gefahr für `history` ab und
benennt sie im Doc-Kommentar, lässt sie für `decisions` aber stehen — an der
einen Stelle, die als einzige die Sortierung selbst zusagt. Wer den Bestand
später erweitert, liest den Doc-Kommentar bei `history` und nimmt an, die Frage
sei geklärt.

## Was zu tun wäre

Die drei Entscheidungsdatensätze durch denselben Aufruf schicken wie die
Verlaufsdateien:

```rust
let pfad = schreiben(&decisions, name, inhalt);
geaendert_setzen(&pfad, 1_700_000_000 + nummer as u64 * 60);
```

Damit steht die erwartete Antwort `Dritte Frage?` auf einer gesetzten Zahl statt
auf der Geschwindigkeit des Dateisystems. Ob `runde()` (`:2173-2180`) dieselbe
Behandlung braucht, hängt daran, ob eine Probe je die Reihenfolge seiner drei
Entscheidungsdatensätze zusichert; heute tut es keine, und die Zeile gehört dann
dort nicht hin.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.
