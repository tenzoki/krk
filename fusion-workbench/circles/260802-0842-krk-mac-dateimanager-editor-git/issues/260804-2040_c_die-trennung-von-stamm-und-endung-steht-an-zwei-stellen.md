Die Trennung von Stamm und Endung eines Dateinamens steht an zwei Stellen

---

Zwei Module beantworten dieselbe Frage: wo hört der Stamm eines Dateinamens auf
und wo fängt die Endung an.

- `crates/krk-core/src/operation/umbenennen.rs`, `namen_teilen` (privat, aus
  S15): eine eigene Rechnung über `rfind('.')`, für `freier_name`.
- `crates/krk-core/src/umbenennen/regel.rs`, `stamm_und_endung` (aus S17): der
  Weg über `std::path::Path::file_stem` und `extension`, für die fortlaufende
  Nummerierung.

---

Beide liefern heute dasselbe Ergebnis; das ist geprüft (`archiv.tar.gz` →
Stamm `archiv.tar`, `.gitignore` → Stamm ohne Endung, `liesmich` → keine
Endung). Genau das ist die Lage, in der eine Abweichung später unbemerkt
entsteht: die erste Änderung an einer der beiden Stellen findet keine Prüfung,
die sie gegen die andere hält.

S17 konnte es nicht auflösen: `operation/umbenennen.rs` steht in der Dateiliste
des Plans als **lesend**, und `namen_teilen` ist privat. Die Auflösung ist
klein: `namen_teilen` öffentlich machen (oder nach `crate::verzeichnis` ziehen)
und `stamm_und_endung` durch einen Aufruf ersetzen. Die Signaturen
unterscheiden sich leicht — `namen_teilen` liefert die Endung mit ihrem Punkt
als `&str`, `stamm_und_endung` als `String` —, das ist beim Zusammenlegen zu
vereinheitlichen.

Gefunden bei der Umsetzung von Schritt 17.

---
Resolved: Eine Trennung, an einer Stelle. `namen_teilen` in `crates/krk-core/src/operation/umbenennen.rs` ist jetzt öffentlich, `stamm_und_endung` in `crates/krk-core/src/stapelumbenennen/regel.rs` ist entfallen, und `Regel::anwenden` ruft `namen_teilen`.

**Geblieben ist die Rechnung über `rfind('.')` und nicht der Weg über `Path`.** Der Datensatz lässt offen, welche der beiden bleibt; die Kommentare an beiden Stellen begründeten je die eigene. Zwei Gründe für die verbliebene: sie liefert geliehene Ausschnitte, während `Path::extension` den Punkt streicht und ihn wieder anzusetzen je Aufruf eine `String` kostete (genau das war die Signaturabweichung, die der Datensatz nennt); und sie steht dort, wo die übrigen Namensfunktionen des Kerns stehen, neben `name_pruefen` und `freier_name`.

**Die Begründung der abgelösten Fassung ist nicht verloren, sondern zur Prüfung geworden.** Sie lautete: "getrennt wird so, wie `std::path::Path` trennt". `die_trennung_stimmt_mit_der_trennung_von_path_ueberein` rechnet jetzt beide Wege nebeneinander und vergleicht sie über zehn Namen, darunter `archiv.tar.gz`, `.gitignore`, `datei.`, `..foo`, `.x.y` und den leeren Namen. Damit ist die Zusage prüfbar, statt an zwei Stellen behauptet zu werden.

**Ein einziger Name trennt die beiden**, gefunden mit einem Probeprogramm unter `/tmp` am 260805-0947, das beide Fassungen nebeneinander über elf Namen laufen ließ:

```
".."   rfind=(".", ".")   path=("..", "")   ABWEICHUNG
```

Alle übrigen zehn stimmen überein. `..` kann KRK nicht antreffen: `name_pruefen` weist ihn als `Namensfehler::Punktname` ab, und die Prüfung `punkt_und_doppelpunkt_sind_keine_namen` hält das fest. Der Name steht deshalb bewusst nicht in der Vergleichsliste, mit dieser Begründung im Kommentar. Das Probeprogramm ist wieder entfernt.

Nebenbefund aus derselben Messung, weil ich das Gegenteil vermutet hatte: `Path::extension("datei.")` liefert `Some("")` und damit `"."`, nicht `None`. Beide Fassungen erhalten den nachgestellten Punkt.

**Am laufenden, signierten Bündel gegengeprüft am 260805-0947.** Drei Dateien `IMG_a.jpg`, `IMG_b.jpg`, `IMG_c.jpg` unter `/tmp/krk-stapelprobe`, alle markiert, Stapel-Umbenennen mit `IMG_` → `Urlaub `, Nummer ab 7 mit drei Stellen. Die Vorschau zeigte:

```
IMG_a.jpg -> Urlaub a007.jpg
IMG_b.jpg -> Urlaub b008.jpg
IMG_c.jpg -> Urlaub c009.jpg
```

Nach Return trugen die drei Dateien genau diese Namen. Die Nummer steht am Stamm und `.jpg` hinten; das ist die Zusage, die an `namen_teilen` hängt. Der Prüfordner ist wieder entfernt.

Geprüft am 260805-0947: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
