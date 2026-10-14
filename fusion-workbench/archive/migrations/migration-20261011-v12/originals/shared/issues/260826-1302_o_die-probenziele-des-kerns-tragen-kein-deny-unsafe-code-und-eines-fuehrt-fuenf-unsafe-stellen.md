Die Probenziele des Kerns tragen kein `deny(unsafe_code)`, und eines führt fünf `unsafe`-Stellen

---

`#![deny(unsafe_code)]` steht an der Kistenwurzel `crates/krk-core/src/lib.rs:1`. Jede Datei unmittelbar unter `crates/krk-core/tests/` ist eine **eigene** Kiste und erbt das Attribut nicht; keine der fünfzehn setzt es selbst. `crates/krk-core/tests/textkopien.rs:61-72` führt entsprechend fünf `unsafe`-Konstrukte ohne jede Ausnahmeerklärung — und die Zählprobe C4.5, die „genau zwei Stellen" zusagt, kann sie nicht sehen, weil sie nach `#![allow(unsafe_code)]` sucht und dort keines nötig ist.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/textkopien.rs:56-76`; `crates/krk-core/tests/baum.rs:56-84` (C4.5); `crates/krk-core/src/lib.rs:1`

## Die fünf Stellen

```rust
// crates/krk-core/tests/textkopien.rs:61-73
unsafe impl GlobalAlloc for Zaehlend {
    unsafe fn alloc(&self, anfrage: Layout) -> *mut u8 {
        …
        unsafe { System.alloc(anfrage) }
    }
    unsafe fn dealloc(&self, zeiger: *mut u8, anfrage: Layout) {
        unsafe { System.dealloc(zeiger, anfrage) }
    }
}
```

Sie sind sachlich einwandfrei: der Kommentar darüber (`:58-60`) begründet die Sicherheitsbedingung, beide Methoden reichen unverändert an `System` weiter, und ein zählender Allokator ist ohne `unsafe` nicht zu haben. **Der Befund ist nicht der Code, sondern die Buchführung.**

## Was `CLAUDE.md` und die Zählprobe sagen

`CLAUDE.md`: „`krk-core`, `krk-ui` und `krk-bench` tragen `#![deny(unsafe_code)]` an ihrer Kistenwurzel; die Ausnahme `#![allow(unsafe_code)]` steht nur in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`. **Der Bau erzwingt diese Grenze.**"

Der Bau erzwingt sie für `krk-core/src/`. Für `krk-core/tests/` erzwingt er nichts, und dort steht `unsafe`.

`baum.rs:65-83` zählt Dateien, die die **Zeile** `#![allow(unsafe_code)]` tragen, und findet genau zwei. Die Zahl stimmt und beantwortet eine andere Frage als die, die ein Leser des Probennamens vermutet: nicht „wo steht `unsafe`", sondern „wo ist die Sperre ausdrücklich geöffnet". Wo keine Sperre steht, braucht es keine Öffnung, und die Zählung schweigt.

Am 260826 nachgezählt (`grep -rn '\bunsafe\b' crates/krk-core/tests/`): fünf Treffer, alle in `textkopien.rs`, keiner in den übrigen vierzehn Dateien.

## Warum das kein Nebenschauplatz ist

Ein Probenziel ist gebauter Code dieses Projekts. Die Zusage „`deny(unsafe_code)` an der Kistenwurzel, zwei begründete Ausnahmen" ist eine der wenigen Grenzen dieses Baums, die überhaupt maschinell gehalten werden; dass fünfzehn Kisten daneben ohne sie laufen, gehört zu ihrer Auskunft dazu und steht heute nirgends.

## Richtung

Drei Möglichkeiten, in aufsteigendem Aufwand:

1. **Nichts am Code, eine Zeile an der Prosa.** `CLAUDE.md` und der Kopf von `baum.rs` sagen künftig, dass die Grenze für `src/` gilt und die Probenziele eigene Kisten ohne sie sind.
2. **`#![deny(unsafe_code)]` in die vierzehn Dateien ohne `unsafe`, `#![allow(unsafe_code)]` in `textkopien.rs`.** Dann trägt der Baum drei Ausnahmen, die Zählprobe zieht auf drei nach und benennt die dritte — und das `unsafe` steht wieder dort, wo eine Zusage es sichtbar macht. Kostet fünfzehn Zeilen und macht die Zahl in `CLAUDE.md` wieder wahr.
3. **Eine eigene Zählprobe über die Probenziele**, die zusagt, welche von ihnen `unsafe` führen dürfen. Genauer als 2, aber eine zweite Zählung neben einer, die schon da ist.

Zu wählen ist zwischen 1 und 2; das ist eine Nutzerfrage, weil sie bestimmt, wie weit die Grenze reichen soll.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
