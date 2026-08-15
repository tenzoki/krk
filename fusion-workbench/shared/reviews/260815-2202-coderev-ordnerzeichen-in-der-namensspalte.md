# Durchsicht: das Ordnerzeichen in der Namensspalte (`3b128c3`)

**Sender:** coderev
**Datum:** 260815-2202
**Reviewed-range:** `c27d845..3b128c3`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Grundlage:** `shared/decisions/260815-2056_i_woran-erkennt-der-nutzer-in-der-dateiliste-einen-ordner.md`,
`shared/history/260815-2110-coder-ein-ordner-traegt-einen-schraegstrich-in-der-namensspalte.md`

## Zusammenfassung

Die Änderung hält ihre teuerste Zusage: **der Schrägstrich kommt an keiner Stelle des Baums
in die Nähe eines Namens.** `namensform` hat genau einen Rufer, dessen Ergebnis genau ein
Ziel, und der einzige Rückweg ist über `becomeFirstResponder` abgesichert; ein Restpfad
bleibt und wird von `name_pruefen` gefangen. Die neue Unterklasse ist sauber gebaut, ihre
drei Untergrenzen sind am SDK nachgezählt und stimmen auf die Zeile. Acht Befunde stehen
daneben, keiner kritisch: zwei betreffen Aussagen, die der Baum nicht trägt, einer ein neues
Fehlverhalten bei einem Zeichendurchgang während der Bearbeitung, die übrigen sind
Absicherungen, die dieses Projekt sonst setzt.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 4 |
| Niedrig | 4 |

## Was geprüft ist und hält

**Kein Leck der Anzeigeform.** Vollständig am Baum nachgegangen und nicht abgeleitet:

```
namensform (tabelle.rs:346)
  └─ genau ein Rufer: beschriften, Zweig Spalte::Name (:2718)
       └─ genau ein Rufer: zellenansicht (:2595)
            └─ genau ein Ziel: feld.setStringValue (:2596)

Leser eines Namenszellentextes im ganzen Baum:
  umbenennung_beenden (:1751)   ── gegen name_pruefen abgesichert
  wird_ersthelfer     (:2881)   ── nimmt das Zeichen ab
```

Sortierung, Filter (`krk-core/src/verzeichnis/filter.rs`), Zwischenablage, Vorschau,
Stapelumbenennen und die Dateioperationen lesen `eintrag.name` und bekommen die Anzeigeform
nie zu sehen; `grep` über `crates/` findet außerhalb von `tabelle.rs` keine Nennung von
`namensform`, `ohne_ordnerzeichen` oder `ORDNERZEICHEN`. Der eine Restpfad, auf dem die
Anzeigeform doch in `umbenennung_beenden` ankommt, ist Befund 2 — er endet in einer
Ablehnung, nicht in einer falschen Umbenennung.

**Die Unterklasse bricht den Ereignisabgriff nicht.** `ersthelfer_gehoert_appkit` fragt mit
`isKindOfClass(NSTextField::class())` (`ereignisse.rs:688-690`), und ein `Namensfeld` ist
eines. Hätte die Stelle auf Klassengleichheit geprüft, wären während einer Umbenennung
plötzlich KRK-Befehle durchgekommen; sie tut es nicht.

**Die Wiederverwendung ist dicht.** Der Vorrat von `makeViewWithIdentifier:owner:` ist nach
Kennung getrennt, und jede der vier Spalten trägt ihre eigene (`tabelle.rs:245-252`). Ein
Feld der Namensspalte kommt nur in die Namensspalte zurück; `downcast::<NSTextField>` trifft
die Unterklasse.

**Der Rückweg über `target` trägt.** `NSControl.target` ist im Kopf des Systems
`@property (nullable, weak) id target` (`NSControl.h:24`); ein gestorbener Delegierter
liefert `nil`. Was daran offen ist, steht in Befund 5 und betrifft die nächste Änderung,
nicht den heutigen Zustand.

**Die drei Untergrenzen stimmen auf die Zeile.** Am SDK nachgesehen:
`becomeFirstResponder` steht auf `NSResponder.h:105`, `abortEditing` auf `NSControl.h:89`,
`target` auf `NSControl.h:24`, alle drei ohne `API_AVAILABLE` und damit seit 10.0. Die
jüngste Berührung der Datei bleibt die 11.0 aus `NSTableViewStyle`; der Modulkopf stimmt
weiter.

**`becomeFirstResponder` ist die richtige Tür.** Die naheliegende Alternative
`-[NSTextField textShouldBeginEditing:]` (`NSTextField.h:34`) fällt aus demselben Grund aus
wie die Delegiertenfassung: `NSText` stellt die Frage erst beim ersten Ändern des Textes.
Der Schrägstrich stünde bis zum ersten Tastendruck im Editor.

**`make check` läuft hier grün.** Selbst gefahren am 260815-2200:
`cargo build --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` ohne
Ausgabe, `cargo fmt --all --check` ohne Ausgabe, `cargo test --workspace` mit 1 180
bestandenen Proben und keiner roten.

## Befunde nach Thema

### Aussagen, die der Baum nicht trägt

**1 (mittel) — „genau diese Schleife messen L3 und L10" ist falsch.**
`tabelle.rs:325-346`, `:3281-3283`, dazu die Commit-Nachricht, der
Entscheidungsdatensatz und das Sitzungsprotokoll. L2, L3 und L10 laufen auf der kopflosen
Strecke (`krk-bench/src/messen.rs:1199-1202`, Beschriftungen `:1073-1082` und `:1153-1160`);
sie baut keine `NSTableView` und ruft `zellenansicht` nie. Die Zeichenschleife der
Dateiliste ist in diesem Baum von **keiner** der zehn Zusagen gemessen — was den Verzicht
auf ein `stat` je Zeile stärker begründet als der heutige Satz, nicht schwächer.
Datensatz `260815-2202_o_vier-stellen-sagen-l3-und-l10-messen-die-zellenschleife-…`.

**3 (mittel) — der Doc-Kommentar von `umbenennung_beenden` nennt den Fokusverlust als
Aufrufer der Aktion.** `tabelle.rs:1727-1729`. Die Messtabelle in
`shared/issues/260815-2125_o_…`, aus derselben Sitzung, sagt für den Fokusverlust „Aktion:
nein". Der Commit hat genau diesen Absatz angefasst und den falschen Halbsatz stehen
lassen. Datensatz `260815-2204_o_der-doc-kommentar-von-umbenennung-beenden-…`.

### Ein neuer Ausgang, den niemand betrachtet hat

**2 (mittel) — ein Zeichendurchgang während der Bearbeitung schreibt `Name/` in den
Feldeditor.** `zellenansicht` beschriftet die Zelle in jedem Durchgang (`tabelle.rs:2596`),
auch wenn sie gerade der offene Editor ist; der `coder` hat gemessen, dass ein
`setStringValue:` dann in den Feldeditor zurückschreibt. Ausgelöst wird der Durchgang ohne
Zutun des Nutzers, von der Dateisystemwache (`auffrischung::ordner_neu_lesen` →
`tabelle.rs:827-842`) oder vom Takt des Lesevorgangs (`:2337`, `:2357`); der Aufschub aus
`schiebt_auffrischung_auf` greift nur für ein Stapel-Umbenennen. Return liefert danach
`Bilder/` an `umbenennung_beenden`, `name_pruefen` weist ab, und der Nutzer liest eine
Meldung über ein Zeichen, das er nie getippt hat. **Vor der Änderung endete dieselbe Folge
still als `Unveraendert`.** Kein Datenverlust. Datensatz
`260815-2203_o_ein-zeichendurchgang-waehrend-einer-offenen-umbenennung-…`.

### Absicherungen, die dieses Projekt sonst setzt

**4 (mittel) — die Zusage „an genau einer Stelle und nirgends sonst" hält keine Zählprobe.**
Der Entscheid sagt es ausdrücklich, und das ist eine Aussage über den Baum, an keinem
Rückgabewert ablesbar. `quellbaum::aufrufstellen` (`krk-ui/src/quellbaum.rs:133`) ist das
Werkzeug, das dieser Baum für genau solche Zusagen hält — für die zwei Filterregeln, für die
Ersthelferfrage, für den einen Menübauer. Hier hält es nichts. Datensatz
`260815-2205_o_die-zusage-der-schraegstrich-entsteht-an-genau-einer-stelle-…`.

**5 (niedrig) — das Ziel der Feldaktion hat zwei Leser, die Setzstelle nennt einen.**
`tabelle.rs:2808-2811` gegen `:2947-2950`. Wird das Ziel je umgehängt, fällt Zusage 3 still:
`delegierter()` liefert `None`, und `bearbeitung_abbrechen` geht wortlos weiter. Der
Modulkopf derselben Datei führt den Vergleichsfall vor („`clickedRow` … hat seit dem 260812
zwei Abnehmer statt einen"). Datensatz `260815-2206_o_das-ziel-der-feldaktion-…`.

**6 (niedrig) — `wird_ersthelfer` behandelt das Nein der Oberklasse nicht.**
`tabelle.rs:2879-2889`. Liefert `[super becomeFirstResponder]` `false`, ist das Zeichen weg
und nichts holt es zurück. `inference:` in diesem Baum nicht erreichbar gezeigt; die
Fallunterscheidung ist trotzdem unvollständig, und der `else`-Zweig kostet zwei Zeilen.
Datensatz `260815-2207_o_wird-ersthelfer-nimmt-das-ordnerzeichen-weg-…`.

**7 (niedrig) — der Filter nimmt den Schrägstrich an.** `traegt_ein_dateiname`
(`krk-core/src/verzeichnis/filter.rs:64`) fragt laut Doc-Zeile „Ob ein Dateiname dieses
Zeichen tragen kann" und antwortet für `/` mit ja, während zwei andere Stellen des Baums das
Gegenteil festhalten. Der Widerspruch ist alt; sichtbar wird er jetzt, weil der Nutzer
`Bilder/` liest und `bilder/` tippen kann. Datensatz `260815-2208_o_der-filter-nimmt-den-…`.

**8 (niedrig) — die Abnahme steht nur im Sitzungsprotokoll.** Der Entscheidungsdatensatz
trägt `_i_` und ist endständig; die zwei Nachweise eine Stufe hinter der Taste und der
ungemessene Klickeinstieg stehen unter `## Offen` einer Datei ohne Marker und fallen aus
jeder Suche. Datensatz `260815-2209_o_die-drei-umbenenn-zusagen-…`.

## Übergreifend

**Der Zeichendurchgang der Zeile ist die eine Rückholstelle, und drei Ausgänge münden nicht
in ihn.** Return (über `umbenennung_beenden`) und Escape (über `abortEditing`) erreichen ihn;
der Fokusverlust erreicht ihn nicht (`260815-2125`, Befund besteht), und der
Zeichendurchgang **während** der Bearbeitung läuft in die falsche Richtung (Befund 2). Die
drei hängen zusammen: alle drei fragen, wann die Zelle die Anzeigeform trägt und wann den
Namen. Eine Antwort an einer Stelle — `zellenansicht` fragt vor dem Beschriften nach
`currentEditor`, und `Namensfeld` überschreibt `textDidEndEditing:` — deckte zwei davon ab
und beantwortet dabei die offene Nutzerfrage nicht mit. Der Nachtrag dazu steht im
Datensatz `260815-2125`.

**Die Messdisziplin des `coder` ist der Grund, warum diese Änderung trägt** — und derselbe
Maßstab fehlt bei den Zahlen. Sechs Fragen an AppKit sind am wirklichen Hauptfaden gemessen
und haben die Empfehlung des Entscheids gekippt; die Behauptung über L3 und L10 ist dagegen
weitergereicht worden, ohne dass jemand `messen.rs` aufgeschlagen hätte. Beides steht in
derselben Commit-Nachricht.

## Reihenfolge

**Kein Auslieferungshindernis.** Die Version 0.4.3 kann mit diesem Stand gehen: die drei
Zusagen des Entscheids halten, soweit sie gemessen sind, und der schlimmste bekannte Ausgang
ist eine überflüssige Fehlermeldung.

1. Befund 1 und 3 — zwei Prosaberichtigungen, jede eine Handvoll Zeilen, und beide betreffen
   Sätze, auf die sich der nächste Entwurf stützen wird.
2. Befund 2 zusammen mit dem Nachtrag zu `260815-2125` — eine Frage, zwei Hälften, eine
   Antwort. Vorher am Hauptfaden messen, wie alles andere an dieser Datei.
3. Befund 4, 5, 6 — Absicherungen, in einem Zug zu erledigen.
4. Befund 7 und 8 — unabhängig, jederzeit.
