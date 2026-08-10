# Die Herkunft eines Öffnens ist im Delegierten erzwungen und nicht am Editorbereich

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Behebung von `260810-0418`
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::datei_oeffnen:968`, `Ausgangsmelder:662`), `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::editor_oeffnen_lassen`)
**Cross-references:** `issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`, C2, C7

---

## Der Befund

Der Defekt `260810-0418` ist behoben, und zwar mit dem Weg, den seine Frage
„was zu prüfen wäre" als den besseren beschreibt, aber nur bis zur Grenze des
Delegierten. `Anwendungsdelegierter::editor_oeffnen_lassen` ist jetzt die eine
Stelle, an der der Delegierte den Editor eine Datei aufnehmen lässt, und sie
nimmt die Herkunft (`Oeffnungsherkunft::Befehl` oder `::Sitzung`) als
**Pflichtargument**. Ein vierter Öffnungsweg im Delegierten, der seine Herkunft
nicht nennt, übersetzt nicht.

**Was der Übersetzer weiter nicht sieht:** einen Aufruf von
`Editorbereich::datei_oeffnen`, der an dieser Stelle vorbeigeht. Heute gibt es
keinen — die vier Wege des Delegierten sind alle, die es gibt —, aber die Zusage
hängt an dieser Tatsache und nicht an einer Prüfung.

## Der Weg, der es erzwingen würde

Die Herkunft wandert in die Kette, statt neben ihr zu liegen:

1. `Editorbereich::datei_oeffnen(&self, pfad: &Path, herkunft: Oeffnungsherkunft)`
   — dann kann kein Aufrufer sie weglassen, wo auch immer er steht.
2. Der Wert wandert mit dem Ladevorgang durch `Editormodell::oeffnen` und kommt
   mit dem Ladeausgang zurück, oder — ohne das Modell anzufassen — der
   `Editorbereich` merkt sie sich zum laufenden Ladevorgang und gibt sie dem
   `Ausgangsmelder` als zweites Argument mit.

Der zweite Zuschnitt ist der kleinere und verletzt den Schnitt zwischen
`editormodell` und `appkit` nicht: das Modell müsste dann nichts von Befehlen
wissen. Er verlangt allerdings eine Änderung an `Ausgangsmelder` und damit an
jeder Stelle, die ihn einträgt.

## Warum es hier nicht gebaut wurde

Die Dateigrenze der Sitzung, in der `260810-0418` behoben wurde, lief um
`editormodell.rs` und `anwendung.rs`; an `appkit/editor.rs` arbeiteten parallel
andere. Der gebaute Weg löst den gemeldeten Fehler vollständig — ein F4 während
der Wiederherstellung erbt die Marke nicht mehr —; offen ist allein die
Erzwingung über die Kistengrenze hinweg.

## Was heute hält

Alles, was `260810-0418` benannte. Die Marke bezeichnet jetzt das **zuletzt
begonnene** Öffnen, und höchstens dieses liefert einen Ladeausgang, weil
`Editormodell::oeffnen` den laufenden Ladevorgang ersetzt und der Empfänger des
überholten Fadens fällt. Die eine Ausnahme davon führt der Datensatz
`260810-1029` daneben.
