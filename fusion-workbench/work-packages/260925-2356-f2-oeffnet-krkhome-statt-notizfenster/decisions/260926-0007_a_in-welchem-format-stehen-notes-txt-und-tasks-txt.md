# In welchem Format stehen die Einträge in notes.txt und tasks.txt?

---
**Domain:** code
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260925-2356-f2-oeffnet-krkhome-statt-notizfenster.md, 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0007_*_wo-werden-notizen-und-aufgaben-bearbeitet-und-wo-gerendert.md

---

## Question

Die Directive verlangt „in strukturierter Form gespeicherte" Notizen (Thema und Notiz) und Aufgaben (Text und ein Erledigt-Kästchen), beide in Dateien mit der Endung `.txt` unter `~/krkhome/`. Sie sagt nicht, wie ein Eintrag in der Datei aussieht. Die Antwort legt fest, ob die Dateien außerhalb von KRK lesbar und von Hand änderbar bleiben, ob eine Notiz mehrere Zeilen tragen kann, was mit einer von Hand beschädigten Zeile geschieht, und ob Stufe 1 des Spec ohne eigenen Editor überhaupt brauchbar ist: bis Stufe 3 und 4 bearbeitet der Nutzer die Dateien im bestehenden Editor als Text. Dieselbe Form gilt für den entschlüsselten Inhalt von `.secrets.txt`, weil die Directive dort „gleiche Darstellung der Einträge" verlangt.

## Options

1. **Markdown-nahe Textform.** Eine Notiz beginnt mit einer Zeile `## <Thema>`, darunter folgt der Notiztext über beliebig viele Zeilen bis zur nächsten solchen Zeile. Eine Aufgabe ist eine Zeile `- [ ] <Text>`, erledigt `- [x] <Text>`; die Reihenfolge der Zeilen ist die Reihenfolge der Aufgaben.
   - Pros: in jedem Textprogramm lesbar und änderbar; mehrzeilige Notizen gehen; die Aufgabenform ist die verbreitete Schreibweise für Aufgabenlisten, die auch GitHub und viele Notizprogramme lesen. Stufe 1 ist damit schon ein brauchbarer Zustand: der Nutzer schreibt die Form im Editor von Hand. Die gerenderte Vorschau aus der Runde 6 kennt Markdown bereits (Inferenz: ob sie Aufgabenkästchen darstellt, prüft der Planer; heute ruft sie den Zerleger ohne Zusatzoptionen auf).
   - Cons: eine von Hand geschriebene Zeile, die keiner der beiden Formen folgt, braucht eine Regel (Vorschlag im Spec: sie bleibt beim Sichern unverändert stehen und wird im Sondereditor als Text ohne Struktur gezeigt). Ein Thema, das selbst mit `## ` beginnt oder einen Zeilenumbruch trägt, ist nicht darstellbar. Die Endung `.txt` lässt andere Programme die Markdown-Form nicht erkennen.
2. **Tabulatorgetrennte Zeilen.** Je Eintrag eine Zeile: `<Thema><Tab><Notiz>` beziehungsweise `[ ]<Tab><Aufgabe>` und `[x]<Tab><Aufgabe>`.
   - Pros: kommt der Tabellenvorstellung der Directive („<topic>  <note>") am nächsten; jede Tabellenkalkulation öffnet die Datei; eine Zeile ist ein Eintrag, die Zerlegung ist trivial.
   - Cons: eine Notiz kann keinen Zeilenumbruch tragen, oder er muss als Zeichenfolge (etwa `\n`) maskiert werden, und dann ist die Datei von Hand nicht mehr bequem zu schreiben. Ein Tabulator ist in den meisten Editoren unsichtbar; ein versehentlich getipptes Leerzeichen statt Tab zerstört einen Eintrag still.
3. **TOML in einer `.txt`-Datei.** `[[notiz]]` mit `thema = "…"` und `text = """…"""`, `[[aufgabe]]` mit `text` und `erledigt = true/false`.
   - Pros: die Form, in der KRK alle übrigen eigenen Dateien führt; mehrzeiliger Text und Sonderzeichen sind sauber geregelt; eine beschädigte Datei lässt sich nach dem bestehenden Muster beiseitelegen.
   - Cons: von Hand mühsam und fehleranfällig (Anführungszeichen, Maskierung); die Endung `.txt` behauptet eine Form, die die Datei nicht hat; Stufe 1 ist ohne eigenen Editor kaum benutzbar.

## Constraints

- Die Dateinamen `notes.txt`, `tasks.txt` und `.secrets.txt` stehen in der Directive und werden von keiner Möglichkeit geändert.
- Kodierung UTF-8 ohne Bytefolgenmarke, Zeilenende `\n`: das ist die Zusage, unter der der bestehende Editor seinen Stand hält (`crates/krk-core/src/text/datei.rs`, Modulkopf).
- Eine von Hand geänderte Datei darf beim nächsten Sichern durch KRK keinen Inhalt verlieren, den KRK nicht versteht.

## Recommendation

Wir empfehlen Möglichkeit 1. Sie ist die einzige, unter der Stufe 1 ohne Sondereditor ein brauchbarer Zustand ist, sie trägt mehrzeilige Notizen, und die Datei bleibt dem Nutzer lesbar, auch wenn KRK einmal nicht mehr da ist. Der Preis ist eine Regel für fremde Zeilen; der Spec schlägt sie vor (fremde Zeilen bleiben unverändert stehen), und sie ist dieselbe Haltung, die das Projekt beim Beiseitelegen beschädigter Dateien schon einnimmt: nichts, was der Nutzer geschrieben hat, verschwindet still.

---
Answered: dieser Datensatz `## Options` und 260926-0017-zweitlesung-spec-f2-krkhome.md — Möglichkeit 1, Markdown-nahe Textform, mit den vier Regeln der Zweitlesung (fremde Zeile wandert mit der Aufgabe darüber, Notizzeile mit `## ` wird abgewiesen, Aufgaben großzügig gelesen und nur berührte Zeilen neu geschrieben, `## ` begründet); ruled by user, Kai Stalmann <kai@stalmann.org>
