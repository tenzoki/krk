# Coder: Die Regel des Kontextmenüs, ohne AppKit

**Datum:** 2026-08-25 09:27
**Status:** Complete
**Agent:** coder
**Baumstand:** `ab74c9e` plus die Änderungen dieses Schritts

## Auftrag

Schritt 4 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`: das
neue Modul `crates/krk-ui/src/kommandos/kontextmenue.rs`, angemeldet in `kommandos/mod.rs`, dazu
die Sätze der Statuszeile und `erzeugt_genau_ein_ziel` in `kommandos/operationen.rs`. Kein
AppKit, vollständig ohne Fenster prüfbar, nach dem Vorbild von `kommandos/rueckschritt.rs`.

Drei beantwortete Datensätze binden den Schritt, alle am 260824-2120 entschieden: der Archivname
hängt die Endung an (`decisions/260825-0711_*_wie-heisst-das-archiv-einer-einzelnen-datei-mit-endung.md`,
Möglichkeit 1); ein Archiv wird an der Endung erkannt, ohne Rücksicht auf die Schreibung und ohne
Dateizugriff (`decisions/260825-0711_*_woran-erkennt-unzip-dass-eine-datei-ein-zip-ist.md`,
Möglichkeit 1); Unzip wirkt auf die betroffenen Einträge und entpackt **jedes** Archiv darin
(`decisions/260825-0727_*_nimmt-unzip-die-betroffenen-eintraege-*`, Möglichkeit 3).

## Was entstanden ist

`crates/krk-ui/src/kommandos/kontextmenue.rs`, neu, mit sechs öffentlichen Stücken:

- `Kontextbefehl` mit drei Werten (`Zippen`, `Entpacken`, `ImFinderZeigen`), der Tafel `ALLE`,
  `titel()`, `menuemarke()` und `von_menuemarke()`.
- `Entpackbefund` mit `Archive(Vec<(PathBuf, PathBuf)>)`, `Keines` und `Mehrere`.
- `ist_zipname(name)`, `archivname(betroffen, ordner)`, `ordnername_zum_archiv(archiv)` und
  `entpackziel(modell, betroffen, ordner)`.

In `kommandos/operationen.rs` dazu `nichts_zu_packen()` als dritter Eingang von
`nichts_betroffen`, `kein_archiv()`, `mehrere_archive()`, `kein_finder()` und
`erzeugt_genau_ein_ziel(art)`.

Achtzehn Proben stehen in `#[cfg(test)]` neben dem Code in `kontextmenue.rs`, fünf weitere in
`operationen.rs`. Darunter die Tafel über alle drei `Kontextbefehl`-Werte mit Titel und Marke,
der Rundweg Marke → Wert → Marke, der Rundweg Name → Archiv → Name über vier Namensgestalten und
die Tafel über alle sechs Werte von `Art`.

## Sieben Stellen, an denen der Plan im Baum nicht hielt

**Erstens: `krk_core::operation::namen_teilen` gibt es unter diesem Pfad nicht.** Der Baum trägt
die Trennung als `krk_core::operation::umbenennen::namen_teilen`; die Zeile `pub use umbenennen::{…}`
in `operation/mod.rs` führt `freier_name`, `name_pruefen` und `umbenennen`, aber nicht
`namen_teilen`. Genommen ist der bestehende Pfad, wie ihn `krk-core/src/stapelumbenennen/regel.rs`
schon nimmt. Ein zweiter Weg über eine neue Wiederausfuhr wäre zwei Namen für ein Stück gewesen,
und genau das vermeidet die Datei, in der `namen_teilen` steht, ausdrücklich.

**Zweitens: `archivname` trennt gar nichts, und das ist die Folge der Nutzerwahl.** Der Plan
verlangt die Bildung „über `namen_teilen` statt über eine zweite Trennung von Stamm und Endung".
Die anhängende Regel braucht Stamm und Endung aber nicht auseinander: aus `bericht.txt` wird
`bericht.txt.zip`, indem `.zip` angehängt wird. Die eine Trennung des Paares steht deshalb allein
in `ordnername_zum_archiv` und in `ist_zipname`; eine zweite gibt es nicht, und der Zweck der
Vorgabe ist damit erfüllt, ihr Wortlaut nicht.

**Drittens: `Entpackbefund::Archiv(PathBuf)` ist zu `Archive(Vec<(PathBuf, PathBuf)>)` geworden.**
Der Plan schreibt „genau ein Archiv, hier ist es"; die jüngere Nutzerentscheidung lässt jedes
betroffene Archiv entpacken, und Schritt 3 hat `Art::Entpacken { ziele: Vec<PathBuf> }` daraufhin
schon auf eine Liste gestellt. Der Befund liefert die Paare aus Archiv und Zielordner, also genau
das, was `Auftrag::entpacken(paare)` entgegennimmt. `Keines` und `Mehrere` bleiben, und `Mehrere`
gilt seitdem **allein der Ersatzregel**: mehrere *betroffene* Archive sind kein Fehlbefund
mehr, sondern der Regelfall.

**Viertens: die Umrechnung heißt `menuemarke` und nicht `marke`.** Die kurze Fassung ließ die
bestehende Probe `abwurfregel::tests::die_marke_hat_genau_einen_aufrufer` mit sechs Aufrufern rot
werden. Sie zählt Aufrufstellen über den Namen im Quelltext (`crate::quellbaum::aufrufstellen`)
und kann zwei Begriffe desselben Namens nicht trennen; die Abwurfmarke aus C4 der Runde 13 trägt
ihn bereits. Behoben ist die Wurzel und nicht die Zahl in der fremden Probe: zwei verschiedene
Marken in einer Kiste brauchen zwei Namen. `von_menuemarke` wäre der Zählung ohnehin entgangen,
weil ein `_` davor die Fundstelle als Teil eines längeren Namens ausscheidet — die kurze Fassung
wäre also nur an der einen Hälfte aufgefallen.

**Fünftens: das Modul trägt bis Schritt 7 eine Ausnahme mit Ablaufdatum.** `krk-ui` hat kein
Bibliotheksziel, also ist `pub` dort keine Wurzel der Erreichbarkeitsrechnung: der Übersetzer
meldete alle elf Stücke als unbenutzt, und `-D warnings` hielt den Bau an. Das Modul trägt
deshalb `#![cfg_attr(not(test), expect(dead_code, …))]` am Kopf, die vier neuen Funktionen in
`operationen.rs` je ein `#[cfg_attr(not(test), expect(…))]`. `expect` und nicht `allow`, wie es
der Kopf von `rueckschritt.rs` für dieselbe Lage beschreibt: sobald das letzte Stück einen
Aufrufer bekommt, meldet der Übersetzer die unerfüllte Erwartung und hält den Bau an, bis die
Zeilen weg sind. Am Modul und nicht elfmal am Stück, weil sonst elf Zeilen zu je einem anderen
Zeitpunkt verfielen.

**Sechstens: `erzeugt_genau_ein_ziel` hängt beim Entpacken an einer Zahl und nicht am Wert.**
`Art::Entpacken { ziele }` liefert `ziele.len() == 1`. Der Plan schreibt die Funktion als „reine,
vollständige Rechnung über `Art`", und das bleibt sie; die Zahl steht in der Art selbst. Der
Grund ist die dritte Nutzerentscheidung: seit ein Vorgang mehrere Archive tragen kann, wird der
Zielordner-Konflikt je Archiv gefragt, und dann trägt das Ankreuzfeld „für alle weiteren" wieder
seinen Gegenstand. Bei genau einem Archiv trägt es ihn nicht, und die Antwort ist dieselbe wie
beim Packen. Schritt 8 bekommt damit eine Antwort, die für beide neuen Arten stimmt.

**Siebtens: der Modulkopf von `kommandos/mod.rs` trug eine Zahl, die mit diesem Modul falsch
geworden wäre.** Die Zeile „Zehn Module entlang dessen geschnitten …" und der Satz
„`abwurfregel` … ist das eine Modul hier, das kein Tastenbefehl ist" hätten beide nachgezogen
werden müssen. Die Zahl ist gefallen und durch die Anweisung ersetzt, sie mit
`grep -c '^pub mod'` zu zählen — dieselbe Antwort, die der geschlossene Datensatz
`shared/issues/260823-1032_*_zwei-zahlen-im-modulkopf-der-kommandos-*` für die zwei Zahlen daneben
schon gegeben hat, und dieselbe, die `CLAUDE.md` für jede mitwachsende Aufzählung gibt. Der Satz
über `abwurfregel` spricht jetzt von zwei Modulen, die kein Tastenbefehl sind, und nennt für
beide denselben Grund, aus dem sie trotzdem hier wohnen.

## Zwei Entscheidungen, die der Bau ausgeschrieben hat

**Die Marke zählt ab eins und nicht ab null.** Ein `NSMenuItem`, an dem niemand `setTag:` gerufen
hat, trägt die Null. Begänne die Zählung dort, liefe jeder solche Eintrag beim Zurückrechnen auf
`Kontextbefehl::Zippen` hinaus, und ein fremder Menüeintrag löste das Packen aus.
`von_menuemarke(0)` liefert `None`, und die Probe
`die_null_und_alles_daneben_benennen_keinen_befehl` hält es fest.

**Die Endungsregel gilt auch für Ordner, und der Preis ist genannt.** Ein Ordner namens
`sicherung.zip` wird als Archiv angeboten; der Vorgang überspringt ihn und nennt den Grund in der
Abschlussliste. Der Ausweg wäre, in `entpackziel` den Typ des `Eintrag` mitzulesen — er steht dort
ohne Dateizugriff da. Er ist nicht genommen, weil er die Regel entzweite: die betroffenen
Einträge kommen als bloße Pfade herein und trügen den Typ nicht mit, und dann entschiede über
denselben Ordner einmal so und einmal anders, je nachdem, ob er markiert ist. Der Modulkopf
schreibt die Erwägung aus.

## Abnahme

`make check` läuft grün, Exit 0. `cargo test -p krk-ui` zählt 780 Proben statt 757 vor diesem Schritt.

## Was dieser Schritt nicht tut

Kein Menü, keine Ausführung, keine Zeile AppKit. Die drei Einträge stehen noch nirgends; das
bauen die Schritte 6 und 7. Die drei Entscheidungsdatensätze behalten deshalb ihren Marker `_a_`:
ihre Antworten sind hier gerechnet, aber erst mit dem Menüeintrag und seinem Ausführungszweig in
Code umgesetzt.

Committet ist nichts; das tut der Orchestrator.
