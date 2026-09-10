# Schuldet diese Runde einen Abnahmelauf gegen die Zusage L4?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@qantr.com>
**Cross-references:** `260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md` — derselbe Fall an der Zusage L7, die Runde 14; `260810-2132_*_wird-die-zusage-l9-wieder-angehoben-...`; `crates/krk-bench/src/messen.rs` (die Zusage L4), `messungen/260810-1918-alle-zusagen.txt` (der letzte vollständig saubere Lauf)

---

## Question

L4 sagt zu: **Prozessstart bis bedienbare Prüfsitzung, warm, 1000 ms im Perzentil**
(`crates/krk-bench/src/messen.rs`). Diese Runde legt Arbeit innerhalb dieser Endbedingung
nach: beim Start werden bis zu drei weitere TOML-Dateien gelesen und zerlegt, drei Namens-
mengen verglichen, ein Merker gelesen und geschrieben.

Die Runde 14 stand vor derselben Frage an L7 und hat sie als eigenen Datensatz beantwortet;
CLAUDE.md führt das Ergebnis seither unter „Maximen". Diese Runde stellt sie deshalb
gleichlautend für L4.

**Der Plan setzt die zusätzliche Arbeit hinter den Fassungsvergleich**, und das ändert den
Zuschnitt der Frage. Stimmt die abgelegte Fassung mit der laufenden überein, wird nichts
verglichen und nichts gelesen; übrig bleibt das Lesen des Merkers. Der volle Aufwand fällt
genau einmal je Fassung an, beim ersten Start nach einer Installation.

## Options

1. **Kein eigener Abnahmelauf; L4 kommt auf die Gegenstände der späteren Messrunde.**
   - Pro: Die zusätzliche Arbeit fällt im Dauerbetrieb nicht an, und L4 misst warm und
     wiederholt. Der gemessene Weg ist der ohne Vergleich.
   - Pro: Derselbe Weg, den der Nutzer für L7 in der Runde 14 gewählt hat. Die
     Vergleichbarkeit der Buchführung ist selbst ein Wert.
   - Contra: „Fällt im Dauerbetrieb nicht an" ist eine Aussage über den Plan und nicht über
     das gebaute Bündel. Ohne Lauf steht sie unbelegt da, und die Liste der ungemessenen
     Runden wird um eine länger.

2. **Ein Abnahmelauf gegen L4 gehört zu dieser Runde.**
   - Pro: Die Zusage wird gegen den Baum belegt, der sie berührt, und nicht gegen einen von
     vor fünf Runden. Der letzte vollständig saubere Lauf ist vom 260810 und liegt vor jeder
     seither geschlossenen Runde.
   - Contra: Der Lauf verlangt KRK im Vordergrund und ist damit Nutzerarbeit; kein Agent
     kann ihn fahren. Er misst außerdem alle zehn Zusagen und nicht L4 allein.
   - Contra: Der erste Start nach der Installation, also der Fall mit dem vollen Aufwand,
     ist mit der Messstrecke gar nicht zu treffen: sie liest eine Prüfsitzung und läuft
     wiederholt. Der Lauf belegte damit genau den Weg, der ohnehin unstrittig ist.

3. **Ein Lauf, und dazu eine Messung des ersten Starts nach einer Installation.**
   - Pro: Trifft den Fall, um den es geht.
   - Contra: Es gibt heute keine Messstrecke dafür, und eine zu bauen hieße, eine elfte Zahl
     zu setzen. Keine Runde nach der ersten hat das getan, und eine Zusage, die eine Runde
     nicht messen kann, wäre ein Wunsch.

## Constraints

- Der Abnahmelauf verlangt KRK im Vordergrund; aus dem Hintergrund gestartet meldet die
  Messstrecke `NICHT_IM_VORDERGRUND` statt Zahlen. Das ist Nutzerarbeit, so entschieden am
  260907-2009.
- Keine elfte Zahl. Die zehn Zusagen aus C8 der Runde 1 bleiben, wie sie sind; diese Runde
  fasst keine an.
- Die Antwort gilt für diese Runde. Sie ist kein allgemeiner Satz darüber, wann eine Runde
  einen Lauf schuldet.

## Recommendation

**Möglichkeit 1**, unter der Bedingung, dass der Fassungsvergleich wirklich vor der Erhebung
steht. Der Plan macht das zur Randbedingung des Schrittes, der die Erhebung einbaut, und
eine Probe hält es: ohne Fassungswechsel wird keine der drei Dateien ein zweites Mal
geöffnet. Damit ist der gemessene Weg derselbe wie vor dieser Runde, und ein Lauf belegte
nichts, was nicht schon belegt wäre.

Was offen bleibt und offen bleiben soll: der erste Start nach einer Installation ist von
keiner der zehn Zusagen gedeckt und wird es auch nicht. Wer ihn messen will, setzt eine
elfte Zahl, und das ist eine andere Frage als diese.

---
Answered: `260905-2008-orchestrator-session.md` `## Fortsetzung 260910 — die Runde 24 wird geplant` — Möglichkeit 1: kein eigener Abnahmelauf, L4 kommt auf die Gegenstände der späteren Messrunde; derselbe Weg wie bei L7 in der Runde 14, und der Preis — die Aussage „fällt im Dauerbetrieb nicht an" bleibt unbelegt — ist mitentschieden; ruled by user, Kai Stalmann <kai@stalmann.org>.
