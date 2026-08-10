# Die beiden Rückgängigproben schalten die Betriebsart ab, die sie messen sollen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:429-498` (`stapel_fuellen`, `ein_geleerter_stapel_traegt_keine_rueckgaengig_handlung_mehr`, `ein_geleerter_stapel_traegt_auch_eine_offene_gruppe_nicht_mehr`)
**Cross-references:** `issues/260809-1727_c_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`, Commit `2123e52`

---

## Der Befund

`stapel_fuellen` setzt `verwalter.setGroupsByEvent(false)`, und beide Proben
laufen dahinter. Die Gruppierung je Ereignis ist aber genau die Betriebsart,
in der `stand_einsetzen` zur Laufzeit steht: `setString:` fällt mitten in die
Behandlung eines Tastendrucks, und `NSUndoManager` liefert `groupsByEvent` ab
Werk als `true` aus.

Der Doc-Kommentar der zweiten Probe sagt, sie messe „der Zustand mitten in
einem Ereignis". Sie misst eine von Hand geöffnete Gruppe bei abgeschalteter
Ereignisgruppierung. Das ist ein anderer Mechanismus: bei `groupsByEvent =
true` öffnet der Verwalter die Gruppe selbst bei der ersten Anmeldung und
schließt sie über einen Beobachter der Laufschleife am Ende des Umlaufs. Ob
dieser Beobachter eine Gruppe vorfindet, die `removeAllActions` inzwischen
abgeräumt hat, sagen die beiden Proben nicht.

## Was gemessen wurde

Die Frage selbst ist beantwortet, und die Antwort ist gutartig. Ein
Swift-Programm auf demselben Gerät (macOS 15.7.7, Build 24G720) meldet eine
Handlung an einem Verwalter mit `groupsByEvent = true` an, ruft
`removeAllActions` und lässt danach die Laufschleife einen Umlauf machen:

```
groupsByEvent: true
level nach registrieren: 1 canUndo: true
level nach removeAllActions: 0 canUndo: false
nach dem Umlauf: level: 0 canUndo: false
```

Keine Ausnahme, keine Meldung, keine offene Gruppe. **Die Behebung von
`260809-1727` trägt also auch in der Betriebsart der Laufzeit** — das ist
gemessen und nicht angenommen.

## Warum der Befund trotzdem steht

Er ist einer über die Proben und nicht über den Code. Die beiden Proben stehen
in der Datei als die Begründung dafür, dass `removeAllActions` und nicht
`endUndoGrouping` genommen wurde. Diese Begründung deckt heute die eine
Betriebsart nicht ab, in der die Funktion tatsächlich läuft, und die Messung
oben steht in einem Wegwerf-Programm im Sitzungsverzeichnis und nicht im Baum.

## Was zu prüfen wäre

Ob eine dritte Probe mit `groupsByEvent = true` überhaupt trägt: sie bräuchte
einen Umlauf der Laufschleife, und der Prüfstand von Rust fährt jede Prüfung
auf einem eigenen Faden ohne laufende Schleife. Möglicherweise ist die
richtige Antwort, den Doc-Kommentar auf das zu beschränken, was die beiden
Proben wirklich messen, und die Aussage über die Ereignisgruppierung als
gemessene Nutzerarbeit mit Datum daneben zu setzen — dieselbe Form, die der
Modulkopf für den Vorgabewert von `smartInsertDeleteEnabled` schon fährt.
