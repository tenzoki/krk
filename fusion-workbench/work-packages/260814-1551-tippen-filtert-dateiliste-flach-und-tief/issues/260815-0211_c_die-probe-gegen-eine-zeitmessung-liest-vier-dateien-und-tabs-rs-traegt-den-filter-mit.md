# Die Probe gegen eine Zeitmessung liest vier Dateien, und `tabs.rs` trägt den Filter mit

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.5; `crates/krk-core/tests/verzeichnis.rs`, `im_filter_steht_keine_zeitmessung`; `crates/krk-ui/src/tabs.rs`

---

## Befund

`im_filter_steht_keine_zeitmessung` sucht `Instant`, `Duration` und `::now(` in vier Dateien:

```
krk-core/src/verzeichnis/filter.rs
krk-core/src/verzeichnis/modell.rs
krk-core/src/verzeichnis/durchlauf.rs
krk-ui/src/appkit/tabelle.rs
```

Der Doc-Kommentar begründet die Auswahl mit „den drei Modulen des Kerns, die den Filter tragen, und der einen Senke in `krk-ui`". Seit Schritt F2 stimmt das nicht mehr: `crates/krk-ui/src/tabs.rs` trägt den Filtertext über den Ordnerwechsel (`Tabliste::ordner_setzen`), hält den `Durchlauf` je Tab, entscheidet in `durchlauf_nachziehen_an`, wann einer beginnt und vergeht, und zieht die Befunde in `befunde_einziehen` ein. Ein Zeitgeber, der den Filtertext nach einer Pause zurücksetzte, ließe sich dort ebenso gut unterbringen wie in den vier gelesenen Dateien — und die Probe sähe ihn nicht.

## Warum die Liste nicht einfach zu erweitern ist

`tabs.rs` enthält heute `std::time::Duration` und `std::time::SystemTime`, beides im `#[cfg(test)]`-Modul: die Probe `der_tab_zieht_die_befunde_ein_…` schläft zwischen zwei Einzugstakten, und `eintrag()` baut seine Einträge mit `SystemTime::UNIX_EPOCH`. `code_zeilen` streicht Kommentarzeilen, nicht Prüfmodule. Die Datei anzuhängen macht die Probe also rot, ohne dass eine Uhr im Filter stünde.

Zwei Wege:

1. `code_zeilen` um einen Schnitt am ersten `#[cfg(test)]` erweitern, wie `die_angezeigte_datei_bleibt_bei_zwei_quellen` es für ihre Datei tut, und `tabs.rs` dann aufnehmen.
2. Die Liste lassen und den Doc-Kommentar berichtigen: er sagt heute, die vier Dateien seien die, die den Filter tragen, und das ist seit F2 unvollständig.

Der Befund ist gering im Gewicht — die Runde hat keinen Zeitgeber gebaut, und die Sekundenregel ist nachweislich gefallen. Er gehört trotzdem in die Liste, weil die Probe eine Aussage über den **Filter** macht und nicht über vier Dateien.

---
Resolved: Gefahren ist Weg 1. `code_zeilen_vor_dem_pruefmodul` schneidet am ersten `#[cfg(test)]` und reicht das Ergebnis an `code_zeilen` weiter; `im_filter_steht_keine_zeitmessung` liest damit fünf Dateien statt vier, und `krk-ui/src/tabs.rs` ist die fünfte.

Die Zusicherung ist damit eine über den Filter und nicht über eine Dateiliste: `tabs.rs` trägt seit Schritt F2 den Filtertext über den Ordnerwechsel, hält den `Durchlauf` je Tab, entscheidet in `durchlauf_nachziehen_an`, wann einer beginnt und vergeht, und zieht die Befunde ein. `Duration` und `SystemTime` stehen dort in ihrem Prüfmodul und fallen mit dem Schnitt heraus; vor dem Schnitt trägt die Datei keine der drei Nadeln.

Der Doc-Kommentar nennt jetzt fünf Dateien, sagt in einem eigenen Absatz, warum `tabs.rs` dazugehört und seit wann, und behält die Feststellung, dass `krk-ui/src/appkit/anwendung.rs` weiterhin nicht in der Liste steht: die Uhr dort gehört dem Anzeigeverzug der Dateioperationen und nicht dem Filter.

Berührte Datei: `crates/krk-core/tests/verzeichnis.rs`.
