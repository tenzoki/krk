# Die Textfläche schaltet `allowsUndo` nicht ein und hat damit kein Rückgängig

---
**Domain:** code
**Schwere:** High
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:276-307` (`textflaeche_bauen`)
**Cross-references:** `crates/krk-ui/src/appkit/menue.rs:55-61` und `:209-223`, `resources/default-keymap.toml:607-616`, S7, S16

---

## Der Befund

`textflaeche_bauen` setzt neun Eigenschaften an der `NSTextView`. `allowsUndo`
ist nicht darunter:

```rust
// editor.rs:287-304
let text = NSTextView::initWithFrame(NSTextView::alloc(mtm), rahmen);
text.setEditable(true);
text.setSelectable(true);
text.setRichText(false);
text.setAutomaticQuoteSubstitutionEnabled(false);
text.setAutomaticDashSubstitutionEnabled(false);
text.setAutomaticTextReplacementEnabled(false);
text.setAutomaticSpellingCorrectionEnabled(false);
text.setVerticallyResizable(true);
text.setHorizontallyResizable(false);
…
```

`NSTextView.allowsUndo` steht bei einer programmatisch erzeugten Textansicht ab
Werk auf `NO`. Solange sie es tut, **registriert die Textansicht keine einzige
Rückgängig-Handlung**. Der Setzer steht in den Bindungen zur Verfügung
(`objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1218-1221`,
`setAllowsUndo:`); er wird nicht gerufen.

## Warum das zählt

S7 hat die beiden Menüeinträge "Rückgängig" und "Wiederholen" gebaut, und
`menue.rs:55-61` begründet sie so:

> **`undo:` und `redo:` liegen genauso.** Die `NSTextView` des Editors bringt
> ihren Rückgängigverwalter mit, aber Cmd+Z und Shift+Cmd+Z erreichen ihn nur
> über ein Menükürzel. Ohne die beiden Einträge "Rückgängig" und "Wiederholen"
> hätte der Editor kein Rückgängig.

Der erste Halbsatz stimmt nicht. Die `NSTextView` bringt ihren
Rückgängigverwalter erst mit, **nachdem** jemand `setAllowsUndo(true)` gerufen
hat; vorher gibt es nichts zu verwalten. Die beiden Menüeinträge laufen die
Antwortkette hinunter und finden am Ende einen Rückgängigverwalter ohne einen
einzigen eingetragenen Vorgang. Der Eintrag ist grau, und der Editor hat kein
Rückgängig — genau der Zustand, den S7 ausschließen wollte.

Die Lücke fällt zwischen zwei Schritte: S7 baut das Menü, S16 baut die
Textfläche, und die eine Zeile, die beides verbindet, hat keinen von beiden.
Beide Schritte tragen `[DONE]`.

Der Abnahmelauf fängt es nicht: S7s Abnahmekriterium prüft `make menue` auf zwei
Ausgabezeilen, und dass Rückgängig **wirkt**, ist ausdrücklich auf S42 und
`Nutzerarbeit` verschoben.

## Vorschlag

Eine Zeile in `textflaeche_bauen`, bei den übrigen Eigenschaften:

```rust
// Ohne diese Zeile traegt die Textansicht keinen Rueckgaengigverwalter, und
// die beiden Menueeintraege aus S7 laufen ins Leere.
text.setAllowsUndo(true);
```

Dazu die Begründung in `menue.rs:55-61` berichtigen: die Textansicht bringt
ihren Verwalter nicht mit, sondern bekommt ihn in `editor.rs` eingeschaltet, und
die Menüeinträge sind die zweite Hälfte derselben Sache.

Zu prüfen ist dabei eine Wechselwirkung mit `Editorbereich::stand_einsetzen`:
`setString:` schreibt an der Rückgängigverwaltung vorbei und lässt einen bereits
gefüllten Stapel stehen, der auf einen anderen Text zeigt. Beim Öffnen einer
neuen Datei gehört der Stapel geleert (`undoManager.removeAllActions()`), sonst
nimmt ein Cmd+Z den Text der vorigen Datei zurück. Das ist die Stelle von S24;
hier gehört der Vermerk hin, damit er nicht verlorengeht.

Gemeldet von: `coderev`, Durchsicht Turn 2.

---
Resolved: `textflaeche_bauen` setzt `text.setAllowsUndo(true)`, mit dem Grund
daneben (`crates/krk-ui/src/appkit/editor.rs`). Die Begründung im Modulkopf von
`crates/krk-ui/src/appkit/menue.rs` ist nachgezogen: die `NSTextView` bringt
ihren Rückgängigverwalter mit, benutzt ihn aber erst mit `allowsUndo`, und die
beiden Menüeinträge sind die zweite Hälfte derselben Sache.

Der Vermerk zu `stand_einsetzen` und `removeAllActions()` ist nicht mit
geschlossen worden, sondern abgetrennt nach
`260809-1727_o_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`;
`stand_einsetzen` trägt einen Doc-Absatz, der dorthin zeigt. Er blieb offen,
weil `NSUndoManager` möglicherweise eine Zeile in `crates/krk-ui/Cargo.toml`
braucht, und die lag außerhalb des Umfangs dieses Schrittes.

Dass Rückgängig am laufenden Bündel **wirkt**, bleibt wie in S7 vorgesehen
Nutzerarbeit (S42); geprüft ist hier allein, dass die Zeile steht und der
Arbeitsbereich grün bleibt.
