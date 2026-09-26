Die Probe `ein_abgegebenes_sitzungsrecht_ist_wieder_zu_haben` schlug in einem von drei Läufen von `make check` fehl

---

Am 260926 zwischen 10:10 und 10:20 lief `make check` dreimal hintereinander auf demselben Baum; nur `cargo fmt` lag zwischen dem ersten und dem zweiten Lauf, und `krk-core` war in keinem der drei verändert. Der zweite Lauf brach in `cargo test -p krk-core --test ablage` ab:

```
thread 'ein_abgegebenes_sitzungsrecht_ist_wieder_zu_haben' panicked at crates/krk-core/tests/ablage.rs:2854:5:
nach dem Ende des ersten Halters ist das Recht nicht frei geworden
```

Der erste und der dritte Lauf waren grün. Die Probe nimmt das Sitzungsrecht (`krk_core::ablage::sperre::Sitzungsrecht`), gibt es mit `drop` ab und nimmt es sofort wieder; im Fehllauf war es nach dem `drop` noch belegt.

---
**Domain:** code
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Beobachtet, nicht diagnostiziert.** Ein Verdacht und keine Ursache: die Nachbarprobe `nach_einem_absturz_bekommt_die_naechste_instanz_das_sitzungsrecht` startet einen Kindprozess, und ein Kind, das zwischen Fork und Exec den Deskriptor eines parallel laufenden Halters erbt, hält dessen Sperre, bis es ihn schließt. Ob `Sitzungsrecht` mit einer Sperre auf der offenen Dateibeschreibung arbeitet und ob der Kindprozess über einen Weg mit diesem Fenster startet, ist nicht nachgelesen. Gefunden bei der Behebung von `260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md`, die `krk-core` nicht berührt. Wiederholen: `cargo test -p krk-core --test ablage` in einer Schleife unter Last.
