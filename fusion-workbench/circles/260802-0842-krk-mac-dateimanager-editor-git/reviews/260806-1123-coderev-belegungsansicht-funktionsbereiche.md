# Coderev: Commit ccaf821 — Gliederung der Belegungsansicht nach Funktionsbereichen

**Sender:** coderev
**Umfang:** ausschließlich der Diff von `ccaf821` — `crates/krk-ui/src/belegungsmodell.rs` (Aufzählung `Funktionsbereich`, Zuordnung `bereich`, Zeilenliste, 12 Prüfungen) und `crates/krk-ui/src/appkit/belegungsansicht.rs` (Gruppenzeilen, Auswahlverhalten).
**Prüflauf:** `make check` grün (Bau, Tests, Clippy mit `-D warnings`, fmt), Stand 260806-1123.

## Zusammenfassung

Der Commit hält alle sechs geprüften C3-Abnahmekriterien, bleibt im Modell objc2-frei und folgt beim Gruppenzeilen-Muster der bestehenden Bauart der Leiste. Zwei Befunde, beide niedrig: eine Doku-Drift im Modulkopf und eine unbenannte crate-übergreifende Kopplung in der Auswahlwiederherstellung. Kein Release-Blocker.

## Totals

- Critical: 0
- High: 0
- Medium: 0
- Low: 2

## Befunde

### Low 1 — Modulkopf zitiert den Issue-Pfad mit überholtem Marker

`belegungsmodell.rs:27` verweist auf `issues/260806-1054_p_belegungsansicht-gruppiert-nach-funktionsbereich.md`; die Datei trägt seit dem Abschluss `_c_`. Der zitierte Pfad zeigt ins Leere, und jeder in Code einzementierte Zustandsmarker veraltet beim nächsten Übergang erneut.
Issue: `issues/260806-1123_o_modulkopf-zitiert-den-issue-pfad-mit-ueberholtem-marker.md`

### Low 2 — Auswahlwiederherstellung umgeht die Wählbarkeitsprüfung

`belegungsansicht.rs:286-296` (`nachziehen`) stellt die Auswahl nach `reloadData` programmatisch am alten Index wieder her; `selectRowIndexes_byExtendingSelection` fragt `tableView:shouldSelectRow:` nicht. Nach dem Zurücksetzen baut das Modell die Zeilen neu (`belegungsmodell.rs:377-381`). Heute nicht auslösbar: das Einlesen im Kern erzwingt für jede geladene Belegung exakt den Auslieferungs-Funktionsbestand (`belegung.rs:713-718` weist Unbekanntes ab, `belegung.rs:751-763` ergänzt Fehlendes), die Überschrift-Indizes sind damit vor und nach dem Zurücksetzen identisch. Der tragende Invariant liegt aber in einem anderen Crate und steht an der Aufrufstelle nirgends.
Issue: `issues/260806-1123_o_auswahlwiederherstellung-der-belegungsansicht-umgeht-die-waehlbarkeitspruefung.md`

## Geprüfte Maßstäbe im Einzelnen

**C3-Abnahmekriterien (Spec `planning/260802-1036_o_spec-navigator-geruest.md:163-187`):**

- *Eine Zeile je Funktion:* hält. `eine_zeile_je_funktion` (`belegungsmodell.rs:481-526`) zählt Zeilen = Funktionen + Überschriften und prüft die Eindeutigkeit der Funktionstexte und der bloßen Namen.
- *Zuweisen per Drücken:* unverändert; der Aufnahmeweg über den Ereignisabgriff ist nicht berührt. Neu ist nur, dass eine Überschriftszeile keine Zuweisung nimmt (`belegungsmodell.rs:355-357`, Test `eine_ueberschrift_nimmt_keine_zuweisung_an`).
- *Konfliktmeldung:* unverändert, kommt weiter wörtlich aus dem Kern (`belegungsmodell.rs:368`).
- *Zurücksetzen:* hält; baut die Zeilenliste korrekt neu (`belegungsmodell.rs:377-381`), Test `das_zuruecksetzen_stellt_die_auslieferung_wieder_her`.
- *Speichern beim Verlassen:* unverändert (`geaendert`-Kennzeichen, `belegungsansicht.rs:265-267`).
- *F1–F12 ohne "Fn+":* hält, Test auf Überschriften ausgedehnt (`belegungsmodell.rs:588-619`).

**objc2-Freiheit des Modells:** hält. `belegungsmodell.rs:48` importiert nur `krk_core::tasten`; kein objc2-Bezug in der Datei. Die AppKit-Grenze bleibt bei `appkit/`.

**Vollständige Fallunterscheidung:** hält in zwei Hälften, wie im Modulkopf dokumentiert. Ein neues `Kommando` fällt beim Übersetzen auf: `bereich_des_kommandos` (`belegungsmodell.rs:142-210`) hat keinen Auffangzweig, dasselbe Muster wie `Kommando::wirkungsbereich` im Kern (`belegung.rs:384`). Eine neue Funktion *ohne* Kommando rutscht am Übersetzer vorbei — `bereich` (`belegungsmodell.rs:124-135`) trägt für die Kennungs-Hälfte einen `_ => None`-Zweig —, wird aber von `jede_kennung_hat_einen_funktionsbereich` (`belegungsmodell.rs:534-542`) gegen die Auslieferungsbelegung gefangen und zur Laufzeit vom lauten Abbruch in `gliederung` (`belegungsmodell.rs:419-425`). Der Panic ist über Nutzerdaten nicht erreichbar, weil das Einlesen unbekannte Kennungen abweist (`belegung.rs:713-718`); er fängt allein Programmierfehler, und das steht so im Doc-Kommentar.

**Auswahl- und Navigationsverhalten über Gruppenzeilen:**

- Überschriften sind nicht wählbar: `tableView:shouldSelectRow:` (`belegungsansicht.rs:152-155`) sperrt Maus wie Tastatur; die Pfeiltasten überspringen abgewiesene Zeilen (AppKit-Verhalten, Inferenz — am Bündel nicht selbst nachgemessen, der Commit nennt eine Prüfung am Bündel).
- Anfangsauswahl: korrekt auf der ersten Funktionszeile (`belegungsansicht.rs:403`, `416-419`); `die_zeilen_sind_nach_bereichen_gegliedert` hält `erste_funktionszeile() == Some(1)` fest.
- Gruppenzeilen: `tableView:isGroupRow:` (`belegungsansicht.rs:143-146`), Zellenbau behandelt die spaltenlose Anfrage (`belegungsansicht.rs:317-331`). Einzige Ausnahme von der Sperre ist die programmatische Wiederherstellung — Befund Low 2.

**Cross-cutting:** Das Muster deckt sich mit der Leiste (`leiste.rs:148-157`: `isGroupRow` + `shouldSelectRow` über `Zeile::waehlbar`). Die Leiste hält ihre Auswahl zusätzlich im eigenen Modell auf wählbaren Zeilen (`leistenmodell.rs:140`); die Belegungsansicht verlässt sich allein auf AppKit — daher Low 2 nur hier.

**Nebenprüfungen:** Der Kommentar "57 Funktionen und neun Bereichsüberschriften" (`belegungsansicht.rs:76-78`) stimmt gegen `resources/default-keymap.toml` (57 Einträge) und `Funktionsbereich::ALLE` (9). Die im Issue-Abschluss genannten 12 Prüfungen sind die 12 `#[test]`-Funktionen der Datei. `resources/default-keymap.toml` ist unberührt, wie das Quell-Issue es verlangt.

## Gesamturteil

Sauber. Die Gliederung liegt vollständig im Modell, die Ansicht bleibt Anzeige, kein C3-Kriterium bricht, die Prüfungen tragen die neuen Zusagen. Die zwei Low-Befunde sind Aufräumarbeit, kein Handlungsdruck vor der nächsten Auslieferung.
