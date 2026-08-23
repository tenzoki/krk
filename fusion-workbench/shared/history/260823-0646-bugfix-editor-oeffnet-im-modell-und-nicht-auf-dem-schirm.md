# Bugfix: der Editor öffnet im Fenstermodell und nicht auf dem Schirm

**Date:** 2026-08-23 06:46
**Status:** Complete
**Trigger:** Zwei Defektdatensätze aus dem Abnahmelauf der Runde 14, dazu eine schwerere
Beobachtung des Nutzers vom 260823-0508

## Fehler

Zwei Datensätze, ein Ausführungszweig:

- `shared/issues/260820-1034_p_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md`
- `shared/issues/260820-1034_p_cmd-e-bleibt-in-der-vorschau-wirkungslos-und-ist-in-der-dateiliste-gar-nicht-belegt.md`

Beobachtet am Baumstand `ab11eb8`, Ausgangslage Fokus in der Dateiliste, Vorschau als
stehender Bereich der Fensterzeile:

1. `f4` drücken: der Fokus springt irgendwohin, meist in die Lesezeichenliste. Der Editor
   öffnet nicht, das Ankreuzfeld der Bereichsleiste steht weiter auf Vorschau.
2. Den Fokus von Hand zurück in die Dateiliste legen und die Zeilenmarkierung mit einer
   Pfeiltaste verschieben: jetzt öffnet der Editor.

`cmd+e` mit dem Fokus in der Vorschau: nichts.

Die Dateilisten-Hälfte des zweiten Datensatzes ist kein Defekt. `cmd+e` trägt
`Wirkungsbereich::Vorschau` (`crates/krk-core/src/tasten/belegung.rs:923`) und wird in der
Dateiliste zu Recht abgewiesen. Dort ist nichts geändert.

## Ursache

**`crates/krk-ui/src/appkit/anwendung.rs:4151`, `sichtbarkeit_aendern`: die eine Stelle, die
die Sichtbarkeit im Fenstermodell ändert, schrieb sie nicht auf den Schirm.** Den Schirm
erreichte allein `kommando_ausfuehren` mit seinem `aufteilung_nachziehen()` am Ende des
Befehlsrumpfs. Eine Fortsetzung, die außerhalb eines Befehlsrumpfs läuft, blieb den Schirm
schuldig.

Die Kette, jedes Glied am Baum gelesen:

| Glied | Beleg |
|---|---|
| Der einzige Schreiber der Bereichssichtbarkeit in die Ansichten ist `Aufteilung::anwenden` | `crates/krk-ui/src/appkit/aufteilung.rs:322-325`, `ansicht.setHidden(...)` |
| `anwenden` hat genau einen Rufer, `aufteilung_nachziehen` | `anwendung.rs:4482` (Stand vor der Korrektur) |
| Der hatte vier Aufrufstellen: Aufbau der Oberfläche, Ende von `kommando_ausfuehren`, `breite_aendern`, `anlass_ausfuehren` | `anwendung.rs:1292`, `:3184`, `:4274`, `:6762` |
| `editorausgang_behandeln` steht in keiner davon und ändert trotzdem die Sichtbarkeit, über `fokus_holen(Fokus::Editor)` → `bereich_einblenden` → `sichtbarkeit_aendern` | `anwendung.rs:6316` ff., Zweig `Geoeffnet \| SchonOffen` |
| Es läuft aus dem Einzugstakt des Editorbereichs, einem eigenen `NSTimer` im Takt von 1/60 s, und nicht im Rumpf des Befehls | `crates/krk-ui/src/appkit/editor.rs`, `datei_oeffnen` → `takt_starten` → `einziehen` → `ladeausgang_einziehen` → `melden`; der Melder ist bei `anwendung.rs:1069` eingetragen |

**Wann es entstanden ist.** `784840c` (2026-08-09, „das Getippte steht im Modell, und
gelesen wird auf dem Arbeitsfaden") hat das Lesen auf einen Arbeitsfaden gelegt und die
Fortsetzung aus `im_editor_oeffnen` nach `editorausgang_behandeln` herausgezogen. Davor
stand derselbe Rumpf **innerhalb** des `match`-Zweigs von `kommando_ausfuehren`
(`git show 784840c^:crates/krk-ui/src/appkit/anwendung.rs`, Zeilen 2946-2964), und der
Nachzug am Ende jener Funktion schrieb die neue Sichtbarkeit auf den Schirm.

Die Annahme, auf der der Baum seither stand, ist in `a6b3818` ausdrücklich formuliert:
„Entscheidend ist nach_dem_sichtbarkeitswechsel: es legt die Fensterzeile nicht neu aus, ein
eingeblendeter Bereich bekommt seinen Auslegungsdurchgang allein über kommando_ausfuehren."
Für die Fortsetzung des Editors trägt sie nicht.

**Was daraus folgt, Glied für Glied.** Das Modell führte den Editor als sichtbar und die
Vorschau als ausgeblendet, die Ansichten wussten davon nichts. Deshalb blieb die Fläche
zu und das Ankreuzfeld auf Vorschau. `fokus_setzen(Fokus::Editor)` (`anwendung.rs:2219`)
kam an seiner Sperre gegen ausgeblendete Bereiche **vorbei**, weil die das **Modell** fragt
und das schon umgestellt war, und rief `makeFirstResponder:` auf eine Textfläche, die AppKit
weiter als ausgeblendet führte. Der nächste ausgeführte Befehl (der Pfeiltastendruck) holte
den versäumten Nachzug nach und ließ die Fläche erscheinen.

**Der Verdacht des Ursprungsdatensatzes ist erhoben und widerlegt.** Er nannte die Sperre in
`fokus_setzen` als Ursache und die Reihenfolge in `editor_oeffnen_lassen` als Vermutung.
`fokus_holen` (`anwendung.rs:2069`) blendet den Bereich schon vor dem Fokussetzen ein, die
Reihenfolge im Modell war also nie verkehrt. Verkehrt war, dass zwischen beiden niemand den
Schirm anfasste.

`inference:` Dass der Fokus dabei „meist in der Lesezeichenliste" landet, ist am Code nicht
zu belegen; wohin AppKit den Ersthelferrang neu vergibt, wenn `makeFirstResponder:` auf eine
ausgeblendete Ansicht trifft, entscheidet AppKit und nicht dieser Baum. Für die Behebung ist
es ohne Belang: die Fläche steht jetzt auf dem Schirm, bevor der Rang vergeben wird.

`inference:` Dass der Editor beim nachholenden Pfeiltastendruck „eine andere Datei" zeigt,
erklärt sich daraus, dass die Auswahl inzwischen weitergewandert ist, während der Editor
den Pfad hält, den `f4` ihm gegeben hat. Der Pfad wird in `im_editor_oeffnen`
(`anwendung.rs:6194`) zum Zeitpunkt des Tastendrucks abgeschrieben; ein Weg, auf dem ein
Auswahlwechsel den Editor öffnet, besteht nicht (die vier Wege stehen am Doc-Kommentar von
`editor_oeffnen_lassen`).

## Behebung

Der Nachzug hängt jetzt an der Quelle statt an der Vollständigkeit einer Aufrufliste: wer
die Sichtbarkeit im Fenstermodell ändert, schreibt sie im selben Zug auf den Schirm. Damit
kann keine künftige Fortsetzung ihn vergessen. Er steht **vor** den Nachzügen der einzelnen
Bereiche, weil sowohl `nach_dem_sichtbarkeitswechsel` als auch `fokus_holen` unmittelbar
danach einen Ersthelfer setzen; das ist dieselbe Trennung, die `a6b3818` für das Angleichen
gezogen hat.

| Datei | Änderung |
|------|--------|
| `crates/krk-ui/src/appkit/anwendung.rs:4151` | `sichtbarkeit_aendern` ruft `aufteilung_nachziehen()`, nachdem das Modell geändert ist und bevor die Bereichsnachzüge laufen |
| `crates/krk-ui/src/appkit/anwendung.rs:6316` | `editorausgang_behandeln` beginnt mit `bildschirmbreiten_uebernehmen()`, wie `kommando_ausfuehren` es tut: gemessen wird, solange Modell und Schirm dieselbe Sichtbarkeit meinen |
| `crates/krk-ui/src/appkit/anwendung.rs:2069` | Doc von `fokus_holen`: die Fläche steht auf dem Schirm, bevor der Ersthelfer gesetzt wird |
| `crates/krk-ui/src/appkit/anwendung.rs:4548` | Doc von `bereichsleiste_nachziehen`: „auf jedem Weg genau einmal" ist nicht mehr wahr und steht jetzt als „mindestens einmal" da, mit dem Grund |
| `crates/krk-ui/src/appkit/anwendung.rs:6762` | Kommentar an `anlass_ausfuehren`: die Sichtbarkeit hängt dort nicht mehr an jener Zeile |
| `crates/krk-ui/src/appkit/anwendung.rs`, `mod sichtbarkeitsproben` | drei neue Proben, siehe unten |

**Der Preis, benannt statt versteckt.** Ein Befehl, der einen Bereich umschaltet, legt die
Fensterzeile jetzt zweimal aus: einmal in `sichtbarkeit_aendern`, einmal am Ende von
`kommando_ausfuehren`. Der zweite Ruf bleibt stehen, weil er die Änderungen abdeckt, die
keine Sichtbarkeit sind (Breiten, aktives Dateifenster), und findet ein unverändertes Modell
vor. Geschrieben werden beide Male dieselben Werte. Keine der zehn Zeitzusagen aus C8 misst
einen Sichtbarkeitswechsel.

## Prüfung

- [x] Der ursprüngliche Fehler ist an seiner Wurzel behoben (Codelektüre; der Abnahmelauf
      verlangt KRK im Vordergrund und ist Nutzerarbeit)
- [x] `make check` läuft grün: `cargo build --workspace`, `cargo test --workspace`,
      `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`.
      Rückgabewert 0.
- [x] Keine Rückschritte: alle bestehenden Proben laufen unverändert grün

**Drei Proben ohne Vordergrund**, in `mod sichtbarkeitsproben` neben `fokusnachzugproben`,
in der Bauform der Quelltextproben dieser Datei:

| Probe | Was sie hält |
|---|---|
| `die_geaenderte_sichtbarkeit_kommt_auf_den_schirm` | `sichtbarkeit_aendern` ruft den Nachzug überhaupt |
| `der_nachzug_steht_vor_den_bereichsnachzuegen` | er steht vor `nach_dem_sichtbarkeitswechsel` |
| `die_editorfortsetzung_misst_vor_dem_einblenden` | `editorausgang_behandeln` misst, bevor sie einblendet |

**Jede ist zum Auslösen gebracht worden**, wie es der Baum für Quelltextproben verlangt:
den Ruf entfernt (Proben 1 und 2 rot, Probe 3 grün), den Ruf hinter die Schleife gestellt
(Probe 2 rot, Proben 1 und 3 grün), die Messung entfernt (Probe 3 rot, Proben 1 und 2 grün).
Danach die Datei aus der eigenen Sicherung zurückgestellt.

**Was die Proben nicht sehen.** Sie messen die Verdrahtung und nicht das Bild. Dass die
Fläche danach wirklich auf dem Schirm steht und der Fokus darin, ist Nutzerarbeit; der Weg
von `setHidden:` bis zur gezeichneten Ansicht läuft durch ein `NSSplitView` und verlangt
KRK im Vordergrund.

## Nebenbefunde

Keine gefilt. Zwei Beobachtungen gehören zum Befund und nicht daneben:

- Die Fokusbefehle auf einen ausgeblendeten Randbereich (`FokusLeiste`, `FokusVorschau`,
  `FokusEditor`) trugen dieselbe Reihenfolgeschwäche, weil sie durch dasselbe `fokus_holen`
  laufen. Sie fällt mit derselben Korrektur.
- Der Zweig `Abgewiesen` aus der Sitzung in `editorausgang_behandeln` blendet den Editor
  über `editor_ausblenden` aus und blieb den Schirm aus demselben Grund schuldig. Auch er
  fällt mit derselben Korrektur.
