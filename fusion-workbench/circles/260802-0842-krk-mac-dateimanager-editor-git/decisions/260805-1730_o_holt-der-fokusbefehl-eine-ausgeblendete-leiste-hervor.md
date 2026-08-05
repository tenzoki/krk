# Holt `fokus_leiste` eine ausgeblendete Leiste hervor, oder tut die Taste dann nichts?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md` C5 und C7; `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_*_plan-navigator-geruest-runde-1.md` S18; `crates/krk-ui/src/appkit/anwendung.rs` `fokus_setzen`

---

## Question

C5 sagt zu: "Ein Tastenbefehl setzt den Eingabefokus in die Leiste." C7 erlaubt zugleich, die Leiste auszublenden, und `opt+cmd+l` tut das. Was `shift+cmd+l` tun soll, während die Leiste ausgeblendet ist, sagt keine der beiden Fähigkeiten.

Die Umsetzung von S18 hat sich für die stumme Abweisung entschieden, weil das die Antwort ohne zusätzliche Regel ist: der Fokus geht nicht in einen Bereich, den niemand sieht. Der Nutzer, der die Leiste vergessen hat und `shift+cmd+l` drückt, bekommt damit aber keine Rückmeldung und kann den Befehl für kaputt halten. Die Frage muss jetzt beantwortet werden, weil beide Antworten mit einer Zeile umzusetzen sind und die zweite mit der Zeit teurer wird.

## Options

1. **Stumm abweisen** (heute umgesetzt) — die Taste tut nichts.
   - Pro: keine Sonderregel; dieselbe Antwort, die der Wirkungsbereich bei jedem abgewiesenen Befehl gibt.
   - Contra: keine Rückmeldung; der Nutzer sieht nicht, warum nichts geschieht.
2. **Leiste einblenden und den Fokus setzen** — ein Befehl, zwei Wirkungen.
   - Pro: tut, was der Nutzer will, in einem Tastendruck; dasselbe Muster wie `shift+f3` aus C10, das das Vorschaufenster einblendet, wenn es ausgeblendet war.
   - Contra: `shift+cmd+l` ändert dann die Sichtbarkeit, obwohl `opt+cmd+l` der Befehl dafür ist. Zwei Befehle, die beide einblenden.
3. **Abweisen und es in der Statuszeile sagen** — "die Leiste ist ausgeblendet, Opt+Cmd+L blendet sie ein".
   - Pro: Rückmeldung ohne zweite Wirkung.
   - Contra: die erste Meldung eines abgewiesenen Befehls überhaupt; der Wirkungsbereich ist sonst durchgängig stumm, und die Ausnahme muss sich rechtfertigen.

## Constraints

Die Sichtbarkeit der Bereiche gehört C7 und wohnt im Fenstermodell; ein Befehl aus C5, der sie ändert, greift über seine Fähigkeit hinaus. Umgekehrt ist das Vorbild aus C10 (Shift+F3 blendet das Vorschaufenster ein, blendet es aber nie aus) bereits im Spec beschlossen und spricht für Möglichkeit 2.

## Recommendation

Möglichkeit 2, mit dem Vorbild aus C10 als Begründung: der Nutzer, der den Fokus in die Leiste verlangt, verlangt damit, sie zu sehen. Ausblenden würde `shift+cmd+l` weiterhin nicht, also entsteht keine zweite Wahrheit über die Sichtbarkeit, sondern dieselbe Asymmetrie, die C10 schon trägt.

---
Answered:
Implemented:
Deferred:
Superseded by:
