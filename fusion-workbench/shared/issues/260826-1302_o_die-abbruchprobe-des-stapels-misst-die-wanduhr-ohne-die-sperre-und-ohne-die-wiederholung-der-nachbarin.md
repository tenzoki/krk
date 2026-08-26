Die Abbruchprobe des Stapels misst die Wanduhr ohne die Sperre und ohne die Wiederholung der Nachbarin

---

`ein_abbruch_im_stapel_kehrt_binnen_100_ms_zurueck_und_meldet_die_umbenannten` (`crates/krk-core/tests/operation.rs:715-768`) hält dieselbe 100-ms-Zusage aus C4 wie ihre Nachbarin über der 500-MB-Datei — aber in **einem** Versuch, ohne `ZEITMESSUNG` und damit neben jeder anderen Probe der Datei, die gerade 500 MB durch dasselbe Dateisystem schiebt. Die Nachbarin trägt fünfundzwanzig Zeilen Doc-Kommentar darüber, warum genau das nicht trägt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/operation.rs:715-768`; `:47-48` und `:20-25` (`ZEITMESSUNG` samt Modulkopf); `:306-355` (die Begründung der Nachbarin)

## Die Zusicherung

```rust
// operation.rs:748-751
assert!(
    bis_zur_rueckkehr < Duration::from_millis(100),
    "der Abbruch kam nach {bis_zur_rueckkehr:?} zurueck, erlaubt sind 100 ms"
);
```

Ein Versuch, absolute Wanduhr, kein `let _reihum = ZEITMESSUNG.lock()`.

## Was die Nachbarin über genau diesen Bau sagt

`der_abbruch_mitten_in_einer_500_mb_datei_kehrt_binnen_100_ms_zurueck` (`:356`) hält dieselbe Zahl aus derselben Fähigkeit und begründet ausführlich, warum ein einzelner Versuch sie nicht messen kann (`:306-355`):

> „Ein einzelner Versuch auf einer belasteten Maschine misst also die Platte und nicht die Anwendung, und genau diese Lage stellt `make frisch` her … Unter kuenstlicher Platten- und Rechenlast ueberschritt ein einzelner Versuch die Frist in 1 von 8 bis 2 von 7 Faellen, in der schlechtesten Reihe also in knapp 30 Prozent."

Sie zieht daraus fünf Versuche und die Sperre. Die Stapelprobe zieht nichts daraus, obwohl sie 5.000 `rename(2)` fährt und dabei nebenher `ein_stapel_ueber_5000_namen_laeuft_durch` weitere 5.000 Dateien anlegt und umbenennt, `ein_baum_mit_500_eintraegen_kommt_vollstaendig_an` einen Baum kopiert und die Zip-Proben 32 MB Rauschen schreiben.

## Der zweite Halbsatz: der Modulkopf zählt falsch

```
//! [`ZEITMESSUNG`] laesst deshalb immer nur eine von beiden laufen.
```

Vier Proben nehmen die Sperre, nicht zwei: `:264`, `:365`, `:459`, `:503`. „Von beiden" stimmte, als es zwei waren; `der_abbruchgriff_wirkt_von_einem_faden_ohne_den_lauf` und `dieselben_500_mb_sind_als_klon_lange_vor_der_frist_fertig` sind seither danebengetreten. Der Absatz darüber („Warum die beiden Zeitmessungen sich gegenseitig ausschliessen") trägt dieselbe Zahl.

## Was die Sperre ohnehin nicht leistet

Auch mit ihr laufen die übrigen rund sechzig Proben der Datei nebenher. Die Sperre trennt die vier großen voneinander und nicht von der übrigen Last des Ziels. Das ist heute vertretbar — die zwei absoluten Messungen an `:290` und `:517` messen eine Metadatenoperation, für die 50 beziehungsweise 100 ms reichlich sind —, aber der Modulkopf sagt es nicht, und ein Leser nimmt aus „immer nur eine von beiden" mehr Ruhe mit, als hergestellt ist.

## Richtung

Für die Stapelprobe die Bauform der Nachbarin: die Sperre nehmen, und entweder mehrere Versuche fahren oder — billiger und genauer — den Abbruch wie in `ein_abbruch_beim_entpacken_laesst_das_fertige_stehen_und_raeumt_die_halbe_datei_weg` (`:2613`) an ein Ereignis statt an eine Wanduhr hängen. Jene Probe schreibt in ihrem Kopf aus, warum: „damit steht fest, dass der Lauf wirklich in ihr steht, und die Probe haengt nicht an der Geschwindigkeit des Geraets."

Für den Modulkopf: die Zahl streichen statt sie auf vier zu setzen, wie es diese Werkbank an einem Dutzend Stellen schon getan hat — die Liste wächst, und eine Zahl davor veraltet.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
