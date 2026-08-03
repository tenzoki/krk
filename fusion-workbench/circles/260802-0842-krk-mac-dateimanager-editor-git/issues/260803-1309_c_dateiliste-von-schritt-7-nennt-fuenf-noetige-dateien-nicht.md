Die Dateiliste von Schritt 7 nennt fünf Dateien nicht, die der Schritt anfassen muss

---

S7 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` listet fünf
Dateien. Die Umsetzung am 260803-1309 brauchte fünf weitere. Vier davon fallen unter
die Einbindungsregel, die der Plan im Kopf von `## Implementierungsschritte` selbst
aufstellt; die fünfte ist die Datei, in der die Kommandos ausgeführt werden.

---

**Was der Plan nennt:** `crates/krk-ui/src/appkit/ereignisse.rs`,
`crates/krk-ui/src/appkit/mod.rs` (einbindend), `crates/krk-core/src/tasten/{mod.rs,normalisierung.rs}`,
`crates/krk-core/src/lib.rs` (einbindend), `crates/krk-core/tests/tasten.rs`.

**Was zusätzlich nötig war:**

| Datei | Warum |
|---|---|
| `Cargo.toml` des Workspace | `NSEvent.addLocalMonitorForEventsMatchingMask:handler:` nimmt seinen Rückruf als Objective-C-Block. Ohne die Kiste `block2` lässt sich keiner bauen. Die Versionsangabe gehört nach `[workspace.dependencies]`, wie S1 es für die drei objc2-Kisten hält. |
| `crates/krk-ui/Cargo.toml` | trägt `block2 = { workspace = true }` als unmittelbare Abhängigkeit. |
| `crates/krk-ui/src/main.rs` | `--tasten-protokoll` ist eine Befehlszeilenmarke und braucht dort ihre Auswertung. Der Auftrag an den `coder` hielt diesen Punkt bereits fest und erklärte ihn für zulässig. |
| `crates/krk-ui/src/appkit/anwendung.rs` | richtet den Abgriff nach dem Aufbau der Oberfläche ein und hält ihn fest; ohne einen Halter meldet er sich beim Fallen sofort wieder ab. Trägt außerdem die Marke aus `main.rs` weiter. |
| `crates/krk-ui/src/appkit/tabelle.rs` | führt die Kommandos aus. Auswahl bewegen, Seitenhöhe erfragen und in einen Ordner hineinsteigen sind Sache der Tabelle und ihrer Datenquelle; der Abgriff kennt weder Tabelle noch Modell. Dazu ein neues Feld für den angezeigten Ordner, weil ein Eintrag nur seinen Namen trägt und ohne den Ordner daneben kein Ziel für `Oeffnen` entsteht. |

**Warum das ein Defekt und keine Kleinigkeit ist.** `issues/260802-1900_c_dateilisten-der-planschritte-lassen-wiederholt-die-cargo-toml-aus.md`
hat dieselbe Lücke schon einmal geschlossen und dabei alle Schritte S5 bis S23 unter
der Einbindungsregel durchgesehen. Die zwei `Cargo.toml`-Einträge hier zeigen, dass
die Durchsicht eine Abhängigkeit nicht vorhersehen konnte, die erst bei der Umsetzung
sichtbar wurde. Das ist kein Vorwurf an die Durchsicht, sondern die Grenze dessen, was
eine Dateiliste vorwegnehmen kann. Der Eintrag hier hält den Stand fest, damit der
Plan nachgezogen werden kann und die nächste Durchsicht nicht dieselbe Rechnung
zweimal aufmacht.

**Was zu tun ist.** Die Dateiliste von S7 um die fünf Einträge ergänzen, jeweils mit
dem Vermerk `(einbindend)` dort, wo er zutrifft. Am Code ist nichts zu ändern; er ist
gebaut, geprüft und formatiert.

---
Resolved: Die Dateiliste von S7 trägt die fünf genannten Einträge, dazu die `Cargo.lock`, die die neue Abhängigkeit `block2` mechanisch mitzieht und die versioniert ist; die Regel im Kopf von `## Implementierungsschritte` nennt diesen sechsten Fall seit dem Nachzug vom 260803-1819. Jeder Eintrag trägt den Vermerk, der zutrifft: die beiden `Cargo.toml` als `(einbindend)`, `main.rs`, `anwendung.rs` und `tabelle.rs` als `(erweitert)`. Am Code ist nichts geändert. Nachgezogen am 260803-2007.
