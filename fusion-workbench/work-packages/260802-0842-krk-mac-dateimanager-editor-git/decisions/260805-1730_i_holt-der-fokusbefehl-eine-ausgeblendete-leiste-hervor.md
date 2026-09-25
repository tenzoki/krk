# Holt `fokus_leiste` eine ausgeblendete Leiste hervor, oder tut die Taste dann nichts?

---
**Domain:** code
**Status:** implemented
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

## Antwort des Nutzers vom 260807

**Möglichkeit 2, der Empfehlung folgend: `shift+cmd+l` blendet die Leiste ein und setzt danach den Fokus.** Ausblenden tut der Befehl weiterhin nie; dafür bleibt `opt+cmd+l` aus C7.

**Die Antwort ist eine Aussage über den Befehlstyp und nicht über eine Taste, und sie trägt deshalb weiter als die Frage.** Ein Fokusbefehl holt seinen Bereich hervor, weil wer den Fokus dorthin verlangt, den Bereich zu sehen verlangt. Damit gilt sie unverändert für den dritten Fokusbefehl, den der Nutzer am selben Tag beauftragt hat: `shift+cmd+y` blendet eine ausgeblendete Vorschau ebenso ein, `decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md`. Für das Dateifenster stellt sich der Fall nicht, weil das linke sich nach C7 gar nicht ausblenden lässt und mit dem rechten die Aktivität auf das linke wandert.

**Aus der Antwort folgt eine Regel, die der Spec bis dahin nicht führte.** Drei Befehle außerhalb von C7 können einen Bereich einblenden, und keiner blendet je einen aus: `shift+f3` aus C10, `shift+cmd+l` aus C5 und `shift+cmd+y` aus C6. Bis zum 260807 trug allein `shift+f3` diese Asymmetrie, und sie stand nirgends als Regel, sondern als Eigenschaft eines einzelnen Befehls. C7 nennt sie seither ausdrücklich, damit sie eine Regel ist und nicht dreimal derselbe Zufall. Die Sichtbarkeit bleibt trotzdem die Sache von C7, weil ein Befehl von außerhalb sie nur in die eine Richtung bewegen kann, in der er den Zustand herstellt, den er ohnehin braucht.

**Die Sperre gegen den Fokus in einem ausgeblendeten Bereich fällt nicht weg.** Sie gilt für jeden Aufrufer und nicht nur für den einen, der vorbaut; ein Fokusbefehl blendet ein und setzt danach, statt die Sperre zu ersetzen.

**Umgesetzt wird die Antwort im neuen Planschritt S19b**, zusammen mit dem dritten Fokusbefehl, weil beide dieselbe Stelle betreffen. S18 bleibt abgenommen und unverändert.

---
Answered: `planning/260802-1036_*_spec-navigator-geruest.md`:296 — Möglichkeit 2, Leiste einblenden und Fokus setzen; das Abnahmekriterium steht in C5 (ebd.:289), die daraus abgeleitete Regel in C7 (ebd.:340).
Implemented: `9a47c4a` — `Fenstermodell::einblenden` in `crates/krk-ui/src/fenstermodell.rs` trägt die Asymmetrie jetzt an einer Stelle, und `fokus_holen` nimmt sie ebenso wie `zwischenablage_ansehen`, das sie bis dahin in vier Inline-Zeilen führte. `shift+cmd+l` blendet die Leiste ein und setzt den Fokus, ausblenden tut der Befehl nie. Dieselbe Regel gilt für den neuen `shift+cmd+y`: die Antwort ist eine über den Befehlstyp und nicht über eine Taste.
Deferred:
Superseded by:
