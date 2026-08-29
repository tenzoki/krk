# Was erwartet C2.1 beim Einfügen in ein Terminal: den Namen oder den Pfad?

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `planning/260829-0005_*_spec-cmd-c-und-cmd-x-legen-dateiverweise-ab.md` (C2.1, Diagramm der Directive „Terminal, Textfeld: die Namen"); `planning/260829-0006_*_plan-cmd-c-und-cmd-x-legen-dateiverweise-ab.md` (Schritt 9, Entscheidung 3); `reviews/260829-0051-coderev-runde-22-dateiverweise-in-der-zwischenablage.md` (Thema 4, Messung der Sorten).

---

## Question

C2.1 verlangt: ein Eintrag kopiert, `cmd+v` in einem Terminal, „es erscheint der Name, ohne Ordner". Nach der Runde trägt die Ablage je Eintrag `public.file-url`, und der Ablageserver leitet daraus `NSFilenamesPboardType` ab (gemessen am 260829 mit einem Swift-Programm gegen eine benannte Ablage; der Text `public.utf8-plain-text` liegt daneben am ersten Eintrag). `inference:` Terminal.app liest bei dieser Lage die Dateiverweise vor dem Text und fügt den Pfad mit Shell-Maskierung ein, wie nach einem Kopieren im Finder; am laufenden Terminal ist das in dieser Runde nicht geprüft. Trifft es zu, ist C2.1 am Bündel nicht erfüllbar, solange die Verweise abgelegt werden, und das Kriterium prüft dann gegen die falsche Erwartung. Die Frage steht vor dem Abnahmelauf (Schritt 9), damit der Lauf im Terminal weiß, was er sehen soll. Der Code ist von der Antwort nicht berührt: das Ziel wählt die Sorte, so steht es in A3.

## Options

1. **C2.1 wird auf ein Textfeld ohne Dateiverständnis umformuliert** (TextEdit, ein Suchfeld, der Editor von KRK); das Terminal fällt aus dem Kriterium, und ein Satz im Spec sagt, dass ein Ziel mit Dateiverständnis den Pfad nimmt, wie beim Finder.
   - Pros: kein Code, kein Verlust an Finder-Fähigkeit; das Kriterium prüft, was die Namenszeilen wirklich leisten.
   - Cons: das Diagramm der Directive nennt das Terminal; es ist nachzuziehen.
2. **C2.1 bleibt, und der Abnahmelauf hält das Ergebnis im Terminal als Auskunft fest**, ohne es als erfüllt oder verfehlt zu werten.
   - Pros: nichts zu ändern vor dem Lauf.
   - Cons: ein Kriterium, das weder erfüllt noch verfehlt werden kann, ist keines; die Zählung „40" trüge eines zu viel.
3. **KRK legt die Namen als einzige Textsorte und keine Verweise ab, wenn das Ziel ein Terminal ist.**
   - Pros: keine.
   - Cons: KRK kennt das Ziel nicht; das ist die unentscheidbare Frage aus dem Plankopf, und die Antwort wäre ein Kommando mit zwei Bedeutungen, das A3 ausdrücklich ausschließt.

## Constraints

Die Ablageseite bleibt, wie sie gebaut ist: je Eintrag ein Datei-`NSURL`, daneben die Namen (A3, C5.1). Der Entscheid vom 260811-1610 für die Pfadkopierer gilt fort. Kein Kriterium wird gestrichen, ohne dass die Zählung im Spec nachzieht.

## Recommendation

Option 1, und die Prüfung am Bündel entscheidet, ob die Erschließung trägt: zeigt das Terminal den Namen, bleibt C2.1, wie es ist, und dieser Datensatz wird mit dem Befund geschlossen; zeigt es den Pfad, wird C2.1 auf ein Textfeld umformuliert und das Diagramm zieht nach.

**Answered:** 260829, vom Nutzer im Abnahmelauf: das Terminal fügt den Namen ein, C2.1 hält wie geschrieben; der Code bleibt.
