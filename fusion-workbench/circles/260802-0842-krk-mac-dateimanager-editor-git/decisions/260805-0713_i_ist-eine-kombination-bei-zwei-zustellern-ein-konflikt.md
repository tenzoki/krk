# Ist dieselbe Kombination bei zwei Funktionen ein Konflikt, wenn verschiedene Zusteller sie tragen?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260805-0637_o_cmd-a-liegt-schon-auf-alle-markieren-und-s13b-vergibt-es-ein-zweites-mal.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_a_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C2, C3), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S13, S13b, S13c)

---

## Question

Der Entscheid vom 260805-0000 lässt fünf Menükürzel in `resources/default-keymap.toml` einziehen. Vier davon sind in der Datei frei, das fünfte nicht: `cmd+a` trägt seit S9 die Funktion `alle_markieren`, und der Nutzer hat diese Belegung am 260803-2110 durchgesehen und angenommen. Der `ontocoder` hat den Konflikt bei der Ausführung von S13b bemerkt, vier der fünf Einträge geschrieben und den fünften offengelassen. Die Frage ist damit fällig, bevor S13b zu Ende gehen und S13c anfangen kann, denn die eingebettete Auslieferungsbelegung stürzt bei einem Konflikt beim ersten Zugriff ab und nimmt jeden Test und jeden Programmstart mit.

Dahinter steht keine Frage über eine einzelne Taste, sondern eine über die Konflikterkennung selbst: Sie prüft heute allein, ob zwei Funktionen dieselbe Kombination tragen. Seit die Belegung Funktionen führt, die nicht der Ereignisabgriff ausführt, sondern das Hauptmenü zustellt, ist das zu grob.

## Options

1. **`alle_markieren` zieht um, `cmd+a` geht an den Textbefehl.** `ctrl+a` ist frei, und der Plan hatte die Funktion ohnehin dort vermutet.
   - Pros: die Konflikterkennung bleibt, wie sie ist; kein Eingriff in Code, den S13c sonst nicht anfasste.
   - Cons: ändert eine Belegung, die der Nutzer am 260803-2110 angenommen hat. Die Nachbarschaft der Markierungsbefehle zerfällt, weil `markierung_aufheben` auf `shift+cmd+a` steht und als einziger bei `cmd` bliebe. Und `cmd+a` ist auf dem Mac die erwartete Kombination für "alles auswählen", in einer Liste wie in einem Textfeld.
2. **Der Textbefehl bekommt kein `cmd+a`.** Das Menü "Bearbeiten" trägt "Alles auswählen" ohne Kürzel.
   - Pros: keine Änderung an vorhandenen Belegungen, keine Änderung am Code.
   - Cons: bricht das Abnahmekriterium von C3, das für alle vier Textbefehle einen Eintrag in der Belegung verlangt, und lässt genau den blinden Fleck offen, den der Entscheid vom 260805-0000 schließen wollte. `cmd+a` wählte in einem Textfeld dann gar nichts aus.
3. **Die Konflikterkennung lernt den Fokusvorbehalt.** Zwei Funktionen sind nur dann ein Konflikt, wenn sie im selben Fokuszustand erreichbar sind.
   - Pros: löst den Fall an seiner Wurzel statt an einer Taste; `cmd+a` markiert in der Liste alle Einträge und wählt im Eingabefeld den Text, wie auf dem Mac üblich; keine vom Nutzer angenommene Belegung ändert sich; der Fall `cmd+c` wirft sich nicht ein zweites Mal auf, wenn eine spätere Runde die Dateizwischenablage baut.
   - Cons: greift in `crates/krk-core/src/tasten/belegung.rs` ein, und das Abnahmekriterium von S13c verlangt heute das Gegenteil.

## Constraints

- C3 verlangt, dass die Auslieferungsbelegung konfliktfrei ist und dass KRK eine doppelt vergebene Kombination meldet, statt sie stillschweigend zu überschreiben.
- C3 verlangt seit dem 260805-0000, dass jede Kombination, die in KRK etwas auslöst, in der Belegung steht, von der Konflikterkennung gesehen wird und umbelegbar ist.
- C2 verlangt, dass in jedem Textfeld alle Tasten ihre gewohnte Mac-Bedeutung behalten.
- Die 39 vom `ontocoder` gewählten Kombinationen sind vom Nutzer angenommen; sie ohne ihn zu ändern verstieße gegen jene Annahme (`decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`).
- Die Maxime "supersimpel" schließt eine Sonderregel je Fall aus. Was hier entsteht, muss eine Regel sein.

## Recommendation

Der Datensatz aus der Ausführung von S13b legt die drei Wege mit ihren Kosten aus und empfiehlt bewusst keinen: Möglichkeit 3 bricht als einzige keine Zusage, widerspricht aber dem Abnahmekriterium von S13c, und ob dieser Widerspruch eine Formulierung oder eine Sache ist, gehört dem Nutzer.

---
Answered: Nutzer am 260805 — Möglichkeit 3. Begründung des Nutzers: `cmd+a` soll in der Liste alle Einträge markieren und im Eingabefeld den Text auswählen, wie auf dem Mac üblich; `resources/default-keymap.toml` behält `alle_markieren` auf `cmd+a`, weil er diese Belegung am 260803 durchgesehen und angenommen hat und weil sie die erwartete ist.

### Die Regel, scharf formuliert

**Zwei Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen und denselben Zusteller haben.**

Der **Zusteller** einer Funktion steht in ihrem Feld `gehalten_von`: ohne das Feld stellt der Ereignisabgriff aus S7 zu, mit dem Wert `menue` stellt das Hauptmenü zu. Der Fokusvorbehalt aus S13 teilt jeden Tastendruck genau einem der beiden zu. Steht die Schreibmarke in einem Textfeld, kehrt der Abgriff sofort zurück, und AppKit stellt über das Menü zu. Steht sie sonst irgendwo, schlägt der Abgriff in der Belegung nach. Eine Funktion ist deshalb in genau einem Fokuszustand erreichbar, und ihr Zusteller sagt, in welchem. Zwei Funktionen können einander nur begegnen, wenn beide im selben Fokuszustand erreichbar sind, und das heißt: wenn ihr Zusteller derselbe ist.

Aus der einen Regel folgt alles, ohne eine Liste von Fällen:

| Fall | Zusteller | Konflikt? |
|---|---|---|
| `alle_markieren` und `text_alles_auswaehlen` auf `cmd+a` | Abgriff und Menü | nein |
| zwei vom Menü gehaltene Funktionen auf einer Kombination | Menü und Menü | ja |
| zwei Funktionen des Dateifensters auf einer Kombination | Abgriff und Abgriff | ja |

Die dritte Zeile ist jeder Fall, den die Prüfung bisher überhaupt gesehen hat; an ihr ändert sich nichts. Die zweite ist der Grund, aus dem "kein Konflikt zwischen Menü und Dateifenster" zu grob wäre: zwei Menüeinträge auf derselben Kombination sind sehr wohl einer, und die grobe Fassung ließe sie durch.

### Dieselbe Regel gilt für den Nachschlag, und ohne diese Hälfte trägt sie nicht

Der Ereignisabgriff läuft nur außerhalb eines Textfeldes. **Er darf deshalb nur Funktionen sehen, die er selbst zustellt**, also solche ohne `gehalten_von`. Ohne diese Hälfte hinge das Verhalten an der Reihenfolge der Einträge in der Datei: `Belegung::nachschlag` liefert den ersten Treffer, `alle_markieren` steht heute vor `text_alles_auswaehlen`, und in der Datei des Nutzers bestimmt seine eigene Reihenfolge, welcher zuerst kommt. Stünde der Textbefehl vorn, fände der Abgriff eine Funktion ohne Kommando, reichte den Tastendruck weiter, und `alle_markieren` wäre still tot. Die Regel überlebt eine Umsortierung nur, wenn der Nachschlag den Zusteller kennt.

### `Wirkungsbereich` ist kein zweiter Zusteller

S18 gibt `Kommando` die Eigenschaft `Wirkungsbereich` mit den Werten `Dateifenster`, `Leiste` und `Ueberall` (`decisions/260805-0000_a_welcher-bereich-den-fokus-fuer-die-zwischenablage-befehle-haben-muss.md`). Sie sagt, **wo ein bereits zugestellter Befehl wirkt**, nicht wer ihn zustellt, und sie darf in die Konflikterkennung nicht einfließen. Zwei vom Abgriff zugestellte Funktionen mit verschiedenem Wirkungsbereich bleiben ein Konflikt: der Nachschlag findet nur eine von beiden, und die andere wäre still tot. Beide Eigenschaften in die Prüfung zu nehmen wäre die zweite Regel neben der ersten, also genau das Dickicht, das die Maxime "supersimpel" ausschließt.

### Was die Regel nicht abdeckt

`inference:` Bleibt eine Kombination im Dateifenster unbelegt, reicht der Abgriff sie weiter, und die Antwortkette von AppKit kann sie beantworten. `NSTableView` bringt eine eigene Auswahlaktion mit; hebt der Nutzer die Belegung von `alle_markieren` auf, könnte `cmd+a` über den Menüeintrag "Alles auswählen" trotzdem alle Zeilen markieren. Gemessen ist das nicht. Es ist keine Ausnahme von der Regel, sondern die Frage, was eine leere Belegung zusagt; sie ist erst fällig, wenn die Belegungsansicht aus S20 das Aufheben einer Belegung anbietet.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C3 (drei Abnahmekriterien nachgezogen, eine Festlegung neu); `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` bei S13b (der falsche Absatz, der fünfte Eintrag, das Abnahmekriterium) und S13c (die Regel in beiden Hälften, das Abnahmekriterium in beide Richtungen).
Implemented: 58465bf — S13c hat die Regel an vier Stellen in `crates/krk-core/src/tasten/belegung.rs` umgesetzt.

**Der Hash ist am 260810 im Abgleich nachgetragen.** Er stand bis dahin als Platzhalter `<Hash offen — trägt der Orchestrator nach>` und war der einzige der 42 Datensätze mit dem Marker `_i_`, dessen Beleg sich nicht auflösen ließ. Ermittelt über `git log -S 'gehalten_von' -- crates/krk-core/src/tasten/belegung.rs` und `git log -S 'der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege' -- crates/krk-core/tests/belegung.rs`: beide nennen genau einen Commit, `58465bf feat(ui): S13b und S13c, Menue "Bearbeiten" und die Zustellerregel`. Damit sind das Feld `gehalten_von` und die fünf im Absatz darüber genannten Proben demselben Commit zugeordnet.

`Funktion` trägt das Feld `gehalten_von: Option<String>` mit einem Leser daneben, `Eintrag` dasselbe mit `#[serde(default, skip_serializing_if = "Option::is_none")]`, und `impl From<&Belegung> for Belegungsdatei` reicht es auf dem Rückweg durch. Die Regel greift in `Belegung::konflikte` (Vergleich des Zustellers neben der Kombination), in `Belegung::zuweisen` (dasselbe für die Umbelegung) und in `Belegung::nachschlag` (überspringt jede Funktion mit `gehalten_von`).

**Eine vierte Stelle kam hinzu, die der Entscheid nicht nennt:** `Funktion::kommando` liefert `None`, sobald `gehalten_von` gesetzt ist. Ohne sie hinge die Zusage "eine vom Menü gehaltene Funktion liefert kein Kommando" daran, dass `Kommando::KENNUNGEN` die vier Textbefehle zufällig nicht führt. Messbar wird sie an `fenster_schliessen`, der einzigen Funktion, die seit S13c zugleich ein Kommando hat und in einer Nutzerdatei ein `gehalten_von` tragen könnte.

Belegt durch fünf Prüfungen in `crates/krk-core/tests/belegung.rs`, Abschnitt "Der Zusteller (Schritt 13c)": `cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt`, `zwei_funktionen_desselben_zustellers_auf_einer_kombination_bleiben_ein_konflikt`, `die_umbelegung_vergleicht_den_zusteller_ebenso`, `eine_unbelegte_menuefunktion_nimmt_ihre_kombination_ohne_konflikt_an`, `der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege` und `der_rueckweg_ueber_die_belegungsdatei_traegt_den_zusteller_mit`. Am laufenden Bündel gegengeprüft am 260805-0753: Cmd+A markiert im Dateifenster weiter alle Einträge (drei von drei, abgelesen an der Vorschau des Stapel-Umbenennens) und wählt im Textfeld den Text aus.
