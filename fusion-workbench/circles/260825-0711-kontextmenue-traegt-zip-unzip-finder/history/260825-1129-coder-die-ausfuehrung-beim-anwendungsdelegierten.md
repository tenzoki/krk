# Coder: Die Ausführung beim Anwendungsdelegierten

**Datum:** 2026-08-25 11:29
**Status:** Complete
**Agent:** coder
**Baumstand:** `8b5a5ce` (Schritt 8) plus die Änderungen dieses Schritts, nicht eingecheckt

## Auftrag

Schritt 7 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`, der
letzte Planschritt: den `Kontextmelder` je Fensterseite setzen, `kontextbefehl_ausfuehren`
vollständig und ohne Auffangzweig über `Kontextbefehl` verzweigen, die drei Zweige bauen, die
verfallenden `expect(dead_code)`-Marken der Schritte 4 und 6 wegnehmen und zwei Zählungen in
Doc-Kommentaren nachziehen. Bis zu diesem Schritt war alles gebaut, und kein Zweig führte einen
Kontextmenü-Befehl aus.

## Was entstanden ist

In `crates/krk-ui/src/appkit/anwendung.rs`:

- Der **neunte** Rückruf der Dateiliste in `oberflaeche_aufbauen`, in derselben Schleife über
  `Fensterseite::ALLE` wie die acht darüber und in derselben Form: `objc2::rc::Weak` auf den
  Delegierten, die Seite als `Copy`-Wert mitgeführt, `if let Some(selbst) = schwach.load()`.
- `kontextbefehl_ausfuehren(seite, befehl)`: drei Zeilen, eine je `Kontextbefehl`, kein
  Auffangzweig.
- `zipauftrag_stellen(seite)`: `vorgang_laeuft_schon`, dann `betroffene_eintraege`, bei leerer
  Menge `nichts_zu_packen()`, sonst `kontextmenue::archivname` und `Auftrag::zippen`.
- `entpackauftrag_stellen(seite)`: dieselbe Frage, dann `DateifensterQuelle::entpackbefund`; die
  beiden Fehlbefunde melden `kein_archiv()` und `mehrere_archive()`, sonst `Auftrag::entpacken`
  über die Paarliste.
- `im_finder_zeigen(seite)`: `operationen::ordner_fehlt`, dann `terminal::ordner_oeffnen` mit der
  neuen Konstante `FINDERKENNUNG`, sonst `kein_finder()`.
- Zwei Proben im neuen Modul `kontextproben`.

In `crates/krk-ui/src/appkit/tabelle.rs`: `entpackbefund()` neben `betroffene_eintraege()` und
der Wegfall des `#[expect(dead_code, …)]` an `kontextmelder_setzen`.

In `crates/krk-ui/src/kommandos/kontextmenue.rs` und `kommandos/operationen.rs`: die fünf
verfallenen `expect`-Marken.

## Vier Stellen, an denen der Plan im Baum nicht hielt

**Erstens: `entpackziel` ist vom Anwendungsdelegierten aus nicht zu rufen, und deshalb ist
`tabelle.rs` eine zweite angefasste Datei.** Der Schritt nennt `anwendung.rs` als einzige Datei
und sagt, der Unzip-Zweig rufe `kontextmenue::entpackziel`. Jene Regel verlangt neben den
betroffenen Einträgen das `Ordnermodell` der sichtbaren Zeilen; es liegt in `QuelleIvars`, deren
Felder dateiprivat sind, und der Delegierte kommt an es nicht heran. Gebaut ist deshalb
`DateifensterQuelle::entpackbefund()`, Zeile für Zeile nach dem Vorbild des vorhandenen
`betroffene_eintraege()`: eine Ausleihe des Tabmodells, kein Rechnen. Der Zweig fragt sie und
verzweigt über den `Entpackbefund`. Der Zuschnitt bleibt damit der geplante — die Regel steht in
`kommandos/kontextmenue.rs` und ist ohne Fenster prüfbar; was hinzukam, ist der Weg zum Modell,
nicht eine zweite Regel. Nebenbei sehen beide Fragen dadurch **eine** Ausleihe und damit
denselben Stand des Ordners.

**Zweitens: es sind acht Rückrufe und nicht sechs.** Der Schritt sagt „genau wie die sechs
Rückrufe daneben"; die Schleife über `Fensterseite::ALLE` setzt `aktivierung`, `ordnerwechsel`,
`auswahlmelder`, `umbenennung`, `meldungswechsel`, `vorgang_laeuft`, `abwurf` und
`befehlsantwort_raeumer`, also acht. Der Doc-Kommentar von `befehlsantwort_beidseitig_loeschen`
nennt jenen achten seit der Runde 13 auch so. Der neue ist der neunte; die Form ist von den acht
übernommen und die Abweichung folgenlos.

**Drittens: die zwei nachzuziehenden Zahlen sind andere, als der Plan sie ansagt.** Der Plan
verlangt, `vorgang_laeuft` von „vier Wege, drei und einer" auf „fünf und vier zu einem" zu
stellen; nachgezählt sind es **sechs Wege, fünf und einer**. Zip und Unzip sind zwei neue Frager
und nicht einer: jeder Zweig fragt für sich, weil der Finder-Zweig nicht fragen darf und eine
gemeinsame Vorfrage in `kontextbefehl_ausfuehren` die Verzweigung über den Wert aufgeweicht
hätte. Die fünf, die den meldenden Mantel nehmen, sind `auftrag_stellen`, `stapel_beauftragen`,
`loeschen_nach_rueckfrage`, `zipauftrag_stellen` und `entpackauftrag_stellen`; der eine ohne ihn
bleibt der Abwurf. Ebenso bekommt `auftrag_starten` nicht den fünften, sondern den **fünften und
sechsten** Rufer; sein Kopf zählt sie jetzt einzeln auf, statt nur ihre Zahl zu nennen.

Daneben stand in `loeschen_nach_rueckfrage` der Kommentar, `vorgang_laeuft_schon` baue die
Meldung „für alle drei Frager". Es sind fünf; die Zahl ist gefallen und durch „für jeden ihrer
Frager" ersetzt, weil sie mit jedem weiteren Weg wieder falsch würde.

**Viertens: es verfallen fünf `expect`-Marken und nicht eine.** Der Schritt nennt die an
`kontextmelder_setzen` und sagt, weitere seien zu prüfen. Der Übersetzer hat sie genannt: dazu
die Marke am **ganzen Modul** `kommandos/kontextmenue.rs` und je eine an `nichts_zu_packen`,
`kein_archiv`, `mehrere_archive` und `kein_finder` in `kommandos/operationen.rs`. Alle fünf sind
weg. An ihrer Stelle steht je ein Satz, der den Rufer nennt — dieselbe Form, die Schritt 8 für
`erzeugt_genau_ein_ziel` gewählt hat. Der Abschnitt „Die Ausnahme mit Ablaufdatum" im Modulkopf
von `kontextmenue.rs` ist zu „Wer hier hereinruft" geworden und hält fest, dass die Ausnahme
abgelaufen ist, statt sie zu beschreiben.

`ordner_fehlt` hat mit dem Finder-Zweig seinen zweiten Aufrufer bekommen. Weder sein
Doc-Kommentar noch der Modulkopf von `terminal.rs` nennt eine Zahl — Schritt 5 hat das bewusst
so gelassen —, beide nennen die Frage und ihre zwei Steller. Nachzuziehen war dort nichts.

## Was die zwei neuen Proben halten

Die Falle dieser Runde hat drei Stellen, und der Übersetzer hält davon eine: dass jeder
`Kontextbefehl` einen Zweig hat. Die Probe aus Schritt 6 hält eine zweite, den Selektor. Die
zwei verbleibenden schließt `mod kontextproben`:

- `der_kontextmelder_wird_beim_aufbau_gesetzt` zählt über den Quellbaum, dass
  `kontextmelder_setzen` genau eine Aufrufstelle hat und dass sie in `anwendung.rs` steht. Ohne
  diesen Aufruf fällt jeder Klick still durch das `let Some(melden)` in `kontextbefehl_melden` —
  ohne Fehler, ohne Meldung, ohne rote Probe. Gezählt wird die Aufrufstelle und nicht die zwei
  Fensterseiten: der Aufruf steht in der Schleife und läuft zweimal, im Quelltext steht er
  einmal.
- `jeder_kontextbefehl_erreicht_seine_wirkung` liest den Rumpf von `kontextbefehl_ausfuehren`
  und die drei Zweigrümpfe über `zettelproben::rumpf`: jeder Befehl nennt seinen Zweig, und jeder
  Zweig ruft seine Wirkung (`Auftrag::zippen(`, `Auftrag::entpacken(`,
  `terminal::ordner_oeffnen(`). Ein Zweig mit leerem Rumpf übersetzt und ließe jede andere Probe
  grün.

Beide Grenzen stehen an ihren Doc-Kommentaren: die erste sieht nicht, ob der Aufruf in einem nie
erreichten Zweig steht, die zweite liest den Rumpf und nicht den Aufrufbaum darunter.

## Was dieser Schritt nicht getan hat

- **Kein Commit**, wie beauftragt. Die fünf Entscheidungsdatensätze dieses Circles behalten
  deshalb ihren Marker `_a_`: ein `Implemented:` ohne Commit-Hash wäre kein Beleg, und den Hash
  gibt es noch nicht. Dieselbe Überlegung wie im Nachtrag zu Schritt 8.
- **KRK ist nicht gestartet worden.** Dass die drei Einträge am gebauten Bündel wirken, ist
  Nutzerarbeit und Teil des Abnahmelaufs.
- **Die fremde Prosastelle aus Schritt 6 ist weiter offen**: der Kopf von
  `crates/krk-ui/src/appkit/teilen.rs` sagt unter „Ein Menue, ein Bauer, drei Flaechen", die
  Dateiliste baue kein Menü. Für den Freigabeeintrag stimmt der Satz, als Aussage über die
  Dateiliste seit Schritt 6 nicht mehr. Die Datei stand nicht im Auftrag.

## Abnahme

`make check` läuft grün, Exit 0. 792 Proben in `krk-ui` statt 790 vor dem Schritt, die zwei
neuen sind die beiden oben. `cargo doc -p krk-ui --no-deps` meldet zu keiner der neuen Stellen
einen unaufgelösten Verweis.
