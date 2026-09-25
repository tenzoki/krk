# Für welche Sprachen hebt die Formatansicht Syntax hervor, und was ist ein einklappbarer Block?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/decisions/260802-0842_*_editor-formatansicht-je-dateityp.md` (die beantwortete Vorfrage), `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md` §"1. Formatansicht", `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md` (C3)

---

## Question

Der Nutzer hat am 260807-2139 die erste Möglichkeit aus `shared/decisions/260802-0842_*_editor-formatansicht-je-dateityp.md` gewählt: eine Formatansicht je Dateityp, und für Code heißt das Syntaxhervorhebung mit einklappbaren Blöcken. Damit ist entschieden, **was** die Formatansicht bei Code zeigt. Offen ist, **wofür**, und diese zweite Frage bestimmt den Umfang der Runde stärker als jede andere.

Der Grund ist, dass Syntaxhervorhebung keine Fähigkeit ist, die man einmal baut, sondern eine je Sprache. Jede Sprache braucht die Kenntnis ihrer Schlüsselwörter, ihrer Zeichenketten, ihrer Kommentare und ihrer Blockgrenzen. Zwischen "Rust und TOML" und "die vierzig Sprachen, die ein üblicher Editor kennt" liegt eine Größenordnung an Arbeit, und zwischen "von Hand geschriebene Regeln" und "eine fremde Kiste einbinden" eine Entscheidung über eine Abhängigkeit, die das Projekt bisher nur viermal getroffen hat.

Die einklappbaren Blöcke tragen dieselbe Frage ein zweites Mal. Ein Block ist in Rust und C etwas anderes als in Python, wo die Einrückung ihn bildet, und wieder etwas anderes in Markdown, wo eine Überschrift ihren Abschnitt aufspannt. Ohne Antwort baut die Runde entweder eine Regel, die nur für geschweifte Klammern greift, oder drei Sonderfälle nebeneinander.

Die Frage ist nicht durch die Wahl vom 260807-2139 mit beantwortet. Jene Wahl stand zwischen drei Zuschnitten der Ansicht und nannte keine Sprache.

## Options

1. **Die Sprachen dieses Projekts, von Hand** — Rust, TOML, Markdown und Shell, jede mit einer eigenen, im Projekt geschriebenen Regel. Alles übrige bekommt die Textansicht mit Umbruch. Einklappbar sind geschweifte Klammern in Rust, Tabellen in TOML und Abschnitte unter Überschriften in Markdown.
   - Pro: keine neue Abhängigkeit, keine fremde Datenbasis, vollständige Kontrolle über die Geschwindigkeit. Deckt genau die Dateien ab, die der Nutzer in KRK selbst bearbeitet.
   - Contra: vier Regeln, die je einzeln gepflegt sein wollen, und eine fünfte Sprache verlangt eine fünfte. Wer eine Python-Datei öffnet, sieht unformatierten Text.

2. **Eine gängige Kiste einbinden** — eine Rust-Kiste für Syntaxhervorhebung übernimmt Erkennung und Einfärbung für einige Dutzend Sprachen; das Projekt schreibt keine Sprachregel selbst.
   - Pro: deckt weit mehr ab, als vier von Hand geschriebene Regeln je erreichen, und mit einem Bruchteil des Aufwands.
   - Contra: eine Abhängigkeit mit eigener Datenbasis, eigener Größe und eigener Geschwindigkeit. Ob eine solche Kiste die Maxime "superschnell" auf dem Referenzgerät von 2018 hält, ist ungemessen, und der Abnahmelauf, an dem man es messen würde, ist aus dieser Runde ausgeklammert. Die Einklappbarkeit bringen solche Kisten in aller Regel nicht mit.

3. **Eine Sprache in dieser Runde, weitere später** — Markdown wird gerendert, wie es die Wahl vom 260807-2139 ohnehin verlangt, und Code bekommt in dieser Runde die Textansicht mit Umbruch. Syntaxhervorhebung und Einklappbarkeit kommen als eigenes Vorhaben.
   - Pro: hält die Runde auf dem, was ohne eine Entscheidung über Sprachen und Abhängigkeiten baubar ist. Der Editor steht damit früher.
   - Contra: weicht von der Festlegung des Nutzers vom 260807-2139 ab. Er hat Syntaxhervorhebung für Code ausdrücklich gewählt.

## Constraints

- Die Wahl vom 260807-2139 steht: Markdown wird gerendert, Code bekommt Syntaxhervorhebung mit einklappbaren Blöcken, einfacher Text bekommt Umbruch am Fensterrand und eine lesbare Schriftgröße. Eine Antwort, die Code die Hervorhebung ganz nimmt, ändert diese Wahl und braucht die Zustimmung des Nutzers.
- Die Technologiewahl der Runde 1 bindet: Rust mit AppKit über `objc2`, außerhalb der App-Sandbox (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1134_i_sprache-und-ui-werkzeugkasten.md`).
- Das Projekt führt heute vier fremde Kisten mit Wirkung auf die Anwendung, jede mit einer geschriebenen Begründung in `Cargo.toml`. Eine fünfte fügt sich in dieses Muster ein und braucht dieselbe Begründung.
- Der Abnahmelauf ist aus dieser Runde ausgeklammert. Eine Antwort, deren Tragfähigkeit von einer Messung abhängt, kann in dieser Runde nicht abgenommen werden.

## Recommendation

Wir empfehlen keine der drei, weil die Wahl davon abhängt, was der Nutzer mit dem Editor tun will, und das wissen wir nicht. Wer KRK zum Bearbeiten seiner eigenen Projektdateien benutzt, ist mit Möglichkeit 1 vollständig bedient; wer beliebige Quelltexte fremder Projekte durchsieht, ist es nicht.

Was wir sagen können, ist der Unterschied im Preis. Möglichkeit 1 ist Arbeit im Projekt und kostet je Sprache, aber nichts an Abhängigkeit und nichts an ungemessener Geschwindigkeit. Möglichkeit 2 ist eine einmalige Einbindung und kostet eine Abhängigkeit sowie eine offene Frage zur Geschwindigkeit, die in dieser Runde nicht zu schließen ist. Möglichkeit 3 verschiebt beides und ändert dafür eine Festlegung, die der Nutzer erst gestern getroffen hat.

Ein Hinweis zur Einklappbarkeit, unabhängig von der Wahl: sie ist eine eigene Fähigkeit und keine Zugabe zur Hervorhebung. Beide brauchen Kenntnis der Sprache, aber verschiedene: die Hervorhebung braucht Wortarten, die Einklappbarkeit braucht Blockgrenzen. Eine Antwort, die nur die erste beschafft, hat die zweite nicht mit erledigt.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"5. Sprachen der Syntaxhervorhebung" und §"6. Einklappbare Blöcke" — Möglichkeit 2 gewählt: eine fertige Rust-Kiste übernimmt Erkennung und Einfärbung; das Projekt schreibt keine Sprachregel selbst. Sie wird die fünfte fremde Kiste mit Wirkung auf die Anwendung und braucht eine Begründung in Cargo.toml. Zwei Preise sind angenommen: die Geschwindigkeit auf dem Referenzgerät ist ungemessen (der Abnahmelauf ist aus dieser Runde ausgeklammert), und die Kiste bringt die einklappbaren Blöcke nicht mit. Die einklappbaren Blöcke entfallen deshalb in dieser Runde und kommen als eigenes Vorhaben; die Festlegung vom 260807-2139 ("Syntaxhervorhebung mit einklappbaren Blöcken") ist damit zur Hälfte zurückgenommen. Entschieden vom Nutzer am 260808-0017.
Implemented: `ef47206` und `41309cc` — `syntect` 5.3.0 trägt Erkennung und Einfärbung, `two-face` 0.5.2 bringt den erweiterten Sprachsatz mit TOML nach; beide stehen mit geschriebener Begründung in `Cargo.toml:103-161`, beide ohne Vorgabemerkmale. Der Sprachsatz wird in `crates/krk-ui/src/hervorhebung.rs:315` über `two_face::syntax::extra_newlines` geladen. Die vier zugesagten Sprachen sind in `crates/krk-ui/tests/syntaxkiste.rs:23` abgenommen. Der zweite angenommene Preis ist im Baum eingelöst: einklappbare Blöcke sind nicht gebaut, eine Suche nach `einklapp` und `klappbar` über `crates/` und `resources/` liefert null Treffer. Planschritte S32, S33 und S34 tragen `[DONE]`. Nachgeprüft im Abgleich am 260810.

Der beim Entscheid angenommene Preis „Geschwindigkeit ungemessen" ist inzwischen gemessen und fällt schlecht aus: `issues/260810-0054_o_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md` weist 0,3 MB/s nach. Das ist ein offener Defekt an der umgesetzten Antwort und kein Grund, die Antwort als nicht umgesetzt zu führen.
