# Wohnt die Git-Anbindung in `krk-core` oder in einer fünften Kiste `krk-git`?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 5); `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_*_sprache-und-ui-werkzeugkasten.md`; `crates/krk-core/src/lib.rs:13-15`; die Zuordnungsgründe zu `icu_collator`, `regex` und `zip` in `crates/krk-core/Cargo.toml`

---

## Question

Der Workspace führt vier Mitglieder, und genau eines davon ist die Bibliothek ohne Fensterwerkzeug. `gix` bringt auf dem Bauziel 98 weitere Pakete mit, gemessen am 260830 mit `cargo tree -e normal` gegen `x86_64-apple-darwin`, und darunter `libc` und `rustix`; `cargo tree -p krk-core --target x86_64-apple-darwin -e normal` führt heute null Vorkommen von `libc`. Die Frage ist zu entscheiden, bevor der Plan der Stufe A geschrieben wird, weil sie den Zuschnitt des Workspace berührt und weil jede fremde Kiste dieses Projekts ihre Begründung an einer bestimmten Stelle trägt.

## Options

1. **Modul `krk-core/src/git/`, `gix` als Abhängigkeit von `krk-core`** — dieselbe Einordnung, die `icu_collator`, `regex` und `zip` bekommen haben.
   - Pros: folgt der Regel, die dieses Projekt dreimal ausgeschrieben hat, nämlich dass alles ohne Fensterwerkzeug in den Kern gehört und dort geprüft wird; `krk-ui` hat kein Bibliotheksziel, eine Datei unter `crates/krk-ui/tests/` erreicht nichts aus jener Kiste, und die Proben brauchen den selbstabräumenden Prüfordner aus `crates/krk-core/tests/gemeinsam/mod.rs`; ein Gitleser liefert Namen, Marken, Hashes und Zeitpunkte und damit keine Darstellung, was ihn von `syntect` und `pulldown-cmark` trennt.
   - Cons: `cargo test -p krk-core` übersetzt fortan 98 zusätzliche Pakete mit; `krk-core` bekommt `libc` in seinen Teilbaum, was heute nicht der Fall ist.
2. **Fünfte Kiste `krk-git` neben den vier bestehenden** — `krk-ui` bindet sie, `krk-core` sieht sie nicht.
   - Pros: hält die 98 Pakete und `libc` aus dem Teilbaum des Kerns heraus; eine spätere Ablösung von `gix` bliebe auf eine Kiste beschränkt.
   - Cons: es gäbe dann zwei Kisten ohne AppKit mit derselben Begründung, und bei jedem künftigen fensterfreien Modul wäre neu zu fragen, in welche von beiden es gehört, also das Sonderfall-Dickicht aus `critical-stance.md` §2; die Kisten dieses Projekts sind bisher nach Programmen geschnitten (`krk-bench`, `xtask`) und nicht nach Abhängigkeiten; der Gewinn ist klein, weil die Übersetzung zwischengespeichert wird und `krk-ui` die Kiste ohnehin über `krk-core` mitzieht, sobald der Gitbefund ins Ordnermodell geht.
3. **Modul in `krk-ui`** — der Gitleser wohnt bei der Oberfläche.
   - Pros: keine neue Abhängigkeit für den Kern.
   - Cons: nicht ohne Fenster prüfbar, weil `krk-ui` kein Bibliotheksziel führt; und der Gitbefund muss ohnehin in `Ordnermodell` hinein, das im Kern liegt, also überquerte der Datenweg die Kistengrenze zweimal.

## Constraints

- Der Kern kennt AppKit nicht (`crates/krk-core/src/lib.rs:13-15`); daran ändert keine Antwort etwas.
- Jede fremde Kiste steht mit Begründung und Merkmalsauswahl in der Wurzel-`Cargo.toml`.
- Die Proben der Stufe A brauchen ein angelegtes Prüfrepository und damit den selbstabräumenden Prüfordner; von dem gibt es drei anerkannte Fassungen, eine je Kiste, und eine vierte anzulegen ist ausgeschlossen (`crates/krk-core/tests/baum.rs`, Zählprobe).
- `#![deny(unsafe_code)]` gilt in jeder Kiste; `gix` selbst trägt `#![deny(missing_docs, unsafe_code)]` und ändert daran nichts.

## Recommendation

Wir empfehlen Möglichkeit 1. Die Regel, nach der dieses Projekt seine Kisten schneidet, ist die Prüfbarkeit ohne Fenster, und sie ist an drei fremden Kisten schon angewandt worden; eine vierte Kiste einzuführen, um eine Abhängigkeitszahl zu drücken, ersetzt eine klare Regel durch eine Ermessensfrage. Der genannte Nachteil ist real und messbar, aber er trifft die Bauzeit und nicht die Ausführung.

Wer Möglichkeit 2 wählt, sollte die Regel dazu ausschreiben, nach der künftig zwischen `krk-core` und `krk-git` entschieden wird; ohne sie ist die zweite Kiste die teurere Antwort.
