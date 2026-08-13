Fünf Stellen nennen 79 Funktionen und 73 mit Kommando; die Belegung führt 82 und 76

---

`resources/default-keymap.toml` führt **82** Funktionen, davon **76** mit einem
`krk_core::tasten::Kommando` und sechs ohne (die zugestellten Textbefehle). Am 260813-1345
nachgezählt: 82 `[[funktion]]`-Blöcke, 88 Kombinationen in den `tasten`-Listen, 76 Varianten in
`Kommando` (`crates/krk-core/src/tasten/belegung.rs`).

Fünf Prosastellen in zwei Dateien nennen weiter 79 und 73.

---

**Schwere:** niedrig. Kein Verhalten, kein Bau. Keine Probe hängt an einer der fünf Zahlen.

| Stelle | Was dort steht | Richtig |
|---|---|---|
| `crates/krk-ui/src/appkit/menue.rs:128` | „Die sechs sind die einzigen der 79 Funktionen ohne `krk_core::tasten::Kommando`" | 82 |
| `crates/krk-ui/src/appkit/menue.rs:799-801` | „Sie tragen als einzige der 79 Funktionen kein `krk_core::tasten::Kommando`" | 82 |
| `crates/krk-ui/src/appkit/menue.rs:867` | „Fuer 73 der 79 Funktionen ist der Wirkungsbereich aus der Belegung entscheidbar" | 76 von 82 |
| `crates/krk-ui/src/belegungsausgabe.rs:45` | „Gezaehlt wird ueber alle 79 Funktionen, und die Ziffer einer Lage heisst ueberall dasselbe" | 82 |
| `crates/krk-ui/src/belegungsausgabe.rs:48` | „Die erste Lage traegt die 73 Funktionen mit `Kommando`" | 76 |

**Die Zahl 79 ist aus der Runde 3 und nicht aus der Runde 8.** Sie kam mit `90b02d4` in
`menue.rs` und war damals richtig. Die Belegung ist danach zweimal gewachsen, zuletzt in der
Runde 7 (`40b5fb0` bringt `opt+cmd+n` für die weitere Instanz, `dff167a` zieht den Baum nach).
Die Runde 8 hat `resources/default-keymap.toml` nicht angefasst — C6.2 verlangt das
ausdrücklich, und es hält.

**Warum der Datensatz im gemeinsamen Speicher liegt.** Er ist beim Abgleich der Runde 8
gefunden worden, entstammt aber nicht ihrer Directive: die Stellen standen vor ihr falsch da und
liegen in Zeilen, die diese Runde nicht berührt hat. Nach der Herkunftsregel gehört er damit
hierher und nicht in den Speicher des Circles.

**Der Satz an `belegungsausgabe.rs:45` sagt seine eigene Falle mit an**: „die Ziffer einer Lage
heisst ueberall dasselbe: im Modulkopf, an den Zweigen von `wirkung` und in der Probe". Die
Zusage über die drei gleichlautenden Stellen hält; falsch ist die Zahl, über die gezählt wird,
und sie ist an allen dreien dieselbe.

**Was zu tun ist**

Die fünf Zahlen nachziehen. Wer sie prüfbar machen will, hat den Ort schon: die Probe
`die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`, die
`belegungsausgabe.rs:45-48` selbst nennt, zählt die Lagen und könnte die Summe gegen die
Belegung halten, statt sie im Modulkopf als Zahl zu führen.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Verwandt: `shared/issues/260812-2253_o_claude-md-nennt-fuer-kommando-68-varianten-der-baum-traegt-75.md`
  (dieselbe Zahlensorte, andere Datei; jener Datensatz nennt 75 und ist selbst überholt, es sind
  76) und `circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0810_o_die-zahl-39-im-kopf-der-belegungsdatei-steht-im-praesens-und-ist-ungeprueft.md`.
