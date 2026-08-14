Neun Stellen sprechen weiter von „vier Dateien" im Ablageordner; es sind sechs

---

C5 sagt zu: „Der Ablageordner führt nach dieser Runde sechs Dateien." Der Kopf von
`crates/krk-core/src/ablage/pfade.rs` ist darauf gezogen („sechs Dateien in zwei
Formaten"), der von `crates/krk-core/src/ablage/mod.rs` nur in seinen ersten Absätzen. Neun
Stellen sagen weiter „vier":

| Datei:Zeile | Aussage |
|---|---|
| `krk-core/src/ablage/mod.rs:40` | „die vier Lade- und Schreibmethoden haengen an einem `Zugang`" — es sind sechs, `text_laden` und `text_sichern` sind dazugekommen |
| `krk-core/src/ablage/mod.rs:45` | „`Ablage::pfad` liefert den Pfad einer der vier Dateien" |
| `krk-core/src/ablage/mod.rs:359` | „Der Ablageordner mit den vier Dateien" |
| `krk-core/src/ablage/mod.rs:361` | „Wer eine der vier Dateien anfassen will" |
| `krk-core/src/ablage/mod.rs:401` | `Ablage::pfad`: „Der Pfad einer der vier Dateien." |
| `krk-core/src/ablage/mod.rs:442` | `Zugang::pfad`: „Der Pfad einer der vier Dateien." |
| `krk-core/src/ablage/lesezeichen.rs:106` | „wie C7 und C11 der Runde 1 es fuer alle vier Ablagedateien zusagen" |
| `krk-core/src/text/datei.rs:687` | „denselben Weg, den die vier Ablagedateien nehmen" |
| `krk-ui/src/belegungsausgabe.rs:89` | „den `krk_core::text::datei` beim Sichern des Editors und die vier Ablagedateien gehen" |

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Die Zahl ist in diesem Projekt aber die
Form, in der eine Zusage nachgelesen wird: C5 nennt sie, `pfade.rs` nennt sie, und der Kopf
des Moduls, das die Dateien hält, nennt zwei verschiedene.

**Zwei Stellen sind ausdrücklich richtig und bleiben stehen:** `Zugang::laden` (`:447`) und
`Zugang::sichern` (`:512`) sagen „eine der vier Dateien" und meinen jetzt genau die vier
TOML-Dateien — dass sie keine andere annehmen, hält der `debug_assert_eq!` auf
`Format::Toml` in ihrem Rumpf. Die Formulierung gewinnt an Genauigkeit, wenn sie „eine der
vier TOML-Dateien" sagt, ist aber nicht falsch.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Zeilennummern am Stand `dd2643e` gezählt; das Suchmuster war `vier Dateien` und
  `vier Ablagedateien` über `crates/`.
