# Der Kopf von sys.rs nennt vier Fremdaufrufe und bindet acht Funktionen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, beim Nachziehen der beiden Aufzählungen aus `260810-0955`
**Betroffen:** `crates/krk-core/src/verzeichnis/sys.rs:1` (Modulkopf, erste Zeile) und `crates/krk-core/src/verzeichnis/sys.rs:9-14` (das Diagramm der Fremdaufrufe)
**Cross-references:** `issues/260810-0955_*_die-aufzaehlung-der-fremdaufrufe-nennt-drei-und-es-sind-vier.md` (derselbe Defekt in den beiden Köpfen außerhalb des Moduls, dort behoben), `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md` (Anlass der `fcntl`-Bindung)

---

## Der Befund

Der Modulkopf von `verzeichnis/sys.rs` beginnt mit "Die Systemschicht des Kerns: die vier Fremdaufrufe, die KRK braucht" und führt darunter ein Diagramm mit genau vier Zeilen, eine je Aufruf. Gebunden sind acht Funktionen, alle acht in diesem Modul und alle acht auch gerufen:

| Bindung | Zeile | Gerufen in |
|---|---|---|
| `getattrlistbulk` | 109 | `Schwungleser::naechster_schwung`, Zeile 197 |
| `copyfile` | 395 | `mit_zustand_kopieren`, Zeile 592 |
| `copyfile_state_alloc` | 403 | `datei_kopieren`, Zeile 538 |
| `copyfile_state_free` | 406 | `datei_kopieren`, Zeile 547 |
| `copyfile_state_set` | 409 | `mit_zustand_kopieren`, Zeilen 567 und 575 |
| `copyfile_state_get` | 412 | `statusrueckruf` Zeile 504, `wurde_geklont` Zeile 623 |
| `renamex_np` | 415 | `im_datentraeger_verschieben`, Zeile 656 |
| `fcntl` | 684 | `blockierend_stellen`, Zeilen 745 und 754 |

Vier ist damit die Zahl der Schnittstellen und nicht die der Bindungen. Die vier `copyfile_state_*` sind Zubehör von `copyfile(3)` und keine eigene Schnittstelle — ohne sie lässt sich der Fortschrittsrückruf nicht setzen und die Zahl der kopierten Bytes nicht abfragen —, aber sie sind vier weitere `unsafe`-Aufrufe über die Sprachgrenze, und der Kopf zählt Fremdaufrufe.

## Warum das zählt

Es ist derselbe Defekt, den `260810-0955` in `lib.rs` und `verzeichnis/mod.rs` gemeldet hat, eine Ebene tiefer und mit derselben Begründung: eine Aufzählung, die eine Bindung verschweigt, verschweigt genau das, was sie aufzuzählen behauptet. Hier zählt es einen Grad mehr als dort, weil dieses Modul die eine Stelle im Kern mit `#![allow(unsafe_code)]` ist und sein Kopf der Ort, an dem ein Leser die Reichweite dieser Ausnahme nachsieht.

`Low`, weil kein Verhalten daran hängt und die vier verschwiegenen Bindungen fünfzehn Zeilen unter der Zahl im Quelltext stehen, mit ihrer C-Signatur als Doc-Kommentar. Wer den Block liest, sieht sie; die Zahl im Kopf erspart ihm das Lesen und ist dabei falsch.

## Was zu tun ist

Zwei Möglichkeiten, und die Wahl gehört dem Ausführenden. Entweder beide Zahlen nennen, so wie `lib.rs` und `verzeichnis/mod.rs` es seit dem 260810 tun ("vier Schnittstellen und acht gebundene Funktionen, denn `copyfile(3)` braucht seine vier `copyfile_state_*`-Helfer"), oder das Diagramm um eine fünfte Zeile für die Helfer erweitern und die Zahl in der ersten Zeile entsprechend fassen. Kein Verhalten, nur der Modulkopf.

Die erste Möglichkeit hält den Kopf mit den beiden anderen wortgleich und ist deshalb die empfohlene.

## Warum es nicht mitgekommen ist

`crates/krk-core/src/verzeichnis/sys.rs` lag außerhalb der Schreibgrenze des Arbeitspakets, das `260810-0955` behoben hat. Die Grenze war ausdrücklich gesetzt, weil parallel andere Agenten im Baum arbeiten.

---
Resolved: Die empfohlene Möglichkeit gewählt, und beides getan. Die erste Zeile
des Modulkopfs heißt jetzt „die vier Schnittstellen, die KRK braucht, und die
acht Funktionen, die sie binden"; das Diagramm trägt unter `copyfile(3)` eine
eingerückte Zeile `copyfile_state_{alloc,free,set,get}`, sodass die vier Helfer
sichtbar sind, ohne eine fünfte Schnittstelle zu behaupten. Darunter steht der
Satz, der `lib.rs` und `verzeichnis/mod.rs` seit dem 260810 wortgleich tragen:
„vier Schnittstellen und acht gebundene Funktionen, denn `copyfile(3)` braucht
seine vier `copyfile_state_*`-Helfer." Dazu die Begründung aus diesem Datensatz —
ohne die Helfer läßt sich der Fortschrittsrückruf nicht setzen und die Zahl der
kopierten Bytes nicht abfragen, eine eigene Schnittstelle sind sie deshalb nicht,
vier weitere Aufrufe über die Sprachgrenze schon — und der Verweis darauf, daß
die dritte Stelle dieser Defekt nachgezogen hat. Ein Satz sagt außerdem, wo die
acht stehen (die drei `unsafe extern "C"`-Blöcke des Moduls) und daß alle acht
gerufen sind. `lib.rs` und `verzeichnis/mod.rs` blieben unangetastet; sie lagen
außerhalb der Schreibgrenze und tragen die Zahl schon richtig.

**Selbst nachgezählt, nicht übernommen, und die Tabelle oben stimmt in jeder
Zeile.** Acht Bindungen in drei `unsafe extern "C"`-Blöcken, aus vier
Schnittstellen; alle acht auch gerufen. Die Zeilennummern der Tabelle trafen vor
dieser Änderung genau (109, 395, 403, 406, 409, 412, 415, 684) und sind durch die
dreizehn neuen Kopfzeilen jetzt um dreizehn verschoben. Zwei Stellen zählen
absichtlich **nicht** mit, und deshalb stehen sie hier: `type Statusrueckruf =
extern "C" fn(…)` ist ein Typalias und keine Bindung, und `extern "C" fn
statusrueckruf(…)` ist eine Funktion von KRK mit C-Aufrufweg, also ein Übergang
in der anderen Richtung. Wer künftig nachzählt und auf zehn kommt, hat diese
beiden mitgenommen.

Kein Verhalten geändert, nur der Modulkopf. Abnahme grün: `cargo build
--workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`,
`cargo fmt -p krk-core -- --check`.
