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

---

**Nicht geschlossen. Am 260810-0919 ist die billigere der beiden Antworten
umgesetzt worden, die tragende steht offen und braucht eine Entscheidung.**

**Umgesetzt: der Kommentar behauptet die Garantie nicht mehr.** `oeffnen` trägt
einen neuen Abschnitt "Geprueft wird der Pfad und nicht der Deskriptor". Er sagt,
dass Schritt 2 und 3 `stat(2)` auf den Pfad fragen und Schritt 4 denselben Pfad
ein zweites Mal öffnet, dass die Reihenfolge deshalb eine Prüfung des
gewöhnlichen Betriebs und keine Eigenschaft der Bauart ist, und er trennt die
beiden Fälle: **Wachsen** fängt die Schranke `take(EDITORGRENZE + 1)`, auch wenn
der Pfad in der Spanne auf eine größere Datei zeigt; **ein Austausch gegen eine
benannte Röhre** fängt sie nicht, dann hängt das `File::open` doch und der Editor
öffnet kommentarlos nichts. Der Verweis auf Punkt 2 der bindenden Reihenfolge
steht dort, wo die Garantie früher behauptet wurde.

**Offen: der Deskriptor statt des Pfades.** Drei Gründe, aus denen das nicht
mitgekommen ist, und der dritte ist der, der die Entscheidung verlangt:

1. **Die Kostenschätzung des Befundes stimmt nicht.** Der Befund sagt, `libc`
   stehe in `krk-core` schon. Das ist nicht so: `crates/krk-core/Cargo.toml`
   führt `serde`, `toml` und `icu_collator` und kein `libc`;
   `verzeichnis/sys.rs` schreibt seine `extern "C"`-Blöcke und seine
   Header-Konstanten selbst. `O_NONBLOCK` wäre also eine eigene Konstante oder
   eine neue Kiste im Baum.
2. **Ein Merkmal an `OpenOptions` genügt nicht.** `O_NONBLOCK` gehört vor dem
   Lesen wieder abgeschaltet: POSIX lässt seine Wirkung auf gewöhnliche Dateien
   offen, und `speculation:` auf einem Netzlaufwerk könnte ein Lesen sonst mit
   `EAGAIN` scheitern — für einen Dateimanager ist das keine ferne Umgebung. Das
   Abschalten ist ein `fcntl`, also eine vierte Fremdbindung in
   `verzeichnis::sys`, dem einen Modul mit `allow(unsafe_code)`. Ungemessen ist
   dabei, ob macOS auf einer SMB- oder FUSE-Fläche `EAGAIN` überhaupt liefert;
   ohne diese Messung ist Punkt 2 eine Vermutung und keine Tatsache.
3. **Der Nachweis "ohne gelesen zu werden" fällt mit.** Die Probe
   `eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden`
   (`crates/krk-core/tests/text.rs:670-702`) belegt die Reihenfolge an den
   Rechten: eine gesperrte Datei über der Grenze kommt als `ZuGross` zurück, eine
   gesperrte genau auf der Grenze als Lesefehler. Öffnet man den Deskriptor
   zuerst, scheitert **beides** am `File::open`, und der Beleg ist nicht mehr
   zu führen — er müsste anders geschnitten werden. Das ist eine Änderung an der
   Bauart und am Nachweis von S10, nicht eine Verbesserung darin.

Damit hängt die Frage nicht am Aufwand allein, sondern an einer Abwägung
zwischen einem seltenen Wettrennen und drei belastbaren Zusagen, die heute
stehen. Sie gehört dem Nutzer.

**Ebenfalls offen: die `mkfifo`-Probe.** Der Befund nennt sie zu Recht als billig
zu haben, aber sie gehört zu den Ordner- und Verknüpfungsproben in
`crates/krk-core/tests/text.rs`, und diese Datei lag außerhalb der Dateigrenze
dieses Arbeitspakets. Eine zweite Fassung von `Pruefordner` in einer Modulprobe
unter `src/` anzulegen, nur um die Grenze zu umgehen, wäre der falsche Ort.

Geändert: ausschließlich `crates/krk-core/src/text/datei.rs`, darin
ausschließlich Kommentar. Kein Verhalten geändert. Abgenommen mit `cargo
build/test/clippy/fmt --workspace`, alle vier auf 0.
