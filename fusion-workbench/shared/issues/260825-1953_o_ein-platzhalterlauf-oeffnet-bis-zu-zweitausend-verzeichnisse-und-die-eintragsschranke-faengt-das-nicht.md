# Ein Platzhalterlauf öffnet bis zu zweitausend Verzeichnisse, und die Eintragsschranke fängt das nicht

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Zeile `**Decidability:**`, Schritt 5, Schritt 9); `shared/decisions/260825-1725_a_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md` (Möglichkeit 1, Cons-Liste); `crates/krk-core/src/leseprofil/bausteine.rs` (`Lauf::gestreut_lesen`); `crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_EINTRAEGE`)

---

## Was ist

Die Zeile `**Decidability:**` des Plans setzt die **gelesenen Einträge** an die Stelle der
geöffneten Verzeichnisse: `HOECHSTENS_EINTRAEGE` soll die Arbeit dort begrenzen, wo
`HOECHSTENS_LESELAEUFE` es nicht mehr kann. Das trägt für die Menge der Daten, die eine
Sammlung anhäuft, und **nicht** für die Zahl der Systemaufrufe, die sie dafür macht.

Die zwei Größen fallen genau dann auseinander, wenn die getroffenen Ordner wenig oder nichts
enthalten. `ordner = "*"` über einem Ordner mit tausend leeren Unterordnern öffnet tausend
Verzeichnisse und sammelt null Einträge; die Schranke greift nie. Dasselbe gilt für
`ordner = "*/issues"`, wo es den Speicher `issues` in den meisten Runden nicht gibt: jeder
Fehlschlag kostet einen Auflösungsversuch und liefert keinen Eintrag.

## Was die Obergrenze wirklich ist

Sie ist da, aber sie steht an einer anderen Stelle als gedacht: der Ordner **vor** dem
Platzhalter wird selbst mit `HOECHSTENS_EINTRAEGE` gelesen, also gibt es höchstens 2.000
Treffer je Platzhalterzeile. Je Zusammenfassung sind bis zu elf Sammlungen buchbar (zwölf
Leseläufe minus den einen für den gemeinsamen Ordner davor), und damit steht als Schranke:

- rund **22.000 Verzeichnisöffnungen** je Zusammenfassung, gegen eine Zusage von zwölf
  Leseläufen,
- dazu je Treffer ein `canonicalize`.

Gemessen an der Werkbank dieses Vorhabens sind es heute 19 Öffnungen je Zeile und damit
nichts. Die Zahl ist keine Aussage über den Alltag, sondern über die Schranke, und die Zusage
C2.8 spricht über die Schranke: ein Muster in `readers.toml` darf die Vorschau nicht anhalten
können.

## Warum das kein Einwand gegen die Bauart ist

Der Entscheid vom 260825-1740 nennt den Preis ausdrücklich in seiner Cons-Liste: „Die Zahl der
Systemaufrufe ist damit nicht mehr aus der Zahl der Läufe abzulesen, sondern erst aus Lauf und
Bestand." Der Datensatz hier zieht daraus die Zahl, die dort nicht steht, und nichts weiter.
Der Modulkopf von `leseprofil::bausteine` schreibt die Eigenschaft seit Schritt 5 aus.

## Was zu tun wäre

Nichts, bevor gemessen ist. Schritt 9 des Plans misst die Kosten an der wirklichen Werkbank;
diese Frage braucht die zweite Messung daneben, nämlich an einem Ordner mit vielen leeren
Unterordnern.

Erst danach ist zu entscheiden, und die Möglichkeiten liegen auf der Hand:

1. Es bleibt, wie es ist. 2.000 Öffnungen sind auf einem Gerät mit APFS keine spürbare Zeit,
   und ein Ordner mit tausend Unterordnern, für den jemand ein Profil schreibt, ist ein
   gedachter und kein wirklicher Fall.
2. Eine eigene Schranke auf die **Treffer** eines Platzhalterlaufs, also eine sechste Zahl im
   Haushalt. Sie wäre vom Bestand unabhängig wie die Eintragsschranke und begrenzte genau die
   Größe, die heute unbegrenzt ist.
3. Die Treffer zählen gegen `HOECHSTENS_EINTRAEGE` mit, auch die, die nichts liefern. Das
   braucht keine neue Zahl, macht aber die Bedeutung der vorhandenen zweideutig.

## Status

Offen und ausdrücklich ungemessen. Kein Bau hängt daran: die Zusagen C6.1 bis C6.9 sind in
ihrer heutigen Fassung gehalten, und die Proben zählen sie nach.
