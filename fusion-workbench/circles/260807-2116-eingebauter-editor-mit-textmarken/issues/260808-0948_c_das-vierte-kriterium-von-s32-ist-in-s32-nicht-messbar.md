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

---
Resolved: Das Kriterium ist geteilt und steht jetzt an beiden Stellen, an denen
es messbar ist. Der Zahlenrahmen von 10 MB ist unverändert, kein `[DONE]` und
kein Status im Kopf des Plans ist berührt.

**In S32** fragt Kriterium 4 nicht mehr das Bündel, sondern den Preis der beiden
Kisten an einem eigenständigen Prüfprogramm, das die Sprachdefinitionen und die
Farbtafeln wirklich lädt. Der Grund steht beim Kriterium: in S32 nennt kein
Modul der Anwendung die Kisten, der Übersetzer wirft den eingebetteten Bestand
wieder weg, und die 128 Byte am `target/release/krk` sind echt und sagen nichts.
Die Ersatzmessung dieses Datensatzes ist als Messvermerk in den Schritt
übernommen: 1.591.544 gegen 418.968 Byte, rund 1,12 MiB. Dort steht auch der
Ausgangspunkt für S33, nämlich die 3.502.046 Byte, die `target/KRK.app` am
260808-0948 trug.

**In S33** ist das Wachstum des Bündels als zusätzliches Abnahmekriterium
eingetragen, mit der Begründung, dass der Bestand hier zum ersten Mal geladen
wird. Es ist nicht bloß gefordert, sondern am 260810-0918 gemessen:
`cargo xtask bundle` läuft durch und signiert, `target/KRK.app` trägt 7.191.902
Byte über vier Dateien, also 3.689.856 Byte oder rund 3,52 MiB mehr als vor S32.
Die Zahl ist ausdrücklich als **obere Schranke** eingetragen und nicht als
Zuordnung: gemessen ist der Stand nach allen 48 Schritten gegen den Stand vor
S32, der Zuwachs enthält also auch den Code von S33 bis S48. Für ein Kriterium,
das eine Grenze nach oben zieht, genügt die Schranke.

Mitgezogen sind zwei Stellen, die dieselbe Aussage führten: der Satz in
`### Frage 2` über die beiden Größen, die S32 abnimmt, und die Zeile in
`## Risiken und Gegenmaßnahmen`, die beide Messungen S32 zuschrieb.

Kein Code ist angefasst. `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` beenden
mit 0.
