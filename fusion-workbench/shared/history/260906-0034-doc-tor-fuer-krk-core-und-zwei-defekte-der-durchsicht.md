# Das Doc-Tor fuer krk-core steht auf Exit 0, dazu zwei Defekte der Durchsicht

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Baum bei Beginn:** `ea90b7e`
**Umfang:** ausschliesslich `crates/krk-core/`

## Auftrag

Eine Nutzerentscheidung umsetzen: die privaten Elemente bleiben privat, die Verweise
darauf bleiben Verweise. Grundlage ist
`260905-2336_*_wird-ein-privates-element-oeffentlich-oder-der-verweis-darauf-zu-fliesstext.md`.
Der Datensatz selbst ist unberuehrt geblieben; der Uebergang gehoert dem Orchestrator.

## Die Abweichung von der Nutzervorgabe, und wie sie gemessen ist

Der Nutzer hatte „werden Fliesstext" (Moeglichkeit 2) vorgegeben, mit ausdruecklicher
Erlaubnis, im begruendeten Fall anders zu entscheiden. Umgesetzt ist:

- **Moeglichkeit 3 fuer die Meldungen der Art `private_intra_doc_links`.** Ein
  `#![allow(rustdoc::private_intra_doc_links)]` an der Kistenwurzel; die Verweise bleiben
  stehen.
- **Moeglichkeit 2 fuer die vier Meldungen an den privaten Modulen `zippen` und
  `entpacken`.** Sie sind Fliesstext geworden, Backticks geblieben, eckige Klammern
  gefallen.

Der Datensatz fuehrt als Contra zu Moeglichkeit 3 an, ein `allow` an der Wurzel decke
kuenftig auch die Verweise, die wirklich falsch sind. **Das trifft nicht zu, und das ist
nachgemessen** an einer Wegwerfkiste ausserhalb des Quellbaums, drei Laeufe mit
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`:

| Lauf | Exit |
|---|---|
| privater Verweis, ohne `allow` | 101 |
| privater Verweis, mit `allow` | 0 |
| privater **und** kaputter Verweis, mit `allow` | 101 |

Der dritte Lauf bricht mit `error: unresolved link to` ab und nennt als Pruefer
`-D rustdoc::broken-intra-doc-links`. `private_intra_doc_links` und
`broken_intra_doc_links` sind zwei Pruefer; das `allow` fuer den ersten laesst den zweiten
unberuehrt. Moeglichkeit 3 gibt die Namenspruefung also nicht auf, Moeglichkeit 2 gaebe sie
fuer 53 Stellen dauerhaft auf.

## Was geaendert ist

- `crates/krk-core/src/lib.rs` — `#![allow(rustdoc::private_intra_doc_links)]` direkt unter
  `#![deny(unsafe_code)]`, mit einem Kommentar, der sagt: warum die Zeile dasteht, dass sie
  `broken_intra_doc_links` nicht anruehrt (mit der Messung vom 260906-0034),
  dass `--document-private-items` diese Verweise aufloest, wie die gedeckten Verweisstellen
  zu zaehlen sind, und mit dem Verweis auf den Entscheidungsdatensatz.
- `crates/krk-core/src/verzeichnis/sys.rs` — vier Verweise zu Fliesstext:
  `crate::operation::zippen` und `crate::operation::entpacken` im Modulkopf (die zwei
  Archivwege der Huellen-Aufzaehlung) und dieselben zwei im Doc-Kommentar an
  `ohne_warten_oeffnen`.

**Kein Element ist `pub` geworden, und ein zweites `allow` steht nirgends im Baum.**

## Zugabe: zwei Defekte der Durchsicht vom 260826

- `260826-1221_*_zwei-verzweigungen-ueber-art-tragen-einen-auffangzweig-und-halten-den-bau-nicht-an.md`
  — geschlossen. `Auftrag::neuer_name` und `Auftrag::entpackziel`
  (`crates/krk-core/src/operation/auftrag.rs`) zaehlen die uebrigen fuenf Arten jetzt
  einzeln auf statt `_ => None`. Eine siebte Art haelt damit den Bau an.
- `260826-1225_*_die-merkliste-der-lesungen-begruendet-ihre-form-mit-einer-schranke-die-der-code-nicht-haelt.md`
  — geschlossen. Der Doc-Kommentar an `Lauf::staende`
  (`crates/krk-core/src/leseprofil/bausteine.rs`) nennt die Schranke, die der Code haelt.
  Kein Verhalten geaendert.

**Offen gelassen und warum.** Neun weitere Datensaetze mit Stempel `260826-12*` unter
`shared/issues/` liegen in `crates/krk-core/`. Zwei davon waren ausdruecklich ausgenommen,
weil ihre richtige Behebung einen Typ aendert, dessen Rufer in `krk-ui` steht
(`260826-1221_*_der-freie-name-gibt-nach-tausend-versuchen-einen-belegten-namen-heraus.md`,
`260826-1223_*_lesen-trennt-den-deskriptormangel-nicht-obwohl-beide-nachbarlesewege-es-tun-und-die-trennung-tragend-heisst.md`).
Die uebrigen sieben tragen je eine Gabelung, die nicht dem Bearbeiter gehoert: die
Kollisionspruefung im Stapelumbenennen waegt eigene Faltung gegen einen Systemaufruf je
Zeile ab und trifft dieselbe Wurzel wie ein Befund in `krk-ui`; die tiefe Suche ab Werk ist
nach dem Datensatz selbst „eine Nutzerfrage und keine Codeaenderung"; `COPYFILE_EXCL`, die
fuenf rufer­losen Namen, die zwei rufer­losen Ablagezugaenge, der Zehnerblock und die
Zustellerregel stellen je die Wahl zwischen Streichen und Behalten. Halb behoben waeren sie
schlechter als offen.

## Noch offen aus dem Entscheidungsdatensatz

Die Empfehlung des Datensatzes hat eine dritte Haelfte, die hier nicht umgesetzt ist:
`cargo doc` als fuenftes Kommando in `make check`. Das liegt ausserhalb des Umfangs dieser
Aufgabe (`Makefile`), und ohne es bleibt das Tor gruen, aber ungefahren. Ein eigener
Datensatz dafuer ist nicht angelegt: die Frage steht schon in `260905-2336` unter
`## Randbedingungen` und in der Empfehlung.

## Pruefung

| Kommando | Exit |
|---|---|
| `cargo test -p krk-core` | 0 |
| `cargo clippy -p krk-core --all-targets -- -D warnings` | 0 |
| `cargo fmt -p krk-core -- --check` | 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps` | 0 |

Das Doc-Kommando lieferte vor der Aenderung Exit 101 bei 53 Meldungen (49 mit dem Wortlaut
`links to private item`, 4 mit `unresolved link to`), danach Exit 0 bei null Meldungen.
`make check` ist auftragsgemaess nicht gefahren.
