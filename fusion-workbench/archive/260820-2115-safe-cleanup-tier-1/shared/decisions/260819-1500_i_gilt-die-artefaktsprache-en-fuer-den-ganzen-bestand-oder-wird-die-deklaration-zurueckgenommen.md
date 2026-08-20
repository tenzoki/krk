# Gilt die Artefaktsprache `en` für den ganzen künftigen Bestand, oder wird die Deklaration zurückgenommen?

---
**Domain:** code
**Status:** implemented
**Filed by:** curator
**Cross-references:** `CLAUDE.md:4` gegen `CLAUDE.md:178`; `shared/issues/260817-1610_*_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md`; `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`; `shared/history/260819-1440-reconciliation.md`; `shared/history/260819-1500-curator-run.md`, Abschnitt 8

---

## Question

`CLAUDE.md` deklariert in seinem Kopf zwei Sprachen und sagt in seinem Rumpf eine dritte Sache, die zu keiner von beiden passt. Ein Agent, der beide Stellen liest, kann nicht beiden folgen.

**Zeile 4:** `**Artifact language:** en`. Nach `rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, entscheidet allein die Fläche, und für jede Datei, die **für den eigenen Gebrauch des Projekts** persistiert — Specs und Pläne, Defekt- und Entscheidungsdatensätze, Sitzungsprotokolle, Durchsichten, Analysen, Memos, das Portfolio — gilt damit Englisch. Dieselbe Regel zieht Commit-Messages und die Zeilen der Übersichtstafel ausdrücklich auf dieselbe Seite.

**Zeile 178:** „Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen."

Beide Sätze sind bindend, beide stehen in derselben Datei, und sie widersprechen einander in zwei Punkten: welche Sprache eine persistierte Projektdatei trägt, und auf welcher Seite der Grenze eine Commit-Message steht.

**Warum die Frage jetzt gestellt wird.** Die Zeile 4 ist am 260817-1909 im Commit `c75972c` hinzugekommen, von Hand und vom Nutzer selbst. Der Abschnitt `## Sprache` stammt aus der Zeit davor und ist seither nicht angefasst worden. Der Bestand hat sich der Deklaration nur zum kleinen Teil angeschlossen: Spec, Plan, beide Durchsichten, die Sitzungsprotokolle und die Entscheidungsdatensätze der Runde 13 sind deutsch, englisch sind allein die elf Defektdatensätze, die zwei `coderev`-Durchgänge gefilt haben. Der Abgleich vom 260819-1440 hält fest, dass ihn der Widerspruch an diesem Tag unmittelbar Arbeit gekostet hat: er hat die Sprache seiner Artefakte aus der Aufgabenstellung des Nutzers genommen, weil die Deklaration ihm etwas anderes sagte als der Bestand, in dem er schrieb.

**Warum der Kurator die Frage nicht selbst entscheidet.** Beide Seiten sind lebendig und vertretbar. Die Kopfzeile ist eine ausdrückliche Wahl des Nutzers; der Schlusssatz beschreibt richtig, was das Projekt tut. Eine Änderung an einer der beiden Zeilen wäre die stille Festlegung auf eine Lesart, und der Nutzer hat sie nicht getroffen.

## Options

1. **Die Deklaration gilt: neue Artefakte werden englisch, der vorhandene Bestand bleibt deutsch.** Der Schlusssatz des Abschnitts `## Sprache` wird auf seinen wahren Umfang gebracht — deutsch bleiben Bezeichnernamen im Code? nein, die sind ohnehin englisch; deutsch bleibt allein, was schon geschrieben ist. Alles, was ein Agent ab heute persistiert, ist englisch, Commit-Messages eingeschlossen.
   - Pro: Folgt der Regel, wie sie geschrieben steht, und der Wahl, die der Nutzer im Kopf der Datei getroffen hat. Verlangt keine Übersetzung: `## Project language` sagt ausdrücklich, dass vorhandene Artefakte nicht übersetzt werden.
   - Kontra: Der Bestand wird auf Dauer zweisprachig, und zwar nicht nach Sachgebieten getrennt, sondern nach dem Datum. Ein Datensatz von 2026-08-16 und sein Nachtrag von 2026-08-20 stehen dann in verschiedenen Sprachen in derselben Datei — die Form hat dieser Baum in `shared/issues/260816-1232_*_…` bereits, wo ein deutscher Rumpf einen deutschen und einen englischen Nachtrag trägt. Die Prosa dieses Projekts ist daneben ungewöhnlich dicht und tragend; sie in einer zweiten Sprache fortzuschreiben, kostet Genauigkeit an genau den Stellen, an denen dieses Projekt sie am nötigsten braucht.
   - Was sie verbaut: die Einsprachigkeit des Bestands, endgültig. Was einmal englisch geschrieben ist, wird nach derselben Regel nicht zurückübersetzt.

2. **Die Deklaration wird zurückgenommen: die Zeile `**Artifact language:** en` fällt aus `CLAUDE.md`.** Damit steuert `**Language:** de` wieder beide Flächen, `bin/fusion-rules` gibt für Langform-Agenten wieder `default-voice-de.yaml` aus, und der Schlusssatz des Abschnitts `## Sprache` stimmt wieder wörtlich.
   - Pro: Stellt die Einsprachigkeit her, die der Bestand tatsächlich hat, und zwar ohne eine einzige Datei zu übersetzen. Die Fallkette ist ausdrücklich dafür gebaut: eine fehlende zweite Zeile heißt „nicht deklariert", und dann gilt die erste für beide Flächen, ohne Warnung und ohne Sonderweg. Elf englische Defektdatensätze bleiben als Bestand stehen, so wie die Regel es für vorhandene Artefakte ohnehin vorsieht.
   - Kontra: Nimmt eine Wahl zurück, die der Nutzer vor zwei Tagen von Hand getroffen hat, und dieser Datensatz weiß nicht, warum er sie getroffen hat. Wenn der Grund war, die Artefakte dieses Projekts für einen englischsprachigen Leser zugänglich zu machen, fällt dieser Zweck ersatzlos weg.
   - Was sie verbaut: nichts dauerhaft. Die Zeile lässt sich jederzeit wieder setzen, und dann greift Möglichkeit 1 ab jenem Tag.

3. **Die Deklaration bleibt und bekommt eine ausdrückliche Übergangsregel im Abschnitt `## Sprache`.** Wie Möglichkeit 1, dazu ein Absatz, der die Grenze mit Datum benennt („für alles, was ab dem 260817 entsteht") und sagt, was mit einem Nachtrag an einer deutschen Datei geschieht: er folgt der Sprache der Datei, in die er geschrieben wird, und nicht der Deklaration.
   - Pro: Löst den einen Fall, an dem die Regel wie geschrieben unbrauchbar wird, nämlich den Nachtrag an einem vorhandenen Datensatz. Genau dieser Fall ist in diesem Baum schon eingetreten und wird mit jedem Abgleich häufiger.
   - Kontra: Trägt eine projekteigene Ausnahme zu einer Regel nach, die fusion selbst ausformuliert hat, und legt sie an einer Stelle ab, die kein Werkzeug prüft. Eine solche Zeile läuft auseinander; dieses Projekt führt für genau diese Gestalt einen offenen Datensatz (`shared/issues/260814-1955_*_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`).
   - Was sie verbaut: nichts, was Möglichkeit 1 nicht auch verbaut; sie ist Möglichkeit 1 mit einer Präzisierung.

## Constraints

- **Die zwei Deklarationszeilen sind formgebunden.** `**Language:** de` und `**Artifact language:** en` stehen in einem festen Format, das `bin/fusion-rules` liest. Sie dürfen umformuliert weder werden noch in einen anderen Abschnitt wandern; die zulässigen Werte sind `en` und `de`. Wer Möglichkeit 2 wählt, **entfernt** die zweite Zeile, statt sie auf `de` zu setzen — beides führt zum selben Ergebnis, aber das Entfernen ist der Weg, den die Regel für ein einsprachiges Projekt beschreibt.
- **Vorhandene Artefakte werden nicht übersetzt.** Das gilt unter jeder der drei Möglichkeiten. Keine Antwort auf diese Frage darf einen Übersetzungsdurchgang über die 163 Entscheidungsdatensätze, die 442 Sitzungsprotokolle oder die 585 Defektdatensätze auslösen.
- **Der Abschnitt `## Sprache` beschreibt den Mechanismus nach dem Kuratorenlauf vom 260819-1500 richtig, gleich wie diese Frage ausgeht.** Der Eintrag L12 jenes Laufs berichtigt allein, welches Profil aus welcher Zeile folgt; er nimmt der Antwort hier nichts vorweg.

## Recommendation

**Möglichkeit 2**, mit einer ausdrücklichen Rückfrage an den Nutzer nach seinem Beweggrund.

Die Begründung ist keine über Sprachen, sondern eine über Kosten. Die Prosa dieses Projekts ist sein wertvollstes Nicht-Code-Erzeugnis: sie trägt die Fallunterscheidungen, die Begründungen und die Warnungen, an denen die Arbeit hängt, und sie ist durchgehend in einer Sprache geschrieben, in der ihr Autor genau ist. Ein Bestand, der ab einem Datum die Sprache wechselt, ohne dass sich der Leserkreis geändert hätte, zahlt diese Genauigkeit und bekommt nichts dafür, solange kein englischsprachiger Leser benannt ist.

**Confidence:** `inference:` — die Empfehlung ruht auf dem gemessenen Bestand (deutsch mit elf englischen Ausnahmen) und auf der Beobachtung, dass der Abgleich vom 260819-1440 die Deklaration bereits übergangen hat. Sie ruht **nicht** auf einer Kenntnis der Absicht des Nutzers beim Setzen der Zeile; die Commit-Message `fusion.json added` sagt dazu nichts. Wenn ein englischsprachiger Leser vorgesehen ist, kippt die Empfehlung auf Möglichkeit 3.

---
Answered:
Implemented: `CLAUDE.md` — die Zeile `**Artifact language:** en` ist entfernt und der Abschnitt `## Sprache` auf den Stand danach gezogen (Commit siehe Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`, Aufgabe T1). Ausgefuehrt vom coder, Protokoll `shared/history/260819-2040-artefaktsprache-deklaration-zurueckgenommen.md`.
Deferred:
Superseded by:

## Antwort 260819-2032

**Möglichkeit 2.** Die Zeile `**Artifact language:** en` fällt aus `CLAUDE.md`; damit steuert
`**Language:** de` wieder beide Flächen, und der Bestand ist wieder einsprachig, ohne dass eine
Datei übersetzt wird.

Der Nutzer hat die Frage an dem Punkt beantwortet, an dem sie unmittelbar Arbeit steuerte: der
Spec der nächsten Runde stand vor dem Schreiben, und die Deklaration hätte ihn englisch verlangt,
in einen Bestand hinein, der bis auf elf Defektdatensätze deutsch ist.

Die elf englischen Defektdatensätze bleiben stehen. `## Project language` sieht für vorhandene
Artefakte ausdrücklich keine Übersetzung vor, und diese Antwort löst keine aus.
