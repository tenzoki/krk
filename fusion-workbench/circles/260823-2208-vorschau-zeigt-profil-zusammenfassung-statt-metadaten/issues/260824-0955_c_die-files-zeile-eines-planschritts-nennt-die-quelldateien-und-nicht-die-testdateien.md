Die `Files:`-Zeile eines Planschritts nennt die Quelldateien und nicht die Testdateien, die derselbe Schritt braucht

---

Schritt 4 des Plans nennt unter `Files:` die zwei Quelldateien
`crates/krk-core/src/verzeichnis/leser.rs` und `crates/krk-core/src/text/datei.rs`. Sein
Abschnitt `Changes:` endet mit sechs Proben — drei zu `lesen_hoechstens`, drei zu
`anlesen` —, und keine davon kann in einer der zwei genannten Dateien stehen: alle sechs
brauchen einen Prüfordner, und den führt `krk-core` allein unter
`crates/krk-core/tests/gemeinsam/mod.rs`. Eine vierte Fassung daneben im Quellbaum machte
die Zählprobe `genau_drei_pruefordner_fassungen_stehen_im_baum`
(`crates/krk-core/tests/baum.rs:114`) rot, und genau dafür steht sie da. Die Zeile nennt
damit zwei Dateien, während der Schritt vier braucht.

---

**Es ist die zweite Fundstelle desselben Musters in diesem Plan.** Schritt 8 (die
Ablagehälfte, `ablage/leseprofile.rs`) nennt unter `Files:` die zwei Quelldateien und
`crates/krk-core/tests/baum.rs`, aber nicht `crates/krk-core/tests/ablage.rs` — obwohl
der Defekt `260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`
genau dort das Nachziehen verlangt und ihn ausdrücklich an Schritt 8 hängt. Der Schreiber
jenes Datensatzes hat die Lücke schon gesehen und in Prosa festgehalten
(`history/260824-0940-coder-readers-toml-als-siebte-ablagedatei.md`, Abschnitt „Ein
Befund, gefiltert": „dessen Dateiliste nennt `tests/ablage.rs` nicht"), ohne dass daraus
ein Datensatz über die `Files:`-Zeile selbst geworden wäre.

**Was die Lücke kostet, ist nicht der Tippfehler, sondern die Frage an den Nutzer.** Der
Executor eines Schrittes liest `Files:` als seinen Arbeitsbereich. Steht die Datei nicht
darin, die er anfassen muss, hat er drei Wege, und zwei davon sind falsch: die Proben
weglassen und den Beleg einem späteren Schritt überlassen, oder sie in eine Datei
schreiben, die der Bereich zulässt. Der dritte ist, den Nutzer zu fragen, und genau das
ist am 260824-0952 geschehen; der Nutzer hat den Arbeitsbereich um die zwei Testdateien
erweitert (Möglichkeit 1: `crates/krk-core/tests/verzeichnis.rs` und
`crates/krk-core/tests/text.rs`) mit der Begründung, ein Beleg erst acht Schritte später
hieße acht Schritte lang auf einer Behauptung zu bauen.

**Die Wurzel ist eine Regel, die im Plan nirgends steht:** wo eine Probe hingehört, folgt
in diesem Baum aus der Kiste, in der ihr Gegenstand liegt, und für `krk-core` heißt das
`tests/<modul>.rs` und nie den Quellbaum. `## Testing Strategy` des Plans sagt zwar,
welche Proben es geben soll, aber nicht, in welche Datei sie kommen, und die `Files:`-Zeile
je Schritt ist die einzige Stelle, die das je Schritt sagen könnte.

**Der Weg zur Behebung** ist nicht das Nachtragen an Schritt 4 allein: der ist mit der
Antwort des Nutzers vom 260824-0952 versorgt. Zu berichtigen sind die `Files:`-Zeilen
aller noch nicht erledigten Schritte, die Proben verlangen, um die Testdateien, in denen
diese Proben stehen werden — namentlich Schritt 8 um `crates/krk-core/tests/ablage.rs`.
Wer den Plan dafür anfasst, prüft die Zeilen der Schritte 3, 5, 6, 9, 10, 11 und 12 im
selben Durchgang: alle sieben nennen Proben, und mindestens die Schritte 3, 5, 6 und 12
nennen mit `crates/krk-core/src/leseprofil/*.rs` bzw. `crates/krk-core/tests/leseprofil.rs`
schon je eine Datei, in der ihre Proben stehen können, während die Schritte 9, 10 und 11
in `krk-ui` liegen, wo `#[cfg(test)]`-Module neben dem Code die Regel sind und die Lücke
deshalb nicht entsteht.

**Filed by:** coder
**Cross-references:**
`planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`
(Schritte 4 und 8),
`issues/260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`,
`history/260824-0940-coder-readers-toml-als-siebte-ablagedatei.md`

---
Resolved: Die noch offenen Schritte 7 bis 12 sind am 260824-1224 einzeln durchgesehen; genau einer trug die Luecke. **Schritt 8** ist um `crates/krk-core/tests/ablage.rs` nachgezogen, mit dem Grund dabei: die Proben zu C1.1, C1.2 und C1.5 bis C1.8 brauchen einen Pruefordner, und den erreicht nur eine Datei unter `tests/`; dieselbe Datei zieht nach dem Defekt `260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md` ohnehin nach. Die zwei Proben ueber den `AUSLIEFERUNGSTEXT` brauchen keinen Pruefordner und stehen wie ihre Vorlage in `ablage/einstellungen.rs` neben dem Code. **Ohne Luecke:** Schritt 7 verlangt keine Probe (er schreibt eine Datei unter `resources/`, ihre Abnahme fuehrt Schritt 8); die Schritte 9, 10 und 11 liegen in `krk-ui`, wo die Probe neben dem Code steht und die Luecke nicht entstehen kann; Schritt 12 nennt `crates/krk-core/tests/leseprofil.rs` bereits, die Kindprobe zu C6.9 eingeschlossen. **Die Wurzel, die dieser Datensatz benennt, steht jetzt im Plan**: `## Testing Strategy` traegt den Abschnitt „Wo eine Probe hingehoert" mit der Regel je Kiste und dem Satz, dass die `Files:`-Zeile auch die Testdatei nennt.
