# `CLAUDE.md` sagt nicht, dass die tiefe Suche ab Werk steht und ein Anschlag den Unterbaum abläuft

---
**Domain:** code
**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `CLAUDE.md` (Rundentabelle Zeile 10, der Absatz „Das Tippen im Dateifenster filtert seit der Runde 10", der Absatz „Der Durchlauf über den Unterbaum hält genau einen Verzeichnisdeskriptor"); `crates/krk-core/src/verzeichnis/modell.rs:374`; `crates/krk-ui/src/tabs.rs:897`; Commit `20c9833`; `shared/decisions/260826-0923_*_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-jetzt-wo-ein-anschlag-ihn-ab-werk-ausloest.md`

---

## Was ist

`20c9833` hat die Vorbelegung des Ankreuzfelds „Deep" auf „ein" gestellt
(`Ordnermodell::neu`, `crates/krk-core/src/verzeichnis/modell.rs:374`). `CLAUDE.md` sagt darüber
nichts. Selbst gefahren am 260826-1024:

```
grep -no 'Deep[^,.]*\|tiefe[nr]* Suche\|Vorbelegung\|inhaltsschwelle' CLAUDE.md
  → 24, 25 (die Tabellenzeilen 10 und 11, die die zwei Ankreuzfelder benennen)
  → 143    (der Filterabsatz: `inhaltsschwelle` ohne Zahl, und die Regel des Ordnerwechsels)
```

**Keine Falschaussage.** Die drei Stellen beschreiben die Ankreuzfelder und die Filterregeln und
nennen an keiner einen Anfangszustand; sie gelten nach `20c9833` unverändert. Der Befund ist
eine Lücke und keine überholte Zeile. Der `coder` hat das in seiner Übergabe so eingeschätzt,
der `coderev` hat es unabhängig nachgeprüft (`shared/reviews/260826-0923-coderev-…`, Abschnitt
„`CLAUDE.md` wird durch die Deep-Änderung nicht falsch"), und diese Lesung ist hier ein drittes
Mal am Baum bestätigt.

## Warum das zählt

Der Abschnitt „Was man nicht sieht, wenn man es nicht weiß" beansprucht genau diesen Fall für
sich: eine Eigenschaft, die an ihrer Stelle richtig ist und für einen Leser, der nur `CLAUDE.md`
kennt, unsichtbar. Die Folge ist hier keine Kleinigkeit. `Tabliste::durchlauf_nachziehen_an`
stößt den Durchlauf über den Unterbaum an, sobald ein Filtertext steht und die tiefe Suche gilt
(`crates/krk-ui/src/tabs.rs:897`), und „ein Filtertext steht" heißt **ein** Zeichen
(`modell.rs`, `filter_steht`). Der erste Anschlag im Dateifenster startet damit ab Werk einen
Faden, der den Unterbaum abläuft; bis zum 260826 verlangte derselbe Weg vorher einen Klick des
Nutzers.

`CLAUDE.md` führt den Durchlauf in einem eigenen Absatz („hält genau einen
Verzeichnisdeskriptor, gleich wie tief der Baum ist") und beschreibt dort seine Kosten. Wer den
Absatz liest, erfährt, wie teuer der Lauf ist, und nicht, dass ihn seit dem 260826 ein
Tastendruck auslöst. Das ist die Hälfte, die zählt.

Der Baum selbst sagt es an seiner einen Stelle richtig: der Kommentar über `tief: true`
(`modell.rs:371-373`) benennt sogar die Nebenwirkung auf die Schwelle des Inhaltsfilters. Die
Lücke besteht allein in `CLAUDE.md`.

## Was zu tun wäre

Ein Halbsatz im Absatz „Das Tippen im Dateifenster filtert seit der Runde 10" oder im Absatz
über den Durchlauf: die tiefe Suche steht ab Werk, und ein Anschlag genügt, um den Unterbaum
anzulaufen. **Keine Zahl in Prosa** und keine zweite Stelle für die Vorbelegung: sie steht an
genau einer Stelle im Baum, und `CLAUDE.md` hat für diesen Fall die Form, das Kommando statt der
Kopie zu nennen.

Ob der Durchlauf daneben eine eigene Zeichenschwelle bekommt, ist eine andere Frage und liegt
als Entscheidungsdatensatz vor
(`shared/decisions/260826-0923_*_bekommt-der-tiefe-durchlauf-eine-eigene-zeichenschwelle-…`).
Dieser Datensatz verlangt keine Verhaltensänderung, sondern einen Satz.

**Der Abgleich ändert `CLAUDE.md` nicht.** Der Auftrag hat es ausgeschlossen, und die Fläche
gehört dem `curator`.

**Schwere:** mittel. Kein Fehlverhalten. Eine Eigenschaft, die die Kostenrechnung eines
laufenden Vorgangs verschiebt, fehlt in der Datei, die jeder Agent zuerst liest.

**Gefunden:** reconciler, Schlussabgleich der Sitzung `260825-1659` gegen `e5ec81a..c95f28b`.

---
Resolved: 88f18ed — der Filterabsatz in `CLAUDE.md` sagt jetzt, dass die tiefe Suche ab Werk steht und schon der erste Anschlag den Durchlauf über den Unterbaum anstößt; er zeigt dafür auf `Ordnermodell::neu` statt eine Zahl zu doppeln, und zitiert die offene Frage zur Inhaltsschwelle, ohne sie zu beantworten.
