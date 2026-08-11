# Codedurchsicht: Vier Tastenbefehle für Pfade, das Öffnen und Cmd+W (Turn 1)

**Datum:** 2026-08-11, 19:16
**Absender:** `coderev`
**Circle:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`
**Umfang:** `git diff 6939249..HEAD -- crates/ resources/`, fünf Commits (`a358d86`, `3d48f34`, `d23bfdb`, `cee5276`, `5487695`), 15 Dateien, 1062 eingefügte und 89 entfernte Zeilen
**Maßstab:** Spec `planning/260811-1552_o_spec-*.md` (5 Fähigkeiten, 62 Kriterien), Plan `planning/260811-1648_o_plan-*.md` (5 Schritte), die sieben beantworteten Datensätze unter `decisions/`

## Zusammenfassung

Die fünf Schritte bauen, was Spec und Plan bestellt haben. `make check` läuft grün (Ausgang 0, 654 Proben in 16 Zielen), und die vier Behauptungen, die die `coder` gemessen statt angenommen haben, halten alle vier einer eigenen Nachmessung stand — die Fundstellen zu `NSControl::target` stimmen auf die Zeile genau, in der Bindung wie im Systemkopf. Die ganze Verbotsseite hält: 74 Funktionen, 82 Kombinationen, 68 Kommandos, sieben Wirkungsbereiche, eine Zeile in `waehrend_blatt_erlaubt`, kein neues Menükürzel, drei Prüfordner-Fassungen, kein neuer Auffangzweig.

Sechs Befunde stehen dagegen, keiner davon am Verhalten der vier Befehle. Fünf betreffen Kommentare und Proben, einer eine Meldung im Wortlaut. Der schwerste ist ein neu geschriebener Satz im Modulkopf der Zwischenablage, der eine Lage zusagt, die seit dem 260805 nicht mehr gilt — genau die Fehlerart, gegen die diese Runde angetreten ist.

## Zahlen

| Schwere | Zahl | Befunde |
|---|---|---|
| Kritisch | 0 | — |
| Hoch | 0 | — |
| Mittel | 2 | die falsche Zusage im Kopf der Zwischenablage; drei unvollständig gewordene Modulköpfe |
| Niedrig | 4 | der halbierte Doppelklick-Löschweg; der buchstabengenaue Wachposten; die Abweisungsmeldung im Singular; der Satz für die leere Menge gegen den Wortlaut von C2 |

Alle sechs liegen als eigene Dateien unter `issues/` dieses Circles, jede mit dem Marker offen.

## Was nachgemessen und nicht übernommen ist

### Die vier Behauptungen der `coder`

**S2, keine Probe fasst die echte Zwischenablage an.** Bestätigt auf zwei Wegen. Statisch: `text_schreiben` hat genau zwei Aufrufer, beide in `appkit/tabelle.rs:833` und `:861`, keiner in einem Prüfmodul; `generalPasteboard` steht an drei Stellen, alle drei in `appkit/zwischenablage.rs`. Gemessen: ein Wachposten in der Zwischenablage überlebt `cargo test --workspace` unverändert.

**S3, keine Probe startet ein Programm.** Bestätigt. `standardprogramm::oeffnen` hat einen Aufrufer, `tabelle.rs:900`, und der steht in einer privaten Methode ohne Prüfmodul. Gemessen: die Zahl der Anwendungen mit Oberfläche war vor und nach dem Lauf sechs.

**S4, `NSControl::target` wird schwach geführt.** Bestätigt, und die vier Fundstellen des `SAFETY`-Blocks (`tabelle.rs:2138-2170`) stimmen auf die Zeile:

| Zitat | Steht dort |
|---|---|
| `objc2-app-kit-0.3.2/…/NSTableView.rs:242` | `#[unsafe(super(NSControl, NSView, NSResponder, NSObject))]` — die Vererbung, auf die sich der Block beruft |
| `objc2-app-kit-0.3.2/…/NSControl.rs:91-93` | der Doc-Block über `setTarget:`, endend mit "This is a [weak property]" |
| `AppKit.framework/Headers/NSControl.h:24` | `@property (nullable, weak) id target;` samt dem Zusatz über 10.10 |
| `AppKit.framework/Headers/NSTableView.h:275-278` | `clickedRow` und `doubleAction`, beide ohne `API_AVAILABLE` |

Der Ring Quelle → Tabelle → Ziel → Delegierter → Quelle bleibt damit an der Kante Tabelle → Ziel offen. Das Zwischenobjekt mit `objc2::rc::Weak`, das der Plan als zweiten Ausgang vorgesehen hatte, wird nicht gebraucht, und der Kommentar sagt, welcher Fall vorlag.

**S1, neun Kommentarstellen mit 71 und 65 sind nachgezogen.** Bestätigt, und keine alte Zahl steht mehr. Ein Durchgang über `crates/`, `resources/`, `Makefile` und `README.md` nach 71, 79 und 65 findet nur noch Treffer, die etwas anderes meinen: die 65 Prozent aus der Zeitzusage L9 und eine Zeilenangabe in einer Spike-Messung. Die `coder` haben dabei über den Plan hinausgearbeitet: er nannte vier Stellen in `belegungsausgabe.rs`, nachgezogen sind sechs dort und drei in `appkit/menue.rs`.

### Die Verbotsseite

Jede geprüft, jede hält:

| Zusage | Gemessen |
|---|---|
| `resources/default-keymap.toml` führt 74 Funktionen | 74 Blöcke `[[funktion]]` |
| … mit 82 Kombinationen | 82 Einträge in den `tasten`-Listen |
| `Kommando::KENNUNGEN` trägt 68 | die Länge steht im Typ und hält den Übersetzer; `Kommando` zählt 68 Varianten |
| `Wirkungsbereich` bleibt bei sieben Werten | sieben, `beschriftung` unverändert |
| `waehrend_blatt_erlaubt` bleibt eine Zeile | `operationen.rs:208-210`, ein Vergleich |
| `menue.rs` ohne neues Kürzel | `setKeyEquivalent` steht einmal in der Datei, und der Diff fasst die Zeile nicht an |
| drei Prüfordner-Fassungen | `krk-core/tests/gemeinsam/mod.rs:51`, `krk-ui/src/pruefordner.rs:47`, `krk-bench/src/wegwerfordner.rs:33` |
| kein neuer Auffangzweig | der Diff fügt keine Zeile mit `_ =>` ein |
| `#![deny(unsafe_code)]` in allen drei Kisten | `krk-core/src/lib.rs:1`, `krk-ui/src/main.rs:1`, `krk-bench/src/main.rs:1`; die einzigen `allow` stehen in `verzeichnis/sys.rs:71` und `appkit/mod.rs:1` |
| kein `canonicalize` auf dem neuen Weg | nur die Erwähnung im Doc-Kommentar von `pfadtext` |
| kein `gekuerzt_fuer_anzeige` auf dem neuen Weg | keine Fundstelle in `tabelle.rs` und `operationen.rs` |
| `NSWorkspace` an vier Stellen | `volumes.rs`, `terminal.rs`, `zwischenablage.rs`, `standardprogramm.rs`; die drei weiteren Treffer sind Prosa in Modulköpfen |
| die Auslieferungsbelegung ist konfliktfrei | die Probe `die_auslieferungsbelegung_ist_konfliktfrei` läuft in `make check` durch |

Zur Konflikterkennung eine Anmerkung zum Weg: `make tasten` baut das Bündel und startet es im Protokollmodus, verlangt also KRK im Vordergrund und ist damit Nutzerarbeit. Die Zusage selbst hängt nicht daran — sie steht als Probe in `crates/krk-core/tests/belegung.rs:122` und ist mit `make check` mitgelaufen.

### Die zwei Ungleichheiten, die Absicht sind

Beide trägt der Code, und keine ist verwischt.

```
return          ──> betroffene_eintraege().pfade  ──┐
                                                    ├──> mit_standardprogramm_oeffnen(&[PathBuf])
Doppelklick ──> in_zeile_einsteigen(clickedRow)     │        └──> standardprogramm::oeffnen je Pfad
                    │  true  ──> eingestiegen, fertig
                    └─ false ──> from_ref(&pfad) ───┘
```

**Die Menge.** `tabelle.rs:800` gibt der Taste die ganze Menge, `tabelle.rs:953` gibt dem Doppelklick `std::slice::from_ref(&pfad)`, also genau die angeklickte Zeile. Eine Markierung anderswo geht ihn nicht an.

**Die Verzweigung.** Der Doppelklick fragt `in_zeile_einsteigen` und verzweigt an dessen Rückgabewert (`tabelle.rs:947`). Die Taste fragt nichts: `standardprogramm::oeffnen` prüft keinen Typ, und `mit_standardprogramm_oeffnen` prüft ihn auch nicht. Ein Ordner geht damit an das System.

Der Rechts-Pfeil bleibt, was er war. `auswahl_oeffnen` verwirft den Rückgabewert von `in_zeile_einsteigen` und löst auf einer Datei nichts aus; der Rumpf steht genau einmal im Baum.

## Befunde nach Themen

### Schriftliche Zusicherungen

**Mittel — der Modulkopf der Zwischenablage sagt neu zu, Cmd+C und Cmd+V seien ab Werk unbelegt.** `crates/krk-ui/src/appkit/zwischenablage.rs:53-54` schreibt seit `d23bfdb`: "Cmd+C und Cmd+V bleiben ab Werk unbelegt, wie es C3 der Runde 1 zugesagt hat." Der Baum sagt das Gegenteil: `resources/default-keymap.toml:712-722` führt beide Kombinationen mit `gehalten_von = "menue"`, und der Kopf derselben Datei schreibt den Wechsel in den Zeilen 64-68 aus. Der Halbsatz stand schon vorher da und war schon vorher falsch, seit dem 260805; S2 hat den Absatz um ihn herum neu gefasst, ihn mitgenommen und mit "wie es C3 der Runde 1 zugesagt hat" zusätzlich bekräftigt. Er ist damit eine Aussage dieses Commits. Datei: `issues/260811-1916_o_der-neu-gefasste-modulkopf-der-zwischenablage-sagt-cmd-c-und-cmd-v-seien-ab-werk-unbelegt.md`.

**Mittel — drei Modulköpfe zählen auf, was sie tragen, und führen die drei neuen Befehle nicht.** `appkit/mod.rs:70-71` nennt für `zwischenablage` "die beiden Berührungen aus C10" und kennt `text_schreiben` nicht; derselbe Überblick nennt jedes Modul des Verzeichnisses beim Namen, `standardprogramm` steht allein in der `mod`-Liste bei Zeile 139. `kommandos/mod.rs:15-16` und `kommandos/operationen.rs:3-8` beschreiben den Inhalt von `operationen` als Dateioperationen und Terminal-Antworten; dazu stehen dort jetzt sieben Funktionen der Runde 4. Datei: `issues/260811-1916_o_drei-modulkoepfe-zaehlen-auf-was-sie-tragen-und-fuehren-die-drei-neuen-befehle-nicht.md`.

**Die beiden Zusicherungen, die der Spec ausdrücklich genannt hat, sind dagegen richtig neu gefasst.** Der Kopf von `blaetter/mod.rs:224-240` sagt jetzt, was der Code trägt, und jede seiner drei Aussagen ist nachgeprüft: die Eingabetaste ist belegt (`default-keymap.toml:528`), die Blattsperre steht vor dem Fokusvorbehalt (`anwendung.rs:1986` vor `:1998`), und die beiden Kombinationen mit Zusatztaste sind unbelegt geblieben — `cmd+return` und `opt+return` stehen in keiner `tasten`-Liste. Der Kopf von `zwischenablage.rs` hat die Zusage "KRK schreibt die Zwischenablage in keinem Fall" durch die Lage nach C1 und C2 ersetzt, und was er über die eine geschriebene Sorte sagt, hält der Rumpf: `setString_forType` mit `NSPasteboardTypeString`, kein `writeObjects:`.

### Meldungen und Proben

**Niedrig — der Wachposten gegen "geöffnet" fängt ein großgeschriebenes "Öffn…" nicht.** `operationen.rs:1504` prüft `!meldung.contains("öffn")`, und `str::contains` vergleicht buchstabengenau: `"Öffnet".contains("öffn")` ist falsch. Der Wachposten ist dadurch nicht leer — dieselbe Schleife verlangt `contains("System")`, und die naheliegenden falschen Sätze fallen dort auf. Er hält aber weniger, als sein Doc-Kommentar sagt. Dieselbe Eigenschaft trägt die Prüfung in `operationen.rs:1440`. Datei: `issues/260811-1916_o_der-wachposten-gegen-geoeffnet-faengt-ein-grossgeschriebenes-oeffn-nicht.md`.

**Niedrig — die Abweisungsmeldung der Zwischenablage nennt "den Pfad" im Singular.** `ablage_weist_ab()` (`operationen.rs:840`) steht auch dann, wenn `eintragspfad_kopieren` dreißig Pfade abgelegt hätte (`tabelle.rs:864`). Jede andere Meldung dieser Runde unterscheidet Einzahl und Mehrzahl. Datei: `issues/260811-1916_o_die-abweisungsmeldung-der-zwischenablage-nennt-den-pfad-im-singular-und-steht-auch-bei-mehreren.md`.

**Niedrig — der Satz für die leere Menge sagt nicht, dass nichts zu kopieren war.** C2 sagt zu: "die Statuszeile sagt, dass nichts zu kopieren war". `nichts_betroffen()` liefert "nichts markiert und nichts ausgewählt" (`operationen.rs:833`) und nennt die Lage statt der Folge. Das Verbot aus demselben Kriterium, kommentarlos nichts zu tun, ist eingehalten. Die Abweichung ist von den `coder` gemeldet und im Doc-Kommentar begründet, und die Begründung trägt: der Plan typisiert die Funktion gemeinsam für Kopierer und Öffner, schlägt aber zwei Texte vor, und "der Ordner ist leer" wäre daneben unwahr, weil eine leere Menge auch während eines Lesevorgangs in einem vollen Ordner entsteht. Was bleibt, ist eine Abnahmefrage an den Nutzer. Datei: `issues/260811-1916_o_der-satz-fuer-die-leere-menge-sagt-nicht-dass-nichts-zu-kopieren-war.md`.

### Zustand über die Bereiche hinweg

**Niedrig — der Doppelklick räumt die Befehlsantwort nur an seiner eigenen Fensterseite weg.** `tabelle.rs:943` ruft `befehlsantwort_loeschen` an der Quelle, in der geklickt wurde; die Regel, auf die sich der Doc-Kommentar beruft, läuft über beide Seiten (`anwendung.rs:2009-2010`, `for seite in Fensterseite::ALLE`). Der Fall: links `shift+cmd+c` mit sieben markierten Einträgen, dann ein Doppelklick rechts — links steht "7 Pfade kopiert" weiter. Kein Abnahmekriterium ist berührt; der Kommentar sagt "dieselbe Regel und keine zweite" und beschreibt damit eine breitere Regel, als der Code führt. Datei: `issues/260811-1916_o_der-doppelklick-raeumt-die-befehlsantwort-nur-an-seiner-eigenen-fensterseite-weg.md`.

## Beobachtungen ohne Defekt

**Der Doppelklick auf die leere Fläche räumt die Befehlsantwort.** `befehlsantwort_loeschen()` steht vor der Prüfung auf eine Zeile kleiner als null (`tabelle.rs:943-946`). C3 sagt für diesen Klick "tut nichts" zu. Ich lese das Kriterium als "kein Einstieg, kein Öffnen" und nicht als "keine sichtbare Änderung", denn ein Doppelklick ist die nächste Handlung des Nutzers und die Regel über der Befehlsantwort ist genau darauf gebaut. Kein Defekt, aber eine Stelle, die bei der Sichtprüfung am Bündel auffallen kann.

**Der Doppelklick trägt keine Blattsperre.** `inference:` Er braucht keine: ein Blatt am Hauptfenster ist fenstermodal, und AppKit nimmt dem Elternfenster für seine Dauer die Mauseingabe ab. Ich habe das nicht am laufenden Bündel geprüft.

**`tabelle.rs` nennt im Modulkopf keine macOS-Untergrenze.** Die drei neu angesprochenen Stellen (`setTarget:`, `setDoubleAction:`, `clickedRow`) sind stattdessen am `SAFETY`-Block dokumentiert, mit Zeile und ohne `API_AVAILABLE`. Die Gewohnheit aus `CLAUDE.md` verlangt die Angabe im Kopf; der Widerspruch ist bekannt und liegt als `shared/issues/260811-1648_o_die-untergrenzen-angabe-im-modulkopf-steht-in-sieben-von-32-appkit-modulen.md`, der `tabelle.rs` namentlich führt. Kein neuer Datensatz, ein Querverweis.

**Die beiden Module, die der Plan nachzieht, tragen die Angabe.** `standardprogramm.rs:59-67` und `zwischenablage.rs:76-87` nennen jede angesprochene Klasse und Methode mit ihrer Untergrenze und das Bündelziel 15.0.

## Die drei gemeldeten Abweichungen vom Plan

Jede trägt ihre Begründung, und jede ist am Baum nachgelesen.

**S3, `mit_standardprogramm_oeffnen` liefert `()` statt `bool`.** Trägt. Der Zweig in `DateifensterQuelle::kommando_ausfuehren` fällt wie die Nachbarn in den gemeinsamen Rückgabewert `true` (`tabelle.rs:806`); der Befehl war zuständig, auch wenn er nur etwas zu melden hatte. Ein `bool` hätte an dieser Stelle keinen Abnehmer.

**S2, `nichts_betroffen` bekam einen Text ohne Verb.** Trägt, mit einem Rest, der oben als eigener Befund steht.

**S1, zwei Proben in `tests/belegung.rs` mussten geändert werden.** Trägt, und beide Änderungen sind notwendig und nicht bequem. `die_ab_werk_freien_kombinationen_kommen_nicht_vor` prüft eine Liste, die von zwei auf ein Element geschrumpft ist; die Schleife darüber wäre unter `clippy::single_element_loop` rot geworden, und der Kommentar sagt, dass sie mit einer zweiten frei gehaltenen Kombination zurückkommt. `die_auslieferungsbelegung_fuehrt_einundsiebzig_funktionen` trägt die Zahl im Namen und musste mit ihr wandern.

**Eine vierte Abweichung ist nicht gemeldet und harmlos.** S1 führt `appkit/menue.rs` mit dem Vermerk "nicht anfassen: keine der drei Funktionen bekommt ein Menükürzel". Die Datei ist angefasst, aber nur an drei Kommentarzahlen (71 → 74, 65 → 68); kein Kürzel ist dazugekommen. Die Zusage hinter dem Vermerk ist eingehalten, die Buchstabentreue nicht, und der Baum ist dadurch richtiger geworden statt falscher.

**Eine fünfte, ebenfalls harmlos.** Der Plan nennt in Frage 6 fünf Funktionen ohne AppKit; gebaut sind sieben. `ablage_weist_ab` trägt einen Text, den Frage 7 in ihrer Tabelle schon vorschlägt, und `eintragsname` ist eine private Hilfe von `oeffnungsmeldung`.

## Übergreifendes

**Alle sechs Befunde liegen an derselben Kante: der Text neben dem Code.** Kein einziger betrifft, was einer der vier Befehle tut. Vier von sechs sind Aussagen, die weiter reichen als das, was darunter steht — ein Kopf, der eine alte Belegung zusagt, drei Aufzählungen, die eine neue Zeile nicht kennen, ein Kommentar, der eine breitere Löschregel behauptet, eine Probe, die mehr zu halten vorgibt. Das ist genau die Sorte, die der Spec unter `## Zwei schriftliche Zusicherungen, die diese Runde bricht` benennt und die der Plan in Befund 4 von zwei auf fünf Stellen erweitert hat.

**Die Methode, die die fünf gefunden hat, ist an drei Stellen nicht zu Ende gegangen.** Befund 4 prüft, was durch die Runde **falsch** wird. Er prüft nicht, was durch sie **unvollständig** wird: eine Aufzählung, die weiterhin richtig ist und nur eine Zeile zu wenig führt. Beide Modulköpfe `mod.rs` und der Kopf von `operationen.rs` fallen in die zweite Klasse, und die Zusage im Kopf der Zwischenablage fiel in die erste und stand nur nicht auf der Liste, weil sie schon vor der Runde falsch war.

**Für den nächsten Plan taugt daraus eine Frage neben Befund 4:** welche Datei zählt auf, was das Modul trägt, und wächst dieses Modul in dieser Runde? Sie ist mit zwei `grep` zu beantworten und hätte hier drei der sechs Befunde vorweggenommen.

## Empfohlene Reihenfolge

**Vor dem Abschluss der Runde:** die beiden mittleren Befunde. Beide sind Textänderungen von wenigen Zeilen in Dateien, die die Runde ohnehin angefasst hat, und beide sind billiger jetzt als in einer späteren Runde, in der niemand mehr weiß, warum der Satz dasteht.

**Vor dem Abschluss, wenn der Nutzer den Wortlaut prüft:** der Satz für die leere Menge. Er ist eine Abnahmefrage an C2 und keine Reparatur; die Antwort kann auch heißen, dass das Kriterium umformuliert wird.

**Aufräumen, jederzeit:** der Wachposten, die Abweisungsmeldung und der Doppelklick-Löschweg. Keiner hält den Abschluss auf.

**Nicht Sache dieser Durchsicht:** die sieben Abnahmekriterien am laufenden Bündel, die der Plan unter `## Was am gebauten Bündel zu prüfen ist` führt. Sie verlangen KRK im Vordergrund und sind Nutzerarbeit; keiner der sechs Befunde ändert daran etwas.

---

## Abgleichsvermerk 260811-2157 (`reconciler`)

**Alle sechs Befunde sind geschlossen, und jede Behebung ist einzeln gegen den Baum gelesen.** Die
Datensaetze liegen unter `issues/` dieses Circles, saemtlich mit dem Marker geschlossen; jeder traegt
den Vermerk mit Datei und Zeile.

| Befund | Stand am 260811-2157 |
|---|---|
| falsche Zusage im Kopf der Zwischenablage | behoben, `zwischenablage.rs:53-58` |
| drei unvollstaendig gewordene Modulkoepfe | behoben, `appkit/mod.rs:10,22,77-80`, `kommandos/mod.rs:15-17`, `operationen.rs:3-13` |
| halbierter Doppelklick-Loeschweg | Kommentar geaendert statt Verhalten, `tabelle.rs:995-1003` |
| buchstabengenauer Wachposten | behoben, `operationen.rs:1479-1486` und `:1556-1559` |
| Abweisungsmeldung im Singular | behoben, `ablage_weist_ab()` sagt „Text" |
| Satz fuer die leere Menge gegen C2 | behoben, `nichts_zu_kopieren()` und `nichts_zu_oeffnen()` |

**Die vier nachgemessenen Behauptungen und die Verbotsseite haelt der Abgleich unabhaengig fest.**
Nachgezaehlt: 74 Bloecke `[[funktion]]` und 82 Eintraege in den `tasten`-Listen von
`resources/default-keymap.toml`, 68 Varianten in `Kommando`, 68 als Laenge im Typ von
`KENNUNGEN` (`belegung.rs:488`), sieben Werte in `Wirkungsbereich`, null `_ =>` in
`Kommando::wirkungsbereich` und in `bereich_des_kommandos`. Der `SAFETY`-Block zu `setTarget:`
steht bei `tabelle.rs:2300-2340`.

**Eine Zahl dieser Durchsicht ist inzwischen groesser:** die Zusammenfassung nennt „654 Proben in 16
Zielen". Ein eigener Lauf am 260811-2157 zaehlt **795 bestandene Proben in 16 Zielen**, null
gescheitert, eine uebersprungen, Ausgang 0. Der Unterschied stammt aus den fuenf Commits nach dieser
Durchsicht und ist kein Widerspruch.
