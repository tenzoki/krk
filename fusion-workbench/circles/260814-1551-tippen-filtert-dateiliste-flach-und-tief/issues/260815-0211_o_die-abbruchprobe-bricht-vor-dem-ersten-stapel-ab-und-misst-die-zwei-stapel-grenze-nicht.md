# Die Abbruchprobe bricht vor dem ersten Stapel ab und misst die Zwei-Stapel-Grenze nicht

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C3.4; `crates/krk-core/tests/verzeichnis.rs`, `der_abbruch_greift_in_einem_ordner_ohne_unterordner`; `crates/krk-core/src/verzeichnis/durchlauf.rs:256-273`

---

## Befund

C3.4 sagt zwei Dinge zu: der Abbruch greift **innerhalb von zwei Stapeln**, und er wird **an der Stapelgrenze** geprüft und nicht beim Absteigen. Die Probe legt 5.000 Dateien in einen Ordner ohne Unterordner an, prüft eigens, dass es mehr als zwei Stapel sind — und ruft dann `abbrechen()` unmittelbar nach `starten()`:

```rust
let durchlauf = Durchlauf::starten(…);
durchlauf.abbrechen();
assert_eq!(befunde_einsammeln(&durchlauf), Vec::new(), …);
```

Zu diesem Zeitpunkt hat der Arbeitsfaden noch keinen einzigen Stapel geholt. Die erste Prüfung des Abbruchkennzeichens in `unterbaum_entscheiden` steht **vor** dem ersten `stapel_holen` (der frisch angelegte `Ebene` trägt einen leeren Stapel), greift also sofort. Der Ordner wird mit **null** gelesenen Einträgen verlassen.

**Was die Probe damit hält:** dass die Prüfung nicht ausschließlich am Abstiegszweig hängt. Läge sie dort, liefe der Ordner ohne Unterordner durch und schickte einen Befund; die Probe würde rot. Das ist die zweite Zusage von C3.4, und sie ist geprüft.

**Was sie nicht hält:** die erste Zusage. Wie viele Stapel zwischen dem Setzen des Kennzeichens und dem Ende des Fadens liegen, misst nichts — die Probe setzt es, bevor der erste geholt wird. Die 5.000 Dateien und die Zusicherung `> 2 * STAPELGROESSE` spielen im Ablauf keine Rolle; sie lesen sich aber wie der Beleg für die Zahl „zwei".

## Was zu tun wäre

Eine Probe, die die Grenze wirklich misst, muss den Faden erst arbeiten lassen und dann abbrechen, und sie braucht eine Größe, an der die geleistete Arbeit abzulesen ist. Der Durchlauf gibt heute keine her: er meldet je Auftrag genau einen Befund und sonst nichts. Möglich wäre ein Prüfordner, dessen Treffer hinter der Zwei-Stapel-Grenze liegt — bricht man nach dem Anlauf ab, darf der Treffer nicht mehr gemeldet werden.

Solange keine solche Probe steht, gehört an den Doc-Kommentar der bestehenden, was sie entscheidet und was nicht — so, wie `crates/krk-ui/src/quellbaum.rs` es für die Zählproben dieses Baums verlangt.
