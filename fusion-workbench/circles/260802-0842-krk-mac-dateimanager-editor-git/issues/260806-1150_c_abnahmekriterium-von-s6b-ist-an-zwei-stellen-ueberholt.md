Das Abnahmekriterium von S6b ist an zwei Stellen überholt

---

Schritt 6b des Plans
`planning/260802-1428_o_plan-navigator-geruest-runde-1.md:647` nennt drei
Abnahmen. Zwei davon können seit S8, S13 und S21 nicht mehr erfüllt werden,
und zwar nicht wegen der Umsetzung von S6b, sondern weil beide Sätze einen
Bestand beschreiben, der zum Zeitpunkt ihrer Niederschrift galt.

**Erstens: „`grep -n 'eprintln!' crates/krk-ui/src/appkit/anwendung.rs`
findet nichts."** Der `None`-Zweig des Tastenabgriffs, den S6b meint, ist
entfernt. Übrig bleiben sechs `eprintln!` in derselben Datei
(Zeilen 634, 643, 652, 2311, 2377, 2387), und alle sechs gehören dem
Messmodus aus S8 und S21. Sie sind dort der **richtige** Kanal: der
Messmodus wird ausschließlich unmittelbar aus dem Terminal gestartet
(`target/KRK.app/Contents/MacOS/krk --messmodus …`), hat also eine
Standardfehlerausgabe, und jede der sechs Meldungen endet mit
`std::process::exit`, weil es dann keine Zahl gibt. Sie durch ein modales
Hinweisfenster zu ersetzen wäre falsch: ein Messlauf, der auf einen Klick
wartet, misst nichts mehr.

Der Satz meint erkennbar den einen Zweig und nicht die Datei. Fix: das
Kriterium auf ihn einengen, etwa „`grep -n 'eprintln!'` in
`tastenabgriff_einrichten` und `tastenabgriff_nachziehen` findet nichts; die
verbleibenden Vorkommen gehören dem Messmodus".

**Zweitens: „dass `hinweis.rs` die einzige Datei mit einem `NSAlert` ist."**
Das war wahr, als S6b geschrieben wurde — der Schritt sagt selbst, die
gemeinsame Hülle der Blätter entstehe erst in S13 und S6b dürfe nicht darauf
warten. S13, S16 und S17 sind inzwischen abgenommen, und
`crates/krk-ui/src/appkit/blaetter/mod.rs` baut einen `NSAlert` für die
Blätter am Fenster. Zwei Dateien legen damit einen an, `blaetter/mod.rs` und
`hinweis.rs`; die vier weiteren Treffer unter `blaetter/` sind reine
Kommentarerwähnungen.

Zusammengelegt gehören die beiden nicht, und derselbe Planschritt sagt das
zwei Zeilen vorher: ein Blatt hängt an einem Fenster, sperrt nur dieses,
kehrt sofort zurück und liefert eine Antwort, mit der die Arbeit weitergeht;
der Hinweis braucht kein Fenster, sperrt die Anwendung, kehrt erst nach der
Bestätigung zurück und ist die letzte Ausgabe vor dem Beenden. Fix: das
Kriterium auf „`hinweis.rs` ist die einzige Datei mit einem
**anwendungsmodalen** `NSAlert`" ändern, oder auf „außer der Blätterhülle
aus S13 legt keine weitere Datei einen `NSAlert` an".

---

Gefunden bei der Umsetzung von S6b am 260806. Beide Sätze sind
Buchführung, keine Verhaltensfrage: an dem, was S6b baut, ändert sich
nichts, und der Schritt ist nach seinem dritten Kriterium (Sichtprüfung am
laufenden Bündel) abgenommen. Wer die beiden Sätze aber wörtlich nimmt,
hält S6b für unerfüllt oder reißt fremde Schritte ein.
Adressat: planner oder reconciler. Schwere: niedrig, Plandrift.

---
Resolved: Beide Kriterien im Plan neu gefasst (Plannachzug 260806-1313). Kriterium 1 verlangt jetzt kein eprintln! in tastenabgriff_einrichten und tastenabgriff_nachziehen statt in der ganzen Datei; die sechs übrigen gehören dem Messmodus und haben ein exit dahinter. Kriterium 2 verlangt NSAlert::new an zwei Stellen, davon allein hinweis.rs anwendungsmodal über runModal, statt hinweis.rs als einzige Stelle; blaetter/mod.rs legt seit S13 einen an, und derselbe Planschritt begründet zwei Zeilen vorher, warum beide getrennt bleiben.
