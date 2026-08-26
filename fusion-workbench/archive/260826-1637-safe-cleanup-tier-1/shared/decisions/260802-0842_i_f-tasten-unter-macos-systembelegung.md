# Wie erreicht KRK die Tasten F3 bis F8, die macOS ab Werk selbst belegt?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1105_c_directive-zeile-widerspricht-loeschantwort.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1330_c_abnahmegeraet-hat-keine-physische-f-tastenreihe.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1417_o_directive-zeile-sagt-freie-funktionstasten-zu.md`, `spikes/fn-tasten/messung-A.txt`

---

## Question

Die Vorbelegung des Circles legt die Norton-Funktionen auf F3 bis F8. Auf einem Mac lösen genau diese Tasten ab Werk Systemfunktionen aus, etwa Mission Control, Tastaturbeleuchtung und Lautstärke. Eine Anwendung sieht das nackte F5 nur, wenn der Nutzer in den Systemeinstellungen "F1, F2 usw. als Standard-Funktionstasten verwenden" aktiviert hat oder beim Drücken die Fn-Taste hält. Ohne eine Festlegung liefert KRK eine Vorbelegung aus, die auf einem unveränderten Mac schlicht nicht reagiert, und der erste Eindruck ist ein Werkzeug, dessen dokumentierte Tasten nichts tun. Die Frage muss vor dem Aktivierungs-Spec beantwortet sein, weil sie die ausgelieferte Standardbelegung und den Text des Erststart-Dialogs bestimmt.

## Options

1. **Fn+F3 bis Fn+F8 als ausgelieferte Belegung** — KRK belegt die Kombination mit Fn, die auf jedem Mac ohne Systemänderung ankommt.
   - Pro: funktioniert sofort nach der Installation, ohne dass der Nutzer etwas umstellt. Keine Kollision mit Mission Control und den Medientasten.
   - Contra: zwei Tasten statt einer, das widerspricht dem Norton-Gefühl. Auf externen PC-Tastaturen ohne Fn-Taste nicht erreichbar.

2. **Nacktes F3 bis F8, mit Hinweis beim Erststart** — KRK belegt die reinen F-Tasten und erklärt beim ersten Start, wie der Nutzer die Systemeinstellung umstellt.
   - Pro: exakt die Norton-Belegung, eine Taste pro Funktion. Nutzer, die Norton-Kürzel wollen, haben die Einstellung oft ohnehin schon aktiv.
   - Contra: verlangt einen Eingriff in die Systemeinstellungen, der global für alle Anwendungen wirkt. Wer den Hinweis wegklickt, hat ein Werkzeug mit toten Tasten.

3. **Beides ab Werk belegt** — sowohl F3 bis F8 als auch Fn+F3 bis Fn+F8 lösen dieselbe Funktion aus, je nachdem, was bei diesem Nutzer ankommt.
   - Pro: funktioniert in beiden Systemzuständen, ohne dass der Nutzer etwas wissen muss.
   - Contra: die Belegungstabelle trägt doppelte Einträge, was die freie Konfigurierbarkeit unübersichtlicher macht. Beim Umbelegen muss der Nutzer zwei Zeilen ändern statt einer.

4. **Norton-Belegung auf andere Tasten legen** — die Funktionen aus F3 bis F8 wandern auf Kombinationen, die macOS frei lässt.
   - Pro: keine Kollision, keine Systemeinstellung, kein Erklärbedarf.
   - Contra: verfehlt die ausdrückliche Vorgabe des Nutzers, F3 bis F8 wie bei Norton zu belegen.

## Constraints

- Jede Taste bleibt frei konfigurierbar. Die Antwort betrifft nur die ausgelieferte Vorbelegung, nicht die Freiheit des Nutzers, sie zu ändern.
- Die Norton-Zuordnung selbst steht fest: F3 Ansehen, F4 Bearbeiten, F5 Kopieren, F6 Verschieben und Umbenennen, F7 Ordner anlegen, F8 Löschen.
- Löschen ist zusätzlich auf Shift+Delete vorbelegt und bleibt damit auch dann erreichbar, wenn F8 auf einem System nicht ankommt.

Die Abschnitte `## Question`, `## Options`, `## Constraints` und `## Recommendation` geben den Stand bei der Ablage am 260802-0842 wieder und werden nicht nachgeführt. Der letzte Punkt oben ist überholt: Shift+Delete ist seit dem 260802-1105 ab Werk unbelegt. Maßgeblich sind die Antwort des Nutzers und der Nachtrag weiter unten.

## Recommendation

Option 3 löst das Problem ohne Rückfrage an den Nutzer und ohne Eingriff in dessen Systemeinstellungen. Der Nachteil, zwei Einträge in der Belegungstabelle zu führen, lässt sich entschärfen, indem die Konfigurationsoberfläche beide als eine Zeile mit zwei Auslösern darstellt. Diese Empfehlung ist eine Abwägung, keine geprüfte Aussage; die Entscheidung liegt beim Nutzer.

## Antwort des Nutzers

Der Nutzer hat am 260802-1105 Möglichkeit 1 gewählt: ausgeliefert wird ausschließlich die Fn-Kombination, Fn+F3 bis Fn+F8. Die nackten Funktionstasten bleiben unbelegt. Begründung des Nutzers: die Belegung funktioniert damit auf jedem Mac ohne Systemeingriff.

Die Empfehlung des Shapers, Möglichkeit 3, ist damit abgelehnt. Der Vorteil von Möglichkeit 1 gegenüber Möglichkeit 3 liegt in der Belegungsansicht: sie führt je Funktion eine Zeile statt zweier. Der im Contra genannte Fall einer externen PC-Tastatur ohne Fn-Taste bleibt bestehen und ist über die freie Konfigurierbarkeit aufzufangen, nicht über die Vorbelegung.

## Nachtrag 260802-1409: die Messung, und was sie an der Lage ändert

Die Antwort des Nutzers bleibt gültig, ihre Begründung und ihre Formulierung nicht. Drei Befunde haben die Frage seit dem 260802-1105 verschoben.

**Die Wahl zwischen Möglichkeit 1 und Möglichkeit 3 bestand technisch nie.** Gemessen wurde am 260802-1137 auf dem Abnahmegerät `MacBookPro15,1`, macOS 15.7.7, Systemeinstellung "F1, F2 usw." aus. Fn+F3, Fn+F5 und Fn+F8 kommen in einer gewöhnlichen Anwendung im Vordergrund als gewöhnliche `keyDown`-Ereignisse an, mit den Tastencodes 99, 96 und 100, den Zeichen U+F706, U+F708 und U+F70B und gesetztem Modifikator `function` (0x800000). Beleg: `spikes/fn-tasten/messung-A.txt`, Ereignisse #03 bis #05. Der Modifikator `function` weist keine körperlich gedrückte fn-Taste nach; AppKit setzt ihn bei jeder Taste aus dem Funktionstasten-Zeichenbereich. KRK kann Fn+F3 und ein nacktes F3 deshalb nicht unterscheiden. Die ausgelieferte Belegung ist der Tastencode, nicht die Fingerhaltung, und "die nackten F-Tasten bleiben frei" beschreibt keinen erreichbaren Zustand. Möglichkeit 1 und Möglichkeit 3 fallen in der Umsetzung zusammen; der Vorteil, den die Antwort des Nutzers für Möglichkeit 1 nennt, die eine Zeile je Funktion, stellt sich damit von selbst ein.

**Zur Lesart der Messdatei.** Die Selbstauswertung in `spikes/fn-tasten/messung-A.txt` meldet Frage 2 als beantwortet und ist darin falsch. Das rohe Protokoll zeigt bei Ereignis #08 ein `flagsChanged geändert=+function` unmittelbar vor dem zweiten Abschnitt und bei #12 das zugehörige `-function`; der Nutzer musste fn halten, weil sein Gerät ohne fn keine F3 erzeugt. Die korrigierte Auswertung derselben Rohdaten steht in `spikes/fn-tasten/messung-A-neuauswertung.txt`. Ob die nackten Funktionstasten auf einem Gerät mit echter Tastenreihe ankommen, ist unverändert ungemessen, ebenso die Wirkung der Systemeinstellung. Beides bindet den Plan nicht, weil KRK den Tastencode belegt und kein Abnahmekriterium am Ergebnis hängt; die Begründung steht im Spec in C3.

**Das Abnahmegerät trägt einen Touch Bar.** Dort heißt die Funktionstaste nicht "Taste drücken", sondern "fn halten und auf Glas tippen". Für eine Anwendung, deren erste Maxime die Tastatursteuerung ist, verfehlt das den Zweck. Der Nutzer hat am 260802-1409 entschieden: die Norton-Reihe bleibt auf den Funktionstasten, und jede dieser Funktionen trägt ab Werk zusätzlich ein Mac-typisches Cmd-Kürzel. Zwei Wege ab Werk auf dieselbe Funktion, in derselben Zeile der Belegungsansicht. Seine Alltagstastatur ist die eingebaute Tastatur eines Apple-Silicon-MacBooks mit echter Funktionstastenreihe; der Touch Bar steht also nur bei der Abnahme im Weg, nicht bei der täglichen Arbeit.

Damit ist die Antwort vom 260802-1105 erweitert, nicht ersetzt. Die gewählte Möglichkeit 1 trägt weiterhin, sie heißt nur anders: KRK belegt die Funktionstasten F3 bis F8 und schreibt sie auch so, ohne "Fn+" davor. Die konkreten Cmd-Kürzel und die Regel, nach der sie gewählt wurden, stehen im Spec und werden hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderlaufen.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`, Abschnitt C3 — Möglichkeit 1 gewählt und am 260802-1409 um einen zweiten Weg erweitert: KRK belegt die Funktionstasten F3 bis F8 als Tastencode, ohne Systemeingriff, und legt jede Norton-Funktion zusätzlich auf ein Cmd-Kürzel. Beleg für die Zustellung der Tastenereignisse: `spikes/fn-tasten/messung-A.txt`.
Implemented: `6b4fb2d` (S7, Tastencodes ohne Systemeingriff) und `d1a8ab1` (S9) — F3 bis F8 als Tastencode belegt, jede Norton-Funktion zusätzlich auf einem Cmd-Kürzel, `resources/default-keymap.toml:101-123`.
Deferred:
Superseded by:
