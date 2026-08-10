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

---
Resolved: Am 260810-0955 ist die tragende Antwort umgesetzt: `oeffnen`
(`crates/krk-core/src/text/datei.rs`) öffnet jetzt **zuerst** und prüft danach am
**Deskriptor**. Der Pfad wird von genau einem Aufruf aufgelöst, damit ist das
Fenster zwischen Prüfung und Öffnen zu — nicht bewacht, sondern nicht mehr
vorhanden.

**Die drei Gründe der Notiz vom 260810-0919 sind alle drei abgearbeitet.**

1. **Keine neue Kiste, keine `libc`.** `crates/krk-core/Cargo.toml` ist
   unangetastet. `verzeichnis/sys.rs` trägt die neue Bindung selbst, wie die drei
   bestehenden: `O_NONBLOCK`, `F_GETFL` und `F_SETFL` als eigene Konstanten
   (gegen `$(xcrun --show-sdk-path)/usr/include/sys/fcntl.h` geprüft: `0x4`, `3`,
   `4`) und ein `extern "C"`-Eintrag für `fcntl`. Der Eintrag ist **variadisch**
   deklariert, weil Apples arm64-ABI variadische Argumente auf dem Stapel
   übergibt und feste in Registern; drei feste Argumente wären ein
   ABI-Fehler ohne Übersetzerwarnung.
2. **`O_NONBLOCK` wird abgeschaltet, und zwar noch im Öffner.** Die neue Funktion
   `verzeichnis::sys::ohne_warten_oeffnen` öffnet mit `O_NONBLOCK` und nimmt es
   über `fcntl(F_GETFL)` plus `fcntl(F_SETFL)` wieder ab, bevor sie den
   Deskriptor herausgibt. Sofort und nicht später, weil die Gefahr allein am
   `open` steht und ein `F_SETFL` nichts anhält. Die Modul-Probe
   `ein_geoeffneter_deskriptor_traegt_o_nonblock_nicht_mehr` fragt den Deskriptor
   selbst und nicht den Quelltext daneben. Die Vermutung über `EAGAIN` auf einer
   SMB- oder FUSE-Fläche bleibt ungemessen; das Abschalten macht sie
   gegenstandslos, statt sie zu beantworten.
3. **Der Nachweis "ohne gelesen zu werden" ist neu geschnitten und nicht
   verloren.** Der alte Schnitt hing an den Rechten und ist nach dem Umbau
   *gegenstandslos*, nicht bloß ungenau: POSIX prüft das Leserecht beim `open`,
   also scheitern beide gesperrten Dateien jetzt schon dort. Fall 10 in
   `crates/krk-core/tests/text.rs` steht deshalb auf drei Proben:
   - `eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden` hält
     den deterministischen Teil: der eine Byte Unterschied entscheidet, und die
     Datei auf der Grenze wird vollständig gelesen (16 MB Stand).
   - `zwei_gigabyte_werden_ohne_arbeitsspeicher_abgewiesen` trägt den Satz "ohne
     gelesen zu werden". Seine Zeitschranke ist von der absoluten halben Sekunde
     auf eine **relative** umgestellt: gemessen wird gegen das vollständige Lesen
     von 16 MB auf derselben Maschine. Zwei Gigabyte sind das 128-fache davon,
     zugelassen ist das Achtfache.
   - `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` und
     `eine_benannte_roehre_wird_abgewiesen_und_haelt_das_oeffnen_nicht_an` sind
     neu, dazu `eine_gesperrte_datei_kommt_mit_dem_systemfehler_zurueck` für die
     Deckung, die der alte Schnitt beiläufig mitbrachte.

**Ein Ergebnis, das im Befund nicht stand und das die Bauart betrifft:** im
**Ergebnis** sind "vor dem Lesen geprüft" und "nach dem Lesen geprüft" nicht zu
unterscheiden, weil die Schranke `take(EDITORGRENZE + 1)` auch den zweiten Fall
mit `ZuGross` beantwortet. Beobachtbar ist allein der **Aufwand**. Deshalb kann
der Nachweis von S10 nur eine Ressourcenschranke sein, und deshalb steht er jetzt
dort, wo gemessen wird.

**Die `mkfifo`-Probe ist mitgekommen** und liegt bei den Ordner- und
Verknüpfungsproben in `crates/krk-core/tests/text.rs`, nicht in einer zweiten
Fassung unter `src/`. Angelegt wird die Röhre über `mkfifo(1)` als Unterprozess:
`mkfifo(2)` wäre eine fünfte Fremdbindung in `verzeichnis::sys`, und dort steht,
was KRK braucht. KRK legt keine Röhren an.

**Die Wettrennen-Probe ist nachgemessen und nicht nur gemeint.** Mit der alten
Reihenfolge wieder eingesetzt fällt sie aus, in beiden Profilen (`debug` und
`release`, je ein Lauf am 260810): der Lesefaden bleibt im `File::open` an der
Röhre stehen und die Zeitschranke von 15 Sekunden schlägt zu. Zwei Fassungen
davor liefen **durch** und belegten nichts, und beide Sackgassen stehen im Kopf
der Probe, damit die nächste Fassung sie nicht wiederholt:

- Ein Tauscher, der wegbenennt und zurücklegt, lässt den umkämpften Pfad die
  meiste Zeit **fehlen** (gemessen: 12.830 von 20.000 Durchläufen sahen `ENOENT`);
  das Fenster wurde nie getroffen. Jetzt liegt an dem Pfad immer etwas, Datei
  oder Röhre, gelegt über eine harte Verknüpfung und ein `rename` darüber.
- Eine feste Zahl von Durchläufen genügt nicht: im Profil `release` ist der
  Lesefaden so schnell, dass der Tauscher nur 994 Tausche schafft. Der Lesefaden
  läuft jetzt, bis **beides** erreicht ist, Durchläufe und Tausche, mit einer
  Obergrenze als Notbremse. Ein Tauscher, der still aufhört, lässt die Probe
  ausfallen statt durchlaufen.

Geändert: `crates/krk-core/src/text/datei.rs`,
`crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/tests/text.rs`. Sonst
nichts; `Cargo.toml` war nicht nötig.

Abnahme am 260810-1002, alle vier auf 0: `cargo build --workspace`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets` (keine
Warnung) und `cargo fmt --all --check`. Dazu `cargo fmt -p krk-core --check` → 0.
Die Textproben liefen je dreimal in `debug` und `release` durch, ohne Ausfall.

Zwei Zwischenstände gehören dazu, weil parallel an `crates/krk-ui/` gearbeitet
wurde und die Kommandos den ganzen Baum sehen: um 0950 fielen `cargo test
--workspace` und `cargo clippy --workspace --all-targets` mit 101 aus, an einem
`CStr`-Fehler in `crates/krk-ui/src/appkit/editor.rs:2717`; um 0955 fiel
`cargo fmt --all --check` mit 1 aus, an derselben Datei in Zeile 3153. Beides lag
außerhalb der Dateigrenze dieses Arbeitspakets und war zur Abnahme um 1002 vom
Nachbarpaket behoben.

**Offen geblieben, als eigener Datensatz:** die Aufzählung der Fremdaufrufe in
`crates/krk-core/src/lib.rs:11` und `crates/krk-core/src/verzeichnis/mod.rs:11`
nennt drei und muss `fcntl` mitnennen. Beide Dateien liegen außerhalb der
Dateigrenze; der Defekt ist
`issues/260810-0955_o_die-aufzaehlung-der-fremdaufrufe-nennt-drei-und-es-sind-vier.md`.
