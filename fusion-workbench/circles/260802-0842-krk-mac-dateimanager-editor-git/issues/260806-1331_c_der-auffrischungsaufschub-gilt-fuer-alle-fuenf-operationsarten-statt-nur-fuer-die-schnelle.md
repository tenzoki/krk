Der Auffrischungsaufschub gilt für alle fünf Operationsarten statt nur für die schnelle

---

`gehoert_zu_vorgang` (`crates/krk-ui/src/auffrischung.rs:162`) und sein Aufruf
in der Dateisystemwache (`crates/krk-ui/src/appkit/anwendung.rs:1230`) setzen
die Auffrischung für **jeden** laufenden Vorgang aus, unabhängig von seiner
Art. Der behobene Defekt betraf allein das Stapel-Umbenennen. Für eine lange
Kopie oder Verschiebung ändert der Aufschub das Verhalten in eine Richtung, die
der Defekt nicht verlangt hat.

---

**Was der Defekt war.** Beim Stapel-Umbenennen von 5.000 Einträgen meldete
FSEvents schneller, als ein Lesevorgang fertig wurde. Da `ordner_neu_lesen` sein
Ordnermodell leert, bevor der erste Stapel anhängt, setzte jede Meldung den
Lesevorgang zurück, und die Liste blieb für die ganze Laufzeit leer
(`issues/260805-1337_*_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`).

**Was der Aufschub daneben ändert.** Eine Kopie von 50 GB in einen Zielordner,
den ein Dateifenster zeigt, meldet über Minuten hinweg in gemächlichem Takt.
Zwischen zwei Meldungen wurde ein Lesevorgang bisher fertig, und der Nutzer sah
die Dateien nacheinander erscheinen. Seit `fd5e3c5` steht der Zielordner
stattdessen bis zum Abschluss unverändert da und füllt sich in einem Schlag.
Dasselbe gilt für das Löschen und für den Papierkorb.

**Die Ursache liegt eine Schicht tiefer.** Das eigentliche Problem ist, dass ein
neu angestoßener Lesevorgang das Ordnermodell leert, bevor er liefert. Der
Aufschub umgeht das an der Meldestelle, statt es an der Lesestelle zu beheben,
und bezahlt dafür mit dem Verhalten aller anderen Operationsarten.

**Drei Wege, und die Wahl gehört zum Fix:**

1. Den Aufschub auf `Art::UmbenennenImStapel` einschränken. Kleinster Eingriff,
   löst den gemeldeten Defekt vollständig, lässt Kopie und Verschiebung beim
   alten Verhalten. Nachteil: die Ursache bleibt stehen, und die nächste schnelle
   Operationsart trifft wieder auf sie.
2. Den Lesevorgang so ändern, dass er das Ordnermodell erst mit dem ersten
   gelieferten Stapel ersetzt statt es vorab zu leeren. Behebt die Ursache für
   jede Art und macht den Aufschub überflüssig. Größerer Eingriff in `tabelle.rs`
   und den Leseweg.
3. Beim heutigen Stand bleiben und die Änderung als gewollt festhalten. Dann
   gehört sie in C4 oder C9 als ausgeschriebenes Verhalten, nicht allein in
   einen Modulkommentar.

**Betrifft:** `krk-ui` (`auffrischung.rs`, `appkit/anwendung.rs`). C4 und C9.
Keine Zeitzusage aus C8 berührt.

---
Resolved: Nutzerentscheid 260806 — der Aufschub gilt nur noch für schnelle Vorgänge. Die Einordnung steht als schiebt_auffrischung_auf(&Art) -> bool in crates/krk-ui/src/auffrischung.rs, vollständige Fallunterscheidung ohne Auffangzweig an genau einer Stelle; eine probeweise sechste Art-Variante bricht den Bau an vier Stellen ab. gehoert_zu_vorgang heißt jetzt auffrischung_aufgeschoben, weil die übergebene Liste nicht mehr die Ordner des Vorgangs sind, sondern die aufgeschobenen. Vorgang::ordner bleibt die eine Aufzählung für Aufschub und Abschlussauffrischung.

Vier Prüfungen decken alle fünf Operationsarten; dass sie beißen, ist gezeigt: der Stand von fd5e3c5 wiederhergestellt lässt 2 fehlschlagen, Art::UmbenennenImStapel => false lässt 3 fehlschlagen. Am Bündel ist keine Richtung als Verhalten vorgeführt, und der Grund ist gemessen: die einzige Dateioperation, die der Messmodus auslöst, ist die Kopie aus L8/L9, die rund 300 ms nach F5 abbricht — bei 0,3 s Sammelverzögerung des FSEventStream trifft der erste Meldestapel frühestens zusammen mit dem Abbruch ein. Gegenprobe, dass der Weg überhaupt arbeitet: 0 Verzeichnisleser-Fäden ohne Zutun, 29 bei 30 fremden Änderungen im angezeigten Ordner.

Vorbehalt als eigener Eintrag weitergeführt: issues/260806-1445_o_ein-schnelles-verschieben-koennte-dieselbe-meldelawine-ausloesen-wie-das-stapel-umbenennen.md.
