# Analysis: achtzehn offene Entscheidungsfragen, verdichtet zur Vorlage im Chat

**Date:** 2026-09-06 21:00
**Type:** Document Study
**Status:** Complete
**Requested by:** Kai Stalmann

## Question

Wie lauten die achtzehn offenen Entscheidungsfragen 39 bis 56 aus `fusion-workbench/shared/decisions/`, so verdichtet, dass der Nutzer sie im Chat nacheinander ohne Rückfrage beantworten kann, und welche davon hat der Baum inzwischen faktisch entschieden?

## Scope

Achtzehn Dateien unter `fusion-workbench/shared/decisions/`, vollständig gelesen. Dazu die von ihnen genannten Quellstellen im Baum, die verwandten Defektdatensätze und die Verlaufsdatei der vierten Behebungsschleife.

Baumstand: HEAD `8276170a96d52f325af4606525f4e241ec93ac14`, Commitdatum 2026-09-06, Zweig `main`, 17 Commits vor `origin/main`. Alle Gegenwartsaussagen unten sind an diesem Stand gemessen. Keine der achtzehn Dateien trägt eine gefüllte Zeile `Answered:` oder `Answer located:`; auf der Platte steht zu keiner eine Antwort.

## Findings

### 39. Bekommt der Veröffentlichungsbefehl ein bequemes Kurzkommando, oder tippt man ihn weiter voll aus?

Zwei der drei Auslieferungswege haben eine Kurzform (`./release.sh`, `./certify-only.sh`), der dritte nicht: das nachträgliche Veröffentlichen ruft man mit dem vollen Pfad zu `cargo`. Gebraucht wird gerade er, wenn ein Auslieferungslauf am Netz gescheitert ist.

a) Keine Kurzform, wie heute — schließt aus, dass der Störungsfall bequem läuft; die README erklärt dafür den vollen Aufruf.
b) Ein Make-Ziel `make veroeffentlichen VERSION=…` — schließt aus, dass der Befehlsname an nur zwei Stellen steht; es kommt eine dritte dazu.
c) Make-Ziel und Skript `./publish-only.sh` — schließt aus, dass die Projektwurzel bei zwei Skripten bleibt.

**Empfehlung:** a, weil b sich jederzeit in zwei Zeilen nachziehen lässt, sobald du den Weg zum ersten Mal wirklich brauchst.
**Noch aktuell?** ja. Die Projektwurzel führt `release.sh` und `certify-only.sh`, kein `publish-only.sh`; das `Makefile` hat kein Ziel `veroeffentlichen` (nur `release` nennt das Wort in seinem Hilfetext).
**Datei:** `260821-1115_o_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-wie-certify-only-sh.md`

### 40. Darf das Bauwerkzeug ein nachinstalliertes Programm über den Suchpfad rufen, statt seinen Ort fest einzutragen?

Systemprogramme ruft das Bauwerkzeug mit vollem Pfad. `gh`, das für die Releaseseite gebraucht wird, liegt je nach Mac woanders, wird also über den Suchpfad gefunden. Ob das die Regel für jedes künftige nachinstallierte Programm wird, ist nirgends festgelegt.

a) Suchpfad, und die Regel wird ausgeschrieben: mit macOS geliefert heißt fester Pfad, nachinstalliert heißt Suchpfad — schließt aus, dass der Ort des Programms unabhängig von der Umgebung feststeht.
b) Eine Liste bekannter Orte nacheinander durchprobieren — schließt aus, dass eine Installation über einen ungewöhnlichen Weg gefunden wird; die Liste muss jemand pflegen.
c) Suchpfad wie a, zusätzlich meldet der Lauf einmal, welches Programm er gefunden hat — schließt aus, dass die Stelle bei drei Aufrufen bleibt.

**Empfehlung:** a, weil nur sie auf beiden Mac-Bauarten und über jeden Installationsweg trifft.
**Noch aktuell?** ja, und breiter als beschrieben. Der Datensatz nennt `gh` als erste Ausnahme; heute rufen außerdem `rustup` (`xtask/src/release.rs:639`), `iconutil` (`xtask/src/bundle.rs:463`) und `cargo` (`xtask/src/version.rs:303`, `messen.rs:71`, `bundle.rs:537`) über den Suchpfad. `gh` ist also nicht die Ausnahme, sondern der vierte Fall, und die Regel fehlt weiterhin.
**Datei:** `260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`

### 41. Muss man ab Werk fünf Zeichen tippen, bevor der Filter in Dateiinhalte schaut, oder wieder drei?

Der Inhaltsfilter springt bei flacher Suche ab drei Zeichen an, bei tiefer ab fünf. Seit „Deep" ab Werk eingeschaltet ist, gilt für jeden neuen Nutzer die Fünf, ohne dass das jemand verlangt hätte.

a) Es bleibt bei der Staffelung, ab Werk gilt die Fünf — schließt aus, dass der Inhaltsfilter unabhängig vom Zustand von „Deep" gleich reagiert.
b) Eine Schwelle für beide Fälle, die Staffelung fällt — schließt aus, dass drei Zeichen bei tiefer Suche noch vor dem Auslesen eines ganzen Unterbaums schützen.
c) Die tiefe Schwelle sinkt auf vier — schließt aus, dass die Zahlen auf eine Messung zurückgehen; keine der drei ist gemessen.

**Empfehlung:** a, weil die Begründung der Staffelung sich nicht geändert hat, nur wer sie auslöst, und für b keine Messung vorliegt.
**Noch aktuell?** ja. `crates/krk-core/src/verzeichnis/filter.rs:240-242` gibt unverändert 5 bei tiefer und 3 bei flacher Suche.
**Datei:** `260826-0859_o_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md`

### 42. Soll schon der erste Tastendruck den ganzen Unterbaum durchsuchen, oder erst ab mehreren Zeichen?

Seit „Deep" ab Werk anhakt, startet der erste Anschlag im Dateifenster einen Durchlauf über den gesamten Unterbaum. Der hat keine Tiefengrenze und keinen Deckel; in einem Heimatordner mit vielen Projekten sind das Zehntausende Verzeichnisse, und jeder weitere Anschlag beginnt von vorn.

a) So lassen, ein Zeichen genügt weiter — schließt aus, dass die teuerste Wirkung an einem teureren Anlass hängt; gemessen ist der Preis nicht.
b) Der Durchlauf bekommt eine eigene Zeichenschwelle wie der Inhaltsfilter — schließt aus, dass die Liste sich beim Tippen gleichmäßig verhält; bei Zeichen n springt sie um.
c) Erst messen, was ein Durchlauf ohne Treffer kostet, dann entscheiden — schließt aus, dass die Frage jetzt fällt; bis dahin gilt a.

**Empfehlung:** c mit a in der Zwischenzeit, weil dieselbe fehlende Messung auch Frage 41 entscheidet und eine Messung beide bedient.
**Noch aktuell?** ja. `filter_steht` ist unverändert „Filtertext nicht leer" (`crates/krk-core/src/verzeichnis/modell.rs:1029-1031`), und `durchlauf_nachziehen_an` (`crates/krk-ui/src/tabs.rs:1077`) stößt darauf hin an.
**Datei:** `260826-0923_o_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-jetzt-wo-ein-anschlag-ihn-ab-werk-ausloest.md`

### 43. Soll „Überschreiben" beim Kopieren und Verschieben das alte Ziel in den Papierkorb räumen statt es endgültig zu löschen?

Im Kontextmenü bedeutet „Überschreiben" seit dem 25.08. „in den Papierkorb". Beim Kopieren, beim Verschieben und beim Abwurf bedeutet dieselbe Schaltfläche desselben Blattes weiterhin „unwiederbringlich weg".

a) Kopieren und Verschieben räumen auch in den Papierkorb — schließt aus, dass ein Überschreiben auf einem Datenträger ohne Papierkorb noch gelingt; es wird dort übersprungen und gemeldet.
b) Es bleibt ungleich, und der Grund wird im Code festgehalten — schließt aus, dass eine Schaltfläche eine Bedeutung hat; ein ganzer Ordner kann weiterhin endgültig fallen.
c) Das Blatt beschriftet die Schaltfläche je nach Fall verschieden — schließt aus, dass du dich auf eine Regel verlassen kannst; du musst den Text lesen.

**Empfehlung:** a, weil das dieselbe Wahl ist, die du am 25.08. für denselben Wortlaut derselben Schaltfläche schon getroffen hast.
**Noch aktuell?** ja. `ziel_klaeren` ruft im Zweig „Überschreiben" unverändert `loeschen::baum_entfernen` (`crates/krk-core/src/operation/mod.rs:484`).
**Datei:** `260826-1221_o_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`

### 44. Soll der Zehnerblock einer externen Tastatur ganz angeschlossen werden, halb bleiben oder ganz herausfallen?

Der Zehnerblock ist halb angeschlossen, und niemand hat das entschieden: seine Ziffern lösen aus, seine Eingabetaste, sein Komma und zwei seiner Rechenzeichen nicht. Der Zustand ist nebenbei entstanden, als der Nachschlag von Tastencodes auf gemeldete Zeichen umgestellt wurde.

a) So lassen, nur die falsche Erklärung im Code richtigstellen — schließt aus, dass Eingabetaste und Komma des Blocks belegbar werden.
b) Der Block kommt ganz herein, mit eigenen Namen für seine siebzehn Tasten — schließt nichts vom Heutigen aus, fügt nur hinzu; kostet siebzehn Einträge für Tasten, die dein MacBook nicht hat.
c) Der Block wird ganz abgewiesen — schließt aus, dass eine funktionierende Taste funktionierend bleibt, und hinge an einer ungemessenen Annahme, die im Fehlfall die Pfeiltasten träfe.

**Empfehlung:** a jetzt, b als eigene Runde, wenn du mit externer Tastatur danach fragst.
**Noch aktuell?** ja, mit einer Verschiebung: seit Runde 20 lösen `plus` und `minus` des Blocks aus (`crates/krk-core/src/tasten/parser.rs:367-368`). Eingabetaste, Komma, Stern und Schrägstrich weiter nicht.
**Datei:** `260826-1223_o_loesen-die-zifferntasten-des-zehnerblocks-dieselbe-funktion-aus-wie-die-obere-reihe.md`

### 45. Schreibt KRK dem Nutzer „Größe" oder „Groesse"?

In der Statuszeile stehen beide Schreibweisen nebeneinander: „ist beschaedigt" neben „Einträgen", teils in derselben Sitzung. Menü, Spaltenüberschriften und Editormeldungen tragen Umlaute, die Meldungen aus Ablage und Leseprofilen die Umschrift.

a) Umlaute für alles, was der Nutzer liest; die Umschrift bleibt für Kommentare und Bezeichner — schließt nichts Späteres aus; rund fünfzehn Zeichenketten sind nachzuziehen, dazu die Proben, die auf ihren Wortlaut prüfen.
b) Umschrift überall, auch in der Oberfläche — schließt aus, dass die deutsche Oberfläche richtig geschrieben ist („Groesse", „Ueber KRK"); trifft mehr Stellen als a.
c) Es bleibt gemischt — schließt aus, dass die nächste Runde die Frage nicht wieder von vorn entscheidet; so ist der heutige Zustand entstanden.

**Empfehlung:** a, weil die Trennlinie „liest das ein Mensch oder der Übersetzer" an jeder Zeichenkette entscheidbar ist und die Oberfläche schon heute mehrheitlich Umlaute trägt.
**Noch aktuell?** ja. Beide Schreibweisen stehen unverändert: `crates/krk-core/src/leseprofil/mod.rs:562` („traegt ein leeres Stueck") gegen `crates/krk-ui/src/spalten.rs:148` („Größe") und `crates/krk-ui/src/menuemodell.rs:117` („Über KRK").
**Datei:** `260826-1225_o_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`

### 46. Soll ein Test, der unter Administratorrechten nichts messen kann, still durchlaufen oder laut abbrechen?

Vier Tests stellen ihren Prüffall über entzogene Dateirechte her. Unter `root` greifen die Rechte nicht, der Fall tritt nicht ein. Zwei dieser Tests springen still ab, zwei würden schlicht rot — und keine der beiden Formen steht irgendwo als Regel.

a) Still überspringen mit einer Zeile auf der Fehlerausgabe — schließt aus, dass du merkst, wenn eine Zusage nicht gemessen wurde; die Zeile verschluckt `cargo test`.
b) Mit klarem Text abbrechen: „dieser Lauf kann die Zusage nicht messen" — schließt aus, dass ein Lauf unter `root` grün wird; er meldet vier rote Tests.
c) Den Prüffall ohne Rechte herstellen — trägt für diese vier nicht: „kein Leserecht" ist der Prüffall selbst.

**Empfehlung:** b, wenn dieser Baum nie unter `root` geprüft werden soll; falls doch, kippt die Abwägung zu a, und dann gehört der übersprungene Fall am Ende des Laufs gemeldet statt in eine verschluckte Zeile.
**Noch aktuell?** ja. `crates/krk-core/tests/text.rs:728` überspringt still, `crates/krk-core/tests/operation.rs:540` und `:783` prüfen die Voraussetzung nicht.
**Datei:** `260826-1302_o_schweigt-eine-probe-die-unter-root-nichts-messen-kann-oder-faellt-sie-aus.md`

### 47. Womit wird sichergestellt, dass eine Liste neben einer Aufzählung vollständig bleibt: mit einem Test oder mit einer fremden Bibliothek?

Der Baum führt fünfzehn Listen, die je einen Eintrag pro Variante einer Aufzählung haben sollen. Der Übersetzer prüft nur die Anzahl, nie die Namen. Ein vergessener Eintrag übersetzt und stürzt zur Laufzeit ab.

a) Ein Test liest die Varianten aus dem Quelltext und hält sie gegen die Liste — schließt aus, dass der Fehler beim Übersetzen auffällt; er fällt erst beim Testlauf.
b) Die Bibliothek `strum` einbinden und die Liste ableiten — schließt aus, dass das Projekt ohne diese Abhängigkeit auskommt; dafür wird der vergessene Eintrag ein Übersetzungsfehler.
c) Eine handgeschriebene Nummernzuordnung mit Zusicherung — schließt aus, dass es bei einer Liste bleibt; ersetzt eine Liste durch zwei.

**Empfehlung:** a für jetzt, mit Wiedervorlage, falls eine weitere Liste denselben Griff braucht; ein späterer Umstieg auf b streicht den Test, statt ihn umzubauen.
**Noch aktuell?** ja, und der Baum ist der Empfehlung weiter gefolgt, ohne sie zu bestätigen: `strum` steht nirgends, und der Helfer `varianten_der_aufzaehlung` läuft inzwischen an drei Aufzählungen (`crates/krk-core/tests/belegung.rs:1784`, `:2112`, `crates/krk-core/tests/git.rs:694`) statt an einer. Die Zahl der Listen ist von elf auf fünfzehn gestiegen (`grep -rn 'const ALLE' crates/*/src`).
**Datei:** `260826-1811_o_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`

### 48. Soll die Vorschau für den Archivordner eine zweite Zahl zeigen, obwohl die eigentlich gewünschte nicht zählbar ist?

Du hattest verlangt, das Profil für `fusion-workbench/archive/` solle beide Lesarten von „Einträgen" zeigen: die Archivläufe und die archivierten Dateien. Die zweite lässt sich mit dem festgelegten Bausteinsatz nicht zählen, weil die Zählung nur eine Ebene tief geht.

a) Es bleibt bei der einen Zeile „Läufe", der Kommentar erklärt die Lücke — schließt aus, dass du den Umfang der Ablage in der Vorschau siehst.
b) Eine zweite Zeile zählt die abgelegten Speicher (heute 15) — schließt aus, dass die Zahlen zusammenpassen: 5 Läufe neben 15 Speichern liest sich wie eine feinere Auflösung und ist keine; die wahre Dateizahl ist 167.
c) Der Bausteinsatz bekommt eine Tiefenangabe und zählt im Unterbaum — schließt aus, dass der Kostenhaushalt der Leseprofile ablesbar bleibt; ein Unterbaumlauf kostet, was erst am Bestand feststeht.

**Empfehlung:** a, weil die Zahl aus b in die falsche Richtung gelesen wird und eine irreführende Zeile schlechter ist als keine. c ist sachlich richtig, aber ein Eingriff in den Kern.
**Noch aktuell?** ja. `resources/default-readers.toml:501-511` führt unverändert „Läufe" und „Zuletzt abgelegt"; der erklärende Satz zur fehlenden Dateizahl steht im Kommentar noch nicht.
**Datei:** `260831-1353_o_bekommt-das-ablageprofil-eine-zweite-umfangszeile-obwohl-die-dateizahl-nicht-zaehlbar-ist.md`

### 49. Soll KRK Handänderungen an der Tastenbelegungsdatei schützen, oder bleibt es beim Hinweis?

Die Datei `keymap.toml` hat zwei Schreiber, die nichts voneinander wissen: dich mit einem Editor, und die Belegungsansicht auf `f1`. KRK liest die Datei nur beim Start; die Belegungsansicht schreibt beim Verlassen den Stand vom Start zurück. Jede Handänderung seit dem Start ist danach fort, ohne Rückfrage.

a) Es bleibt beim Hinweis in der Statuszeile — schließt aus, dass gewarnt wird, wer die Datei außerhalb von KRK öffnet; der Hinweis fällt außerdem mit dem nächsten Tastenbefehl.
b) KRK lädt die Datei nach, sobald sie sich auf der Platte ändert — schließt aus, dass die Anwendung ohne einen zweiten Dateibeobachter auskommt; Menü und Tastenabgriff werden zu einem Zeitpunkt neu gebaut, den du nicht ausgelöst hast.
c) Die Belegungsansicht liest vor dem Sichern nach und fragt bei Abweichung — schließt aus, dass das Verlassen der Ansicht ohne Rückfrage bleibt.

**Empfehlung:** keine; der Datensatz spricht selbst keine aus. Die Kosten von b (ein neuer Beobachter) und c (eine weitere Rückfrage) sind zu verschieden, um sie ohne dich zu wägen.
**Noch aktuell?** ja. Der Hinweis steht (`crates/krk-ui/src/kommandos/operationen.rs:1399`, gerufen aus `crates/krk-ui/src/appkit/anwendung.rs:4345`); ein Nachladen oder eine Rückfrage gibt es nicht.
**Datei:** `260901-0734_o_haelt-krk-die-belegungsdatei-gegen-ihren-zweiten-schreiber-oder-bleibt-es-beim-hinweis.md`

### 50. Was passiert mit einer Meldung, die entsteht, während ein Dialogblatt die Statuszeile verdeckt?

Ein Dialogblatt sitzt senkrecht mittig im Fenster. Bei der Startgröße bleibt die Statuszeile frei; bei einem klein gezogenen Fenster verdeckt schon das Stapelumbenennen sie ganz, und dann siehst du die Meldung nicht — und wenn danach der Vorgang selbst in die Zeile schreibt, nie.

a) Es bleibt bei der Statuszeile — schließt aus, dass die Meldung bei kleinem Fenster ankommt; dafür bleibt es bei einem Meldungsweg.
b) Das Blatt selbst nimmt die Meldung auf — schließt aus, dass es beim einen Meldungsweg bleibt; alle elf Blätter bräuchten eine Stelle dafür.
c) Zusätzlich ein Ton bei jeder Abweisung — schließt aus, dass die Auskunft sagt, was nicht ausgeführt wurde; der Ton zeigt nur, dass es einen Satz gibt.
d) Das Fenstermindestmaß steigt, bis auch das höchste Blatt die Zeile frei lässt — schließt aus, dass du dein Fenster so klein ziehen kannst wie heute; die Zahl veraltet mit dem nächsten Blatt.

**Empfehlung:** a vorerst, weil der gemessene Fall (Startgröße 1280 × 720) die Zeile frei lässt und für die Lücke kein Befund vorliegt. c ist der billigste Zusatz, falls sie jemanden trifft.
**Noch aktuell?** ja. Der Satz steht an einer Stelle (`crates/krk-ui/src/kommandos/blattmeldung.rs:145`), ein zweiter Weg oder ein Ton ist nicht gebaut.
**Datei:** `260904-2047_o_wohin-geht-die-blattmeldung-wenn-das-blatt-die-statuszeile-verdeckt.md`

### 51. Steht die Installationsregel künftig an drei Stellen im Wortlaut, oder an einer mit Verweisen?

Die Regel „die neue Fassung über die alte kopieren, die alte nicht vorher löschen" stammt aus der Untersuchung des Lesezeichenverlusts und steht ausformuliert an drei Stellen, die du beim Installieren liest: im Kopf der README, im festen Text jeder Releaseseite, und seit dem 05.09. in der Anleitung, die im Releasepaket mitreist. Nur eine der drei ist von einem Test gehalten.

a) Drei Stellen bleiben, jede mit ihrem eigenen Lesemoment — schließt aus, dass die drei Wortlaute aneinander gehalten werden; sie laufen auseinander.
b) Die Anleitung wird die eine Quelle, die anderen zwei verweisen darauf — schließt aus, dass die Regel auf der Releaseseite ohne Download lesbar bleibt; genau das war die Zusage aus der Untersuchung.
c) Der Releasetext bleibt der Wortlaut, und das Bauwerkzeug setzt ihn beim Packen in die Anleitung ein — schließt aus, dass die ausgelieferte Anleitung dieselbe Datei ist wie die eingecheckte.

**Empfehlung:** keine; der Datensatz spricht keine aus. Die Wahl hängt daran, was dir mehr wert ist: ein Wortlaut oder drei erreichbare Leser. b bricht dabei die Zusage an der Stelle, an der sie erkämpft wurde.
**Noch aktuell?** ja, unverändert drei Stellen: `README.md:35`, `HowTo.md:15`, `xtask/src/veroeffentlichung.rs:750`.
**Datei:** `260905-1659_o_wo-wohnt-die-betriebsregel-jetzt-da-sie-den-nutzer-an-drei-stellen-erreicht.md`

### 52. Bekommen die drei Testordner-Fassungen die Verfallwarnung des Übersetzers, oder keine von ihnen?

Diese Hilfsobjekte räumen ihren Ordner auf, sobald sie freigegeben werden. Wer eines anlegt, ohne es festzuhalten, legt einen Namen fest und räumt sofort wieder ab — genau der Fall, für den das Projekt seit dem 11.08. die Verfallwarnung setzt. Keine der drei Fassungen trägt sie, und sie sollen zeichengleich bleiben.

a) Alle drei bekommen sie, in einem Durchgang über drei Teilprojekte — schließt aus, dass die Fassungen auseinanderlaufen; kostet drei Zeilen in drei Teilprojekten.
b) Keine bekommt sie, und der Verzicht wird im Modulkopf begründet — schließt aus, dass die Projektregel hier gilt; die Begründung „ist doch offensichtlich" ist genau die, die die Regel von 260811 verworfen hat.
c) Nur die Fassung im Messwerkzeug bekommt sie — schließt aus, dass die drei gleich bleiben; die nächste Durchsicht liest die Abweichung als Fehler.

**Empfehlung:** a, zusammen mit dem nächsten Durchgang, der ohnehin alle drei Teilprojekte anfasst; c wäre der billigste Eingriff und die teuerste Folge.
**Noch aktuell?** ja. Keine der drei trägt die Warnung (`crates/krk-bench/src/wegwerfordner.rs:39`, `crates/krk-ui/src/pruefordner.rs:53`, `crates/krk-core/tests/gemeinsam/mod.rs:97`). Der Nebenbefund `260826-1305` ist inzwischen geschlossen, ohne diese Frage zu berühren.
**Datei:** `260905-2155_o_bekommen-die-drei-pruefordner-fassungen-must-use-oder-keine.md`

### 53. Werden zwei der vier Messordner künftig auch gegen ihren tatsächlichen Inhalt geprüft, oder bleibt es bei der ehrlichen Beschriftung?

Ein Messordner ist erst gedeckt, wenn zweierlei stimmt: die Zusage neben dem Ordner und die tatsächlich gelesene Eintragszahl. Für zwei der vier Ordner gibt es nur die Zusage. Ein hineingerutschter `.DS_Store` fiele bei ihnen nicht auf, und drei Zeitzusagen messen dann auf einem Bestand, den keine Zusage meint.

a) Es bleibt bei der Beschriftung; der Berichtskopf unterscheidet beide Lagen schon sichtbar — schließt aus, dass alle vier Ordner geprüft sind; der Unterschied hängt an einer Formulierung.
b) Die zweite Hälfte wird nachgezogen, ein Zählen ohne Zeitmessung genügt — schließt aus, dass der zweite Messordner kalt gemessen wird; das Vorablesen wärmt den Systemcache, und genau dagegen sind die zwei getrennten Startwerte gebaut.
c) Nur der kleine Unterordner bekommt sie — schließt aus, dass die Regel für alle vier gilt; die Ausnahme muss wieder erklärt werden.

**Empfehlung:** a, mit einem Satz im Berichtskopf, der den Unterschied benennt statt ihn an einer Formulierung hängen zu lassen. b kauft die Deckung mit genau der Cache-Wirkung, gegen die die Anlage gebaut ist.
**Noch aktuell?** ja. Der Berichtskopf trägt weiter beide Formen (`crates/krk-bench/src/bericht.rs:100-106`), der zugrunde liegende Befund `260826-2155_*_pruefordner-b-und-der-l6-unterordner-werden-nur-gegen-ihren-steckbrief-gehalten-und-der-kommentar-sagt-b-werde-nicht-gelesen.md` ist offen.
**Datei:** `260905-2155_o_bekommen-pruefordner-b-und-der-l6-unterordner-die-zweite-haelfte-der-deckung.md`

### 54. Bleiben Fehlermeldungen über nicht mehr änderbare Texte für immer in der Liste offener Arbeit?

Neun Datensätze beschrieben Fehler im Text abgeschlossener Runden. Solche Texte werden nicht nachgeführt, also schien der Befund richtig und trotzdem unschließbar. Die Frage war, wie man ihn dann führt.

a) So lassen, sie bleiben offen (aus dem Text erschlossen) — schließt aus, dass eine Erhebung offener Arbeit nur echte Arbeit zählt.
b) Als erledigt schließen, mit einem Vermerk, der die Unbehebbarkeit ausschreibt (aus dem Text erschlossen) — schließt aus, dass „erledigt" nur „behoben" heißt.
c) Ins Archiv verschieben — schließt aus, dass die Aussage neben dem Text steht, über den sie spricht.

**Empfehlung:** entfällt.
**Noch aktuell?** nein, weil der Baum das inzwischen entschieden hat: du hast die Einordnung „nicht behebbar" am 06.09. zurückgewiesen, und die vierte Behebungsschleife hat 30 von 36 dieser Datensätze geschlossen, davon 29 über einen Nachsatz unter dem unveränderten Bestandstext (Commit `8276170`). Alle neun namentlich genannten Datensätze tragen heute den Abschlussmarker, etwa `260813-1345_*_neun-abnahmekriterien-tragen-probe-und-haben-keine.md` im Speicher der Runde 8. Die Verlaufsdatei sagt es selbst: „Diese Schleife macht sie weitgehend gegenstandslos" (`260906-0509-coder-vierte-behebungsschleife-die-klasse-s-neu-sortiert.md:169-172`). Nur Frage 56 unten ist von dem Rest übrig.
**Datei:** `260906-0202_o_werden-defektdatensaetze-ueber-eingefrorene-spec-und-plantexte-geschlossen-oder-bleiben-sie-offen.md`

### 55. Werden neunzehn Verlaufsdateien mit falscher Uhrzeit im Namen umbenannt, oder bekommen sie nur einen Vermerk?

Vier offene Datensätze halten denselben Vorgang fest: ein Agent hat die Uhrzeit im Dateinamen geschätzt statt sie abzufragen, teils drei Stunden zu früh. Der Verlaufsspeicher wird nach dem Namen sortiert gelesen, und die Reihenfolge ist die einzige Auskunft darüber, was worauf folgte; hier erscheint Schritt 2 nach Schritt 7.

a) Umbenennen auf die belegte Zeit, alle Verweise nachziehen — schließt aus, dass Aufzeichnungen unangetastet bleiben; die Uhrzeit im Namen ist zugleich die Kennung, über die zitiert wird, und das Suchmuster muss die Kurzform ohne `.md` mitnehmen, sonst entstehen tote Zeiger.
b) Vermerk am Datensatz, Namen bleiben — schließt aus, dass der Verlaufsspeicher selbst richtig sortiert; wer nur die Namensliste überfliegt, erfährt vom Vermerk nichts.
c) Vermerk im Kopf jeder betroffenen Datei — schließt aus, dass es bei vier Handgriffen bleibt; es sind neunzehn Dateien, und wer nur die Liste überfliegt, öffnet keine.
d) Die Ursache prüfbar machen und den Bestand liegen lassen — schließt nichts von a bis c aus; behebt aber nichts vom Vorhandenen und braucht eine Toleranzgrenze, über die wieder zu entscheiden wäre.

**Empfehlung:** b für den Bestand und d als eigene Frage daneben, weil der Schaden real, aber kleiner ist als der Preis, neunzehn Aufzeichnungen umzubenennen und jeden Verweis über ein Suchmuster nachzuziehen, das in diesem Projekt schon fünfmal zu eng war.
**Noch aktuell?** ja. Alle vier Datensätze sind offen (`260824-1758_*_die-zeitstempel-in-dateinamen-laufen-der-uhr-voraus-bis-zu-drei-stunden.md`, `260818-0343_*_zwei-dateien-dieser-sitzung-tragen-einen-zeitstempel-fast-zwei-stunden-in-der-zukunft.md`, `260812-1805_*_sechs-sitzungsprotokolle-tragen-einen-zeitstempel-aus-der-zukunft.md`, `260828-1044_*_fuenf-history-dateien-der-runde-20-tragen-zeitstempel-die-nach-ihrem-eigenen-commit-liegen.md`), keine Datei ist umbenannt.
**Datei:** `260906-0206_o_werden-dateinamen-mit-vorauslaufendem-zeitstempel-umbenannt-oder-vermerkt.md`

### 56. Darf ein Agent eine doppelte Überschrift in einem abgeschlossenen Text umbenennen, oder nur einen Hinweis daruntersetzen?

Deine Entscheidung vom 06.09. erlaubt, eine falsche Angabe in einem abgeschlossenen Text als Nachsatz zu berichtigen, nie im Text selbst. Damit wurden 24 Datensätze geschlossen. Einer bleibt: ein Text trägt zweimal dieselbe Überschrift, eine Suche findet nur die erste. Falsch ist nichts, fehlend ist die Unterscheidbarkeit, und die stellt allein ein Eingriff in den Bestandstext her.

a) Die Regel deckt es nicht, der eine Datensatz bleibt offen — schließt aus, dass er je geschlossen wird; dafür bleibt die Regel nach dem Ort entscheidbar: alles Neue steht unten, nichts Altes wird angefasst.
b) Eine Überschrift ist Form und kein Sachtext und darf berichtigt werden, mit Vermerk im Nachsatz — schließt aus, dass die Regel ohne Auslegung anwendbar bleibt; „Form" und „Sachtext" sind nicht nach dem Ort trennbar, und die Grenze wandert beim nächsten Mal.
c) Die Umstellung gehört dir oder einem Lauf mit ausdrücklichem Auftrag, nicht einem Ausführer — schließt aus, dass die Sache sich von selbst erledigt; sie wartet, bis jemand einen solchen Lauf ansetzt.

**Empfehlung:** a, weil der Preis ein einzelner offener Datensatz geringer Schwere ist und der Gewinn eine Regel, die keine Auslegung braucht; die Schleife hat 30 von 36 Fällen nur deshalb geschlossen, weil die Regel scharf war.
**Noch aktuell?** ja. `260819-1440_*_ein-spec-traegt-zwei-reconciliation-log-ueberschriften-und-eine-suche-findet-nur-die-erste.md` ist offen; der Nachsatz im Text der Runde 10 weist auf beide Abschnitte hin, hebt den Befund aber nicht auf.
**Datei:** `260906-0509_o_deckt-die-nachsatzregel-auch-eine-umstellung-im-bestandstext-ab.md`

## Implications

Siebzehn der achtzehn Fragen sind unverändert offen und am heutigen Baum belegt. Eine ist gegenstandslos: Nummer 54 ist von der vierten Behebungsschleife überholt, deren Auslöser dein eigener Einspruch war.

Drei Fragen hängen an derselben fehlenden Messung. Nummer 41 und 42 lassen sich beide erst entscheiden, wenn feststeht, was ein Durchlauf ohne Treffer über einen echten Unterbaum kostet; Nummer 42 sagt das selbst. Wer eine Messrunde ansetzt, beantwortet zwei Fragen mit einer Zahl.

Zwei Fragen betreffen dieselbe Schaltfläche und denselben Wortlaut, den du am 25.08. schon einmal entschieden hast: Nummer 43 zieht die damalige Antwort auf Kopieren und Verschieben durch oder lässt zwei Bedeutungen stehen.

Bei drei Fragen spricht der Datensatz bewusst keine Empfehlung aus (49, 51 und in Teilen 46), weil die Abwägung von deiner Gewichtung abhängt und nicht von einer Messung.

## Recommendations

Die achtzehn Blöcke sind zur Vorlage im Chat bestimmt. Nummer 54 kann übersprungen werden. Bei den übrigen siebzehn genügt je ein Buchstabe; die Antworten trägt anschließend ein Abgleichslauf in die Zeile `Answered:` der jeweiligen Datei ein.

## Filed Issues

keine

## Sources

- Achtzehn Datensätze unter `fusion-workbench/shared/decisions/`, je vollständig gelesen
- `crates/krk-core/src/verzeichnis/filter.rs:240-242`, `crates/krk-core/src/verzeichnis/modell.rs:1029-1031`, `crates/krk-ui/src/tabs.rs:1077`
- `crates/krk-core/src/operation/mod.rs:471-491`
- `crates/krk-core/src/tasten/parser.rs:367-368`
- `crates/krk-core/src/leseprofil/mod.rs:562`, `crates/krk-ui/src/spalten.rs:148`, `crates/krk-ui/src/menuemodell.rs:117`
- `crates/krk-core/tests/text.rs:721-728`, `crates/krk-core/tests/operation.rs:540`, `:783`
- `crates/krk-core/tests/gemeinsam/mod.rs:97`, `:458`; `crates/krk-core/tests/belegung.rs:1784`, `:2112`; `crates/krk-core/tests/git.rs:694`
- `resources/default-readers.toml:491-511`
- `crates/krk-ui/src/kommandos/operationen.rs:1399`, `crates/krk-ui/src/appkit/anwendung.rs:4345`
- `crates/krk-ui/src/kommandos/blattmeldung.rs:145`
- `README.md:35`, `HowTo.md:15`, `xtask/src/veroeffentlichung.rs:750`
- `crates/krk-bench/src/wegwerfordner.rs:39`, `crates/krk-ui/src/pruefordner.rs:53`, `crates/krk-bench/src/bericht.rs:100-106`
- `xtask/src/release.rs:639`, `xtask/src/bundle.rs:463`, `xtask/src/version.rs:303`
- `260906-0509-coder-vierte-behebungsschleife-die-klasse-s-neu-sortiert.md:150-172`
- `260906-0203_*_darf-ein-agent-den-spec-oder-plan-einer-geschlossenen-runde-berichtigen.md:44`
- `git show --stat 8276170`

## Open Questions

- [ ] Die siebzehn noch offenen Fragen selbst; sie sind der Gegenstand dieser Vorlage.
