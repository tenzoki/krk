Der Doc-Kommentar von `Unerreichbar` zählt drei Gründe auf, und `stat(2)` scheitert an mehr

---

`crates/krk-core/src/verzeichnis/verweisziel.rs:129-133` beschreibt den dritten Wert so:

```rust
/// Der Name loest sich nicht auf: hinter ihm steht nichts, was von hier aus
/// erreichbar waere. Er zeigt ins Leere, im Ring, oder eine Stufe des
/// Pfades laesst sich nicht durchschreiten.
```

Der zweite Satz liest sich als vollständige Aufzählung, und er ist es nicht. Am
Referenzgerät gemessen scheitert `std::fs::metadata` an mindestens zwei weiteren Zuständen,
die unter keinen der drei genannten fallen.

---

**Gefunden am:** 260815-1844, Stand `60a8ca5`
**Gefunden von:** coderev, Durchsicht des Bereichs `e37a1e3..60a8ca5`
**Schwere:** niedrig. Kein Fehlverhalten am Code. Die Einordnung ist in allen gemessenen
Fällen richtig; falsch ist allein die Beschreibung, und sie steht an der Stelle, an der ein
Leser nachschlägt, was der Wert bedeutet.
**Betroffen:** `crates/krk-core/src/verzeichnis/verweisziel.rs:129-133`, mitbetroffen
`crates/krk-core/tests/verzeichnis.rs:1934-1937`
**Domain:** code

## Am Referenzgerät gemessen

macOS 24.6.0, uid 502, je eine Verknüpfung auf das genannte Ziel, durch eine wortgleiche
Nachbildung des Rumpfes von `bestimmen` (Stand `7fae5ba`):

| Zustand | `bestimmen` liefert | fällt unter einen der drei Gründe |
|---|---|---|
| Verknüpfung ins Leere | `Unerreichbar {"No such file or directory (os error 2)"}` | ja, „ins Leere" |
| Ring aus zwei Verknüpfungen | `Unerreichbar {"Too many levels of symbolic links (os error 62)"}` | ja, „im Ring" |
| Zwischenverzeichnis mit Modus `000` | `Unerreichbar {"Permission denied (os error 13)"}` | ja, „nicht durchschreiten" |
| Kette aus 40 Verknüpfungen **ohne Ring** | `Unerreichbar {"Too many levels of symbolic links (os error 62)"}` | **nein** |
| Namensteil mit 300 Zeichen | `Unerreichbar {"File name too long (os error 63)"}` | **nein** |
| Verknüpfung auf `datei.txt/unterpfad` | `Unerreichbar {"Not a directory (os error 20)"}` | nur bei weiter Lesart |

Die vierte Zeile ist der Fall, der die Aufzählung am deutlichsten sprengt: `ELOOP` entsteht
auf macOS ab `SYMLOOP_MAX` (32) aufgelösten Verknüpfungen, ganz ohne Ring. Eine Kette dieser
Länge ist kein Kunststück — verschachtelte Verknüpfungsbäume erzeugen sie beiläufig.

## Die Zusicherung eine Ebene darüber bleibt heil

`verweisziel.rs:105-118` sagt zu, die drei Werte seien „überschneidungsfrei und vollständig
… auch für die Zustände, die die Werte benennen". Das trägt weiterhin: der `Err`-Zweig ist
ein Auffangzweig über jeden Fehlschlag von `stat(2)`, und der Satz „hinter ihm steht nichts,
was von hier aus erreichbar wäre" stimmt für den zu langen Namen wie für die zu lange Kette.
Angefasst wird also nicht der Schnitt, sondern der Satz, der ihn erläutert.

## Dieselbe Verengung steht in einer Probe

`crates/krk-core/tests/verzeichnis.rs:1934-1937` sagt zur Ring-Probe: „`ELOOP` ist der
zweite Weg, auf dem das Aufloesen scheitert, und er kommt aus demselben `open(2)` wie das
fehlende Ziel". Zwei Angaben darin tragen nicht mehr. Der Aufruf ist seit `7fae5ba` kein
`open(2)` mehr — diese Hälfte gehört zu
`shared/issues/260815-1752_*_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md`,
wo sie im Abgleich vom 260815-1844 nachgetragen ist. Die andere Hälfte gehört hierher:
`ELOOP` ist nicht der zweite von zwei Wegen, und er bedeutet nicht „Ring".

## Vorschlag

Den erläuternden Satz von einer Liste in eine Regel umschreiben, mit den Fällen als
Beispielen statt als Aufzählung: `Unerreichbar` trägt jeden Fehlschlag von `stat(2)` am
Pfad, also alles, was die Auflösung des Namens verhindert — ein fehlendes Ziel, eine zu
lange oder ringförmige Verknüpfungskette, ein nicht durchschreitbarer Zwischenschritt, ein
zu langer Name. Damit deckt der Satz auch den nächsten `errno`, den niemand aufgezählt hat.
Bei der Ring-Probe denselben Halbsatz nachziehen.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und die Directive keiner Runde.

---
Resolved: Der erlaeuternde Satz an `verweisziel.rs` ist eine Regel mit Beispielen statt einer
Liste: der Wert traegt **jeden** Fehlschlag von `stat(2)` am Pfad, haeufig ein fehlendes Ziel,
eine ringfoermige oder schlicht zu lange Verknuepfungskette, eine Stufe ohne
Durchschreitrecht oder ein zu langer Name, „das sind Beispiele und keine Liste". Keine
`errno`-Aufzaehlung, die selbst wieder veraltet. Bei der Ring-Probe
(`tests/verzeichnis.rs`) ist beides nachgezogen: der Aufruf heisst `stat(2)`, und ein Satz
haelt fest, dass macOS `ELOOP` ab `SYMLOOP_MAX` auch ohne Ring meldet.

Nicht angefasst und als eigener Datensatz gefuehrt: dieselbe dreigliedrige Aufzaehlung steht
ein drittes Mal in `crates/krk-ui/src/appkit/tabelle.rs:1432`, ausserhalb der Betroffen-Liste
dieses Datensatzes
(`shared/issues/260815-1858_*_die-dritte-aufzaehlung-der-unerreichbar-gruende-steht-im-einstiegsweg-und-ist-dieselbe-verengung.md`).
