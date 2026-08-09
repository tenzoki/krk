# Bugfix: Eine ungesicherte Änderung fiel beim zweiten F4 auf dieselbe Datei

**Date:** 2026-08-09 21:48
**Status:** Complete
**Trigger:** Defektdatensatz `issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`, aus eigener Bedienung des Nutzers am laufenden Bündel

## Error

> wenn im editor was geändert wurde und man mit dem viewer die gleiche datei
> anzeigt (vor speichern) ist die änderung weg.

Übersetzungsstand `111c72e`.

## Root Cause

`Editormodell::jetzt_oeffnen` (`crates/krk-ui/src/editormodell.rs:543-550` im
Stand `111c72e`) fragte den bisher gehaltenen Pfad nicht und las bedingungslos
neu. `uebernehmen` (ebd. `563-578`) setzte den Plattenstand ein und löschte die
Abweichungsmarke; `Editorbereich::datei_oeffnen`
(`crates/krk-ui/src/appkit/editor.rs:280-286`) schrieb ihn über
`stand_einsetzen` in die `NSTextView`.

Der Weg des Nutzers: F4 → tippen → `f3`/`cmd+y` blendet die Vorschau ein und
verdrängt den Editor nach C1 (S18) → F4 auf denselben Eintrag, weil F4 der
einzige Befehl ist, der den Editor mit seiner Datei zurückholt.

**Verstärkend, aber nicht behoben:** `Editormodell::bearbeiten` hat in
`crates/krk-ui/src/appkit/` keinen Aufrufer, denn der Delegierte
`textDidChange:` kommt erst mit S26. Das Getippte stand deshalb nie im Modell,
sondern allein in der `NSTextView`. Eine Behebung über die Abweichungsmarke war
damit ausgeschlossen — sie ist immer `false`.

### Die drei Fragen des Datensatzes

1. **Beim Ausblenden oder beim Einblenden?** Beim Einblenden, und nur über F4.
   `Aufteilung::anwenden` setzt `setHidden`
   (`crates/krk-ui/src/appkit/aufteilung.rs:201-205`); die Fläche behält ihren
   Textspeicher, `Editorbereich` sein `RefCell<Editormodell>`
   (`appkit/editor.rs:192-200`), und `Editormodell::schliessen` hat keinen
   Aufrufer.
2. **Liest der Editor beim Einblenden neu?** Ja. Der Stempel aus S15 entscheidet
   dabei nichts: er wird beim Öffnen gesetzt, nie gelesen — `fremd_geaendert`
   hat bis heute keinen Aufrufer.
3. **Zeigt die Vorschau den Plattenstand?** Ja, und das ist richtig. Sie ist am
   Verlust unbeteiligt; `shift+cmd+e` (`fokus_editor`) holt den Editor ohne
   Neulesen zurück und verlor die Änderung nie.

## Fix

Die kleinste Änderung, die den Verlust beendet: die Datei, die der Editor schon
hält, wird nicht ein zweites Mal gelesen. Die Rückfrage aus S28 ist nicht
angefangen worden.

| File | Change |
|------|--------|
| `crates/krk-ui/src/editormodell.rs:392-425` | `Ladeausgang` bekommt den dritten Wert `SchonOffen`; die Doc begründet, warum er nicht mit `Geoeffnet` zusammenfällt |
| `crates/krk-ui/src/editormodell.rs:478-492` | neu: `Editormodell::haelt_bereits`, die eine Stelle für „dieselbe Datei" |
| `crates/krk-ui/src/editormodell.rs:608-611` | `jetzt_oeffnen` kehrt mit `SchonOffen` zurück, **bevor** es liest |
| `crates/krk-ui/src/editormodell.rs:552-565` | Doc an `oeffnen`: S24 hat die Abkürzung mitzunehmen, sonst kommt der Verlust stumm zurück |
| `crates/krk-ui/src/appkit/editor.rs:280-293` | Doc an `datei_oeffnen`: der Vergleich nennt `Geoeffnet` namentlich und darf nicht auf „nicht abgewiesen" gelockert werden |
| `crates/krk-ui/src/appkit/anwendung.rs:2751-2762` | der Zweig nimmt `SchonOffen` mit auf: hervorholen und Fokus setzen, wie bei `Geoeffnet` |
| `crates/krk-ui/src/editormodell.rs` (Proben) | `ein_zweites_oeffnen_derselben_datei_wirft_den_bearbeiteten_stand_nicht_weg`, `eine_andere_datei_wird_weiterhin_gelesen` |

**Warum an dieser Stelle und nicht in `appkit/editor.rs`:** die Frage „hält der
Editor diese Datei schon" ist eine Frage an den Stand und nicht an die Fläche.
Im Modell ist sie außerdem ohne Fenster prüfbar — `crates/krk-ui` hat keine
Probe, die AppKit-Objekte baut, und der Verlust wäre sonst nicht nachstellbar
gewesen.

**Der Preis, im Code benannt:** F4 auf die schon gehaltene Datei liest sie auch
dann nicht neu, wenn sie sich von außen geändert hat. Ein Befehl zum Neulesen
existiert nicht, und C2 sagt keinen zu; die Änderung von außen trägt S31.

**Nicht behoben, bewusst:** F4 auf eine **andere** Datei wirft den ungesicherten
Stand weiterhin ohne Rückfrage. Das ist der zweite Anlass aus S28 und braucht
S25.

## Verification

- [x] Original error resolved — `ein_zweites_oeffnen_derselben_datei_wirft_den_bearbeiteten_stand_nicht_weg` fällt ohne die Abkürzung (gemessen: `left: Geoeffnet, right: SchonOffen`, Stand ist der Plattenstand) und ist mit ihr grün
- [x] `cargo build --workspace` → 0
- [x] `cargo test --workspace` → 0, alle Reihen grün
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → 0
- [x] `cargo fmt --all --check` → 0
- [x] No regressions introduced

**Nutzerarbeit, nicht abgenommen:** die Wirkung am laufenden Bündel. Kein Agent
kann sie prüfen, weil KRK dafür im Vordergrund stehen muss.

## Unrelated Issues Found

`issues/260809-2148_o_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`
— S25 hängt an S24 und steht damit vor S26, das die Rückschreibung
`textDidChange:` → `bearbeiten` baut. In der geplanten Reihenfolge schriebe
`cmd+s` den unveränderten Plattenstand zurück und meldete eine gelungene
Sicherung. Auch S28 und S29 lesen `hat_ungesicherten_stand` und bekämen ohne die
Rückschreibung immer `false`.
