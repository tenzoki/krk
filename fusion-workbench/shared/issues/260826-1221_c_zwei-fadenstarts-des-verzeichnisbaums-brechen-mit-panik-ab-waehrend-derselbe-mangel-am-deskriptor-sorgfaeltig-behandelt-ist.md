Zwei Fadenstarts des Verzeichnisbaums brechen mit Panik ab, waehrend derselbe Mangel am Deskriptor sorgfaeltig behandelt ist

---

`Lesevorgang::starten` (`leser.rs:114-117`) und `Durchlauf::starten` (`durchlauf.rs:265-277`)
beenden `thread::Builder::spawn` mit `.expect(...)`. Scheitert der Start, weil dem Prozess die
Faeden ausgegangen sind (`EAGAIN` aus `pthread_create`), geraet der **rufende** Faden in Panik.
Das ist bei beiden der Hauptfaden.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Affected:** `crates/krk-core/src/verzeichnis/leser.rs:117`,
`crates/krk-core/src/verzeichnis/durchlauf.rs:277`; dieselbe Form ausserhalb dieser Durchsicht
in `crates/krk-core/src/operation/mod.rs:165`
**Tree state:** `004ff72`
**Domain:** code

## Was dasteht

```rust
// crates/krk-core/src/verzeichnis/durchlauf.rs:265-277
thread::Builder::new()
    .name(format!("krk-durchlauf-{generation}"))
    .spawn(move || { ... })
    .expect("Arbeitsfaden fuer den Durchlauf laesst sich nicht starten");
```

`thread::Builder::spawn` liefert ausdruecklich ein `io::Result` — anders als `thread::spawn`,
das selbst in Panik geraet — und existiert genau dafuer, dass der Aufrufer den Fehlschlag
behandeln kann. Beide Stellen nehmen den `Result` entgegen und werfen ihn weg.

## Warum das im Widerspruch zum Rest des Moduls steht

Derselbe Modulbaum behandelt die **Erschoepfung der Deskriptortabelle** als erstklassige
Nicht-Antwort und hat dafuer eigenen Code, eigene Prosa und eigene Proben:

- `sys::ist_deskriptormangel` (`sys.rs:343-346`) trennt `EMFILE`/`ENFILE` von jedem anderen
  Fehler;
- `durchlauf::unterbaum_entscheiden` gibt bei Mangel `None` statt eines negativen Befunds
  (`durchlauf.rs:514`);
- `umfang::zaehlen` liefert `Umfang::Unentschieden` (`umfang.rs:256`, `:282`);
- `inhalt::traegt_der_inhalt` liefert `Inhaltsbefund::Unentschieden` (`inhalt.rs:151`);
- `verzeichnis/loeschzielbefund.rs:85-99` erklaert die ganze dreiwertige Antwort als
  Verallgemeinerung dieser Haltung.

Der Fadenvorrat ist die Schwestergroesse der Deskriptortabelle: prozessweit, geteilt, von
aussen erschoepfbar, und der naechste Versuch kann gelingen. Er wird an diesen zwei Stellen
gegenteilig behandelt.

## Wann das eintritt

`inference:` nicht gemessen. Ein Durchlauf und ein Lesevorgang je Tab, dazu die Faeden der
Operationsmaschine und die der Vorschau; ein aus dem Finder gestartetes Buendel bekommt beide
Grenzen klein, und der Modulkopf von `durchlauf.rs:125-131` fuehrt genau dieses Argument fuer
die Deskriptoren schon selbst. Der Schaden ist im Panikfall vollstaendig: eine Panik auf dem
Hauptfaden nimmt die laufende Sitzung mit, samt allem, was `krk_core::ablage` noch nicht
geschrieben hat.

## Richtung

Beide Signaturen liefern heute `Self` und muessten `io::Result<Self>` liefern oder eine
Ersatzlage benennen. Der billigere Weg fuer den `Lesevorgang` ist der vorhandene:
`Abschluss::Fehler(io::Error)` steht bereits (`leser.rs:64`), und ein Lesevorgang, der gar
nicht erst angelaufen ist, ist der Sache nach genau das. Fuer den `Durchlauf` ist der
Gegenwert der geschlossene Kanal, den `Durchlauf::befunde` (`durchlauf.rs:285-289`) schon
richtig deutet: „ein geschlossener Kanal ohne weitere Meldung heisst nicht, dass die
restlichen Auftraege keinen Treffer tragen: er heisst, dass sie nicht entschieden sind."

`crates/krk-core/src/operation/mod.rs:165` traegt dieselbe Zeile und liegt ausserhalb dieser
Durchsicht; wer die zwei hier anfasst, sieht sie mit an.

Also seen: 260831-1444 by coderev — `Gitlauf::starten` (`crates/krk-core/src/git/lauf.rs:174-181`) ist die vierte Stelle derselben Form: `thread::Builder::new().spawn(…).expect(…)` in einer Funktion, die `Self` liefert, während der Deskriptormangel im selben Modul sorgfältig als `Oeffnung::Unentschieden` getrennt wird.

---
Resolved: Alle vier Stellen liefern den gescheiterten Fadenstart jetzt ueber den Kanal, den der Rufer ohnehin liest, statt in Panik zu geraten; keine Signatur hat sich geaendert, kein Rufer in `krk-ui` oder `krk-bench` musste angefasst werden. `Lesevorgang::starten` (`crates/krk-core/src/verzeichnis/leser.rs`) sendet die vorhandene `Abschluss::Fehler(io::Error)`. `Durchlauf::starten` (`crates/krk-core/src/verzeichnis/durchlauf.rs`) und `Gitlauf::starten` (`crates/krk-core/src/git/lauf.rs`) lassen den Sender fallen; der geschlossene Kanal ist genau die Lage, die `Durchlauf::befunde` und `Gitlauf::meldungen` schon als „nicht entschieden" ausschreiben. `operation::starten` (`crates/krk-core/src/operation/mod.rs`) meldet `Meldung::Fertig` mit einer Abschlusszeile je Quelle; `Lauf::neu` nimmt den Faden dafuer als `Option`. Gehalten wird es von `kein_fadenstart_im_baum_wirft_seinen_rueckgabewert_weg` in `crates/krk-core/tests/baum.rs`, nachgeprueft durch versuchsweises Wiedereinsetzen von `.expect(…)` an zwei der vier Stellen (die Probe wurde rot); an den anderen zwei haelt der Uebersetzer, weil `.expect` dort `JoinHandle` statt `Option<JoinHandle>` liefert. Der erste Entwurf der Probe las bis zum naechsten Strichpunkt und sah keine der vier Stellen, weil der Rumpf des Abschlusses selbst einen traegt; gelesen wird deshalb bis zum ersten Strichpunkt ausserhalb jeder Klammer.
