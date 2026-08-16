Die Probe gegen Zeitmessung im Filter erreicht zwei Dateien des Filterwegs nicht, und eine davon kann sie nicht erreichen
---
`im_filter_steht_keine_zeitmessung` (`crates/krk-core/tests/verzeichnis.rs:1718-1739`) nennt fünf
Dateien und sucht in ihnen nach den Nadeln `Instant`, `Duration` und `::now(`. Der Weg eines
getippten Zeichens reicht heute schon über zwei weitere Dateien, die nicht in der Liste stehen:
`crates/krk-core/src/verzeichnis/sys.rs` und `crates/krk-core/src/verzeichnis/leser.rs`. Der
Durchlauf öffnet jeden Ordner über `sys::Schwungleser` und holt `STAPELGROESSE` aus `leser.rs`.

**`sys.rs` kann die Liste nicht betreten.** Die Datei führt `std::time::Duration` an vier Stellen
(`:94`, `:387`, `:391`, `:392`), und zwar zur Umrechnung der Änderungszeit eines Eintrags und nicht
zur Messung. Die Nadel `Duration` kann die beiden Verwendungen nicht trennen; die Datei
einzutragen erzeugte einen Fehlalarm, der die Probe dauerhaft rot ließe.
---
Gefunden beim Planen der elften Runde (Inhaltsfilter), beim Prüfen, welche neuen Dateien des
Filterwegs die Liste nach C6.8 aufzunehmen hat. Die zwei Dateien dieser Runde,
`crates/krk-core/src/text/datei.rs` und `crates/krk-core/src/verzeichnis/inhalt.rs`, sind frei von
allen drei Nadeln und treten der Liste bei; der Plan trägt das als Schritt A2.

Der Befund betrifft die **Reichweite** der Probe und nicht ihren Bestand: sie hält, was sie prüft,
und prüft weniger, als ihr Name verspricht. Zwei Wege stehen offen und keiner gehört in diese
Runde: die Nadeln so verfeinern, dass eine Zeitumrechnung von einer Zeitmessung zu trennen ist,
oder die Zusage von „im Filter steht keine Zeitmessung" auf „in den Dateien, die der Filter
besitzt, steht keine" zurücknehmen und das ausschreiben. Die erste Möglichkeit ist eine Suche im
Quelltext, die eine Bedeutungsfrage entscheiden soll; `crates/krk-core/tests/baum.rs` schreibt im
Modulkopf aus, warum das nicht trägt.
