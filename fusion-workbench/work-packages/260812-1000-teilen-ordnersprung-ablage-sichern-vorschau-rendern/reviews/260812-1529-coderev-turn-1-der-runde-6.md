# Codedurchsicht: Turn 1 der Runde 6 — Teilen, Ordnersprung, Ablagesicherung

**Sender:** coderev
**Reviewed-range:** `4d4402d..d6eff4b`
**Not-opened:** `resources/default-keymap.toml`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/_t_circle.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_braucht-die-vorschau-mit-gerendertem-markdown-mehr-mindestbreite.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_was-tut-die-nummernspalte-bei-gerendertem-markdown.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_welchen-umfang-von-markdown-rendert-die-vorschau.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_wie-erfaehrt-der-nutzer-dass-eine-ablagedatei-zur-seite-gelegt-wurde.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_a_zeigt-die-vorschau-lokale-html-dateien-gerendert.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_an-welchen-drei-flaechen-haengt-das-neue-kontextmenue.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_oeffnet-der-ordnersprung-einen-neuen-tab-oder-wechselt-er-den-aktiven.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_teilt-krk-auch-ordner-oder-nur-dateien.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_was-tut-der-ordnersprung-wenn-es-keinen-zielordner-gibt.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_wie-heisst-die-zur-seite-gelegte-ablagedatei-und-was-geschieht-beim-zweiten-mal.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_i_wird-die-datei-im-zielordner-ausgewaehlt.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1105_a_die-statuszeile-zieht-ueber-die-volle-fensterbreite-und-laesst-sich-blaettern.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1145-planner-session.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1204-coder-ablage-beschaedigte-datei-zur-seite-legen.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1351-ontocoder-belegung-ordner-der-datei-und-teilen.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1432-coder-ordnersprung-in-den-ordner-der-angezeigten-datei.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1500-coder-kontextmenue-an-den-drei-flaechen.md`, `fusion-workbench/circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1600-coder-rechtsklick-bewegt-die-auswahl.md`, `fusion-workbench/circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_s_ist-die-neue-leiste-die-statuszeile-aus-c1-oder-eine-zweite-flaeche.md`, `fusion-workbench/shared/issues/260812-1438_o_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`

Alle fünfzehn Rust-Dateien des Bereichs sind geöffnet. Die oben genannten
Werkbank-Dokumente liegen außerhalb des zugewiesenen Bereichs („Dein Bereich
sind die Rust-Dateien"); `resources/default-keymap.toml` ist dem `ontorev`
zugewiesen und hier nur nach den zwei neuen Kennungen und der Zählzeile
durchsucht, nicht gelesen.
`history/260812-1434-coder-teilen-ueber-die-tastatur.md` ist in Teilen gelesen
und deshalb nicht in der Liste.

---

## Zusammenfassung

Die sechs gefahrenen Planschritte sind sauber gebaut: `make check` läuft am
Baum durch, die Fokusverzweigung des Teilens und die Rechnung über die
angezeigte Datei sind vollständig und überschneidungsfrei, die
Ablagesicherung hält die vorgeschriebene Reihenfolge, und jede
macOS-Untergrenze in den geänderten Modulköpfen stimmt mit dem SDK überein.
Fünf Befunde stehen dagegen, und **die beiden gewichtigen fallen beide erst am
Bündel oder beim Nutzer auf**: der Freigabedialog wird aus einem Tastendruck
statt aus einem Mausdruck geöffnet, was der Kopf des Systems ausdrücklich
verlangt, und eine Ablagedatei mit ungültigem UTF-8 fällt an der neuen
Sicherung vorbei. Kein Befund hält einen der fünf noch offenen Planschritte
auf.

## Zahlen

| Gewicht | Zahl |
|---|---|
| Kritisch (Freigabeblocker, Sicherheit, Datenverlust) | 0 |
| Hoch (Korrektheitsfehler, gebrochener Ablauf) | 0 |
| Mittel (Korrektheitsrisiko, Wartbarkeit) | 2 |
| Niedrig (Kosmetik, Aufräumen) | 3 |

Alle fünf sind als eigene Datensätze unter
`circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/issues/`
abgelegt; alle fünf sind aus der Directive dieser Runde entstanden und gehören
deshalb in deren Speicher und nicht in den gemeinsamen.

## Was am Baum geprüft und in Ordnung ist

Diese Liste steht hier, weil sie die Grenze der Befunde zieht: was hier steht,
ist nachgesehen und nicht bloß nicht beanstandet.

- **`make check` läuft grün.** `cargo clippy --workspace --all-targets -- -D warnings`
  Exit 0 und `cargo test --workspace` Exit 0, am 260812 gefahren; 410 Proben im
  Binärziel `krk`, wie die Sitzungsaufzeichnung des `coder` es angibt.
- **Die Deckung des Untergrenzenabschnitts ist gehalten.** 33 von 35 Dateien
  unter `crates/krk-ui/src/appkit/` rekursiv, ohne ihn nur `koordinaten.rs` und
  `mod.rs`. Die neue `teilen.rs` trägt ihn.
- **Jede Zahl in den geänderten Köpfen ist am SDK gegengelesen**, nicht am
  Kommentar: `NSSharingServicePicker` 10.8 (`NSSharingService.h:253`),
  `initWithItems:` (`:261`), `showRelativeToRect:ofView:preferredEdge:` ohne
  eigene Angabe (`:271`), `standardShareMenuItem` 13.0 (`:281`),
  `NSURL`-Konformität zu `NSPasteboardWriting` (`NSPasteboard.h:469`),
  `NSMenu.insertItem:atIndex:` (`:89`), `numberOfItems` (`:118`),
  `removeAllItems` 10.6 (`:112`), `delegate` schwach (`:156`), `NSMenuDelegate`
  (`:269`), `menuNeedsUpdate:` (`:271`), `NSMenuItem.separatorItem`
  (`NSMenuItem.h:27`), `NSResponder.menu` (`NSResponder.h:111`), `NSView.bounds`
  (`NSView.h:139`), `NSTableView.clickedRow` ohne `API_AVAILABLE`
  (`NSTableView.h:276`). Keine Abweichung.
- **Die Ablagesicherung hält die vorgeschriebene Reihenfolge und den einen
  Zweig.** `beiseite_legen` (`crates/krk-core/src/ablage/mod.rs:397-411`) bildet
  den Pfad, fragt `try_exists`, schreibt nur dann, und wird ausschließlich im
  Zweig `Grund::Beschaedigt` gerufen (`:344-357`). Kopiert und nicht
  verschoben; `atomar::schreiben` ist der eine Schreibweg, und die Probe
  `ein_gescheitertes_zur_seite_legen_wird_gemeldet_und_verspricht_keine_datei`
  weist das nach, indem sie die Nachbardatei des atomaren Schreibens versperrt.
- **`angezeigtedatei::welche` ist über die vier Eingaben vollständig und
  überschneidungsfrei.** Beide Zweige fragen zuerst die Sichtbarkeit, die
  Bedingungen schließen sich nach `Bereich::teilt_flaeche_mit` aus, und was
  durch beide fällt, ist `None`. Der Doc-Kommentar begründet, warum die
  Sichtbarkeit entscheidet und nicht das Halten; ohne ihn baut der nächste
  Leser die Abfrage um.
- **`teilen::worauf` deckt alle fünf Fokuswerte**, und jede Zuordnung stimmt mit
  C1.2 des Plans überein: Dateifenster und `Anderswo` auf die betroffenen
  Einträge, Vorschau und Editor auf die angezeigte Datei, Leiste auf nichts.
  Die Tafel im Prüfmodul ist von Hand geschrieben und nicht aus der Funktion
  abgeleitet, und eine zweite Probe hält fest, dass sie jeden Wert genau einmal
  nennt.
- **`clickedRow` liefert `-1`, und der Fall ist behandelt.**
  `operationen::rechtsklick_zielzeile` fängt ihn über `usize::try_from` ab, die
  Zeile jenseits der Liste über `Ordnermodell::eintragsindex`, die markierte
  Zeile über `ist_markiert`; fünf Proben decken die vier Antworten.
- **`betroffene` bleibt die eine Auswahlregel.** Sie ist nicht angefasst; der
  Rechtsklick bewegt die Auswahl **vor** ihr, über `zeile_setzen` und damit über
  `auswahl_merken`, den Weg der Tastatur. Nachgezählt hat sie nach dieser Runde
  sieben Abnehmer (vorher fünf, an `4d4402d` gezählt), und die Zahl im
  Doc-Kommentar stimmt damit.
- **Die Vorschau wird kein zweiter bedienbarer Textbereich.** Ihre `NSTextView`
  bleibt `setEditable(false)` und `setSelectable(false)`
  (`crates/krk-ui/src/appkit/vorschau.rs:701-702`), nimmt den Ersthelferrang
  also nicht an; `ersthelfer_gehoert_appkit` (`appkit/ereignisse.rs:536`) sieht
  sie nie und braucht keine Anmeldung. Die neue Delegiertenrolle ändert daran
  nichts.
- **Die Blattregel weist `Teilen` und `OrdnerDerDatei` ab, solange ein Blatt
  steht.** `waehrend_blatt_erlaubt` lässt allein den Abbruch durch, und beide
  neuen Befehle gehen durch `kommando_ausfuehren`. Die andere Richtung ist ein
  eigener Befund, siehe unten.
- **`#[must_use]` ist dort gesetzt, wo die Projektregel es verlangt**, und nicht
  darüber hinaus im schädlichen Sinn: `anbieten` (das stille Fallenlassen
  verschluckte die Meldung an die Statuszeile), `rechtsklick_zielzeile` (es
  verschluckte die Auswahlbewegung), `beiseite_legen`. Dass `worauf` ihn trägt
  und `angezeigtedatei::welche` nicht, ist keine Lücke: beide sind reine
  Rechnungen, deren Wegwerfen nichts verbirgt, und `fenstertitel::titel`
  daneben trägt ihn aus demselben Grund seit jeher nicht.
- **Die beiden Zählproben tragen, was sie behaupten** — mit einer benannten
  Grenze. `allein_diese_datei_baut_den_freigabewaehler` zählt Dateien, die
  `NSSharingServicePicker::` enthalten, und `es_gibt_genau_einen_menuebauer`
  zählt Fundstellen von `fn eintrag_anfuegen` und `.standardShareMenuItem(`.
  Beide Nadeln stehen zusammengesetzt, damit die Probe sich nicht selbst zählt,
  und das ist richtig gelöst. Die Grenze: eine umbenannte Einfuhr
  (`use ... as SP;` und danach `SP::alloc()`) entkäme der ersten. Das ist keine
  Lücke, die ich melde — die Probe misst eine Schreibweise und sagt das selbst.

## Befunde

### Mittel

**M1 — Der Freigabedialog wird nicht aus einem Mausdruck heraus geöffnet.**
`crates/krk-ui/src/appkit/teilen.rs:222`. Der Kopf des Systems stellt an
`showRelativeToRect:ofView:preferredEdge:` eine Bedingung, die der Tastenweg
`shift+cmd+s` nicht erfüllt: „Note that this method must be called on
mouseDown" (`NSSharingService.h:268-271`, am SDK gelesen). Der einzige Weg zu
`anbieten` führt über den Tastenabgriff auf `NSEventMask::KeyDown`. Betroffen
ist C1.1, das erste Kriterium der Runde. `inference:` Was bei Verletzung
geschieht, sagt das SDK nicht, und ohne laufendes Bündel ist es nicht zu sehen;
der Rechtsklickweg über `standardShareMenuItem` ist von der Bedingung nicht
betroffen, ein Abnahmelauf, der nur ihn prüft, beantwortet die Frage also
nicht. Ausweichweg, falls der Tastenweg nicht hält: den vorhandenen
`eintrag_anfuegen` in ein eigenes `NSMenu` setzen und dieses über
`popUpMenuPositioningItem:atLocation:inView:` an derselben Ankerfläche
aufklappen — ein Menübauer und eine Hülle blieben es damit.
Datensatz: `issues/260812-1529_o_der-freigabedialog-wird-nicht-aus-einem-mausdruck-heraus-geoeffnet.md`.

**M2 — Eine Ablagedatei mit ungültigem UTF-8 wird nicht zur Seite gelegt.**
`crates/krk-core/src/ablage/mod.rs:323-340`. `fs::read_to_string` scheitert
nicht nur an einem Zugriffsfehler, sondern auch mit
`io::ErrorKind::InvalidData`, wenn die Bytes kein gültiges UTF-8 sind. Beides
fällt in `Grund::NichtLesbar` und trägt `Beiseite::Nicht`, begründet mit „von
einer Datei, die sich nicht lesen liess, gibt es keinen Inhalt zu sichern".
Für `InvalidData` ist dieser Satz falsch: die Datei steht da, ist vollständig
und trägt die Arbeit des Nutzers. Danach greift genau der Schaden, gegen den
C3 gerichtet ist — der nächste gewöhnliche Schreibvorgang überschreibt sie, und
für `bookmarks.toml` und `session.toml` ist das jedes Beenden. Der Weg dorthin
ist nicht ausgedacht: `keymap.toml` und `settings.toml` sind von Hand änderbar,
und ein Editor, der einen Umlaut in Latin-1 sichert, erzeugt genau diese Lage.
Die Probe zu C3.5 prüft den Zweig mit einem **Ordner** an der Stelle der Datei,
also mit dem Fall, für den die Begründung stimmt.
Datensatz: `issues/260812-1529_o_eine-ablagedatei-mit-ungueltigem-utf-8-wird-nicht-zur-seite-gelegt.md`.

### Niedrig

**N1 — Die Besitzregel des Freigabewählers gilt nur in einer der zwei Hüllen.**
`teilen.rs:115-136` gegen `:253-262`. `anbieten` hält den
`NSSharingServicePicker` in einem `thread_local!` fest und begründet das
ausführlich; `eintrag_anfuegen` baut in derselben Datei einen zweiten und lässt
ihn nach dem Semikolon fallen, ohne ein Wort dazu. `NSMenuItem.target` ist
ausdrücklich schwach (`NSMenuItem.h:93`), `representedObject` stark (`:98`),
und welches von beiden `standardShareMenuItem` benutzt, sagt Apple nicht.
`inference:` Wahrscheinlich unbedenklich, weil das von Apple gezeigte Muster
den Wähler ebenfalls lokal hält. Der Befund ist nicht, dass eine der Stellen
falsch wäre, sondern dass dieselbe Frage zweimal entgegengesetzt beantwortet
ist und nur eine Antwort eine Begründung trägt. Zwei Nebenbefunde stehen im
Datensatz: die Reihenfolge „erst zeigen, dann festhalten" kauft nicht, was ihr
Kommentar ihr zuschreibt, und freigegeben wird der Wähler nie.
Datensatz: `issues/260812-1529_o_die-besitzregel-des-freigabewaehlers-gilt-nur-in-einer-der-zwei-huellen.md`.

**N2 — Die Blattregel sieht den Freigabedialog nicht.** `blatt_steht`
(`crates/krk-ui/src/appkit/anwendung.rs:2063`) fragt `NSWindow::attachedSheet`,
und der Freigabedialog ist kein Blatt. Die Runde hat damit zum ersten Mal einen
stehenden Systemdialog, den die eine Sperre dieses Programms nicht sieht, und
kein Planschritt und kein Datensatz der Runde stellt die Frage, was in dieser
Zeit mit einem Tastendruck geschehen soll. `inference:` Vermutlich nicht
erreichbar, weil `showRelativeToRect:` eine Verfolgungsschleife fährt; ob ein
lokaler `NSEvent`-Beobachter darin gerufen wird, ist eine Zusage von AppKit,
die niemand gelesen hat. Am Bündel mit einer einzigen Beobachtung zu klären:
Dialog über `shift+cmd+s` öffnen und `cmd+w` drücken.
Datensatz: `issues/260812-1529_o_die-blattregel-sieht-den-freigabedialog-nicht.md`.

**N3 — Zwei Doc-Kommentare nennen Zahlen, die der Baum nicht trägt.**
`anwendung.rs:2357` nennt den Ordnersprung den „dritten Aufrufer" von
`ordner_lesen`; es sind zehn, und vor der Runde waren es neun.
`angezeigtedatei.rs:73-79` rechnet aus zwei Wahrheitswerten und zwei Wahlwerten
„acht Lagen"; es sind sechzehn, nach Abzug der ausgeschlossenen zwölf, und
geprüft sind acht. Beide Sätze stammen aus dem Plan und sind mit den Schritten
3 in den Baum gewandert. Der Schaden ist nicht die Zahl, sondern was sie
verspricht: wer `ordner_lesen` umbaut und dem Satz glaubt, prüft drei Stellen
und übersieht sieben.
Datensatz: `issues/260812-1529_o_zwei-doc-kommentare-der-runde-nennen-zahlen-die-der-baum-nicht-traegt.md`.

## Was quer liegt

**Alle fünf Befunde teilen eine Eigenschaft: keiner ist von einer Probe zu
finden.** Zwei hängen an einem Satz im Kopf des Systems, den niemand gelesen
hat (M1, N1), einer an einer Lage, die keine Probe herstellt (M2), einer an
einer Frage, die niemand gestellt hat (N2), und einer an einer Zahl in Prosa
(N3). Das ist kein Zufall dieser Runde, sondern die Kehrseite ihres Zuschnitts:
die beiden Hüllen um die Freigabedienste tragen ausdrücklich keine Probe, und
das ist begründet und richtig. Der Preis davon ist, dass die Abnahme an dieser
Stelle Lesen und Augenschein am Bündel heißt, und die Runde hat für den
Augenschein-Teil kein Loch, aber für den Lese-Teil eines: die
Verfügbarkeitszeilen sind Zeile für Zeile am SDK gegengelesen worden, die
Sätze drei Zeilen darüber nicht.

**Zwei der Befunde beschreiben dieselbe Lücke von zwei Seiten.** M2 und der
schon offene Datensatz
`issues/260812-1204_o_eine-semantisch-widerspruechliche-keymap-toml-wird-nicht-zur-seite-gelegt.md`
sind der zweite und der dritte Weg an der Sicherung vorbei; die Sicherung hängt
an einem einzigen Zweig von `Ablage::laden`, und **beide** Nachbarzweige tragen
eine Begründung, die für ihren Regelfall stimmt und für einen Sonderfall nicht.
Wer einen der beiden behebt, sollte den anderen mit ansehen: eine Lösung, die
die Frage „gibt es hier Inhalt zu sichern" einmal richtig stellt, deckt beide.

**N1 und N2 lösen sich in einer Richtung gemeinsam.** Wer den Wähler nach dem
Schließen des Dialogs freigibt — über `NSSharingServicePickerDelegate`, das
AppKit auch beim Abbrechen ruft —, hat zugleich die Auskunft „steht ein
Freigabedialog", die N2 für den größeren der beiden Zuschnitte braucht.

## Reihenfolge

**Vor dem nächsten Abnahmelauf am Bündel** gehören M1 und N2 in die Liste der
Dinge, die der Nutzer dabei ausdrücklich ansieht. Beide sind mit je einem
Handgriff zu beantworten, und beide sind nur so zu beantworten. M1 entscheidet
über C1.1, das erste Kriterium der Runde.

**Vor dem Abschluss der Runde** ist M2 zu beheben oder ausdrücklich als Lage
anzunehmen. Es ist der einzige Befund, an dem ein Nutzer Bestand verlieren
kann, und die Runde ist genau dazu angetreten. Der Aufwand ist kein Einzeiler,
weil beide Wege eine vollständige Fallunterscheidung des Kerns anfassen; das
gehört zu einer Antwort dazu und ist kein Grund, die Frage zu vertagen.

**Aufräumen, wann es passt:** N1 und N3. Keiner hält etwas auf, beide kosten
wenige Zeilen, und beide sind billiger, solange die Runde noch offen ist und
niemand die Sätze für die Wahrheit hält.

**Nicht Gegenstand dieser Durchsicht** und nur der Vollständigkeit halber: die
fünf noch nicht gefahrenen Planschritte 7 bis 11, `resources/default-keymap.toml`
(beim `ontorev`), und die drei bereits offenen Datensätze der Runde, die kein
zweites Mal gemeldet sind — der Weg über `belegung::laden`
(`260812-1204`), die Zählung in `CLAUDE.md` (`shared/260812-1438`) und die
Frage nach der Markierung anderswo beim Rechtsklick (`260812-1516`).
