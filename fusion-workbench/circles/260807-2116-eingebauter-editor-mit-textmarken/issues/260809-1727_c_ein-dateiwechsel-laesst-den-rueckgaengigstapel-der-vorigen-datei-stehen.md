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

---

Resolved: 260810-0310, `coder`. `Editorbereich::stand_einsetzen` ruft hinter
`setString:` die neue Funktion `rueckgaengigstapel_leeren`, und die ruft
`NSUndoManager::removeAllActions`. Der Verwalter kommt über
`NSResponder::undoManager`; vor dem Einhängen der Fläche in ein Fenster liefert
er `None`, und dort ist auch nichts zu leeren.

**Die Vorprüfung aus dem Abschnitt „Vorschlag" ist gefahren:**
`NSUndoManager` steht in `objc2-foundation` bereits zur Verfügung, weil
`objc2-app-kit` das Merkmal `objc2-foundation/NSUndoManager` selbst einschaltet
(`cargo tree -p krk-ui -i objc2-foundation -e features`). `crates/krk-ui/Cargo.toml`
brauchte keine Zeile.

**Zwei Dinge, die die Umsetzung gegenüber dem Vorschlag geklärt hat:**

- **Der Verwalter gehört dem Fenster, nicht der Textfläche.**
  `NSResponder::undoManager` geht die Antwortkette hinauf. Wer sonst noch in
  demselben Fenster Rückgängig-Handlungen anmeldete, verlöre sie mit. Heute ist
  das niemand: der Editor ist die einzige Fläche in KRK, die `allowsUndo`
  einschaltet.
- **Eine offene Gruppe hält `removeAllActions` nicht auf.** `setString:` fällt
  mitten in die Ereignisbehandlung, und `NSUndoManager` gruppiert ab Werk je
  Ereignis; zur Aufrufzeit kann eine Gruppe offen stehen. Gemessen, nicht
  angenommen: die Probe
  `ein_geleerter_stapel_traegt_auch_eine_offene_gruppe_nicht_mehr` lässt eine
  Gruppe offen und prüft danach `canUndo() == false` und `groupingLevel() == 0`.

**Drei Proben halten die Zusage fest**, alle in `crates/krk-ui/src/appkit/editor.rs`
unter `mod tests`, alle ohne Fenster: `NSUndoManager` steht für sich.

- `ein_geleerter_stapel_traegt_keine_rueckgaengig_handlung_mehr`
- `ein_geleerter_stapel_traegt_auch_eine_offene_gruppe_nicht_mehr`
- `ohne_verwalter_geschieht_nichts`

**Der Preis ist abgetrennt und nicht stillschweigend erledigt.** Der Stapel ist
nach einem Ersetzen und nach dem Nachrichten der Fläche jetzt **leer** statt
falsch: `cmd+z` tut dort nichts, statt gegen einen Stand zu wirken, den die
Fläche nicht mehr trägt. Das ist die kleinere Fehlwirkung und ein eigener
Defekt,
`issues/260810-0303_o_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`.
Er ist nicht mit behoben, weil Dateiwechsel und Ersetzen an derselben einen
Schreibstelle entgegengesetzte Behandlungen verlangen.
