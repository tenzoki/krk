# Ein Scratchpad, das per Taste mittig erscheint und sich selbst sichert

**Filed by:** k1

KRK braucht einen Notizzettel: eine automatisch persistierte Datei, die auf Tastendruck erscheint, mittig über den anderen Bereichen eingeblendet in einem editierbaren Fenster. Der Nutzer pastet dort kurz etwas hinein, bearbeitet es, legt es ab. `Esc` beendet, gespeichert wird von selbst.

**Warum das eine eigene Runde ist und keine Kleinigkeit.** Eine Fläche „mittig über den anderen Bereichen" ist auf dem Mac entweder ein Blatt am Hauptfenster oder ein eigenes Fenster, und davon hängt ab, ob KRKs Tastenbefehle dahinter noch wirken. Genau diese Mechanik hat die achte Runde am 260813 angefasst: `zulaessigkeit::zulaessig` fragt seither, ob das Schlüsselfenster KRKs Hauptfenster oder ein daran hängendes Blatt ist, und `blatt_steht` hält bei stehendem Blatt jeden Befehl außer den erlaubten an. Der Zettel muss sich in diese eine Regel einordnen, statt eine zweite daneben zu stellen.

**Offene Punkte, die eine Klärungsrunde beantworten müsste.** Wo die Datei liegt (`~/Library/Application Support/KRK/` führt heute die Ablage). Was „automatisch" heißt: bei jedem Zeichen, nach einer Ruhezeit, beim Schließen. Was `Esc` bei ungesicherten Änderungen tut, wo es sonst überall der Weg heraus ist. Ob es einen Zettel gibt oder mehrere. Ob der Zettel den bestehenden Editorkern aus `krk-core/src/text/` benutzt oder eine eigene, schlankere Fläche ist. Und welche Taste ihn holt, wo die Belegung 82 Funktionen mit 88 Kombinationen führt.

**Related:** `crates/krk-core/src/text/` (der Editorkern); `crates/krk-ui/src/kommandos/zulaessigkeit.rs` (die eine Zulässigkeitsregel); `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags` (die Runde, die sie um die Schlüsselfensterfrage erweitert hat)
