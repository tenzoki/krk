# Ersetzt die Durchsicht Eintrag für Eintrag die Nutzerarbeit Teil 3 des Plans, und wird das Paket ohne die drei Nutzerarbeiten `done`?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1711-reviewer-abschlussdurchsicht-lokalisierung-luecke-der-zwei-durchsichten.md, 261014-1640-reviewer-durchsicht-der-zwei-uebersetzten-sprachtabellen-eintrag-fuer-eintrag.md, 261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md, 261001-0735_*_spec-oberflaeche-lokalisierbar-deutsch-und-franzoesisch.md

---

## Question

Der Plan dieses Pakets nennt unter `## Where this work stops` drei Nutzerarbeiten am gebauten Bündel als Schließungsbedingung: Teil 1, die Systemeinstellung „Apps → Sprache“ bietet KRK mit genau Deutsch, Französisch und Englisch an; Teil 2, je Sprache einmal gestartet und die Flächen aus C2 des Specs abgegangen; Teil 3, `fr.rs` und `en.rs` Fläche für Fläche gegen die laufende Anwendung gelesen, wonach der Nutzer die Kopfzeile je Datei entfernt. „Das Arbeitspaket bleibt `claimed`, bis der Nutzer die drei Nutzerarbeiten gefahren hat; `done` schreibt der Nutzer oder der Orchestrator auf sein Wort.“ Der Spec verlangt in C4 dasselbe („der Nutzer prüft sie Fläche für Fläche in der laufenden Anwendung“), und die ersten drei Kriterien von C1 sind nur am Bündel prüfbar.

Seit `8fa6d6d` steht am Kopf beider Tabellen statt „vom Nutzer noch nicht Flaeche fuer Flaeche durchgesehen“ die Zeile „am 261014 Eintrag fuer Eintrag gegen `de.rs` durchgesehen; in der laufenden Anwendung noch nicht“, und `CLAUDE.md` hat den Nachsatz „bis dahin ist das Arbeitspaket nicht `done`“ verloren. Die Durchsicht vom 261014 hatte das empfohlen; ein Wort des Nutzers dazu gibt es nicht. Daneben sind die Auslieferungen 3.0.0 bis 3.3.0 nach dieser Arbeit gefahren, für die der Plan die drei Nutzerarbeiten als Vorbedingung nennt; ob Teil 1 und 2 vor einer davon gefahren sind, belegt kein Datensatz.

Die Durchsicht Eintrag für Eintrag ist eine andere Prüfung als Teil 3: sie liest jeden Eintrag gegen die deutsche Quelle, sieht aber keine abgeschnittene Beschriftung, kein Blatt, kein Menü und keinen Satz im Zusammenhang seiner Fläche. Ob sie Teil 3 ersetzt und ob das Paket ohne die drei Nutzerarbeiten schließt, entscheidet der Nutzer, dessen Arbeiten das sind.

## Options

1. **Der Plan gilt wörtlich: das Paket bleibt `claimed`, bis der Nutzer die drei Arbeiten gefahren hat und es sagt.** Die Kopfzeile bleibt, wie sie seit `8fa6d6d` steht, denn sie ist wahr.
   - Folgen: der autonome Lauf endet ohne `done`; nichts am Code ändert sich; die Durchsicht vom 261014 und diese bleiben Vorarbeit für Teil 3, nicht sein Ersatz. Wer die drei Arbeiten fährt, folgt dem Plan Absatz für Absatz (Systemeinstellung, je Sprache die Flächenliste, Durchsicht in der Anwendung) und nimmt danach die Kopfzeile je Datei auf „übersetzt am 261001, durchgesehen am 261014 und am <Datum> in der laufenden Anwendung“ oder streicht sie.
2. **Die Durchsicht Eintrag für Eintrag gilt als Teil 3; Teil 1 und 2 bleiben Nutzerarbeit, und das Paket wird `done` als „gebaut“.** Nach der Konvention von `CLAUDE.md` sagt `done` über den Abnahmelauf nichts, und die Kopfzeile sagt weiter, dass die Anwendung nicht durchgesehen ist.
   - Folgen: `done` auf das Wort des Nutzers; Teil 1 und 2 stehen dann wie der Abnahmelauf gegen die zehn Zusagen als Nutzerarbeit ohne Paket daneben, und kein Datensatz hält sie; die Kriterien 1 bis 3 von C1 und das vierte von C4 bleiben ungeprüft, obwohl vier Auslieferungen sie schon tragen. Der Plan ist archiviert und wird nicht nachgezogen; dieser Datensatz ist dann die Stelle, die Teil 3 überholt.
3. **Alle drei Arbeiten gelten als Abnahme außerhalb des Pakets, das Paket wird jetzt `done`.** Wie bei den Runden, deren Abnahmelauf KRK im Vordergrund verlangt: gebaut, nicht abgenommen.
   - Folgen: wie 2, und zusätzlich fällt die Schließungsbedingung des Plans ganz; was Teil 1 am Bündel prüft (die Systemeinstellung erkennt die drei `.lproj`-Ordner), ist die zweite Haltebedingung des Specs, und sie wäre dann nie geprüft worden. Wer eine spätere Auslieferung fährt, hat keine Vorbedingung mehr zu lesen.

Die Möglichkeiten 2 und 3 brauchen je ein Wort des Nutzers; 1 braucht keines, sie ist der Stand.

## Constraints

Spec C4, viertes Kriterium, und C1, Kriterien 1 bis 3; Plan `## Where this work stops`, die drei Nutzerarbeiten und der letzte Absatz; `CLAUDE.md`, „Der Abnahmelauf verlangt KRK im Vordergrund“ und die Lesart von `done`. Kein Agent kann eine der drei Arbeiten fahren. Die Kopfzeile von `fr.rs` und `en.rs` muss bei jeder Möglichkeit wahr bleiben; heute ist sie es.

## Recommendation

Möglichkeit 1, weil sie der Stand ist und nichts vorwegnimmt: der Plan hat die drei Arbeiten dem Nutzer zugewiesen, und zwei Durchsichten am Quelltext können die Prüfung am Bündel nicht ersetzen, die der Spec für C1 verlangt. Wählt der Nutzer 2, ist die Kopfzeile schon richtig und `CLAUDE.md` braucht keine Änderung; wählt er 3, gehört in `CLAUDE.md` ein Satz, dass die Lokalisierung wie die zehn Zusagen ungemessen ausgeliefert ist.
