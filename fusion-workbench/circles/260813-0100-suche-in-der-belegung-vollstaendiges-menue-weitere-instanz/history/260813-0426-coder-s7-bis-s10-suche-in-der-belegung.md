# Coder: S7 bis S10 der Runde 7 — jedes getippte Zeichen sucht in der Belegungsansicht

**Datum:** 260813-0426
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte S7, S8, S9 und S10 aus
`circles/260813-0100-…/planning/260813-0205_o_plan-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz.md`,
in einem Lauf, samt dem Nachtrag vom 260813-0233
**Abnahme:** `cargo build --workspace` Exit 0, `cargo fmt --all --check` Exit 0,
`cargo clippy --workspace --all-targets -- -D warnings` Exit 0, `cargo test --workspace`
Exit 0. Proben im Binärziel `krk` vorher 511, nachher 533; in der Bibliothek `krk-core`
vorher 142, nachher 146; in `xtask` unverändert 46. Kein Bündelbau, kein Vordergrundlauf,
keine Messung; `target/KRK.app` ist unberührt.

## Was gebaut wurde

**S7.** `umlaufen` in `crates/krk-core/src/text/suche.rs` nimmt jetzt die **Zahl** der
Kandidaten statt der Trefferliste und ist damit von der Einheit unabhängig. Daneben stehen
`erster_ab_stelle(stellen, ab)` und `naechster_stelle(stellen, ab)` über aufsteigend
sortierten Zeilennummern; sie benutzen dieselbe Ringrechnung. `erster_ab`, `naechster` und
`voriger` sind in Signatur und Verhalten unverändert und rufen sie mit. Vier neue Randfälle
sind geprüft: leere Liste, Stelle vor der ersten, Stelle auf der letzten, Umlauf dahinter.

**S8.** `Suchlage` in `crates/krk-ui/src/belegungsmodell.rs` hält Suchtext, Trefferzeilen und
die Stelle darin, ohne eine Zeile AppKit. Sie bekommt `zeichen_anhaengen`,
`letztes_zeichen_weg`, `naechster_treffer`, `zielzeile` und `meldung`, dazu `nachrechnen` —
siehe unten unter „Was über den Plan hinausgeht". Die Trefferrechnung läuft über die neue
private `Belegungsmodell::zeile_traegt`, die `funktionstext` und `tastentext` liest, also
genau die zwei Spalten auf dem Schirm. Bereichsüberschriften sind ohne eigenen Zweig
ausgeschlossen: beide Wege liefern für sie `None`. Die Aufnahmeregel für ein Zeichen ist
`krk_core::verzeichnis::sprungmarke::traegt_ein_dateiname`, und es gibt keine zweite.

**S9.** `crates/krk-ui/src/appkit/belegungsansicht.rs` schaltet die eingebaute Tippauswahl der
`NSTableView` ab, hält eine `Suchlage` und bietet dem Fänger drei Wege an:
`suchzeichen_aufnehmen`, `suchzeichen_wegnehmen`, `zum_naechsten_treffer`. Alle drei schreiben
in die vorhandene Meldungszeile und setzen die Auswahl über den aus `nachziehen`
herausgezogenen `auswahl_setzen`. Die drei Schaltflächentasten stehen als Werte in
`SCHALTFLAECHEN`; `zeigen` setzt daraus die Tastenentsprechungen, `erlaeuterung()` schreibt
daraus den Satz unter der Überschrift. „Zuweisen" liegt auf Cmd+T, „Fertig" auf Cmd+Eingabe
über `Taste::EingabeMitBefehl`, „Auslieferungszustand" unverändert auf Cmd+R.

**S10.** Der Fänger nimmt jetzt `Fn(Tastendruck, Option<char>) -> bool`; `behandeln` reicht
`getipptes_zeichen(ereignis)` mit. `Anwendungsdelegierter::tastendruck_fangen` hat zwei
Stationen, und ihre Reihenfolge **ist** der Vorrang aus C1.15. Die Zuordnung steht als reine
Funktion `faengerstation(nimmt_auf, druck, zeichen) -> Faengerstation` neben dem Delegierten,
als vollständige Fallunterscheidung ohne Auffangzweig; siehe unten.

## Die drei Punkte, die die Diagrammprüfung ausdrücklich verlangt hat

**1. Der Vorrang der zwei `esc`-Bedeutungen ist gebaut, und zwar als Reihenfolge.** Läuft eine
Aufnahme, antwortet `faengerstation` für **jeden** Tastendruck `Aufnahme` — für das
Suchzeichen so gut wie für die Eingabetaste, die Rücktaste und `esc`. Die Probe
`waehrend_der_aufnahme_bekommt_die_suche_nichts` prüft genau das über sechs Fälle. Es gibt
keine dritte Regel und keine nebenläufige Region: der Wächter `[keine Aufnahme]` ist die
Stellung der zweiten Station hinter der ersten.

**2. Eingabetaste und Rücktaste bei leerem Suchtext bleiben wirkungslos** (C1.8, C1.17).
`Suchlage::naechster_treffer` und `Suchlage::letztes_zeichen_weg` liefern dann `false`, und die
Ansicht lässt daraufhin auch ihre **Meldungszeile stehen**, statt sie mit einer leeren
Suchmeldung zu überschreiben. Damit bleibt eine Zuweisungsbestätigung nach C1.10 sichtbar.
Das Ereignis ist trotzdem verbraucht: welche Station ein Ereignis bekommt, entscheidet der
Tastencode und nicht der Stand der Suche, und eine Fallunterscheidung nach dem Stand wäre die
zweite Regel, die C1.17 ausdrücklich ausschließt.

**3. Eingabetaste und Rücktaste fallen in die Suche und nicht in den Nachschlag** — der
Schritt gilt, nicht das Bild. `faengerstation` fragt beide vor dem Zeichenzweig ab.

## Was über den Plan hinausgeht, und warum

**`Suchlage::nachrechnen`, eine sechste Methode.** Der Plan nennt fünf. Ohne die sechste wäre
die Trefferliste nach einer Zuweisung oder einem Zurücksetzen veraltet: gesucht wird auch über
die Spalte „Belegung", und beide Vorgänge ändern sie. Die Eingabetaste spränge danach auf
Zeilen, die den Suchtext nicht mehr tragen. `Belegungsquelle::nachziehen` ruft sie jetzt mit,
ohne die Meldungszeile anzufassen.

**`Faengerstation` als reine Funktion neben dem Delegierten.** Der Plan verlangt für S10 die
Abnahme von C1.15 „als Fallunterscheidung über die zwei Stationen" und von C1.13. Beide
hängen sonst an `Anwendungsdelegierter` und wären ohne Fenster nicht prüfbar. Die Zuordnung
ist deshalb aus `tastendruck_fangen` herausgezogen; sie steht in derselben Datei, die der
Plan nennt, kennt kein AppKit und trägt jetzt fünf gewöhnliche Prüfungen. Keine davon braucht
`MainThreadMarker::new_unchecked`; der offene Zustand aus `issues/260810-1001` wächst durch
diese Runde nicht.

## Der Prüfvorbehalt zur eingebauten Tippauswahl

Der Spec hält offen, ob die Tippauswahl der `NSTableView` heute in der F1-Ansicht überhaupt
wirkt. Am Baum und am SDK nachgelesen, ungemessen:

- `allowsTypeSelect` steht ab Werk auf `YES` (`NSTableView.h:371-373`).
- Der Baum führt weder `tableView:typeSelectStringForTableColumn:row:` noch
  `nextTypeSelectMatchFromRow:` noch `shouldTypeSelectForEvent:`.
- Der Kopfkommentar zum Delegiertenweg sagt für den Vorgabefall: „By default, all cells with
  text in them are searched" (`NSTableView.h:665-667`). Die Zellen dieser Ansicht sind ein
  `NSView` mit einem `NSTextField` darin.

**Daraus folgt `inference:`, dass die Tippauswahl vor dieser Runde vermutlich wirkte** — belegt
ist es nicht, denn der Vorgabefall des Headers spricht von zellenbasierten Tabellen, und diese
ist ansichtsbasiert. Gemessen werden kann es nur am Bündel im Vordergrund.

**Die Frage ist nach S10 zweifach beantwortet, und die zweite Antwort hängt an nichts.** Der
Schalter steht jetzt auf `false`, und unabhängig davon erreicht kein Suchzeichen die Tabelle
mehr: die zweite Station des Fängers verbraucht es im lokalen Ereignisabgriff, also vor
`NSApplication::sendEvent:`. Selbst wenn AppKit den Schalter überginge, hätte die Tippauswahl
nichts mehr zu sehen.

## Was am Bündel zu sehen bleibt

Sechs Dinge, alle Nutzerarbeit und keines von einem Agenten abzunehmen:

1. Die springende Auswahl beim ersten getippten Zeichen und die Meldungszeile mit Suchtext,
   Trefferzahl und Stelle (C1.1, C1.9, C1.10).
2. Cmd+T weist zu, Cmd+Eingabe schließt, `esc` verlässt und sichert (C1.13, C1.16).
3. **Ob Cmd+T und Cmd+R die Schaltflächen überhaupt erreichen.** Beide tragen in
   `resources/default-keymap.toml` eine Funktion (`tab_neu`, `sortierrichtung_umkehren`), und
   beide sind bei stehendem Blatt unzulässig, weil `waehrend_blatt_erlaubt` allein den Abbruch
   durchlässt. Die Kürzel erreichen die Schaltflächen deshalb nur, wenn die Ausgrauung aus S6
   dem Menüeintrag sein Kürzel wirklich abnimmt. Das ist dieselbe Annahme, die die
   Risikotabelle des Plans für C2.6 nennt; sie trägt seit dieser Runde auch C1.16.
4. Dass die eingebaute Tippauswahl schweigt (C1.11).
5. Dass die Erläuterungszeile lesbar bleibt: sie ist um die Suche länger geworden.
6. L1 und L4 aus C8 der Runde 1, wie im Plan vorgesehen. Der Zeichenzweig kostet nichts
   Neues; die Trefferrechnung läuft über rund 90 Zeilen je Tastendruck und ist gegen das
   Budget eines Tastendrucks klein. Gemessen ist das nicht. `inference:`

## Abnahme im Einzelnen

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo test --workspace` | 0 |

Neue Prüfungen, 22 im Binärziel `krk` und 4 in `krk-core`:

- `krk-core`, `text::suche`: vier Randfälle der zwei neuen Stellenfunktionen.
- `belegungsmodell::suchproben`, zwölf: C1.1 bis C1.10, C1.12 und C1.17.
- `appkit::anwendung::faengerproben`, sechs: C1.13, C1.15, die Zusatztastengrenze, Cmd+Eingabe
  und das Ereignis ohne Zeichen.
- `appkit::belegungsansicht::tests`, drei: C1.11 über den gesetzten Schalter, C1.16 über die
  Werte der drei Kürzel und über die Erläuterungszeile.
- `appkit::ereignisse`, eine: C1.14, keine `keyDown:`-Überschreibung im Baum.

**Zwei der neuen Prüfungen lesen den Quellbaum statt eines Rückgabewerts**, und beide zählen
Erklärungen und keine Aufrufer, nach der Regel aus `crate::quellbaum`: der abgeschaltete
Schalter (C1.11) und die fehlende `keyDown:`-Überschreibung (C1.14). Für C1.11 ist das die
Wahl gegen eine gebaute `NSTableView`, die den Hauptfaden verlangte, den `libtest` nicht
hergibt. Was sie nicht sagt, steht in ihrem Doc-Kommentar: ob AppKit den Schalter befolgt.

## Zwei Befunde aus dem Bauen, beide beim ersten Lauf rot geworden

**Der Suchtext „datum" trifft mehr als eine Zeile.** Die erste Prüfung nahm an, „datum" fände
allein „Spalte Änderungsdatum ein- und ausblenden"; die Auslieferungsbelegung führt daneben
„Nach Änderungsdatum sortieren". Die Prüfung liest jetzt die ganze Trefferliste statt der
ersten Zielzeile und hält fest, dass alle drei Schreibweisen dieselbe Liste finden.

**Die Zeitgeberprobe fand ihre eigene Prosa.** Der Doc-Kommentar zu
`die_suche_fuehrt_keinen_zeitgeber` nannte den Typnamen wörtlich und ließ die Probe an sich
selbst scheitern. Die Nadeln stehen jetzt zusammengesetzt da und der Kommentar umschreibt den
Namen — dieselbe Vorkehrung, die `es_gibt_genau_einen_menuebauer` seit jeher trägt.

## Was nicht angefasst wurde

- Kein Bündelbau, kein `make bundle`, `make run`, `make frisch` oder `cargo xtask bundle`.
  `target/KRK.app` trägt unverändert den Stand vom 260813-0000.
- `resources/default-keymap.toml` ist unberührt; die drei Schaltflächentasten der
  Belegungsansicht liegen außerhalb der Belegung und bleiben dort.
- Der Defekt `issues/260813-0201_o_ein-kommentar-in-blaetter-mod-rs-nennt-eine-taste-variante-die-es-nicht-gibt.md`
  bleibt offen. S9 fasst `Blatt::mit_schaltflaechen` als Aufrufer an, aber nicht die Zeile mit
  dem falschen Kommentar; der Plan sagt ausdrücklich, dass er stehen bleibt.
- Der offene Datensatz
  `shared/decisions/260813-0053_o_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`
  bleibt offen. Gebaut ist seine Empfehlung (Möglichkeit 1); sie hat beim Bauen getragen, und
  es gab keinen Anlass, davon abzuweichen. Die Antwort gehört dem Nutzer.
- Die veralteten Zahlen in `CLAUDE.md` (68 Kommandos, 31 von 33 Dateien unter `appkit/`) sind
  nicht nachgezogen; der Plan legt das an den Schluss der Runde.

## Kein Commit

Der Orchestrator committet. Die fünf geänderten Dateien:

- `crates/krk-core/src/text/suche.rs`
- `crates/krk-ui/src/belegungsmodell.rs`
- `crates/krk-ui/src/appkit/belegungsansicht.rs`
- `crates/krk-ui/src/appkit/ereignisse.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
