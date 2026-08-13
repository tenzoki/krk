Ein ausdrücklich nicht behobener Rest ist in einem geschlossenen, fremden Datensatz aufgehoben worden

---

Der geschlossene Datensatz
`issues/260813-0540_c_eine-vierte-pruefordner-fassung-steht-im-baum-und-die-probe-sieht-sie-nicht.md`
endet mit einem Absatz „Nicht behoben: der Griff in das echte Temporärverzeichnis" und schiebt
diesen Rest weiter:

> Mit dem Umzug fällt `sperre.rs` als Greifer weg, `crates/krk-core/src/verzeichnis/sys.rs:950`
> greift weiter dorthin. […] Der Punkt bleibt in `shared/issues/260810-1925_*` aufgehoben und
> ist hier nicht eigens abgelegt.

**Der Verweis trägt den Punkt nicht.** Am 260813 nachgesehen:
`shared/issues/260810-1925_c_eine-probe-schreibt-ins-echte-temporaerverzeichnis-und-raeumt-dort-jetzt-fremde-messplaene-ab.md`
ist **geschlossen**, und er handelt von der Probe
`der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung` in
`crates/krk-bench/src/messen.rs`. Von `verzeichnis/sys.rs` steht dort kein Wort. Ein
geschlossener Datensatz über eine andere Datei hebt nichts auf.

**Die Fundstelle stimmt daneben nicht.** Es ist nicht eine Probe an `sys.rs:950`, sondern es
sind **zwei**: `ein_zweiter_deskriptor_auf_dieselbe_datei_bekommt_die_sperre_nicht`
(`crates/krk-core/src/verzeichnis/sys.rs:962`) und
`ein_geoeffneter_deskriptor_traegt_o_nonblock_nicht_mehr` (`:1004`). Beide legen ihre Datei
über `std::env::temp_dir()` an, beide tragen die Prozesskennung im Namen.

---

**Schwere:** gering. Beide Proben tragen die Prozesskennung im Dateinamen und stoßen deshalb
nicht mit einem gleichzeitigen Lauf zusammen; `Messplanwaechter` räumt sie nicht ab, weil sie
nicht `krk-messplan-*.toml` heißen. Der Befund ist die Buchführung und nicht die Wirkung: die
Zusage aus `CLAUDE.md`, Prüfordner einzelner Testläufe gehörten nicht ins echte
Temporärverzeichnis, gilt weiter, und die zwei benannten Ausnahmen stehen ab jetzt nirgends,
wo jemand sie wiederfindet.

**Warum das zählt.** Ein Rest, den ein schließender Datensatz weiterschiebt, ist genau so viel
wert wie das Ziel, auf das er zeigt. Zeigt er auf einen geschlossenen Datensatz über eine
andere Datei, verschwindet er mit dem Schließen — und die Zahl „18 von 22 behoben" liest sich
danach vollständiger, als sie ist.

**Gefunden:** reconciler, Abgleich der Runde 7, beim Nachlesen der achtzehn `Resolved:`-Zeilen

**Betroffen:** `crates/krk-core/src/verzeichnis/sys.rs:962`, `:1004`,
`issues/260813-0540_*_eine-vierte-pruefordner-fassung-steht-im-baum-und-die-probe-sieht-sie-nicht.md`
(Schlussabsatz)

**Domain:** code

## Zwei Wege

1. **Diesen Datensatz als den Ort nehmen.** Er benennt die zwei Proben, ihre Fundstellen und
   den Grund, aus dem sie heute unschädlich sind. Der Rest ist damit aufgehoben, und der
   Schlussabsatz von `260813-0540` zeigt auf etwas, das ihn trägt.
2. **Die zwei Proben auf die anerkannte Prüfordner-Fassung ihrer Kiste ziehen**
   (`crates/krk-core/tests/gemeinsam/mod.rs`), wie es der Umzug aus `sperre.rs` schon getan hat.
   Das setzt voraus, dass eine Probe, die eine Sperrdatei über einen rohen Deskriptor öffnet,
   das auch unter einem Prüfordner tun kann; nachgesehen ist es nicht.

Weg 1 kostet nichts und ist die Buchführung. Weg 2 beseitigt die Ausnahme und ist eine eigene
kleine Arbeit.
