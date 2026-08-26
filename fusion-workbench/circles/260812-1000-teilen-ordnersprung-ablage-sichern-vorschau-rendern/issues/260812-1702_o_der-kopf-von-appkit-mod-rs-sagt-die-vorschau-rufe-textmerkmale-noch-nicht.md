Der Kopf von appkit/mod.rs sagt, die Vorschau rufe textmerkmale noch nicht

---

`crates/krk-ui/src/appkit/mod.rs` beschreibt im Modulkopf jedes Modul des
Verzeichnisses. Zwei Stellen sind seit Planschritt 9 der Runde 6 falsch:

1. Der Absatz zu `textmerkmale` endet mit: „Heute ruft allein [`editor`] hier
   herein; die Vorschau kommt mit dem Schritt dazu, der ihr gerendertes
   Markdown trägt." Dieser Schritt ist gebaut. `appkit/vorschau.rs` ruft
   `textmerkmale::anwenden`, `::zuruecksetzen`, `::grundschrift` und
   `::tafel_der_erscheinung`.
2. Das Kastenbild darüber führt `textmerkmale` als Kind von `editor`:

   ```text
   ──> editor   ──> crate::editormodell
   │             ──> nummernspalte ──> krk-core::text::zeilen
   │             ──> textmerkmale  ──> crate::hervorhebung
   ```

   Die Vorschau hängt zwei Zeilen darüber und führt `nummernspalte`, aber nicht
   `textmerkmale`. Beide Module haben jetzt dieselben zwei Aufrufer, und die
   Zeichnung sagt es nur für eines.

Dazu kommt eine dritte Stelle, die keine Falschaussage ist, aber unvollständig
wird: der Absatz „[`textmerkmale`] setzt dieselbe Einfärbung und die
Ansichtswahl aus `crate::editormodell` in Merkmale um" nennt nicht, dass dort
seit dem 260812 auch die Wahl zwischen den beiden Farbtafeln wohnt.

---

**Wie schwer es wiegt**

Folgenlos für Bau, Proben und Bündel. Der Schaden ist die Übersicht: dieser Kopf
ist die Stelle, an der man nachsieht, wer wen ruft, und er sagt für genau die
Naht das Falsche, die Schritt 7 eigens geschaffen hat.

**Warum Schritt 9 es nicht behoben hat**

`appkit/mod.rs` stand nicht in der Dateiliste des Schrittes, und die Aufgabe
nannte die drei erlaubten Dateien ausdrücklich abschließend. Die Korrektur ist
drei Zeilen und keine Entscheidung.

**Was zu tun ist**

Den Satz auf „Editor und Vorschau rufen hier herein" ändern, `textmerkmale` im
Kastenbild auch unter `vorschau` führen und die Tafelwahl im Absatz weiter unten
nennen. Der Kopf von `appkit/textmerkmale.rs` trägt die richtige Fassung dieser
drei Aussagen bereits und ist die Vorlage.

**Kontext**

Gefunden beim Bau von Planschritt 9 der Runde 6; im Bericht jenes Schrittes
genannt. Herkunft: Circle der Runde 6.

Also seen: 260826-1416 by coderev — `mod.rs:63-65` sagt weiter „Heute ruft allein editor hier herein"; `vorschau.rs:275,745,1187,1212,1530` rufen `textmerkmale` an fünf Stellen.
