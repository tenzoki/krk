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

---

## Antwort des Nutzers, 260823-0942: eine vierte Gestalt, nicht eine der drei

Der Nutzer hat zuerst die F-Tasten-Möglichkeit gestrichen („keine F-Taste, etwas besser
zugängliches") und danach die Frage selbst umgestellt. Seine Begründung widerlegt die Annahme, auf
der die Empfehlung oben ruhte, und zwar an genau der Stelle, an der sie sich als `inference:`
gekennzeichnet hat: **der Fokus steht nach `f3` nicht in der Vorschau, sondern bleibt in der
Dateiliste.** Ein Umschalter, der die Vorschau als Ausgangspunkt nimmt, trifft damit einen
Zustand, den der Nutzer im Betrieb selten hat. Möglichkeit 1 ist deshalb nicht gewählt, sondern
gegenstandslos.

**Beschlossen ist:**

| Fokus | `cmd+e` tut |
|---|---|
| Dateiliste | öffnet die ausgewählte Datei im Editor und legt den Fokus hinein (dasselbe wie `f4`) |
| Editor | **schließt** den Editor, die Vorschau zeigt die Datei wieder, der Fokus geht in die Dateiliste |
| Vorschau | bleibt wie bisher: die angezeigte Datei im Editor öffnen |

**Der Rückweg schließt und blendet nicht aus.** Die Wahl ist dem Nutzer am 260823-0942 mit ihrem
Preis vorgelegt worden und er hat sie so getroffen: `editor_schliessen` gibt die Datei frei und
löst die Nachfrage aus C4 aus, `editor_umschalten` behält den Stand. Auf dem Rückweg kommt damit
bei ungesichertem Stand die Nachfrage, also gerade dann, wenn der Nutzer eben getippt hat. Dafür
hält kein unsichtbarer Editor ungesicherte Änderungen. Wer das später umdreht, dreht eine
bewusste Wahl um und nicht ein Versehen.

**Die Vorschau-Zeile ist nicht vom Nutzer entschieden, sondern beibehalten.** Sie steht heute so
am Baum, sie zeigt in dieselbe Richtung wie die neue Dateilisten-Zeile, und der Nutzer hat nicht
verlangt, sie zu entfernen. Wer sie streichen will, fragt ihn.

**Eine Spannung gehört benannt.** Der Kommentar bei `editor_schliessen` in
`resources/default-keymap.toml` führt den Satz, eine ausgelieferte Kombination einer abgenommenen
Runde wechsle ihre Bedeutung nicht; er ist der Grund, warum `editor_umschalten` auf `opt+cmd+b`
und nicht auf `opt+cmd+e` liegt. `cmd+e` behält hier seine bisherige Richtung und bekommt zwei
Bereiche und einen Rückweg dazu, wechselt seine Bedeutung also nicht, sondern erweitert sie. Der
Nutzer hat die Erweiterung ausdrücklich verlangt.

**Der Wirkungsbereich wächst** von `Vorschau` auf Dateifenster, Vorschau und Editor. Die
Fallunterscheidung gehört nach dem Vorbild von `kommandos/rueckschritt.rs` als reine Funktion mit
genau einem Rufer geschrieben und nicht als `if` im Ausführungszweig; diese Auflage aus der
Empfehlung oben gilt unverändert.

---
Answered: `shared/decisions/260820-1034_a_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md` (dieser Datensatz, Abschnitt „Antwort des Nutzers, 260823-0942") — `cmd+e` wird der Rundweg Dateiliste → Editor → Dateiliste, der Rückweg schließt.

---
Implemented: `28cbb7b` — `cmd+e` ist der Rundweg. Die Fallunterscheidung steht als reine Funktion
mit genau einem Rufer in `crates/krk-ui/src/kommandos/rundweg.rs`, wie die Auflage es verlangt;
alle drei Zweige rufen bestehende Rümpfe, und `editor_schliessen` ist für den Rückweg
herausgezogen statt abgeschrieben. `Wirkungsbereich::Vorschau` ist gefallen und
`Wirkungsbereich::Dateibereiche` an seine Stelle getreten, weil `Vorschau` genau diesen einen
Befehl trug. Die Kennung heißt jetzt `editor_rundweg`. **Die Abnahme von Hand steht noch aus.**

---
**Abgenommen am 260823-1320.** Der Nutzer hat die drei Handgriffe von Hand gefahren und berichtet,
es funktioniere: `cmd+e` aus der Dateiliste öffnet die Datei im Editor mit dem Fokus darin, ein
zweites `cmd+e` schließt ihn und gibt den Fokus an die Dateiliste zurück, und „Abbrechen" an der
Nachfrage nach ungesichertem Stand lässt den Editor stehen und die Vorschau draußen. Der letzte
Handgriff war der einzige, den keine Probe deckt.
