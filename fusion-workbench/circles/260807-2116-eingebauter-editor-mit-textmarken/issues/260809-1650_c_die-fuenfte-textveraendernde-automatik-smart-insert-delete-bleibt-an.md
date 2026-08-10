# Die fünfte textverändernde Automatik, Smart Insert/Delete, bleibt an

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht Turn 2 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:45-52` (Modulkopf), `:290-296` (`textflaeche_bauen`)
**Cross-references:** C4, S16

---

## Der Befund

Der Modulkopf von `appkit/editor.rs` schreibt die Zusage aus:

> **Reiner Text.** `setRichText(false)` und die vier abgeschalteten Ersetzungen
> halten fest, was der Nutzer tippt: eine Zeichenkette, die beim Sichern Zeichen
> für Zeichen wieder in der Datei steht. Eine typografische Ersetzung von
> Anführungszeichen oder Bindestrichen ändert Programmtext still, und die Zusage
> aus C4 lautet, dass der gesicherte Stand der getippte ist.

Die vier sind richtig gewählt: Anführungszeichen, Bindestriche, Textersetzung
und Rechtschreibkorrektur sind die vier Automatiken, die tippenden Text
verändern. **Es gibt eine fünfte, die eingefügten und ausgeschnittenen Text
verändert, und sie ist nicht abgeschaltet:** `smartInsertDeleteEnabled`
(`objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1405-1412`).

Sie fügt beim Einfügen eines Wortes ein Leerzeichen davor oder dahinter ein und
entfernt beim Ausschneiden ein überzähliges. In Prosa ist das gemeint; in
Programmtext ist es eine stille Änderung, die niemand getippt hat — genau die
Sorte, gegen die die vier anderen abgeschaltet sind.

`speculation:` Der Vorgabewert für eine programmatisch erzeugte `NSTextView` ist
nach der AppKit-Dokumentation `YES`. Ich habe ihn im laufenden Bündel **nicht**
gemessen; ohne Fenster ist er nicht zu erheben, und die Messung ist
Nutzerarbeit.

## Warum das zählt

Die Zusage aus C4 lautet, dass der gesicherte Stand der getippte ist. Sie ist
entweder vollständig oder sie trägt nicht: eine fünfte Automatik, die
durchrutscht, macht die vier abgeschalteten zu einer halben Maßnahme, und der
Modulkopf behauptet dann mehr, als der Code hält.

Der Fall ist heute nicht auslösbar, weil `Editorbereich` noch keinen
Rückkanal in das Modell hat (S26) und der Editor niemand sichtbar ist. Er wird
es mit S26, und dann ist die stille Änderung in der Datei.

## Vorschlag

Eine Zeile bei den vier bestehenden, mit derselben Begründung:

```rust
text.setSmartInsertDeleteEnabled(false);
```

Dazu den Modulkopf von „die vier abgeschalteten Ersetzungen" auf fünf ziehen und
den Unterschied benennen: vier greifen beim Tippen, die fünfte beim Einfügen und
Ausschneiden.

Zwei weitere Kandidaten habe ich geprüft und **nicht** als Befund geführt:
`automaticDataDetection` und `automaticLinkDetection` wirken allein auf
Rich-Text und sind mit `setRichText(false)` ohne Wirkung;
`continuousSpellChecking` und `grammarChecking` setzen vorübergehende Merkmale
des Layoutverwalters und fassen den Textspeicher nicht an, so wie die Einfärbung
der Formatansicht aus C3 es auch nicht tut.

Gemeldet von: `coderev`, Durchsicht Turn 2.

---
Resolved: `textflaeche_bauen` schaltet `smartInsertDeleteEnabled` ab, der Modulkopf
zählt jetzt fünf statt vier, und die Vollständigkeit hält eine Probe fest statt
der Aufmerksamkeit des nächsten Lesers.

**Die Zeile steht.** `crates/krk-ui/src/appkit/editor.rs`, `textflaeche_bauen`:
`text.setSmartInsertDeleteEnabled(false);`, abgesetzt von den vier tippenden, mit
dem Unterschied als Kommentar — vier greifen beim Tippen, die fünfte beim
Einfügen und Ausschneiden.

**Die `speculation:` des Datensatzes ist eingelöst, und sie stimmte.** Der
Vorgabewert ist `true`, gemessen und nicht der Dokumentation entnommen: an einer
`NSTextView`, die `textflaeche_bauen` selbst geliefert hat. Die Messung brauchte
kein Fenster und war damit keine Nutzerarbeit; sie stand als Wegwerf-Probe da und
ist nach der Messung wieder gefallen, weil eine Textfläche im Prüfstand außerhalb
des Hauptfadens entsteht und das die Begründung an `verwalter_ohne_fenster`
aufhöbe.

**Es gibt keine sechste dieser Form, und das ist jetzt maschinell gehalten.** Die
Probe `keine_unbekannte_automatik_steht_an_der_textflaeche` zählt zur Laufzeit
über die Objective-C-Laufzeit auf, welche Schalter der Form `set…Enabled:` die
Klasse `NSTextView` trägt — zwölf auf macOS 15.6 — und hält sie gegen zwei
Aufstellungen: `ABGESCHALTET` (die fünf) und `GEDULDET` (die sieben übrigen, jede
mit ihrem Grund). Ein dreizehnter hält den Bau an, bis jemand ihn eingeordnet
hat. Die Probe braucht weder Fläche noch Fenster: Klasse und Selektoren stehen
für sich. Sie misst dafür **nicht**, ob die fünf Zeilen noch stehen und was sie
bewirken; das bleibt Nutzerarbeit am laufenden Bündel.

Die sieben geduldeten und ihr Grund: Verknüpfungs- und Datenerkennung zeichnen
aus statt zu ändern und sind mit `setRichText(false)` ohnehin ohne Wirkung;
Rechtschreib- und Grammatikprüfung setzen vorübergehende Merkmale des
Layoutverwalters; die Spracherkennung wählt, woran jene beiden messen; die
Textvervollständigung legt Kandidaten vor, die der Nutzer wählt; die Schrittsuche
wählt aus und schreibt nicht.

**Zwei Proben halten die Zusage fest**, beide in
`crates/krk-ui/src/appkit/editor.rs` unter `mod tests`:

- `keine_unbekannte_automatik_steht_an_der_textflaeche`
- `die_fuenfte_automatik_steht_unter_den_abgeschalteten`

**Ein Fund liegt außerhalb der Form und ist abgetrennt.** `NSTextView` trägt seit
macOS 15 die Schreibwerkzeuge (`writingToolsBehavior`), die markierten Text
umschreiben; ihr Vorgabewert überlässt dem System die Wahl, und sie sind kein
Schalter der Form `set…Enabled:`. Sie unterscheiden sich von den fünf darin, dass
der Nutzer sie eigens aufruft — ob C4 sie trotzdem ausschließt, ist eine Frage an
den Nutzer und nicht an den Übersetzer:
`issues/260810-0512_o_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`.
