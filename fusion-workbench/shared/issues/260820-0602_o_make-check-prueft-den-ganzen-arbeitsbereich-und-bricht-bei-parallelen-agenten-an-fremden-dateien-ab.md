`make check` prüft den ganzen Arbeitsbereich und bricht bei parallelen Agenten an fremden Dateien ab

---

`make check` fährt unter anderem `cargo fmt --all --check`, und `--all` heißt: alle Mitglieder
des Arbeitsbereichs, nicht die Dateien des laufenden Schrittes. Arbeiten zwei Agenten
gleichzeitig an verschiedenen Dateien, hält der Lauf des einen an einer noch unformatierten
Datei des anderen an. Die Meldung nennt dann eine Datei, mit der der prüfende Agent nichts zu
tun hat.

---

**Gefilt von:** orchestrator, Sitzung `260819-2026`
**Gefunden:** am eigenen Lauf, während der Schritte 1 bis 3 der Runde 14.
**Schwere:** niedrig in der Wirkung, hoch in der Verführung. Nichts geht kaputt, solange der
Agent die Lage erkennt. Der naheliegende Ausweg ist aber `cargo fmt --all` **ohne** `--check`,
und der formatiert die halbfertige Datei des anderen Agenten mit — mitten in dessen
Bearbeitung.
**Baumstand:** `91f8727`.

## Was beobachtet wurde

Der Coder von Schritt 2 (`crates/krk-ui/src/markdown.rs`) bekam beim ersten `make check` einen
Abbruch an `cargo fmt --all --check`, gemeldet für `crates/krk-ui/src/appkit/vorschau.rs` —
die Datei des gleichzeitig laufenden Coders von Schritt 3. Er hat richtig gehandelt: seine
eigene Datei einzeln mit `rustfmt` formatiert, statt `cargo fmt --all` zu fahren, und im
Bericht vermerkt, dass die fremde Datei nicht ihm gehört. Der zweite Lauf war grün.

Das Verhalten ist keine Fehlfunktion von `make check`. Es ist die richtige Prüfung für den
Normalfall, in dem ein Lauf den ganzen Baum verantwortet. Die Voraussetzung fällt weg, sobald
mehrere Agenten zugleich schreiben.

## Warum das nicht bloß eine Unannehmlichkeit ist

Die Prüfung hat drei weitere Bestandteile, und alle drei fassen den ganzen Arbeitsbereich:
`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`.
Ein grünes Ergebnis gehört damit **dem Baum in jenem Augenblick** und nicht dem Schritt des
prüfenden Agenten. Die `Verification:`-Zeile eines Berichts sagt bei parallelem Lauf also
weniger, als sie bei einem Lauf allein sagt: sie belegt, dass der Baum zu diesem Zeitpunkt
grün war, nicht dass die Änderung dieses Schrittes für sich grün ist.

In dieser Sitzung ist daraus kein Schaden entstanden, weil jeder Schritt einzeln committet und
danach erneut geprüft wurde. Als Zusage taugt das aber nicht; es ist eine Eigenschaft der
Reihenfolge und nicht des Verfahrens.

## Mögliche Richtungen

Nicht entschieden, hier nur festgehalten:

- Der Dispatch weist den Agenten an, seine Datei einzeln mit `rustfmt` zu formatieren und
  `cargo fmt --all` nie ohne `--check` zu fahren. Das ist, was der Coder von Schritt 2 von
  sich aus getan hat.
- Der Orchestrator fährt die Abnahme selbst, einmal nach dem Zusammenführen, statt sie je
  Agent zu verlangen. Kostet die Zuordnung eines roten Ergebnisses zu seinem Schritt.
- Parallele Dispatches werden auf Schritte beschränkt, die keine gemeinsame Prüfung teilen.
  In einem Cargo-Arbeitsbereich sind das keine.

---
Abgleich 260820-0834: **trifft zu und bindet kuenftige Arbeit, nicht diese Runde.** Der
Abgleichslauf hat `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` allein gefahren, ohne
zweiten Agenten im Baum, und alle vier sind gruen. Damit ist die Zusage dieser Runde belegt und
der Befund unberuehrt: er handelt von der Aussagekraft eines gruenen Ergebnisses bei parallelem
Lauf, nicht vom Ergebnis selbst. Er bleibt offen.
