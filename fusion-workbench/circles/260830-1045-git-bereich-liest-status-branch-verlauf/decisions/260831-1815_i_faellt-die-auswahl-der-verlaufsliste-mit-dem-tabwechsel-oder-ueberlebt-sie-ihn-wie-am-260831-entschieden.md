# Fällt die Auswahl der Verlaufsliste mit dem Tabwechsel, oder überlebt sie ihn, wie am 260831 entschieden?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md` (der Entscheid, dessen Wortlaut hier zur Frage steht); `260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md` (der Befund der Durchsicht); `crates/krk-ui/src/tabs.rs` (`Tabliste::waehlen`, `Tabliste::gitlauf_nachziehen_an`); `crates/krk-ui/src/gitmodell.rs` (`Gitmodell::zuruecksetzen`); `crates/krk-ui/src/appkit/git.rs` (Modulkopf `# Die Auswahl wohnt im Gitmodell und nicht hier`)

---

## Question

Der Nutzer hat am 260831-0120 Möglichkeit 2 gewählt, und die Antwortzeile jenes
Datensatzes nennt den sichtbaren Unterschied ausdrücklich: „sie übersteht damit
den Tabwechsel". Der gebaute Baum hält das nicht. `Tabliste::waehlen` ruft
`gitlauf_nachziehen_an(verlassen)`, und dessen dritte Zeile ist
`self.tabs[stelle].gitmodell.zuruecksetzen()` — unbedingt, vor jeder
Bedingungsprüfung. `Gitmodell::zuruecksetzen` setzt `*self = Self::neu()` und
nimmt Kopf, Verlauf, Zusammenfassung **und** die Auswahl mit. Beim
Zurückwechseln entsteht der Verlauf neu, und die Auswahl steht auf `None`.

Die Frage muss jetzt beantwortet werden, weil drei Prosastellen den Wortlaut des
Entscheids tragen und damit heute die Unwahrheit sagen. Sie an den Baum
anzugleichen hieße, eine Nutzerentscheidung stillschweigend zurückzunehmen; den
Baum anzugleichen ist eine Verhaltensänderung und braucht dieselbe Zustimmung,
mit der der Entscheid vom 260831 gefallen ist. Ein Coder darf keine von beiden
allein wählen. Die drei Stellen stehen bis zur Antwort unverändert; der
Durchsichtsbefund bleibt offen.

## Was am Baum wirklich gilt

Die halbe Aussage trifft zu, und das ist der Grund, aus dem die Prosa so lange
gestanden hat: der Wechsel des **aktiven Dateifensters** lässt die Auswahl
stehen, weil jede `Tabliste` ihren eigenen Lauf und ihr eigenes Gitmodell hält.
Der **Tabwechsel** innerhalb eines Dateifensters wirft sie weg.

## Options

1. **Der Baum zieht nach: der verlassene Tab behält sein Gitmodell.**
   `gitlauf_nachziehen_an` setzt das Modell nicht mehr unbedingt zurück,
   sondern nur dort, wo der Ordner sich ändert.
   - Pros: der Entscheid vom 260831 gilt so, wie der Nutzer ihn beantwortet hat;
     die drei Prosastellen stimmen ohne Änderung; das Halteverhalten der Tabs
     ist überall sonst in KRK dasselbe.
   - Cons: eine Verhaltensänderung mitten in einer Durchsicht, deren Auftrag
     ausdrücklich Prosa ist. `gitlauf_nachziehen_an` ist heute „die eine Stelle,
     an der ein Gitlauf entsteht und vergeht", und ihre drei Zeilen räumen den
     Lauf, die wartende Markenmeldung und das Modell in einem Zug; sie
     aufzuspalten heißt, für jede der drei einzeln zu entscheiden, wann sie
     fällt. C4.6 („ein Ordnerwechsel lässt keine Auswahl des vorigen Ordners
     stehen") und die Zusage aus dem Modulkopf von `tabs.rs` — mit dem Lauf
     fällt der Verlauf auf die ersten fünfzig zurück — müssen dabei beide
     stehen bleiben. Ein Tab, dessen Verlauf beim Zurückwechseln neu entsteht,
     dürfte seine alte Auswahl außerdem nicht auf einen Verlauf legen, der noch
     nicht wieder da ist.
2. **Die Prosa zieht nach: die Auswahl fällt mit dem Tabwechsel.**
   Die drei Stellen nennen als Grund, was zutrifft — den Wechsel des aktiven
   Dateifensters —, und der Datensatz vom 260831 bekommt einen Nachtrag, der
   festhält, dass sein Nebensatz über den Tabwechsel nicht gebaut ist.
   - Pros: keine Zeile Code in einer Durchsicht, die Prosa nachzieht; die drei
     Stellen stimmen sofort; die Heimat der Auswahl bleibt das `Gitmodell`, und
     die zwei Gründe, aus denen Möglichkeit 2 damals gewonnen hat — der eine
     Stand, das gefallene `expect(dead_code)` — bleiben unberührt.
   - Cons: der Nutzer hat den Tabwechsel als den sichtbaren Unterschied genannt,
     an dem er die Möglichkeit erkannt hat. Ihn wegzuschreiben nimmt der Wahl
     ihren Gegenstand: Möglichkeit 1 jenes Datensatzes wäre dann billiger
     gewesen und hätte dasselbe geliefert.

## Constraints

- C4.6 bleibt: ein Ordnerwechsel lässt keine Auswahl des vorigen Ordners stehen.
- C4.2 bleibt: ein nachgeladener Schwung hängt hinten an und lässt die Auswahl
  stehen.
- A10 bleibt: mit dem Lauf fällt sein Empfänger, und kein Befund des alten
  Ordners kommt mehr an.
- Es gibt danach **eine** Heimat für die Auswahl und nicht zwei; das hält der
  Entscheid vom 260831 unabhängig davon, wie diese Frage ausgeht.
- Eine Auswahl darf nie auf einen Verlauf zeigen, den das Modell nicht hält.

## Recommendation

Keine. Die Frage ist nicht die zwischen einem richtigen und einem falschen
Stand, sondern die, welche der beiden Aussagen des Entscheids vom 260831 die
verbindliche war: die über die Heimat der Auswahl (gebaut) oder die über den
Tabwechsel (nicht gebaut). Das kann nur der Nutzer sagen, der sie getroffen hat.

---
Answered: 260831-1820-coder-der-dreizehnte-befund-die-prosa-folgt-dem-baum.md:34 — Möglichkeit 2, entschieden vom Nutzer am 260831-1755: die Auswahl der Verlaufsliste fällt mit dem Tabwechsel. Der gebaute Zustand ist der gewollte, kein Verhalten ändert sich, und die Prosa zieht nach. Von den zwei Aussagen der Antwortzeile vom 260831-0120 bleibt damit die erste verbindlich — die Heimat der Auswahl im `Gitmodell` —, und die zweite über den Tabwechsel ist zurückgenommen; Möglichkeit 1 mit ihrer Aufspaltung von `gitlauf_nachziehen_an` ist verworfen.

Implemented: `crates/krk-ui/src/gitmodell.rs`, `crates/krk-ui/src/tabs.rs`, `crates/krk-ui/src/appkit/git.rs` — vier Prosastellen sagen jetzt beide Hälften: die Auswahl übersteht den Wechsel des aktiven Dateifensters (jede `Tabliste` hält ihr eigenes Gitmodell) und fällt mit dem Tabwechsel (`Tabliste::waehlen` ruft für den verlassenen Tab `gitlauf_nachziehen_an`, dessen dritte Zeile `zuruecksetzen` unbedingt ausführt). Die vierte ist der Modulkopf `# Ein Gitmodell je Tab` in `gitmodell.rs`, den der Defektdatensatz nicht führte und der dieselbe Aussage in einer Begründung trug. Keine Zeile Code; `make check` — exit 0. Der Defekt `260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md` ist damit geschlossen, und `260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md` trägt den Nachtrag.

**Warum `_i_` und nicht `_a_`:** die Antwort lautet „der gebaute Zustand gilt", und ihre einzige Realisierung ist der Nachzug der Prosa. Der ist vollständig auf der Platte und oben mit Pfaden zitiert; ein `_a_` behauptete, es stünde noch etwas aus.
