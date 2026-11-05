`HowTo.md` und der Kommentar an `Text::BlattSteht` folgen den neuen Fäden nicht
---
Schwere: Niedrig. Zwei Stellen beschreiben den Stand vor dem Umbau oder lassen einen neuen Rufer aus. Am Verhalten ändert das nichts.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg 1.** `HowTo.md:1486` und `:1487`: „Geräte und Orte", „die bei jedem Aufbau frisch vom System kommen“. Seit dem Umbau kommt die Aufzählung vom Faden (`crates/krk-ui/src/appkit/anwendung.rs`, `geraete_nachfragen`, `geraete_aufzaehlen_lassen`). Beim Start zeigt die Gruppe also zunächst nur, was ohne Aufzählung feststeht, und auf einem hängenden Datenträger bleibt es dabei. Der benachbarte Absatz `:1517` bis `:1521` sagt genau das für den Zusatz „(fehlt)“, dieser nicht. Beim Fehlschlag bleibt die alte Liste stehen (Kommentar an `geraete_aufzaehlen_lassen`), also auch nicht „frisch“.

**Beleg 2.** `crates/krk-core/src/sprache/schluessel.rs:1140` bis `:1142`: der Kommentar an `Text::BlattSteht` nennt `blattmeldung::satz` und `loeschbefund_einziehen`. Als dritter Rufer fehlt `pin_erfragen` (`anwendung.rs:12401`, Prüfung `:12410`).

**Richtung.** Den Satz in `HowTo.md` auf den Ablauf bringen, ohne Fachwort. Den dritten Rufer ergänzen, oder im Kommentar die Art der Lage statt der Rufer nennen, wie es `541fcd4b` an `PLATTENFRAGE` getan hat.

**Abnahme.** Beide Stellen lesen sich gegen den Code richtig. Am Wortlaut hängt keine Probe, die rot würde; `grep -n 'BlattSteht' crates/krk-ui/src/appkit/anwendung.rs` zeigt die Rufer.

---
Resolved: `HowTo.md` `## Lesezeichen` sagt zu „Geräte und Orte“ den Ablauf: die Datenträger kommen kurz nach dem Start hinzu, und antwortet eine Platte nicht, bleibt die Liste beim letzten Stand. Der Kommentar an `Text::BlattSteht` (`crates/krk-core/src/sprache/schluessel.rs`) nennt die Art der Lage statt der Rufer.
