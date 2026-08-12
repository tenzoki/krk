# Coder, Schritt 5: Zwei neue Umschaltbefehle, Editor und linkes Dateifenster

**Datum:** 260812-0548
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 5
**Abnahme:** `make check` — Exit 2. Der Baum bleibt rot, und der Grund liegt außerhalb dieses
Schrittes: `issues/260812-0548_o_make-check-bleibt-auch-nach-schritt-5-rot-….md`

## Auftrag

Zweierlei in einem Zug. Erstens Schritt 5 des Plans: die beiden Kommandos
`ErstesFensterUmschalten` und `EditorUmschalten`, ihre Wirkungsbereiche, ihre
Funktionsbereiche, ihre Zweige beim Anwendungsdelegierten und die neue Funktion
`editor_umschalten`. Zweitens der Nachtrag der drei Proben, die seit Schritt 4 gegen die
Belegungsdatei stehen und zu keinem Planschritt gehören
(`issues/260812-0533_*`, Abschnitt `## Was hilft`). Nicht committen; der Orchestrator trägt ein.

Der Baum war beim Beginn rot, mit 32 Fehlschlägen: zwei in `krk-core`, dreißig in `krk-ui`.

## Was entstanden ist

**`crates/krk-core/src/tasten/belegung.rs`**

- `Kommando` trägt zwei Varianten mehr, `ErstesFensterUmschalten` neben
  `ZweitesFensterUmschalten` und `EditorUmschalten` neben `EditorSchliessen`. `KENNUNGEN` steht
  auf 70 Einträgen; die Feldbreite in der Typangabe hat den Bau angehalten, bis beide Zeilen
  standen, wie der Plan es vorhergesagt hat.
- `wirkungsbereich`: beide im Zweig `Wirkungsbereich::Ueberall`, bei den drei bestehenden
  Umschaltbefehlen. **Die Aufzählung `Wirkungsbereich` selbst ist unverändert**, ebenso
  `Bereich` und `Fokus`.
- **`editor_umschalten` fällt in diesem Zweig auf, und der Kommentar dort sagt jetzt warum.**
  Jeder andere Befehl mit `editor_` im Namen trägt weiter unten `Wirkungsbereich::Editor`. Der
  Unterschied ist derselbe wie zwischen `vorschau_umschalten` und den Befehlen, die in der
  Vorschau arbeiten: ein Umschalter braucht seinen Bereich nicht, er stellt ihn her. Mit
  `Wirkungsbereich::Editor` wäre gerade der Befehl abgewiesen, der den Editor wieder loswerden
  will.
- Die Dokumentationskommentare der beiden Editor-Befehle **verweisen aufeinander und nennen den
  Unterschied**: das Schließen gibt die Datei auf und löst die Nachfrage aus C4 der Editor-Runde
  aus, das Umschalten blendet aus und behält den Stand. Der Plan hat die Verwechslung der beiden
  als Risiko benannt; ohne die zwei Kommentare läse sich der eine wie eine Dublette des anderen.

**`crates/krk-ui/src/belegungsmodell.rs`**

- `bereich_des_kommandos`: `ErstesFensterUmschalten` zu `Funktionsbereich::Fenster`, neben dem
  rechten Dateifenster; `EditorUmschalten` zu `Funktionsbereich::Editor`, neben dem Schließen.
- Ein Kommentar begründet, warum der Editorschalter unter „Editor" steht und die beiden
  Dateifenster unter „Fenster". Es ist dieselbe Regel und kein Widerspruch: die Gliederung fragt
  nach der Gegend der Anwendung, und ein Dateifenster ist keine eigene Gegend der Belegung, es
  gibt keinen Abschnitt dafür.
- Neue Probe `die_beiden_neuen_umschalter_stehen_in_ihrem_bereich`, wie der Plan sie verlangt.

**`crates/krk-ui/src/appkit/anwendung.rs`**

- Zwei Zweige in `kommando_ausfuehren`: `ErstesFensterUmschalten` geht auf
  `bereich_umschalten(Bereich::Links)`, `EditorUmschalten` auf die neue Funktion.
- `editor_umschalten` ist kurz und gebaut wie `fokus_editor_holen`: ist der Editor ausgeblendet
  und hält keine Datei, geschieht nichts und wird nichts gemeldet; sonst geht der Weg durch
  `bereich_umschalten(Bereich::Editor)` und damit durch dieselbe Stelle wie die vier anderen
  Bereiche. **Die Bedingung steht beim Anwendungsdelegierten und nicht im Fenstermodell**, aus
  dem Grund, den der Datensatz `decisions/260812-0415_a_was-tut-der-editorschalter-ohne-datei-im-editor.md`
  nennt: das Fenstermodell weiß von Dateien nichts. Umgesetzt ist dessen Antwort vom 260812-0430,
  Möglichkeit 1, ohne Meldung verwerfen.
- **`editor_schliessen` ist unverändert geblieben**, bis auf seinen Dokumentationskommentar, der
  jetzt auf den Umschalter verweist.

## Die drei Proben aus dem Befund des ontocoders

Der Vorschlag des Befunds ist nach eigener Prüfung übernommen worden, in allen drei Punkten.

1. **`die_auslieferungsbelegung_fuehrt_vierundsiebzig_funktionen` ist gestrichen**, nicht auf 79
   umgeschrieben. Ihre Zahlenzusage trägt `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
   vollständig und ohne Literal. **Der zweite Teil ist erhalten**, als
   `die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung`, ohne Zahl im Namen und
   ohne Zählung im Rumpf. Erhalten und nicht gestrichen, weil er in dieser Kiste nirgends sonst
   steht: `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` erreicht elf der
   dreizehn, denn `text_rueckgaengig` und `text_wiederholen` tragen kein Kommando, das Menü
   stellt sie zu.
2. **Die Ausnahme in `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` ist
   von `reserviert_fuer` gelöst**, als benannte Liste `OHNE_KOMBINATION_AB_WERK` der drei
   Spaltenkennungen im Prüfcode, mit Verweis auf `decisions/260812-0306_a_bekommen-die-spaltenschalter-tastenbefehle.md`.
   Sie **erlaubt** die leere Tastenliste und **verlangt** sie nicht: der Datensatz begründet die
   leere Liste mit der Knappheit der freien Kombinationen und verbietet keine spätere. Der Name
   der Probe bleibt, weil Plan und Befund sie so nennen.
3. **Der letzte `assert_eq!` in `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
   nennt die drei Spaltenschalter beim Namen**, statt eine Zahl zu vergleichen. Eine Zahl sagte
   nicht, welche Funktion aus der Ausgabe fällt, und das ist die Auskunft, die ein Leser der
   Probe braucht.

**Eine vierte Probe kam dazu, und sie gehört diesem Schritt.**
`der_bereich_editor_fuehrt_die_zwoelf_befehle_der_runde` heißt jetzt
`der_bereich_editor_fuehrt_genau_die_befehle_des_editors` und führt dreizehn Kennungen: mit
`editor_umschalten` tritt eine Funktion unter die Überschrift „Editor", und die Probe zählt am
Ende ab. Der Name nennt die Zahl nicht mehr, aus dem Grund, den
`die_ab_werk_freien_kombinationen_kommen_nicht_vor` in `crates/krk-core/tests/belegung.rs` schon
ausschreibt: eine Zahl im Namen bindet die Probe an die Größe ihrer Liste statt an ihre Zusage.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | grün |
| `cargo fmt --all --check` | grün, Exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | grün |
| `cargo test --workspace` | **rot**, Exit 101 |
| `make check` | **rot**, Exit 2 |

`krk-core` ist vollständig grün, einschließlich der beiden Proben, die Schritt 4 rot
zurückgelassen hatte (`crates/krk-core/tests/belegung.rs`: 45 bestanden, 0 gescheitert). Rot
bleiben 28 Proben im Binärziel `krk`: 345 bestanden, 28 gescheitert.

**Jeder der 28 Fehlschläge nennt `spalte_groesse_umschalten`, und keiner nennt
`erstes_fenster_umschalten` oder `editor_umschalten`.** Die Ursache ist eine Stelle:
`belegungsmodell::bereich` findet für eine Kennung ohne Kommando nur dann einen
Funktionsbereich, wenn sie namentlich im zweiten Zweig steht, und dort stehen allein die sechs
vom Menü zugestellten Textbefehle. `nach_bereichen` bricht daraufhin laut ab, und mit ihm jede
Probe, die die Belegungsansicht oder die Markdown-Ausgabe über die Auslieferungsbelegung baut.
Die drei Spaltenschalter bekommen ihr Kommando in Schritt 7.

**Damit ist die Abnahme des Schrittes verfehlt und der Schritt selbst nicht.** Der Plan ordnet
die Belegungsdatei bewusst vor die Kommandos, die sie nennen, und der Preis dieser Ordnung ist
ein roter Baum über drei Schritte statt über einen. Der ontocoder hat das für Schritt 4 schon
festgehalten; der eigene Datensatz für Schritt 5 ist
`issues/260812-0548_o_make-check-bleibt-auch-nach-schritt-5-rot-die-drei-spaltenkennungen-warten-auf-schritt-7.md`.
Er schlägt vor, die Abnahme der Schritte 4, 5 und 6 im Plan nachzuziehen.

## Was offen bleibt

- **`make check` ist rot** und wird es bis zum ersten Teil von Schritt 7 bleiben, den drei
  Varianten in `Kommando` samt ihren Zeilen in `wirkungsbereich` und `bereich_des_kommandos`.
- **Eine Zusage ist ungeprüft**: der neue letzte `assert_eq!` in
  `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` wird heute nicht erreicht, weil
  die Probe vorher am fehlenden Funktionsbereich abbricht. Wer Schritt 7 abnimmt, liest sie
  eigens nach.
- **Der Datensatz `260812-0533` bleibt offen und steht auf `_p_`.** Seine Punkte 1 bis 3 sind
  erledigt, sein Punkt 4 nicht: die zehn Prosastellen, die mit 74 Funktionen und 68 Kommandos
  rechnen, sind erst nach Schritt 7 endgültig nachzuziehen. Eine der zehn ist nebenbei
  weggefallen, der Kommentar „die Datei fuehrt alle 74" in `belegungsausgabe.rs`; neun stehen.
- **Die Wirkung der beiden Tasten am laufenden Bündel ist ungesehen.** `opt+cmd+left` und
  `opt+cmd+b` sind gebaut und geprüft, soweit es ohne Fenster geht; der Augenschein gehört zu
  Schritt 8 und ist Nutzerarbeit.
