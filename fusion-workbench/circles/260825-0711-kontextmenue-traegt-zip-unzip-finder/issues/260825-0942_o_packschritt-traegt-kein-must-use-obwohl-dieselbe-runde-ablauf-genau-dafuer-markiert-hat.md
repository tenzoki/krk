`Packschritt` traegt kein `#[must_use]`, obwohl dieselbe Runde `Ablauf` genau dafuer markiert hat

---

Diese Runde hat `Ablauf` mit `#[must_use]` am Typ versehen und die Begruendung danebengeschrieben: ein fallen gelassenes `Abgebrochen` bliebe unbemerkt, der Lauf liefe ueber die abgebrochene Stelle hinaus weiter. Der im selben Zug neu angelegte `Packschritt` traegt dieselbe Last — er unterscheidet `Weiter`, `Abgebrochen` und `ArchivHin` — und die Marke nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-core/src/operation/mod.rs:113-128` — `Ablauf` mit `#[must_use]` am Typ, samt Begruendung: "Die Marke am Typ deckt jede Stelle, die ihn heute zurueckgibt, und jede, die es einmal tun wird."
- `crates/krk-core/src/operation/zippen.rs:60-74` — `Packschritt`, ohne Marke. Zurueckgegeben von `quellen_packen`, `eintrag_packen`, `datei_packen`, `ordner_packen` und `verknuepfung_packen`.

## Was heute gilt

Alle fuenf Rueckgaben werden ausgewertet; der Baum ist an dieser Stelle in Ordnung. Der Befund gilt der Zusage und nicht dem Stand: `CLAUDE.md` haelt fest, dass ein Rueckgabewert, dessen stilles Fallenlassen unbemerkt bliebe, in diesem Vorhaben `#[must_use]` bekommt, und `Packschritt` ist der Wert, der ueber den Abbruch des Packlaufs entscheidet. Ein `ArchivHin`, das jemand fallen liesse, liefe weiter in ein Archiv, das nicht mehr zu schreiben ist.

## Vorschlag

`#[must_use]` an `Packschritt`, mit derselben Begruendung wie an `Ablauf` und mit einem Verweis darauf, damit die zwei Marken als ein Paar lesbar bleiben.

`Zielentscheid` (`crates/krk-core/src/operation/mod.rs:131`) traegt die Marke ebenfalls nicht. Der Wert ist aelter als diese Runde, und dieselbe Ueberlegung gilt fuer ihn: er entscheidet, ob ueberhaupt geschrieben wird.

## Umfang

`krk-core`, `operation/zippen.rs` und `operation/mod.rs`.
