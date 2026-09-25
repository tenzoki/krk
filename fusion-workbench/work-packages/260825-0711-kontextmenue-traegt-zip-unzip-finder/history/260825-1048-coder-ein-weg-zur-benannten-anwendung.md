# Coder: Ein Weg zur benannten Anwendung, für Terminal und Finder

**Datum:** 2026-08-25 10:48
**Status:** Complete
**Agent:** coder
**Baumstand:** `299d1e1` plus die Änderungen dieses Schritts

## Auftrag

Schritt 5 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`:
`appkit/terminal::ordner_oeffnen` bleibt unverändert und bekommt einen zweiten Aufrufer, der
Modulkopf wird auf die Frage nachgezogen, die das Modul beantwortet, und
`operationen::terminalordner_fehlt` wird zu `operationen::ordner_fehlt`. `kein_terminal` bleibt,
wie es ist; für den Finder steht der eigene Satz aus Schritt 4 bereit.

Gesperrt war `crates/krk-ui/src/appkit/tabelle.rs`, wo ein zweiter Coder gleichzeitig den Menübau
aus Schritt 6 baut. Die Datei ist nicht angefasst worden, und dieser Schritt hätte dort auch
nichts zu ändern gehabt.

## Was geändert ist

**`crates/krk-ui/src/appkit/terminal.rs`.** Die Kopfzeile lautet nicht mehr „Die eine Beruehrung
mit dem System, die C11 braucht", sondern „Wie ein Ordner an eine ueber ihre Buendelkennung
benannte Anwendung kommt". Darunter steht ein neuer Abschnitt `# Zwei Wege stellen diese Frage`:
das Modul heißt nach dem ersten Weg und beantwortet die Frage für beide, der Terminal-Befehl aus
C11 bringt die Kennung aus `settings.toml` mit, der Finder-Eintrag des Kontextmenüs die feste
Kennung des Finders, und beide gehen durch `ordner_oeffnen`. Der Abschnitt nennt daneben, was die
zwei Wege außerhalb dieser Datei unterscheidet, nämlich woher die Kennung kommt und welchen Satz
der Aufrufer meldet: `kein_terminal` nennt die eingestellte Kennung, weil der Nutzer sie
berichtigen kann, `kein_finder` nennt keine. Der Rumpf von `ordner_oeffnen` ist Zeile für Zeile
derselbe geblieben, ebenso der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`:
keine neue Klasse und keine neue Methode ist angesprochen.

**`crates/krk-ui/src/kommandos/operationen.rs`.** `terminalordner_fehlt` heißt `ordner_fehlt`. Der
Rumpf ist unverändert, ebenso beide Sätze, die er liefert. Der Doc-Kommentar sagt jetzt, dass der
Name das Terminal nicht mehr nennt und warum: gefragt wird von jedem Befehl, der einen Ordner an
eine über ihre Bündelkennung benannte Anwendung übergibt, und ein Aufruf mit „terminal" im Namen
aus dem Finder-Zweig wäre die Doppelbenennung, die dieses Projekt vermeidet. Die Bereichs-
überschrift darüber heißt statt „Das Terminal im angezeigten Ordner (C11, Schritt 18c)" jetzt
„Der angezeigte Ordner an einer benannten Anwendung (C11, Schritt 18c)"; die Herkunftsangabe
bleibt stehen, weil die Funktion aus C11 stammt. Der letzte Absatz des Doc-Kommentars nannte das
Leserecht, „das eine Terminal-Sitzung in einem Ordner nicht braucht", und nennt es jetzt für
beide Wege.

**`crates/krk-ui/src/appkit/standardprogramm.rs`** und **`crates/krk-ui/src/appkit/anwendung.rs`**
ziehen mit dem Namen mit: der Verweis im Doc-Kommentar von `oeffnen` und der Aufruf in
`terminal_oeffnen`. Beide Rümpfe sind sonst unverändert; das Verhalten des Terminal-Befehls ändert
sich nicht.

## Drei Stellen, an denen der Plan im Baum nicht hielt

**Erstens: es sind acht Fundstellen und nicht vier.** Der Plan schreibt „die vier Fundstellen des
alten Namens ziehen mit", und vier ist die Zahl der **Dateien**. Der Name stand achtmal:
`operationen.rs` fünfmal (die Definition, der Doc-Verweis der Probe und drei Aufrufe in ihrem
Rumpf), `terminal.rs`, `standardprogramm.rs` und `anwendung.rs` je einmal. Alle acht sind
umgezogen. Ein übersehener Name hätte den Bau angehalten, wie der Plan es unter „Risks" annimmt;
die Abweichung ist damit folgenlos und nur der Zählung wegen festgehalten.

**Zweitens: der Doc-Kommentar der Funktion nannte das Terminal sehr wohl.** Der Plan begründet die
Umbenennung damit, dass „ihr Rumpf bereits allgemein ist, ihre Texte das Terminal nicht nennen".
Für die zwei gelieferten Sätze stimmt das, für den Doc-Kommentar nicht: seine Kopfzeile lautete
„Ob der Ordner noch da ist, den der **Terminal-Befehl** uebergeben soll (C11)", und sein letzter
Absatz sprach von der Terminal-Sitzung. Beide Stellen sind mit umgezogen, denn eine Funktion, die
für zwei Wege da ist und in ihrer ersten Zeile einen davon nennt, hätte die Umbenennung nur halb
vollzogen.

**Drittens: die Probe heißt weiter `ein_fehlender_terminalordner_nennt_den_pfad`.** Sie ist die
einzige Stelle im Baum, die den Terminal-Ordner noch im Namen führt, und sie ist absichtlich
stehen geblieben: der Plan nennt sie nicht, sie prüft den Fall, für den C11 sie verlangt hat, ihr
Prüfordner heißt `terminalordner`, und der geschlossene Datensatz
`archive/260819-1613-safe-cleanup-tier-1/shared/issues/260810-1753_c_*` führt sie unter diesem
Namen. Wer sie umbenennen will, entscheidet das gegen die Nachvollziehbarkeit jenes Datensatzes
und nicht in diesem Schritt.

## Was dieser Schritt bewusst nicht tut

`ordner_fehlt` hat nach diesem Schritt weiter **genau einen** Aufrufer, den Terminal-Zweig in
`Anwendungsdelegierter::terminal_oeffnen`. Der zweite kommt mit Schritt 7, der den Finder-Zweig
setzt. Der Modulkopf von `terminal.rs` und der Doc-Kommentar von `ordner_fehlt` sprechen deshalb
um einen Schritt vor: sie beschreiben, wofür das Stück da ist, und nicht, wie viele Rufer heute
Nachmittag dastehen. Eine Zahl steht an keiner der zwei Stellen, gerade damit sie mit Schritt 7
nicht falsch wird.

`kein_finder` und die drei Geschwister aus Schritt 4 tragen weiter ihr
`#[cfg_attr(not(test), expect(dead_code, …))]`. Dieser Schritt ruft keines davon; er nennt
`kein_finder` allein in einem Doc-Verweis, und ein Doc-Verweis ist für den Übersetzer kein
Aufruf. Die `expect`-Marken fallen mit Schritt 7.

## Abnahme

`make check` — Exit 0. Alle vier Kommandos grün, mit der gleichzeitigen Änderung des zweiten
Coders in `tabelle.rs` im Baum.

Zusätzlich `cargo doc -p krk-ui --no-deps --document-private-items`: keine der vier neuen oder
umgezogenen Doc-Verweisungen (`ordner_oeffnen`, `ordner_fehlt`, `kein_terminal`, `kein_finder`)
steht unter den unaufgelösten Verweisen. Die Warnungen, die der Lauf sonst ausgibt, standen schon
vor diesem Schritt im Baum und sind nicht angefasst.
