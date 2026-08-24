Die Meldung einer Ersetzung verspricht den Auslieferungszustand, den `readers.toml` nicht bekommt

---

`Ersetzung`s `Display` (`crates/krk-core/src/ablage/mod.rs:392`) schreibt in **jedem** seiner
Zweige den Satzteil „und wird durch den Auslieferungszustand ersetzt". Für sechs der sieben
Ablagedateien stimmt er. Für `readers.toml` stimmt er seit Schritt 8 der Runde 16 nicht: nach
der zweiten Abweichung, die der Modulkopf von `ablage/leseprofile.rs` ausschreibt, tritt bei
einer beschädigten Datei **kein Profil** an ihre Stelle und nicht die Auslieferungsfassung.

Gefunden vom `coder` beim Bau der Proben zu C1.6 und C1.7 am 260824-1457, ausdrücklich nicht
im Vorbeigehen behoben.

---

**Warum es zählt.** Die Meldung erscheint beim Start in der Statuszeile, und der Nutzer liest
ihren Anfang — das sagt der Doc-Kommentar derselben Funktion über sich selbst. Wer dort liest,
seine `readers.toml` sei durch den Auslieferungszustand ersetzt worden, sucht anschließend
nach den fünf mitgelieferten Profilen, die er nicht bekommen hat. Der Satz führt genau in dem
Fall in die Irre, für den er geschrieben ist.

**Warum es kein Defekt der Runde 16 allein ist.** Der Satz ist der gemeinsame aller sieben
Ablagedateien und stand vor dieser Runde schon so da. Die Runde 16 hat ihn nicht falsch
gemacht, sondern die erste Datei hinzugefügt, für die er falsch ist. Wer ihn berichtigt,
entscheidet damit auch, ob `Grund` oder `Datei` die Auskunft trägt, was an die Stelle einer
beschädigten Datei tritt — heute trägt sie keiner von beiden, sondern der Formatierer als
feststehende Prosa.

**Nicht abnahmerelevant für diese Runde.** C1.6 verlangt Meldung, Beiseitelegen und
Weiterarbeiten ohne Profile; alle drei stehen und sind belegt. Der Wortlaut der Meldung ist in
keinem Kriterium der Runde 16 festgelegt.

---
Resolved:
