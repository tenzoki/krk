# Ein Schreiben in die Textfläche lässt keinen Rückgängigstapel mehr stehen

**Status:** Complete
**Agent:** coder
**Domain:** code
**Circle:** 260807-2116-eingebauter-editor-mit-textmarken
**Behobener Defekt:** `issues/260809-1727_c_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`
**Abgetrennter Defekt:** `issues/260810-0303_o_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`

---

## Die Ursache

`textflaeche_bauen` schaltet seit der Behebung von `260809-1644`
`setAllowsUndo(true)` ein, und die `NSTextView` trägt seither einen gefüllten
Rückgängigstapel. `Editorbereich::stand_einsetzen` ersetzt den Text der Fläche
über `setString:`, und `setString:` schreibt an der Rückgängigverwaltung vorbei:
der Stapel blieb stehen und zeigte danach auf einen Text, den die Fläche nicht
mehr trug.

Drei Wege erreichen die Stelle, und sie sind nacheinander dazugekommen:

```
  Dateiwechsel (S22)   ──┐
  Ersetzen (S37)       ──┼──> stand_erneuern ──> stand_einsetzen ──> setString:
  flaeche_richten      ──┘                                            (d5993f1)
```

Der Dateiwechsel ist der schwerste Fall: der stehengebliebene Stapel gehörte zu
einer **anderen** Datei, und ein `cmd+z` danach nahm eine Änderung an einem Text
zurück, der nicht mehr im Fenster stand.

## Die Behebung

`stand_einsetzen` ruft hinter `setString:` die neue Funktion
`rueckgaengigstapel_leeren`, und die ruft `NSUndoManager::removeAllActions`. Die
Stelle ist die, die der Datensatz verlangt: `stand_einsetzen` ist ausweislich
des Modulkopfs die **eine** Stelle, die den Text der Fläche ersetzt, und ein
Aufrufer, der es selbst täte, wäre die zweite.

**Die Vorprüfung aus dem Datensatz ist gefahren.** `NSUndoManager` steht in
`objc2-foundation` bereits zur Verfügung, weil `objc2-app-kit` das Merkmal
`objc2-foundation/NSUndoManager` selbst einschaltet; nachgesehen mit
`cargo tree -p krk-ui -i objc2-foundation -e features`. `crates/krk-ui/Cargo.toml`
brauchte keine Zeile.

## Zwei Eigenschaften, die die Umsetzung geklärt hat

**Der Verwalter gehört dem Fenster und nicht der Textfläche.**
`NSResponder::undoManager` geht die Antwortkette hinauf; vor dem Einhängen der
Fläche in ein Fenster liefert er `None`, und der erste Aufruf aus
`Editorbereich::bauen` kommt genau dorthin. Wer sonst noch in demselben Fenster
Rückgängig-Handlungen anmeldete, verlöre sie mit. Heute ist das niemand: der
Editor ist die einzige Fläche in KRK, die `allowsUndo` einschaltet.

**Eine offene Rückgängig-Gruppe hält `removeAllActions` nicht auf.** `setString:`
fällt mitten in die Ereignisbehandlung, und `NSUndoManager` gruppiert ab Werk je
Ereignis: zur Aufrufzeit kann eine Gruppe offen stehen. Das ist gemessen und
nicht angenommen, und es ist der Grund, aus dem die Funktion `removeAllActions`
nimmt und nicht `endUndoGrouping` — das zweite verlangt eine offene Gruppe und
wirft ohne eine.

## Die Prüfung des Preises, den `flaeche_richten` mitbrachte

Der Doc-Kommentar von `flaeche_richten` (Commit `d5993f1`, Behebung von
`260810-0215`) hielt fest, dass ein `cmd+z` gleich nach einem eingefügten CRLF
gegen einen Stand wirkt, den die Fläche nicht mehr trägt. **Diese Fehlwirkung
nimmt die Behebung mit**: der Stapel ist danach leer statt falsch.

**Ein Rest bleibt und ist als eigener Defekt abgelegt.** Nach einem Ersetzen und
nach einem eingefügten CRLF tut `cmd+z` jetzt nichts, statt das Falsche zu tun.
Der Verlauf des Nutzers geht dabei verloren, und beim Sammelersetzen
(`ctrl+cmd+r`, eine Änderung an der ganzen Datei in einem Zug) wiegt das am
schwersten.

Warum er nicht mit behoben ist: Dateiwechsel und Ersetzen verlangen an derselben
einen Schreibstelle **entgegengesetzte** Behandlungen.

```
                    Rückgängigstapel danach
  Dateiwechsel  ──> muss leer sein      (sonst zeigt er auf eine andere Datei)
  Ersetzen      ──> soll gefüllt sein   (der Nutzer will es zurücknehmen)
  CRLF-Richten  ──> soll gefüllt sein   (der Nutzer will es zurücknehmen)
```

`stand_einsetzen` kennt ihren Anlass nicht. Ein rückgängigfähiger Schreibweg
(`shouldChangeTextInRange:replacementString:`, `replaceCharactersInRange:withString:`,
`didChangeText`) löste zudem über `didChangeText` den Rückweg zum Modell aus,
und eine frisch geöffnete Datei trüge danach sofort das Abweichungszeichen — die
Annahme, die der Modulkopf von `editor.rs` ausdrücklich führt. Der Zuschnitt
gehört in einen eigenen Durchgang:
`issues/260810-0303_o_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`.

Der gewöhnliche Anschlag ist nicht betroffen: `text_zurueckschreiben` ruft
`flaeche_richten` nur, wenn `Editormodell::bearbeiten` `true` liefert, also wenn
die Wandlung zugegriffen hat.

## Die Proben

Drei, alle in `crates/krk-ui/src/appkit/editor.rs` unter `mod tests`, alle ohne
Fenster: `NSUndoManager` steht für sich.

- `ein_geleerter_stapel_traegt_keine_rueckgaengig_handlung_mehr` — eine
  angemeldete Handlung, danach `canUndo() == false` und `canRedo() == false`.
- `ein_geleerter_stapel_traegt_auch_eine_offene_gruppe_nicht_mehr` — eine zweite
  Handlung, deren Gruppe offen bleibt; danach `canUndo() == false` und
  `groupingLevel() == 0`. Diese Probe misst die Eigenschaft, auf der die Wahl
  von `removeAllActions` ruht.
- `ohne_verwalter_geschieht_nichts` — der Regelfall vor dem Einhängen.

Der Prüfstand von Rust lässt jede Probe auf einem eigenen Faden laufen, und
`NSUndoManager` ist in `objc2` als hauptfadengebunden geführt. Die Hilfsfunktion
`verwalter_ohne_fenster` nimmt deshalb `MainThreadMarker::new_unchecked`; der
Grund steht an ihr im Einzelnen. Der Verwalter hängt an keinem Fenster, die
Gruppierung je Ereignis ist abgewählt, und er wird auf demselben Faden erzeugt,
benutzt und fallengelassen.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/editor.rs` — der Aufruf in `stand_einsetzen`, die
  Funktion `rueckgaengigstapel_leeren`, `NSUndoManager` im Einfuhrblock, drei
  Proben und eine Hilfsfunktion, dazu die nachgezogenen Doc-Kommentare am
  Modulkopf, an `stand_einsetzen` und an `flaeche_richten`.

## Abnahme

`make check` läuft mit Rückgabewert 0 durch: Bau, 304 Proben in `krk-ui` plus
die übrigen Kisten, `clippy -D warnings`, `fmt --check`.
