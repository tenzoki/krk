# Drei Prosabefunde der Bündel D und E

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Baum bei Beginn:** `f79f964`
**Abnahme:** `make check` — exit 0

## Auftrag

Drei Datensätze aus den Durchsichten der Bündel D und E, alle drei reine Prosa. Kein
Eingriff in Logik, Signatur oder Probe; die Tafel und der Rumpf der Rückschritt-Regel
bleiben unangetastet.

## Der Kern: zwei Lesarten derselben sicherheitsrelevanten Regel

Der Baum trug bis zu diesem Durchgang beide Aussagen über den falschen Zweig der
Rückschritt-Regel nebeneinander. `crates/krk-ui/src/kommandos/rueckschritt.rs:26-33` sagt
seit `522cf51`, dass dieser Zweig nichts mehr wegräumt, sondern fragt; vier Doc-Kommentare
unter `crates/krk-ui/src/appkit/` sagten weiter, er räume Dateien weg. Wer den einen liest,
hält die Regel für den Schutz vor einem Datenverlust; wer den anderen liest, für den Schutz
vor einer lästigen Nachfrage. Die vier Stellen sind auf den Stand des Modulkopfs
nachgezogen, mit dessen Begründung und ohne eine zweite daneben: die Rückfrage macht die
Fallunterscheidung **milder und nicht überflüssig**, weil eine Frage, die auf jeden
berichtigten Vertipper aufgeht, weggeklickt statt gelesen wird.

## Geändert

### `crates/`

| Datei und Zeile (nach der Änderung) | Was jetzt dasteht |
|---|---|
| `krk-ui/src/appkit/anwendung.rs:4468-4474` | „**Der eine Zweig dieser Runde, dessen falsche Haelfte die Loeschrueckfrage aufgehen laesst**", dazu zwei Sätze Begründung: seit dem 260817 räumt diese Hälfte nichts mehr, sie fragt, und die Unterscheidung ist dadurch milder und nicht überflüssig |
| `krk-ui/src/appkit/anwendung.rs:2892-2897` | dieselbe Überschrift auf dem `Kommando::InPapierkorb`-Zweig von `kommando_ausfuehren`; die zweite Hälfte mit: „alles andere geht unveraendert in die Rueckfrage vor dem Papierkorb" |
| `krk-ui/src/appkit/anwendung.rs:2659-2661` | die Folge einer Divergenz der beiden Fassungen ist kein Löschen mehr, sondern eine Rückfrage auf einen berichtigten Vertipper |
| `krk-ui/src/appkit/ereignisse.rs:298-301` | dieselbe Korrektur am Doc-Kommentar von `Anschlag::ist_nackter_rueckschritt` |
| `krk-ui/src/appkit/anwendung.rs:4541` | mitgezogener fünfter Kandidat: „Wie vor der Runde 10 (C1.16, C1.20): der Weg in den Papierkorb, seit dem 260817 mit seiner Rueckfrage davor" statt „Wie vor dieser Runde" |
| `krk-ui/src/kommandos/loeschwarnung.rs:167` | Perfekt statt Futur: „`operationen::loeschfrage`, der Wortlaut des endgueltigen Loeschens, ist mit jenem Loeschweg weggefallen." |

### `fusion-workbench/`

`history/260817-2356-coder-e15-kommentare-und-claude-md.md`: die Zahl der nach dem
Durchgang verbleibenden Zeilen von 33 auf 34 berichtigt, und die drei schon beim Schreiben
um eine Zeile zu niedrigen Zitate (`loeschbestaetigung.rs:73` → `:74`, `:172` → `:173`,
`:179` → `:180`).

## Gemessen

```
$ grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l
      34            # am Arbeitsbaum und an 522cf51 gleich; der Bericht sagte 33
$ grep -rniE "raeumte? .{0,20}(Dateien )?weg|wegraeumt" --include="*.rs" crates/krk-ui/src/appkit
                    # von den fünf Stellen keine mehr; übrig bleiben Statuszeile,
                    # Tabelle und ereignisse.rs:307, das seit Bündel D richtig steht
$ make check
      exit 0
```

Zwei weitere Suchen über den ganzen Baum haben keine zusätzliche Stelle derselben Klasse
gefunden: ein Futur-Muster über „fällt/kommt … weg/dazu" und eine Suche nach
„ohne Rückfrage" und „ohne Nachfrage". Die Treffer der zweiten betreffen andere
Gegenstände (Anlegen, Lesezeichen, Belegungsausgabe) oder sind datierte Rückblicke wie
`anwendung.rs:4441`.

## Meinen die Datensätze 2 und 3 dieselbe Zeile?

Bei einer Zeile ja, im Übrigen nein. Beide treffen sich an
`crates/krk-ui/src/kommandos/loeschwarnung.rs:167`: Datensatz `260817-2243` meldet den
hängenden Symbolverweis im Futur, Datensatz `260818-0026` meldet dieselbe Zeile als in die
falsche Klasse gelegt. Behoben ist eine Stelle, und beide Datensätze verweisen aufeinander.
Der übrige Gegenstand von `260818-0026` ist ein anderer, nämlich die Zahl 33 gegen 34 im
Sitzungsbericht; er ist mit abgetragen.

Statt `loeschwarnung.rs:167` in eine andere Klasse umzuordnen, ist die Zeile richtiggestellt
worden. Damit trifft die Einordnung „datierter Rückblick, richtig" zu, und die Aussage
„keine der Zeilen trifft mehr eine falsche Aussage" hält.

## Geschlossene Datensätze

- `issues/260818-0025_c_four-doc-comments-still-say-the-wrong-branch-of-the-backspace-rule-deletes-files.md`
- `issues/260818-0026_c_the-sweep-of-step-15-reports-33-remaining-lines-and-the-search-returns-34.md`
- `issues/260817-2243_c_the-loeschwarnung-module-header-still-says-loeschfrage-will-fall-and-it-fell-in-the-same-commit.md`

## Abweichung vom Auftrag, benannt

Der Auftrag nennt die Datensatz-Ablage als den einen Werkbank-Speicher, in den zu schreiben
ist. Der Sitzungsbericht `history/260817-2356-…` ist trotzdem angefasst worden, weil die
`## Direction` von `260818-0026` genau das verlangt („Correct the number to 34 in the
session record") und der Datensatz ohne diese Korrektur nicht ehrlich zu schließen wäre.
Geändert ist eine Zahl und sind drei Zeilennummern, sonst nichts. Die übrigen
Zeilennummern des Berichts sind bewusst stehen geblieben: er zeichnet den Stand zu
`522cf51` auf, und diese Nummern verschieben sich mit jedem späteren Commit, dem hiesigen
eingeschlossen.

## Nicht committet

Der Orchestrator committet.
