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

---

## Der Preis ist gemessen, und der Datensatz bleibt offen

Gemessen am 260810-1057 auf diesem Gerät, `--release`, aus einem Prüfziel mit
`harness = false`, das den Hauptfaden selbst hält. Der Text ging als `NSString`
herein, wie ihn `NSTextView::string` liefert, und dann denselben Weg, den
`text_zurueckschreiben` geht:

```
       Byte   NSString->String   ist_in_gehaltener_form   in_gehaltene_form   Summe
    229 029           0,98 ms                  0,017 ms            0,015 ms   1,02 ms
  1 832 232           7,61 ms                  0,130 ms            0,131 ms   7,87 ms
 19 467 465          88,30 ms                  1,805 ms            1,858 ms  91,96 ms
```

**Der Preis liegt zu 96 Prozent im Umschreiben aus UTF-16 und nicht in der
Wandlung.** Die drei Posten, die dieser Datensatz aufzählt, sind damit anders
gewichtet, als er annahm: Posten 1 (die Kopie aus dem Textspeicher) ist der
ganze Preis, Posten 2 (`in_gehaltene_form`) kostet zwei Prozent, und Posten 3
(die zweite vollständige Kopie bei `\r\n`) fällt **nicht je Anschlag** an,
sondern nur auf dem Weg, der ohnehin die Fläche neu beschreibt — `in_gehaltene_form`
gibt einen Text in gehaltener Form ohne eine einzige Kopie zurück.

**Ab wann es stockt:** bei 229 kB sind 1,0 ms je Anschlag nicht zu bemerken, bei
1,8 MB sind 7,9 ms noch unter einer Bildlänge von 16,7 ms, und bei 19 MB sind
92 ms rund fünf Bildlängen — dort stockt das Tippen sichtbar. Die Grenze des
Editors liegt bei 16 MB, also liegt der schlechteste erreichbare Fall bei rund
75 ms je Anschlag.

## Warum er offen bleibt

**Der Schnitt, der ihn erledigt, liegt außerhalb dieser Datei.** Ein Rückweg, der
nur die geänderte Stelle nimmt, verlangt eine Änderung an
`Editormodell::bearbeiten` und an der Normalisierungsstelle in
`krk-core/src/text/datei.rs`; beides lag in dieser Sitzung nicht in der
Dateigrenze, an beiden arbeiteten parallel andere Agenten. Behoben ist allein der
Nachbardefekt `260810-0054`, und dessen Antwort trägt hier nicht.

**Die Annahme, beide Stellen lebten von derselben Antwort, ist damit widerlegt**,
und zwar mit Zahlen. Drei Unterschiede:

- **Verschiedene Fäden.** Dieser Preis fällt auf dem **Hauptfaden** an und stockt
  das Tippen; der Preis aus `260810-0054` fällt auf einem Arbeitsfaden an und
  lässt die Oberfläche bedienbar.
- **Verschiedene Größenordnungen.** 92 ms bei 19 MB gegen 7 000 ms bei 1,8 MB.
- **Verschiedene Angaben.** Das Fortschreiben der Einfärbung braucht
  `editedRange` aus `NSTextStorage` **nicht**: ein Zeilenvergleich findet die
  geänderte Zeile in 0,13 bis 12 ms selbst, und genau so ist er gebaut. Dieser
  Defekt braucht die geänderte Stelle dagegen wirklich, weil ohne sie der ganze
  Text aus UTF-16 umgeschrieben werden muss, um überhaupt vergleichen zu können.

Wer ihn angeht, braucht deshalb eine eigene Antwort und keine übernommene:
entweder `editedRange` und `changeInLength` am Eingang der Fläche, oder einen
Stand, der nicht als `String` gehalten wird.

**Aufgefallen bei:** der Messung am 260810-1057, im Durchgang zu den fünf
Datensätzen um die Textfläche.
