# Abgleich 260807-1022 — vor der Entscheidung über den Abschluss der Runde 1

**Sitzung:** 260806-2257 (Turns 25 und 26), Stand `710ce84`
**Circle:** `260802-0842-krk-mac-dateimanager-editor-git`
**Domäne:** code
**Status:** Complete

## Umfang

| Größe | Geprüft | Geändert |
|---|---|---|
| Pläne | 2 (Plan und Spec der Runde 1) | 1 (Abgleichseintrag im Plan) |
| Planschritte | 38 | 0 |
| Entscheidungsdatensätze | 39 (34 im Circle, 5 in `shared/`) | 6 Statuskopfzeilen |
| Defekte | 155 (151 geschlossen, 3 offen, 1 zurückgestellt) | 0 Marker, 3 neu angelegt |
| Durchsichten | 8 | 0 |
| Messberichte | 14 | 0 |

Der Bau ist grün: `cargo test --workspace` am Stand `710ce84` beendet mit 0.

## Was der Dateibestand sagt

### Der Plan trägt seine 38 Schritte zu Recht

Ausgezählt an den Überschriften der Implementierungsschritte: S1 bis S23 mit den
fünfzehn Nachträgen S4b, S6b, S9b, S11b, S11c, S13b, S13c, S16b, S16c, S17b,
S17c, S18b, S18c, S19b und S19c. Alle 38 tragen `[DONE]`, keiner `[IN PROGRESS]`,
keiner ist unmarkiert.

**S19b und S19c sind nicht nur markiert, sondern erfüllt.** Beide sind am
260807 entstanden und in derselben Sitzung abgenommen worden, was den Verdacht
nahelegt, der Marker sei mit dem Schritt zugleich gesetzt worden. Er ist es
nicht:

- S19b verlangt `Kommando::FokusVorschau` mit der Kennung `fokus_vorschau` und
  dem Wirkungsbereich `Ueberall`, genau eine Stelle, die einem Fokusbefehl
  seinen Bereich hervorholt, und genau eine, die einblendet ohne je
  auszublenden. Belegt: `crates/krk-core/src/tasten/belegung.rs:295` (Variante),
  `:363` (Kennung in `KENNUNGEN`), `:429` (Wirkungsbereich bei den beiden
  anderen Fokusbefehlen); `crates/krk-ui/src/kommandos/fokus.rs` (Hervorholen),
  `crates/krk-ui/src/appkit/anwendung.rs:1510` (Weiche),
  `crates/krk-ui/src/belegungsmodell.rs:198` (Funktionsbereich).
- S19c verlangt einen 58. Eintrag mit `shift+cmd+y` und die auf 58 und 65
  gezogenen Zahlen im Kopfkommentar. Belegt: `resources/default-keymap.toml:347`
  trägt `id = "fokus_vorschau"`; `grep -c '^tasten = .*shift+cmd+y'` liefert 1
  wie vorgeschrieben; `grep -c '^\[\[funktion\]\]'` liefert 58; Zeile 30 des
  Kopfkommentars nennt "58 Funktionen mit zusammen 65 Kombinationen".
- Das gemeinsame Kriterium beider, `cargo test --workspace` mit 0, ist am Stand
  `710ce84` erfüllt.

Der Marker des Plans bleibt `_o_` und die Statuszeile "In Arbeit". Die Bedingung
hat gewechselt, nicht die Lage: die L9-Frage, die die Runde offen hielt, ist
beantwortet und umgesetzt; offen ist der Abnahmelauf am gebauten Bündel. Über
den Abschluss befindet der Nutzer.

### Die fünf Entscheidungen tragen ihren `_i_`-Marker

Drei belegen mit einem Commit und sind am Code nachgeprüft:

- `260805-1730` (Fokusbefehl holt die Leiste hervor) → `9a47c4a`,
  `Fenstermodell::einblenden` trägt die Asymmetrie an einer Stelle.
- `260805-2216` (Tastenweg des Fokus in die Vorschau) → `9a47c4a`, siehe S19b
  und S19c oben.
- `260806-0014` (L9) → `d569f8a`, `Abnahmemass::AnteilImBild` trägt seither
  Bildlänge, Mindestanteil und Obergrenze; die Konstante
  `ANTEIL_IM_BILD_PROZENT` ist im ganzen Baum verschwunden.

**Zwei lauten "so lassen" und belegen mit einem Pfad statt einem Commit.** Der
Abgleich trägt diese Lesart mit, und zwar aus dem Wortlaut der Konvention: `_i_`
heißt "code or data on disk now reflects the decision", nicht "eine Änderung war
dafür nötig". `_a_` dagegen heißt ausdrücklich "not yet realised in code or
data" — und genau das trifft hier nicht zu.

- `260805-1845` (`settings.toml` wird einmal beim Start gelesen): nachgeprüft,
  `crates/krk-core/src/ablage/einstellungen.rs` hat genau einen Lesepfad, und
  S18c hält den Wert am Anwendungsdelegierten. Der Zustand auf der Platte ist
  die Antwort.
- `260805-2252` (kein Entfernen-Befehl in der Belegungsansicht): nachgeprüft,
  `crates/krk-ui/src/appkit/belegungsansicht.rs` weist zu, setzt zurück und
  speichert, und kennt keinen Entfernen-Befehl.

Beide Datensätze sagen den Grund selbst und schreiben ihn nicht klein: "Der
Marker steht auf umgesetzt, weil der Zustand auf der Platte die Entscheidung
trägt, nicht weil etwas gebaut worden wäre." Das ist die ehrliche Formulierung.
Ein `_a_` wäre hier falsch, weil es einen ausstehenden Umsetzungsschritt
behauptete, den es nicht gibt.

**Eine Einschränkung, die den Marker nicht kippt.** Beide `Implemented:`-Zeilen
nennen den Pfad ohne Zeilennummer; die Konvention sieht `<path>:<line>` vor. Die
Aussage stimmt trotzdem, und beide Dateien sind klein genug, dass der Verweis
trägt.

### Sechs Statuskopfzeilen liefen gegen ihren Dateinamen

Alle sechs sind richtiggestellt, von `answered` beziehungsweise `open` auf
`implemented`:

| Datensatz | war | ist |
|---|---|---|
| `260802-1810_i_sortierung-ohne-sprachsensitive-kollation` | open | implemented |
| `260805-1730_i_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor` | answered | implemented |
| `260805-1845_i_wann-eine-von-hand-geaenderte-settings-toml-wirkt` | answered | implemented |
| `260805-2216_i_tastenweg-des-fokus-in-das-vorschaufenster` | answered | implemented |
| `260805-2252_i_entfernen-einer-einzelnen-kombination-in-der-belegungsansicht` | answered | implemented |
| `260806-0014_i_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet` | answered | implemented |

Der erste ist der ältere Fall: er steht seit dem 260806 auf `_i_` mit
Commit-Beleg `16e4558`, und die Kopfzeile blieb dabei auf `open` stehen. Der
Dateibestand insgesamt: **31 `_i_`, 8 `_o_`, kein `_a_`**.

### Die offenen Defekte stehen zu Recht offen

Drei tragen `_o_`, einer `_d_`:

- `260806-1304` (Sitzungslauf bei L6): `81d10c1` hat den verworfenen
  Auswahlversuch zum Abbruch gemacht, und der zweite Verdacht ist am
  Programmtext ausgeräumt. Welcher der beiden Fälle der Abbruch vom 260806 war,
  beantwortet erst der nächste vollständige Sitzungslauf. Offen zu Recht.
- `260807-0219` (drei Aufrufer werfen den Auswahlversuch weg): `5d7e299` hat
  zwei der drei Stellen ausgeräumt; die dritte, `vorgang_beenden` beim
  Stapel-Umbenennen, bleibt und verlangt einen Nutzerentscheid, weil die
  denkbare Meldung den Nutzer in einem Ordner träfe, über den er nichts wissen
  wollte. Offen zu Recht. **Randnotiz zur Ablage:** der Punkt ist inzwischen
  eher ein Entscheid als ein Defekt; er ist hier belassen, weil der
  Ausgangsbefund ein weggeworfener Rückgabewert ist.
- `260807-0930` (Meldung zur Bündelkennung): der ausgeschriebene Preis der
  Antwort auf `260805-1845`, nicht entschieden. Offen zu Recht.
- `260805-0000` (toter Netzpfad): `_d_` mit benanntem Auslöser, nachgeprüft
  gegen die vier Abnahmekriterien von C9, von denen keines den Fall verlangt.

**Kein geschlossener Defekt hält eine Sache offen.** Nachgesehen für die vier
mechanisch prüfbaren Schließungen dieser Sitzung: `resources/Info.plist:82`
trägt `CFBundleDevelopmentRegion = de` (`260807-0745`); `resources/` enthält
keinen ausgeschriebenen Zustandsmarker mehr (`260807-0755`);
`crates/krk-ui/src/appkit/belegungsansicht.rs:78-86` nennt die Zahl der
Funktionen nicht mehr, weil die Konstante nie an ihr hing (`260807-1015`);
`resources/default-keymap.toml:347` trägt den Eintrag `fokus_vorschau`
(`260807-0922`).

### Die Zusage L9 lautet an allen vier Stellen gleich

Nachgeprüft Wort für Wort:

| Stelle | Wortlaut |
|---|---|
| Vorspann der Abnahmekriterien, Spec:355 | "statt der 95 Prozent im ersten Bild noch 85 Prozent, dazu jede Eingabe spätestens im zweiten Bild" |
| Zusagentabelle, Spec:367 | "jede Eingabe erscheint spätestens im zweiten Bild, mindestens 85 % im ersten" |
| Messvorschrift, Spec:382 | "mindestens 85 Prozent … erreichen das erste Bild, womit höchstens drei von zwanzig es verpassen dürfen, und keine einzige Eingabe liegt über zwei Bildlängen" |
| Auswertung, `crates/krk-bench/src/messen.rs:1117-1129` | `mindestanteil_prozent: 85, obergrenze_bilder: Some(2)` |

Auch die Ganzzahl-Regel deckt sich: "höchstens drei von zwanzig" gegen
`erreicht * 100 >= werte.len() * mindestanteil_prozent` in `Zusage::gehalten_in`
(ebd.:600), und "spätestens das zweite Bild" gegen `wert <= grenze` (ebd.:602),
was genau zwei Bildlängen noch hält.

**Der Fehler liegt woanders, und er ist gefunden.** Zwei Stellen des Plans
sagen, die Auswertung könne die neue Fassung nicht abnehmen — der Nachzug
260807-0832 im Kopf ("Offen bleibt daraus ein Defekt an der Messstrecke") und
`### Frage 5`, das die Konstante `ANTEIL_IM_BILD_PROZENT` als heutigen Zustand
nennt. Beides war 16 Minuten lang richtig und ist seit `d569f8a` falsch. Der
zitierte Defekt trägt `_c_`.

### `CLAUDE.md` — nachgezählt

| Behauptung | Befund |
|---|---|
| "alle 38 Schritte tragen `[DONE]`" | stimmt, ausgezählt |
| "Workspace mit vier Mitgliedern" | stimmt, `Cargo.toml` nennt vier |
| "Rust 1.97.1, beide Mac-Architekturen" | stimmt, `rust-toolchain.toml` |
| "Fünf Eigenschaften" in der Fallenliste | stimmt, fünf Absätze |
| "Drei Fallunterscheidungen … ohne Auffangzweig" | stimmt, keine der drei trägt `_ =>` |
| "Geprüft am 260807-1200" | **falsch**, der Commit trägt 260807-1011 |
| "Zwei binden künftige Arbeit" | **zu niedrig**, es sind fünf im Circle |
| `decisions/260802-1036_a_leistungszusagen-navigator.md` (Zeile 17) | **überholter Marker**, die Datei trägt `_i_` |

## Was für die Entscheidung über den Rundenabschluss zählt

**Der Abnahmestand der zehn Zeitzusagen ist ungleich, und die Ungleichheit ist
gewachsen.** Die Abnahmereihe `messungen/260805-2207-MacBookPro15-1-abnahme.txt`
stammt vom 260805 und liegt damit vor allen sechzehn Commits dieser Sitzung.

| Zusage | Messstand |
|---|---|
| L2 (erste Bildschirmseite, 10.000 Einträge) | **geteilt**: Kernanteil frisch (260807-0002), Zeichenanteil vom 260805-2207 |
| L3 (10.000 Einträge gelesen und sortiert) | **frisch**, 260807-0002, fünf Runden gehalten |
| L10 (100.000 Einträge) | **frisch**, 260807-0002, fünf Runden gehalten |
| L9 (Tastatur während laufender Kopie) | **nachgerechnet**, die 100 Einzelwerte vom 260805-2207 unter der neuen Regel, belegt als Prüfung in `crates/krk-bench/src/messen.rs:2179-2232`: gehalten in allen fünf Runden, Höchstwerte 1,15 bis 1,41 Bildlängen |
| L1, L4, L5, L6, L7, L8 | **vom 260805-2207**, nicht nachgemessen |

Die frische Messung vom 260807-0002 deckt genau eine Änderung ab, die Umstellung
der Lesestelle aus `5f2e45d`. Sie ist am Stand `2fbab30` gegen die Umstellung
gefahren worden, also **vor** den vier Commits, die danach kamen.

**Drei Änderungen dieser Sitzung liegen nach der letzten Messung auf einem Weg,
den eine der zehn Zusagen misst.**

- `880cb70` setzt `CFBundleDevelopmentRegion = de` in der `Info.plist`. Damit
  wechselt der `NSByteCountFormatter` der Größenspalte auf Deutsch — er läuft je
  sichtbarer Zeile, dazu in den Metadatenzeilen der Vorschau und im fünften Rang
  der Statuszeile. Berührte Zusagen: L2 (erste Bildschirmseite), L6 (Einstieg in
  den Unterordner), L7 (Vorschau), L8 (Fortschritt in der Statuszeile).
- `5d7e299` zieht die Auswahl am Namen nach `Tabliste::auswahl_auf_namen` und
  ändert damit den Weg, auf dem eine Auswahl gesetzt wird. Berührte Zusagen: L1
  (Tastendruck bis die Auswahl umspringt), L6 (misst Auswählen und Öffnen).
- `9a47c4a` erweitert die Aufzählung `Kommando` und die Weiche in
  `anwendung.rs`, durch die jeder Tastendruck läuft, und legt einen 58. Eintrag
  in die Auslieferungsbelegung. Berührte Zusagen: L1 und L9 (beide messen
  Tastendrücke).

Dazu, vor der Messung vom 260807-0002 und deshalb für L2-Kern, L3 und L10 schon
abgedeckt: `3e9613a` (Zelle der Spalte Typ, läuft je sichtbarer Zeile) und
`5f2e45d` (Lesestelle). Für L4, L5 und L6, die ebenfalls Ordner lesen, deckt die
Messung vom 260807-0002 nichts ab, weil sie kopflos fährt.

**Nicht berührt sind** `4db66ed` (Bauwerkzeug), `d569f8a` (Auswertung, kein
Laufzeitpfad), `81d10c1` (Messstrecke selbst) sowie die vier Dokumentcommits.

**In einem Satz:** drei der zehn Zusagen stehen auf frischen Zahlen, eine auf
nachgerechneten, sechs auf Zahlen von vor sechzehn Commits — und drei dieser
Commits haben Wege angefasst, die genau jene sechs messen. Ein Abnahmelauf am
gebauten Bündel schließt die Lücke; er verlangt KRK im Vordergrund aus einem
Terminalfenster und damit den Nutzer.

## Neu angelegte Defekte

- `issues/260807-1022_o_der-plan-fuehrt-den-messstrecken-defekt-an-zwei-stellen-noch-als-offen.md`
- `issues/260807-1022_o_zweiundzwanzig-verweise-in-lebenden-dokumenten-tragen-einen-ueberholten-zustandsmarker.md`
- `issues/260807-1022_o_claude-md-zaehlt-die-bindenden-offenen-fragen-zu-niedrig-und-nennt-eine-pruefzeit-die-noch-nicht-war.md`

## Randnotiz

`fusion-workbench/agentstate.yaml` steht auf dem Stand vom 260806-2325: Turn 25,
Aufgabe D2 laufend, eine Aufgabe erledigt, zwei Commits. Tatsächlich sind zwei
Turns gelaufen und sechzehn Commits gefallen. Der Datensatz gehört dem
Orchestrator und wird beim Sitzungsabschluss ohnehin fortgeschrieben oder
gelöscht; hier steht er nur, damit niemand ihn als Stand liest.

## Geänderte Dateien

- Sechs Entscheidungsdatensätze, je die Kopfzeile `**Status:**`
- `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Eintrag im
  Abschnitt `## Reconciliation Log`
- `history/260806-2257-orchestrator-session.md`, angehängter Abschnitt
  `## Coherence`
- Drei neue Defektdateien
