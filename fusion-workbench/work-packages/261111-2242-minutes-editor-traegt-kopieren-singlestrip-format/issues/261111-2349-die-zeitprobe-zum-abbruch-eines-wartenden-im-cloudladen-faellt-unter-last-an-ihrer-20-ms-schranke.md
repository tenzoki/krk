Die Zeitprobe zum Abbruch eines Wartenden im Cloudladen fällt unter Last an ihrer 20-ms-Schranke
---
`cloudladen::tests::ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf` (`crates/krk-ui/src/cloudladen.rs`) verlangt, dass der wartende Faden binnen 20 ms nach dem Setzen der Abbruchmarke zurückkehrt. In einem von zwei unmittelbar aufeinanderfolgenden Läufen von `make check` am 261111 brauchte er 29,5 ms, und der Lauf brach mit Exit 2 ab; der zweite Lauf am selben Baum war grün. Die Datei war in beiden Läufen unverändert gegenüber HEAD.
---
**Filed by:** orchestrator, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261025-0010-die-zeitprobe-zum-abbruch-in-einer-500-mb-datei-scheitert-auch-ohne-aenderung-in-drei-von-fuenf-laeufen.md, 260823-1210_*_ein-make-check-von-neun-ist-mit-2-abgebrochen-und-hat-sich-nicht-wiederholt.md

Beleg: die Ausgabe des ersten Laufs nennt `panicked at crates/krk-ui/src/cloudladen.rs:236:9: der Wartende brauchte 29.497917ms nach dem Abbruch`, bei `1535 passed; 1 failed` im Prüfziel `-p krk-ui --bin krk`. Die Probe misst eine Wanduhrspanne, während die übrigen Proben des Ziels parallel laufen; die Schranke von 20 ms ist dem Takt des Wartens entnommen und trägt keinen Zuschlag für die Zuteilung des Fadens.

Abnahme: die Probe besteht in zwanzig aufeinanderfolgenden Läufen von `cargo test -p krk-ui --bin krk` bei voller Parallelität, oder sie prüft die Eigenschaft (der Wartende kehrt ohne Belegung zurück, sobald die Marke steht) ohne eine Wanduhrschranke dieser Größe.

---
Resolved: `crates/krk-ui/src/cloudladen.rs`, Probe `cloudladen::tests::ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf`. Der Betriebscode war richtig (`belegen` wartet je `SCHRITT` und fragt danach die Marke); zu eng war allein die Wanduhrschranke der Probe. Sie prüft jetzt an Zuständen (Zahl der Fragen vor dem Abbruch, genau ein Nein, keine Belegung, Platz weiter gehalten) und trägt als einzige Zeitgrenze die Hängegrenze `HAENGT` von zweihundert Schritten. Zwanzig Läufe von `cargo test -p krk-ui --bin krk` in Folge grün. Nicht mitbehoben: die zwei Proben zu `abwarten` in derselben Datei tragen dieselbe 20-ms-Schranke weiter.
