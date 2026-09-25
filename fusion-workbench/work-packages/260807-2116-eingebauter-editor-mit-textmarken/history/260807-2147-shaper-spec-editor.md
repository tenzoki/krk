# Shaper — Spec der Editor-Runde

**Geschrieben:** 260807-2147
**Agent:** `shaper`, in-Circle-Klärungsmodus, ohne Rückfragewerkzeug
**Circle:** `260807-2116-eingebauter-editor-mit-textmarken` (aktiv seit 260807-2132)
**Status:** Complete

## Auftrag

Den Spec der Runde 2 schreiben, auf Grundlage der Directive, der vier Festlegungen des
Nutzers vom 260807-2139, der Übergabe an die Editor-Runde und des Codes. Keine Fragen
stellen; was darüber hinaus offen ist, als Entscheidungsdatensatz ablegen.

## Ergebnis

**Spec:** `planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, acht
Fähigkeiten C1 bis C8, drei Mermaid-Diagramme, ein eigener Abschnitt zum Verhältnis zu
den zehn Zeitzusagen und einer zu den vollständigen Fallunterscheidungen.

**Fünf Entscheidungsdatensätze**, alle unter `decisions/` mit dem Marker `_o_`:

| Datei | Bindet |
|---|---|
| `260807-2147_o_welche-dateien-oeffnet-der-editor-ueberhaupt.md` | C2, C3 |
| `260807-2147_o_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md` | C3 |
| `260807-2147_o_wie-greift-die-nachfrage-bei-der-sitzungssicherung.md` | C4 |
| `260807-2147_o_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md` | C6 |
| `260807-2147_o_traegt-eine-textmarke-auch-einen-bereich-oder-nur-eine-stelle.md` | C6 |

**Ein Defekt:** `issues/260807-2147_o_der-circle-datensatz-steht-auf-t-und-nennt-sich-im-kopf-anticipated.md`.

## Die drei Befunde aus der Codedurchsicht

**1. Der Fokusvorbehalt weist heute jeden Tastendruck ab, sobald der Ersthelfer eine
`NSTextView` ist.** `ersthelfer_nimmt_text` in `crates/krk-ui/src/appkit/ereignisse.rs:386-395`
prüft auf `NSTextView`, `NSTextField` und `NSText` und reicht in allen drei Fällen
unverändert an AppKit weiter. Ein Editor auf derselben Klasse ist von einem Feldeditor
nicht zu unterscheiden, und mit dem Fokus im Editor wirkte kein Tastenbefehl von KRK mehr.
Das ist der schärfste Befund dieser Durchsicht und steht als eigene Fähigkeit C7 im Spec
sowie im Gatehinweis der Kopfzeile.

**2. Die Zuordnung der Kennung `bearbeiten` steht doppelt, sobald F4 ein Kommando
bekommt.** `bereich` in `crates/krk-ui/src/belegungsmodell.rs:131` ordnet `"bearbeiten"`
heute über den **Namen** dem `Funktionsbereich::Editor` zu, weil es zu ihr kein Kommando
gibt. Der Zweig darüber greift, sobald es eines gibt; die Namenszeile wird dann toter
Text mit einer zweiten Wahrheit. Sie gehört in derselben Änderung entfernt.

**3. Beim Beenden gibt es heute keinen Ort für eine Nachfrage.**
`crates/krk-ui/src/appkit/anwendung.rs:1162` hält fest, dass kein
`applicationShouldTerminate:` existiert und die Aufrufer von `beenden` nicht mit einer
Rückkehr rechnen. Die Zusage des Nutzers, dass die Nachfrage beim Beenden greift, ist
damit eine neue Stelle und kein Nachziehen.

## Die geprüfte Aussage zur macOS-26-Frage

Der Auftrag verlangte eine Prüfung statt einer Annahme. Geprüft am Dateibestand:
`objc2-app-kit 0.3.2` führt `NSTextView`, `NSTextStorage`, `NSLayoutManager`,
`NSTextContentManager` und `NSTextLayoutManager` als Merkmale, `objc2-foundation 0.3`
führt die Markdown-Auswertung von `NSAttributedString`, und `.cargo/config.toml` setzt
`MACOSX_DEPLOYMENT_TARGET = 15.0`. Der offene Datensatz
`260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` bindet
diese Runde damit **nicht**; seine eigene Randbedingung sagt voraus, dass er erst bindet,
wenn eine Runde die erste neuere Schnittstelle anspricht.

Geprüft ist daneben, dass `objc2-app-kit` **keinerlei** Verfügbarkeitsangaben führt: die
erzeugten Dateien tragen keine. `inference:` Daraus folgt, dass die Untergrenze nicht vom
Übersetzer erzwungen wird. Der Spec verlangt deshalb vom Plan, für jede angesprochene
Textklasse ihre Untergrenze zu nennen.

## Die beiden Entscheidungen, die der Auftrag verlangte

**Der Defekt `cmd+y` gehört in diese Runde.** Vier Gründe stehen in C8 des Specs, angeführt
von diesem: jeder Fokusbereich hat nach der Ordnung der Runde 1 einen Fokusbefehl, alle
drei bestehenden tragen `shift+cmd+<Buchstabe>`, und ein vierter liefe in denselben Fehler.
Der Einwand, die Runde klammere die Restarbeit der Runde 1 aus, trägt nicht: die
Ausklammerung des Nutzers gilt wörtlich den Messreihen, und die Sitzungshistorie der
Aktivierungsrunde ordnet den Defekt bereits dieser Runde zu.

**Diese Runde setzt keine eigene Zeitzusage.** Drei Gründe stehen im Spec, angeführt von
diesem: eine Zeitzusage ist nur eine Zusage, wenn sie abgenommen wird, und die Abnahme ist
aus dieser Runde ausgeklammert. An die Stelle einer elften Zahl treten zwei Kriterien, die
ohne die Messstrecke prüfbar sind, und eine Aufstellung der drei bestehenden Zusagen
(L1, L4, L7), deren Wege diese Runde berührt.

## Die drei Fallunterscheidungen aus CLAUDE.md

`Kommando::wirkungsbereich` ist berührt, `bereich_des_kommandos` ist berührt,
`schiebt_auffrischung_auf` ist **nicht** berührt. Der Grund für die dritte: die
Fallunterscheidung geht über `krk_core::operation::Art`, und das Sichern einer Datei aus
dem Editor ist kein Auftrag der Operationsmaschine. Die Auffrischung des betroffenen
Dateifensters läuft über den bestehenden FSEvents-Weg.

Der Spec nennt daneben eine vierte, die `CLAUDE.md` nicht führt: `Bereich` in
`crates/krk-ui/src/fenstermodell.rs:50-68` mit `ALLE: [Bereich; 4]` und allem, was an
dieser Zahl hängt.

## Was der Shaper nicht getan hat

Kein Mittel für die Textdarstellung festgelegt, keine Klasse vorgeschrieben, keine
Schrittreihenfolge gesetzt, keine Belegung außer F4 vergeben. Der Circle-Datensatz ist
nicht angefasst worden; der Defekt zu seinen Kopfzeilen liegt unter `issues/`.
