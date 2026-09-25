# Die Typprüfung des Editors steht jetzt am Deskriptor

**Status:** Complete
**Agent:** coder
**Anlass:** `issues/260809-1652_*_die-typpruefung-steht-auf-dem-pfad-und-nicht-auf-dem-deskriptor.md`

---

## Was gebaut wurde

`text::datei::oeffnen` prüfte Typ und Größe mit `stat(2)` am **Pfad** und öffnete
danach denselben Pfad ein zweites Mal. Zwischen beiden Aufrufen lag ein Fenster;
wurde der Pfad darin gegen eine benannte Röhre getauscht, hing das `File::open`,
der Arbeitsfaden des Ladevorgangs endete nie, und der Editor öffnete
kommentarlos nichts.

Jetzt öffnet `oeffnen` zuerst und fragt danach den **Deskriptor**:

```text
  vorher:  stat(Pfad) ──> Typ ──> Größe ──> open(Pfad) ──> read
                                            ^ zweite Auflösung des Namens
  jetzt:   open(Pfad) ──> fstat(fd) ──> Typ ──> Größe ──> read
           ^ die einzige Auflösung des Namens
```

Das Fenster ist nicht bewacht, sondern nicht mehr vorhanden: nur ein Aufruf löst
den Namen noch auf.

Neu in `verzeichnis::sys` steht `ohne_warten_oeffnen`. Sie öffnet mit
`O_NONBLOCK`, damit eine benannte Röhre den Aufruf nicht anhält, und nimmt das
Kennzeichen über `fcntl(F_GETFL)` plus `fcntl(F_SETFL)` wieder ab, bevor sie den
Deskriptor herausgibt. Sofort und nicht erst vor dem Lesen: die Gefahr steht
allein am `open`, und ein `F_SETFL` hält nichts an.

## Drei Entscheidungen, die im Code begründet stehen

**Keine neue Kiste.** `crates/krk-core/Cargo.toml` ist unangetastet.
`verzeichnis/sys.rs` schreibt die vierte Bindung selbst, wie die drei
bestehenden: `O_NONBLOCK`, `F_GETFL` und `F_SETFL` als eigene Konstanten, gegen
`sys/fcntl.h` der installierten SDK geprüft (`0x4`, `3`, `4`).

**`fcntl` ist variadisch deklariert.** Apples arm64-ABI übergibt variadische
Argumente auf dem Stapel und feste in Registern. Drei feste Argumente wären
derselbe Aufruf mit dem falschen Argumentweg, und der Übersetzer hätte keine
Gelegenheit, das zu bemerken.

**Die Ausnahme bleibt eine.** Aller `unsafe`-Code steht in `verzeichnis/sys.rs`,
hinter einer sicheren Hülle. `text/datei.rs` sieht kein Kennzeichen und keinen
Deskriptor als Zahl.

## Der Nachweis von S10 ist neu geschnitten

Der alte Nachweis "abweisen, ohne zu lesen" hing an den Rechten: zwei Löcher,
beide mit Rechten 000, um ein Byte verschieden. Nach dem Umbau ist er nicht
ungenau, sondern **gegenstandslos** — POSIX prüft das Leserecht beim `open`, also
scheitern beide schon dort.

Beim Neuschneiden kam ein Ergebnis heraus, das im Befund nicht stand: **im
Ergebnis sind "vor dem Lesen geprüft" und "nach dem Lesen geprüft" nicht zu
unterscheiden**, weil die Schranke `take(EDITORGRENZE + 1)` auch den zweiten Fall
mit `ZuGross` beantwortet. Beobachtbar ist allein der Aufwand. Der Nachweis kann
deshalb nur eine Ressourcenschranke sein, und er steht jetzt dort, wo gemessen
wird:

| Probe | was sie hält |
|---|---|
| `eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden` | der eine Byte Unterschied entscheidet; auf der Grenze wird vollständig gelesen |
| `zwei_gigabyte_werden_ohne_arbeitsspeicher_abgewiesen` | "ohne gelesen zu werden", jetzt gegen das Lesen von 16 MB auf derselben Maschine statt gegen eine halbe Sekunde |
| `eine_benannte_roehre_wird_abgewiesen_und_haelt_das_oeffnen_nicht_an` | die `mkfifo`-Probe, die der Befund verlangt hat |
| `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` | das Wettrennen selbst |
| `eine_gesperrte_datei_kommt_mit_dem_systemfehler_zurueck` | die Deckung, die der alte Schnitt beiläufig mitbrachte |

## Zwei Sackgassen der Wettrennen-Probe, beide gemessen

Die Probe wurde gegen die **alte** Reihenfolge geprüft, weil eine Probe, die
unter beiden Bauarten durchläuft, nichts über den Umbau sagt. Zwei Fassungen
liefen durch und belegten nichts:

1. **Ein Tauscher, der wegbenennt und zurücklegt**, lässt den umkämpften Pfad die
   meiste Zeit fehlen: 12.830 von 20.000 Durchläufen sahen `ENOENT`, 818 eine
   Datei, 6.352 eine Röhre, und das Fenster wurde nie getroffen. Jetzt liegt an
   dem Pfad immer etwas, gelegt über eine harte Verknüpfung und ein `rename`
   darüber. Dabei ist eine Falle zu umgehen, die im Kopf der Probe steht:
   `rename` auf denselben Inode tut nichts und meldet Erfolg, wodurch die
   Zwischenverknüpfung liegenblieb und der Tauscher nach **einem** Tausch aufhörte.
2. **Eine feste Zahl von Durchläufen** genügt nicht. Im Profil `release` ist der
   Lesefaden so schnell, dass der Tauscher nur 994 Tausche schafft; die eingebaute
   Zählung hat das gefangen und die Probe ausfallen lassen. Der Lesefaden läuft
   jetzt, bis Durchläufe **und** Tausche erreicht sind, mit einer Obergrenze als
   Notbremse.

Die dritte Fassung fällt mit der alten Reihenfolge in beiden Profilen aus (je ein
Lauf): der Lesefaden bleibt im `File::open` stehen, die Zeitschranke von 15
Sekunden schlägt zu. Mit der neuen läuft sie in beiden Profilen je dreimal durch.

## Geänderte Dateien

- `crates/krk-core/src/text/datei.rs` — `oeffnen` und sein Kopf
- `crates/krk-core/src/verzeichnis/sys.rs` — `ohne_warten_oeffnen`,
  `blockierend_stellen`, die `fcntl`-Bindung, eine Modulprobe
- `crates/krk-core/tests/text.rs` — Fall 10 neu geschnitten, drei Proben dazu

`Cargo.toml` war nicht nötig. `crates/krk-ui/**` und `text/suche.rs` blieben
unberührt.

## Abnahme

Stand 260810-1002:

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo fmt --all --check` | 0 |
| `cargo fmt -p krk-core --check` | 0 |

Zwei Zwischenstände gehören in dieses Protokoll, weil die vier Kommandos den
ganzen Baum sehen und parallel an `crates/krk-ui/` gearbeitet wurde. Um 0950
fielen `cargo test --workspace` und `cargo clippy --workspace --all-targets` mit
101 aus, an einem `CStr`-Fehler in `crates/krk-ui/src/appkit/editor.rs:2717`; um
0955 fiel `cargo fmt --all --check` mit 1 aus, an derselben Datei in Zeile 3153.
Beides lag außerhalb der Dateigrenze und war zur Abnahme behoben. `cargo build
--workspace` war durchgehend 0, der Fehler saß also in einem `cfg(test)`-Teil.

Anzumerken bleibt ein eigener Fehlgriff: dieses Arbeitspaket hat zwischendurch
`cargo fmt --all` gefahren statt `cargo fmt -p krk-core`. Das kann jene fremde
Datei berührt haben, bevor sie erneut geändert wurde. Wer unter einer Dateigrenze
arbeitet, formatiert die eigene Kiste und nicht den Baum.

## Offen geblieben

`crates/krk-core/src/lib.rs:11` und `crates/krk-core/src/verzeichnis/mod.rs:11`
zählen die Fremdaufrufe des Kerns namentlich auf und nennen drei. Beide liegen
außerhalb der Dateigrenze; der Defekt ist
`issues/260810-0955_o_die-aufzaehlung-der-fremdaufrufe-nennt-drei-und-es-sind-vier.md`.
