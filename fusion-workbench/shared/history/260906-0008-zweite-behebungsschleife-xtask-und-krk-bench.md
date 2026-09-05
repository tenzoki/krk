# Zweite Behebungsschleife: acht Defekte und das Dokumentationstor in `xtask/` und `crates/krk-bench/`

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Datum:** 260906-0008
**Baumstand bei Beginn:** `ba0c6bd`
**Grenze:** `xtask/`, `crates/krk-bench/`, `Makefile`, `release.sh`, `certify-only.sh`. Nicht
`CLAUDE.md`, nicht `README.md`, nicht die Wurzel-`Cargo.toml`, nicht `crates/krk-core/`, nicht
`crates/krk-ui/`; zwei weitere Agenten arbeiteten zeitgleich in den zwei Kisten.

## Verifikation

```
cargo test -p xtask -p krk-bench                                  → exit 0 (164 + 67 Proben)
cargo clippy -p xtask -p krk-bench --all-targets -- -D warnings   → exit 0
cargo fmt -p xtask -p krk-bench -- --check                        → exit 0
RUSTDOCFLAGS="-D warnings" cargo doc -p xtask -p krk-bench --no-deps → exit 0
```

Keine Auslieferung angestoßen, kein `cargo xtask bundle`, kein Git-Kommando über den ganzen Baum.
`make check` bewusst nicht gefahren: es prüft den ganzen Arbeitsbereich und wäre an den Dateien
der parallel laufenden Agenten abgebrochen.

## Das Dokumentationstor

Vorher sechs Warnungen auf `krk-bench`, null auf `xtask`; nachher null und null. Alle sechs waren
Verweise auf `crate::wegwerfordner::Wegwerfordner` in `fixture.rs` und `messen.rs`. Das Modul ist
`#[cfg(test)]`, `cargo doc` übersetzt ohne `cfg(test)`, und rustdoc fand es deshalb nicht. Die
sechs stehen jetzt als Code und nicht als Verweis; die Begründung dazu steht bei der
Moduldeklaration in `crates/krk-bench/src/main.rs`, samt der Angabe, warum ein
`#[cfg(any(test, doc))]` dort nicht hilft (das Modul ist privat, und rustdoc trüge dann
`private_intra_doc_links` statt `broken_intra_doc_links` vor).

**Verhältnis zur offenen Frage
`shared/decisions/260905-2336_o_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`,**
die ein zeitgleich laufender Agent für `krk-core` abgelegt hat: sie ist von diesem Durchgang
**nicht** vorweggenommen. Sie behandelt Verweise auf **private** Elemente
(`links to private item`); die sechs hier waren `unresolved link to` auf ein Modul, das es im
Dokumentationslauf gar nicht gibt. Für diese Art schreibt jede der vier dortigen Optionen
denselben Weg vor — Option 3 ausdrücklich („Fuer die vier braucht es zusaetzlich Option 1 oder
2"), Option 4 ebenso —, und Option 1 (öffentlich machen) ist bei einem `#[cfg(test)]`-Modul
nicht fahrbar. Fällt die Frage auf Option 1 oder 3, bleiben die sechs Stellen davon unberührt.

## Geschlossen (acht)

| Datensatz | Was getan wurde |
|---|---|
| `260826-1441` | Die drei offenen Stellen der Seite B (`certify-only.sh`, `Makefile`, `xtask/src/main.rs`) auf den Aufwand gestellt, die Station-1-Aussage als Bedingung formuliert. Dazu der Modulkopf von `xtask/src/beglaubigung.rs`, den der Abgleich vom 260905 als Vorlage geführt hatte: er trug die dritte, im Befund als falsch benannte Bedingung („nicht mehr allein auf HEAD") weiter. |
| `260826-1449` | Der Hilfetext zu Station 2 nennt beide Hälften der Prüfung. |
| `260821-1532` | `rustup` in `release::ziele_pruefen` begründet; der Modulkopf von `veroeffentlichung.rs` nennt `gh` nicht mehr als erste Ausnahme; drei Werkzeugaufzählungen durch ein Zählkommando ersetzt. Die Risikotabelle des Plans ist archiviert und bleibt unangetastet. |
| `260826-1302` (ein Lauf ohne Runden) | `runden == 0` weisen `Durchstich::fahren` und `Gesamtlauf::fahren` jetzt selbst ab, in der Form von `Messreihe::fahren`; zwei neue Proben. |
| `260826-1305` | Nachgeprüft, kein Code geändert: `Sitzungswaechter` und die zwei `bestanden` tragen `#[must_use]` seit dem 260905, der Rest hängt an `decisions/260905-2155`. |
| `260826-1443` | `veroeffentlichung::release_frei_pruefen` an Station 1, mit eigener Meldung und einer Probe, die die zwei Meldungen gegeneinander hält. |
| `260826-1444` | Die Aufzählung der Ablagedateien aus `RELEASETEXT` heraus; die Probe hält die neue Form von der anderen Seite. |
| `260826-1447` | `traegt_beide_architekturen` als dritte Frage von `signaturstand_pruefen`, gegen `release::ARCHITEKTUREN`; zwei neue Proben und eine neue Aufzeichnung `NUR_ARM`. |

## Unangetastet auf `_o_`

**Ausserhalb der Grenze** (`crates/krk-core/`, `crates/krk-ui/`), obwohl sie `xtask` oder
`krk-bench` nennen — meist in einem Suchkommando oder als Rufer:
`260826-1221` (Abschluss ist abgebrochen; Rufer in `messen.rs`),
`260826-1221` (zwei Verzweigungen über `Art`),
`260826-1225` (zwei öffentliche Zugänge der Ablage),
`260826-1302` (Probenziele des Kerns ohne `deny(unsafe_code)`),
`260826-1422` (Probenhelfer `liste` in `tabs.rs`),
`260826-1442` (Prüfordner von `krk-ui`, einstufig — mit Abgleichnotiz, siehe unten),
`260826-1442` (Spannenstrecke ohne Vordergrundvorbehalt),
`260826-1442` (Syntaxhervorhebung ohne Messstelle),
`260826-1442` (`Messplan::ordner_a` fällt auf die Wurzel zurück).

**An eine offene Nutzerfrage gebunden, laut Dispatch ausdrücklich nicht meine:**
`260826-1302` (vierte Prüfordner-Fassung `Wegwerfwurzel`) und `260826-2155` (Prüfordner B).
Entschieden ist an beiden nichts.

## Abgleichnotiz ohne Schließung

`260826-1442` (Prüfordner von `krk-ui`) fragte, ob `krk-bench/src/wegwerfordner.rs` dieselbe
Lücke hat. Antwort im Datensatz nachgetragen: die Bauform ist dieselbe (einstufiges
`remove_dir_all`), der Fall tritt dort aber nicht ein, weil in dieser Kiste niemand Rechte
zurückdreht (`grep -rn 'set_permissions\|0o000\|from_mode' crates/krk-bench/src` gibt keine
Zeile aus).

## Neu abgelegt

`shared/issues/260906-0008_o_readme-und-claude-md-fuehren-station-1-mit-drei-bedingungen-und-die-beglaubigung-mit-zwei-pruefungen.md`
— Folge der zwei Behebungen `260826-1443` und `260826-1447`: die `README.md` beschreibt Station 1
mit drei Bedingungen und die Beglaubigung mit zwei Prüfungen, der Code führt je eine mehr. Beide
Dateien lagen ausserhalb der Grenze.

## Zahlen im Baum

Keine Zahl ist als Zahl berichtigt worden. Was an ihre Stelle trat:

- Die drei Aufzählungen der Werkzeuge mit vollem Pfad (`veroeffentlichung.rs`-Modulkopf,
  `bundle.rs` bei `SYMBOLGROESSEN`, und dieselbe Aussage in `release.rs`) → das Kommando
  `grep -rhoE 'Command::new\("/usr/bin/[a-z]+"' xtask/src | sort -u`. Am Baumstand geprüft: es
  gibt genau die sechs Aufrufziele aus und zählt keine Doc-Zeile, keine Probe und keine
  Prosastelle mit, weil das Muster den Aufrufausdruck verlangt und nicht den blossen Pfad. Die
  Werkzeuge über den Suchpfad zählt daneben `grep -rn 'Command::new("[a-z]' xtask/src`; dass `gh`
  über die Konstante `GH` läuft und dort nicht auftaucht, steht an der Stelle dabei.
- „die zwei Bedingungen der Beglaubigung" (`signaturstand_pruefen`, zweimal) und „die zwei
  Pruefungen" (Modulkopf von `beglaubigung.rs`) → der Zeiger auf `signaturstand_pruefen`, dessen
  Rumpf die Fragen aufzählt.
- „die vier Ablagedateien" in `RELEASETEXT` → gar keine Liste mehr, und eine Probe, die eine neue
  verhindert.
- „drei Bedingungen" von Station 1 im Modulkopf von `release.rs` und im Hilfetext → beide zählen
  weiter auf, und das ist Absicht: es ist die Aufzählung selbst, die der Leser braucht, und sie
  steht jetzt an beiden Stellen vollständig. Die dritte Stelle, `README.md:272`, ist der neue
  Befund oben.

**Wo eine Zahl stehen blieb:** in der Begründung bei der Moduldeklaration in
`crates/krk-bench/src/main.rs` („Sechs solcher Verweise standen bis zum 260905"). Sie ist als
historische Angabe gestempelt und wird nicht mitwachsen; das Zählkommando daneben
(`RUSTDOCFLAGS="-D warnings" cargo doc -p krk-bench --no-deps`) gibt den heutigen Stand.
