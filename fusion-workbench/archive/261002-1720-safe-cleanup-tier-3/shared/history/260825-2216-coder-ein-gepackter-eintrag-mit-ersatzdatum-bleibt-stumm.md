# Ein gepackter Eintrag mit Ersatzdatum bleibt stumm

**Agent:** coder
**Datum:** 2026-08-25, 22:05 bis 22:16
**Aufgabe:** R-2, Runde 2 der Sitzung zur Runde 18 — Befund M1 der Durchsicht
`fusion-workbench/shared/reviews/260825-2127-coderev-runde-18-vorschau-vertieft-und-zwei-fehler.md`,
Datensatz `shared/issues/260825-2127_c_ein-gepackter-eintrag-mit-ersatzdatum-steht-in-der-liste-der-uebersprungenen.md`
**Status:** Complete

## Was entstanden ist

`crates/krk-core/src/operation/zippen.rs`: `zeit_uebernehmen` meldet nichts mehr. Die drei
Rufe von `Steuerung::ueberspringen` — unlesbares Änderungsdatum, Zeitpunkt außerhalb 1980
bis 2107, Zusatzfeld nicht angehängt — sind gefallen; der Eintrag trägt in allen drei Lagen
das Vorgabedatum und steht vollständig im Archiv, wie vorher auch. Der Parameter `steuerung`
ist aus `zeit_uebernehmen` und aus den drei Wahlbauern `dateiwahl`, `ordnerwahl` und
`verknuepfungswahl` gefallen, die ihn nur durchreichten; `dateiwahl` hat damit auch den
`pfad` verloren, den es allein für die Meldung brauchte. Der Fehlschlag von `add_extra_data`
steht als `let _ =` mit dem Grund daneben, nach der Regel dieses Projekts: „ich brauche den
Wert nicht".

Die Begründung steht **einmal**, am Doc-Kommentar von `zeit_uebernehmen`: die Abschlussliste
ist die Liste der nicht bearbeiteten Einträge, das Blatt schreibt sie so aus, und eine Datei,
die im Archiv liegt, dort zu nennen, wäre die falsche Auskunft. Der Kommentar nennt
`super::entpacken` als das andere Ende mit derselben Wahl und den Datensatz als den Stand
davor. Der Modulkopf (Abschnitt „Jeder Eintrag traegt das Aenderungsdatum seiner Quelle")
verweist auf die Funktion statt die Regel ein zweites Mal zu tragen.

`entpacken.rs` ist nicht angefasst. Sein Inline-Kommentar an `zeit_setzen` trägt dieselbe
Aussage in seinen Worten; eine gemeinsame Stelle für beide Enden hätte einen zweiten Baustein
gebraucht, für einen Satz, den beide Dateien nun je einmal in ihrem eigenen Zusammenhang
sagen und aufeinander verweisen. Das ist die kleinere Änderung, und sie hält die Sperre der
Aufgabe ein.

`crates/krk-core/tests/operation.rs`: die Probe
`ein_zeitpunkt_vor_1980_faellt_auf_das_vorgabedatum_und_erzeugt_eine_zeile` heißt jetzt
`…_und_bleibt_aus_der_abschlussliste`. Sie prüft `bericht.uebersprungen.is_empty()` und
behält die drei übrigen Zusagen: Inhalt im Archiv, MS-DOS-Feld auf 1980-01-01, erweitertes
Zeitfeld auf Sekunde 0.

## Gegenprobe

Die neue Probe ist zuerst gegen den alten Code gefahren worden und war rot:
`die Datei liegt im Archiv und steht trotzdem in der Liste der uebersprungenen: [Uebersprungen
{ …/alt.txt, grund: "das Aenderungsdatum liegt ausserhalb dessen, was ein Zip-Eintrag fassen
kann; der Eintrag traegt das Vorgabedatum" }]`. Nach dem Umbau grün, `cargo test -p krk-core
--test operation`: 61 von 61.

## Was nicht getan wurde

Kein eigener Meldeweg (Weg 2 des Datensatzes) und keine Weitung von `Uebersprungen` (Weg 3).
Die Aufgabe hat Weg 1 vorgegeben, und `Steuerung` trägt keine Meldungsart für „gepackt, aber
mit Ersatzdatum"; eine neue anzulegen wäre die dritte Form gewesen, die die Aufgabe
ausschließt. Ob der Nutzer das Ersatzdatum überhaupt erfahren soll, ist damit nicht
entschieden, sondern so beantwortet wie am Entpacken: über das Datum in der Dateiliste.

## Abnahme

`make check` — exit 2. Der Lauf bricht in `cargo test --workspace` an
`ein_elfter_unterspeicher_kostet_einen_elften_leselauf` (`crates/krk-core/tests/leseprofil.rs:3166`)
ab; diese Probe steht nicht in HEAD (`5595026`), sondern im ungesicherten Stand der parallel
laufenden Aufgabe an `leseprofil.rs` und `resources/default-readers.toml`, beide fuer diese
Aufgabe gesperrt. Clippy und `fmt --check` sind in `make check` gar nicht mehr drangekommen und
deshalb einzeln gefahren: `cargo clippy -p krk-core --all-targets` ohne Warnung,
`cargo fmt -p krk-core --check` exit 0, `cargo test -p krk-core --test operation` 61 von 61.
