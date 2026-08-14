# Shaper: Tippen filtert die Dateiliste, flach und über den ganzen Unterbaum

**Datum:** 2026-08-14
**Agent:** shaper (anticipated-circle mode)
**Status:** Complete
**Ergebnis:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`

## Der Entwurf

Tippen im Dateifenster soll die Liste filtern statt die Auswahl anzuspringen, und ein Schalter soll die Suche auf den ganzen Unterbaum ausdehnen. Der Entwurf kam nicht aus dem Ablagespeicher; der einzige dort offene Eintrag (`shared/backlog/260813-2033_p_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`) betrifft den Editor-Einstieg und ist unberührt geblieben. Es gab deshalb keine Beförderung zu vermerken.

## Der Verlauf

Der Lauf war der zweite Dispatch. Eine frühere Shaper-Instanz hatte die Klärungsrunde gefahren, ihre vier Fragen ohne `AskUserQuestion` an den Orchestrator zurückgegeben, und der Nutzer hat alle vier beantwortet. Dieser Lauf hat die Antworten in einen Circle überführt und den Befund der Vorgängerin an den genannten Stellen nachgeprüft, statt ihn zu übernehmen.

## Die vier Antworten

1. **Einstieg.** Tippen ohne Zusatztaste filtert. Das sechste Abnahmekriterium von C2 der Runde 1 wird ersetzt, nicht ergänzt. Kein neues Kürzel und kein neues Bedienelement für den Einstieg.
2. **Darstellung.** Flache Trefferliste, kein Baum. Jede Trefferzeile nennt den Unterordner, in dem sie liegt; `Eintrag` bekommt dafür ein Feld. `tabelle.rs` bleibt eine `NSTableView`. Die Baumvariante hat der Nutzer nach der Kostenfrage fallen gelassen, und das steht im Circle-Datensatz, damit es niemand als Versehen liest.
3. **Großer Baum.** Laufende Anzeige mit Abbruch, gebaut auf dem vorhandenen Lesevorgang. Keine zweite Mechanik, kein Deckel, keine Tiefengrenze, keine elfte Zeitzusage.
4. **Ordner.** Bei flacher Suche sichtbar, bei tiefer Suche gefiltert.

Die fünf Vorbelegungen der Vorgängerin gelten unverändert: Filter je Tab und beim Ordnerwechsel geleert, „Tief" wirkt erst bei stehendem Filtertext, `Esc` löscht zuerst den Filtertext, die Statuszeile nennt gezeigte gegen vorhandene Einträge, Suchen und Ersetzen über mehrere Dateien bleibt außerhalb.

## Was dieser Lauf am Baum nachgeprüft hat

Stand `43dfe90`, gelesen am 260814. Der Befund der Vorgängerin hält an allen genannten Stellen. Vier Dinge sind über ihn hinaus gefunden worden, und jedes hat einen Entscheidungsdatensatz nach sich gezogen:

- `kommandos::operationen::betroffene` läuft bereits allein über die sichtbaren Zeilen und baut die Pfade als `ordner.join(&eintrag.name)`. Beides trifft die Runde: die erste Hälfte macht das Verhalten der Markierung unter dem Filter zum Regelfall, die zweite ist für einen Treffer aus einem Unterordner der falsche Pfad.
- Die Statuszeile hat fünf Ränge in einer vollständigen Fallunterscheidung ohne Auffangzweig, und ihr Modulkopf hält fest, dass die Zahl der Einträge in dieselbe Zeile kommt und nicht in eine zweite. Die Filterzahl wäre der sechste Rang und kollidiert mit dem Markierungsstand.
- `angezeigtedatei::welche` kennt nur Vorschau und Editor. Von einem Treffer in dessen Ordner führt heute kein Weg.
- `traegt_ein_dateiname` aus dem Sprungmarken-Modul hat mit der Tippsuche der Belegungsansicht einen zweiten Nutzer und darf mit der Sprungmarke nicht mitfallen.

Nicht betroffen ist das Stapelumbenennen: es holt seine Namen aus derselben `betroffene`-Auswahl und prüft Kollisionen gegen `alle_namen`, das über den vollen Bestand läuft.

## Die sechs offenen Fragen

Alle sechs liegen unter `decisions/` dieses Circles, jede als eigener Datensatz im Zustand offen.

| Frage | Empfehlung |
|---|---|
| Passt der Filter auf den Namensanfang oder auf jede Stelle des Namens? | jede Stelle |
| Welche Tastenkombination schaltet die tiefe Suche? | keine |
| Wo steht die Filterzahl in der Rangfolge der einen Statuszeile? | über dem Markierungsstand |
| Was geschieht mit einer Markierung, die der Filter ausblendet? | bestehende Regel behalten, Statuszeile sagt es |
| Wie kommt der Nutzer von einem tiefen Treffer in dessen Ordner? | dritte Quelle für den Ordnersprung |
| Steigt die tiefe Suche in symbolische Verknüpfungen hinab? | nein |

## Was danach kommt

Der Circle steht als vorgesehen (`_a_`). Der Nutzer hat gewählt, das Vorhaben sofort zu fahren; die Aktivierung und der anschließende Spec-Dispatch liegen beim Orchestrator und nicht bei diesem Lauf.
