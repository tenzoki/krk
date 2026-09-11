Der Modulkopf nennt zwei Proben unter Namen, die es nicht gibt
---
Der Doc-Kommentar an `neuerungen_erheben` (`crates/krk-ui/src/appkit/anwendung.rs:459-461`)
belegt die Zusage „ohne Fassungswechsel wird keine der verglichenen Dateien ein zweites Mal
geöffnet" mit zwei namentlich genannten Proben. Beide Namen treffen nichts im Baum:

| genannt (Zeile) | im Baum |
|---|---|
| `bei_gleichem_merker_wird_keine_datei_geoeffnet` (459) | `bei_gleichem_merker_wird_keine_der_drei_dateien_geoeffnet` (`anwendung.rs:11126`) |
| `bei_neuer_fassung_werden_die_verglichenen_dateien_geoeffnet` (461) | `bei_neuer_fassung_werden_die_drei_dateien_geoeffnet` (`anwendung.rs:11153`) |

Die zweite Stelle im selben Modul, `anwendung.rs:11054-11055`, nennt beide **richtig**. Der
Kopf ist also die einzige falsche von zwei, und wer ihm folgt, sucht zweimal vergeblich.

Die Sache selbst ist in Ordnung: die Proben stehen, sie zählen wirklich die Öffnungen
(`geoeffnete(&ordner)` gegen die leere Liste), und die Gegenprobe eicht den Zähler an drei.
Falsch ist allein der Zeiger.

**Gemessen:** Am 260911 hat die Abnahme der Runde 24 an der Endbedingung zu L4 gehangen, die
genau diese Probe verlangt. Eine Suche nach den zwei im Kopf genannten Namen kam leer zurück,
und die Endbedingung stand kurz als „zählt niemand" da, bevor sie über den Code statt über den
Kommentar nachgeprüft wurde. Das ist der Preis dieses Defekts: ein Zeiger, der ins Leere geht,
ist von einem fehlenden Mechanismus nicht zu unterscheiden.

**Abnahme:** Beide Namen in `anwendung.rs:459-461` stehen so da, wie die Proben heißen, und
`grep -n 'bei_gleichem_merker\|bei_neuer_fassung' crates/krk-ui/src/appkit/anwendung.rs` liefert
für jeden genannten Namen auch eine `fn`-Zeile.
---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Gefunden beim Abarbeiten der Endbedingungen der Runde 24, nicht von der Durchsicht dieser Runde
(die fünf Defektdatensätze daneben stammen von ihr).
