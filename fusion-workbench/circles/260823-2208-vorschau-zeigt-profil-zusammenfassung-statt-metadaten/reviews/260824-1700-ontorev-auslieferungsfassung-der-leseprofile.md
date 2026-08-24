# Durchsicht: die Auslieferungsfassung der Leseprofile

**Reviewed-range:** `abe1a31..f9e34e7`
**Not-opened:** `crates/krk-core/src/ablage/mod.rs`, `crates/krk-core/src/text/datei.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/baum.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/vorschau.rs`, `crates/krk-ui/src/vorschaumodell.rs`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0600_a_der-titel-aus-der-ueberschriftenzeile-erreicht-keinen-einzigen-defektdatensatz.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0634_i_bekommt-das-circle-profil-eine-vierte-zustandszeile-fuer-die-abgelegten-runden.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1313_i_deckt-das-speicherprofil-auch-decisions-memos-und-investigations-ab.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1234-analyst-raeumung-der-spec-und-planbuchfuehrung.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1246-coder-raeumung-der-code-befunde-aus-zwei-durchsichten.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1457-coder-die-drei-fehlenden-probenpflichten-aus-schritt-8.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1538-analyst-zwei-antworten-in-spec-und-plan.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1559-coder-der-anzeigezweig-und-der-weg-der-profile.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1612-coder-der-siebte-inhalt-und-die-profile-am-arbeitsfaden.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1650-ontocoder-vierte-zustandszeile-und-drei-speichernamen.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1755-coder-die-anwendung-laedt-die-profile-und-uebergibt-sie.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-1902-coder-die-zaehlproben-zu-c6.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-0634_c_c6-1-sagt-der-feldbaustein-lese-kein-verzeichnis-seine-form-aus-c3-verlangt-es.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-0940_c_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-0955_c_die-files-zeile-eines-planschritts-nennt-die-quelldateien-und-nicht-die-testdateien.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1014_c_c3-14-nennt-bis-zur-grenze-lesen-als-den-leseweg-und-schritt-4-hat-anlesen-gebaut.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1014_c_vierzehn-prosastellen-der-ablage-sagen-weiter-vier-und-ein-offener-datensatz-schuetzt-drei-davon.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1014_c_zwei-doc-kommentare-in-datei-rs-tragen-eine-messung-an-einem-werkbankdatensatz-im-praesens.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1042_c_schritt-3-zaehlt-vier-abweisungen-auf-ein-unuebersetzbares-muster-in-einem-baustein-ist-eine-fuenfte.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1124_c_c4-3-sagt-eine-zeile-je-profilzeile-und-c3-9-verlangt-einen-absatz.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1124_c_zwei-feldmuster-der-auslieferungsfassung-verankern-mit-dach-und-koennen-nie-treffen.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1214_c_zusammenfassen-nimmt-auch-eine-datei-an-und-c2-6-haengt-allein-am-kuenftigen-rufer.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1215_c_die-abgeschnittene-zaehlung-zeigt-ueber-treffer-und-c6-5-verlangt-ueber-2000.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1216_c_zwei-bausteintische-in-einer-zeile-werden-schweigend-angenommen-und-der-untere-faellt-weg.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1217_c_ein-tippfehler-in-einem-bausteintisch-kostet-alle-profile-und-die-meldung-nennt-ihn-nicht.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1218_c_die-probe-zur-teillesung-laesst-zwei-ergebnisse-zu-und-belegt-die-haelfte-nicht-die-sie-belegen-soll.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1242_c_die-kommentarzeilen-der-auslieferungsfassung-sagen-nicht-dass-ein-schreibfehler-die-ganze-datei-kostet.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1313_c_der-datensatz-zur-vierten-zustandszeile-nennt-ein-verzeichnis-ausserhalb-der-drei-zeilen-es-sind-zwei.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1508_o_die-meldung-einer-ersetzung-verspricht-den-auslieferungszustand-den-readers-toml-nicht-bekommt.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, `fusion-workbench/circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/reviews/260824-1220-coderev-turn-2-profilmodell-ohne-fenster.md`

**Sender:** ontorev
**Gegenstand:** `resources/default-readers.toml`, die Auslieferungsfassung der Leseprofile aus den
Commits `8433935` und `b5bf2e3`.
**Nicht Gegenstand:** der Rust-Anteil dieses Bereichs. Die Kisten `krk-core` und `krk-ui` liegen
beim parallel laufenden `coderev`; die vier Rust-Dateien unter `crates/krk-core/src/leseprofil/`
und `crates/krk-core/tests/leseprofil.rs` sind hier allein als Nachschlagewerk geöffnet worden,
um zu bestimmen, was ein Muster dieser Datei tut. Kein Befund dieser Durchsicht ist ein Befund
über sie.
**Werkzeug:** ein Wegwerfprogramm außerhalb des Baumes gegen `regex` 1.13.1 und `toml` 1.1.4,
dieselben Fassungen, die `Cargo.lock` führt. Es liest die Auslieferungsfassung mit den Strukturen
aus `leseprofil::datei`, baut die Erkennung aus `leseprofil::erkennung` nach und rechnet jeden
Baustein am echten Bestand dieser Werkbank. Gezählt ist, nicht gelesen.

---

## Summary

Die Datei trägt fünf Profile, alle Muster übersetzen, jede Zeile nennt genau einen Baustein, jedes
Feldmuster trägt genau eine Fanggruppe. Die zwei Änderungen aus `b5bf2e3` wirken beide wie
zugesagt, und die Zahlen des `ontocoder` sind an jeder Stelle nachgerechnet und stimmen. Acht
Befunde stehen daneben, sieben davon in den Kommentarzeilen, einer in einem Feldmuster. **Keiner
hält die Runde auf.**

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 6 |

---

## Was geprüft und in Ordnung ist

**Alle 21 regulären Ausdrücke der Datei sind am echten Bestand gemessen.** Fünf Erkennungsmuster,
sechs Feldmuster mit ihren sechs Dateimustern, vier Zählmuster und sechs Vorhandenseinsmuster; die
Erkennung lief über alle 154 Verzeichnisse unter `fusion-workbench/`.

| Profil | erkannte Ordner |
|---|---|
| die Wurzel | 1 |
| ein Speicher | 99 |
| ein Defektspeicher | 19 |
| alle Runden | 1 |
| eine Runde | 18 |
| ohne Profil | 16 |

Die sechs Feldbausteine treffen: `Projekt` → `krk`, `Eingerichtet` → `2026-08-24T14:40:36+0200`,
`fusion-Fassung` → `10.6.0`, `Aktive Runde` → der Name des aktiven Circles,
`Sitzung` → die erste Zeile unter `## Current`, `Directive` → **18 von 18** Circle-Datensätzen. Die
zwei Muster, die der Befund `issues/260824-1124_*_zwei-feldmuster-…` als nie treffend nachgewiesen
hatte, sind berichtigt und treffen jetzt; die Warnung dazu steht in der Datei
(`resources/default-readers.toml:112-117`) und ist sachlich richtig.

Die Zählungen stimmen gegen die Kriterien: `shared/issues` 54 offen, `circles` 18 Runden,
`shared/history` 118 Datensätze, wie C5.1, C5.2, C5.4 und C5.5 es angeben.

**Die zwei Änderungen aus `b5bf2e3`, einzeln nachgerechnet.**

1. *Die vierte Zustandszeile.* Über die achtzehn Rundenverzeichnisse antworten die vier Zeilen
   0 / 1 / 15 / 2 mit „ja", zusammen 18. **Jedes einzelne Verzeichnis bejaht genau eine Zeile**,
   keines zwei und keines null. Die Markerverteilung im Bestand ist `_b_` 10, `_c_` 5, `_d_` 2,
   `_t_` 1. Die Zahlen des `ontocoder` sind bestätigt.
2. *Die neun Namen im Pfadmuster.* Innerhalb des Rahmens, den die Commit-Nachricht nennt (die 118
   Unterordner unter `shared/` und in den achtzehn Runden), bleibt **kein** Speicher ohne Profil:
   99 + 19 + 0 = 118, vorher 78 + 19 + 21 = 118. Auch diese Zahlen stimmen. Außerhalb des Rahmens
   liegen sechs Speicher unter `archive/`, dazu der Befund 7 unten.

**Der Schreibfehler-Kommentar stimmt, an der Datei gemessen.** `resources/default-readers.toml:29-34`
sagt, ein verschriebener Schlüssel in einem Baustein koste die ganze Datei und die Meldung nenne
den Schlüssel. Nachgemessen durch Verändern je einer Stelle und Neulesen: `zahlung` statt
`zaehlung` und `feldmusster` statt `feldmuster` werden beide abgewiesen, und beide Meldungen nennen
den Schlüssel samt der erwarteten Namen. Auch die drei Reichweiten aus `:36-49` decken sich mit
`leseprofil::datei`.

**Der Haushalt hält, und die Grenze ist genau erreicht.** Siehe `## Der Haushalt` unten.

---

## Befunde nach Thema

### Thema 1: das, was die Zusammenfassung zeigt, stimmt nicht mit dem Bestand überein

**B1 (Medium) — Der Defektspeicher zählt zwei von vier Markern, und vier Datensätze fallen durch.**
`resources/default-readers.toml:228` und `:233-239`. Gemessen über die neunzehn Defektspeicher:
163 `_o_`, 439 `_c_`, **4 `_d_`**, 0 `_p_`. Für `shared/issues` zeigt das Profil 54 und 27,
zusammen 81 von 82 Datensätzen; der eine fehlende trägt `_d_`. Das Profil führt keine Gesamtzahl,
also ist die Lücke unsichtbar. Der Kommentar nennt zwei Marker, das Vokabular hat vier
(`rules/fusion-workbench-conventions.md` `## State Markers — issues and planning`).
Datensatz: `issues/260824-1649_o_der-defektspeicher-zaehlt-zwei-von-vier-markern-…`

**B2 (Medium) — Die Zeile „Sitzung" liefert bei leerem `## Current`-Abschnitt die nächste
Überschrift.** `resources/default-readers.toml:193`. Das gierige `\s*` im Muster
`(?s)## Current\n\s*(.+?)\n` überspringt die Leerzeile und fängt `## This Turn`. Zugesagt ist für
diesen Fall der Platzhalter (`:168-170` und C5.8). Ein zweiter Ausgang ist verkehrt herum falsch:
steht `## Current` als letzter Abschnitt ohne Zeilenende am Dateiende — die Gestalt, die
`skills/setup/SKILL.md:107-114` schreibt —, trifft das Muster nicht, obwohl der Wert dasteht.
Wie oft die zwei Gestalten wirklich vorkommen, ist **nicht** gemessen: `orchestrator-live.md` steht
in `.gitignore:16`, es gibt keine Aufzeichnung.
Datensatz: `issues/260824-1650_o_die-zeile-sitzung-liefert-bei-leerem-current-abschnitt-…`

**B6 (Low) — Die Verlaufszeile des Rundenprofils trägt kein `muster` und listet eine `.gitkeep`.**
`resources/default-readers.toml:302-304`. Genau ein Fall im Bestand:
`circles/260804-0933-…/history/` führt allein `.gitkeep`, 0 Bytes, und die Zeile zeigt den
Dateinamen als Verlaufstitel. Die zwei anderen `juengste`-Zeilen und das Beispiel im
Kommentarkopf tragen `muster = '\.md$'`.
Datensatz: `issues/260824-1654_o_die-verlaufszeile-des-rundenprofils-traegt-kein-muster-…`

### Thema 2: Kommentarzahlen, die der Bestand nicht trägt

**B3 (Low) — Der Kopf des Speicherprofils nennt achtzehn Orte und neun Speicher je Runde.**
`resources/default-readers.toml:205-206`. Gemessen: 99 Ordner, davon 9 unter `shared/` und 90 in
den achtzehn Runden, also **fünf** je Runde. Vier der neun Namen — `backlog`, `consult`,
`investigations`, `memos` — können in einer Runde nicht stehen:
`rules/fusion-workbench-conventions.md:78` beschränkt sie auf `shared/`, gemessen null Vorkommen.
Die Zahl ist mit `b5bf2e3` von „zwölf" auf „achtzehn" nachgezogen worden und nach demselben
Rechenweg falsch geblieben.
Datensatz: `issues/260824-1651_o_der-kopf-des-speicherprofils-nennt-achtzehn-orte-…`

**Keine andere Zahl in den Kommentaren ist falsch.** Einzeln gehalten: „fünf Profile" (`:19`,
`:33`) stimmt; „vier Bausteine" (`:59-62`, `:76`) stimmt; die sechs Zahlen des Haushalts (`:143-147`)
stimmen gegen `HOECHSTENS_LESELAEUFE` 12, `HOECHSTENS_OEFFNUNGEN` 24, `HOECHSTENS_EINTRAEGE` 2000,
`HOECHSTENS_BYTES` 64 KB, `HOECHSTENS_JUENGSTE` 10; „die sechs übrigen" der Wurzelzusammenfassung
(`:170`) stimmt gegen sieben Zeilen; „die vier Zeilen decken die sechs Marker" (`:262-265`) stimmt
gegen das Vokabular der Circle-Datensätze; „seine fünf Marker" für `decisions` (`:210`) stimmt.
Die zwei mit `b5bf2e3` nachgezogenen Köpfe sind also einer richtig und einer falsch.

### Thema 3: was die Datei dem Nutzer für die eigene Änderung schuldet

Der Datensatz `issues/260824-1242_*_die-kommentarzeilen-der-auslieferungsfassung-sagen-nicht-dass-ein-schreibfehler-die-ganze-datei-kostet.md`
ist geschlossen, und die Warnung steht: `:25-49`, drei Reichweiten, an der Datei nachgemessen und
richtig. Drei Lücken bleiben daneben.

**B4 (Low) — Ein abschließender Schrägstrich in `ordner` kostet die Zeile, und die Aufzählung nennt
ihn nicht.** `resources/default-readers.toml:70-73` nennt „ein absoluter Pfad, ein `..` und ein
doppelter Schrägstrich". `Ortsangabe::aus_angabe` weist fünf Formen ab; die zwei ungenannten sind
`.` und der **abschließende** Schrägstrich, und `ordner = "planning/"` ist keine Verschreibung,
sondern eine gewöhnliche Schreibgewohnheit. Nachgemessen: die Zeile verliert ihren Baustein und
zeigt `--`.
Datensatz: `issues/260824-1652_o_ein-abschliessender-schraegstrich-in-ordner-…`

**B5 (Low) — Zwei Bausteinbeschreibungen sagen weniger, als der Baustein tut.** `feld` (`:96-99`)
verschweigt, dass ein Dateimuster mit mehreren Treffern keine bestimmte Datei wählt; `vorhandensein`
(`:105-106`) sagt „ja" oder „nein" und lässt den dritten Ausgang weg, den ein fehlender `ordner`
erzeugt. Die fünf ausgelieferten Dateimuster sind an beiden Enden verankert, die Datei selbst also
in Ordnung; der Satz, der den Nutzer davor bewahrt, steht im Rust-Quelltext und nicht in der Datei,
die er liest.
Datensatz: `issues/260824-1653_o_zwei-bausteinbeschreibungen-sagen-weniger-als-der-baustein-tut.md`

**B8 (Low) — Der Kopf braucht das Wort „Ablage" in zwei Bedeutungen.** `:4` meint KRKs Bestandsort,
`:19` die Werkbank von fusion. Dazu `:7`, „die zweite Datei der Ablage, die von Hand gepflegt
wird": richtig unter der Definition aus `ablage/mod.rs:59-63`, für den Nutzer aber die dritte, denn
`keymap.toml` ändert er ebenfalls von Hand.
Datensatz: `issues/260824-1656_o_der-kopf-der-auslieferungsfassung-braucht-das-wort-ablage-…`

### Thema 4: Reichweite der Erkennung

**B7 (Low) — Sechs Speicher unter `archive/` bleiben ohne Profil.** Von den 16 Verzeichnissen ohne
Profil sind sechs Speicher, verschoben vom Archivschritt von `/fusion:cleanup`; die übrigen zehn
sind Hüllen und `.guard-state`-Ordner und sollen keines bekommen. Ob das ein Mangel ist, ist eine
Frage und keine Feststellung: `rules/fusion-workbench-conventions.md:48` führt `archive/` nicht als
Speicher neben den übrigen. Der Datensatz legt beide Seiten hin.
Datensatz: `issues/260824-1655_o_sechs-speicher-unter-archive-bleiben-ohne-profil-…`

---

## Der Haushalt

Nachgerechnet je Profil, schlimmster Fall:

| Profil | Zeilen | Leseläufe | Öffnungen |
|---|---|---|---|
| die Wurzel | 7 | 3 | 5 |
| ein Speicher | 2 | 1 | 10 |
| ein Defektspeicher | 3 | 1 | 10 |
| alle Runden | 1 | 1 | 0 |
| **eine Runde** | 9 | **5** | **11** |

Die fünf Leseläufe des Rundenprofils sind: der erkannte Ordner einmal, `planning` **zweimal** (die
Zeilen „Spec" und „Plan" nennen denselben Unterordner, und ein Baustein mit `ordner` kostet je
einen Leselauf), `decisions` und `history` je einmal. Die elf Öffnungen sind: der Circle-Datensatz
für die Directive, dazu zehn Verläufe.

**Zur ausdrücklichen Frage: ja, eine weitere Zeile in diesem Profil bricht die Zusage, wenn sie
eine Öffnung kostet.** C6.7 sagt höchstens 7 Leseläufe und höchstens 11 Öffnungen zu. Bei den
Leseläufen sind zwei frei, bei den Öffnungen keine. Ein zusätzlicher `feld` oder ein zweiter
`juengste` in diesem Profil bricht C6.7 sofort; eine `zaehlung` oder ein `vorhandensein` ohne
`ordner` kostet nichts, mit `ordner` einen der zwei freien Leseläufe.

**Eine Einladung dazu spricht die Datei nicht aus, und die Zusage ist bewacht.** Der Kommentarblock
`:136-155` nennt die Grenzen 12 und 24 aus C6.4, und das ist für den Nutzer die richtige Auskunft:
C6.7 bindet die **ausgelieferte** Datei und nicht die, die der Nutzer danach pflegt. Die
ausgelieferte hält
`crates/krk-core/tests/leseprofil.rs:2131-2141`, und zwar mit `assert_eq!((5, 11))` und nicht mit
`<=`: der Kommentar über der Probe schreibt aus, warum. Eine zehnte Zeile im Rundenprofil färbt
diese Probe rot, bevor sie irgendwohin ausgeliefert wird.

Der einzige Satz, der in diese Richtung zeigt, steht bei einem anderen Profil und ist unschädlich:
`:211-213` lädt dazu ein, `decisions` aus der Aufzählung zu nehmen und ihm ein eigenes Profil nach
dem Vorbild des Defektspeichers zu geben. Nachgerechnet kostete das einen Leselauf und zehn
Öffnungen, weit unter 12 und 24, und ließe das Rundenprofil unberührt.

---

## Querliegende Beobachtungen

**Sieben der acht Befunde liegen in Kommentarzeilen und keiner in der Erkennung.** Das ist kein
Zufall der Stichprobe: die Muster und Bausteine sind in dieser Runde zweimal gegen den Bestand
gemessen worden, die Prosa daneben nicht. Eine Datei, die zur Hälfte Kommentar ist und deren Zweck
genau diese Hälfte ist, braucht für ihre Prosa dieselbe Messung wie für ihre Muster.

**Ein bereits offener Datensatz widerspricht einer richtigen Kommentarzeile dieser Datei, und die
Datei ist nicht die falsche Seite.** `:31-32` sagt, KRK arbeite nach einer beschädigten
`readers.toml` „ohne jedes Profil weiter". Das stimmt. Die Meldung, die der Nutzer dazu in der
Statuszeile liest, sagt „und wird durch den Auslieferungszustand ersetzt" und stimmt nicht; das
hält `issues/260824-1508_o_die-meldung-einer-ersetzung-verspricht-den-auslieferungszustand-…`
fest, offen und dem `coderev`-Bereich zugehörig. Kein zweiter Datensatz von hier.

**Zwei Zahlenpaare der Runde sind unabhängig bestätigt.** Die vier Zustandszeilen (0/1/15/2) und
die Speicherzählung (99/19/0 von 118) hat der `ontocoder` gemessen und diese Durchsicht mit einem
eigenen Programm nachgerechnet. Beide Male stimmt jede Stelle. Die Aufforderung, sie nicht zu
übernehmen, hat nichts zutage gefördert — außer dem Rahmen, den die zweite Zahl trägt und der
`archive/` nicht enthält.

---

## Empfohlene Reihenfolge

**Nichts davon hält die Runde auf.** Kein Befund berührt ein Abnahmekriterium: C5.1 bis C5.10 und
C6.1 bis C6.9 sind an dieser Datei gemessen und halten.

1. **Vor dem Schließen der Runde**, weil beide eine Zusammenfassung falsch zeigen und beide zwei
   Zeilen kosten: B1 (Defektspeicher) und B2 (Zeile „Sitzung"). B1 berührt die Gestalt eines
   ausgelieferten Profils, also gehört die Wahl zwischen „Datensätze" und „Zurückgestellt" dem
   Nutzer.
2. **Mit derselben Hand, wenn eine ohnehin an die Datei geht**: B3, B4, B5, B6, B8. Fünf
   Kommentarberichtigungen und ein `muster = '\.md$'`.
3. **Als Frage an den Nutzer und nicht als Arbeit**: B7. Ob die Speicher unter `archive/` ein
   Profil bekommen sollen, ist eine Entscheidung und kein Mangel.
