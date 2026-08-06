# Ziehen die Kürzel des Hauptmenüs in die Konflikterkennung aus C3 ein, oder bleiben sie daneben?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-0907_c_fenster-schliessen-bleibt-als-einzige-belegung-ausserhalb-der-konflikterkennung.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1040_c_macos-legt-selbst-einen-zweiten-fensterschliessen-eintrag-mit-kuerzel-an.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1309_o_ohne-menue-bearbeiten-laesst-sich-in-kein-textfeld-einfuegen.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C2, C3, C7)

---

## Question

C3 sagt zu, jede Tastenkombination sei frei belegbar, und verlangt eine Konflikterkennung, die eine doppelt vergebene Kombination meldet. Beides gilt heute für die Belegungsdatei und nicht für das Hauptmenü. Zwei Kombinationen lösen bereits eine Funktion aus, ohne dass die Konflikterkennung sie sieht und ohne dass der Nutzer sie ändern kann: Shift+Cmd+W am Menüeintrag "Fenster schließen" und Opt+Shift+Cmd+W an einem "Close All", das AppKit von sich aus dazustellt. Vier weitere kämen mit dem Menü "Bearbeiten" dazu, das KRK braucht, weil ohne es kein Textfeld etwas eingefügt bekommt und C2 das Einfügen ausdrücklich zusagt. Die Frage ist deshalb jetzt fällig, weil derselbe Schritt, der das Menü baut, den blinden Fleck von zwei auf sechs vergrößerte, wenn er ihn nicht schließt.

## Options

1. **Die Menükürzel bleiben außerhalb der Konflikterkennung.** Die Auslieferungsbelegung bekommt einen Kommentar, der die betroffenen Kombinationen als vergeben ausweist, damit ein späterer Eintrag sie nicht ein zweites Mal vergibt.
   - Pros: kein Eingriff am Menü, kein neues Feld in der Belegungsdatei.
   - Cons: ein Kommentar ist keine Prüfung. Die Grundhaltung aus C3, dass jede Taste frei belegbar ist, gälte für sechs Kombinationen nicht, und der blinde Fleck wüchse mit jedem weiteren Menüeintrag.
2. **Die Menükürzel ziehen in die Belegung ein.** Jede von ihnen bekommt einen Eintrag in `resources/default-keymap.toml`, gekennzeichnet als vom Menü gehalten, und das Hauptmenü nimmt seine Kürzel von dort statt sie im Programmtext festzulegen.
   - Pros: eine Quelle für alle Kombinationen; die Konflikterkennung sieht jede; der Nutzer kann jede umbelegen; der Mechanismus steht seit S12, weil Cmd+N schon heute zugleich in der Belegung und am Menüeintrag steht.
   - Cons: ein neues optionales Feld in der Datendatei; das Menü braucht die Belegung beim Aufbau; "Close All" braucht eine eigene Antwort, weil AppKit seine Kombination selbst wählt.
3. **Kein Menü "Bearbeiten", und die Textbefehle anders herstellen.** KRK fängt Cmd+X, Cmd+C, Cmd+V und Cmd+A im Ereignisabgriff ab und ruft die Aktionen selbst auf.
   - Pros: keine Menükürzel, kein neues Feld.
   - Cons: baut die Antwortkette von Hand nach, die AppKit mitbringt, und macht aus vier Systembefehlen vier KRK-Funktionen; das ist der zweite Mechanismus neben einem vorhandenen, den die Maxime "supersimpel" ausschließt.

## Constraints

- C3 verlangt für jede Funktion aus C1 bis C7 mindestens einen umbelegbaren Tastenbefehl und eine Konflikterkennung, die eine doppelt vergebene Kombination meldet.
- C2 verlangt, dass der Nutzer in die Pfadeingabe einen Pfad **einfügen** kann und dass in jedem Textfeld alle Tasten ihre gewohnte Mac-Bedeutung behalten.
- C3 hält Cmd+C und Cmd+V für eine Dateizwischenablage einer späteren Runde frei.
- Die Auslieferungsbelegung ist eine Datendatei und gehört dem `ontocoder`; das Menü ist Code und gehört dem `coder`.

## Recommendation

Möglichkeit 2, mit "Close All" als Sonderfall, der verschwindet statt einen Eintrag zu bekommen.

---
Answered: Nutzer am 260805-0000 — Möglichkeit 2. Begründung des Nutzers: sonst wächst ein blinder Fleck, und die Grundhaltung aus C3, dass jede Taste frei belegbar ist, gälte für sechs Kombinationen nicht.

**Was daraus folgt, ausgeschrieben.**

Fünf Kombinationen bekommen einen Eintrag in `resources/default-keymap.toml`: `fenster_schliessen` auf `shift+cmd+w` sowie die vier Textbefehle `text_ausschneiden`, `text_kopieren`, `text_einfuegen` und `text_alles_auswaehlen` auf `cmd+x`, `cmd+c`, `cmd+v` und `cmd+a`. Die vier Textbefehle tragen ein neues optionales Feld `gehalten_von = "menue"`; es sagt, **wer den Tastendruck zustellt**, und nicht, was er tut. Ohne das Feld wäre eine solche Zeile von einer Zeile ohne Kommando nicht zu unterscheiden, und die gibt es aus einem anderen Grund: `belegung_ansehen` hat heute kein Kommando, weil S20 es erst baut, und bekommt eines; die vier Textbefehle bekommen nie eines.

**"Close All" bekommt keinen Eintrag, sondern verschwindet.** Eine Kombination, die AppKit selbst wählt, lässt sich weder aus der Belegung setzen noch umbelegen; ein Eintrag dafür wäre genau die Ausnahme, die dieser Entscheid beseitigt. "Fenster schließen" bekommt deshalb den eigenen Selektor `fensterSchliessen:` am Anwendungsdelegierten, so wie "Fenster einblenden" ihn seit S12 hat, und ohne `performClose:` im Menü stellt AppKit keine Zweitform dazu.

**Der Mechanismus ist nicht neu.** Cmd+N steht seit S9b und S12 zugleich als `fenster_einblenden` in der Belegung und am Menüeintrag "Fenster einblenden". Der Ereignisabgriff aus S7 sieht jeden Tastendruck vor der Menübehandlung von `NSApplication`, führt den Befehl aus und schluckt das Ereignis; der Menüeintrag zeigt das Kürzel an, und eine Umbelegung wirkt auf beide Wege. Der Modulkopf von `crates/krk-ui/src/appkit/menue.rs` schreibt das heute schon aus. Neu ist allein, dass das Menü sein Kürzel aus der Belegung nimmt, statt es als Zeichenkette im Programmtext zu führen.

**Wie Cmd+C und Cmd+V nebeneinander bestehen, und warum die Reservierung aus C3 dabei eingelöst und nicht gebrochen wird.** Ein Menükürzel und eine Belegung sind zwei Zusteller derselben Taste, und der Fokusvorbehalt aus S13 entscheidet, welcher zum Zug kommt: steht die Schreibmarke in einem Textfeld, kehrt der Abgriff sofort zurück und AppKit stellt zu; steht sie im Dateifenster, schlägt der Abgriff nach. Eine Kombination kann deshalb im Textfeld den Textbefehl tragen und im Dateifenster eine Funktion von KRK. Dieselbe Trennung trägt schon heute `cmd+left`, das im Textfeld die Schreibmarke bewegt und im Dateifenster aufsteigt.

Die spätere Dateizwischenablage braucht darum **keine zweite Belegung** auf Cmd+C: `copy:` und `paste:` gehen die Antwortkette hinunter und landen bei dem, was den Fokus hat. Wer sie am Dateifenster beantwortet, hat die Dateizwischenablage; der Menüeintrag bleibt derselbe, die Belegung bekommt keine zweite Zeile, und die Konflikterkennung hat nichts zu melden. Die Reservierung in C3 begründet sich selbst damit, die vertraute Mac-Bedeutung nicht zu überschreiben — und das Menü "Bearbeiten" überschreibt sie nicht, es stellt sie überhaupt erst her: ohne dieses Menü erreicht Cmd+V kein Textfeld, gemessen am 260804-1309 am laufenden Bündel. Geändert hat sich der Wortlaut der Reservierung, nicht ihre Sache.

**Wie die Zusage geprüft wird.** Nicht durch Aufzählen der heute bekannten Zusätze, denn eine Aufzählung veraltet mit der nächsten macOS-Version, und genau diesen Fall hat das Vorhaben mit "Close All" schon erlebt. S13c bekommt eine Befehlszeilenmarke `--menue-protokoll`, die das gebaute Hauptmenü mit allen Kürzeln ausliest; das Abnahmekriterium vergleicht diese Menge gegen `resources/default-keymap.toml`. Dieselbe Prüfung fängt auch die Einträge, die macOS an ein Menü "Bearbeiten" hängt.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C3 (drei neue Abnahmekriterien, vier neue Festlegungen), C7 (Kriterium zu Shift+Cmd+W), C10 (Festlegung zur Reservierung); `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` als neue Schritte S13b und S13c.
Implemented: `58465bf` (S13b und S13c) — die fünf Menükürzel stehen in `resources/default-keymap.toml`, das Hauptmenü nimmt seine Kürzel aus der Belegung, `--menue-protokoll` liest das gebaute Menü aus.
