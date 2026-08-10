# Wie löst der Nutzer die Ausgabe der Belegung aus?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md` (Directive und Grounding), `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_o_welche-belegung-schreibt-die-ausgabe-bei-offener-belegungsansicht.md` (hängt an dieser Antwort), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_i_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0713_i_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md` (die Zustellerregel, zitiert wo sie liegt)

---

## Question

Die Ausgabe braucht einen Auslöser, und KRK kennt dafür zwei Wege, die sich im Verhalten unterscheiden. Der eine ist eine Funktion in der Belegung, die der Ereignisabgriff ausführt; der andere ist ein Eintrag im Hauptmenü, den die Antwortkette beantwortet. Die Wahl entscheidet mehr als die Bequemlichkeit: sie bestimmt, ob die Ausgabe umbelegbar ist, ob die Konflikterkennung sie sieht, und ob sie bei offener Belegungsansicht überhaupt erreichbar ist. Sie muss vor dem ersten Planschritt fallen, weil jeder der drei Wege eine andere Datei anfasst.

Der Sachstand dazu, am Code geprüft: `resources/default-keymap.toml` führt heute 71 Funktionen, davon 65 mit einem Kommando und sechs, die das Hauptmenü zustellt (`gehalten_von = "menue"`). Solange die Belegungsansicht als Blatt steht, führt der Ereignisabgriff nichts aus und reicht jeden Tastendruck an AppKit weiter; allein `abbrechen` kommt durch (`crates/krk-ui/src/appkit/belegungsansicht.rs`, Modulkopf).

## Options

1. **Eine Funktion in der Belegung, mit einem Kommando.** Sie steht als 72. Eintrag in `resources/default-keymap.toml`, bekommt einen Wert in `Kommando`, eine Zeile in `Kommando::wirkungsbereich` und eine in `bereich_des_kommandos`.
   - Pro: die Ausgabe ist umbelegbar, wie C3 der Runde 1 es für jede Funktion zusagt, und die Konflikterkennung sieht ihre Kombination. Sie steht in der Belegungsansicht und damit in ihrer eigenen Ausgabe. Sie fügt sich in das bestehende Muster ohne Sonderfall.
   - Contra: eine ab Werk freie Kombination muss gefunden werden. Bei offener Belegungsansicht ist sie nicht auslösbar, weil der Abgriff dann nichts ausführt; wer die Belegung gerade ändert, muss das Blatt erst verlassen.
2. **Ein Eintrag im Hauptmenü.** Die Ausgabe steht als Menüzeile, mit oder ohne Kürzel.
   - Pro: sie ist ohne Tastenkenntnis auffindbar, und sie bleibt bei offener Belegungsansicht erreichbar, weil die Antwortkette weiterläuft. Für eine Funktion, die man selten und bewusst aufruft, ist ein Menüeintrag der gewöhnliche Mac-Weg.
   - Contra: eine vom Menü zugestellte Funktion bekommt nie ein Kommando (`Funktion::kommando`), und ihr Kürzel ist nur über den Belegungseintrag umbelegbar. Sie erreicht bei offener Belegungsansicht eine Belegung, die der Nutzer gerade ändert, und wirft damit die Anschlussfrage auf, welchen Stand sie schreibt.
3. **Beides: ein Belegungseintrag, den das Hauptmenü zustellt.** Genau die Form, die die sechs Textbefehle heute haben.
   - Pro: auffindbar im Menü und umbelegbar über die Belegungsansicht, mit einem einzigen Eintrag als Wahrheit über das Kürzel. Das Muster ist gebaut und geprüft.
   - Contra: dieselbe Anschlussfrage wie Möglichkeit 2, und die Funktion trägt kein Kommando, also muss die Ausführung an der Antwortkette hängen statt an der Zuleitung, die jedes andere Kommando geht.

## Constraints

- Jede Kombination, die in KRK etwas auslöst, steht in der Belegung, wird von der Konflikterkennung gesehen und ist umbelegbar (C3 der Runde 1). Jede Antwort muss das halten; Möglichkeit 2 hält es nur, wenn der Menüeintrag sein Kürzel aus der Belegung nimmt, so wie die sieben heutigen Einträge es tun.
- Zwei Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen **und** denselben Zusteller haben. Eine Kombination, die unter zwei Zustellern doppelt liegt, meldet die Konflikterkennung nicht.
- Eine ab Werk gewählte Kombination darf keine der 79 ausgelieferten doppeln und keine der ausdrücklich frei gehaltenen belegen.

## Recommendation

**Wir empfehlen Möglichkeit 1**, eine gewöhnliche Funktion mit Kommando, und zwar aus einem Grund, der die anderen beiden nicht trifft: die Ausgabe ist eine Handlung von KRK an KRKs eigenen Daten und keine Textbearbeitung in einem Feld, das die Antwortkette beantwortet. Die sechs Menüfunktionen sind Textbefehle, und sie stehen dort, weil AppKit sie ausführt und nicht KRK. Für die Ausgabe trifft das nicht zu.

Der Nachteil, bei offener Belegungsansicht nicht auslösbar zu sein, wiegt dabei leichter, als er zunächst aussieht: er macht die fünfte Frage dieses Circles gegenstandslos, statt sie zu beantworten. Wer die Belegung ändert, verlässt das Blatt, und danach schreibt die Ausgabe genau den gesicherten Stand.

**Was daran eine Auslegung ist:** ob der Nutzer die Ausgabe im Menü sucht. Wer sie einmal im Jahr braucht, findet einen Menüeintrag eher als eine Tastenkombination, die er nicht auswendig weiß. Das kann nur er sagen, und es spricht für Möglichkeit 3.

---
Answered:
Implemented:
Deferred:
Superseded by:
