Die Versionszahlprobe findet „1.11.1“ in einer Messangabe von tabelle.rs, und make check ist auf HEAD rot

---

Seit `0d4485e` (Version 1.11.1) scheitert `appkit::titelzusatz::tests::die_versionszahl_steht_in_keiner_quelldatei` in `krk-ui`: die Nadel `env!("CARGO_PKG_VERSION")` = `1.11.1` steht als Teilfolge im Kommentar `„11.11.11 11:11“` in `crates/krk-ui/src/appkit/tabelle.rs` (Zeile 4772, gemessene Breite eines Datumsmusters). `make check` endet damit am Ziel `test` mit Exit 2, unabhängig von jeder Arbeit dieses Pakets.

---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Beleg.** `cargo test --workspace --no-fail-fast` am 260926-0223: 25 Testziele grün, eines rot mit `die Version 1.11.1 steht als Zeichenkette in ["krk-ui/src/appkit/tabelle.rs"]`. `grep -n '1\.11\.1' crates/krk-ui/src/appkit/tabelle.rs` trifft allein die Messzeile. Der Kommentar stammt aus `3d2c613`; rot wurde die Probe erst mit dem Anheben der Version.

**Nicht behoben, weil beide Wege eine Wahl sind, die nicht beim Schritt 1.1 liegt.** Der Doc-Kommentar der Probe sagt ausdrücklich, ein solcher Treffer sei „ein Anlass hinzusehen und nicht einer, die Nadel zu verengen“; die Messzeile umzuschreiben änderte eine aufgezeichnete Messung. Zu entscheiden ist, ob die Messangabe ein anderes, gleich breites Muster nennt, oder ob die Probe Kommentarzeilen auslässt und damit ihre Zusage (C1.2 der Runde 8) auf Code einengt.

**Abnahme.** `make check` endet mit 0, und die Probe findet die Version weiterhin in einer Zeichenkette im Code, wenn man sie dort einsetzt.

---
Resolved: Der Messkommentar in `crates/krk-ui/src/appkit/tabelle.rs` beschreibt die Probezeichenkette aus lauter Einsen in Worten statt sie auszuschreiben; die Messwerte bleiben unverändert, und die Probe ist nicht eingeengt, wie ihr Doc-Kommentar es verlangt. `make check` ist grün.
