Der Doc-Kommentar von `WaechterIvars::pruefung` nennt das PIN-Blatt als einzigen Setzer; seit Schritt 2 setzt es auch die Namenseingabe
---
`crates/krk-ui/src/appkit/blaetter/mod.rs`, Feld `pruefung` der Struktur `WaechterIvars`, sagt: „Wahlfrei, weil allein das PIN-Blatt es setzt ([`Blatt::bestaetigung_pruefen`])“. Seit Schritt 2 des Plans `260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md` ruft auch `namenseingabe::geprueft_zeigen` `Blatt::bestaetigung_pruefen`, sobald seine Vorlage eine Pruefung traegt; der Satz ist damit falsch. Modulkopf und der Doc-Kommentar von `Blatt::bestaetigung_pruefen` sind im selben Schritt berichtigt; das Feld nicht, weil die Vorgabe des Schritts in `mod.rs` allein diese zwei Stellen freigibt.

Beleg: `grep -n 'allein das PIN-Blatt' crates/krk-ui/src/appkit/blaetter/mod.rs` gegen `grep -rn 'bestaetigung_pruefen(' crates/krk-ui/src`.

Abnahme: der Doc-Kommentar des Felds nennt keinen einzigen Setzer mehr, sondern verweist wie der Doc-Kommentar von `Blatt::bestaetigung_pruefen` auf die Rufer; `grep -n 'allein das PIN-Blatt' crates/krk-ui/src/appkit/blaetter/mod.rs` gibt nichts aus.
---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>
Gefunden beim Bau von Schritt 2 (Namensblatt mit Grund, Pruefung und Abbruchmeldung); ein Einzeiler, der in Schritt 3 mit `mod.rs` mitgehen kann.
---
Resolved: mit Schritt 3 des Plans `260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md`. Der Doc-Kommentar des Felds `WaechterIvars::pruefung` in `crates/krk-ui/src/appkit/blaetter/mod.rs` nennt keinen einzigen Setzer mehr und verweist auf die Rufer von `Blatt::bestaetigung_pruefen`. Die Abnahme-Erhebung `grep -n 'allein das PIN-Blatt'` trifft weiter eine Zeile, den Doc-Kommentar von `Blattgriff::verdeckt_machen` („heute allein das PIN-Blatt“); die Aussage dort gilt dem Verdecken und ist wahr, und der Schritt gibt in `mod.rs` allein die Stelle dieses Datensatzes frei. Das Erhebungsmuster war weiter als der Befund.
