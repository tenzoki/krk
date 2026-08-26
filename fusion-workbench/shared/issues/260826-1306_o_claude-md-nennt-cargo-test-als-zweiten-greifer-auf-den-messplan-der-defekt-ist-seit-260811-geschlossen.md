CLAUDE.md nennt `cargo test` als zweiten Greifer auf den Messplan; der Defekt ist seit dem 260811 geschlossen

---

`CLAUDE.md` schreibt unter „Was man nicht sieht": „Vorausgesetzt ist dabei, dass nie zwei Läufe
zugleich auf dieses Verzeichnis greifen, und der zweite Greifer ist nicht nur ein zweiter
Messlauf: die Probe `der_messplan_traegt_die_pruefsitzung_…` ruft `plan_schreiben`, also räumt
auch `cargo test` ab" — mit Verweis auf `shared/issues/260810-1925_*`. Dieser Datensatz trägt
seit dem 260811 den Marker `_c_`, und der Baum trägt den zweiten Greifer nicht mehr.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Mittel
**Betroffen:** `CLAUDE.md`
**Cross-references:** `shared/issues/260810-1925_c_eine-probe-schreibt-ins-echte-temporaerverzeichnis-und-raeumt-dort-jetzt-fremde-messplaene-ab.md`

## Nachgemessen am Baum, HEAD 4a57028

`plan_schreiben` (`crates/krk-bench/src/messen.rs:1661`) ist die einzige Stelle im ganzen Baum,
die `std::env::temp_dir()` für den Messplan wählt, und sie hat genau **einen** Rufer:
`Gesamtlauf::fahren` (`messen.rs:1029`). Ein `grep -rn 'plan_schreiben' crates/ xtask/` findet
sonst nur Doc-Kommentare.

Die genannte Probe `der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung` geht
seit der Behebung über `plan_in_verzeichnis_schreiben` und einen `Wegwerfordner`
(`messen.rs:2720-2721`); die zweite Messplan-Probe
`ein_neuer_waechter_raeumt_fremde_plaene_ab_und_laesst_den_eigenen_stehen` geht über
`Messplanwaechter::in_verzeichnis` mit demselben Wegwerfordner (`messen.rs:2769`). Beide fassen
das echte Temporärverzeichnis nicht mehr an. Der `Resolved:`-Vermerk des geschlossenen
Datensatzes hält das Gegengeprüfte fest: „eine fremde `krk-messplan-999999.toml` im echten
Temporärverzeichnis überlebt ein volles `make check`."

## Warum das gemeldet und nicht nur bemerkt wird

Der Satz steht nicht in einem Nebensatz, sondern in dem Abschnitt, den `CLAUDE.md` als „Eigen-
schaften, von denen jede schon einmal eine Sitzung gekostet hat" führt. Er tut zwei Dinge
gleichzeitig falsch: er warnt vor einer Wechselwirkung, die es nicht gibt, und er verweist zum
Beleg auf einen Datensatz, der genau das Gegenteil aussagt. Wer ihm folgt, verzichtet ohne Grund
auf `make check` während eines Messlaufs — oder, schlimmer, liest den `_c_`-Datensatz als
irrtümlich geschlossen und öffnet ihn wieder.

**Die Voraussetzung selbst bleibt richtig und muss stehen bleiben:** zwei gleichzeitige
**Messläufe** räumen einander weiterhin den Plan ab, und der Doc-Kommentar an
`Messplanwaechter::in_verzeichnis` (`messen.rs:1603-1619`) schreibt beides sauber aus — die
weiterhin geltende Zusage und den weggefallenen zweiten Beteiligten. Nur der Nachsatz über
`cargo test` gehört gestrichen.

## Denkbarer Weg

In `CLAUDE.md` den Halbsatz ab „und der zweite Greifer ist nicht nur ein zweiter Messlauf" bis
zum Verweis auf `260810-1925` streichen und die Voraussetzung stehen lassen. Der Doc-Kommentar im
Code ist bereits die richtige Fassung und kann als Vorlage dienen.
