# Was tut Cmd+V mit einem Dateiverweis, sobald die Dateizwischenablage gebaut ist?

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Cross-references:** `circles/260828-1041-dateilistenfilter-nimmt-eingaben-per-paste/_*_circle.md` (Directive und Grounding, Absatz zum Einhängepunkt); `resources/default-keymap.toml:81-84` und `:990-997` (die Reservierung); `crates/krk-ui/src/appkit/menue.rs:105-116` (derselbe Satz am Menü); `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-0000_*_menuekuerzel-in-die-konflikterkennung-oder-daneben.md` (der Nutzerentscheid, der `cmd+v` freihält)

---

## Question

Die Belegung und das Menü „Bearbeiten" halten `cmd+v` seit dem 260805 für eine Dateizwischenablage einer späteren Runde frei: wer `paste:` am Dateifenster beantwortet, hat die Kombination. Der Nutzer hat am 260828 entschieden, dass `cmd+v` im Dateifenster den Filtertext füllt, und dass ein Dateiverweis aus dem Finder dabei mit seinem Namen eingefügt wird. Damit ist der Einhängepunkt besetzt, und die eine Geste, die eine Dateizwischenablage am dringendsten braucht — Datei im Finder kopiert, in KRK eingefügt —, hat schon eine Bedeutung. Die Frage muss nicht jetzt beantwortet werden; sie muss jetzt aufgeschrieben werden, damit die spätere Runde sie vorfindet und nicht die Filterregel für einen Defekt hält.

## Options

1. **Der Dateiverweis wechselt die Bedeutung, sobald die Dateizwischenablage steht** — `cmd+v` mit einem Verweis in der Ablage fügt dann die Datei ein, mit Text weiter den Filtertext.
   - Pro: jede Sorte tut, was die naheliegende Quelle meint; die Reservierung wird eingelöst.
   - Contra: dieselbe Taste tut je nach unsichtbarem Ablageinhalt zwei sehr verschiedene Dinge, und ein Nutzer, der sich an das Filtern per Finder-Verweis gewöhnt hat, verliert es.
2. **Der Filter behält `cmd+v` ganz, die Dateizwischenablage bekommt eine eigene Kombination** — etwa `shift+cmd+v`, die heute „Ablage beiseitelegen" trägt (`default-keymap.toml:151`), oder eine freie.
   - Pro: keine Doppelbedeutung; die Regel dieser Runde bleibt unverändert.
   - Contra: die Reservierung wird nie eingelöst, und die Mac-übliche Geste für das Einfügen von Dateien liegt woanders.
3. **Die Dateizwischenablage kommt nicht** — die Frage erledigt sich; KRK bewegt Dateien über die Vorgänge aus C4 der Runde 1 und den Abwurf aus fremden Anwendungen (Runde 13).
   - Pro: keine Entscheidung nötig.
   - Contra: sie ist seit dem 260805 vorgesehen, und diese Möglichkeit sagt sie ab.

## Constraints

`cmd+v` bleibt eine vom Menü gehaltene Funktion; ein zweiter Menüeintrag und eine zweite Zeile in der Belegung sind nach dem Entscheid vom 260805 ausgeschlossen. Es gibt genau eine Hülle um `NSPasteboard`.

## Recommendation

Keine; die Frage gehört der Runde, die die Dateizwischenablage baut, und bindet bis dahin nur insofern, als der Spec dieser Runde die Doppelbelegung nicht als Dauerzustand ausschreibt.

---
Reconciled 260829-1223: weiter offen, keine Antwort im Baum. Gesucht in `planning/260829-1052_*_spec-…` (A6 besetzt den Einhängepunkt und beantwortet die Frage ausdrücklich nicht), `planning/260829-1102_*_plan-…` (`## Where this Circle stops`, vorletzte Klauseln) und `shared/analyses/` (kein Treffer). Der Baum nach `3722c89` nimmt `cmd+v` mit einem Dateiverweis als Filtertext (`crates/krk-core/src/zwischenablage.rs:163`, `filtertext_aus` auf `Einfuegequelle::Verweise`); genau diese Lage ist der Gegenstand der Frage, keine ihrer drei Möglichkeiten ist gewählt. Keine Vorbedingung dieser Runde; der Datensatz bindet die Runde, die die Dateizwischenablage baut.

---
Answered: 260905-2008-orchestrator-session.md `## Fuenf weitere Entscheidungen am 260907-2009 beantwortet` — Keine der drei vorgelegten Moeglichkeiten, sondern eine vierte des Nutzers: cmd+f fuegt in den Filtertext ein, cmd+v ist fuer das Einfuegen einer Datei reserviert. cmd+f ist dabei nicht frei, dort liegt der Mac-Standard zum Suchen im Text; es traegt aber nach der Regel vom 260805 fuer cmd+a, weil zwei Zusteller in verschiedenen Wirkungsbereichen kein Konflikt sind. Bis die Dateizwischenablage steht, schweigt cmd+v im Dateifenster; die Doppelbelegung auf Zeit und die Statuszeilenmeldung hat der Nutzer verworfen; ruled by user, Kai Stalmann <kai@stalmann.org>.

**Berichtigung 260908 zur Begründung der `Answered:`-Zeile, nicht zu ihrem Ergebnis.** Der
Halbsatz „weil zwei Zusteller in verschiedenen Wirkungsbereichen kein Konflikt sind" gibt die
Regel vom 260805 verkehrt wieder. Sie lautet: zwei Funktionen sind genau dann ein Konflikt,
wenn sie dieselbe Kombination tragen und **denselben Zusteller** haben; ihr Abschnitt
„`Wirkungsbereich` ist kein zweiter Zusteller" sagt ausdrücklich, dass zwei vom Abgriff
zugestellte Funktionen mit verschiedenem Wirkungsbereich ein Konflikt **bleiben**
(`260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`).

Das Ergebnis der Antwort ist davon unberührt: `cmd+f` trägt das Einfügen in den Filtertext,
und es geht — aber über den anderen **Zusteller**. `filter_einfuegen` steht seit dem
260907-2046 mit `gehalten_von = "menue"` in `resources/default-keymap.toml` und ist damit
nach dem Zusteller gebaut. Wer die Begründung wörtlich nähme und die Funktion mit
`Wirkungsbereich::Dateifenster` und ohne `gehalten_von` einträgt, legte zwei vom Abgriff
zugestellte Funktionen auf `cmd+f`; die eingebettete Auslieferungsbelegung bricht bei einem
Konflikt beim ersten Zugriff ab. Der Datensatz dazu ist
`260907-2026_*_die-antwortzeile-zu-cmd-f-begruendet-das-paar-mit-dem-wirkungsbereich-die-regel-vom-260805-nennt-den-zusteller.md`.
