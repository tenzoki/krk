Ein Sitzungsschreiber kann sein Sitzungsrecht ueberleben

---

Turn 2 hat C3.9 am Typ befestigt, und das trägt: `Sitzungsschreiber::neu` und `::mit_takt`
verlangen ein `&Sitzungsrecht` und liefern `None`, wenn es niemand hält
(`crates/krk-core/src/ablage/sitzung.rs:452-467`). `Sitzungsrecht` hat ein privates Feld und
genau zwei Erzeuger, `nehmen` und `ohne` (`crates/krk-core/src/ablage/sperre.rs:172-201`); ein
gehaltenes Recht ist ohne `flock` nicht herzustellen. **Am Entstehen kommt heute niemand
vorbei**, und das war der Fehler.

**Das Recht wird dabei nur geliehen, nicht gehalten.** Die Ausleihe endet mit dem Aufruf, der
Schreiber trägt keine Lebenszeit:

```rust
let recht = Sitzungsrecht::nehmen(ablage.ort())?;
let mut schreiber = Sitzungsschreiber::neu(&recht).unwrap();
drop(recht);                       // die flock-Sperre ist frei
schreiber.beenden(jetzt, zugang);  // schreibt weiter, und der Uebersetzer sagt nichts
```

Zugesagt ist mehr als das. Der Doc-Kommentar am Feld `sitzungsrecht` des
Anwendungsdelegierten sagt seit Turn 2 (`crates/krk-ui/src/appkit/anwendung.rs:538-541`):

> Die Regel „nur die Halterin schreibt die Sitzung" haelt danach der Uebersetzer.

Der Übersetzer hält „war Halterin, als der Schreiber entstand". Zwischen beidem liegt die
Spanne, in der geschrieben wird, und die deckt er nicht.

**Kein Aufrufer nimmt den Weg heute.** Nachgesehen an allen dreien: der Anwendungsdelegierte
legt das Recht in `ivars.sitzungsrecht` ab und hält es bis zum Prozessende
(`anwendung.rs:1239-1247`); `Messplan::herstellen` hält es bis zum Ende der Funktion
(`crates/krk-ui/src/messmodus.rs:308-342`); die Probe `schreiber_mit_recht` gibt es
absichtlich mit zurück (`crates/krk-core/tests/ablage.rs:1487-1494`). Die Zusage ist also
erfüllt und nur nicht erzwungen.

---

**Schwere:** gering. Kein Fehlverhalten im Baum. Der Datensatz steht, weil der schließende
Datensatz und zwei Doc-Kommentare den Übersetzer als Garanten nennen und er es für die
Schreibspanne nicht ist.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0719

**Betroffen:** `crates/krk-core/src/ablage/sitzung.rs:425-434`, `:452-467`,
`crates/krk-ui/src/appkit/anwendung.rs:538-541`

**Domain:** code

## Vorschlag

Zwei Wege, und der billigere reicht wahrscheinlich.

**Weg 1 — die Aussage auf das bringen, was gilt.** „Er entsteht nur, wenn dieser Prozess das
Recht hält" ist wahr und vom Übersetzer gehalten; „nur die Halterin schreibt" ist eine Stufe
mehr und steht weiter an der Lebensdauer des Rechts, die der Aufrufer wählt. Ein Satz an
`Sitzungsschreiber` sagt das: das Recht gehört mindestens so lange gebunden wie der Schreiber,
und `#[must_use]` an `Sitzungsrecht` erinnert nur daran, es überhaupt zu binden.

**Weg 2 — die Lebenszeit mitführen**, also `Sitzungsschreiber<'a>` mit `recht: &'a
Sitzungsrecht` im Feld. Dann hält der Übersetzer wirklich, was der Kommentar sagt. Der Preis
ist eine Lebenszeit an einem Typ, der im Anwendungsdelegierten in einem `RefCell<Option<…>>`
neben dem Recht selbst wohnt; ob die zwei Felder eines `ivars` einander so borgen können, ist
nicht nachgesehen und wäre vor einer Zusage zu messen.
