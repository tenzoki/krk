Die Doc der Abwurfmeldungs-Tafel haengt an der Probe des Abwurfziels

---

Im Pruefmodul von `tabelle.rs` stehen zwei Doc-Bloecke ohne Trennung hintereinander ueber **einem**
`#[test]`: die Doc von `die_tafel_der_abwurfmeldung_geht_auf` (sechs mal sechs Gruende, C7) und die von
`die_tafel_des_abwurfziels_geht_auf` (acht Faelle, C4). Beide gehoeren damit zur zweiten Probe; die erste
steht 65 Zeilen tiefer ohne jede Doc.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/tabelle.rs:5266-5286`: "Die Tafel der Abwurfmeldung, vollstaendig: sechs
  gemerkte Gruende mal sechs eben gefaellte (C7) …" — endet `:5286` ohne Leerzeile.
- `:5287-5304`: "Die ganze Tafel des Abwurfziels: zwei Zeilenlagen mal vier Zeilenbefunde …".
- `:5305-5306`: `#[test] fn die_tafel_des_abwurfziels_geht_auf()`.
- `:5370-5371`: `#[test] fn die_tafel_der_abwurfmeldung_geht_auf()` — ohne Doc.

## Umfang

`krk-ui`, `appkit/tabelle.rs`, Pruefmodul.
