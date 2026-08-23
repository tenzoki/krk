//! Die Textflaeche des eingebauten Editors: eine `NSTextView` in einer
//! `NSScrollView`, angebunden an das Modell aus [`crate::editormodell`]
//! (C1 bis C6).
//!
//! ```text
//! ┌──────────────────────────────┐
//! │ • lies.md                    │  der Kopf: Dateiname und Abweichungszeichen
//! ├──────────────────────────────┤
//! │ NSScrollView                 │  der fuenfte Bereich der Fensterzeile
//! │  ┌────┬─────────────────────┐│
//! │  │ 12 │ NSTextView          ││  editierbar, ein Textspeicher
//! │  └────┴─────────────────────┘│  links die Nummernspalte aus C10
//! └──────────────────────────────┘
//! ```
//!
//! # Der Kreis, den diese Datei schliesst
//!
//! ```text
//!   F4 ──> datei_oeffnen(Pfad, Herkunft) ──> Editormodell::oeffnen ──┐
//!                                                    │ Arbeitsfaden  │
//!   Einzugstakt (1/60 s) ──> Editormodell::einziehen <───────────────┘
//!            │
//!            ├─ Geoeffnet ──> stand_einsetzen ──> NSTextView
//!            └─ jeder Ausgang ──> melden(Ausgang, Herkunft) ──> Anwendungsdelegierter
//!                                                 │ Zurueckgehalten: Blatt
//!   zurueckgehaltenes_uebernehmen  <───────────────┤ (sichern / verwerfen)
//!   zurueckgehaltenes_fallenlassen <───────────────┘ (abbrechen)
//!
//!   opt+cmd+e ──> Blatt ──> schliessen ──> stand_einsetzen, kopf_nachziehen
//!   cmd+e (im Editor) ─┘ derselbe Weg, und die Vorschau kommt danach zurueck
//!
//!   Tippen ──> textDidChange: ──> Editormodell::bearbeiten ──> kopf_nachziehen
//!                                          │ gewandelt
//!                                          └> flaeche_richten ──> NSTextView
//!
//!   cmd+s ──> sichern ──> Editormodell::sichern ──┬─ gelungen ─> kopf_nachziehen
//!                                                 └─ jeder Ausgang ─> nach oben
//! ```
//!
//! **Die Herkunft reist in der Kette und liegt nicht daneben.**
//! [`Oeffnungsherkunft`] ist ein Pflichtargument von
//! [`Editorbereich::datei_oeffnen`] und kommt mit jedem Ausgang durch den
//! [`Ausgangsmelder`] zurueck. Wer den Editor eine Datei aufnehmen laesst, sagt
//! damit, wer sie verlangt hat — ein Aufruf, der es nicht sagt, uebersetzt
//! nicht, und zwar von jeder Stelle des Programms aus und nicht nur aus dem
//! Anwendungsdelegierten. Bis zum 260810 stand die Angabe als Feld beim
//! Delegierten und war allein dort erzwungen; die beiden Datensaetze dazu sind
//! `issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`
//! und
//! `issues/260810-1028_*_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`.
//!
//! **Der untere Pfeil ist der Rueckweg, und ohne ihn ist das Modell blind.**
//! Bis S26 hatte [`Editormodell::bearbeiten`] keinen Aufrufer: das Getippte
//! stand allein in der `NSTextView`, `hat_ungesicherten_stand` blieb `false`,
//! und ein Sichern schriebe den Plattenstand zurueck und meldete Erfolg
//! (`issues/260809-2148_*_s25-sichern-schriebe-den-plattenstand-weil-die-rueckschreibung-erst-s26-baut.md`).
//! `textDidChange:` ist die eine Stelle, die AppKit dafuer vorsieht.
//!
//! **Und diese eine Stelle deckt Rueckgaengig und Wiederherstellen nur auf
//! TextKit 1.** Gemessen am 260810 auf macOS 15.7.7 (Build 24G720), dreimal
//! reproduziert: eine `NSTextView` auf TextKit 2 aendert bei einem `undo` ihren
//! Text, **ohne** `textDidChange:` zu verschicken; auf TextKit 1 verschickt sie
//! genau eines, mit dem schon zurueckgenommenen Text. Einziger Unterschied ist der
//! Zugriff auf `layoutManager`, und der laesst AppKit auf den aelteren
//! `NSLayoutManager` zurueckfallen. KRKs Flaeche ist deshalb TextKit 1, und das
//! ist seit dem 260810-1243 **Absicht mit ihrem Grund**: [`textflaeche_bauen`]
//! fasst den Verwalter eigens an, und
//! [`die_gebaute_flaeche_steht_auf_textkit_1`](tests::die_gebaute_flaeche_steht_auf_textkit_1)
//! haelt den Bau an, wenn jemand den Rueckfall wegnimmt. Vorher entstand er als
//! Nebenwirkung von `textmerkmale::zuruecksetzen` und von
//! [`super::nummernspalte`], die den Verwalter aus einem anderen Grund
//! anfassen, und ein Nachziehen der
//! Nummernspalte auf `NSTextLayoutManager` haette bei gruenem Bau und gruenen
//! Proben ein `cmd+s` hinterlassen, das den zurueckgenommenen Text sichert
//! (`issues/260810-1243_*_dass-ein-cmd-z-ueberhaupt-im-modell-ankommt-haengt-an-textkit-1-und-das-steht-nirgends-als-tragend.md`).
//! **Dass `undo` auf TextKit 1 ein `textDidChange:` verschickt, ist gemessen und
//! nicht von Apple zugesagt**; was hier verlangt ist, ist die benannte
//! Abhaengigkeit und nicht ihre Beseitigung.
//!
//! **`setString:` loest den Rueckweg nicht aus.** Eine `NSTextView` meldet ihrem
//! Delegierten allein die Aenderungen des Nutzers; ein programmatisch gesetzter
//! Text laeuft an `didChangeText` vorbei. Darauf ruht, dass eine frisch
//! geoeffnete Datei nicht sofort als geaendert gilt — sichtbar wird ein Bruch
//! dieser Annahme sofort, naemlich als Abweichungszeichen am Kopf einer eben
//! geoeffneten Datei.
//!
//! **Die Nummernspalte ist nicht hier gebaut, sondern eingehaengt.**
//! [`super::nummernspalte`] haelt sie, und die Vorschau haengt dieselbe Klasse
//! ein; C10 sagt eine Anzeige fuer beide Flaechen zu und nicht zwei aehnliche.
//! Im Editor steht sie immer: der Spec laesst sie nicht abschalten.
//!
//! **Was hier steht und was im Modell.** Welche Datei der Editor haelt, ihr
//! Stand, ob er von der Datei abweicht, die Ansichtswahl, der Dateityp und der
//! Suchlauf wohnen in [`Editormodell`] und damit ausserhalb von `appkit/`.
//! Diese Datei setzt den gehaltenen Stand in die Textflaeche um und rechnet
//! ihn nicht nach. Derselbe Schnitt wie [`super::vorschau`] neben
//! [`crate::vorschaumodell`] und [`super::tabelle`] neben [`crate::tabs`].
//!
//! **Die Textflaeche geht als Objekt nach aussen, nicht als Klasse.**
//! [`Editorbereich::textflaeche`] ist die eine Zugriffsfunktion darauf, und sie
//! beantwortet die Naemlichkeitsfrage des Fokusvorbehalts: der Ereignisabgriff
//! reicht jeden Tastendruck an AppKit weiter, sobald der Ersthelfer eine
//! `NSTextView` **ist**, und diese eine ist die Ausnahme davon. Gefragt wird
//! nach der Naemlichkeit und nicht nach der Art, denn der Feldeditor eines
//! Textfeldes ist ebenfalls eine `NSTextView`, und eine Frage nach der Art
//! trennte die beiden nicht. Der Vergleich selbst steht beim
//! Anwendungsdelegierten, der die Flaeche haelt; `super::ereignisse` kennt den
//! Editor nicht.
//!
//! **Ein Textspeicher und kein zweiter Textbestand.** Die `NSTextView` bringt
//! ihren `NSTextStorage`, ihren `NSLayoutManager` und ihren `NSTextContainer`
//! selbst mit; KRK baut keinen davon von Hand und haelt daneben keine zweite
//! Zeichenkette. Der Stand steht im Modell, die Darstellung in der Flaeche, und
//! [`Editorbereich::stand_einsetzen`] ist die eine Stelle, die den Text der
//! Flaeche ersetzt — und damit auch die eine, die den Rueckgaengigverlauf
//! regelt. Beides gehoert zusammen: `setString:` schreibt an der
//! Rueckgaengigverwaltung vorbei (gemessen, siehe [`Verlauf`]), und ein
//! stehengebliebener Stapel zeigte danach auf einen Text, den die Flaeche nicht
//! mehr traegt. Beim Dateiwechsel war das sogar der Text einer **anderen**
//! Datei.
//!
//! **Was danach im Stapel steht, sagt der Anlass und nicht die Schreibstelle.**
//! [`Verlauf`] ist der Wert, in dem der Aufrufer es sagt, und die Aufzaehlung
//! seiner drei Anlaesse steht dort. Ein Dateiwechsel laesst den Verlauf fallen,
//! ein Ersetzen aus S37 traegt ihn als eine Handlung weiter, und das Nachrichten
//! der Flaeche nach einem eingefuegten `\r\n` traegt genau eine Handlung und
//! laesst alles davor fallen — warum es nicht mehr sein kann, steht an
//! [`Editorbereich::flaeche_richten`], und es ist eine Eigenschaft der Sache und
//! nicht der Sorgfalt.
//!
//! **Eine Handlung im Stapel haelt den geaenderten Bereich und nicht die Datei.**
//! Der Stapel eines `NSUndoManager` hat keine Tiefengrenze; eine Handlung, die den
//! ganzen Stand abschreibt, macht den gehaltenen Speicher deshalb zum Produkt aus
//! Dateigroesse und Zahl der Handlungen. [`Umkehrpunkt`] traegt statt dessen
//! Stelle, entfernte Zeichen und Zahl der eingefuegten Bytes; die Zahlen und der
//! Grund gegen `setLevelsOfUndo` stehen dort.
//!
//! **Und der Stapel als Ganzes traegt ein Budget in Bytes.** Ein einzelner
//! Bereich ist klein, solange die geaenderten Stellen beieinander liegen; deckt er
//! die ganze Datei, hilft die Darstellung nicht mehr, und ein wiederholter Befehl
//! legt je Ruf eine Dateigroesse ab
//! (`issues/260810-1314_*_ein-wiederholtes-sammelersetzen-legt-je-ruf-einen-bereich-in-dateigroesse-in-den-stapel.md`).
//! [`STAPELBUDGET`] deckelt die Summe, [`Stapellast`] zaehlt sie mit, und
//! ueberschritten wird sie nicht: der Umbau, der darueber hinausginge, geht als
//! [`Verlauf::TraegtNurDiese`] durch die eine Schreibstelle und steht danach
//! allein im Stapel. **Das Tippen ist davon nicht beruehrt**, und der Grund ist
//! nicht Sorgfalt, sondern der Zaehler: er zaehlt allein die Handlungen, die
//! [`Editorbereich::umkehrung_anmelden`] anmeldet, und das Tippen meldet dort
//! keine an.
//!
//! **Ein Rueckgaengig bildet den Suchlauf neu.** `Editormodell::bearbeiten`
//! beendet ihn, weil die Byteversaetze der Treffer nach einer Aenderung ungueltig
//! sind; nach einem zurueckgenommenen Ersetzen ist der Text aber der von vorher,
//! also ist die Trefferliste ausrechenbar. [`Editorbereich::umkehren`] rechnet sie
//! ueber denselben Weg nach, den `cmd+f` geht, und der Nutzer bezahlt ein `cmd+z`
//! nicht mehr mit seiner laufenden Suche
//! (`issues/260810-1244_*_ein-cmd-z-nach-einem-ersetzen-loescht-den-suchlauf-den-das-ersetzen-eigens-aufgebaut-hat.md`).
//!
//! **Der Kopf ist die zweite Anzeige neben der Statuszeile, und er ist eine
//! andere Art von Aussage.** Die Statuszeile traegt Antworten auf Befehle; der
//! Kopf traegt einen Zustand, naemlich welche Datei der Editor haelt und ob ihr
//! Stand von der Platte abweicht. Das zweite Abnahmekriterium von C4 verlangt
//! ausdruecklich, dass der Nutzer den ungesicherten Stand **ohne** Hinsehen auf
//! die Statuszeile bemerkt; eine Meldung dort waere die falsche Form, weil sie
//! mit dem naechsten Befehl verschwaende. Eine zweite Meldeflaeche entsteht
//! damit nicht: der Kopf beantwortet keine Frage und meldet kein Ereignis.
//!
//! **Was der Editor zu melden hat, geht als Wert nach oben und nicht als
//! fertige Zeile an eine Flaeche.** [`Editormeldung`] benennt es; wohin es
//! geht, weiss diese Datei nicht. Der Anwendungsdelegierte nimmt den Wert und
//! stellt ihn in die **eine** Meldeflaeche des Fensters aus C1 der Runde 1, auf
//! den obersten ihrer sechs Raenge. Eine zweite Meldeflaeche neben ihr entsteht
//! nicht: die Uebergabe an diese Runde sagt das zu, und C1 wiederholt es unter
//! "Der Editor bekommt keine eigene Meldezeile".
//!
//! **Reiner Text.** `setRichText(false)` und die neun abgeschalteten
//! Einstellungen halten fest, was der Nutzer tippt: eine Zeichenkette, die beim
//! Sichern Zeichen fuer Zeichen wieder in der Datei steht. Eine typografische
//! Ersetzung von Anfuehrungszeichen oder Bindestrichen aendert Programmtext
//! still, und die Zusage aus C4 lautet, dass der gesicherte Stand der getippte
//! ist.
//!
//! **Sieben davon sind Automatiken, die achte und die neunte sind die
//! Schreibwerkzeuge** — der Unterschied ist, wer sie auslaest, und der Abschnitt
//! „Die Schreibwerkzeuge aus macOS 15" weiter unten fuehrt ihn aus.
//!
//! **Die sieben zerfallen in drei Gruppen, und jede folgende war ueber der
//! vorigen uebersehen.** Vier greifen beim **Tippen**: Anfuehrungszeichen,
//! Bindestriche, Textersetzung, Rechtschreibkorrektur. Die fuenfte,
//! `smartInsertDeleteEnabled`, greift beim **Einfuegen und Ausschneiden** — sie
//! setzt ein Leerzeichen vor oder hinter ein eingefuegtes Wort und nimmt beim
//! Ausschneiden ein ueberzaehliges fort; sie blieb an, weil die Aufzaehlung
//! nach den vier tippenden aufhoerte
//! (`issues/260809-1650_*_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`).
//! Die sechste und die siebte, `inlinePredictionType` (macOS 14) und
//! `mathExpressionCompletionType` (macOS 15), blieben an, weil die Aufzaehlung
//! **und die Probe darunter** nach der Namensform `set…Enabled:` fragten und
//! diese beiden `set…Type:` heissen
//! (`issues/260810-0416_*_zwei-weitere-textveraendernde-automatiken-stehen-an-und-die-probe-sieht-sie-nicht.md`).
//! In Prosa ist jede der sieben gemeint; in Programmtext ist jede eine
//! Aenderung, die niemand getippt hat. Die Vorgabewerte sind **gemessen** an
//! der Flaeche, die [`textflaeche_bauen`] liefert, nicht der Dokumentation
//! entnommen.
//!
//! **Zu einer Einstellung fuehren mehrere Tueren, und es sind drei Sorten.**
//! Die Vererbungskette von `NSTextView` traegt sechsunddreissig Selektoren der
//! sechs Namensformen aus `FORMEN` — auf macOS 15.7.7, dem Geraet, auf dem
//! gemessen wurde; jede Zahl in diesem Abschnitt ist von dort. `EINSTELLUNGEN`
//! unter `mod tests` ordnet jeden einzeln ein, und die Probe darunter haelt fest,
//! dass keiner fehlt. Drei Sorten Tuer stehen darunter:
//!
//! - **Die zehn Paare.** Zehn der `set…Type:` haben je einen `set…Enabled:`
//!   daneben, und die beiden legen einander um: `automaticQuoteSubstitutionEnabled`
//!   auf `NO` setzt `smartQuotesType` auf `No`, und umgekehrt. Gemessen, je Paar
//!   einzeln und in beiden Richtungen, von der Probe
//!   `jede_zweite_tuer_und_ihre_erste_legen_einander_um` an einer eigens
//!   gebauten Flaeche — nicht mehr von einem Programm, das der Baum nicht
//!   fuehrt (`issues/260810-0748_*_die-kopplung-der-zehn-paare-traegt-den-commit-und-ist-im-baum-durch-nichts-gehalten.md`).
//! - **Die eine Sammeltuer.** `setEnabledTextCheckingTypes:` ist eine
//!   Bitmaske ueber mehrere Automatiken auf einmal. Setzt man sie an KRKs
//!   Flaeche auf den Werkswert zurueck, kommen **vier** der sieben
//!   abgeschalteten wieder an — Anfuehrungszeichen, Bindestriche,
//!   Textersetzung, Rechtschreibkorrektur — und die Grammatikpruefung geht
//!   dabei aus. Deshalb steht sie in `EINSTELLUNGEN` als eigene Antwort
//!   `SammeltuerZu` und nicht als eine der zehn Paare
//!   (`issues/260810-0746_*_es-gibt-eine-dritte-tuer-und-sie-liegt-ausserhalb-aller-drei-namensformen.md`).
//!   **Gesetzt wird sie nicht**: sie waere eine zweite Stelle mit einer Meinung
//!   darueber, was abgeschaltet ist, und die einzelnen Zeilen sind die erste.
//! - **Die Tuer ohne Zwilling.** Ohne jede zweite Tuer stehen die sechste und
//!   die siebte oben sowie die vier Schreibwerkzeug-Einstellungen weiter unten.
//!   Bei den letzteren ist auch das **gemessen** und nicht angenommen:
//!   `setWritingToolsBehavior(None)` laesst die drei uebrigen unberuehrt stehen,
//!   sie legen einander also nicht um.
//!
//! **Zwei Tueren zu einer Einstellung sind nicht derselbe Speicher**, und die
//! schwaechere Aussage ist die gemessene: jede legt die andere um, und die
//! erste kann `Default` weder herstellen noch anzeigen. `NSTextInputTraitType`
//! hat drei Werte, der Wahrheitswert zwei. Wer die zweite Tuer auf `Default`
//! stellt und den Wahrheitswert der ersten liest, bekommt eine Systemvorgabe,
//! die je Einstellung anders ausfaellt — an acht Paaren `YES`, an
//! `linkDetectionType` und `dataDetectionType` `NO`; und wer diesen gelesenen
//! Wahrheitswert unveraendert zurueckschreibt, steht danach auf `Yes` oder `No`
//! und nie wieder auf `Default`. Beides misst
//! `die_erste_tuer_kann_default_weder_herstellen_noch_anzeigen`
//! (`issues/260810-0750_*_derselbe-speicher-ist-eine-stufe-staerker-als-die-messung-hergibt.md`).
//! Fuer den Schluss, dass zehn Paare keine zehn eigenen Zeilen brauchen,
//! genuegt die schwaechere Aussage: `NO` an der ersten Tuer nagelt die zweite
//! auf `No` fest, und das ist an allen zehn gemessen.
//!
//! **Dass es bei neun bleibt, haelt ein Stolperdraht aus zwei Quellen fest,
//! und nur eine der beiden ist geschlossen.**
//! `keine_unbekannte_einstellung_steht_an_der_textflaeche` zaehlt zur Laufzeit
//! auf, was diese Fassung von macOS traegt, und verlangt, dass jeder Fund in
//! `EINSTELLUNGEN` unter `mod tests` eine Antwort hat. Die Aufzaehlung kommt aus
//! zwei Quellen, die einander die Luecken deckeln:
//!
//! - **Das Protokoll `NSTextInputTraits` — der sachliche Schnitt, und er
//!   braucht keine Namensform.** Wer Mitglied dieses Protokolls ist, ist eine
//!   Texteingabe-Einstellung, gleich wie der Selektor endet. Vierzehn
//!   Pflichtmerkmale fuehrt es auf diesem Geraet, und `protocol_copyPropertyList`
//!   liefert sie vollstaendig. Diese Quelle ist der Grund, aus dem
//!   `allowedWritingToolsResultOptions` nicht mehr durchfaellt
//!   (`issues/260810-0745_*_der-stolperdraht-sieht-drei-der-vier-schreibwerkzeug-einstellungen-nicht.md`).
//!   Sie laeuft ueber `objc2::ffi` und damit ueber rohes FFI; das ist in diesem
//!   Teilbaum zulaessig, denn `super`s `mod.rs` traegt die eine Ausnahme von
//!   `#![deny(unsafe_code)]` und Lint-Regeln schlagen in die eingebetteten
//!   Module durch. Die Gegenbehauptung in der Nachricht zu `d9fc2c8` ist falsch
//!   und mit ihr der Schluss, der Schnitt sei unerreichbar
//!   (`issues/260810-0749_*_die-begruendung-unsafe-verbiete-den-sachlichen-schnitt-ist-falsch.md`).
//! - **Die sechs Namensformen ueber der ganzen Vererbungskette — die
//!   Heuristik.** Sie faengt die sechzehn, die `NSTextView` neben den vierzehn
//!   des Protokolls fuehrt: zwoelf `set…Enabled:`, die Sammeltuer, die
//!   Inhaltsart und die beiden Schreibwerkzeug-Einstellungen, die das Protokoll
//!   nicht kennt. Vierzehn und sechzehn sind die dreissig, die an `NSTextView`
//!   selbst stehen; sechs weitere bringt die Kette. Die Kette
//!   laeuft von `NSTextView` bis `NSObject` und nicht nur ueber die Klasse
//!   selbst: `class_copyMethodList` liefert die ererbten Methoden **nicht**, und
//!   `NSView` und `NSResponder` tragen zusammen sechs Selektoren dieser Formen
//!   (`issues/260810-0751_*_die-aufzaehlung-sieht-nur-die-klasse-selbst-und-nicht-ihre-oberklassen.md`).
//!
//! Drei Grenzen bleiben, und keine davon ist zu schliessen:
//!
//! - **Die Proben messen das Geraet, auf dem sie laufen, und nicht das
//!   Zielsystem.** Das Buendel zielt auf macOS 15 und wird bis macOS 26
//!   unterstuetzt; die Aufzaehlung kommt aus der Laufzeit von `cargo test`. Eine
//!   Einstellung, die Apple in macOS 26 dazulegt, faellt erst dem auf, der auf
//!   macOS 26 prueft. Zur Uebersetzungszeit ist das nicht zu erzwingen: Rust
//!   sieht die Kopfdateien des SDK nicht, und `objc2` bildet keine
//!   Verfuegbarkeitsgrenze ab, sondern schaltet die beiden neuen Setzer ueber
//!   ein Cargo-Merkmal.
//! - **Die Namensform ist nicht der Schnitt, den die Sache verlangt** — fuer die
//!   zweite Quelle. "Alles, was den Textspeicher anfassen kann" ist an einem
//!   Selektornamen nicht entscheidbar, und die zehn Paare oben zeigen es von der
//!   anderen Seite: da sind zwei Namen eine Sache. Sechs Formen sind ein
//!   breiterer Stolperdraht als drei und kein Vollstaendigkeitsbeweis. Die erste
//!   Quelle hat diese Grenze nicht; sie hat statt ihrer die eigene, dass
//!   `protocol_copyPropertyList` die Pflichtmerkmale liefert und ein
//!   nachtraeglich als `@optional` erklaertes Merkmal nicht. Genau darin deckeln
//!   sich die beiden: was aus dem Protokoll faellt, faengt die Kette, solange
//!   die Form bekannt ist, und umgekehrt.
//! - **Nur eine Richtung haelt den Bau an.** Was die Laufzeit traegt und
//!   `EINSTELLUNGEN` nicht kennt, ist der gefaehrliche Fall und wird eine
//!   Zusicherung, die die Namen nennt. Was `EINSTELLUNGEN` kennt und die
//!   Laufzeit nicht mehr traegt, ist der harmlose — eine Einstellung, die es
//!   nicht gibt, aendert keine Zeichen — und wird ein Hinweis. Eine gruene Reihe
//!   auf einem unterstuetzten System faerbt er nicht rot
//!   (`issues/260810-0417_*_die-laufzeitprobe-bindet-den-bau-an-die-macos-version-des-pruefenden-geraets.md`).
//!   Der Hinweis geht **nicht** ueber `eprintln!`: `libtest` faengt die
//!   Standardausgabe eines Tests ab und gibt sie nur bei einem Fehlschlag oder
//!   unter `--nocapture` aus, und dieser Zweig laeuft genau dann, wenn der Test
//!   nicht fehlschlaegt. Er geht deshalb ueber [`std::io::stderr`] unmittelbar
//!   an den Fehlerkanal des Prozesses, an dem die Abfangvorrichtung nicht
//!   haengt — gemessen, nicht der Dokumentation entnommen
//!   (`issues/260810-0747_*_der-hinweis-der-gegenrichtung-wird-von-libtest-verschluckt-und-erreicht-niemanden.md`).
//!
//! **Und die neun Zeilen selbst haelt eine Probe.** Die Zeilen stehen seit der
//! Runde 9 in [`super::textautomatik::automatiken_abschalten`], das
//! [`textflaeche_bauen`] und die Flaeche des Zettels rufen; die Probe baut beide
//! Flaechen, liest jede der neun zurueck und vergleicht sie mit einer frisch
//! gebauten `NSTextView`: an KRKs Flaechen steht jede aus, an der frischen jede
//! anders. Was daran Nutzerarbeit bleibt, ist die Wirkung im
//! laufenden Buendel — dass getippte Anfuehrungszeichen als getippte in der
//! Datei stehen —, nicht mehr die Frage, ob die Zeilen stehen und greifen.
//!
//! # Die Schreibwerkzeuge aus macOS 15
//!
//! **Sie sind ausgeschlossen, und das war eine Lesart und keine Codefrage.** Sie
//! schreiben markierten Text um, und das Korrekturlesen wirkt ueber eine ganze
//! Datei; danach steht in `NSTextView::string` nicht mehr das Getippte, und ueber
//! `Editormodell::stand` geht es beim Sichern in die Datei. Von den sieben
//! Automatiken unterscheiden sie sich **in der Art und nicht im Grad**: die sieben
//! greifen ohne Zutun, die Schreibwerkzeuge auf einen ausdruecklichen Aufruf aus
//! dem Kontextmenue. Genau dieser Unterschied entschied die Frage nicht, sondern
//! stellte sie.
//!
//! Entschieden hat sie der Nutzer am 260810 gegen die Schreibwerkzeuge
//! (`decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`): ein
//! Editor fuer Code und Text darf Text nicht stillschweigend umschreiben lassen,
//! und der Gegenstand entscheidet — ein Umschreiben von Programmtext in
//! fluessigere Prosa ist in keiner Lesart von C4 gemeint. Die Faehigkeit ist damit
//! nicht verloren; sie steht in jedem anderen Textfeld des Systems.
//!
//! **Es sind vier Einstellungen und nicht eine**
//! (`issues/260810-0745_*_der-stolperdraht-sieht-drei-der-vier-schreibwerkzeug-einstellungen-nicht.md`),
//! und sie zerfallen in zwei Paare. Alle vier Werkswerte sind an der Flaeche aus
//! [`textflaeche_bauen`] **gemessen** und nicht der Dokumentation entnommen.
//!
//! - **Zwei tragen einen Aus-Wert und je eine Zeile.** `writingToolsBehavior`
//!   steht ab Werk auf `Default` und ueberlaesst dem System die Wahl; `None`
//!   (`-1`, nicht `1`) ist die Absage. `allowsWritingToolsAffordance` — das
//!   Sinnbild, mit dem sich die Werkzeuge selbst anbieten — steht ab Werk **an**.
//!   Sie geht ueber [`super::textautomatik::setzen_falls_vorhanden`] und nicht
//!   ueber einen Aufruf, weil das SDK sie erst ab macOS 15.4 und nur an
//!   `NSTextField` fuehrt.
//! - **Zwei tragen keinen und bekommen keine Zeile.**
//!   `allowedWritingToolsResultOptions` und `writingToolsAllowedInputOptions` sind
//!   Bitmasken, deren Null `…ResultDefault` heisst — "das System waehlt" — und
//!   nicht "nichts". Einen Wert, der nichts zulaesst, fuehrt die Aufzaehlung
//!   nicht, und beide stehen ab Werk schon auf Null. Eine Zeile waere ein Aufruf
//!   ohne Wirkung. In `EINSTELLUNGEN` stehen sie deshalb als
//!   `Gegenstandslos` — sie beschreiben, **was** eine Faehigkeit duerfte, die
//!   nicht laeuft. Dass sie dabei keine zweiten Tueren sind, ist gemessen: das
//!   Abschalten des Verhaltens laesst beide unberuehrt.
//!
//! **Die Formatansicht aus C3 widerspricht dem nicht, und der Grund ist nicht,
//! wo ihre Merkmale liegen.** Sie setzt Farbe und Unterstreichung als
//! voruebergehende Merkmale des Layoutverwalters und die Markdown-Auszeichnung
//! als Merkmale des Textspeichers; warum sie geteilt werden muss, steht im
//! Modulkopf von [`crate::hervorhebung`]. In die Datei geraet weder das eine
//! noch das andere, weil der Sicherungsweg
//! [`Editormodell::stand`](crate::editormodell::Editormodell::stand) schreibt
//! und der aus `NSTextView::string` kommt — den **Zeichen** der Flaeche. Kein
//! Merkmal wird auf diesem Weg auch nur gelesen.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSScrollView`, `NSTextView`, `NSTextContainer`,
//! `NSTextField`, `NSFont`, `NSColor` und `NSTimer` stehen seit macOS 10.0 zur
//! Verfuegung, seit C1 der Runde 6 ebenso `NSMenu` und `NSEvent`, die der
//! Menuehaken entgegennimmt. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`). Keine von ihnen ist nach macOS 15 hinzugekommen,
//! und deshalb braucht keine der Beruehrungen in dieser Datei eine
//! Verfuegbarkeitspruefung zur Laufzeit. `NSFont` und `NSColor` fehlten in
//! dieser Aufzaehlung, obwohl Kopf und Grundschrift sie ansprechen; beide sind
//! am SDK nachgelesen und tragen dort keine Angabe (`NSFont.h:24`,
//! `NSColor.h:77`).
//!
//! **`NSLayoutManager` steht nicht in dieser Aufzaehlung, sondern seit macOS
//! 10.7** (`NSLayoutManager.h:65`, am SDK gelesen). Bis zum 260812 fuehrte
//! dieser Kopf ihn ohne eigene Angabe und damit als 10.0; der Datensatz dazu
//! ist `shared/issues/260812-1558_*_zwei-modulkoepfe-nennen-fuer-nslayoutmanager-macos-10-0-das-sdk-sagt-10-7.md`.
//! Die Zahl ist folgenlos fuer das Buendel und trotzdem zu berichtigen: die
//! Angabe ist die einzige Gegenmassnahme dieses Projekts gegen den Absturz, den
//! `objc2` nicht abfaengt, und eine falsche wird geglaubt.
//!
//! **`NSTextStorage` und `NSMutableParagraphStyle` fasst diese Datei seit dem
//! Umzug der Merkmalsumsetzung nicht mehr an, und seit dem 260812 ebenso wenig
//! `NSAppearance` samt den beiden Erscheinungsnamen**; ihre Angaben stehen im
//! Kopf von [`super::textmerkmale`], wo seither auch die Wahl der Farbtafel
//! wohnt. Den Layoutverwalter fragt diese Datei weiter, an zwei
//! Stellen: dem Zugriff in [`textflaeche_bauen`], der den Rueckfall auf
//! TextKit 1 herstellt, und der Probe, die diesen Rueckfall festhaelt. Die
//! Meldung ueber den Wechsel des Erscheinungsbildes nimmt sie ebenfalls weiter
//! entgegen: `viewDidChangeEffectiveAppearance` steht seit macOS 10.14
//! (`NSView.h:378`).
//!
//! **Eine Beruehrung an einem Protokoll ist juenger als ihre Klasse und liegt
//! weit unter dem Zielsystem**: die Delegiertenmethode
//! `textView:menu:forEvent:atIndex:` steht seit macOS 10.5
//! (`NSTextView.h:628`). Der Rest des Menueweges — `NSMenu` selbst und das
//! Zurueckgeben eines Menues — traegt im Kopf des Systems keine Angabe und
//! steht damit seit 10.0.
//!
//! Fuenf **Methoden** sind juenger als ihre Klasse, und **vier von ihnen liegen
//! auf oder unter dem Zielsystem** und brauchen deshalb keine Pruefung:
//! `setInlinePredictionType:` steht seit macOS 14,
//! `setMathExpressionCompletionType:` und `setWritingToolsBehavior:` seit macOS
//! 15, und `NSTextView.textLayoutManager` seit macOS 12 — die letzte fragt allein
//! die Probe, die den Rueckfall auf TextKit 1 festhaelt, und
//! `NSTextLayoutManager` selbst wird nirgends benannt.
//!
//! **Die fuenfte liegt darueber und ist die einzige gehuetete Beruehrung dieser
//! Datei.** `setAllowsWritingToolsAffordance:` fuehrt das SDK erst ab macOS 15.4
//! und nur an `NSTextField`; die Laufzeit von 15.7.7 antwortet an `NSTextView`
//! darauf, aber undokumentiert. Sie geht deshalb ueber
//! [`super::textautomatik::setzen_falls_vorhanden`], das `respondsToSelector:`
//! **vorher** fragt. Wer
//! eine Methode aus macOS 15.1 oder spaeter anfasst, nimmt diesen einen Weg und
//! baut keine Versionsabfrage daneben.
//!
//! **Die Proben unter `mod tests` sprechen daneben nichts an, was eine
//! Verfuegbarkeitsfrage stellt.** Sie fragen die Laufzeit nach Namen: die Klasse
//! ueber `AnyClass::get`, das Protokoll `NSTextInputTraits` ueber
//! `AnyProtocol::get`, die Werte ueber `valueForKey:` und `setValue:forKey:` aus
//! `NSObject` (macOS 10.0). Ein Name, den diese Fassung von macOS nicht fuehrt,
//! ist deshalb kein Absturz, sondern ein Fund der Probe — und darin liegt ihr
//! Zweck. Eine Zahl fuer die Untergrenze von `NSTextInputTraits` steht hier
//! bewusst nicht: sie wird nirgends gebunden, sondern nachgefragt.

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::rc::Rc;

use block2::RcBlock;
use objc2::rc::{Retained, Weak};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSEvent, NSFont, NSMenu, NSScrollView, NSTextAlignment,
    NSTextDelegate, NSTextField, NSTextView, NSTextViewDelegate, NSView,
};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect,
    NSRunLoop, NSRunLoopCommonModes, NSSize, NSString, NSTimeInterval, NSTimer, NSUInteger,
    NSUndoManager, ns_string,
};

#[cfg(test)]
use objc2_foundation::{NSDate, NSDefaultRunLoopMode};

use krk_core::text::{
    Abweisung, Fund, Markensprung, Treffer, Zeilenindex, Zeilenlage, datei, marke,
};

use crate::editormodell::{Ansicht, Editormodell, Ladeausgang, Sicherungsausgang, Suchlauf};
use crate::hervorhebung::{
    Abholung, Darstellungsart, Einfaerbungsstand, Einfaerbungsvorgang, Formatierung, Tafel,
};

use super::koordinaten;
use super::nummernspalte::{self, Nummernspalte};
use super::statuszeile;
use super::teilen;
use super::textautomatik;
use super::textmerkmale;

/// Was der Editor dem Nutzer zu sagen hat (C1, C2, C6).
///
/// **Ein Wert und keine Zeichenkette am Meldeort.** Jede Meldung des Editors
/// ist die Antwort auf einen Tastenbefehl, und jede geht denselben einen Weg
/// nach oben; der Wortlaut steht deshalb hier an einer Stelle und nicht bei den
/// sechs Befehlen, die ihn ausloesen. Wer eine siebte Meldung braucht, setzt
/// eine Variante dazu und bekommt vom Uebersetzer die fehlende Zeile in
/// [`Self::text`] angezeigt.
///
/// **Die Aufzaehlung ist vollstaendig und hat keinen Auffangzweig**, wie die
/// drei uebrigen dieser Art im Programm. Sie ist heute kurz, weil erst zwei der
/// sechs Ausloeser gebaut sind; die vier uebrigen kommen mit ihren Schritten und
/// tragen ihre Variante bei:
///
/// ```text
///  gebaut    Abweisung beim Oeffnen        krk_core::text::datei::oeffnen (S10)
///  gebaut    Markenstelle geaendert        krk_core::text::marke (S12)
///  gebaut    gelungenes Sichern            krk_core::text::datei::sichern (S9)
///  gebaut    gescheitertes Sichern         dieselbe Stelle (S25)
///  gebaut    Zeilennummer ausserhalb       krk_core::text::zeilen (S35)
///  gebaut    Stand der Suche               crate::editormodell::Suchlauf (S36)
///  gebaut    Zahl der ersetzten Treffer    krk_core::text::suche (S37)
/// ```
///
/// **Das gelungene Sichern meldet sich, obwohl der Kopf es schon zeigt.** Die
/// beiden sagen Verschiedenes: der Kopf traegt den Zustand, naemlich dass nichts
/// mehr abweicht, und die Statuszeile die Antwort auf den Tastendruck, naemlich
/// dass eben geschrieben wurde. Wer `cmd+s` an einer unveraenderten Datei
/// drueckt, sieht am Kopf nichts geschehen und braucht trotzdem eine Antwort;
/// kommentarlos nichts zu tun ist in keinem Fall zulaessig.
///
/// **Kommentarlos nichts zu tun ist in keinem Fall zulaessig**; das steht so im
/// zehnten Abnahmekriterium von C2 und im achten von C6, und dieser Wert ist
/// die Form, in der ein Befehl seinen Grund abgibt.
///
/// **Seit S39 hat jeder Wert seinen Ausloeser, und keine Ausnahme steht mehr an
/// dieser Aufzaehlung.** S22 brachte den ersten: F4 weist eine Datei ab und gibt
/// den Grund ueber [`Self::Abgewiesen`] nach oben. Die beiden Zeilen
/// `#[allow(dead_code)]`, die danach noch an [`Self::MarkenstelleGeaendert`] und
/// [`Self::markenstelle`] standen, sind mit dem Sprung auf eine Textmarke
/// gefallen; ihr Ausloeser war nie F4, und die Ankuendigung aus S21, S22 loese
/// beide ab, war fuer die Haelfte richtig.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Editormeldung {
    /// Der Editor nimmt die Datei nicht an (C2).
    ///
    /// Die drei Gruende bleiben unterschieden, weil das zehnte
    /// Abnahmekriterium von C2 es verlangt und weil der Datensatz
    /// `decisions/260807-2147_*_welche-dateien-oeffnet-der-editor-ueberhaupt.md`
    /// es ausdruecklich fordert. Unterschieden werden sie in
    /// [`Abweisung::meldung`] und hier nicht ein zweites Mal.
    Abgewiesen(Abweisung),
    /// Die Textmarke ist gesprungen, aber ihre gemerkte Stelle war fort (C6).
    ///
    /// Der gemerkte Zeileninhalt stand weder auf der gemerkten Nummer noch in
    /// den `krk_core::text::marke::NAHFENSTER` Zeilen darum. Die Marke fuehrt
    /// **trotzdem** an die gemerkte Nummer; gemeldet wird, dass die Stelle
    /// sich geaendert hat, statt kommentarlos irgendwohin zu fuehren.
    ///
    /// **Ein Wert fuer beide Auskuenfte des Sprungs**, und das ist die Antwort
    /// auf `issues/260809-1631_*_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`.
    /// Die Begruendung steht an [`Self::markenstelle`].
    MarkenstelleGeaendert {
        /// Die gemerkte Zeilennummer, ab 1 gezaehlt, an die die Marke gefuehrt
        /// hat.
        zeile: u32,
        /// Wo jene Nummer im heutigen Text liegt.
        ///
        /// Sie sagt, **wohin** die Schreibmarke gekommen ist: auf die Zeile
        /// selbst, an den Dateianfang oder an das Dateiende. Der Satz nennt
        /// beides in einem Zug.
        lage: Zeilenlage,
    },
    /// Der Stand steht in der Datei (C4).
    Gesichert {
        /// Die geschriebene Datei. Der volle Pfad, wie bei jeder anderen
        /// Meldung ueber eine Datei; der Kopf des Editorbereichs nennt daneben
        /// den blossen Namen.
        pfad: PathBuf,
    },
    /// Es wurde nicht geschrieben, und der Stand des Editors steht unveraendert
    /// da (C4).
    ///
    /// **Der Satz kommt fertig aus dem Modell** und wird hier nicht ein zweites
    /// Mal gebaut: [`crate::editormodell::Sicherungsausgang::Gescheitert`]
    /// traegt ihn, weil dort entschieden wird, woran es lag — am Schreiben
    /// selbst oder an einer Datei, die sich von aussen geaendert hat.
    SichernGescheitert {
        /// Der Grund, wie das Modell ihn formuliert hat.
        grund: String,
    },
    /// Was im Blatt des Zeilensprungs stand, ist keine Zeilennummer (C5).
    ///
    /// Der Sprung unterbleibt dann, und die Schreibmarke bleibt stehen. Die
    /// leere Eingabe kommt hier **nicht** an: sie ist die Abwesenheit einer
    /// Eingabe und wird wie ein Abbruch behandelt, wie bei der Pfadeingabe aus
    /// C2 der Runde 1.
    KeineZeilennummer {
        /// Was der Nutzer geschrieben hat, ohne umschliessende Leerzeichen.
        eingabe: String,
    },
    /// Die gewuenschte Zeilennummer war 0 (C5).
    ///
    /// Zeilennummern zaehlen ab 1, und die 0 ist deshalb keine Zeile. Der
    /// Sprung fuehrt trotzdem irgendwohin, naemlich an den Textanfang; die
    /// Regel steht in `krk_core::text::zeilen` und wird hier nicht nachgebaut.
    ZeileVorDerErsten,
    /// Die gewuenschte Zeilennummer lag ueber der Zeilenzahl (C5).
    ///
    /// Der Sprung fuehrt an das Dateiende, und C5 verlangt, dass der Grund
    /// gemeldet wird, statt kommentarlos nichts zu tun.
    ZeileHinterDerLetzten {
        /// Wie viele Zeilen die Datei hat, die leere letzte mitgezaehlt.
        zeilenzahl: usize,
    },
    /// Wie viele Treffer die Datei enthaelt und der wievielte angesteuert ist
    /// (C5).
    ///
    /// **Der Satz kommt fertig aus dem Modell**, wie bei
    /// [`Self::SichernGescheitert`]: `crate::editormodell::Suchlauf::meldung`
    /// baut ihn, weil dort steht, wie viele Treffer es gibt und welcher gerade
    /// ansteht. Er traegt zugleich die erfolglose Suche, die C5 ebenfalls
    /// gemeldet haben will; ein zweiter Wert dafuer waere eine zweite Stelle
    /// mit einer Meinung darueber, was ein Treffer ist.
    Suchstand {
        /// Der Satz, wie das Modell ihn formuliert hat.
        satz: String,
    },
    /// Es laeuft keine Suche, an der ein Befehl ansetzen koennte (C5).
    ///
    /// Der Ausgang von `cmd+g`, `ctrl+cmd+g`, `shift+cmd+r` und `ctrl+cmd+r`
    /// ohne ein vorangegangenes `cmd+f`. Kommentarlos nichts zu tun ist in
    /// keinem Fall zulaessig.
    KeineSuche,
    /// So viele Treffer sind in einem Zug ersetzt worden (C5).
    Ersetzt {
        /// Die Zahl der ersetzten Treffer; 0, wenn keiner gefunden wurde.
        zahl: usize,
    },
}

impl Editormeldung {
    /// Die Meldung des Markensprungs, falls er eine hat (C6).
    ///
    /// **Die Fallunterscheidung ueber den Fund steht hier und nicht beim
    /// Aufrufer.** Ein getroffener und ein verschobener Sprung melden nichts,
    /// weil beide an der richtigen Stelle landen; allein der dritte Fall
    /// meldet. Ein vierter Fund haelt den Bau an und erzwingt die Antwort auf
    /// die Frage, ob er zu melden ist.
    ///
    /// **Beide Auskuenfte des Sprungs gehen in einen Satz**, und keine
    /// Vorrangregel entscheidet zwischen ihnen. Das ist die Antwort auf
    /// `issues/260809-1631_*_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`,
    /// und sie folgt aus dem Verhaeltnis der beiden: `Markensprung` traegt den
    /// Fund und die Lage der angesteuerten Nummer, und die beiden sind **nicht
    /// unabhaengig**. `krk_core::text::marke::wiederfinden` liefert
    /// [`Fund::Getroffen`] und [`Fund::Verschoben`] nur fuer eine Nummer, deren
    /// Zeile es gibt — `Zeilenindex::inhalt_der_zeile` beantwortet jede andere
    /// mit `None`. Eine von [`Zeilenlage::Getroffen`] verschiedene Lage kommt
    /// deshalb allein mit [`Fund::NichtGefunden`] vor.
    ///
    /// Daraus folgt der Zuschnitt: die erste Auskunft entscheidet **ob**
    /// gemeldet wird, die zweite **wohin** die Schreibmarke gekommen ist. Ein
    /// Vorrang zwischen zwei Saetzen waere falsch, denn er taete so, als koennte
    /// jede der beiden Auskuenfte fuer sich stehen; die zweite tut es nicht.
    /// Ein zusammengesetzter Fall neben den beiden einfachen waere ein dritter
    /// Wert fuer denselben Sachverhalt.
    ///
    /// Die drei Saetze stehen in [`Self::text`]; die Fallunterscheidung ueber
    /// die Lage ist dort vollstaendig und ohne Auffangzweig.
    pub fn markenstelle(sprung: &Markensprung) -> Option<Self> {
        match sprung.fund {
            Fund::Getroffen | Fund::Verschoben => None,
            Fund::NichtGefunden => Some(Self::MarkenstelleGeaendert {
                zeile: sprung.zeile,
                lage: sprung.sprung.lage,
            }),
        }
    }

    /// Der Satz, der dem Nutzer gezeigt wird.
    ///
    /// Vollstaendig und ohne Auffangzweig: eine neue Variante haelt den Bau an
    /// und erzwingt ihren Satz, statt still einen fremden zu bekommen.
    pub fn text(&self) -> String {
        match self {
            Self::Abgewiesen(abweisung) => abweisung.meldung(),
            // Ein Satzanfang fuer alle drei, weil alle drei dasselbe zuerst zu
            // sagen haben; die Lage sagt danach, wohin die Schreibmarke gekommen
            // ist. Vollstaendig und ohne Auffangzweig, wie die Fallunterscheidung
            // im Zeilensprung darunter.
            Self::MarkenstelleGeaendert { zeile, lage } => {
                let wohin = match lage {
                    Zeilenlage::Getroffen => format!("die Marke führt auf Zeile {zeile}"),
                    Zeilenlage::VorDerErsten => {
                        "Zeilen zählen ab 1; die Schreibmarke steht am Dateianfang".to_owned()
                    }
                    Zeilenlage::HinterDerLetzten => format!(
                        "die Datei hat keine Zeile {zeile} mehr; die Schreibmarke steht am Dateiende"
                    ),
                };
                format!("die gemerkte Stelle hat sich geändert; {wohin}")
            }
            Self::Gesichert { pfad } => format!("{} gesichert", pfad.display()),
            Self::SichernGescheitert { grund } => grund.clone(),
            Self::KeineZeilennummer { eingabe } => format!("„{eingabe}“ ist keine Zeilennummer"),
            Self::ZeileVorDerErsten => {
                "Zeilen zählen ab 1; die Schreibmarke steht am Dateianfang".to_owned()
            }
            Self::ZeileHinterDerLetzten { zeilenzahl } => {
                format!("die Datei hat {zeilenzahl} Zeilen; die Schreibmarke steht am Dateiende")
            }
            Self::Suchstand { satz } => satz.clone(),
            Self::KeineSuche => "es läuft keine Suche".to_owned(),
            // Die drei Faelle sind ueberschneidungsfrei und vollstaendig; der
            // Unterschied ist die deutsche Zahlform und nicht die Sache.
            Self::Ersetzt { zahl } => match zahl {
                0 => "kein Treffer ersetzt".to_owned(),
                1 => "ein Treffer ersetzt".to_owned(),
                zahl => format!("{zahl} Treffer ersetzt"),
            },
        }
    }
}

/// Der Stand, den ein `cmd+z` wiederherstellt (C5).
///
/// **Die Zeichen und die Auswahl gehoeren zusammen.** Ein Rueckgaengig, das den
/// Text wiederherstellt und die Schreibmarke am Dateianfang liegen laesst, ist
/// die halbe Handlung: derselbe Grund, aus dem
/// [`Editorbereich::flaeche_richten`] die Schreibmarke mitrechnet, statt sie
/// wandern zu lassen.
///
/// # Der Punkt traegt den geaenderten Bereich und nicht den ganzen Stand
///
/// Bis zum 260810-1241 stand hier der **ganze** Stand vor dem Umbau als
/// `String`, und der Stapel des `NSUndoManager` hat keine Tiefengrenze. Damit
/// war der gehaltene Speicher das Produkt aus Dateigroesse und Zahl der
/// Handlungen: an der Editorgrenze von 16 MB und den hundert Ersetzungen, die C5
/// mit „der wievielte gerade angesteuert ist" selbst anbietet, rund 1,6 GB, frei
/// erst mit dem naechsten Dateiwechsel
/// (`issues/260810-1241_*_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-und-ist-unbegrenzt.md`).
///
/// **Die Abschrift war nicht unvermeidlich, und deshalb steht hier keine
/// Tiefengrenze, sondern eine andere Darstellung.** Ein Umbau des Textes ist der
/// Austausch **eines** Bereichs: `entfernt` an der Stelle `anfang` gegen
/// `eingefuegt` Bytes. Alles davor und alles danach ist in beiden Staenden
/// dasselbe und braucht nicht aufgehoben zu werden. Gemessen an einem Stand von
/// 16 MB mit einer Ersetzung darin:
///
/// ```text
///   Darstellung                 je Handlung      100 Handlungen
///   ganzer Stand (bis 1241)     16 777 219 B     1 677 721 900 B
///   geaenderter Bereich         3 B              300 B
/// ```
///
/// Die Zahlen stehen nicht hier, weil sie hier geglaubt werden sollen, sondern
/// weil
/// [`ein_umkehrpunkt_traegt_den_geaenderten_bereich_und_nicht_den_ganzen_stand`](tests::ein_umkehrpunkt_traegt_den_geaenderten_bereich_und_nicht_den_ganzen_stand)
/// sie an derselben Grenze nachrechnet und den Bau anhaelt, wenn eine spaetere
/// Fassung wieder den ganzen Stand aufhebt.
///
/// **`setLevelsOfUndo` steht bewusst nirgends.** Eine Tiefengrenze gaebe es nur
/// fuer den ganzen Verwalter, und der traegt nach der Anmeldung in
/// [`Editorbereich::umkehrung_anmelden`] die Handlungen der Flaeche mit — also
/// das Tippen. Sie zu begrenzen aendert eine Zusage, die weder C4 noch C5 macht.
/// Und sie faengt den Preis nicht, um den es geht: eine Grenze in **Handlungen**
/// laesst hundert Handlungen von je einer Dateigroesse zu, also genau die
/// 1,6 GB, die `260810-1241` gefunden hat.
///
/// **Was der Bereich nicht deckelt, deckelt das Budget.** Der Bereich ist klein,
/// solange die geaenderten Stellen beieinander liegen; deckt er die ganze Datei —
/// ein Sammelersetzen, dessen Ersatztext den Suchtext enthaelt —, ist er so lang
/// wie sie. Die Summe ueber alle angemeldeten Handlungen deckelt deshalb
/// [`STAPELBUDGET`], gezaehlt von [`Stapellast`] und angewandt in
/// [`Editorbereich::verlauf_fuer_umbau`].
///
/// **Was der Umbau nicht abschafft, ist die voruebergehende Abschrift.** Wer
/// einen Punkt bildet, hat beide Staende gleichzeitig zu halten, und einer von
/// beiden kommt bei drei der vier Anlaesse als Kopie aus dem Modell. Sie faellt
/// am Ende des Blocks, in dem sie entstand, und geht deshalb in keinen Stapel
/// ein; der Preis ist ein `memcpy` je Handlung neben den beiden Durchgaengen, die
/// `krk_core::text::suche` fuer dieselbe Handlung ohnehin faehrt.
struct Umkehrpunkt {
    /// Der Byteversatz **im Stand nach dem Umbau**, ab dem sich die beiden
    /// Staende unterscheiden. Eine Zeichengrenze, siehe
    /// [`gemeinsamer_anfang`].
    anfang: usize,
    /// Die Zeichen, die der Stand **vor** dem Umbau ab [`Self::anfang`] trug. In
    /// gehaltener Form, weil sie aus dem Modell kommen.
    entfernt: String,
    /// Wie viele Bytes der Stand **nach** dem Umbau ab [`Self::anfang`] traegt.
    /// Sie treten fuer [`Self::entfernt`] zurueck.
    eingefuegt: usize,
    /// Die Auswahl der Flaeche vor dem Umbau, in AppKits Koordinate.
    auswahl: NSRange,
}

impl Umkehrpunkt {
    /// Der Punkt, der `nachher` wieder zu `vorher` macht.
    ///
    /// **Ein Bereich und nicht mehrere.** Ein Sammelersetzen aendert viele
    /// Stellen; dieser Punkt fasst sie in **einen** Bereich von der ersten bis
    /// zur letzten zusammen. Das ist mehr als das Notwendige und trotzdem
    /// richtig: die Wiederherstellung ist zeichengleich, und die Zahl der
    /// Bereiche zu fuehren hiesse, die Regeln des Ersetzens hier ein zweites Mal
    /// zu tragen. Was ein Ersetzen geaendert hat, weiss `krk_core::text::suche`
    /// und nicht diese Datei.
    ///
    /// Die beiden Staende werden geliehen und nicht genommen: der Punkt haelt
    /// danach allein den Unterschied.
    fn zwischen(vorher: &str, nachher: &str, auswahl: NSRange) -> Self {
        let anfang = gemeinsamer_anfang(vorher, nachher);
        let schwanz = gemeinsamer_schwanz(vorher, nachher, anfang);
        Self {
            anfang,
            entfernt: vorher[anfang..vorher.len() - schwanz].to_owned(),
            eingefuegt: nachher.len() - schwanz - anfang,
            auswahl,
        }
    }

    /// Der Stand, der aus `stand` entsteht, wenn dieser Punkt darauf wirkt.
    ///
    /// **Der Guertel ist der Schnitt auf eine Zeichengrenze**, und er steht hier
    /// aus demselben Grund wie der in [`Editorbereich::auswahl_setzen`]: die
    /// beiden Versaetze sind gegen genau den Stand gebildet, der hier
    /// hereinkommt, und passen deshalb. Ein Versatz, der es doch nicht taete,
    /// waere in Rust keine falsche Anzeige, sondern eine Panik mitten in der
    /// Ereignisbehandlung. Dass er passt, haelt die Zusicherung fest; dass er
    /// notfalls nicht abstuerzt, der Schnitt.
    fn angewandt_auf(&self, stand: &str) -> String {
        let anfang = bis_zur_zeichengrenze(stand, self.anfang);
        let bis = bis_zur_zeichengrenze(stand, self.anfang + self.eingefuegt);
        debug_assert!(
            anfang == self.anfang && bis == self.anfang + self.eingefuegt,
            "der Umkehrpunkt gehoert zu einem anderen Stand als dem, auf den er wirkt"
        );
        let mut neu = String::with_capacity(stand.len() - (bis - anfang) + self.entfernt.len());
        neu.push_str(&stand[..anfang]);
        neu.push_str(&self.entfernt);
        neu.push_str(&stand[bis..]);
        neu
    }

    /// Wie viele Bytes dieser Punkt im Stapel haelt.
    ///
    /// Die drei `usize` und der `NSRange` daneben sind eine feste Groesse und
    /// haengen nicht an der Datei; gezaehlt wird deshalb, was am Halde haengt.
    ///
    /// **Zwei Leser und dieselbe Zahl.** [`Stapellast`] fuehrt die Summe
    /// gegen [`STAPELBUDGET`], und die Messung aus
    /// `issues/260810-1241_*_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-und-ist-unbegrenzt.md`
    /// liest sie einzeln. Eine zweite Rechnung neben dieser gaebe es nicht
    /// umsonst: die Zahl, die der Zaehler fuehrt, waere dann eine andere als die,
    /// die die Probe nachrechnet.
    fn getragene_bytes(&self) -> usize {
        self.entfernt.len()
    }
}

/// Wie viele Bytes die angemeldeten Rueckgaengig-Handlungen zusammen halten
/// duerfen.
///
/// # Die Zahl ist geliehen und nicht erfunden
///
/// Es ist `krk_core::text::datei::EDITORGRENZE`, also die Dateigrenze des Editors
/// aus C2. Die Begruendung ist ein Verhaeltnis und keine Vorliebe: der Editor
/// nimmt eine Datei bis zu dieser Groesse an und haelt sie danach zweimal, im
/// Modell und in der Textflaeche. Ein Verlauf, der mehr haelt als die groesste
/// Datei, die der Editor ueberhaupt oeffnet, kostet mehr als der Gegenstand, um
/// den es geht. Eine Zahl daneben — 8 MB, 64 MB — waere eine zweite Meinung
/// darueber, was dieser Editor an Text an sich heranlaesst, und die erste steht in
/// `datei`.
///
/// **Was daraus folgt, in Zahlen.** Wie viele Sammelersetzen ueber den **ganzen**
/// Text nebeneinander im Stapel stehen, ist das Budget geteilt durch die
/// Dateigroesse:
///
/// ```text
///   Datei    nebeneinander im Stapel   im Stapel
///   16 MB    eines, dann geraeumt      ≤ 16 MB + eines   gemessen
///    1 MB    16                        ≤ 16 MB + eines   dieselbe Teilung
///  256 kB    64                        ≤ 16 MB + eines   dieselbe Teilung
/// ```
///
/// Gemessen ist die erste Zeile, an der Editorgrenze und mit den Staenden, die
/// `ctrl+cmd+r` herstellt:
/// [`der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung`](tests::der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung).
/// Die beiden darunter sind dieselbe Teilung und keine zweite Messung.
///
/// Die obere Schranke ist damit `STAPELBUDGET` **plus eine Handlung** und nicht
/// `STAPELBUDGET`: die Handlung, die das Budget sprengt, wird nicht abgewiesen,
/// sondern raeumt vor sich auf. Ein Ersetzen, das nicht ruecknehmbar waere, wuerde
/// C5 widersprechen; ein Verlauf, der davor faellt, widerspricht ihm nicht.
///
/// **Das Tippen bleibt unbegrenzt.** Der Zaehler zaehlt allein, was
/// [`Editorbereich::umkehrung_anmelden`] anmeldet, und das sind die vier Anlaesse
/// aus [`Verlauf`] — nicht die Handlungen, die die `NSTextView` fuer jeden
/// Anschlag selbst anmeldet. Deren Tiefe beschraenkt kein Abnahmekriterium, und
/// dieses Budget beschraenkt sie auch nicht.
const STAPELBUDGET: usize = datei::EDITORGRENZE as usize;

/// Beide Mac-Architekturen sind 64-bittig, und die Umrechnung oben ist deshalb
/// verlustfrei. Die Zusicherung faengt den Tag, an dem eine dritte dazukommt,
/// beim Bauen und nicht im Betrieb — dieselbe Form wie
/// `assert!(EDITORGRENZE > 1024 * 1024)` in `krk_core::text::datei`.
const _: () = assert!(STAPELBUDGET as u64 == datei::EDITORGRENZE);

/// Die Bytes, die eine angemeldete Handlung im Stapel haelt, solange sie darin
/// steht.
///
/// # Warum das Zaehlen an der Handlung haengt und nicht am Anmelden
///
/// Ein `NSUndoManager` sagt nicht, wann er eine Handlung fallen laesst, und er
/// laesst sie auf vier Wegen fallen: sie wird ausgefuehrt, der
/// Wiederherstellungsstapel wird von einer neuen Anmeldung geraeumt, `removeAllActions`
/// raeumt beide, oder das Objekt selbst geht fort. Ein Zaehler, der beim Anmelden
/// hochgeht und den Rest raet, waere an allen vier Wegen falsch — und dabei nicht
/// vorsichtig falsch, sondern in der Richtung, in der er dem Nutzer den Verlauf
/// nimmt, den er noch haette haben koennen.
///
/// **Deshalb zaehlt der Wert selbst.** Diese Huelle wohnt im Block, den
/// [`Editorbereich::umkehrung_anmelden`] anmeldet; hebt der Verwalter den Block
/// auf, faellt sie mit ihm, und `Drop` traegt die Bytes ab. Der Zaehler stimmt
/// damit auf jedem der vier Wege, ohne dass einer von ihnen hier genannt werden
/// muesste.
///
/// # Die Freigabe des Blocks ist geschlossen und nicht gemessen
///
/// Dass der Verwalter den Block festhaelt und ihn mit der Handlung wieder
/// freigibt, ist die Regel von Objective-C fuer einen Block, den ein Objekt
/// aufbewahrt; nachgemessen ist sie hier **nicht**. Eine Messung braeuchte einen
/// `NSUndoManager`, also einen `MainThreadMarker`, und darueber steht eine offene
/// Nutzerentscheidung an den vier Proben, die ihn heute behaupten
/// (`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`).
/// Eine fuenfte daneben zu stellen haette die Frage vergroessert, statt sie zu
/// beantworten.
///
/// **Die Schranke haengt an der Annahme nicht.** Traefe sie nicht zu, ginge der
/// Zaehler nur hoch und nie herunter; das Budget griffe dann bei **jedem**
/// Umbau, und der Stapel hielte statt „Budget plus eine Handlung" genau eine
/// Handlung. Was eine falsche Annahme kostet, ist also die Tiefe des Verlaufs und
/// nicht die Schranke. Der Preis stuende in derselben Richtung wie der Fall
/// darunter.
///
/// **Genauigkeit im Augenblick leistet die Huelle ohnehin nicht.** Gibt AppKit den
/// Block an einen Freigabeverbund weiter, faellt sie einen Umlauf der Laufschleife
/// spaeter, und der Zaehler steht bis dahin zu hoch. Die Richtung ist die
/// vorsichtige: geraeumt wird dann eher als noetig, nie spaeter.
/// [`Editorbereich::umkehren`] fragt den Zaehler ohnehin nicht — ein `cmd+z`
/// raeumt keinen Verlauf.
struct Stapellast {
    /// Der Punkt, dessen Bytes gezaehlt sind.
    punkt: Umkehrpunkt,
    /// Der Zaehler des Editorbereichs, geteilt mit allen anderen Handlungen im
    /// Stapel. Er ueberlebt den Editorbereich, weil eine Handlung es kann.
    zaehler: Rc<Cell<usize>>,
}

impl Stapellast {
    /// Traegt die Bytes des Punktes an und uebernimmt ihn.
    fn angemeldet(punkt: Umkehrpunkt, zaehler: &Rc<Cell<usize>>) -> Self {
        zaehler.set(zaehler.get() + punkt.getragene_bytes());
        Self {
            punkt,
            zaehler: Rc::clone(zaehler),
        }
    }
}

impl Drop for Stapellast {
    /// **Der Guertel ist die Saettigung und nicht die Rechnung**, wie an
    /// [`Umkehrpunkt::angewandt_auf`]: abgetragen wird genau, was
    /// [`Self::angemeldet`] an demselben Punkt angetragen hat, und `entfernt`
    /// aendert sich dazwischen nicht. Ein Unterlauf ist damit ausgeschlossen; dass
    /// er es ist, haelt die Zusicherung fest, und dass ein `Drop` notfalls nicht
    /// mitten im Abbau in Panik geraet, die Saettigung.
    fn drop(&mut self) {
        let bytes = self.punkt.getragene_bytes();
        debug_assert!(
            self.zaehler.get() >= bytes,
            "der Zaehler traegt weniger als diese Handlung angemeldet hat"
        );
        self.zaehler.set(self.zaehler.get().saturating_sub(bytes));
    }
}

/// Die Regel des Budgets: passt der Punkt neben `gehalten`, tritt er dazu; passt
/// er nicht, tritt er an die Stelle des ganzen Verlaufs.
///
/// **Sie steht als Funktion und nicht als Methode**, weil sie vom Editorbereich
/// nichts braucht als eine Zahl. Damit ist sie ohne Fenster pruefbar, und
/// [`Editorbereich::verlauf_fuer_umbau`] reicht ihr allein den Zaehler herein —
/// dieselbe Aufteilung wie bei [`kopfzeile`], die entscheidet, was im Kopf steht,
/// ohne den Kopf zu kennen.
///
/// Der Vergleich ist `>` und nicht `>=`: ein Punkt, der das Budget genau
/// ausfuellt, passt hinein.
fn verlauf_fuer_umbau(punkt: Umkehrpunkt, gehalten: usize) -> Verlauf {
    if gehalten + punkt.getragene_bytes() > STAPELBUDGET {
        Verlauf::TraegtNurDiese(punkt)
    } else {
        Verlauf::Traegt(punkt)
    }
}

/// Der erste Byteversatz, an dem sich zwei Texte unterscheiden, auf eine
/// Zeichengrenze abgerundet.
///
/// **Die Grenze wird in beiden Texten verlangt und nicht nur in einem.** Vor der
/// ersten Abweichung sind die Bytes gleich, also ist dort auch die Grenze
/// dieselbe; **an** ihr kann der eine Text eine Zeichengrenze tragen und der
/// andere ein Folgebyte. Ein Schnitt dort waere eine Panik beim Zerschneiden,
/// und die Abrundung endet spaetestens bei 0, weil dort jeder Text eine Grenze
/// hat.
fn gemeinsamer_anfang(vorher: &str, nachher: &str) -> usize {
    let mut anfang = vorher
        .as_bytes()
        .iter()
        .zip(nachher.as_bytes())
        .take_while(|(links, rechts)| links == rechts)
        .count();
    while anfang > 0 && !(vorher.is_char_boundary(anfang) && nachher.is_char_boundary(anfang)) {
        anfang -= 1;
    }
    anfang
}

/// Wie viele Bytes am Ende zweier Texte uebereinstimmen, ohne hinter `ab`
/// zurueckzugreifen und auf eine Zeichengrenze abgerundet.
///
/// `ab` ist der gemeinsame Anfang aus [`gemeinsamer_anfang`]; ohne diese Schranke
/// koennten sich Anfang und Schwanz ueberlappen und der herausgeschnittene
/// Bereich haette eine negative Laenge.
fn gemeinsamer_schwanz(vorher: &str, nachher: &str, ab: usize) -> usize {
    let mut schwanz = vorher.as_bytes()[ab..]
        .iter()
        .rev()
        .zip(nachher.as_bytes()[ab..].iter().rev())
        .take_while(|(links, rechts)| links == rechts)
        .count();
    while schwanz > 0
        && !(vorher.is_char_boundary(vorher.len() - schwanz)
            && nachher.is_char_boundary(nachher.len() - schwanz))
    {
        schwanz -= 1;
    }
    schwanz
}

/// Der naechste Byteversatz bei oder vor `versatz`, an dem `text` ein Zeichen
/// beginnt.
///
/// `is_char_boundary(0)` ist an jedem Text wahr, also endet die Schleife.
fn bis_zur_zeichengrenze(text: &str, versatz: usize) -> usize {
    let mut versatz = versatz.min(text.len());
    while !text.is_char_boundary(versatz) {
        versatz -= 1;
    }
    versatz
}

/// Was aus dem Rueckgaengigverlauf wird, wenn der Text der Flaeche ersetzt wird.
///
/// **`setString:` schreibt an der Rueckgaengigverwaltung vorbei** — gemessen am
/// 260810 auf macOS 15.7.7 (Build 24G720): eine `NSTextView` mit `allowsUndo`
/// meldet nach einem `setString:` keine Handlung an, `canUndo` bleibt `false`.
/// Damit hat jeder Weg, der den Text der Flaeche ersetzt, die Frage zu
/// beantworten, was danach im Stapel steht, und **kann sie nicht offenlassen**:
/// ein stehengebliebener Stapel zeigte auf einen Text, den die Flaeche nicht
/// mehr traegt, und ein `cmd+z` darauf wirkte gegen falsche Stellen
/// (`issues/260809-1727_*_ein-dateiwechsel-laesst-den-rueckgaengigstapel-der-vorigen-datei-stehen.md`).
///
/// **Den Anlass kennt allein der Aufrufer**, und deshalb kommt die Antwort als
/// Wert herein, statt in [`Editorbereich::stand_einsetzen`] geraten zu werden.
/// Das ist die Behebung von
/// `issues/260810-0303_*_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`:
/// bis dahin leerte die eine Schreibstelle den Stapel bei jedem Anlass, weil
/// sie keinen von ihnen unterscheiden konnte.
///
/// ```text
///   Anlass                     Verlauf danach
///   Dateiwechsel, Schliessen ─> Faellt        der Verlauf gehoerte einer
///                                             anderen Datei
///   Ersetzen (S37)           ─> Traegt        der Nutzer nimmt das Ersetzen
///                                             zurueck, und was davor liegt
///                                             bleibt zuruecknehmbar
///   Ersetzen ueber dem       ─> TraegtNurDiese  der Bereich allein ist so gross
///   Stapelbudget                              wie das Budget; was davor liegt
///                                             kann nicht bleiben, siehe
///                                             `STAPELBUDGET`
///   CRLF-Richten             ─> TraegtNurDiese  der Nutzer nimmt das Einfuegen
///                                             zurueck; was davor liegt kann
///                                             nicht bleiben, siehe unten
/// ```
///
/// Die Aufzaehlung ist vollstaendig und hat keinen Auffangzweig, wie die
/// uebrigen dieser Art im Programm: ein fuenfter Anlass haelt den Bau an und
/// erzwingt die Antwort.
///
/// **Vier Anlaesse und drei Antworten, und der Schnitt liegt richtig.** Das
/// Ersetzen steht zweimal darin, weil es zwei verschiedene Fragen beantwortet:
/// dass der Nutzer es zuruecknehmen kann, und was der Verlauf davor kostet. Die
/// erste beantwortet C5, die zweite das Budget, und
/// [`Editorbereich::verlauf_fuer_umbau`] ist die eine Stelle, die zwischen den
/// beiden Antworten waehlt.
///
/// # Warum es drei Antworten sind und nicht zwei
///
/// Bis zum 260810-1044 gab es die dritte nicht, und das CRLF-Richten nahm die
/// erste: ein `cmd+z` unmittelbar nach einem eingefuegten `\r\n` tat nichts
/// (`issues/260810-1044_*_ein-eingefuegtes-crlf-bleibt-nicht-ruecknehmbar-und-der-grund-liegt-am-eingang-der-flaeche.md`).
/// [`Verlauf::Traegt`] konnte es nicht sein, und der Grund steht an
/// [`Editorbereich::flaeche_richten`]: die Handlung, die die Flaeche fuer das
/// Einfuegen selbst angemeldet hat, zeigt auf einen Bereich, der um die Zahl der
/// weggefallenen `\r` zu lang ist. Bliebe sie stehen, loeschte ein zweites
/// `cmd+z` Zeichen hinter dem Eingefuegten mit.
///
/// **Der Stapel laesst kein einzelnes Herausnehmen zu**, und deshalb wird er
/// geleert und die eine gueltige Handlung danach neu angemeldet. Der Preis ist
/// benannt: der Verlauf **vor** dem Einfuegen faellt mit. Er fiel vorher auch,
/// nur ohne Gegenwert; was hinzukommt, ist das Einfuegen selbst.
enum Verlauf {
    /// Der Verlauf faellt: er zeigte auf einen Text, den die Flaeche nach dem
    /// Schreiben nicht mehr traegt.
    Faellt,
    /// Der Verlauf traegt den Umbau als eine Handlung, und der genannte
    /// Umkehrpunkt ist der Stand, den sie wiederherstellt.
    Traegt(Umkehrpunkt),
    /// Der Verlauf faellt, und die genannte Handlung ist danach die einzige, die
    /// darin steht.
    TraegtNurDiese(Umkehrpunkt),
}

/// Die Groesse, mit der die Flaeche entsteht, bevor die Aufteilung sie auslegt.
///
/// Die Breite ist die Anfangsbreite des Bereichs aus
/// [`crate::fenstermodell::Bereich::anfangsbreite`]; sie gilt nur bis zum
/// ersten Auslegen und ist danach ohne Bedeutung.
const AUFBAUGROESSE: NSSize = NSSize::new(460.0, 400.0);

/// Der Takt, in dem der Hauptfaden die Meldung des Arbeitsfadens abholt.
///
/// Dieselbe Zahl wie der Einzugstakt der Vorschau und des Dateifensters, aus
/// demselben Grund: haeufiger zu fragen braechte nichts, weil nicht oefter
/// gezeichnet wird.
const LADETAKT: NSTimeInterval = 1.0 / 60.0;

/// Das Zeichen, das einen ungesicherten Stand am Kopf anzeigt (C4).
///
/// **Vor dem Namen und nicht dahinter.** Der Kopf ist so breit wie der
/// Editorbereich, und der laesst sich bis auf 320 Punkte schmal ziehen; ein
/// langer Dateiname wird dann rechts gekuerzt, und ein Zeichen am Ende ginge
/// mit. Vorn steht es an einer festen Stelle und bleibt in jeder Breite
/// sichtbar.
const ABWEICHUNGSZEICHEN: &str = "•";

define_class!(
    /// Die Ansicht, in der Kopf und Textflaeche haengen — und die Stelle, an
    /// der KRK den Wechsel des Erscheinungsbildes bemerkt (S34).
    ///
    /// **Sie traegt genau eine Aufgabe ueber die einer `NSView` hinaus.**
    /// `viewDidChangeEffectiveAppearance` ist die eine Stelle, die AppKit fuer
    /// die Frage "hat das System auf Dunkel umgestellt" vorsieht, und sie ist
    /// eine Methode einer Ansicht. Der [`Editorbereich`] ist keine Ansicht,
    /// sondern ein `NSObject`, also braucht die Meldung eine Ansicht, die sie
    /// annimmt und weiterreicht.
    ///
    /// Die Rueckverbindung ist **schwach**, sonst schloesse sich der Ring
    /// Editorbereich → Ansicht → Rueckverweis → Editorbereich. Dieselbe Form
    /// wie der Rueckruf der Tableiste in [`super::vorschau`].
    // SAFETY:
    // - Die Oberklasse NSView stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSView)]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Option<Weak<Editorbereich>>>]
    pub struct Editorsicht;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Editorsicht {}

    impl Editorsicht {
        /// Das System hat auf Hell oder Dunkel umgestellt (S34).
        // SAFETY: Die Signatur entspricht der von NSView.
        #[unsafe(method(viewDidChangeEffectiveAppearance))]
        fn erscheinung_gewechselt(&self) {
            // SAFETY: Die Oberklasse beantwortet dieselbe Nachricht ohne
            // Argument und ohne Rueckgabe. Sie zuerst, weil AppKit hinter
            // dieser Methode die Erscheinung der Unteransichten nachzieht und
            // KRK danach eine bereits umgestellte Flaeche vorfindet.
            let _: () = unsafe { msg_send![super(self), viewDidChangeEffectiveAppearance] };
            let editor = self.ivars().borrow().as_ref().and_then(Weak::load);
            if let Some(editor) = editor {
                editor.erscheinung_nachziehen();
            }
        }
    }
);

impl Editorsicht {
    /// Eine Ansicht mit dem genannten Rahmen, noch ohne Rueckverweis.
    fn neu(mtm: MainThreadMarker, rahmen: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(RefCell::new(None));
        // SAFETY: `initWithFrame:` von NSView hat die hier angenommene
        // Signatur.
        unsafe { msg_send![super(this), initWithFrame: rahmen] }
    }

    /// Traegt den Rueckverweis nach, sobald es den Editorbereich gibt.
    fn ziel_setzen(&self, editor: &Editorbereich) {
        *self.ivars().borrow_mut() = Some(Weak::from_retained(&editor.retain()));
    }
}

/// Wer verlangt hat, dass der Editor eine Datei aufnimmt (C2, C7).
///
/// **Zwei Werte, ueberschneidungsfrei und vollstaendig.** Entweder ein Befehl
/// des Nutzers hat geoeffnet, oder die Wiederherstellung der Sitzung beim Start;
/// einen dritten Anlass gibt es nicht, und ein neuer haelt an jeder Stelle den
/// Bau an, die die beiden Werte unterscheidet.
///
/// **Er ist ein Pflichtargument und kein Kennzeichen daneben.** Jedes Oeffnen
/// geht durch [`Editorbereich::datei_oeffnen`], das diesen Wert entgegennimmt,
/// und ein Aufruf, der ihn nicht nennt, uebersetzt nicht — gleich von welcher
/// Stelle des Programms aus. Zwei Datensaetze stehen dahinter, und der zweite
/// hat den ersten zu Ende gebaut:
/// `issues/260810-0418_*_ein-f4-waehrend-der-wiederherstellung-erbt-die-marke-aus-sitzung.md`
/// (die Herkunft lag als `Cell` neben der Kette, wurde allein von der
/// Wiederherstellung gesetzt und erst beim Ladeausgang verbraucht, und ein
/// Oeffnen in dieser Spanne erbte sie) und
/// `issues/260810-1028_*_die-herkunft-eines-oeffnens-ist-im-delegierten-erzwungen-und-nicht-am-editorbereich.md`
/// (die Erzwingung endete an der Grenze des Anwendungsdelegierten).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Oeffnungsherkunft {
    /// Ein Befehl des Nutzers: `f4`, `cmd+e` aus der Dateiliste oder aus der
    /// Vorschau, oder der Sprung auf eine Textmarke aus C6.
    Befehl,
    /// Die Datei, die `session.toml` gemerkt hat, beim Start (C7).
    Sitzung,
}

impl Oeffnungsherkunft {
    /// Ob die Wiederherstellung der Sitzung geoeffnet hat und nicht ein Befehl
    /// des Nutzers.
    ///
    /// **Die eine Stelle, die die beiden Werte unterscheidet**, und sie tut es
    /// mit einer vollstaendigen Fallunterscheidung ohne Auffangzweig: ein dritter
    /// Anlass haelt hier den Bau an, statt still den Zweig des Befehls zu
    /// bekommen. Ein Vergleich mit `==` an der Aufrufstelle taete das nicht, und
    /// die Zusage im Kopf dieser Aufzaehlung haengt daran.
    #[allow(
        clippy::match_like_matches_macro,
        reason = "`matches!` prueft die Vollstaendigkeit nicht, und genau die ist hier der Zweck"
    )]
    pub fn ist_aus_sitzung(self) -> bool {
        match self {
            Oeffnungsherkunft::Sitzung => true,
            Oeffnungsherkunft::Befehl => false,
        }
    }
}

/// Die Senke, an die jeder [`Ladeausgang`] samt seiner [`Oeffnungsherkunft`]
/// geht.
///
/// Ein eigener Name, weil der Typ an drei Stellen steht — Feld, Setzer und
/// Aufrufstelle — und ausgeschrieben an jeder von ihnen dieselbe Zeile waere.
///
/// **Die Herkunft kommt hier zurueck und wird nicht beim Empfaenger gemerkt.**
/// Ein Feld beim Anwendungsdelegierten, das auf seinen Verbrauch am naechsten
/// Ausgang wartet, war genau die Bauart aus `260810-0418`; ein zweites Argument
/// kann kein Ausgang verlieren und kein zweiter ueberschreiben.
pub type Ausgangsmelder = Box<dyn Fn(Ladeausgang, Oeffnungsherkunft)>;

/// Was der Editorbereich haelt.
pub struct EditorIvars {
    /// Die Ansicht, die in die Aufteilung gehaengt wird: Kopf und Bildlauf
    /// darin.
    ///
    /// Eine [`Editorsicht`] und keine blosse `NSView`, weil an ihr die eine
    /// Meldung haengt, mit der AppKit den Wechsel des Erscheinungsbildes
    /// anzeigt (S34).
    bereich: Retained<Editorsicht>,
    /// Der Kopf mit dem Dateinamen und dem Abweichungszeichen (C4).
    kopf: Retained<NSTextField>,
    /// Die Textflaeche selbst, editierbar und mit einem Textspeicher.
    ///
    /// Die Bildlaufansicht um sie herum steht hier **nicht**: sie haengt in
    /// [`Self::bereich`], der sie festhaelt, und niemand hier spricht sie an.
    /// Wer sie braucht — S33, um nach einem Ansichtswechsel die Nummernspalte
    /// neu zeichnen zu lassen —, bekommt sie ueber `enclosingScrollView`.
    text: Retained<NSTextView>,
    /// Der Stand des Editors, ohne AppKit.
    modell: RefCell<Editormodell>,
    /// Der Zeitgeber, der die Meldung des Arbeitsfadens abholt.
    ///
    /// Er haelt das Objekt als Ziel fest, und das Objekt haelt ihn; der Ring
    /// bricht mit `invalidate`, wie beim Einzugstakt der Vorschau.
    takt: RefCell<Option<Retained<NSTimer>>>,
    /// Die Senke, an die jeder [`Ladeausgang`] geht.
    ///
    /// Sie haelt den Anwendungsdelegierten **schwach**; die Begruendung steht
    /// an [`Editorbereich::melder_setzen`]. `None` heisst: der Aufbau ist noch
    /// nicht so weit, und dann gibt es auch niemanden, der etwas anfinge.
    melden: RefCell<Option<Ausgangsmelder>>,
    /// Wer das **zuletzt begonnene** Oeffnen verlangt hat (C2, C7).
    ///
    /// **Warum "das zuletzt begonnene" genau die richtige Auskunft ist.**
    /// Gelesen wird seit S24 auf einem Arbeitsfaden, und zwischen dem Beginn des
    /// Oeffnens und seinem Ausgang laeuft die Ereignisschleife. Hoechstens das
    /// zuletzt begonnene Oeffnen liefert trotzdem einen Ausgang:
    /// [`Editormodell::oeffnen`] ersetzt den laufenden Ladevorgang, sein
    /// Empfaenger faellt, und das `send` des ueberholten Fadens scheitert still.
    /// Geschrieben wird das Feld von **jedem** [`Editorbereich::datei_oeffnen`],
    /// also von jedem Oeffnen, und gelesen von [`Editorbereich::melden`], also
    /// von jedem Ausgang.
    ///
    /// **Auch die zurueckgehaltene Datei aus C4 gehoert zum zuletzt begonnenen
    /// Oeffnen**, und deshalb braucht sie keine zweite Angabe daneben. Solange
    /// das Blatt aus C4 steht, kommt kein weiterer Oeffnungsbefehl durch:
    /// [`crate::kommandos::zulaessigkeit::zulaessig`] laesst bei stehendem Blatt
    /// vier Kommandos durch — `Abbrechen` ueber
    /// [`crate::kommandos::operationen::waehrend_blatt_erlaubt`] und `Beenden`,
    /// `FensterSchliessen` und `FensterEinblenden` ueber
    /// [`crate::kommandos::zulaessigkeit::immer_erreichbar`] —, und keines der
    /// vier oeffnet eine Datei. Ein zweites Feld fuer die zurueckgehaltene
    /// Datei unterschiede damit etwas, das nicht auseinanderlaufen kann.
    ///
    /// Bis zum 260818 stand hier „jedes Kommando ausser dem Abbruch". Der
    /// Schluss hielt und haelt, die Begruendung war zu eng
    /// (`issues/260817-1302_*_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-*.md`).
    ///
    /// **Der Aufbau setzt [`Oeffnungsherkunft::Befehl`], und niemand liest ihn.**
    /// Alle drei Aufrufer von [`Editorbereich::melden`] setzen einen Wert voraus,
    /// den es nur nach einem `datei_oeffnen` gibt: dieses selbst schreibt vor dem
    /// Melden, [`Editorbereich::ladeausgang_einziehen`] laeuft allein zu einem
    /// gestarteten Ladevorgang, und
    /// [`Editorbereich::zurueckgehaltenes_uebernehmen`] kehrt ohne
    /// zurueckgehaltene Datei um.
    herkunft: Cell<Oeffnungsherkunft>,
    /// Das laufende Einfaerben, falls eines laeuft (C3).
    ///
    /// Hoechstens eines. Der Editor haelt hoechstens eine Datei und zeigt
    /// hoechstens eine Ansicht; ein zweiter Lauf daneben faerbte denselben Text
    /// ein zweites Mal ein. Fallengelassen wird der Vorgang beim Wechsel in die
    /// Rohansicht und beim Schliessen: sein Empfaenger faellt mit, und das
    /// `send` des ueberholten Fadens scheitert still.
    einfaerbung: RefCell<Option<Einfaerbungsvorgang>>,
    /// Der aufgehobene Stand des letzten fertigen Einfaerbungslaufs (C3).
    ///
    /// **Er ist die Vorlage, aus der der naechste Lauf fortschreibt**, und ohne
    /// ihn kostete jeder Anschlag einen vollen Durchgang: 0,3 MB/s, gemessen,
    /// also 4,5 s bei 1,5 MB. Wie das Fortschreiben rechnet und was es an
    /// Speicher kostet, steht an
    /// [`crate::hervorhebung::Einfaerbungsstand`].
    ///
    /// **Er wohnt hier und nicht im Vorgang**, weil er den Vorgang ueberleben
    /// muss: der Vorgang endet mit seiner Lieferung, die Vorlage gilt bis zur
    /// naechsten. Waehrend ein Lauf laeuft, steht hier `None` — die Vorlage ist
    /// dann im Arbeitsfaden und kommt mit dem Ergebnis zurueck. Ein zweiter
    /// Halter daneben waere eine zweite Wahrheit darueber, welcher Text
    /// aufgehoben ist.
    ///
    /// Fallengelassen wird sie, wo es nichts mehr fortzuschreiben gibt: beim
    /// Wechsel in die Rohansicht und ohne gehaltene Datei. Ein Wechsel der
    /// Farbtafel oder der Sprache braucht das **nicht** —
    /// [`crate::hervorhebung::fortschreiben`] erkennt es am Schluessel und
    /// rechnet von vorn.
    einfaerbungsstand: RefCell<Option<Einfaerbungsstand>>,
    /// Ob der laufende Lauf ueberholt ist und nach seiner Rueckkehr sofort ein
    /// neuer zu starten ist (C3).
    ///
    /// **Das ist die ganze Zusammenfassung schneller Anfragen.** Wer tippt,
    /// stellt je Anschlag eine Anfrage; laeuft schon eine, wird nicht eine
    /// zweite gestartet, sondern diese Marke gesetzt. Damit lebt zu jedem
    /// Zeitpunkt hoechstens ein Faden, und der letzte Stand wird genau einmal
    /// eingefaerbt, statt jeder Zwischenstand einmal.
    ///
    /// Sie traegt beide Anlaesse: einen geaenderten Text und eine gewechselte
    /// Farbtafel. Beide verlangen dasselbe, naemlich einen neuen Lauf, und eine
    /// zweite Marke daneben unterschiede etwas, das dieselbe Antwort hat.
    einfaerbung_erneut: Cell<bool>,
    /// Welche der beiden Farbtafeln gerade gilt (S34).
    tafel: Cell<Tafel>,
    /// Der Ersatztext, den der Nutzer zuletzt im Blatt aus S36 eingetragen hat
    /// (C5).
    ///
    /// **Er steht hier und nicht im Modell**, und zwar aus demselben Grund, aus
    /// dem der Suchtext dort steht: `crate::editormodell::Suchlauf` haelt, was
    /// die Rechnung braucht, und `krk_core::text::suche` bekommt den Ersatztext
    /// als Parameter herein. Was der Nutzer in ein Eingabefeld geschrieben hat,
    /// ist dagegen eine Angabe der Oberflaeche: sie ueberlebt einen beendeten
    /// Suchlauf, weil das naechste `cmd+f` sie als Startwert wieder anbietet.
    ///
    /// Leer heisst: der Ersatz ist die leere Zeichenkette, also loescht das
    /// Ersetzen den Treffer. Das ist eine gueltige Absicht und kein
    /// Sonderfall; "es wurde noch nichts eingetragen" und "es wurde
    /// ausdruecklich nichts eingetragen" verlangen dieselbe Handlung, und ein
    /// `Option` daneben unterschiede etwas, das dieselbe Antwort hat.
    ersatz: RefCell<String>,
    /// Wie viele Bytes die angemeldeten Rueckgaengig-Handlungen zusammen halten.
    ///
    /// **Ein `Rc` und kein `Cell` fuer sich**, weil die Zaehlung nicht hier
    /// wohnt, sondern in den Handlungen: jede haelt eine [`Stapellast`], die
    /// beim Anmelden antraegt und in ihrem `Drop` abtraegt. Eine Handlung kann
    /// den Editorbereich ueberleben — der Verwalter haelt sie, der Bereich haelt
    /// den Verwalter nicht —, und deshalb gehoert der Zaehler beiden.
    ///
    /// Wer ihn liest, ist [`Editorbereich::verlauf_fuer_umbau`], und was er
    /// daraus macht, steht an [`STAPELBUDGET`]. Geschrieben wird er allein von
    /// [`Stapellast`]; eine zweite schreibende Stelle waere eine zweite Wahrheit
    /// darueber, was im Stapel steht.
    stapelbytes: Rc<Cell<usize>>,
}

define_class!(
    /// Der Editorbereich (C1 bis C6).
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = EditorIvars]
    pub struct Editorbereich;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Editorbereich {}

    // SAFETY: `NSTextDelegate` stellt keine Bedingungen. Die Textflaeche haelt
    // ihren Delegierten schwach ("This is a weak property",
    // `objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1258-1263`), und der
    // Editorbereich haelt die Flaeche stark; ein Ring entsteht deshalb nicht,
    // und der Delegierte lebt so lange wie die Flaeche.
    unsafe impl NSTextDelegate for Editorbereich {
        /// Der Nutzer hat getippt, eingefuegt oder geloescht (C4).
        ///
        /// **Der Rueckweg aus der Flaeche ins Modell**, und die eine Stelle,
        /// die ihn geht; siehe den Modulkopf.
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(textDidChange:))]
        fn text_geaendert(&self, _meldung: &NSNotification) {
            self.text_zurueckschreiben();
        }
    }

    // SAFETY: `NSTextViewDelegate` stellt keine Bedingungen. Er steht hier,
    // weil `NSTextView::setDelegate:` genau diesen Protokolltyp verlangt; die
    // Aenderungsmeldung `textDidChange:` kommt aus dem Obertyp
    // `NSTextDelegate`, der Menuehaken darunter aus diesem Protokoll selbst.
    unsafe impl NSTextViewDelegate for Editorbereich {
        /// Haengt den Teilen-Eintrag in das Kontextmenue der Textflaeche
        /// (C1 der Runde 6, sechstes Kriterium).
        ///
        /// **KRKs Eintrag tritt neben das, was AppKit gibt, und nimmt nichts
        /// weg.** Dieser Haken bekommt das fertig gebaute Menue der
        /// `NSTextView` — Ausschneiden, Kopieren, Rechtschreibung, die
        /// Schreibwerkzeuge — und gibt es **ergaenzt** zurueck. Deshalb geht
        /// der Editor diesen Weg und nicht den ueber `setMenu:`, den die
        /// Dateiliste nimmt: eine Tabelle bringt kein eigenes Menue mit, eine
        /// Textflaeche schon, und ein gesetztes Menue traete an dessen Stelle.
        /// Die beiden Anschlussarten stehen im Kopf von [`super::teilen`]
        /// nebeneinander.
        ///
        /// Geteilt wird die Datei, die der Editor haelt. Sie ist die
        /// angezeigte, denn das Menue geht nur dort auf, wo der Nutzer
        /// hinklickt, und geklickt hat er in den sichtbaren Editor; eine
        /// Abfrage der Sichtbarkeit ueber [`crate::angezeigtedatei::welche`]
        /// beantwortete hier eine Frage, die der Klick schon beantwortet hat.
        /// Haelt der Editor keine Datei, geschieht nichts und das Menue bleibt,
        /// wie AppKit es gebaut hat.
        // SAFETY: Die Signatur entspricht der des Protokolls
        // (`NSTextView.h:628`).
        #[unsafe(method_id(textView:menu:forEvent:atIndex:))]
        fn kontextmenue(
            &self,
            _flaeche: &NSTextView,
            menue: &NSMenu,
            _ereignis: &NSEvent,
            _stelle: NSUInteger,
        ) -> Option<Retained<NSMenu>> {
            let pfade: Vec<PathBuf> = self.pfad().into_iter().collect();
            teilen::eintrag_anfuegen(menue, &pfade, self.mtm());
            Some(menue.retain())
        }
    }

    impl Editorbereich {
        /// Der Rueckruf des Zeitgebers.
        // SAFETY: Die Signatur passt zu der, die NSTimer aufruft.
        #[unsafe(method(ladenEinziehen:))]
        fn laden_einziehen(&self, _zeitgeber: &NSTimer) {
            self.einziehen();
        }
    }
);

impl Editorbereich {
    /// Baut Kopf und Textflaeche mit einem Modell, das noch keine Datei haelt.
    pub fn bauen(mtm: MainThreadMarker) -> Retained<Self> {
        let bereich = Editorsicht::neu(mtm, NSRect::new(NSPoint::ZERO, AUFBAUGROESSE));
        bereich.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        // Der Bildlauf fuellt alles unter dem Kopf und waechst mit; der Kopf
        // klebt oben und waechst nur in der Breite. Dieselbe Aufteilung wie
        // Tableiste und Inhaltsflaeche in `super::vorschau`.
        let (rolle, text) = textflaeche_bauen(
            mtm,
            NSRect::new(
                NSPoint::ZERO,
                NSSize::new(
                    AUFBAUGROESSE.width,
                    AUFBAUGROESSE.height - statuszeile::HOEHE,
                ),
            ),
        );
        bereich.addSubview(&rolle);

        let kopf = kopf_bauen(mtm);
        kopf.setFrame(NSRect::new(
            NSPoint::new(
                statuszeile::EINZUG,
                AUFBAUGROESSE.height - statuszeile::HOEHE,
            ),
            NSSize::new(
                AUFBAUGROESSE.width - statuszeile::EINZUG,
                statuszeile::HOEHE,
            ),
        ));
        bereich.addSubview(&kopf);

        let tafel = textmerkmale::tafel_der_erscheinung(&bereich);
        let this = Self::alloc(mtm).set_ivars(EditorIvars {
            bereich,
            kopf,
            text,
            modell: RefCell::new(Editormodell::neu()),
            takt: RefCell::new(None),
            melden: RefCell::new(None),
            herkunft: Cell::new(Oeffnungsherkunft::Befehl),
            einfaerbung: RefCell::new(None),
            einfaerbungsstand: RefCell::new(None),
            einfaerbung_erneut: Cell::new(false),
            tafel: Cell::new(tafel),
            ersatz: RefCell::new(String::new()),
            stapelbytes: Rc::new(Cell::new(0)),
        });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };

        // Der Rueckweg aus der Flaeche ins Modell (C4). Er steht hier und nicht
        // in `textflaeche_bauen`, weil es das Objekt erst ab dieser Zeile gibt.
        this.ivars()
            .text
            .setDelegate(Some(ProtocolObject::from_ref(&*this)));
        // Derselbe Grund an der Ansicht: der Wechsel des Erscheinungsbildes
        // laeuft ueber sie hierher, und "hierher" gibt es erst ab dieser Zeile.
        this.ivars().bereich.ziel_setzen(&this);

        // Die Flaeche zeigt von der ersten Zeichnung an den Stand des Modells
        // und nicht irgendeinen. Beim Aufbau ist er leer, weil der Editor keine
        // Datei haelt; die Zeile steht trotzdem hier, damit es genau einen Weg
        // vom Modell in die Flaeche gibt und keinen Anfangszustand daneben. Der
        // Kopf und die Ansicht folgen derselben Regel.
        //
        // `Faellt` ist hier ohne Wirkung und trotzdem die richtige Antwort: die
        // Flaeche haengt noch in keinem Fenster, hat also keinen Verwalter, und
        // es gibt keinen Umbau, den ein `cmd+z` zuruecknehmen koennte.
        this.stand_einsetzen(Verlauf::Faellt);
        this.kopf_nachziehen();
        this.darstellung_nachziehen();
        this
    }

    /// Die Ansicht, die in die Aufteilung gehaengt wird.
    ///
    /// Der ganze Bereich mit Kopf und Bildlauf, nicht die Bildlaufansicht
    /// allein: die Fokusabfrage aus S43 fragt nach dem Enthaltensein in dieser
    /// Ansicht, und die Textflaeche liegt darin.
    pub fn sicht(&self) -> &NSView {
        &self.ivars().bereich
    }

    /// Traegt die Senke ein, die jeden [`Ladeausgang`] bekommt.
    ///
    /// Gerufen vom Aufbau der Oberflaeche, mit einem Rueckruf, der den
    /// Anwendungsdelegierten **schwach** haelt: sonst schloesse sich der Ring
    /// Delegierter → Editorbereich → Rueckruf → Delegierter. Derselbe Zuschnitt
    /// wie `Hauptfenster::melder_setzen` und die uebrigen Melder dieses
    /// Projekts.
    ///
    /// **Warum der Ausgang ueberhaupt einen Rueckweg braucht.** Seit S24 liest
    /// der Editor auf einem Arbeitsfaden; wann eine Datei steht oder abgewiesen
    /// ist, weiss der Befehl, der sie angefordert hat, zu seiner eigenen Zeit
    /// nicht mehr. Der eine Ausgangstyp geht deshalb nicht mehr als Rueckgabe
    /// an den Aufrufer, sondern durch diese Senke — und zwar **jeder** Ausgang,
    /// auch der sofort feststehende [`Ladeausgang::SchonOffen`], damit es eine
    /// Behandlung gibt und nicht zwei.
    pub fn melder_setzen(&self, melden: Ausgangsmelder) {
        *self.ivars().melden.borrow_mut() = Some(melden);
    }

    /// Die Textflaeche, an der die Naemlichkeitsfrage des Fokusvorbehalts
    /// haengt.
    ///
    /// Sie geht allein zum Vergleichen nach aussen: der Anwendungsdelegierte
    /// haelt sie gegen den Ersthelfer des Fensters, so wie er es fuer die Liste
    /// der Leiste und die Inhaltsflaeche der Vorschau seit der Runde 1 tut.
    /// Wer sie zum Schreiben braucht, geht ueber das Modell und
    /// [`Self::stand_einsetzen`].
    pub fn textflaeche(&self) -> &NSTextView {
        &self.ivars().text
    }

    /// Ob der Editor eine Datei haelt (C1, C2).
    ///
    /// Der Fokusbefehl aus C1 fragt danach: einen ausgeblendeten Editor ohne
    /// Datei holt er nicht hervor. Die Frage geht an das Modell und wird hier
    /// nicht aus der Textflaeche beantwortet — ein leerer Text ist keine
    /// fehlende Datei.
    pub fn haelt_datei(&self) -> bool {
        self.ivars().modell.borrow().haelt_datei()
    }

    /// Ob der Editor Aenderungen haelt, die nicht in der Datei stehen (C4).
    ///
    /// Die Frage der drei Anlaesse aus C4: der Anwendungsdelegierte stellt sie,
    /// bevor er einen Anlass ausfuehrt, der den Stand verloere. Sie geht an das
    /// Modell und wird hier nicht aus der Textflaeche beantwortet.
    pub fn hat_ungesicherten_stand(&self) -> bool {
        self.ivars().modell.borrow().hat_ungesicherten_stand()
    }

    /// Die Datei, die der Editor haelt, falls er eine haelt (C11).
    ///
    /// Der Fenstertitel fragt danach: steht der Fokus im Editor, zeigt der
    /// Titel den vollen Pfad dieser Datei, auch dann, wenn das aktive
    /// Dateifenster einen anderen Ordner zeigt. Die Frage geht wie
    /// [`Self::haelt_datei`] an das Modell und wird hier nicht ein zweites Mal
    /// beantwortet.
    ///
    /// Der Pfad wird abgeschrieben und nicht ausgeliehen: die Ausleihe des
    /// Modells endet mit dieser Zeile, und der Aufrufer traegt den Wert durch
    /// AppKit-Aufrufe, die hierher zuruecklaufen koennen.
    pub fn pfad(&self) -> Option<PathBuf> {
        self.ivars().modell.borrow().pfad().map(Path::to_path_buf)
    }

    /// Der Satz ueber eine fremde Aenderung, einmal je Aenderung (C4).
    ///
    /// **Verglichen wird im Modell und hier nicht ein zweites Mal.** Diese
    /// Funktion reicht die Frage hinein und den Satz heraus; wann er kommt und
    /// warum nur einmal, steht an [`Editormodell::fremdaenderung_melden`].
    ///
    /// Die Flaeche wird dabei nicht angefasst. Was der Nutzer getippt hat,
    /// bleibt stehen: C4 sagt zu, ihn zu **unterrichten**, und nicht, ihm seinen
    /// Stand wegzunehmen.
    pub fn fremdaenderung_melden(&self) -> Option<String> {
        self.ivars().modell.borrow_mut().fremdaenderung_melden()
    }

    /// Nimmt die genannte Datei auf und zeigt ihren Stand (C2).
    ///
    /// **Der eine Weg, auf dem eine Datei in den Editor kommt.** Beide
    /// Einstiege aus C2 gehen ueber ihn und legen damit dieselbe Pruefung an,
    /// wie es das neunte Abnahmekriterium von C2 verlangt; der Sprung auf eine
    /// Textmarke aus C6 kommt spaeter dazu.
    ///
    /// **Sie kehrt sofort zurueck und nennt keinen Ausgang.** Gelesen und
    /// geprueft wird auf dem Arbeitsfaden des Modells, und die Antwort holt
    /// [`Self::einziehen`] ab; wer wissen will, wie es ausgegangen ist, haengt
    /// sich ueber [`Self::melder_setzen`] ein. Steht der Ausgang schon fest,
    /// weil der Editor die Datei bereits haelt, geht er durch dieselbe Senke,
    /// nur eben sofort.
    ///
    /// **Was der Nutzer davon sieht:** F4 auf eine grosse Datei blendet den
    /// Editor nicht sogleich ein, sondern erst, wenn sie gelesen ist. Das ist
    /// die Reihenfolge, die das elfte Abnahmekriterium von C2 verlangt — erst
    /// die Pruefung, dann die Flaeche —, und sie bleibt mit dem Arbeitsfaden
    /// erhalten, weil auch die Pruefung dort laeuft. Der Gegenwert steht in
    /// S24: waehrend des Lesens bleiben die beiden Dateifenster bedienbar.
    ///
    /// Entschieden wird nichts hier: die Pruefung steht in
    /// `krk_core::text::datei::oeffnen` und ist ueber [`Editormodell::oeffnen`]
    /// erreichbar. Bei [`Ladeausgang::Abgewiesen`] bleibt der bisherige Stand
    /// vollstaendig stehen, und der Grund geht als Wert nach oben; wohin er
    /// dort kommt, weiss diese Datei nicht (siehe den Modulkopf).
    ///
    /// **Die Herkunft ist Pflicht und kommt mit dem Ausgang zurueck.** Wer
    /// oeffnet, sagt, ob ein Befehl des Nutzers oder die Wiederherstellung der
    /// Sitzung es verlangt hat; der Wert geht unveraendert durch den
    /// [`Ausgangsmelder`] an den Empfaenger des Ausgangs. Damit gibt es keinen
    /// Weg in den Editor, der die Angabe schuldig bleibt — auch keinen von
    /// ausserhalb des Anwendungsdelegierten, und das ist der Unterschied zum
    /// Stand vom 260810-1028. Wo sie bis zum Ausgang liegt und warum das keine
    /// Marke neben der Kette ist, steht an [`EditorIvars::herkunft`].
    pub fn datei_oeffnen(&self, pfad: &Path, herkunft: Oeffnungsherkunft) {
        self.ivars().herkunft.set(herkunft);
        let sofort = self.ivars().modell.borrow_mut().oeffnen(pfad);
        match sofort {
            Some(ausgang) => self.melden(ausgang),
            None => self.takt_starten(),
        }
    }

    /// Schreibt den gehaltenen Stand in die Datei (C4).
    ///
    /// **Geschrieben wird im Modell und hier nicht ein zweites Mal.** Diese
    /// Funktion reicht den Befehl hinein und den Ausgang heraus; die
    /// Sicherungsform, die Stempelpruefung und der atomare Schreibweg stehen in
    /// [`Editormodell::sichern`] und darunter in `krk_core::text::datei`.
    ///
    /// **Was sie beitraegt, ist der Kopf.** Nach einem gelungenen Sichern
    /// meldet das Modell keine Abweichung mehr, und ohne diesen Ruf truege der
    /// Kopf sein Zeichen weiter, obwohl nichts mehr abweicht. Nach einem
    /// gescheiterten bleibt der Kopf, wie er ist, weil auch die Abweichung
    /// bleibt.
    ///
    /// **Der Stand kommt nicht aus der Textflaeche.** Er steht im Modell, weil
    /// `textDidChange:` ihn bei jeder Aenderung dorthin zurueckschreibt (siehe
    /// den Modulkopf). Ihn hier ein zweites Mal aus der Flaeche zu holen waere
    /// der zweite Rueckweg, und der eine bestehende waere damit nicht mehr die
    /// Wahrheit ueber den Stand des Editors.
    ///
    /// Die Ausleihe des Modells endet vor dem Ruf an den Kopf, wie ueberall in
    /// dieser Datei.
    pub fn sichern(&self) -> Sicherungsausgang {
        let ausgang = self.ivars().modell.borrow_mut().sichern();
        if matches!(ausgang, Sicherungsausgang::Gesichert(_)) {
            self.kopf_nachziehen();
        }
        ausgang
    }

    /// Nimmt die zurueckgehaltene Datei jetzt auf (C4).
    ///
    /// Der Weg zurueck aus der Nachfrage, wenn der Nutzer mit "sichern" oder
    /// "verwerfen" geantwortet hat. Was danach zu tun ist, ist genau das, was
    /// [`Self::einziehen`] fuer [`Ladeausgang::Geoeffnet`] tut, und deshalb
    /// steht es hier in derselben Form: Stand in die Flaeche, Kopf nachziehen,
    /// Ausgang durch dieselbe Senke. Eine zweite Behandlung desselben Wertes
    /// entsteht damit nicht — der Anwendungsdelegierte sieht `Geoeffnet` und
    /// holt Fokus und Titel nach, ohne diesen Weg vom gewoehnlichen zu
    /// unterscheiden.
    ///
    /// Wartete nichts, geschieht nichts. Der Fall ist im Ablauf nicht
    /// erreichbar, weil allein der Rueckruf der Nachfrage hierher fuehrt;
    /// stillschweigend nichts zu tun ist trotzdem richtig, denn es gibt keine
    /// Datei, ueber die etwas zu melden waere.
    pub fn zurueckgehaltenes_uebernehmen(&self) {
        let ausgang = self
            .ivars()
            .modell
            .borrow_mut()
            .zurueckgehaltenes_uebernehmen();
        let Some(ausgang) = ausgang else {
            return;
        };
        if ausgang == Ladeausgang::Geoeffnet {
            // Ein Dateiwechsel: der Verlauf gehoerte der vorigen Datei.
            self.stand_erneuern(Verlauf::Faellt);
        }
        self.melden(ausgang);
    }

    /// Laesst die zurueckgehaltene Datei fallen (C4).
    ///
    /// Der Weg zurueck aus der Nachfrage, wenn der Nutzer abgebrochen hat oder
    /// das Sichern gescheitert ist. Die Flaeche wird dabei nicht angefasst: sie
    /// traegt unveraendert den Stand, den der Nutzer behalten wollte.
    pub fn zurueckgehaltenes_fallenlassen(&self) {
        self.ivars()
            .modell
            .borrow_mut()
            .zurueckgehaltenes_fallenlassen();
    }

    /// Gibt die gehaltene Datei auf und leert die Flaeche (C1, C4).
    ///
    /// Gerufen, wenn der Editor geschlossen wird — nach der Nachfrage aus C4,
    /// die dem Anwendungsdelegierten gehoert. Der Stand faellt im Modell, und
    /// die beiden Anzeigen ziehen ueber dieselben zwei Stellen nach wie nach
    /// jedem anderen Wechsel des Gehaltenen.
    pub fn schliessen(&self) {
        self.ivars().modell.borrow_mut().schliessen();
        // Die Datei ist aufgegeben, und mit ihr ihr Verlauf.
        self.stand_einsetzen(Verlauf::Faellt);
        self.kopf_nachziehen();
        // Ohne Datei gibt es keine Sprache und nichts einzufaerben; der Ruf
        // raeumt die gesetzten Merkmale ab und laesst einen laufenden
        // Einfaerbungsfaden fallen.
        self.darstellung_nachziehen();
    }

    /// Holt die Meldung des Arbeitsfadens ab (C2).
    ///
    /// **Der Vergleich nennt [`Ladeausgang::Geoeffnet`] namentlich und darf
    /// nicht auf "nicht abgewiesen" gelockert werden.** Das ist die Haelfte der
    /// Behebung vom 260809, die in dieser Datei steht: bei
    /// [`Ladeausgang::SchonOffen`] hat das Modell nicht gelesen, und die Flaeche
    /// traegt das, was der Nutzer getippt und noch nicht gesichert hat; ein Ruf
    /// von [`Self::stand_einsetzen`] schriebe den Plattenstand darueber, und
    /// genau so ging die Aenderung des Nutzers verloren
    /// (`issues/260809-2029_*_eine-ungesicherte-aenderung-ist-fort-wenn-die-vorschau-dieselbe-datei-zeigt.md`).
    /// Dass jener Ausgang seit S24 gar nicht mehr hier ankommt, weil das Modell
    /// ihn entscheidet, bevor ein Faden startet, macht die Namensnennung nicht
    /// ueberfluessig: sie ist die Stelle, an der ein spaeter dazukommender
    /// Ausgang auffaellt, statt still mitzulaufen.
    ///
    /// **Der Takt endet, sobald nichts mehr laeuft**, und zwar auf beiden
    /// Wegen: nach einer eingetroffenen Meldung und nach einem Faden, der ohne
    /// Meldung gefallen ist. Der zweite Fall hinterlaesst allein die Zeile auf
    /// der Standardfehlerausgabe, die `Ladevorgang::starten` schreibt; dasselbe
    /// gilt seit der Runde 1 fuer die Vorschau.
    ///
    /// **Ein Takt fuer zwei Arbeitsfaeden.** Seit S33 laeuft neben dem Lesen das
    /// Einfaerben auf einem eigenen Faden, und beide werden hier abgeholt. Ein
    /// zweiter Zeitgeber daneben fragte im selben Sechzigstel dieselbe
    /// Laufschleife ein zweites Mal; er braechte kein Bild frueher, weil in
    /// einem Bild nur einmal gezeichnet wird.
    fn einziehen(&self) {
        self.ladeausgang_einziehen();
        self.einfaerbung_einziehen();
        if !self.ivars().modell.borrow().laedt_noch() && self.ivars().einfaerbung.borrow().is_none()
        {
            self.takt_beenden();
        }
    }

    /// Holt die Meldung des Lesefadens ab (C2).
    fn ladeausgang_einziehen(&self) {
        let eingetroffen = self.ivars().modell.borrow_mut().einziehen();
        let Some(ausgang) = eingetroffen else {
            return;
        };
        if ausgang == Ladeausgang::Geoeffnet {
            // Die neue Datei kann eine andere Besetzung der Formatansicht
            // verlangen als die vorige: Schrift, Umbruch und Einfaerbung
            // haengen am Dateityp und an der Sprache, die die Kiste kennt.
            // `Faellt`, weil der Verlauf auf den Text der vorigen Datei zeigte.
            self.stand_erneuern(Verlauf::Faellt);
        }
        self.melden(ausgang);
    }

    /// Gibt den Ausgang samt seiner Herkunft an die Senke weiter, falls jemand
    /// zuhoert.
    ///
    /// Die Ausleihe steht waehrend des Rufs, wie bei `Hauptfenster::melden`.
    /// Sie ist lesend, und der einzige schreibende Zugriff auf dieselbe Zelle
    /// ist [`Self::melder_setzen`] beim Aufbau; ein Ruf, der ueber AppKit
    /// hierher zuruecklaeuft, nimmt eine zweite Leseausleihe und keine
    /// schreibende.
    ///
    /// **Die Herkunft wird hier gelesen und nicht verbraucht.** Sie gehoert dem
    /// zuletzt begonnenen Oeffnen und gilt bis zum naechsten; der Grund und die
    /// drei Aufrufer dieser Funktion stehen an [`EditorIvars::herkunft`].
    fn melden(&self, ausgang: Ladeausgang) {
        let herkunft = self.ivars().herkunft.get();
        let melden = self.ivars().melden.borrow();
        if let Some(melden) = melden.as_ref() {
            melden(ausgang, herkunft);
        }
    }

    /// Haengt den Zeitgeber in die Laufschleife, falls er noch nicht laeuft.
    fn takt_starten(&self) {
        if self.ivars().takt.borrow().is_some() {
            return;
        }
        // SAFETY: `self` ist das Ziel und beantwortet `ladenEinziehen:` mit der
        // erwarteten Signatur. Der Zeitgeber wird unten in die Laufschleife
        // gehaengt; `NSRunLoopCommonModes` ist ein Fremdsymbol von Foundation.
        // Dieselbe Form wie der Einzugstakt der Vorschau.
        let zeitgeber = unsafe {
            let zeitgeber = NSTimer::timerWithTimeInterval_target_selector_userInfo_repeats(
                LADETAKT,
                self,
                sel!(ladenEinziehen:),
                None,
                true,
            );
            NSRunLoop::currentRunLoop().addTimer_forMode(&zeitgeber, NSRunLoopCommonModes);
            zeitgeber
        };
        *self.ivars().takt.borrow_mut() = Some(zeitgeber);
    }

    /// Nimmt den Zeitgeber aus der Laufschleife und loest den Ring auf.
    fn takt_beenden(&self) {
        if let Some(zeitgeber) = self.ivars().takt.borrow_mut().take() {
            zeitgeber.invalidate();
        }
    }

    /// Schreibt zurueck, was der Nutzer in die Flaeche getippt hat (C4).
    ///
    /// **Der Rueckweg**, und die eine Stelle, an der der Stand der Flaeche zum
    /// Stand des Modells wird. Er nimmt den ganzen Text und nicht die geaenderte
    /// Stelle; der Grund und der Preis stehen an [`Editormodell::bearbeiten`],
    /// das ihn dabei durch `krk_core::text::datei::in_gehaltene_form` fuehrt.
    /// Eine `NSTextView` bewahrt eingefuegten Text zeichengetreu auf, also
    /// kommt ein `\r\n` aus einer Windows-Quelle hier an und darf nicht weiter.
    ///
    /// **Hat die Wandlung zugegriffen, wird die Flaeche nachgezogen.** Sie
    /// behielte sonst ihre `\r`, waehrend der Stand sie nicht mehr traegt, und
    /// von der eingefuegten Stelle an zeigte dieselbe Zahl in den beiden Texten
    /// auf Verschiedenes; [`Self::flaeche_richten`] fuehrt aus, was daran
    /// haengt. Der gewoehnliche Anschlag kommt an dieser Zeile vorbei, weil
    /// [`Editormodell::bearbeiten`] dann `false` liefert.
    ///
    /// **Die Abschrift des alten Standes entsteht vor der Wandlung, und nur dann,
    /// wenn sie bevorsteht.** Der Umkehrpunkt braucht den Stand, den das Modell
    /// **vor** dem Einfuegen hielt; danach ist der fort, und `260810-1044` fuehrte
    /// genau das als den Grund, aus dem ein eingefuegtes `\r\n` nicht
    /// zuruecknehmbar war. Gefragt wird mit
    /// `krk_core::text::datei::ist_in_gehaltener_form`, derselben Bedingung, an
    /// der [`Editormodell::bearbeiten`] seine Wandlung entscheidet, und die
    /// Abschrift entsteht deshalb **nicht** je Tastendruck, sondern nur auf dem
    /// Weg, der die Flaeche ohnehin neu beschreibt. Das ist der Unterschied zu der
    /// Kette, die `260810-0424` als zu teuer fuehrt.
    ///
    /// **Der Punkt selbst entsteht danach und haelt die Abschrift nicht.** Er
    /// braucht beide Staende, um den geaenderten Bereich zu finden, und faengt
    /// deshalb den Augenblick ab, in dem beide vorliegen: der alte als Abschrift,
    /// der neue im Modell. Die Abschrift faellt mit dem Block, der Bereich geht in
    /// den Stapel. Warum nicht der ganze Stand, steht an [`Umkehrpunkt`].
    ///
    /// **Was der zusaetzliche Durchlauf kostet, ist gemessen** (260810, dieses
    /// Geraet, `--release`): 0,017 ms bei 229 kB, 0,13 ms bei 1,8 MB, 1,8 ms bei
    /// 19 MB. Daneben stehen 0,98 / 7,6 / 88 ms fuer das Umschreiben des Textes
    /// aus UTF-16, das jedem Ruf hierher vorausgeht; der Aufpreis liegt bei zwei
    /// Prozent.
    ///
    /// # Der ganze Anschlag kostet, und der Preis ist angenommen
    ///
    /// Gemessen in derselben Reihe, je Anschlag und auf dem Hauptfaden:
    ///
    /// ```text
    ///        Byte   Summe je Anschlag   davon in `string().to_string()`
    ///     229 029             1,02 ms                            96 %
    ///   1 832 232             7,87 ms                            97 %
    ///  19 467 465            91,96 ms                            96 %
    /// ```
    ///
    /// **Der Preis liegt in der ersten Zeile dieser Funktion und nicht in der
    /// Wandlung.** Bei 229 kB ist eine Millisekunde je Anschlag nicht zu
    /// bemerken, bei 1,8 MB bleiben 7,9 ms unter einer Bildlaenge von 16,7 ms,
    /// und an der Editorgrenze von 16 MB stehen rund 75 ms je Anschlag, also
    /// gute vier Bildlaengen: **von einigen Megabyte an stockt das Tippen
    /// sichtbar.**
    ///
    /// **Angenommen, nicht verschwiegen.** Billiger wird es nur mit dem
    /// geaenderten Bereich, den `NSTextStorage` mit jeder Aenderung meldet
    /// (`editedRange`, `changeInLength`), und der verlangt einen Stand, der sich
    /// fortschreibt, statt neu gelesen zu werden — also eine Aenderung an
    /// [`Editormodell::bearbeiten`] und an der einen Normalisierungsstelle in
    /// `krk_core::text::datei`. Der Rueckweg aus der Flaeche haette dann einen
    /// zweiten Eingang und eine zweite Wahrheit darueber, was der gehaltene
    /// Stand ist; genau das schliesst der Modulkopf von
    /// [`crate::editormodell`] aus. Ein billigeres Umschreiben an dieser Stelle
    /// gibt es nicht: `to_string` geht ueber `UTF8String`, und jeder schnellere
    /// Zugriff auf die Zeichen braucht `unsafe`, das in dieser Kiste allein
    /// [`super`] traegt. Der Datensatz mit den vollen Zahlen ist
    /// `issues/260809-2322_*_der-ganze-stand-geht-je-tastendruck-durch-bearbeiten.md`.
    ///
    /// **Der Kopf wird nur beim Uebergang nachgezogen.** Die Abweichungsmarke
    /// geht von falsch nach wahr und bleibt dort bis zum naechsten Sichern; sie
    /// bei jedem Anschlag neu in ein `NSTextField` zu schreiben hiesse, je
    /// Tastendruck ein Auslegen anzustossen, das nichts aendert.
    ///
    /// Die Ausleihe des Modells endet vor dem Ruf an den Kopf, wie ueberall in
    /// dieser Datei.
    fn text_zurueckschreiben(&self) {
        let stand = self.ivars().text.string().to_string();
        let auswahl = self.ivars().text.selectedRange();
        let (war_abweichend, umkehrpunkt) = {
            let mut modell = self.ivars().modell.borrow_mut();
            let vorher = modell.hat_ungesicherten_stand();
            // Die Abschrift entsteht allein auf dem Wandlungsweg und faellt am
            // Ende dieses Blocks; in den Stapel geht nur der Unterschied, den
            // `Umkehrpunkt::zwischen` daraus bildet.
            let alter_stand =
                (!datei::ist_in_gehaltener_form(&stand)).then(|| modell.stand().to_owned());
            let gewandelt = modell.bearbeiten(stand);
            debug_assert_eq!(
                gewandelt,
                alter_stand.is_some(),
                "die Frage vor der Wandlung und ihr Ausgang muessen dieselbe sein"
            );
            let umkehrpunkt = alter_stand
                .map(|alter_stand| Umkehrpunkt::zwischen(&alter_stand, modell.stand(), auswahl));
            (vorher, umkehrpunkt)
        };
        if let Some(punkt) = umkehrpunkt {
            self.flaeche_richten(punkt);
        }
        if !war_abweichend {
            self.kopf_nachziehen();
        }
        // Die Einfaerbung gehoert zu dem Stand, aus dem sie gebildet wurde. Wer
        // ein Anfuehrungszeichen tippt, macht aus dem Rest der Datei eine
        // Zeichenkette, und ohne diesen Ruf bliebe die alte Farbe stehen. Die
        // Anfrage kostet nichts, solange schon eine laeuft; siehe
        // [`Self::einfaerbung_anfordern`].
        self.einfaerbung_anfordern();
    }

    /// Schreibt Dateiname und Abweichungszeichen in den Kopf (C4).
    ///
    /// **Die eine Stelle, die den Kopf beschreibt.** Sie wird gerufen, wo sich
    /// eine der beiden Angaben aendern kann: beim Aufbau, nach einem gelungenen
    /// Oeffnen, beim Uebergang in den ungesicherten Stand, nach einem
    /// gelungenen Sichern und seit S28 nach dem Schliessen.
    ///
    /// Was dort steht, entscheidet [`kopfzeile`] ohne AppKit und ist deshalb
    /// ohne Fenster pruefbar.
    fn kopf_nachziehen(&self) {
        let zeile = {
            let modell = self.ivars().modell.borrow();
            kopfzeile(modell.pfad(), modell.hat_ungesicherten_stand())
        };
        self.ivars()
            .kopf
            .setStringValue(&NSString::from_str(&zeile));
    }

    /// Schreibt den gehaltenen Stand in die Textflaeche.
    ///
    /// **Die eine Stelle, die den Text der Flaeche ersetzt.** Ein
    /// Ansichtswechsel geht nicht ueber sie, sondern ueber die
    /// voruebergehenden Merkmale des Layoutverwalters; deshalb kann er nichts
    /// verlieren.
    ///
    /// Die Ausleihe des Modells endet, bevor der Aufruf in das Textsystem
    /// faellt. `NSString::from_str` steht noch darin, und das ist kein Bruch
    /// der Regel: es kopiert Bytes und ruft nichts zurueck, waehrend
    /// `setString:` den Layoutverwalter, die Delegierten und damit
    /// moeglicherweise KRK selbst erreicht. Den Stand stattdessen zu klonen
    /// hiesse, eine Datei von 16 MB zweimal zu kopieren statt einmal.
    ///
    /// **Und die eine Stelle, die den Rueckgaengigverlauf regelt.** `setString:`
    /// schreibt an der Rueckgaengigverwaltung vorbei, und deshalb hat jeder Ruf
    /// hierher zu sagen, was danach im Stapel steht; was die drei Anlaesse
    /// verlangen und warum, steht an [`Verlauf`]. Bis zum 260810 stand hier eine
    /// Antwort fuer alle drei — der Stapel wurde immer geleert —, und das kostete
    /// dem Nutzer den Verlauf an zwei Anlaessen, an denen die Datei dieselbe
    /// blieb
    /// (`issues/260810-0303_*_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`).
    ///
    /// **Die Anmeldung geht dem Schreiben voraus.** Der Verwalter soll die
    /// Handlung auch dann tragen, wenn `setString:` unten am Text nichts mehr
    /// aendert; und der Umkehrpunkt kommt ohnehin vom Aufrufer, der ihn vor der
    /// Aenderung des Modells genommen hat.
    ///
    /// **Bei [`Verlauf::TraegtNurDiese`] wird zuerst geleert und dann
    /// angemeldet**, und die Reihenfolge ist die ganze Aussage dieses Wertes: was
    /// vorher im Stapel stand, zeigt auf einen Text, den es nicht mehr gibt, und
    /// die eine Handlung, die danach darin steht, ist die gueltige. Dass eine
    /// Anmeldung nach `removeAllActions` stehen bleibt und wirkt, ist
    /// **gemessen** und nicht angenommen:
    /// [`eine_anmeldung_nach_dem_leeren_steht_im_stapel`](tests::eine_anmeldung_nach_dem_leeren_steht_im_stapel).
    ///
    /// Die beiden aelteren Anlaesse behalten ihre Reihenfolge Zeile fuer Zeile:
    /// [`Verlauf::Faellt`] leert **nach** dem Schreiben, [`Verlauf::Traegt`]
    /// meldet **vor** ihm an und leert nicht.
    fn stand_einsetzen(&self, verlauf: Verlauf) {
        let leeren_danach = match verlauf {
            Verlauf::Faellt => true,
            Verlauf::Traegt(punkt) => {
                self.umkehrung_anmelden(punkt);
                false
            }
            Verlauf::TraegtNurDiese(punkt) => {
                rueckgaengigstapel_leeren(self.ivars().text.undoManager().as_deref());
                self.umkehrung_anmelden(punkt);
                false
            }
        };
        let stand = {
            let modell = self.ivars().modell.borrow();
            NSString::from_str(modell.stand())
        };
        self.ivars().text.setString(&stand);
        if leeren_danach {
            rueckgaengigstapel_leeren(self.ivars().text.undoManager().as_deref());
        }
    }

    /// Meldet beim Rueckgaengigverwalter der Flaeche eine Handlung an, die den
    /// genannten Umkehrpunkt wiederherstellt (C5).
    ///
    /// **Die Handlung geht in denselben Verwalter, in dem die Flaeche ihr
    /// Tippen fuehrt**, und das ist die Voraussetzung dafuer, dass beides
    /// nebeneinander bestehen kann: [`Self::umkehren`] stellt genau die Zeichen
    /// wieder her, die die Flaeche vor dem Umbau trug, und damit passen die
    /// aelteren Handlungen der Flaeche wieder auf den Text, gegen den sie
    /// aufgezeichnet wurden. Ein zweiter, eigener Verwalter neben dem der
    /// Flaeche truege den Umbau in einen anderen Stapel als das Tippen, und ein
    /// `cmd+z` nahme die beiden dann in der falschen Reihenfolge zurueck.
    ///
    /// **`None` ist kein Fehler.** `NSResponder::undoManager` findet den
    /// Verwalter erst, wenn die Flaeche in einem Fenster steht; vor dem
    /// Einhaengen gibt es keinen, und dann gibt es auch keinen Verlauf, in dem
    /// eine Handlung stehen koennte. Die Anlaesse dieser Funktion — die beiden
    /// Ersetzen aus S37 — kommen lange danach.
    ///
    /// **Der Verwalter haelt das Ziel nicht fest** ("this does not strongly
    /// retain target", `objc2-foundation-0.3.2/src/generated/NSUndoManager.rs`),
    /// und der Block haelt den Editorbereich deshalb **schwach**: stark
    /// geschlossen liefe der Ring Flaeche → Verwalter → Block → Editorbereich →
    /// Flaeche. Ist der Bereich fort, tut die Handlung nichts.
    ///
    /// **Die Bytes des Punktes werden hier angetragen und nicht hier abgetragen.**
    /// Angemeldet wird eine [`Stapellast`], und sie traegt ab, wenn der Verwalter
    /// den Block aufhebt; warum das Zaehlen an der Handlung haengt und nicht an
    /// dieser Zeile, steht dort. Ohne Verwalter geht der Punkt in der Zeile
    /// darueber verloren, und dann ist auch nichts zu zaehlen.
    fn umkehrung_anmelden(&self, punkt: Umkehrpunkt) {
        let Some(verwalter) = self.ivars().text.undoManager() else {
            return;
        };
        let selbst = Weak::from_retained(&self.retain());
        let last = Stapellast::angemeldet(punkt, &self.ivars().stapelbytes);
        let handlung = RcBlock::new(move |_ziel: NonNull<AnyObject>| {
            if let Some(editor) = selbst.load() {
                editor.umkehren(&last.punkt);
            }
        });
        // SAFETY: `self` ist ein Objective-C-Objekt und wird vom Verwalter nur
        // als Kennung gehalten und an den Block zurueckgereicht, nicht
        // angesprochen; der Block nimmt es nicht, sondern laedt seinen eigenen
        // schwachen Verweis.
        unsafe { verwalter.registerUndoWithTarget_handler(self, &handlung) };
    }

    /// Was aus dem Verlauf wird, wenn dieser Punkt in den Stapel geht: der Punkt
    /// tritt dazu, oder er tritt an die Stelle des ganzen Verlaufs.
    ///
    /// **Die eine Stelle, die das Budget anwendet.** Beide Ersetzungsbefehle aus
    /// C5 gehen durch sie, und was sie entscheidet, steht an [`STAPELBUDGET`]:
    /// passt der Punkt neben das, was schon im Stapel steht, traegt der Verlauf
    /// ihn zusaetzlich; passt er nicht, steht er danach allein darin. Der Nutzer
    /// merkt das erste an nichts und das zweite daran, dass ein zweites `cmd+z`
    /// nichts mehr tut.
    ///
    /// **Der gewoehnliche Weg ist der erste**, und das ist keine Hoffnung,
    /// sondern die Rechnung: ein einzelnes `shift+cmd+r` haelt so viele Bytes wie
    /// der ersetzte Treffer lang ist, also drei fuer ein `foo`. Bis das Budget
    /// von 16 MB voll ist, braucht es Millionen davon. Getroffen wird der zweite
    /// Weg von einem Sammelersetzen, dessen Bereich die ganze Datei deckt — genau
    /// der Fall aus
    /// `issues/260810-1314_*_ein-wiederholtes-sammelersetzen-legt-je-ruf-einen-bereich-in-dateigroesse-in-den-stapel.md`.
    ///
    /// **Ein `cmd+z` fragt hier nicht.** [`Self::umkehren`] meldet seinen Gegenweg
    /// als [`Verlauf::Traegt`] an und geht an dieser Stelle vorbei, weil ein
    /// Rueckgaengig keinen Verlauf raeumen darf: es nimmt eine Handlung vom
    /// Stapel und legt eine von derselben Groesse auf den anderen, die Summe
    /// bleibt also, wo sie war. Ein Budget, das auch dort zugriffe, koennte einen
    /// Nutzer, der `cmd+z` und `shift+cmd+z` gegeneinander laufen laesst, um
    /// seinen Verlauf bringen.
    ///
    /// Gerechnet wird in [`verlauf_fuer_umbau`], damit die Regel ohne Fenster
    /// pruefbar ist; hier steht allein, woher der Zaehler kommt.
    fn verlauf_fuer_umbau(&self, punkt: Umkehrpunkt) -> Verlauf {
        verlauf_fuer_umbau(punkt, self.ivars().stapelbytes.get())
    }

    /// Stellt einen Umkehrpunkt her und meldet den Gegenweg an (C5).
    ///
    /// **Der Gegenweg zuerst.** Waehrend eines Rueckgaengig legt
    /// `NSUndoManager` jede Anmeldung auf den Wiederherstellungsstapel; die
    /// Reihenfolge hier ist deshalb die, die `cmd+z` und `shift+cmd+z`
    /// gegeneinander laufen laesst. Genommen wird er vor dem Ruf an
    /// [`Editormodell::bearbeiten`], weil das Modell danach den anderen Stand
    /// haelt.
    ///
    /// **Der Stand kommt aus dem Modell und ist deshalb in gehaltener Form.** Der
    /// wiederhergestellte Stand entsteht aus dem gehaltenen und dem Bereich, den
    /// der Punkt traegt, und beide sind gehalten; [`Editormodell::bearbeiten`]
    /// wandelt daran nichts und meldet keine Nachrichtung der Flaeche. Die
    /// Zusicherung haelt das fest, statt den Wert still fallenzulassen.
    ///
    /// # Der Suchlauf wird neu gebildet und nicht mitgeschleppt
    ///
    /// [`Editormodell::bearbeiten`] beendet den Suchlauf, und das ist beim Tippen
    /// richtig: dort sind die Byteversaetze der Treffer nach der Aenderung
    /// ungueltig. Beide Ersetzungswege bauen ihn deshalb eigens neu auf, damit
    /// `cmd+g` und `shift+cmd+r` weiterlaufen — und ein `cmd+z` warf ihn bis zum
    /// 260810-1244 wieder fort. Der Nutzer bezahlte eine zurueckgenommene
    /// Ersetzung mit seiner laufenden Suche, und das zweite `shift+cmd+r`
    /// antwortete `Editormeldung::KeineSuche`
    /// (`issues/260810-1244_*_ein-cmd-z-nach-einem-ersetzen-loescht-den-suchlauf-den-das-ersetzen-eigens-aufgebaut-hat.md`).
    ///
    /// **Genau hier ist die Trefferliste ausrechenbar**, denn der Text ist der von
    /// vorher. Gerechnet wird sie ueber
    /// [`Editormodell::suche_starten`](crate::editormodell::Editormodell::suche_starten)
    /// — denselben Weg, den `cmd+f` und die beiden Ersetzungswege gehen — und ab
    /// derselben Stelle, an die dieser Ruf die Schreibmarke setzt. Damit steuert
    /// der wiederhergestellte Suchlauf den Treffer an, an dem der Nutzer steht.
    ///
    /// **`bearbeiten` bleibt dabei, wie es ist.** Es kann Tippen und Rueckgaengig
    /// nicht unterscheiden; den Anlass kennt allein der Aufrufer, genau wie bei
    /// [`Verlauf`].
    fn umkehren(&self, punkt: &Umkehrpunkt) {
        let auswahl = self.ivars().text.selectedRange();
        let gegenweg = {
            let mut modell = self.ivars().modell.borrow_mut();
            let wiederhergestellt = punkt.angewandt_auf(modell.stand());
            let gegenweg = Umkehrpunkt::zwischen(modell.stand(), &wiederhergestellt, auswahl);
            let gesucht = modell.suchlauf().map(|lauf| lauf.gesucht().to_owned());
            let gewandelt = modell.bearbeiten(wiederhergestellt);
            debug_assert!(
                !gewandelt,
                "der Stand kam aus dem Modell und traegt keine Zeichen, die die Wandlung anfasst"
            );
            if let Some(gesucht) = gesucht {
                let ab_versatz = koordinaten::in_bytes(modell.stand(), punkt.auswahl.location);
                let _ = modell.suche_starten(&gesucht, ab_versatz);
            }
            gegenweg
        };
        self.stand_erneuern(Verlauf::Traegt(gegenweg));
        self.auswahl_setzen(punkt.auswahl);
    }

    /// Setzt die Auswahl der Flaeche, auf die Laenge des heutigen Textes
    /// beschnitten, und blaettert sie ins Bild.
    ///
    /// **Der Schnitt ist der Guertel und nicht die Rechnung.** Der Bereich
    /// kommt aus einem [`Umkehrpunkt`] und ist gegen genau den Text
    /// aufgezeichnet, den die Zeile darueber wiederhergestellt hat; er passt
    /// also. Ein `NSRange` hinter dem Text beantwortet AppKit mit einer
    /// Objective-C-Ausnahme, und die ist in Rust nicht zu fangen und beendet das
    /// Programm — derselbe Grund, aus dem
    /// [`super::textmerkmale::anwenden`] die Laenge vorweg prueft.
    fn auswahl_setzen(&self, auswahl: NSRange) {
        let laenge = self.ivars().text.string().length();
        let anfang = auswahl.location.min(laenge);
        let bereich = NSRange::new(anfang, auswahl.length.min(laenge - anfang));
        self.ivars().text.setSelectedRange(bereich);
        self.ivars().text.scrollRangeToVisible(bereich);
    }

    /// Traegt einen von aussen gewechselten Stand in die Flaeche und zieht die
    /// beiden Anzeigen nach.
    ///
    /// **Die drei Schritte, die zusammengehoeren**, und die eine Stelle, an der
    /// sie stehen: der Text, der Kopf, die Darstellung. Drei Aufrufer gehen
    /// durch sie — ein gelungenes Oeffnen, die uebernommene zurueckgehaltene
    /// Datei und seit S37 das Ersetzen —, und ohne diese Funktion waeren es
    /// drei Stellen mit derselben Reihenfolge und der ersten Gelegenheit, sie
    /// verschieden zu schreiben.
    ///
    /// **Sie gilt nicht fuer den Nutzer, der tippt.** Dessen Weg ist der
    /// umgekehrte: die Flaeche traegt den Stand schon, und
    /// [`Self::text_zurueckschreiben`] holt ihn ab. `setString:` von hier aus
    /// setzte ihm die Schreibmarke an den Anfang und leerte den
    /// Rueckgaengigstapel bei jedem Anschlag.
    ///
    /// Was aus dem Rueckgaengigverlauf wird, entscheidet der Aufrufer; die
    /// Antwort geht unveraendert an [`Self::stand_einsetzen`] weiter, und
    /// welchen Anlass welche Antwort verlangt, steht an [`Verlauf`].
    fn stand_erneuern(&self, verlauf: Verlauf) {
        self.stand_einsetzen(verlauf);
        self.kopf_nachziehen();
        self.darstellung_nachziehen();
    }

    /// Bringt die Textflaeche auf den gehaltenen Stand, nachdem die Wandlung in
    /// die gehaltene Form beide auseinandergebracht hat.
    ///
    /// **Der Anlass ist einer**, und [`Self::text_zurueckschreiben`] ist die
    /// einzige Stelle, die ihn kennt: der Nutzer hat Text eingefuegt, den eine
    /// `NSTextView` zeichengetreu aufbewahrt, `krk_core::text::datei` hat ihn
    /// auf dem Weg in den Stand gewandelt, und die Flaeche traegt seither
    /// Zeichen, die der Stand nicht traegt. Von der eingefuegten Stelle an
    /// zeigte danach jede Stelle in den beiden Texten auf Verschiedenes, und
    /// die vier Wege durch [`super::koordinaten`] — Zeilensprung, Suche,
    /// Markensprung und die Auskunft ueber die Schreibmarkenzeile — rechneten
    /// gegen den falschen Text. Der Defekt ist
    /// `issues/260810-0215_*_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`.
    ///
    /// # Zwei Wege standen zur Wahl, und dieser haelt die Zusage ohne zweite
    ///
    /// Der andere waere gewesen, das `\r` am Eingang der Flaeche abzufangen,
    /// ueber `textView:shouldChangeTextInRanges:replacementStrings:`. Er ist
    /// nicht genommen, weil er die Regeln der Wandlung ein zweites Mal tragen
    /// muesste und dabei **nicht dieselben** waeren: die Bytefolgenmarke faellt
    /// nach ihrer Stelle im ganzen Text, ein eingefuegtes Stueck kennt seine
    /// Stelle aber nur beim Einfuegen. Ein Loeschen, das eine Marke aus der
    /// Mitte an den Anfang rueckt, ginge an so einem Eingangsfilter vorbei und
    /// braechte die beiden erneut auseinander. Diese Stelle hier vergleicht
    /// stattdessen das Ergebnis und kommt deshalb ohne eine einzige Regel der
    /// Wandlung aus.
    ///
    /// # Warum hier eine Handlung steht und der Verlauf davor trotzdem faellt
    ///
    /// Seit dem 260810-1044 ist das Einfuegen zuruecknehmbar: der Umbau geht als
    /// [`Verlauf::TraegtNurDiese`] durch die eine Schreibstelle und meldet dort
    /// eine Handlung an, die den Stand **vor** dem Einfuegen wiederherstellt. Der
    /// Umkehrpunkt entsteht in [`Self::text_zurueckschreiben`] und nur auf diesem
    /// Weg; der Preis steht dort.
    ///
    /// **Der Verlauf davor kann trotzdem nicht bleiben**, und das ist eine
    /// Eigenschaft der Sache und nicht der Sorgfalt. Die Handlung, die die
    /// Flaeche fuer das Einfuegen selbst angemeldet hat, zeigt auf einen Bereich,
    /// der um die Zahl der weggefallenen `\r` zu lang ist; bliebe sie stehen,
    /// loeschte ein zweites `cmd+z` Zeichen hinter dem Eingefuegten mit. Ein
    /// `NSUndoManager` laesst keine einzelne Handlung herausnehmen, also faellt
    /// der Stapel und die eine gueltige Handlung wird danach neu angemeldet.
    /// **Was der Nutzer davon merkt:** das erste `cmd+z` nimmt das Einfuegen
    /// zurueck, ein zweites tut nichts. Vor dem 260810-1044 tat schon das erste
    /// nichts (`issues/260810-1044_*_ein-eingefuegtes-crlf-bleibt-nicht-ruecknehmbar-und-der-grund-liegt-am-eingang-der-flaeche.md`),
    /// und der Verlauf davor fiel ebenso.
    ///
    /// **Der Eingangsfilter bleibt ungenommen**, aus den Gruenden zwei Abschnitte
    /// weiter oben. Er waere der Weg, den Verlauf davor mitzuretten, und kostete
    /// die Wandlungsregeln ein zweites Mal.
    ///
    /// **Die Schreibmarke bleibt, wo sie stand.** Sie waere sonst nach jedem
    /// Einfuegen aus einer Windows-Quelle am Dateianfang, also genau in dem
    /// Augenblick, in dem der Nutzer weiterschreiben will. Wohin sie wandert,
    /// rechnet `krk_core::text::datei::versatz_nach_der_wandlung` und nicht
    /// diese Zeile; gezeigt wird sie ueber [`Self::stelle_zeigen`], denselben
    /// Weg, den Zeilensprung und Suche gehen.
    fn flaeche_richten(&self, punkt: Umkehrpunkt) {
        // Die Flaeche traegt in dieser Zeile noch den ungewandelten Text; das
        // Umschreiben aus UTF-16 kostet einen zweiten Durchlauf und faellt
        // allein auf diesen Weg, nicht auf jeden Tastendruck.
        let vorher = self.ivars().text.string().to_string();
        let schreibmarke = koordinaten::in_bytes(&vorher, self.schreibmarke_in_utf16());
        let versatz = {
            let modell = self.ivars().modell.borrow();
            datei::versatz_nach_der_wandlung(&vorher, schreibmarke, modell.stand())
        };
        self.stand_erneuern(Verlauf::TraegtNurDiese(punkt));
        self.stelle_zeigen(versatz, versatz);
    }

    // ------------------------------------------------------------------
    // Der Zeilensprung, die Suche und das Ersetzen (C5)
    // ------------------------------------------------------------------

    /// Die Zeile, in der die Schreibmarke steht: Nummer ab 1 und ihr Inhalt
    /// (C6).
    ///
    /// **Sie steht hier und nicht beim Aufrufer**, weil hier der gehaltene
    /// Stand und die Textflaeche beieinander liegen und die Umrechnung zwischen
    /// den beiden Koordinaten genau einmal vorkommen soll; der Defekt
    /// `issues/260810-0036_*_dem-editor-fehlt-die-auskunft-ueber-die-zeile-der-schreibmarke.md`
    /// fuehrt den Grund im Einzelnen. Gerechnet wird in
    /// [`super::koordinaten`] und in `krk_core::text::zeilen`.
    ///
    /// Vier Eigenschaften, die der Aufrufer sich merken darf:
    ///
    /// - **`None`, wenn der Editor keine Datei haelt.** Ohne Datei gibt es
    ///   keine Stelle, die eine Marke bezeichnen koennte.
    /// - **Eine Zeile und kein Bereich.** Ist mehrzeilig ausgewaehlt, gilt die
    ///   Zeile am **Anfang** der Auswahl. `selectedRange` nennt allein den
    ///   kleineren Versatz, und in welche Richtung der Nutzer gezogen hat, geht
    ///   daraus nicht hervor; der Anfang ist damit der einzige Versatz, den
    ///   AppKit verlaesslich liefert.
    /// - **Der Inhalt kommt aus dem gehaltenen Stand und nicht von der
    ///   Platte.** Dieselbe Regel, die das neunte Abnahmekriterium von C5 der
    ///   Suche gibt: der Editor merkt sich, was er zeigt.
    /// - **Der Inhalt ist die Zeile ohne ihren Umbruch**, so wie
    ///   `Zeilenindex::inhalt_der_zeile` sie liefert;
    ///   `krk_core::text::marke::wiederfinden` vergleicht spaeter gegen genau
    ///   diese Form.
    ///
    /// Der Aufrufer steht seit S38: `cmd+d` mit dem Fokus im Editor baut sein
    /// `krk_core::ablage::Ziel::Textstelle` aus dieser Auskunft und aus
    /// [`Self::pfad`].
    pub fn schreibmarkenzeile(&self) -> Option<(u32, String)> {
        let stelle = self.schreibmarke_in_utf16();
        let modell = self.ivars().modell.borrow();
        if !modell.haelt_datei() {
            return None;
        }
        let stand = modell.stand();
        let versatz = koordinaten::in_bytes(stand, stelle);
        let index = Zeilenindex::neu(stand);
        let nummer = index.zeile_am_versatz(versatz);
        let inhalt = index.inhalt_der_zeile(stand, nummer)?.to_owned();
        // Eine Datei von hoechstens 16 MB hat hoechstens 16 Millionen Zeilen;
        // der Rueckfall ist unerreichbar und steht da, weil ein `as` die Zahl
        // still verdrehte.
        Some((u32::try_from(nummer).unwrap_or(u32::MAX), inhalt))
    }

    /// Wo die Schreibmarke in AppKits Koordinate steht.
    ///
    /// Der Anfang der Auswahl, und der Grund dafuer steht an
    /// [`Self::schreibmarkenzeile`].
    ///
    /// # Die Annahme, auf der die Umrechnung ruht
    ///
    /// Die Stelle zaehlt in den **Zeichen der Flaeche**, und umgerechnet wird
    /// sie gegen den **gehaltenen Stand**. Beide sind Zeichen fuer Zeichen
    /// dieselben, und vier Wege halten sie so: `stand_einsetzen` schreibt den
    /// Stand in die Flaeche, `text_zurueckschreiben` holt ihn zurueck,
    /// [`Self::flaeche_richten`] richtet die Flaeche nach, wenn die Wandlung in
    /// die gehaltene Form dabei zugegriffen hat, und der Ansichtswechsel aus C3
    /// fasst den Textspeicher gar nicht an.
    ///
    /// **Der vierte Weg ist der juengste**, und ohne ihn brach die Annahme: wer
    /// Text mit `\r\n` aus einer Windows-Quelle einfuegte, hatte danach in der
    /// Flaeche zwei Zeichen, wo der Stand eines trug, und jede Stelle hinter der
    /// eingefuegten war um die Zahl der `\r` verschoben. Behoben mit
    /// `issues/260810-0215_*_der-stand-und-der-text-der-flaeche-laufen-nach-einem-eingefuegten-crlf-auseinander.md`.
    fn schreibmarke_in_utf16(&self) -> usize {
        self.ivars().text.selectedRange().location
    }

    /// Waehlt den Byteversatzbereich in der Flaeche aus und blaettert ihn ins
    /// Bild.
    ///
    /// **Die eine Stelle, die eine Stelle des Standes in der Flaeche sichtbar
    /// macht.** Der Zeilensprung reicht denselben Versatz zweimal herein und
    /// bekommt damit eine Schreibmarke ohne Ausdehnung; die Suche reicht Anfang
    /// und Ende eines Treffers herein und bekommt ihn ausgewaehlt.
    ///
    /// Die Nummernspalte wird danach neu gezeichnet: der Sprung kann den
    /// sichtbaren Ausschnitt verschoben haben, und die Spalte bemerkt das zwar
    /// an der Klemme, aber nicht, wenn der Ausschnitt schon stimmte und allein
    /// die Auswahl gewandert ist. Der Ruf kostet ein Bild und ist der Vermerk
    /// aus S46.
    fn stelle_zeigen(&self, anfang: usize, ende: usize) {
        let umgerechnet = {
            let modell = self.ivars().modell.borrow();
            koordinaten::in_utf16(modell.stand(), &[anfang, ende])
        };
        let [von, bis] = umgerechnet[..] else {
            return;
        };
        let bereich = NSRange::new(von, bis.saturating_sub(von));
        self.ivars().text.setSelectedRange(bereich);
        self.ivars().text.scrollRangeToVisible(bereich);
        self.nummernspalte_nachziehen();
    }

    /// `cmd+j`: setzt die Schreibmarke an den Anfang der genannten Zeile (C5).
    ///
    /// **Gerechnet wird in `krk_core::text::zeilen` und hier nicht ein zweites
    /// Mal.** Insbesondere die Regel fuer eine Nummer ueber der Zeilenzahl: der
    /// Sprung fuehrt an das Dateiende und meldet den Grund. Sie steht dort
    /// einmal, weil die Textmarke aus C6 dieselbe braucht.
    ///
    /// **Die leere Eingabe meldet nichts und springt nicht.** Sie ist die
    /// Abwesenheit einer Eingabe und kein Fehler, wie der Abbruch des Blattes;
    /// dieselbe Wahl trifft die Pfadeingabe aus C2 der Runde 1. Alles Uebrige,
    /// was keine Zahl ist, bekommt seinen Satz.
    ///
    /// `None` heisst: es gibt nichts zu melden, weil der Sprung eine Zeile
    /// getroffen hat oder gar nicht stattfand.
    pub fn zeile_anspringen(&self, eingabe: &str) -> Option<Editormeldung> {
        let eingabe = eingabe.trim();
        if eingabe.is_empty() {
            return None;
        }
        let Ok(nummer) = eingabe.parse::<usize>() else {
            return Some(Editormeldung::KeineZeilennummer {
                eingabe: eingabe.to_owned(),
            });
        };

        let (sprung, zeilenzahl) = {
            let modell = self.ivars().modell.borrow();
            let index = Zeilenindex::neu(modell.stand());
            (index.anfang_der_zeile(nummer), index.zeilenzahl())
        };
        self.stelle_zeigen(sprung.versatz, sprung.versatz);

        // Vollstaendig und ohne Auffangzweig: eine vierte Lage haelt den Bau an
        // und erzwingt die Antwort auf die Frage, ob sie zu melden ist.
        match sprung.lage {
            Zeilenlage::Getroffen => None,
            Zeilenlage::VorDerErsten => Some(Editormeldung::ZeileVorDerErsten),
            Zeilenlage::HinterDerLetzten => {
                Some(Editormeldung::ZeileHinterDerLetzten { zeilenzahl })
            }
        }
    }

    /// Setzt die Schreibmarke auf die gemerkte Stelle einer Textmarke (C6).
    ///
    /// **Wohin gesprungen wird, entscheidet `krk_core::text::marke` und nicht
    /// diese Zeile.** Dort steht die Regel: erst die gemerkte Nummer, dann der
    /// Vergleich des gemerkten Inhalts, bei Abweichung die Suche im Fenster von
    /// `marke::NAHFENSTER` Zeilen, und bleibt sie ohne Treffer, fuehrt die Marke
    /// **trotzdem** an die gemerkte Nummer. Ein zweiter Rechenweg entsteht hier
    /// nicht, und die Regel fuer eine Nummer ueber der Zeilenzahl kommt
    /// ebenfalls von dort — sie ist dieselbe, die der Zeilensprung aus C5
    /// darueber benutzt.
    ///
    /// **Gesucht wird im gehaltenen Stand und nicht in der Datei auf der
    /// Platte**, wie bei der Suche aus C5: der Editor prueft gegen das, was er
    /// zeigt.
    ///
    /// **Der Aufrufer ist `crate::appkit::anwendung` nach einem gelungenen
    /// Ladevorgang**, also erst dann, wenn die Datei der Marke im Editor steht.
    /// Vor dem Sprung wird sie deshalb hier weder geoeffnet noch geprueft; das
    /// hat `krk_core::text::datei::oeffnen` auf dem einen Weg schon getan, den
    /// [`Self::datei_oeffnen`] fuehrt.
    ///
    /// `None` heisst: es gibt nichts zu melden, weil der Sprung die gemerkte
    /// Stelle wiedergefunden hat.
    pub fn marke_anspringen(&self, zeile: u32, zeileninhalt: &str) -> Option<Editormeldung> {
        let sprung = {
            let modell = self.ivars().modell.borrow();
            marke::wiederfinden(modell.stand(), zeile, zeileninhalt)
        };
        // Ohne Ausdehnung, wie beim Zeilensprung: eine Marke bezeichnet eine
        // Stelle und keinen Bereich.
        self.stelle_zeigen(sprung.sprung.versatz, sprung.sprung.versatz);
        Editormeldung::markenstelle(&sprung)
    }

    /// Der Suchtext des laufenden Suchlaufs und der zuletzt eingetragene
    /// Ersatztext (C5).
    ///
    /// Die beiden Startwerte des Blattes aus S36. Der Suchtext kommt aus dem
    /// Modell, weil dort steht, wonach gesucht wird; laeuft keine Suche, ist er
    /// leer.
    pub fn suchtexte(&self) -> (String, String) {
        let gesucht = self
            .ivars()
            .modell
            .borrow()
            .suchlauf()
            .map(|lauf| lauf.gesucht().to_owned())
            .unwrap_or_default();
        (gesucht, self.ivars().ersatz.borrow().clone())
    }

    /// `cmd+f`: beginnt eine Suche im gehaltenen Stand (C5).
    ///
    /// **Gesucht wird ueber den gehaltenen Stand und nicht ueber die Datei auf
    /// der Platte.** Das neunte Abnahmekriterium von C5 verlangt es, und es
    /// faellt von selbst an: `krk_core::text::suche` nimmt eine Zeichenkette
    /// entgegen und keinen Pfad.
    ///
    /// **Gesucht wird im Text der Datei und nicht in seiner Darstellung**, und
    /// deshalb wirkt die Suche in beiden Ansichten aus C3 gleich. Auch das
    /// faellt von selbst an: die Einfaerbung nach S33 liegt in den
    /// voruebergehenden Merkmalen des Layoutverwalters und fasst den
    /// Textspeicher nicht an.
    ///
    /// Angesteuert wird der erste Treffer ab der Schreibmarke; hinter dem
    /// letzten laeuft die Suche um. Die Regel steht in `krk_core::text::suche`.
    pub fn suche_beginnen(&self, gesucht: &str, ersatz: &str) -> Editormeldung {
        let stelle = self.schreibmarke_in_utf16();
        let treffer = {
            let mut modell = self.ivars().modell.borrow_mut();
            let ab_versatz = koordinaten::in_bytes(modell.stand(), stelle);
            modell.suche_starten(gesucht, ab_versatz)
        };
        *self.ivars().ersatz.borrow_mut() = ersatz.to_owned();
        self.treffer_zeigen(treffer);
        self.suchmeldung()
    }

    /// `cmd+g`: steuert den naechsten Treffer an (C5).
    pub fn weitersuchen(&self) -> Editormeldung {
        self.weiter_mit(Editormodell::weitersuchen)
    }

    /// `ctrl+cmd+g`: steuert den vorigen Treffer an (C5).
    pub fn rueckwaerts_suchen(&self) -> Editormeldung {
        self.weiter_mit(Editormodell::rueckwaerts_suchen)
    }

    /// Die gemeinsame Haelfte der beiden Befehle darueber.
    ///
    /// Sie unterscheiden sich allein in dem Schritt, den sie im Modell tun; der
    /// Umlauf steckt in `krk_core::text::suche` und nicht hier. Derselbe
    /// Zuschnitt wie `Editormodell::weiter_mit` eine Ebene tiefer.
    ///
    /// Ohne Treffer bleibt die Schreibmarke stehen, wie das fuenfte
    /// Abnahmekriterium von C5 es verlangt: [`Self::treffer_zeigen`] fasst die
    /// Flaeche dann nicht an.
    fn weiter_mit(&self, schritt: fn(&mut Editormodell) -> Option<Treffer>) -> Editormeldung {
        let treffer = schritt(&mut self.ivars().modell.borrow_mut());
        self.treffer_zeigen(treffer);
        self.suchmeldung()
    }

    /// `shift+cmd+r`: ersetzt den angesteuerten Treffer und rueckt vor (C5).
    ///
    /// **Ein Ersetzen ist eine ungesicherte Aenderung im Sinne von C4 und
    /// schreibt nicht von sich aus in die Datei.** Das achte Abnahmekriterium
    /// von C5 verlangt es, und es faellt von selbst an: `Editormodell` setzt
    /// die Abweichungsmarke und ruft nicht `sichern`.
    ///
    /// **Der Ersatztext geht durch `krk_core::text::datei::in_gehaltene_form`,
    /// und zwar vor dem Ersetzen.** Das tut `Editormodell` in
    /// `ersetzung_vorbereiten`; hier steht keine zweite Wandlung daneben. Der
    /// Grund, aus dem die Reihenfolge zaehlt, steht im Modulkopf von
    /// `crate::editormodell`: eine Wandlung danach verschoebe jeden Byteversatz
    /// hinter der ersetzten Stelle.
    ///
    /// **Steht kein Treffer an, wird nichts angefasst.** Der Stand geht dann
    /// nicht durch [`Self::stand_erneuern`], und der Rueckgaengigstapel bleibt
    /// stehen; gemeldet wird, warum nichts geschah.
    ///
    /// **Ein `cmd+z` nimmt das Ersetzen zurueck**, seit dem 260810: der Umbau
    /// geht als [`Verlauf::Traegt`] durch die eine Schreibstelle und meldet dort
    /// eine Handlung an, statt den Verlauf zu leeren
    /// (`issues/260810-0303_*_ein-ersetzen-und-ein-eingefuegtes-crlf-verlieren-den-rueckgaengigverlauf.md`).
    /// Die Abschrift des alten Standes entsteht **vor** dem Ruf ins Modell; danach
    /// haelt das Modell den neuen Stand, und es gaebe nichts mehr abzuschreiben.
    /// Der Umkehrpunkt selbst entsteht **danach**, weil er beide Staende braucht,
    /// und haelt allein den ersetzten Bereich; siehe [`Umkehrpunkt`]. Was daran
    /// Nutzerarbeit bleibt, ist die Wirkung im laufenden Buendel — dass ein
    /// `cmd+z` nach einem Ersetzen den vorigen Stand samt Schreibmarke **und den
    /// Suchlauf** zeigt und ein zweites den Anschlag davor —, nicht mehr die
    /// Frage, ob die Handlung angemeldet wird.
    pub fn treffer_ersetzen(&self) -> Editormeldung {
        let steht_an = self
            .ivars()
            .modell
            .borrow()
            .suchlauf()
            .and_then(Suchlauf::angesteuert)
            .is_some();
        if !steht_an {
            return self.suchmeldung();
        }

        let ersatz = self.ivars().ersatz.borrow().clone();
        let auswahl = self.ivars().text.selectedRange();
        let (treffer, punkt) = {
            let mut modell = self.ivars().modell.borrow_mut();
            // Die Abschrift steht vor der Aenderung und faellt am Ende dieses
            // Blocks; in den Stapel geht allein der ersetzte Bereich, und der ist
            // so lang wie der eine Treffer. Siehe [`Umkehrpunkt`].
            let vorher = modell.stand().to_owned();
            let treffer = modell.treffer_ersetzen(&ersatz);
            let punkt = Umkehrpunkt::zwischen(&vorher, modell.stand(), auswahl);
            (treffer, punkt)
        };
        let verlauf = self.verlauf_fuer_umbau(punkt);
        self.stand_erneuern(verlauf);
        self.treffer_zeigen(treffer);
        self.suchmeldung()
    }

    /// `ctrl+cmd+r`: ersetzt alle Treffer in einem Zug und nennt ihre Zahl
    /// (C5).
    ///
    /// Danach steht kein Treffer mehr an; der Suchlauf bleibt mit seinem
    /// Suchtext stehen, damit `cmd+f` ihn wieder anbietet. Ohne laufende Suche
    /// geschieht nichts.
    ///
    /// **Ein `cmd+z` nimmt das Sammelersetzen in einem Zug zurueck**, denselben
    /// Weg wie das einzelne darueber. Hier wiegt es am schwersten: der Befehl
    /// aendert eine ganze Datei auf einen Tastendruck, und genau dort erwartet
    /// ein Nutzer, es zuruecknehmen zu koennen.
    ///
    /// **Ohne Treffer wird nichts abgeschrieben.** Die Trefferzahl steht im
    /// Suchlauf und ist damit vor der Aenderung bekannt; bis zum 260810-1241
    /// entstand die Abschrift trotzdem, und ein `ctrl+cmd+r` auf einen Suchlauf
    /// ohne Treffer kopierte an einer Datei von 16 MB 16 MB und warf sie fort. Die
    /// Zahl im Suchlauf ist keine zweite Wahrheit ueber die Treffer, sondern die
    /// erste: `Editormodell` bildet die Liste nach jeder Aenderung im **neuen**
    /// Stand neu oder beendet den Lauf.
    ///
    /// # Der Fall, in dem ein Bereich so gross ist wie die Datei
    ///
    /// **Der geaenderte Bereich hilft hier nicht immer.** Enthaelt der Ersatztext
    /// den Suchtext, findet der naechste Ruf wieder Treffer, und der Bereich
    /// zwischen dem ersten und dem letzten deckt beinahe die ganze Datei. Wer `a`
    /// durch `aa` ersetzt und den Befehl wiederholt, legt je Ruf eine
    /// Dateigroesse ab
    /// (`issues/260810-1314_*_ein-wiederholtes-sammelersetzen-legt-je-ruf-einen-bereich-in-dateigroesse-in-den-stapel.md`).
    ///
    /// **Der Bereich bleibt trotzdem einer und wird keine Liste.** Eine Liste der
    /// einzelnen Stellen waere in der Groesse des **Ersetzten** statt in der des
    /// Bereichs, und das klingt kleiner, als es an diesem Fall ist: sie kostet je
    /// Stelle einen Versatz. Gerechnet an derselben Datei von 16 MB, Suchtext `a`,
    /// Ersatztext `aa`:
    ///
    /// ```text
    ///   Abstand der Treffer   Treffer     ein Bereich   eine Liste (8 B je Stelle)
    ///           16 Bytes      1 048 576      16,0 MB       8,0 MB
    ///            8 Bytes      2 097 152      16,0 MB      16,0 MB
    ///            4 Bytes      4 194 304      16,0 MB      32,0 MB
    /// ```
    ///
    /// Der Umschlag liegt bei einem Treffer je acht Bytes, und darunter ist die
    /// Liste teurer als der Bereich. Die Liste ist dabei in ihrer guenstigsten
    /// Form gerechnet — nur die Versaetze, Such- und Ersatztext einmal daneben —,
    /// und nicht in der, die ein `Vec<Umkehrpunkt>` haette. Sie loeste den Fall also nicht, sondern
    /// verschoebe ihn — und sie kostete dabei, was die Stellen von
    /// `krk_core::text::suche` bis hierher zu tragen kostet, also eine zweite
    /// Wahrheit darueber, was ein Ersetzen geaendert hat. Siehe [`Umkehrpunkt`].
    ///
    /// **Gedeckelt ist der Fall an der Summe und nicht am einzelnen Bereich**,
    /// ueber [`Self::verlauf_fuer_umbau`] und [`STAPELBUDGET`]: der zweite Ruf
    /// raeumt, was der erste hinterlassen hat, und der Stapel haelt danach eine
    /// Dateigroesse statt so vieler, wie der Nutzer Rufe abgibt. Was der Nutzer
    /// davon merkt: das erste `cmd+z` nimmt das letzte Sammelersetzen zurueck, ein
    /// zweites tut nichts.
    pub fn alle_treffer_ersetzen(&self) -> Editormeldung {
        let Some(anstehend) = self.ivars().modell.borrow().suchlauf().map(Suchlauf::zahl) else {
            return Editormeldung::KeineSuche;
        };
        if anstehend == 0 {
            return Editormeldung::Ersetzt { zahl: 0 };
        }

        let ersatz = self.ivars().ersatz.borrow().clone();
        let auswahl = self.ivars().text.selectedRange();
        let (zahl, punkt) = {
            let mut modell = self.ivars().modell.borrow_mut();
            // Die Abschrift steht vor der Aenderung und faellt am Ende dieses
            // Blocks; in den Stapel geht der Bereich vom ersten bis zum letzten
            // ersetzten Treffer. Siehe [`Umkehrpunkt`].
            let vorher = modell.stand().to_owned();
            let zahl = modell.alle_treffer_ersetzen(&ersatz);
            let punkt = Umkehrpunkt::zwischen(&vorher, modell.stand(), auswahl);
            (zahl, punkt)
        };
        // Ohne Treffer hat sich der Stand nicht bewegt, und die Flaeche neu zu
        // beschreiben kostete den Rueckgaengigverlauf fuer nichts. Erreichbar ist
        // der Zweig nach der Abfrage oben nicht mehr: `Suchlauf::zahl` und
        // `suche::alle_ersetzen` zaehlen mit **derselben** Funktion
        // (`suche::alle`) im **selben** Stand, also ist `zahl` gleich
        // `anstehend`. Er bleibt stehen, weil sein Preis eine angemeldete
        // Handlung ohne Wirkung waere, und die nimmt der Nutzer mit einem `cmd+z`
        // ins Leere hin.
        if zahl > 0 {
            let verlauf = self.verlauf_fuer_umbau(punkt);
            self.stand_erneuern(verlauf);
        }
        Editormeldung::Ersetzt { zahl }
    }

    /// Waehlt den angesteuerten Treffer in der Flaeche aus, falls es einen
    /// gibt.
    ///
    /// Ohne Treffer geschieht nichts, und die Schreibmarke bleibt stehen; das
    /// fuenfte Abnahmekriterium von C5 verlangt genau das.
    fn treffer_zeigen(&self, treffer: Option<Treffer>) {
        if let Some(treffer) = treffer {
            self.stelle_zeigen(treffer.anfang, treffer.ende);
        }
    }

    /// Der Satz ueber den laufenden Suchlauf (C5).
    ///
    /// **Gebaut wird er im Modell**, weil dort steht, wie viele Treffer es gibt
    /// und der wievielte ansteht; diese Funktion waehlt allein zwischen "es
    /// laeuft eine Suche" und "es laeuft keine".
    fn suchmeldung(&self) -> Editormeldung {
        match self.ivars().modell.borrow().suchlauf() {
            Some(lauf) => Editormeldung::Suchstand {
                satz: lauf.meldung(),
            },
            None => Editormeldung::KeineSuche,
        }
    }

    // ------------------------------------------------------------------
    // Die beiden Ansichten (C3)
    // ------------------------------------------------------------------

    /// Wechselt zwischen Rohansicht und Formatansicht (C3).
    ///
    /// **Der ganze Wechsel ist ein Ruf ins Modell und ein Nachziehen der
    /// Darstellung.** Der Textspeicher wird dabei nicht angefasst, und deshalb
    /// kann der Wechsel nichts verlieren: es gibt keinen zweiten Textbestand, in
    /// den etwas verlorengehen koennte, und
    /// [`Editormodell::ansicht_umschalten`] fasst weder den Stand noch die
    /// Abweichungsmarke an. Das zehnte Abnahmekriterium von C3 ist damit eine
    /// Eigenschaft der Bauart und keine Zusage der Sorgfalt.
    ///
    /// **Die Schreibmarke bleibt, wo sie ist**, und zwar ohne eigenen Bau: sie
    /// haengt an Zeichenstellen des Textspeichers, und der bleibt Zeichen fuer
    /// Zeichen derselbe. Das elfte Abnahmekriterium von C3 faellt daraus an.
    pub fn ansicht_umschalten(&self) {
        self.ivars().modell.borrow_mut().ansicht_umschalten();
        self.darstellung_nachziehen();
    }

    /// Setzt Grundschrift, Umbruch und Merkmale auf die gewaehlte Ansicht (C3).
    ///
    /// **Die eine Stelle, an der die Darstellung entsteht**, und sie kennt vier
    /// Aufrufer: den Aufbau, ein gelungenes Oeffnen, das Schliessen und den
    /// Ansichtswechsel. Alle vier stellen dieselbe Frage — welche Ansicht,
    /// welche Datei —, und eine zweite Stelle daneben waere die erste
    /// Gelegenheit, sie verschieden zu beantworten.
    ///
    /// Die drei Sachen, die sich aendern, stehen in `### Frage 7` des Plans: die
    /// Schrift, der Umbruch und die Merkmale. Sie werden hier in dieser
    /// Reihenfolge gesetzt, und die Reihenfolge zaehlt: `setFont:` und
    /// `setTextColor:` einer `NSTextView` schreiben ueber den **ganzen**
    /// Textspeicher, also muessen sie vor den Auszeichnungen stehen, die
    /// einzelne Stellen davon ueberschreiben.
    ///
    /// **Die Einfaerbung kommt nicht von hier, sondern spaeter.** Sie laeuft auf
    /// einem Arbeitsfaden (0,3 MB/s, gemessen; siehe `crate::hervorhebung`), und
    /// diese Funktion fordert sie nur an. Bis sie eintrifft, steht der Text in
    /// der Grundfarbe da — dieselbe Spanne, die schon beim Lesen einer grossen
    /// Datei vergeht, und aus demselben Grund.
    fn darstellung_nachziehen(&self) {
        let (ansicht, art) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.ansicht(),
                crate::hervorhebung::art(modell.pfad(), modell.typ()),
            )
        };

        self.grundschrift_setzen(ansicht, art);
        self.umbruch_setzen(ansicht == Ansicht::Format);
        textmerkmale::zuruecksetzen(&self.ivars().text, ansicht, art);

        match ansicht {
            Ansicht::Format => self.einfaerbung_anfordern(),
            // Die Rohansicht zeigt die Zeichen ohne Einfaerbung; ein laufender
            // Faden hat nichts mehr abzuliefern und faellt mit seinem
            // Empfaenger.
            Ansicht::Roh => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                *self.ivars().einfaerbungsstand.borrow_mut() = None;
                self.ivars().einfaerbung_erneut.set(false);
            }
        }

        self.nummernspalte_nachziehen();
    }

    /// Setzt die Grundschrift der Flaeche und damit auch die des naechsten
    /// Anschlags (C3).
    ///
    /// **Eine Regel und keine drei.** Fest geschrieben wird, was Zeichen fuer
    /// Zeichen gelesen wird: die Rohansicht immer, und die Formatansicht bei
    /// Code. Alles Uebrige — einfacher Text und Markdown — bekommt die
    /// Systemschrift mit dem Lesezuschlag aus [`super::textmerkmale`]. Das ist
    /// die "lesbare Schriftgroesse", die C3 fuer einfachen Text zusagt, und
    /// zugleich die Grundschrift, ueber der die Markdown-Ueberschriften ihre
    /// Stufen haben.
    ///
    /// `setFont:` schreibt ueber den ganzen Textspeicher **und** setzt die
    /// Merkmale des naechsten Anschlags. Beides ist gewollt: ohne das zweite
    /// truege ein neu getipptes Zeichen die Schrift der vorigen Ansicht.
    fn grundschrift_setzen(&self, ansicht: Ansicht, art: Darstellungsart) {
        self.ivars()
            .text
            .setFont(Some(&textmerkmale::grundschrift(ansicht, art)));
        // Die Systemfarbe und nicht die der Tafel: sie loest sich in Hell wie in
        // Dunkel gegen den Grund der Flaeche auf, und der Grund bleibt nach S34
        // die Systemfarbe. Aus der Tafel kommen allein die Vordergrundfarben
        // der Wortarten, und die setzt die Einfaerbung darueber.
        self.ivars().text.setTextColor(Some(&NSColor::textColor()));
    }

    /// Schaltet den Umbruch am Fensterrand ein oder aus (C3).
    ///
    /// Die Rohansicht zeigt die Zeichen der Datei, also auch ihre Zeilenlaengen:
    /// ohne Umbruch und mit einem waagerechten Schieber. Die Formatansicht
    /// bricht am Fensterrand um, wie C3 es fuer einfachen Text ausdruecklich
    /// zusagt und wie es fuer die beiden anderen Besetzungen ebenso gilt.
    ///
    /// **Der Rahmen der Flaeche wird beim Einschalten zurueckgesetzt.** In der
    /// Rohansicht waechst sie mit der laengsten Zeile; bliebe sie so breit,
    /// laege der Umbruchrand ausserhalb des Sichtbaren, und der Umbruch griffe
    /// erst beim naechsten Auslegen aus einem anderen Anlass.
    fn umbruch_setzen(&self, umbruch: bool) {
        let text = &self.ivars().text;
        let Some(rolle) = text.enclosingScrollView() else {
            return;
        };
        let breite = rolle.contentSize().width;
        rolle.setHasHorizontalScroller(!umbruch);
        text.setHorizontallyResizable(!umbruch);
        // SAFETY: Der Behaelter wird von der Flaeche selbst mitgebracht und hier
        // nur eingestellt; kein fremdes Objekt wird gehalten.
        if let Some(behaelter) = unsafe { text.textContainer() } {
            behaelter.setWidthTracksTextView(umbruch);
            behaelter.setContainerSize(if umbruch {
                NSSize::new(breite, f64::MAX)
            } else {
                NSSize::new(f64::MAX, f64::MAX)
            });
        }
        if umbruch {
            let hoehe = text.frame().size.height;
            text.setFrameSize(NSSize::new(breite, hoehe));
        }
    }

    /// Fordert eine Einfaerbung des gehaltenen Standes an (C3).
    ///
    /// **Hoechstens ein Faden zur Zeit.** Laeuft schon einer, wird kein zweiter
    /// gestartet, sondern nur vermerkt, dass sein Ergebnis ueberholt sein wird;
    /// er wird dann nach seiner Rueckkehr sofort wiederholt. Damit kostet ein
    /// Tastendruck waehrend eines laufenden Laufs nichts, und der Nutzer bekommt
    /// die Einfaerbung des letzten Standes statt der jedes Zwischenstandes.
    ///
    /// **Die Rohansicht fordert nie an**, und die Abfrage steht hier und nicht
    /// bei den drei Aufrufern. Sie zeigt die Zeichen der Datei ohne
    /// Einfaerbung; eine Anfrage von dort brachte eine Lieferung zurueck, und
    /// [`Self::formatierung_anwenden`] faerbte die Rohansicht ein. Der Weg ist
    /// erreichbar, seit [`Self::text_zurueckschreiben`] bei jedem Anschlag
    /// anfordert — dort wird nicht nach der Ansicht gefragt, sondern gemeldet,
    /// dass sich der Text geaendert hat.
    ///
    /// Ohne gehaltene Datei geschieht ebenso nichts: es gibt keinen Pfad, an dem
    /// die Kiste eine Sprache erkennen koennte, und nichts einzufaerben. Der
    /// aufgehobene Stand faellt dann mit, weil er zu einer Datei gehoerte, die
    /// der Editor nicht mehr haelt.
    ///
    /// **Der aufgehobene Stand wandert in den Lauf hinein.** Er ist die Vorlage,
    /// aus der [`crate::hervorhebung::fortschreiben`] den unveraenderten Anfang
    /// und den unveraenderten Schwanz uebernimmt; ohne ihn kostete jeder
    /// Anschlag einen vollen Durchgang. Wo er zwischen zwei Laeufen wohnt und
    /// wann er faellt, steht an [`EditorIvars::einfaerbungsstand`].
    fn einfaerbung_anfordern(&self) {
        if self.ivars().einfaerbung.borrow().is_some() {
            self.ivars().einfaerbung_erneut.set(true);
            return;
        }
        let angaben = {
            let modell = self.ivars().modell.borrow();
            if !modell.haelt_datei() || modell.ansicht() != Ansicht::Format {
                None
            } else {
                Some((
                    modell.stand().to_owned(),
                    modell.pfad().map(Path::to_path_buf),
                    modell.typ(),
                ))
            }
        };
        let Some((stand, pfad, typ)) = angaben else {
            if !self.ivars().modell.borrow().haelt_datei() {
                *self.ivars().einfaerbungsstand.borrow_mut() = None;
            }
            return;
        };
        let vorlage = self.ivars().einfaerbungsstand.borrow_mut().take();
        let vorgang =
            Einfaerbungsvorgang::starten(vorlage, stand, pfad, typ, self.ivars().tafel.get());
        *self.ivars().einfaerbung.borrow_mut() = Some(vorgang);
        self.ivars().einfaerbung_erneut.set(false);
        self.takt_starten();
    }

    /// Holt die Meldung des Einfaerbungsfadens ab (C3).
    ///
    /// **Ein ueberholtes Ergebnis wird nicht angewendet, sondern fallengelassen
    /// und sofort neu angefordert.** Es waere nicht nur veraltet: seine Bereiche
    /// zeigten in einen Text, der inzwischen kuerzer sein kann, und ein
    /// `NSRange` hinter dem Text beantwortet AppKit mit einer
    /// Objective-C-Ausnahme. Die ist in Rust nicht zu fangen und beendet das
    /// Programm.
    ///
    /// **Fallen laesst es allein seine Formatierung, nicht seinen aufgehobenen
    /// Stand.** Der Stand beschreibt einen Text, der wirklich gerechnet worden
    /// ist, und ist damit auch fuer den ueberholten Fall die richtige Vorlage:
    /// der naechste Lauf schreibt von ihm auf den heutigen Text fort. Ihn
    /// zusammen mit der Formatierung wegzuwerfen kostete jeden zweiten Anschlag
    /// einen vollen Durchgang.
    ///
    /// **Angewendet wird aus einer eigenen Bindung und nicht aus der Zelle
    /// heraus.** [`Self::formatierung_anwenden`] ruft in das Textsystem, und ein
    /// Weg von dort hierher zurueck nahme eine zweite Ausleihe derselben Zelle;
    /// dieselbe Regel wie bei jeder anderen Ausleihe in dieser Datei.
    fn einfaerbung_einziehen(&self) {
        let abholung = {
            let vorgang = self.ivars().einfaerbung.borrow();
            match vorgang.as_ref() {
                Some(vorgang) => vorgang.abholen(),
                None => return,
            }
        };
        match abholung {
            Abholung::Laeuft => {}
            // Der Faden ist ohne Meldung gefallen; darauf zu warten hat keinen
            // Sinn mehr. Derselbe Zweig und derselbe Grund wie beim Lesevorgang.
            // Der Faden ist mit der Vorlage gefallen; der naechste Lauf rechnet
            // deshalb von vorn.
            Abholung::Weggefallen => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                self.ivars().einfaerbung_erneut.set(false);
            }
            Abholung::Fertig(stand) => {
                *self.ivars().einfaerbung.borrow_mut() = None;
                let stand = *stand;
                let ueberholt = self.ivars().einfaerbung_erneut.replace(false);
                if !ueberholt {
                    self.formatierung_anwenden(stand.formatierung());
                }
                *self.ivars().einfaerbungsstand.borrow_mut() = Some(stand);
                if ueberholt {
                    self.einfaerbung_anfordern();
                }
            }
        }
    }

    /// Traegt eine fertige Formatierung in die Flaeche (C3).
    ///
    /// **Drei Schritte und kein vierter.** Die Umsetzung selbst wohnt in
    /// [`super::textmerkmale`], weil Editor und Vorschau dieselbe brauchen und
    /// zwei davon zwei Wahrheiten waeren; hier steht allein, was diese Flaeche
    /// beisteuert. Erst die beiden Angaben aus dem Modell, und die Ausleihe
    /// endet **vor** dem Ruf: die Umsetzung ruft in das Textsystem, und ein
    /// gehaltener `RefCell`-Ausleihschein waere die erste Gelegenheit fuer einen
    /// Programmabbruch, wenn AppKit auf dem Weg zurueckmeldet.
    ///
    /// **Nachgezogen wird nur, wenn gesetzt wurde.** Die Auszeichnungen aendern
    /// die Zeilenkaesten, und die Nummern stuenden sonst neben dem zuletzt
    /// gezeichneten Umbruch. Hat der Guertel in
    /// [`super::textmerkmale::anwenden`] die Lieferung abgewiesen, ist an der
    /// Flaeche nichts geschehen und nichts nachzuziehen.
    fn formatierung_anwenden(&self, formatierung: &Formatierung) {
        let (ansicht, art) = {
            let modell = self.ivars().modell.borrow();
            (
                modell.ansicht(),
                crate::hervorhebung::art(modell.pfad(), modell.typ()),
            )
        };
        if textmerkmale::anwenden(&self.ivars().text, formatierung, art, ansicht) {
            self.nummernspalte_nachziehen();
        }
    }

    /// Zieht die Farbtafel auf das gewechselte Erscheinungsbild nach (S34).
    ///
    /// Gerufen von [`Editorsicht`], der einen Stelle, an der AppKit den Wechsel
    /// meldet. Hat sich die Tafel nicht geaendert, geschieht nichts: die Meldung
    /// kommt auch bei Wechseln, die Hell und Dunkel nicht betreffen, und ein
    /// Einfaerbungslauf ueber eine Datei von 16 MB ist kein Preis fuer nichts.
    ///
    /// Ob ueberhaupt einzufaerben ist, fragt
    /// [`Self::einfaerbung_anfordern`] und nicht diese Stelle; die Antwort
    /// steht dort einmal.
    fn erscheinung_nachziehen(&self) {
        let neue = textmerkmale::tafel_der_erscheinung(&self.ivars().bereich);
        if neue == self.ivars().tafel.get() {
            return;
        }
        self.ivars().tafel.set(neue);
        self.einfaerbung_anfordern();
    }

    /// Laesst die Nummernspalte neu zeichnen.
    ///
    /// Umbruch und Schrift aendern die Zeilenkaesten des Layoutverwalters, ohne
    /// dass der Textspeicher eine Meldung verschickt, an der die Spalte es
    /// bemerken koennte; ohne diesen Ruf zeigte die Formatansicht die Nummern
    /// des zuletzt gezeichneten Umbruchs, und das fuenfte Abnahmekriterium von
    /// C10 waere gebrochen. Der Vermerk stammt aus S46.
    fn nummernspalte_nachziehen(&self) {
        if let Some(rolle) = self.ivars().text.enclosingScrollView() {
            nummernspalte::spalte_neu_zeichnen(&rolle);
        }
    }
}

/// Was am Kopf des Editorbereichs steht (C4).
///
/// **Eine reine Funktion, damit die Anzeige des ungesicherten Standes ohne
/// Fenster abzunehmen ist.** Sie bekommt die beiden Angaben und gibt die Zeile;
/// woher die Angaben kommen und wohin die Zeile geht, steht in
/// [`Editorbereich::kopf_nachziehen`].
///
/// **Der Name und nicht der Pfad.** Der volle Pfad steht seit S48 im
/// Fenstertitel, solange der Fokus im Editor steht; ihn hier zu wiederholen
/// braechte zwei Anzeigen derselben Angabe und liesse in einem schmalen Editor
/// den Namen als erstes wegfallen. Ein Pfad ohne letzten Bestandteil ist auf
/// dem Mac kein Ziel des Editors; kaeme trotzdem einer, steht er ganz da, statt
/// dass der Kopf leer bliebe.
///
/// Ohne Datei bleibt der Kopf leer: der Editor zeigt dann nichts, was einen
/// Namen haette, und ein Platzhalter waere ein Wort ueber ein Nichts.
fn kopfzeile(pfad: Option<&Path>, ungesichert: bool) -> String {
    let Some(pfad) = pfad else {
        return String::new();
    };
    let name = pfad.file_name().map_or_else(
        || pfad.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    if ungesichert {
        format!("{ABWEICHUNGSZEICHEN} {name}")
    } else {
        name
    }
}

/// Baut den Kopf: eine einzeilige Beschriftung ueber der Textflaeche.
///
/// **Dieselben beiden Masse wie die Statuszeile der Dateifenster**, Hoehe und
/// Einzug, und aus demselben Grund: es ist dieselbe Form, naemlich eine Zeile
/// in der kleinen Systemschrift am Rand eines Bereichs. Zwei eigene Zahlen
/// daneben waeren zwei Antworten auf dieselbe Frage, und der Nutzer saehe zwei
/// verschieden hohe Streifen nebeneinander.
///
/// Die Farbe ist die zurueckgenommene Beschriftungsfarbe, wie bei der
/// Statuszeile ohne Meldung: der Kopf ist eine Angabe und keine Warnung. Das
/// Abweichungszeichen traegt die Aussage, nicht die Farbe; damit haengt sie
/// nicht am Farbsehen.
fn kopf_bauen(mtm: MainThreadMarker) -> Retained<NSTextField> {
    let kopf = NSTextField::labelWithString(ns_string!(""), mtm);
    kopf.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    kopf.setTextColor(Some(&NSColor::secondaryLabelColor()));
    kopf.setAlignment(NSTextAlignment::Left);
    kopf.setMaximumNumberOfLines(1);
    // Am oberen Rand festgemacht, in der Breite mitwachsend: der Abstand nach
    // unten ist beweglich, der nach oben nicht.
    kopf.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
    );
    kopf
}

/// Baut die Textflaeche: eine editierbare `NSTextView` in einer
/// Bildlaufansicht.
///
/// Editierbar und auswaehlbar, waehrend die Textanzeige der Vorschau
/// (`super::vorschau`) seit der Runde 14 auswaehlbar, aber nicht bearbeitbar
/// ist. **Der Unterschied ist damit nur noch einer**, und es ist der, der die
/// beiden Flaechen unterscheidet: der Editor bearbeitet, die Vorschau zeigt.
///
/// Beide nehmen damit den Fokus als Textsystem — der Editor, weil er ihn
/// braucht, die Vorschau, weil eine auswaehlbare Flaeche ihn nimmt und die
/// Runde 14 das in Kauf genommen hat. Der Fokusvorbehalt laesst deshalb beide
/// ueber die Naemlichkeitsfrage aus dem Modulkopf durch; bis zu jener Runde war
/// die Textflaeche des Editors die einzige, die er so durchliess.
///
/// Die Schrift ist die feste Schreibmaschinenschrift des Nutzers in
/// Systemgroesse; die Mindestbreite des Bereichs (320 Punkte) ist an ihr
/// gerechnet. Welche Schrift die Formatansicht aus C3 setzt, entscheidet ein
/// spaeterer Schritt.
fn textflaeche_bauen(
    mtm: MainThreadMarker,
    rahmen: NSRect,
) -> (Retained<NSScrollView>, Retained<NSTextView>) {
    let rolle = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), rahmen);
    rolle.setHasVerticalScroller(true);
    rolle.setAutohidesScrollers(true);
    rolle.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    let text = NSTextView::initWithFrame(NSTextView::alloc(mtm), rahmen);
    text.setEditable(true);
    text.setSelectable(true);
    // Reiner Text, und die sieben Automatiken aus: der gesicherte Stand ist der
    // getippte. Der Grund steht im Modulkopf, die Zeilen selbst seit der Runde 9
    // in `super::textautomatik` — die eine Antwort fuer die beiden bearbeitbaren
    // Flaechen dieses Programms, den Editor und den Notizzettel.
    textautomatik::automatiken_abschalten(&text);
    // Ohne diese Zeile traegt die Textansicht keine einzige
    // Rueckgaengig-Handlung, und die beiden Menueeintraege aus S7 finden am
    // Ende der Antwortkette einen leeren Verwalter vor. `allowsUndo` steht bei
    // einer programmatisch erzeugten `NSTextView` ab Werk auf `NO`; die
    // Menueseite derselben Sache steht in `super::menue`.
    text.setAllowsUndo(true);
    // Und ohne diese Zeile kommt keine dieser Handlungen im Modell an. Der
    // Rueckweg aus der Flaeche ist `textDidChange:`, und eine `NSTextView` auf
    // TextKit 2 verschickt es bei einem `undo` **nicht**; auf TextKit 1 verschickt
    // sie genau eines, mit dem schon zurueckgenommenen Text. Gemessen am 260810 auf
    // macOS 15.7.7 (Build 24G720), dreimal reproduziert, einziger Unterschied der
    // Zugriff auf `layoutManager`. Der Zugriff ist der Umschalter: er laesst AppKit
    // auf den aelteren `NSLayoutManager` zurueckfallen.
    //
    // Bis zum 260810-1243 stand hier keine Zeile, und der Rueckfall entstand als
    // **Nebenwirkung** von `textmerkmale::zuruecksetzen` und der Nummernspalte, die den
    // Verwalter aus einem anderen Grund anfassen. Wer die Nummernspalte auf
    // `NSTextLayoutManager` nachzieht — der Weg, den Apple fuer neue Arbeit
    // vorsieht —, bekaeme einen gruenen Bau, gruene Proben und ein `cmd+s`, das
    // den zurueckgenommenen Text sichert. Deshalb steht der Rueckfall hier als
    // Absicht mit ihrem Grund und nicht dort als Folge
    // (`issues/260810-1243_*_dass-ein-cmd-z-ueberhaupt-im-modell-ankommt-haengt-an-textkit-1-und-das-steht-nirgends-als-tragend.md`).
    //
    // SAFETY: Der Verwalter gehoert der Flaeche, die ihn hier selbst anlegt; er
    // wird nur erfragt und nicht gehalten.
    let _ = unsafe { text.layoutManager() };
    text.setVerticallyResizable(true);
    text.setHorizontallyResizable(false);
    text.setMinSize(NSSize::ZERO);
    text.setMaxSize(NSSize::new(f64::MAX, f64::MAX));
    text.setAutoresizingMask(NSAutoresizingMaskOptions::ViewWidthSizable);
    if let Some(schrift) = NSFont::userFixedPitchFontOfSize(NSFont::systemFontSize()) {
        text.setFont(Some(&schrift));
    }
    rolle.setDocumentView(Some(&text));
    // Die Nummernspalte aus C10, dieselbe Klasse, die die Vorschau einhaengt.
    // Sie steht im Editor immer: der Spec laesst sie nicht abschalten, und der
    // Editor zeigt ausschliesslich den Inhalt einer Datei.
    Nummernspalte::einhaengen(mtm, &rolle, &text);
    (rolle, text)
}

/// Leert Rueckgaengig- und Wiederherstellungsstapel eines Verwalters.
///
/// **Der Aufrufer ist einer**, [`Editorbereich::stand_einsetzen`], und der Grund
/// steht dort. Hier steht, was die Funktion vom Verwalter annimmt und was sie
/// von ihm nicht annimmt.
///
/// **`None` ist keine Ausnahme, sondern der Normalfall vor dem Einhaengen.**
/// `NSResponder::undoManager` geht die Antwortkette hinauf und findet den
/// Verwalter erst, wenn die Flaeche in einem Fenster steht; der erste Aufruf
/// aus [`Editorbereich::bauen`] kommt davor. Dort ist nichts zu leeren, weil die
/// Flaeche noch leer ist.
///
/// **Eine offene Gruppe haelt sie nicht auf.** `setString:` faellt mitten in die
/// Ereignisbehandlung, und `NSUndoManager` gruppiert ab Werk je Ereignis: zur
/// Aufrufzeit kann eine Gruppe offen stehen. `removeAllActions` raeumt beide
/// Stapel **und** die offene Gruppe ab; `endUndoGrouping` an derselben Stelle
/// wuerde ohne offene Gruppe eine Ausnahme werfen. **Drei Pruefungen messen
/// das, und die Arbeitsteilung unter ihnen gehoert dazu**, weil sie bis zum
/// 260810 eine Luecke hatte
/// (`issues/260810-0420_*_die-beiden-rueckgaengigproben-schalten-die-betriebsart-ab-die-sie-messen-sollen.md`):
/// die beiden `…_traegt_keine_rueckgaengig_handlung_mehr` und
/// `…_traegt_auch_eine_offene_gruppe_nicht_mehr` messen eine **von Hand**
/// geoeffnete Gruppe bei abgeschalteter Ereignisgruppierung, und
/// `ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung` misst die
/// Betriebsart der Laufzeit, also `groupsByEvent = true` samt einem Umlauf der
/// Laufschleife danach.
///
/// **Der Verwalter gehoert dem Fenster und nicht der Textflaeche.** Wer sonst
/// noch in demselben Fenster Rueckgaengig-Handlungen anmeldet, verliert sie
/// hier mit. **Heute ist das niemand, und das ist gemessen und nicht
/// geschlossen** — am 260810 auf macOS 15.7.7 (Build 24G720), nachdem die
/// Begruendung an dieser Stelle ueber die falsche Flaeche gefuehrt worden war
/// (`issues/260810-0419_*_der-rueckgaengigverwalter-des-fensters-traegt-auch-den-feldeditor-der-umbenennung.md`).
/// Die Flaeche, um die es geht, ist nicht das Suchfeld, sondern das
/// beschreibbare Namensfeld der Umbenennung aus C4 der Runde 1
/// (`super::tabelle`, `feld.setEditable(true)`); ein `NSTextField` bekommt beim
/// Bearbeiten den Feldeditor des **Fensters**, und der ist selbst eine
/// `NSTextView`. Gemessen wurde an einem Fenster mit beidem darin:
///
/// ```text
///   Verwalter des Feldeditors      NSCellUndoManager   ─┐ zwei Objekte,
///   Verwalter des Fensters         NSUndoManager       ─┘ nicht dasselbe
///   removeAllActions am Fenster ─> Feldeditor: canUndo bleibt wahr
///   undo im Feld danach         ─> der getippte Name ist zurueckgenommen
/// ```
///
/// Der Feldeditor bekommt seinen Verwalter also von der `NSTextField`, die ihn
/// ausleiht, und nicht aus der Antwortkette. Damit ist die Textflaeche des
/// Editors die einzige in KRK, die diesen Verwalter benutzt, und dieser Ruf
/// nimmt niemandem etwas fort.
///
/// **Die Grenze liesse sich enger ziehen, und sie ist es nicht.**
/// `undoManagerForTextView:` am Delegierten gaebe der Flaeche einen **eigenen**
/// Verwalter, und dass er vom Menue aus erreichbar bliebe, ist mitgemessen:
/// `undo:` beantwortet in der ganzen Antwortkette allein `NSWindow` — nicht
/// `NSTextView`, nicht `NSApplication`, nicht `NSResponder` —, und `NSWindow`
/// nimmt dabei den Verwalter des **Ersthelfers** und nicht seinen eigenen. Der
/// Weg steht damit offen, wird aber nicht genommen: es gibt keinen zweiten
/// Anmelder, und ein Verwalter mehr waere ein Mechanismus ohne Fall.
fn rueckgaengigstapel_leeren(verwalter: Option<&NSUndoManager>) {
    if let Some(verwalter) = verwalter {
        verwalter.removeAllActions();
    }
}

/// Die Meldungen sind reine Werte und brauchen kein Fenster; deshalb stehen die
/// Pruefungen hier und nicht unter `Nutzerarbeit`.
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::ffi::{CStr, CString, c_uint};
    use std::io::Write;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Mutex;

    use krk_core::text::marke::wiederfinden;
    use krk_core::text::suche;
    use objc2::runtime::{AnyClass, AnyProtocol, Sel};
    use objc2_app_kit::{NSTextInputTraitType, NSWritingToolsBehavior};
    use objc2_foundation::NSNumber;

    use super::*;
    // Die lesende Seite der Merkmalsfrage bildet denselben Setzernamen wie die
    // setzende; seit der Runde 9 wohnt er in `super::super::textautomatik`.
    use super::super::textautomatik::setzername;

    fn pfad() -> PathBuf {
        PathBuf::from("/tmp/probe.txt")
    }

    /// Das zehnte Abnahmekriterium von C2: jede Abweisung nennt ihren Grund,
    /// und "zu gross" ist von "nicht als Text lesbar" zu unterscheiden.
    #[test]
    fn die_drei_abweisungsgruende_tragen_drei_verschiedene_saetze() {
        let saetze = [
            Editormeldung::Abgewiesen(Abweisung::KeinGueltigesZiel {
                pfad: pfad(),
                grund: "ein Ordner".into(),
            })
            .text(),
            Editormeldung::Abgewiesen(Abweisung::ZuGross {
                pfad: pfad(),
                groesse: 20 * 1024 * 1024,
            })
            .text(),
            Editormeldung::Abgewiesen(Abweisung::NichtAlsTextLesbar { pfad: pfad() }).text(),
        ];
        for satz in &saetze {
            assert!(
                !satz.is_empty(),
                "kommentarlos nichts zu sagen ist unzulässig"
            );
        }
        assert_ne!(saetze[0], saetze[1]);
        assert_ne!(saetze[1], saetze[2]);
        assert_ne!(saetze[0], saetze[2]);
    }

    /// Das achte Abnahmekriterium von C6: nur der Fehlschlag meldet, und er
    /// meldet, dass die Stelle sich geaendert hat.
    #[test]
    fn allein_die_nicht_wiedergefundene_markenstelle_meldet_sich() {
        let text = "eins\nzwei\ndrei\n";
        // Der gemerkte Inhalt steht auf der gemerkten Nummer.
        assert_eq!(
            Editormeldung::markenstelle(&wiederfinden(text, 2, "zwei")),
            None
        );
        // Er steht daneben und wird im Fenster wiedergefunden.
        assert_eq!(
            Editormeldung::markenstelle(&wiederfinden(text, 1, "drei")),
            None
        );
        // Er ist fort: die Marke fuehrt trotzdem, und der Sprung meldet sich.
        let sprung = wiederfinden(text, 2, "vier");
        assert_eq!(sprung.fund, Fund::NichtGefunden);
        let meldung = Editormeldung::markenstelle(&sprung)
            .expect("ein nicht wiedergefundener Inhalt meldet sich");
        assert_eq!(
            meldung,
            Editormeldung::MarkenstelleGeaendert {
                zeile: 2,
                lage: Zeilenlage::Getroffen,
            }
        );
        assert!(
            meldung.text().contains('2'),
            "die Meldung nennt die Zeile, an die sie geführt hat"
        );
    }

    /// Der zusammengesetzte Fall aus
    /// `issues/260809-1631_*_ein-markensprung-kann-zwei-meldungen-zugleich-haben-und-die-zeile-traegt-eine.md`:
    /// die gemerkte Stelle ist fort **und** die Datei ist kuerzer als die
    /// gemerkte Nummer. Ein Satz sagt beides, und keine der beiden Auskuenfte
    /// faellt weg.
    #[test]
    fn eine_marke_auf_eine_gekuerzte_datei_meldet_beide_auskuenfte_in_einem_satz() {
        let text = "eins\nzwei\ndrei\n";
        let sprung = wiederfinden(text, 500, "Zeile 500");
        assert_eq!(sprung.fund, Fund::NichtGefunden);
        assert_eq!(sprung.sprung.lage, Zeilenlage::HinterDerLetzten);

        let satz = Editormeldung::markenstelle(&sprung)
            .expect("der zusammengesetzte Fall meldet sich")
            .text();
        assert!(
            satz.contains("geändert"),
            "die erste Auskunft steht im Satz: {satz}"
        );
        assert!(
            satz.contains("500") && satz.contains("Dateiende"),
            "die zweite Auskunft steht im selben Satz: {satz}"
        );
    }

    /// Die dritte Lage, erreichbar allein aus einer von Hand geaenderten
    /// `bookmarks.toml` mit `zeile = 0`. Auch sie bekommt ihren eigenen Satz,
    /// statt in einen der beiden anderen zu fallen.
    #[test]
    fn eine_gemerkte_nummer_null_meldet_den_dateianfang() {
        let text = "eins\nzwei\ndrei\n";
        let sprung = wiederfinden(text, 0, "kommt nicht vor");
        assert_eq!(sprung.sprung.lage, Zeilenlage::VorDerErsten);

        let satz = Editormeldung::markenstelle(&sprung)
            .expect("auch die Nummer 0 meldet sich")
            .text();
        assert!(satz.contains("geändert"), "{satz}");
        assert!(satz.contains("Dateianfang"), "{satz}");
    }

    /// Die drei Saetze des Markensprungs sind verschieden. Ohne diese Probe
    /// koennte eine Lage still den Satz einer anderen bekommen.
    #[test]
    fn die_drei_lagen_des_markensprungs_tragen_drei_verschiedene_saetze() {
        let saetze: Vec<String> = [
            Zeilenlage::Getroffen,
            Zeilenlage::VorDerErsten,
            Zeilenlage::HinterDerLetzten,
        ]
        .into_iter()
        .map(|lage| Editormeldung::MarkenstelleGeaendert { zeile: 7, lage }.text())
        .collect();
        assert_ne!(saetze[0], saetze[1]);
        assert_ne!(saetze[1], saetze[2]);
        assert_ne!(saetze[0], saetze[2]);
    }

    /// C4: beide Ausgaenge des Sicherns melden sich, und sie melden
    /// Verschiedenes.
    ///
    /// Der Grund des Fehlschlags kommt fertig aus dem Modell; geprueft wird
    /// hier, dass die Meldung ihn unveraendert weitergibt, statt einen zweiten
    /// Satz daneben zu bauen.
    #[test]
    fn das_sichern_meldet_gelingen_und_fehlschlag_verschieden() {
        let gelungen = Editormeldung::Gesichert { pfad: pfad() }.text();
        assert!(
            gelungen.contains("probe.txt"),
            "die Meldung nennt die geschriebene Datei: {gelungen}"
        );

        let grund = "/tmp/probe.txt ließ sich nicht sichern: Permission denied";
        let gescheitert = Editormeldung::SichernGescheitert {
            grund: grund.to_owned(),
        }
        .text();
        assert_eq!(
            gescheitert, grund,
            "der Grund des Modells geht unverändert durch"
        );
        assert_ne!(gelungen, gescheitert);
    }

    /// C5 verlangt beim Zeilensprung, dass der Grund genannt wird, statt
    /// kommentarlos nichts zu tun. Die drei Faelle, in denen die Nummer keine
    /// Zeile bezeichnet, tragen deshalb drei verschiedene Saetze.
    #[test]
    fn die_drei_verfehlten_zeilensprünge_tragen_drei_verschiedene_saetze() {
        let saetze = [
            Editormeldung::KeineZeilennummer {
                eingabe: "zwölf".to_owned(),
            }
            .text(),
            Editormeldung::ZeileVorDerErsten.text(),
            Editormeldung::ZeileHinterDerLetzten { zeilenzahl: 42 }.text(),
        ];
        for satz in &saetze {
            assert!(
                !satz.is_empty(),
                "kommentarlos nichts zu sagen ist unzulässig"
            );
        }
        assert!(
            saetze[0].contains("zwölf"),
            "die Meldung nennt, was der Nutzer geschrieben hat: {}",
            saetze[0]
        );
        assert!(
            saetze[2].contains("42"),
            "die Meldung nennt die Zeilenzahl der Datei: {}",
            saetze[2]
        );
        assert_ne!(saetze[0], saetze[1]);
        assert_ne!(saetze[1], saetze[2]);
        assert_ne!(saetze[0], saetze[2]);
    }

    /// C5 verlangt, dass die Suche Trefferzahl und Stelle nennt und dass eine
    /// erfolglose Suche sich meldet. Beide Saetze baut das Modell; geprueft wird
    /// hier, dass die Meldung sie unveraendert weitergibt.
    #[test]
    fn der_suchstand_gibt_den_satz_des_modells_unveraendert_weiter() {
        let satz = "Treffer 2 von 7";
        assert_eq!(
            Editormeldung::Suchstand {
                satz: satz.to_owned()
            }
            .text(),
            satz
        );
        assert_ne!(
            Editormeldung::KeineSuche.text(),
            satz,
            "eine gar nicht laufende Suche ist etwas anderes als eine ohne Treffer"
        );
        assert!(!Editormeldung::KeineSuche.text().is_empty());
    }

    /// C5 verlangt, dass das Ersetzen aller Treffer ihre Zahl nennt. Die drei
    /// Zahlformen sind verschieden, und keine ist leer.
    #[test]
    fn das_sammelersetzen_nennt_die_zahl_in_jeder_form() {
        let keiner = Editormeldung::Ersetzt { zahl: 0 }.text();
        let einer = Editormeldung::Ersetzt { zahl: 1 }.text();
        let viele = Editormeldung::Ersetzt { zahl: 7 }.text();
        assert!(!keiner.is_empty());
        assert_ne!(keiner, einer);
        assert_ne!(einer, viele);
        assert!(
            viele.contains('7'),
            "die Meldung nennt die Zahl der ersetzten Treffer: {viele}"
        );
    }

    /// Das zweite Abnahmekriterium von C4, an der Stelle gemessen, an der der
    /// Satz entsteht: der Kopf traegt den Namen, und ein ungesicherter Stand
    /// setzt ein Zeichen davor.
    #[test]
    fn der_kopf_zeigt_den_namen_und_bei_abweichung_ein_zeichen() {
        let pfad = PathBuf::from("/tmp/tief/lies.md");

        assert_eq!(kopfzeile(Some(&pfad), false), "lies.md");
        let abweichend = kopfzeile(Some(&pfad), true);
        assert_ne!(
            abweichend, "lies.md",
            "ein ungesicherter Stand ist am Kopf zu sehen"
        );
        assert!(abweichend.contains("lies.md"), "der Name bleibt lesbar");
        assert!(
            abweichend.starts_with(ABWEICHUNGSZEICHEN),
            "das Zeichen steht vorn, wo eine Kürzung von rechts es nicht erreicht: {abweichend}"
        );
    }

    /// Der Kopf nennt den Namen und nicht den Pfad; den vollen Pfad traegt der
    /// Fenstertitel aus C11.
    #[test]
    fn der_kopf_nennt_den_namen_und_nicht_den_pfad() {
        let pfad = PathBuf::from("/Users/jemand/Projekte/krk/lies.md");
        assert_eq!(kopfzeile(Some(&pfad), false), "lies.md");
    }

    /// Ohne gehaltene Datei bleibt der Kopf leer.
    #[test]
    fn ohne_datei_bleibt_der_kopf_leer() {
        assert_eq!(kopfzeile(None, false), "");
        assert_eq!(
            kopfzeile(None, true),
            "",
            "ohne Datei gibt es auch nichts, was abweichen könnte"
        );
    }

    /// Der Defekt 260810-0215, in der Rechnung nachgespielt, die
    /// [`Editorbereich::flaeche_richten`] fuehrt.
    ///
    /// Das Fenster fehlt dieser Pruefung, die beiden Zeichenketten nicht: die
    /// Textflaeche steht als gewoehnliches `String` da, und was `setString:`
    /// spaeter tut, steht hier als Zuweisung. Gemessen wird, was der Defekt
    /// benennt — dass dieselbe Stelle in beiden Texten auf dasselbe zeigt.
    #[test]
    fn nach_einem_eingefuegten_crlf_zeigt_dieselbe_stelle_in_beiden_texten_auf_dasselbe() {
        // Was eine `NSTextView` nach dem Einfuegen aus einer Windows-Quelle
        // traegt, und wo AppKit die Schreibmarke danach hat: hinter dem
        // Eingefuegten.
        let mut flaeche = String::from("erste\r\nzweite\r\ndritte");
        let schreibmarke = koordinaten::in_utf16(&flaeche, &[13])[0];

        let mut modell = Editormodell::neu();
        assert!(
            modell.bearbeiten(flaeche.clone()),
            "das Modell verlangt, die Flaeche nachzuziehen"
        );
        assert_eq!(modell.stand(), "erste\nzweite\ndritte");

        // Ungerichtet: die Schreibmarke steht in der Flaeche hinter „zweite“,
        // dieselbe Zahl zeigt im Stand aber schon in die dritte Zeile.
        let ungerichtet = koordinaten::in_bytes(modell.stand(), schreibmarke);
        assert_eq!(&flaeche[..13], "erste\r\nzweite");
        assert_eq!(
            &modell.stand()[..ungerichtet],
            "erste\nzweite\n",
            "genau das ist die Abweichung, die 260810-0215 benennt"
        );

        // Gerichtet: die Stelle wird mitgerechnet, dann bekommt die Flaeche den
        // Stand — dieselben zwei Schritte wie in `flaeche_richten`.
        let versatz = datei::versatz_nach_der_wandlung(
            &flaeche,
            koordinaten::in_bytes(&flaeche, schreibmarke),
            modell.stand(),
        );
        flaeche = modell.stand().to_owned();

        assert_eq!(flaeche, modell.stand(), "beide tragen dieselben Zeichen");
        assert_eq!(
            &modell.stand()[..versatz],
            "erste\nzweite",
            "die Schreibmarke steht wieder, wo der Nutzer sie hatte"
        );

        // Und von jetzt an trifft jede Stelle des Standes in der Flaeche
        // dieselbe: das ist die Zusage, auf der Zeilensprung, Suche,
        // Markensprung und die Schreibmarkenzeile rechnen.
        for stelle in (0..=modell.stand().len()).filter(|s| modell.stand().is_char_boundary(*s)) {
            let in_der_flaeche = koordinaten::in_utf16(&flaeche, &[stelle])[0];
            assert_eq!(
                koordinaten::in_bytes(modell.stand(), in_der_flaeche),
                stelle,
                "die Stelle {stelle} kommt nicht zurueck"
            );
        }
    }

    /// Die Staende, an denen der Umkehrpunkt geprueft wird: je ein Paar aus
    /// `vorher` und `nachher` samt einem Wort, das sagt, was dazwischen geschah.
    ///
    /// Die Liste deckt die Faelle, in denen ein Schnitt auf Byteebene falsch
    /// laege: mehrbytige Zeichen unmittelbar an der Aenderung, ein Zeichen, das
    /// durch ein anderes derselben Bytelaenge ersetzt wird, eine Aenderung am
    /// Anfang, eine am Ende, der leere Text in beide Richtungen und ein Austausch
    /// des ganzen Textes.
    const UMKEHRFAELLE: &[(&str, &str, &str)] = &[
        ("ein Treffer in der Mitte", "eins foo zwei", "eins bar zwei"),
        ("laenger geworden", "eins foo zwei", "eins barbar zwei"),
        ("kuerzer geworden", "eins foobar zwei", "eins f zwei"),
        ("am Anfang eingefuegt", "zwei", "eins zwei"),
        ("am Ende angehaengt", "eins", "eins zwei"),
        ("ganz ersetzt", "eins", "zwei"),
        ("aus nichts", "", "eins"),
        ("zu nichts", "eins", ""),
        ("unveraendert", "eins", "eins"),
        (
            "Umlaute rings um die Stelle",
            "gröÖßer foo ächz",
            "gröÖßer bar ächz",
        ),
        ("ein Zeichen gegen ein gleich langes", "ä", "ö"),
        ("Bildzeichen ausserhalb der BMP", "a🌍b", "a🌦b"),
        (
            "mehrere Stellen auf einmal, wie beim Sammelersetzen",
            "a foo b foo c foo d",
            "a bar b bar c bar d",
        ),
        ("nur Zeilenenden", "eins\nzwei\n", "eins\n\nzwei\n"),
    ];

    /// Der Umkehrpunkt haelt den geaenderten Bereich und nicht den ganzen Stand
    /// (Defekt 260810-1241).
    ///
    /// **Gemessen an der Editorgrenze und nicht geschaetzt.** Gerechnet wird an
    /// einem Stand von `krk_core::text::datei::EDITORGRENZE` Bytes mit **einer**
    /// Ersetzung darin — dem Weg, den C5 mit `shift+cmd+r` selbst anbietet — und
    /// verglichen wird gegen das, was die Darstellung bis zum 260810-1241 hielt:
    /// eine Abschrift des ganzen Standes je Handlung.
    ///
    /// Die Probe haelt den Bau an, sobald der Punkt wieder mit der Dateigroesse
    /// waechst. Die Schranke ist der Kilobyte-Sprung: sie faellt aus, wenn ein
    /// Punkt an einer Datei von 16 MB mehr als 1 kB haelt, und laesst offen, wie
    /// genau der Bereich zugeschnitten ist.
    #[test]
    fn ein_umkehrpunkt_traegt_den_geaenderten_bereich_und_nicht_den_ganzen_stand() {
        let haelfte = usize::try_from(datei::EDITORGRENZE / 2).expect("16 MB passen in usize");
        let vorher = format!("{}foo{}", "a".repeat(haelfte), "b".repeat(haelfte));
        let nachher = vorher.replacen("foo", "quux", 1);

        let punkt = Umkehrpunkt::zwischen(&vorher, &nachher, NSRange::new(0, 0));
        let vorher_je_handlung = vorher.len();
        let jetzt_je_handlung = punkt.getragene_bytes();

        assert_eq!(
            jetzt_je_handlung, 3,
            "der Punkt haelt mehr als das ersetzte `foo`: {jetzt_je_handlung} Bytes"
        );
        assert!(
            jetzt_je_handlung < 1024,
            "eine Handlung haelt an einer Datei von {vorher_je_handlung} Bytes wieder \
             {jetzt_je_handlung} Bytes — der Stapel des NSUndoManager hat keine Tiefengrenze, \
             also ist das die Dateigroesse mal der Zahl der Handlungen (260810-1241)"
        );
        // Der Fall des Datensatzes: hundert Ersetzungen hintereinander.
        assert!(
            jetzt_je_handlung * 100 < vorher_je_handlung / 1024,
            "hundert Handlungen halten {} Bytes; vor dem Umbau waren es {} Bytes",
            jetzt_je_handlung * 100,
            vorher_je_handlung * 100
        );
        assert_eq!(
            punkt.angewandt_auf(&nachher),
            vorher,
            "der Punkt stellt den Stand nicht zeichengleich wieder her"
        );
    }

    /// Der Stapel haelt hoechstens das Budget und die letzte Handlung (Defekt
    /// 260810-1314).
    ///
    /// # Der Fall, den sie faehrt
    ///
    /// Der des Datensatzes, und an der Editorgrenze gemessen statt an einem
    /// kleinen Stellvertreter: eine Datei von `krk_core::text::datei::EDITORGRENZE`
    /// Bytes, ein `a` nahe dem Anfang und ein `a` nahe dem Ende, Ersatztext `aa`.
    /// Der Bereich zwischen dem ersten und dem letzten Treffer ist damit die ganze
    /// Datei — genau der Fall, in dem die Darstellung des Umkehrpunkts aus
    /// `260810-1241` nichts einspart —, und weil der Ersatztext den Suchtext
    /// enthaelt, findet jeder weitere Ruf wieder Treffer.
    ///
    /// Ersetzt wird mit `krk_core::text::suche::alle_ersetzen`, also mit der
    /// Funktion, die hinter `ctrl+cmd+r` steht. Die Staende sind deshalb die, die
    /// der Befehl herstellt, und nicht von Hand gebaute daneben.
    ///
    /// # Was sie messt und was sie nicht schaetzt
    ///
    /// Zwei Durchgaenge ueber dieselbe Folge von Rufen. Der erste haelt die Punkte
    /// nicht und summiert allein ihre Bytes: ohne Budget faellt keiner von ihnen,
    /// also **ist** diese Summe, was der Stapel bis zum 260810-1314 hielt. Der
    /// zweite haelt sie wirklich, in einem `Vec`, das [`Verlauf::TraegtNurDiese`]
    /// leert, bevor es die Handlung darauf legt — dieselbe Reihenfolge wie in
    /// [`Editorbereich::stand_einsetzen`] —, und liest den Zaehler ab, den
    /// [`Stapellast`] fuehrt. Gemessen wird also die Zahl, die auch im Betrieb
    /// entscheidet.
    #[test]
    fn der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung() {
        /// So viele Rufe, wie die Rechnung braucht: mit dem zweiten greift das
        /// Budget, der dritte zeigt, dass es weiter greift. Mehr kosteten hier
        /// allein Laufzeit, denn jeder Ruf laeuft dreimal ueber 16 MB.
        const RUFE: usize = 3;

        let grenze = usize::try_from(datei::EDITORGRENZE).expect("16 MB passen in usize");
        let anfangsstand = format!("a{}a", "b".repeat(grenze - 2));
        let auswahl = NSRange::new(0, 0);

        let mut je_ruf = Vec::new();
        let mut stand = anfangsstand.clone();
        for _ in 0..RUFE {
            let neu = suche::alle_ersetzen(&stand, "a", "aa").stand;
            je_ruf.push(Umkehrpunkt::zwischen(&stand, &neu, auswahl).getragene_bytes());
            stand = neu;
        }
        let ohne_budget: usize = je_ruf.iter().sum();
        let groesster = *je_ruf.iter().max().expect("mindestens ein Ruf");

        let zaehler = Rc::new(Cell::new(0));
        let mut stapel: Vec<Stapellast> = Vec::new();
        let mut geraeumt = 0;
        let mut stand = anfangsstand;
        let mut voriger = String::new();
        for _ in 0..RUFE {
            let neu = suche::alle_ersetzen(&stand, "a", "aa").stand;
            let punkt = Umkehrpunkt::zwischen(&stand, &neu, auswahl);
            match verlauf_fuer_umbau(punkt, zaehler.get()) {
                Verlauf::Traegt(punkt) => stapel.push(Stapellast::angemeldet(punkt, &zaehler)),
                Verlauf::TraegtNurDiese(punkt) => {
                    stapel.clear();
                    stapel.push(Stapellast::angemeldet(punkt, &zaehler));
                    geraeumt += 1;
                }
                Verlauf::Faellt => unreachable!("ein Ersetzen laesst den Verlauf nicht fallen"),
            }
            voriger = std::mem::replace(&mut stand, neu);
        }
        let mit_budget = zaehler.get();

        println!(
            "{RUFE} Rufe an einer Datei von {grenze} Bytes: je Ruf {je_ruf:?} Bytes, \
             ohne Budget {ohne_budget} Bytes, mit Budget {mit_budget} Bytes, \
             {geraeumt} mal geraeumt"
        );
        // Die Zahl je Ruf steht fest, und daran haengt die Hochrechnung im
        // Datensatz: hundert Rufe halten ohne Budget hundertmal diese Zahl, mit
        // Budget weiter eine. Waechst oder faellt sie zwischen den Rufen, ist die
        // Hochrechnung keine Multiplikation mehr und der Datensatz zu berichtigen.
        assert!(
            je_ruf.iter().all(|bytes| *bytes == groesster),
            "die Rufe halten verschieden viel: {je_ruf:?}"
        );
        assert!(
            ohne_budget > STAPELBUDGET,
            "die Probe misst den Fall nicht: {ohne_budget} Bytes bleiben unter dem Budget von \
             {STAPELBUDGET}, also greift es nie"
        );
        assert!(
            mit_budget <= STAPELBUDGET + groesster,
            "der Stapel haelt {mit_budget} Bytes und damit mehr als das Budget von \
             {STAPELBUDGET} samt der groessten Handlung von {groesster} (260810-1314)"
        );
        assert!(
            mit_budget < ohne_budget,
            "das Budget spart nichts: {mit_budget} gegen {ohne_budget} Bytes"
        );

        // Ruecknehmbar bleibt der letzte Ruf, und das ist die Zusage aus C5, die
        // das Raeumen nicht antasten darf.
        let letzter = stapel.last().expect("der letzte Ruf steht im Stapel");
        assert_eq!(
            letzter.punkt.angewandt_auf(&stand),
            voriger,
            "das letzte Sammelersetzen ist nicht mehr zuruecknehmbar"
        );

        drop(stapel);
        assert_eq!(
            zaehler.get(),
            0,
            "der Zaehler steht nicht wieder auf null, nachdem der Stapel fort ist"
        );
    }

    /// Das Tippen bleibt unbegrenzt, und der Zaehler ist der Grund.
    ///
    /// **Sie messt eine Abwesenheit**, und deshalb steht sie hier: das Budget
    /// greift ueber [`Stapellast`], und eine Handlung, die die `NSTextView` fuer
    /// einen Anschlag selbst anmeldet, geht durch keine [`Stapellast`]. Ein
    /// Zaehler, der bei null steht, kann keine Raeumung ausloesen — `0` plus ein
    /// Punkt von drei Bytes liegt unter jedem Budget.
    ///
    /// Der Gegenweg dazu waere `setLevelsOfUndo`, und der Grund gegen ihn steht an
    /// [`Umkehrpunkt`]: eine Tiefengrenze gilt fuer den ganzen Verwalter und traefe
    /// das Tippen mit.
    #[test]
    fn ein_gewoehnlicher_umbau_bleibt_neben_dem_verlauf_und_erst_der_volle_stapel_wird_geraeumt() {
        let punkt = Umkehrpunkt::zwischen("eins foo zwei", "eins bar zwei", NSRange::new(0, 0));
        assert_eq!(punkt.getragene_bytes(), 3, "der Punkt haelt mehr als `foo`");
        assert!(
            matches!(verlauf_fuer_umbau(punkt, 0), Verlauf::Traegt(_)),
            "ein Umbau von drei Bytes raeumt den Verlauf"
        );

        // Und die Gegenrichtung: voll ist voll, gleich wie klein der Umbau ist.
        let punkt = Umkehrpunkt::zwischen("eins foo zwei", "eins bar zwei", NSRange::new(0, 0));
        assert!(
            matches!(
                verlauf_fuer_umbau(punkt, STAPELBUDGET),
                Verlauf::TraegtNurDiese(_)
            ),
            "ein voller Stapel nimmt noch eine Handlung dazu"
        );
    }

    /// Der Punkt stellt jeden Stand zeichengleich wieder her, und sein Gegenweg
    /// fuehrt zurueck.
    ///
    /// **Beide Richtungen in einer Probe**, weil sie in [`Editorbereich::umkehren`]
    /// dieselbe Zeile sind: der Gegenweg entsteht dort aus dem gehaltenen und dem
    /// wiederhergestellten Stand, und ein `shift+cmd+z` wendet ihn an. Faellt eine
    /// der beiden Richtungen aus, ist `cmd+z` und `shift+cmd+z` nicht mehr
    /// gegeneinander lauffaehig.
    #[test]
    fn ein_umkehrpunkt_und_sein_gegenweg_stellen_beide_staende_zeichengleich_her() {
        for (was, vorher, nachher) in UMKEHRFAELLE {
            let auswahl = NSRange::new(0, 0);
            let punkt = Umkehrpunkt::zwischen(vorher, nachher, auswahl);
            let zurueck = punkt.angewandt_auf(nachher);
            assert_eq!(&zurueck, vorher, "{was}: das Rueckgaengig trifft nicht");

            let gegenweg = Umkehrpunkt::zwischen(nachher, &zurueck, auswahl);
            assert_eq!(
                &gegenweg.angewandt_auf(&zurueck),
                nachher,
                "{was}: das Wiederherstellen trifft nicht"
            );
        }
    }

    /// Der Punkt bleibt in der gehaltenen Form, und deshalb meldet
    /// [`Editormodell::bearbeiten`] beim Umkehren keine Nachrichtung der Flaeche.
    ///
    /// **Daran haengt die Zusicherung in [`Editorbereich::umkehren`].** Waere der
    /// wiederhergestellte Stand nicht in gehaltener Form, wandelte das Modell ihn,
    /// und Flaeche und Stand liefen auseinander — genau der Defekt `260810-0215`,
    /// nur aus der anderen Richtung.
    #[test]
    fn ein_wiederhergestellter_stand_ist_in_gehaltener_form() {
        for (was, vorher, nachher) in UMKEHRFAELLE {
            assert!(
                datei::ist_in_gehaltener_form(vorher) && datei::ist_in_gehaltener_form(nachher),
                "{was}: die Probe misst nur Staende, die selbst gehalten sind"
            );
            let punkt = Umkehrpunkt::zwischen(vorher, nachher, NSRange::new(0, 0));
            assert!(
                datei::ist_in_gehaltener_form(&punkt.angewandt_auf(nachher)),
                "{was}: der wiederhergestellte Stand ist nicht in gehaltener Form"
            );
        }
    }

    /// Ein Verwalter fuer sich, ohne Flaeche und ohne Fenster.
    ///
    /// **Das `new_unchecked` ist hier vertretbar und sonst nirgends.** Der
    /// Pruefstand von Rust laesst jede Pruefung auf einem eigenen Faden laufen,
    /// und `MainThreadMarker::new()` gaebe dort `None`. Was der Marker
    /// absichert, ist die Fadenbindung von AppKits Fensterwerkzeug; ein
    /// `NSUndoManager` haengt an keinem Fenster, sondern an der
    /// Ereignisschleife seines Fadens, und die ist hier mit
    /// `setGroupsByEvent(false)` abgewaehlt. Der Verwalter dieser Pruefung
    /// wird ausserdem auf demselben Faden erzeugt, benutzt und fallengelassen.
    fn verwalter_ohne_fenster() -> Retained<NSUndoManager> {
        NSUndoManager::new(unsafe { MainThreadMarker::new_unchecked() })
    }

    /// Meldet eine Rueckgaengig-Handlung an, wie eine `NSTextView` es beim
    /// Tippen tut.
    ///
    /// Die Handlung selbst tut nichts — gemessen wird, ob der Verwalter sie
    /// **hat**, nicht was sie taete.
    fn handlung_anmelden(verwalter: &NSUndoManager, ziel: &NSObject) {
        let handlung = RcBlock::new(|_ziel: NonNull<AnyObject>| {});
        // SAFETY: `ziel` ist ein NSObject und wird vom Verwalter nur als
        // Kennung gehalten; der Block spricht es nicht an.
        unsafe { verwalter.registerUndoWithTarget_handler(ziel, &handlung) };
    }

    /// Meldet eine Handlung an und schliesst ihre Gruppe **von Hand**, bei
    /// abgeschalteter Ereignisgruppierung.
    ///
    /// **Das ist nicht die Betriebsart der Laufzeit**, und der Unterschied stand
    /// bis zum 260810 nicht dabei
    /// (`issues/260810-0420_*_die-beiden-rueckgaengigproben-schalten-die-betriebsart-ab-die-sie-messen-sollen.md`).
    /// `groupsByEvent` steht ab Werk auf `true` und schliesst die Gruppe erst am
    /// Ende eines Umlaufs der Laufschleife; in einer Pruefung laeuft keine, also
    /// gaebe es hier ohne die Abschaltung nie eine geschlossene Gruppe. Die
    /// beiden Pruefungen darunter messen deshalb den **Mechanismus**
    /// `removeAllActions` an einer geschlossenen und an einer offenen Gruppe, und
    /// nicht die Betriebsart. Die Betriebsart hat ihre eigene Pruefung:
    /// [`ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung`].
    fn stapel_fuellen(verwalter: &NSUndoManager, ziel: &NSObject) {
        verwalter.setGroupsByEvent(false);
        verwalter.beginUndoGrouping();
        handlung_anmelden(verwalter, ziel);
        verwalter.endUndoGrouping();
        assert!(
            verwalter.canUndo(),
            "die Voraussetzung der Pruefung: der Stapel traegt eine Handlung"
        );
    }

    /// Der Defekt 260809-1727: nach einem Schreiben ueber `setString:` darf
    /// kein Rueckgaengig-Verlauf stehenbleiben, der auf den Text der vorigen
    /// Datei zeigt.
    ///
    /// Das Fenster fehlt dieser Pruefung, der Verwalter nicht: `NSUndoManager`
    /// steht fuer sich und braucht weder Flaeche noch Fenster. Gemessen wird
    /// der Schritt, den [`Editorbereich::stand_einsetzen`] seit diesem Defekt
    /// hinter `setString:` setzt.
    #[test]
    fn ein_geleerter_stapel_traegt_keine_rueckgaengig_handlung_mehr() {
        let verwalter = verwalter_ohne_fenster();
        let ziel = NSObject::new();
        stapel_fuellen(&verwalter, &ziel);

        rueckgaengigstapel_leeren(Some(&verwalter));

        assert!(
            !verwalter.canUndo(),
            "nach dem Leeren zeigt keine Handlung mehr auf den vorigen Text"
        );
        assert!(
            !verwalter.canRedo(),
            "der Wiederherstellungsstapel gehoert zum selben Verlauf und faellt mit"
        );
    }

    /// Eine **offene** Gruppe haelt `removeAllActions` ebenso wenig auf wie eine
    /// geschlossene, und nichts wirft.
    ///
    /// Das ist der Grund, aus dem
    /// [`rueckgaengigstapel_leeren`] `removeAllActions` nimmt und nicht
    /// `endUndoGrouping`: das zweite verlangt eine offene Gruppe und wirft ohne
    /// eine.
    ///
    /// **Die Gruppe ist hier von Hand geoeffnet**, und das ist nicht dasselbe
    /// wie die Gruppe, die der Verwalter in der Betriebsart der Laufzeit selbst
    /// oeffnet; siehe [`stapel_fuellen`] und
    /// [`ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung`].
    #[test]
    fn ein_geleerter_stapel_traegt_auch_eine_offene_gruppe_nicht_mehr() {
        let verwalter = verwalter_ohne_fenster();
        let ziel = NSObject::new();
        stapel_fuellen(&verwalter, &ziel);

        // Eine zweite Handlung, deren Gruppe offen bleibt.
        verwalter.beginUndoGrouping();
        handlung_anmelden(&verwalter, &ziel);

        rueckgaengigstapel_leeren(Some(&verwalter));

        assert!(
            !verwalter.canUndo(),
            "auch die offene Gruppe ist weg, nicht nur die geschlossene"
        );
        assert_eq!(
            verwalter.groupingLevel(),
            0,
            "und der Verwalter steht wieder ausserhalb jeder Gruppe"
        );
    }

    /// Dieselbe Frage in der Betriebsart, in der [`Editorbereich::stand_einsetzen`]
    /// zur Laufzeit steht: `groupsByEvent` auf dem Werkswert `true`.
    ///
    /// **Die Betriebsart ist ein anderer Mechanismus und keine Einstellung
    /// daneben.** Bei `groupsByEvent = true` oeffnet der Verwalter die Gruppe
    /// selbst bei der ersten Anmeldung und schliesst sie ueber einen Beobachter
    /// der Laufschleife am Ende des Umlaufs. Die Frage, die die beiden Pruefungen
    /// darueber nicht beantworten: findet dieser Beobachter eine Gruppe vor, die
    /// `removeAllActions` inzwischen abgeraeumt hat, und wirft er dann? Der
    /// Defekt ist
    /// `issues/260810-0420_*_die-beiden-rueckgaengigproben-schalten-die-betriebsart-ab-die-sie-messen-sollen.md`;
    /// die Antwort stand bis zum 260810 in einem Wegwerf-Programm im
    /// Sitzungsverzeichnis und steht seither hier.
    ///
    /// **Der Umlauf der Laufschleife ist der Kern dieser Pruefung.** Ohne ihn
    /// kommt der Beobachter nicht zum Zug, und dann misst sie nichts, was die
    /// beiden anderen nicht schon messen. `libtest` fuehrt seine Proben auf
    /// eigenen Faeden ohne laufende Schleife; `NSRunLoop::currentRunLoop` legt
    /// dem Faden eine an, und der Verwalter haengt seinen Beobachter beim
    /// Anmelden in genau diese ein. Ein Umlauf ohne Quelle kehrt sofort zurueck,
    /// die Zeitgrenze ist deshalb keine Wartezeit, sondern eine Obergrenze.
    #[test]
    fn ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung() {
        let verwalter = verwalter_ohne_fenster();
        assert!(
            verwalter.groupsByEvent(),
            "der Werkswert von groupsByEvent ist true; steht er auf false, misst diese \
             Pruefung dieselbe Betriebsart wie die beiden darueber"
        );

        let ziel = NSObject::new();
        handlung_anmelden(&verwalter, &ziel);
        assert_eq!(
            verwalter.groupingLevel(),
            1,
            "der Verwalter hat die Gruppe selbst geoeffnet — genau das tut er zur Laufzeit \
             mitten in der Behandlung eines Tastendrucks"
        );
        assert!(
            verwalter.canUndo(),
            "die Voraussetzung der Pruefung: der Stapel traegt eine Handlung"
        );

        rueckgaengigstapel_leeren(Some(&verwalter));
        assert!(!verwalter.canUndo(), "die Handlung ist fort");
        assert_eq!(
            verwalter.groupingLevel(),
            0,
            "und mit ihr die Gruppe, die der Verwalter selbst geoeffnet hatte"
        );

        // Der Umlauf, in dem der Beobachter die Gruppe schliessen wollte, die es
        // nicht mehr gibt. Wirft er, faellt die Pruefung mit dem Programm.
        //
        // SAFETY: `NSDefaultRunLoopMode` ist ein Fremdsymbol von Foundation,
        // dieselbe Form wie `NSRunLoopCommonModes` beim Einzugstakt.
        let umlauf = NSRunLoop::currentRunLoop().runMode_beforeDate(
            unsafe { NSDefaultRunLoopMode },
            &NSDate::dateWithTimeIntervalSinceNow(0.05),
        );
        let _ = umlauf;

        assert_eq!(
            verwalter.groupingLevel(),
            0,
            "nach dem Umlauf steht der Verwalter ausserhalb jeder Gruppe"
        );
        assert!(
            !verwalter.canUndo(),
            "und der Umlauf hat keine Handlung zurueckgebracht"
        );
    }

    /// Ohne Verwalter geschieht nichts. Der Fall ist der Regelfall vor dem
    /// Einhaengen der Flaeche in ein Fenster, nicht ein Fehler.
    #[test]
    fn ohne_verwalter_geschieht_nichts() {
        rueckgaengigstapel_leeren(None);
    }

    /// Die Mechanik, auf der [`Verlauf::TraegtNurDiese`] ruht: eine Anmeldung
    /// **nach** `removeAllActions` steht im Stapel, wirkt, und die Handlungen
    /// davor sind fort.
    ///
    /// **Die Frage ist eine eigene und nicht schon beantwortet.**
    /// `removeAllActions` raeumt bei `groupsByEvent = true` auch die Gruppe ab,
    /// die der Verwalter mitten in der Behandlung eines Tastendrucks selbst
    /// geoeffnet hat (gemessen von
    /// [`ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung`]). Ob er
    /// danach im **selben** Ereignis eine neue Gruppe oeffnet und die Anmeldung
    /// annimmt, folgt daraus nicht — genau davon haengt aber ab, ob ein `cmd+z`
    /// nach einem eingefuegten `\r\n` etwas tut
    /// (`issues/260810-1044_*_ein-eingefuegtes-crlf-bleibt-nicht-ruecknehmbar-und-der-grund-liegt-am-eingang-der-flaeche.md`).
    ///
    /// Die Probe faehrt die Betriebsart der Laufzeit, samt Umlauf der
    /// Laufschleife danach: der Beobachter, der die Gruppe schliesst, kommt damit
    /// zum Zug.
    #[test]
    fn eine_anmeldung_nach_dem_leeren_steht_im_stapel() {
        let verwalter = verwalter_ohne_fenster();
        assert!(
            verwalter.groupsByEvent(),
            "diese Probe misst die Betriebsart der Laufzeit"
        );
        let ziel = NSObject::new();
        let wert = Rc::new(Cell::new(2u8));

        // Was die Flaeche fuer das Einfuegen selbst angemeldet hat, und was ein
        // zweites `cmd+z` erreichen wuerde: der Stapel traegt eine Handlung, die
        // auf den ungewandelten Text zeigt.
        handlung_anmelden(&verwalter, &ziel);
        assert!(verwalter.canUndo());

        // Die Reihenfolge aus `stand_einsetzen` fuer `TraegtNurDiese`.
        rueckgaengigstapel_leeren(Some(&verwalter));
        wert_anmelden(&verwalter, &ziel, Rc::clone(&wert), 1);

        assert!(
            verwalter.canUndo(),
            "die Anmeldung nach dem Leeren steht im Stapel — ohne das taete ein cmd+z nach \
             einem eingefuegten \\r\\n weiter nichts"
        );

        // Der Umlauf, in dem der Beobachter die Gruppe schliesst, die es beim
        // Leeren nicht mehr gab.
        //
        // SAFETY: `NSDefaultRunLoopMode` ist ein Fremdsymbol von Foundation,
        // dieselbe Form wie beim Einzugstakt.
        let _ = NSRunLoop::currentRunLoop().runMode_beforeDate(
            unsafe { NSDefaultRunLoopMode },
            &NSDate::dateWithTimeIntervalSinceNow(0.05),
        );
        assert!(
            verwalter.canUndo(),
            "und der Umlauf hat sie nicht wieder fortgenommen"
        );

        verwalter.undo();
        assert_eq!(wert.get(), 1, "das Rueckgaengig hat gewirkt");
        assert!(
            !verwalter.canUndo(),
            "und ein zweites cmd+z tut nichts: die Handlung der Flaeche ist mit dem Leeren \
             gefallen, und genau das ist der benannte Preis"
        );
        assert!(
            verwalter.canRedo(),
            "der Weg zurueck steht offen, wie beim Ersetzen aus S37"
        );
    }

    /// Meldet eine Handlung an, die einen Wert herstellt und dabei den Gegenweg
    /// anmeldet — die Bauart von [`Editorbereich::umkehren`], ohne Flaeche und
    /// ohne Modell.
    ///
    /// Der Zaehler steht fuer den gehaltenen Stand, den das Ersetzen aus S37 hin
    /// und her traegt; gemessen wird die Mechanik und nicht der Text.
    fn wert_anmelden(verwalter: &NSUndoManager, ziel: &NSObject, wert: Rc<Cell<u8>>, ziffer: u8) {
        let verwalter_hier = verwalter.retain();
        let ziel_hier = ziel.retain();
        let handlung = RcBlock::new(move |_ziel: NonNull<AnyObject>| {
            // Der Gegenweg zuerst, in genau der Reihenfolge, die
            // `Editorbereich::umkehren` haelt.
            let vorher = wert.get();
            wert_anmelden(&verwalter_hier, &ziel_hier, Rc::clone(&wert), vorher);
            wert.set(ziffer);
        });
        // SAFETY: `ziel` ist ein NSObject und wird vom Verwalter nur als Kennung
        // gehalten; der Block spricht es nicht an.
        unsafe { verwalter.registerUndoWithTarget_handler(ziel, &handlung) };
    }

    /// Die Mechanik, auf der das rueckgaengigfaehige Ersetzen aus S37 ruht: eine
    /// Anmeldung **waehrend** eines Rueckgaengig landet im
    /// Wiederherstellungsstapel und nicht wieder im Rueckgaengigstapel.
    ///
    /// **Ohne diese Eigenschaft gaebe es kein Wiederherstellen und moeglicherweise
    /// einen Ring.** [`Editorbereich::umkehren`] meldet den Gegenweg an, bevor es
    /// den Stand herstellt, und verlaesst sich darauf, dass `NSUndoManager` die
    /// Anmeldung in diesem Augenblick anders einordnet als sonst. Die Probe haelt
    /// die Eigenschaft fest, statt sie der Dokumentation zu entnehmen; sie
    /// braucht dafuer weder Flaeche noch Fenster, weil `NSUndoManager` fuer sich
    /// steht.
    ///
    /// **Die Ereignisgruppierung bleibt auf dem Werkswert.** `undo` schliesst
    /// eine offene Gruppe der obersten Ebene selbst, also braucht diese Probe
    /// keinen Umlauf der Laufschleife und keine Abschaltung — sie laeuft in
    /// derselben Betriebsart wie der Editor.
    #[test]
    fn eine_anmeldung_waehrend_eines_rueckgaengig_landet_im_wiederherstellungsstapel() {
        let verwalter = verwalter_ohne_fenster();
        let ziel = NSObject::new();
        // 2 ist der Stand nach dem Ersetzen, 1 der davor.
        let wert = Rc::new(Cell::new(2u8));
        wert_anmelden(&verwalter, &ziel, Rc::clone(&wert), 1);

        assert!(
            verwalter.canUndo(),
            "die angemeldete Handlung steht im Stapel"
        );
        assert!(
            !verwalter.canRedo(),
            "und der Wiederherstellungsstapel ist leer"
        );

        verwalter.undo();
        assert_eq!(
            wert.get(),
            1,
            "das Rueckgaengig hat den vorigen Stand hergestellt"
        );
        assert!(
            verwalter.canRedo(),
            "die Anmeldung aus der Handlung steht im Wiederherstellungsstapel — ohne das \
             waere ein Ersetzen einmal zurueckzunehmen und nie wiederherzustellen"
        );
        assert!(
            !verwalter.canUndo(),
            "und sie steht nicht wieder im Rueckgaengigstapel; sonst liefe cmd+z im Kreis"
        );

        verwalter.redo();
        assert_eq!(
            wert.get(),
            2,
            "das Wiederherstellen hat den neuen Stand zurueckgebracht"
        );
        assert!(
            verwalter.canUndo(),
            "und der Weg zurueck steht wieder offen: die beiden wechseln sich ab"
        );
    }

    /// Die Namensformen, in denen `NSTextView` seine Einstellungen fuehrt — die
    /// **heuristische** Haelfte des Schnitts.
    ///
    /// Sechs sind es heute: die alte boolesche `set…Enabled:`, die dreiwertige
    /// `set…Type:` aus macOS 14, ihre Sammelform `set…Types:`, die eine
    /// `set…Behavior:` und die beiden Formen der Schreibwerkzeuge `set…Options:`
    /// und `set…Affordance:`. Der Modulkopf sagt unter „Die Namensform ist nicht
    /// der Schnitt", warum diese Aufzaehlung ein Stolperdraht ist und kein
    /// Beweis, und welche zweite Quelle daneben steht.
    ///
    /// **Drei Formen waren es bis zum 260810**, und die drei fehlenden haben je
    /// einen belegten Fall gekostet: `Types:` die Sammeltuer
    /// `setEnabledTextCheckingTypes:`, `Options:` zwei
    /// Schreibwerkzeug-Einstellungen und `Affordance:` die dritte. Wer eine
    /// siebte Form braucht, hat einen Fall dafuer — sonst waechst hier eine
    /// Liste, die immer breiter und nie vollstaendiger wird.
    const FORMEN: [&str; 6] = [
        "Enabled:",
        "Type:",
        "Types:",
        "Behavior:",
        "Options:",
        "Affordance:",
    ];

    /// Das Protokoll, dessen Mitgliedschaft ohne Namensform entscheidet — die
    /// **geschlossene** Haelfte des Schnitts.
    ///
    /// Wer hier Mitglied ist, ist eine Texteingabe-Einstellung, gleich wie der
    /// Selektor endet. Vierzehn Pflichtmerkmale fuehrt es auf diesem Geraet.
    const MERKMALSPROTOKOLL: &CStr = c"NSTextInputTraits";

    /// Wie eine Einstellung der Textflaeche zur Zusage aus C4 steht.
    ///
    /// **Fuenf Antworten, und die Aufzaehlung hat keinen Auffangzweig.** Die
    /// dritte und die vierte trennen zwei Sorten Tuer, die vor dem 260810 eine
    /// waren: eine Tuer auf **eine** Einstellung und eine Tuer auf **mehrere**.
    /// Die beiden werden verschieden nachgemessen und lassen sich deshalb nicht
    /// in einer Variante fuehren
    /// (`issues/260810-0746_*_es-gibt-eine-dritte-tuer-und-sie-liegt-ausserhalb-aller-drei-namensformen.md`).
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Einordnung {
        /// [`textautomatik::automatiken_abschalten`] schaltet sie ab, das
        /// [`textflaeche_bauen`] und die Flaeche des Zettels rufen, weil sie
        /// Zeichen in den Text bringt oder aus ihm nimmt, die der Nutzer nicht
        /// getippt hat.
        ///
        /// Dass sie an der gebauten Flaeche wirklich aus steht, misst
        /// [`die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`].
        Abgeschaltet,
        /// Sie darf anbleiben, weil sie den Textspeicher nicht anfasst.
        Geduldet,
        /// Zweite Tuer zu **einer** genannten Einstellung: beide legen einander
        /// um. Sie eigens zu setzen schaltete ab, was schon aus ist.
        ///
        /// Die Kopplung misst [`jede_zweite_tuer_und_ihre_erste_legen_einander_um`],
        /// je Paar einzeln und in beiden Richtungen. „Derselbe Speicher" waere
        /// eine Stufe zu stark: was die erste Tuer nicht kann, ist `Default`
        /// herstellen oder anzeigen, und das misst
        /// [`die_erste_tuer_kann_default_weder_herstellen_noch_anzeigen`].
        ZweiteTuerZu(&'static str),
        /// Eine Tuer auf **mehrere** genannte Einstellungen zugleich, naemlich
        /// die Bitmaske `setEnabledTextCheckingTypes:`.
        ///
        /// Sie wird in KRK **nicht gesetzt**: sie waere eine zweite Stelle mit
        /// einer Meinung darueber, was abgeschaltet ist, und die einzelnen
        /// Zeilen in [`textautomatik::automatiken_abschalten`] sind die erste. Dass sie auf
        /// dieselben Bits sieht, misst
        /// [`die_sammeltuer_ist_eine_sicht_auf_dieselben_bits`].
        SammeltuerZu(&'static [&'static str]),
        /// Sie beschreibt, **was** eine Faehigkeit duerfte, die an dieser Flaeche
        /// abgeschaltet ist — und traegt selbst keinen Wert, der nichts zulaesst.
        /// Genannt ist der Setzer, der die Faehigkeit abschaltet.
        ///
        /// **Der Unterschied zu [`Einordnung::ZweiteTuerZu`] ist gemessen.** Eine
        /// zweite Tuer wird vom Genannten **umgelegt**; eine gegenstandslose
        /// Einstellung bleibt stehen, wo sie stand, und verliert nur ihren
        /// Gegenstand. Beides misst
        /// [`die_gegenstandslosen_stehen_unberuehrt_und_ihr_traeger_steht_aus`].
        ///
        /// Zwei sind es, die beiden Bitmasken der Schreibwerkzeuge: ihre Null
        /// heisst `…ResultDefault`, also "das System waehlt", und nicht "nichts".
        /// [`textautomatik::automatiken_abschalten`] setzt sie deshalb **nicht**
        /// — eine Zeile waere ein Aufruf ohne Wirkung, und [`aus_bedeutet`] kennt
        /// fuer die Form `Options:` folgerichtig keinen Aus-Wert.
        Gegenstandslos(&'static str),
        /// Bekannt, benannt, und die Einordnung haengt an einer Lesart von C4,
        /// die der Nutzer zu treffen hat. Der Datensatz steht dabei.
        ///
        /// **Heute steht keine Einstellung hier, und das ist der Zustand nach
        /// einer getroffenen Entscheidung**, nicht ein leer gebliebener Platz. Die
        /// vier Schreibwerkzeug-Einstellungen standen bis zum 260810 so; die
        /// Variante bleibt, weil die naechste Lesart, die dem Nutzer gehoert,
        /// wieder eine braucht.
        #[allow(dead_code)]
        NochOffen(&'static str),
    }

    /// Jede Einstellung, die diese Laufzeit in einer der [`FORMEN`] oder als
    /// Mitglied von [`MERKMALSPROTOKOLL`] traegt, mit ihrer Antwort auf die
    /// Frage aus C4.
    ///
    /// **Sechsunddreissig sind es, und die Aufstellung ist die Vorlage der
    /// Messungen und nicht ihr Nachtrag.** Was hier als `Abgeschaltet`,
    /// `ZweiteTuerZu` oder `SammeltuerZu` steht, fahren die Proben weiter unten
    /// an einer eigens gebauten `NSTextView` nach. Eine Zeile, die etwas
    /// Falsches behauptet, haelt damit den Bau an, statt eine Behauptung zu
    /// bleiben.
    ///
    /// **Abgeschaltet sind sieben.** Vier greifen beim Tippen, die fuenfte beim
    /// Einfuegen und Ausschneiden (Defekt 260809-1650), die sechste und die
    /// siebte tragen keinen booleschen Schalter und rutschten deshalb durch die
    /// Vorform dieser Probe (Defekt 260810-0416).
    ///
    /// **Geduldet sind die uebrigen**, und je Sorte aus einem eigenen Grund:
    /// - Die beiden Erkennungen zeichnen einen Fund als Verknuepfung aus und
    ///   aendern kein Zeichen; `setRichText(false)` nimmt ihnen ohnehin die
    ///   Wirkung.
    /// - Rechtschreib- und Grammatikpruefung setzen voruebergehende Merkmale
    ///   des Layoutverwalters — derselbe Schnitt, aus dem die Einfaerbung der
    ///   Formatansicht aus C3 nicht in die Datei geraet.
    /// - Die Spracherkennung waehlt, **woran** die beiden vorigen messen, und
    ///   nicht den gemessenen Text.
    /// - Die Textvervollstaendigung legt im Vorschlagsstreifen der Touch Bar
    ///   Kandidaten vor; ein Zeichen kommt erst in den Text, wenn der Nutzer
    ///   einen davon waehlt, und das ist eine Eingabe und keine Automatik.
    /// - Die Schrittsuche waehlt einen Fund aus und schreibt nicht; das
    ///   Ersetzen aus C5 laeuft nicht ueber diesen Schalter.
    /// - Die Inhaltsart (`contentType`) sagt dem System, wofuer ein Feld
    ///   gedacht ist, damit es Ausfuellvorschlaege machen kann. An KRKs Flaeche
    ///   steht sie ab Werk auf `nil`, es gibt also nichts vorzuschlagen; und
    ///   auch mit Wert traegt erst die Wahl des Nutzers Zeichen ein.
    /// - Die sechs aus `NSView` und `NSResponder` gehoeren keiner Textklasse und
    ///   kennen keinen Textspeicher: welche Beruehrungsarten eine Ansicht
    ///   annimmt, wie ihr Fokusring aussieht, ob Gestenerkenner anspringen, und
    ///   drei Angaben fuer die Bedienungshilfen. Sie stehen hier, weil die
    ///   Aufzaehlung ueber die ganze Vererbungskette laeuft und nicht, weil sie
    ///   je zur Frage aus C4 gehoert haetten
    ///   (Defekt 260810-0751).
    const EINSTELLUNGEN: &[(&str, Einordnung)] = &[
        // Die vier, die beim Tippen greifen.
        (
            "setAutomaticQuoteSubstitutionEnabled:",
            Einordnung::Abgeschaltet,
        ),
        (
            "setAutomaticDashSubstitutionEnabled:",
            Einordnung::Abgeschaltet,
        ),
        (
            "setAutomaticTextReplacementEnabled:",
            Einordnung::Abgeschaltet,
        ),
        (
            "setAutomaticSpellingCorrectionEnabled:",
            Einordnung::Abgeschaltet,
        ),
        // Die fuenfte, beim Einfuegen und Ausschneiden.
        ("setSmartInsertDeleteEnabled:", Einordnung::Abgeschaltet),
        // Die sechste und die siebte, ohne booleschen Zwilling.
        ("setInlinePredictionType:", Einordnung::Abgeschaltet),
        ("setMathExpressionCompletionType:", Einordnung::Abgeschaltet),
        // Die geduldeten der alten Form.
        ("setAutomaticLinkDetectionEnabled:", Einordnung::Geduldet),
        ("setAutomaticDataDetectionEnabled:", Einordnung::Geduldet),
        ("setContinuousSpellCheckingEnabled:", Einordnung::Geduldet),
        ("setGrammarCheckingEnabled:", Einordnung::Geduldet),
        (
            "setAutomaticLanguageIdentificationEnabled:",
            Einordnung::Geduldet,
        ),
        ("setAutomaticTextCompletionEnabled:", Einordnung::Geduldet),
        ("setIncrementalSearchingEnabled:", Einordnung::Geduldet),
        // Die zehn zweiten Tueren. Keine braucht eine eigene Zeile in
        // `textautomatik::automatiken_abschalten`.
        (
            "setSmartQuotesType:",
            Einordnung::ZweiteTuerZu("setAutomaticQuoteSubstitutionEnabled:"),
        ),
        (
            "setSmartDashesType:",
            Einordnung::ZweiteTuerZu("setAutomaticDashSubstitutionEnabled:"),
        ),
        (
            "setTextReplacementType:",
            Einordnung::ZweiteTuerZu("setAutomaticTextReplacementEnabled:"),
        ),
        (
            "setAutocorrectionType:",
            Einordnung::ZweiteTuerZu("setAutomaticSpellingCorrectionEnabled:"),
        ),
        (
            "setSmartInsertDeleteType:",
            Einordnung::ZweiteTuerZu("setSmartInsertDeleteEnabled:"),
        ),
        (
            "setSpellCheckingType:",
            Einordnung::ZweiteTuerZu("setContinuousSpellCheckingEnabled:"),
        ),
        (
            "setGrammarCheckingType:",
            Einordnung::ZweiteTuerZu("setGrammarCheckingEnabled:"),
        ),
        (
            "setLinkDetectionType:",
            Einordnung::ZweiteTuerZu("setAutomaticLinkDetectionEnabled:"),
        ),
        (
            "setDataDetectionType:",
            Einordnung::ZweiteTuerZu("setAutomaticDataDetectionEnabled:"),
        ),
        (
            "setTextCompletionType:",
            Einordnung::ZweiteTuerZu("setAutomaticTextCompletionEnabled:"),
        ),
        // Die Inhaltsart, ohne Zwilling und ohne Wert.
        ("setContentType:", Einordnung::Geduldet),
        // Die eine Sammeltuer: eine Bitmaske ueber fuenf der geführten
        // Einstellungen. Der Werkswert setzt die vier tippenden Automatiken und
        // nimmt die Grammatikpruefung fort — beides gemessen.
        (
            "setEnabledTextCheckingTypes:",
            Einordnung::SammeltuerZu(&[
                "setAutomaticQuoteSubstitutionEnabled:",
                "setAutomaticDashSubstitutionEnabled:",
                "setAutomaticTextReplacementEnabled:",
                "setAutomaticSpellingCorrectionEnabled:",
                "setGrammarCheckingEnabled:",
            ]),
        ),
        // Die vier Schreibwerkzeug-Einstellungen. Es sind vier und nicht eine: wer
        // die Schreibwerkzeuge ausschliesst, schliesst sie ueber
        // `writingToolsBehavior` allein nicht aus (Defekt 260810-0745). Dass C4
        // sie ausschliesst, ist am 260810 entschieden
        // (`decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`);
        // zwei tragen daraufhin einen Aus-Wert und je eine Zeile in
        // `textautomatik::automatiken_abschalten`, zwei tragen keinen — der
        // Grund steht bei `Einordnung::Gegenstandslos`.
        ("setWritingToolsBehavior:", Einordnung::Abgeschaltet),
        ("setAllowsWritingToolsAffordance:", Einordnung::Abgeschaltet),
        (
            "setAllowedWritingToolsResultOptions:",
            Einordnung::Gegenstandslos("setWritingToolsBehavior:"),
        ),
        (
            "setWritingToolsAllowedInputOptions:",
            Einordnung::Gegenstandslos("setWritingToolsBehavior:"),
        ),
        // Die sechs aus `NSView` und `NSResponder`, die die Aufzaehlung ueber die
        // Vererbungskette mitbringt. Keine fasst einen Textspeicher an.
        ("setAllowedTouchTypes:", Einordnung::Geduldet),
        ("setFocusRingType:", Einordnung::Geduldet),
        ("setGesturesEnabled:", Einordnung::Geduldet),
        ("setAccessibilityContainerType:", Einordnung::Geduldet),
        ("setAccessibilityEnabled:", Einordnung::Geduldet),
        ("setAccessibilityRulerMarkerType:", Einordnung::Geduldet),
    ];

    /// Die Antwort zu einem Selektornamen, oder `None`, wenn
    /// [`EINSTELLUNGEN`] ihn nicht kennt.
    fn einordnung_von(name: &str) -> Option<Einordnung> {
        EINSTELLUNGEN
            .iter()
            .find(|(eintrag, _)| *eintrag == name)
            .map(|(_, einordnung)| *einordnung)
    }

    /// Die Setzer der Merkmale aus [`MERKMALSPROTOKOLL`], aus der Laufzeit.
    ///
    /// **Das ist der sachliche Schnitt, und er kommt ohne Namensform aus.**
    /// `protocol_copyPropertyList` liefert die Pflichtmerkmale des Protokolls;
    /// aus jedem Merkmalsnamen wird der Setzer nach der Regel, die Objective-C
    /// selbst anwendet, wenn eine Eigenschaft keinen eigenen Setzer nennt: `set`
    /// davor, der erste Buchstabe gross, ein Doppelpunkt dahinter. Dass keines
    /// der vierzehn einen eigenen Setzernamen fuehrt, ist nachgesehen (kein
    /// Merkmal traegt das Attribut `S`), und wenn eines es tuete, kaeme hier ein
    /// Name heraus, den die Klasse nicht traegt — die Probe nennt ihn dann als
    /// unbekannt, statt ihn zu verschweigen.
    ///
    /// **Rohes FFI, und das ist hier zulaessig.** `objc2` fuehrt die
    /// Protokoll-Aufzaehlung nur in seinem `ffi`-Modul; die sichere
    /// Schnittstelle hat sie nicht. Der Modulkopf sagt unter „Das Protokoll
    /// `NSTextInputTraits`", warum das keine Grenze verletzt.
    fn setzer_des_protokolls() -> BTreeSet<String> {
        let protokoll = AnyProtocol::get(MERKMALSPROTOKOLL).unwrap_or_else(|| {
            panic!(
                "das Protokoll {MERKMALSPROTOKOLL:?} steht auf diesem System nicht — \
                 dann ist die eine geschlossene Haelfte des Schnitts fort und die \
                 Aufzaehlung haengt allein an FORMEN"
            )
        });
        let mut anzahl: c_uint = 0;
        let liste =
            unsafe { objc2::ffi::protocol_copyPropertyList(protokoll as *const _, &mut anzahl) };
        assert!(
            !liste.is_null() && anzahl > 0,
            "{MERKMALSPROTOKOLL:?} fuehrt kein einziges Pflichtmerkmal"
        );
        let mut setzer = BTreeSet::new();
        for stelle in 0..anzahl as usize {
            let merkmal = unsafe { *liste.add(stelle) };
            let name = unsafe { CStr::from_ptr(objc2::ffi::property_getName(merkmal)) };
            setzer.insert(setzername(&name.to_string_lossy()));
        }
        unsafe { objc2::ffi::free(liste.cast()) };
        setzer
    }

    /// `setSmartQuotesType:` wird zu `smartQuotesType` — der Weg zurueck, den
    /// `valueForKey:` braucht. Die Hinrichtung ist
    /// [`setzername`](super::super::textautomatik::setzername), und die steht seit
    /// der Runde 9 in `super::textautomatik`, weil beide Seiten der Frage — die
    /// setzende dort, die lesende hier — denselben Namen bilden muessen.
    fn merkmalsname(setzer: &str) -> String {
        let kern = setzer
            .strip_prefix("set")
            .and_then(|rest| rest.strip_suffix(':'))
            .unwrap_or_else(|| panic!("{setzer} ist kein Setzer der Form set…:"));
        let mut zeichen = kern.chars();
        let erstes = zeichen.next().expect("ein Setzername hat einen Kern");
        format!("{}{}", erstes.to_lowercase(), zeichen.as_str())
    }

    /// Was diese Laufzeit an Einstellungen traegt, aus beiden Quellen.
    ///
    /// **Die Kette laeuft bis `NSObject`.** `class_copyMethodList`, das hinter
    /// `instance_methods` steht, liefert die Methoden der Klasse selbst und
    /// **keine** ererbten; eine Einstellung, die Apple statt an `NSTextView` an
    /// `NSText`, `NSView` oder `NSResponder` legt, fiele sonst stumm aus der
    /// Aufzaehlung (Defekt 260810-0751).
    fn getragene_einstellungen() -> BTreeSet<String> {
        let mut getragen = setzer_des_protokolls();
        let mut klasse =
            Some(AnyClass::get(c"NSTextView").expect("die Klasse NSTextView steht im Programm"));
        while let Some(stufe) = klasse {
            getragen.extend(
                stufe
                    .instance_methods()
                    .iter()
                    .map(|methode| methode.name().name().to_string_lossy().into_owned())
                    .filter(|name| {
                        name.starts_with("set") && FORMEN.iter().any(|form| name.ends_with(form))
                    }),
            );
            klasse = stufe.superclass();
        }
        getragen
    }

    /// Die Zusage aus C4 ist entweder vollstaendig oder sie traegt nicht: eine
    /// achte Automatik, die durchrutscht, machte die sieben abgeschalteten zu
    /// einer halben Massnahme.
    ///
    /// Die Probe fragt deshalb nicht die sieben ab, die sie kennt, sondern die
    /// Laufzeit: [`getragene_einstellungen`] zaehlt auf, was das Protokoll fuehrt
    /// und was die Vererbungskette in den [`FORMEN`] traegt, und die Probe
    /// verlangt, dass jeder Fund in [`EINSTELLUNGEN`] eine Antwort hat.
    ///
    /// **Die beiden Richtungen sind verschiedene Fragen und bekommen
    /// verschiedene Antworten** (Defekt 260810-0417). Was die Laufzeit traegt und
    /// die Aufstellung nicht kennt, haelt den Bau an: dort ist C4 offen. Was
    /// die Aufstellung kennt und die Laufzeit nicht mehr traegt, ist ein Hinweis
    /// und kein Fehlschlag — eine Einstellung, die es nicht gibt, aendert keine
    /// Zeichen, und KRK wird auf macOS 15 bis 26 unterstuetzt, waehrend diese
    /// Aufzaehlung allein das Geraet sieht, auf dem sie laeuft.
    ///
    /// **Der Hinweis geht nicht ueber `eprintln!`.** `libtest` faengt die
    /// Standardausgabe eines Tests ab und gibt sie nur bei einem Fehlschlag oder
    /// unter `--nocapture` aus; dieser Zweig laeuft genau dann, wenn der Test
    /// **nicht** fehlschlaegt, und ging deshalb auf allen begangenen Wegen ins
    /// Leere (Defekt 260810-0747). Ein Schreiben auf [`std::io::stderr`] geht am
    /// Abfang vorbei, weil der an den Druckmakros haengt und nicht am
    /// Fehlerkanal des Prozesses. Nachzustellen: eine erfundene Zeile in
    /// [`EINSTELLUNGEN`] eintragen und `cargo test` **ohne** weitere Schalter
    /// fahren; der Hinweis steht in der Ausgabe, die Reihe bleibt gruen.
    ///
    /// **Weder Flaeche noch Fenster.** Klasse, Protokoll und Selektoren stehen
    /// fuer sich, und die Aufzaehlung braucht keine Instanz. Dass die sieben
    /// Zeilen in [`textflaeche_bauen`] wirken, misst sie nicht — das misst
    /// [`die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`].
    #[test]
    fn keine_unbekannte_einstellung_steht_an_der_textflaeche() {
        let getragen = getragene_einstellungen();
        let eingeordnet: BTreeSet<String> = EINSTELLUNGEN
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect();

        let unbekannt: Vec<&str> = getragen
            .difference(&eingeordnet)
            .map(String::as_str)
            .collect();
        assert!(
            unbekannt.is_empty(),
            "diese Laufzeit traegt {} Einstellung(en), die EINSTELLUNGEN nicht kennt: \
             {unbekannt:?} — wer sie ergaenzt, beantwortet zuerst, ob sie Zeichen aendert (C4), \
             und prueft, ob sie nicht bloss eine weitere Tuer zu einer bekannten ist",
            unbekannt.len()
        );

        let verschwunden: Vec<&str> = eingeordnet
            .difference(&getragen)
            .map(String::as_str)
            .collect();
        if !verschwunden.is_empty() {
            let _ = writeln!(
                std::io::stderr(),
                "Hinweis aus {}: {verschwunden:?} steht in EINSTELLUNGEN, aber weder an der \
                 Vererbungskette von NSTextView noch in {MERKMALSPROTOKOLL:?} dieses Systems. \
                 C4 ist davon nicht beruehrt — was es nicht gibt, aendert keine Zeichen. Wer \
                 aufraeumt, streicht den Eintrag.",
                module_path!()
            );
        }
    }

    /// Die Aufstellung ist in sich stimmig: kein Name doppelt, und jeder Verweis
    /// zeigt auf Eintraege, die selbst eine Antwort tragen.
    ///
    /// Ohne diese Probe koennte ein Verweis auf einen Verweis zeigen oder ins
    /// Leere, und die Aufstellung saehe vollstaendig aus, ohne es zu sein. Drei
    /// Antworten nennen einen anderen Eintrag: [`Einordnung::ZweiteTuerZu`] mit
    /// ihrem einen Ziel, [`Einordnung::SammeltuerZu`] mit ihren mehreren und
    /// [`Einordnung::Gegenstandslos`] mit dem Traeger, dessen Abschaltung ihr den
    /// Gegenstand nimmt. Fuer die dritte ist die Pruefung die tragende Haelfte
    /// ihrer Begruendung: eine Einstellung ist nur dann gegenstandslos, wenn das
    /// Genannte wirklich aus ist.
    #[test]
    fn jeder_verweis_zeigt_auf_beantwortete_einstellungen() {
        let mut gesehen = BTreeSet::new();
        for (name, _) in EINSTELLUNGEN {
            assert!(
                gesehen.insert(name),
                "{name} steht zweimal in EINSTELLUNGEN"
            );
        }

        for (name, einordnung) in EINSTELLUNGEN {
            for ziel in ziele_von(einordnung) {
                match einordnung_von(ziel) {
                    Some(Einordnung::Abgeschaltet | Einordnung::Geduldet) => {}
                    andere => panic!(
                        "{name} zeigt auf {ziel}, und das traegt keine eigene Antwort: {andere:?}"
                    ),
                }
            }
        }
    }

    /// Die Eintraege, die eine Antwort namentlich nennt, oder nichts, wenn sie
    /// keinen nennt.
    fn ziele_von(einordnung: &'static Einordnung) -> &'static [&'static str] {
        match einordnung {
            Einordnung::ZweiteTuerZu(ziel) => std::slice::from_ref(ziel),
            Einordnung::SammeltuerZu(ziele) => ziele,
            Einordnung::Gegenstandslos(traeger) => std::slice::from_ref(traeger),
            Einordnung::Abgeschaltet | Einordnung::Geduldet | Einordnung::NochOffen(_) => &[],
        }
    }

    /// Der eine Ort, an dem eine Probe eine AppKit-Ansicht baut.
    ///
    /// **Hier steht eine Notluege, und sie steht genau hier.**
    /// `MainThreadMarker::new_unchecked` behauptet den Hauptfaden, und `libtest`
    /// fuehrt seine Proben auf eigenen Faeden. Was die Proben darunter tun,
    /// traegt die Behauptung: sie bauen eine `NSTextView`, lesen und setzen
    /// Merkmale und lassen sie fallen. Kein Fenster, keine Zeichnung, keine
    /// Ereignisschlange, kein Ersthelfer — nichts, was AppKit an den Hauptfaden
    /// bindet. Nachgemessen: sechs vollstaendige Laeufe von
    /// `cargo test --workspace` nach dem Umbau, ohne Absturz und ohne Meldung.
    /// Was das **nicht** belegt, gehoert dazu: Apple sagt fuer eine `NSView`
    /// den Hauptfaden zu, und diese Zusage nimmt die Probe nicht in Anspruch,
    /// sondern umgeht sie. Wer hier eine Probe dazulegt, die zeichnet, ein
    /// Fenster anfasst oder einen Ersthelfer setzt, verlaesst den gemessenen
    /// Bereich.
    ///
    /// **Die Sperre serialisiert sie.** Mehrere Proben, die gleichzeitig auf
    /// verschiedenen Faeden AppKit-Objekte bauen, waeren eine zweite Behauptung
    /// ueber AppKit, die niemand geprueft hat; unter der Sperre baut zu jeder
    /// Zeit hoechstens eine. Ein Fehlschlag vergiftet die Sperre, und die
    /// naechste Probe nimmt sie trotzdem: der Fehlschlag steht schon in der
    /// Reihe, und ein zweiter Name daneben verdeckte ihn nur.
    ///
    /// # Der Ausweg ist gemessen und noch nicht gebaut
    ///
    /// Die Notluege ist zu ersetzen, nicht zu rechtfertigen, und drei Wege
    /// stehen dafuer im Datensatz
    /// `issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`.
    /// Zwei Messungen vom 260810 schneiden die Wahl unter ihnen zu, beide auf
    /// macOS 15.7.7 (Build 24G720) mit Rust 1.97.1:
    ///
    /// ```text
    ///   cargo test                          MainThreadMarker::new() ─> None
    ///   cargo test -- --test-threads=1      MainThreadMarker::new() ─> None
    ///   [[test]] mit harness = false        MainThreadMarker::new() ─> Some
    /// ```
    ///
    /// **`libtest` gibt den Hauptfaden auch bei einem Prueffaden nicht her** —
    /// die naheliegende Abhilfe ist gemessen und traegt nicht. **Ein Pruefziel
    /// mit `harness = false` traegt**, und zwar ohne ein zweites Pruefkommando:
    /// `cargo test` fuehrt es mit, `make check` bleibt unveraendert. Beides ist
    /// am 260810-1139 in diesem Projekt nachgemessen und nicht uebernommen.
    ///
    /// **Und doch fehlt dem zweiten Weg mehr als zwei Dateien**, wie es hier bis
    /// zum 260810-1139 stand: `krk-ui` hat **kein Bibliotheksziel**. Die Kiste
    /// fuehrt allein `[[bin]] name = "krk"`, und eine Prueflaufdatei unter
    /// `tests/` ist eine eigene Kiste — sie erreicht nichts aus `krk-ui`, gleich
    /// ob [`textflaeche_bauen`] und [`EINSTELLUNGEN`] `pub` sind oder nicht.
    /// Gemessen als Uebersetzungsfehler `E0433`. Es fehlt also ein `src/lib.rs`
    /// samt Umbau von `main.rs`, oder ein zweiter Kistenkopf unter `src/`, der
    /// die Oberflaeche ein zweites Mal uebersetzt und `cfg(test)` verliert. Beide
    /// Wege und die geaenderte Empfehlung stehen im Datensatz
    /// `decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.
    ///
    /// Bis dahin steht die Notluege hier — nicht weil sie zulaessig waere,
    /// sondern weil der Rueckbau die vier Messungen kostete, die
    /// `260810-0748`, `260810-0750`, `260810-0746` und `260810-0512` in den Baum
    /// gebracht haben.
    fn an_einer_flaeche<T>(arbeit: impl FnOnce(MainThreadMarker) -> T) -> T {
        static SPERRE: Mutex<()> = Mutex::new(());
        let _wache = SPERRE
            .lock()
            .unwrap_or_else(|vergiftet| vergiftet.into_inner());
        arbeit(unsafe { MainThreadMarker::new_unchecked() })
    }

    /// Der Rahmen, in dem die Proben ihre Flaechen bauen. Die Groesse spielt
    /// keine Rolle: gelesen werden Merkmale und nicht Masse.
    fn probenrahmen() -> NSRect {
        NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(100.0, 100.0))
    }

    /// Liest ein Merkmal der Flaeche ueber seinen Namen.
    ///
    /// **Ueber `valueForKey:` und nicht ueber den Lesernamen.** Die Lesernamen
    /// sind nicht einheitlich — `isAutomaticQuoteSubstitutionEnabled` traegt das
    /// `is`, `smartInsertDeleteEnabled` nicht —, der Merkmalsname dagegen ist
    /// genau der Setzername ohne `set` und ohne Doppelpunkt. Ein Weg statt einer
    /// Ausnahmeliste.
    fn merkmal(flaeche: &NSTextView, merkmal: &str) -> isize {
        merkmal_falls_vorhanden(flaeche, merkmal).unwrap_or_else(|| {
            panic!(
                "die Flaeche fuehrt kein Merkmal {merkmal} — sie kennt {} nicht",
                setzername(merkmal)
            )
        })
    }

    /// Dasselbe wie [`merkmal`], aber ohne Abbruch: `None`, wenn diese Laufzeit
    /// den Namen nicht fuehrt.
    ///
    /// **Der Unterschied ist, wer den Namen fuehrt** (Defekt 260810-1246). Kommt
    /// er aus [`EINSTELLUNGEN`], ist ein fehlender Name die Meldung, dass KRKs
    /// Aufstellung nachzuziehen ist, und der Abbruch ist die richtige Antwort.
    /// Kommt er von Apple und steht nicht auf der Untergrenze, die das Projekt
    /// zusagt, faerbte er `cargo test` auf einem unterstuetzten System rot, ohne
    /// dass am ausgelieferten Code etwas falsch waere — dort gilt der Zuschnitt,
    /// den `260810-0417` fuer die Nachbarprobe gewaehlt hat: Hinweis statt
    /// Fehlschlag.
    ///
    /// # Gefragt wird vorher und nicht am Ergebnis
    ///
    /// **`valueForKey:` liefert fuer einen unbekannten Schluessel nicht `nil`**,
    /// sondern laeuft in `valueForUndefinedKey:` und wirft
    /// `NSUnknownKeyException`. Gemessen am 260810 auf macOS 15.7.7 (Build 24G720),
    /// an einer `NSTextView` in Swift: der Prozess endet mit Signal 6 und der
    /// Meldung "this class is not key value coding-compliant for the key …". Eine
    /// Objective-C-Ausnahme ist in Rust nicht zu fangen; sie beendet das **ganze**
    /// Pruefprogramm und nicht die eine Probe. Ein `Option` am Rueckgabewert
    /// allein haette den Defekt deshalb nicht behoben, und der Datensatz nennt den
    /// Abbruch eine Panik, was ihn zu harmlos beschreibt.
    ///
    /// **Gefragt wird nach dem Setzer und nicht nach dem Leser.** Die Lesernamen
    /// sind nicht einheitlich — `automaticQuoteSubstitutionEnabled` liest sich
    /// `isAutomaticQuoteSubstitutionEnabled` —, und `valueForKey:` sucht der Reihe
    /// nach mehrere Formen ab; eine Frage nach dem blossen Merkmalsnamen meldete
    /// die Haelfte der Aufstellung als fehlend. Der Setzer ist die eine Form, die
    /// dieses Modul ohnehin als kanonisch fuehrt, und ein Merkmal ohne Setzer gibt
    /// es an keiner dieser Klassen.
    fn merkmal_falls_vorhanden(flaeche: &NSTextView, merkmal: &str) -> Option<isize> {
        let setzer =
            CString::new(setzername(merkmal)).expect("ein Setzername traegt kein Nullbyte");
        if !flaeche.respondsToSelector(Sel::register(&setzer)) {
            return None;
        }
        let schluessel = NSString::from_str(merkmal);
        let wert: Option<Retained<NSNumber>> =
            unsafe { msg_send![flaeche, valueForKey: &*schluessel] };
        Some(
            wert.expect("ein Merkmal mit Setzer antwortet auf valueForKey:")
                .integerValue(),
        )
    }

    /// Setzt ein Merkmal der Flaeche ueber seinen Namen. Gegenstueck zu
    /// [`merkmal`].
    fn merkmal_setzen(flaeche: &NSTextView, merkmal: &str, wert: isize) {
        let schluessel = NSString::from_str(merkmal);
        let zahl = NSNumber::new_isize(wert);
        let _: () = unsafe { msg_send![flaeche, setValue: &*zahl, forKey: &*schluessel] };
    }

    /// Der Wert, auf dem eine abgeschaltete Einstellung steht: `NO` an einer
    /// booleschen Tuer, `No` an einer dreiwertigen.
    ///
    /// **Die Unterscheidung ist vollstaendig und hat keinen Auffangzweig**, und
    /// jede Form traegt einen **eigenen** Aus-Wert: ein stiller Rueckfall auf `No`
    /// haette bei `set…Behavior:` eine falsche Erwartung gemessen und sie als
    /// Fehlschlag der Flaeche gemeldet — dort ist das Aus die `-1` von
    /// [`NSWritingToolsBehavior::None`] und nicht die `1`.
    ///
    /// Vier Formen stehen heute hier, und `Abgeschaltet` tragen neun
    /// Einstellungen: fuenf `set…Enabled:`, zwei `set…Type:`, ein `set…Behavior:`
    /// und ein `set…Affordance:`. Die Ziele der Tueren sind alle `set…Enabled:`.
    ///
    /// **`Options:` fehlt hier mit Absicht.** Die beiden Bitmasken der
    /// Schreibwerkzeuge tragen keinen Wert, der nichts zulaesst — ihre Null heisst
    /// "das System waehlt" —, und darum stehen sie als
    /// [`Einordnung::Gegenstandslos`] und nicht als `Abgeschaltet`. Wer sie
    /// umtruege, bekaeme hier den Abbruch, und das ist die richtige Antwort: es
    /// waere ein Aus-Wert zu erfinden, den Apple nicht fuehrt.
    fn aus_bedeutet(setzer: &str) -> isize {
        if setzer.ends_with("Enabled:") || setzer.ends_with("Affordance:") {
            0
        } else if setzer.ends_with("Type:") {
            NSTextInputTraitType::No.0
        } else if setzer.ends_with("Behavior:") {
            NSWritingToolsBehavior::None.0
        } else {
            panic!(
                "{setzer} traegt keine der vier Formen, deren Aus-Wert hier bekannt ist. \
                 Wer eine Einstellung der Formen Types: oder Options: auf Abgeschaltet \
                 setzt, traegt ihren Aus-Wert zuerst hier ein — sie ist nicht `No`, und \
                 bei den Bitmasken der Schreibwerkzeuge gibt es sie nicht"
            )
        }
    }

    /// Die sieben Zeilen aus [`textautomatik::automatiken_abschalten`] wirken,
    /// und das steht nicht mehr allein in der Prosa.
    ///
    /// **Drei Flaechen, ein Zeuge.** Gemessen wird an den **zwei** bearbeitbaren
    /// Flaechen, die KRK baut — der des Editors aus [`textflaeche_bauen`] und der
    /// des Notizzettels aus
    /// [`blaetter::zettel::textflaeche_bauen`](crate::appkit::blaetter::zettel::textflaeche_bauen)
    /// —, und beide werden gegen denselben Zeugen gestellt: eine frisch erzeugte
    /// `NSTextView`. An den unseren steht jede abgeschaltete Einstellung aus, am
    /// Zeugen jede **anders**. Die zweite Haelfte ist die tragende: ohne sie liefe
    /// die Probe gruen durch, wenn eine Einstellung ab Werk schon aus waere und
    /// die Zeile fehlte.
    ///
    /// # Warum hier zwei Flaechen stehen und die Aufstellung trotzdem einmal
    ///
    /// Seit der Runde 9 lautet die Aussage „jede bearbeitbare Flaeche in KRK" und
    /// nicht mehr „die Flaeche des Editors" (C3 der Runde 9). Beide Flaechen
    /// rufen dieselbe Abschaltung, und [`EINSTELLUNGEN`] steht deshalb weiter an
    /// **einer** Stelle: eine zweite Aufstellung fuer den Zettel koennte von
    /// dieser abweichen, und dann sagte jede der beiden Proben etwas ueber eine
    /// andere Liste. Die Schleife ueber die zwei Flaechen ist die billigere
    /// Haelfte, die Aufstellung die teure.
    ///
    /// **Was die Probe nicht sieht:** eine dritte bearbeitbare Flaeche einer
    /// spaeteren Runde, die die Abschaltung nicht ruft. Der Bau haelt dabei nicht
    /// an; ob er es kuenftig tut, ist als Frage gefilt
    /// (`decisions/260814-0656_*_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md`).
    ///
    /// Die Aufstellung liefert die Namen. Wer eine zehnte Einstellung als
    /// `Abgeschaltet` eintraegt, ohne die Zeile in
    /// [`textautomatik::automatiken_abschalten`] zu schreiben, bekommt hier den
    /// Fehlschlag — und nicht erst der Nutzer am laufenden Buendel. **Die Zeile
    /// gehoert dorthin und nicht in [`textflaeche_bauen`]**: dort geschrieben
    /// bekaeme sie allein der Editor, und der Zettel stuende ohne sie da — zwei
    /// Wahrheiten darueber, was „abgeschaltet" heisst.
    ///
    /// # Eine Einstellung, die diese Laufzeit nicht fuehrt, ist ein Hinweis
    ///
    /// Neun sind es seit dem 260810, und die neunte —
    /// `setAllowsWritingToolsAffordance:` — ist die erste, die **oberhalb der
    /// Untergrenze** des Buendels liegen kann: das SDK fuehrt sie erst ab macOS
    /// 15.4 und nur an `NSTextField`, das Buendel zielt auf 15.0. Deshalb liest
    /// diese Probe ueber [`merkmal_falls_vorhanden`] und nicht ueber [`merkmal`],
    /// dessen Abbruch das **ganze** Pruefprogramm beendete.
    ///
    /// **Fehlt sie, faellt die Probe nicht.** Eine Einstellung, die es nicht gibt,
    /// aendert kein Zeichen; C4 ist von ihr nicht beruehrt. Das ist derselbe
    /// Zuschnitt, den `260810-0417` fuer die Gegenrichtung des Stolperdrahts
    /// gewaehlt hat — Hinweis statt Fehlschlag —, und aus demselben Grund geht der
    /// Hinweis ueber [`std::io::stderr`] und nicht ueber `eprintln!`.
    #[test]
    fn die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus() {
        let abgeschaltet: Vec<&str> = EINSTELLUNGEN
            .iter()
            .filter(|(_, einordnung)| *einordnung == Einordnung::Abgeschaltet)
            .map(|(name, _)| *name)
            .collect();
        assert!(
            !abgeschaltet.is_empty(),
            "ohne eine abgeschaltete Einstellung misst diese Probe nichts"
        );

        an_einer_flaeche(|mtm| {
            let (_rolle, editorflaeche) = textflaeche_bauen(mtm, probenrahmen());
            let (_bildlauf, zettelflaeche) =
                crate::appkit::blaetter::zettel::textflaeche_bauen(mtm, probenrahmen());
            let unsere = [
                ("die Flaeche des Editors", editorflaeche),
                ("die Flaeche des Notizzettels", zettelflaeche),
            ];
            let frische = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
            for setzer in abgeschaltet {
                let name = merkmalsname(setzer);
                let aus = aus_bedeutet(setzer);
                let mut gemessen = false;
                for (woher, flaeche) in &unsere {
                    let Some(unser_wert) = merkmal_falls_vorhanden(flaeche, &name) else {
                        continue;
                    };
                    gemessen = true;
                    assert_eq!(
                        unser_wert, aus,
                        "{setzer} steht an {woher} nicht auf aus — C4 verlangt, dass der \
                         gesicherte Stand der getippte ist, und C3 der Runde 9, dass es \
                         fuer jede bearbeitbare Flaeche gilt"
                    );
                }
                if !gemessen {
                    let _ = writeln!(
                        std::io::stderr(),
                        "Hinweis aus {}: diese Laufzeit fuehrt {setzer} nicht. Was es nicht \
                         gibt, aendert kein Zeichen; C4 ist davon nicht beruehrt. Die \
                         Untergrenze des Buendels ist macOS 15.0.",
                        module_path!()
                    );
                    continue;
                }
                assert_ne!(
                    merkmal(&frische, &name),
                    aus,
                    "{setzer} steht schon ab Werk auf aus; dann sagt diese Probe ueber die \
                     Zeile in der Abschaltung nichts, und der Vergleich braucht einen \
                     anderen Zeugen"
                );
            }
        });
    }

    /// Die beiden gegenstandslosen Einstellungen stehen an KRKs Flaeche
    /// **unberuehrt**, und die Faehigkeit, die sie beschreiben, steht aus.
    ///
    /// **Das ist die Probe, die [`Einordnung::Gegenstandslos`] von
    /// [`Einordnung::ZweiteTuerZu`] trennt.** Eine zweite Tuer wird vom Genannten
    /// umgelegt — das misst [`jede_zweite_tuer_und_ihre_erste_legen_einander_um`].
    /// Eine gegenstandslose Einstellung wird **nicht** umgelegt: gemessen am
    /// 260810 auf macOS 15.7.7 laesst `setWritingToolsBehavior(None)` beide
    /// Bitmasken auf ihrer Null stehen. Sie verliert nur ihren Gegenstand, weil
    /// die Faehigkeit, deren Ergebnisformen sie beschreibt, nicht mehr laeuft.
    ///
    /// Zwei Behauptungen haelt sie fest:
    ///
    /// 1. KRKs Flaeche traegt denselben Wert wie eine frische: [`textflaeche_bauen`]
    ///    setzt sie nicht, und das ist Absicht und kein Vergessen.
    /// 2. Dieser Wert ist die Null, also der Vorgabewert "das System waehlt". Waere
    ///    er es einmal nicht, waere die Begruendung "es gibt keinen Aus-Wert"
    ///    nachzupruefen.
    ///
    /// Die dritte — dass der genannte Traeger wirklich aus ist — steht nicht hier,
    /// sondern in [`jeder_verweis_zeigt_auf_beantwortete_einstellungen`], das sie
    /// fuer alle drei verweisenden Antworten in einem Zug prueft.
    #[test]
    fn die_gegenstandslosen_stehen_unberuehrt_und_ihr_traeger_steht_aus() {
        let gegenstandslos: Vec<&str> = EINSTELLUNGEN
            .iter()
            .filter(|(_, einordnung)| matches!(einordnung, Einordnung::Gegenstandslos(_)))
            .map(|(name, _)| *name)
            .collect();
        assert!(
            !gegenstandslos.is_empty(),
            "ohne eine gegenstandslose Einstellung misst diese Probe nichts"
        );

        an_einer_flaeche(|mtm| {
            let (_rolle, unsere) = textflaeche_bauen(mtm, probenrahmen());
            let frische = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
            for setzer in gegenstandslos {
                let name = merkmalsname(setzer);
                let Some(unser_wert) = merkmal_falls_vorhanden(&unsere, &name) else {
                    let _ = writeln!(
                        std::io::stderr(),
                        "Hinweis aus {}: diese Laufzeit fuehrt {setzer} nicht. Eine \
                         Einstellung, die es nicht gibt, laesst auch nichts zu.",
                        module_path!()
                    );
                    continue;
                };
                assert_eq!(
                    unser_wert,
                    merkmal(&frische, &name),
                    "{setzer} steht an KRKs Flaeche anders als an einer frischen — dann setzt \
                     textflaeche_bauen sie doch, und sie gehoert nicht mehr auf \
                     Gegenstandslos"
                );
                assert_eq!(
                    unser_wert, 0,
                    "{setzer} steht nicht mehr auf seinem Vorgabewert Null. Die Einordnung \
                     Gegenstandslos haengt daran, dass die Null 'das System waehlt' heisst \
                     und es keinen Wert gibt, der nichts zulaesst — das ist nachzupruefen"
                );
            }
        });
    }

    /// Die gebaute Flaeche steht auf TextKit 1, und daran haengt der Rueckweg des
    /// Rueckgaengig (Defekt 260810-1243).
    ///
    /// **Die Reihenfolge der beiden Fragen ist die ganze Probe.** Ein Zugriff auf
    /// `layoutManager` **loest** den Rueckfall aus; wer ihn zuerst fragte, machte
    /// seine eigene Antwort wahr und misst nichts. Gefragt wird deshalb zuerst
    /// `textLayoutManager`: steht der noch, ist die Flaeche auf TextKit 2 und
    /// `textDidChange:` feuert bei einem `undo` nicht.
    ///
    /// **Was ausfaellt, wenn jemand die Zeile in [`textflaeche_bauen`] wegnimmt:**
    /// diese Probe, und sonst nichts. Der Bau bliebe gruen, alle uebrigen Proben
    /// blieben gruen — keine von ihnen faehrt ein `undo` an einer Flaeche in einem
    /// Fenster —, und der Nutzer bekaeme ein `cmd+s`, das den zurueckgenommenen
    /// Text sichert, weil `Editormodell::sichern` den nie nachgezogenen Stand
    /// schreibt.
    ///
    /// Sie liest zwei Merkmale und baut kein Fenster; damit bleibt sie in dem
    /// Bereich, den [`an_einer_flaeche`] gemessen hat.
    #[test]
    fn die_gebaute_flaeche_steht_auf_textkit_1() {
        an_einer_flaeche(|mtm| {
            let (_rolle, unsere) = textflaeche_bauen(mtm, probenrahmen());
            assert!(
                unsere.textLayoutManager().is_none(),
                "die Flaeche aus textflaeche_bauen steht auf TextKit 2. Dann verschickt ein \
                 `undo` kein textDidChange:, der Rueckweg ins Editormodell faellt aus, und ein \
                 cmd+s sichert den zurueckgenommenen Text (260810-1243). Der Rueckfall auf \
                 TextKit 1 gehoert in textflaeche_bauen und ist dort begruendet"
            );
            // SAFETY: Der Verwalter gehoert der Flaeche; er wird nur erfragt.
            assert!(
                unsafe { unsere.layoutManager() }.is_some(),
                "die Flaeche fuehrt weder den einen noch den anderen Layoutverwalter"
            );
        });
    }

    /// Die Kopplung der zehn Paare, nachgemessen statt behauptet.
    ///
    /// **Daran haengt die Entscheidung, `textflaeche_bauen` nicht um zehn Zeilen
    /// zu ergaenzen**, und im Baum hielt sie vorher nichts: das Messprogramm war
    /// nirgends abgelegt, und die beiden Aufstellungsproben pruefen die
    /// Aufstellung gegen sich selbst
    /// (`issues/260810-0748_*_die-kopplung-der-zehn-paare-traegt-den-commit-und-ist-im-baum-durch-nichts-gehalten.md`).
    /// Entkoppelt eine spaetere Fassung von macOS ein Paar, haelt diese Probe den
    /// Bau an, statt gruen zu bleiben.
    ///
    /// Je Paar zwei Richtungen und je Richtung eine eigene Flaeche, damit die
    /// zweite Messung nicht auf dem Ergebnis der ersten sitzt.
    #[test]
    fn jede_zweite_tuer_und_ihre_erste_legen_einander_um() {
        let paare: Vec<(&str, &str)> = EINSTELLUNGEN
            .iter()
            .filter_map(|(name, einordnung)| match einordnung {
                Einordnung::ZweiteTuerZu(erste) => Some((*name, *erste)),
                _ => None,
            })
            .collect();
        assert_eq!(paare.len(), 10, "die Aufstellung fuehrt zehn Paare");

        an_einer_flaeche(|mtm| {
            for (zweite, erste) in paare {
                let zweite_tuer = merkmalsname(zweite);
                let erste_tuer = merkmalsname(erste);

                let hin = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
                merkmal_setzen(&hin, &erste_tuer, aus_bedeutet(erste));
                assert_eq!(
                    merkmal(&hin, &zweite_tuer),
                    NSTextInputTraitType::No.0,
                    "{erste} auf aus laesst {zweite} nicht auf No stehen — die beiden sind \
                     entkoppelt, und dann braucht {zweite} eine eigene Zeile in \
                     textflaeche_bauen"
                );

                let her = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
                merkmal_setzen(&her, &zweite_tuer, NSTextInputTraitType::No.0);
                assert_eq!(
                    merkmal(&her, &erste_tuer),
                    aus_bedeutet(erste),
                    "{zweite} auf No laesst {erste} nicht auf aus stehen — die Kopplung \
                     traegt nur in eine Richtung, und die Aufstellung behauptet beide"
                );
            }
        });
    }

    /// Zwei Tueren zu einer Einstellung sind nicht derselbe Speicher, und diese
    /// Probe haelt den Unterschied fest.
    ///
    /// **„Derselbe Speicher" war eine Stufe zu stark** (Defekt 260810-0750).
    /// `NSTextInputTraitType` hat drei Werte, der Wahrheitswert zwei, und zwei
    /// Messungen zeigen, dass die erste Tuer den dritten nicht fuehrt:
    ///
    /// 1. **Sie zeigt ihn nicht.** Steht die zweite Tuer auf `Default`, liest die
    ///    erste eine Systemvorgabe, und die faellt je Einstellung anders aus.
    ///    Waeren es dieselben Bits, gaebe es diesen Auflösungsschritt nicht.
    /// 2. **Sie stellt ihn nicht her.** Schreibt man den eben gelesenen
    ///    Wahrheitswert unveraendert zurueck, steht die zweite Tuer danach auf
    ///    `Yes` oder `No` und nie wieder auf `Default`.
    ///
    /// Die erste Messung prueft die Probe daran, dass die Vorgabe **nicht bei
    /// allen Paaren gleich** ausfaellt; welche zwei aus der Reihe fallen, ist
    /// eine Systemeigenschaft und nicht Gegenstand einer Zusicherung.
    #[test]
    fn die_erste_tuer_kann_default_weder_herstellen_noch_anzeigen() {
        let paare: Vec<(&str, &str)> = EINSTELLUNGEN
            .iter()
            .filter_map(|(name, einordnung)| match einordnung {
                Einordnung::ZweiteTuerZu(erste) => Some((*name, *erste)),
                _ => None,
            })
            .collect();

        let vorgaben = an_einer_flaeche(|mtm| {
            let mut vorgaben = Vec::new();
            for (zweite, erste) in paare {
                let zweite_tuer = merkmalsname(zweite);
                let erste_tuer = merkmalsname(erste);

                let flaeche = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
                merkmal_setzen(&flaeche, &zweite_tuer, NSTextInputTraitType::Default.0);
                let gelesen = merkmal(&flaeche, &erste_tuer);
                merkmal_setzen(&flaeche, &erste_tuer, gelesen);
                assert_ne!(
                    merkmal(&flaeche, &zweite_tuer),
                    NSTextInputTraitType::Default.0,
                    "{erste} hat {zweite} auf Default zurueckgestellt — dann waere die erste \
                     Tuer doch dieselbe Sache wie die zweite, und der Modulkopf sagt das \
                     Gegenteil"
                );
                vorgaben.push(gelesen);
            }
            vorgaben
        });

        assert!(
            vorgaben.iter().any(|wert| *wert != vorgaben[0]),
            "alle Paare loesen Default zum selben Wahrheitswert auf ({vorgaben:?}) — dann \
             traegt die Messung die Aussage nicht mehr, dass die Vorgabe je Einstellung \
             verschieden ausfaellt"
        );
    }

    /// Die Sammeltuer sieht auf dieselben Bits, die die einzelnen Zeilen legen.
    ///
    /// **Das ist die dritte Tuer** und die einzige, die mehrere Automatiken auf
    /// einmal umlegt (Defekt 260810-0746). Zwei Messungen halten sie:
    ///
    /// 1. Die Maske an der Flaeche aus [`textflaeche_bauen`] hat gegenueber einer
    ///    frischen Flaeche **nur Bits verloren** und keines dazugewonnen. Die
    ///    sieben Zeilen legen also dieselben Bits, die die Maske fuehrt.
    /// 2. Setzt man die Maske an unserer Flaeche auf den Werkswert zurueck, so
    ///    aendert das jede Einstellung, die die Aufstellung als Ziel der
    ///    Sammeltuer nennt. Deshalb wird sie in KRK nicht gesetzt.
    ///
    /// Die Zahlenwerte der Maske stehen bewusst nirgends: gemessen wird der
    /// Unterschied zwischen zwei Flaechen und nicht eine Konstante von Apple.
    #[test]
    fn die_sammeltuer_ist_eine_sicht_auf_dieselben_bits() {
        let (sammeltuer, ziele) = EINSTELLUNGEN
            .iter()
            .find_map(|(name, einordnung)| match einordnung {
                Einordnung::SammeltuerZu(ziele) => Some((*name, *ziele)),
                _ => None,
            })
            .expect("die Aufstellung fuehrt eine Sammeltuer");
        assert_eq!(
            sammeltuer, "setEnabledTextCheckingTypes:",
            "diese Probe kennt die Maske dieser einen Sammeltuer"
        );

        an_einer_flaeche(|mtm| {
            let (_rolle, unsere) = textflaeche_bauen(mtm, probenrahmen());
            let frische = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
            let werkswert = frische.enabledTextCheckingTypes();
            let unsere_maske = unsere.enabledTextCheckingTypes();
            assert_ne!(
                unsere_maske, werkswert,
                "die sieben Zeilen lassen die Maske unberuehrt — dann fuehrt sie andere \
                 Bits als die Einstellungen, die die Aufstellung ihr zuschreibt"
            );
            assert_eq!(
                unsere_maske & !werkswert,
                0,
                "unsere Maske traegt ein Bit, das die frische nicht hat — die sieben Zeilen \
                 schalten dann etwas ein"
            );

            let vorher: Vec<isize> = ziele
                .iter()
                .map(|ziel| merkmal(&unsere, &merkmalsname(ziel)))
                .collect();
            unsere.setEnabledTextCheckingTypes(werkswert);
            for (ziel, alt) in ziele.iter().zip(vorher) {
                assert_ne!(
                    merkmal(&unsere, &merkmalsname(ziel)),
                    alt,
                    "der Werkswert der Maske laesst {ziel} unberuehrt — dann ist {ziel} kein \
                     Ziel der Sammeltuer und die Aufstellung nennt es zu Unrecht"
                );
            }
        });
    }

    /// Der Vorgabewert der Schreibwerkzeuge ueberlaesst dem System die Wahl, und
    /// ihre Angebotsflaeche steht ab Werk an — **an einer frischen `NSTextView`**.
    ///
    /// **Gemessen und nicht der Dokumentation entnommen** (Defekt 260810-0512, der
    /// den Wert als `speculation:` gefuehrt hat). Das war der Grund, aus dem die
    /// Lesart von C4 ueberhaupt eine Frage war: waeren die Schreibwerkzeuge ab Werk
    /// aus, haette es nichts zu entscheiden gegeben.
    ///
    /// **Seit dem 260810 misst diese Probe die frische Flaeche und nicht mehr
    /// KRKs.** Die Frage ist gegen die Schreibwerkzeuge entschieden
    /// (`decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`), und an
    /// KRKs Flaeche stehen beide jetzt aus — das haelt
    /// [`die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`] fest. Was hier
    /// bleibt, ist die Aussage darunter: dass die beiden Zeilen in
    /// [`textflaeche_bauen`] etwas **aendern** und nicht einen Werkswert
    /// wiederholen. Dieselbe Aussage steckt in der zweiten Haelfte jener Probe;
    /// hier steht sie mit den Namen der Werte statt als Ungleichheit, weil der
    /// Werkswert die gemessene Groesse ist, auf die sich der Entscheidungsdatensatz
    /// beruft.
    ///
    /// # Die beiden Merkmale sind nicht von einer Art, und die Probe behandelt sie
    /// verschieden
    ///
    /// Der Defekt 260810-1246 hat den Unterschied an den Kopfdateien belegt:
    ///
    /// ```text
    ///   writingToolsBehavior            NSTextView.h:434   macos(15.0)
    ///   allowedWritingToolsResultOptions NSTextView.h:435  macos(15.0)
    ///   writingToolsAllowedInputOptions  in keiner Kopfdatei
    ///   allowsWritingToolsAffordance     nur an NSTextField, macos(15.4)
    /// ```
    ///
    /// **Nur das erste steht auf der Untergrenze, die das Projekt zusagt**, und nur
    /// es ist deshalb eine Zusicherung. Die Angebotsflaeche fuehrt das SDK allein
    /// an `NSTextField` und erst ab macOS 15.4; an `NSTextView` antwortet die
    /// Laufzeit von 15.7.7, aber undokumentiert. Sie unbedingt zu lesen band den
    /// Bau wieder an die Fassung des pruefenden Geraets — auf 15.0 bis 15.3, die
    /// KRK unterstuetzt, oder sobald Apple den Zugang fortnimmt —, und zwar nicht
    /// mit einem Fehlschlag, sondern mit dem Abbruch des ganzen Pruefprogramms;
    /// siehe [`merkmal_falls_vorhanden`].
    ///
    /// **Fehlt sie, steht ein Hinweis und kein Fehlschlag.** Er geht ueber
    /// [`std::io::stderr`] und nicht ueber `eprintln!`, aus demselben Grund wie bei
    /// [`keine_unbekannte_einstellung_steht_an_der_textflaeche`]: `libtest` faengt
    /// die Druckmakros ab und gibt sie bei einer gruenen Probe nicht aus.
    #[test]
    fn der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl() {
        an_einer_flaeche(|mtm| {
            let frische = NSTextView::initWithFrame(NSTextView::alloc(mtm), probenrahmen());
            assert_eq!(
                merkmal(&frische, "writingToolsBehavior"),
                NSWritingToolsBehavior::Default.0,
                "eine frische NSTextView steht nicht mehr auf Default. Stuende sie ab Werk \
                 auf None, waere die Zeile in automatiken_abschalten ueberfluessig und die \
                 Entscheidung aus dem Datensatz gegenstandslos geworden"
            );
            match merkmal_falls_vorhanden(&frische, "allowsWritingToolsAffordance") {
                Some(angebotsflaeche) => assert_ne!(
                    angebotsflaeche, 0,
                    "die Angebotsflaeche steht an einer frischen Flaeche schon aus — dann \
                     nimmt die Zeile in automatiken_abschalten nichts fort, und der Grund, aus \
                     dem der Datensatz sie mitfuehrt, ist ein anderer geworden"
                ),
                None => {
                    let _ = writeln!(
                        std::io::stderr(),
                        "Hinweis aus {}: diese Laufzeit fuehrt allowsWritingToolsAffordance an \
                         NSTextView nicht. Das SDK fuehrt sie nur an NSTextField und erst ab \
                         macOS 15.4; die Untergrenze des Buendels ist 15.0. C4 ist davon nicht \
                         beruehrt — was es nicht gibt, bietet nichts an. Wer aufraeumt, streicht \
                         den Eintrag aus EINSTELLUNGEN.",
                        module_path!()
                    );
                }
            }
        });
    }

    /// Die beiden Defekte, an der Stelle festgehalten, an der sie entstanden
    /// sind: die Aufzaehlung hoerte erst nach den vier tippenden Automatiken auf
    /// (260809-1650) und fragte danach nach einer Namensform, die zwei weitere
    /// nicht tragen (260810-0416).
    #[test]
    fn die_drei_nachgereichten_automatiken_stehen_unter_den_abgeschalteten() {
        for name in [
            "setSmartInsertDeleteEnabled:",
            "setInlinePredictionType:",
            "setMathExpressionCompletionType:",
        ] {
            assert_eq!(
                einordnung_von(name),
                Some(Einordnung::Abgeschaltet),
                "{name} bringt Zeichen in den Text, die niemand getippt hat"
            );
        }
    }
}
