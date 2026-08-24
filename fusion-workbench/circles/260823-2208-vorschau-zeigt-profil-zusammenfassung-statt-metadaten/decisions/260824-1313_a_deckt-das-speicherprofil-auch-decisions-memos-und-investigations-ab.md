# Deckt das Speicherprofil auch `decisions`, `memos` und `investigations` ab?

---
**Domain:** data
**Filed by:** ontocoder
**Cross-references:** `planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (C5.2, C5.3), `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md` (Schritt 7), `resources/default-readers.toml`

---

## Question

Das Pfadmuster des Speicherprofils zählt sechs Speichernamen auf:
`analyses`, `backlog`, `consult`, `history`, `planning`, `reviews`. Der
Defektspeicher steht als eigenes Profil daneben. Damit bleiben drei Speicher
ohne Profil, und einer davon ist der meistbenutzte der Werkbank: **jeder
`decisions`-Ordner**, gemeinsam wie in jeder Runde. Dazu kommen
`shared/memos` und `shared/investigations`.

Gemessen am 260824-1313 mit der fertigen `resources/default-readers.toml` gegen
den echten Bestand dieser Werkbank: 78 Ordner treffen das Speicherprofil, 19 den
Defektspeicher, **21 keines von beiden** — `shared/decisions`,
`shared/investigations`, `shared/memos` und die 18 `decisions`-Ordner der
Runden. Sie zeigen weiter die Metadatenanzeige.

Das ist kein Abweichen von der Vorgabe: C5.2 zählt die sechs Namen einzeln auf,
Schritt 7 des Plans schreibt genau dieses Muster vor, und beide sind vom Nutzer
freigegeben. Die Frage ist, ob die Auslassung gewollt war. Der Circle-Baustein
zählt die Entscheidungsdatensätze einer Runde, die Runde weiß also von diesem
Speicher; nur beim Betreten sagt er nichts über sich.

## Options

1. **Es bleibt bei den sechs** — die Auslassung war gewollt, und ein
   Entscheidungsspeicher trägt seinen Stand ohnehin im Markervokabular, das ein
   Zählmuster nicht in einer Zeile abbildet.
   - Pros: Keine Änderung. Die freigegebenen Kriterien C5.2 und C5.3 bleiben im
     Wortlaut stehen.
   - Cons: Der Speicher, den `CLAUDE.md` als „bindende Grundlage" führt, ist der
     einzige, den die Vorschau nicht zusammenfasst.
2. **Die drei Namen kommen in dieselbe Aufzählung** — aus sechs Alternativen
   werden neun, und `decisions`, `memos`, `investigations` zeigen Zahl und
   jüngste zehn wie die übrigen.
   - Pros: Zwei Zeilen TOML, kein neues Profil, kein neuer Baustein. Der
     Haushalt ändert sich nicht.
   - Cons: Ein Entscheidungsspeicher bekommt damit dieselbe Zusammenfassung wie
     ein Analysespeicher, obwohl seine fünf Marker mehr hergäben. C5.2 und C5.3
     ändern ihren Wortlaut.
3. **`decisions` bekommt ein eigenes Profil wie der Defektspeicher** — mit je
   einer Zählung für offen, beantwortet und umgesetzt, dazu die jüngsten zehn;
   `memos` und `investigations` kommen in die Aufzählung aus Möglichkeit 2.
   - Pros: Die Zusammenfassung beantwortet, was man an einem
     Entscheidungsspeicher wissen will, nämlich wie viel Grundlage noch offen
     ist.
   - Cons: Ein sechstes Profil und fünf Zeilen mehr. Der Spec bekäme ein
     siebtes Abnahmekriterium in C5, das der Nutzer nicht freigegeben hat.

## Constraints

Jede Antwort bleibt innerhalb des festen Bausteinsatzes aus C3; ein fünfter
Baustein kommt in dieser Runde nicht dazu. Die Grenzen aus C6 sind von allen
drei Möglichkeiten unberührt: keine fügt einem bestehenden Profil eine Zeile
hinzu, und ein neues Profil zählt seinen Haushalt für sich.

## Recommendation

Möglichkeit 2. Sie kostet zwei Zeilen und schließt die Lücke, und die
Unterscheidung nach Markern, die Möglichkeit 3 brächte, ist eine eigene Frage,
die diese Runde nicht stellen muss: sie ließe sich später ohne Bruch
nachziehen, indem `decisions` aus der Aufzählung wieder herausgenommen wird und
ein eigenes Profil bekommt.

---
Answered: 260824-1505 — Möglichkeit 2. Aus sechs Alternativen des Pfadmusters
werden neun: `decisions`, `memos` und `investigations` kommen hinzu und zeigen
Zahl und jüngste zehn wie die übrigen Speicher.

**Der Preis ist gemessen und beträgt null.** Betroffen ist das Pfadmuster, also
die Erkennung, und kein Baustein. Kein neues Profil, kein neuer Baustein, keine
Zeile mehr in einem vorhandenen Profil; die Grenzen aus C6 sind unberührt. Die
Änderung schließt die Lücke, die der Datensatz am Bestand gemessen hat: 21
Ordner ohne Profil, darunter alle achtzehn Entscheidungsspeicher.

**C5.2 und C5.3 werden berichtigt statt überschrieben**, in derselben Form wie
die vier Kriterien vom 260824-1250: die Berichtigung steht neben dem
freigegebenen Wortlaut und nicht an seiner Stelle.

**Die Unterscheidung nach Markern bleibt offen und ist kein Rückstand.** Ein
eigenes Profil für `decisions` — Möglichkeit 3 — ließe sich später ohne Bruch
nachziehen, indem der Name aus der Aufzählung wieder herausgenommen wird. Wer
das aufgreift, stellt dafür eine eigene Frage.

Implemented:
Deferred:
Superseded by:
Retired:
