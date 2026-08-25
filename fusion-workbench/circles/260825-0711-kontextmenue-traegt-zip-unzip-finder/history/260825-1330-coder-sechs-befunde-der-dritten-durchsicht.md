# Die sechs Befunde der dritten Durchsicht, dazu ein Modulkopf aus einem älteren Datensatz

**Status:** Complete
**Agent:** coder
**Datum:** 260825-1330
**Baumstand bei Beginn:** `f464bc5`

## Auftrag

Alle sechs Befunde der dritten Durchsicht der Runde 17 beheben
(`reviews/260825-1249-coderev-runde-17-dritte-durchsicht-das-ziel-eines-laufs-und-seine-quellen.md`,
die Datensätze daneben unter `issues/` mit dem Stempel `260825-1249`), dazu die Codehälfte
von `shared/issues/260825-0727_*_claude-md-nennt-zwei-aufrufer-von-ohne-warten-oeffnen-…`.
Der Nutzer hat für Befund 1 ausdrücklich Möglichkeit 1 gewählt und verlangt, dass alle
sechs vor dem Rundenabschluss fallen.

## Geändert

Fünf Codedateien. Keine Ontologie, keine Belegung, kein `Cargo.toml`.

1. `crates/krk-ui/src/kommandos/kontextmenue.rs`
2. `crates/krk-ui/src/appkit/anwendung.rs`
3. `crates/krk-ui/src/kommandos/operationen.rs`
4. `crates/krk-core/src/operation/zippen.rs`
5. `crates/krk-core/src/verzeichnis/sys.rs`

## Was je Befund geschehen ist

**B10 — der Schnitt faltet jetzt.** `ist_ziel_des_laufs` vergleicht über den neuen privaten
Helfer `gleicher_eintrag`: letzter Bestandteil mit `eq_ignore_ascii_case`, Elternteil
buchstabengetreu, kein Dateizugriff. Beide Rufer erben das, weil die Regel weiterhin genau
einmal dasteht. Die Ungenauigkeit auf einem groß-/kleinschreibungsempfindlichen Datenträger
steht im Doc-Kommentar ausgeschrieben, wie der Nutzer es verlangt hat. Möglichkeiten 2 und 3
verworfen.

**B11 — der Entpackschnitt ist ein Festpunkt.** `ohne_die_eigenen_ziele` entscheidet vom
längsten Archivpfad zum kürzesten und schreibt die Zielliste dabei fort; aufgenommen wird
allein das Ziel eines bleibenden Paares. Aus `{a.zip, a.zip.zip, a.zip.zip.zip}` bleiben
zwei Paare. Die Reihenfolge folgt aus `paar` und ist keine Annahme: ein Zielname ist der um
`ENDUNG` gekürzte Archivname, und wo er das nicht ist, trägt er `ERSATZSTAMM`, der nicht auf
`.zip` endet und deshalb kein Archiv dieser Liste treffen kann. Ausgegeben wird weiterhin in
der Reihenfolge der Eingabe.

**B12 — der Schnitt meldet sich.** `operationen::abschlusstext` nimmt ein viertes Argument
`ausgelassen` und hängt den Halbsatz an, hinter dem zu den übersprungenen Einträgen. Die
Zahl reist als `Vorgang::ausgelassen` durch `auftrag_starten`; die vier übrigen Wege hinein
reichen null herein. Beim Packen rechnet der Aufrufer sie als
`auswahl.pfade.len() - quellen.len()`, beim Entpacken trägt sie
`Entpackbefund::Archive { paare, ausgelassen }` mit, weil `entpackziel` zwischen zwei Regeln
entscheidet.

**B13 — die Zusage hängt nicht mehr an einer ungelesenen Zeile.** Zwei Proben in
`kontextproben`: `der_packauftrag_reicht_die_quellen_aus_packziel_weiter` hält die Kette im
bestehenden Zweig als Paarung, `ein_packauftrag_entsteht_in_der_oberflaeche_genau_einmal`
fängt den zweiten Eingang. Der Modulkopf von `zippen.rs` nennt beide beim Namen.

**B14 — der Entpackschneider hat seine eigene Probe.**
`ein_einzelnes_archiv_behaelt_seinen_zielordner`, und der Doc-Kommentar der Packprobe sagt
jetzt, dass sie nur den Packschneider prüft.

**B15 — fünf Zitate stehen wieder auf einer Zeile.** Die drei neuen und die zwei älteren aus
frühen Commits derselben Runde, denen der Befund ausdrücklich mitgilt.

**`sys.rs` — drei Stellen zählten daneben, eine mehr als der Datensatz nannte.** Die
Aufzählung nach dem Zählkommando, die ASCII-Skizze am Kopf (sie führte vier Aufrufer unter
`fcntl(2)`) und der Doc-Kommentar von `ohne_warten_oeffnen` selbst, dort an zwei Stellen.

## Wo ein Datensatz im Baum nicht hielt

**Die erste Fassung der neuen Entpackprobe war blind, und die Gegenprobe hat es gezeigt.**
Der Datensatz zu B14 schlägt "ein Archiv markiert, `entpackziel` gerufen, das Paar steht"
vor, fünf Zeilen. So gebaut bleibt die Probe auch bei einem zu weiten Schnitt grün: die
Markierung fällt leer aus, und die Ersatzregel liefert dasselbe eine Paar zurück. Behoben
durch ein zweites Archiv im Ordnermodell, damit die Ersatzregel `Mehrere` antwortet; die
Begründung steht im Doc-Kommentar der Probe.

**Der Datensatz zu B10 nennt `eq_ignore_ascii_case` an einer Stelle als "die falsche
Antwort".** Das gilt seiner eigenen Abwägung nach und ist mit der Nutzerwahl für
Möglichkeit 1 überholt; der Absatz "Warum das keine Zeile nebenbei ist" bleibt als
Aufzeichnung des damaligen Standes stehen.

**Die Zeilenangabe `operation/zippen.rs:348` im Zusatz vom 260825-1230 stimmt nicht mehr**;
der Aufrufer steht jetzt auf 362, verschoben durch die Prosa, die diese Sitzung dem Modulkopf
hinzugefügt hat. Im Schlussvermerk des Datensatzes benannt.

## Gefahrene Gegenproben

Für jede Probe, die einen Defekt festhält, ist die Behebung versuchsweise zurückgenommen
worden; danach war der Baum wiederhergestellt.

| Zurückgenommen | Rot geworden |
|---|---|
| die Faltung in `gleicher_eintrag` | die zwei Schreibungsproben; `ein_aehnlich_benanntes_archiv_bleibt_quelle` blieb grün |
| der Festpunkt in `ohne_die_eigenen_ziele` | `aus_einer_kette_von_drei_archiven_bleiben_zwei` |
| ein Archiv gilt als sein eigenes Ziel | `ein_einzelnes_archiv_behaelt_seinen_zielordner` und fünf weitere |
| `Auftrag::zippen(auswahl.pfade, ziel)` statt `quellen` | `der_packauftrag_reicht_die_quellen_aus_packziel_weiter` |
| ein zweiter `Auftrag::zippen`-Aufruf in `operationen.rs` | `ein_packauftrag_entsteht_in_der_oberflaeche_genau_einmal` |
| der Halbsatz im `abschlusstext` | `der_abschlusstext_nennt_die_ausgelassenen_eintraege` |
| die Entpackzahl auf null gerechnet | die drei Entpackproben mit `ausgelassen`-Feld |

## Datensätze

Alle sechs `260825-1249`-Datensätze mit `Resolved:`-Notiz versehen und auf `_c_` umbenannt.
`shared/issues/260825-0727_*` ebenfalls, denn die offene Codehälfte ist damit erledigt.

## Abnahme

`make check` (Bau, Proben, Clippy unter `-D warnings`, `cargo fmt --all --check`) — Exit 0.
Der Abnahmelauf am gebauten Bündel steht aus und ist Nutzerarbeit; KRK ist nicht gestartet
worden.
