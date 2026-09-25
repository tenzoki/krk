# Woher nimmt die Wurzelzusammenfassung ihre Sitzungsinfo?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`, `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_was-zeigt-die-zusammenfassung-wenn-ein-baustein-ins-leere-greift.md`, `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260823-2208_a_liefert-krk-ein-fertiges-fusion-workbench-profil-mit.md`

---

## Question

Die Zusammenfassung der Werkbankwurzel trägt nach der Skizze des Nutzers sieben Angaben, und
sechs davon sind am Bestand auffindbar: Projektname und Setup-Datum und Plugin-Version aus
`.fusion-setup`, der aktive Circle aus `.active-circle`, die Zahl der Circles aus `circles/`,
die Zahl der offenen Defekte aus `shared/issues/`. Die siebte, „Sitzungsinfo", nennt keine
Datei, und die Datei, die am nächsten läge, steht nicht da: `agentstate.yaml` fehlt in dieser
Werkbank am 260824-0600, wie schon bei der Erhebung am 260824-0541. Sie ist in `.gitignore`
geführt und entsteht erst, wenn der Orchestrator Zustand schreibt.

Die Frage zählt, weil KRK das Profil mitliefert und der Nutzer es nicht pflegt (Entscheid vom
260824-0530) und weil ein Baustein, der ins Leere greift, seit dem 260824-0555 einen sichtbaren
Platzhalter setzt. Eine Zeile, die auf `agentstate.yaml` zeigt, stünde also im Auslieferungsstand
als Platzhalter da, und zwar dauerhaft, solange keine Sitzung läuft. Die Antwort bestimmt den
Wortlaut des Abnahmekriteriums über die Wurzelzusammenfassung.

## Options

1. **Auf `orchestrator-live.md` zeigen** — Die Datei steht da, wird an Ort und Stelle
   fortgeschrieben und trägt in ihren ersten Zeilen Turn, Aufgaben, Commits, Fehler,
   Startzeit und den laufenden Schritt.
   - Pros: Der Auslieferungsstand zeigt sofort etwas Wahres, und zwar genau die Auskunft,
     die „Sitzungsinfo" meint. Die Datei ist die einzige der drei Sitzungsflächen, die in
     dieser Werkbank tatsächlich vorhanden ist.
   - Cons: Sie ist eine Anzeigedatei für Menschen, kein Datenformat, und ihre Zeilenform kann
     sich mit jeder fusion-Fassung ändern. Der Ausdruck, der eine Angabe herauszieht, hängt
     dann an einem Layout und nicht an einem Schlüssel.
2. **Auf `agentstate.yaml` zeigen und den Platzhalter in Kauf nehmen** — Die Zeile steht, und
   solange die Datei fehlt, steht der Platzhalter.
   - Pros: `agentstate.yaml` ist das erklärte Zustandsformat und schlüsselbasiert, also der
     haltbarere Bezugspunkt. Der Platzhalter sagt zutreffend, dass gerade keine Sitzung
     Zustand geschrieben hat.
   - Cons: Der erste Eindruck der Runde wäre in dieser Werkbank ein Platzhalter an der
     Wurzel, und der Nutzer kann nicht unterscheiden, ob sein mitgeliefertes Profil veraltet
     ist oder nur keine Sitzung läuft.
3. **Die Sitzungsinfo entfällt im mitgelieferten Profil** — Sechs Angaben statt sieben; wer
   die siebte will, trägt sie selbst nach.
   - Pros: Die mitgelieferte Datei behauptet nichts, was sie nicht halten kann, und trägt
     keine Zeile, deren Quelle von der Laufzeit abhängt.
   - Cons: Eine der sieben skizzierten Angaben fällt aus der Runde, und gerade die, die sich
     am häufigsten ändert. Der Nutzer bekommt sie nur, wenn er die Datei selbst anfasst.

## Constraints

Der Entscheid vom 260824-0555 gilt: ein Baustein ohne Wert setzt einen sichtbaren Platzhalter
und lässt die übrigen Zeilen stehen. Der Entscheid vom 260824-0530 gilt: das Profil wird
mitgeliefert und ist beim ersten Start wirksam, und seine Pflege liegt beim Projekt. Eine
Antwort, die den Auslieferungsstand mit Platzhaltern füllt, verschiebt damit die Wirkung
dieses zweiten Entscheids, ohne ihn aufzuheben.

## Recommendation

Möglichkeit 1. Die Runde wird an ihrem sichtbaren Ergebnis abgenommen, und `orchestrator-live.md`
ist die einzige der drei Kandidatinnen, die in dieser Werkbank etwas zu zeigen hat. Das Risiko
ist begrenzt und benannt: ändert fusion die Zeilenform, greift dieser eine Baustein ins Leere
und setzt seinen Platzhalter, während die sechs übrigen Angaben der Wurzel weiter stimmen.
Genau diese Wirkungsrichtung war der Grund, den Platzhalter zu wählen.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:93 — Sitzungsinfo aus orchestrator-live.md (Möglichkeit 1).
Implemented: 260824-1849, Commit `8433935`, Schritt 7 des Plans; das Feldmuster ist mit `942172b` ersetzt worden. Die Zeile „Sitzung" des Wurzelprofils in `resources/default-readers.toml:209` zieht ihren Wert über ein Feldmuster aus `orchestrator-live.md`. Belegt durch `crates/krk-core/tests/leseprofil.rs::die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`, die den Wert der Zeile gegen einen Prüfordner in Werkbankgestalt hält.
