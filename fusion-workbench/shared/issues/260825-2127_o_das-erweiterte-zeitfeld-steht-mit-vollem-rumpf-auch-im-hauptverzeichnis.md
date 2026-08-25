# Das erweiterte Zeitfeld steht mit vollem Rumpf auch im Hauptverzeichnis des Archivs

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/operation/zippen.rs:680-703` (`zeit_uebernehmen`, der Kommentar bei `:690-692`), `:610-616` (`FELD_ERWEITERTE_ZEIT`); `Cargo.toml:176-201` (die Messtabelle); `crates/krk-core/tests/operation.rs:1416-1462` (`jeder_eintrag_traegt_beide_zusatzfelder_mit_der_epochensekunde`)

---

## Was ist

KRK schreibt `0x5455` mit neun Byte Rumpf: ein Kennzeichenbyte mit beiden gesetzten Bits,
danach Änderungs- und Zugriffszeit zu je vier Byte. Übergeben wird es mit
`add_extra_data(kennung, rumpf, false)`, also für den lokalen Kopf. Der Kommentar daneben
sagt, was danach geschieht:

```
// Beide Felder gehen in den lokalen Kopf; die Kiste wiederholt ihn im
// Hauptverzeichnis, ein zweiter Eintrag als "nur zentral" stuende dort
// also doppelt.
```

Das trifft zu. `zip-8.6.0/src/write.rs:2491-2523` schreibt beim Abschluss den lokalen
Zusatzfeldblock (nur um Ausrichtungsfelder gekürzt) unverändert in den Zentraleintrag und
hängt einen etwaigen `central_extra_field` daneben. Der Zentraleintrag trägt damit dasselbe
neun Byte lange `0x5455`.

## Warum das ein Befund ist

`inference:` **Die Info-ZIP-Festlegung zu `0x5455` sieht für den Zentraleintrag allein die
Änderungszeit vor**, also fünf Byte: Kennzeichenbyte und `ModTime`; die Zugriffs- und
Erzeugungszeit gehören nur in den lokalen Kopf. Diese Aussage stammt aus meiner Kenntnis des
Formats und ist in diesem Durchgang **nicht** gegen die Quelle geprüft; was sie entscheidet,
ist `proginfo/extrafld.txt` aus Info-ZIPs eigenem Bestand.

Trifft sie zu, schreibt KRK ein Archiv, dessen Zentraleinträge in diesem einen Feld nicht der
Festlegung folgen. Der praktische Schaden ist erkennbar klein: jedes Zusatzfeld trägt seine
Länge, ein Leser überliest also, was er nicht erwartet, und die Messtabelle in der
Wurzel-`Cargo.toml` weist für `/usr/bin/unzip` und `/usr/bin/ditto` nach, dass beide die
richtige Zeit liefern. Ungemessen bleiben die Werkzeuge, die der Plan selbst als ungemessen
benennt — die Archivierungsfunktion des Finders — und jedes fremde System.

## Was zu tun wäre

Nichts, bevor die Festlegung nachgelesen ist. Danach zwei Wege:

1. **Es bleibt, wie es ist.** Zwei Werkzeuge sind gemessen, beide liefern richtig, und die
   Länge im Feldkopf schützt jeden weiteren Leser vor dem Übermaß.
2. **Der Zentraleintrag bekommt die kurze Form.** `add_extra_data` kennt dafür das dritte
   Argument: ein `0x5455` mit fünf Byte als „nur zentral" neben dem neun Byte langen lokalen.
   Ob die Kiste dann beide schreibt statt den lokalen zu wiederholen, ist am gebauten Archiv
   nachzumessen und nicht anzunehmen — der Kommentar bei `zippen.rs:690` vermutet heute das
   Gegenteil.

Wer 2 wählt, misst mit `archivzusatzfelder` am **Zentraleintrag** und braucht dafür eine
Zählung, die lokalen und zentralen Block auseinanderhält; die vorhandene Probe tut das nicht.

**Schwere:** gering, latent. Kein bekannter Leser nimmt Schaden; der Datensatz hält die Lage
fest, damit sie nicht als Rätsel wiederkehrt, wenn ein drittes Werkzeug abweicht.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.
