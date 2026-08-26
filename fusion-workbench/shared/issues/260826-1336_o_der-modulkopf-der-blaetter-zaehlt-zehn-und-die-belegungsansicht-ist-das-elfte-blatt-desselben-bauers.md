Der Modulkopf der Blätter zählt zehn, und die Belegungsansicht ist das elfte Blatt desselben Bauers

---

`crates/krk-ui/src/appkit/blaetter/mod.rs:4` sagt „KRK hat zehn" und zählt die zehn Module des
Verzeichnisses auf (`:5-13`). `belegungsansicht.rs:747-759` baut daneben mit
`Blatt::mit_schaltflaechen` ein elftes Blatt („Tastaturbelegung", eine Schaltfläche „Fertig"),
das am selben Fenster hängt, denselben Griff nach `offenes_blatt` legt (`anwendung.rs:3817`)
und dieselbe Sperre auslöst. Derselbe Modulkopf nennt es sechzig Zeilen später selbst als Blatt
(`:367`: „die Tastaturbelegung und der Notizzettel (beide 'Fertig')").

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/mod.rs`

Die Zählprobe `jedes_blatt_nennt_seine_liegenlassende_schaltflaeche` (`:1027-1045`) sieht die
Belegungsansicht, weil sie über den Quellbaum und nicht über das Verzeichnis zählt; die
Prosa des Kopfes sieht sie nicht. Wer nach dem Kopf „alle Blätter" durchgeht — etwa für die
Behebung von `260826-1325` und `260826-1332` —, übersieht das eine, das außerhalb des
Verzeichnisses liegt.

Denkbarer Weg: „zehn" streichen oder „zehn hier und eines in `belegungsansicht.rs`" schreiben,
nach der Regel aus `CLAUDE.md`, dass Zahlen in Prosa mit der nächsten Runde falsch werden.
