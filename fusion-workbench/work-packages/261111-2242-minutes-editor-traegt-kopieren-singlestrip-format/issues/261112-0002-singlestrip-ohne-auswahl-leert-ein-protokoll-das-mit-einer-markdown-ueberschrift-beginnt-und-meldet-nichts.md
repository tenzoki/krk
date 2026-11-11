Singlestrip ohne Auswahl leert ein Protokoll, das mit einer Markdown-Überschrift beginnt, und meldet nichts
---
Ein Klick auf „Singlestrip“ vor einer Protokolldatei ohne Auswahl rechnet am ganzen Stand. Singlestrip liest ein `#` am Wortanfang als Shell-Kommentar und wirft ihn samt allen Zeilen danach weg. Eine Markdown-Datei beginnt in aller Regel mit `# Titel`, also bleibt vom ganzen Protokoll der leere Text. Die Statuszeile sagt dazu nichts, und `HowTo.md` nennt die Folge für ein Protokoll nicht.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-2242-minutes-editor-traegt-kopieren-singlestrip-format.md, 261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md, 261112-0002-reviewer-protokollknoepfe-vor-der-auslieferung.md

**Schwere:** Medium. Auf der Platte geht nichts verloren: `cmd+z` nimmt die Umformung zurück, und in die Datei kommt sie erst mit `cmd+s`. Wer danach sichert, ohne hinzusehen, hat das Protokoll geleert.

Beleg, am Quelltext gelesen und nicht im laufenden Bündel ausgeführt:

- `crates/krk-ui/src/appkit/editor.rs`, `protokoll_umgeformt`: bei `auswahl.length == 0` ist der Ausschnitt `(0, stand.len())`.
- `crates/krk-core/src/text/glaetten.rs`, `befehl`: meldet `zeile_lesen` für eine Zeile `Zeilenende::Kommentar(stelle)` und wartet kein Heredoc, hängt der Rumpf `zeile[..stelle]` an und verlässt die Schleife mit `break` („Sonst gehoert alles danach zum Kommentar und faellt mit“). `am_wortanfang` sagt für `stelle == 0` ja. Für den Stand `# Treffen\n\nText\n` ist das Ergebnis damit der leere Text.
- `crates/krk-ui/src/appkit/editor.rs`, `protokoll_umformen`: nach einem Umbau ist die Antwort `None`, eine Meldung gibt es allein für „nichts zu ändern“.
- Die Hauptfadenprobe `die_reihe_des_protokolls_folgt_datei_und_form` legt ihr Protokoll selbst als `# Treffen\n\nText\n` an; die Gestalt ist die gewöhnliche.
- `HowTo.md`, `### Protokolle in minutes/`: „Singlestrip und Format wirken auf die Auswahl, und ohne Auswahl auf den ganzen Text, wie unter ‚Singlestrip und Format‘ beschrieben.“ Der Abschnitt dort erklärt den Kommentar an einem Shell-Befehl und nennt keine Überschrift.

In der Quicknote gilt dieselbe Regel, dort steht aber ein kopierter Befehl und kein Markdown. Der Auftrag verlangt dieselbe Wirkung an beiden Orten; welche der Richtungen unten gilt, entscheidet der Nutzer.

Richtungen, eine genügt:

1. `HowTo.md` sagt unter `### Protokolle in minutes/`, dass Singlestrip ohne Auswahl ein Protokoll ab der ersten Überschrift entfernt und für eine markierte Befehlszeile gedacht ist.
2. `protokoll_umformen` meldet nach einem Umbau, der den Text auf null Zeichen bringt, einen Satz in der Statuszeile mit dem Hinweis auf `cmd+z`.
3. Singlestrip verlangt vor einem Protokoll eine Auswahl und meldet ohne sie, dass nichts markiert ist.

Abnahme: ein Klick auf „Singlestrip“ ohne Auswahl vor `# Treffen\n\nText\n` lässt den Nutzer nicht ohne Auskunft vor einem leeren Editor stehen. Entweder nennt `HowTo.md` die Folge an der Stelle des Protokolls, oder eine Probe hält die Meldung beziehungsweise die verlangte Auswahl.
