Eine Datei über 4 GiB lässt sich in kein Zip kopieren
---
Ein neuer Zip-Eintrag entsteht über die Kiste ohne `large_file(true)`. `zip-8.6.0` bricht das Schreiben ab, sobald ein Eintrag mehr als `u32::MAX` Bytes aufnimmt. `set_auto_large_file` ändert daran nichts, denn es wirkt allein auf den Datenbeschreiber. Der Umbau scheitert deshalb an jeder Datei über 4 GiB. Das Archiv bleibt unverändert, der Bericht nennt den englischen Wortlaut der Kiste, und der Nutzer kann große Dateien nie in ein Zip legen. Der Plan verspricht in E13 „ZIP64 je Eintrag über `set_auto_large_file`“.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium (Funktion fehlt für große Dateien; kein Schaden am Archiv)
**Domain:** code
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`, `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`

**Belege**

- Nachgestellt an einer Kopie des Stands `8c8579af`: eine dünn belegte Datei mit 4 GiB + 4096 Bytes, kopiert in ein kleines Zip. Bericht: `uebersprungen: [… g.zip, grund: "das Archiv ist unverändert geblieben: Large file option has not been set"]`.
- `crates/krk-core/src/archiv/umbau.rs`, `neuen_zip_eintrag`: `zipform::zeit_eintragen(FullFileOptions::default(), …).unix_permissions(neuer.rechte)`, ohne `large_file`.
- `zip-8.6.0/src/write.rs`, `Write for ZipWriter`: `if self.stats.bytes_written > spec::ZIP64_BYTES_THR { … if !is_large_file { … "Large file option has not been set" } }`. `set_auto_large_file` (`write.rs:1023`) liest allein `write_data_descriptor` (`write.rs:1430`).
- `inference:`, nicht nachgestellt: `crates/krk-core/src/operation/zippen.rs` baut seine Wahl auf dieselbe Art (`ZipWriter::new(…).set_auto_large_file()`, kein `large_file(`) und liegt damit unter derselben Grenze. Der Befund wäre dort älter als dieses Paket.

**Behebung (Richtung)**

Die Größe der Quelle steht vor `start_file` fest (`Geoeffnet::groesse`). Ab `u32::MAX` setzt der Umbau `large_file(true)`, sonst nicht, damit kleine Einträge kein ZIP64-Feld bekommen. Dieselbe Regel gehört an eine Stelle in `zipform` und wird auch von `zippen.rs` genutzt.

**Abnahme**

Eine Probe, mit `#[ignore]` wegen der Laufzeit, kopiert eine dünn belegte Datei über 4 GiB in ein Zip. Danach liest `/usr/bin/unzip -t` das Ergebnis ohne Fehler, und das Inhaltsverzeichnis nennt die volle Größe. Ist der Befund an `zippen.rs` bestätigt, gilt dieselbe Probe für das Zippen.

---
Resolved: `crates/krk-core/src/archiv/zipform.rs`, neue Regel `braucht_zip64` und `grosse_form_waehlen` (ZIP64 ab `u32::MAX` minus 1/256, weil gepackte Daten etwas groesser sein koennen, und bei unbekannter Groesse), genutzt in `archiv/umbau.rs`, `neuen_zip_eintrag`, und in `operation/zippen.rs`, `dateiwahl`; der Befund an `zippen.rs` war bestaetigt (`set_auto_large_file` allein), sein Kommentar ist berichtigt. Proben: `eine_datei_ab_vier_gib_und_eine_unbekannter_groesse_bekommen_zip64` im Pruefmodul von `zipform.rs` (schnell, prueft das ZIP64-Feld im lokalen Kopf), dazu mit `#[ignore]` wegen der Laufzeit `eine_datei_ueber_vier_gib_kommt_in_ein_zip` (`tests/archiv_operation.rs`) und `eine_datei_ueber_vier_gib_laesst_sich_zippen` (`tests/operation.rs`), je mit einer duennen Datei von 4 GiB + 4096 Bytes und `/usr/bin/unzip -t`.
