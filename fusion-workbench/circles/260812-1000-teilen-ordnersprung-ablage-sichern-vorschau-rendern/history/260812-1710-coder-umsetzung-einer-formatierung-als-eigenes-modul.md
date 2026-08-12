# Schritt 7: Die Umsetzung einer Formatierung wird ein eigenes Modul

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 7
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 410, nachher 410

---

## Was gebaut wurde

Ein mechanischer Umzug ohne Verhaltensänderung. Die Umsetzung einer
`Formatierung` in die Merkmale einer `NSTextView` wohnt jetzt in
`crates/krk-ui/src/appkit/textmerkmale.rs` statt in `editor.rs`, weil die
Vorschau mit Schritt 9 dieselbe braucht und zwei Umsetzungen zwei Wahrheiten
darüber wären, wie eine Überschrift aussieht.

Umgezogen sind `LESEZUSCHLAG`, `UEBERSCHRIFTSFAKTOREN`, `LISTENEINZUG`,
`schriftmerkmal`, `einzugsmerkmal`, `feste_schrift`, `nsfarbe`, `grundschrift`,
der Rumpf von `Editorbereich::formatierung_anwenden` als `pub fn anwenden` und
`Editorbereich::merkmale_zuruecksetzen` als `pub fn zuruecksetzen`.

**Zwei Stücke stehen nicht in der Liste des Plans und sind trotzdem
mitgegangen**, weil sie sonst über die Modulgrenze zurückgreifen müssten:
`grundschrift` rechnet aus `LESEZUSCHLAG` und `feste_schrift`, und
`zuruecksetzen` braucht sie; `LISTENEINZUG` hat mit `einzugsmerkmal` seinen
einzigen Leser. `grundschrift` ist deshalb `pub` und hat mit
`Editorbereich::grundschrift_setzen` weiterhin einen zweiten Aufrufer im
Editor. `LESEZUSCHLAG` bleibt privat, und der Doc-Verweis darauf in
`grundschrift_setzen` nennt jetzt das Modul statt der Konstante — eine
`pub`-Konstante ohne Codeleser wäre unter `-D warnings` toter Code.

**Der Rückgabewert trägt `#[must_use]` und hat einen Verbraucher.** `anwenden`
meldet mit `false`, dass der Gürtel die Lieferung abgewiesen hat: fehlender
Textspeicher, fehlender Layoutverwalter oder eine Länge, die nicht zu
`Formatierung::laenge` passt. `Editorbereich::formatierung_anwenden` zieht die
Nummernspalte genau dann nach, wenn gesetzt wurde. **Das ist der Grund, aus dem
das Verhalten unverändert bleibt**: die alte Fassung sprang bei jedem der drei
Fälle über `return` heraus und ließ das Nachziehen damit ebenfalls aus. Ein
unbedingtes Nachziehen hinter dem Ruf wäre die eine Verhaltensänderung dieses
Schrittes gewesen.

**`Editorbereich::formatierung_anwenden` ist damit drei Schritte:** die beiden
Angaben aus dem Modell holen und die Ausleihe beenden, `textmerkmale::anwenden`
rufen, bei `true` die Nummernspalte nachziehen.

## Was der Modulkopf sagt

Der Kopf von `textmerkmale.rs` nennt beide Verbraucher und sagt ausdrücklich,
dass die Vorschau heute noch keiner ist: „Heute ruft allein `super::editor` hier
herein. Die Vorschau bekommt ihre Auszeichnungen mit dem Schritt, der das
gerenderte Markdown in ihre Textfläche trägt; bis dahin ist der zweite
Verbraucher der Grund für den Schnitt und noch nicht sein Nutzer."

Der Kommentar über die zwei Listen und zwei Orte steht jetzt an `anwenden`. Er
begründet den Schnitt zwischen Textspeicher und Layoutverwalter, nicht den
Aufrufer, und gehört deshalb dorthin.

## Die Untergrenzen-Angaben, am SDK gegengelesen

`textmerkmale.rs` trägt den Abschnitt `# Ab welchem macOS die angesprochenen
Klassen stehen`. Die Deckung unter `crates/krk-ui/src/appkit/` steht danach bei
34 von 36 Dateien (vorher 33 von 35, rekursiv gezählt); ohne den Abschnitt sind
weiterhin `koordinaten.rs` und `mod.rs`, beide begründet.

Gelesen im SDK unter
`/Applications/Xcode.app/…/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers`:

| Berührung | SDK | Fundstelle |
|---|---|---|
| `NSTextView` | keine Angabe → 10.0 | `NSTextView.h:76` |
| `NSTextStorage` | `macos(10.0)` | `NSTextStorage.h:37` |
| `NSLayoutManager` | **`macos(10.7)`** | `NSLayoutManager.h:65` |
| `NSFont` | keine Angabe → 10.0 | `NSFont.h:24` |
| `NSColor` | keine Angabe → 10.0 | `NSColor.h:77` |
| `NSMutableParagraphStyle` | `macos(10.0)` | `NSParagraphStyle.h:112` |
| `addTemporaryAttribute:value:forCharacterRange:` | `macos(10.5)` | `NSLayoutManager.h:360` |
| `colorWithSRGBRed:green:blue:alpha:` | `macos(10.7)` | `NSColor.h:90` |

**Die 10.7 an `NSLayoutManager` ist ein Fund und keine Übernahme.** Die
Modulköpfe von `editor.rs` und `nummernspalte.rs` führen dieselbe Klasse seit
jeher unter „stehen seit macOS 10.0". Alle drei Zahlen liegen weit unter dem
Zielsystem 15.0, die Sache ist also folgenlos; der neue Kopf sagt die gelesene
Zahl und nennt die Abweichung in einem Satz, damit niemand ihn für den
schlampigen hält. Die beiden vorhandenen Köpfe sind nicht angefasst: eine
Korrektur allein in `editor.rs` risse die zwei Stellen auseinander, und
`nummernspalte.rs` liegt außerhalb der drei Dateien dieses Schrittes.

## Was im Kopf von `editor.rs` nachgezogen ist

`NSTextStorage` und `NSMutableParagraphStyle` fasst die Datei nicht mehr an;
ihre Angaben stehen jetzt nebenan. `NSFont` und `NSColor` **fehlten** in der
Aufzählung, obwohl Kopf, Grundschrift und `textflaeche_bauen` sie ansprechen und
weiter ansprechen — sie sind nachgetragen, mit ihren SDK-Fundstellen. Den
Layoutverwalter fragt `editor.rs` weiter an zwei Stellen, dem Zugriff in
`textflaeche_bauen` und der Probe, die den Rückfall auf TextKit 1 festhält.

## Proben

**Keine ist umgezogen, weil es keine gab.** Die Suche über den Prüfteil von
`editor.rs` nach `Formatierung`, `Auszeichnung`, `nsfarbe`, `einzugsmerkmal`,
`schriftmerkmal` und `UEBERSCHRIFTSFAKTOREN` findet nichts: die Umsetzung war im
Baum durch keine Probe gemessen, weder vorher noch nachher. Die Zahl 410 im
Binärziel `krk` ist deshalb unverändert, wie sie es bei einem reinen Umzug sein
soll. Dass hier eine Lücke liegt, ist ein Befund und keine Folge dieses
Schrittes; sie zu schließen verlangt eine `NSTextView` und damit den
Hauptfaden, den `libtest` nicht hergibt — dieselbe Lage, die
`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`
offenhält.

## Der Befund aus der Durchsicht von Turn 1

`issues/260812-1529_o_zwei-doc-kommentare-der-runde-nennen-zahlen-die-der-baum-nicht-traegt.md`
betrifft `anwendung.rs:2357` und `angezeigtedatei.rs:73-79`. Keine der beiden
Dateien gehört zu diesem Schritt, also ist nichts nachgezogen und der Datensatz
bleibt offen.

Nachgezogen ist dagegen die Zahl im Kopf von `appkit/mod.rs`:
„Fünfundzwanzig Module" → „Sechsundzwanzig Module", weil dieser Schritt das
sechsundzwanzigste anlegt. Dazu der Pfeil `editor ──> textmerkmale ──>
crate::hervorhebung` im Überblick, ein Absatz zum neuen Modul und sein Eintrag
in der Aufstellung der genannten Ziele.

## Dateien

- neu `crates/krk-ui/src/appkit/textmerkmale.rs`
- `crates/krk-ui/src/appkit/editor.rs` (5619 → 5388 Zeilen)
- `crates/krk-ui/src/appkit/mod.rs`
