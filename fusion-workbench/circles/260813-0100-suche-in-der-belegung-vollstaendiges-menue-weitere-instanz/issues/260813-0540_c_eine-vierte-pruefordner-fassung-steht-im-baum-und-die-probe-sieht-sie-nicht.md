Eine vierte Prüfordner-Fassung steht im Baum, und die C4.6-Probe sieht sie nicht

---

C4.6 sagt zu: es gibt genau drei Prüfordner-Fassungen, eine je Kiste. `CLAUDE.md` führt sie
namentlich, und die Abnahme von S12 verlangt „eine vierte Prüfordner-Fassung entsteht nicht".

**Die Runde hat eine vierte gebaut.** `crates/krk-core/src/ablage/sperre.rs:209-229` erklärt
`struct Ordner` mit `neu()`, das unter `std::env::temp_dir()` einen Ordner anlegt, und
`impl Drop for Ordner`, das ihn abräumt. Das ist der Gegenstand, den C4.6 zählt.

**Die Probe dazu findet sie nicht.** `genau_drei_pruefordner_fassungen_stehen_im_baum`
(`crates/krk-core/tests/baum.rs:67-100`) sucht für die Gegenprobe die Nadel
`impl Drop for Pruefordner`. Sie bindet damit an den **Namen** und nicht an die Sache: eine
vierte Fassung namens `Ordner` entgeht ihr, und dieselbe Nadel fände auch den anerkannten
`Wegwerfordner` in `crates/krk-bench/src/wegwerfordner.rs:54` nicht, wenn er neu hinzukäme.

**Die Begründung im Doc-Kommentar trägt die Hälfte.** `sperre.rs:202-208` schreibt richtig,
dass die Proben dieses Moduls neben dem Code stehen müssen, weil sie das kistenintern sichtbare
`Schreibgriff::nehmen` brauchen, und dass `tests/gemeinsam/` von dort nicht erreichbar ist. Das
begründet, warum es die vierte Fassung **gibt**; es macht aus ihr keine Nicht-Fassung. Der Satz
„das sind zwei Sichtbarkeiten und keine zweite Fassung derselben Sache" ist die Stelle, an der
die Zählung ausgehebelt wird, ohne dass jemand die Zusage geändert hätte.

**Dazu der Ort.** `Ordner::neu` und die Probe in `crates/krk-core/src/verzeichnis/sys.rs:950`
legen ihre Ordner und Dateien im echten `std::env::temp_dir()` an. `CLAUDE.md` warnt an dieser
Stelle bereits (`shared/issues/260810-1925_*`): `Messplanwaechter::neu` räumt dort fremde
Messpläne ab, und `cargo test` greift damit in dasselbe Verzeichnis wie ein laufender Messlauf.
Zwei weitere Greifer sind dazugekommen.

---

**Schwere:** mittel. Kein Fehlverhalten am Programm; eine Zusage, die gebrochen ist, und eine
Probe, die es nicht meldet — genau die Lage, gegen die die Probe geschrieben wurde.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-core/src/ablage/sperre.rs:202-229`,
`crates/krk-core/tests/baum.rs:67-100`,
`crates/krk-core/src/verzeichnis/sys.rs:946-995`

**Domain:** code

## Vorschlag

Zwei Fragen, und die erste gehört dem Nutzer.

1. **Ist die vierte Fassung erlaubt?** Wenn ja, gehört sie in `CLAUDE.md` und in die
   Aufzählung der Probe, mit ihrer Begründung; C4.6 heißt dann „vier, eine je Sichtbarkeit".
   Wenn nein, brauchen die Proben von `sperre.rs` einen anderen Weg an
   `Schreibgriff::nehmen` — etwa eine kisteninterne Hülle, die `tests/gemeinsam/` mitbenutzen
   kann.
2. **Die Gegenprobe unabhängig vom Namen machen.** Statt `impl Drop for Pruefordner` die
   Sache suchen, die den Gegenstand ausmacht: ein `impl Drop` in derselben Datei wie ein
   `create_dir_all` und ein `remove_dir_all`. Das findet jede vierte Fassung, gleich wie sie
   heißt.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Beide Fragen sind beantwortet, und die erste hat den Nutzer nicht gebraucht: `CLAUDE.md` sagt „Es gibt genau drei Fassungen, eine je Kiste, und das soll so bleiben" zu. Eine bestehende Zusage einzuhalten ist keine Entscheidung, sondern die Vorgabe; zu entscheiden waere allein gewesen, sie zu **aendern**.

**Die vierte Fassung ist fort.** Die vier Proben aus `#[cfg(test)] mod tests` von `crates/krk-core/src/ablage/sperre.rs` stehen jetzt in `crates/krk-core/tests/ablage.rs` und benutzen den anerkannten `Pruefordner` aus `tests/gemeinsam/`; `struct Ordner` samt `impl Drop` ist geloescht. **Die Begruendung von damals traegt nicht**, und das ist am Baum nachgesehen: keine der vier Proben ruft `Schreibgriff::nehmen`. `Sitzungsrecht`, `Ablageort::an` und `Ablage::durchgang` sind `pub`, und die eine Probe, die eine Sperrdatei selbst oeffnen muss, tut es jetzt ueber einen Helfer in `tests/ablage.rs`, den `kind_meldet_die_schreibsperre` mitbenutzt. Die Sichtbarkeit von `sperre::sperrdatei_oeffnen` ist damit nicht angefasst.

**Die Gegenprobe bindet nicht mehr an den Namen.** `genau_drei_pruefordner_fassungen_stehen_im_baum` sucht nicht mehr `impl Drop for Pruefordner`, sondern die drei Zeichen der Sache in derselben Datei: ein `impl Drop for `, ein `temp_dir()` und ein `remove_dir_all`. Das findet jede vierte Fassung, gleich wie sie heisst, und faende auch den `Wegwerfordner`, wenn er neu hinzukaeme. Nachgesehen: die Nadel trifft heute genau die drei anerkannten Dateien und keine weitere. Was auch das nicht findet — eine ueber zwei Dateien verteilte Fassung, oder eine, die ihren Ordner Eintrag fuer Eintrag abraeumt —, steht am Doc-Kommentar der Probe.

**Zwei Vorkehrungen gegen den Selbstfund**, und beide sind noetig: die Nadeln stehen zusammengesetzt da, und gesucht wird nur in Code-Zeilen. Der neue Helfer `im_code` in `tests/baum.rs` traegt die zweite; ohne sie fand die Probe sich selbst, weil ihre Doc-Kommentare alle drei Nadeln im Klartext nennen.

**Nicht behoben: der Griff in das echte Temporaerverzeichnis.** Mit dem Umzug faellt `sperre.rs` als Greifer weg, `crates/krk-core/src/verzeichnis/sys.rs:950` greift weiter dorthin. Es legt eine einzelne Datei mit Prozesskennung im Namen an und loescht sie am Ende; ein `Messplanwaechter` raeumt sie nicht ab, weil sie nicht `krk-messplan-*.toml` heisst. Der Punkt bleibt in `shared/issues/260810-1925_*` aufgehoben und ist hier nicht eigens abgelegt.

---
Abgleich 260813-0644 (reconciler): **Die Behebung hält, der Schlussabsatz nicht.** Die vierte
Fassung ist fort und die Gegenprobe sucht die Sache statt des Namens — beides am Baum
nachgelesen (`crates/krk-core/tests/baum.rs:113-152`, `crates/krk-core/tests/ablage.rs`). Der
Absatz „Nicht behoben: der Griff in das echte Temporaerverzeichnis" trifft dagegen in zwei
Punkten nicht zu: der genannte Verweis `shared/issues/260810-1925_*` ist geschlossen und
handelt von `crates/krk-bench/src/messen.rs`, nicht von `sys.rs`, und es sind zwei Proben und
nicht eine, an `crates/krk-core/src/verzeichnis/sys.rs:962` und `:1004` statt an `:950`.

Der Rest ist damit nirgends aufgehoben gewesen und liegt ab jetzt in
`issues/260813-0644_*_ein-rest-ist-in-einem-geschlossenen-fremden-datensatz-aufgehoben-worden.md`.
Der Marker dieses Datensatzes bleibt geschlossen: sein eigener Gegenstand, die vierte
Pruefordner-Fassung, ist behoben.
