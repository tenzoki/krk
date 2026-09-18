# Welche Anwendungen zeigt „Öffnen mit" bei einer gemischten Mehrfachauswahl?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

---

## Question

Der Auftrag sagt zum Untermenü zweierlei, und beides steht im Singular gegen den Plural:

- die Einträge sind „die Anwendungen, die das System für den Dateityp **des betroffenen Eintrags** liefert";
- „Mehrfachauswahl: **alle** markierten Einträge gehen an dieselbe gewählte Anwendung, wie im Finder."

Solange genau ein Eintrag betroffen ist, fallen beide Sätze zusammen. Sind mehrere markiert und tragen sie **verschiedene** Typen — eine `.txt` neben einer `.png` —, gibt es keinen einen Dateityp mehr, nach dem zu fragen wäre, und die Frage wird entscheidbar erst durch eine Regel.

Gebaut ist heute die Regel, die dem Wortlaut am nächsten steht: gefragt wird der **erste** betroffene Eintrag, also der oberste auf dem Schirm, und die gewählte Anwendung bekommt danach alle. Sie steht als reine Funktion `oeffnungsbezug` in `crates/krk-ui/src/kommandos/kontextmenue.rs` und hat genau einen Rufer, `DateifensterQuelle::eigene_kontexteintraege_anfuegen`; eine andere Antwort ist ein Austausch dieser einen Funktion.

## Options

1. **Der erste betroffene Eintrag entscheidet** (heute gebaut). Das Untermenü zeigt die Anwendungen seines Typs, und alle betroffenen Einträge gehen an die gewählte.
   - Pro: folgt dem Wortlaut des Auftrags; kostet **einen** Gang zu LaunchServices je Rechtsklick, gleich wie viele Einträge markiert sind; das Untermenü ist nie leer, wenn für den angeklickten Eintrag etwas angemeldet ist.
   - Contra: bei gemischter Markierung bietet es Anwendungen an, die das System für die übrigen Einträge nicht nennt. Was die Anwendung mit ihnen tut, entscheidet sie; KRK erfährt es nicht, denn die Übergabe liefert nichts zurück.
2. **Nur die Anwendungen, die das System für jeden betroffenen Eintrag nennt** (Schnittmenge).
   - Pro: jeder Eintrag des Untermenüs hält, was er verspricht — die gewählte Anwendung ist für jede übergebene Datei angemeldet.
   - Contra: ein Gang zu LaunchServices **je markiertem Eintrag**, bei jedem Rechtsklick und auf dem Hauptfaden; bei dreißig markierten Einträgen dreißig Abfragen, bevor das Menü aufgeht. Dazu bleibt bei gemischter Markierung häufig nichts übrig, und der Nutzer liest „das System nennt keine Anwendung", obwohl es für jeden einzelnen Eintrag welche nennt.
3. **Die Vereinigung über alle betroffenen Einträge.**
   - Pro: nichts fehlt; jede Anwendung, die irgendeinen der Einträge öffnen kann, steht da.
   - Contra: teuer wie Möglichkeit 2 und mit einer schlechteren Zusage als Möglichkeit 1 — die Liste wird mit jeder zusätzlichen Markierung länger, und die Wahrscheinlichkeit, dass eine gewählte Anwendung mit einem Teil der Übergabe nichts anfangen kann, steigt.

## Constraints

- **Der Finder ist hier kein Maßstab, der nachgeschlagen wäre.** Was er bei gemischter Auswahl anzeigt, ist in dieser Sitzung nicht gemessen worden; der Auftrag zitiert ihn für die **Übergabe** („alle gehen an dieselbe"), nicht für die Zusammenstellung der Liste.
- Die Kosten sind gerechnet und nicht gemessen: `URLsForApplicationsToOpenURL:` ist ein Aufruf je Eintrag, und die Möglichkeiten 2 und 3 machen daraus einen je markiertem Eintrag. Wie lange ein solcher Aufruf auf dem Referenzgerät braucht, ist ungemessen. `speculation:` dass dreißig davon vor dem Aufklappen eines Kontextmenüs spürbar wären.
- Keine der drei Möglichkeiten ändert etwas an der Übergabe selbst: sie geht in jedem Fall an **eine** Anwendung und trägt **alle** betroffenen Einträge.
- Eine Antwort tauscht `oeffnungsbezug` gegen eine Funktion, die statt eines Bezugseintrags eine Liste von Anwendungslisten zusammenführt; der Rufer und die Hülle `crate::appkit::oeffnenmit` bleiben, wie sie sind.

## Recommendation

Möglichkeit 1 vorerst beibehalten, und zwar so lange, wie die Kosten der beiden anderen ungemessen sind. Der Auslöser, der die Frage wieder aufmacht, ist eine gemischte Mehrfachauswahl, bei der die angebotene Anwendung mit den übrigen Einträgen erkennbar nichts anfängt.
