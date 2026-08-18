# Coder — Schritt 7: Die reine Regel — Marke, Schreibrecht, Urteil

**Datum:** 260818-2155
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 7
**Baumstand beim Beginn:** `79f52af`. Der Arbeitsbaum war **nicht** sauber: zwei
weitere Agenten arbeiteten zeitgleich in `appkit/anwendung.rs` (Schritt 9) und
`appkit/zwischenablage.rs` (Schritt 6). Beide Dateien blieben unangetastet.

## Was der Auftrag war

Das erste Stück von Bündel B, das ohne Fenster auskommt: ein neues Modul
`crates/krk-ui/src/kommandos/abwurfregel.rs` mit den fünf Typen und den zwei
reinen Funktionen aus dem Abschnitt `## Data Structures` des Plans, dazu die
Anmeldung in `kommandos/mod.rs`. Keine Zeile `objc2`. Vorbild in Form und
Geist ist `kommandos/rueckschritt.rs`.

## Was entstanden ist

**`crates/krk-ui/src/kommandos/abwurfregel.rs`** (neu, 938 Zeilen) mit sechs
Typen und zwei Funktionen:

| Stück | Was es trägt |
|---|---|
| `Abwurfmarke` | `Zeile` und `Liste` — beide sagen zugleich, welcher Ordner das Ziel ist |
| `Schreibrecht` | `Ja`, `Nein`, `Unbekannt` |
| `Abwurfgrund` | die fünf Gründe; nur `KeineDatei` trägt eine Meldung |
| `Abwurfvorgang` | `Kopieren`, `Verschieben` |
| `Abwurfurteil` | `Ausfuehren(…)` oder `Abweisen(…)`, kein dritter Zweig |
| `Abwurflage` | die sechs Tatsachen, jede mit genau einem Beschaffer |
| `marke(auf_die_zeile, typ_der_zeile)` | die Tafel aus C4, fünf Arme über acht Kombinationen |
| `urteil(&Abwurflage)` | die Reihenfolge aus C6 und die Tafel aus C5 |

Beide Funktionen tragen `#[must_use]` mit ausgeschriebenem Grund, beide
`#[cfg_attr(not(test), expect(dead_code, reason = …))]` bis Schritt 10.

**`crates/krk-ui/src/kommandos/mod.rs`** — `pub mod abwurfregel;`, der Eintrag
im Modulverzeichnis des Kopfes, „Acht Module" auf „Neun", und ein Absatz
darüber, warum das eine Modul hier wohnt, das kein Tastenbefehl ist.

## Jede Tafel steht dreimal, und keine Erwartung ist gerechnet

Wie der Plan es verlangt: im Doc-Kommentar, als `match`, und als Probenfeld.
Auffangzweige gibt es nirgends; `Typ` und `Schreibrecht` sind geschlossen, also
hält der Übersetzer die Vollständigkeit.

- `die_tafel_der_marke_geht_auf` — alle 8 Kombinationen von `auf_die_zeile`
  mal `typ_der_zeile`.
- `die_tafel_der_abweisungen_geht_auf` — alle 24 Kombinationen der vier
  führenden Größen, das Angebot fest auf „beides".
- `die_tafel_des_angebots_geht_auf` — alle 4 Kombinationen der angebotenen
  Menge.

**Die 24er-Tafel bleibt eine Zusage schuldig**, und die Lücke ist eigens
geschlossen: sie stellt das Angebot in allen 24 Zeilen fest, misst also nicht,
dass die vier Abweisungen es gar nicht lesen.
`die_vier_abweisungen_fragen_das_angebot_nicht` fährt jeden der vier Gründe
über alle vier Angebotslagen. Zusammen decken beide die 96 Kombinationen der
sechs Größen ab, ohne dass eine Tafel mit 96 Zeilen dastünde. Der Doc-Kommentar
beider Proben sagt das gegenseitig.

Dazu vier Proben, die je eine Zusage und nicht ein Tafelfeld messen:
`allein_die_ordnerzeile_wird_hervorgehoben`,
`eine_verknuepfung_verhaelt_sich_wie_eine_datei`,
`ein_unbekanntes_schreibrecht_laesst_durch` und
`kopieren_geht_dem_verschieben_vor`.

## Die zwei Aufruferzählungen erwarten null, und das ist gemessen

`die_marke_hat_noch_keinen_aufrufer` und `das_urteil_hat_noch_keinen_aufrufer`,
nach dem Vorbild von `die_regel_hat_genau_einen_aufrufer`. Schritt 10 setzt
beide auf eins.

**Sie lesen `krk-ui/` und nicht den ganzen Baum, und das ist keine stille
Verengung.** Zwei Gründe stehen an der Stelle:

1. `krk-ui` hat kein Bibliotheksziel. Keine andere Kiste erreicht dieses Modul,
   ein Aufrufer außerhalb des Präfixes kann es nicht geben. Das Präfix ist der
   genaue Umfang der Zusage, nicht ihre Beschneidung.
2. Für `urteil` ist es **nötig**: `krk-bench` führt unter demselben Namen eine
   eigene Funktion, die das Urteil einer Zeitzusage formuliert
   (`krk-bench/src/messen.rs:1986`). `quellbaum::aufrufstellen` unterscheidet
   Namensgleiches nicht.

Nachgemessen, nicht angenommen: ohne das Präfix zählt `urteil` **5**
Fundstellen aus `krk-bench` (`messen.rs:1932`, `:2504`, `:2515`, `:2576`,
`bericht.rs:360`), mit dem Präfix 0. Ein Aufruf der Bauform
`abwurfregel::urteil(&lage)` zählt als 1. Die Zählung misst also, was sie sagt.
Die verbleibende Blindheit steht im Doc-Kommentar, wie `quellbaum` es verlangt:
ein `use … as anders` wird nicht gesehen, und eine zweite Fassung **innerhalb**
von `krk-ui` würde als Aufrufer mitgezählt statt als Doppelbau erkannt.

## Drei Entscheidungen, die der Plan offenließ

**1. `Schreibrecht` trägt seine eigene `expect(dead_code)`-Ausnahme.** Der Plan
hängt die Ausnahme an die beiden Funktionen. Das reicht nicht: die drei Werte
von `Schreibrecht` werden hier nur **gelesen** und nirgends gebaut, und der
Übersetzer meldet sie einzeln (`variants … are never constructed`). Gebaut
werden sie in `abwurf::beschreibbarkeit`, also in **Schritt 8** und nicht in
Schritt 10. Die Ausnahme steht deshalb an der Aufzählung selbst und nennt
Schritt 8 als ihr Ablaufdatum. Die übrigen fünf Typen brauchen keine: sie
entstehen in den Rümpfen der zwei Funktionen, die ihre Ausnahme schon tragen.

**2. `Abwurfvorgang` steht in diesem Modul.** Der Dispatch zählt ihn nicht
unter den Typen auf, der Abschnitt `## Data Structures` des Plans führt ihn
aber in derselben Datei, und `Abwurfurteil::Ausfuehren(Abwurfvorgang)` braucht
ihn. `tabelle.rs` nennt ihn in Schritt 10 in seinen Ivars.

**3. Der Datensatz zum unentscheidbaren Schreibrecht behält `_a_`.** Die
Antwort ist in der Tafel von `urteil` ausgeschrieben und im Modulkopf mit ihrem
Grund zitiert, aber der Weg, den sie regiert, steht erst nach Schritt 10, und
`Implemented:` verlangt einen Commit-Hash, den dieser Lauf nicht hat. Die
Umbenennung auf `_i_` gehört an denselben Ort wie die des Zusatztasten-
Datensatzes, den der Plan ausdrücklich nach dem Commit von Schritt 10 umsetzt.

## Was ausdrücklich nicht angefasst wurde

- `crates/krk-ui/src/appkit/anwendung.rs` und
  `crates/krk-ui/src/appkit/zwischenablage.rs` — Schritt 9 und Schritt 6, in
  der Hand zweier anderer Agenten.
- Keine `use objc2`-Zeile im neuen Modul. Nachgeprüft: die einzige `use`-Zeile
  ist `use krk_core::verzeichnis::Typ;`.
- `verweisziel::bestimmen` steht nicht in der Tafel von `marke`. Die Signatur
  lässt den Einbau gar nicht zu; sie nimmt einen bereits bestimmten `Typ`
  entgegen.

## Ein Nebeneffekt, der zu melden ist

Der Formatierlauf lief als `cargo fmt -p krk-ui` und damit über die ganze
Kiste, nicht nur über die zwei eigenen Dateien. `crates/krk-ui/src/appkit/anwendung.rs`
trägt danach denselben Änderungszeitpunkt, es ist also möglich, dass rustfmt
die Datei eines zeitgleich arbeitenden Agenten umformatiert hat. Der Eingriff
wäre rein kosmetisch — rustfmt entfernt keinen Code, und `fmt --check` verlangt
dasselbe Ergebnis ohnehin —, aber er lag außerhalb der zugewiesenen Grenze.
Richtig wäre `rustfmt` auf den zwei eigenen Dateien gewesen.

## Abnahme

`make check` — Exit 0, alle vier Kommandos grün (Bau, 1346 Proben, `fmt
--check`, Clippy unter `-D warnings`). Die zehn neuen Proben laufen grün. Vor
dem Lauf geprüft, dass weder `/tmp` noch `$TMPDIR` eine `krk-messplan-*.toml`
führt; es lief kein Messlauf.

**Nicht committet** — der Nutzer committet selbst.
