# Shaper: Spec für „Tippen filtert die Dateiliste, flach und tief"

**Datum:** 2026-08-14
**Agent:** shaper (in-Circle-Klärung)
**Status:** Complete
**Ergebnis:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`

## Der Auftrag

Der Orchestrator hat den Spec für den aktiven Circle bestellt und dabei zwei Berichtigungen der Directive mitgegeben, die der Nutzer am 260814-1610 vorgenommen hat: die tiefe Ansicht ist ein gefilterter Ordnerbaum und keine flache Trefferliste, und der Schalter „Deep" ist ein Ankreuzfeld ohne Tastenkombination. Fünf der sechs Entscheidungsdatensätze aus der ersten Klärungsrunde waren zu diesem Zeitpunkt beantwortet.

## Was dieser Lauf am Baum nachgeprüft hat

Stand `43dfe90`, gelesen am 260814-1830. Der Befund der ersten Klärungsrunde hält an allen genannten Stellen. Zwei Zeilenangaben in der Aufnahme des Circle-Datensatzes stimmen nicht: `sicht_neu_aufbauen` steht bei `modell.rs:427-442` und nicht bei `:246-258`, und `markierungsstand` beginnt bei `:357` und nicht bei `:350`. Der Spec führt die berichtigten Angaben.

Vier Feststellungen kommen über die erste Runde hinaus:

- **Der Filter wohnt heute im falschen Stockwerk.** `Sprungmarke` liegt in den Ivars der Tabellenansicht (`appkit/tabelle.rs:363`) und gehört damit dem Dateifenster; `Tabinhalt` (`tabs.rs`) führt keinen Suchtext. Der Umzug ändert eine Zuordnung und fügt nicht bloß ein Feld hinzu.
- **Der Vergleich ist im Baum schon einmal geschrieben.** `Belegungsmodell::zeile_traegt` (`belegungsmodell.rs:536-541`) ist genau der Vergleich, den der Nutzer gewählt hat: `to_lowercase` und `contains`, keine Faltung von Umlauten.
- **Die drei Spaltenschalter der Bereichsleiste sind fensterweit** und liegen in `Spaltensichtbarkeit` am `Fenstermodell` (`fenstermodell.rs:374`, `:590-598`). „Deep" wäre das erste Feld der Leiste, dessen Gegenstand einem einzelnen Tab gehören könnte.
- **Im Dateifenster gibt es keine Rücktaste für den Filter.** `delete` und `cmd+delete` tragen `in_papierkorb`, `opt+cmd+delete` trägt `endgueltig_loeschen`, `ctrl+delete` das Löschen in der Lesezeichenleiste. Frei ist unter den Rückschritt-Kombinationen allein `shift+delete`. Der Modulkopf der Tippsuche aus der Runde 7 hält ausdrücklich fest, dass die Sekundenregel der Sprungmarke ihren Grund darin hat, dass es in der Dateiliste keine Rücktaste gibt — und diese Runde nimmt die Sekundenregel weg.

Gezählt am 260814: `resources/default-keymap.toml` führt 83 Einträge, `Kommando` trägt 77 Varianten.

## Was das berichtigte Modell billiger macht

Der gefilterte Ordnerbaum nimmt drei Lasten weg, die die abgelöste flache Trefferliste getragen hätte. `Eintrag` braucht kein Pfadfeld, weil jede gezeigte Zeile im angezeigten Ordner liegt. `kommandos::operationen::betroffene` bleibt unverändert, weil `ordner.join(&eintrag.name)` damit wieder der richtige Pfad ist. Und `angezeigtedatei::welche` bekommt keine dritte Quelle, weil es keine Trefferzeile außerhalb ihres Ordners gibt. Der Durchlauf wird obendrein billiger: er hört je Ordner beim ersten Treffer auf, während die flache Liste den ganzen Unterbaum hätte lesen müssen.

## Vier neue offene Fragen

Der Lauf hat vier Fragen gefunden, die die erste Klärungsrunde unter dem alten Modell nicht haben konnte. Alle vier liegen als Datensätze unter `decisions/` dieses Circles, alle vier offen, und der Spec fährt bei jeder auf der Empfehlung des Datensatzes.

| Frage | Empfehlung | Warum sie neu ist |
|---|---|---|
| Bleibt der Filtertext bei einem Ordnerwechsel stehen, wenn „Deep" aus ist? | nein, geleert | Bei „Deep" an muss er stehenbleiben, sonst hat das neue Modell keinen Gegenstand. Die Vorbelegung des Nutzers kannte diesen Fall nicht. |
| Gilt das Ankreuzfeld „Deep" je Tab oder je Fenster? | je Tab | Die acht vorhandenen Felder sind alle fensterweit; der Filter gehört dem Tab. |
| Wie nimmt der Nutzer ein einzelnes Zeichen des Filters zurück? | gar nicht in dieser Runde | Die Sekundenregel fällt weg, und die Rückschritt-Taste ist im Dateifenster vergeben. |
| An welcher Stelle der Bedeutungen von `Esc` steht der Filtertext? | zuletzt | Die Directive sagt „zuerst"; das setzte den Filter vor den Abbruch eines laufenden Kopiervorgangs. |

Die sechste Frage der ersten Runde bleibt offen wie beauftragt; der Spec fährt auf der Empfehlung, den neuen Rang der Statuszeile über den Markierungsstand zu setzen.

## Der Spec

Sechs Fähigkeiten, 62 Abnahmekriterien. 56 sind allein an einer Probe abzunehmen, sechs tragen einen Anteil am laufenden Bündel, und jedes davon nennt die Beobachtung, mit der es abgenommen wird. Zwei der sechs sind reine Bündelkriterien. Die Kennzeichnung folgt dem Muster der Runde 8, das in ihrem Abschluss durchgekommen ist, und der Lehre aus dem Abschluss der Runde 9, die 21 Kriterien ohne vollen Beleg gelassen hat.

Der Spec trägt zwei Mermaid-Bilder: wie eine Zeile entsteht, und was der Durchlauf tut und was ihn beendet. Das zweite trägt genau einen Kreis, das Absteigen, und der Spec begründet ihn.

## Was offen bleibt für den Orchestrator

Der Circle-Datensatz `_t_circle.md` trägt in seinem Abschnitt `## Directive` unverändert die beiden Aussagen, die der Nutzer überholt hat, und sein Feld `**Active spec/plan:**` steht auf `(none yet)`. Beides zu ändern liegt außerhalb der Befugnis dieses Laufs: der Shaper darf einen Circle-Datensatz allein im Portfolio-Aktivierungsmodus bearbeiten, und dieser Lauf war eine Klärung im laufenden Circle. Der Spec führt die berichtigte Fassung und nennt in einem eigenen Abschnitt, welche zwei Aussagen des Datensatzes er ersetzt.
