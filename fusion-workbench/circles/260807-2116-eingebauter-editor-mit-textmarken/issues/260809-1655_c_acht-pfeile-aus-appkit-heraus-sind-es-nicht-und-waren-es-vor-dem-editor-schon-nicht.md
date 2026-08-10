# „Acht Pfeile" aus `appkit` heraus sind es nicht, und waren es vor dem Editor schon nicht

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/appkit/mod.rs:74-84`
**Cross-references:** S16

---

## Der Befund

S16 hat den Satz im Modulkopf von sieben auf acht Pfeile gezogen:

> **Acht Pfeile** führen aus diesem Verzeichnis heraus, und alle acht tragen nur
> gewöhnliche Rust-Werte: [`bildtakt`] gibt `crate::messmodus` …, [`tabelle`]
> hält das Tabmodell aus `crate::tabs` und rechnet mit `crate::kommandos`,
> [`aufteilung`] rechnet die Breiten mit `crate::fenstermodell`,
> [`belegungsansicht`] hält die Arbeitskopie … , [`editor`] hält den Stand aus
> `crate::editormodell`, und [`fsevents`] wie [`volumes`] reichen Pfade an
> `crate::auffrischung`.

Gezählt über die `use crate::`-Zeilen des Verzeichnisses sind es mehr, und drei
der genannten stimmen nicht:

- **`anwendung.rs` kommt in der Aufzählung gar nicht vor** und trägt allein acht
  Pfeile: `auffrischung`, `belegungsmodell`, `fenstermodell`, `kommandos`,
  `leistenmodell`, `messmodus`, `tabs` — dazu `kommandos::fokus` und
  `kommandos::operationen` als zwei getrennte Wege.
- **`leiste` → `leistenmodell`**, **`vorschau` → `vorschaumodell`** und
  **`zwischenablage` → `vorschaumodell`** fehlen ebenfalls.
- **`volumes` reicht keine Pfade an `auffrischung`**, sondern zieht
  `leistenmodell::Ort` (`volumes.rs:53`). **`fsevents` trägt überhaupt keine
  `use crate::`-Zeile.** Beide sind namentlich als die zwei Pfeile nach
  `auffrischung` genannt.

Die Zählung war schon vor dieser Runde falsch: bei sieben fehlten dieselben
Stellen. S16 hat sie fortgeschrieben, ohne sie nachzurechnen.

## Warum das zählt

Der Satz behauptet Vollständigkeit („alle acht"), und die Zusage dahinter ist
die Architekturgrenze, an der dieses Projekt hängt: kein `objc2`-Wert verlässt
`appkit/`. Die Zusage selbst hält — jeder der genannten und der ungenannten
Pfeile trägt gewöhnliche Rust-Werte. Falsch ist die **Zählung**, die sie
belegen soll. Ein Leser, der die Grenze nachprüfen will, prüft acht Stellen und
hält das Verzeichnis danach für durchgesehen.

Kein Bau und keine Probe fängt das: es ist eine Zahl in Prosa.

## Vorschlag

Die Zahl fällt weg, und der Satz sagt die Regel statt der Zählung:

> Jeder Pfeil aus diesem Verzeichnis heraus trägt nur gewöhnliche Rust-Werte;
> keines der Ziele nennt eine `objc2`-Kiste.

Die Aufzählung der wichtigsten Ziele kann stehen bleiben, dann aber ohne den
Anspruch, alle zu sein. Eine Zahl, die niemand messen kann, veraltet auf
demselben Weg wie die Aufstellung der offenen Fragen, die aus `CLAUDE.md`
deshalb entfernt wurde.

Wer die Zahl behalten will, braucht eine Probe, die sie erhebt — etwa ein
`grep -c '^use crate::'` über `crates/krk-ui/src/appkit/` in einem Abnahmeschritt.
Dann ist sie belegt und nicht behauptet.

Gemeldet von: `coderev`, Durchsicht Turn 2.

---
Resolved: Der Vorschlag ist umgesetzt. Der Modulkopf von
`crates/krk-ui/src/appkit/mod.rs` sagt die Regel statt der Zählung: „Jeder Weg
aus diesem Verzeichnis heraus trägt nur gewöhnliche Rust-Werte; keines der Ziele
nennt eine `objc2`-Kiste." Die Zahl fällt weg, und der Absatz sagt, warum sie
weggefallen ist, samt dem Verweis auf diesen Datensatz und samt dem Kommando,
mit dem sich eine Zahl belegen ließe (`grep -rn 'use crate::'
crates/krk-ui/src/appkit/`).

Die Aufzählung bleibt stehen, jetzt ohne den Anspruch, alle zu sein, und mit den
drei berichtigten Stellen: `anwendung` ist aufgenommen und trägt seine neun
Ziele namentlich (`auffrischung`, `belegungsmodell`, `editormodell`,
`fenstermodell`, `fenstertitel`, `kommandos::fokus` und `kommandos::operationen`
getrennt, `leistenmodell`, `messmodus`, `tabs`); `leiste`, `vorschau` und
`zwischenablage` sind aufgenommen; `volumes` steht mit
`crate::leistenmodell::Ort` und nicht mit `auffrischung`.

**Ein vierter Fehler stand nicht im Datensatz und ist mitberichtigt:
`bildtakt` nennt `crate::messmodus` nicht.** Es trägt gar keine
`use crate::`-Zeile, sondern nimmt beim Einrichten eine gewöhnliche Rust-Senke
entgegen und meldet ihr Rate und Zeitpunkte, genau wie `fsevents` seine Pfade.
Der Modulkopf unterscheidet deshalb jetzt zwei Lesarten ausdrücklich: der
ASCII-Überblick oben zeichnet, **wohin Werte fließen**, eine `use crate::`-Zeile
sagt, welches Modul einen Nachbarn draußen **nennt**. Beide fallen nicht
zusammen, und `bildtakt` wie `fsevents` stehen im Überblick mit einem Pfeil und
in der Aufzählung nicht. Der Überblick ist unverändert, weil er als Wertefluss
richtig ist.

Erhoben am 260810-0918 über `grep -rn 'use crate::'
crates/krk-ui/src/appkit/`: 24 Zeilen in 9 der 22 Dateien, 11 verschiedene
Zielmodule. Die Zahlen stehen ausdrücklich **nicht** im Modulkopf, weil sie
dort niemand hält.

`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden
mit 0. `cargo doc -p krk-ui --no-deps --document-private-items` läuft durch, und
keine der 23 Warnungen betrifft `appkit/mod.rs`: die Verweise des neuen Absatzes
lösen auf.
