Drei fest verdrahtete Zahlen im Code brechen mit den neuen Einträgen aus S9b

---

S9b trägt drei Funktionen in `resources/default-keymap.toml` nach; die Datei
wächst damit von 46 auf 49 Funktionen und von 52 auf 55 Kombinationen. Drei
Stellen in `crates/krk-core` schreiben die alten Zahlen fest und schlagen
seitdem fehl. Zwei Prüfungen fallen um:

- `crates/krk-core/src/tasten/belegung.rs:578` — `assert_eq!(belegung.funktionen().len(), 46)`, muss 49 heißen.
- `crates/krk-core/src/tasten/belegung.rs:584` — `assert_eq!(kombinationen, 52)`, muss 55 heißen.
- `crates/krk-core/tests/belegung.rs:488` — `assert_eq!(belegung.funktionen().len(), 46)`, muss 49 heißen.

Dazu ein Kommentar, der dieselbe Zahl in Prosa nennt und mitgezogen werden
sollte: `crates/krk-core/src/tasten/belegung.rs:223` spricht von "den 52
ausgelieferten" Kombinationen.

Ausführender: `coder` (Rust-Code und Testdateien liegen außerhalb des
`ontocoder`). Die Datenänderung von S9b ist unabhängig davon gültig: die
Auslieferungsbelegung ist gültiges TOML, und
`die_auslieferungsbelegung_ist_konfliktfrei` sowie
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` laufen grün
durch. Es fallen ausschließlich die beiden Zählprüfungen.

---

Gemessen am 260804-0907 nach der Umsetzung von S9b:

    cargo test -p krk-core --test belegung
    → 25 passed; 1 failed
      die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht
      left: 49, right: 46 (tests/belegung.rs:488)

    cargo test -p krk-core --lib
    → 25 passed; 1 failed
      tasten::belegung::tests::die_auslieferungsbelegung_traegt_die_erwarteten_zahlen
      left: 49, right: 46 (src/tasten/belegung.rs:578)

Das Abnahmekriterium von S9b verlangt `cargo test -p krk-core --test belegung`
mit Beendigungscode 0. Solange dieser Defekt offen ist, kann S9b das Kriterium
nicht erfüllen; die drei Zahlen sind der einzige Grund.

Der Plan hat die Zahlen nicht bedacht. Sein Absatz "Die drei Kennungen dürfen
vor ihren Kommandos landen, und das ist geprüft" prüft allein die Richtung
Kommando → Kennung und schließt daraus, S9b breche keine Prüfung. Die
Zählprüfungen liegen quer dazu: sie hängen an der Größe der Datei und nicht an
den Kennungen. Wer den Defekt behebt, sollte den Absatz in
`planning/260802-1428_o_plan-navigator-geruest-runde-1.md` unter `#### 9b.`
mitziehen, damit die Begründung nicht weiter mehr behauptet, als sie geprüft
hat.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Schritt 9b),
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260804-0907-s9b-drei-kombinationen-nachgetragen.md`

---
Resolved: 260804-1015. Keine der vier Zahlen steht mehr im Code. Die beiden Prüfungen sagen jetzt zu, was ihr Name behauptet, und der Kommentar nennt eine Größenordnung statt eines Zählwerts.

**Warum nicht 46→49 und 52→55.** Der Datensatz schlägt vor, die Zahlen hochzuziehen. Das behebt den Lauf und stellt dieselbe Falle für den nächsten Nachtrag wieder auf: `resources/default-keymap.toml` ist eine Datentabelle, die mit jeder Runde wächst, und eine Prüfung, die ihre Größe als Literal führt, prüft die Größe der Datei statt der Zusage, für die sie dasteht. Die vier Stellen sind deshalb einzeln gefragt worden, was sie eigentlich behaupten wollen.

**`tests/belegung.rs` — `die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht`.** Die Zusage steht im Namen und zerfällt in zwei. Erstens führt die geladene Belegung denselben *Wortschatz* wie die Auslieferungsbelegung, weil die Funktionen, die die Nutzerdatei nicht nennt, unbelegt hinzutreten müssen; geprüft wird das gegen `Belegung::auslieferung()` und nicht gegen eine Zahl. Der neue Helfer `kennungen` sortiert dafür, denn eine Nutzerdatei holt die Funktionen, die sie nennt, nach vorne. Zweitens ist *belegt* allein, was die Nutzerdatei nennt: die Datei nennt `kopieren`, also trägt genau `kopieren` Tasten und sonst niemand. Das ist die Aussage "ersetzt, ergänzt nicht" ohne jeden Zählwert, und sie ist schärfer als die 46 es war — die 46 hätte auch eine Belegung durchgelassen, die die ausgelieferten Kombinationen dazumischt. Der Nachsatz über `verschieben` ist entfallen: er sagte weniger als die beiden Prüfungen, die ihn jetzt einschließen.

**`src/tasten/belegung.rs` — `die_auslieferungsbelegung_traegt_die_erwarteten_zahlen`.** Diese Prüfung hatte keine Zusage außer den Zahlen selbst; sie war ein Änderungsmelder. Was sie sinnvoll zusichern kann, ist, dass `Belegung::bauen` beim Einlesen nichts verliert. Sie heißt darum jetzt `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren`, liest `AUSLIEFERUNGSTEXT` ein zweites Mal als rohe `Belegungsdatei` und hält die gebaute Belegung dagegen: gleich viele Funktionen, gleich viele Kombinationen. Das fängt einen echten Fehlerfall, den `bauen` sonst stillschweigend schluckt, nämlich dieselbe Kombination zweimal innerhalb einer Funktion. Ein `!datei.funktionen.is_empty()` davor verhindert, dass beide Vergleiche bei einer leeren Datei gegenstandslos grün werden.

**`src/tasten/belegung.rs:223` — der Kommentar.** Er begründet, warum `nachschlag` eine lineare Schleife ist und kein Nachschlagbaum. Das Argument hängt an der Größenordnung, nicht am Zählwert: "die wenigen Dutzend ausgelieferten Kombinationen" trägt es genauso und veraltet nicht. Eine 55 an dieser Stelle wäre die schwächere Wiederholung derselben Falle, weil Prosa von keiner Prüfung eingeholt wird.

**Eine fünfte Fundstelle.** `eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt` (`tests/belegung.rs`) schrieb `assert_eq!(funktion.tasten().len(), 3)` — zwei ausgelieferte Wege plus der zugewiesene `ctrl+k`. Derselbe Fehler in klein: die 3 stammt aus den Daten. Der Kommentar daneben sagt, was gemeint ist ("die ausgelieferten Wege bleiben daneben stehen"), und genau das steht jetzt da: jede Kombination, die `kopieren` ab Werk trägt, muss danach noch da sein, und dazu genau eine mehr. Sie bricht jetzt erst, wenn `kopieren` wirklich eine Kombination verliert.

**Nachweis, dass der Fix den nächsten Nachtrag überlebt.** An eine Sicherung von `resources/default-keymap.toml` gehängt und die Datei danach zurückgespielt: ein vierter Eintrag `probe_vierter_nachtrag` mit `ctrl+p`. `cargo test -p krk-core --test belegung` und `cargo test -p krk-core --lib` beide 26 von 26, ohne eine Zeile Code anzufassen. Gegen den alten Stand wären beide umgefallen. Zweite Probe, damit die Prüfungen nicht bloß gegenstandslos grün sind: derselbe Eintrag mit `tasten = ["ctrl+p", "ctrl+p"]` lässt `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren` mit der vorgesehenen Meldung fehlschlagen. Beide Proben sind zurückgenommen; `git hash-object resources/default-keymap.toml` steht wieder auf `13faa4f2ab604a1d2af2c99f778400962889b57f`, dem Wert von vorher. Die Datei ist unangetastet.

**Abnahme.** `cargo build --workspace`, `cargo test --workspace` (170 bestanden, 2 ausgelassen), `cargo fmt --all --check` und `cargo clippy --workspace --all-targets` alle mit Rückgabewert 0. `cargo test -p krk-core --test belegung` 26 von 26, `cargo test -p krk-core --lib` 26 von 26.

Nicht mitgenommen: der Absatz unter `#### 9b.` in `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, den der letzte Abschnitt oben verlangt. Die Plandatei liegt außerhalb dieser Aufgabe; dafür steht `260804-1015_o_planabsatz-zu-9b-behauptet-mehr-als-er-geprueft-hat.md`.
