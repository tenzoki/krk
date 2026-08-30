# Coder-Sitzung: Schritt 7 der Runde 23, der Git-Bereich als Ansicht

**Date:** 2026-08-31
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 7
**HEAD:** `3090441` (nicht committet; der Orchestrator committet)

## Was gebaut wurde

### `crates/krk-ui/src/appkit/git.rs` (neu) — die drei Flächen

`Gitfenster` nach dem Muster von `Vorschaufenster`: ein `NSObject` mit
Trägeransicht, das zugleich Datenquelle und Delegierter seiner Tabelle ist —
wie die Lesezeichenleiste und anders als das Dateifenster, wo Quelle und
Delegierter getrennt sind. Zwei kleine `NSView`-Unterklassen stehen daneben,
jede mit genau einem Grund:

- **`Gitsicht`**, der Träger, überschreibt `resizeSubviewsWithOldSize:`. Das
  Etikett der Einzelheiten bricht um, seine Höhe hängt also an seiner Breite,
  und ohne diese Meldung bliebe sie auf dem Stand des letzten `zeigen` stehen —
  beim Schmalerziehen des Fensters wären die untersten Zeilen abgeschnitten.
- **`Einzelheitenflaeche`**, der Inhalt der unteren Rolle, bejaht `isFlipped`.
  Eine `NSScrollView` zeigt den Ursprung ihres Inhalts, und der liegt bei einer
  gewöhnlichen Ansicht **unten**; ohne diese Klasse begänne die Anzeige einer
  langen Commit-Nachricht an ihrem Ende. Das Etikett selbst kann die Antwort
  nicht geben: `NSTextField` legt seinen Text daran aus.

Die drei Flächen stehen mit Autoresizing untereinander und **ohne zweite
`NSSplitView`** (Entscheidung 5): Kopf oben mit fester Höhe, Einzelheiten unten
mit fester Höhe, die Liste bekommt, was übrig bleibt.

**Die Schnittstelle ist die aus dem Plan**, ohne Zusatz: `bauen(mtm)`,
`sicht()`, `fokusansicht()` (die Verlaufsliste), `zeigen(&Gitmodell)`,
`kommando_ausfuehren(kommando)`, `nachlademelder_setzen`.

### Die fünf Zusagen des Schritts, und woran jede hängt

**Ohne Auswahl bleibt die Fläche der Einzelheiten leer** (C3.5). Der leere Text
ist kein Sonderfall mit eigenem Zweig, sondern das, was `None` liefert:
`einzelheiten_schreiben` fragt die Auswahl, holt sich den Text der Zeile und
schreibt `unwrap_or_default()`.

**Während des Nachladens erscheint keine Platzhalterzeile und kein
Fortschrittsanzeiger** (C4.4, A8). Das Modul kennt keinen Zwischentext und kann
deshalb keinen zeigen; es schreibt, was das Modell hergibt, und das ist vor der
ersten Antwort die leere Zeichenkette. `kopftext` verzweigt vollständig über die
vier Fälle „oben leer / unten leer" und lässt weg, was nichts sagt — ohne diesen
Zweig trüge der Kopf einen führenden oder abschließenden Umbruch und zeigte eine
leere Zeile, wo A8 nichts verlangt.

**Der Bereich blendet sich nie selbst aus** (C6.4). Nachgezählt:
`grep -rn 'sichtbar_setzen' crates/krk-ui/src` liefert drei Dateien, und alle
Aufrufstellen liegen in `fenstermodell.rs`; die Nennungen in `appkit/git.rs` und
`appkit/anwendung.rs` sind Prosa, und die in `git.rs` sagt gerade, dass die Datei
ihn nicht ruft.

**Keine Meldung geht in die Statuszeile** (C6.6). `grep -n 'eprintln!|println!'`
über `appkit/git.rs`: keine Fundstelle; das Wort „Hinweis" steht dort einmal, im
Modulkopf, im Satz, der die Zusage schreibt.

**`Aufteilung::bauen` nimmt die Ansicht als sechsten Parameter und rahmt sie wie
die fünf anderen** (C1.6, C2.4). Das steht seit Schritt 1: sechster Parameter,
sechster `gerahmt(mtm, git)` im Literal von `Aufteilung::rahmen`, `rahmenrolle`
färbt mit, `Bereich::ALLE` führt Git. **Für diesen Schritt blieb dort nichts zu
tun**, und `appkit/aufteilung.rs` ist deshalb nicht angefasst.

### Wo die Auswahl wohnt, und warum das eine Nutzerfrage geworden ist

Schritt 6 hat der Auswahl ein Feld im `Gitmodell` gegeben; Schritt 7 hat sie in
den Ivars des `Gitfenster` gebaut. **Das ist kein Versehen, sondern die Folge
der Signaturen, die der Plan setzt:** `zeigen` bekommt das Modell lesend,
`kommando_ausfuehren(kommando)` bekommt es gar nicht, und `Tabinhalt::gitmodell`
schließt einen Schreiber von außen ausdrücklich aus. Entscheidung 5 sagt
daneben, `kommando_ausfuehren` bewege „die Auswahl der Liste".

Der Kommentar am `expect(dead_code)` in `gitmodell.rs` liest den Schnitt anders
und nennt `auswahl_setzen` unter den Ablesern von `zeigen`. Beides zugleich geht
nicht. Der sichtbare Unterschied ist einer: ob eine Auswahl im Verlauf einen
Tabwechsel übersteht — es gibt **ein** Gitfenster und **ein Gitmodell je Tab**.
Kein Kriterium des Specs verlangt das eine oder das andere, also ist es keine
Coder-Entscheidung: der Datensatz
`decisions/260831-0120_o_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md`
legt drei Möglichkeiten mit ihren Kosten vor.

**Was der gebaute Stand trotzdem hält, und ohne zweite Meldung:** `zeigen`
behält die Auswahl nur dort, wo die Zeile an ihrer Stelle **wortgleich** dieselbe
geblieben ist. Die Verlaufszeile trägt den Kurzhash (A5), also heißt „derselbe
Text an derselben Stelle" hier „derselbe Commit". Ein nachgeladener Schwung hängt
hinten an und lässt die Auswahl stehen (C4.2), ein neuer Ordner ersetzt die
Zeilen und nimmt sie mit (C4.6), und ein Tabwechsel auf einen anderen Ordner
ebenso. Die Frage ist damit aus den Eingaben entscheidbar, die `zeigen` hat.

### Vier Stellen über den Plantext hinaus, jede mit Grund

**1. `crates/krk-core/src/git/texte.rs` bekommt `einzelheiten(&Commit)`.** C3.4
verlangt für den ausgewählten Commit die vollständige Nachricht, den Autor mit
E-Mail, das Datum und den vollen Hash. Schritt 3 hat vier Texte angelegt und
diesen nicht; er musste also entstehen. Er steht im Kern und nicht in `git.rs`,
weil der Modulkopf jener Datei den Grund schon schreibt: `krk-ui` hat kein
Bibliotheksziel, und ein Satz ohne Probe ist der Satz, den die nächste Runde
unbemerkt ändert. Das Datum kommt aus `kalendertext`, wie in der Zeile darüber;
eine zweite Datumsform wäre eine zweite Antwort auf dieselbe Frage. Zwei Proben
dazu, eine über die vier Angaben und eine über den Zeilenumbruch, den `git` an
jede Nachricht hängt.

**2. `crates/krk-ui/src/gitmodell.rs` bekommt `einzelheiten(zeile)`.** Der
durchreichende Ableser dazu, neben `verlaufszeile`. **Nach der Stelle gefragt
und nicht nach der Auswahl**, weil der Git-Bereich die Auswahl seiner Liste
selbst hält und die Einzelheiten deshalb zu jeder Zeile braucht.

**3. Die Zählprobe `die_zuordnung_auf_eine_ansicht_steht_in_der_vorschau_genau_einmal`
(`appkit/vorschau.rs`) zieht ihre Erwartungsliste nach.** Sie schreibt die
Dateien aus, die `fn fokusansicht` tragen, und erwartete zwei;
`Gitfenster::fokusansicht` ist die dritte. Der Plan verlangt den Namen
ausdrücklich, und die Zusage der Probe — es bleibt bei **einer** Zuordnung in
der Vorschau — ist davon nicht berührt: die neue Stelle beantwortet dieselbe
Frage für einen **anderen** Bereich. Der Doc-Kommentar schreibt das jetzt aus,
samt dem Satz, dass ein weiterer fokussierbarer Bereich seine eigene Zeile
mitbringt.

**4. `appkit/git.rs` trägt eine Ausnahme von der Totprüfung an seinem Kopf.**
Der Platzhalter in `appkit/anwendung.rs` bleibt stehen: ihn zu ersetzen hieße,
`Gitfenster::bauen` zu rufen und das Ergebnis in den Ivars des
Anwendungsdelegierten zu halten, und das ist die Einhängung, die Schritt 8
trägt. Solange sie fehlt, hat die ganze Datei keinen Rufer.

Die Ausnahme steht deshalb **einmal, am Modulkopf**, und nicht ein Dutzend Mal
an den einzelnen Stücken: `Gitfenster::bauen` ist der eine Eingang, von dem aus
jedes Stück erreichbar ist, und ohne ihn ist keines es — die Datei ist in dieser
Frage ganz oder gar nicht. Damit ist das Ablaufdatum scharf: sobald Schritt 8
`bauen` ruft, lebt alles, der Übersetzer meldet die Erwartung als unerfüllt und
zwingt zum Entfernen der Zeile.

**Sie steht ohne `cfg_attr(not(test))`, anders als die vier aus Schritt 6**, und
das ist gemessen und nicht geraten: die Proben unten reichen an die reinen
Funktionen heran und nicht an die AppKit-Hälfte, weil `libtest` den Hauptfaden
nicht hergibt. Mit `not(test)` meldete `cargo clippy --all-targets` unter
`-D warnings` fünfzehn tote Stücke, und `make check` wäre rot. Der Grund steht am
Attribut.

## Die vier `expect(dead_code)`-Zeilen aus Schritt 6

Der Auftrag sagt, ich sei ihr Ableser und solle die betroffenen Zeilen
entfernen, sobald der Übersetzer die Erwartung als unerfüllt meldet. **Er meldet
es bei keiner der vier, und keine ist entfernt.** Warum, je Zeile:

| Stelle | Stand nach Schritt 7 |
|---|---|
| `gitmodell.rs:135`, die Leseseite | `zeigen` ruft sechs der Stücke; `auswahl`, `auswahl_setzen` und `ausgewaehlter_commit` bleiben ohne Rufer, also bleibt die Erwartung erfüllt. Der **Grund** ist nachgezogen: er nannte „Schritt 7" und nennt jetzt die drei Stücke und den Datensatz `260831-0120`. |
| `tabs.rs:202`, `Tabinhalt::gitmodell` | Ableser ist `gitanzeige_nachziehen` aus Schritt 8. Unverändert. |
| `tabs.rs:1093`, `git_gefragt_setzen` | Rufer ist `gitbedarf_nachziehen` aus Schritt 8. Unverändert. |
| `tabs.rs:1126`, `verlauf_nachladen` | Der Nachlademelder **entsteht** in diesem Schritt, aber sein Rufer ist die Einhängung beim Anwendungsdelegierten, und die ist Schritt 8. Unverändert; die Reason-Zeile sagt „aus Schritt 7 und 8" und ist damit weiterhin richtig. |

Wer Schritt 8 fährt, entfernt alle vier plus die neue am Kopf von `git.rs`.

## Die AppKit-Klassen und ihre Untergrenzen

Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` steht im
Modulkopf von `appkit/git.rs` und führt jede angesprochene Klasse und Methode
mit ihrer Untergrenze, am SDK gelesen
(`MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Headers`). Seit 10.0 und
ohne eigene Angabe: `NSView`, `NSScrollView`, `NSTableView`, `NSTableColumn`,
`NSTextField`, `NSFont`, `NSColor`, `NSIndexSet`, `NSNotification`, `NSObject`,
`NSString`, die drei bedienten Protokolle und die vier Aufzählungen, dazu die
gerufenen Methoden von `initWithFrame:` bis `indexSetWithIndex:`.

**Zehn Berührungen sind jünger als ihre Klasse, und die höchste liegt bei 11.0:**
`preferredMaxLayoutWidth` 10.8 (`NSTextField.h:45`), `usesSingleLineMode` und
`lineBreakMode` 10.10 (`NSControl.h:62`, `:65`), `labelColor` und
`secondaryLabelColor` 10.10 (`NSColor.h:201`, `:202`), `NSFontWeightRegular` und
`monospacedDigitSystemFontOfSize:weight:` 10.11 (`NSFontDescriptor.h:170`,
`NSFont.h:62`), `maximumNumberOfLines` 10.11 (`NSTextField.h:49`),
`labelWithString:` 10.12 (`NSTextField.h:93`), `tableView:viewForTableColumn:row:`
10.7 (`NSTableView.h:593`), `setUsesAutomaticRowHeights:` 10.13
(`NSTableView.h:574`), `NSTableViewStyle` samt `setStyle:` 11.0
(`NSTableView.h:77`, `:377`). **Keine liegt über macOS 15**, und das Bündel zielt
auf 15.0.

Die Deckung des Abschnitts über das Verzeichnis ist nachgezählt: außer
`koordinaten.rs` und `mod.rs`, den zwei begründeten Ausnahmen, trägt ihn jede
Datei unter `crates/krk-ui/src/appkit/` (C9.9).

## Warum die Fläche der Einzelheiten kein `NSTextView` ist

Der Modulkopf trägt den Absatz. Kurz: eine `NSTextView` wäre die dritte eigene
Textfläche von KRK, und dann müsste `Anwendungsdelegierter::ist_eigene_textflaeche`
entscheiden, ob sie sich dort anmeldet — der Ereignisabgriff fragt nach der
**Nämlichkeit** des Ersthelfers und nicht nach seiner Klasse. **Ein Etikett ist
keine `NSTextView`, und die Frage stellt sich gar nicht**: es nimmt den
Ersthelferrang nicht an, kein Tastendruck landet in ihm, und es steht folglich
nicht bei `ist_eigene_textflaeche`. Der Preis ist danebengeschrieben: der Text
lässt sich nicht markieren und nicht kopieren. E13 verlangt eine Fläche, die
zeigt.

## Die Proben

Elf neue in `appkit/git.rs`, zwei in `krk-core/src/git/texte.rs`, alle in
`#[cfg(test)]`-Modulen neben dem Code. **Keine baut eine `NSTableView` oder ein
`NSTextField`:** die prüfbare Aussage jedes Kriteriums liegt in einer reinen
Funktion, und die drei — `kopftext`, `ziel`, `haelt_die_auswahl` — stehen
deshalb außerhalb der Klasse, nach dem Muster von
`crate::kommandos::rueckschritt`. Eine Probe, die `MainThreadMarker::new_unchecked`
behauptet, entsteht damit gar nicht erst; die Lesezeichenleiste, das Vorbild
dieses Schritts, trägt selbst keine.

| Probe | Kriterium |
|---|---|
| `ein_frisches_modell_traegt_einen_leeren_kopf` | A8, C4.4 |
| `der_kopf_traegt_die_zeile_oben_und_die_zusammenfassung_darunter` | C3.1, C3.2 |
| `ohne_markenmeldung_traegt_der_kopf_genau_eine_zeile` | A8 |
| `ohne_repository_steht_der_satz_aus_a14_allein_im_kopf` | C6.1 |
| `ohne_auswahl_faengt_der_pfeil_in_seiner_laufrichtung_an` | C4.2 |
| `an_beiden_raendern_bewegt_sich_nichts` | C4.2 |
| `eine_leere_liste_bewegt_sich_nicht` | C4.2 |
| `ein_nachschlag_laesst_die_auswahl_stehen` | C4.2 |
| `ein_anderer_ordner_nimmt_die_auswahl_mit` | C4.6, C3.5 |
| `ein_zurueckgesetzter_verlauf_laesst_keine_auswahl_stehen` | C4.6 |
| `jede_zeile_traegt_die_einzelheiten_ihres_commits` | C3.4, E13 |
| `texte::die_einzelheiten_tragen_die_vier_angaben_in_voller_form` | C3.4, E13 |
| `texte::der_umbruch_am_ende_der_nachricht_erzeugt_keine_leerzeile` | C3.4 |

**Eine trägt mehr als ihren Namen.** `ein_anderer_ordner_nimmt_die_auswahl_mit`
sichert zuerst zu, dass beide Ordner **gleich viele** Commits haben. Ohne diese
Zeile bewiese sie nur, dass eine Längenprüfung greift, und die Regel, die sie
halten soll, ist der Vergleich des Inhalts.

## `#[must_use]`

An `Gitfenster::kommando_ausfuehren` (ein nicht ausgeführtes Kommando läuft
weiter, und ein still fallengelassener `false` verschluckte es), an
`texte::einzelheiten`, an `Gitmodell::einzelheiten` und an den drei reinen
Funktionen `kopftext`, `ziel` und `haelt_die_auswahl` — jede von ihnen ist eine
Antwort ohne Nebenwirkung, deren stilles Fallenlassen unbemerkt bliebe.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, 886 Proben im Binärziel von
`krk-ui`, davon die elf neuen, und 234 in der Bibliothek von `krk-core`, davon die zwei neuen.

Kein `git stash`, kein `git checkout .`, kein `git reset --hard`, kein
`git clean`, kein `git restore .`. Nicht committet.
