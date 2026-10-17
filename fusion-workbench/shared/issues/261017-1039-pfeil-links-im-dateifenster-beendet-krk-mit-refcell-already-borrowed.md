Pfeil links im Dateifenster beendet KRK mit `RefCell already borrowed`
---
Seit den sftp-Ordnern (`bfb5ce77`, ausgeliefert mit 3.5.0) beendet jedes `left` im Dateifenster den Prozess, gleich welcher Eintrag gewählt ist und gleich ob der Ort lokal oder fern ist. `DateifensterQuelle::ordner_aufwaerts` (`crates/krk-ui/src/appkit/tabelle.rs`) prüfte `self.ivars().tabs.borrow().aktiver().tabort().clone()` als Ausdruck eines `match`; der Temporärwert `Ref` lebt bis zum Ende des ganzen `match`, und `ordner_lesen` bzw. `fern_lesen` im Zweig verlangen dieselbe Tabliste mit `borrow_mut`. Die Panik läuft auf dem Hauptfaden aus dem Block des Ereignisabgriffs hinaus und beendet KRK mit Exit-Code 101, also ohne Absturzbericht unter `~/Library/Logs/DiagnosticReports/`.
---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

## Fehlerbericht

Nutzer, Version 3.5.0, wörtlich: „wenn ich jetzt auf eine zip gehe und dann pfeil rechts drücke oder anschließend pfeil links, stürtst die app ab, inhalte werden nicht angezeigt.“

## Befund

Die Zip-Datei ist nicht die Ursache. `right` liegt auf `oeffnen` (`resources/default-keymap.toml`) und landet über `Kommando::Oeffnen` in `DateifensterQuelle::in_zeile_einsteigen`; für `Typ::Datei` antwortet der Weg lokal wie fern mit `Einstieg::KeinOrdner` und tut nichts. Das ist die festgelegte Antwort für jede Datei („der Rechts-Pfeil tut nichts“, Kommentar im Rumpf), und daher kommt „Inhalte werden nicht angezeigt“: das Anzeigen von Zip-Inhalten gibt es nicht. Erst das folgende `left` (`ordner_aufwaerts`, `Kommando::OrdnerAufwaerts`) stürzt ab, und zwar in jedem Ordner.

Nachgestellt am laufenden Programm, mit einem vorübergehend eingebauten Startweg, der unter eigenem `HOME` einen Prüfordner mit `archiv.zip` öffnet, die Datei wählt und `oeffnen`, `ordner_aufwaerts` als synthetische Tastenereignisse über `ereignisse::funktion_senden` in die eigene Ereignisschlange stellt (der Startweg ist wieder entfernt):

```
REPRO: sende oeffnen, key=true
REPRO: sende ordner_aufwaerts, key=true
thread 'main' panicked at crates/krk-ui/src/appkit/tabelle.rs:1815:27:
RefCell already borrowed
   6: DateifensterQuelle::ordner_lesen      tabelle.rs:1815
   7: DateifensterQuelle::ordner_aufwaerts  tabelle.rs:3208
   8: DateifensterQuelle::kommando_ausfuehren
   ...
  12: Anwendungsdelegierter::abgriff_aufsetzen::{closure#1}
exit=101
```

Derselbe Lauf mit `ordner_aufwaerts` allein in einem Ordner ohne Zip-Datei endet ebenso mit 101. Vor `bfb5ce77` stand die Ausleihe in einer eigenen Bindung (`let hier = self.ivars().tabs.borrow().aktiver().ordner().to_path_buf();`) und endete vor dem Ruf.

Absturzberichte gibt es keine: KRK beendet sich nach der Panik regulär mit 101, und ReportCrash schreibt dafür nichts. Im Systemprotokoll der letzten zwei Tage steht kein Absturz von `krk`.

## Ähnliche Stellen

Jeder Prüfausdruck eines `match`, `if let` oder `while let` in `crates/krk-ui/src`, der eine `RefCell` leiht, ist durchgesehen (19 Stellen). Allein `ordner_aufwaerts` ruft im Zweig einen Weg, der dieselbe Zelle mit `borrow_mut` verlangt; die übrigen 18 lesen im Zweig nur oder leihen eine andere Zelle. Die mit 3.5.0 neu hinzugekommenen benannten Ausleihen der Tabliste (`betroffene_namen`, `betroffene_namen_und_ordnerzahl`, `fern_eintrag_in_zeile`) enden vor jedem Ruf, der sie verändert.

Eine Panik auf dem Hauptfaden beendet KRK auch an jeder anderen Stelle. Ein Auffangen am Ereignisabgriff (`catch_unwind`) ist hier nicht eingebaut: es ließe die Tabliste und die Oberfläche in einem Zustand mitten im Wechsel stehen, und die Ursache ist die Bauform der Ausleihe, nicht das Fehlen eines Netzes.

## Behebung

`ordner_aufwaerts` bindet den geklonten `Tabort` vor dem `match` an `ort`; die Ausleihe endet mit der Anweisung. Zwei Proben in `crates/krk-ui/src/appkit/tabelle.rs`:

- `der_aufstieg_haelt_die_tabliste_nicht_geliehen` baut eine Datenquelle mit Tabelle auf behauptetem Hauptfaden, fährt `Kommando::OrdnerAufwaerts` lokal und an einem fernen Ort über `fernprobe::ferner_ort` und prüft den erreichten Ort. Ohne Behebung rot mit `RefCell already borrowed`.
- `kein_pruefausdruck_leiht_die_tabliste` liest die Datei und weist jeden Prüfausdruck eines `match`, `if let` oder `while let` ab, der `tabs.borrow` enthält. Ohne Behebung rot mit der Stelle in `ordner_aufwaerts`. Sie sieht keine benannte Ausleihe und keine andere Zelle als `tabs`.

`right` auf einer Datei bleibt ohne Wirkung und ohne Meldung, wie festgelegt. Das Anzeigen von Zip-Inhalten gehört nicht zu dieser Behebung.

Die Änderung berührt keinen der zehn Zeitwege aus C8: sie verschiebt das Ende einer Ausleihe vor einen Ruf, der ohnehin folgt.

---
Resolved: `ordner_aufwaerts` leiht die Tabliste in einer eigenen Bindung statt im Prüfausdruck des `match` (`crates/krk-ui/src/appkit/tabelle.rs`, `DateifensterQuelle::ordner_aufwaerts`); die Proben `der_aufstieg_haelt_die_tabliste_nicht_geliehen` (lokal und fern, zur Laufzeit) und `kein_pruefausdruck_leiht_die_tabliste` (Bauform, ganze Datei) halten es.
