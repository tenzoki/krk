Der ganze Stand geht je Tastendruck durch `bearbeiten`

---

Entstanden mit S26 am 260809-2322, als der Rückweg aus der Textfläche ins Modell
gebaut wurde. Kein Fehlverhalten: der Editor rechnet richtig. Was hier steht, ist
ein benannter Preis, der ungemessen ist und deshalb nicht als „geprüft" gelten
darf.

---

## Der Befund

`Editorbereich::text_zurueckschreiben` (`crates/krk-ui/src/appkit/editor.rs`)
holt bei **jedem** `textDidChange:` den vollständigen Text aus der `NSTextView`
und reicht ihn an `Editormodell::bearbeiten`. Das kostet je Tastendruck:

1. eine Kopie des ganzen Textes aus dem Textspeicher in eine Rust-Zeichenkette,
2. einen Durchlauf von `krk_core::text::datei::in_gehaltene_form` darüber,
3. bei einer Datei mit `\r\n` zusätzlich eine zweite vollständige Kopie.

Bei einer Datei an der Grenze von 16 MB (`datei::EDITORGRENZE`) sind das 16 MB
Kopie und 16 MB Durchlauf je Anschlag, auf dem Hauptfaden.

`speculation:` **Ungemessen.** Es gibt keine Messung, ab welcher Dateigröße das
Tippen auf dem Referenzgerät stockt, und diese Runde misst es nicht: der
Abnahmelauf ist aus ihr ausgeklammert.

## Warum es trotzdem so gebaut ist

Der Modulkopf von `editormodell.rs` führt genau zwei Eingänge für fremden Text,
und beide gehen durch `in_gehaltene_form`, die eine Normalisierungsstelle des
Programms. Der größere dieser beiden ist der ganze Stand aus der Textfläche.
Ein Rückweg, der nur die geänderte Stelle nähme, wäre ein dritter Eingang und
bräuchte eine zweite Wahrheit darüber, was der gehaltene Stand ist.

Der Plan hat diese Bauart gewählt (S26, `editormodell.rs` Modulkopf), und der
Defekt `260809-2148` hat den dritten Weg — den Stand erst beim Sichern aus der
Fläche holen — ausdrücklich verworfen.

## Der Ausweg, falls er gebraucht wird

Benannt und nicht zu suchen: `NSTextStorage` meldet mit jeder Änderung den
geänderten Bereich (`editedRange`) und die Längenänderung (`changeInLength`)
mit. Ein Stand, der sich daran fortschreibt, kostet die geänderte Stelle statt
der ganzen Datei. Er gehörte nach `krk-core` und wäre dort ohne Fenster
abzunehmen — dieselbe Antwort, die der Plan für den Zeilenindex der
Nummernspalte (S46) in seiner Risikotabelle führt. Beide Stellen stellen
dieselbe Frage und würden von derselben Antwort leben.

## Was zuerst zu tun wäre

Messen, nicht bauen. Eine Datei von 16 MB im Editor öffnen und tippen; stockt es
nicht, bleibt dieser Datensatz offen und unbearbeitet liegen. Das verlangt KRK im
Vordergrund und ist damit Nutzerarbeit.

**Aufgefallen bei:** dem Bau von S24 und S26 am 260809-2322.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (Schritt 26, Risikotabelle Zeile „Neuaufbau des Zeilenindex"),
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-2148_c_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`
