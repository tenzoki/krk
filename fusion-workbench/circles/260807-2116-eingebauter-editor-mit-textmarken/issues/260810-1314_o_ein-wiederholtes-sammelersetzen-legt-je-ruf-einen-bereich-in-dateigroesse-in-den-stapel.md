Ein wiederholtes Sammelersetzen legt je Ruf einen Bereich in Dateigroesse in den Stapel
---
`Umkehrpunkt` traegt seit `260810-1241` den Bereich zwischen der ersten und der letzten geaenderten Stelle. Bei einem Sammelersetzen, dessen Ersatztext den Suchtext enthaelt, findet der naechste `ctrl+cmd+r` wieder Treffer, und dieser Bereich deckt beinahe die ganze Datei. Wiederholte Rufe legen deshalb je Ruf einen Bereich in Dateigroesse in einen Stapel ohne Tiefengrenze.
---
**Schwere:** Niedrig
**Gefunden:** bei der Behebung von `260810-1241`, als Restrisiko des dort gewaehlten Umbaus
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`
**Zusammenhang:** `issues/260810-1241_*_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-und-ist-unbegrenzt.md`

## Belegstellen

`Editorbereich::alle_treffer_ersetzen` benennt den Fall im Doc-Kommentar, und die
Rechnung steht dort:

```text
  Suchtext `a`, Ersatztext `aa`
  Ruf 1: jedes `a` wird `aa`     erster Treffer nahe dem Anfang,
                                 letzter nahe dem Ende
  Ruf 2: jedes `a` wird `aa`     dieselbe Lage, und der Stand ist gewachsen
  …
```

`Umkehrpunkt::zwischen` bildet den Bereich aus dem gemeinsamen Anfang und dem
gemeinsamen Schwanz beider Staende. Liegen die geaenderten Stellen ueber die
ganze Datei verteilt, sind beide kurz, und `entfernt` ist so lang wie die Datei.

`levelsOfUndo` steht bei einem `NSUndoManager` ab Werk auf `0`, also unbegrenzt;
`setLevelsOfUndo` steht nirgends im Baum, und der Grund dafuer steht an
`Umkehrpunkt`.

## Fehlszenario

Eine Datei nahe der Editorgrenze von 16 MB, `cmd+f` nach einem haeufigen
Buchstaben, ein Ersatztext, der ihn enthaelt, dann `ctrl+cmd+r` mehrfach. Je Ruf
kommt ein Bereich in Dateigroesse in den Stapel, und die Datei waechst dabei.

Der Fall ist am Code belegt und nicht gefahren. Er ist **nicht** der Fall aus
`260810-1241`: dort waren es hundert einzelne Ersetzungen mit `shift+cmd+r`, und
die halten seit dem Umbau je drei Bytes. Hier braucht es einen Ersatztext, der
den Suchtext enthaelt, und einen Nutzer, der den Sammelbefehl wiederholt.

## Warum keine Tiefengrenze hilft

`setLevelsOfUndo` begrenzt die Zahl der Handlungen und nicht die Bytes. Bei einer
Grenze von hundert Handlungen und einer Datei von 16 MB bliebe das Produkt
1,6 GB — dieselbe Zahl, die `260810-1241` gefunden hat. Dazu gaelte die Grenze
fuer den ganzen Verwalter und damit auch fuer das Tippen, dessen Tiefe heute
unbegrenzt ist und von keinem Abnahmekriterium beschraenkt wird.

## Was zu pruefen waere

Drei Wege, keiner davon empfohlen, weil keiner gemessen ist:

1. **Mehrere Bereiche je Handlung statt eines.** Ein Sammelersetzen kennt seine
   Stellen; ein Umkehrpunkt aus einer Liste von Bereichen waere in der Groesse
   des Ersetzten. Der Preis: `appkit/editor.rs` muesste die Stellen erfahren, und
   heute weiss sie allein `krk_core::text::suche`. Das ist ein Umbau an der
   Grenze zwischen Kern und Oberflaeche.
2. **Eine Schranke in Bytes ueber dem eigenen Stapel.** Sie verlangte einen
   eigenen Stapel neben dem des `NSUndoManager`, und der Modulkopf von
   `appkit/editor.rs` schliesst genau das aus: ein zweiter Verwalter truege den
   Umbau in einen anderen Stapel als das Tippen.
3. **Nichts tun und den Fall benannt lassen.** Das ist der heutige Stand. Er ist
   vertretbar, solange niemand gemessen hat, dass ein Nutzer diesen Weg geht.

Die Entscheidung darueber gehoert nicht in eine Behebung; sie ist die Frage, was
ein Editor an seiner Grenze von 16 MB an Speicher halten darf, und die
Durchsicht `260810-1248` fuehrt sie unter den uebergreifenden Beobachtungen.
