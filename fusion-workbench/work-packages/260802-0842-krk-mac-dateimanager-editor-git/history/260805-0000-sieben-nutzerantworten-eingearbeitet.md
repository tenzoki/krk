# Sieben Nutzerantworten in Spec und Plan eingearbeitet, neun Defekte geschlossen

**Agent:** planner
**Datum:** 260805-0000
**Status:** Complete
**Auftrag:** Nutzer. Die sieben Fragen aus der Triage vom 260804-2318 sind beantwortet, alle sieben Empfehlungen übernommen; einzuarbeiten in Spec und Plan, die acht zugehörigen Defekte zu schließen, dazu der selbst gemeldete Zähldefekt.
**Berührte Dateien:** `planning/260802-1036_o_spec-navigator-geruest.md`, `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, neun Datensätze unter `issues/`, sechs neue unter `decisions/`, zwei neue unter `issues/`
**Nicht berührt:** `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein `[DONE]`-Vermerk geändert, kein Commit.

---

## Ergebnis in einem Satz

Neun Defekte geschlossen und drei neue Planschritte angelegt, S13b und S13c für das Menü "Bearbeiten" samt der Aufnahme aller Menükürzel in die Konflikterkennung und S16c für die Sichtbarkeit der Markierung; keine der zehn Zahlen aus C8 geändert; zwei neue Defekte entstanden, offen bleiben damit 14.

## Die zwei Antworten, die den Umfang der Runde vergrößern

**Das Menü "Bearbeiten" und die Menükürzel gehören zusammen und liegen in einem Codeschritt.** Der Nutzer hatte zwei Punkte getrennt gestellt, die aneinanderhängen: dass Menükürzel in die Konflikterkennung aus C3 einziehen, und dass ohne ein Menü "Bearbeiten" in kein Textfeld eingefügt werden kann, was C2 ausdrücklich zusagt. Der Zusammenhang ist eine Rechnung: KRK trägt heute zwei Kombinationen außerhalb der Konflikterkennung, das Menü brächte vier weitere, und den blinden Fleck erst zu verdreifachen und dann zu schließen wäre zweimal dieselbe Arbeit. Der Codeschritt ist **S13c**; die Datenänderung an der Belegungsdatei gehört dem `ontocoder` und liegt als **S13b** davor, wie bei S9b und S11c.

Der Mechanismus ist keiner, der neu erfunden werden muss, und das war der Befund des Rechercheschritts: Cmd+N steht seit S9b und S12 zugleich als `fenster_einblenden` in der Belegung und am Menüeintrag "Fenster einblenden". Der Modulkopf von `crates/krk-ui/src/appkit/menue.rs` schreibt aus, warum das kein Widerspruch ist, nämlich weil der Ereignisabgriff jeden Tastendruck vor der Menübehandlung sieht. S13c verallgemeinert diese eine Stelle: das Hauptmenü nimmt die Kürzel **aller** seiner Einträge aus der Belegung, statt sie im Programmtext festzulegen.

"Close All" bekommt dabei keinen Eintrag, sondern verschwindet. Eine Kombination, die AppKit selbst wählt, lässt sich weder aus der Belegung setzen noch umbelegen; ein Eintrag dafür wäre genau die Ausnahme, die der Entscheid beseitigt. "Fenster schließen" bekommt deshalb den eigenen Selektor `fensterSchliessen:` am Anwendungsdelegierten, wie ihn "Fenster einblenden" seit S12 trägt. Geprüft wird das nicht durch Aufzählen der heute bekannten Zusätze — eine Aufzählung veraltet mit der nächsten macOS-Version, und genau diesen Fall hat das Vorhaben mit "Close All" schon erlebt —, sondern durch eine Befehlszeilenmarke `--menue-protokoll`, die das gebaute Hauptmenü ausliest, und einen Vergleich gegen `resources/default-keymap.toml`.

**Die Markierung bekommt fette Schrift und einen fünften Rang.** Beide Teilfragen hatte der Nutzer dem Planner überlassen. Die fette Schrift ist eine Form und keine Farbe, braucht keine Fläche und lässt die vier Spalten aus C1 unberührt; sie geht denselben Weg wie die Farbe, die `zellenansicht` ohnehin in jedem Durchgang setzt, weil die Zellenansichten wiederverwendet werden. Verworfen sind eine fünfte Spalte, die der Auflage widerspricht, und ein Zeichen vor dem Namen, das den angezeigten Namen vom wirklichen unterscheidbar machte und die Namen in einer Proportionalschrift gegeneinander verschöbe.

Der Markierungsstand bekommt einen fünften Rang und nicht den vierten, und die Begründung ist dieselbe, die S16b für die Trennung von Befehlsantwort und Fenstermeldung schon einmal gegeben hat: zwei Aussagen mit verschiedenen Lebensdauern in einem Feld ergäben ein Feld mit zwei Löschregeln. Die Tabmeldung trägt einen Ordner, der sich nicht lesen ließ, und muss stehen bleiben, während der Nutzer markiert und die Markierung wieder aufhebt. Der neue Rang steht **unter** ihr, weil ein nicht lesbarer Ordner ein Fehler ist und eine Markierungszahl keiner; er ist der Ruhezustand der Zeile. Sein Preis ist kleiner als der eines Feldes, weil er keines ist: er wird aus dem Ordnermodell des sichtbaren Tabs errechnet, sooft die Zeile geschrieben wird. Ein Feld hätte vier Schreiber und vier Gelegenheiten zu veralten.

## Die vier Antworten ohne neuen Schritt

Der Fokusvorbehalt für die beiden C10-Befehle kostet keinen eigenen Mechanismus, weil S18 ihn ohnehin bauen muss. Bis heute gibt es genau einen fokussierbaren Bereich und genau eine Funktion, die danach fragt; mit der Lesezeichenleiste wird die Frage für jedes Kommando fällig. `Kommando` bekommt deshalb in S18 eine Eigenschaft `Wirkungsbereich`, die Zuleitung fragt sie einmal, und die Einzelabfrage aus S16 geht darin auf. S19 trägt für die beiden C10-Befehle nur noch einen Wert ein. Hier ist eine Abfrage weniger entstanden und nicht eine mehr.

Das vierte Abnahmekriterium von C7 wird am Modell nachgewiesen; S12 bleibt abgenommen und bekommt eine Notiz. C9 sagt die selbsttätige Auffrischung nur noch für lokale Dateisysteme zu, und was der Nutzer auf einem Netzpfad stattdessen erlebt, steht als eigenes Abnahmekriterium dort, nicht nur im Plan. C4 schreibt aus, dass die gemeldete Eintragszahl die angefassten Einträge zählt; kein Codeeingriff, `crates/krk-core/src/operation/fortschritt.rs` zählt bereits so.

## Die Prüfvorschrift für die AppKit-Grenze

Sie hängt am Abnahmekriterium von S23 und nicht nachträglich an S6, wie der Nutzer entschieden hat: ein Grep dort trägt die ganze Grenze statt der Hälfte und gilt für alle Schritte statt für einen abgenommenen. Gesucht wird die `use`-Zeile, weil eine `objc2`-Bindung ohne sie nicht zustande kommt, gleich ob die Kiste sie als sicher oder als unsicher führt.

**Die Verankerung am Zeilenanfang ist gemessen und nicht angenommen.** `grep -rn 'use objc2' crates/krk-ui/src` ohne Anker liefert am Stand vom 260805-0000 sechs Treffer außerhalb von `src/appkit/`, und alle sechs sind Modulkommentare der Form "In dieser Datei steht keine `use objc2`-Zeile": `fenstermodell.rs`, `tabs.rs`, `auffrischung.rs`, `messmodus.rs`, `kommandos/mod.rs` und `kommandos/operationen.rs`. Verankert liefert dieselbe Suche nichts, die Prüfung geht am heutigen Stand also auf. Es ist dieselbe Falle, die dieses Vorhaben bei der `unsafe`-Vorschrift schon zweimal getroffen hat, bei S2 und bei S6.

## Zwei Zahlen sind aus Abnahmekriterien entfernt

Die Zahl der C4-Kriterien stand im Abnahmekriterium von S16 auf "achtzehn", C4 führt neunzehn, und der Vorgängerdefekt hatte sie schon einmal von sechzehn auf achtzehn gezogen und dabei um eins verfehlt. Sie ist entfallen statt ein drittes Mal nachgezogen zu werden. Beim Nachziehen fiel derselbe Fehler ein zweites Mal auf: das Abnahmekriterium von S12 sprach von "den acht Abnahmekriterien aus C1 und den sieben aus C7", während C1 schon vorher neun Zeilen der Form `- [ ]` führte. Auch dort ist die Zahl entfallen. Eine Zahl, die etwas in einer anderen Datei zählt, geht mit jeder Änderung dort schief; das ist dieselbe Sorte Prüfung wie die drei fest verdrahteten Zahlen, die mit S9b umgefallen sind.

## Der Graph, nachgerechnet

Drei Knoten und sieben Kanten sind dazugekommen, keine vorhandene ist weggefallen. Maschinell aus dem Mermaid-Block gezählt: **34 Knoten, 52 Kanten**, Verhältnis 1,53, zyklenfrei, kein Knoten ohne Kante, und jede Kante läuft von der kleineren zur größeren Schrittnummer. Höchster Ausgangsgrad 4 bei S1, höchster Eingangsgrad **7 bei S23**, ohne ausgehende Kante allein S23 und S6b. Der gestiegene Eingangsgrad von S23 ist die Aussage dieses Knotens und kein Entwurfsfehler: er ist der Endpunkt der Runde, und was auf ihn zeigt, zählt auf, was die Runde vollständig macht.

## Zwei neue Defekte

**Ein toter Netzpfad lässt den Lesefaden hängen.** Der Datensatz zum Netzlaufwerk trug eine zweite, ungemessene Beobachtung, die die Einengung von C9 nicht auflöst: ein Netzpfad, dessen Server verschwindet, lässt Systemaufrufe blockieren statt scheitern, und der Lesefaden prüft sein Abbruchkennzeichen zwischen zwei Aufrufen. Sie betrifft den Zugriff und nicht die Auffrischung und wäre mit dem beantworteten Defekt verschwunden. Jetzt: `issues/260805-0000_o_ein-toter-netzpfad-laesst-den-lesefaden-haengen.md`.

**Zehn Verweise tragen einen überholten Marker.** Bei der Schlussprüfung aller Datensatzverweise in Spec und Plan gegen den Dateibestand: zehn Pfade nennen `_o_`, wo die Datei `_a_`, `_i_` oder `_c_` trägt, und in sieben Fällen behauptet der umgebende Satz zusätzlich einen Zustand, den es nicht mehr gibt. Das ist ein Abgleichdurchgang und keine Textkorrektur; gemeldet als `issues/260805-0000_o_zehn-verweise-in-spec-und-plan-tragen-einen-ueberholten-marker.md`. Zwei Verweise auf den Cmd+W-Defekt hat dieser Durchgang mitgezogen, weil er die umgebenden Absätze ohnehin neu geschrieben hat.

## Was ausdrücklich nicht getan wurde

- Kein Eingriff in `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`.
- Kein `[DONE]`-Vermerk geändert. S13b, S13c und S16c sind neue Schritte und tragen keinen.
- Kein Commit.
- Der Defekt `issues/260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md` bleibt offen. Er ist eine Codesache und schließt mit der Umsetzung von S13c; er hat einen Nachtrag bekommen, der die Zuordnung nennt.

**Offene Defekte nach diesem Durchgang: 14** (21 minus 9 geschlossene plus 2 neue).
