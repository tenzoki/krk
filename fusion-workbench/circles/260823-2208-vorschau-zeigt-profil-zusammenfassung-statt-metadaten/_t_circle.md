# Das Vorschaufenster zeigt für erkannte Orte eine Profil-Zusammenfassung statt der Metadaten

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode)
**Active spec/plan:** circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md
**Active session history:** circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

**Die Anzeige, die ersetzt wird, ist die Metadatenanzeige der Vorschau.** `krk-ui/src/vorschaumodell.rs` führt `Inhalt` mit drei Zweigen: Text bis 1 MB, Bild bis 64 MB, sonst `Inhalt::Metadaten`. Ein Ordner endet dort immer (`laden`, der frühe Rückgabezweig für Ordner und Verknüpfungen). Ob die Zusammenfassung ein vierter Zweig wird oder eine Nutzlast des vorhandenen, entscheidet der Plan und nicht diese Runde.

**Der Bestandsort trägt heute sechs Dateien, und die siebte hält den Bau an.** `krk-core/src/ablage/pfade.rs` führt die Aufzählung `Datei` mit `Datei::ALLE` und ohne Auffangzweig; `readers.toml` wird die siebte. Die Vorlage dafür ist `settings.toml`: `krk-core/src/ablage/einstellungen.rs` legt sie beim ersten Start wörtlich aus einer über `include_str!` eingebetteten Auslieferungsfassung an und schreibt sie danach nie wieder, damit ihre Kommentarzeilen stehen bleiben. Eine von Hand gepflegte Definitionsdatei ist genau diese Sorte. Daneben steht `Leerbefund` in derselben Datei: eine Ablagedatei ohne einen einzigen obersten Schlüssel bedeutet je nach Schreiber `Vorgabe` oder `Beschädigt`, und für eine handgepflegte Datei ist es `Vorgabe`.

**TOML steht schon im Baum, ein Mustervergleich nicht.** Die Wurzel-`Cargo.toml` führt `serde` und `toml = "1"` als Arbeitsbereichs-Abhängigkeiten; das Lesen der Definitionsdatei braucht keine fremde Kiste. Für die Ortserkennung über ein Pfadmuster gibt es dagegen nichts Vorhandenes: eine Suche nach Glob- oder Mustervergleich in `krk-core/src` liefert keinen Treffer.

**Das Zählen und Lesen, das eine Zusammenfassung braucht, gibt es als Maschinerie bereits.** `krk-core/src/verzeichnis/durchlauf.rs` läuft über einen Unterbaum und hält dabei genau einen Verzeichnisdeskriptor, gleich wie tief der Baum ist; `krk-core/src/verzeichnis/filter.rs` liest Dateiinhalt bis zu einer Grenze und gibt den Dateideskriptor vor dem nächsten Kandidaten wieder frei. Zählungen und die zehn jüngsten Titel eines Speichers gehören auf diesen Weg und nicht auf einen zweiten daneben.

**Die Zusammenfassung fällt in die Endbedingung der Zeitzusage L7.** L7 sagt zu: „Vorschau einer Textdatei bis 1 MB sichtbar, sonst die Metadaten" in 100 ms (Spec der Runde 1, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`, Abschnitt C8). Die Metadatenanzeige eines Ordners liegt heute in diesem „sonst"; eine Zusammenfassung, die mehrere Dutzend Dateien liest, arbeitet damit innerhalb einer bestehenden Zusage. L7 steht seit dem 260819-2242 ohnehin auf den Gegenständen der späteren Messrunde (`shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`).

**Fehler nehmen den vorhandenen Weg.** Laufende Fehler trägt die Statuszeile, und genau ein Fehler bricht über das modale Hinweisfenster ab; so entschieden am 260804-0830 (`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`, Modulkopf `krk-ui/src/appkit/hinweis.rs`). Eine unlesbare `readers.toml` und eine Profilregel, die ins Leere greift, gehören in diese Aufteilung und nicht in eine eigene.

**Was der Nutzer am 260823 entschieden hat**, in einer Klärungsrunde zu vier Fragen: die Runde nimmt allein die Zusammenfassung und nicht die Leseoperationen, die der Backlogeintrag daneben nennt; eine Profilregel rechnet aus einem festen Bausteinsatz und trägt keine eigene Ausdruckssprache; ein Profil erkennt seinen Ort über Pfadmuster **und** Kennzeichendatei, wobei das Pfadmuster vorgeht und ohne Treffer die heutige Metadatenanzeige stehen bleibt; die Profile stehen in einer eigenen `readers.toml` und nicht in `settings.toml`. Der Entwurf sprach von `krk-rc.yaml`; die Wahl der eigenen TOML-Datei ersetzt beides, Name und Format.

**Der Backlogeintrag bleibt offen.** `shared/backlog/260823-2136_o_readerconventions-profile-fuer-dateizugriff.md` nennt zwei Hälften, „welche Orte welche Leseoperationen erfordern" und „was im Vorschaufenster erscheint". Diese Runde nimmt die zweite. Der Eintrag ist deshalb nicht als übernommen geschlossen: eine Schließung nähme die erste Hälfte ungelesen mit.

**Zwei Fragen dieser Runde sind offen und stehen als Datensätze in `decisions/` dieses Circles:** ob ein Profil nur für Ordner oder auch für einzelne Dateien gilt, und ob KRK ein fertiges fusion-workbench-Profil mitliefert.

## Dependencies

(keine)

## Turn log

## Activation proposal

**Vorgeschlagen zur Aktivierung am 260823-2241, durch den Playmaker-Lauf `260823-2241`.**

Dieser Circle ist der einzige vorgesehene im Portfolio, und er ist der am besten
vorbereitete, den das Projekt seit dem 260821 hatte. Sein Abschnitt `## Dependencies` ist
leer, es gibt also keine Vorbedingung, die auf einen Abschluss wartet. Seine Grundlage
zitiert keinen einzigen offenen Entscheidungsdatensatz aus einem fremden Speicher: die vier
Verweise nach draußen führen auf
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`,
auf `circles/260802-0842-…/decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`
(umgesetzt), auf `shared/decisions/260819-2216_*_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`
(beantwortet) und auf den Rückstandseintrag, aus dem die Runde stammt. Offen sind allein
die zwei Fragen, die der Shaper am 260823 in `decisions/` dieses Circles abgelegt hat: ob
ein Profil nur für Ordner oder auch für einzelne Dateien gilt, und ob KRK ein fertiges
fusion-workbench-Profil mitliefert. Beide sind Fragen der Runde selbst und gehören in ihre
Klärung, nicht vor ihre Aktivierung.

**Zwei Punkte, die der Plan zu tragen hat.** Der Bestandsort trägt heute sechs Dateien in
einer Aufzählung ohne Auffangzweig, und `readers.toml` wird die siebte; das ist eine
Übersetzerprüfung und keine Fleißarbeit. Und die Zusammenfassung fällt in die Endbedingung
der Zeitzusage L7, die seit dem 260819-2242 ohnehin auf den Gegenständen der späteren
Messrunde steht.

Der Playmaker benennt den Marker nicht um. Die Aktivierung fährt der Nutzer über
`/fusion:next`, oder der Orchestrator nach bestätigtem Vorschlag.
