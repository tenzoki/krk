The `## Sprache` paragraph in CLAUDE.md predates the artifact-language declaration and now contradicts it

---

CLAUDE.md line 4 declares `**Artifact language:** en` beside `**Language:** de`, but the
`## Sprache` section further down still explains the single-declaration case: it states that
the `**Language:** de` line governs which style profiles apply and that `bin/fusion-rules`
therefore emits `default-voice-de.yaml` for long-form agents. The helper now emits
`default-voice-en.yaml`. The section also states, in its closing paragraph, that prose in
this project is German — which is still true of the tree and of every artifact written
before 260817-1600, and no longer true of artifacts written after it.

---

**Filed by:** orchestrator, session `260817-1208`
**Found by:** coder, while finishing task T7 (plan step 8); it deliberately filed no record
and reported the observation instead, on the ground that normative project text is the
curator's remit.
**Severity:** low. Nothing breaks and no build stops. The cost is that the section a new
agent reads to learn which language to write in describes the configuration the project had
until this afternoon.
**Affects:** `/Users/k1/Projects/productive/krk/CLAUDE.md` line 4 against its `## Sprache`
section (the paragraph beginning `Die Zeile **Language:** de oben deklariert…` and the two
sentences after it).
**Tree state:** `c260e64` plus the uncommitted CLAUDE.md edit.

## What the rule actually says

`rules/fusion-workbench-conventions.md` `## Project language` defines four surfaces, and the
two declarations now split them: terminal output takes the chat language (`de`), files that
persist for the project's own use take the artifact language (`en`), a customer deliverable
takes the language its dispatch names, and text shipped to consuming projects is English
regardless. The same section states that existing artifacts are **not** translated, so the
German corpus stays as it is and the boundary applies going forward. Commit messages fall on
the persisted side by an explicit user decision recorded in that rule.

None of that is wrong in the tree. What is wrong is the description of it in CLAUDE.md.

## Why this is the curator's and not a coder's

The section is normative project text, and its wording is load-bearing in two directions:
the closing sentence tells every agent that prose in this project is German, and the
paragraph above it tells the reader which mechanism decides that. A coder rewriting either
would be authoring the rule rather than following it. `agents/curator.md` reconciles
CLAUDE.md against what the project actually does, at a user gate, which is the shape this
change needs — the user chose the second declaration, and the text has to be brought to it
rather than the other way round.

## Suggested resolution

Run `/fusion:cleanup --only claude-md`, or dispatch the curator directly. Three statements
in the section need to move: which line steers which profile family, which profile the
helper emits for long-form agents, and the scope of the sentence that prose is German. The
`**Language:** de` line and the `**Artifact language:** en` line above it are fixed-format
declarations and must not be reworded or relocated.

---
Also seen: 260819-0057 by reconciler — unverändert offen, und der Widerspruch ist in dieser
Sitzung ein zweites Mal aufgefallen. `CLAUDE.md:4` deklariert weiter `**Artifact language:** en`,
während der Abschnitt `## Sprache` in seinem Schlussabsatz sagt „Prosa in diesem Projekt ist
deutsch" und im Absatz davor beschreibt, `**Language:** de` steuere die Profile für Langform-
Agenten. Der Bestand gibt dem Schlussabsatz recht und der Deklaration nicht: Spec, Plan, beide
Durchsichten, die vierzehn Sitzungsprotokolle und die Entscheidungsdatensätze der Runde 13 sind
deutsch; englisch sind allein die elf Defektdatensätze, die die beiden `coderev`-Durchgänge
gefilt haben. Der Orchestrator dieser Sitzung hat denselben Befund unter „Note for a later pass"
in `shared/history/260818-1117-orchestrator-session.md` festgehalten. Nichts davon ist in diesem
Abgleich geändert worden: `CLAUDE.md` ist der Gegenstand eines Kuratorendurchgangs und nicht
eines Abgleichs.

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, unverändert, und in diesem Durchgang unmittelbar spürbar.** `CLAUDE.md:4` deklariert `**Artifact language:** en`. Daneben sagt `:176`, die Zeile `**Language:** de` steuere für Langform-Agenten `default-voice-de.yaml`, und `:178` schließt mit „Prosa in diesem Projekt ist deutsch". Die drei Aussagen stehen unvereinbar nebeneinander.

**Was der Widerspruch heute gekostet hat:** `bin/fusion-rules reconciler` gibt für diesen Durchgang allein `fusion-workbench/stilwerk/chat-voice-de.yaml` aus, also die Kurzform auf Deutsch, während die Artefaktsprache nach der Kopfzeile `en` wäre. Die Sprache der Abgleichsartefakte ist deshalb aus der Aufgabenstellung des Nutzers genommen und nicht aus der Deklaration. Der Marker bleibt `_o_` für den Durchgang des Kurators.

---
Resolved: Der Nutzer hat die zugrunde liegende Frage am 260819-2032 entschieden
(`shared/decisions/260819-1500_*_gilt-die-artefaktsprache-en-fuer-den-ganzen-bestand-oder-wird-die-deklaration-zurueckgenommen.md`,
Möglichkeit 2): die Zeile `**Artifact language:** en` ist aus `CLAUDE.md` entfernt, und der
Abschnitt `## Sprache` beschreibt wieder den einsprachigen Fall — `**Language:** de` steuert beide
Flächen, und `bin/fusion-rules` gibt für Langform-Agenten wieder `default-voice-de.yaml` aus.
Damit stimmen die drei Aussagen, die dieser Datensatz als unvereinbar führt, wieder überein, und
der Schlusssatz „Prosa in diesem Projekt ist deutsch" ist wieder wörtlich wahr.

Der Datensatz nennt den Kuratorendurchgang als den richtigen Weg, weil ein Coder sonst die Regel
selbst schriebe statt ihr zu folgen. Diese Grenze ist eingehalten: die Wahl zwischen den drei
Möglichkeiten hat der Nutzer an einem Gate getroffen, der Coder hat nur den Text auf die
getroffene Wahl gezogen. Die elf englischen Defektdatensätze bleiben als Bestand stehen.
