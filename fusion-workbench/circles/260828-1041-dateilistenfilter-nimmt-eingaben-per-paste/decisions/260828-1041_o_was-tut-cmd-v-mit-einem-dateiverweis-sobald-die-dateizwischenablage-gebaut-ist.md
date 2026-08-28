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
