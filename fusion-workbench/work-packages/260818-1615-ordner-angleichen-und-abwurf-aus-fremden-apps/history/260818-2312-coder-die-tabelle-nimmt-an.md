# Coder — Schritt 10: Die Tabelle nimmt an

**Datum:** 260818-2312
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 10
**Spec:** `shared/planning/260818-1510_o_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md`, C4 bis C7
**Baumstand beim Beginn:** `8d5baf6`, Arbeitsbaum sauber. Kein zweiter Agent lief.

## Was der Auftrag war

Der letzte Schritt der Runde: die Dateiliste nimmt einen Abwurf aus einer
fremden Anwendung wirklich an. Fünf Teile in der vom Plan gesetzten
Reihenfolge — Anmeldung, Ivars, die zwei Protokollmethoden, die zwei Rümpfe,
das Einhängen der Rückrufe.

## Was entstanden ist

**`crates/krk-ui/src/appkit/tabelle.rs`**

- **(a) Anmeldung** in `Dateifenster::bauen`, zwischen `setDataSource`/
  `setDelegate` und dem Doppelklick: `tabelle.registerForDraggedTypes(&abwurf::sorten())`.
- **(b) Vier neue Ivars** statt der drei des Plans: `vorgang_laeuft`, `abwurf`,
  `gemeldeter_abwurfgrund` und `beschlossener_vorgang`. Der vierte ist eine
  Abweichung und unten begründet. Dazu die Setzer `vorgang_laeuft_setzen` und
  `abwurf_setzen` sowie die Typnamen `Vorgangsfrage` und `Abwurfmelder`.
- **(c) Die zwei Protokollmethoden** `tableView:validateDrop:proposedRow:proposedDropOperation:`
  und `tableView:acceptDrop:row:dropOperation:` im vorhandenen
  `unsafe impl NSTableViewDataSource`-Block, je ein Einzeiler auf einen Rumpf
  daneben.
- **(d) Die zwei Rümpfe** `abwurf_pruefen` und `abwurf_annehmen`, dazu
  `vorgang_laeuft_fragen` und die reine Funktion `abwurfmeldung` mit der
  Konstanten `KEINE_DATEI`.
- Der Untergrenzen-Abschnitt des Modulkopfs bekommt die acht Berührungen des
  Abwurfs, jede am SDK nachgelesen.
- `befehlsantwort_loeschen` räumt den gemerkten Abwurfgrund mit weg.

**`crates/krk-ui/src/appkit/anwendung.rs`** — **(e)** die zwei Rückrufe in der
Schleife von `oberflaeche_aufbauen`, in derselben Form wie die fünf
vorhandenen, der Delegierte je **schwach** gehalten. Der `expect(dead_code)` an
`abwurf_ausfuehren` ist gefallen.

**Gefallene `expect(dead_code)`-Vermerke**, alle vom Übersetzer eingefordert:
`zwischenablage::dateiverweise`, `abwurfregel::marke`, `abwurfregel::urteil`,
`abwurf::sorten`, `abwurf::beschreibbarkeit`, `abwurf::angebot`,
`abwurf::zeiger` und `Anwendungsdelegierter::abwurf_ausfuehren`. Die
Modulkopf-Abschnitte, die sie ankündigten, sind auf die Vergangenheitsform
gezogen.

**Die zwei Aufruferzählungen** in `abwurfregel.rs` stehen auf eins;
`die_marke_hat_noch_keinen_aufrufer` heißt jetzt
`die_marke_hat_genau_einen_aufrufer`, ebenso die Schwesterprobe.

## Zwei neue Proben, beide ohne Fenster

- `genau_eine_ansicht_meldet_sich_fuer_einen_abwurf_an` — eine
  Aufruferzählung über den ganzen Baum. Sie hält das letzte Kriterium von C4:
  die Lesezeichen- und Geräteleiste nimmt keinen Abwurf an, und getragen wird
  das allein davon, dass `leiste.rs` sich nicht anmeldet.
- `die_tafel_der_abwurfmeldung_geht_auf` — die vollständige 6×6-Tafel der
  Entdopplung, Erwartungen als Werte und nicht gerechnet.

**Keine Probe baut eine `NSTableView` oder ein `NSDraggingInfo`**, wie der Plan
es unter Punkt 8 seiner offenen Fragen festlegt. C4 bis C7 bleiben sämtlich
Nutzerarbeit.

## Die Entscheidungen, die der Plan offen ließ

**1. Woher `abwurf_annehmen` seinen Vorgang nimmt — ein viertes Ivar.**
Der Plan nennt drei Ivars und sagt für `abwurf_annehmen` nur, dass es „Ziel und
Quellen noch einmal bestimmt". Der `Abwurfvorgang` fehlt darin, und AppKit
reicht ihn in `acceptDrop:` nicht mit. Es gäbe drei Wege: `urteil` ein zweites
Mal rufen (bricht die Aufruferzählung und wäre eine zweite Beurteilung),
`angebot` erneut lesen und die Tafel aus C5 daneben noch einmal bauen (zweite
Wahrheit), oder das gefällte Urteil merken. Gewählt ist der dritte:
`beschlossener_vorgang: Cell<Option<Abwurfvorgang>>`. Es steht **neben** und
nicht **in** `gemeldeter_abwurfgrund`, weil die Löschregeln entgegengesetzt
sind: der gemerkte Grund fällt mit der Befehlsantwort, dieses Feld darf das
nicht — ein Tastendruck während eines stehenden Ziehvorgangs nähme dem
Loslassen sonst seinen Vorgang. Ein Feld mit zwei Löschregeln ist derselbe
Sonderfall, den `fenstermeldung` und `vorgangsanzeige` schon vermeiden.

**2. Der gemerkte Abwurfgrund fällt mit der Befehlsantwort.**
Der Plan sagt, die Meldung bleibe stehen und falle mit der nächsten
Befehlsantwort. Über das **Gedächtnis** sagt er nichts. Stünde es länger als
die Zeile, die es beschreibt, bliebe eine zweite gleiche Ziehbewegung nach
einem Tastendruck stumm: die Entdopplung verglich gegen einen Grund, dessen
Meldung es nicht mehr gibt. `befehlsantwort_loeschen` räumt es deshalb mit weg,
und die Löschregel des Rangs 1 gilt damit für beide Felder an einer Stelle.

**3. „Das Ziel ist der Quellordner" heißt: jede Quelle liegt darin.**
Der Spec spricht unter C6 vom „Ordner, aus dem gezogen wird", im Singular, und
setzt damit einen Ziehvorgang aus einem Ordner voraus. Ein Abwurf kann aus
mehreren kommen. Gewählt ist `all` und nicht `any`: liegt nur ein Teil der
Einträge im Ziel, ist es nicht der Fall aus C6, und die Einträge, die dort schon
stehen, treffen auf dieselbe Konfliktrückfrage wie bei F5 und F6. Das ist genau
die Antwort, die `auftrag_stellen` für die Auswahl gibt, wo alle Quellen ohnehin
aus einem Ordner kommen. `any` wiese den ganzen Abwurf ab, und zwar stumm —
`SelberOrdner` trägt keine Meldung.

**4. Ein fehlender Rückruf gilt als „es läuft ein Vorgang".**
`vorgang_laeuft_fragen` liefert `true`, wenn der Rückruf nicht steht. Das ist
die vorsichtige Füllung einer Tatsache, die KRK dann nicht messen kann, und sie
fällt mit den übrigen fünf in dieselbe Regel statt in einen eigenen Ausgang
daneben — ein früher Rückgabewert `NSDragOperation::None` an dieser Stelle wäre
eine dritte Übersetzung von `NSDragOperation` neben `angebot` und `zeiger`, die
der Kopf von `abwurf.rs` ausdrücklich ausschließt. Eintreten kann der Fall
nicht: der Rückruf steht seit `oberflaeche_aufbauen`, und ein Ziehvorgang
braucht ein stehendes Fenster.

**5. Der Text der einen Meldung**: „die Quelle liefert keine Datei auf dem
Datenträger", als Konstante `KEINE_DATEI` in `tabelle.rs`. Der Spec gibt unter
C7 den Inhalt vor, nicht den Wortlaut.

**6. Das Zurücklesen der Zeile in `acceptDrop:`.** `abwurf_annehmen` bestimmt
das Ziel aus der Zeile, die `abwurf_pruefen` an der Tabelle gesetzt hat, und
fällt damit keine zweite Marke — `marke` behält seinen einen Aufrufer. Der Kopf
des Systems sagt es zu: „'row' and 'dropOperation' contain the values previously
set in the validateDrop: method" (`NSTableView.h:785`).

## Was dem Plan widerspricht

**Die Anmeldung braucht keinen `unsafe`-Block.** Der Plan verlangt sie „in einem
eigenen `unsafe`-Block mit eigenem SAFETY-Satz". `objc2` bindet
`registerForDraggedTypes:` **sicher**
(`objc2-app-kit-0.3.2/src/generated/NSView.rs:1412-1414`, ein `pub fn` ohne
`unsafe`); ein Block davor ist ein `unused_unsafe` und hält den Bau unter
`-D warnings` an. Gemessen, nicht angenommen: der erste Bau hat genau das
gemeldet. An der Stelle steht jetzt der Grund samt Fundstelle statt eines
SAFETY-Satzes, der nichts zu tragen hätte.

**Vier `expect(dead_code)`-Vermerke fallen mehr, als der Plan nennt.** Er nennt
die aus den Schritten 6, 7 und 9, also drei. Die vier aus Schritt 8
(`abwurf.rs`) fallen ebenso, weil deren Aufrufer erst hier entstehen — der Kopf
jener Datei sagt es selbst, der Plan hat es in seiner Schritt-10-Zeile
vergessen. Der Übersetzer hat alle vier eingefordert.

**Zwei Zeilenangaben des Plans an das SDK sind neu gelesen.** Der Modulkopf
führt jetzt `NSDragging.h:79` für `draggingPasteboard`, `NSTableView.h:25` für
`NSTableViewDropOperation`, `:319` für `setDropRow:dropOperation:` und `:783`
beziehungsweise `:787` für die zwei Protokollmethoden. Keine trägt ein
`API_AVAILABLE`; alle stehen seit 10.0, die Untergrenze des Bündels ist 15.0.

**Der Plan nennt `abwurf_ausfuehren` den dritten Rufer von `auftrag_starten`.**
Es ist der vierte; der Baum sagt es, und der Doc-Kommentar an jener Funktion
schreibt es seit Schritt 9 aus. Ein Datensatz deckt es bereits ab, ein zweiter
entsteht nicht.

## Prüfung

`make check` — Exit 0. Vier Kommandos in einem Zug: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`. Vor dem Lauf geprüft,
dass weder `/tmp` noch `$TMPDIR` eine `krk-messplan-*.toml` führt; kein Messlauf
stand daneben.

## Was offen bleibt

- **C4 bis C7 sind gebaut und nicht abgenommen.** Sämtliche Abnahmekriterien
  verlangen einen Ziehvorgang aus einer zweiten Anwendung; kein Agent kann ihn
  auslösen. Die Liste steht im Plan unter „Nutzerarbeit".
- **`shared/decisions/260818-1453_*_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md`**
  trägt weiter `_a_`. Der Plan setzt den Wechsel auf `_i_` „nach dem Commit von
  Schritt 10" an, und dieser Schritt hat nicht committet; der Datensatz braucht
  den Kurzhash, den es noch nicht gibt.
- **Der Abnahmelauf der zehn Zeitzusagen ist nicht gefahren** und für diese
  Runde auch nicht vorgesehen; der Plan begründet es unter „Testing Strategy".
