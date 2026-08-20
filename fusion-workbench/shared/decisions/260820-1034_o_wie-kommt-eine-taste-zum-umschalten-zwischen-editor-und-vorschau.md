# Wie kommt eine gut zugängliche Taste zum Umschalten zwischen Editor und Vorschau?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** `shared/issues/260820-1034_o_cmd-e-bleibt-in-der-vorschau-wirkungslos-und-ist-in-der-dateiliste-gar-nicht-belegt.md`; `resources/default-keymap.toml:134-136,559-561,800-802,809-811,817-819,832-834`; `CLAUDE.md`, Abschnitt „Was man nicht sieht", Absatz zum Ausführungszweig

---

## Question

Der Nutzer meldet am 260820-1030 aus dem Abnahmelauf der Runde 14: „Umschalten zwischen Editor
und Vorschau braucht EINE gut zugängliche Switch-Taste: das ist jetzt noch zu umständlich.
Eventuell `cmd+e` umwidmen?"

Editor und Vorschau sind der fünfte Bereich der Fensterzeile und teilen sich die Fläche
**zeitlich**. Zwischen ihnen zu wechseln ist damit die häufigste Bewegung in diesem Bereich, und
sie hat heute keine eigene Taste.

## Die Lage am Baum, am 260820-1034 erhoben

| Kombination | Funktion | Was sie tut |
|---|---|---|
| `shift+cmd+y` | `fokus_vorschau` | Fokus in das Vorschaufenster |
| `shift+cmd+e` | `fokus_editor` | Fokus in den Editor |
| `f3`, `cmd+y` | `vorschau_umschalten` | Vorschau anzeigen und ausblenden |
| `opt+cmd+b` | `editor_umschalten` | Editor ein- und ausblenden |
| `opt+cmd+e` | `editor_schliessen` | Editor schließen |
| `cmd+e` | `editor_aus_vorschau` | die in der Vorschau gezeigte Datei im Editor öffnen |

Umschalten heißt heute: zwei verschiedene Kombinationen, je nachdem, wohin man will, und beide
mit zwei Zusatztasten. Das ist die Umständlichkeit, die der Nutzer meint.

**`cmd+e` ist nicht frei.** Es trägt `editor_aus_vorschau`, und dieser Befehl ist zugleich als
wirkungslos gemeldet (Datensatz oben). Ein Umwidmen entscheidet also nebenbei, ob jener Befehl
behoben oder abgeschafft wird — das gehört ausgesprochen und nicht nebenbei mitgenommen.

## Options

1. **`cmd+e` wird symmetrisch: derselbe Befehl führt hin und zurück.** Mit dem Fokus in der
   Vorschau öffnet er die angezeigte Datei im Editor und setzt den Fokus dorthin — also das, was
   `editor_aus_vorschau` heute verspricht. Mit dem Fokus im Editor führt er zurück in die
   Vorschau. Der Wirkungsbereich wächst von `Vorschau` auf beide Bereiche.
   - Dafür: eine Taste, eine Bewegung, und der gemeldete Defekt wird bei der Gelegenheit behoben
     statt umgangen. Der Name „Im Editor bearbeiten" wäre zu ändern.
   - Dagegen: ein Befehl mit zwei Bedeutungen, abhängig vom Fokus. Dieses Projekt hat für
     `delete` bereits eine solche Fallunterscheidung und hält sie ausdrücklich für
     sicherheitsrelevant und teuer (`kommandos/rueckschritt.rs`). Hier ist nichts destruktiv, der
     Preis also geringer — aber es ist dasselbe Muster.
   - Was sie verbaut: `cmd+e` als eigenständigen „öffne im Editor"-Befehl aus anderen Bereichen.

2. **Ein neuer Befehl `bereich_tauschen` auf einer freien Kombination**, `cmd+e` bleibt, wie es
   ist, und wird als Defekt getrennt behoben.
   - Dafür: jede Kombination behält eine Bedeutung. Der Defekt wird als Defekt behandelt und
     nicht durch eine Umwidmung überdeckt.
   - Dagegen: eine Kombination mehr im Kopf des Nutzers, und die freie muss erst gefunden werden.
     `f-Tasten` sind knapp; `f3` liegt schon auf der Vorschau.
   - Was sie verbaut: nichts.

3. **`f3` wird die Umschalttaste**, und das Ein- und Ausblenden der Vorschau behält allein
   `cmd+y`.
   - Dafür: eine Taste ohne Zusatztaste, die zugänglichste Lösung überhaupt, und sie liegt schon
     auf diesem Bereich. Die Nachbarschaft zu `f4` („Bearbeiten") ist stimmig.
   - Dagegen: `f3` ändert seine Bedeutung, und das ist die einzige Möglichkeit hier, die eine
     bestehende Gewohnheit bricht. `vorschau_umschalten` verliert eine seiner zwei Tasten.
   - Was sie verbaut: `f3` als Ein-/Ausblendetaste, dauerhaft.

## Constraints

- **Ein neuer Befehl braucht drei Stellen, und nur zwei hält der Übersetzer.** Eine Zeile in
  `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und eine in
  `bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`) fordert der Bau ein. Den
  Ausführungszweig in `Anwendungsdelegierter::kommando_ausfuehren` fordert er **nicht** — er endet
  auf einen Auffangzweig. Ein Befehl ohne eigenen Zweig übersetzt, besteht jede Probe, steht mit
  Namen im Hauptmenü und tut nichts. Genau diese Gestalt ist gerade als Defekt gemeldet.
- **Eine neue Funktion kommt bei jedem Nutzer mit eigener `keymap.toml` unbelegt an**
  (`shared/issues/260814-0656_o_…`). Das trifft Möglichkeit 2 und 3, nicht Möglichkeit 1.
- Der Wirkungsbereich muss beide Bereiche erreichen, sonst wirkt die Taste nur in einer Richtung.

## Recommendation

**Möglichkeit 1**, mit einer Einschränkung.

Die Begründung ist nicht die Sparsamkeit an Tasten, sondern dass sie zwei Dinge zusammenführt,
die zusammengehören: `editor_aus_vorschau` ist bereits „von der Vorschau in den Editor, mit dieser
Datei". Was fehlt, ist der Rückweg. Eine zweite Taste für den Rückweg wäre eine zweite Antwort auf
dieselbe Frage. Und der gemeldete Defekt wird dabei behoben, statt neben einer neuen Taste stehen
zu bleiben.

**Die Einschränkung:** ein Befehl mit zwei fokusabhängigen Bedeutungen ist in diesem Projekt ein
bekannt teures Muster. Er gehört als **reine Funktion** mit genau einem Rufer geschrieben, so wie
`kommandos/rueckschritt.rs` es für die Rückschritt-Taste vormacht, und nicht als `if` im
Ausführungszweig.

**Confidence:** `inference:` — die Empfehlung ruht auf der Belegungstabelle oben, die am Baum
erhoben ist, und auf der Vermutung, dass der Nutzer beim Wechsel in den Editor ohnehin die in der
Vorschau gezeigte Datei meint. Trifft das nicht zu, wenn er also im Editor eine andere Datei
halten will als die Vorschau zeigt, kippt sie auf Möglichkeit 2.

---
Answered:
Implemented:
Deferred:
Superseded by:
