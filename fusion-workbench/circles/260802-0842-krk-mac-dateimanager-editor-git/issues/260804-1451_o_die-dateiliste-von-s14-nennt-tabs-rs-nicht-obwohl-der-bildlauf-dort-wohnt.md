Die Dateiliste von S14 nennt `tabs.rs` nicht, obwohl der Bildlauf dort wohnt

---

S14 sagt zu: "Auswahl und Bildlaufposition überleben eine Auffrischung, soweit
die Einträge noch existieren." Die Dateiliste des Schrittes nennt dafür
`crates/krk-ui/src/appkit/tabelle.rs` ("gesetzt und gelesen werden beide über
die Hülle, die S12 dort anlegt"), aber nicht
`crates/krk-ui/src/tabs.rs`. Beides zusammen geht nicht auf: die
Bildlaufposition und der Name des ausgewählten Eintrags wohnen in `tabs.rs`,
und `tabs.rs` hat für das Wiederherstellen beider keine öffentliche Naht, die
von außen zu bedienen wäre.

---

**Was genau fehlt.** Eine Auffrischung ist ein zweiter Lesevorgang auf
denselben Ordner. Der vorhandene Weg dafür wäre
`Tabliste::ordner_setzen(derselbe_ordner, auswahl)`. Der baut den Tab aus
einem frischen `Tabzustand::auf(ordner)` neu auf, und der trägt `bildlauf =
0.0`; damit steht `Tabinhalt::bildlauf_offen` auf `false`, und
`DateifensterQuelle::gemerkten_bildlauf_herstellen` überspringt die
Wiederherstellung. Die Bildlaufposition ginge verloren, und die Liste spränge
bei jeder fremden Änderung an den Anfang.

Von außen ist das nicht zu heilen. `Tabinhalt::bildlauf_setzen` ist öffentlich,
`bildlauf_offen` nicht, und ohne dieses Kennzeichen stellt die Ansicht die
gemerkte Position nicht her. Ein eigenes Kennzeichen in `tabelle.rs` wäre ein
zweiter Wiederherstellungsweg neben dem, den die Sitzungswiederherstellung
schon benutzt.

**Was der `coder` am 260804 getan hat.** `crates/krk-ui/src/tabs.rs` bekam eine
Methode `Tabliste::aktiven_neu_lesen()`. Sie liest den Tab über
`Tabinhalt::zustand()` aus, also in genau der Form, in der er auch in
`session.toml` stünde, und baut ihn daraus neu auf. Damit tragen `wunschauswahl`
und `bildlauf_offen` die Auffrischung, und zwar dieselben zwei Felder, die die
Sitzungswiederherstellung ohnehin benutzt. Ein zweiter Mechanismus ist nicht
entstanden; die Datei stand nur nicht in der Liste.

**Was zu entscheiden ist.** Ob die Dateiliste von S14 um
`crates/krk-ui/src/tabs.rs` (erweitert) ergänzt wird, oder ob der `planner`
einen anderen Zuschnitt sieht. Der Eintrag hier hält die Abweichung offen, weil
die Dateiliste des Plans bindend ist und der `coder` sie überschritten hat.

**Aufgefallen bei:** der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.
