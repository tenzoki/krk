Der `include_str!`-Pfad in Schritt 11 liegt eine Verzeichnisebene zu hoch

---

Schritt 11 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` schreibt unter `Änderungen` vor: "Einlesen der Auslieferungsbelegung über `include_str!("../../../resources/default-keymap.toml")`". Derselbe Schritt legt die einlesende Datei nach `crates/krk-core/src/tasten/`. Von dort aus zeigt der genannte Pfad auf `crates/resources/default-keymap.toml`, ein Verzeichnis, das es nicht gibt.

`include_str!` löst relativ zum Verzeichnis der einbindenden Datei auf. Von `crates/krk-core/src/tasten/` sind es vier Schritte bis zur Projektwurzel, nicht drei: `../` ist `src/`, `../../` ist `krk-core/`, `../../../` ist `crates/`, und erst `../../../../` ist die Wurzel.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11 am 260803-2317.

**Was die Umsetzung getan hat.** `crates/krk-core/src/tasten/belegung.rs` schreibt `include_str!("../../../../resources/default-keymap.toml")` und übersetzt damit. Die Absicht des Plans ist unverändert eingelöst: die ausgelieferte Belegung ist einkompiliert, und es gibt keinen Start ohne Belegung.

**Warum es trotzdem gemeldet ist.** Der Plantext ist die Vorlage für den nächsten Leser, und ein wörtlich übernommener Pfad scheitert bei ihm mit einer Meldung über eine fehlende Datei, deren Ursache er erst suchen muss. Der Fehler ist harmlos, weil der Übersetzer ihn findet; er ist keine stille Abweichung.

**Was zu tun ist.** In Schritt 11 unter `Änderungen` den Pfad auf `include_str!("../../../../resources/default-keymap.toml")` ziehen. Eine Zeile.
