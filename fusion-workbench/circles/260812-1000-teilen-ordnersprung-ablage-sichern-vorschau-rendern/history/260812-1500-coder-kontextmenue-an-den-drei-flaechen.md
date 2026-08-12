# Schritt 6: Das Kontextmenü an den drei Flächen

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 6
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0

---

## Was gebaut wurde

Ein Rechtsklick zeigt in Dateiliste, Editor und Vorschau ein Kontextmenü mit dem
Teilen-Eintrag des Systems. Gebaut wird es an genau einer Stelle,
`teilen::eintrag_anfuegen`; die Flächen beantworten allein, welche Einträge
betroffen sind.

**Zwei Anschlussarten, und der Unterschied ist die Bauart der Fläche.** Eine
`NSTextView` baut ihr Kontextmenü selbst und bietet dafür einen
Delegiertenhaken; eine Tabelle und eine Bildansicht bauen keines und nehmen das
Menü der Ansicht. Fünf Ansichten verteilen sich auf die beiden Wege:

| Ansicht | Weg |
|---|---|
| Textfläche des Editors | `textView:menu:forEvent:atIndex:` |
| Textanzeige der Vorschau | `textView:menu:forEvent:atIndex:` |
| Dateiliste (`NSTableView`) | `setMenu:` + `menuNeedsUpdate:` |
| Bildansicht der Vorschau | `setMenu:` + `menuNeedsUpdate:` |
| Inhaltsfläche der Vorschau | `setMenu:` + `menuNeedsUpdate:` |

Der Haken bekommt das Menü, das AppKit gebaut hat, und gibt es **ergänzt**
zurück; KRKs Eintrag tritt damit neben Ausschneiden, Kopieren und die
Schreibwerkzeuge, statt sie zu ersetzen. Der andere Weg leert das Menü und baut
es bei **jedem** Rechtsklick neu, weil die betroffenen Einträge sich zwischen
zwei Klicks ändern.

**`crates/krk-ui/src/appkit/teilen.rs`**

- Das `#[expect(dead_code, …)]` an `eintrag_anfuegen` ist weg. Es war
  `expect` und nicht `allow`, damit der Übersetzer die Zeile meldet, sobald sie
  überflüssig wird; genau das ist eingetreten.
- Der Modulkopf trägt eine Skizze der beiden Anschlussarten und sagt, welche
  Ansicht welchen Weg nimmt. Der Satz „die Flächen hängt erst der nächste
  Schritt an" ist ersetzt.
- Der Doc-Kommentar an `eintrag_anfuegen` nennt die fünf Aufrufer und den einen
  Unterschied zwischen den Wegen: auf dem zweiten leert der Aufrufer vorher, auf
  dem ersten nicht, weil dort der Bestand das ist, was AppKit gebaut hat.
- Zwei Zählproben auf den Quellbaum, unten eigens beschrieben.

**`crates/krk-ui/src/appkit/tabelle.rs`**

`DateifensterQuelle` ist jetzt `NSMenuDelegate`. `menuNeedsUpdate:` liest die
betroffenen Einträge, leert das Menü und ruft den Bauer. `Dateifenster::bauen`
hängt ein leeres `NSMenu` an die `NSTableView` und setzt die Quelle als seinen
Delegierten. Der SAFETY-Block nennt die Haltrichtungen: das Menü hält den
Delegierten schwach (`NSMenu.h:156`), die Tabelle hält das Menü stark, und
`Dateifenster` hält die Quelle — der Ring bleibt an der letzten Kante offen.

Die Ausleihe des Tabmodells endet in der ersten Zeile von `menuNeedsUpdate:`,
vor dem ersten Objective-C-Aufruf. Das ist die Regel, die der Modulkopf jener
Datei für jede Ausleihe aufstellt, und sie hat hier einen scharfen Grund: das
Menü ruft in dieselbe Quelle zurück.

**`crates/krk-ui/src/appkit/editor.rs`**

`Editorbereich` war bereits `NSTextViewDelegate` mit leerem Rumpf; er beantwortet
jetzt `textView:menu:forEvent:atIndex:`. Geteilt wird die Datei, die der Editor
hält. Eine Abfrage über `angezeigtedatei::welche` steht dort **nicht**, und der
Doc-Kommentar sagt warum: das Menü geht nur auf, wo der Nutzer hinklickt, und
geklickt hat er in den sichtbaren Editor. Die Sichtbarkeitsfrage wäre eine
zweite Antwort auf etwas, das der Klick schon beantwortet hat.

**`crates/krk-ui/src/appkit/vorschau.rs`**

`Vorschaufenster` wird der Delegierte seiner Textanzeige (`NSTextDelegate` leer,
weil `NSTextViewDelegate` ihn voraussetzt, plus der Menühaken) und der Delegierte
**eines** Menüs, das Bildansicht und Inhaltsfläche sich teilen. Dazu die private
`teilbare_pfade`, keine oder eine Datei: ein Tab zeigt höchstens eine.

**Alle drei Ansichten bekommen das Menü, und das ist keine Vorsicht ohne Anlass.**
Wo ein Rechtsklick in der Vorschau ankommt, hängt am Inhalt. Ob eine Ansicht ohne
eigenes Menü die rechte Maustaste an ihre Übergeordnete weiterreicht, ist eine
Zusage von AppKit, die wir nicht gelesen haben, und eine Fläche ohne Menü wäre
der stille Fehlschlag, den C1.6 ausschließt.

## Zwei Punkte, die in den Bericht gehören

**`method_id` statt `method` für den Menühaken.** Die Delegiertenmethode liefert
`NSMenu *` zurück, und `define_class!` nimmt einen `Option<Retained<…>>` nur unter
`#[unsafe(method_id(…))]` entgegen; unter `#[unsafe(method(…))]` fehlt dem
Rückgabetyp `Encode`, und der Bau hielt mit zwölf Fehlern an. Die Familie ist
`none`, der Eintrag geht also autoreleased und +0 zurück, wie das Protokoll es
verlangt. Beide Flächen, Editor und Vorschau, nehmen dieselbe Schreibweise.

**Ein Menü für zwei Ansichten der Vorschau, nicht zwei.** Ein zweites trüge
denselben einen Eintrag und bräuchte denselben Delegierten. Ein Kontextmenü ist
kein Untermenü und hat keinen Elternteil, zwei Ansichten dürfen dasselbe Objekt
halten; der SAFETY-Block sagt es. Welche der beiden angeklickt wurde, ändert die
Antwort nicht — geteilt wird die Datei des aktiven Tabs, ob sie als Bild oder als
Text dasteht.

## Die zwei Zählproben

Der Plan verlangt sie „wie sie das Projekt für die Kistengrenze von
`hervorhebung.rs` schon führt". **Dort ist es eine Messung von Hand**, kein
Prüffall: der Modulkopf von `hervorhebung.rs` sagt „die Abnahme von S16 misst die
Grenze, indem sie den Kistennamen zählt". Eine Zählprobe **im Baum** gab es
bisher nicht; diese beiden sind die ersten. Sie lesen `crates/krk-ui/src/` über
`env!("CARGO_MANIFEST_DIR")` rekursiv ein und laufen bei jedem `make check` mit.

| Probe | Was sie hält |
|---|---|
| `appkit::teilen::tests::allein_diese_datei_baut_den_freigabewaehler` | genau ein Aufrufer von `NSSharingServicePicker` (C1.7, C1.8) |
| `appkit::teilen::tests::es_gibt_genau_einen_menuebauer` | genau eine Erklärung von `eintrag_anfuegen` und genau ein Holen von `standardShareMenuItem` (C1.7) |

Zwei Eigenheiten sind im Code begründet und gehören hierher:

- **Die Nadel der ersten Probe trägt zwei Doppelpunkte**,
  `NSSharingServicePicker::`. Ohne sie schlug die Probe an `appkit/mod.rs` fehl,
  das die Klasse in seinem Modulkopf in Prosa **nennt**. Nennen ist keine
  Berührung. Wer einen Wähler **baut**, kommt an `::alloc` und `::initWithItems`
  nicht vorbei, und an einen fertigen kommt er nicht heran, weil aus `teilen.rs`
  keiner herauskommt. Die Schreibweise trennt damit genau die beiden Fälle.
- **Die Nadeln der zweiten Probe stehen zusammengesetzt da**, über `concat!`.
  Die Probe liegt in dem Baum, den sie liest; als ein Stück geschrieben fände
  jede Nadel sich selbst und zählte eine Fundstelle zu viel.

## Am SDK gegengelesen

Alle Untergrenzen sind am SDK gelesen (`MacOSX.sdk`, AppKit-Kopfdateien) und
nicht aus dem Plan übernommen. **Eine Zahl weicht vom Plan ab:**

| Berührung | Ort | Ab | Plan |
|---|---|---|---|
| `textView:menu:forEvent:atIndex:` | `NSTextView.h:628` | 10.5 | 10.5 ✓ |
| `@protocol NSMenuDelegate` | `NSMenu.h:269` | keine Angabe | 10.0 ✓ |
| `menuNeedsUpdate:` | `NSMenu.h:271` | keine Angabe | 10.0 ✓ |
| `NSResponder.menu` | `NSResponder.h:111` | keine Angabe | 10.0 ✓ |
| `NSMenu.delegate` | `NSMenu.h:156` | keine Angabe | — |
| `@protocol NSTextDelegate` | `NSText.h:200` | keine Angabe | — |
| `@protocol NSTextViewDelegate` | `NSTextView.h:576` | keine Angabe | — |
| **`NSMenu.removeAllItems`** | **`NSMenu.h:112`** | **10.6** | 10.0 ✗ |

Der Plan führt `removeAllItems` unter denen, die „keine eigene Angabe tragen und
damit seit 10.0 stehen". Der Kopf sagt `API_AVAILABLE(macos(10.6))`. Die Zahl
liegt weit unter der Untergrenze des Bündels und ändert nichts an der Sache; die
drei Modulköpfe nennen 10.6 und nicht 10.0.

Das Bündel zielt auf 15.0. Keine Berührung dieses Schritts braucht eine
Verfügbarkeitsprüfung zur Laufzeit.

**Die Deckung des Abschnitts `# Ab welchem macOS die angesprochenen Klassen
stehen` ist nicht gesunken.** Am 260812 rekursiv nachgezählt mit `grep -rL` über
`crates/krk-ui/src/appkit/`: **33 von 35 Dateien**, ohne ihn weiterhin allein
`koordinaten.rs` und `mod.rs`, beide begründet. Alle drei berührten Dateien
tragen ihn und haben ihn um die neuen Berührungen ergänzt.

**Berichtigt am 260812-1600.** Diese Stelle nannte zuerst „23 von 25" und
erklärte daraufhin die Zahlen in CLAUDE.md („31 von 33") und im Bericht zu
Schritt 5 („33 von 35") für beide falsch. Der Fehler lag hier: gezählt war
`crates/krk-ui/src/appkit/*.rs`, also allein die oberste Ebene, und unter
`appkit/` liegt daneben das Unterverzeichnis `blaetter/` mit zehn weiteren
Dateien. Rekursiv sind es 35 Dateien, 33 davon mit dem Abschnitt. **Der Bericht
zu Schritt 5 hat damit recht behalten**, und offen bleibt allein die Zahl in
CLAUDE.md. Der Defekt
`shared/issues/260812-1438_o_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`
trägt sie bereits und braucht von hier nichts weiter; dieser Schritt hat ihn
nicht angefasst.

## Abnahme

Vier Kommandos, keine Messung, kein Bündelbau, kein Vordergrundlauf:

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

405 Proben im Binärziel `krk`, gegenüber 403 vorher: die beiden Zählproben. Kein
bestehender Prüffall ist angefasst worden.

Von C1 ist mit diesem Schritt ohne laufendes Bündel nachgewiesen: **C1.7** in
beiden Hälften, jetzt durch eine Probe und nicht mehr durch Augenschein.
**Am Bündel zu sehen und Nutzerarbeit bleiben C1.1, C1.6 und die zweiten Hälften
von C1.4 und C1.5.**

## Datensätze

`decisions/260812-1000_a_an-welchen-drei-flaechen-haengt-das-neue-kontextmenue.md`
ist von beantwortet auf umgesetzt gezogen (`_a_` → `_i_`) und trägt die
`Implemented:`-Zeile mit den drei Dateien. Möglichkeit 1 jenes Datensatzes ist
gebaut: alle drei Flächen, ein Bauer.

## Ein Defekt, und er wiegt

`decisions/260812-1145_a_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md`
**ist seit dem 260812-1200 beantwortet, und zwar mit Möglichkeit 2** — der
Rechtsklick setzt die Auswahl auf die angeklickte Zeile, es sei denn, sie ist
bereits markiert. Der Datensatz lehnt Möglichkeit 1 ausdrücklich ab.

Der Plan ist um 1145 geschrieben, die Antwort um 1200 gegeben. Der Wortlaut von
Schritt 6 führt den Datensatz seither als offen, verlangt Möglichkeit 1 und sagt
„solange sie offen ist, gilt die Regel ohne Ausnahme"; die Aufgabenstellung an
diesen Agenten hat den Wortlaut wiederholt. Gebaut ist deshalb Möglichkeit 1, und
damit steht der Code gegen einen bindenden Nutzerentscheid.

Nicht behoben, und der Grund gehört dazu: die Aufgabenstellung verlangt
Möglichkeit 1 wörtlich, und eine Abweichung auf eigene Faust wäre eine
Entscheidung des Agenten über eine Verhaltensfrage, die der Nutzer bereits
entschieden hat. Der Widerspruch gehört ihm vorgelegt.

Gefiled als
`issues/260812-1500_o_der-rechtsklick-bewegt-die-auswahl-nicht-obwohl-der-nutzerentscheid-es-verlangt.md`,
mit dem Weg zur Behebung: `clickedRow` in `menue_auffrischen`, die Auswahl über
`auswahl_merken`, keine zweite Auswahlregel. Die Änderung ist klein und sitzt
allein in `tabelle.rs`.
