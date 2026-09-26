# Was tut `return` in einer Notizzelle, und welche Tasten tragen die zwei Editoren?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0107-zweitlesung-plan-f2-krkhome.md

---

## Question

Der Plan schlägt Tasten für die Editorbefehle vor und lässt `return` in einer Notizzelle einen Zeilenumbruch schreiben. Die Zweitlesung des Plans verlangt, dass der Nutzer über `return` entscheidet, weil die Mac-Konvention für Tabellen das Gegenteil ist, und merkt an, dass `shift+cmd+delete` im Finder „Papierkorb entleeren“ heißt.

## Options

1. **`return` schreibt einen Zeilenumbruch**, beendet wird mit `cmd+return` oder einem Klick daneben.
   - Pros: Notizen sind mehrzeilig, der häufige Fall braucht keine Zusatztaste.
   - Cons: weicht von der Mac-Konvention für Tabellenzellen ab.
2. **`return` beendet die Bearbeitung**, ein Umbruch mit `opt+return`.
   - Pros: Mac-Konvention.
   - Cons: jede Zeile einer mehrzeiligen Notiz braucht die Zusatztaste.

Tasten wie im Plan: `shift+cmd+return` anlegen, `cmd+return` bearbeiten und Zelle übernehmen, `opt+cmd+up`/`opt+cmd+down` verschieben, `shift+cmd+delete` löschen, `shift+cmd+x` abhaken, `shift+cmd+p` PIN ändern; keine davon ist in `resources/default-keymap.toml` oder der Textbelegung von macOS belegt.

## Constraints

- Jede Taste bleibt in der Belegungsansicht (F1) änderbar.

## Recommendation

Möglichkeit 1 und die Tasten wie im Plan.

---
Answered: dieser Datensatz `## Recommendation` — Möglichkeit 1, `return` schreibt einen Zeilenumbruch; die sieben Tasten wie im Plan, `shift+cmd+delete` trotz der Finder-Bedeutung; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: d356ca6 — return schreibt im Notiztext einen Umbruch; die Tasten in 970abb0 und d3a9983
