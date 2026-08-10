Die Einfärbung läuft mit 0,3 MB/s und hängt beim Tippen in großen Dateien hinterher

---

Entstanden mit S33 am 260810-0054. Kein Fehlverhalten: die Oberfläche bleibt zu
jedem Zeitpunkt bedienbar, und die Einfärbung ist am Ende richtig. Was hier
steht, ist ein **gemessener** Preis und ein benannter Ausweg.

---

## Die Messung

Am 260810 auf diesem Gerät, `--release`, an `crates/krk-ui/src/appkit/anwendung.rs`
(193 kB) und Vielfachen davon:

```
    192 866 Byte ->  0,64 s   (0,30 MB/s)
  1 542 928 Byte ->  4,60 s   (0,34 MB/s)
  7 714 640 Byte -> 23,12 s   (0,33 MB/s)
```

Drei Wege gemessen, alle drei gleich schnell: nur parsen; parsen mit
`HighlightIterator`; parsen mit Wortartenstapel und Farbe, also der Weg, den KRK
geht. Der Aufwand steckt vollständig in `ParseState::parse_line`, also in den
Sprachregeln von Sublime Text und `fancy-regex`. Die Farbe kostet nichts dazu.

Die Grenze des Editors liegt bei 16 MB (`datei::EDITORGRENZE`). Eine Datei an
dieser Grenze braucht danach knapp eine Minute für einen Durchgang.

## Was S33 daraus gemacht hat

Die Einfärbung läuft auf einem Arbeitsfaden, in derselben Bauart wie das Lesen
aus S24: ein Faden je Anfrage, `sync_channel(1)`, kein Generationszähler, weil
eine neue Anfrage den alten Empfänger fallen lässt. Der Hauptfaden holt die
Antwort mit demselben Zeitgeber ab, der schon das Lesen abholt.

Schnelle Anfragen werden zusammengefasst: läuft schon ein Faden, wird kein
zweiter gestartet, sondern nur vermerkt, dass sein Ergebnis überholt sein wird.
Damit lebt zu jedem Zeitpunkt höchstens ein Faden, und der letzte Stand wird
genau einmal eingefärbt statt jeder Zwischenstand einmal.

## Der Preis, der bleibt

**Die Einfärbung hängt beim Tippen hinterher, und zwar um einen ganzen
Durchgang.** In einer Datei von 1,5 MB sind das rund 4,5 Sekunden: wer ein
Anführungszeichen tippt, sieht den Rest der Datei erst nach dieser Spanne als
Zeichenkette. In einer Datei von 200 kB ist es gut eine halbe Sekunde und fällt
kaum auf; in einer von 16 MB ist die Einfärbung während des Tippens praktisch
nutzlos.

Dazu kommt: jeder abgeschlossene Durchgang schreibt bei Markdown Merkmale in den
Textspeicher, und das lässt die Nummernspalte ihren Zeilenindex neu bauen. Bei
Code und einfachem Text fällt das nicht an, weil dort keine Merkmale in den
Speicher gehen.

`speculation:` **Ungemessen ist, ab welcher Dateigröße es den Nutzer stört.** Das
verlangt KRK im Vordergrund und ist damit Nutzerarbeit; der Abnahmelauf ist aus
dieser Runde ausgeklammert.

## Der Ausweg, falls er gebraucht wird

Benannt und nicht zu suchen: `ParseState` ist zeilenweise fortschreibbar. Wer je
Zeile den Zustand am Zeilenanfang aufhebt, kann nach einer Änderung an der
geänderten Zeile wieder einsteigen, statt am Dateianfang, und muss nur so weit
laufen, bis der Zustand wieder mit dem aufgehobenen zusammenfällt. Das ist der
Weg, den ausgewachsene Editoren gehen. Er braucht die geänderte Stelle, und die
liegt schon bereit: `NSTextStorage` meldet `editedRange` und `changeInLength` mit.

Dieselbe Angabe würde
`issues/260809-2322_o_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`
bedienen. **Beide Stellen stellen dieselbe Frage und lebten von derselben
Antwort**, und das ist der Grund, sie zusammen zu bewerten statt einzeln.

## Was zuerst zu tun wäre

Messen, nicht bauen. Eine Rust-Datei von einigen hundert Kilobyte im Editor
öffnen, in die Formatansicht wechseln und tippen. Stockt nichts und stört das
Nachziehen nicht, bleiben beide Datensätze offen liegen.

**Aufgefallen bei:** dem Bau von S33 am 260810-0054, bei der Messung, die die
Wahl zwischen Hauptfaden und Arbeitsfaden entschieden hat.

Cross-references:
`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (`### Frage 7`, `### Frage 8`, Schritt 33),
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260809-2322_o_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`,
`crates/krk-ui/src/hervorhebung.rs` (Modulkopf, Abschnitt „Ein Durchgang, zwei Verbraucher")

---
Resolved: Der benannte Ausweg ist gebaut. `crate::hervorhebung::fortschreiben`
schreibt den vorigen Durchgang fort, statt ihn zu wiederholen: Zeilenvergleich,
Wiedereinstieg am letzten Haltepunkt vor der geänderten Zeile, Rechnen bis der
Zustand des Zerlegers wieder mit einem aufgehobenen zusammenfällt,
Zusammensetzen aus übernommenem Anfang, neu gerechneter Mitte und verschobenem
Schwanz. `Einfaerbungsstand` ist der aufgehobene Stand; er wohnt zwischen zwei
Anfragen in `EditorIvars::einfaerbungsstand` und wandert mit jeder Anfrage in den
Arbeitsfaden und zurück.

**Gemessen an der gebauten Fassung**, 260810 auf diesem Gerät, `--release`, an
`crates/krk-ui/src/appkit/anwendung.rs` und dem Achtfachen davon, ein Zeichen in
die Zeile in der Mitte eingefügt:

```
    229 029 Byte   voll  1 458 ms   fortgeschrieben   9,6 ms   (152-fach)
  1 832 232 Byte   voll 10 475 ms   fortgeschrieben  10,8 ms   (968-fach)
```

Der Wiedereinstieg hängt nicht an der Dateigröße, sondern an der Zahl der Zeilen
bis zum Wiederanschluss. Der Aufpreis auf den **ersten** Durchgang ist das
Aufheben der Zustände und liegt bei sechs bis neun Prozent (912 gegen 864 ms bei
229 kB, 7 692 gegen 7 074 ms bei 1,8 MB, in der Vormessung ohne die gebaute
Fassung).

**Zwei Zahlen aus der Vormessung haben den Zuschnitt bestimmt, und beide standen
vorher in keinem Datensatz.** Je Zeile einen Zustand aufzuheben kostet rund
780 Byte: an einer Datei von 19 MB mit 394 060 Zeilen wuchs der Prozess von
23,3 MB auf 331,5 MB, also um das Sechzehnfache der Datei. Deshalb steht ein
Haltepunkt nur alle `ZUSTANDSABSTAND = 32` Zeilen — rund 10 MB statt 331 MB —,
und der Wiedereinstieg rechnet dafür bis zu 32 Zeilen zu viel, bei 0,19 ms je
Zeile also höchstens etwa 6 ms.

**Die gemessene Geschwindigkeit des vollen Durchgangs ist bestätigt**, mit einer
Abweichung: nachgemessen wurden 0,25 MB/s statt der hier geführten 0,30 MB/s, an
derselben Datei, die seit dem 260810-0054 auf 229 kB gewachsen ist. Dieselbe
Größenordnung, dieselbe Aussage.

**Die Annahme, dieser Datensatz und `260809-2322` lebten von derselben Antwort,
ist widerlegt**, und der Grund steht in `260809-2322`. Insbesondere braucht das
Fortschreiben `editedRange` aus `NSTextStorage` **nicht**: der Zeilenvergleich
findet die geänderte Zeile selbst, in 0,13 ms bei 229 kB und 12 ms bei 19 MB. Der
Schnitt zwischen `hervorhebung.rs` und AppKit bleibt damit, wie er war.

**Was Nutzerarbeit bleibt:** die Wirkung am laufenden Bündel, also dass die
Einfärbung beim Tippen mitläuft. Die Gleichheit von „von vorn" und
„fortgeschrieben" ist dagegen keine Nutzerarbeit mehr: sie steht als Zusicherung
im Baum und wird an 14 Änderungen in Rust, an sieben in Markdown und über zehn
Haltepunktabstände hinweg gemessen (`ein_fortgeschriebener_durchgang_gleicht_dem_vollen`,
`das_fortschreiben_traegt_auch_die_markdown_auszeichnungen`,
`das_fortschreiben_traegt_ueber_viele_haltepunkte` in `crates/krk-ui/src/hervorhebung.rs`).
Verglichen wird dabei die **Wirkung** Zeichen für Zeichen und nicht die
Buchführung: zwei aneinanderliegende Stücke derselben Farbe sind dasselbe wie
ein zusammengezogenes, und an den beiden Nähten fällt genau dieser Unterschied
an.
