# Acht Befunde aus zwei Durchsichten, und fünf abhängige Stellen dazu

**Date:** 2026-08-23 11:37
**Status:** Complete
**Agent:** coder
**Grundlage:** acht Defektdatensätze in `shared/issues/`, Marker `260823-0730`, `-0733`,
`-1031` bis `-1036`. HEAD beim Beginn: `471d801`.

## Was getan ist, je Datensatz

| Datensatz | Sache | was jetzt gilt |
|---|---|---|
| `1031` | `editor_rundweg` erhob den Fokus ein zweites Mal | nimmt `fokus: Fokus` als Argument, wie die drei Geschwister im selben `match` |
| `0730` | drei Prosastellen um den Nachzug aus `df8163d` | alle drei nachgezogen, zwei Zählungen durch Regeln ersetzt |
| `1032` | zwei Zahlen im Modulkopf der Kommandos | gestrichen statt korrigiert; die Aussage ist die Reihenfolge |
| `1033` | drei Stellen behaupten, `false` gebe den Tastendruck weiter | am Baum nachgeprüft, trifft zu; alle drei richtiggestellt |
| `1034` | `vorschau_danach` hielt keine Probe | vier Quelltextproben in `mod rundwegproben` |
| `0733` | die Probe deckte einen von zwei Zweigen | prüft jetzt die Stellung statt einer Reihenfolge |
| `1035` | der Rückweg blendet die Vorschau zu breit ein | kein Verhalten geändert, Frage als Entscheidungsdatensatz vorgelegt |
| `1036` | zwei Proben schreiben `editor_rundweg` der falschen Runde zu | Zuschreibung richtiggestellt, nicht getilgt |

## Die Reihenfolge war nicht beliebig

`1031` zuerst. Der Fix, `editor_rundweg` den einmal erhobenen Fokus als Argument zu geben,
macht die Aufruferzahl an `fokus_bei` von selbst wieder richtig: `self.fokus()` hat wieder
genau fünf Aufrufer (`anwendung.rs` Zeilen 1353, 1898, 4334, 6474, 6966), und die Prosa
dort sagt „fünf". Wer die Zahl zuerst auf sechs gesetzt hätte, hätte sie danach wieder
zurücksetzen müssen. Die drei Prosa-Datensätze sind nach den Codeänderungen gezogen.

## Die Gestalt, um die es ging: fünf abhängige Stellen

Der Auftrag verlangte, nicht nur die genannten Stellen nachzuziehen, sondern nach den
jeweils abhängigen zu suchen. Gefunden und mitgezogen:

**Zu `0730`** (dieselbe Menge, dieselbe Verschiebung durch `df8163d`):

1. Der Doc-Kommentar von `bildschirmbreiten_uebernehmen` sagte „Zwei Anlässe tun das" und
   zählte `kommando_ausfuehren` und `sitzung_bauen`. `df8163d` hat mit
   `editorausgang_behandeln` den dritten hinzugefügt; `git log -L 6435,6435` weist die
   Zeile jenem Commit zu. Der Satz trägt jetzt die Regel und keine Zahl.
2. Der Kommentar im Rumpf von `sitzung_bauen` nannte sich „der zweite der **beiden**
   Anlässe". Er verweist jetzt auf die Regel.

**Zu `1033`** (dieselbe falsche Aussage über denselben Rückgabewert):

3. `terminal_oeffnen` und `weitere_instanz_starten` in `anwendung.rs`, beide mit dem Satz
   „Ein `false` gäbe den Tastendruck an AppKit weiter".
4. Beide Leerwege von `bereichskommando`, die Zweige `Fokus::Vorschau` und `Fokus::Editor`.
   Der Editorzweig ist der folgenreichste: er behauptete, ein `Wirkungsbereich::Ueberall`
   werde mit dem Fokus im Editor zu einem Zeichen in der Textfläche. Er wird geschluckt,
   und zwar seit der Runde 7 und mit Absicht. **Kein Defekt, sondern eine Wahl, die der
   Kommentar falsch wiedergab.**
5. `umbenennung_beginnen` in `appkit/tabelle.rs`, auf demselben Kommandoweg über
   `Tabelle::kommando_ausfuehren`.

**Geprüft und nicht angefasst**: die Aussagen auf dem Zeichenweg
(`Ordnermodell::zeichen_anhaengen`, `::letztes_zeichen_weg`,
`Tabelle::filterzeichen_tippen`, der Zweig `Eingabe::Zeichen` in `eingabe_ausfuehren`).
Dort wird der Rückgabewert wirklich bis zum Abgriff durchgereicht, und die Prosa stimmt.

Für `1032` gibt es keine abhängige Stelle: `grep -rn "Module" crates/krk-ui/src/kommandos/`
liefert allein die Zeile „Zehn Module" im Modulkopf, und die steht unmittelbar über der
Aufzählung dieser zehn, ist also ihr eigener Beleg.

## Was `1033` behauptet, und ob es stimmt

Es stimmt. `Anwendungsdelegierter::kommando_ausfuehren` endet auf ein nacktes `true`, und
der Modulkopf von `appkit/ereignisse.rs` schreibt die Regel aus: „Geschluckt wird, was
zulässig war, und nicht mehr, was gewirkt hat. Bis zur Runde 7 lautete die Grenze
‚ausgeführt'."

## Die neuen Proben, und dass sie auslösen

`mod rundwegproben` in `anwendung.rs`, neben `sichtbarkeitsproben` und über denselben Weg
(`zettelproben::{diese_datei, rumpf}`):

- `opt_cmd_e_schliesst_ohne_die_vorschau_danach`
- `der_rueckweg_schliesst_mit_der_vorschau_danach`
- `die_abgelehnte_nachfrage_liest_das_feld_nicht`
- `die_ausgefuehrte_nachfrage_liest_das_feld` (die Gegenprobe zur Verneinung)

Alle vier zum Auslösen gebracht: die zwei Wahrheitswerte vertauscht, den
`if vorschau_danach`-Block entfernt, `{ .. }` durch `{ vorschau_danach }` ersetzt. Vier von
vier rot. Danach aus der Sicherung zurückgestellt.

`die_editorfortsetzung_misst_als_erste_anweisung` ersetzt
`die_editorfortsetzung_misst_vor_dem_einblenden`. Zum Auslösen gebracht, indem die Messung
aus der ersten Zeile des Rumpfs in den Zweig `Geoeffnet | SchonOffen` wanderte. Die neue
Probe wird dabei rot; **die alte wäre grün geblieben**, an demselben Baumstand
nachgerechnet: Messung bei Zeichen 376, `fokus_holen(` bei 468. Der Datensatz `0733` trifft
damit zu, und zwar gemessen und nicht geschlossen.

## `1035` ist keine Codeänderung geworden

Der Datensatz bezeichnet sich selbst als Frage an den Nutzer, und die Zeile steht so, wie
der Entscheid vom 260823-0942 sie formuliert. Möglichkeit 2 des Datensatzes braucht einen
gemerkten Zustand, und beim Ausschreiben der Folgen zeigt sich ein Einwand, den der Befund
noch nicht führt: **„der Hinweg" ist gar nicht wohldefiniert.** Der Fokus kommt auch über
`f4`, über `opt+cmd+b` und über die Wiederherstellung aus der Sitzung in den Editor, und
aus jeder dieser Lagen ist der Rückweg erreichbar. Entweder merken sich alle diese Wege die
Sichtbarkeit, oder der Rückweg findet keinen oder einen veralteten Wert vor.

Vorgelegt als `shared/decisions/260823-1137_o_holt-der-rueckweg-von-cmd-e-die-vorschau-auch-dann-zurueck-wenn-der-nutzer-sie-selbst-ausgeschaltet-hatte.md`.
Geändert ist allein, was der Code über die Zeile behauptet: die Begründung „Umkehrung eines
Hinwegs, der sie verdrängt hat" trägt nur für einen Teil der Fälle, und beide Prosastellen
sagen jetzt die Regel, die wirklich gebaut ist.

## Berührte Dateien

- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/kommandos/mod.rs`
- `crates/krk-ui/src/kommandos/rundweg.rs`
- `crates/krk-ui/src/belegungsmodell.rs`
- `crates/krk-core/tests/belegung.rs`

## Prüfung

`make check` — exit 0 (Bau, Proben, `fmt --check`, `clippy --workspace --all-targets --
-D warnings`).

**Ein Lauf von neun ist mit 2 abgebrochen und hat sich nicht wiederholt.** Seine Ausgabe
ist nicht erhalten: er lief nach `/dev/null`, weil allein der Rückgabewert gebraucht war.
Acht weitere Läufe am selben Baumstand sind grün, vier davon `cargo test --workspace`
einzeln. Als eigener Datensatz festgehalten:
`shared/issues/260823-1210_o_ein-make-check-von-neun-ist-mit-2-abgebrochen-und-hat-sich-nicht-wiederholt.md`.

## Was `CLAUDE.md` betrifft

Nichts. Keine Aussage dort hängt an einer der berührten Stellen; die Datei ist nicht
angefasst.

## Was der Nutzer von Hand nachsehen muss

Nichts am Verhalten: `cmd+e`, `f4`, `opt+cmd+e` und `opt+cmd+b` sind unverändert. Offen ist
allein die Entscheidung zu `1035`.
