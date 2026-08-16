# E3: Das zehnte Ankreuzfeld

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt E3
**Baumstand vor der Arbeit:** `3dd799a` plus die Arbeitsbaumstände von E1 und E2
**Vorbedingungen:** E1 und E2, beide im Arbeitsbaum
**Erfüllt:** C2.1, C2.2, C2.3
**Nicht committet:** E3 landet zusammen mit E1 und E2. Der Baum war zwischen E1 und diesem Schritt rot, und mit diesem Schritt ist er wieder grün.

## Was entstanden ist

Zwei Dateien, und der Schalter „Content" ist Zeile für Zeile die Bauart von
„Deep".

**`crates/krk-ui/src/appkit/bereichsleiste.rs`** bekommt die zwei Konstanten
`KOMMANDO_DES_INHALTS` und `AUFSCHRIFT_DES_INHALTS`, den Selektor
`inhaltGedrueckt:` in der `Leistenquelle`, das Feld `inhaltsschalter` neben
`tiefenschalter`, den Bau samt `einhaengen` am Ende von
`Bereichsleiste::bauen` und die vierte Zeile in `zustaende_setzen`.

Eingehängt ist der Schalter mit `ABSTAND` und nicht mit `GRUPPENABSTAND`: die
zwei Sucheinstellungen des sichtbaren Tabs sind eine Gruppe, und der größere
Abstand trennt Gruppen. Auf dem Schirm steht „Content" damit unmittelbar
rechts von „Deep", weil die Reihenfolge der Schalter die Reihenfolge der
`einhaengen`-Aufrufe ist.

C2.2 fällt ohne eigene Zeile an. `schalter_bauen` setzt
`setRefusesFirstResponder(true)` für jeden Schalter in einer Zeile, und der
zehnte geht durch dieselbe Zeile wie die neun vorhandenen. `Fokus::ALLE` bleibt
bei fünf Werten, und die Probe hält es fest.

**`crates/krk-ui/src/appkit/anwendung.rs`**: `bereichsleiste_nachziehen` holt
die Quelle des aktiven Dateifensters einmal aus und fragt sie zweimal, nach
`tiefe_suche_steht` und nach `inhaltssuche_steht`. Das ist die Stelle, die
`inhaltssuche_steht` aus E1 gefehlt hat; ihr Fehlen hat
`cargo clippy -- -D warnings` mit `method inhaltssuche_steht is never used`
angehalten, und mit diesem Schritt ist der Baum wieder grün.

**Kein vierter Anlass des Nachzugs.** Der Stand von „Content" hängt am selben
`Ordnermodell` des sichtbaren Tabs wie der von „Deep", und die drei Anlässe der
Runde 10 decken ihn deshalb mit ab: der Ordner- und Tabwechsel über
`ordnerwechsel_setzen`, der Wechsel des aktiven Dateifensters über
`aufteilung_nachziehen`. Damit ist C2.3 erfüllt, ohne dass eine Zeile
hinzugekommen wäre. Der Kommentar am Ordnerwechsel-Rückruf nennt jetzt beide
Schalter statt nur „Deep".

## Was an den Proben und Doc-Zahlen nachgezogen ist

`die_leiste_traegt_neun_schalter` heißt `die_leiste_traegt_zehn_schalter` und
prüft zehn. `der_neunte_schalter_heisst_deep_und_steht_rechts_von_typ` heißt
`die_zwei_letzten_schalter_heissen_deep_und_content_und_stehen_rechts_von_typ`
und prüft jetzt drei Stellen von rechts: „Content", „Deep", „Typ".
`alle_schalter()` nimmt den Eintrag auf und liest dafür dieselben Konstanten,
aus denen `bauen` den Schalter baut.

`der_neunte_schalter_gibt_fokus_keinen_sechsten_wert` heißt
`der_zehnte_schalter_gibt_fokus_keinen_sechsten_wert`. Der Plan nennt diese
Umbenennung nicht ausdrücklich, verlangt aber, dass die Doc-Stellen, die
„neun" sagen, danach „zehn" sagen; ein Probenname, der den zehnten Schalter
den neunten nennt, wäre genau der Rückstand, den die Regel meint.

`jeder_schalter_nennt_genau_ein_eigenes_kommando` und
`jeder_schalter_wirkt_aus_jedem_fokus` halten von selbst, wie der Plan es
ansagt.

Der Modulkopf trägt den Abschnittstitel `# Die zwei letzten Schalter sind
einzelne Felder und keine dritte Sammlung` statt `# Der neunte Schalter ist
eine Gruppe und keine dritte Sammlung`. Die Aussage bleibt dieselbe und gilt
jetzt für zwei einzelne Felder: eine Aufzählung über zwei Werte bräuchte einen
Namen, eine Reihenfolge und ein Nachschlagen und spräche damit über mehr, als
es gibt.

## Keine neue AppKit-Berührung

Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` in
`bereichsleiste.rs` ist unverändert, und das ist eine Feststellung und kein
Vergessen: der zehnte Schalter geht durch `schalter_bauen` und `einhaengen`,
also durch genau die Methoden, die die neun vorhandenen schon ansprechen.
`checkboxWithTitle:target:action:` bleibt mit 10.12 die höchste Untergrenze
der Datei. `anwendung.rs` bekommt gar keine neue Berührung, nur einen zweiten
Ruf an eine Methode dieses Projekts.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, darunter
`cargo clippy --workspace --all-targets -- -D warnings`, das vor diesem Schritt
angehalten hat. Die sieben Proben von `appkit::bereichsleiste::tests` laufen
grün. Die Wettrennprobe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
ist im selben Lauf durchgelaufen und nicht angefasst.

## Was Nutzerarbeit bleibt

Vier Wirkungen sind nur am laufenden Bündel zu sehen und von keiner Probe
gedeckt:

- C2.1 am Schirm: das Feld steht mit der Aufschrift „Content" rechts neben
  „Deep", und die Leiste behält ihre 18 Punkte Höhe.
- C2.2 am Schirm: ein Klick auf das Feld verschiebt die Fokusanzeige nicht.
- C2.3 am Schirm: bei zwei Tabs mit verschiedenem Stand zieht die Leiste beim
  Tabwechsel nach.
- C2.5: nach einem Neustart steht „Content" aus.
