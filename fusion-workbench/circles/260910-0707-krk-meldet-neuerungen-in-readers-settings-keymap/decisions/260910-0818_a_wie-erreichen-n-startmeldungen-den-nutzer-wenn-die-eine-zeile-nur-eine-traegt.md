# Wie erreichen n Startmeldungen den Nutzer, wenn die eine Zeile nur eine trägt?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@qantr.com>
**Cross-references:** `260820-2235_*_die-startmeldungen-ueberschreiben-einander-und-nur-die-letzte-erreicht-den-nutzer.md` — der Defekt, dessen Behebung diese Frage ist; `260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` — die Antwort vom 260804-0830, von der Möglichkeit 2 für diesen einen Fall abweicht; `crates/krk-ui/src/appkit/anwendung.rs` (`oberflaeche_aufbauen`, `sitzung_laden`), `crates/krk-ui/src/appkit/tabelle.rs` (`meldung_zeigen`), `crates/krk-ui/src/appkit/statuszeile.rs` (`Rang`, `Quellen`, `kurzhinweis_nachziehen`)

---

## Question

`oberflaeche_aufbauen` sammelt jede Startmeldung in einem `Vec` und stellt sie am Ende in
einer Schleife in die Statuszeile. Die Zeile hält genau eine Fenstermeldung; von n Meldungen
sieht der Nutzer die n-te. Der Defekt `260820-2235` hat das gemessen und die Behebung
ausdrücklich als Entscheidung und nicht als Ableitung stehen gelassen.

**Diese Runde bringt eine weitere Startmeldung**, die Zeile über die Neuerungen an den drei
von Hand gepflegten Ablagedateien. Sie tritt in genau diese Kollision, und die Grundlage des
Circles verlangt, dass die Runde den Defekt behebt oder ihre Zeile ausdrücklich gegen ihn
stellt.

**Die Frage ist mit dem heutigen Mechanismus nicht zu beantworten, und das ist der Kern.**
`Rang::Fenstermeldung` ist ein Fach, und n Sätze in ein Fach zu legen verliert n-1 davon,
gleich in welcher Reihenfolge. Was sich ändern muss, ist nicht die Auswahl, sondern der
Kanal.

## Options

1. **Eine Meldung aus n bauen**: die Sätze verbinden und einmal setzen.
   - Pro: Billig, ein Kanal, kein neues Bauteil, keine Abweichung von der Antwort vom
     260804-0830. Der Fall „genau eine Meldung" bleibt Wort für Wort, was er ist.
   - Contra: Es tauscht das Überschreiben gegen das Abschneiden. Die Statuszeile kürzt
     rechts; von drei Sätzen ist der dritte nicht zu sehen. Statt der letzten Meldung sieht
     der Nutzer die erste, und das ist kein Gewinn, sondern eine andere Auswahl derselben
     Größe eins.
   - Contra: Was übrig bleibt, trägt der Kurzhinweis (`kurzhinweis_nachziehen` setzt ihn,
     sobald der Text nicht in die Zeile passt). Ein Kurzhinweis verlangt die Maus und
     verlangt, dass der Nutzer ahnt, dass dort etwas steht. In einer Anwendung, deren erste
     Maxime die Tastatur ist, ist das der falsche Kanal für die einzige Auskunft über eine
     gescheiterte Ablage.
   - Contra: Die Statuszeile fällt beim nächsten Ordner- oder Tabwechsel
     (`fenstermeldung_loeschen`). Ein langer, abgeschnittener Satz hat damit dieselbe kurze
     Lebensdauer wie ein kurzer.

2. **Mehr als eine Startmeldung ist ein Blatt.** Genau eine Meldung geht unverändert in die
   Statuszeile; ab der zweiten fährt ein Blatt herunter, das alle aufführt, gebaut wie die
   Abschlussliste der übersprungenen Einträge (`Blatt::mit_schaltflaechen` und
   `erlaeuterung_setzen`, `blaetter/uebersprungen.rs`).
   - Pro: Jede Meldung erreicht den Nutzer, und zwar über die Tastatur. Das Blatt hat eine
     schließende Schaltfläche auf der Eingabetaste, und `Esc` schließt es ebenfalls.
   - Pro: Es ist die Bauform, die dieselbe Runde für die Namen auf Abruf ohnehin baut. Ein
     zweiter Anzeigeweg entsteht nicht.
   - Pro: Der Anlass ist selten. Ein Start mit mehr als einer Meldung heißt, dass an der
     Ablage mehreres zugleich nicht stimmt; das ist die Lage, in der ein Blatt richtig ist,
     und nicht der Alltag.
   - Contra: Es weicht für diesen einen Fall von der Antwort vom 260804-0830 ab
     (Möglichkeit 1: die laufenden Fehler trägt die Statuszeile, allein der fehlende
     Tastenabgriff bricht ab). Die Abweichung ist begrenzt und benannt, aber sie ist eine.
   - Contra: Ein stehendes Blatt sperrt jeden Tastenbefehl bis auf vier. Beim Start heißt
     das: der Nutzer bestätigt, bevor er arbeitet. Bei einer Meldung tritt das nicht ein.
   - Contra, und er trifft die Directive: **die Zeile dieser Runde steht dann nicht immer in
     der Statuszeile.** Kommt beim Start noch etwas anderes hinzu, wandert sie mit den
     übrigen ins Blatt. Der Wortlaut der Directive nennt die Statuszeile; er beschreibt den
     Normalfall, und die Abweichung im Mehrfachfall gehört mit entschieden.

3. **Eine Warteschlange mit Verweildauer**: die Meldungen laufen nacheinander durch die eine
   Zeile, jede für ein paar Sekunden.
   - Pro: Ein Kanal, keine Sperre, keine Abweichung von 260804-0830.
   - Contra: Ein neues Bauteil, ein Takt, den es heute nicht gibt, und die Frage, was mit der
     Schlange geschieht, wenn der Nutzer in der Zwischenzeit den Ordner wechselt und
     `fenstermeldung_loeschen` greift. Der Defekt nennt das Fehlen einer Verweildauer als
     Befund und nicht als Lücke, die zu füllen wäre.
   - Contra: Wer beim Start nicht auf die Zeile blickt, verpasst jede Meldung, die vor seinem
     ersten Blick abgelaufen ist. Das macht die Auskunft schlechter und nicht besser:
     heute steht die letzte wenigstens, bis er hinsieht.

## Constraints

- **Der Fall „genau eine Meldung" bleibt Wort für Wort, was er heute ist.** Das fordert der
  Defekt `260820-2235` ausdrücklich, und jede Antwort hält es ein.
- Kein zweiter Anzeigeweg neben Statuszeile und Blatt. Die Aufteilung steht im Kopf von
  `crates/krk-ui/src/appkit/hinweis.rs`: modal und endgültig dort, am Fenster und mit
  Antwort in `blaetter`. Ein drittes Bedienelement wäre die zweite Wahrheit, die der
  Datensatz vom 260803-2025 in seiner zweiten Randbedingung ausschließt.
- Der fehlende Tastenabgriff bleibt der eine Fehler, der mit `hinweis::zeigen` abbricht.
  Diese Frage betrifft ihn nicht.
- Die Zeitzusage L4 („Prozessstart bis bedienbare Prüfsitzung", 1000 ms) misst bis zur
  bedienbaren Sitzung. Ein Blatt, das auf eine Bestätigung wartet, steht nach dieser Zusage;
  ob es sie berührt, hängt daran, ob der Messlauf überhaupt eine Meldung erzeugt. Der
  Messmodus liest eine Prüfsitzung und keine gewöhnliche Ablage, und die Antwort auf diese
  Frage muss sagen, ob das so bleibt.

## Recommendation

**Möglichkeit 2.** Möglichkeit 1 behebt den Defekt nicht, sie verschiebt ihn: aus „der Nutzer
sieht die letzte" wird „der Nutzer sieht die erste", und das Übrige hängt an einem
Kurzhinweis, den eine tastaturgeführte Anwendung nicht als Auskunftsweg anbieten kann.
Möglichkeit 3 kostet ein neues Bauteil und macht die Auskunft für den Nutzer, der beim Start
nicht hinsieht, schlechter als heute.

Die Abweichung von der Antwort vom 260804-0830 ist der Preis, und sie ist eng: sie greift
allein den Start und allein ab der zweiten Meldung. Die Antwort von damals hat für **einen**
laufenden Fehler entschieden, und die Lage mit mehreren zugleich stand dort nicht zur Wahl.

Zum Contra, das die Directive trifft: die Startzeile dieser Runde ist eine Startmeldung wie
die anderen und geht denselben Weg. Im Normalfall — nichts ist beschädigt — ist sie die
einzige und steht in der Statuszeile, wie die Directive es sagt. Steht daneben eine Meldung
über eine beschädigte Ablagedatei, ist das Blatt der richtige Ort für beide, denn dann hat
KRK dem Nutzer zwei Dinge zu sagen und nicht eines.

---
Answered: `260905-2008-orchestrator-session.md` `## Fortsetzung 260910 — die Runde 24 wird geplant` — Möglichkeit 2: genau eine Startmeldung geht unverändert in die Statuszeile, ab der zweiten fährt ein Blatt herunter, das alle aufführt, gebaut wie die Abschlussliste der übersprungenen Einträge; die begrenzte Abweichung von der Antwort vom 260804-0830 und der Fall, in dem die Zeile dieser Runde mit ins Blatt wandert, sind mitentschieden; ruled by user, Kai Stalmann <kai@stalmann.org>.
