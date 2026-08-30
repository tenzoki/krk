# Coder-Sitzung: Schritt 4 der Runde 23, der Gitlauf

**Date:** 2026-08-30, 260830-2358
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 4
**HEAD:** `1d84f2b` (nicht committet; der Orchestrator committet)

## Was gebaut wurde

### `crates/krk-core/src/git/lauf.rs` (neu)

`Gitlauf` nach der Bauform von `Durchlauf`, Zeile für Zeile: `starten(ordner, frage, generation)`
kehrt sofort zurück, `meldungen() -> &Receiver<Gitmeldung>`, `abbrechen()`, `impl Drop` setzt das
Kennzeichen und wartet nicht. Faden `krk-gitlauf-<n>`, `sync_channel(3)`.

`Gitfrage { Ganz, WeitererVerlauf { ab: ObjectId } }` und
`Gitmeldung { Kopf(Kopf), Verlauf(Vec<Commit>), Marken(Vec<(String, Marke)>) }`, beide vollständig
und ohne Auffangzweig. `VERLAUFSSCHRITT = 50` steht hier und nicht beim Anzeigenden, weil dieses
Modul der einzige Rufer von `Gitleser::verlauf` ist.

**Der Modulkopf trägt die zwei Unterschiede zum `Durchlauf`**, wie der Plan es verlangt: die
Kanaltiefe ist die Zahl der Antworten und kein Rückstaumaß, und die Marken kommen in einem Stück,
mit den zwei Gründen aus Entscheidung 3 (A8 will keine fortschreitend gefüllte Spalte; das
Nachschlagewerk über den Bestand wird einmal je Lauf gebaut und nicht je Schwung).

### Zwei Stellen, an denen der Bau über den Plantext hinausgeht

**Die Abbruchprüfung steht vor vier Einheiten und nicht vor drei.** Der Plan sagt „prüft das
Abbruchkennzeichen vor jeder der drei". Gebaut ist die Prüfung zusätzlich vor `oeffnen`. Der Grund
steht im Modulkopf: `gix::discover` kostet im positiven Fall gemessen 346 bis 900 µs und sieht je
Ebene bis zur Wurzel nach `.git`; die Regel des Durchlaufs lautet „geprüft wird vor jeder Einheit,
die dauern kann", und das Öffnen ist eine. Eine Prüfung weniger wäre eine Ausnahme von dieser
Regel ohne Grund.

**`Oeffnung::KeinRepository` meldet `Kopf::KeinRepository` bei beiden Fragen und nicht nur bei
`Ganz`.** Jeder Lauf muss öffnen, also beantwortet jeder die Frage, ob dieser Ordner überhaupt in
einem Repository liegt; ein Nachschlag auf einen Ordner, dessen Repository inzwischen weg ist,
meldet sie statt zu schweigen. Die Zusage aus dem Plan hält dabei unverändert: `WeitererVerlauf`
liefert **genau eine** Meldung, entweder den Verlauf oder diese eine entschiedene Verneinung. Der
Modulkopf schreibt es aus; eine Ausnahme wäre eine zweite Regel für dieselbe Frage.

### `crates/krk-core/src/git/mod.rs`

`pub mod lauf;` dazu. Der Abschnitt `# Was hier nicht wohnt` — er kündigte `git/lauf.rs` als
kommend an — ist durch `# Der eine Weg herein: der Kanal` ersetzt, der die Regel aus C7.1
ausschreibt und die Zählprobe namentlich nennt.

### `crates/krk-core/tests/gemeinsam/mod.rs` — **nicht in der Dateiliste des Schrittes**

`pub fn aufrufstellen(inhalt, name) -> usize`, die Bauform aus `krk-ui/src/quellbaum.rs`.

**Warum diese Datei trotzdem angefasst ist.** Der Plan verlangt die Zählprobe „über
`quellbaum::aufrufstellen`". Jene Funktion ist `pub(crate)` in `krk-ui`, einer Kiste mit nur einem
Binärziel; `crates/krk-core/tests/git.rs` erreicht sie nicht — dieselbe Kistengrenze, aus der es
drei Prüfordner-Fassungen gibt und aus der `quelldateien` schon zweimal im Baum steht. Die
Alternativen waren beide schlechter: eine private dritte Fassung in `tests/git.rs` (ein Doppelbau
mehr statt einem) oder eine Nadel über `contains("marken(")`, die `gueltige_marken(` in
`krk-ui/src/leistenmodell.rs` dreimal falsch träfe. Der Doc-Kommentar der neuen Fassung nennt die
Kistengrenze, den Defekt `260813-0540`, aus dem die Bauform stammt, und den Satz, dass wer eine
der beiden ändert, die andere mitändert.

### `crates/krk-core/tests/git.rs`

Fünf Proben dazu, dazu zwei Helfer (`meldungen_einsammeln`, `art`) und ein Abschnitt im Modulkopf,
der sagt, warum die Proben zweier Schritte in einer Datei stehen (beide brauchen dasselbe
Prüfrepository, und `repository` legt es an genau einer Stelle an).

| Probe | Kriterium |
|---|---|
| `ein_ganzer_lauf_meldet_kopf_verlauf_und_marken_in_dieser_reihenfolge` | C6.1 (Laufhälfte), A8 |
| `ein_nachschlag_meldet_allein_den_verlauf` | C4.2, C4.3 |
| `ein_abgebrochener_lauf_meldet_nichts_mehr` | Bedingung 3, Abbruchzusage |
| `ein_ordner_ohne_repository_meldet_kein_repository_und_danach_nichts` | C6.1 (Laufhälfte), E5 |
| `keine_statusabfrage_steht_ausserhalb_des_gitmoduls` | C7.1 |

**Die Reihenfolgeprobe vergleicht Meldungsarten und nicht Werte.** Eine Probe, die nur die Menge
der Meldungen hielte, ließe die umgekehrte Reihenfolge durch, und die Reihenfolge ist der
Gegenstand von A8.

**Die Abbruchprobe trägt einen Kontrollauf davor**, wie
`der_abbruch_greift_in_einem_ordner_ohne_unterordner` in `tests/verzeichnis.rs`: derselbe Ordner
meldet ohne Abbruch alle drei Auskünfte. Ohne ihn bestünde sie auch dann, wenn der Lauf
grundsätzlich nichts meldete. Ihre Voraussetzung steht im Doc-Kommentar und wird nicht
verschwiegen: das Kennzeichen wird auf dem Hauptfaden gesetzt, während der Arbeitsfaden erst
anläuft. Zehn Läufe hintereinander sind grün.

## Wie die Zählprobe für C7.1 gebaut ist

**Die prüfbare Gestalt der Zusage steht im Doc-Kommentar, und die Umformulierung ist der Kern.**
„Keine Statusabfrage läuft auf dem Hauptfaden" ist am Quelltext nicht entscheidbar — auf welchem
Faden eine Zeile läuft, sagt keine Nadel. Entscheidbar ist die Frage dahinter: gibt es überhaupt
einen zweiten Weg an den Status neben dem Kanal? `Gitleser::marken` ist der teure der vier Wege
(12 bis 164 ms gemessen); ruft ihn allein `git/lauf.rs`, kann ihn niemand sonst auf den Hauptfaden
legen.

Gebaut ist sie in zwei Teilen, nach dem Muster von
`keine_code_zeile_unter_leseprofil_erreicht_den_ausblendeschalter` in `tests/baum.rs`:

1. **Gegenprobe zuerst.** `aufrufstellen` auf `krk-core/src/git/lauf.rs` muss mehr als null
   liefern. Ohne sie bestünde die Probe nach jeder Umbenennung des Weges.
2. **Die Zählung.** Über `gemeinsam::quelldateien()`, gefiltert auf `/src/` und ohne
   `krk-core/src/git/`; erwartet wird null, und der Fehlschlag nennt die Dateien mit ihrer Zahl.

Die Nadel steht mit `concat!` zusammengesetzt da, weil die Datei in dem Baum liegt, den sie liest.
Die Grenze `/src/` ist dieselbe, die der Nutzer am 260830 für
`git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` gewählt hat: gezählt wird
ausgelieferter Code, und eine Probe, die den Leser prüft, ruft ihn und läuft auf keinem
Zeichendurchgang. Was die Nadel nicht sieht — ein `use … as anders;` und die
`#[cfg(test)]`-Module unter `src/`, die mitgezählt bleiben — steht im Doc-Kommentar.

## Prüfläufe

| Kommando | Ergebnis |
|---|---|
| `cargo test -p krk-core --test git` | 15 bestanden, 1 stillgelegt (die Kindprobe), 0 gescheitert |
| `ein_abgebrochener_lauf_meldet_nichts_mehr`, zehn Läufe | zehnmal grün |
| `make check` | **Exit 0** (build, test --workspace, fmt --check, clippy --all-targets -D warnings) |

`genau_drei_pruefordner_fassungen_stehen_im_baum` ist grün; eine vierte Prüfordner-Fassung
entsteht nicht. `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` bleibt grün:
dieser Schritt bringt keinen neuen `git`-Aufruf.

## Gefilte Datensätze

- `issues/260830-2358_o_die-datenstrukturen-des-plans-fuehren-vier-gitleser-signaturen-die-schritt-3-verworfen-hat.md`

## Was der nächste Schritt wissen muss

Schritt 5 baut den Gitbefund ins Ordnermodell. Zwei Dinge kommen aus diesem Schritt:

- **`Gitmeldung::Marken` kommt in einem Stück und höchstens einmal je Lauf.** Ein `gitmarken_setzen`
  je Schwung ist nicht vorgesehen und wäre das Flackern, das A8 ausschließt.
- **Ausbleiben heißt unentschieden.** Ein geschlossener Kanal ohne `Marken`-Meldung ist kein
  „dieser Ordner hat keine Marken"; das Modell lässt die Spalte dann, wie sie war, und schreibt
  nichts.

Die Generation liegt den Meldungen **nicht** bei — jeder Tab liest allein aus seinem eigenen Kanal
—, und `gitmarken_setzen` hält sie trotzdem gegen: der Befund trägt hier einen Namen, den auch ein
neuer Ordner führen kann, während der Durchlauf einen Eintragsindex trägt, den das Modell am
Bestandsende von selbst verwirft. Der Doc-Kommentar von `Gitlauf::starten` sagt es.
