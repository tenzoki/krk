Der Dateikopf der Belegung beschreibt den Fokusvorbehalt als frühen Ausstieg vor dem Nachschlag; seit S1 bis S3 fragt er danach

---

`resources/default-keymap.toml` ist nach ihrem eigenen ersten Satz „die alleinige Quelle der
ausgelieferten Tastenbelegung", und ihr Kopf erklärt auf zwölf Zeilen, wie ein Tastendruck
zwischen Textfeld und Dateifenster aufgeteilt wird. Zwei Sätze dieser Erklärung sind seit
Commit `9da33bc` (S1 bis S3 der Runde 7) am Baum widerlegt. Die Datei ist in derselben
Prüfspanne angefasst worden (`40b5fb0`, S15), aber an einer anderen Stelle; der Kopf steht
unverändert.

**Der erste Satz — `resources/default-keymap.toml:76-78`:**

> Steht die Schreibmarke in einem Textfeld, reicht der Ereignisabgriff den Tastendruck
> unveraendert an AppKit weiter, und erst dort wirkt das Menue; steht sie im Dateifenster,
> schlaegt er in dieser Datei nach.

Der Abgriff schlägt jetzt in **beiden** Lagen nach. `behandeln` ruft `belegung.nachschlag`
ohne jede vorherige Frage nach dem Ersthelfer (`crates/krk-ui/src/appkit/ereignisse.rs:522`)
und reicht beide Ausgänge an die Senke weiter. Ob der Tastendruck an AppKit zurückfällt,
entscheidet erst die Senke: `kommandos::zulaessigkeit::zulaessig` im Kommandozweig, die
ausdrückliche Abfrage auf `lage.ersthelfer_gehoert_appkit` im Zeichenzweig
(`crates/krk-ui/src/appkit/anwendung.rs:2491-2494`).

**Der zweite Satz — `resources/default-keymap.toml:79-80`:**

> Der Vorbehalt fragt vor dem Nachschlag und nicht nach der Kombination, und darum traegt er
> eine nackte Taste genauso wie eine mit Zusatztaste.

Er fragt nicht mehr vor dem Nachschlag. Der Code sagt es an der Stelle selbst, an der der
frühe Ausstieg stand:

> `crates/krk-ui/src/appkit/ereignisse.rs:517` — „Hier stand bis zur Runde 7 der
> Fokusvorbehalt als frueher Ausstieg. […] Der Abgriff reicht beide Ausgaenge des Nachschlags
> unveraendert weiter und fragt nicht mehr nach dem Ersthelfer."

Der Modulkopf derselben Datei führt die neue Lage vollständig aus (`:84-94`).

**Die Folgerung des zweiten Satzes hält, ihre Begründung nicht.** Eine nackte Taste wird
weiterhin so behandelt wie eine mit Zusatztaste, aber nicht mehr, weil eine Frage vor dem
Nachschlag steht, sondern weil **zwei** Stellen hinter dem Nachschlag denselben Wert lesen:
`zulaessig` für den Kommandozweig und die eigene Abfrage im Sprungmarkenzweig. Wer die Datei
liest und die Begründung übernimmt, leitet aus ihr ab, dass eine einzige Stelle den Fall
trägt, und übersieht die zweite.

---

**Schwere:** mittel. Kein Befehl fällt aus, keine Kombination ändert sich. Die Datei ist aber
die einzige Quelle der Belegung und die erste, die eine spätere Runde liest, bevor sie eine
Kombination vergibt; ihr Kopf beschreibt seit dieser Runde einen Weg, den der Code nicht mehr
geht. Genau dieselbe Sorte Fehler hat der Modulkopf von `ereignisse.rs` bis zum 260808
getragen, und der Code hat ihn ausdrücklich vermerkt (`:37-39`).

**Gefunden:** ontorev, bei der Durchsicht von `resources/default-keymap.toml` über
`ca66c39..40b5fb0` am 260813-0532.

**Betroffen:** `resources/default-keymap.toml:74-80`.

**Domain:** data — die Behebung gehört dem `ontocoder`. Der Code ist in Ordnung und braucht
keine Änderung; nachzuziehen ist allein die Prosa.

**Nicht betroffen:** der Begriff „Fokusvorbehalt" selbst. Er lebt weiter und benennt jetzt
Bestandteil (2) der Zulässigkeitsregel (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:35`,
`:268`, `:382`). Die drei übrigen Stellen der Belegungsdatei, die ihn nennen (`:74`, `:90`,
`:604`), stimmen weiter: sie sprechen von der Aufteilung und nicht von ihrer Reihenfolge.

**Nicht behoben durch:** `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs:1546`) prüft die eine Zählzeile des Kopfes und sonst
keine. Für Prosa gibt es in dieser Datei keine Probe, und das ist der Grund, aus dem der Satz
still veralten konnte.

## Empfehlung

Die beiden Sätze auf die Fassung des Modulkopfs von `crates/krk-ui/src/appkit/ereignisse.rs`
zurückführen, ohne die Aussage zu verlieren, die für den Leser der Belegungsdatei zählt: dass
dieselbe Kombination im Textfeld etwas anderes bedeuten darf als im Dateifenster und dass das
kein zweiter Mechanismus ist. Etwa:

> Der Abgriff schlägt jede Kombination in dieser Datei nach und entscheidet nicht selbst, ob
> sie wirkt. Das tut die Zulässigkeitsregel am Anwendungsdelegierten, und der Fokusvorbehalt
> ist einer ihrer Bestandteile: gehört der Ersthelfer AppKit, fällt der Tastendruck
> unverändert an AppKit zurück, und erst dort wirkt das Menü. Die Regel fragt nicht nach der
> Kombination, und darum trägt sie eine nackte Taste genauso wie eine mit Zusatztaste.

**Den Datumsvermerk mitnehmen**, wie die Datei es an anderen Stellen tut (`:57-64`, `:818-823`):
ein Satz, der sagt, was bis zur Runde 7 galt, hält die nächste Durchsicht davon ab, den alten
Stand für den richtigen zu halten.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813, in `resources/default-keymap.toml` und ausschliesslich an Kommentarzeilen — keine Kennung, keine Taste, keine Zahl ist beruehrt.

Die beiden widerlegten Saetze sind durch die Fassung des Modulkopfs von `crates/krk-ui/src/appkit/ereignisse.rs` ersetzt: der Abgriff schlaegt jede Kombination in dieser Datei nach und entscheidet nicht selbst, ob sie wirkt; das tut die Zulaessigkeitsregel am Anwendungsdelegierten, und der Fokusvorbehalt ist einer ihrer Bestandteile. Die Aussage, die fuer den Leser dieser Datei zaehlt — dieselbe Kombination darf im Textfeld etwas anderes bedeuten als im Dateifenster, und das ist kein zweiter Mechanismus —, steht unveraendert davor.

Der empfohlene Datumsvermerk ist mitgenommen: ein eigener Absatz sagt, dass der Vorbehalt bis zur Runde 7 als frueher Ausstieg vor dem Nachschlag stand, nennt Commit `9da33bc` und die Schritte S1 bis S3, und nennt die **zwei** Stellen hinter dem Nachschlag, die heute denselben Wert lesen. Damit ist auch die Folgerung des zweiten Satzes wieder begruendet und nicht nur behauptet.

Mitgenommen ist die Beobachtung ohne Befundcharakter aus derselben Durchsicht (`:62-63`): der Satz zur Eingabetaste behauptete, ein stehendes Blatt fange sie ab, **bevor** sie hier nachgeschlagen werde. Die Blattregel sitzt beim Anwendungsdelegierten und damit hinter dem Nachschlag; der Satz sagt es jetzt.
