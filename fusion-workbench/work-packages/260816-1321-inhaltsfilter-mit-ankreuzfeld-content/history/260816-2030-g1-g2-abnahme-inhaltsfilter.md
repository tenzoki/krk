# G1 und G2: der Baumanteil gefahren, die Abnahmeliste geschrieben

**Datum:** 2026-08-16
**Agent:** coder (schreibt auf; der Durchgang selbst ist Nutzerarbeit)
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Strang G, Schritte G1 und G2
**Spec:** `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md`, 57 Abnahmekriterien
**Baumstand:** `3dd799a` plus A1 `5c7f5b9`, C1 `4a54212`, A2 `7283d55`, B1 `32fd038`, D1 `09baffd`, Strang E `37ca972`, F1 `f7cf88b`, F2 `c8fd829`
**Erfüllt:** die probengestützte Hälfte aller sechs Fähigkeiten (G1); die Vorlage für die Bündelhälfte (G2)
**Nicht committet:** auf Ansage des Nutzers.

## G1 — `make check` über den fertigen Stand

`make check` — **exit 0**, „alle vier gruen". Alle vier Kommandos in einem Zug:
`cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`. Die
Kindproben unter `ulimit -n 64` sind als `ignored` geführt und vom Elternteil gefahren; die
Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist durchgelaufen.

## G1 — der Spec gegen den Baum

Die eigentliche Arbeit des Schritts. Gelesen sind alle 57 Kriterien einzeln gegen den Baum,
nicht gegen die `Erfüllt:`-Felder der Schritte, und für jedes ist der Beleg benannt: ein
Probenname mit Datei, ein Kommando am Diff, oder eine Beobachtung am Bündel.

| | insgesamt | nur Baum | Baum + Bündel | nur Bündel |
|---|---|---|---|---|
| C1 | 12 | 1 | 11 | 0 |
| C2 | 10 | 1 | 9 | 0 |
| C3 | 9 | 4 | 4 | 1 |
| C4 | 10 | 2 | 8 | 0 |
| C5 | 5 | 2 | 3 | 0 |
| C6 | 9 | 9 | 0 | 0 |
| Z | 2 | 1 | 0 | 1 |
| **Summe** | **57** | **20** | **35** | **2** |

**Kein Kriterium ohne Beleg.** 37 tragen einen Bündelanteil und sind Nutzerarbeit; zwei davon
haben am Baum überhaupt keinen Beleg und können keinen haben, C3.9 („die Anwendung bleibt
bedienbar") und Z1, das dieselbe Aussage in der Sprache der Zeitzusagen wiederholt.

**Der Spec dieser Runde trägt keine Spalte `(Probe)`/`(Bündel)`.** Der Plan sagt unter G2, er
führe beide schon; das trifft auf den Spec der Runde 10 zu und auf diesen nicht. Die
Kennzeichnung ist deshalb in der Abnahmeliste hergeleitet und dort als hergeleitet
ausgewiesen.

## Drei Befunde

**Erstens: C4.3 und C6.2 sind keinem Schritt zugewiesen, und beide halten.** Sie kommen in
keinem `Erfüllt:`-Feld der zwölf Planschritte und in keinem der elf Sitzungsprotokolle vor.
C4.3 (`Esc` beendet den Durchlauf) hält über `abbrechen` → `filter_leeren` →
`nach_filteraenderung` → `durchlauf_nachziehen_an`, das `durchlauf = None` als erste Zeile
setzt; belegt von `tabs.rs::ohne_seine_drei_bedingungen_beginnt_kein_durchlauf`. C6.2 (der
Filtertext wird einmal je Suche kleingeschrieben) hält über `Ordnermodell::filter_uebernehmen`
als der einen Entstehungsstelle; belegt von
`verzeichnis.rs::der_kleingeschriebene_filtertext_laeuft_mit` und am Diff. Datensatz:
`issues/260816-2020_o_zwei-abnahmekriterien-sind-keinem-schritt-des-plans-zugewiesen.md`.

**Zweitens: drei Prosastellen beschreiben einen von dieser Runde abgelösten Stand.**
`traegt_die_folge` hat seit A2 drei Rufer; `verzeichnis/filter.rs` sagt an vier Stellen zwei,
darunter im Bild des Modulkopfs. Dazu zwei Absätze in `verzeichnis/sys.rs`, die den zweiten
Aufrufer von `ohne_warten_oeffnen` noch außerhalb der Kiste verorten (seit A1 falsch) und für
`ist_deskriptormangel` einen statt zweier Frager nennen. **Kein Abnahmekriterium ist gebrochen**
— C6.1 und C6.3 halten, und die Zählprobe führt korrekt drei. Datensatz:
`issues/260816-2015_o_der-vergleich-hat-drei-rufer-und-die-prosa-an-seinem-ort-nennt-zwei.md`.

**Drittens: nichts repariert.** Der Auftrag sagt es ausdrücklich, und beide Befunde sind
Prosa- beziehungsweise Buchführungsbefunde ohne Wirkung auf das Verhalten der Anwendung.

## G2 — die Abnahmeliste

`messungen/260816-abnahme-inhaltsfilter.md`, neu. Der Leser ist der Nutzer.

**Der Prüfordner steht als Kommandoblock da und nicht als Beschreibung.** Er ist auf diesem
Gerät gefahren und läuft durch: elf Dateien im flachen Ordner (Namenstreffer, Inhaltstreffer,
Großschreibung, Umlaut, ungültiges UTF-8, 1.200.017 Bytes, zwei ohne Leserecht, benannte
Röhre, Verknüpfung, eine stumme für die Markierung), ein kleiner Unterbaum mit einem
Inhaltstreffer in der dritten Ebene, und ein großer Baum aus 20.000 Dateien, damit ein Lauf
lange genug läuft, um ihm zuzusehen. In den großen Baum werden die zwei Dateien kopiert, die
zusammen mit einer Markierung alle vier Satzteile der Statuszeile zugleich hinstellen — das
ist die Vorbereitung, die F2 verlangt hat.

**Zwei Eigenschaften stehen vor dem Durchgang**, damit sie niemand für Defekte hält:
`verweis.txt` steht bei ausgeschaltetem „Deep" immer (eine Verknüpfung zählt für die
Sichtbarkeit als Ordner), und `roehre` steht nie, sobald ein Filtertext steht.

**28 Beobachtungen an vier Orten**, 25 nummerierte und drei Farbbeobachtungen. Jede nennt, was
zu tun ist, was zu sehen sein muss und welche Kriterien sie deckt. Die Farbbeobachtungen stehen
getrennt, weil sie **zweimal** zu fahren sind, hell und dunkel, mit einem Tafelwechsel im
laufenden Betrieb als eigener Zeile.

**Sechs ungemessene Stellen sind offen benannt:** der Kontrast der Dämpfung gegen die blaue
Auswahlfläche (die gemessenen 3,95 : 1 und 5,89 : 1 gelten gegen den Listenhintergrund); ob
der Farbunterschied als Absetzung gelesen wird; die Dauer eines Inhaltsdurchlaufs; die Länge
der Abbruchspanne aus C4.7 (belegt ist **wo** geprüft wird, nicht wie lange es dauert); die
Dateiseite von C3.5 unter abgesenkter Deskriptorgrenze; und der Abnahmelauf der zehn
Zeitzusagen, der seit dem 260810 nicht gefahren ist, während diese Runde L1 und L6 berührt.

**Z2 ist am Diff abgenommen:** `grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u`
liefert dieselben zehn Zahlen wie vorher, und `crates/krk-bench/` ist von keinem der acht
Commits angefasst.

## Am Plan nachgezogen

G1 und G2 auf `[DONE]`. Damit stehen **alle zwölf Schritte** auf `[DONE]`, und der Kopf des
Plans trägt `**Status:** Complete`. Die Umbenennung der Plandatei auf `_c_` macht der Nutzer.

## Was Nutzerarbeit bleibt

Der ganze Durchgang. 37 der 57 Kriterien tragen einen Bündelanteil, und kein Agent kann ihn
fahren: aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden fokusgebundenen
Befehl ab. Ob die Kriterien halten, sagt der Lauf.
