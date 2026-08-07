Der Größenformatierer schreibt nicht nur "Zero", sondern jede Byte-Angabe auf Englisch

---

`issues/260805-1130_o_der-groessenformatierer-schreibt-zero-kb-auf-englisch.md` hält
fest, "allein das Wort für null ist englisch". Das ist zu eng gefasst. Gemessen am
260806-1210: **jede** Angabe unterhalb von 1.000 Bytes kommt englisch aus demselben
Grund, und dazu gehören alle kleinen Dateien, nicht nur die leeren.

Die Messung ist ein Foundation-Programm mit demselben `NSByteCountFormatter` und
demselben `CountStyle::File`, den `crates/krk-ui/src/appkit/tabelle.rs` anlegt, einmal
ohne Bündel und einmal in einem Bündel, dessen `Info.plist` `CFBundleLocalizations`
auf `de, en` setzt:

| Bytes | ohne Localizations (wie KRK heute) | mit `CFBundleLocalizations = de, en` |
|-------|------------------------------------|--------------------------------------|
| 0     | `Zero KB`                          | `0 KB`                               |
| 1     | `1 byte`                           | `1 Byte`                             |
| 512   | `512 bytes`                        | `512 Byte`                           |
| 999   | `999 bytes`                        | `999 Byte`                           |
| 1.000 | `1 KB`                             | `1 KB`                               |
| 10.240| `10 KB`                            | `10 KB`                              |

`Bundle.main.preferredLocalizations` liefert ohne den Schlüssel `["en"]` und mit ihm
`["de"]`. Sichtbar ist das in der Größenspalte des Dateifensters (C1, seit S12), in
den Metadatenzeilen des Vorschaufensters (C6, seit S19) und im fünften Rang der
Statuszeile (C1, seit S16c).

---

**Was daraus für die beiden Wege des ursprünglichen Defekts folgt.**

*Weg 2, `setAllowsNonnumericFormatting(false)` in `tabelle.rs`, löst die Sache nicht.*
Gemessen liefert er `0 bytes`, `1 byte`, `512 bytes` — die Zahl stimmt, das Wort
bleibt englisch. Er tauscht ein englisches Wort gegen ein anderes und lässt jede
kleine Datei unverändert englisch beschriftet. Als Reparatur taugt er nicht.

*Weg 1, `CFBundleLocalizations` in `resources/Info.plist` um `de` erweitern, löst sie
vollständig*, für null wie für jede Byte-Angabe darunter, und ohne eine zweite
Rechnung neben dem gemeinsamen Formatierer.

**Ein Weg über Code besteht nicht.** Naheliegend wäre, dieselbe Registrierungsebene
der Nutzervorgaben zu benutzen, über die `menue::systemzusaetze_unterdruecken` schon
drei Systemzusätze abstellt. Am 260806-1212 gegengeprüft: mit
`AppleLanguages = ["de"]` und `AppleLocale = "de_DE"` über `registerDefaults:`
bleibt `preferredLocalizations` bei `["en"]` und die Ausgabe bei `Zero KB`. Foundation
schneidet die Sprachwahl gegen die **Sprachen des Bündels**, und die stehen allein in
der `Info.plist`. Ohne Eintrag dort gibt es nichts zu schneiden.

**Adressat: `ontocoder`.** `resources/Info.plist` ist eine Bündelbeschreibung und
gehört nicht dem `coder`; deshalb steht hier eine Meldung und keine Reparatur. Zu
prüfen ist dabei zweierlei, weil die Änderung über diese Spalte hinaus wirkt: erstens,
dass `xtask` die Datei unverändert in das Bündel trägt und die Versionsersetzung nicht
stört, zweitens, dass keine andere Foundation-Ausgabe von KRK dadurch eine unerwünschte
deutsche Fassung bekommt. KRK hat heute keine eigenen `.lproj`-Ordner; der Schlüssel
sagt Foundation allein, welche Sprachen das Programm anbietet.

**Aufgefallen bei:** der Bearbeitung der sechs offenen Oberflächendefekte am
260806. Der ursprüngliche Defekt bleibt offen und behält seinen Gegenstand; diese
Meldung erweitert seinen Umfang und schließt einen seiner beiden Wege aus.

---
Resolved: `resources/Info.plist:17-49` führt jetzt `CFBundleLocalizations` mit `de`
an erster und `en` an zweiter Stelle. Gemessen am gebauten Bündel treffen alle sechs
Werte die vorhergesagte Spalte: `0 KB`, `1 Byte`, `512 Byte`, `999 Byte`, `1 KB`,
`10 KB`; `preferredLocalizations` wechselt von `["en"]` auf `["de"]`. Die beiden
aufgetragenen Prüfungen sind erledigt. **Erstens** trägt `xtask` die Datei
unverändert: `diff resources/Info.plist target/KRK.app/Contents/Info.plist` meldet
genau die Versionszeile, `plutil -p` zeigt Schlüssel, Reihenfolge und
`CFBundleExecutable = krk`; die Versionsersetzung ist ein `str::replace`
(`xtask/src/bundle.rs:200-210`) und der Binärname eine Textsuche am Schlüsselnamen
(`:241-250`), beide vom neuen `<array>` unberührt. **Zweitens** ändert sich außer den
Byte-Angaben nur eine weitere Ausgabe, und sie ist erwünscht: die
Papierkorb-Fehlermeldung aus `appkit/papierkorb.rs:58` kommt deutsch
(`Die Datei „x" existiert nicht.` statt `The file "x" doesn't exist.`). Nicht
betroffen sind die Spalte „Änderungsdatum" (`NSDateFormatter` folgt der Systemregion,
gemessen `02.02.26, 03:40` vor wie nach), das Hauptmenü (`--menue-protokoll` liefert
dieselben sieben Zeilen) und die Standardknöpfe der Blätter, deren Beschriftungen KRK
in `appkit/blaetter/mod.rs:346` und `appkit/hinweis.rs:69` alle selbst setzt.
`make check` grün, `make bundle` gebaut und signiert.
Bericht: `history/260807-0743-ontocoder-die-sprache-des-buendels-und-die-pfadzitate.md`.
