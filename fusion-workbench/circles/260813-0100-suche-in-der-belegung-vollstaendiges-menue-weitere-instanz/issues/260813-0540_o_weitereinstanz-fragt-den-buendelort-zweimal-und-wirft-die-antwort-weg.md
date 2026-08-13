`weitereinstanz::starten` fragt den Bündelort zweimal und wirft die Antwort weg

---

`eigenes_buendel` bestimmt den Ort des laufenden Bündels und liefert ihn als `PathBuf`
(`crates/krk-ui/src/appkit/weitereinstanz.rs:84-91`). Sein Doc-Kommentar nennt es „die eine
Stelle des Programms, die den eigenen Bündelort bestimmt".

`starten` benutzt den Rückgabewert nicht (`:98-112`):

```rust
let adresse = NSBundle::mainBundle().bundleURL();
if eigenes_buendel().is_none() {
    return Some(OHNE_BUENDEL);
}
```

Es fragt `NSBundle::mainBundle().bundleURL()` selbst ein zweites Mal, benutzt diese `NSURL`
für den Start und braucht von `eigenes_buendel` allein die Ja-Nein-Antwort. Der `PathBuf`, den
jene Funktion baut, wird angelegt und fallengelassen.

**Ein Nebenausgang fällt dabei falsch aus.** `eigenes_buendel` liefert auch dann `None`, wenn
die Endung `.app` stimmt, `adresse.path()` aber nichts liefert (`:90`). `starten` meldet in
diesem Fall „KRK laeuft nicht aus einem Buendel", obwohl es das tut. Der Fall ist an einem
Bündel nicht zu erwarten; die Meldung wäre trotzdem die falsche Auskunft.

---

**Schwere:** gering. Zwei Systemaufrufe statt einem auf einem Weg, der einmal je Tastendruck
läuft, und eine Meldung, die einen unwahrscheinlichen Fall falsch benennt.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/weitereinstanz.rs:84-112`

**Domain:** code

## Vorschlag

`eigenes_buendel` die `NSURL` liefern lassen statt eines `PathBuf`, den niemand braucht, und
`starten` allein daraus schöpfen:

```rust
let Some(adresse) = eigenes_buendel() else {
    return Some(OHNE_BUENDEL);
};
```

Dann gibt es wirklich nur eine Stelle, die den Ort bestimmt, der `PathBuf` entfällt, und die
Prüfung auf die Endung entscheidet allein. Die Zählprobe
`der_eigene_buendelort_wird_an_genau_einer_stelle_bestimmt` (`:129-142`) zählt Dateien und
bleibt davon unberührt; sie sähe den doppelten Aufruf ohnehin nicht.
