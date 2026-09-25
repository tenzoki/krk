# Was zeigt die Zusammenfassung, wenn ein einzelner Baustein ins Leere greift?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260823-2208_a_liefert-krk-ein-fertiges-fusion-workbench-profil-mit.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`, `crates/krk-core/src/ablage/mod.rs` (`Ersetzung`, `Grund`)

---

## Question

Ein Profil trifft seinen Ort, und ein einzelner Baustein darin findet nichts: die genannte Datei steht nicht da, der genannte Präfix kommt nicht vor, der genannte Unterordner fehlt. Der Fall ist nicht der Ausnahmefall, sondern der Regelfall des mitgelieferten Profils. Der Nutzer hat am 260824-0530 entschieden, dass KRK ein fertiges fusion-workbench-Profil mitliefert und dass es beim ersten Start wirksam wird; ändert fusion seine Ablagekonventionen, greifen einzelne Bausteine ins Leere, während die übrigen weiter stimmen. Bei der Erhebung am 260824-0541 stand `agentstate.yaml` im Beispielbestand gar nicht da, obwohl der Backlogeintrag die Sitzungsinfo an der Wurzel nennt. Die Antwort entscheidet, was der Nutzer in genau dieser Lage sieht, und sie gehört vor die Abnahmekriterien, weil sie den Wortlaut des Kriteriums über die Wurzelzusammenfassung bestimmt.

## Options

1. **Die Zeile entfällt still** — Ein Baustein ohne Wert erzeugt keine Zeile; die Zusammenfassung zeigt, was sie gefunden hat.
   - Pros: Die Anzeige bleibt sauber und trägt nur Wahres. Ein veraltetes Profil verschlechtert die Anzeige nicht, sondern verkürzt sie, und das ist dieselbe Wirkungsrichtung wie beim Profil ohne Treffer, das auf die Metadaten zurückfällt.
   - Cons: Der Nutzer sieht dem Fehlen nichts an. Ein Tippfehler im Präfix und ein tatsächlich leerer Speicher sehen gleich aus, und ein Profil, das zur Hälfte ins Leere greift, wirkt wie ein kurzes Profil.
2. **Die Zeile steht mit einem Platzhalter** — Die Beschriftung bleibt, an der Stelle des Wertes steht ein Zeichen für „nicht gefunden".
   - Pros: Das Fehlen ist sichtbar und an der Stelle, an der es entstanden ist. Der Nutzer erkennt einen Tippfehler in seiner eigenen `readers.toml` sofort, ohne die Datei danebenzulegen.
   - Cons: Ein veraltetes mitgeliefertes Profil füllt die Anzeige mit Platzhalterzeilen. Die Zusammenfassung eines fremden Verzeichnisses, das nur das Pfadmuster erfüllt, bestünde fast ganz aus ihnen.
3. **Die ganze Zusammenfassung fällt auf die Metadaten zurück** — Findet ein Baustein nichts, gilt der Ort als nicht erkannt.
   - Pros: Eine Regel, dieselbe wie beim Profil ohne Treffer, und der Nutzer sieht nie eine halbe Zusammenfassung.
   - Cons: Ein einziger fehlender Baustein nimmt sechs richtige mit. Bei einem mitgelieferten Profil, dessen Pflege der Nutzer nicht in der Hand hat, heißt das: eine Änderung an fusions Ablage schaltet die Anzeige an allen sechs Orten ab, ohne zu sagen, warum.

## Constraints

Der Nutzer hat am 260804-0830 festgelegt, wie KRK Fehler zeigt: die laufenden trägt die Statuszeile, und genau ein Fehler bricht über das modale Hinweisfenster ab. Eine unlesbare `readers.toml` gehört in diese Aufteilung, und ein ins Leere greifender Baustein ist kein Fehler derselben Schwere: er ist eine Aussage über den angezeigten Ort und nicht über die Definitionsdatei. Die Antwort auf diese Frage darf die Aufteilung deshalb nicht verschieben. Ohne Treffer eines Profils bleibt nach dem Entscheid vom 260823 die heutige Metadatenanzeige stehen; jede Antwort hier muss sich von dieser Regel unterscheiden lassen, sonst sind es zwei Regeln für einen Fall.

## Recommendation

Möglichkeit 2. Die Runde liefert ein Profil mit, dessen Pflege dem Projekt und nicht dem Nutzer obliegt, und ein sichtbarer Platzhalter ist die einzige der drei Antworten, an der sich das Veralten überhaupt ablesen lässt. Möglichkeit 1 verbirgt es, und Möglichkeit 3 bestraft sechs richtige Bausteine für einen falschen. Der Preis ist benannt: ein weit veraltetes Profil zeigt eine Liste von Platzhaltern, und das ist die Aufforderung, die `readers.toml` anzufassen, die es auch sein soll.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:79 — Die Zeile steht mit einem Platzhalter (Möglichkeit 2).
Implemented: 260824-1849, Commits `f013227` (Schritt 3) und `abe1a31` (Schritt 6). `Wert::Nicht` ist der Platzhalter `--`; eine beim Laden abgewiesene Zeile behält ihre Beschriftung und trägt ihn dauerhaft, ein Baustein ohne Fund trägt ihn im Einzelfall. Belegt durch `crates/krk-core/tests/leseprofil.rs::das_vorhandensein_antwortet_ja_und_nein_und_die_abgewiesene_zeile_bleibt`.
