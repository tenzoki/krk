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
Resolved: 260824-1740 vom coder. Die Auskunft trägt jetzt `Datei` und nicht mehr der Formatierer. Neu in `ablage/pfade.rs`: `enum Ersatz { Auslieferungszustand, Nichts }` mit `Ersatz::satzteil` und `Datei::ersatz`, eine vollständige Fallunterscheidung ohne Auffangzweig wie `Datei::format` und `Datei::leerbefund` daneben; `Datei::Leser` ist die eine mit `Ersatz::Nichts`. `Ersetzung` trägt dafür das neue Feld `welche: Datei` neben dem Pfad — der Pfad ist nicht ableitbar, weil `belegung::fuer_den_betrieb` ohne Ablageordner den nackten Dateinamen einträgt. Alle neun Bauplätze nachgezogen. Der Satzteil heißt für `readers.toml` „und nichts tritt an ihre Stelle"; der Kern nennt dabei kein Profil, weil die Ablage den Inhalt nicht kennt. `Grund` trägt die Auskunft ausdrücklich nicht: derselbe Grund trifft jede Datei. Belegt von zwei neuen Proben in `tests/ablage.rs` (`die_meldung_zu_readers_toml_verspricht_keinen_auslieferungszustand` über alle fünf `Beiseite`-Lagen samt Gegenprobe an `settings.toml`, `genau_readers_toml_bekommt_keinen_ersatz` über `Datei::ALLE`); `pruefe_meldung` prüft den Satzteil je Datei. Drei Prosastellen mitgezogen: die Doc-Kommentare an `Grund` und `Ersetzung` und der Modulkopf von `ablage/leseprofile.rs`.
