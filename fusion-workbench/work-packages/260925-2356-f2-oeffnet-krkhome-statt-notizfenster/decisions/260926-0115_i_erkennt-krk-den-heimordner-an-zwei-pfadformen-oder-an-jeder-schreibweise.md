# Erkennt KRK `~/krkhome` an zwei Pfadformen oder an jeder Schreibweise des Ordners?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0007_*_spec-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0107-zweitlesung-plan-f2-krkhome.md, 260926-0007_*_ist-der-ort-krkhome-fest-oder-einstellbar.md

---

## Question

Die erste Fassung des Plans erkannte den Ordner, indem sie beide Seiten im Augenblick der Frage über `realpath(3)` auflöste: `~/krkhome` und den gefragten Pfad. Die Zweitlesung des Plans hat gezeigt, dass diese Frage in `Tabliste::lesen_starten` auf dem Hauptfaden stünde, bei jedem Ordnerwechsel und für jeden Tab, den die Sitzung beim Start wiederherstellt. Auf einem hängenden Netzlaufwerk blockiert `realpath` des Tab-Ordners die Ereignisschleife, und das träfe die Zusagen L1, L4, L6 und L9. Der Plan fragt deshalb am gefragten Pfad nichts mehr ab: `Heimordner` hält zwei Formen des Ordners, die geschriebene (`<benutzerverzeichnis>/krkhome`) und die aufgelöste, und vergleicht den gefragten Pfad als Text mit beiden.

Damit erkennt KRK den Ordner über den Verweis `~/krkhome` und über sein Ziel, aber nicht über eine **dritte Schreibweise**: einen zweiten Verweis an anderer Stelle, der auf denselben Ordner zeigt, oder einen Pfad, dessen Bestandteile selbst Verweise sind, die die aufgelöste Form auflöst und der Tab-Pfad nicht. Das Kriterium C2.6 des Spec verlangt, dass die Erkennung „den Ordner über seinen aufgelösten Ort fest[hält] und nicht über den Pfadtext“; seine Probe (Verweis und Ziel geben dieselbe Antwort) hält mit zwei Formen, sein Wortlaut für die gefragte Seite nicht.

Die Frage bindet über diesen Plan hinaus, weil jede Regel, die am Ordner hängt (F2, Vorschau, Editor, „steht immer“, der Inhaltsfilter), dieselbe Erkennung fragt, und weil ein einstellbarer Ort, den der Spec als spätere Arbeit vermerkt, dieselbe Frage erbt.

## Options

1. **Zwei Pfadformen, Vergleich als Text.** Die geschriebene Form steht fest; die aufgelöste entsteht beim Bau aus `read_link` am Verweis im Benutzerverzeichnis, ohne das Ziel zu berühren, und bei jedem F2 über `canonicalize`, nachdem `bereitstellen` am Ziel ohnehin gearbeitet hat. `ist` und `sonderdatei` stellen keinen Systemaufruf.
   - Pros: kein Dateisystemaufruf auf dem Hauptfaden je Lesevorgang; keine der zehn Zeitzusagen berührt; eine Stelle, rein und ohne Fenster prüfbar.
   - Cons: eine dritte Schreibweise wird nicht erkannt; dort fehlt `.secrets.txt` bei ausgeblendeten Verstecken, und Vorschau und Editor zeigen `notes.txt` und `tasks.txt` wie gewöhnliche Textdateien. Der Wortlaut von C2.6 wird auf „über den Verweis und über sein Ziel“ eingeschränkt.
2. **Gerät und Inode, erhoben im Lesefaden.** Der Lesefaden, der den Ordner ohnehin öffnet, erhebt Gerät und Inode am offenen Deskriptor und gibt sie mit dem ersten Stapel zurück; `Heimordner` hält Gerät und Inode von `~/krkhome`, erhoben bei F2. Vorschau und Editor fragen am offenen Deskriptor der Datei, den sie ohnehin haben.
   - Pros: jede Schreibweise wird erkannt, der Wortlaut von C2.6 hält vollständig.
   - Cons: die Ausnahme „steht immer“ kann erst mit dem ersten Stapel greifen und nicht vor ihm; der Leseweg des Verzeichnislesers bekommt eine zweite Rückgabe, und Vorschau und Editor müssen die Frage vom Pfad an den Deskriptor verlegen. Das ist der teurere Umbau und trifft den Teil des Baums, den die Runden 10 und 11 mit Kindproben abgesichert haben.

## Constraints

- Kein `realpath`, `stat` oder `canonicalize` am Tab-Ordner auf dem Hauptfaden (Zweitlesung des Plans, Frage 7; Zusagen L1, L4, L6, L9).
- Die Erkennung steht an einer Stelle, und F2, Vorschau, Editor und „steht immer“ fragen nur sie (C2.6).
- Über den Verweis `~/krkhome` und über sein Ziel gibt die Erkennung dieselbe Antwort (C2.6, C4.1, C7.11).

## Recommendation

Wir empfehlen Möglichkeit 1. Der Weg, auf dem der Nutzer den Ordner erreicht, ist F2, und F2 öffnet die geschriebene Form; wer den Ordner über das Ziel des Verweises ansteuert, trifft die aufgelöste. Eine dritte Schreibweise entsteht nur, wenn der Nutzer sie selbst anlegt, und dann zeigt KRK die Dateien wie jede andere und verliert nichts. Möglichkeit 2 ist genauer und bleibt der Weg, wenn sich die Lücke im Gebrauch zeigt; der Modulkopf von `heimordner/mod.rs` nennt sie. Der Plan baut auf Möglichkeit 1; wählt der Nutzer sie, zieht der `requirements-designer` den Wortlaut von C2.6 nach.

---
Answered: dieser Datensatz `## Recommendation` — Möglichkeit 1, zwei Pfadformen als Text verglichen; der Wortlaut von C2.6 im Spec wird nachgezogen; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: 87d5ff6 — zwei Pfadformen als Text verglichen, ohne Systemaufruf; ergänzt durch ffd0d7f für .secrets.txt
