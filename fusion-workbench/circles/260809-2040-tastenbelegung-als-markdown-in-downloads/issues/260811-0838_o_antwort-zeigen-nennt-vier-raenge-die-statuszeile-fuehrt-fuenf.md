`antwort_zeigen` nennt vier Ränge, die Statuszeile führt fünf

---

`crates/krk-ui/src/appkit/anwendung.rs:3290-3291` beschreibt die Befehlsantwort so:

```
/// Der oberste der vier Raenge, siehe
/// [`crate::appkit::statuszeile::zeile`].
```

Die Statuszeile führt seit der Runde 2 fünf Ränge. `crates/krk-ui/src/appkit/statuszeile.rs:40` sagt es im Modulkopf ausdrücklich („**Der fuenfte Rang hat als einziger kein Feld.**"), `:109` nennt ihn in der Löschregel, und `:299` beschreibt ihn am Feld: „Der fuenfte Rang aus S16c: er steht unter allen vieren."

Geprüft am 260811-0838 beim Entwurf des Plans zur Runde 3. Der Spec jener Runde stützt sich auf dieselbe Stelle und zählt richtig: „Die Statuszeile trägt fünf Ränge nach dem Alter der Aussage", mit Verweis auf `anwendung.rs:3296`.

## Warum das mehr ist als eine Zahl

Der Kommentar verweist auf die Datei, die die Rangfolge trägt, und gibt dabei eine Zahl an, die diese Datei widerlegt. Wer die Rangfolge aus dem Kommentar übernimmt, statt dem Verweis zu folgen, baut auf einer Aufzählung, der ein Glied fehlt. Genau diese Fehlerform hat in der Runde 2 einen Fehlbefund erzeugt: `circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-aus-c4-wird-von-der-antwort-still-ueberschrieben.md` entstand, weil eine Sperre für die einzige gehalten wurde, die es gibt.

Die Aussage des Kommentars über den **obersten** Rang bleibt richtig. Falsch ist allein die Zahl der Ränge, und sie ist bei der Erweiterung um den fünften Rang nicht mitgezogen worden.

## Behebung

Eine Zeile in `crates/krk-ui/src/appkit/anwendung.rs`: „Der oberste der fünf Ränge". Der Verweis auf `statuszeile::zeile` bleibt, wie er ist. Wer den Fehler behebt, prüft im selben Zug die Nachbarstelle `Dateifenstersicht::melden`, deren Kommentar „einen Rang tiefer" sagt und damit von derselben Aufzählung ausgeht.

Der Plan der Runde 3 fasst die Datei in S3 an und könnte die Zeile mitnehmen. Er tut es ausdrücklich **nicht**: die Änderung gehört nicht zur Ausgabe der Tastenbelegung, und ein Schritt, der nebenbei fremde Kommentare berichtigt, macht seinen eigenen Diff unlesbar.
