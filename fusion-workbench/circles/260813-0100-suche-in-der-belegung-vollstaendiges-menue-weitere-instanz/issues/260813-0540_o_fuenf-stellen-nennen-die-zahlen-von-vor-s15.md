Fünf Stellen nennen die Zahlen von vor S15

---

S14 und S15 haben `Kommando` von 75 auf 76 Varianten und die Auslieferungsbelegung von 81 auf
82 Funktionen gebracht (`resources/default-keymap.toml:34`, `grep -c '^\[\[funktion\]\]'`
liefert 82; `Kommando::KENNUNGEN` trägt `[(Kommando, &str); 76]`,
`crates/krk-core/src/tasten/belegung.rs:566`). Fünf Stellen aus S6 tragen weiter die Zahlen
von davor, alle in derselben Prüfspanne geschrieben:

| Stelle | steht da | stimmt |
|---|---|---|
| `crates/krk-ui/src/appkit/menue.rs:9` | „neun Obermenues, zweiundachtzig Eintraege" | 83 benannte Einträge (82 Funktionen und der Sonderposten) |
| `crates/krk-ui/src/appkit/menue.rs:173` | „an jedem der zweiundachtzig Eintraege" | 83 |
| `crates/krk-ui/src/appkit/anwendung.rs:655` | „an jedem der zweiundachtzig Eintraege" | 83 |
| `crates/krk-ui/src/menuemodell.rs:19` | „82 Eintraege statt zehn" | 83 |
| `crates/krk-ui/src/appkit/menue.rs:26` und `:307` | „Fuenfundsiebzig Selektoren", „Eine Methode statt fuenfundsiebzig" | 76 |

Die Leiste trägt insgesamt 84 `NSMenuItem`: 82 Befehls- und Textbefehlseinträge, der
Sonderposten „Tastenbelegung als Markdown sichern" und der eine Trenner darüber.

**Keine Probe hält diese Zahlen.** Das ist Absicht und richtig so —
`jede_funktion_der_belegung_steht_genau_einmal_im_menue`
(`crates/krk-ui/src/menuemodell.rs:383-408`) zählt ausdrücklich gegen `Belegung::funktionen()`
statt gegen eine Zahl im Programmtext, und der Doc-Kommentar sagt, warum. Genau deshalb
veralten die Zahlen in der Prosa unbemerkt.

---

**Schwere:** mittel. Kein Fehlverhalten; fünf Stellen, die ein Leser für nachgezählt hält und
die es einmal waren. `CLAUDE.md` führt dieselbe Sorte Befund schon zweimal offen
(`shared/issues/260812-2253_*` zu den 68 Varianten, `260812-1438_*` zu den 31 von 33 Dateien),
und diese Runde legt eine dritte daneben.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/menue.rs:9`, `:26`, `:173`, `:307`,
`crates/krk-ui/src/appkit/anwendung.rs:655`, `crates/krk-ui/src/menuemodell.rs:19`

**Domain:** code

## Vorschlag

Die sechs Stellen nachziehen. Wo die Zahl nichts trägt, sie streichen: „Eine Methode statt
einer je Befehl" sagt dasselbe wie „statt fünfundsiebzig" und veraltet nicht. Der Plan der
Runde nennt in seinem Text ebenfalls durchgehend 82 Einträge; er ist eine Aufzeichnung eines
Standes und bleibt nach der Ortsregel aus `CLAUDE.md` unangetastet.
