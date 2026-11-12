Zwei Proben zu `abwarten` im Cloudladen tragen dieselbe freie 20-ms-Wanduhrschranke weiter
---
`cloudladen::tests::abwarten_bricht_ab_oder_wartet_die_dauer` und `cloudladen::tests::ein_nein_mitten_in_der_wartezeit_beendet_sie` (`crates/krk-ui/src/cloudladen.rs`) messen eine Wanduhrspanne gegen 20 ms, während die übrigen Proben des Ziels parallel laufen. Die zweite hat die Bauform der Probe, die am 261111 an derselben Schranke gefallen ist (Faden, Marke, Spanne unter 20 ms). Gefallen ist bisher keine der zwei.
---
**Filed by:** orchestrator, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md, 261112-0826-zwei-offene-defekte-nach-den-protokollknoepfen-beheben.md

Beleg: die `Resolved:`-Zeile des zitierten Datensatzes nennt die zwei Proben als nicht mitbehoben. Die Lösung dort (Fragen zählen statt Millisekunden messen, als einzige Zeitgrenze die Hängegrenze `HAENGT` am Takt des Betriebscodes) passt nach dem Bericht der Behebung unmittelbar.

Abnahme: keine der zwei Proben trägt eine freie Wanduhrschranke dieser Größe mehr; was sie über `abwarten` zusagen (bricht beim ersten Nein ab, wartet sonst die Dauer), halten sie an Zuständen oder an einer Grenze, die am Takt des Betriebscodes hängt und sagt, was sie nicht mehr fängt. `cargo test -p krk-ui --bin krk` läuft zwanzigmal in Folge grün.
