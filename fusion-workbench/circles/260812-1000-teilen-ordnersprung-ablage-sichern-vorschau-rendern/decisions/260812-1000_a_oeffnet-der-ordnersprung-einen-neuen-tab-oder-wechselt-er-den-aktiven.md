# Öffnet der Ordnersprung einen neuen Tab, oder wechselt er den Ordner des aktiven?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/tabs.rs:508` (`ordner_setzen`, wechselt den sichtbaren Tab); `crates/krk-ui/src/tabs.rs:463` (`oeffnen`, legt einen Tab an); `crates/krk-ui/src/appkit/tabelle.rs:628` (`ordner_lesen`, die Hülle darum)

---

## Question

Das aktive Dateifenster führt mehrere Tabs. Der neue Befehl kann den Ordner des sichtbaren Tabs wechseln oder einen neuen Tab anlegen. Beide Wege sind gebaut: `Tabliste::ordner_setzen` wechselt, `Tabliste::oeffnen` legt an.

Die Frage ist zu stellen, weil sie darüber entscheidet, was der Nutzer verliert. Ein Wechsel wirft den Ordner weg, den der Tab vorher zeigte; ein neuer Tab lässt ihn stehen und kostet dafür einen Tab, den der Nutzer irgendwann wieder schließt. Der Ordnersprung wird häufig aus einer Lage heraus benutzt, in der der Nutzer im Dateifenster gerade woanders steht, sonst bräuchte er ihn nicht.

Der Bestand gibt eine Richtung vor: **jede Navigation dieses Programms wechselt den sichtbaren Tab.** `ordner_setzen` trägt im Kopf die Aufzählung „hinein, hinaus, über die Pfadeingabe, aus der Zwischenablage", und der Sprung aus C10, der einer Adresse aus der Zwischenablage folgt, ist der nächste Verwandte des neuen Befehls. Er wechselt.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Den Ordner des sichtbaren Tabs wechseln.** Wie jede andere Navigation.
   - Folge: der Befehl reiht sich ohne Ausnahme in die vorhandene Navigation ein, und `ordner_lesen` bekommt einen weiteren Aufrufer. Die Tabzahl ändert sich nie.
   - Preis: der Ordner, den der Tab vorher zeigte, ist weg. Es gibt keinen Rückweg außer der Pfadeingabe, denn KRK führt keine Ordnerhistorie.

2. **Einen neuen Tab anlegen.** Der bisherige Tab bleibt stehen, der neue zeigt den Zielordner und wird sichtbar.
   - Folge: nichts geht verloren, und der Nutzer kommt mit einem Tabwechsel zurück. Das ist auch das Verhalten, das ein Nutzer aus dem Finder mit „Im übergeordneten Ordner zeigen" kennt.
   - Preis: eine Ausnahme in der Navigation. Alle anderen Wege wechseln, dieser legt an, und der Unterschied ist an der Oberfläche nicht ablesbar. Wer den Befehl mehrfach benutzt, sammelt Tabs.

3. **Wechseln, mit einer zweiten Kombination für den neuen Tab.** Der gewöhnliche Befehl wechselt, eine Variante legt an.
   - Folge: der Nutzer hat beides, und die Reihe der zweiten Stufe (`ctrl+cmd+X`) hat für genau diesen Zweck schon drei Einträge.
   - Preis: eine zweite Kombination aus einem knappen Vorrat, ein zweites Kommando in vier vollständigen Fallunterscheidungen, ein zweiter Eintrag in der Belegungsansicht. Für einen Unterschied, den der Nutzer bisher nirgends verlangt hat.

## Constraints

- KRK führt keine Ordnerhistorie und keinen Rückwärtsbefehl. Was ein Wechsel wegwirft, ist weg.
- Ist der Zielordner bereits der Ordner des sichtbaren Tabs, darf der Befehl ihn nicht ohne Not neu lesen. `Tabliste::schliessen` zeigt die Bauart: „Schon der Standardordner. Ihn ein zweites Mal zu lesen wäre Arbeit ohne sichtbare Wirkung." Der Befehl setzt in diesem Fall allein die Auswahl, wofür `Tabliste::auswahl_auf_namen` (`crates/krk-ui/src/tabs.rs:585`) da ist.
- Welches der beiden Dateifenster betroffen ist, hat der Nutzer bereits festgelegt: das aktive.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige ohne Ausnahme, und die Ausnahme wöge hier schwer: die Navigation dieses Programms hat heute genau eine Regel, und ein Befehl, der sich als Navigation anfühlt und sich anders verhält, ist an nichts zu erkennen.

Möglichkeit 3 empfehlen wir nicht, obwohl sie technisch sauber wäre. Sie kostet eine Kombination aus einem Vorrat, der laut dem Datensatz `260812-1000_*_welche-tastenkombinationen-bekommen-die-zwei-neuen-befehle.md` ohnehin knapp ist, und sie löst ein Problem, das der Nutzer nicht gemeldet hat.


## Antwort 260812-1105

**Moeglichkeit 1.**

Der Ordnersprung wechselt den Ordner des aktiven Tabs, er oeffnet keinen neuen.

Sie ist die einzige Moeglichkeit ohne Ausnahme, und die Ausnahme woege hier schwer: die Navigation
dieses Programms hat genau eine Regel, und ein Befehl, der sich wie Navigation anfuehlt und sich
anders verhaelt, ist an nichts zu erkennen.

Eine eigene Kombination fuer die Tab-Form ist abgelehnt: sie kostet aus einem knappen Vorrat und
loest ein Problem, das der Nutzer nicht gemeldet hat.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
