# Vier Defekte im Kernmodul `text/`: drei geschlossen, einer halb und mit Begründung offen

**Status:** Complete
**Domäne:** code
**Ausführender:** `coder`
**Aktiver Circle:** `260807-2116-eingebauter-editor-mit-textmarken` (`_t_`)
**Grundlage:** vier Defektdatensätze im Circle, `260809-1610`, `260809-1652`, `260809-1728`, `260808-1413` (`umlaufen`)
**Abnahme:** `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`, `cargo fmt --all --check`, alle vier mit Rückgabewert 0

**Ein Hinweis zur Ausstattung dieses Laufs:** `fusion-rules coder` gibt `chat-voice-de.yaml` aus, aber kein `default-voice-de.yaml`. Das Chat-Profil ist gelesen und angewandt; für die Langform dieses Berichts und der Defektnotizen gilt die Artefaktsprache aus `CLAUDE.md`, Zeile `**Language:** de`, ohne abweichende Artefaktsprache. Derselbe Befund steht für den `coder` im Bericht vom 260810-0822, für den `ontocoder` im Bericht vom 260810-0914 und für den `reconciler` im Abgleich vom 260810-0810.

**Die Dateigrenze dieses Pakets** lautete auf `crates/krk-core/src/text/**` und, allein für den ersten Defekt und allein für die Konstante samt Zusicherung, auf `crates/krk-ui/src/vorschaumodell.rs`. `appkit/`, `editormodell.rs`, `resources/`, `Cargo.toml` und `planning/` waren ausgenommen, weil dort parallel andere Agenten arbeiteten. Die Grenze ist gehalten; sie ist an einer Stelle zur Sache geworden und steht dort begründet (Defekt 2, `mkfifo`-Probe).

---

## 1. `260809-1610` — die halbe Zusicherung hat ihre andere Hälfte bekommen

Die Zusicherung `EDITORGRENZE > TEXTGRENZE` stand in `krk-core/src/text/datei.rs` gegen die 1 MB der Vorschau **als Zahl**, weil `krk-core` die Kiste `krk-ui` nicht kennt. Sie fing damit ein Absenken der Editorgrenze und nicht ein Anheben der Vorschaugrenze.

Die fehlende Richtung steht jetzt in `crates/krk-ui/src/vorschaumodell.rs`, unmittelbar unter der bestehenden `BILDGRENZE > TEXTGRENZE`:

```rust
const _: () = assert!(krk_core::text::datei::EDITORGRENZE > TEXTGRENZE);
```

**Abweichung vom Vorschlag, und der Grund dafür.** Der Datensatz legte die vollständige Zusicherung in den Übergang aus der Vorschau in den Editor, Schritt 23, weil dort beide Grenzen ohnehin nebeneinander zu lesen sind. Sie liegt jetzt neben `TEXTGRENZE` selbst. Eine Zusicherung wirkt überall in der Kiste gleich; gelesen wird sie nur da, wo jemand hinsieht, und wer `TEXTGRENZE` anhebt, liest die Zeile darunter und nicht den Übergangscode aus S23. Dazu steht an dieser Stelle schon eine Zusicherung derselben Form über dieselbe Zahl.

Die zweite Möglichkeit des Datensatzes — `TEXTGRENZE` nach `krk-core` ziehen, damit beide Zahlen an einem Ort stehen — ist ungenutzt geblieben. Sie ist jetzt auch unnötig: beide Richtungen sind gefangen, ohne die Vorschau umzubauen.

Der Kommentar an der halben Zusicherung in `datei.rs` nennt die Gegenrichtung jetzt als gebaut statt als anstehend.

## 2. `260809-1652` — der Kommentar behauptet die Garantie nicht mehr, die Bauart bleibt offen

**Dies ist der Defekt, der nicht geschlossen ist.** Der Datensatz führt zwei Antworten: den Deskriptor prüfen statt des Pfades, oder den Kommentar auf das zurücknehmen, was er hält. Umgesetzt ist die zweite. Die erste braucht eine Entscheidung des Nutzers, und drei Gründe sagen, warum.

**Umgesetzt.** `oeffnen` trägt einen Abschnitt "Geprueft wird der Pfad und nicht der Deskriptor". Er hält fest, dass Schritt 2 und 3 `stat(2)` auf den Pfad fragen und Schritt 4 denselben Pfad ein zweites Mal öffnet, und er trennt die beiden Folgen: **Wachsen** fängt die Schranke `take(EDITORGRENZE + 1)`, auch wenn der Pfad in der Spanne auf eine größere Datei zeigt; **ein Austausch gegen eine benannte Röhre** fängt sie nicht, dann hängt das `File::open` doch, der Arbeitsfaden endet nie, und der Editor öffnet kommentarlos nichts. Punkt 2 der bindenden Reihenfolge verweist auf diesen Abschnitt, dort, wo er vorher eine Eigenschaft der Bauart behauptete.

**Offen, und warum.**

1. **Die Kostenschätzung des Datensatzes stimmt nicht.** Er sagt, `libc` stehe in `krk-core` schon; `crates/krk-core/Cargo.toml` führt `serde`, `toml` und `icu_collator` und kein `libc`. `verzeichnis/sys.rs` schreibt seine drei `extern "C"`-Blöcke und seine Header-Konstanten selbst. `O_NONBLOCK` wäre eine eigene Konstante oder eine neue Kiste — und `Cargo.toml` lag außerhalb der Dateigrenze.
2. **Ein Merkmal an `OpenOptions` genügt nicht.** `O_NONBLOCK` gehört vor dem Lesen wieder abgeschaltet, weil POSIX seine Wirkung auf gewöhnliche Dateien offen lässt und `speculation:` ein Lesen auf einem Netzlaufwerk sonst mit `EAGAIN` scheitern könnte. Für einen Dateimanager ist eine SMB-Fläche keine ferne Umgebung. Das Abschalten ist ein `fcntl`, also eine vierte Fremdbindung in `verzeichnis::sys`, dem einen Modul des Kerns mit `allow(unsafe_code)` — außerhalb der Dateigrenze und außerhalb von `text/`.
3. **Der Nachweis "ohne gelesen zu werden" fällt mit.** `eine_datei_ueber_der_grenze_wird_abgewiesen_ohne_gelesen_zu_werden` (`crates/krk-core/tests/text.rs:670-702`) belegt die Reihenfolge an den Rechten: eine gesperrte Datei über der Grenze kommt als `ZuGross` zurück, eine gesperrte genau auf der Grenze als Lesefehler. Öffnet man den Deskriptor zuerst, scheitert beides am `File::open`, und der Beleg ist nicht mehr zu führen. Das ist eine Änderung an der Bauart **und** am Nachweis von S10 und keine Verbesserung darin.

Die Frage hängt damit nicht am Aufwand allein, sondern an einer Abwägung zwischen einem seltenen Wettrennen und drei heute stehenden Zusagen. Sie gehört dem Nutzer. Ungemessen und als Vermutung markiert ist dabei Punkt 2: ob macOS auf einer SMB- oder FUSE-Fläche `EAGAIN` für eine gewöhnliche Datei überhaupt liefert, ist hier nicht geprüft.

**Auch offen: die `mkfifo`-Probe.** Der Datensatz nennt sie zu Recht als billig zu haben, aber ihr Ort ist `crates/krk-core/tests/text.rs`, bei den Ordner- und Verknüpfungsproben, und diese Datei lag außerhalb der Dateigrenze. Eine zweite Fassung von `Pruefordner` in einer Modulprobe unter `src/` anzulegen, nur um die Grenze zu umgehen, wäre der falsche Ort für dieselbe Sache.

## 3. `260809-1728` — der Modulkopf nennt beide Eingänge

Der Modulkopf von `krk_core::text::datei` zählte genau einen anstehenden Fall auf, den Ersatztext aus C5, und ließ den größeren weg: den Stand, den die `NSTextView` zurückgibt. Beide sind seit dem Schließen von `260809-1646` gebaut.

Jetzt stehen beide als Aufzählung, der größere zuerst, mit dem Grund, aus dem er der größere ist — eine `NSTextView` bewahrt eingefügten Text zeichengetreu auf. Beide sind ausdrücklich als gebaut bezeichnet und mit ihren Fundstellen genannt, `bearbeiten` und `ersetzung_vorbereiten`. Der Pfeil im ASCII-Bild trägt "der Stand aus der Textflaeche und der Ersatztext aus C5" statt "jeder andere Text, der in den Stand geraet (S37)".

**Ein Punkt über den Vorschlag hinaus.** Der Modulkopf von `editormodell.rs` führt beide Eingänge schon vollständig aus, mit eigenem Bild und mit der Begründung, warum der größere ein `bool` zurückgibt. Der Absatz in `datei.rs` verweist deshalb dorthin, statt die Erklärung ein zweites Mal zu schreiben. Zwei Fassungen derselben Erklärung wären genau die Doppelung, die dieses Modul sonst meidet.

## 4. `260808-1413` — `voriger` läuft jetzt wirklich über `umlaufen`

Der Satz an `umlaufen` behauptete, die eine Stelle des Umlaufs zu sein; `voriger` trug seinen Umlauf und seine Leerlistenbehandlung in einer zweiten Formulierung selbst. Der Datensatz nennt zwei Wege und empfiehlt den ersten, `voriger` über `umlaufen` zu führen.

**Der erste Weg, mit einer anderen Rechnung als der vorgeschlagenen.** Der Datensatz schlug

```rust
davor.checked_sub(1).or_else(|| umlaufen(treffer, treffer.len().saturating_sub(1)))
```

vor. Damit stünde der Satz weiterhin nur halb: der Umlauf nach hinten zielt in dieser Fassung auf `treffer.len() - 1`, und diese Zahl **ist** die Umlaufregel. Sie stünde wieder in `voriger`, und `umlaufen` bliebe darin auf die Leerlistenprüfung beschränkt.

Gebaut ist deshalb die Ringrechnung. `umlaufen` antwortet `stelle % treffer.len()` statt `if stelle < len { stelle } else { 0 }`, und `voriger` ist der Schritt zurück als Schritt um `len - 1` nach vorn:

```rust
umlaufen(treffer, davor + treffer.len().saturating_sub(1))
```

Für `erster_ab` und `naechster` ist das dasselbe wie vorher, weil ihre Stelle höchstens `len` ist und `len % len` gleich `0`. Alle drei Richtungen und die leere Liste kommen jetzt aus einer Funktion. Der Satz an `umlaufen` nennt den Ring ausdrücklich und warnt davor, ihn auf `if stelle < len` zurückzukürzen, weil das `voriger` seinen Umlauf nähme — der Defekt hätte sich sonst über den Umweg der Vereinfachung wieder eingestellt.

Kein Verhalten geändert. `die_auswahl_laeuft_in_beide_richtungen_um` läuft unverändert grün; dazu ist `ein_einziger_treffer_wird_aus_jeder_richtung_wieder_erreicht` neu, der scharfe Fall der Ringrechnung mit dem Summanden 0.

---

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-core/src/text/datei.rs` | Modulkopf (beide Eingänge, ASCII-Bild), Kommentar an der halben Zusicherung, Punkt 2 der bindenden Reihenfolge, neuer Abschnitt "Geprueft wird der Pfad und nicht der Deskriptor". Nur Kommentar, keine Zeile Code. |
| `crates/krk-core/src/text/suche.rs` | `voriger` und `umlaufen` samt Dokumentation, eine neue Probe. |
| `crates/krk-ui/src/vorschaumodell.rs` | Die neue Zusicherung `EDITORGRENZE > TEXTGRENZE` samt Begründung, nichts sonst. |

Die vier Defektdatensätze tragen ihre Notiz unter `---`. Drei sagen `Resolved:`; `260809-1652` sagt ausdrücklich "Nicht geschlossen" und nennt, was umgesetzt ist und was entschieden werden muss. Die Umbenennung der Marker macht der Nutzer.

## Abnahme

Alle vier Kommandos in einem Zug, mit vorangestelltem `export PATH="$HOME/.cargo/bin:$PATH"`:

```text
cargo clippy --workspace --all-targets   exit 0
cargo fmt --all --check                  exit 0
cargo build --workspace                  exit 0
cargo test --workspace                   exit 0
```

`cargo test --workspace` zählt in `krk-core` 140 grüne Modulproben, darunter die neue in `text::suche`; die Proben aus `tests/text.rs` laufen unverändert grün, insbesondere die drei, die an der Reihenfolge von `oeffnen` hängen.

## Was ein Nachfolger wissen sollte

- **`260809-1652` braucht eine Entscheidung, keinen weiteren Code.** Die drei Gründe stehen im Datensatz und oben. Wer sie beantwortet, entscheidet mit über den Nachweis von S10 in `tests/text.rs` und über eine vierte Fremdbindung in `verzeichnis/sys.rs`.
- **Die `mkfifo`-Probe steht weiter aus** und gehört zu den Ordner- und Verknüpfungsproben in `crates/krk-core/tests/text.rs`. Sie prüft die heutige Zusage und nicht eine künftige: was keine gewöhnliche Datei ist, wird abgewiesen.
- **Die Kostenschätzung eines Befundes ist selbst zu prüfen.** `260809-1652` nannte `libc` als vorhandene Abhängigkeit von `krk-core`; sie ist es nicht. Der Kern schreibt seine Systemaufrufe selbst, an genau einer Stelle.
