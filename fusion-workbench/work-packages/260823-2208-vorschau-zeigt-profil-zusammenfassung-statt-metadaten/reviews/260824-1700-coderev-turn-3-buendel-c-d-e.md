# Durchsicht: Bündel C, D und E — die Auslieferungsfassung, die Anzeige und die abzählbaren Grenzen

**Reviewed-range:** `abe1a31..f9e34e7`
**Not-opened:** `resources/default-readers.toml`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1234-analyst-raeumung-der-spec-und-planbuchfuehrung.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1246-coder-raeumung-der-code-befunde-aus-zwei-durchsichten.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1313-ontocoder-auslieferungsfassung-der-leseprofile.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1457-coder-die-drei-fehlenden-probenpflichten-aus-schritt-8.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1538-analyst-zwei-antworten-in-spec-und-plan.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1559-coder-der-anzeigezweig-und-der-weg-der-profile.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1612-coder-der-siebte-inhalt-und-die-profile-am-arbeitsfaden.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1650-ontocoder-vierte-zustandszeile-und-drei-speichernamen.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1755-coder-die-anwendung-laedt-die-profile-und-uebergibt-sie.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1902-coder-die-zaehlproben-zu-c6.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0600_a_der-titel-aus-der-ueberschriftenzeile-erreicht-keinen-einzigen-defektdatensatz.md`

**Datum:** 2026-08-24, 17:00
**Absender:** `coderev`
**Gegenstand:** die elf Commits `615190a` bis `f9e34e7`, Planschritte 7, 8, 9, 10, 11, 12 und 14
**Maßstab:** Spec `planning/260824-0613_o_spec-…` in der **berichtigten** Fassung (C3.8, C3.14,
C4.3, C6.1 vom 260824-1224; C5.2, C5.3, C5.6 und A7 vom 260824-1505), Plan
`planning/260824-0640_o_plan-…`, `CLAUDE.md`

Die vorige Durchsicht (`reviews/260824-1220-coderev-turn-2-…`) trug `**Not-opened:** none`, es
war also keine Dateiliste zu übernehmen. Die Bereiche stoßen aneinander: jene endete auf
`abe1a31`, dieser beginnt dort.

**Zur Not-opened-Liste.** `resources/default-readers.toml` ist der Gegenstand einer parallelen
`ontorev`-Durchsicht und war laut Auftrag auszulassen; gelesen sind daraus allein die Stellen,
über die Code eine Aussage macht — die Zahl der Kommentarzeilen und die vier Zustandszeilen des
Rundenprofils. Die zwölf Verlaufsdatensätze und die zwei `_a_`-Entscheidungen sind
Buchführung des Bauens und tragen keine Codeaussage; die zwei `_i_`-Entscheidungen, der Spec,
der Plan, die sechzehn Defektdatensätze des Circles und die Durchsicht aus Turn 2 sind gelesen.

---

## Summary

Die Bündel C, D und E sind sauber gebaut. `cargo build`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` laufen am 260824-1640
grün, kein Testziel meldet einen Fehlschlag, und clippy gibt keine einzige Warnung aus. **Kein
Fehlverhalten im ausgelieferten Programm gefunden, kein Auslieferungshindernis, und kein
Planschritt ist aufzuhalten.**

Sieben Befunde sind neu. Zwei davon sind mittelschwer: eine Probe, die den einen Zweig, für den
sie geschrieben ist, an keiner Stelle erreicht, und ein freigegebenes Abnahmekriterium, dem der
Bau widerspricht, ohne dass die Abweichung irgendwo steht. Die übrigen fünf sind Zahlen und
Verweise in Doc-Kommentaren, die nicht mehr stimmen.

Die vier ausdrücklich gestellten Prüfaufträge sind einzeln beantwortet, drei davon durch
Messung und nicht durch Lesen; die Antworten stehen unten unter `## Die vier Prüfaufträge`.

---

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 2 |
| Niedrig | 5 |

Alle sieben liegen als eigene Datensätze unter `issues/` dieses Circles.

---

## Die vier Prüfaufträge

### 1. Der vierte Fall einer Probe, die mehr behauptet als sie misst

**Gefunden.** `ein_abgeschnittenes_zeichen_am_ende_nimmt_der_datei_nicht_ihren_text`
(`crates/krk-core/src/leseprofil/bausteine.rs:549-568`) trägt den Kommentar „Der Deckel faellt
mitten in das zweibytige „ü"" und schneidet in Wahrheit hinter einem ganzen Zeichen.

`"Überschrift"` ist zwölf Bytes lang, `C3 9C` für das `Ü` und zehn ASCII-Bytes danach.
`&ganz[..ganz.len() - 1]` nimmt das abschließende `t` weg:

```text
b'\xc3\x9cberschrift'  ->  12 Bytes
b'\xc3\x9cberschrif'   ->  gültiges UTF-8, "Überschrif"
```

Damit läuft `lesbarer_anfang` im ersten Zweig, und der zweite —

```rust
Err(fehler) if fehler.error_len().is_none() => {
    std::str::from_utf8(&bytes[..fehler.valid_up_to()]).ok()
}
```

— wird von keiner Probe des Baums erreicht. Er ist der Grund, aus dem die Funktion existiert;
der Doc-Kommentar darüber schreibt ihn über neun Zeilen aus. Die Funktion ist privat, also kann
ihn allein dieses Prüfmodul erreichen. Datensatz
`issues/260824-1650_o_die-probe-zur-naht-des-deckels-…`.

### 2. Die Deskriptorprobe zu C6.9 misst wirklich

**Die Behauptung des `coder` stimmt, nachgemessen und nicht übernommen.** Gefahren am
260824-1642 am gebauten Testziel:

```text
ohne ulimit:   panicked ... "das Kind bekommt 96 Deskriptoren; die Grenze 24 hat
               nicht gegriffen, und die Probe messte nichts"       -> FAILED
mit ulimit 24: test kind_fasst_mit_einem_freien_deskriptor_zusammen ... ok
Elternprobe:   eine_zusammenfassung_haelt_nie_mehr_als_einen_deskriptor_zugleich ... ok
```

Die Grenze wird von `kind_mit_deskriptorgrenze` (`crates/krk-core/tests/gemeinsam/mod.rs:334-351`)
über `/bin/sh -c "ulimit -n 24 && exec …"` gesetzt und nicht behauptet; die Zusicherung
`vorrat < 4 * GRENZE_DESKRIPTOREN` (`tests/leseprofil.rs:2413-2417`) fängt eine wirkungslose
Absenkung, weil die Schleife darüber bei genau `4 * GRENZE_DESKRIPTOREN` abbricht. Die Probe
misst zudem **strenger** als C6.9 verlangt: sie lässt genau **einen** freien Deskriptor stehen,
während C6.9 einen Verzeichnis- **und** einen Dateideskriptor zugleich erlaubte.

**Eine Blindheit bleibt und ist die des Hauses:** fehlt die Umgebungsvariable, kehrt die
Kindprobe still zurück und gilt als bestanden. Dieselbe Form tragen die vier Kindproben der
Runde 10 in `tests/verzeichnis.rs` und `tests/umfang.rs`; sie ist kein Befund dieser Runde und
kein Datensatz.

### 3. Die Zahlen zu C6.7 sind gegen die eingebettete Fassung gemessen

**Ja, und der `assert_eq!` hält, was seine Prosa verspricht.** `ausgelieferte()`
(`crates/krk-core/tests/leseprofil.rs:1540-1552`) zerlegt
`krk_core::ablage::leseprofile::AUSLIEFERUNGSTEXT`, also den Text, den `include_str!` aus
`resources/default-readers.toml` einkompiliert, und hält vorher fest, dass er ohne Meldung
durch `datei::pruefen` geht und fünf Profile trägt. Ein Nachbau im Quelltext der Probe kommt an
keiner Stelle vor.

Die Zusicherungen sind **beidseitig** und nicht bloß obere Schranken:

```rust
assert_eq!((haushalt.leselaeufe(), haushalt.oeffnungen()), (5, 11), …);   // eine Runde
assert_eq!((haushalt.leselaeufe(), haushalt.oeffnungen()), (3, 5), …);    // die Wurzel
```

Der `<= 7 && <= 11` daneben ist die Zusage aus C6.7 selbst und steht neben der Gleichheit, nicht
an ihrer Stelle. Beide Fälle prüfen zusätzlich die **Beschriftungsliste**, also welches Profil
gegriffen hat, und die gezogenen Werte; ein Lauf, der nichts findet, käme ebenfalls billig
davon und würde hier rot. Die Rechnung stimmt am Code nach: das Rundenprofil liest den
erkannten Ordner (Erkennung über `kennzeichen`), `planning` zweimal (zwei Vorhandensein mit
`ordner`), `decisions` und `history` — fünf; es öffnet den Circle-Datensatz und zehn
Verlaufsdateien — elf. Die vierte Zustandszeile aus Schritt 14 trägt `muster` ohne `ordner` und
kostet deshalb nichts, wie die Berichtigung zu C5.6 es zusagt.

### 4. `sitzung_laden` liest in einem Durchgang, und die Aussage über den Messmodus stimmt

**Beides bestätigt.** `crates/krk-ui/src/appkit/anwendung.rs:1536-1562` trägt genau einen
`ablage.durchgang(|zugang| …)`, und darin stehen Sitzung, Einstellungen und Leseprofile
nebeneinander; ein zweiter Durchgang entsteht nicht. Die Zählprobe
`die_leseprofile_werden_im_baum_genau_einmal_geladen`
(`crates/krk-ui/src/appkit/anwendung.rs:8710-8735`) hält das über den ganzen Quellbaum und
benennt ihre eine Blindheit (die Weitergabe als Wert in `tests/ablage.rs`) selbst.

Der Messmodus: `sitzung_laden` verzweigt über `ivars.messaufgabe`, und **alle vier Aufgaben
kehren mit `return` zurück**, bevor der Durchgang läuft — `Aufgabe::Start` und
`Aufgabe::Spannen` in einem Zweig (`:1449-1451`), `Aufgabe::Sitzung` (`:1452-1462`),
`Aufgabe::SitzungsStart` (`:1463-1495`). `AnwendungsIvars::profile` bleibt damit auf
`Arc::default()` stehen, und `profile_setzen` überträgt einen leeren Satz. Die Reihenfolge in
`oberflaeche_aufbauen` stimmt: `sitzung_laden()` in Zeile 1097, `profile_setzen` in Zeile 1215.

---

## Befunde nach Thema

### Thema 1: eine Probe, die ihren Zweig nicht erreicht

**Befund 1.1 — die Naht des Deckels wird an keiner Stelle geschnitten.** Schwere: mittel.
Datensatz `issues/260824-1650_o_die-probe-zur-naht-des-deckels-schneidet-hinter-einem-ganzen-zeichen-und-misst-den-zweig-nicht.md`.
Ausgeschrieben oben unter Prüfauftrag 1.

Der Befund ist der vierte seiner Art in dieser Runde, und alle vier folgen demselben Muster:
die Zusicherung ist richtig formuliert, aber die Eingabe daneben erreicht den Fall nicht, den
sie beschreibt — zwei ununterscheidbare Fälle (`260824-1218`), ein stillschweigend kürzendes
`zip` (`260824-0940`), eine obere statt einer beidseitigen Schranke
(`die_profile_haben_genau_einen_schreiber_und_einen_rufer`, in Schritt 11 nachgezogen), und
jetzt ein Schnitt an der falschen Bytegrenze.

### Thema 2: der Bau ist entschieden, die Buchführung nicht

**Befund 2.1 — C6.5, A5 und zwei Stellen des Plans sagen weiter „über 2.000".** Schwere:
mittel. Datensatz
`issues/260824-1651_o_c6-5-a5-und-planschritt-6-sagen-weiter-ueber-2-000-und-kein-offener-datensatz-traegt-es.md`.

Gebaut ist `mindestens {Treffer} (Lesung bei 2000 Einträgen abgebrochen)`
(`crates/krk-core/src/leseprofil/mod.rs:566-580`); C6.5 (Spec, Zeile 309), Festlegung A5 (Zeile
86) und der Plan (Zeilen 198 und 319) nennen weiter „über 2.000".

**Das ist die Hälfte, die der geschlossene Befund `260824-1215_c_…` ausdrücklich offen gelassen
hat** („Punkt 2 bleibt offen und gehört nicht hierher"), und sie wird von keinem offenen
Datensatz getragen. Beide Planungsdateien sind nach dem Schluss jenes Befundes bearbeitet
worden — der Spec um 15:41, der Plan um 16:32, der Befund um 12:43 — und sind in zwei
Durchgängen daran vorbeigegangen. Sieben andere geänderte Kriterien dieser Runde tragen eine
Berichtigung unter ihrer Liste; C6.5 ist das einzige ohne eine.

**Die Querschnittsbeobachtung aus Turn 2 hat sich damit zur Hälfte bewahrheitet.** Jene
Durchsicht nannte drei Fälle von „der Bau ist entschieden, die Buchführung nicht" und riet,
sie in einem Zug nachzuziehen. Zwei sind nachgezogen, der dritte ist beim Schließen seines
Datensatzes durchgefallen.

### Thema 3: Zahlen und Verweise in Doc-Kommentaren, die nicht mehr stimmen

Fünf Stellen, alle niedrig, alle einzeilig zu beheben, keine mit Wirkung auf das Verhalten.
Dieses Projekt führt solche Abweichungen als Defekte, und `CLAUDE.md` selbst schreibt aus,
warum: eine Zahl in Prosa ist dort viermal in vier Tagen falsch geworden.

| Befund | Stelle | sagt | ist |
|---|---|---|---|
| 3.1 | `crates/krk-core/tests/leseprofil.rs:482-484` | „jede der **vier** Eingaben" | drei Paare in der Schleife |
| 3.2 | `crates/krk-ui/src/vorschaumodell.rs:91` und `:672` | „der eine Aufrufer … **in diesem Baum**" | elf Aufrufstellen im Baum, eine in `krk-ui` |
| 3.3 | `crates/krk-core/src/leseprofil/mod.rs:264` | „auf **drei** Vorhandensein-Zeilen" | vier seit Schritt 14 |
| 3.4 | `crates/krk-core/src/ablage/mod.rs:147` | „**drei** der fünf TOML-Dateien tragen `deny_unknown_fields`" | vier, und der Absatz nennt die vierte selbst |
| 3.5 | `crates/krk-core/tests/ablage.rs:2973` | `[`jede_toml_datei_mit_ladeweg_wird_bei_beschaedigung_zur_seite_gelegt`]` | die Probe heißt ohne `_mit_ladeweg_` |

Datensätze: `issues/260824-1652_o_die-probe-zum-verschriebenen-schluessel-…`,
`issues/260824-1653_o_zwei-prosastellen-sagen-zusammenfassen-…`,
`issues/260824-1654_o_der-doc-kommentar-am-bausteinsatz-…`,
`issues/260824-1655_o_ablage-mod-sagt-drei-der-fuenf-…`,
`issues/260824-1656_o_ein-doc-verweis-in-tests-ablage-…`.

**Befund 3.2 ist die Umkehrung eines bekannten Fehlers dieses Baums.** Der Datensatz
`shared/issues/260813-0540_*_die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste.md`
beschreibt eine Probe, die zu eng las, während ihre Prosa weit sprach. Hier liest die Probe
richtig eng — sie filtert auf `krk-ui/` und sagt es in ihrem eigenen Doc-Kommentar —, und
allein die zwei Stellen darüber sprechen weiter vom ganzen Baum.

**Befund 3.5 fällt in keine Prüfung.** Ein Testziel wird von `cargo doc` nicht dokumentiert,
also meldet kein `intra_doc_link`-Lint den toten Verweis; `make check` bleibt grün.

**Eine Nachbarzeile ist schon erfasst und nicht neu zu melden.**
`crates/krk-core/src/ablage/mod.rs:158` („Die drei uebrigen tragen `Leerbefund::Vorgabe`: zwei
davon pflegt der Nutzer von Hand") ist seit `readers.toml` ebenfalls falsch, steht aber in der
Tabelle des offenen Datensatzes
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-…`, dort unter der inzwischen
verschobenen Zeilennummer `:154`. Der Befund 3.4 verweist darauf.

---

## Was geprüft wurde und gehalten hat

**Die Räumung der vierzehn Befunde aus zwei Durchsichten (`06dbb4c`) hält, einzeln
nachgelesen.** `zusammenfassen_gezaehlt` fragt seit `:170` am **aufgelösten** Pfad, ob ein
Verzeichnis dasteht, und hält C2.6 damit an der Stelle, an der sie zu halten ist, statt am
Rufer (`260824-1214`). `Zeilendatei` trägt vier benannte `Option`-Felder statt einer
unmarkierten Auswahl hinter `#[serde(flatten)]`; `zerlegen` zählt sie und meldet „keiner" wie
„zwei" mit Namen (`260824-1216`), und `deny_unknown_fields` an der Zeile lässt die Meldung des
Tisches durch (`260824-1217`). Die Teillesungsprobe hängt ihr Muster an alle 2.001 Dateien und
nicht mehr an eine (`260824-1218`). Der `zip` der Beiseitelegeprobe trägt jetzt eine
Längengleichheit davor (`260824-0940`). `text::datei::anlesen` trägt die Messung am
Werkbankdatensatz mit Datum und im Präteritum (`260824-1014`).

**Die drei Stellen, die der Übersetzer für den siebten `Inhalt` einfordert, sind sämtlich
beantwortet**, und es sind genau drei: `Vorschaumodell::zeigt_dateitext`
(`vorschaumodell.rs:564`), `Vorschaufenster::anzeigen` (`vorschau.rs:1088`) und `einzufaerben`
(`vorschau.rs:1475`). Keine davon trägt einen Auffangzweig; `grep "_ =>"` findet in beiden
Dateien genau einen, und der steht über `Kommando` in `Vorschaufenster::kommando_ausfuehren`
und nicht über `Inhalt`. **Der siebte `Inhalt` berührt keine Stelle, an der ein fehlender
Zweig still durchginge** — die zwei Auffangzweige aus `CLAUDE.md`,
`Anwendungsdelegierter::kommando_ausfuehren` und `Tabelle::kommando_ausfuehren`, verzweigen
über `Kommando`, und die Runde 16 nimmt kein Kommando auf.

**C4.6 fällt aus dem Weg heraus, den die Zusammenfassung nimmt.** `text_zeigen`
(`vorschau.rs:1183-1189`) setzt `quellbezug_setzen(None)`; ohne Quellbezug reicht
`Vorschautext::auswahl_ablegen` an die Oberklasse durch. Eine zweite Abfangstelle entsteht
nicht, und die Probe `der_quellbezug_wird_an_genau_zwei_stellen_gesetzt` hält es.

**Die Haushaltsrechnung stimmt an jeder Bausteinsorte.** `am_ort` benutzt für eine leere
Ortsangabe den einen gemerkten Leselauf und liest für einen Unterordner genau einmal
(`bausteine.rs:291-303`); `Haushalt::oeffnungen_nehmen` nimmt ganz oder gar nicht und trägt
einen `checked_add` gegen den Überlauf. Die elf Fälle der Tabelle in
`ein_baustein_kostet_hoechstens_einen_leselauf_…` und die acht in
`die_zahl_der_oeffnungen_folgt_der_bausteinsorte` decken jede Sorte an beiden Orten ab.

**Die Regel über die Teillesung steht einmal da und wird dreimal angewandt**, und die drei
Anwendungen sind einzeln belegt: `zaehlen` liefert `UeberGrenze`, `vorhandensein` liefert `ja`
bei Treffer und sonst den Platzhalter, `juengste` liefert den Platzhalter.

**Der Deskriptorhaushalt hält auch an der neuen Stelle.** `angelesener_text` öffnet und gibt
frei, bevor der nächste Kandidat drankommt; keine Stelle in `bausteine.rs` hält einen Ordner
offen, während sie eine Datei liest.

**Die Ablagehälfte nimmt die Vorlage Zeile für Zeile.** Die sieben Fälle aus C1.1 bis C1.8
sind einzeln belegt, darunter die zwei Abweichungen von `einstellungen.rs`: die Anlage steht
vor dem Lesen (`eine_fehlende_readers_toml_entsteht_byteweise_…` prüft, dass der erste Start
schon mit fünf Profilen arbeitet), und eine beschädigte Datei führt zu keinem Profil statt zur
Auslieferungsfassung (`eine_kaputte_datei_…` und `eine_nicht_anlegbare_readers_toml_meldet_sich`).
Verglichen wird Byte für Byte gegen `AUSLIEFERUNGSTEXT` und nicht über eine Zählung.

**`tests/baum.rs` hält die Schreibwegzusage.** `ablage/leseprofile.rs` steht an der
alphabetisch richtigen Stelle in der Liste von
`nur_benannte_dateien_erreichen_das_atomare_schreiben`.

**`make check` läuft in allen vier Teilen grün**, gefahren am 260824-1640: `cargo build`,
`cargo test --workspace` (alle Ziele grün, `ignored` nur die vorhandenen Kindproben),
`cargo clippy --workspace --all-targets` ohne eine einzige Warnung, `cargo fmt --all --check`
sauber.

**Kein neuer `#![allow(unsafe_code)]`, kein neues Paket, kein Geheimnis im Baum.** Die Runde
nimmt in diesem Bereich keine fremde Kiste auf; `regex` kam mit Schritt 1 und liegt vor diesem
Bereich.

---

## Zwei Beobachtungen ohne Datensatz

**`zielordner` ruft `std::fs::canonicalize` je Zeile mit Ortsangabe, auch wenn der Haushalt
schon erschöpft ist** (`crates/krk-core/src/leseprofil/bausteine.rs:274-284`). Das bricht keine
Zusage: C6.4 zählt Verzeichnisleseläufe und Dateiöffnungen, und ein `realpath` ist keines von
beiden. Der Kommentar bei `:166-169` beziffert die uncounted Systemaufrufe aber als „einen je
Zusammenfassung", und bei einem dreizehnzeiligen Profil sind es vierzehn. Wer den Satz beim
nächsten Anfassen genauer macht, nimmt einem späteren Leser eine Rechnung ab.

**`gekappte_anzahl(0)` liefert 0**, und ein `juengste = { anzahl = 0 }` ergibt damit den
Platzhalter statt einer Meldung. C6.3 begrenzt nur nach oben, also ist das keine Abweichung;
es ist die einzige Angabe der Datei, die stillschweigend nichts tut.

---

## Querschnitt

**Der Code dieser drei Bündel entscheidet sorgfältig, und was fehlt, ist durchweg die Prosa
daneben.** Von sieben Befunden betreffen fünf einen Doc-Kommentar und einer eine
Spec-Berichtigung; genau einer betrifft eine Probe, und auch dort ist der geprüfte Zweig
richtig gebaut. Das ist eine andere Handschrift als in Bündel B, wo vier von fünf Befunden
Zusagen ohne Halter waren: die Halter sind in diesen Bündeln gebaut worden, und was ihnen
hinterherhinkt, sind die Sätze, die sie beschreiben.

**Drei der fünf Zahlbefunde sind in derselben Sitzung entstanden, in der die Zahl sich
geändert hat.** Die vierte Zustandszeile (Schritt 14) hat den Kommentar bei
`leseprofil/mod.rs:264` überholt, die fünfte TOML-Datei (Schritt 8) den bei `ablage/mod.rs:147`,
die Umbenennung einer Probe (Schritt 8) den Verweis bei `tests/ablage.rs:2973`. Keiner davon
hätte einen zweiten Durchgang gebraucht: jeder steht in derselben Datei wie die Änderung, die
ihn falsch gemacht hat.

**Die Zählproben sind das Beste an Bündel E, und ihre Grenzen stehen bei ihnen.** Vier von
ihnen benennen im eigenen Doc-Kommentar, was sie nicht sehen — die Aufrufform statt des
Aufrufs, die eine Kiste statt des Baums, den leeren Profilsatz, den `use … as anders`. Das ist
die Bauanleitung aus dem Kopf von `krk-ui/src/quellbaum.rs`, Punkt 3, und sie ist hier
durchgehalten. Befund 3.2 ist genau deshalb einer: die Probe hält ihre Grenze fest, und die
zwei Stellen, die sie zusammenfassen, tun es nicht.

---

## Reihenfolge

**Vor dem Abnahmelauf**, damit der Nutzer kein Kriterium abhakt, dem der Bau widerspricht:

1. `260824-1651_o_c6-5-a5-und-planschritt-6-…` — die Berichtigung unter C6 nachtragen, A5 und
   die zwei Planstellen mitziehen. Reine Schreibarbeit in Spec und Plan.

**Wann es passt, ohne einen Schritt aufzuhalten:**

2. `260824-1650_o_die-probe-zur-naht-des-deckels-…` — den Schnitt an eine Bytegrenze legen, die
   in einem Zeichen liegt, und den Kommentar mitziehen.
3. `260824-1653_o_zwei-prosastellen-sagen-zusammenfassen-…` — zweimal „in diesem Baum" durch
   „in `krk-ui`" ersetzen.
4. `260824-1654_o_der-doc-kommentar-am-bausteinsatz-…` — die Zahl streichen oder auf vier
   setzen.
5. `260824-1655_o_ablage-mod-sagt-drei-der-fuenf-…` — den Absatz in einem Zug richtigstellen;
   die Nachbarzeile `:158` gehört zu `shared/issues/260821-1023_o_…` und wird dort
   nachgetragen.
6. `260824-1652_o_die-probe-zum-verschriebenen-schluessel-…` — entscheiden, ob ein vierter Fall
   dazukommt oder die Zahl auf drei geht.
7. `260824-1656_o_ein-doc-verweis-in-tests-ablage-…` — `_mit_ladeweg_` streichen.

**Kein Befund hält einen Planschritt auf, und keiner ist ein Auslieferungshindernis.** Die
Runde steht damit vor der Nutzerarbeit, und die sieben Punkte aus `## Nutzerarbeit` des Plans
sind das, was noch aussteht.

---

**Abgleich 260824-1852.** Alle sieben Befunde dieser Durchsicht stehen als Datensätze unter
`issues/` dieser Runde und tragen den Marker geschlossen (`_c_`); geräumt sind sie mit `7180b3e`
und `79209c8`. Nachgelesen am Baum: die Probe zur Naht schneidet jetzt mitten in einem Zeichen
(`crates/krk-core/src/leseprofil/bausteine.rs`, Prüfmodul), die Zeile „Sitzung" trägt das Muster
`## Current\n(?:[^\S\n]*\n)*[^\S\n]*([^#\n][^\n]*)` (`resources/default-readers.toml:210`), und
der Modulkopf von `crates/krk-core/src/leseprofil/bausteine.rs:47` sagt „mindestens" wie
`Wert::UeberGrenze`. **Ein Rest steht noch:** die Fehlermeldung der Teillesungsprobe
(`crates/krk-core/tests/leseprofil.rs:1382`) trägt weiter die alte Aussage; der Datensatz dafür
ist `issues/260824-1852_*_die-meldung-der-teillesungsprobe-…`. Der Text dieser Durchsicht bleibt
unverändert.
