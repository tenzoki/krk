Das Kommando `fokus_vorschau` steht im Code und noch nicht in der Auslieferungsbelegung

---

**Domain:** data
**Filed by:** coder (bei der Umsetzung der beiden Fokusentscheide vom 260807)
**Für:** ontocoder
**Cross-references:** `crates/krk-core/src/tasten/belegung.rs` `Kommando::KENNUNGEN`,
`resources/default-keymap.toml`,
`decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md`

---

`Kommando::FokusVorschau` trägt seit dieser Umsetzung die Kennung
`fokus_vorschau`. `resources/default-keymap.toml` führt dafür noch keinen
`[[funktion]]`-Block, und die Prüfung
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
(`crates/krk-core/src/tasten/belegung.rs:993`) schlägt deshalb fehl. Sie ist
die Brücke zwischen beiden Beständen und tut hier genau das, wofür es sie gibt.
`cargo test --workspace` ist bis zum Nachtrag rot, an dieser einen Zusage und an
keiner weiteren.

**Der Nachtrag.** Ein Block hinter `fokus_dateifenster`, in der C5-Gruppe der
Datei:

```toml
[[funktion]]
id = "fokus_vorschau"
name = "Fokus in das Vorschaufenster"
tasten = ["shift+cmd+y"]
```

Die Kombination hat der Nutzer am 260807 bestimmt. Der Buchstabe ist nicht frei
gewählt: die Vorschau trägt in dieser Belegung schon das `y` (`cmd+y` blendet
sie ein und aus, `resources/default-keymap.toml:101`), und der Fokusbefehl erbt
ihn, wie `l` und `d` es für die Leiste und das Dateifenster tun.

`shift+cmd+y` ist in der ausgelieferten Belegung sonst nirgends vergeben; der
Eintrag löst keinen Konflikt aus. Geprüft ist das durch einen Probelauf mit
genau diesem Block: `make check` läuft damit in allen vier Kommandos grün. Der
Block selbst ist wieder entfernt worden, weil `resources/` dem ontocoder gehört.

**Warum die Reihenfolge so ist.** Der Code muss zuerst stehen, denn eine
Kennung in der Belegungsdatei ohne Kommando dahinter wäre eine Funktion, die in
der Belegungsansicht steht und nichts tut. Zwischen beiden Schritten ist der
Baum rot; das ist der Preis der Trennung nach Dateibesitz und kein Defekt.

---
Resolved: `resources/default-keymap.toml` führt den Block `fokus_vorschau` auf `shift+cmd+y`, im C5-Block hinter `fokus_dateifenster`. Der Wortlaut des `coder` ist übernommen; dazu kommt der Kommentar, der den Buchstaben begründet, und die Kopfzeile `Ausgeliefert sind 58 Funktionen mit zusammen 65 Kombinationen` (vorher 57 und 64), die der Nachtrag sonst hätte auseinanderlaufen lassen. `shift+cmd+y` ist sonst nirgends vergeben, geprüft dreifach: über die geparste Datei (65 Kombinationen, keine doppelte je Zusteller), über KRK eigene Konflikterkennung (`die_auslieferungsbelegung_ist_konfliktfrei`, `crates/krk-core/tests/belegung.rs`) und über das gebaute Hauptmenü (`make menue` listet sieben Kürzel, `shift+cmd+y` ist keines). `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` läuft grün, `make check` in allen vier Kommandos. Das Abnahmekriterium von S20 trägt unverändert: `Belegung::zuruecksetzen` setzt aus `Belegung::auslieferung()`, und die liest über `include_str!` genau diese Datei, der Eintrag steht also auf beiden Seiten des Vergleichs. Offen bleibt der Kommentar `crates/krk-ui/src/appkit/belegungsansicht.rs:76`, der weiter 57 Funktionen nennt; dafür steht `260807-1015_o_der-kommentar-zur-tabellenhoehe-nennt-57-funktionen-und-die-belegung-fuehrt-58.md`.
