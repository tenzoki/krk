# Consultation: Schlussdurchsicht des Arbeitspakets „F2 öffnet ~/krkhome“ vor Abschluss und Auslieferung

**Date:** 2026-09-26 10:47
**Status:** Complete
**Requested by:** Kai Stalmann, über den Orchestrator; Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`

## Question

Ist das Arbeitspaket fertig genug, um es als erledigt zu markieren und eine Auslieferung zu erwägen? Sechs Einzelfragen: (1) Deckung der Abnahmekriterien C1–C7 durch Code und Probe, (2) Sicherheit von `.secrets.txt` unter dem Bedrohungsmodell des Nutzers, (3) Einordnung der vier offenen Defekte, (4) Lücken in `CLAUDE.md`, (5) Berührung der zehn Zeitzusagen, (6) die Liste der Nutzerprüfungen.

## Kurzurteil

**Noch nicht auslieferbar. Vor der Auslieferung stehen drei kleine Dinge, keines davon eine Fähigkeit.**

1. **`make check` ist auf HEAD rot.** Die Ursache ist ein Datensatz und kein Code: der geschlossene Defekt `260926-1010_*_…` schreibt seinen Abschlussvermerk als `**Resolved:**`, und die Probe `jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk` (`xtask/src/werkbank.rs:100`) verlangt `Resolved:` am Zeilenanfang (`xtask/src/werkbank.rs:44`). Alle übrigen Proben sind grün.
2. **Die Zellen der drei Eintragstabellen korrigieren beim Tippen.** Der eigene Feldeditor `Zelleneditor` (`crates/krk-ui/src/appkit/eintragsansicht.rs:675-684`) ruft `textautomatik::automatiken_abschalten` nicht. Autokorrektur, typografische Anführungszeichen, Textersetzung und die Schreibwerkzeuge stehen dort auf den Werten des Systems. In `.secrets.txt` ändert das ein Passwort still, bevor es verschlüsselt wird.
3. **Eine leere `.secrets.txt`, über eine dritte Schreibweise des Ordners geöffnet, geht über den Klartextweg.** Jede Sperre für die Geheimnisse hängt an `Editormodell::ist_geheimnisdatei` (`crates/krk-ui/src/editormodell.rs:1321`), und die erkennt nur zwei Pfadformen. Über eine dritte Form öffnet der Editor die null Bytes als gewöhnlichen Text, und `cmd+s` schreibt Klartext. Der Spec sagt an `…_spec-…md:466` wörtlich: „KRK öffnet `.secrets.txt` nie über den gewöhnlichen Textweg.“

Dazu kommen zwei Punkte, die keine Mängel sind. Erstens verlangt die Auslieferung nach den Regeln in `README.md` die Stufe **Major** und nicht Minor. Zweitens ist der Arbeitsbaum nicht sauber: `fusion-workbench/orchestrator-events.jsonl` ist geändert, und Station 1 bricht an einer geänderten verfolgten Datei ab.

## Context

- 29 Commits zwischen `d5fb749` und `e22c255`, 54 Dateien unter `crates/`, `xtask/`, `resources/` und in den drei Anleitungen, +15 723 / −3 233 Zeilen.
- Der jüngste Tag ist `v1.11.1` auf `0d4485e`. Er liegt vor `d5fb749`, also ist keine der fünf Stufen einzeln ausgeliefert worden; eine Auslieferung jetzt bringt alle fünf auf einmal.
- `make check` habe ich am 260926 vor 10:47 auf `e22c255` gefahren. Ergebnis: rot, genau eine Probe, siehe Kurzurteil 1; alle übrigen Läufe grün. `crates/` und `resources/` sind grün, `cargo tree` für beide Mac-Ziele ohne `cc` und ohne `-sys`-Paket.
- Die Zuordnung der Kriterien zu Code und Probe habe ich auf zwei Hilfsagenten verteilt und ihre Stichproben selbst nachgelesen (`editormodell.rs:3769`, `verzeichnis/modell.rs:904-930`, `anwendung.rs:6652-6685`).

## Analysis

### 1. Abnahmekriterien

**C1, C2, C3, C4 und C6: jedes Kriterium ist gebaut, und fast jedes hält eine Probe.** Ein Hilfslauf hat `cargo test -p krk-core -p krk-ui --no-fail-fast` gefahren: 1 890 Proben grün, keine rot.

| Kriterium | Befund |
|---|---|
| C1.1 Belegung | `resources/default-keymap.toml:1360-1362` „Notizordner öffnen“ mit `f2` und `cmd+k`, keine Kombination entfernt (am Diff erhoben). `f2` hält eine Probe (`crates/krk-core/tests/belegung.rs:1756`), **`cmd+k` keine** |
| C1.2–C1.4 | Proben `belegung.rs:1717` (Nutzerbelegung mit `notizzettel`), `kommandos/zulaessigkeit.rs:1358` (Blatt), `anwendung.rs:10034` (eigener Zweig) |
| C2.1–C2.5 | exklusives Anlegen mit Rennprobe (`heimordner/bereitstellen.rs:374`, Probe `:459`), kein Anlegen beim Start (`anwendung.rs:10144`), Rundlauf auch für `* [X]` und Einrückung (`tests/heimordner.rs:311`, `:328`), fremde Zeilen (`:398`) |
| C2.6 eine Erkennung | `Heimordner::ist`/`sonderdatei` (`heimordner/mod.rs:204`, `:216`); alle Frager der Oberfläche gehen darüber. Proben: Verweis und Ziel (`tests/heimordner.rs:54`), kein Systemaufruf (`:253`), das Wort `krkhome` nur an der Erkennung (`tests/baum.rs:1397`). **Keine Probe für die dritte Schreibweise**, und die „eine Stelle“ hält nur die Wortprobe |
| C3.1 kein Blatt | die Dateien sind gelöscht, **keine eigene Probe** |
| C3.2–C3.6 | Übernahme nur beim Anlegen des Ordners, `## `-Zettel abgewiesen, alte Zettel unverändert, altes `session.toml` lesbar: je eine Probe (`tests/heimordner.rs:1101`, `:1132`, `:1202`, `:1217`, `:1006`; `tests/ablage.rs:3736`). Dass die Meldung in der Statuszeile ankommt (`anwendung.rs:5058`), hält keine Probe |
| C4.1–C4.3 | Proben `vorschaumodell.rs:1167`, `:1198`, `:1217` (Bytes und Änderungszeit unverändert), `markdown.rs:3330` |
| C6.1–C6.8 | Proben für alle acht (`tests/heimordner.rs:472-607`, `appkit/editor.rs:7205`, `:7251`, `:7275`, `:7563`, `zulaessigkeit.rs:1551`, `anwendung.rs:9989`, `kommandos/operationen.rs:3016`). Budget und `esc` in der Zelle hängen überwiegend an Quelltextproben |

Eine Abweichung im Wortlaut: C6.6 sagt „allein, wenn der Editor `tasks.txt` hält“. Die fünf `Eintrag*`-Befehle wirken aber auch an `notes.txt` und `.secrets.txt`. Das verlangt C5.4 („dieselben Befehle“), und es ist richtig so. Den Widerspruch trägt der Wortlaut von C6.6, nicht der Code.

**C5 und C7: alle Kriterien gebaut, bis auf zwei Stellen von einer Probe gehalten.**

| Kriterium | Befund |
|---|---|
| C5.1–C5.7 | gebaut und von Proben gehalten (`crates/krk-core/tests/heimordner.rs:698`, `:738`, `:806`, `:905`, `:935`; `appkit/editor.rs:8355`, `:8420`; `anwendung.rs:9989`) |
| C5.8 `cmd+return` übernimmt | gebaut (`appkit/editor.rs:2573`), **ohne Probe**; gehalten ist nur, dass `handlung_ausfuehren` zuerst `zelle_uebernehmen` ruft |
| C5.9 `esc` leert keinen Filter | gebaut, die Zelle steht in `abbrechen` vor dem Filtertext (`anwendung.rs:6674-6685`, selbst nachgelesen), **ohne Probe** für diese Reihenfolge |
| C7.1–C7.16 | gebaut und von Proben gehalten; Einzelnachweise unten unter Frage 2 |
| C7.7 „ältere Formatversion“ | nur der Teil mit abweichenden Parametern ist belegt (`heimordner.rs:1537`). Eine zweite Version gibt es nicht, und `Kopf::lesen` weist jede andere ab (`tresor.rs:341`). Das ist richtig so, lässt sich heute aber nicht prüfen |
| C7.17 kein C-Code | am `cargo tree` beider Ziele erhoben, Begründung in der Wurzel-`Cargo.toml` |
| C7.18, C7.19 | `README.md:128-175` beschreibt den Kopf, `heimordner.rs:1537` baut eine Datei allein nach dieser Beschreibung; `HowTo.md:690-795` nennt PIN, Grenze und Zwischenablage |

**Keine Lücke ist ein nicht erfüllter Kriteriumstext.** Die vier fehlenden Proben (C5.8, C5.9, die Reihenfolge in `abbrechen`, „kein Blatt“ in C7.1) sind Nachträge und halten die Auslieferung nicht auf. Der eine inhaltliche Widerspruch zum Spec steht unter Frage 2, Befund A.

### 2. Sicherheit von `.secrets.txt`

**Unter dem Bedrohungsmodell (versehentliches Lesen durch Agenten, Werkzeuge und Indexer) hält der Bau, mit einer Lücke im Klartextweg und einer Schwäche an den Zellen.** Die Wege, die ich geprüft habe:

| Weg | Befund | Beleg |
|---|---|---|
| Sichern des Editors | nur Chiffrat; der Schreibweg nimmt den Typ `Chiffrat` und keine Bytes, verschlüsselt wird vor `atomar::schreiben` | `editormodell.rs:600-617`, `:1597-1640` |
| Nachbardatei des atomaren Schreibens | trägt Chiffrat; die Probe fängt die Bytes vor dem `rename` ab | `editormodell.rs:3477` |
| PIN ändern | schreibt neu verschlüsselt über denselben Weg | `editormodell.rs:1790`, Probe `:3873` |
| `session.toml` | nennt `.secrets.txt` nie als Editordatei; die Tabs tragen höchstens den **Namen** als Auswahl, keinen Inhalt | `fenstermodell.rs:109-111`, Probe `:3153`; `ablage/sitzung.rs:91` |
| Lesezeichen (Textmarke) | verweigert; behoben mit `3c6905b` | `editormodell.rs:1139`, Probe `:3372` |
| Tastenprotokoll auf stdout | verdeckt bei stehendem PIN-Blatt oder gehaltener `.secrets.txt`; behoben mit `e22c255` | `appkit/ereignisse.rs:670`, Proben `geheimes_tippen_steht_verdeckt_im_protokoll` und die Gegenprobe |
| Menüprotokoll, Messmodus, übrige `println!`/`eprintln!` | geben Menütitel, Zahlen und Fehlertexte aus, keinen Dateiinhalt; die `println!` in `editor.rs:5147` und `hervorhebung.rs:1599/1640` stehen in Proben | selbst per `grep` erhoben |
| Statuszeile | zeigt nur auf dem Bildschirm; die Abweisung einer `## `-Zeile nennt die Regel und nicht die Zeile | `heimordner/eintraege.rs:463` |
| Vorschau | liest für `.secrets.txt` nichts, auch nicht für die leere; die Probe fährt sie mit Dateimodus `000` | `vorschaumodell.rs:830`, Probe `:1259` |
| Inhaltsfilter in `~/krkhome` | kein Inhaltsauftrag, bei beiden Stellungen des Umschalters | `verzeichnis/modell.rs:917-924`, Probe `crates/krk-core/tests/verzeichnis.rs:1658` |
| Tiefe Suche aus `~` | liest das Chiffrat, wie entschieden (`260926-0050_*_wie-weit-reicht-der-inhaltsfilter-…`) | — |
| Rückgängigstapel | wird beim Schließen und bei jedem Dateiwechsel geleert | `appkit/editor.rs:3044-3057`, `:3386` |
| Wiederherstellung durch macOS | kein Ordner `~/Library/Saved Application State/*krk*` auf diesem Gerät | selbst erhoben |
| Schreibwerkzeuge an der Rohansicht | abgeschaltet | `appkit/textautomatik.rs:162`, `:172` |
| Zwischenablage | erlaubt, entschieden und in `HowTo.md:788` benannt | `260926-0033_*_…` |
| Ziehen in den Finder, Dienste | offen, siehe Frage 3 | `260926-1011_*_…` |

**Befund A — die dritte Schreibweise öffnet eine leere `.secrets.txt` als Text.** Alle drei Sperren fragen dieselbe Stelle, `ist_geheimnisdatei` (`editormodell.rs:1321`): das Öffnen ohne PIN (`:1276`), die Abweisung eines im Klartext gelesenen Standes (`:1349`) und der Klartextzweig des Sicherns (`:1611-1618`). Diese Stelle fragt `Heimordner::sonderdatei` und damit einen Textvergleich gegen zwei Pfadformen (`crates/krk-core/src/heimordner/mod.rs:204-224`). Erreicht der Nutzer den Ordner über eine dritte Form, antworten alle drei „kein Geheimnis“. Beispiele: ein anders geschriebener Pfad in der Pfadeingabe (`~/KrkHome` auf einem Dateisystem ohne Unterscheidung der Schreibung), `/System/Volumes/Data/Users/<name>/krkhome`, ein zweiter Verweis, oder ein Verweisziel über einen weiteren Verweis vor dem ersten F2 einer Sitzung (`heimordner/mod.rs:54-59`). Dann gilt:

- Eine Datei mit Chiffrat ist harmlos. `String::from_utf8` weist die Zufallsbytes ab (`text/datei.rs:832`), der Editor meldet „kein Text“.
- **Eine Datei mit null Bytes, also genau der Zustand, in dem F2 sie anlegt, öffnet als leerer Text.** Tippt der Nutzer seine Geheimnisse hinein und sichert, stehen sie im Klartext auf der Platte, wohl in einem Ordner, den ein Werkzeug gerade mitliest. Das nächste Öffnen über den normalen Weg meldet dann einen Kopfschaden, und der Klartext bleibt liegen.

Der Nutzer hat die zwei Pfadformen gewählt (`260926-0115_*_…`). Der Datensatz und der Modulkopf nennen als Preis nur, dass `notes.txt` und `tasks.txt` wie gewöhnliche Dateien erscheinen und „nichts verloren“ geht. Den Klartextweg für die Geheimnisse nennen weder der Datensatz noch der Modulkopf. **Inference:** Der Fall ist selten, weil F2 immer die geschriebene Form öffnet. Die Folge trifft aber genau das, was Stufe 5 schützen soll. Und sie widerspricht dem Satz des Spec an `:466`.

**Befund B — die Zellen korrigieren beim Tippen.** `Zelleneditor::neu` (`eintragsansicht.rs:675-684`) setzt nur `setFieldEditor` und `setAllowsUndo`. `automatiken_abschalten` hat genau einen Rufer, die Rohansicht (`appkit/editor.rs:4537`). Der Modulkopf von `textautomatik.rs` hält fest, was dieselbe Klasse `NSTextView` ab Werk tut. Autokorrektur, Ersetzungen und Vorhersage stehen dort auf „System wählt“, und `smartInsertDelete` ist an. **Inference, nicht am Bündel ausgelöst:** Für den Feldeditor der Zellen gilt dasselbe, weil er dieselbe Klasse ist und KRK ihn selbst baut. In `notes.txt` und `tasks.txt` ist das ein Ärgernis. In `.secrets.txt` ändert es ein Passwort still: `teh` wird `the`, `"` wird `“`. Außerdem bieten die Schreibwerkzeuge an, markierten Text umzuschreiben. Sie tun das nur auf eine bewusste Handlung des Nutzers hin, aber der Datensatz `260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md` schließt sie für Flächen aus, deren Inhalt in eine Datei zurückgeschrieben wird. Genau das ist eine Zelle.

**Hält eine Probe „kein Klartext auf der Platte“ gegen einen Rückfall?** Zum Teil. Es sind drei Klassen:

- **Der Schreibweg des Editors ist gut gehalten.** `kein_weg_schreibt_klartext_nach_secrets_txt` (`editormodell.rs:3769`) liest den Quelltext. Sie hält fest, dass `atomar::schreiben` genau einmal steht und nur ein `Chiffrat` nimmt, dass ein `Chiffrat` genau an einer Stelle entsteht und dass `datei::sichern` genau einen Rufer in `krk-ui` hat, der vorher die Erkennung fragt. Ein zweiter Klartextschreiber im Editormodell oder in `krk-ui` macht sie rot. Die Probe an der Nachbardatei (`:3477`) hält die Bytes selbst.
- **Seitenwege sind einzeln gehalten, nicht als Klasse.** Textmarke und Tastenprotokoll haben je eine eigene Probe. Einen neuen Seitenweg sieht keine Probe, etwa eine Ablage, die eine Editorzeile mitnimmt, oder eine neue Ausgabe. Diese Klasse hat in dieser Arbeit zweimal zugeschlagen (`260926-1004`, `260926-1010`). `Editormodell::haelt_geheimnisse` (`editormodell.rs:1157`) ist als „die eine Antwort für jeden Weg“ angelegt, aber ob ein neuer Weg sie fragt, prüft nichts. Die Probe `die_zeile_der_schreibmarke_hat_genau_einen_rufer` ist der Keim einer Klassenprobe: eine Zählprobe über die Leser von `Editormodell::stand` außerhalb des Editors fände den nächsten Seitenweg.
- **Befund A sieht keine Probe**, denn jede Probe fragt die Erkennung und nicht den Ordner.

### 3. Die vier offenen Defekte

| Defekt | Urteil | Begründung |
|---|---|---|
| `260926-0257` Deskriptor in `Textstand::Unlesbar` | **später** | ein Feld ohne Leser, kein Verhalten; Aufräumen |
| `260926-0840` `cmd+z` im Umbenennungsfeld | **später** | kein Datenverlust (`shift+cmd+z` stellt her), nichts erreicht die Platte; auch bei offener `.secrets.txt` bleibt die Wirkung im Speicher. Die Behebung ist die Ausdehnung des vorhandenen `Zelleneditor`-Musters über `windowWillReturnFieldEditor:toObject:` (`appkit/fenster.rs`) auf die Namenszelle, kein neuer Mechanismus |
| `260926-1011` Ziehen in den Finder | **vor der Auslieferung zu beantworten, nicht zu bauen**; Empfehlung unten | — |
| `260926-1012` wackelnde Probe | **später, aber beobachten** | nur die Probe, nicht der Betrieb: KRK hält sein Sitzungsrecht die ganze Laufzeit. **Inference:** Die Nachbarproben starten einen Kindprozess (`crates/krk-core/tests/ablage.rs:2616`, `:2718`), und ein Kind, das beim Start den Deskriptor eines parallel laufenden Halters kurz miterbt, hält dessen `flock`. Das passt zum Befund, ist aber nicht nachgewiesen. Die Auslieferung hängt nicht an `make check` (`xtask/src/release.rs`, Station 1 fragt Tag und Arbeitsbaum), ein roter Lauf kostet also nur einen zweiten |

**Empfehlung zu `260926-1011`: Das Ziehen ist ein Kopieren. Die Entscheidung `260926-0033` deckt es, und `HowTo.md` bekommt einen Satz dazu.**

Der Maßstab ist das Bedrohungsmodell aus `260926-0007_*_wie-wird-secrets-txt-verschluesselt-…`. Abzuwehren ist, dass ein Werkzeug den Inhalt **versehentlich** liest. Eine Datei, die der Nutzer selbst aus einer Markierung auf den Schreibtisch zieht, ist eine bewusste Ausfuhr. Sie gehört in dieselbe Klasse wie `cmd+c` und danach Einfügen und Sichern in TextEdit, und diesen Weg hat der Nutzer erlaubt. C7.8 („an keine Stelle auf der Platte“) ist eine Zusage über KRKs eigene Schreibwege und nicht über Dateien, die der Nutzer mit fremden Programmen anlegt. Ein Riegel wäre die Art Einzellösung, vor der der Datensatz zu den Zellen schon warnt. Er bräuchte ein Überschreiben der Ziehquelle an der Rohansicht, ein zweites am Feldeditor der Zellen und einen Riegel für die Dienste. Und er ließe den gleichwertigen Weg über die Zwischenablage offen.

Der eine Unterschied zur Zwischenablage gehört in den Satz der Anleitung: der Ausschnitt ist eine **Datei**, Spotlight indiziert sie, und sie bleibt, bis der Nutzer sie löscht. Vorschlag für `HowTo.md` nach dem Absatz an `:788`, sinngemäß: „Dasselbe gilt für markierten Text, den man aus dem Editor auf den Schreibtisch oder in ein Finder-Fenster zieht, und für den Menüpunkt ‚Dienste‘: daraus wird eine Datei mit Klartext, die Spotlight findet.“ Beschließen muss das der Nutzer als Ergänzung zu `260926-0033`. Der Defekt schließt dann mit dieser Antwort und dem Satz.

### 4. `CLAUDE.md`

**Beschrieben sind:** der Eintritt `zelle_uebernehmen` samt Grenze der Rufer-Probe (`CLAUDE.md:155`), `form_passt` und `Editorform` als Hälfte des dritten Zulässigkeitsbestandteils (`:157`, `:90`), die Zelle als eigene Textfläche über `laufende_zelle` (`:153`), die Kryptokisten ohne Vorgabemerkmale und festgenagelt (`:92`), die Aufgabentabelle als weitere Stelle, die `copy:` beantwortet (`:88`).

**Es fehlen Einträge unter „Was man nicht sieht“:**

1. **Der Schreibweg für `.secrets.txt` nimmt nur ein `Chiffrat`**, `datei::sichern` hat genau einen Rufer, und die Probe `kein_weg_schreibt_klartext_nach_secrets_txt` hält beides. Wer einen zweiten Sicherungsweg baut, etwa „Sichern unter“, muss es wissen.
2. **Jeder Weg, auf dem Editorinhalt KRK verlässt, fragt `Editormodell::haelt_geheimnisse`**, und keine Probe erzwingt das. Die zwei Defekte dieser Arbeit (Textmarke, Tastenprotokoll) sind genau diese Falle.
3. **Die Sperre sitzt in `Editormodell::oeffnen` und nicht bei einem Einstieg.** Jeder künftige Weg in den Editor erbt sie. Wer an ihr vorbei lädt, öffnet Geheimnisse als Text.
4. **Die Heimordner-Regel:** `Heimordner::ist` und `sonderdatei` vergleichen Pfadtext mit zwei Formen und stellen keinen Systemaufruf, weil sie auf dem Hauptfaden je Lesevorgang laufen (L1, L4, L6, L9). Ein `realpath` an dieser Stelle ist der Rückfall, den `260926-0115` abwehrt. Eine dritte Schreibweise wird nicht erkannt, mit der Folge aus Befund A.
5. **„Steht immer“ und der Ausschluss vom Inhaltsfilter stehen im Zweig der Verstecke von `zeilengrund_von`.** Die Probe `die_ausnahme_steht_im_zweig_der_verstecke` hält das, und der Plan macht einen Abnahmelauf gegen L3/L10 davon abhängig, dass sie grün bleibt.
6. **Der eigene Feldeditor der Zellen** (`Zelleneditor`, geliefert über `windowWillReturnFieldEditor:toObject:` in `appkit/fenster.rs`) mit seinem eigenen Rückgängigverwalter. Dazu die Warnung aus `260926-0840`, dass ein anderer Feldeditor bei leerem Stapel den Verlauf des Editors erreicht.
7. **Die Ablage führt die Zettel nicht mehr.** `note-1.txt` und `note-2.txt` bleiben im Ablageordner liegen und werden nicht mehr gelesen. Die Übernahme kennt ihre Pfade außerhalb von `Datei::ALLE`.
8. **Das Tastenprotokoll verdeckt** bei Geheimnissen. Wer mit `--tasten-protokoll` eine Frage zu Tasten in der Tabelle von `.secrets.txt` untersucht, sieht `(verdeckt)` und keinen Defekt.

Die Punkte 1 bis 3 wiegen am schwersten, weil jeder davon einmal Klartext auf die Platte gebracht hat oder bringen würde.

### 5. Die zehn Zeitzusagen

**Keine Zusage ist nach dem Code messbar verschoben, und der Plan schuldet keinen Abnahmelauf, solange drei Proben grün sind.** Die drei sind: der Start erreicht weder `bereitstellen` noch `aufgeloest_erneuern`, `ist` und `sonderdatei` stellen keinen Systemaufruf, und der Namensvergleich steht im Zweig der Verstecke (Plan `260926-0050_*_…`, „Where this work stops“). Alle drei sind im Lauf von heute grün.

| Zusage | Berührt? | Einordnung |
|---|---|---|
| L2, L3, L10 (Lesen und Sortieren) | kaum | `zeilengrund_von` (`verzeichnis/modell.rs:916-930`): ein gewöhnlicher Eintrag zahlt dieselbe eine Bool-Frage wie vorher, ein versteckter in einem fremden Ordner eine `Option`-Frage mehr, einen Namensvergleich zahlt nur `~/krkhome`. **Inference:** unter jeder Messauflösung |
| L4 (Start) | ja, klein | beim Start baut KRK den `Heimordner` mit `symlink_metadata` und `read_link` an einem Eintrag des Benutzerverzeichnisses (`heimordner/mod.rs:146-160`); die Ablage liest zwei Zettel weniger. L4 steht ohnehin seit dem 260910 auf der Liste der späteren Messrunde |
| L1, L9 (Tastendruck bis Zeichendurchgang) | ja, klein | je Anschlag kommen `laufende_zelle` in `ist_eigene_textflaeche` und `form_passt` in der Zulässigkeit dazu; `tasten_verdeckt` nur bei eingeschaltetem Protokoll. **Inference:** Mikrosekunden |
| L5, L6 (Tab, Unterordner) | kaum | ein Textvergleich je Lesevorgang in `Tabliste::lesen_starten` |
| L7 (Vorschau) | ja, klein | ein Textvergleich je Auswahl; die Tabellenform der Aufgaben rendert nur für die zwei Dateien. L7 steht ohnehin auf der späteren Liste |
| L8 (Kopie) | nein | — |

**Kandidat für einen Lauf des Nutzers ist L1**, und zwar nicht, weil sie verletzt wäre. L1 ist die einzige Zusage, die nach `CLAUDE.md` noch „ohne Vorbehalt“ hält, und diese Arbeit hat den Ereignisweg (`appkit/ereignisse.rs`, +163 Zeilen) und die Zulässigkeit (`kommandos/zulaessigkeit.rs`, +591 Zeilen) angefasst. Der Lauf misst alle zehn in einem Zug. **Ein voller Lauf nach dieser Auslieferung** nähme L1, L4 und L7 auf einmal von der Liste. Er ist nicht auslieferungsentscheidend; der Plan macht ihn ausdrücklich von den drei Proben abhängig, und die sind grün.

### 6. Nutzerprüfungen am laufenden Bündel

Vorher F1, `cmd+r`, Ansicht verlassen. Geordnet nach Risiko, zusammengeführt aus Spec, Plan, Zweitlesung Stufe 3 und den Defekten:

1. **Geheimnisse:** leere `.secrets.txt` mit F4 öffnen, PIN zweimal festlegen, Eintrag anlegen, `cmd+s`, `cat ~/krkhome/.secrets.txt` zeigt keinen Klartext; die Sicherung friert nicht ein.
2. Editor schließen, neu öffnen: falsche PIN ergibt „PIN falsch oder Datei verändert“, richtige PIN die Tabelle; mit entsperrter Datei beenden und starten: kein Blatt, Editor ohne die Datei.
3. PIN ändern (`shift+cmd+p`), neu starten: nur die neue PIN öffnet.
4. In einer Zelle von `.secrets.txt` `teh ` und `"x"` tippen, übernehmen, sichern, erneut öffnen: steht es unverändert da? (Befund B)
5. Content-Filter in `~/krkhome` nach einem Eintragstext, Verstecke aus und ein: kein Treffer auf `.secrets.txt`; `shift+cmd+h`: `.secrets.txt` bleibt, `.DS_Store` folgt.
6. `--tasten-protokoll` aus dem Terminal, PIN eingeben und in `.secrets.txt` tippen: nur `(verdeckt)`; `grep secrets ~/Library/Application\ Support/KRK/bookmarks.toml` ist leer.
7. Übernahme: mit Text in beiden alten Zetteln und ohne `~/krkhome` F2: beide als Notizen, `note-1.txt`/`note-2.txt` unverändert; `notes.txt` löschen, F2: leer, keine zweite Übernahme.
8. Eigene Belegung: Start ohne Abweisungsmeldung, F2 und `cmd+k` aus allen fünf Bereichen führen nach `~/krkhome`, zweimal F2 ergibt einen Tab, nach Neustart findet F2 ihn wieder.
9. `~/krkhome` als Datei: Meldung, kein Tab; als Verweis: Tab zeigt das Ziel, Fehlendes entsteht dort.
10. Zellen-Rückgängig (Zweitlesung Stufe 3): B hochschieben, B öffnen, `cmd+z` ohne Tippen ändert nichts; drei Zeichen, viermal `cmd+z`; `cmd+w` beim Tippen, dann `cmd+q`: Text gesichert oder Rückfrage.
11. Aufgaben: drei anlegen, mittlere hoch, sichern; Kästchen-Klick hakt ab und wieder auf; löschen und `cmd+z`; Rohansicht zeigt die Änderung vor dem Sichern; andere Datei: Befehle ausgegraut.
12. Notizen: mehrzeilig mit `return`, `cmd+return` übernimmt; `## x` im Text wird abgewiesen; zwei Absätze und `esc` übernehmen, `cmd+z` nimmt zurück.
13. Vorschau: `notes.txt` mit Themen, `tasks.txt` mit Kästchen, hell und dunkel; eine fremde Zeile erscheint als Text.
14. Ziehen aus der Rohansicht von `.secrets.txt` auf den Schreibtisch (`260926-1011`): entsteht ein Ausschnitt? Das Ergebnis geht in die Antwort auf den Defekt ein.
15. `cmd+z` im Umbenennungsfeld bei offenem, geändertem Editor (`260926-0840`): verliert der Editor sein Getipptes?

## Recommendations

**Vor der Auslieferung:**

1. **Den Abschlussvermerk in `260926-1010_*_…` auf `Resolved:` am Zeilenanfang stellen**, danach `make check` grün. Wer: der Autor des Datensatzes, also `code-implementer` oder der Orchestrator; eine Zeile.
2. **`Zelleneditor::neu` ruft `textautomatik::automatiken_abschalten`** (`eintragsansicht.rs:675-684`), dazu eine Probe nach dem Muster der Einordnungsprobe `EINSTELLUNGEN` in `appkit/editor.rs`. Wer: `code-implementer`. Das ist die vorhandene Regel an einem zweiten Rufer und kein neuer Mechanismus. Vorher Nutzerprüfung 4, damit die Behebung einen Anlass am Bündel hat.
3. **Befund A als Defekt ablegen und dem Nutzer eine Antwort vorlegen.** Meine Empfehlung ist **eine** Regel an der einen Schreibgrenze: Für eine Datei namens `.secrets.txt` fragt der Klartextzweig von `sichern_ueber` und der Leseauftrag `Leseauftrag::Text` zusätzlich, ob `canonicalize` des Elternordners der kanonischen Form des Heimordners gleicht. Weil das nur bei diesem einen Dateinamen und nur beim Öffnen und Sichern läuft, bleibt es aus dem Hauptfaden-Pfad je Lesevorgang heraus, den `260926-0115` schützt. Die Frage gehört als zweite, genaue Form in `Heimordner` selbst, damit die Erkennung an einer Stelle bleibt. Das ist eine Wahl an einer beantworteten Entscheidung und damit Sache des Nutzers. Ablegen: `code-implementer` oder `reviewer` als Defekt, der Datensatz zur Wahl über `analyst` (Typ 7).
4. **Die Frage zu `260926-1011` beantworten lassen** (Empfehlung oben: Kopieren, Satz in `HowTo.md`), dann den Satz schreiben. Wer: der Nutzer, dann `document-editor` oder `code-implementer` für `HowTo.md`.
5. **Die Versionszahl nach `README.md` ableiten: Major, also `2.0.0`.** Die Stufe greift an zwei Stellen. Die Bedeutung eines Tastenbefehls ändert sich: F2 und `cmd+k` öffnen den Notizordner statt des Notizblatts. Und `note-1.txt` und `note-2.txt` unter `~/Library/Application Support/KRK/` werden nicht mehr gelesen. Die Zahl nennt, wer die Auslieferung fährt, vor dem Lauf.
6. **`fusion-workbench/orchestrator-events.jsonl` eintragen oder zurücksetzen**, sonst bricht Station 1 am unsauberen Arbeitsbaum ab.

**Später:**

7. Die acht Einträge unter Frage 4 in `CLAUDE.md` nachziehen, die Punkte 1 bis 3 zuerst. Wer: `policy-curator` über `/fusion:curate` oder der nächste Schreiber von `CLAUDE.md`.
8. Proben nachtragen für C5.8 (`cmd+return` übernimmt), C5.9 (Rang der Zelle vor dem Filtertext in `abbrechen`) und „kein Blatt bei F2“ (C7.1).
9. Eine Klassenprobe für Seitenwege: eine Zählprobe über die Leser von `Editormodell::stand` und `schreibmarkenzeile` außerhalb des Editors, jeder mit Frage nach `haelt_geheimnisse`. Sie fände den dritten Fall der Art `260926-1004`/`-1010`, bevor er ausgeliefert ist.
10. `260926-0840`: das `Zelleneditor`-Muster auf die Namenszelle ausdehnen. `260926-0257`: das Feld streichen. `260926-1012`: die Probe unter Last wiederholen und, wenn die Vermutung hält, die Proben mit Kindprozess in eine eigene Prüfkiste legen oder das Recht mit kurzer Wiederholung nehmen.
11. Ein voller Abnahmelauf der zehn Zusagen durch den Nutzer nach der Auslieferung, mit L1 als Grund und L4 und L7 als Beifang.

## Open Questions

- [ ] Befund A: genügt dem Nutzer die zweite, genaue Form nur an der Schreib- und Lesegrenze, oder soll `.secrets.txt` gar nicht über den Klartextweg gehen, gleich in welchem Ordner? Das hieße, auch fremde Dateien dieses Namens nicht mehr als Text zu bearbeiten.
- [ ] `260926-1011`: bestätigt der Nutzer, dass Ziehen und Dienste unter `260926-0033` fallen?
- [ ] Soll die Schließungsnotiz des Arbeitspakets je Stufe sagen, dass die Abnahme am Bündel nicht gefahren ist, wie „Where this work stops“ es verlangt, falls der Nutzer vor dem Abschluss nicht abnimmt?

## Sources

- Spec `260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`, Plan `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md` (Abschnitte Stufe 3 und „Where this work stops“)
- Datensätze `260926-0115_*_erkennt-krk-den-heimordner-…md`, `260926-0033_*_…`; Defekte `260926-0257_*_…`, `260926-0840_*_…`, `260926-1011_*_…`, `260926-1012_*_…`, `260926-1010_*_…`, `260926-1004_*_…`
- Code: `crates/krk-ui/src/editormodell.rs`, `appkit/editor.rs`, `appkit/eintragsansicht.rs`, `appkit/textautomatik.rs`, `appkit/anwendung.rs`, `appkit/ereignisse.rs`, `crates/krk-core/src/heimordner/mod.rs`, `verzeichnis/modell.rs`, `text/datei.rs`, `xtask/src/werkbank.rs`, `crates/krk-bench/src/messen.rs`, `crates/krk-core/tests/ablage.rs`
- `README.md` `### Versionsstufen`, `HowTo.md:690-800`, `CLAUDE.md`
- `make check` auf `e22c255`, `git log d5fb749..HEAD`, `git tag`
- Frühere Zweitlesungen `260926-0017-zweitlesung-spec-f2-krkhome.md`, `260926-0107-zweitlesung-plan-f2-krkhome.md`, `260926-0811-zweitlesung-stufe-3-aufgabeneditor.md`
