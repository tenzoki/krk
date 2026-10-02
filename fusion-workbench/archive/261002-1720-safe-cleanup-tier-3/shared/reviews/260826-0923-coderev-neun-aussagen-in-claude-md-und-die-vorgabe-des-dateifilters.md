# coderev — die neun Aussagen in `CLAUDE.md` und die Vorgabe des Dateifilters

**Reviewed-range:** `e5ec81a..20c9833`
**Not-opened:** `resources/default-readers.toml`, `fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`, `fusion-workbench/shared/decisions/260825-1725_i_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-erreicht-ein-baustein-die-eintraege-mehrerer-gleichartiger-unterordner.md`, `fusion-workbench/shared/decisions/260825-1725_i_wie-kommt-ein-aenderungsdatum-in-eine-profilzeile.md`, `fusion-workbench/shared/decisions/260825-1725_i_wo-wohnt-die-umrechnung-von-systemtime-in-buergerliche-ortszeit.md`, `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_c_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md`, `fusion-workbench/shared/issues/260825-2230_c_der-plan-der-runde-18-verlangt-in-schritt-3-noch-die-zeile-in-der-abschlussliste-die-acc9671-gestrichen-hat.md`, `fusion-workbench/shared/issues/260825-2126_c_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md`, `fusion-workbench/shared/issues/260825-2126_c_der-flight-kommentar-nennt-drei-felder-in-fusion-setup-es-sind-zwei.md`, `fusion-workbench/shared/issues/260826-0902_o_keine-probe-im-baum-haelt-die-vier-zahlen-der-flight-profile.md`, `fusion-workbench/shared/issues/260826-0903_o_die-zeichengleichheit-der-zwei-werkbankpaare-wird-je-durchsicht-von-hand-gemessen-und-von-nichts-gehalten.md`, `fusion-workbench/shared/issues/260826-0904_o_der-doppelungshinweis-sagt-vor-der-ortsangabe-und-fuenf-von-sieben-fusion-zeilen-haben-keine.md`, `fusion-workbench/shared/history/260826-0810-ontocoder-die-vier-flight-speicher-tragen-jetzt-ihre-datumszeile.md`, `fusion-workbench/shared/reviews/260826-0139-coderev-dritte-nachdurchsicht-die-beispielzahl-vier-haelt.md`, `fusion-workbench/shared/reviews/260826-0145-ontorev-dritte-nachdurchsicht-der-profildatei-vor-der-auslieferung.md`, `fusion-workbench/shared/reviews/260826-0906-ontorev-die-datumszeilen-der-vier-flight-speicher.md` (nur der Kopf gelesen)

Die letzten fünfzehn Einträge sind die Übernahme aus der `Not-opened`-Zeile der Durchsicht
`260826-0906-ontorev`. Von ihren fünfzehn habe ich sechs geöffnet
(`260825-1725_i_liest-eine-zusammenfassung-…`, `260825-1725_i_nimmt-ein-klick-auf-die-tableiste-…`,
`260826-0157-reconciliation.md`, `260826-0818-curator-run.md` und die beiden `260826-0149_o_*`),
neun stehen oben. Alle neun liegen außerhalb des Gegenstands beider Commits: sie gehören der
Vorschau- und Leseprofil-Arbeit der Runde 18 und stehen in diesem Bereich nur, weil `d08dbac`
ihre Marker umbenannt hat.

## Zusammenfassung

Beide Commits halten in der Sache. Alle neun Belege von `fb50fcd` habe ich selbst gefahren, und
alle neun bestätigen die neue Fassung; die vier Feststellungen von `20c9833` habe ich am Baum
nachgeprüft, und alle vier tragen, die schwerste — die zehn Zeitzusagen aus C8 sind nicht
betroffen — vollständig. **Kein Befund ist kritisch, keiner hält die Auslieferung an.** Die zwei
Befunde mittlerer Schwere betreffen beide `fb50fcd`: eine falsche Runden-Zuschreibung und drei
Defektdatensätze, die der Lauf behoben und offen gelassen hat. Der eine offene Punkt an `20c9833`
ist keine Fehlfunktion, sondern eine Frage, die der `coder` an der Nachbargröße gestellt und an
der Hauptgröße nicht gestellt hat.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 2 Defekte, 1 Entscheidungsfrage |
| Gering | 2 Defekte, 1 Vermerk an einem bestehenden Datensatz |

## Teil 1 — `fb50fcd`: die neun Belege, selbst gefahren

Jedes Kommando am 260826-0923 gegen den Baumstand `20c9833` gefahren. Die Spalte „hält" sagt, ob
das Ergebnis die neue Fassung in `CLAUDE.md` trägt.

| | Kommando und Ergebnis | hält |
|---|---|---|
| L01 | `ls fusion-workbench/circles/*/_[bc]_circle.md \| wc -l` → **17**; die Rundentabelle führt 18 Runden | ja |
| L02 | dieselbe Messung; die 17 Verzeichnisse decken sich Zeile für Zeile mit den Tabellenzeilen 1 bis 17 | ja |
| L03 | dieselbe Messung | ja |
| L04 | `_b_` → **12**, `_c_` → **5**, Summe 17 | ja |
| L05 | `sed -n '203,206p' Cargo.toml` → `features = [ "deflate-flate2", "unreserved" ]` | ja |
| L06 | `awk '/pub const ALLE/,/\];/' crates/krk-core/src/ablage/pfade.rs` → `[Datei; 7]` mit `Belegung`, `Lesezeichen`, `Sitzung`, `Einstellungen`, `Leser`, `Zettel(Erster)`, `Zettel(Zweiter)` | ja |
| L07 | `anwendung.rs:1225-1230` trägt beide Empfänger in der genannten Reihenfolge | ja, bis auf die Runden-Zuschreibung |
| L08 | `git for-each-ref … refs/tags` → am **2026-08-22** steht kein Tag (15.08. bis 21.08. lückenlos, dann 23.08.) | ja |
| L09 | `ls fusion-workbench/circles/*/_d_circle.md` → **zwei**; `grep -ci makro CLAUDE.md` → **0** vor `fb50fcd`, **1** danach; `b8e198e` datiert 2026-08-17 04:51 | ja |

### L07, die einzige Zusagenrücknahme: beide Hälften geprüft

Die erste Hälfte hält. `crates/krk-ui/src/appkit/anwendung.rs:1225-1230`:

```rust
fenster.melder_setzen(Box::new(move || {
    if let Some(selbst) = schwach.load() {
        selbst.aktives_dem_ersthelfer_nachziehen();
        selbst.fokusanzeige_nachziehen();
    }
}));
```

Die zweite Hälfte hält, und die Kette ist vollständig nachgelesen:
`aktives_dem_ersthelfer_nachziehen` (`:4648`) ruft `aktives_setzen` (`:4577`), das ruft
`aufteilung_nachziehen` (`:4825`), und das ruft `aufteilung.anwenden(&breiten, &sichtbar)`
(`:4833`). `fokusanzeige_nachziehen` (`:5057`) schreibt ausschließlich `rahmen_setzen` und
`titel_nachziehen`.

Die Probe `der_nachzug_der_anzeige_ruehrt_die_auslegung_nicht_an` (`:8691`) besteht und hält, was
die neue Fassung ihr zuschreibt: sie liest den **Rumpftext** von `fokusanzeige_nachziehen` und
verlangt die Abwesenheit von `anwenden(`, `setHidden(`, `aufteilung_nachziehen(` und
`aktives_setzen(`. Ihr eigener Doc-Kommentar grenzt ab, was sie nicht sieht („einen Weg, der über
eine dritte Funktion dorthin führt"). Die Zuschreibung in `CLAUDE.md` ist damit genau und nicht zu
weit gefasst.

**Ein Nebenbefund, kein Defekt:** `aufteilung_nachziehen` ruft seinerseits
`fokusanzeige_nachziehen` (`:4834`). Auf dem Melderpfad läuft der Nachzug der Anzeige also
zweimal. Der Kommentar bei `:1217-1223` benennt das und begründet es („Das ist der billigere
Fehler"). Nichts daran ist neu oder falsch dargestellt.

### Befund 1 (mittel) — „seit der Runde 14" ist falsch

`CLAUDE.md` sagt jetzt: „**Am Melder hängen seit der Runde 14 zwei Empfänger**". Der zweite
Empfänger kam mit `76ceb68` am **2026-08-19 11:20** in den Baum. Die Runde 13 war um **08:12**
geschlossen (`c09ff3a`), die Runde 14 wurde um **22:31** aktiv (`258bd7c`). Der Commit trägt kein
einziges Werkbank-Artefakt (`git show --stat 76ceb68 | grep fusion-workbench` ist leer); seine
Artefakte liegen unter `shared/`. Er gehört zu keiner Runde.

Der Fehler ist genau die Klasse, die derselbe Commit an vier Stellen behebt. Er stammt nicht vom
Kuratorenlauf, sondern aus dem Datensatz, den er umgesetzt hat: `260823-1336` schreibt zweimal
„Runde 14". Beide Stellen sind mitzuberichtigen, sonst kommt sie zurück.

Abgelegt als `shared/issues/260826-0923_o_claude-md-schreibt-den-zweiten-empfaenger-der-runde-14-zu-er-landete-ohne-aktiven-circle.md`.

### Befund 2 (mittel) — drei behobene Datensätze stehen offen

`fb50fcd` erledigt, was drei offene Defektdatensätze verlangen; alle drei tragen weiter `_o_`.
Der Kuratorenlauf weist den Marker in seinem Abschnitt 9 ausdrücklich dem `reconciler` zu, und der
letzte Abgleich lief um **260826-0157**, also **vor** `fb50fcd` (**0831**). Seither hat niemand
nachgezogen.

- `260823-1336_o_…` (L07) — vollständig umgesetzt, vor dem Schließen ist Befund 1 zu berichtigen.
- `260823-1649_o_…` (L08) — vollständig umgesetzt.
- `260820-2056_o_claude-md-nennt-eine-zaehlprobe-unter-einem-namen-den-der-baum-nicht-traegt.md` —
  schon vorher behoben; `cargo test --workspace` fährt
  `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` unter diesem Namen.

Zwei bleiben zu Recht offen. `260825-1859_o_…` (L05) ist in `CLAUDE.md` behoben, aber seine
`Also seen`-Zeile trägt dieselbe Wendung in `Cargo.toml:155-158`, und dort steht sie unverändert:
„`default-features = false` mit dem **einen** Merkmal `deflate-flate2`". `260826-0149_o_die-runde-18-…`
hat von seinen vier Möglichkeiten nur die dritte bekommen; die zweite gehört dem Nutzer.

Abgelegt als `shared/issues/260826-0923_o_drei-behobene-claude-md-datensaetze-stehen-weiter-offen-und-niemand-ist-dafuer-beauftragt.md`.

### Befund 3 (gering) — „alles unter `shared/`" trifft nicht ganz

Zeile 18 der Tabelle sagt „kein Circle-Datensatz, alles unter `fusion-workbench/shared/`", der
Absatz darüber „vollständig unter `fusion-workbench/shared/`". Zwei Commits der Runde 18 schreiben
in den Circle der Runde 17:
`git log --name-only 2a77012..HEAD -- fusion-workbench/circles/` liefert `e922c9e` und `d08dbac`,
beide am Defektdatensatz zum Zip-Zeitstempel — also gerade an dem Vorgang, den Zeile 18 selbst als
Arbeit der Runde 18 führt.

Nach der Herkunftsregel liegt der Datensatz richtig, wo er liegt; die Runde 18 hat ihn nur
geschlossen. Die Folgerung des Absatzes hängt nicht daran. Als `Also seen`-Zeile an
`shared/issues/260826-0149_o_die-runde-18-hat-keinen-circle-datensatz-….md` vermerkt, kein
eigener Datensatz.

### Befund 4 (gering) — die Ablage-Aufzählung steht neben ihrem Zeiger

L06 setzt den Zeiger „**Welche Dateien das sind, sagt `Datei::ALLE` … und nicht diese Zeile**" und
lässt die Aufzählung davor stehen, jetzt auf sechs Glieder gebracht. Sie stimmt heute und veraltet
mit dem achten `Datei`-Wert. `CLAUDE.md` behandelt dieselbe Klasse an drei anderen Stellen anders
und ausdrücklich: bei `Kommando` steht „**keine Zahl**", bei `ohne_warten_oeffnen` der Befehl statt
der Liste, bei `Wirkungsbereich` die Zahl mit Zähldatum. Die neue Stelle nimmt keine der drei
Formen an.

Abgelegt als `shared/issues/260826-0923_o_die-ablage-aufzaehlung-steht-neben-dem-zeiger-der-sie-ersetzen-sollte-und-veraltet-mit-der-achten-datei.md`.

### Befund 5 (gering) — die Pfadregel ist für Zeile 18 nicht total

Der Absatz unter der Tabelle liest Pfade der Form `planning/…`, `decisions/…`, `analyses/…`,
`issues/…` relativ zum Verzeichnis des jeweils genannten Circles, „ohne Nennung gilt die Runde 2".
Zeile 18 nennt keinen Circle. Für sie greift stillschweigend der Zweig, der für „gar keine Nennung"
gedacht ist, und der zeigt auf den Circle der Runde 2.

Heute läuft nichts falsch, und das ist Zufall der Zitierweise:
`grep -oE '`[^`]*(planning|decisions|issues|analyses)/[^`]*`' CLAUDE.md | sort -u` zeigt, dass
jedes Zitat zur Runde 18 den Vorsatz `shared/` trägt und damit gar keines „der Form `issues/…`"
ist. Die Lücke wächst mit der nächsten Runde ohne Circle.

Abgelegt als `shared/issues/260826-0923_o_die-pfadregel-nach-der-rundentabelle-ist-fuer-die-neue-zeile-18-nicht-mehr-total.md`.

### Die sachlichen Ergänzungen in L05 und L06 halten

L05 behauptet, `unreserved` hebe die Prüfung auf, die sonst das Zusatzfeld `0x5855` abwiese. Der
Baum trägt beide Hälften: `Cargo.toml:176-181` schreibt es aus („ohne es weist
`ExtendedFileOptions::add_extra_data` jede Feldkennung ab, die in `EXTRA_FIELD_MAPPING` steht"),
und `crates/krk-core/src/operation/zippen.rs:623-624` nennt `FELD_INFOZIP_UNIX: u16 = 0x5855` mit
demselben Grund; `:688` ist der Aufruf. Die Wendung „das `ditto(1)` als einziges Zeitfeld liest"
ist präzise: die Messtabelle in `Cargo.toml:189-193` zeigt, dass `ditto` das andere Zusatzfeld
`0x5455` übergeht.

L06 nennt `Datei::ALLE` als Quelle im Baum; die Stelle existiert und trägt die sieben Werte. Die
Aussage, die alte Aufzählung habe die Einstellungen und die Leseprofile übergangen, hält gegen
`git show fb50fcd~1:CLAUDE.md`.

### Zur Frage nach neuen veraltenden Zahlen

Von den neun Einträgen führt einer eine Aufzählung ein, die mit der nächsten Runde falsch werden
kann: L06 (Befund 4). L07 führt eine Zahl ein („zwei Empfänger"), und keine Probe im Baum hält
sie — die Zählprobe `aktives_setzen_hat_genau_zwei_aufrufer` zählt die Rufer von `aktives_setzen`
und nicht die Empfänger des Melders. Ein dritter Empfänger machte den Satz still falsch. Das ist
kein eigener Defekt, sondern dieselbe Gestalt wie Befund 4 und dort mitgenannt.

Die Tabellenzeile 18 wächst mit jeder Runde, aber das ist die Bauart der Tabelle und in
`CLAUDE.md` ausdrücklich so gewollt.

Die drei Kandidaten K01 bis K03, die der Kuratorenlauf nicht zur Freigabe gestellt hat, sind nicht
Gegenstand dieser Durchsicht. Ich bin unabhängig auf keinen von ihnen gestoßen.

## Teil 2 — `20c9833`: die Vorgabe des Dateifilters

Alle vier Feststellungen des `coder` halten. Selbst am Baum geprüft, nicht am Bericht.

**1. Die Sitzung führt den Stand nicht mit.** `krk_core::ablage::sitzung::Tab`
(`crates/krk-core/src/ablage/sitzung.rs:82-113`) trägt fünf Felder: `ordner`, `auswahl`,
`verstecke_ausgeblendet`, `sortierung`, `bildlauf`. Keines ist Deep oder Content. Die Vorgabe
greift bei jedem Start.

**2. Ein neuer Tab erbt nichts.** `Tabinhalt` wird an genau **fünf** Stellen gebaut, und alle fünf
gehen durch `Tabinhalt::aus_zustand` (`tabs.rs:413`, `:418`, `:494`, `:550`, `:568`, `:672`) und
damit durch `Ordnermodell::neu` (`:101`). Übernommen werden dort ausschließlich Sortierung und der
Stand der versteckten Einträge (`:102-103`). Der Ordnerwechsel innerhalb eines Tabs trägt `tief`,
`inhalt` und den Filtertext hinüber (`ordner_setzen`, `:657`, `:678`). Der einzige Rufer von
`Ordnermodell::neu` außerhalb der Prüfmodule ist daneben `krk-bench/src/messen.rs:206`.

**3. Das Ankreuzfeld zieht aus demselben Wert.** `bereichsleiste_nachziehen` (`anwendung.rs:4920`)
liest `quelle.tiefe_suche_steht()`, und das ist `tabs.aktiver().modell().tief()`
(`tabelle.rs:2853-2856`). Der erste Ruf läuft beim Aufbau über `aufteilung_nachziehen`
(`anwendung.rs:1426` → `:4835`). Ein Modell auf `true` mit leerem Kästchen kann es nicht geben.

**4. Die zehn Zeitzusagen aus C8 sind nicht betroffen — die Behauptung trägt vollständig.**
Drei unabhängige Prüfungen:

- `Ordnermodell::zeilengrund_von` verlässt den Prüfschritt bei leerem Filtertext mit
  `Zeilengrund::Steht` (`modell.rs:733-735`), also **31 Zeilen vor** der Frage nach der Tiefe
  (`:766`).
- `Tabliste::durchlauf_nachziehen_an` gibt bei `!filter_steht()` `false` zurück (`tabs.rs:897`).
- Keine der beiden Messstrecken setzt je einen Filtertext. `grep -rn 'filtertext_setzen'` findet in
  `crates/krk-bench/src/` und `crates/krk-ui/src/messmodus.rs` keinen Treffer, und der einzige
  Tastendruck der Strecke ist `Anweisung::Taste` → `ereignisse::pfeil_ab_senden`
  (`anwendung.rs:7814`); `traegt_ein_dateiname('\u{F701}')` ist `false`, geprüft von der Probe
  `ein_wagenruecklauf_und_eine_funktionstaste_tragen_kein_dateiname` (`filter.rs:166`).

Dazu: keine der zehn Zusagen misst überhaupt das Tippen. `messen.rs:765-1130` führt sie einzeln
aus — L1 „Tastendruck bis Ende des Zeichendurchgangs" (Pfeil ab), L2/L3/L10 Lesevorgänge, L4
Prozessstart, L5 Tab- und Fensterwechsel, L6 Einstieg, L7 Vorschau, L8 Kopie, L9 Bildanteil.

### Die elf roten Proben: keine hat ihren Gegenstand verloren

Geprüft an jeder Aufrufstelle der zwei Helfer. Die Bauart ist richtig gewählt: `gefiltert` und
`handmodell` schalten flach, und jede Probe, die den tiefen Zweig misst, ruft danach ausdrücklich
`tief_setzen(true)` — so etwa `die_tiefe_suche_hebt_die_schwelle_auf_fuenf_zeichen`
(`tests/verzeichnis.rs:1311`), `ein_ausgeblendeter_eintrag_bekommt_keinen_auftrag` (`:1519`) und
`die_auftragsliste_traegt_je_typ_die_richtige_art` (`:1570`). Der gemessene Zweig ist danach in
jedem Fall derselbe wie vorher.

Die eine Probe, die eine Aussage über die **Vorbelegung** trug, ist genau die, die sie abgegeben
hat: `bei_flacher_suche_bleibt_jeder_ordner_stehen` (`:766`) behauptete „die flache Suche ist die
Vorbelegung" und sagt jetzt „diese Probe fährt flach; `gefiltert` schaltet ab". Die Aussage ist
nicht verschwunden, sondern in die neue Probe umgezogen. Das ist die richtige Behandlung und
nicht das Umschreiben einer Zusage.

### Die zwei neuen Proben halten die Vorgabe und werden rot, wenn man sie zurückdreht

Beide laufen und bestehen (`cargo test --workspace`, 0 Fehlschläge über 20 Prüfziele).

`die_tiefe_suche_ist_die_vorbelegung` (`tests/verzeichnis.rs:797`) prüft in der ersten Zeile
`assert!(frisch.tief())` auf einem `Ordnermodell::neu(1)` — genau der Wert, den `20c9833`
gekippt hat. Ein Zurückdrehen macht sie in der ersten Zeile rot. Sie hält daneben die
mitverschobene Schwelle: vier Zeichen wirken nicht, das fünfte schon.

`ein_neuer_tab_traegt_die_vorbelegung_der_tiefen_suche` (`tabs.rs:1604`) setzt den Nachbartab
ausdrücklich auf `false` und verlangt vom neuen Tab `true`. Der Aufbau ist richtig: mit
Vererbung wäre der neue Tab `false`, mit zurückgedrehter Vorgabe ebenfalls. Beide Fehler machen
sie rot, und sie trennt sie durch ihre Meldung.

*(Die Rot-Aussage ist am Quelltext gelesen und nicht durch einen Lauf mit zurückgedrehter Vorgabe
belegt; ein solcher Lauf hätte den Baum ändern müssen, und diese Durchsicht ändert nichts. Die
Assertion steht unmittelbar auf dem gekippten Wert, die Folgerung ist damit nicht abgeleitet,
sondern abgelesen.)*

### Die Schwelle des Inhaltsfilters: die Frage trifft den Sachverhalt

`shared/decisions/260826-0859_o_…` trifft ihn genau. Nachgeprüft: `filter::inhaltsschwelle(tief)`
gibt `if tief { 5 } else { 3 }` (`filter.rs:157-159`), und der eine Frager ist
`Ordnermodell::inhalt_wirkt` (`modell.rs:1080`). Beide Constraints des Datensatzes halten am Baum,
die drei Möglichkeiten sind disjunkt, und die Empfehlung ist begründet. Nicht gedoppelt.

### Befund 6 (mittel, Entscheidungsfrage) — der Durchlauf selbst hat keine Schwelle

Der `coder` hat die Frage für die **Nebenwirkung** gestellt und für die **Hauptwirkung** nicht.

`Tabliste::durchlauf_nachziehen_an` stößt den Durchlauf über den Unterbaum an, sobald
`filter_steht()` und `tief()` gelten (`tabs.rs:897`), und `filter_steht()` ist
`!self.filtertext.is_empty()` (`modell.rs:936`) — **ein** Zeichen. Der erste Anschlag im
Dateifenster startet damit ab Werk einen Faden, der den Unterbaum abläuft. Der Durchlauf trägt
dagegen ausgeschrieben keine Schranke: „**Keine Tiefengrenze und keine Zaehlung gegen eine
Grenze** … **Keinen Deckel auf die Trefferzahl**" (`durchlauf.rs:136-148`). Der erste Treffer
entscheidet einen Auftrag, also greift die Ersparnis nur dort, wo es einen Treffer gibt; ein
Filtertext ohne Treffer läuft vollständig ab.

Ohne „Content" liest der Lauf **keine** Datei — `zeilengrund_von` erteilt einen
`Auftragsart::Inhalt` nur bei `inhalt_wirkt()` (`modell.rs:757-763`), und „Content" bleibt ab Werk
aus. Es geht allein um den Verzeichnisdurchlauf.

Der Vergleich, der die Frage schärft: die Profil-Zusammenfassung der Vorschau liest ebenfalls
Verzeichnisse ohne ausdrückliches Zutun des Nutzers und trägt **vier** Schranken, jede an ein
Abnahmekriterium gebunden (`leseprofil/mod.rs:111`, `:122`, `:138`, `:141`). Die Runde 18 hat
`HOECHSTENS_EINTRAEGE` gerade deshalb umgedeutet, weil `HOECHSTENS_LESELAEUFE` „es nicht mehr
kann: die Zahl der Unterordner eines Ordners wächst mit dem Bestand". Genau diese Größe läuft der
Filter ungedeckelt ab.

Abgelegt als `shared/decisions/260826-0923_o_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-jetzt-wo-ein-anschlag-ihn-ab-werk-ausloest.md`,
mit drei Möglichkeiten und der Empfehlung „erst messen", weil dieselbe fehlende Messung auch
`260826-0859` entscheidet.

### `CLAUDE.md` wird durch die Deep-Änderung nicht falsch

Gegen die Fassung nach `fb50fcd` geprüft. `grep -i 'deep\|tief\|inhaltsschwelle\|filter'` liefert
sieben Stellen; keine nennt eine Vorbelegung. Der Filterabsatz (Zeile 143) nennt `inhaltsschwelle`
ohne Zahl und die Regel des Ordnerwechsels, beide gelten unverändert. Die Tabellenzeilen 10 und 11
beschreiben die Ankreuzfelder und nicht ihren Anfangszustand. Der `coder` hat recht: es fehlt
allein die Vorgabe selbst, und sie steht an genau einer Stelle mit einem ausführlichen
Doc-Kommentar darüber.

### Der offene Datensatz „je Tab oder je Fenster" wird nicht vorentschieden

`circles/260814-1551-…/decisions/260814-1830_o_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`
bleibt offen, und das Verhalten hat sich nicht geändert: ein neuer Tab bekam schon vorher ein
frisches Modell, nur mit dem anderen Wert. Zwei Beobachtungen dazu, keine davon eine
Vorentscheidung:

- Die neue Probe `ein_neuer_tab_traegt_die_vorbelegung_der_tiefen_suche` schreibt das
  Je-Tab-Verhalten erstmals fest. Ihr eigener Doc-Kommentar sagt es und benennt die Folge: „Fällt
  die offene Frage einmal auf ‚je Fenster', wird diese Probe rot und ist dann zu Recht rot." Das
  ist die richtige Behandlung.
- Das Gewicht der Frage verschiebt sich. Der Kontra-Punkt der Möglichkeit 2 lautet: „Bei ‚Deep' an
  stößt jeder Tabwechsel in einen gefilterten Tab einen Durchlauf an." Dieser Fall ist seit
  `20c9833` der Normalfall statt der Ausnahme. Die Möglichkeiten stehen unverändert, ihre Kosten
  nicht. Wer die Frage später beantwortet, sollte das mitlesen.

## Was quer liegt

**Ein Datensatz, der eine Aussage trägt, trägt auch ihren Fehler weiter.** Befund 1 stammt nicht
vom Kuratorenlauf, sondern aus `260823-1336`, das der Lauf wörtlich umgesetzt hat. Befund 3 steht
in `260826-0149` und ist von dort nach `CLAUDE.md` gewandert. Beide Male hat der Lauf getan, was
er sollte, und der Fehler stand schon im Auftrag. Wer einen Datensatz umsetzt, prüft seine
Belegzeilen mit.

**Behoben und offen fallen in diesem Projekt auseinander, weil zwei Agenten sich die Arbeit
teilen.** Die Konvention sagt für einen Defekt: die Behebung und die Schließung sind dasselbe
Ereignis. Hier behebt der `curator` und schließt der `reconciler`, und dazwischen liegt jedes Mal
eine Lücke, in der der Speicher falsch antwortet. Der Kuratorenlauf benennt das für einen Fall
selbst und übersieht die zwei, die er gerade erzeugt hat.

**Zwei Mechanismen lesen Verzeichnisse, einer ist gedeckelt und einer nicht.** Bis gestern
unterschied sie, dass der ungedeckelte einen Klick verlangte. Seit `20c9833` verlangt er einen
Anschlag. Das ist Befund 6.

## Reihenfolge

Nichts davon hält eine Auslieferung an.

1. Befund 1 — die falsche Zuschreibung an zwei Stellen richten, bevor `260823-1336` schließt.
2. Befund 2 — die drei Marker nachziehen, dazu die eine Zeile in `Cargo.toml:157`.
3. Befund 6 — vor die nächste größere Filterarbeit legen, zusammen mit `260826-0859`.
4. Befunde 4 und 5 — Aufräumen an `CLAUDE.md`, beim nächsten Kuratorenlauf.

## Abnahme

Selbst gefahren am 260826-0923 gegen `20c9833`:

```
cargo test --workspace              0 Fehlschläge über 20 Prüfziele
cargo clippy --workspace --all-targets   exit 0, keine Warnung
cargo fmt --all --check             exit 0
```

**Der Stand kann ausgeliefert werden.**

---

## Abgleichvermerk — 260826-1024

Nachgetragen vom `reconciler` beim Schlussabgleich der Sitzung gegen `e5ec81a..c95f28b`. Die
Befunde selbst sind unverändert; hier steht allein, welche seither erledigt sind.

| Befund | Stand | Beleg |
|---|---|---|
| 1 (mittel) — „seit der Runde 14" ist falsch | **erledigt** | `c95f28b` stellt `CLAUDE.md:141` auf „seit dem 260819 (`76ceb68`)"; der Quelldatensatz `shared/issues/260823-1336_*` trägt einen `Revised by: c95f28b`-Vermerk und steht auf `_c_`. Der abgelegte Datensatz `260826-0923_*_claude-md-schreibt-den-zweiten-empfaenger-…` ist geschlossen |
| 2 (mittel) — drei behobene Datensätze stehen offen | **erledigt** | Die drei Marker sind nachgezogen: `260823-1336` → `_c_` (`fb50fcd`), `260823-1649` → `_c_` (`fb50fcd`), `260820-2056` → `_c_` (`90f8ac1`). Der abgelegte Datensatz `260826-0923_*_drei-behobene-claude-md-datensaetze-…` ist geschlossen. `260825-1859_*` bleibt offen: `Cargo.toml:157` trägt unverändert „mit dem einen Merkmal `deflate-flate2`" |
| 3 (gering) — „alles unter `shared/`" trifft nicht ganz | steht als `Also seen`-Zeile an `260826-0149_*`, das mit `fb50fcd` geschlossen ist. Die Zeile bleibt dort stehen und trägt weiter | |
| 4 (gering) — die Ablage-Aufzählung neben ihrem Zeiger | **offen** | `shared/issues/260826-0923_o_die-ablage-aufzaehlung-…` |
| 5 (gering) — die Pfadregel ist für Zeile 18 nicht total | **offen** | `shared/issues/260826-0923_o_die-pfadregel-nach-der-rundentabelle-…` |
| 6 (Entscheidungsfrage) — der Durchlauf hat keine Schwelle | **offen und unbeantwortet** | `shared/decisions/260826-0923_o_bekommt-der-tiefe-durchlauf-…` |

**Zwei Feststellungen dieser Durchsicht sind unabhängig nachgeprüft und tragen.** Die Vorbelegung
steht an genau einer Stelle (`crates/krk-core/src/verzeichnis/modell.rs:374`, mit dem Kommentar
über der Zeile, der die Nebenwirkung auf die Schwelle benennt), und `filter_steht()` ist ein
Zeichen (`tabs.rs:897`). `make check` über `c95f28b` selbst gefahren, alle vier Kommandos grün.

**Eine Lücke ist neu abgelegt.** Die Einschätzung „`CLAUDE.md` wird durch die Deep-Änderung nicht
falsch" hält, und der Abgleich hat sie am Baum ein drittes Mal bestätigt. Was fehlt, ist der
Satz: `shared/issues/260826-1024_o_claude-md-sagt-nicht-dass-die-tiefe-suche-ab-werk-steht-….md`.
