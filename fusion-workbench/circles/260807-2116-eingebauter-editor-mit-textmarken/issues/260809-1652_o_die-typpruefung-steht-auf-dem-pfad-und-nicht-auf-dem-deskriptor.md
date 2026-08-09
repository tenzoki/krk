# Die Typprüfung steht auf dem Pfad und nicht auf dem Deskriptor

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-core/src/text/datei.rs:254-302` (`oeffnen`)
**Cross-references:** C2 Abnahmekriterien 6 und 8, S10, `crates/krk-ui/src/editormodell.rs:336-361` (`Ladevorgang::starten`)

---

## Der Befund

`oeffnen` hält die Reihenfolge, die S10 verlangt, und hält sie richtig: erst
`metadata`, dann `is_file`, dann die Größe, erst danach `File::open` und
`read_to_end` mit `take(EDITORGRENZE + 1)`. Nachgerechnet und bestätigt: die
Größe wird vor dem Lesen erhoben, und die Schranke hält die Grenze ein, statt sie
vorherzusagen. Die Prüfung ist auch wirklich die eine — beide Einstiege und der
Markensprung rufen sie.

**Die Typprüfung sichert aber nicht, wogegen sie geschrieben ist.** Der
Kommentar begründet ihre Lage so (`datei.rs:233-237`):

> **Alles, was keine gewöhnliche Datei ist, fällt hier heraus**, der Ordner
> voran. Diese Frage steht **vor** dem Öffnen und nicht erst vor dem Lesen, weil
> ein `File::open` auf eine benannte Röhre so lange hängt, bis jemand
> hineinschreibt; das wäre eine angehaltene Anwendung ohne Meldung.

Geprüft wird jedoch der **Pfad** (`std::fs::metadata(pfad)`, Zeile 260), und
geöffnet wird danach derselbe **Pfad** noch einmal (`File::open(pfad)`, Zeile
277). Zwischen beiden Aufrufen liegt ein Fenster, in dem der Pfad auf etwas
anderes zeigen kann. Wird er in dieser Spanne durch eine benannte Röhre ersetzt,
läuft genau das ein, was der Kommentar ausschließen will: `File::open` blockiert.

Dasselbe gilt für die Größe: `angaben.len()` gehört zu der Datei, die zum
Zeitpunkt von `metadata` da war. Für den Fall, dass die Datei wächst, hat S10
die Schranke `take(EDITORGRENZE + 1)` nachgezogen — für den Fall, dass sie
**ausgetauscht** wird, gibt es keine.

## Warum das zählt

Das sechste Abnahmekriterium von C2 lautet, eine Datei über der Grenze stehe zu
keinem Zeitpunkt vollständig im Arbeitsspeicher; das achte, ein Ordner werde
immer abgewiesen. Beide gelten dem gewöhnlichen Betrieb und sind dort erfüllt.
Der Kommentar an der Funktion behauptet aber mehr, nämlich eine Eigenschaft der
Bauart, und die trägt nicht.

Die Folge ist kein Absturz, sondern ein stilles Hängen: `oeffnen` läuft auf dem
Arbeitsfaden `krk-editor` (`editormodell.rs:340-351`). Ein blockierender
`File::open` friert die Oberfläche nicht ein, aber der Faden endet nie, der
Ladevorgang bleibt für immer offen, `laedt_noch()` bleibt `true`, und der Editor
öffnet **kommentarlos nichts** — was das zehnte Abnahmekriterium von C2
ausdrücklich ausschließt.

Der Fall braucht ein Wettrennen und ist damit selten. Er ist nicht bloß
theoretisch: der Editor ist gerade für wachsende Protokolldateien gedacht, und
ein Werkzeug, das eine Datei durch eine Röhre ersetzt, ist auf einem
Entwicklerrechner nichts Ausgefallenes.

## Vorschlag

Den Deskriptor prüfen statt des Pfades. Auf macOS öffnet
`OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK)` eine benannte
Röhre, ohne zu blockieren; danach beantwortet `datei.metadata()` Typ und Größe
für **genau die** Datei, die geöffnet wurde:

```rust
let datei = OpenOptions::new().read(true).custom_flags(O_NONBLOCK).open(pfad)…;
let angaben = datei.metadata()…;
// dann is_file, dann die Groesse, dann lesen
```

Das kostet die `libc`-Abhängigkeit in `krk-core`. Sie steht dort schon:
`crates/krk-core/src/verzeichnis/sys.rs` ist die eine Datei mit
`#![allow(unsafe_code)]` und arbeitet mit denselben Systemaufrufen. Ob der
Aufwand die Seltenheit des Falls rechtfertigt, ist eine Abwägung, die nicht
`coderev` trifft.

**Die billigere Antwort**, falls der Aufwand nicht getragen werden soll: den
Kommentar auf das zurücknehmen, was er hält. Er behauptet heute eine Garantie
und liefert eine Wahrscheinlichkeit, und das ist die Sorte Satz, die beim
nächsten Defekt in die Irre führt.

Ungeprüft ist der Fall in beiden Formen: `crates/krk-core/tests/text.rs` deckt
den Ordner ab und die Verknüpfung, aber keine benannte Röhre und kein Gerät.
Eine Probe mit `mkfifo` wäre in derselben Datei billig zu haben und hielte fest,
dass „das ist keine gewöhnliche Datei" wirklich greift.

Gemeldet von: `coderev`, Durchsicht Turn 2.
