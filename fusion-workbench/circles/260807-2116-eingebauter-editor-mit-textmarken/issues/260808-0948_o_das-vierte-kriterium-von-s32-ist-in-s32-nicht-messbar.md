Das vierte Kriterium von S32 ist in S32 nicht messbar und gehört an S33

---

S32 nennt als viertes Kriterium: „Das Bündel wächst durch sie um weniger als 10 MB",
zu messen als Größe von `target/KRK.app` vor und nach der Einbindung. Gemessen am
260808-0948 wuchs das Programm `target/release/krk` durch beide Kisten um **128 Byte**.

Die Zahl ist echt und sagt nichts. S32 bindet die Kisten ein, aber kein Modul der
Anwendung ruft sie auf; der Übersetzer wirft den gesamten eingebetteten Bestand
wieder weg. Der Zuwachs entsteht erst, wenn S33 die Sprachdefinitionen und die
Farbtafeln tatsächlich lädt.

Das Kriterium ist damit an der falschen Stelle abgenommen: es prüft in S32 eine
Größe, die in S32 noch nicht existiert, und es steht nicht in S33, wo sie entsteht.

---

**Ersatzmessung, damit die Frage nicht offen bleibt.** Ein eigenständiges
Prüfprogramm, das `two_face::syntax::extra_newlines()` und
`syntect::highlighting::ThemeSet::load_defaults()` wirklich lädt, wiegt 1.591.544
Byte; dasselbe Programm ohne die beiden Kisten wiegt 418.968 Byte. Der Preis
beider Kisten beträgt damit rund **1,12 MiB**, gut ein Zehntel der zugestandenen
10 MB. Die Messung liegt unter
`scratchpad/groessenprobe` der Sitzung 260808-0948 und ist Wegwerfcode.

Vorschlag: das Kriterium in S33 wiederholen, dort mit `cargo xtask bundle` vor und
nach dem Schritt, und in S32 auf die Ersatzmessung verweisen. Der Zahlenrahmen von
10 MB bleibt unverändert; strittig ist allein, wo er geprüft wird.

**Stand des Bündels am 260808-0948**, als Ausgangspunkt für S33: `target/KRK.app`
trägt 3.502.046 Byte über vier Dateien.

Gemeldet von: `coder`, bei der Umsetzung von S32.
