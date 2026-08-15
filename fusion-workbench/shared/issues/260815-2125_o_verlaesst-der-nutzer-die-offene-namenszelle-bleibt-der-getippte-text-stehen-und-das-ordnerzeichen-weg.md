Verlaesst der Nutzer die offene Namenszelle, bleibt der getippte Text stehen und das Ordnerzeichen weg

---

Das Umbenennen an Ort und Stelle aus C4 kennt drei Ausgaenge, aber nur zwei sind
verdrahtet. Return schickt die Aktion `umbenennungBeendet:`, Escape laeuft in
`abortEditing`. **Der dritte Ausgang ist der Fokusverlust** — ein Klick in eine andere
Zeile, in das andere Dateifenster, in eine andere Anwendung —, und er schickt weder das
eine noch das andere. Die Bearbeitung endet, `controlTextDidEndEditing:` kommt, eine
Aktion kommt nicht, und niemand zeichnet die Zeile neu.

Die Folge stand schon vor dem Ordnerzeichen im Baum und ist seit dem 260815 sichtbarer:
der getippte Text bleibt in der Zelle stehen, obwohl nicht umbenannt worden ist, und ein
Ordner steht dabei zusaetzlich ohne seinen Schraegstrich da. Beides bis zum naechsten
Zeichendurchgang der Zeile.

---

**Schwere:** mittel. Kein Datenverlust und keine falsche Dateioperation — umbenannt wird
nichts, was der Nutzer nicht mit Return bestaetigt hat. Falsch ist allein die Anzeige, und
sie behauptet eine Umbenennung, die nicht stattgefunden hat.
**Gefunden von:** coder, beim Einbau des Ordnerzeichens (Nutzerentscheid 260815-2058)
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `DateifensterQuelle::umbenennung_beenden`
und `Namensfeld`
**Domain:** code

## Was gemessen ist

Am 260815 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem Hauptfaden, an einer
`NSTableView` mit bearbeitbarer Zelle:

| Ausgang der Bearbeitung | Aktion | `controlTextDidEndEditing:` | `abortEditing` |
|---|---|---|---|
| Return (`insertNewline:`) | ja, **nach** der Meldung | ja | nein |
| Escape (`cancelOperation:`) | nein | **nein** | ja |
| Fokusverlust (`makeFirstResponder:` auf die Tabelle) | **nein** | ja | nein |

`sendsActionOnEndEditing` der Zelle steht auf 0, und das ist die Vorgabe des Systems.

## Warum es der Auftrag vom 260815 nicht mitbehoben hat

Zwei Gruende, beide inhaltlich:

1. **Der Nutzerentscheid nennt den Fall nicht.** Zugesagt sind drei Dinge: der Beginn ohne
   Zeichen, Return liest den getippten Text, Escape stellt die Anzeigeform her. Der
   Fokusverlust ist keines davon.
2. **Die naheliegende Behebung waere eine Reihenfolgefalle.** Ein Zeichendurchgang in
   `controlTextDidEndEditing:` kommt **vor** der Aktion; `umbenennung_beenden` laese danach
   die Anzeigeform statt des getippten Textes und wollte einen Ordner auf `Name/`
   umbenennen. Wer den Fall dort behebt, muss die Textbewegung aus `userInfo` auswerten
   (`NSTextMovement`), also eine Fallunterscheidung einziehen, oder die Verdrahtung des
   Endes umbauen.

## Die Frage, die vorher zu entscheiden ist

**Was soll ein Fokusverlust tun?** Die Antwort ist nicht offensichtlich und aendert das
Verhalten von C4:

- **Verwerfen wie Escape.** Dann ist die Zelle wieder die Anzeigeform, und ein
  Versehen kostet nichts. Der Finder macht es nicht so.
- **Uebernehmen wie Return.** Dann benennt ein Klick daneben um. Das ist das Verhalten des
  Finders und von ForkLift, und es ist die groessere Aenderung: `sendsActionOnEndEditing`
  auf 1, und die Umbenennung laeuft ohne ausdrueckliche Bestaetigung.

Solange das nicht entschieden ist, bleibt der Zustand am Code, wie er ist.
