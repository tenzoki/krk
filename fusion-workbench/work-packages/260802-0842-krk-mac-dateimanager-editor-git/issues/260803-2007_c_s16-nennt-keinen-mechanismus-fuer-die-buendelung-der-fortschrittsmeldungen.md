S16 sagt "höchstens einmal je Bild" zu und nennt keinen Mechanismus dafür

---

Die `Änderungen` von S16 verlangen, dass das Fortschrittsblatt den Fortschritt
"gebündelt anzeigt, höchstens einmal je Bild". Dieselbe Zusage steht in
`### Frage 6` als struktureller Träger von L9. Welcher Takt die Bildgrenze
meldet, sagt kein Schritt. Der einzige vorhandene Kandidat ist der
`CADisplayLink` aus S8, und ihn dafür zu verwenden ist nicht offensichtlich
richtig.

---

**Was vorliegt.** `crates/krk-ui/src/appkit/bildtakt.rs` aus S8 hält die Hülle
`Zeichenende` um einen `CADisplayLink` auf der Inhaltsansicht. Sie nimmt beim
Einrichten eine gewöhnliche Rust-Senke und gibt den Takt beim Fallenlassen
wieder frei. Für die Frühmessung wird sie in
`crates/krk-ui/src/appkit/anwendung.rs` nur dann eingehängt, wenn ein Messlauf
ansteht.

**Warum die naheliegende Wiederverwendung nicht ohne Weiteres taugt.** Ein
`CADisplayLink` weckt den Prozess an jeder Bildgrenze, am Referenzgerät also
alle 16,667 ms, und zwar unabhängig davon, ob es etwas zu zeichnen gibt. Für
einen Messlauf von zwanzig Wiederholungen ist das folgenlos. Ein Dateimanager
läuft dagegen stundenlang, und die Maxime "superschnell" bezieht sich auf die
Reaktion, nicht auf die Bereitschaft, dauerhaft Strom zu ziehen. Der Takt
dürfte deshalb höchstens während einer laufenden Operation hängen, was ihn zu
einem dritten Lebenszyklus neben Messlauf und Anwendung machte.

**Drei Wege, ohne Empfehlung, weil die Grundlage für eine fehlt.**

1. Den `CADisplayLink` aus S8 während einer laufenden Operation einhängen und
   bei ihrem Ende wieder fallen lassen. Ein Mechanismus für zwei Zwecke, aber
   ein zusätzlicher Lebenszyklus.
2. Einen gewöhnlichen `NSTimer` mit der Bildlänge als Intervall, der nur
   während einer Operation läuft. Einfacher im Lebenszyklus, aber ein zweiter
   Taktgeber neben dem aus S8, und er trifft die Bildgrenze nicht, sondern
   nähert sie an.
3. Gar kein Takt: der Arbeitsfaden meldet jeden Fortschritt, und der
   Hauptfaden verwirft eine Meldung, solange die vorige noch nicht gezeichnet
   ist. Das braucht keinen Zeitgeber, verlagert die Bündelung aber in eine
   Zustandsprüfung, deren Richtigkeit schwerer zu belegen ist.

**Warum das ein eigener Eintrag ist und nicht in der Dateiliste steht.** Die
Durchsicht der Dateilisten vom 260803-2007 hätte `bildtakt.rs` als
`(lesend)` in S16 eintragen können. Damit wäre Weg 1 stillschweigend gewählt,
und die Wahl ist eine Entwurfsentscheidung mit einer Nebenwirkung auf den
Energieverbrauch. Die Dateiliste von S16 lässt den Punkt deshalb offen.

**Dringlichkeit.** Bindet S16 und keinen Schritt davor. Zu klären, bevor S16
begonnen wird.

**Aufgefallen bei:** der Durchsicht der Dateilisten von S9 bis S23 unter der
erweiterten Regel, `issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`.

---
Resolved: Weg 3 ist umgesetzt, gar kein Takt. `crates/krk-ui/src/kommandos/operationen.rs`
hält die `Buendelung`: der Arbeitsfaden meldet jeden Fortschritt, der
Vermittlerfaden in `crates/krk-ui/src/appkit/anwendung.rs` setzt einen Weckruf
über die Hauptschlange ab, und `Buendelung::melden` verwirft ihn, solange der
vorige nicht gezeichnet ist. `bildtakt.rs` ist unberührt; ein Zeitgeber ist
nirgends entstanden.

**Der Haken ist belegt.** Die Richtigkeit der Zustandsprüfung hängt an einer
Reihenfolge auf dem Hauptfaden: erst `gezeichnet`, dann den Stand lesen, dann
zeichnen. Sie steht im Modulkopf ausgeschrieben und in
`die_buendelung_haelt_die_zahl_der_weckrufe_klein` festgehalten (5.000 Meldungen
bei zehn Zeichendurchgängen ergeben genau zehn Weckrufe). Am laufenden Bündel
gemessen am 260804 mit einer Kopie von 5.000 Einträgen, dreimal wiederholt:
2.363 bis 2.422 Meldungen des Arbeitsfadens gegen 29 bis 93 Weckrufe und ebenso
viele Zeichendurchgänge des Hauptfadens. Das Verhältnis liegt zwischen 26:1 und
82:1, und die Zahl der Zeichendurchgänge bleibt weit unter der Zahl der Bilder,
die in derselben Spanne vergangen sind. Der Hauptfaden wird nicht überschwemmt.

Verworfen wird ausdrücklich der Weckruf und nicht die Meldung: der Stand steht
in `Vorgangszustand`, und der Hauptfaden liest beim nächsten Durchgang den
neuesten. Ein verworfener Weckruf kostet keinen Zwischenstand.
