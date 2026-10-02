Die Grundschrift der zwei Textflächen entstand an drei Stellen, und der Modulkopf sagt: an einer
---
`textmerkmale::grundschrift` (`crates/krk-ui/src/appkit/textmerkmale.rs`) ist nach dem Doc-Kommentar über ihr „die eine Stelle", an der die Schriftgröße von Editor und Vorschau entsteht — „zwei Rechnungen daneben wären die erste Gelegenheit, dass eine gelöschte Überschrift in einer anderen Schrift landete als der, in der ihre Zeile getippt wird". Zwei solche Rechnungen standen daneben: `textmerkmale::anwenden` schrieb `NSFont::systemFontSize() + LESEZUSCHLAG` als Literal aus, und `editor::textflaeche_bauen` setzte die Schrift der frisch gebauten Fläche mit `NSFont::userFixedPitchFontOfSize(NSFont::systemFontSize())` selbst.
---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

Gefunden am 260907 beim Umstellen der Grundschrift auf die kleine Systemschriftgröße (K13, Auftrag 1). **Am Baum von vorher war keine der beiden Stellen sichtbar falsch**, und das ist der Grund, aus dem sie so lange standen: alle drei Rechnungen setzten auf `NSFont::systemFontSize()` auf und lieferten dieselben Zahlen. Ein Fehler wurden sie erst in dem Augenblick, in dem jemand die Grundlage ändert — also genau bei der Arbeit, für die der Doc-Kommentar die Warnung geschrieben hat.

Was jede der beiden Stellen gekostet hätte:

- **`anwenden`.** Die Überschriftsstufen, die beiden Betonungen und der Quelltextblock rechnen über der `grundgroesse` dieser Funktion. Wäre allein `grundschrift` heruntergezogen worden, stünden Markdown-Überschriften weiter über einer Grundlage von 13 Punkten, während ihr Fließtext auf 11 sitzt; die Stufenfolge aus `UEBERSCHRIFTSFAKTOREN` bezöge sich damit auf eine Größe, die auf der Fläche nirgends vorkommt.
- **`textflaeche_bauen`.** Die Fläche des Editors entstünde in der alten Größe und bekäme die neue erst mit dem nächsten `darstellung_nachziehen`. Die Vorschau hat diesen Fehler nicht: ihre `textanzeige` ruft seit der Runde 6 `grundschrift` und begründet das an Ort und Stelle.

Der Doc-Kommentar von `LESEZUSCHLAG` sagt, es sei der Abstand der Formatansicht „über die der Rohansicht"; mit einem eigenen Literal in `anwenden` wäre auch das nicht mehr wahr gewesen.

---
Resolved: 260907-1717 vom coder, im selben Auftrag, in dem der Befund entstand. Die Fallunterscheidung über Schriftart und Größe steht jetzt einmal, in `textmerkmale::grundmerkmale`; `grundschrift` und die neue `grundgroesse` fragen sie, `anwenden` ruft `grundgroesse(ansicht, art)` statt zu rechnen, und `editor::textflaeche_bauen` setzt `textmerkmale::grundschrift(Ansicht::Roh, Darstellungsart::EinfacherText)` — dieselbe Zeile, die die Vorschau schon hatte. Der Doc-Kommentar von `grundschrift` spricht seitdem von drei Aufrufern statt von zweien und verweist für die Größe auf `grundmerkmale`; `LESEZUSCHLAG` sagt ausdrücklich, dass er ein Zuschlag auf die dortige Grundlage ist und mitgeht, wenn die sich ändert.

Abnahme gefahren: `grep -n 'NSFont::systemFontSize()\|NSFont::smallSystemFontSize()' crates/krk-ui/src/appkit/textmerkmale.rs crates/krk-ui/src/appkit/editor.rs` nennt zwei Zeilen — `textmerkmale.rs:407` in `grundmerkmale`, die Grundlage der zwei Textflächen, und `editor.rs:3162` in `kopf_bauen`, die Beschriftung über der Fläche und keine Textfläche. Keine der beiden nennt mehr `systemFontSize`. Dazu `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` und `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, je Ausgang 0.

**Was die Abnahme nicht deckt:** dass die neue Größe auf der Fläche richtig aussieht. Das verlangt das laufende Bündel und ist Nutzerarbeit.
