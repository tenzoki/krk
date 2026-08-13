Die bewachte Luecke ist nicht die Luecke: elf Schreibwege an der Sperre vorbei bleiben

---

Der Modulkopf der Ablage beschreibt die Lücke seit Turn 2 richtig
(`crates/krk-core/src/ablage/mod.rs:33-39`): `atomar::schreiben` ist `pub`, `Ablage::pfad`
liefert den Pfad ohne Durchgang, und „wer beides zusammennimmt, kann an der Sperre
vorbeischreiben". Der Absatz darunter sagt dann, wer das bewacht (`:41-45`):

> Diese eine Luecke bewacht deshalb eine Probe und kein Typ:
> `nur_benannte_dateien_erreichen_das_atomare_schreiben` in `krk-core/tests/baum.rs`.

**Die Probe bewacht nur die eine Hälfte, und es ist die Hälfte, die man nicht braucht.** Sie
zählt, wer `atomar::schreiben` erreichen kann. Ein Schreibweg an der Sperre vorbei braucht
`atomar::schreiben` überhaupt nicht: `Ablage::pfad` plus jede beliebige Schreibfunktion der
Standardbibliothek genügt, und genau das ist der Weg, den der ursprüngliche Befund zitiert hat
(`fs::write(ablage.pfad(Datei::Belegung), keymap)`).

**Elf Stellen dieser Bauart stehen weiter im Baum**, alle in
`crates/krk-core/tests/ablage.rs`:

```
505, 552, 612, 692   fs::write(ablage.pfad(Datei::Sitzung), alt)
894, 1031            fs::write(ablage.pfad(welche), KAPUTT)
950                  fs::create_dir(ablage.pfad(Datei::Lesezeichen))
1167                 fs::create_dir(ablage.pfad(Datei::Sitzung))
1192                 fs::write(ablage.pfad(Datei::Lesezeichen), KAPUTT)
2201                 fs::write(ablage.pfad(Datei::Sitzung), KAPUTT)
2346                 fs::write(ablage.pfad(Datei::Lesezeichen), alt)
```

Zwei Stellen derselben Bauart sind in Turn 2 unter einen Durchgang gezogen worden
(`crates/krk-core/tests/belegung.rs:45-48` und
`alle_vier_dateien_ueberstehen_schreiben_und_wiedereinlesen` in `tests/ablage.rs:372-380`),
elf gleichartige daneben nicht, und der `Resolved:`-Absatz des geschlossenen Datensatzes nennt
sie nicht. Die elf sind der Sache nach vertretbar — sie stellen einen Altbestand oder eine
beschädigte Datei her, also gerade das, was `Zugang::sichern` nicht schreiben kann. Ein
Datensatz ist dies trotzdem, weil zwei Sätze im Modulkopf jetzt sagen, die Lücke sei bewacht,
und sie ist es nicht.

**Kein Produktionsweg ist betroffen.** Nachgezählt über `crates/*/src`: die einzigen Aufrufe
von `Ablage::pfad` außerhalb eines Durchgangs sind Meldungsfelder und Rückgabewerte
(`crates/krk-core/src/tasten/belegung.rs:1339`, `crates/krk-ui/src/messmodus.rs:311`), keiner
schreibt.

---

**Schwere:** mittel. Kein Fehlverhalten im Betrieb; die Zusage im Modulkopf ist weiter reichend
als das, was die genannte Probe leistet, und das ist derselbe Befund, den der geschlossene
Datensatz beheben sollte.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0716

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:41-45`,
`crates/krk-core/tests/ablage.rs:505,552,612,692,894,950,1031,1167,1192,2201,2346`

**Domain:** code

## Vorschlag

Zwei Wege, und der erste reicht.

**Weg 1 — den Satz auf das bringen, was die Probe leistet.** Sie bewacht nicht die Lücke,
sondern die Zahl der Dateien, die `atomar::schreiben` überhaupt erreichen. Der zweite
Bestandteil, `Ablage::pfad` plus ein Schreibaufruf, ist unbewacht, und der Grund gehört dazu:
elf Proben nehmen ihn absichtlich, weil sie einen Altbestand oder eine beschädigte Datei
herstellen, die keine Serialisierung liefern kann.

**Weg 2 — eine zweite Probe, die den zweiten Bestandteil zählt**, also Dateien, in denen
`.pfad(` und ein Schreibaufruf der Standardbibliothek in derselben Zeile stehen. Sie zählte
heute elf Stellen in einer Datei und wäre damit eine Liste zum Pflegen und keine Wache; sie
lohnt erst, wenn `Zugang::pfad` auch den Altbestandsfall abdeckt.
