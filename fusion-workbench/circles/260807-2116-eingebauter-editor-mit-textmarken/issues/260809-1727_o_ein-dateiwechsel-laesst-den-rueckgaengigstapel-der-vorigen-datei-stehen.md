# Ein Dateiwechsel lässt den Rückgängigstapel der vorigen Datei stehen

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev (Durchsicht Turn 2), abgetrennt beim Schließen von `260809-1644` durch coder
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::stand_einsetzen`)
**Cross-references:** `issues/260809-1644_c_die-textflaeche-schaltet-allowsundo-nicht-ein-und-hat-damit-kein-rueckgaengig.md`, S24

---

## Der Befund

Seit `textflaeche_bauen` `setAllowsUndo(true)` setzt, trägt die `NSTextView`
einen gefüllten Rückgängigstapel. `stand_einsetzen` ersetzt den Text der Fläche
über `setString:`, und `setString:` schreibt an der Rückgängigverwaltung vorbei:
der Stapel bleibt stehen und zeigt danach auf den Text der **vorigen** Datei.
Ein Cmd+Z nach einem Dateiwechsel nimmt eine Änderung an einem Text zurück, der
nicht mehr in der Fläche steht.

Der Vermerk stand im Defekt `260809-1644` und ist beim Schließen von dort
hierher gezogen worden, damit er nicht verlorengeht.

## Warum er heute nicht auffällt

`stand_einsetzen` hat genau einen Aufrufer, `Editorbereich::bauen`, und dabei
ist die Fläche leer und der Stapel es auch. Erreichbar wird der Fall mit dem
Öffnen aus Schritt 24, das eine zweite Datei in dieselbe Fläche setzt.

## Vorschlag

`stand_einsetzen` leert den Stapel, unmittelbar vor oder nach `setString:`:

```rust
if let Some(verwalter) = self.ivars().text.undoManager() {
    verwalter.removeAllActions();
}
```

`stand_einsetzen` ist die richtige Stelle dafür, weil sie ausweislich ihres
eigenen Doc-Kommentars die **eine** Stelle ist, die den Text der Fläche
ersetzt; ein Aufrufer, der es selbst täte, wäre die zweite.

**Zu prüfen ist vorher, ob `NSUndoManager` in den Merkmalen der Kiste
`objc2-foundation` überhaupt angeschaltet ist.** Ist er es nicht, braucht der
Schritt eine Zeile in `crates/krk-ui/Cargo.toml`, und die lag außerhalb des
Umfangs des Schrittes, der diesen Defekt abgetrennt hat.

---
Nachtrag vom 260810-0220, `coder`, bei der Umsetzung von S37: **der Fall hat
einen zweiten Auslöser bekommen, und er trifft dieselbe Datei.**

`Editorbereich::treffer_ersetzen` und `alle_treffer_ersetzen` schreiben den
geänderten Stand über `stand_erneuern` in die Textfläche zurück, also über
`setString:`. Der Rückgängigstapel zeigt danach auf den Text vor diesem
Schreibvorgang; ein `cmd+z` unmittelbar nach einem Ersetzen wirkt gegen einen
Stand, den die Fläche nicht mehr trägt.

Bisher stand hier nur der Dateiwechsel, bei dem der stehengebliebene Stapel
wenigstens zu einer **anderen** Datei gehört. Der neue Weg ist häufiger und
näher am Nutzer: er ersetzt und macht rückgängig, ohne die Datei gewechselt zu
haben.

Die Behebung ist unverändert dieselbe und gehört nach wie vor an
`stand_einsetzen`, nicht an die beiden Ersetzungswege — es gibt genau eine
Stelle, die den Text der Fläche ersetzt, und dort ist der Stapel zu leeren
(`NSUndoManager::removeAllActions`) oder das Schreiben rückgängigfähig zu
machen.
