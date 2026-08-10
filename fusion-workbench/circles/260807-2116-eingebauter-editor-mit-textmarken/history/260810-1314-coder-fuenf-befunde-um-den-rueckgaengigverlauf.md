# Fünf Befunde um den Rückgängigverlauf, die Formatansicht und den Prüfcode

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1314
**Datei:** `crates/krk-ui/src/appkit/editor.rs` (die einzige)

## Auftrag

Behebung von fünf Befunden der Durchsicht Turn 3
(`reviews/260810-1248-coderev-turn-3-die-behebung-der-achtunddreissig-defekte.md`),
alle in `crates/krk-ui/src/appkit/editor.rs`: die Befunde 1, 2, 3, 5 und 7 jener
Durchsicht. Gesperrt waren `hervorhebung.rs`, `vorschaumodell.rs`, `krk-core/**`
und `resources/**`, weil parallel bearbeitet; nicht anzufassen waren die vier
Instanzproben mit `MainThreadMarker::new_unchecked`.

## Befund 1 (Hoch): der Rückgängigstapel hielt je Handlung eine ganze Abschrift

`issues/260810-1241_o_…`

**Die Frage der Aufgabenstellung ist beantwortet, und die Antwort ist Nein:** die
Abschrift war nicht unvermeidlich. Eine Handlung braucht den geänderten Bereich
und nicht die Datei. `Umkehrpunkt` trägt jetzt `anfang`, `entfernt: String`,
`eingefuegt: usize` und wie bisher die Auswahl; gebildet wird er von
`Umkehrpunkt::zwischen(vorher, nachher, auswahl)` aus dem gemeinsamen Anfang und
dem gemeinsamen Schwanz, angewandt von `Umkehrpunkt::angewandt_auf(stand)`.

**Eine Darstellung und keine vier.** Alle vier Anlässe gehen durch dieselben zwei
Funktionen: das CRLF-Richten in `text_zurueckschreiben`, die beiden Ersetzungswege
und das Rückgängig selbst. Ein Sammelersetzen ändert viele Stellen, und der Punkt
fasst sie in **einen** Bereich von der ersten bis zur letzten zusammen — mehr als
das Notwendige und trotzdem exakt. Die Stellen zu führen hieße, die Regeln des
Ersetzens in `appkit/editor.rs` ein zweites Mal zu tragen; was ein Ersetzen
geändert hat, weiß `krk_core::text::suche`.

**Die Messung, an `EDITORGRENZE` und mit Gegenprobe.** Die zweite Zeile stammt aus
demselben Prüfcode, nachdem ich die alte Darstellung wieder eingesetzt hatte:

```text
                                  je Handlung        100 Handlungen
  ganzer Stand (bis 260810-1241)   16 777 219 B       1 677 721 900 B
  geänderter Bereich (jetzt)                3 B                 300 B
```

Die Probe ist `ein_umkehrpunkt_traegt_den_geaenderten_bereich_und_nicht_den_ganzen_stand`.
Sie hält den Bau an, sobald eine Handlung an einer Datei von 16 MB mehr als 1 kB
hält; mit der alten Darstellung meldet sie `der Punkt haelt mehr als das ersetzte
foo: 16777219 Bytes`, und die beiden Gleichheitsproben bleiben dabei grün — die
Probe misst also Speicher und nicht Richtigkeit.

**Der zweite Teil ebenfalls behoben.** `alle_treffer_ersetzen` fragt vor der
Abschrift `suchlauf().map(Suchlauf::zahl)`. Das ist keine zweite Wahrheit über die
Treffer: `Suchlauf` und `suche::alle_ersetzen` zählen mit **derselben** Funktion
`suche::alle` im selben Stand — nachgelesen in `krk-core/src/text/suche.rs:180`.

**Keine Tiefengrenze, und das ist eine Entscheidung mit Begründung.**
`setLevelsOfUndo` gälte für den ganzen Verwalter und damit für das Tippen, dessen
Tiefe heute unbegrenzt ist und die kein Abnahmekriterium beschränkt. Vor allem
löste sie den einen Fall nicht, der bleibt: eine Grenze in **Handlungen** fängt
einen Preis in **Bytes** nicht. Der Fall ist als eigener Datensatz abgelegt
(`260810-1314_o_ein-wiederholtes-sammelersetzen-…`) und im Doc-Kommentar von
`alle_treffer_ersetzen` benannt.

**Was bleibt und benannt ist:** die vorübergehende Abschrift. Wer einen Punkt
bildet, hält beide Stände gleichzeitig, und bei drei der vier Anlässe kommt der
alte als Kopie aus dem Modell. Sie fällt am Ende ihres Blocks und geht in keinen
Stapel; der Preis ist ein `memcpy` je Handlung neben den beiden Durchgängen, die
`suche` für dieselbe Handlung ohnehin fährt. Beim vierten Anlass entfällt sie
ganz: `umkehren` baut den wiederhergestellten Stand, den es `bearbeiten` ohnehin
übergibt, und vergleicht gegen ihn.

## Befund 2: ein `cmd+z` löschte den Suchlauf

`issues/260810-1244_o_…` — Weg 1, der empfohlene. `umkehren` merkt sich den
Suchtext vor dem Ruf an `bearbeiten` und bildet den Suchlauf danach über
`suche_starten` neu, ab der Stelle, an die dasselbe Rückgängig die Schreibmarke
setzt. Kein zweiter Weg zur Trefferliste, `bearbeiten` unangefasst.

## Befund 5: die Formatansicht nahm nichts wieder heraus

`issues/260810-1245_o_…` — Weg 2, der empfohlene, mit einer dritten
Zusammenlegung dazu, die ihn erst möglich macht:

1. `grundschrift(ansicht, art)` ist als freie Funktion herausgezogen; zwei
   Aufrufer, eine Rechnung.
2. `merkmale_zuruecksetzen(ansicht, art)` nimmt jetzt Absatzstil **und** Schrift
   heraus. Der Satz „`setFont:` erledigt die Schrift" ist gefallen — er galt nur
   für die vier Anlässe von `darstellung_nachziehen`, nicht für das Tippen.
3. `formatierung_anwenden` ruft es hinter der Längenprüfung und hat sein eigenes
   `setTemporaryAttributes:` mit dem leeren Verzeichnis **abgegeben**: eine
   Stelle, die zurücknimmt, nicht zwei halbe.

`crate::hervorhebung` ist nicht angefasst; die Behebung sitzt dort, wo die
Merkmale auf den Textspeicher gehen. Die Probe aus dem Vorschlag ist **nicht**
gebaut: sie bräuchte eine fünfte Instanzprobe unter der offenen Frage aus
`decisions/260810-1044_o_…`, die ausdrücklich nicht in einem Nebenzug beantwortet
werden soll. Was heute hält, ist die Zusammenlegung selbst.

## Befund 3: TextKit 1 war nirgends als tragend benannt

`issues/260810-1243_o_…` — `textflaeche_bauen` fasst `layoutManager` jetzt eigens
an, unmittelbar hinter `setAllowsUndo(true)` und mit dem Grund daneben; der
Modulkopf trägt denselben Satz samt Messung und samt dem Hinweis, dass sie
gemessen und nicht zugesagt ist. Die Probe ist
`die_gebaute_flaeche_steht_auf_textkit_1`, und die Reihenfolge ihrer beiden Fragen
ist ihre ganze Aussage: `textLayoutManager` **zuerst**, weil ein Zugriff auf
`layoutManager` den Rückfall selbst auslöst.

**Die Gegenprobe hat mehr gezeigt als erwartet.** Mit entfernter Zeile fällt die
Probe aus und meldet „steht auf TextKit 2". Vor dieser Behebung war die Fläche am
Ende von `textflaeche_bauen` also tatsächlich noch TextKit 2; der Rückfall geschah
erst im `darstellung_nachziehen` aus `Editorbereich::bauen`. Die Zusage hing damit
an der Reihenfolge zweier Aufrufe in einer anderen Funktion.

## Befund 7: eine Probe las ein Merkmal unter der Untergrenze

`issues/260810-1246_o_…` — und dabei ist eine Prämisse des Datensatzes falsch.

**`valueForKey:` liefert für einen unbekannten Schlüssel kein `nil`.** Es läuft in
`valueForUndefinedKey:` und wirft `NSUnknownKeyException`. Gemessen am 260810 auf
macOS 15.7.7 (Build 24G720) an einer `NSTextView` in Swift: Signal 6, „this class
is not key value coding-compliant for the key …". Eine Objective-C-Ausnahme ist in
Rust nicht zu fangen und beendet das **ganze** Prüfprogramm. Der vorgeschlagene
Weg — „dasselbe `msg_send!`, ohne die Panik" — hätte den Defekt deshalb nicht
behoben.

`merkmal_falls_vorhanden` fragt daher `respondsToSelector:` **vorher**, und zwar
nach dem **Setzer**: die Lesernamen sind nicht einheitlich, und eine Frage nach
dem bloßen Merkmalsnamen meldete die Hälfte von `EINSTELLUNGEN` als fehlend.
`setzername` stand dafür schon im Baum. Beide Richtungen sind gegengeprüft: mit
dem echten Namen läuft die Zusicherung, mit einem erfundenen steht der Hinweis auf
`std::io::stderr` und die Reihe bleibt grün.

## Was außerhalb der Dateigrenze liegt

Der Verweis in die Gegenrichtung, den `260810-1243` für
`nummernspalte.rs:89-93` verlangt. Abgelegt als
`260810-1314_o_der-verweis-in-die-gegenrichtung-fehlt-der-nummernspalte-noch.md`.
Tragend ist er nach dieser Behebung nicht mehr, weil der Rückfall nicht mehr an
der Nummernspalte hängt.

## Abnahme

```text
cargo build --workspace                 exit 0
cargo test --workspace                  exit 0   751 Proben, 0 Fehlschläge
cargo clippy --workspace --all-targets  exit 0
cargo fmt -p krk-ui -- --check          exit 0
```

Vier neue Proben: die Speichermessung, die Gleichheit in beiden Richtungen über
vierzehn Fälle, die gehaltene Form des wiederhergestellten Standes und der
Stolperdraht für TextKit 1. Drei von ihnen sind gegengeprüft, also gefahren mit
eingesetztem Fehler, und fallen dann aus.

**Nicht gefahren ist das laufende Bündel.** Die Wirkung der Befunde 1, 2 und 5 am
Bündel im Vordergrund bleibt Nutzerarbeit, aus dem Grund, der in `CLAUDE.md` unter
„Was man nicht sieht" steht.
