# S18, S19, S20 und S22: der Editor wird erreichbar und zeigt eine Datei

**Status:** Complete
**Agent:** coder
**Datum:** 260809-1921
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Plan:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Schritte 18, 19, 20 und 22

## Warum vier Schritte in einem Übersetzungsstand

Alle vier erweitern `crates/krk-ui/src/appkit/anwendung.rs`, und drei von ihnen
dieselben Funktionen. Einzeln gefahren hätten sie sich gegenseitig
überschrieben. Der Nutzer hat am 260808 entschieden, Planschritte dort ad hoc
zusammenzulegen, wo der Schnitt nicht trägt, statt den Plan neu zu schneiden.

Von S19 fehlte nur der `anwendung.rs`-Anteil; der `aufteilung.rs`-Anteil stand
seit dem 260808. `aufteilung.rs` trägt aus diesem Stand keine geänderte Zeile.

## Was umgesetzt ist

### S18: Editor und Vorschau schließen einander aus

Die Regel steht als **Zuordnung** und nicht als zwei Zweige.
`Bereich::teilt_flaeche_mit` nennt das Paar Vorschau/Editor einmal, vollständig
und ohne Auffangzweig. `Fenstermodell::umschalten` blendet danach das
Gegenüber aus, sobald ein Bereich sichtbar geworden ist; beide Richtungen
fallen damit aus einer Zeile an, und `einblenden` erbt sie, weil es durch
`umschalten` geht.

Dafür ist eine Schreibstelle entstanden, die es vorher nicht gab.
`Fenstermodell::sichtbar_setzen` ist seither die einzige Stelle, die ein Feld
von `Sichtbarkeit` schreibt, und die freie Funktion `sichtbar_in` die einzige,
die eines liest; `Fenstermodell::sichtbar` fragt dort nach. Ohne diese
Bündelung stünde der Ausschluss neben den vier Zuweisungen statt in ihnen.

Der dritte Satz des ersten Abnahmekriteriums von C1 — "Beide zugleich sichtbar
zu haben ist über keinen Weg erreichbar" — schließt `session.toml` ein.
`vorschau = true` neben `editor = true` liest `serde` anstandslos ein, und
`Fenstermodell::aus_sitzung` stellt die Zusicherung deshalb her, in derselben
Bauform, die es seit der Runde 1 für das ausgeblendete aktive Dateifenster
trägt. Weichen muss der Editor, aus demselben Grund, aus dem
`Sichtbarkeit::default` ihn ausblendet: er hält beim Start keine Datei.

In der Ansicht hängt der Nachzug seither an der **gemessenen** Änderung und
nicht am genannten Bereich. Ein Aufruf bewegt jetzt zwei Bereiche;
`nach_dem_sichtbarkeitswechsel` allein für den genannten zu fahren, ließe den
Fokus in einer Vorschau stehen, die niemand mehr sieht.
`Anwendungsdelegierter::sichtbarkeit_aendern` vergleicht deshalb die
Sichtbarkeit vorher gegen die nachher und zieht für jeden geänderten Bereich
nach. Der Ausschluss bleibt damit vollständig im Fenstermodell, und `appkit`
kennt ihn nicht: es erfährt sein Ergebnis.

Aus demselben Zug fällt die Literalliste `[Lesezeichen, Vorschau]` im
Fokusnachzug weg. Sie fragt jetzt `Bereich::seite`, die eine Stelle, die
aufzählt, welche Bereiche Dateifenster sind; der Editor wäre in der Literalliste
sonst stumm gefehlt.

### S19: die Breite des Editors

`breite_aendern` wirkt seit diesem Stand auf den Bereich mit dem **Fokus** und
nicht mehr fest auf das aktive Dateifenster. Die Änderungszeile des Plans
behauptete, das sei schon so; gemessen war es nicht so — die Funktion las
`Bereich::von_seite(modell.aktiv())`, und das dritte Abnahmekriterium von C1
("verstellen die Breite des Editors, solange er den Fokus hat") wäre unerfüllt
geblieben. Richtig bleibt der Schluss des Plans daraus: ein Befehl kommt nicht
dazu, weil `bereich_verbreitern` und `bereich_verschmaelern`
`Wirkungsbereich::Ueberall` tragen.

Welcher Bereich zu einem Fokuswert gehört, sagt `fokus::holt_hervor` und keine
zweite Zuordnung daneben — dieselbe, die `fokus_setzen` schon liest.
`Fokus::Dateifenster` und `Fokus::Anderswo` liefern dort `None` und fallen auf
das aktive Dateifenster. Damit verstellt derselbe Befehl jetzt auch die Breite
der Lesezeichenleiste und der Vorschau, wenn der Fokus dort steht; das ist die
Verallgemeinerung, die C1 verlangt, und keine Ausnahme für den Editor.

`sitzung_bauen` trug keine Zeile bei. Es ruft
`breiten_uebernehmen(aufteilung.gemessene_breiten())`, und beides steht seit dem
260808 auf `[f64; 5]`; die Editorbreite lief damit schon durch.

### S20: der Fokusbefehl in den Editor

`Anwendungsdelegierter::fokus_editor_holen` weist den Befehl ab, wenn der Editor
ausgeblendet ist **und** keine Datei hält, und ruft sonst
`fokus_holen(Fokus::Editor)` wie die drei bestehenden Fokusbefehle. Die
Bedingung ist ein Und und kein Oder, weil C1 sie so schreibt: steht die Fläche
auf dem Schirm, nimmt sie den Fokus auch ohne Datei, damit der Nutzer erfährt,
wo seine Tasten ankommen.

In `holt_hervor` gehört die Bedingung nicht. Das ist eine reine Zuordnung ohne
Zustand, und ein Vorbehalt darin träfe die drei übrigen Fokusbefehle mit.

Ein zweiter Befehl für den Weg heraus entsteht nicht: `fokus_dateifenster`
trägt `Wirkungsbereich::Ueberall` und wirkt im Editor.

### S22: F4 öffnet den ausgewählten Eintrag

`Anwendungsdelegierter::im_editor_oeffnen` nimmt den ausgewählten Eintrag des
aktiven Dateifensters, legt ihm die Prüfung aus S10 an und holt bei Erfolg den
Editor mit `fokus_holen(Fokus::Editor)` hervor; die Vorschau verschwindet dabei
über S18, ohne dass diese Funktion sie nennt. Die Reihenfolge ist die, die das
elfte Abnahmekriterium von C2 verlangt: erst die Prüfung, dann die Fläche. Eine
Datei, die der Editor ohnehin abweist, blendet ihn nicht ein.

Geprüft wird an der einen Stelle, `krk_core::text::datei::oeffnen`. Eine zweite
Regel entsteht nicht, und die Abweisung geht über `Editormeldung::Abgewiesen`
und `editormeldung_zeigen` auf Rang 1 der Statuszeile des aktiven
Dateifensters — die Meldewege aus S21, jetzt mit ihrem ersten Auslöser.

Der Übergang in den gehaltenen Stand steht seither an einer Stelle:
`Editormodell::uebernehmen`. Beide Lesewege gehen hindurch, `einziehen` vom
Arbeitsfaden und das neue `jetzt_oeffnen` vom rufenden Faden. Der Umstieg auf
den Arbeitsfaden wechselt damit nur den Aufrufer und nicht das Ergebnis.

F4 auf leerer Auswahl meldet "es ist nichts ausgewählt" und verbraucht den
Tastendruck — derselbe Satz und derselbe Weg, den `endgueltig_loeschen` seit der
Runde 1 dafür führt. Eine siebte Variante in `Editormeldung` entsteht dafür
nicht: eine leere Auswahl ist keine Abweisung einer Datei, sondern gar keine
Datei.

## Zwei benannte Zwischenstände

**Gelesen wird auf dem Hauptfaden.** Der Arbeitsfaden aus S15 steht gebaut da,
aber der Takt, der seine Antwort abholt, entsteht erst mit dem Schritt, der das
Lesen auf den Arbeitsfaden legt; bis dahin fände `einziehen` niemanden, der ihn
ruft. Solange der Editor eine große Datei einliest, hält der Hauptfaden an. Der
Preis steht am Doc-Kommentar von `Editormodell::jetzt_oeffnen`.

**Beim Wechsel auf eine andere Datei fällt ein ungesicherter Stand ohne
Rückfrage.** Die Nachfrage aus C4 kommt mit ihrem eigenen Schritt.

## Abweichung von der Dateizeile des Plans

`crates/krk-ui/src/appkit/editor.rs` ist mitgezogen, und es geht nicht ohne.
`Editorbereich` hält das `Editormodell`, und ohne einen Weg dorthin käme keine
Datei in den Editor; ein zweites `Editormodell` beim Anwendungsdelegierten wäre
der zweite Stand, den der Modulkopf von `editormodell.rs` ausschließt.
Dazugekommen sind genau zwei Zugriffsfunktionen: `haelt_datei` für S20 und
`datei_oeffnen` für S22. Die Substanz aus S16 und S21 ist unberührt.

## Die `#[allow(dead_code)]` aus S21

Zwei der **drei** Zeilen sind gefallen; S21 zählte drei als zwei, weil es die
Zeile am Rumpf von `impl Editormeldung` mit der am Wert zusammenzog.

| Zeile | Stand |
|---|---|
| `Editormeldung` (der Wert selbst) | gefallen, Auslöser ist F4 |
| `impl Editormeldung` (der ganze Rumpf) | gefallen |
| `Anwendungsdelegierter::editormeldung_zeigen` | gefallen |
| `Editormeldung::MarkenstelleGeaendert` | **bleibt**, jetzt einzeln |
| `Editormeldung::markenstelle` | **bleibt**, jetzt einzeln |

Die beiden verbliebenen tragen den richtigen ablösenden Schritt im Kommentar:
ihr Auslöser ist der Sprung auf eine Textmarke und nicht F4. Die Ankündigung aus
S21, S22 löse beide ab, war für die Hälfte richtig. Gemessen mit entfernten
Zeilen: `cargo clippy --workspace --all-targets` meldet die Variante als nie
gebaut und die Funktion als nie benutzt, und der Arbeitsbereich stünde rot, weil
`make lint` mit `-D warnings` fährt.

Das `#![allow(dead_code)]` in `editormodell.rs` bleibt unangetastet; sein
ablösender Schritt ist S37, gemessen in S16.

## Geänderte Dateien

- `crates/krk-ui/src/fenstermodell.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/editor.rs`
- `crates/krk-ui/src/editormodell.rs`
- `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (vier
  Schritte auf `[DONE]`, mit ihren Umsetzungsvermerken)

## Abnahme

Die vier Kommandos laufen durch: `cargo build --workspace`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets`,
`cargo fmt --all --check`. `cargo xtask bundle` baut und signiert.

Die Grenzen halten: `grep -c 'objc2' crates/krk-ui/src/editormodell.rs` liefert
0, und `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src`
nennt weiterhin genau `appkit/mod.rs`.

Neue Prüfungen:

- `der_ausschluss_ist_gegenseitig` — die Zuordnung ist symmetrisch; ein
  einseitiger Eintrag verlöre sonst eine Richtung stumm.
- `der_editor_schliesst_die_vorschau_und_die_vorschau_den_editor` — Satz eins
  und zwei des ersten Abnahmekriteriums von C1.
- `keine_folge_aus_zwei_aufrufen_zeigt_editor_und_vorschau_zugleich` — Satz
  drei, über jedes Paar aus zwei Aufrufen an `umschalten` und `einblenden` über
  jeden Bereich, geprüft nach **jedem** der beiden Aufrufe.
- `eine_von_hand_gesetzte_sitzung_zeigt_nicht_beide_zugleich` — derselbe Satz
  für `session.toml`, mit Gegenprobe.
- `eine_verstellte_editorbreite_ueberlebt_die_sitzung` — die Agentenseite des
  fünften Abnahmekriteriums von C1, über `toml::to_string` und
  `toml::from_str`, also über dieselbe Zeichenkette, die auf die Platte geht.
- `eine_datei_ueber_der_grenze_wird_gestellt_und_nicht_aufgenommen` — die
  Reihenfolge aus dem elften Abnahmekriterium von C2. Die Prüfdatei bekommt ihre
  Größe über `set_len` und nicht über 16 MB geschriebener Bytes: entschieden
  wird an der Größe aus `stat(2)`.
- `der_sofortige_weg_und_der_arbeitsfaden_hinterlassen_denselben_stand` — beide
  Lesewege gehen durch `uebernehmen`.

Die Probe zu `holt_hervor(Fokus::Editor)` aus dem Abnahmekriterium von S20 stand
schon: `crates/krk-ui/src/kommandos/fokus.rs` prüft die Zuordnung seit S3 für
alle fünf Werte.

## Nutzerarbeit

Kein Agent kann diese Punkte abnehmen; sie brauchen das laufende Bündel im
Vordergrund. `make bundle` ist gefahren, `target/KRK.app` steht signiert.

1. **F4 auf einer Textdatei.** Auswahl im linken Dateifenster auf eine `.txt`
   oder `.rs`, F4 drücken. Der Editor erscheint am rechten Rand, die Vorschau
   verschwindet, der Text steht da, und die Schreibmarke blinkt im Editor: ein
   getipptes Zeichen landet im Text und nicht in der Dateiliste.
2. **F4 auf einem Ordner.** Die Statuszeile sagt, dass ein Ordner keinen Text
   hat. Der Editor bleibt, wie er war.
3. **F4 auf einem Bild.** Die Statuszeile sagt, dass die Datei sich nicht als
   Text lesen lässt — ein anderer Satz als in Punkt 2.
4. **F4 auf einer Datei über 16 MB.** Anlegen mit
   `mkfile -n 20m ~/Desktop/gross.txt`. Die Statuszeile nennt die Größe, und
   der Satz ist wieder ein anderer.
5. **F4 auf leerer Auswahl**, etwa in einem leeren Ordner. Die Statuszeile sagt
   "es ist nichts ausgewählt".
6. **Die Vorschau holt sich die Fläche zurück.** Bei offenem Editor F3 drücken
   (`vorschau_umschalten`). Der Editor verschwindet, die Vorschau steht, und der
   Fokus liegt im Dateifenster: `up` bewegt die Auswahl.
7. **`shift+cmd+e` mit leerem Editor.** Direkt nach dem Start, ohne dass F4
   gelaufen ist: nichts geschieht, der Editor bleibt ausgeblendet.
8. **`shift+cmd+e` nach F4.** Erst F4 auf einer Textdatei, dann mit F3 die
   Vorschau einblenden (der Editor geht), dann `shift+cmd+e`: der Editor kommt
   mit seiner Datei zurück, die Vorschau geht, und der Fokus steht im Text.
9. **`shift+cmd+d` führt zurück** aus dem Editor in das aktive Dateifenster.
10. **Die Breite, Teil eins.** Bei offenem Editor mit dem Fokus darin
    `ctrl+right` und `ctrl+left` drücken. Der Editor wird breiter und
    schmaler; die Lesezeichenleiste behält ihre Breite, und die beiden
    Dateifenster geben ab, was der Editor bekommt. Zur Gegenprobe mit
    `shift+cmd+d` in das Dateifenster wechseln: dieselben beiden Tasten
    verschieben dort wieder die Trennlinie zwischen den Dateifenstern.
11. **Die Breite, Teil zwei.** Den Editor mit `ctrl+left` so weit
    verschmälern, wie er geht: er hört bei rund 40 Zeichen Zeilenbreite auf,
    und die beiden Dateifenster fallen dabei nicht unter ihre Mindestbreite.
12. **Die Breite, Teil drei.** Eine verstellte Editorbreite, dann KRK beenden
    und neu starten. Der Editor kommt auf der verstellten Breite zurück.
13. **Rund ein Drittel beim ersten Öffnen.** Nur nachvollziehbar mit einer
    frischen `session.toml` (die vorhandene wegräumen): beim ersten F4 nimmt
    der Editor rund ein Drittel der Fensterbreite.

## Kein neuer Befund

Aus diesem Stand ist kein Defekt entstanden. Die beiden Zwischenstände oben
sind benannte Planschritte und keine Befunde.
