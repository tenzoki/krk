# Der Doc-Kommentar der Meldungsprobe nennt vier Lagen, und die Probe führt fünf

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Low
**Affected:** `crates/krk-core/tests/ablage.rs:1354`
**Tree state:** `4a57028`

---

## Was ist

Der erste Satz des Doc-Kommentars und der Name der Probe drei Zeilen darunter
widersprechen sich:

```rust
// tests/ablage.rs:1354-1361
/// Jede der vier Lagen traegt ihren eigenen Satz, und keiner ist mehrzeilig
/// (C3.7, C3.8).
///
/// Die Saetze werden an gebauten Werten geprueft und nicht an einem Ablauf:
/// die Fallunterscheidung ist ueber `Beiseite` vollstaendig, und eine Probe
/// ueber die fuenf Werte prueft sie ebenso vollstaendig.
#[test]
fn die_meldung_unterscheidet_die_fuenf_lagen_und_bleibt_einzeilig() {
```

Der zweite Absatz desselben Kommentars sagt schon „die fünf Werte", der Name sagt
fünf, und der Rumpf zählt fünf auf (`:1432-1438`): `Nicht`, `Gesichert`,
`Gekuerzt`, `SchonVorhanden`, `Gescheitert`. `Beiseite` trägt fünf Varianten.
Allein der erste Satz steht auf vier.

Die Schwesterprobe `die_meldung_zu_readers_toml_verspricht_keinen_auslieferungszustand`
(`:1448-1459`) spricht durchgehend von fünf und nennt diese hier ausdrücklich ihre
Gegenprobe.

`Gekuerzt` ist am 260814 dazugekommen, mit der Zetteldatei über `EDITORGRENZE`;
die Vier stammt aus der Zeit davor.

## Was zu tun wäre

Die Vier im ersten Satz auf Fünf ziehen. Eine Zahl an dieser Stelle ist
vertretbar, weil `Beiseite` eine vollständige Fallunterscheidung ist und die Probe
sie einzeln aufzählt: eine sechste Variante hält den Bau ohnehin an.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.
