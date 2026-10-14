# Eine Auffrischung stößt die Vorschau mit an, und die Kosten sind ungemessen

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md` (Schritt 7); `shared/decisions/260825-1725_*_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md` (Cons-Liste); `crates/krk-ui/src/appkit/tabelle.rs` (`nach_lesebeginn`, `auswahl_merken`); Commit `9322d5d`

---

## Was ist

`nach_lesebeginn` zieht Navigation **und** Auffrischung gemeinsam nach. Seit
`9322d5d` meldet es dabei auch die Auswahl, also stößt jeder FSEvents-Lauf im
angezeigten Ordner die Vorschau mit an: sie liest den weiterhin ausgewählten
Eintrag neu.

Dazu ein zweiter Fall: ein Aufstieg kostet eine Ordnerzusammenfassung, die die
wiederhergestellte `wunschauswahl` unmittelbar danach ersetzt. Gerechnet wird
also etwas, das niemand zu sehen bekommt.

## Warum das zählt

Beides folgt aus der Stelle, die der Plan gewählt hat, und die Cons-Liste des
Entscheids hat es vorhergesehen. Es ist damit kein Fehlgriff, sondern ein
bekannter Preis, den bisher **niemand beziffert hat**.

Der Preis ist auch nicht offensichtlich klein: eine Zusammenfassung darf bis zu
zwölf Verzeichnisleseläufe und vierundzwanzig Dateiöffnungen kosten, und ein
Ordner unter FSEvents kann in kurzer Folge mehrfach melden.

## Was zu tun wäre

Messen, bevor gebaut wird. Zwei Fragen, die eine Messung beantworten müsste:
wie oft eine Auffrischung im Alltag meldet, und was eine überflüssige
Zusammenfassung an der wirklichen Werkbank kostet. Die Kostenmessung aus
Schritt 10 des Plans liefert die zweite Hälfte davon; die erste liefert sie
nicht.

Erst danach ist zu entscheiden, ob es einen Vergleich gegen den zuletzt
gemeldeten Pfad braucht oder ob der Preis getragen wird.

## Status

Offen und ausdrücklich ungemessen.
