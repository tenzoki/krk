# Coder: Das Konfliktblatt bei genau einer Zieldatei

**Datum:** 2026-08-25 11:40
**Status:** Complete
**Agent:** coder
**Baumstand:** `a34a3f0` plus die Änderungen dieses Schritts

## Auftrag

Schritt 8 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`, dazu
eine Prosastelle, die Schritt 6 stehen lassen musste.

**Aufgabe A:** Nach der Antwort auf `decisions/260825-0711_a_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
(Möglichkeit 2) bietet das Konfliktblatt bei genau einer Zieldatei drei Antworten statt vier, und
das Ankreuzfeld „für alle weiteren" entfällt. Überschreiben auf `Taste::EingabeMitBefehl`,
Umbenennen auf `Taste::EingabeMitWahl`, Abbrechen auf `Taste::Eingabe` mit
`Wirkung::Liegenlassen`. Die Rückrechnung von der Schaltflächenstelle auf `Konfliktantwort`
bekommt eine zweite Tafel, und beide Tafeln bekommen je eine Probe.

**Aufgabe B:** Der Modulkopf von `crates/krk-ui/src/appkit/teilen.rs` sagte „Die Dateiliste, der
Editor und die Vorschau … bauen kein Menue"; seit Schritt 6 baut die Dateiliste dort drei eigene
Einträge. Nur Prosa, keine Zeile Code in jener Datei.

## Was entstanden ist

In `crates/krk-ui/src/appkit/blaetter/konflikt.rs`, vier neue Funktionen und ein umgeschriebener
Modulkopf:

- `schaltflaechen(genau_ein_ziel) -> Vec<Schaltflaeche<'static>>` — beide Gestalten als **eine**
  Angabe, herausgezogen nach dem Vorbild von `loeschbestaetigung::schaltflaechen`. Die volle
  Gestalt ist Zeile für Zeile die bisherige; die gekürzte lässt „Überspringen" weg und legt die
  Eingabetaste auf „Abbrechen".
- `antwort(stelle, genau_ein_ziel, name) -> Konfliktantwort` — die Rückrechnung mit ihren zwei
  Tafeln, beide im Doc-Kommentar ausgeschrieben. Der Auffangzweig fällt in beiden Gestalten auf
  `Abbrechen`; eine vollständige Fallunterscheidung ist über `usize` nicht zu haben, und
  `Blatt::zeigen_mit_wahl` bildet eine unbekannte Antwort ohnehin schon auf `abbruchstelle` ab.
- `tastenhinweis(genau_ein_ziel) -> &'static str` — die Erläuterung, je Gestalt genau einmal.
  Die gekürzte sagt „Return und Esc brechen ab, Cmd+Return überschreibt, Opt+Return benennt um."
- `zeigen` bekommt den Übergabewert `genau_ein_ziel` zwischen `vorschlag` und `fertig` und ruft
  `wahl_fuer_alle_zeigen` nur noch in der vollen Gestalt.

Fünf Proben unter `mod tests`, alle ohne AppKit und ohne Hauptfaden:

- `die_tafel_bei_mehreren_zielen` und `die_tafel_bei_genau_einem_ziel` — die zwei geforderten
  Tafelproben. Beide halten die Rückrechnung gegen die **Beschriftung** an der Stelle aus
  `schaltflaechen` und nicht gegen eine zweite Aufzählung im Probenrumpf; dreht sich eine der
  beiden Reihenfolgen ohne die andere, werden sie rot.
- `die_eingabetaste_traegt_in_keiner_gestalt_das_ueberschreiben` — der Sicherheitsgrund des
  Datensatzes, gemessen an `bestaetigungsstelle`.
- `beide_gestalten_lassen_ueber_abbrechen_liegen` — `abbruchstelle` trifft in beiden Gestalten
  „Abbrechen", und in der gekürzten fallen `abbruchstelle` und `bestaetigungsstelle` zusammen.
- `der_tastenhinweis_nennt_die_tasten_der_gestalt` — der gekürzte Satz sagt kein Überspringen an.

In `crates/krk-ui/src/appkit/anwendung.rs`: `konflikt_fragen(&self, frage, art: &Art)` fragt
`operationen::erzeugt_genau_ein_ziel(art)` und reicht die Antwort an `konflikt::zeigen`. Der
Doc-Kommentar sagt, warum die Rechnung dort steht und nicht im Blatt.

In `crates/krk-ui/src/kommandos/operationen.rs`: das `expect(dead_code)` an
`erzeugt_genau_ein_ziel` ist gefallen, der Doc-Kommentar nennt seinen einen Rufer.

In `crates/krk-ui/src/appkit/teilen.rs`: der Modulkopf, keine Zeile Code.

## Wo der Plan im Baum nicht hielt

**Erstens: eine dritte angefasste Datei.** Der Auftrag nannte `konflikt.rs`, `anwendung.rs`,
`teilen.rs` und ersatzweise `blaetter/mod.rs`. Gebraucht wurde stattdessen
`kommandos/operationen.rs`: `erzeugt_genau_ein_ziel` trug seit Schritt 4 ein
`#[cfg_attr(not(test), expect(dead_code, …))]`, dessen Grundtext den Aufrufer ausdrücklich diesem
Schritt zuschreibt. `unfulfilled_lint_expectations` ist unter `-D warnings` ein Fehler, und der
erste `make check` brach genau daran ab. `blaetter/mod.rs` blieb unberührt: `bestaetigungsstelle`
und `abbruchstelle` rechnen die gekürzte Gestalt ohne Änderung richtig.

**Zweitens: `konflikt_fragen` hatte die `Art` nicht.** Der Schritt sagt, `erzeugt_genau_ein_ziel`
werde dort gefragt, sagt aber nicht, woher der Wert kommt. Der Rufer eine Ebene darüber hält sie
bereits als Klon (`vorgang.art.clone()`); sie wird als `&Art` durchgereicht.

**Drittens: `fuer_alle_weiteren` fragt jetzt die Antwort und nicht die Stelle.** Bisher stand
`fuer_alle && stelle != 3`, und 3 ist die Stelle des Abbruchs — in der gekürzten Gestalt ist es
die 2. Gefragt wird deshalb `antwort != Konfliktantwort::Abbrechen`. In der vollen Gestalt ist
das verhaltensgleich bis auf eine Stelle jenseits der vierten, die es nach
`Blatt::zeigen_mit_wahl` nicht geben kann und die vorher ein angekreuztes Kästchen an einen
Abbruch gehängt hätte.

**Viertens: die gekürzte Gestalt trägt an keiner Schaltfläche `Taste::Escape`.** Ein `NSButton`
trägt genau eine Tastenentsprechung, und „Abbrechen" hat dort die Eingabetaste. Die Escape-Taste
erreicht dieselbe Schaltfläche über den Abbruchbefehl aus `resources/default-keymap.toml` und
`Blattgriff::abbrechen`, den `konflikt_fragen` als `offenes_blatt` hält — der Weg, den die
Löschbestätigung seit der Runde 12 fährt. Der Schritt nennt ihn, der Modulkopf schreibt ihn jetzt
aus.

**Nachgeprüft statt geglaubt:** `erzeugt_genau_ein_ziel` hängt beim Entpacken an
`ziele.len() == 1` (`operationen.rs`), und `Art::Zippen { ziel: PathBuf }` trägt genau ein Ziel,
also ist die gekürzte Gestalt beim Packen immer und beim Entpacken bei einem einzelnen Archiv der
Fall. Alle vier Werte von `Konfliktantwort` werden im Kern weiterhin behandelt, in `zippen.rs`
und in `entpacken.rs`; „Überspringen" ist damit nicht unerreichbar geworden, denn
`konflikt_fragen` bildet einen leeren Namen im Umbenennenfeld weiterhin darauf ab.

## Aufgabe B: die Prosastelle in `teilen.rs`

Der Satz „[`eintrag_anfuegen`] ist der **eine** Menuebauer. Die Dateiliste, der Editor und die
Vorschau … sie bauen kein Menue" ist auf den Freigabeeintrag zurückgeschnitten: `eintrag_anfuegen`
ist der eine Bauer **des Freigabeeintrags**, und keine der drei Flächen baut sich einen zweiten.
Ein Absatz daneben sagt, was der Baum seit Schritt 6 tut, nämlich dass die Dateiliste in
`menuNeedsUpdate:` erst ihre drei eigenen Einträge anlegt und `eintrag_anfuegen` danach ruft, und
warum die beiden Zählproben davon unberührt bleiben.

Daneben **eine Zeile der ASCII-Skizze**, über den Auftrag hinaus: die Gruppenüberschrift „baut
keines und nimmt das Menue der Ansicht" trug für die Dateiliste dieselbe falsche Lesart wie der
Satz darüber und heißt jetzt „bringt kein eigenes Menue mit und nimmt das der Ansicht". Der
Unterschied, den die Skizze zeigt, ist der zwischen dem Delegiertenhaken einer `NSTextView` und
`setMenu:` samt `menuNeedsUpdate:`, und der besteht unverändert.

## Abnahme

`make check` — Exit 0. `krk-ui` fährt 790 Proben statt 785; die fünf neuen stehen alle in
`appkit::blaetter::konflikt::tests`.

## Was offen bleibt

Der Datensatz `decisions/260825-0711_a_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
behält vorerst seinen Marker `_a_`. Die Umsetzung steht im Baum, aber der Auftrag verbot das
Committen, und ein `Implemented:` ohne Commit-Hash wäre kein Beleg. Wer diesen Schritt eincheckt,
trägt den Hash dort nach und benennt `_a_` in `_i_` um.

Schritt 7 hängt an diesem hier und ist nicht gefahren; ohne ihn ist die gekürzte Gestalt am
laufenden Bündel noch nicht auszulösen, weil kein Kontextmenü-Befehl beim Anwendungsdelegierten
ankommt.
