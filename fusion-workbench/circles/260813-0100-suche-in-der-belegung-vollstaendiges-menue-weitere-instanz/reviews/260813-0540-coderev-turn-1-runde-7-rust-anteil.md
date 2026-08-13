# Codeprüfung: Turn 1 der siebten Runde, Rust-Anteil

**Sender:** coderev
**Reviewed-range:** `ca66c39..40b5fb0`
**Not-opened:** `resources/default-keymap.toml`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/decisions/260813-0430_o_wer-bekommt-das-menuekuerzel-wenn-zwei-funktionen-sich-eine-kombination-teilen.md`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0325-coder-s1-bis-s3-zulaessigkeit.md`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0426-coder-s7-bis-s10-suche-in-der-belegung.md`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0445-coder-s4-bis-s6-vollstaendiges-menue.md`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0520-ontocoder-s15-weitere-instanz-in-der-belegung.md`, `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/history/260813-0620-coder-s11-bis-s14-weitere-instanz.md`
**Datum:** 260813-0540
**Zu Not-opened:** Die Belegungsdatei hat der Auftrag ausgenommen; `ontorev` hat sie in derselben Spanne geprüft (`reviews/260813-0532-ontorev-belegungsdatei-weitere-instanz.md`). Die fünf Verlaufsdateien und der Datensatz `260813-0430` sind Aufzeichnungen und kein Code.
**Umfang:** die 30 Rust-Dateien der Spanne, fünf Commits, fünfzehn Planschritte. Die
Belegungsdatei prüft `ontorev` parallel (`reviews/260813-0532-ontorev-…`).

---

## Zusammenfassung

Die fünf Commits liefern, was der Plan verlangt, und der Baum ist grün: `cargo fmt --all
--check`, `cargo clippy --workspace --all-targets` und `cargo test --workspace` laufen am
260813-0540 über 19 Ziele ohne einen Fehlschlag und ohne eine Warnung durch. Die sechs Punkte,
auf die es nach der Beauftragung besonders ankam, halten alle sechs am Baum nachgelesen: die
Zulässigkeitsfrage hat genau eine Antwortstelle mit zwei Frägern, der Ereignisabgriff kennt
den Editor nicht mehr, die Wache vor dem Sprungmarkenpuffer ist gebaut, die Schreibsperre
umfasst den ganzen Lesen-Ändern-Schreiben-Durchgang und wird beim Absturz freigegeben, die
Lesezeichenänderung nennt ihr Ziel begründet als Eintrag, und ein `tag`, den niemand gesetzt
hat, führt weder zu einem Absturz noch zu einem falschen Befehl.

Kein Befund ist ein Freigabehindernis. Was bleibt, sind sechzehn Punkte in zwei Klassen:
**Wachen, die weniger halten als ihre Beschriftung verspricht** — das ist das durchgehende
Muster dieser Runde —, und **Prosa, die den Baum nicht mehr trifft**. Ein Punkt gehört dem
Nutzer und nicht dem `coder`: der bindende Entscheidungsdatensatz zur Ablage sagt „keine
verlorene Änderung an Lesezeichen **und Belegung**" zu, gebaut ist der frische Durchgang nur
für die Lesezeichen.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch (Freigabehindernis, Sicherheit, Datenverlust) | 0 |
| Hoch (Korrektheitsfehler, gebrochener Ablauf) | 0 |
| Mittel | 6 |
| Gering | 10 |

Alle sechzehn liegen als eigene Datensätze unter
`circles/260813-0100-…/issues/`, jeder mit `260813-0540` im Namen.

---

## Was geprüft und in Ordnung befunden ist

Diese sechs Punkte sind einzeln am Baum verfolgt worden, nicht aus dem Plan übernommen.

**1. Die Zulässigkeitsfrage hat eine Antwortstelle, und es gibt keinen zweiten Weg.**
`zulaessig` ist im Baum einmal erklärt (`crates/krk-ui/src/kommandos/zulaessigkeit.rs:114`)
und wird von genau zwei Stellen gerufen: `validateMenuItem:`
(`crates/krk-ui/src/appkit/anwendung.rs:740`) und `kommando_ausfuehren` (`:2586`). Beide fragen
dieselbe `Lage` aus `Self::lage()` (`:2544-2556`), also dieselbe Erhebung desselben
Augenblicks; sie können nicht auseinanderlaufen. Der Doppelbau ist wirklich beseitigt: die drei
eigenen Selektoren `beenden:`, `fensterEinblenden:` und `fensterSchliessen:` sind fort, und alle
drei Kommandos tragen einen eigenen Zweig in `kommando_ausfuehren` (`:2651-2656`) statt durch
den Auffangzweig zu fallen.

Die 140-Felder-Tafel trägt, was sie behauptet, und zwar wegen einer zweiten Probe daneben:
`jeder_stellvertreter_traegt_den_bereich_den_er_vertritt`
(`crates/krk-ui/src/kommandos/zulaessigkeit.rs:242-261`) hält fest, dass keiner der sieben
Stellvertreter auf der Ausnahmeliste steht oder während eines Blattes durchkommt. Ohne diese
Zusicherung könnten die drei abweisenden Viertel der Tafel grün sein, ohne dass die Regel sie
trägt. Das ist sauber gedacht.

**2. Der Abgriff kennt den Editor nicht mehr — nachgeprüft, nicht geglaubt.**
`Tastenabgriff::einrichten` nimmt weder `ist_editorflaeche` noch den `MainThreadMarker`
entgegen (`crates/krk-ui/src/appkit/ereignisse.rs:294-300`), `behandeln` fragt an keiner Stelle
mehr nach dem Ersthelfer (`:495-544`), und `ersthelfer_gehoert_appkit` hat genau eine
Aufrufstelle, `Anwendungsdelegierter::lage` (`anwendung.rs:2552`). Die Nämlichkeitsfrage steht
weiterhin als Abschluss beim Delegierten, der die Fläche hält.

**3. Die Wache vor dem Sprungmarkenpuffer ist gebaut.** Der Zeichenzweig von
`eingabe_ausfuehren` weist ab, sobald ein Blatt steht **oder** der Ersthelfer AppKit gehört
(`crates/krk-ui/src/appkit/anwendung.rs:2491-2494`), und erst danach entscheidet der Fokus,
wohin das Zeichen geht. Über alle drei Ausgänge des Nachschlags nachgerechnet bleibt das
Verhalten dasselbe wie vor der Runde: `Funktion` mit Kommando läuft in `zulaessig`,
`Sprungmarke` in diese Abfrage, `Unbelegt` unverändert an AppKit.

**4. Die zwei Sperren tragen, und es führt heute kein Schreibweg an ihnen vorbei.** Alle
Aufrufstellen von `atomar::schreiben` sind einzeln nachgelesen: drei liegen hinter einem
`Zugang` (`ablage/mod.rs:457`, `:493`, `ablage/einstellungen.rs:184`), zwei schreiben außerhalb
des Ablageordners (`belegungsausgabe.rs:456`, `text/datei.rs:545`). Die Schreibsperre umfasst
den **ganzen** Durchgang: `Ablage::durchgang` nimmt den Griff vor dem Rumpf und gibt ihn im
`Drop` ab (`ablage/mod.rs:363-366`), und `lesezeichen_aendern` liest, ändert und schreibt
vollständig darin (`anwendung.rs:1506-1517`). Beide Sperren werden beim Absturz frei, und das
ist nicht behauptet, sondern mit einem Kind gemessen, das wirklich stirbt
(`crates/krk-core/tests/ablage.rs:1857-1898`, `std::process::abort()` und `SIGABRT` als
erwarteter Ausgang). Eine Verklemmung zwischen den beiden ist ausgeschlossen: das Sitzungsrecht
wird ohne Warten genommen (`sperre.rs:172-178`) und nie, während ein Schreibgriff gehalten wird.
Die beiden Ablage-Werte eines Starts leben nacheinander und nicht zugleich
(`belegung.rs:1324-1327` verwirft seine Ablage am Ende der Funktion).

Die zwei Sperrdateien liegen richtig **neben** den Nutzdateien und nicht auf ihnen; der
Modulkopf begründet das mit der Nachbardatei, die `atomar::schreiben` über ein `rename`
ersetzt (`sperre.rs:37-44`). Das ist der Fehler, den ein weniger sorgfältiger Bau hier gemacht
hätte.

**5. Die Lesezeichenänderung als Eintrag trägt.** Die Begründung des `coder` stimmt: eine
Stelle ist eine Zahl in der Liste, die der Nutzer gesehen hat, und in der frisch gelesenen kann
dort ein anderes stehen. `Lesezeichenliste::stelle_von` vergleicht den ganzen Eintrag, Name und
Ziel (`crates/krk-core/src/ablage/lesezeichen.rs:428-431`), `anwenden` ist die eine Stelle, an
der eine Änderung die Liste erreicht, und sie ruft die vier vorhandenen Rechnungen statt eine
fünfte zu bauen (`:439-475`). Der dritte Ausgang `Verschwunden` ist der Grund für die
Aufzählung statt eines `bool`, und er ist richtig gesetzt.

**6. Der Vorrang der zwei `esc`-Bedeutungen ist vollständig, und ein Suchzeichen kann während
einer Aufnahme nicht im Suchtext landen.** `faengerstation` kehrt bei laufender Aufnahme sofort
mit `Aufnahme` zurück, vor jeder anderen Frage
(`crates/krk-ui/src/appkit/anwendung.rs:313-315`), und `suchzeichen_aufnehmen` hat im ganzen
Baum genau eine Aufrufstelle, nämlich die zweite Station des Fängers (`:2225`). Die eingebaute
Tippauswahl der Tabelle ist abgeschaltet (`belegungsansicht.rs:647`), also gibt es auch keine
zweite Suche daneben. Dass `esc` weiterläuft, hängt an keiner Ausnahme, sondern an der
Aufnahmeregel der Suche, die Steuerzeichen abweist; die Probe hält beide Hälften zusammen
(`anwendung.rs:5911-5943`).

**7. Ein `tag`, den niemand gesetzt hat, tut nichts.** `validateMenuItem:` fragt zuerst die
Aktion und liest den `tag` nur für `krkKommando:` (`anwendung.rs:735-748`); für jede fremde
Aktion antwortet es `true` und überlässt AppKit die Antwortkette, womit die sechs Textbefehle
und die Markdown-Ausgabe ihr heutiges Verhalten behalten. `kommando_zum_tag` fängt beides ab,
den negativen `tag` über `usize::try_from` und den zu großen über `get`
(`menue.rs:422-427`); ein Index außerhalb der Liste ergibt `None`, der Eintrag ist grau, und
`krkKommando:` kehrt wortlos zurück statt abzustürzen.

**Nachgezählt und richtig:** die Untergrenzenangabe steht in 35 von 37 Dateien unter
`crates/krk-ui/src/appkit/`, ohne sie sind nur die zwei begründeten Ausnahmen
`koordinaten.rs` und `mod.rs`. Der Plan sagt genau das.

---

## Befunde nach Themen

### Thema A: Wachen, die weniger halten als ihre Beschriftung

Das durchgehende Muster der Runde. Jede dieser Proben ist richtig gedacht und an ihrer Nadel zu
kurz gebunden.

**A1 · Die Ersthelfer-Zählprobe sieht einen Doppelbau über `downcast_ref` nicht.** *(mittel)*
`die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle`
(`crates/krk-ui/src/appkit/ereignisse.rs:690-731`) sucht `isKindOfClass(`. Derselbe Baum fragt
den Ersthelfer schon an anderer Stelle mit `ersthelfer.downcast_ref::<NSView>()`
(`crates/krk-ui/src/appkit/anwendung.rs:4070`). Eine zweite Fassung in dieser Schreibweise wäre
genau der verbotene Doppelbau und ließe beide Nadeln grün.
→ `issues/260813-0540_o_die-ersthelfer-zaehlprobe-sieht-einen-doppelbau-ueber-downcast-ref-nicht.md`

**A2 · Die Zählproben in `krk-ui` sagen „im Baum" und lesen nur eine Kiste.** *(mittel)*
`krk_ui::quellbaum::quelldateien` liest `crates/krk-ui/src`
(`crates/krk-ui/src/quellbaum.rs:51`); sechs Proben darüber sprechen trotzdem vom ganzen Baum.
Für `isKindOfClass` und `keyDown:` ist das harmlos, weil `krk-core` kein `objc2` kennen darf.
Für `fn zulaessig(` ist es das nicht, und C2.16 sagt „an genau einer Stelle" ohne Kistengrenze
zu. `crates/krk-core/tests/gemeinsam/mod.rs:264` liest dagegen alle Kisten.
→ `issues/260813-0540_o_die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste.md`

**A3 · Zwei Aufruferzählungen hängen an der Schreibweise des Aufrufs.** *(gering)*
`beide_frager_rufen_die_eine_regel` sucht `zulaessigkeit::zulaessig(`
(`kommandos/zulaessigkeit.rs:187`), `der_delegierte_wird_an_genau_drei_stellen_…` sucht
`self.` und `selbst.` als Empfänger (`appkit/menue.rs:1164-1174`). Ein dritter Frager mit
unqualifiziertem Aufruf und ein Rückruf unter anderem Bindungsnamen entgehen beiden.
→ `issues/260813-0540_o_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`

**A4 · Eine vierte Prüfordner-Fassung steht im Baum, und die C4.6-Probe sieht sie nicht.**
*(mittel)* `crates/krk-core/src/ablage/sperre.rs:209-229` erklärt `struct Ordner` mit
`impl Drop for Ordner` — ein selbstabräumender Prüfordner unter `std::env::temp_dir()`.
`genau_drei_pruefordner_fassungen_stehen_im_baum` (`crates/krk-core/tests/baum.rs:88`) sucht
`impl Drop for Pruefordner` und findet ihn nicht; dieselbe Nadel fände auch den anerkannten
`Wegwerfordner` nicht. Der Doc-Kommentar begründet, warum es die Fassung geben **muss**
(`Schreibgriff::nehmen` ist kistenintern), erklärt sie aber zugleich zur Nicht-Fassung. Dazu
greifen `sperre.rs` und `crates/krk-core/src/verzeichnis/sys.rs:950` in das echte
Temporärverzeichnis, vor dem `CLAUDE.md` bereits warnt.
→ `issues/260813-0540_o_eine-vierte-pruefordner-fassung-steht-im-baum-und-die-probe-sieht-sie-nicht.md`

**A5 · „Kein Schreibweg an der Sperre vorbei" ist nicht typgesichert und ungeprüft.**
*(mittel)* Der Modulkopf sagt es als Eigenschaft der Typen zu
(`crates/krk-core/src/ablage/mod.rs:24-26`). Heute stimmt die Aussage über den Baum; die Typen
tragen sie nicht: `pub mod atomar` (`:101`), `Ablage::pfad` (`:345`) und `Ablageort::datei`
lassen sie offen, und `crates/krk-core/tests/belegung.rs:53` nimmt den Weg bereits
(`fs::write(ablage.pfad(Datei::Belegung), keymap)`). Es gibt keine Probe über diese Zusage; die
Zwei-Prozess-Probe zeigt, dass die Sperre wirkt, nicht dass niemand an ihr vorbeischreibt.
→ `issues/260813-0540_o_kein-schreibweg-an-der-sperre-vorbei-ist-nicht-typgesichert-und-ungeprueft.md`

**A6 · Das Verbot von `setEnabled` trifft jede `NSControl`.** *(gering)*
`die_freigabe_eines_eintrags_wird_nirgends_gesetzt` verbietet `setEnabled(` im ganzen
`krk-ui/src` (`appkit/menue.rs:1118-1135`). Die Methode gehört `NSControl`; die nächste
Schaltfläche, die sie braucht, macht die Probe aus einem sachfremden Grund rot, und der
billigste Weg zurück ins Grüne wäre das Streichen der Wache.
→ `issues/260813-0540_o_das-verbot-von-setenabled-trifft-jede-nscontrol-und-nicht-nur-menueeintraege.md`

### Thema B: Zwei Instanzen an der Ablage

**B1 · Die Belegung wird weiter blind überschrieben, obwohl der Datensatz mehr zusagt.**
*(mittel, Entscheidung gehört dem Nutzer)*
`shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-an-der-ablage-…` führt unter
Möglichkeit 1 „keine verlorene Änderung an Lesezeichen **und Belegung**". Gebaut ist der
Durchgang nur für die Lesezeichen; `belegungsansicht_verlassen` schreibt die Arbeitskopie ohne
frisches Lesen (`crates/krk-ui/src/appkit/anwendung.rs:3039-3056`). C3.7 hält — kein Gemisch,
alles unter der Sperre —, die verlorene Änderung bleibt. Spec und Plan verlangen das frische
Lesen ausdrücklich nur für C3.8; der Bau folgt ihnen. Zu entscheiden ist, ob der Datensatz
nachgezogen oder die Belegung nachgebaut wird.
→ `issues/260813-0540_o_die-belegung-wird-weiter-blind-ueberschrieben-obwohl-der-datensatz-mehr-zusagt.md`

**B2 · Der Messmodus schreibt die Sitzung ohne Sitzungsrecht.** *(gering)*
`Messplan::herstellen` baut einen `Sitzungsschreiber::neu()` ohne jede Frage nach dem Recht
(`crates/krk-ui/src/messmodus.rs:300-325`), während der Doc-Kommentar des Typs die Regel „er
entsteht nur, wenn dieser Prozess das Sitzungsrecht hält" ausnahmslos formuliert
(`crates/krk-core/src/ablage/sitzung.rs:424-428`). Unter der Sperre und damit unbedenklich für
die Datei; C3.9 gilt trotzdem nicht ausnahmslos.
→ `issues/260813-0540_o_der-messmodus-schreibt-die-sitzung-ohne-sitzungsrecht.md`

**B3 · Beim Beenden laufen zwei Durchgänge, und der Kommentar nennt einen.** *(gering)*
`applicationWillTerminate:` ruft zuerst `self.sitzung_vormerken()`
(`crates/krk-ui/src/appkit/anwendung.rs:806`), das die Sperre selbst nimmt, und darunter steht
der Kommentar „Ein Durchgang und nicht zwei" (`:814-817`). Der erste Aufruf ist dazu
wirkungslos: die Zeilen darunter bauen denselben Stand und überschreiben den vorgemerkten.
→ `issues/260813-0540_o_beim-beenden-laufen-zwei-durchgaenge-und-der-kommentar-nennt-einen.md`

**B4 · Die Meldung zum verschwundenen Lesezeichen behauptet eine Löschung.** *(gering)*
`Ausgang::Verschwunden` tritt auch ein, wenn die andere Instanz den Eintrag nur **umbenannt**
oder sein Ziel geändert hat, denn `stelle_von` vergleicht den ganzen Eintrag. Der Satz in der
Statuszeile nennt trotzdem eine Löschung (`crates/krk-ui/src/appkit/anwendung.rs:1556-1560`).
→ `issues/260813-0540_o_die-meldung-zum-verschwundenen-lesezeichen-behauptet-eine-loeschung.md`

### Thema C: Prosa, die den Baum nicht mehr trifft

**C1 · Fünf Stellen nennen die Zahlen von vor S15.** *(mittel)* S14 und S15 haben `Kommando`
auf 76 Varianten und die Auslieferungsbelegung auf 82 Funktionen gebracht. Stehen geblieben
sind „zweiundachtzig Eintraege" (`appkit/menue.rs:9`, `:173`; `appkit/anwendung.rs:655`), „82
Eintraege" (`menuemodell.rs:19`) und „fuenfundsiebzig Selektoren" (`appkit/menue.rs:26`,
`:307`). Richtig sind 83 benannte Einträge, 84 `NSMenuItem` mit dem Trenner und 76 Selektoren.
Dass keine Probe die Zahlen hält, ist Absicht und richtig — genau deshalb veralten sie
unbemerkt.
→ `issues/260813-0540_o_fuenf-stellen-nennen-die-zahlen-von-vor-s15.md`

**C2 · Ein Kommentar in `behandeln` nennt noch die alte Schluckregel.** *(gering)*
`crates/krk-ui/src/appkit/ereignisse.rs:528-530` sagt „geschluckt wird nur, was auch
ausgefuehrt wurde" und verweist dabei auf einen Modulkopf, der seit S3 das Gegenteil sagt
(`:154-166`).
→ `issues/260813-0540_o_ein-kommentar-in-behandeln-nennt-noch-die-alte-schluckregel.md`

**C3 · Ein Doc-Kommentar begründet „Bearbeiten" mit einem Mechanismus, den es nicht gibt.**
*(gering)* `crates/krk-ui/src/belegungsmodell.rs:159-163` behauptet,
`menue::systemzusaetze_unterdruecken` setze am Menü dieses Namens an. Die Funktion trägt drei
Namen in `NSUserDefaults` ein und kennt keinen Menütitel (`appkit/menue.rs:283-303`). Der
zugehörige Entscheidungsdatensatz `260813-0159` sagt es unter seinen Randbedingungen selbst
richtig und nennt die Abhängigkeit ausdrücklich ungemessen.
→ `issues/260813-0540_o_ein-doc-kommentar-begruendet-bearbeiten-mit-einem-mechanismus-den-es-nicht-gibt.md`

**C4 · Ein Doc-Kommentar in `tests/belegung.rs` hängt an der falschen Funktion.** *(gering)*
Die zwei neuen Hilfsfunktionen aus S12 sind ohne Leerzeile in den vorhandenen Kommentar von
`ablage_mit` hineingeschoben worden; `ablage_mit` steht seither undokumentiert
(`crates/krk-core/tests/belegung.rs:26-58`).
→ `issues/260813-0540_o_ein-doc-kommentar-in-tests-belegung-rs-haengt-an-der-falschen-funktion.md`

### Thema D: Zwei Kleinigkeiten am Code

**D1 · `weitereinstanz::starten` fragt den Bündelort zweimal und wirft die Antwort weg.**
*(gering)* `eigenes_buendel()` baut einen `PathBuf`, den niemand benutzt; `starten` ruft
`NSBundle::mainBundle().bundleURL()` selbst ein zweites Mal
(`crates/krk-ui/src/appkit/weitereinstanz.rs:98-102`). Ein Nebenausgang fällt dabei falsch aus:
liefert `adresse.path()` nichts, meldet KRK „laeuft nicht aus einem Buendel", obwohl es das tut.
→ `issues/260813-0540_o_weitereinstanz-fragt-den-buendelort-zweimal-und-wirft-die-antwort-weg.md`

**D2 · Der Kürzelfilter des Menümodells greift nur am Kommandozweig.** *(gering)*
`menuemodell::eintrag` filtert die vom Zusteller beanspruchte Kombination nur für Funktionen
**mit** Kommando (`crates/krk-ui/src/menuemodell.rs:243-250`); der Zweig für eine benannte
Funktion ohne Kommando und ohne Zusteller behält sie ungefiltert. Heute unerreichbar und von
`keine_zwei_eintraege_tragen_dieselbe_kombination` abgesichert.
→ `issues/260813-0540_o_der-kuerzelfilter-des-menuemodells-greift-nur-am-kommandozweig.md`

### Eine Beobachtung ohne eigenen Datensatz

`behandeln` ruft `getipptes_zeichen(ereignis)` zweimal, einmal für den Fänger und einmal im
Sprungmarkenzweig (`crates/krk-ui/src/appkit/ereignisse.rs:512`, `:538`). Das ist ein
doppelter, aber korrekter Aufruf auf dem Tastendruckpfad und kein Defekt; ein Datensatz dafür
kostete mehr als die Zeile, die ihn behebt. Er steht hier, damit er nicht übersehen wird,
falls die Abnahme am Bündel L1 anfasst.

---

## Was quer durch die Runde läuft

**Erstens: das Muster der zu kurz gebundenen Nadel.** Sechs der sechzehn Befunde sind
Zählproben, die eine Zusage über den Baum an eine **Schreibweise** binden statt an die Sache.
Der Nachtrag vom 260813-0233 hat den Unterschied zwischen Erklärungs- und Aufruferzählung
sauber herausgearbeitet und `crates/krk-ui/src/quellbaum.rs:16-28` schreibt ihn aus — die
Runde hat damit die richtige Unterscheidung getroffen und die falsche Schlussfolgerung daraus
gezogen: „eine Erklärungszählung hält, was sie verspricht" gilt nur gegen eine Kopie **unter
demselben Namen** und **in derselben Kiste**. Die vierte Prüfordner-Fassung in `sperre.rs` ist
der ausgeführte Beweis: sie steht im Baum, ist begründet, und die Probe, die genau sie zählen
soll, sieht sie nicht.

Das ist kein Grund, die Proben abzuschaffen. Es ist ein Grund, ihre Doc-Kommentare auf das zu
bringen, was sie leisten, und dort nachzuziehen, wo eine zweite Schreibweise im selben Baum
schon existiert (A1) oder eine zweite Kiste in Reichweite liegt (A2).

**Zweitens: zwei Quellbaumleser mit verschiedener Reichweite.** S2 hat den Leser aus
`teilen.rs` herausgezogen, um ihn nicht dreimal abzuschreiben — richtig. Daneben steht seit
S12 ein zweiter, fast wortgleicher in `crates/krk-core/tests/gemeinsam/mod.rs:243-266`, und
der Doc-Kommentar begründet ihn korrekt mit der Kistengrenze. Die zwei unterscheiden sich in
**einem** Punkt, und der steht nirgends nebeneinander: der eine liest eine Kiste, der andere
alle. Wer eine Zählprobe schreibt, wählt damit unbemerkt ihre Reichweite mit.

**Drittens: die Zahl in der Prosa ist in dieser Runde zweimal veraltet, und beide Male
innerhalb derselben Spanne.** S6 hat „zweiundachtzig Einträge" und „fünfundsiebzig Selektoren"
geschrieben, S14 und S15 haben beide Zahlen fünf Commits später überholt. `CLAUDE.md` führt
zwei ältere Befunde derselben Sorte offen. Der Weg, den dieser Baum sonst geht — die Zahl gegen
die Quelle zählen statt sie hinzuschreiben, wie
`jede_funktion_der_belegung_steht_genau_einmal_im_menue` es tut —, ist auf Prosa nicht
anwendbar; die Gegenmaßnahme bleibt, Zahlen aus der Prosa zu streichen, wo sie nichts tragen.

**Viertens, und positiv:** die Runde hat drei Verluste gegenüber heute selbst gefunden,
benannt und abgelegt, bevor die Prüfung sie sah — `esc` im Editor (`decisions/260813-0320`),
der Klick in die Bereichsleiste (`issues/260813-0311`) und das `cmd+a`-Kürzel
(`issues/260813-0416`). Diese Prüfung hat **keinen weiteren** Verlust gegenüber dem Stand vor
der Runde gefunden; erschöpfend durchgezählt ist die Menge der Wege nicht. Drei Wege, die
eine Regression hätten tragen können, sind einzeln nachgerechnet: `cmd+n` bleibt bei geschlossenem Fenster erreichbar (`FensterEinblenden` trägt
`Wirkungsbereich::Ueberall`, und ohne Schlüsselfenster liefert `lage()` `Fokus::Anderswo`);
`cmd+a` erreicht den Feldeditor weiter, weil der Zusteller sein Menükürzel behält; und die
sechs Textbefehle bleiben während eines Blattes bedienbar, weil `validateMenuItem:` für jede
fremde Aktion `true` antwortet.

---

## Reihenfolge

**Nichts hält die Runde auf.** Kein Befund ist ein Freigabehindernis, und keiner macht den
Abnahmelauf am Bündel unmöglich; die Abnahmeliste des Plans gilt unverändert.

**Vor dem nächsten Turn, weil sie Zusagen betreffen, auf die spätere Runden bauen:**

1. B1 — die Zusage im Ablage-Datensatz gegen den Bau stellen. Das ist eine Nutzerentscheidung
   und keine Codeänderung; sie gehört an den Anfang, weil sie den Zuschnitt einer möglichen
   Folgearbeit an `keymap.toml` bestimmt.
2. A4 und A5 — die zwei Wachen am Sperrstrang. Der Strang hat das größte Schadenspotential der
   Runde, und beide Befunde betreffen die Frage, ob eine Verletzung überhaupt auffiele.
3. A1 und A2 — die zwei Wachen an der Zulässigkeitsfrage, aus demselben Grund.

**Aufräumen, wenn Zeit ist:** C1 bis C4, A3, A6, B2 bis B4, D1 und D2. Alle sind einzelne
Zeilen oder einzelne Absätze; keiner ändert Verhalten.

**Am Bündel zu sehen, unverändert nach Plan:** die fünf Fälle der Ausgrauung aus C2.6, C2.7,
C2.18 und C2.19, die Gegenprobe zu „Quit and Keep Windows" auf Opt+Cmd+Q, die springende
Auswahl der Suche, die zweite Instanz mit ihrer Statuszeile, dazu L4, L1 und L9. Zwei Punkte
gehören ausdrücklich dazu, weil der Baum sie nicht beantworten kann: ob Cmd+T und Cmd+R die
Schaltflächen der Belegungsansicht überhaupt erreichen (sie hängen daran, dass die Ausgrauung
dem Menüeintrag sein Kürzel wirklich abnimmt), und ob AppKit `validateMenuItem:` vor jeder
Tastenentsprechung erfragt.
