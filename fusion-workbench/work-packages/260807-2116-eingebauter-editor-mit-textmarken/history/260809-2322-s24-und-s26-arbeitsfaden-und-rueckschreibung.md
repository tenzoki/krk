# S24 und S26: das Lesen auf dem Arbeitsfaden und die Rückschreibung

**Status:** Complete
**Ausführender:** coder
**Datum:** 260809-2322
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken

## Auftrag

S24 („Das Lesen auf dem Arbeitsfaden") und S26 („Der ungesicherte Stand und
seine Anzeige") zusammen und **vor** S25 umsetzen. Der Grund steht im Defekt
`issues/260809-2148_o_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`:
in der geplanten Reihenfolge hätte S25 den unveränderten Plattenstand
zurückgeschrieben **und dabei eine gelungene Sicherung gemeldet**, weil
`Editormodell::bearbeiten` keinen Aufrufer hatte. Von den zwei Wegen, die der
Datensatz vorschlug, ist Weg 2 gegangen worden: S26 vor S25.

## Was gebaut wurde

### S24: ein Leseweg, und er läuft über den Faden

`Editormodell::jetzt_oeffnen` ist gefallen. Es war der benannte Zwischenstand,
solange niemand die Antwort des Arbeitsfadens abholte; der Takt dafür steht
jetzt in `appkit/editor.rs`. Die Abkürzung für die schon gehaltene Datei ist
nach `Editormodell::oeffnen` gewandert, wie der Doc-Kommentar von `jetzt_oeffnen`
es verlangte. `oeffnen` liefert seither `Option<Ladeausgang>`, mit derselben
Bedeutung wie bei `einziehen`: `Some(...)` heißt „hier ist ein Ausgang",
`None` heißt „warte auf den Faden".

`Editorbereich` hat den Einzugstakt bekommen, nach dem Muster der Vorschau: ein
`NSTimer` im Takt von 1/60 s, über `NSRunLoopCommonModes` in die Laufschleife
gehängt, beendet über `invalidate`, sobald nichts mehr lädt.

Der Ausgang eines Öffnens steht damit nicht mehr fest, wenn der auslösende
Befehl zurückkehrt. `Editorbereich::datei_oeffnen` gibt deshalb nichts mehr
zurück; jeder Ausgang geht durch die Senke aus `melder_setzen` an
`Anwendungsdelegierter::editorausgang_behandeln`. Auch der sofort feststehende
`Ladeausgang::SchonOffen` geht diesen Weg, damit es **eine** Behandlung der drei
Ausgänge gibt und nicht zwei.

Die Reihenfolge aus dem elften Abnahmekriterium von C2 hält, ohne dass etwas
dafür gebaut werden musste: weil die Prüfung selbst auf dem Arbeitsfaden läuft,
blendet F4 den Editor erst ein, wenn die Datei gelesen ist. Eine abgewiesene
Datei bekommt die Fläche nie zu sehen.

### S26: der Rückweg und der Kopf

`Editorbereich` ist der Delegierte seiner eigenen `NSTextView` und beantwortet
`textDidChange:`. `text_zurueckschreiben` nimmt den ganzen Text aus der Fläche
und gibt ihn an `Editormodell::bearbeiten`, das ihn durch
`krk_core::text::datei::in_gehaltene_form` führt — die eine Normalisierungsstelle,
und keine zweite daneben. Der Delegierte wird schwach gehalten (Eigenschaft von
`NSTextView`), die Fläche stark; ein Ring entsteht nicht.

Der Editorbereich hat einen Kopf bekommen, weil es keinen gab. `sicht()` liefert
seither einen Bereich aus Kopf und Bildlaufansicht; der Kopf ist eine einzeilige
Beschriftung mit dem Dateinamen, davor ein `•`, solange der Stand abweicht.
Höhe und Einzug sind die der Statuszeile aus der Runde 1, weil es dieselbe Form
ist. Das Zeichen steht **vor** dem Namen: ein schmaler Editor kürzt von rechts,
und ein Zeichen am Ende ginge als erstes mit.

Nachgezogen wird der Kopf nur beim Übergang, nicht bei jedem Anschlag: die
Abweichungsmarke geht einmal von falsch nach wahr und bleibt es bis zum Sichern.
Was im Kopf steht, entscheidet die reine Funktion `kopfzeile`, ohne AppKit und
damit ohne Fenster prüfbar.

## Die Annahme, auf der S26 ruht

`setString:` löst `textDidChange:` **nicht** aus; eine `NSTextView` meldet ihrem
Delegierten allein die Änderungen des Nutzers. Darauf ruht, dass eine frisch
geöffnete Datei nicht sofort als geändert gilt. Die Annahme steht im Modulkopf,
und ein Bruch wäre sofort sichtbar: als Abweichungszeichen am Kopf einer eben
geöffneten Datei. Die Prüfliste des Nutzers fragt danach.

## Geänderte Dateien

- `crates/krk-ui/src/editormodell.rs` — `oeffnen` mit Abkürzung und
  `Option<Ladeausgang>`, `jetzt_oeffnen` entfernt, Modulkopf und vier
  Doc-Kommentare nachgezogen, sechs Proben auf den einen Leseweg gestellt, eine
  entfallen.
- `crates/krk-ui/src/appkit/editor.rs` — Einzugstakt, Ausgangssenke, Delegierter
  `textDidChange:`, Kopf mit Dateiname und Abweichungszeichen, drei neue Proben.
- `crates/krk-ui/src/appkit/anwendung.rs` — Melder beim Aufbau eingetragen,
  `im_editor_oeffnen` stößt nur noch an, `editorausgang_behandeln` neu.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 15 Testbinärdateien, keine fehlgeschlagene |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo fmt --all --check` | 0 |
| `cargo xtask bundle` | baut und signiert `target/KRK.app` |

Die Grenzen halten: `grep -rEln '#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src`
nennt weiterhin genau `appkit/mod.rs`, und `grep -c 'objc2'` liefert für
`editormodell.rs` weiterhin 0.

**Was ein Agent nicht abnehmen kann**, steht in der Prüfliste an den Nutzer: ob
der Kopf auf dem Schirm steht, ob das Zeichen mit dem ersten Tastendruck
erscheint, und ob die Dateifenster während des Ladens einer großen Datei
bedienbar bleiben. Das verlangt KRK im Vordergrund.

## Datensätze

- `issues/260809-2148_c_...` — geschlossen, mit Vermerk, welcher der beiden Wege
  gegangen wurde.
- `issues/260809-2322_o_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`
  — neu. Der ungemessene Preis der Bauart: bei einer Datei nahe 16 MB kostet
  jeder Anschlag eine vollständige Kopie und einen Durchlauf. Der Ausweg ist
  benannt; gebaut wird er nicht auf Verdacht.
- Der Plan trägt S24 und S26 auf `[DONE]`, die umgedrehte Kante im
  Abhängigkeitsbild, die neue Abhängigkeit an S25 und je einen Vermerk an S23
  und S25 über das, was sie vorfinden.
