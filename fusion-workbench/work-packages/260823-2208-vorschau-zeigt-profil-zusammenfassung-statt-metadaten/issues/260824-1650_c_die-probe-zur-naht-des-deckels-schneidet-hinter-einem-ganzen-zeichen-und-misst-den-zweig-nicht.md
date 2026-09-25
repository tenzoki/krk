Die Probe zur Naht des Deckels schneidet hinter einem ganzen Zeichen und erreicht den Zweig nicht, den sie benennt

---

`ein_abgeschnittenes_zeichen_am_ende_nimmt_der_datei_nicht_ihren_text`
(`crates/krk-core/src/leseprofil/bausteine.rs:550-568`) trägt im Namen und im Kommentar die
Zusage, sie schneide „mitten in das zweibytige „ü"". Sie schneidet an keiner Stelle mitten in
ein Zeichen, und der Zweig von `lesbarer_anfang`, für den sie geschrieben ist, läuft in ihr
kein einziges Mal.

Der vierte Fall der Reihe, die dieser Turn schon dreimal gefunden hat: eine Probe, die mehr
behauptet, als sie misst.

---

## Die Rechnung

```
crates/krk-core/src/leseprofil/bausteine.rs:551   let ganz = "Überschrift".as_bytes();
crates/krk-core/src/leseprofil/bausteine.rs:554   // Der Deckel faellt mitten in das zweibytige „ü".
crates/krk-core/src/leseprofil/bausteine.rs:555   let naht = &ganz[..ganz.len() - 1];
```

`"Überschrift"` ist zwölf Bytes lang: `C3 9C` für das `Ü`, danach zehn ASCII-Bytes für
`berschrift`. `ganz.len() - 1` schneidet also das abschließende `t` ab und nicht das zweite
Byte des `Ü`. Nachgemessen am 260824-1640:

```text
b'\xc3\x9cberschrift'  ->  12 Bytes
b'\xc3\x9cberschrif'   ->  gültiges UTF-8, "Überschrif"
```

`naht` ist damit **vollständiges** UTF-8, und `lesbarer_anfang` beantwortet es im ersten Zweig
(`std::str::from_utf8(bytes)` gelingt). Die Zusicherung
`text.starts_with("Übersch")` hält trivial.

## Was dadurch ungemessen bleibt

Der zweite Zweig von `lesbarer_anfang`
(`crates/krk-core/src/leseprofil/bausteine.rs:513-515`):

```rust
Err(fehler) if fehler.error_len().is_none() => {
    std::str::from_utf8(&bytes[..fehler.valid_up_to()]).ok()
}
```

Er ist der ganze Grund, aus dem die Funktion existiert — der Doc-Kommentar darüber schreibt
ihn über neun Zeilen aus —, und keine Probe des Baums erreicht ihn. `lesbarer_anfang` ist
privat, also kann ihn allein dieses Prüfmodul erreichen; die Probe zu C6.6 in
`crates/krk-core/tests/leseprofil.rs:2013` schneidet bei 64 KB durch eine Datei aus lauter
ASCII-Bytes und fällt ebenfalls nicht in ihn.

## Was zu tun ist

Ein Schnitt, der wirklich in ein mehrbytiges Zeichen fällt, also `&ganz[..1]` statt
`&ganz[..ganz.len() - 1]`. Der Rückgabewert ist dann `Some("")` und nicht ein Text, der mit
`Übersch` beginnt; die Zusicherung ist entsprechend zu setzen — etwa ein Text mit einem
mehrbytigen Zeichen **am Ende**, dessen Anfang stehen bleibt: `"Titel Ü"` bei `&…[..7]` liefert
`Some("Titel ")`, und ohne den zweiten Zweig lieferte es `None`. Erst diese Gegenrechnung
belegt den Satz „die Naht des Deckels nimmt der Datei ihren Text" nicht.

**Der Kommentar in Zeile 554 ist mitzuziehen**, sonst bleibt eine Prosastelle stehen, die
etwas anderes sagt als die Zeile darunter tut.

**Schwere:** mittel. Kein Fehlverhalten der Rechnung — der Zweig ist inhaltlich richtig
gebaut —, aber die einzige Probe, die ihn halten soll, hält ihn nicht, und ein späterer Umbau
von `lesbarer_anfang` auf `String::from_utf8(bytes).ok()` bliebe grün.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1640.

**Betroffen:** `crates/krk-core/src/leseprofil/bausteine.rs` (Prüfmodul, Zeilen 549-568)

**Domain:** code

---
Resolved: 260824-1740 vom coder. Die Probe schneidet jetzt an einer echten Naht: `"Titel Ü"` ist acht Bytes, `&ganz[..7]` lässt das erste Byte des `Ü` stehen und nimmt das zweite weg, und `lesbarer_anfang` liefert `Some("Titel ")`. Eine vorangestellte Zusicherung `std::str::from_utf8(naht).is_err()` hält fest, dass der erste Zweig die Antwort nicht schon trägt; daneben stehen weiter der Schnitt hinter einem ganzen Zeichen, ein ungültiges Byte in der Mitte und eines am Ende. Der Kommentar zieht mit. **Der Zweig `Err(fehler) if fehler.error_len().is_none()` läuft nachweislich**: mit ausgehöhltem Zweig (`None` statt `from_utf8(&bytes[..valid_up_to()])`) fällt die Probe an genau dieser Zusicherung um, die alte Fassung blieb unter derselben Aushöhlung grün. Beide Läufe sind gefahren.
