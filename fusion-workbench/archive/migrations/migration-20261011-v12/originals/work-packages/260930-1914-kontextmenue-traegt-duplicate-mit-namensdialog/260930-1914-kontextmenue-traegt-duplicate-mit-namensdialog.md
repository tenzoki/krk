# Das Kontextmenü der Dateiliste trägt Duplicate mit Namensdialog

---
**Domain:** code
**Status:** done
**Claim:** 6c11b1f2 — Kai Stalmann <kai@stalmann.org>, 260930-1914
**Mode:** autonomous
**Active spec/plan:** 260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md (Plan)
**Filed by:** user, Kai Stalmann <kai@stalmann.org>

---

## Directive

"implementiere eine neue Dateiopertion als funktion in rechte maustaste menu: duplicate (nur wenn eine Datei markiert ist), sie soll die datei duplizieren, verlangt aber erst in einem kleinen dialog einen neuen namen, wobei der alte in dem dialog vorausgefüllt ist. überscchreibt keine datei: dateinamen-dialog öffnet erneut mit fehlermeldung. der dialog hat 2 buttons 'duplicate' und 'cancel'." Auf die Frage, ob die Arbeit als eigene Runde angelegt wird: "ja, eigene runden,autonom abarbeiten". Auf die Frage, ob ein einzelner Ordner als Datei zählt: "nur gewöhnliche dateien". Auf die Frage, ob Menüeintrag und Knöpfe englisch heißen, obwohl die Blätter von KRK sonst deutsch beschriftet sind: "ja, mach das auch auf deutsch". Erreicht ist das, wenn das Kontextmenü der Dateiliste den Eintrag führt, er bei genau einer markierten gewöhnlichen Datei wirkt, der Dialog den alten Namen vorausgefüllt zeigt und die zwei deutsch beschrifteten Knöpfe trägt, das Duplikat unter dem neuen Namen im selben Ordner entsteht, und ein schon vergebener Name keine Datei überschreibt, sondern den Dialog mit einer Fehlermeldung erneut öffnet.

## Abschluss

Geschlossen am 260930-2107 als `done`, Commits `3aa7370..b81a284` (Plan `3aa7370`; Schritt 1 `11b9a03`, Schritt 2 `64e6d0b`, Schritt 3 `6e00ff7`, Schritte 4 und 5 `10d2128`; Durchsicht `260930-2058-reviewer-duplizieren-im-kontextmenue-mit-namensblatt.md` in `b81a284`, ihre zwei Prosabefunde dort behoben). `make check` endet auf `b81a284` grün. Vier Defekte hat die Arbeit angelegt und geschlossen, alle vier Prosa (`260930-1950_*`, `260930-2009_*`, `260930-2056_*`, `260930-2057_*` unter `issues/`). **Gebaut, nicht abgenommen:** die neun Punkte des Abnahmelaufs am Bündel (`## Testing Strategy` des Plans) verlangen KRK im Vordergrund und sind Nutzerarbeit. Die zwei offenen Fragen des Plans (Eintrag ausgrauen statt melden; nur den Namensstamm auswählen) sind nicht entschieden und halten nichts auf.

Die Haltebedingungen aus `## Where this work stops` des Plans `260930-1928_*_plan-kontextmenue-traegt-duplizieren-mit-namensblatt.md` sind unter **Mode:** autonomous nicht gestellt worden und stehen hier wörtlich:

1. Die Arbeit ist fertig, wenn die Schritte 1 bis 5 `[DONE]` tragen und `make check` auf dem Commit des letzten Schritts grün endet.
2. Der Eintrag wirkt allein auf genau eine gewöhnliche Datei (N1): gehalten am Baum von der Tafel zu `duplikatbezug` (Schritt 3) und von den Kernproben zu Ordner, Verknüpfung und Röhre (Schritt 1).
3. Menüeintrag, Schaltflächen und Meldungen sind deutsch und tragen Umlaute (N2): gehalten von den Wortlautproben aus Schritt 3.
4. Kein Weg des Duplizierens ersetzt oder entfernt einen vorhandenen Eintrag: gehalten von den Kernproben unter jeder Konfliktregel und von `das_duplizieren_kennt_keinen_weg_der_einen_eintrag_entfernt` (Schritt 1).
5. Nach keinem Commit dieser Arbeit steht ein Menüeintrag da, der nichts tut: der Eintrag und seine Wirkung kommen in Schritt 3 zusammen.
6. Ein Abnahmelauf gegen die zehn Zeitzusagen aus C8 ist nicht geschuldet, solange drei Bedingungen halten. Erstens fügt kein Schritt dem Hauptfaden einen Dateisystemaufruf hinzu; das hält `der_duplikatweg_stellt_auf_dem_hauptfaden_keinen_systemaufruf`. Zweitens ändert kein Schritt den Start, das Lesen und Sortieren eines Ordners, den Tab- und Fensterwechsel oder die Vorschau, also die Wege von L2 bis L7 und L10; der Menübauer bekommt einen Eintrag mehr je Rechtsklick, und keine Zusage misst den Rechtsklick. Drittens bleibt der Weg, den L8 und L9 am Kopieren messen, im Verhalten unverändert: `kopieren::datei` ruft nach Schritt 1 dieselben Systemaufrufe in derselben Folge, und jede vorhandene Probe des Kopierens besteht mit unveränderter Erwartung. Fügt ein Schritt dem Hauptfaden einen Dateisystemaufruf hinzu oder ändert er eine Probe des Kopierens in ihrer Erwartung, hält dieser Schritt an, und die Frage nach dem Abnahmelauf geht an den Nutzer. L4 und L7 bleiben unabhängig davon auf der Liste der späteren Messrunde, auf der sie schon stehen.
7. Der Abnahmelauf am Bündel ist Nutzerarbeit und nicht Teil dieser Arbeit, weil er KRK im Vordergrund verlangt; kein Agent fährt ihn. Seine Punkte führt `## Testing Strategy`.
8. Diese Arbeit liefert nicht aus: ein `./release.sh` braucht einen eigenen, ausdrücklichen Auftrag des Nutzers.
9. Das Arbeitspaket bleibt `claimed`, bis der Nutzer es schließt; dieser Plan setzt `**Status:** done` nicht.
