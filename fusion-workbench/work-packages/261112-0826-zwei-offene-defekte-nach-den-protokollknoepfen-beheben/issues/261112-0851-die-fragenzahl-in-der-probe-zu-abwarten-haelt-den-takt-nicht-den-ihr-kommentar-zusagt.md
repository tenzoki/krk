Die Fragenzahl in der Probe zu `abwarten` hält den Takt nicht, den ihr Kommentar zusagt
---
`cloudladen::tests::abwarten_bricht_ab_oder_wartet_die_dauer` (`crates/krk-ui/src/cloudladen.rs`) verlangt in ihrer zweiten Hälfte zwei bis sechs Fragen in fünf Schritten Dauer. Ihr Doc-Kommentar sagt, diese Zahl halte, dass `abwarten` in Takten fragt, „statt die Dauer in einem Stück zu verschlafen oder ohne Schlaf zu kreisen“. Die erste der zwei Zusagen hält die Schranke nicht, und ihre untere Grenze ist unter Last nicht sicher.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261112-0840-zwei-proben-zu-abwarten-im-cloudladen-tragen-dieselbe-freie-20-ms-wanduhrschranke-weiter.md, 261112-0851-reviewer-schlussdurchsicht-zwei-defekte-nach-den-protokollknoepfen.md

**Schwere:** Low. Betroffen ist allein das Prüfmodul; der Betriebscode ist richtig.

Beleg, am Quelltext gelesen:

- Ein `abwarten`, das nach der ersten Frage die ganze Dauer in einem Stück verschläft und danach einmal fragt, stellt genau zwei Fragen. `(2..=6).contains(&gefragt)` (Zeile 353) lässt es durch. Die Schranke fängt allein das Kreisen ohne Schlaf (mehr als sechs Fragen).
- Der Kommentar in Zeile 350 sagt „nie weniger als die erste vor dem Warten und die letzte danach“. `abwarten` nimmt `beginn`, fragt, und liest dann `beginn.elapsed()`. Wird der Faden zwischen `Instant::now()` und `elapsed()` für fünf Schritte (50 ms) nicht zugeteilt, ist die Dauer nach der ersten Frage schon um, und die Antwort kommt nach einer Frage. Die Probe wird dann rot, obwohl der Betriebscode richtig ist. inference: das Fenster ist wenige Anweisungen breit und der Fall selten; gemessen ist er nicht, und in den zwanzig Läufen der Behebung ist er nicht aufgetreten.

Dass `abwarten` die Dauer nicht in einem Stück verschläft, hält heute die Schwesterprobe `ein_nein_mitten_in_der_wartezeit_beendet_sie`: sie verlangt drei Fragen binnen `HAENGT` bei einer Dauer vom Zehnfachen. Die Eigenschaft ist also gehalten, nur nicht dort, wo der Kommentar es sagt.

Richtung: die untere Grenze auf eins senken oder streichen, und den Kommentar auf das kürzen, was die Zahl hält (kein Kreisen ohne Schlaf), mit einem Verweis auf die Schwesterprobe für den Takt.

Abnahme: der Doc-Kommentar der Probe nennt für die Fragenzahl allein Eigenschaften, an denen ein falsches `abwarten` die Schranke verletzt, und die Schranke kann an einem richtigen `abwarten` unter keiner Zuteilung der Fäden fallen.
