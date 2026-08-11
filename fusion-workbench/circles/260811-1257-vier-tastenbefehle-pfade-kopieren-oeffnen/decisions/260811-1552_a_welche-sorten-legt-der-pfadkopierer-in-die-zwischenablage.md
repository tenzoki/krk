# Welche Sorten legt der Pfadkopierer in die Zwischenablage: nur Text oder Text und Dateiverweis?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:**
`crates/krk-ui/src/appkit/zwischenablage.rs` (Modulkopf, `lesen`),
`crates/krk-core/src/zwischenablage.rs` (`deuten`),
`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/decisions/260811-1258_a_was-kopiert-der-pfadkopierer-bei-stehender-markierung.md`

---

## Question

Die beiden Kopierbefehle schreiben zum ersten Mal in die Zwischenablage. Ein
`NSPasteboard` trägt seinen Inhalt in mehreren Sorten zugleich, und welche
davon KRK ablegt, entscheidet, was ein Einfügen in einer anderen Anwendung
ergibt. Die Frage ist beim Schreiben zu stellen und nicht später: KRK liest
selbst zwei Sorten, und es liest den Dateiverweis **vor** dem Text
(`crates/krk-ui/src/appkit/zwischenablage.rs`, `lesen`).

Der Unterschied ist im Finder am größten. Legt KRK allein Text ab, fügt ein
Cmd+V im Finder nichts ein. Legt KRK daneben einen Dateiverweis ab, kopiert
derselbe Tastendruck die **Datei** in den angezeigten Ordner. Aus "Pfad
kopieren" wird damit für den Finder "Datei kopieren", und das ist eine andere
Handlung als die, die der Befehl heißt.

## Options

1. **Nur Text (`NSPasteboardTypeString`).** Ein Pfad je Zeile, sonst nichts.
   - Pro: Der Befehl heißt "Pfad kopieren" und legt einen Pfad ab. Ein Einfügen
     in ein Terminal, in einen Editor oder in ein Textfeld ergibt genau den
     Pfad. Die Mehrzahl braucht keinen zweiten Weg: mehrere Zeilen in einer
     Zeichenkette sind derselbe Mechanismus wie eine.
   - Contra: Ein Cmd+V im Finder ergibt nichts. Wer die Datei meint, muss sie
     im Finder selbst kopieren.
2. **Text und Dateiverweis (`NSPasteboardTypeFileURL`).**
   - Pro: Ein Einfügen im Finder legt die Datei ab, ein Einfügen in einem
     Textfeld den Pfad. Beide Erwartungen sind bedient.
   - Contra: Der Befehl tut dann je nach Ziel zwei verschiedene Dinge, und das
     zerstörerische von beiden ist das unsichtbare. Ein Dateiverweis trägt
     außerdem **einen** Pfad; mehrere verlangen `writeObjects:` mit mehreren
     `NSURL`, also einen zweiten Schreibweg neben dem für Text. Und KRKs
     eigener Sprung aus der Zwischenablage (C10 der Runde 1) nähme dann den
     Verweis statt des Textes, also bei mehreren Pfaden den ersten und
     wortlos nicht die übrigen.

## Constraints

- KRKs eigene Auswertung liest den Dateiverweis vor dem Text
  (`crates/krk-ui/src/appkit/zwischenablage.rs`, `lesen`). Was KRK schreibt,
  liest KRK also selbst wieder, und die Rangfolge gilt dabei unverändert.
- `krk_core::zwischenablage::deuten` nimmt eine Zeichenkette und liefert
  **einen** Pfad. Bei mehreren kopierten Pfaden ergibt der Sprung aus der
  Zwischenablage deshalb einen Pfad, den es nicht gibt, und die Statuszeile
  meldet das. Das gilt unter beiden Möglichkeiten und ist keine Größe, die
  zwischen ihnen entscheidet.
- Die eine Hülle um `NSPasteboard` bleibt eine. Eine zweite daneben ist unter
  beiden Möglichkeiten ausgeschlossen.

## Recommendation

Möglichkeit 1, nur Text. Der Befehl heißt nach dem, was er kopiert, und ein
Einfügen, das in einer Anwendung einen Pfad und in einer anderen eine Datei
ergibt, ist eine zweite Bedeutung, die der Nutzer nicht sieht, bevor sie
eingetreten ist. Die Mehrzahl gibt den Ausschlag: die Antwort auf
`260811-1258_*` sagt einen Pfad je Zeile zu, und ein Dateiverweis trägt diese
Zusage nicht.

Der Spec `260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md`
trägt diese Möglichkeit als Vorbelegung, damit er nicht an der Frage hängt.
Wer sie umstößt, ändert ein Abnahmekriterium von C2 und eines von C1.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Nur Text.** Nutzerantwort am 260811-1610, der Empfehlung und der Vorbelegung des
Specs folgend.

Was der Nutzer kopiert, ist ein Pfad, und was er einfuegt, ist ein Pfad — an jedem Ziel dasselbe.
Ein `Cmd+V` im Finder legt damit **keine** Datei ab, sondern schreibt den Pfad als Text, wohin er
gerade schreibt.

**Der Preis der anderen Wahl ist damit vermieden**, und er stand nicht im Bequemen: mit einem
Dateiverweis daneben taete derselbe Befehl je nach Ziel etwas anderes, und KRKs eigener Sprung
aus der Zwischenablage naehme den Verweis **vor** dem Text — der kopierte Pfad waere dann nicht
mehr das, was ankommt. Eine Sorte in der Zwischenablage heisst: eine Bedeutung.
