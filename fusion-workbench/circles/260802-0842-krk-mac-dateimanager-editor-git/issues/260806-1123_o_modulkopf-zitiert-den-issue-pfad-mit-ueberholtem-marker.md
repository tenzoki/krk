Der Modulkopf von belegungsmodell.rs zitiert den Issue-Pfad mit überholtem Marker

---

`crates/krk-ui/src/belegungsmodell.rs:27` verweist auf
`issues/260806-1054_p_belegungsansicht-gruppiert-nach-funktionsbereich.md`.
Die Datei heißt seit dem Abschluss
`260806-1054_c_belegungsansicht-gruppiert-nach-funktionsbereich.md`; der
zitierte Pfad zeigt ins Leere. Fix: den Marker aus dem zitierten Pfad
nehmen (nur `260806-1054` und den Slug nennen) oder auf `_c_`
aktualisieren — die markerfreie Form veraltet nicht wieder.

---

Gefunden bei der Coderev-Durchsicht des Commits ccaf821 (Gliederung der
Belegungsansicht nach Funktionsbereichen). Zustandsmarker in Dateinamen
wandern mit dem Bearbeitungsstand (`_o_` → `_p_` → `_c_`); ein in Code
einzementierter Marker ist deshalb ab dem nächsten Übergang falsch.
Adressat: coder. Schwere: niedrig, reine Doku-Drift.
