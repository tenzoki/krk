# Eine Nachbildung der Konfliktschaltflächen steht in `blaetter/mod.rs` und nichts hält sie am Original

---
Die Probe `die_eingabetaste_im_feld_gehoert_ihrer_eigenen_schaltflaeche`
(`crates/krk-ui/src/appkit/blaetter/mod.rs`, Prüfmodul) baut die vier
Schaltflächen des Konfliktblattes von Hand nach, mit Beschriftung, Taste und
Wirkung. Die eine Angabe, gegen die das Blatt gebaut wird, ist
`konflikt::schaltflaechen` (`crates/krk-ui/src/appkit/blaetter/konflikt.rs`), und
zwischen beiden steht kein Mechanismus: die Nachbildung bleibt grün, wenn das
Original seine Reihenfolge, seine Tasten oder seine Beschriftungen ändert.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Aufgefallen bei der Umstellung der ersten Schaltfläche auf zwei Beschriftungen
(260907, Entscheid
`260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`):
die Nachbildung trug weiter „Überschreiben“, während das Blatt den Wortlaut nicht
mehr führt, und keine Probe wurde davon rot. Nachgezogen ist sie von Hand.

Die Nachbildung steht dort nicht ohne Grund: `blaetter/mod.rs` prüft
`bestaetigungsstelle` über **drei** Baupläne, die im Baum auseinandergehen, und
zwei davon (die Löschrückfrage, `Blatt::neu`) sind ebenfalls nachgebaut. Ein
Verweis auf `konflikt::schaltflaechen` wäre die naheliegende Abhilfe für einen
der drei, ist aber eine Richtungsumkehr: heute liest `konflikt.rs` aus
`blaetter/mod.rs` und nicht umgekehrt.

**Abnahmeprüfung:** Wer die Reihenfolge oder eine Beschriftung in
`konflikt::schaltflaechen` ändert und `cargo test --workspace` fährt, bekommt
eine rote Probe, ohne die Nachbildung anzufassen — oder die Nachbildung ist
verschwunden.

---
Resolved: Die Nachbildung ist verschwunden — die zweite Haelfte der Abnahmepruefung. `die_eingabetaste_im_feld_gehoert_ihrer_eigenen_schaltflaeche` holt jeden Bauplan bei seinem Bauer: `konflikt::schaltflaechen` fuer **beide** Gestalten, `loeschbestaetigung::schaltflaechen` fuer die Rueckfrage, `standardschaltflaechen` fuer `Blatt::neu`. Die zwei ersten sind dafuer von privat auf `pub(super)` gestellt, je mit einem Satz, warum.
