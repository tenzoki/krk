# Abgleich zum Aufräumlauf — 260829-1252

**Agent:** reconciler
**Domain:** code
**Anlass:** `/fusion:cleanup` nach der Sitzung, die die Runden 20, 22 und 21 je kohärent geschlossen und zwei Auslieferungen gefahren hat (`d2824c5` 1.3.0, `b9d9cbc` 1.4.0). Kein Circle aktiv; Ausgabe nach `shared/`.

## Umfang und Schranke

Geprüft ist `a5c7a46..b9d9cbc` (39 Quelldateien, +6673/−632 außerhalb der Werkbank). `fusion-cadence-anchor changed-files last_reconcile_commit` hat geantwortet (Code 0): unter `shared/` nennt es allein vier Rückstandseinträge, fünf Playmaker-Berichte, zwei schon geschlossene Defekte (`260826-1302_c_`, `260826-1423_c_`) und die Memos. Die sechs Specs und Pläne unter `shared/planning/`, alle 27 Entscheidungen und alle übrigen Defekte sind seit dem letzten Abgleich (`260827-1532`, davor `260826-2205`) nicht angefasst; gelesen sind sie trotzdem, wo eine der vier Runden ihren Gegenstand berührt (Belege unten). `cargo test --workspace` am Arbeitsbaum: alle Ziele grün (u. a. 860 Proben in `krk-core/tests`, 217 im Kern, 155 in `krk-ui`), 0 Fehlschläge. Die Turn-Zählung ist **unavailable**: `fusion-events turns` bricht ohne `agentstate.yaml` mit Code 3 ab; die Datei ist nach dem Sitzungsende regulär gelöscht. Der Arbeitsbaum war vor diesem Lauf sauber (`git status --short` leer).

Die drei in der Sitzung geschlossenen Circles sind nicht neu abgeglichen (je eigener Abschluss-Abgleich unter `history/`: `260828-1044`, `260829-0734`, `260829-1223`); gelesen sind Kopffelder und Marker.

## Pläne und Specs (6 gelesen, 5 mit Vermerk)

| Datei | Befund | Marker |
|---|---|---|
| `planning/260813-0053_o_spec-suche-in-der-belegung-…` | Berührt: Runden 21/22 geben `paste:`, `copy:`, `cut:` im Dateifenster einen Antwortenden (`anwendung.rs:988-989`, `3722c89`, `1644ada`); Kriterien 8 und 18 halten im Wortlaut, der Satz „keine Zulässigkeitsregel" ist für die Dateiablage überholt (`dfde98c`). Erster `## Reconciliation Log` angelegt. | `_o_` bleibt |
| `planning/260816-1310_o_spec-inhaltsfilter-…` | Berührt: Runde 21 macht aus dem einen Vergleich ein `Muster` mit `*` (`filter.rs:190`, `inhalt.rs`); C1.4 gilt für Filtertexte ohne `*`. Vermerk. | `_o_` bleibt |
| `planning/260816-2240_o_spec-befehle-absetzen-…` | Nicht berührt; Circle `_d_`. Kein Vermerk. | `_o_` bleibt |
| `planning/260819-2216_p_spec-auswahl-und-kopieren-…` | Berührt: die dritte Fläche der Runde 20 kopiert Seitentext über `PDFView` und nicht Quelltext über die Hülle (`betrachter.rs:61-67`); die Zusage gilt weiter der Textvorschau. Vermerk. | `_p_` bleibt |
| `planning/260821-1115_o_spec-artefakt-und-release.md` | Zwei Auslieferungen durch die unveränderte Kette (`xtask/` ohne Diff). Vermerk. | `_o_` bleibt |
| `planning/260825-1725_p_plan-vorschau-vertieft-…` | Runden 19/20 setzen auf Schritt 7 auf; `nach_lesebeginn`/`tab_gewechselt`/`auswahl_merken` ohne Diff; neun Schließungsbedingungen unverändert. Vermerk. | `_p_` bleibt |

## Defekte (201 gelesen nach Titel, 21 gegen den Baum, 4 geschlossen, 4 mit Vermerk)

**Geschlossen `_o_`→`_c_` (4)** — alle vier `CLAUDE.md`-Defekte, deren Berichtigung schon im Baum stand und die der Kuratorenlauf `260826-1637` ausdrücklich an den Reconciler überwiesen hatte; der Abgleich `260826-2205` hat sie nicht bewegt:
- `260825-1859_c_claude-md-nennt-fuer-zip-das-eine-merkmal-…` — `fb50fcd`, `CLAUDE.md:83` nennt `deflate-flate2` und `unreserved`.
- `260826-1306_c_claude-md-nennt-cargo-test-als-zweiten-greifer-…` — `69dfa19`, `CLAUDE.md:129`.
- `260826-0923_c_die-ablage-aufzaehlung-steht-neben-dem-zeiger-…` — `69dfa19`, `CLAUDE.md:123` trägt nur den Zeiger auf `Datei::ALLE`.
- `260826-0923_c_die-pfadregel-nach-der-rundentabelle-…` — `69dfa19`, `CLAUDE.md:34` Langform-Regel.

**Mit Vermerk, weiter offen (4):**
- `260826-1420_o_zwei-probenkoepfe-in-statuszeile-rs-…` — `Rang::ALLE` trägt seit der Runde 20 sieben (`statuszeile.rs:275`); ein Kopf sagt weiter „fuenf" (`:1694`), der andere jetzt „sechs" (`:1598`, `:1651`), beides falsch.
- `260826-1327_o_drei-prosastellen-in-tabelle-rs-zaehlen-fuenf-raenge-…` — `tabelle.rs:864` unverändert, Abstand zu sieben gewachsen.
- `260826-0149_o_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-…` — Tabellenzeile steht, Absatz zum `readers.toml`-Handgriff fehlt weiter. Curator.
- `260826-1223_o_der-grund-fuer-den-ausschluss-des-zehnerblocks-…` — `parser.rs:283-290` begründet seit der Runde 20 zwei Ausnahmen, nicht den Ausschluss; Befund bleibt.

**Gegen den Baum gelesen und unverändert offen (13):** `260825-1922_o_` (beide; die drei Funktionen in `tabelle.rs` ohne Diff), `260826-1418_o_der-modulkopf-von-menue-rs-…` (`menue.rs:124` sagt weiter „kehrt sofort zurueck", obwohl die Runde 22 den Modulkopf angefasst hat), `260826-1418_o_der-zeichenzweig-…` (`anwendung.rs:3077` setzt die Frage weiter von Hand zusammen, trotz zweitem Eingang der Regel), `260826-1420_o_der-modulkopf-von-fokus-rs-…` (`fokus.rs:34` „rund fuenfzig"), `260826-1327_o_zwei-statuszeilentexte-…` (`tabelle.rs:2516` „traegt"), `260826-1303_o_die-probe-zur-tiefen-suche-…` und `260826-1221_o_die-tiefe-suche-ab-werk-…` (`modell.rs:375` `tief: true`), `260826-1225_o_juengste-mit-anzahl-null-…` (`datei.rs:588-592` ohne untere Schranke), `260826-1417_o_sechs-der-zwoelf-kommandos-module-…` (`zulaessig` `:223` ohne `#[must_use]`), `260816-2144_o_die-leertaste-…` (`default-keymap.toml:349`), `260816-1803_o_der-kommentar-zu-deep-…` (`:464`), `260826-1221_o_der-modulkopf-des-ordnermodells-…` (`modell.rs:15`), `260823-1433_o_kommando-ausfuehren-…` (sieben Stellen „immer `true`" in `anwendung.rs`, eine in `rundweg.rs:121`).

**Nicht bewegt, gehört einem Circle:** `circles/260823-2208-…/issues/260824-1852_o_zwei-aussagen-in-claude-md-…` — vom Kuratorenlauf `260826-1637` ebenfalls als gegenstandslos gemeldet; außerhalb des Schreibbereichs dieses Laufs (`$OUT_ISSUE` = `shared/issues`).

## Entscheidungen (27 gelesen, 0 bewegt, 1 mit Vermerk)

- Die vier `_a_` bleiben `_a_`: drei tragen eine Abwesenheit als Antwort (`260816-1310_a_`, `260819-2216_a_` ×2; Lage `shared/issues/260820-2056_o_drei-beantwortete-datensaetze-…`), und `260821-2202_a_` (Web-Betrachter abgesagt) ebenso. Keine der vier Runden hat eine davon in Code umgesetzt.
- `260826-1223_o_loesen-die-zifferntasten-des-zehnerblocks-…`: Vermerk — seit der Runde 20 lösen `+` und `−` des Blocks über das Zeichen `cmd+plus`/`cmd+minus` aus (`parser.rs:362-365`); zwei der vier Rechenzeichen sind damit angeschlossen, die Frage bleibt offen.
- Alle übrigen `_o_` unbeantwortet; die Specs der Runden 19–22 nennen keinen davon als beantwortet (die drei Circle-Abgleiche haben je 24–25 aktive Datensätze gegen ihre Directive gehalten, 0 Konflikte).

## Rückstand (6 gelesen, 0 bewegt — Nutzertor)

- Vier `_c_` mit `Promoted:`-Zeile, die je den richtigen Circle nennt (`260827-1925`, `260828-0909`, `260828-2345`, `260829-0842`) — korrekt.
- Zwei `_o_`, die gebaut sind und die der Playmaker in jedem Bericht seit `260827-0403` zur Schließung vorschlägt, zuletzt `history/260829-1227-playmaker-orchestrator-phase4.md:43-44`: `260813-2033_o_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md` (`cmd+e` öffnet seit dem 260823 im Dateifenster den ausgewählten Eintrag) und `260823-2136_o_readerconventions-profile-fuer-dateizugriff.md` (Runde 16 hat die Leseprofile gebaut). **Nicht geschlossen** — das ist die Bestätigung bei `/fusion:next`.

## Circle-Datensätze (Kopffelder der drei frisch geschlossenen, 1 Befund)

- `circles/260827-2028-vorschau-rendert-pdf-als-betrachter/_c_circle.md:7`: `**Active spec/plan:**` nennt `planning/260828-0712_p_plan-…`, die Datei trägt `_c_` — toter Zeiger im Kopffeld. Die Runden 19, 21 und 22 nennen `_*_` bzw. `_c_` und treffen. Nicht berichtigt (Circle-Datensätze sind nicht Schreibbereich dieses Laufs); Kandidat für `/fusion:next` oder den nächsten Playmaker-Lauf.
- `.active-circle` fehlt, kein `_t_` — konsistent.

## Gemeldet für den Curator (CLAUDE.md nicht geändert)

- `CLAUDE.md` „Projektstand": „`Wirkungsbereich` … trägt sieben Werte" — der Baum trägt seit `2aee690` acht (`Wirkungsbereich::Vorschau`); Datensatz `circles/260827-2028-…/issues/260828-1046_o_claude-md-nennt-sieben-werte-…`.
- `CLAUDE.md` „Was man nicht sieht", Absatz zum Dateifilter: nennt die Zählprobe mit altem Namen und den Vergleich als Teilzeichenfolge; seit der Runde 21 ist er ein Muster mit `*` (`circles/260828-1041-…/issues/260829-1217_o_…`).
- `CLAUDE.md` Rundentabelle endet bei 18; die Runden 19–22 sind kohärent geschlossen und fehlen (Zeilen und ggf. der Absatz zu `cmd+c`/`cmd+x`/`cmd+v` im Dateifenster, zur dritten Vorschaufläche und zum Default-Profil).
- `CLAUDE.md` Kurzabsatz zur Zwischenablage: „ist seit der Runde 4 auch Ziel" — seit der Runde 22 schreibt die Hülle auch Dateiverweise (`3764fb6`), seit 21 liest sie für den Filter (`1b0939a`).
- Offen bleibt `260826-0149_o_` (Absatz zum `readers.toml`-Handgriff).

## Gemeldet, nicht berichtigt

- `shared/history/260826-2245-orchestrator-session.md` steht auf `**Status:** Laufend` mit `**Directive:** noch nicht gesetzt`; die Sitzung ist beendet. Fremde Statuszeilen schreibt der Abgleich nicht um. Die Coherence dieses Laufs ist dort angehängt, weil `$SCAN_HISTORY` allein `shared/history` nennt und es die jüngste Sitzungsdatei dort ist.

## Coherence

Urteil **coherent**, Empfehlung **none**; die drei Kanten stehen im Abschnitt `## Coherence` von `shared/history/260826-2245-orchestrator-session.md`.

## Neue Datensätze

Keine. Nichts gefunden, das nicht schon einen Datensatz trägt.
