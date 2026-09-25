# Aufräumdurchgang: Plantext nachziehen, Zusagen gegen die Messung prüfen, Rest triagieren

**Agent:** planner
**Datum:** 260804-2318
**Status:** Complete
**Auftrag:** Nutzer, vor Beginn der Phase E. Wortlaut: "Ja, wir räumen erst mal auf, falls die engen zeitvorgaben problem machen: aufweichen. pragmatische lösungen planen."
**Berührte Dateien:** `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, `planning/260802-1036_o_spec-navigator-geruest.md`, `decisions/260804-2318_a_fortschrittsschwelle-nach-zeit-statt-nach-menge.md` (neu), 16 Datensätze unter `issues/`
**Nicht berührt:** `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein `[DONE]`-Vermerk geändert, kein Commit.

---

## Ergebnis in einem Satz

Fünfzehn Defekte geschlossen, alle durch eine Änderung an Plan oder Spec; eine Schwelle in C4 bewegt, von 100 Einträgen beziehungsweise 100 MB auf 150 ms Laufzeit; **keine der zehn Zahlen aus C8 geändert**; ein neuer Planschritt S17c; 21 Defekte in vier Einordnungen sortiert.

## Teil 1: warum die Dateilisten wiederkehrend unvollständig waren

Vier Umsetzungen hintereinander, S12, S14, S16 und S17, haben je eine nötige Datei gefunden, die ihre Liste nicht führte. Die vier Nachträge sind gemacht, aber sie sind nicht der Befund. Der Befund ist, dass der Kopf von `## Implementierungsschritte` zwei Zusagen zugleich trug, die nicht beide gelten können: "Die Listen unten tragen keinen Anspruch auf Vollständigkeit" und, in der Behandlung jeder Abweichung, "die Liste ist bindend". Jede Abweichung wurde deshalb zum Defekt, jeder Defekt durch Ergänzen geschlossen, und keine Ergänzung konnte die nächste verhindern, weil der nächste Schritt eine andere Datei findet.

Aufgelöst ist der Widerspruch zugunsten der schwächeren Zusage, weil sie die wahre ist. Die Dateiliste ist ab jetzt eine Leseliste und eine Begründungsliste. Ihr Wert liegt in den Vermerken `(einbindend)`, `(lesend)` und `(erweitert)`, die dem Umsetzenden sagen, welchen vorhandenen Mechanismus er benutzen soll. Eine bei der Umsetzung gefundene zusätzliche Datei ist kein Defekt mehr und bekommt keinen Datensatz; sie gehört in den Sitzungsbericht und wird beim nächsten Anfassen des Plans nachgetragen. **Bindend bleibt die Verbotsseite**: nennt ein Schritt eine Kiste, die er nicht anfassen darf, ist das Überschreiten sehr wohl ein Defekt, weil es eine Entwurfsentscheidung berührt und nicht eine Suchhilfe.

Dazu zwei Herleitungsregeln, die alle vier Fälle mechanisch gefangen hätten und beide aus dem Abnahmekriterium eines Schrittes ableitbar sind:

1. **Kommando-Regel.** Nennt das Abnahmekriterium einen Tastendruck am laufenden Bündel, führt die Dateiliste `crates/krk-core/src/tasten/belegung.rs` als erweitert. Hätte S12 und S16 gefangen.
2. **Naht-Regel.** Benutzt ein Schritt einen vorhandenen Mechanismus wieder, nennt die Liste die Datei, in der dessen **Zustand** wohnt, nicht nur die, die ihn nach außen sichtbar macht; ist der Zustand privat, wächst genau eine öffentliche Naht dazu, und die Liste sagt welche. Hätte S14 und S17 gefangen, beide Male `crates/krk-ui/src/tabs.rs`.

Geschlossen: die vier Dateilisten-Defekte, die vier Abnahmekriterien (S11c, S16b, S15, 9b) und die fünf überholten Beschreibungen (S16b vier Ränge, `### Frage 2`, `### Frage 4` keymap-Zeile, `### Frage 4` und S10 "ersetzt", S13 falsche Datei).

## Teil 2: die Zusagen gegen die Messung

**Eine Schwelle hat sich bewegt.** C4 sagte Fortschritt und Abbruch "ab 100 Einträgen oder 100 MB" zu. Zwei Messungen zeigen, dass die Menge das Gemeinte in beide Richtungen verfehlt: 5.000 `rename(2)` brauchen 525 ms und stehen weit über der Wahrnehmungsgrenze, während 100 Umbenennungen nach rund 10 ms durch sind; eine 500-MB-Kopie innerhalb eines APFS-Datenträgers ist als Klon nach 0,42 ms fertig, und `copyfile(3)` ruft den Statusrückruf dort gar nicht. Neu ist eine Zeitschwelle von 150 ms, und sie ist keine Erfindung, sondern die Regel aus `### Frage 6`, nach der der Fortschritt ohnehin erscheint. **Eine Schwelle für alle fünf Operationsarten ersetzt eine Eintragszahl, eine Datenmenge und die Ausnahme, die der Klonweg sonst gebraucht hätte.** Datensatz `decisions/260804-2318_a_fortschrittsschwelle-nach-zeit-statt-nach-menge.md`.

Daraus folgt der neue Schritt **S17c**: das Stapel-Umbenennen liegt mit 525 ms über der Schwelle und wandert auf die Operationsmaschine aus S15. Der Eingriff ist Wiederverwendung, kein Neubau; Arbeitsfaden, Abbruchkennzeichen, Fortschrittskanal und die Sammlung übersprungener Einträge bringt die Maschine mit, und die Arbeit je Eintrag ist ohnehin `operation::umbenennen`. Der Abhängigkeitsgraph steht damit auf 31 Knoten und 45 Kanten, zyklenfrei, nachgerechnet.

**Nicht angetastet, und das mit Absicht:**

- **L8 bleibt bei 200 ms.** Die Sondenmessung vom 260804-1915 weist über 20 Läufe ein 95. Perzentil von 168,9 ms aus. Unsauber war allein der Weg: S16b nannte einen Unterbefehl `cargo xtask messen`, den es nicht gibt. Die Abnahme liegt jetzt bei S21, wo der Messmodus entsteht.
- **L4 bleibt bei 1000 ms.** Die Zusage hält in jeder gefahrenen Runde, gemessen zwischen 282 und 715 ms im 95. Perzentil. Unerklärt ist die Streuung. S22 bekommt dafür eine Lastbedingung, einen zweiten Vergleichslauf unter bekannter Last und eine neunte Kopfangabe im Bericht; eine Reparatur auf Verdacht am Startpfad findet nicht statt. Der Defekt bleibt bis S22 offen.
- **Die 100 ms des Abbruchkriteriums in S15 bleiben.** S15 sagt jetzt, dass der Fall mit `Uebertragungsart::ImmerBytes` geprüft wird, und warum das kein Behelf ist: innerhalb eines APFS-Datenträgers gibt es kein "mitten in einer Datei".

## Teil 3: Triage der 21 verbliebenen Defekte

| Einordnung | Zahl |
|---|---|
| Code | 8 |
| Nutzerentscheidung | 8 |
| Überholt | 1 |
| Bleibt offen | 4 |

**Code** (ein `coder` behebt es):

| Datensatz | Wo | Was |
|---|---|---|
| `260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen` | Nachzug zu S13 | Das dringendste der 21. C2 sagt das Einfügen in die Pfadeingabe ausdrücklich zu, und es ist am laufenden Bündel gemessen kaputt: ohne Menü "Bearbeiten" erreichen `cut:`, `copy:`, `paste:` und `selectAll:` kein Textfeld. |
| `260803-2317_o_der-include-str-pfad-in-schritt-11-liegt-eine-ebene-zu-hoch` | Plantext S11, `planner` | Ein Zeichen. Nicht angefasst, weil außerhalb des Auftrags. |
| `260803-2007_o_die-metadatenvorschau-aus-c6-verlangt-rechte-die-der-eintrag-nicht-traegt` | S19 | Empfohlen ist Weg 2, die Rechte für den einen angezeigten Eintrag mit einem eigenen Systemaufruf zu erheben, statt `Eintrag` um ein Feld wachsen zu lassen, das bei 100.000 Einträgen auf L3 und L10 durchschlägt. Braucht einen Satz im Plan bei S19. |
| `260803-2025_o_zwei-generationsleser-im-kern-haben-keinen-aufrufer-mehr` | freistehend | `Meldung::generation()` und `Lesevorgang::generation()` entfernen. |
| `260804-1040_o_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28` | Nachzug zu S12 | Heute rein kosmetisch, zählt trotzdem, weil `### Frage 4` die Handänderbarkeit der Datei als Grund für TOML nennt. |
| `260804-1451_o_fseventstreamschedulewithrunloop-ist-seit-macos-13-als-veraltet-gekennzeichnet` | Nachzug zu S14 | Umstellung auf `FSEventStreamSetDispatchQueue`. Zieht die Dateiliste von S14 mit, weil `CFRunLoopRef` dann herausfällt. Veraltet, nicht entfernt; nicht dringend. |
| `260804-2040_o_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen` | mit S17c | Zwei Rechnungen für dieselbe Frage zusammenlegen. S17c fasst `operation/umbenennen.rs` ohnehin an. |
| `260804-2040_o_zwei-module-des-kerns-heissen-umbenennen` | mit S17c | Umbenennung eines der beiden Module, zieht den Plantext von S17 mit. |

**Nutzerentscheidung** (eine Antwort, die der `planner` nicht treffen darf):

| Datensatz | Die Frage |
|---|---|
| `260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen` | Bekommt das Abnahmekriterium des bereits abgenommenen S6 nachträglich eine Prüfvorschrift für die sicher deklarierten `objc2`-Bindungen, oder bleibt die Grenze dort allein über die Dateiliste geführt? |
| `260804-0907_o_c10-sagt-nicht-welcher-bereich-den-fokus-haben-muss` | Wirken `shift+f3` und `opt+cmd+g` nur bei Fokus im Dateifenster, oder überall? C5 hat dieselbe Frage für seine Funktionen beantwortet, C10 nicht. Gehört als Datensatz nach `decisions/` umgetragen. |
| `260804-0907_o_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung` und `260804-1040_o_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an` | Dieselbe Frage, zweimal gestellt: bleiben Menükürzel außerhalb der Konflikterkennung aus C3, oder werden sie hineingezogen? Betroffen sind Shift+Cmd+W und das von AppKit selbst angelegte "Close All" auf Opt+Shift+Cmd+W. Zu einem Datensatz zusammenlegen. |
| `260804-1040_o_der-verworfene-ausblendbefehl-aus-c7-hat-keinen-ausloeser` | Wird das vierte Abnahmekriterium von C7 auf den Nachweis am Modell gezogen, oder bekommt der Befehl einen Weg, auch das linke Dateifenster zu treffen? |
| `260804-1309_o_die-markierung-ist-allein-an-der-farbe-erkennbar` | Bekommt die Markierung ein zweites Kennzeichen neben der Farbe, und trägt die Statuszeile künftig Zahl und Gesamtgröße der markierten Einträge? Zugänglichkeitsfrage und zugleich eine Umfangsfrage an die Statuszeile. |
| `260804-1451_o_auf-einem-netzlaufwerk-frischt-krk-fremde-aenderungen-nicht-auf` | FSEvents deckt Netzdateisysteme nicht ab. Bekommt KRK einen zweiten Auffrischungsweg für Netzpfade, was der Plan sonst ausschließt, oder engt C9 die Zusage auf lokale Dateisysteme ein? Nicht gemessen, zum Nachprüfen fehlt ein Server. |
| `260804-1649_o_die-gemeldete-eintragszahl-bedeutet-beim-verschieben-etwas-anderes-als-beim-kopieren` | Meint C4 "angefasste Einträge", die heutige Lesart, oder "ausgewählte Einträge mal ihre Inhalte"? Die zweite Lesart verlangt den Vorabdurchlauf, den `### Frage 6` ausschließt. Empfohlen ist, die heutige Lesart in C4 auszuschreiben. |

**Überholt** (hat sich seit der Meldung erledigt):

| Datensatz | Warum |
|---|---|
| `260803-2045_o_cmd-w-liegt-in-der-belegung-auf-tab-schliessen-und-im-menue-auf-fenster-schliessen` | Der Datensatz sagt selbst, er schließe mit der Umsetzung von S12. S12 trägt `[DONE]`, sein Abnahmekriterium prüft Shift+Cmd+W für den Menüeintrag und Cmd+W für den Tab, und die Belegungsdatei ist unverändert geblieben. |

**Bleibt offen** (echt, aber nichts davon ist jetzt fällig):

| Datensatz | Wann fällig |
|---|---|
| `260803-1845_o_l4-streut-zwischen-den-runden-viel-staerker-als-die-erste-messung-zeigte` | S22. Der Plan trägt die Bedingung jetzt, die zwei Läufe stehen aus. |
| `260804-1122_o_der-fokusvorbehalt-fuer-tastenbefehle-steht-nur-fuer-die-loeschtasten` | S17b. Der Vorbehalt selbst sitzt seit S13 im Ereignisabgriff; offen sind die Formulierung in C4 und die Durchsicht der Blätter samt Umbenennen-Feld. |
| `260804-1451_o_ein-verdeckter-tab-auf-einem-ausgeworfenen-datentraeger-behaelt-seinen-toten-pfad` | kein Schritt. C9 formuliert die Zusage am Dateifenster, und die trägt der sichtbare Tab. |
| `260804-1816_o_der-abbruchwunsch-erreicht-den-lauf-erst-mit-der-naechsten-meldung` | kein Schritt. Gemessen 292 bis 296 ms bei einer Kopie von 5.000 Einträgen; das Abbruchkriterium aus S15 wird auf Kernebene geprüft und hält dort. **S17c erbt die Lage**, weil es über denselben Vermittlerfaden läuft; die Spanne bleibt klein, weil das Umbenennen je Eintrag meldet. |

## Was ausdrücklich nicht getan wurde

- Kein Eingriff in `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md`.
- Kein `[DONE]`-Vermerk geändert. S17c ist ein neuer Schritt und trägt keinen.
- Kein Commit.
- Die 21 triagierten Defekte sind nicht angefasst, mit einer Ausnahme: der L4-Datensatz hat einen Nachtrag bekommen, der sagt, was der Plan jetzt trägt, und begründet, warum er trotzdem offen bleibt.

## Ein neuer Defekt aus diesem Durchgang

`issues/260804-2318_o_c4-fuehrt-neunzehn-abnahmekriterien-der-plan-sagt-achtzehn.md`. Beim Nachzählen der C4-Kriterien nach der Umformulierung der Fortschrittsschwelle: C4 führt 19 Zeilen der Form `- [ ]`, das Abnahmekriterium von S16 und der Nachzugsvermerk im Plankopf sagen achtzehn. Gegen den Stand `e43316d` gezählt sind es ebenfalls 19, die Abweichung ist also älter als dieser Durchgang. Der Vorgängerdefekt `260804-1832_c_die-zahl-der-c4-abnahmekriterien-...` hat die Zahl von sechzehn auf achtzehn gezogen und dabei um eins verfehlt. Empfohlen ist, die Zahl aus dem Kriterium zu nehmen statt sie ein drittes Mal nachzuziehen: sie zählt etwas, das anderswo steht, und ist damit dieselbe Sorte Prüfung wie die drei fest verdrahteten Zahlen, die mit S9b umgefallen sind. Gemeldet statt behoben, weil außerhalb des Auftrags.

**Offene Defekte nach diesem Durchgang: 22** (36 minus 15 geschlossene plus 1 neuer).
