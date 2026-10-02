# Durchsicht der Runde 1 der Behebungssitzung, zweiter Teil: Schritte 3 bis 6 des Plans `260826-1811`

**Reviewed-range:** `9c02863..fc829c8`
**Not-opened:** none
**Sender:** coderev
**Massstab:** `shared/planning/260826-1811_c_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`, Schritte 3 bis 6 samt Abnahmezeilen und dem Abschnitt „Prüfregeln, die jeder Schritt einhält"; die drei geschlossenen Datensätze `260826-1223_c_…`, `260826-1301_c_…`, `260826-1302_c_sechs-elternproben…`
**Übernommene Not-opened-Liste der vorigen Durchsicht** (`260826-1933-coderev-…`): none
**Gelesen:** alle neun geänderten Quelldateien am Arbeitsbaum (`git status` sauber bis auf `orchestrator-events.jsonl`), dazu die Nachbarstellen `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/zeit.rs`, `crates/krk-bench/src/fixture.rs`, `crates/krk-bench/src/main.rs`, `crates/krk-core/src/tasten/belegung.rs`.
**Gemessen, nicht nur gelesen:** `cargo test --workspace` (exit 0, 23 Probenziele, 0 failed), `cargo clippy --workspace --all-targets` (keine Warnung), `cargo fmt --all --check` (sauber). Kein Messlauf: der verlangt KRK im Vordergrund.

## Summary

Alle vier Behebungen halten das, was ihr Datensatz beschreibt, und der Baum ist an allen vier Abnahmekommandos grün. Die drei stillen Wege des Kindstarters sind zu — zwei durch das Gate, einer strukturell durch den einen Auftragsnamen —, und kein weiterer Starter derselben Klasse steht ohne Zeugen im Baum außer dem bekannten `zeit.rs:68`. Sechs Befunde, keiner davon ein Rückschritt gegenüber dem Stand vor der Behebung: vier Medium, zwei Low. Der schwerste ist eine Folge der Behebung selbst — das Gate hat die sechs fachlichen Fehlermeldungen der Rufer unerreichbar gemacht —, die zwei nächsten liegen in dem, was Schritt 6 **nicht** erreicht hat: der zweite Messweg `durchstich` und zwei von vier Prüfordnern bleiben ungedeckt, während Commit-Betreff und `Resolved:`-Zeile „jeder Prüfordner" sagen.

## Totals

Critical 0 / High 0 / Medium 4 / Low 2.

## Prüfpunkt 1: der Kindstarter (`17e5e4e`) — deckt das Gate alle drei stillen Wege?

**Ja, und zwei davon auf zwei verschiedene Arten.**

| Stiller Weg | Was `libtest` meldet | Was ihn schließt |
|---|---|---|
| Name trifft nicht | `running 0 tests` / `0 passed`, Status 0 | das Gate: `stdout.contains("test result: ok. 1 passed;")` (`gemeinsam/mod.rs:529`) |
| Auftrag trifft nicht | Kind kehrt still zurück, `1 passed`, Status 0 | strukturell: ein Name (`KINDAUFTRAG`, `:485` … `:461`) und ein Leser (`kindauftrag`, `:470-472`); der Starter setzt ihn immer (`:526`) |
| `#[ignore]` verloren | `--ignored` filtert weg, `0 passed`, Status 0 | dasselbe Gate |

Der zweite Weg ist damit nicht *geprüft*, sondern *unmöglich gemacht* — das ist die stärkere Form, und der Modulkopf (`:58-62`) sagt es so. Am Baum nachgezählt: `grep -rn 'AUFTRAG_' crates/krk-core/tests` findet neben `KINDAUFTRAG` nur noch `AUFTRAG_ABBRUCH`, `AUFTRAG_SPERRE` (`ablage.rs`) und `AUFTRAG_ZONE` (`zeit.rs`), also genau die drei, die der Plan ausdrücklich stehen läßt.

**Läuft ein Kind ohne `#[ignore]` wirklich rot?** Ja. Der Sitzungseintrag `260826-2010` hält beide Mutationen mit ihrer Ausgabe fest (Kind-stdout `running 0 tests`), und die Semantik von `--ignored` — nur die stillgelegten Proben — ist die Begründung. Der Probenlauf dieser Durchsicht zeigt daneben zwei Kindläufe mit `1 passed; 80 filtered out`, also Kinder, die wirklich gelaufen sind.

**Weitere Starter derselben Klasse.** Vier im Baum, `zeit.rs:69` ausgenommen (bekannt, `260825-2127`, ausdrücklich nicht Gegenstand):

- `ablage.rs:2418` `kindprobe` — kein `1 passed`-Gate, aber **jeder** seiner vier Rufer hat einen Zeugen im Dateisystem oder im Signal: `status.signal() == Some(SIGABRT)` plus die geschriebene Nachbardatei (`:2443-2462`), `recht.txt` mit `"ohne"` und danach `"gehalten"` (`:2581-2598`), `sperre.txt` mit `"belegt"` und danach `"frei"` (`:2811-2827`). Ein nicht gelaufenes Kind läßt die Datei fehlen oder trägt den Wert des vorigen Laufs; beides ist rot.
- `ablage.rs:2519` `kind_starten` — der einzige Rufer prüft am Ende `liste.wert.zahl() == 2 * ANLEGEZAHL` (`:2879-2883`). Ohne gelaufene Kinder steht dort 0.

**Kein neuer Befund.** Der gemeinsame Starter war der einzige ohne Zeugen, und er hat jetzt einen.

**Was das Gate nicht sieht** (nicht gefiltert, nur vermerkt): trifft ein Name versehentlich eine *andere* stillgelegte Probe derselben Datei, meldet `libtest` `1 passed` und das Gate ist grün. Der Fall setzt voraus, dass zwei Kindproben derselben Datei ineinander umbenannt werden; er ist mit dem einen Auftragsnamen nicht schlimmer geworden.

**Befund 1 (Medium):** die sechs `assert!` der Rufer sind seither unerreichbar. Siehe unten.

## Prüfpunkt 2: die Quelltextprobe (`9a4e495`) — die Blindheiten des Helfers

Der Plan behauptet, `Kommando` sei datenlos mit einer Variante je Zeile. **Am Baum nachgezählt und richtig:** 79 Varianten, alle datenlos, je eine Zeile, `belegung.rs:344-…`; `belegung.rs:697` trägt `; 79]`. Für die vorgesehene Wiederverwendung gilt dasselbe: `Wirkungsbereich` (`belegung.rs:213-…`) ist ebenfalls datenlos, je eine Zeile.

Der Helfer (`gemeinsam/mod.rs:411-461`) ist an jeder Stelle laut, nicht still — das ist die richtige Bauart und deckt sich mit dem, was sein Doc-Kommentar verspricht:

| Fall | Verhalten | im Doc-Kommentar? |
|---|---|---|
| Kopfzeile fehlt (umbenannt, eingerückt, nicht `pub`) | Panik mit Datei und erwarteter Zeile (`:427`) | ja (`:398-400`) |
| Variante mit Daten, `Foo(Bar)`, `Foo { … }` | Panik, die Zeile steht in der Meldung (`:441-446`) | ja (`:401-405`) |
| Variante über mehrere Zeilen | dieselbe Panik | ja |
| Zwei gleichnamige Aufzählungen in einer Datei | nimmt die erste (`position`) | ja (`:406-407`) |
| Block ohne schließende Klammer in Spalte 0 | Panik (`:448-451`) | nein, aber laut |
| Zeilenendkommentar hinter der Variante, fehlendes Komma | Panik über den Kommavergleich | nein, aber laut |
| **`#[cfg(...)]` an einer Variante** | Zeile wird als Attribut übersprungen, die Variante **mitgezählt**, auch wenn sie nicht übersetzt wird | **nein** |

Der letzte ist der einzige, der eine falsche *Antwort* statt eines Abbruchs liefern kann — und auch er endet rot, weil die Variante dann in `KENNUNGEN` fehlt, nur mit einer irreführenden Meldung. **Am Baum gemessen:** kein `pub enum` in `git ls-files '*.rs'` trägt ein `#[cfg` zwischen Kopf und schließender Klammer. Der Fall ist damit heute gegenstandslos; ich lege keinen Datensatz darüber an und vermerke ihn hier, weil der Helfer ausdrücklich zur Wiederverwendung gebaut ist.

`assert!(!varianten.is_empty())` steht (`:452-456`) und schließt die leer laufende Probe.

**Befund 5 (Low):** der Probenname sagt „genau einmal", der Mengenvergleich hält das nicht.
**Befund 6 (Low):** zwei neue Doc-Kommentare schreiben die Zahl 79 fest.

## Prüfpunkt 3: die Prüfordner-Prüfung (`960900d`) — ist die Toleranz eng genug?

**Die Toleranz selbst ist richtig gefaßt.** `Messreihe::fahren` prüft den Steckbrief nur, wenn einer daliegt (`messen.rs:187-198`); im Gesamtlauf kann er nicht fehlen, weil `pruefordner_pruefen` ihn schon vorher verlangt (`:1077`, `:1599-1606`). Die Nachsicht wirkt damit ausschließlich dort, wo sie gemeint ist: bei `messen --kopflos` auf einem fremden Ordner. Der Steckbrief liegt **neben** dem Prüfordner (`fixture.rs:502-521`, `nebenpfad`), zählt also nicht mit; die bestehende Probe auf 3.000 Einträgen bleibt grün, was der Lauf bestätigt.

**Sie läßt trotzdem mehr durch als der Betreff sagt, aber an anderer Stelle.** `pruefordner_pruefen` zählt keinen Eintrag: es hält den *Steckbrief* gegen die Zusage. Der Inhalt wird erst durch die zweite Hälfte gedeckt, den Abgleich der gelesenen Zahl in `Messreihe::fahren` — und die gibt es nur für A und den großen Ordner:

| Ordner | Steckbrief gegen Zusage | gelesen gegen Steckbrief |
|---|---|---|
| A | `:1077` | `:1266` |
| 100k | `:1077` | `:1267` |
| B | `:1077` | — |
| L6-Unterordner | `:1631` | — |

**Der Kommentar zu B trägt nicht.** `bericht.rs:261-262` sagt „er dient dem Fensterwechsel und wird nicht gelesen". Der erste Halbsatz stimmt für `krk-bench`; der zweite ist falsch, denn `plan_in_verzeichnis_schreiben` (`messen.rs:1800-1811`) schreibt B als Tab in beide Dateifenster der Prüfsitzung, und KRK liest ihn bei jedem L5-Tab- und L5-Fensterwechsel. Genau deshalb gehört seine Eintragszahl zur Zusage. Was der Kommentar richtig sagt: eine erfundene Zahl wäre schlimmer.

**Der zweite Messweg ist gar nicht angefaßt.** `Durchstich::fahren` (`messen.rs:763-771`) mißt L2, L3 und L10 auf denselben zwei Prüfordnern und prüft **nichts** — kein `is_dir()`, kein Steckbrief; es ruft auch keine `Messreihe`, also greift die zweite neue Prüfung dort nicht. Seine Felder versprechen weiter „Prüfordner A mit 10.000 Einträgen" (`:709`) und „mit 100.000 Einträgen" (`:711`), also genau die zwei Sätze, die derselbe Commit in `Gesamtlauf` durch `EINTRAEGE_A` und `EINTRAEGE_GROSS` ersetzt hat.

**Die Abhilfe in der Fehlermeldung läuft nicht.** `messen.rs:1594` und `:1603` nennen `` `krk-bench fixture --eintraege {erwartet} --out {ordner}` ``; `--seed` ist Pflicht (`main.rs:163`, Probe `fixture_verlangt_alle_drei_angaben`).

Sonst am Schritt 6 geprüft und in Ordnung: `unterordner_sicherstellen` verhält sich nach dem Umbau in allen vier Eingangslagen wie vorher und prüft zusätzlich nach dem Anlegen; `ueber_runden_einig` (`:1541-1566`) bricht bei leerer Rundenliste ab statt auf 0 zu fallen; `Gesamtrohrunde::default` wird nirgends in `rohrunden` geschoben; die neue Kopfzeile ist von `bericht.rs:896-902` gehalten.

**Befund 2 (Medium):** `--seed` fehlt in der Abhilfe.
**Befund 3 (Medium):** der Durchstich prüft nichts.
**Befund 4 (Medium):** B und der L6-Unterordner nur gegen ihren Steckbrief; der Kommentar zu B.

## Prüfpunkt 4: Prosa gegen Baum

`CLAUDE.md:133` (Schritt 5), Satz für Satz gegen den Baum gelesen:

| Aussage | Beleg |
|---|---|
| „`Kommando::KENNUNGEN`, in derselben Datei wie `wirkungsbereich`" | beide in `krk-core/src/tasten/belegung.rs` (`:697`, `:812`) — trägt |
| „Ohne Zeile dort übersetzt das Kommando" | die Längenangabe zwingt zu 79 Einträgen, nicht zu bestimmten; `wirkungsbereich` und `bereich_des_kommandos` sind die zwei, die der Übersetzer verlangt — trägt |
| „läßt sich in keiner Belegung an eine Taste binden (`Kommando::aus_kennung` findet den Namen nicht)" | `belegung.rs:805-810`, lineare Suche in `KENNUNGEN`, `None` — trägt |
| „bringt `kennung()` und `tag_des_kommandos` zum Absturz" | `belegung.rs:1116` `panic!`, `menue.rs:445` `expect` — trägt |
| „Gehalten wird sie deshalb von einer Probe und nicht vom Bau" | kein Rufer von `Kommando::kennung()` in einem `const`-Kontext; `funktion.kennung()` in `krk-ui` ist eine andere Methode auf `Funktion` — trägt |
| Probenname | siehe Befund 5 |

Keine Zahl im neuen Text; die Einfügung steht hinter dem Satz über den Übersetzer und macht ihn nicht falsch. Der Rückverweis „Der Ausführungszweig hält **er** nicht" bezieht sich weiterhin auflösbar auf den Übersetzer, weil die Einfügung „außerhalb **seiner** Reichweite" sagt.

`menue.rs:441-450`: zitiert jetzt beide Proben mit ihrer Rolle. Die Zuordnung ist richtig — `jedes_kommando_traegt_genau_einen_wirkungsbereich` hält die Eindeutigkeit wirklich (`belegung.rs:1710-1713`, `assert_ne!` über jedes Paar). Der Abschnitt „Ab welchem macOS…" der Datei ist unberührt.

`gemeinsam/mod.rs:64-66` und `:510-512`: die Behauptung über die fachliche Zeile der Rufer trägt nicht (Befund 1).

## Prüfpunkt 5: die Projektregeln des Plans

- **`#[must_use]`**: `pruefordner_pruefen` und `ueber_runden_einig` liefern `io::Result`, `kindauftrag` liefert `Option` — alle drei tragen die Marke über den Typ. `varianten_der_aufzaehlung` liefert `Vec<String>` ohne Marke; die Regel zählt über `crates/*/src` und erreicht `tests/` nicht, deshalb kein Befund, nur ein Hinweis. `ordner_beschreiben_mit_gelesenen` liefert `String` wie sein Geschwister `ordner_beschreiben`, konsistent.
- **Kein `libc`, kein neues `unsafe`**: die vier Diffs enthalten weder das Wort noch einen `extern`-Block; `#![deny(unsafe_code)]` ist unberührt.
- **Keine vierte Prüfordner-Fassung**: die neuen Proben in `messen.rs` nehmen `Wegwerfordner`, die in `belegung.rs` nehmen `Pruefordner` der Kernfassung. Nichts Neues.
- **Keine Zahl in `CLAUDE.md`**, die mit der nächsten Runde falsch wird: eingehalten. Die zwei neuen Zahlen stehen in Doc-Kommentaren (Befund 6).
- **Prosa deutsch, Bezeichner englisch beziehungsweise deutsch nach Hausform**: eingehalten.
- **`make check`**: alle vier Kommandos grün, in dieser Durchsicht selbst gefahren.

## Befunde

1. **Medium** — die sechs fachlichen `assert!` der Rufer von `kind_mit_deskriptorgrenze` sind seit dem Gate unerreichbar; drei Prosastellen behaupten das Gegenteil. `issues/260826-2152_o_die-sechs-fachlichen-assert-der-kindstarter-rufer-sind-seit-dem-gate-unerreichbar.md`
2. **Medium** — die Abhilfe in `pruefordner_pruefen` nennt einen `fixture`-Aufruf ohne `--seed`, und der bricht ab. `issues/260826-2153_o_die-abhilfe-in-pruefordner-pruefen-nennt-einen-fixture-aufruf-ohne-seed-und-der-bricht-ab.md`
3. **Medium** — `Durchstich::fahren` prüft seine Prüfordner überhaupt nicht und verspricht in Prosa weiter 10.000 und 100.000. `issues/260826-2154_o_der-durchstich-prueft-seine-pruefordner-ueberhaupt-nicht-und-verspricht-in-prosa-weiter-zehntausend.md`
4. **Medium** — Prüfordner B und der L6-Unterordner werden nur gegen ihren Steckbrief gehalten; der Kommentar zu B sagt, er werde nicht gelesen. `issues/260826-2155_o_pruefordner-b-und-der-l6-unterordner-werden-nur-gegen-ihren-steckbrief-gehalten-und-der-kommentar-sagt-b-werde-nicht-gelesen.md`
5. **Low** — der Probenname sagt „genau einmal", der Mengenvergleich kann eine Doppelung nicht sehen. `issues/260826-2156_o_der-probenname-sagt-genau-einmal-und-der-mengenvergleich-kann-eine-doppelung-nicht-sehen.md`
6. **Low** — zwei neue Doc-Kommentare schreiben die Zahl 79 fest, einer davon im aufzählungsneutralen Helfer. `issues/260826-2157_o_zwei-neue-doc-kommentare-schreiben-die-zahl-79-fest-einer-davon-im-aufzaehlungsneutralen-helfer.md`

Kein neuer Datensatz zu den `Resolved:`-Zeilen: der offene `260826-1933_o_die-zwei-resolved-zeilen-…` hat eine `Also seen:`-Zeile bekommen, weil die Schritte 3, 4 und 6 denselben Befund fortsetzen — fünf von fünf Datensätzen des Plans tragen jetzt Stempel oder Pfad statt Commit.

## Cross-cutting

**Eine Behebung schließt einen Weg und deckt dabei einen zweiten zu.** Zweimal in diesen vier Commits: das Gate im Kindstarter macht die sechs fachlichen Meldungen unerreichbar (1), und die Prüfordner-Prüfung deckt den Gesamtlauf, während der Durchstich mit derselben Lücke stehen bleibt (3) und zwei von vier Ordnern nur halb gedeckt sind (4). In beiden Fällen ist die Behebung richtig und ihre **Beschreibung** zu breit — Commit-Betreff, `Resolved:`-Zeile und Doc-Kommentar sagen „jeder" und „die fachliche Zeile bleibt", wo der Baum „für zwei von vier" und „nicht mehr erreichbar" sagt. Das ist dieselbe Klasse, die die Vollbaum-Durchsicht dutzendweise gefunden hat, hier von der Behebungsseite her erzeugt.

**Die Zahl in der Prosa neben der Zahl im Code** (6) ist die zweite: `9a4e495` baut eine Probe, die genau dagegen schützt, und schreibt im selben Diff die Zahl viermal in Doc-Kommentare, die keine Probe hält.

**Was diese Runde gut gemacht hat und was zu übernehmen ist:** der Helfer bricht ab, wo er nicht mehr weiß, statt still zu überspringen; der zweite stille Weg des Kindstarters ist strukturell geschlossen statt geprüft; `pruefordner_pruefen` steht an einer Stelle für vier Ordner statt viermal. Alle drei sind die Bauform, die der zweite Plan fortsetzen sollte.

## Recommended sequencing

Nichts blockiert eine Auslieferung; keiner der sechs Befunde ist ein Rückschritt gegenüber `9c02863`.

- **Vor dem nächsten Abnahmelauf des Nutzers**: Befund 2 (die Meldung, die er im Abbruchfall liest, muß laufen) und Befund 4 (der Kopf des Berichts, den derselbe Lauf schreibt).
- **In den zweiten Plan**: Befund 1 (die Vorlage `mit_zeitschranke` steht im selben Modul), Befund 3 (`pruefordner_pruefen` ist schon frei), Befund 6 (zwei Doc-Kommentare).
- **Nutzerentscheidung, klein**: Befund 5, weil der Name an drei Stellen zitiert ist; und die zweite Hälfte von Befund 4, „ehrlich beschriften" gegen „zweite Hälfte nachziehen".

## Verification

`git show <commit>` je Commit und `git diff --stat 9c02863..fc829c8`; die neun geänderten Dateien am Arbeitsbaum gelesen; `grep`/`awk` am Baum für `current_exe`, `--ignored`, `AUFTRAG_`, `pub enum` mit `#[cfg`, `; 79]`, `ordner_b`, `Messreihe::fahren`, `kennung()`; `cargo test --workspace`, `cargo clippy --workspace --all-targets`, `cargo fmt --all --check` gefahren, alle drei grün.

---

**Reconciled: 260826-2205** — die sechs Befunde am Baum `bc5991d` nachgeprüft, alle sechs
stehen zu Recht offen: die Rufer-`assert!` hinter dem Gate (`crates/krk-core/tests/umfang.rs:264`
und fünf gleichartige), `--seed` fehlt weiter in beiden Abhilfen (`crates/krk-bench/src/messen.rs:1594`
und `:1603` gegen `crates/krk-bench/src/main.rs:163`), `Durchstich::fahren` (`messen.rs:763`)
prüft weiter nichts, B und der L6-Unterordner werden weiter nur gegen ihren Steckbrief gehalten
(`messen.rs:1063-1077` gegen `:1266-1268`), der Mengenvergleich an
`crates/krk-core/tests/belegung.rs:1760` kann eine Doppelung weiter nicht sehen, und die Zahl 79
steht weiter in `crates/krk-core/tests/gemeinsam/mod.rs:377-378` und `tests/belegung.rs:1751-1752`.
Die vier Behebungen selbst halten. Die Aussage „`make check`: alle vier Kommandos grün" ist beim
Abgleich unabhängig nachgefahren, über `bc5991d`, Ausstiegscode 0.
