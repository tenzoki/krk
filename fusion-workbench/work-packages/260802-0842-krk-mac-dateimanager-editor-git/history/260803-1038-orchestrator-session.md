# Orchestrator Session — 260803-1038

**Directive:** KRK, native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien über die Tastatur. Erste Runde: lauffähiges Navigator-Gerüst.
**Mode:** (Phase 0 noch offen — Fortsetzung der unterbrochenen Sitzung 260802-1014)
**Status:** In Arbeit

## Aufnahme bei Sitzungsbeginn

**Arbeitsplatz:** `/Users/k1/Projects/productive/krk/fusion-workbench`
**Aktiver Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git` (Marker `_t_`, ein aktiver Circle, keine anticipated)
**Git HEAD:** `def6fa7`

| Größe | Stand |
|---|---|
| Offene/laufende Defekte | 3 (alle im Circle, keine im geteilten Speicher) |
| Offene Planschritte | Plan Runde 1: Schritte 1 bis 5 einschließlich 4b als `[DONE]`, ab Schritt 6 offen; Spec offen |
| Entscheidungen offen (`_o_`) | 5 (2 im Circle, 3 geteilt) |
| Entscheidungen beantwortet (`_a_`) | 6 (4 im Circle, 2 geteilt) |
| Analysen | 1 (Sprache und UI-Werkzeugkasten) |
| Prüfberichte | 2 (beide conceptrev) |
| Commits gesamt | 17 |
| Guard | kein Halt, 1 aufeinanderfolgende Blockade verzeichnet |

### Domänenerkennung

Die Heuristik aus Setup-Schritt 5 liefert **`strategic`**, und das ist hier falsch. Die
Eingangswerte: 17 Commits auf `fusion-workbench/`, 1 Analyse, 3 offene Defekte, 5 offene
Entscheidungen, 16 Rust-Quelldateien, 0 Datendateien. Der erste Zweig greift, weil die Zahl
der offenen Entscheidungen die der offenen Defekte erreicht — er prüft aber nicht, ob
Quellcode vorliegt, und hier liegt ein Cargo-Workspace mit vier Kisten und laufenden Tests.
Die Sitzung arbeitet deshalb mit **`code`**, übereinstimmend mit der vorangegangenen Sitzung
260802-1014 und mit dem Dateibestand.

### Unterbrochene Sitzung

`agentstate.yaml` vom 260802-1800 lag vor und war überholt: er führte Schritt 2 als laufend
und 11 Commits, während im Repository die Schritte 1 bis 5 einschließlich 4b abgeschlossen
und committet sind und 17 Commits stehen. Der Nutzer hat "Fortsetzen, Stand neu erheben"
gewählt. Die Warteschlange wird aus dem Plan und dem Dateibestand neu aufgebaut, nicht aus
der Datei geladen.

### Unfertige Arbeit im Arbeitsverzeichnis

`xtask/src/sign.rs` (+216 Zeilen) und `README.md` (+72) sind geändert und nicht committet.
Sie gehören zum offenen Defekt
`issues/260802-2050_o_signaturidentitaet-wird-nur-unter-einem-festen-namen-gefunden.md`.
Der zugehörige Bericht `history/260802-2253-signaturidentitaet-eindeutige-lage-und-zertifikatskette.md`
trägt Status "In Arbeit" und ist nicht versioniert.

### Beobachtung zu CLAUDE.md

`CLAUDE.md` ist überholt. Der Abschnitt "Projektstand" sagt "Es gibt weiterhin keinen
Quellcode und keine Architektur" und "kein Build-Kommando und kein Testkommando"; beides
stimmt seit Schritt 1 nicht mehr. Auch die Technologiewahl steht dort als offen, obwohl
`decisions/260802-1134_a_sprache-und-ui-werkzeugkasten.md` sie beantwortet. Nachzuziehen.

## Verlauf

### Turn 1 — 260803-1042 bis 260803-1335

Sieben von acht Aufgaben abgeschlossen, sieben Commits von `def6fa7` bis `3e14b38`. Die
achte, die Frühmessung aus Schritt 8, hängt an einer Handlung des Nutzers und nicht an
Code.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T1 | orchestrator | `4884f85` | Die unfertige Signaturarbeit geprüft und committet, Defekt `260802-2050` geschlossen |
| T2 | planner | `3877dbc` | Frage 7 auf den umgesetzten Stand gezogen, die unhaltbare `unsafe`-Prüfvorschrift in den Schritten 2 und 15 ersetzt |
| T3 | coder | `e43316d` | Hilfetext von `cargo xtask` beschreibt die Identitätssuche in drei Stufen |
| T3b | planner | `b427c74` | Nutzerentscheid zur `unsafe`-Grenze festgehalten, Plan an fünf Stellen nachgezogen |
| T4 | coder | `569e8e0` | Schritt 6: Fenster, Menü, echte Dateiliste, vier `define_class!`-Deklarationen |
| T5 | coder | `6b4fb2d` | Schritt 7: Ereignisabgriff, Modifikator-Normalisierung in `krk-core`, Protokollmodus |
| T7 | coder | `3e14b38` | `CLAUDE.md` auf den Projektstand, zwei Entscheidungsmarker nachgezogen |

**Der Nutzerentscheid dieses Turns.** Beim Korrigieren der beiden Plandefekte fand der
planner denselben Fehler an einem dritten Ort, im Abnahmekriterium von Schritt 6. Die
Auflösung konnte dort nicht dieselbe sein: `krk-core` trägt `#![deny(unsafe_code)]` und
lässt den Bau scheitern, `krk-ui` trug `#![warn(unsafe_code)]` und meldete nur. Der Nutzer
hat auf `deny` entschieden. Der Plan hatte die `warn`-Wahl an keiner Stelle begründet,
während er die Wahl von `deny` gegen `forbid` für `krk-core` ausführlich herleitet; die
Begründung steht jetzt in `## Aufbau` und deckt beide Kisten ab. Datensatz:
`decisions/260803-1208_i_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`.

**Was blockiert.** Auf dem Gerät steht ein Schlüsselbund-Dialog von macOS. `codesign`
wartet auf die Freigabe des privaten Schlüssels und braucht einen Klick auf "Immer
erlauben". Ein Hintergrundlauf von `cargo xtask bundle` steht seit 260803-1315 in diesem
Zustand, `target/KRK.app` ist unsigniert. Betroffen sind die Messung aus Schritt 8 und drei
Abnahmepunkte aus Schritt 7: Pfeiltasten und Bildtasten am laufenden Bündel sowie das
Tastenprotokoll für die Codes 99, 96 und 100.

**Defekte.** Sechs geschlossen (`260802-1810`, `260802-1935`, `260802-2050`, `260803-1042`,
`260803-1200`, `260803-1309` zum Entscheidungsmarker), drei neu und offen, alle drei aus
Schritt 7 und alle drei über die Plandatei selbst: die Dateiliste von Schritt 7 nennt fünf
nötige Dateien nicht, das Abnahmekommando filtert nach Prüfungsnamen statt nach Datei, und
das Tastenprotokoll ist über `open` nicht lesbar.

**Ein gemeldeter Befund trägt nicht.** Der coder von T7 hielt die leere Zeile `Implemented:`
in mehreren Entscheidungsdatensätzen für einen Fehler. Sie gehört zur Vorlage aus
`rules/fusion-workbench-conventions.md` und ist bei einem beantworteten Datensatz richtig;
sie wird erst beim Übergang auf umgesetzt gefüllt. Kein Defekt angelegt. Die beiden anderen
Befunde desselben Berichts trugen: fünf Verweise auf den alten `_a_`-Pfad im Plan und ein
Kopf, der `answered` sagte, während der Dateiname `_i_` trug. Beide nachgezogen.

### Turn 2 — 260803-1355 bis 260803-1850

Fünf Aufgaben, fünf Commits. **Phase A ist abgeschlossen und das Messgate bestanden.**

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T9 | planner | `69b016d` | AppKit-Grenze in sechs Dateilisten nachgezogen |
| T8 | coderev | `47db78d` | Code-Prüfung des AppKit-Durchstichs, acht Defekte |
| T6 | coder | `7855f6d` | Schritt 8, Frühmessung: Gate zunächst an L1 verfehlt |
| T10 | planner | `9c3d7e0` | Abnahmemaß für L1 und L9 umgestellt |
| T11 | coder | `9e0cb7a` | Auswertung umgestellt, nachgemessen, Gate bestanden |

**Das Gate, und was es tatsächlich zutage förderte.** Der erste Lauf verfehlte L1: das
95. Perzentil lag in einer von fünf Runden bei 16,225 ms gegen zugesagte 16 ms. Die vier
übrigen Zusagen hielten mit Faktor zwei bis drei Abstand, das vollständige Lesen von
100.000 Einträgen in 0,98 s gegen zugesagte 4 s.

Der Plan sieht für diesen Fall vor, dass der Technologieentscheid zur Debatte steht. Die
Messung trug den Verdacht nicht. Der Spec leitet L1 als "ein Bild bei 60 Hz" her, und ein
Bild sind 16,667 ms; die Zusagentabelle rundet auf 16 ab. Der größte gemessene Einzelwert
lag bei 16,590 ms, also unter einem Bild. Die gemessene Spanne besteht überwiegend aus
Warten auf die Bildgrenze, KRKs eigener Anteil bei 3 bis 8 ms; selbst eine Anwendung ohne
jede Verarbeitungszeit erreichte hier ein 95. Perzentil von rund 15,8 ms. Die Zusage lag
innerhalb der Streuung ihres eigenen Messverfahrens.

**Der Nutzerentscheid, 260803-1810.** Gewählt ist das geänderte Abnahmemaß: nicht mehr das
95. Perzentil der Zeitspanne, sondern der Anteil der Eingaben, die ihr nächstes Bild
erreichen, mindestens 95 Prozent je Runde. Der Technologieentscheid bleibt unangetastet.
Seine Begründung geht über die der Vorlage hinaus und ist deshalb getrennt festgehalten:
die Vorlage argumentiert messtechnisch, der Nutzer wahrnehmungsseitig, eine Spanne dieser
Größe sei für einen Menschen nicht unterscheidbar. Datensatz:
`decisions/260803-1755_i_l1-verfehlt-die-16-ms-zusage-am-bildrand.md`.

**Die Nachmessung zeigt, dass die Umstellung nicht kosmetisch war.** L1 hält mit 100 von
100 Tastendrücken, in jeder Runde 20 von 20. Dieselben Rohdaten ergeben ein 95. Perzentil
zwischen 14,912 und 16,633 ms, das die alte Zusage in vier von fünf Runden verfehlt hätte.
Zwei Maße, ein Datensatz, entgegengesetzte Urteile.

**Zwei Befunde, die eine Annahme widerlegten, darunter meine eigene.** Der planner hat
nachgeprüft, dass `NSScreen.maximumFramesPerSecond` und `NSWindow.screen` sichere
Funktionen sind und außerhalb von `appkit/` anstandslos übersetzt hätten. Von den sechs
gefundenen Grenzverstößen hätten nur drei den Bau abgebrochen; `#![deny(unsafe_code)]`
trägt die Grenze zur Hälfte. Der Defekt zu Schritt 8 war sichtbar, weil `CADisplayLink`
zufällig auf der unsicheren Seite liegt. Betroffen waren sechs Schritte statt der von mir
im Defekt vermuteten vier, und Schritt 15 hatte für den Papierkorb-Aufruf überhaupt keine
Datei. Der Nutzer hat entschieden, die ergänzende Prüfung nicht nachträglich in das
abgenommene Abnahmekriterium von Schritt 6 einzutragen; Defekt `260803-1530` bleibt offen.

**Die Code-Prüfung fällt gut aus.** Hauptfadenregel, Eigentumsverhältnisse und die Inhalte
aller vierzehn `unsafe`-Stellen sind sauber, kein erreichbarer `RefCell`-Doppelzugriff. Von
acht Befunden betrifft einer das Verhalten: die Auswahl übersteht das Sortieren am Ende
eines Lesevorgangs nicht, weil sie nur als Zeilennummer in der Tabelle steht, während
`Ordnermodell::eintragsindex` und `zeile_von` genau dafür existieren und von niemandem
gerufen werden.

**Offen am Ende der Sitzung.** Vierzehn Defekte, davon acht aus der Code-Prüfung und zwei,
die auf eine Nutzerentscheidung warten: die halb erzwungene AppKit-Grenze (`260803-1530`)
und die Streuung von L4 zwischen den Runden (`260803-1845`, 282 bis 715 ms gegen zugesagte
1000 ms, Ursache nicht belegbar, weil der Bericht die Systemlast nicht erhebt). Die
Dateilisten von S9 bis S23 sind noch nicht unter der erweiterten Grenzregel durchgegangen
(`260803-1819`). Fünf Entscheidungen sind offen, drei umgesetzt.

**Nächster Schritt:** S9, die Auslieferungsbelegung als Datentabelle, Ausführender
`ontocoder`. Neun der vierundzwanzig Planschritte tragen `[DONE]`.

### Turn 3 — 260803-1900 bis 260803-2100

Vier Aufgaben, vier Commits. Der Nutzer hat mit "weiter" die Reihenfolge freigegeben; ich
habe die Schuld aus der Code-Prüfung vor die neuen Planschritte gezogen, weil der Code
frisch war und Phase C darauf aufbaut.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T13 | coder | `62ce25f` | Auswahl-Fehler und sechs Belegketten-Defekte |
| T12 | planner | `1ec64bd` | Dateilisten S9 bis S23 durchgegangen, vier Plandefekte |
| T14 | ontocoder | `d1a8ab1` | S9, Auslieferungsbelegung als Datentabelle |
| T15 | coder | `0d8e87e` | S10, Ablage unter Application Support |

**Der Auswahl-Fehler ist anders gelöst als der Defekt vorschlug.** Der Datensatz wollte die
Auswahl in `QuelleIvars` sichern; der `coder` hat sie ins `Ordnermodell` gelegt und an den
Eintragsindex gehängt. Begründung, die trägt: umsortiert wird in `sicht_neu_aufbauen`, und
dorthin führen drei öffentliche Wege, die in der Oberfläche je eine eigene Sicherung
gebraucht hätten. Nachgewiesen hat er die Reparatur, indem er `auswahl_zeile` versuchsweise
auf den alten Stand zurücksetzte und die Prüfung fehlschlagen sah.

**Die Durchsicht der fünfzehn Dateilisten war größer als vermutet.** Zwölf hatten einen
Fund, drei standen sauber. Der Defekt hatte fünf Schritte als wahrscheinlichste Treffer
genannt; sieben weitere kamen dazu, und S15 hatte für den Papierkorb-Aufruf überhaupt keine
Datei. Zwei wiederkehrende Formen stehen jetzt als Merksatz im Kopf der
Implementierungsschritte.

### Turn 4 — 260803-2110 bis 260803-2323

Zwei Aufgaben, zwei Commits. **Phase B ist abgeschlossen, zwölf der vierundzwanzig Schritte
tragen `[DONE]`.**

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T16 | planner | `8bd7b71` | Belegung angenommen, Kürzelschreibweise und F6 nachgezogen |
| T17 | coder | `73b4d88` | S11, Belegungsmaschine |

**Der Nutzerentscheid dieses Turns.** Ich habe ihm die vollständige Auslieferungsbelegung
vorgelegt, weil der Spec nur sieben der 46 Belegungen selbst festlegt und die übrigen 39 der
`ontocoder` gewählt hatte. Er hat sie angenommen ("passt erstmal so") und bestätigt, dass F6
verschiebt und `shift+f6` umbenennt. Die Annahme steht als Entscheidungsdatensatz
`decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`,
nicht als Vermerk im Spec: ein Vermerk hätte die Datendatei verdoppelt, und das "erstmal"
braucht eine Zustandsspur, die ein Spec-Absatz nicht tragen kann.

**Die Ablösung in S11 ist echt und nachgeprüft.** `grep -rn 'VERDRAHTET' crates/` findet
nichts mehr, und außerhalb von `parser.rs` trägt keine Konstante einen Tastencode als Zahl:
`code_von` ist eine `const fn`, sodass auch `ereignisse.rs` seine Zahl von dort holt und ein
Tippfehler den Bau abbricht.

**Eine Regel hat der `coder` selbst ergänzt, und sie war nötig.** Die Belegung kennt 46
Funktionen, gebaut sind fünf. Ohne die Regel "geschluckt wird nur, was auch ausgeführt
wurde" hätte der Ereignisabgriff ab sofort Cmd+W abgefangen und nichts damit getan, womit
der Menüeintrag "Fenster schließen" aus S6 tot gewesen wäre.

**Offen am Ende von Turn 4.** Vierzehn Defekte und sieben Entscheidungen. Zwei davon gaten
S12: was nach dem Schließen des letzten Fensters geschieht (`260803-2007`) und wie KRK dem
Nutzer Fehler zeigt (`260803-2025`, betrifft die Ablage aus S10, deren Ausgabeweg deshalb
an einer Stelle gebündelt ist). Neu und noch nicht vorgelegt: `cmd+y` für die Vorschau
liegt auf einer deutschen Tastatur unter der Taste Z, weil KRK die Stelle belegt und nicht
das Zeichen (`260803-2317`).

**Nächster Schritt:** S12, vier Bereiche, Tabs, aktives Fenster und Sichtbarkeit. Der Plan
nennt ihn den teuersten Einzelposten, und er ist der erste Schritt hinter dem bestandenen
Messgate.

### Turn 5 — 260803-2330 bis 260804-0937

Vier Aufgaben, vier Commits. Der Turn hat den Umfang der Runde erweitert und einen zweiten
Circle ins Portfolio gebracht.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T18 | planner | `e435ca9` | C10, Statuszeile, Weg zurück zum Fenster |
| T19 | ontocoder | `bc95183` | S9b, drei Kombinationen nachgetragen |
| T20 | coder | `bc95183` | Zählprüfungen ohne Literale |
| T21 | shaper | `82c8ea6` | Anticipated Circle für einen eingebauten Web-Betrachter |

**Der Nutzer hat zwei Funktionen nachbeauftragt**, und sie sind eine Erweiterung des
Umfangs und keine Präzisierung: die Zwischenablage ansehen (`shift+f3`) und zu ihrem Inhalt
springen (`opt+cmd+g`). Sie stehen als eigene Fähigkeit C10 im Spec, nicht als Erweiterung
von C6 und C2, weil beide an derselben Auswertung dessen hängen, was in der Zwischenablage
steht; diese Auswertung an zwei Stellen zu beschreiben hieße, zwei Wahrheiten darüber zu
führen. Im Plan sind sie an S13 und S19 verankert, wo die nötigen Bauteile entstehen, statt
danebengestellt. Der Plan ist dabei von 24 auf 26 Schritte gewachsen.

**Drei Entscheidungen des Nutzers am selben Abend.** Das letzte Fenster bekommt einen Weg
zurück; Fehler zeigt eine Statuszeile, mit Abbruch allein beim fehlenden Tastenabgriff;
`cmd+y` bleibt, obwohl es auf einer deutschen Tastatur unter der Taste Z liegt, weil F3 die
Vorschau trägt und die Belegung ab Werk änderbar ist. Am nächsten Morgen drei weitere: die
Zwischenablage-Auswertung liest Text und Dateiverweis, der Menüeintrag heißt "Fenster
einblenden" statt "Neues Fenster", und der eigene Browser bekommt einen anticipated Circle.

**Ein Fehler von mir, vom `planner` gefunden.** Ich hatte ihm mitgegeben, C1 verlange die
Statuszeile ohnehin. Der Spec nannte sie nirgends; die Behauptung stammte aus einem
Defektdatensatz, war von dort in den Entscheidungsdatensatz gewandert und von mir ungeprüft
weitergereicht. Sie steht jetzt als eigenes Abnahmekriterium in C1, mit Vermerk woher sie
kommt. Ein zweiter Fehler von mir, vom `ontocoder` gefunden: der Zwischenablage-Datensatz
trug nach meiner Beantwortung zwei `Answered`-Blöcke und im Kopf noch `Status: open`. Beides
nachgezogen.

**Der Nachtrag in S9b hat drei fest verdrahtete Zahlen im Rust-Code brechen lassen**, und
der naheliegende Fix wäre der falsche gewesen. Die Zahlen auf 49 und 55 zu heben hätte
dieselbe Falle für den nächsten Nachtrag wieder aufgestellt. Der `coder` hat die Prüfungen
stattdessen zahlfrei formuliert und nachgewiesen, dass sie den nächsten Nachtrag überleben:
probeweise ein vierter Eintrag angehängt, beide Prüfungen grün; danach derselbe Block mit
einer doppelten Kombination, und die neue Prüfung fällt mit ihrer vorgesehenen Meldung.
Beide Proben zurückgenommen, die Datei steht wieder auf ihrem Hash.

**Der zweite Circle.** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` steht als
geplant (`_a_`) im Portfolio. Er erweitert die Grenze des aktiven Circles und ist keine
spätere Runde von ihm: eine Runde setzt eine vorhandene Zusage um, dieser holt einen
ausdrücklichen Ausschluss herein. Der `shaper` hat zwei Spannungen in den Antworten des
Nutzers aufgelöst statt sie zu übergehen, eine davon vollständig (Verlauf gegen Zurück und
Vor) und eine zur Hälfte, mit dem Rest als offener Frage im Circle.

**Stand am Ende der Sitzung.** Dreizehn der sechsundzwanzig Planschritte tragen `[DONE]`,
170 Prüfungen laufen grün, `clippy` meldet nichts, und die `unsafe`-Grenze hält in beiden
Kisten mit je genau einer Ausnahme. Achtzehn Defekte sind offen, überwiegend Plantext; fünf
Entscheidungen offen, acht beantwortet, vier umgesetzt.

**Nächster Schritt bleibt S12**, jetzt größer als zu Beginn der Sitzung: er trägt zusätzlich
die Statuszeile und den Weg zurück zum Fenster.

### Turn 6 — 260804-0938 bis 260804-1046

Eine Aufgabe, ein Commit. S12 ist der größte Einzelschritt der Runde gewesen.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T22 | coder | `537fda5` | S12, vier Bereiche, Tabs, Statuszeile und Rückweg zum Fenster |

Der Schritt hat aus einem Fenster mit einer Liste ein Fenster mit einem Aufbau gemacht:
Leiste, zwei Dateifenster mit je eigenen Tabs, Vorschaubereich, Statuszeile. Die Statuszeile
und der Weg zurück zum letzten Fenster sind erst in Turn 5 dazugekommen und in demselben
Schritt mitgebaut worden, statt einen eigenen zu bekommen.

### Turn 7 — 260804-1046 bis 260804-1316

Vier Aufgaben, vier Commits. Der Turn hat eine Belegungsfrage des Nutzers beantwortet und
dabei die Schreibweise erweitern müssen.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T23 | planner | `734f829` | Acht Tastennamen in der Schreibweise, Ordnernavigation neu belegt |
| T24 | ontocoder | `2c95f20` | Commit-Hash im Zwischenablage-Entscheid nachgetragen |
| T25 | ontocoder | `203f606` | S11b, die acht Tastennamen im Parser |
| T26 | coder | `06dc48b` | S11c und S13, Ordnernavigation auf den Pfeilen, C2 fertig |

**Der Nutzer hat gefragt, wie er einen Ordner nach oben kommt**, und die ehrliche Antwort
war: gar nicht. Er hat daraufhin `cmd+links` und `cmd+rechts` bestellt und F1 für die
Belegungsansicht. Die Schreibweise kannte die Pfeiltasten und die Funktionstasten bis dahin
nicht; sie hat acht Namen dazubekommen, bevor die Belegung sie nennen konnte.

### Turn 8 — 260804-1316 bis 260804-1457

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T27 | coder | `8b29a80` | S14, Dateisystem-Beobachtung und Datenträgerwechsel |

FSEvents hängt an `FSEventStreamSetDispatchQueue` und nicht am veralteten
`…ScheduleWithRunLoop`. Die Bindung ist von Hand geschrieben und liegt hinter der
`unsafe`-Grenze in `krk-core`.

### Turn 9 — 260804-1457 bis 260804-1654

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T28 | coder | `daecb45` | S15, Operationsmaschine, der Kern von C4 |

Kopieren über `copyfile(3)`, Umbenennen über `renamex_np(2)` und nicht über `rename(2)`,
weil nur die erste Form `RENAME_EXCL` kennt und damit ein vorhandenes Ziel nicht still
überschreibt.

### Turn 10 — 260804-1654 bis 260804-1818

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T29 | coder | `343a7f3` | S16, Fortschritt, Abbruch, Konflikt und Rückfrage |

### Turn 11 — 260804-1818 bis 260804-1945

Drei Aufgaben, drei Commits. Der Turn hat eine gebaute Lösung wieder abgeräumt.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T30 | planner | `6ed0ed1` | Fortschritt in die Statuszeile, S16b und S17b angelegt |
| T31 | coder | `5a2f05d` | S16b, Fortschritt in der Statuszeile statt im Blatt |
| T32 | coder | `c89ea66` | Vier Ränge in der Statuszeile, S16b abgenommen |

**Das Fortschrittsblatt aus S16 war die falsche Bauform**, und zwar aus zwei Gründen, die
erst die Messung gezeigt hat: ein Blatt sperrt genau das Fenster, das C4 benutzbar zusagt,
und es braucht 354 bis 403 ms zum Anhängen, wo L8 zweihundert verspricht. Der Nutzer hat
entschieden, den Fortschritt in die Statuszeile zu legen. Dort gemessen: 168,9 ms p95. Die
Statuszeile hat dafür Ränge bekommen, mit der Regel, dass Verdrängtes nicht gelöscht wird;
aus vier Rängen sind später in S18 fünf geworden.

### Turn 12 — 260804-1945 bis 260804-2047

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T33 | coder | `91b904e` | S17, Stapel-Umbenennen, Anlegen und die Namenseingabe |

### Turn 13 — 260804-2047 bis 260804-2336

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T34 | planner | `a70baeb` | Aufräumdurchgang, 36 offene Defekte auf 21 |

**Zwei Wurzeln statt zwanzig Symptome.** Erstens trug der Plankopf zwei einander
widersprechende Zusagen darüber, was eine Dateiliste bedeutet; aufgelöst zu "Leseliste mit
Begründungen, bindend ist die Verbotsseite". Zweitens trugen Verweise auf Datensätze deren
Zustandsmarker im Pfad, sodass jeder Zustandswechsel sie brach; 228 Verweise stehen jetzt in
Globform `_*_`, und alle 92 verschiedenen davon sind nachweislich auflösbar.

### Turn 14 — 260804-2336 bis 260805-0025

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T35 | planner | `395e475` | Sieben Nutzerantworten eingearbeitet, drei neue Schritte |

**Der Nutzer hat die Zeitzusagen ausdrücklich zur Disposition gestellt**: "falls die engen
Zeitvorgaben Probleme machen: aufweichen, pragmatische Lösungen planen". Der `planner` hat
acht Fragen vorgelegt, alle sieben Antworten folgten der Empfehlung. Der Plan ist von 33 auf
36 Schritte gewachsen.

### Turn 15 — 260805-0025 bis 260805-0800

Drei Aufgaben, zwei Commits.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T36 | ontocoder | `db1b559` | Die Konflikterkennung lernt den Zusteller |
| T37 | planner | `db1b559` | S13b zur Hälfte, Menü "Bearbeiten" |
| T38 | coder | `58465bf` | S13b und S13c, Menü "Bearbeiten" und die Zustellerregel |

**`cmd+a` hat zwei Funktionen getragen**, und der Plan behauptete, `alle_markieren` liege
auf `ctrl+a`. `git log -L` über die Belegungsdatei hat gezeigt: es lag nie dort. Die
Auflösung ist keine Umbelegung, sondern eine Regel: zwei Funktionen streiten nur dann, wenn
sie dieselbe Kombination **und** denselben Zusteller haben. Was das Menü zustellt, trägt seit
S13c `gehalten_von = "menue"`.

### Turn 16 — 260805-0800 bis 260805-1004

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T39 | ontocoder | `7e33345` | Datensätze der geschlossenen Defekte nachgezogen |
| T40 | planner | `7e33345` | Aufräumdurchgang über den Restbestand |
| T41 | coder | `7e33345` | Vierzehn Defekte geschlossen, S13c abgenommen |

### Turn 17 — 260805-1004 bis 260805-1350

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T42 | coder | `3c7191a` | S16c, S17b und S17c, die Markierung und das Umbenennen |

### Turn 18 — 260805-1350 bis 260805-1539

Drei Aufgaben, drei Commits. Zwei davon kommen aus Wünschen des Nutzers und nicht aus dem
Plan.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T43 | ontocoder | `13f9463` | Ordnernavigation auf die nackten Pfeiltasten |
| T44 | coder | `6089ee3` | Makefile als Hülle um die vorhandenen Kommandos |
| T45 | coder | `3b379c0` | Ziel "frisch" für den Bau von Grund auf |

**Der Nutzer hat die Belegung aus Turn 7 zurückgenommen**: nicht `cmd+links` und
`cmd+rechts`, sondern die nackten Pfeiltasten, links heraus und rechts hinein. `cmd+hoch`
bleibt als zweite Form stehen.

**Das Makefile ist keine zweite Bauform**, sondern eine Hülle: cargo liegt auf diesem Gerät
nicht auf dem Standard-PATH, und jedes Kommando brauchte bis dahin ein vorangestelltes
Export. Eine Falle steckt darin, und sie hat einmal zugeschlagen: GNU make 3.81, die Fassung
die macOS mitbringt, führt ein Rezept aus genau einem Wort direkt aus und schlägt das
Programm dabei in seinem eigenen PATH nach, nicht in dem, den das Makefile exportiert.
`CARGO` steht deshalb absolut.

**Der erste Lauf von `make menue` hat sofort einen Defekt gefunden**: macOS stellt dem Menü
"Bearbeiten" ein AutoFill-Untermenü dazu, das `menue.rs` nicht anlegt. Es ist der vierte
Systemzusatz dieser Art und der harmloseste, weil er keine Tastenkombination trägt.

### Turn 19 — 260805-1539 bis 260805-1735

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T47 | planner | `10642fb` | C11 geplant, den angezeigten Ordner im Terminal öffnen |
| T48 | coder | `7a0c0a6` | S18, Lesezeichen- und Geräteleiste, und der Wirkungsbereich |

**Der Nutzer hat eine Taste bestellt, die das Terminal im angezeigten Ordner öffnet.** Sie
ist als eigene Fähigkeit C11 in den Spec gegangen, weil sie eine Einstellung braucht: welche
Anwendung geöffnet wird, steht ab Werk auf `com.apple.Terminal` und ist änderbar.

**Der Wirkungsbereich aus S18** ist die Antwort darauf, dass es jetzt mehr als einen Ort
gibt, an dem der Fokus liegen kann. Jedes `Kommando` trägt `Dateifenster`, `Leiste` oder
`Ueberall`; die Zuteilung ist eine vollständige Fallunterscheidung ohne Auffangzweig, sodass
ein neues Kommando ohne eigene Zeile nicht übersetzt.

### Turn 20 — 260805-1735 bis 260805-1906

Drei Aufgaben, drei Commits. Der letzte hat einen Fehler behoben, der die Anwendung fast
unbedienbar gemacht hätte.

| Aufgabe | Ausführender | Commit | Inhalt |
|---|---|---|---|
| T49 | ontocoder | `f850f30` | S18b, `ctrl+o` und die Auslieferungseinstellungen |
| T50 | coder | `48e69df` | S18c, das Terminal im angezeigten Ordner (C11) |
| T51 | coder | `63cade1` | Der Eingabefokus liegt beim Start im Dateifenster |

**Seit S18 lag der Eingabefokus beim Start in der Leiste**, und bis zum ersten `shift+cmd+d`
wirkte kein Dateifenster-Befehl. Behoben, indem der Fokus als letzte Zeile von
`oberflaeche_aufbauen` gesetzt wird, nach `makeKeyAndOrderFront`, und am Bündel ohne
vorherigen Tastendruck abgenommen.

**Ein Vorfall beim Prüfen, der nicht wieder passieren darf.** Beim Abnehmen von S18 sind
synthetische Tastendrücke über `osascript` in ein fremdes Fenster gelaufen — eine
Claude-Code-Sitzung des Nutzers in Ghostty —, weil KRK den Vordergrund verloren hatte.
Synthetische Tastendrücke gehören seitdem in KRKs eigene Ereignisschlange über
`postEvent:atStart:`, nie über `osascript`.

---

## Stand bei der Unterbrechung — 260805-2003

Die Sitzung ist an einem sauberen Punkt unterbrochen worden, damit der Nutzer die Umgebung
neu starten kann. Jede begonnene Aufgabe ist abgeschlossen und committet; das
Arbeitsverzeichnis trägt außer dem flüchtigen Sitzungszustand nichts. HEAD steht auf
`63cade1`, 51 Commits seit `def6fa7`.

**Dreißig der sechsunddreißig Planschritte tragen `[DONE]`.** Alle Prüfungen laufen grün,
`clippy` meldet nichts, die `unsafe`-Grenze hält mit je einer Ausnahme in
`krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`, das Bündel ist gebaut und
signiert.

**Es fehlen fünf Schritte:** S19 Vorschaufenster mit eigenen Tabs, S20 Belegungsansicht, und
die drei Schritte der Phase F — S21 Messmodus, S22 Abnahme gegen die Prüfsitzung, S23
Auslieferungspaket. Nächster Schritt ist S19; er trägt zugleich die erste Funktion aus C10
und den offenen Defekt zur Metadatenvorschau.

**Offen sind dreizehn Defekte und sieben Entscheidungen.** Kein Defekt blockiert; zwei sind
an spätere Schritte gebunden (Metadatenvorschau an S19, L4-Streuung an S22). Von den sieben
Entscheidungen hat der Nutzer zwei noch nicht gesehen, beide aus dieser Sitzung und beide mit
Empfehlung: ob der Fokusbefehl eine ausgeblendete Leiste hervorholt, und wann eine von Hand
geänderte `settings.toml` wirkt.

**Ungeprüft geblieben** ist der Linkspfeil in der Pfadeingabe: ob er dort die Schreibmarke
bewegt statt den Ordner zu wechseln, ist abgeleitet und nicht gemessen.
