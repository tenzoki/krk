# Orchestrator-Sitzung — 260813-2342

**Directive:** Ein Notizzettel als Blatt am Hauptfenster: zwei Zettel als anklickbare Tabs, nackte Textfläche, gesichert bei Tabwechsel, Schließen und Beenden, geholt mit `f2` oder `cmd+k`, geschlossen mit `Esc`, zwei einzelne Dateien im Ablageordner.
**Modus:** (Phase 0 offen)
**Status:** Läuft

## Aufnahme beim Start

| Größe | Wert |
|---|---|
| Aktiver Circle | 260813-2332-notizzettel-als-blatt-mit-zwei-zetteln (aktiviert 23:41 über /fusion:next mit Namen) |
| git HEAD | 6d05bef |
| Turn-Budget | 5 |
| Erkannte Domäne | code (132 Quelldateien, 11 Datendateien, git-ls-files) |
| Offene Defekte | 0 im Circle, 10 in shared |
| Offene Fragen | 0 im Circle, 7 in shared |
| Offene Pläne | 1 in shared |
| Guard | haltActive: false |
| Circles | 1 aktiv, 1 vorgesehen, 7 beschränkt, 1 kohärent geschlossen |
| Arbeitswarteschlange | keine tasklist.md |

## Was diese Runde vorfindet

Sieben Klärungsfragen sind vor der Anlage des Circles beantwortet worden, in zwei Runden des shapers. Vor dem Spec steht damit keine offene Frage. Die Grounding-Aufnahme des Circle-Datensatzes trägt drei benannte Folgen — Absturzverlust, Überschreibgefahr bei zwei Instanzen, und die Verträglichkeit von nackter Fläche und Blattform — sowie drei Funde des shapers am Baum.

Vorgängersitzung: shared/history/260813-1006 gibt es nicht; die achte Runde lief unter circles/260813-0939-…/history/260813-1006-orchestrator-session.md und ist kohärent geschlossen.

## Drei Antworten am Spec-Tor (Nutzer, 260814-0005)

**`shift+cmd+w` bei stehendem Zettel: sichern, dann schließen.** Ein vierter Sicherungsmoment neben Tabwechsel, Schließen und Beenden. Der Grund ist die Logik der anderen drei: kein Weg aus dem Zettel heraus verliert Text. Die Ausnahmeliste bleibt unangetastet — `fenster_schliessen` steht seit dem 260813-1125 ausdrücklich darauf, und der Entscheid dazu wird nicht gekippt.

**Unlesbare Zetteldatei: beiseitelegen und mit einem leeren Zettel weiterarbeiten.** Möglichkeit 3 des Datensatzes, die Empfehlung des shapers. Es ist die Antwort, die dieses Projekt für `keymap.toml` und `settings.toml` schon gegeben hat: ein Tippfehler nimmt dem Nutzer die Datei nicht weg. Kein zweiter Zustand am Zettel, keine Sperre, keine Ausnahme im Sicherungsweg; der Preis ist ein sechster Aufrufer von `beiseite_legen` und eine Datei mehr im Ablageordner.

**Spec: erst nachziehen, dann freigeben.** Der shaper arbeitet die beiden Antworten ein, ergänzt das fehlende Abnahmekriterium für `shift+cmd+w` in C1 und berichtigt die beiden Bilder, die sich an der Stelle widersprechen, an der der dritte Sicherungsmoment hängt.

## Die Diagrammprüfung des Spec

Urteil `acceptable` (`reviews/260814-0000-conceptrev-spec-notizzettel-als-blatt-mit-zwei-zetteln.md`). Bild 1: zwölf Knoten, elf Kanten, kein Zyklus, kein Gott-Knoten. Bild 2: drei Zustände, zehn Übergänge; die drei Zyklen sind in einem Zustandsautomaten die Sache selbst. Beide mit mermaid-cli 11.16.0 nach SVG gerendert.

Fünf Befunde, zwei mittel. Der schwerere ist kein Zeichenfehler, sondern eine Lücke im Spec: `shift+cmd+w` kommt über die Ausnahmeliste bei stehendem Blatt durch und ruft `performClose` am Hauptfenster, und kein Kriterium in C1 deckte den Fall. Am Baum geprüft, nicht erschlossen.

**Ein Muster, das die Prüfung benennt:** dieselbe unvollständige Fallunterscheidung — eine Entscheidungsraute mit nur einem Ausgang — ist zum dritten Mal gezeichnet worden, und die zwei früheren Beanstandungen sind nie behoben worden (offener Datensatz `260813-1345_o_die-diagrammbefunde-am-spec-sind-nie-behoben-worden-…` im Circle der achten Runde).

## Plan-Tor (Nutzer, 260814-0715)

**Plan freigegeben.** `planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, sechzehn Schritte in sechs Strängen. Mit der Freigabe angenommen sind die zwei Namenswahlen des planners: die Dateien heißen `note-1.txt` und `note-2.txt` (englisch wie die vier bestehenden Ablagedateien), die Tabs „Zettel 1" und „Zettel 2" (deutsch, weil der Nutzer sie liest).

**Die zwei geweiteten Signaturen in `krk-core` sind genehmigt.** `atomar::schreiben` und `Zugang::beiseite_legen` nehmen künftig einen Leser statt einer Zeichenkette; fünf Aufrufstellen ziehen mit. Der Grund ist keine Bequemlichkeit: beide unlesbaren Fälle des Zettels tragen keinen `&str`, und eine Datei über `EDITORGRENZE` darf nicht in den Speicher. Die Alternative, den Zettel an `atomar::schreiben` vorbeischreiben zu lassen, ist verworfen — sie hätte eine zweite atomar schreibende Stelle angelegt, genau das, was die Probe `nur_benannte_dateien_erreichen_das_atomare_schreiben` verhindern soll.

**Die Abschaltung der Textautomatiken wird über eine Zählprobe am Baum abgesichert**, Möglichkeit 2 des Datensatzes: jede Datei mit `setEditable(true)` muss auch `automatiken_abschalten` nennen. Der blinde Fleck ist benannt und nicht verschwiegen — eine Fläche, die ihre Bearbeitbarkeit anders schreibt, entgeht der Nadel. Der eigene Typ um die bearbeitbare Fläche (Möglichkeit 3) ist als der teuerste verworfen.

## Die Diagrammprüfung des Plans

Urteil `acceptable` (`reviews/260814-0711-conceptrev-plan-notizzettel-als-blatt-mit-zwei-zetteln.md`). **Der Fehler ist nicht zum vierten Mal passiert:** die einzige Entscheidungsraute des Plans trägt vier Ausgänge, und der Absatz darunter behauptet die Vollständigkeit ausdrücklich. Der Arbeitsgraph deckt sich Kante für Kante mit den Abhängigkeitszeilen der Schrittliste — sechzehn Schritte, neunzehn Kanten, kein Zyklus.

Zwei Befunde, beide Auszählungen. Der eine liegt beim Spec: er zeichnet auf dem `Esc`-Weg erst das Sichern und dann das Schließen, der Plan hängt es hinter das Schließen; belegt ist die Fassung des Plans. Kein Abnahmekriterium bindet die Reihenfolge, der Nutzer hat auf einen weiteren shaper-Lauf verzichtet. **Der Spec bleibt damit an dieser Stelle ungenau, und das ist eine bewusste Entscheidung, keine Lücke.**

## Turn 1 — Bilanz

Alle sechzehn Planschritte gebaut, fünf Commits, `make check` exit 0 nach jedem Strang und am Ende über den ganzen Baum.

| Strang | Commit | Inhalt |
|---|---|---|
| A | `9362034` | Ablage führt zwei Zettel, `atomar` schreibt aus einem Leser, `Datei::ALLE` auf sechs |
| B | `a949ff1` | `f2` und `cmd+k` in einer Zeile, Belegung auf 83 Funktionen mit 90 Kombinationen |
| C | `bfea397` | Das zehnte Blatt, drei neue Dateien, `appkit` auf 29 Module |
| D/E/F | `dd2643e` | Vier Sicherungsmomente in einer Erklärung, `Sitzung::zettel` |

**Die Durchsicht hat neun Befunde gefiltert, einer davon hoch, und seine Ursache liegt im Spec.** C4 sagt beides zu — eine gescheiterte Sicherung wirft den Stand nicht weg, und der Zettel liest seine Datei bei jedem Öffnen neu — und beide halten nur gemeinsam, wenn das Neulesen einen abweichenden Stand nicht antastet. `Zettelmodell::oeffnen` setzt den gehaltenen Text auf das Gelesene; damit ist weg, was die gescheiterte Sicherung stehen lassen sollte.

**Die sieben Planabweichungen der Bauer tragen alle sieben**, jede einzeln geprüft. Die eine mit einer Zahl daran ist nachgezählt: in `applicationWillTerminate:` steht genau ein Durchgang, der Defekt aus zweien ist nicht ein zweites Mal gebaut.

**Die vier teuren Zusagen halten alle vier.** Die drei Regeln sind außerhalb der Prüfmodule unangetastet, die Textfläche des Zettels ist nirgends in `ersthelfer_gehoert_appkit` angemeldet, der Schreibfokus geht nach jedem Tabklick zurück, und `Datei::ALLE` und `Format` sind vollständig.

## Zwei Antworten am Coherence-Tor (Nutzer, 260814-0925)

**Welcher Stand beim Öffnen gewinnt: der getippte.** Weicht der gehaltene Text von der Datei ab, bleibt er stehen; neu gelesen wird nur, wenn nichts abweicht. Damit hält die Zusage, die der Nutzer spürt — nichts Getipptes verschwindet stillschweigend — und der zweite Satz von C4 wird eingeschränkt und im Spec neu formuliert. „Die Datei gewinnt immer" ist verworfen, „beim Öffnen nachfragen" ist unmöglich: die Nachfrage wäre ein Blatt über dem Zettelblatt.

**Turn 2 behebt die drei zusammenhängenden Befunde**, erst den Spec an C4, dann alle drei in einem Zug. Getrennt behoben bliebe jeweils der andere Weg offen. Die fünf niedrigen Befunde bleiben für später.

## Coherence

**Verdict:** review-needed

**Edges:**
- Artifact↔Grounding: 43 am Baum nachweisbare Zusagen geprüft und alle 43 gehalten (40 ohne, 3 mit einer benannten Einschränkung), 16 von 16 Planschritten am Baum bestätigt, `make check` exit 0 am Stand `79dab20` — **geflaggt** wegen zwei offener mittlerer Befunde der Durchsicht: `issues/260814-0911_o_` ist der eine, den der Prüfer ausdrücklich „vor dem Abschluss, aber ohne Nutzerfrage" empfiehlt, und er ist nicht behoben (`crates/krk-ui/src/appkit/editor.rs:4854` schickt den nächsten Bauer weiter an `textflaeche_bauen`); `issues/260814-0910_o_` hält eine Zusage des Spec gegen den Bau, die obere Schranke `EDITORGRENZE` für Arbeit auf dem Hauptfaden, und das Beiseitelegen kopiert unbegrenzt (`crates/krk-core/src/ablage/atomar.rs:156`, `io::copy` ohne `take`).
- Artifact↔Directive: alle sieben Commits `6d05bef..HEAD` laufen auf die Directive zu und keiner daneben — `9362034` die Ablage, `a949ff1` die zwei Tastenwege, `bfea397` das zehnte Blatt, `dd2643e` die vier Sicherungsmomente und die Sitzung, `895089d` die Durchsicht, `79dab20` die Behebung der zwei zusammenhängenden Befunde, `edea4d9` Spec und Plan; **nicht geflaggt**, mit einem Vorbehalt an der Fassung und nicht an den Commits: die Directive im Circle-Datensatz nennt weiter drei Sicherungsmomente, der Bau folgt der vom Nutzer am 260814-0005 beantworteten Fassung mit vier (`issues/260814-0637_o_` und `issues/260814-1002_o_die-directive-abweichung-…`, drei Stellen statt der dort genannten zwei).
- Grounding↔Directive: 19 offene Fragen über alle Speicher (7 gemeinsam, 12 über sieben Circles) und keine im Widerspruch zur Directive, dazu die zwei Entscheide dieses Circles von beantwortet auf umgesetzt gezogen (`bfea397`, `9362034`); **nicht geflaggt**, mit zwei Lücken zum Vermerk: zwei aktive Datensätze außerhalb dieses Circles tragen Zahlen, die diese Runde verschoben hat (`shared/decisions/260813-0053_o_was-teilen-sich-zwei-instanzen-…` sagt vier Ablagedateien, es sind sechs; `circles/260813-0100-…/decisions/260813-0320_o_esc-im-editor-…` nennt zwei `Esc`-Empfänger, es sind drei), und für die Frage, wie groß „beiseite" werden darf, gibt es keinen Datensatz, obwohl `issues/260814-0910_o_` sie ausdrücklich in den Spec oder einen Entscheid verweist.

**Rebalance recommendation:** revise Artifact

**Was der Gate daneben wissen muss.** Der Weg von hier zu einem kohärenten Abschluss führt über
Nutzerarbeit, und die ist kein Planschritt: 29 der 72 Abnahmekriterien stehen in den zweiten
Listen der fünf Fähigkeiten und verlangen KRK im Vordergrund, dazu die `performClose:`-Messung
aus dem Abschnitt „Nutzerarbeit" des Plans. Fünf dieser 29 brauchen keinen Blick, sondern einen
Prüfaufbau — eine hergestellte Lage wie ein entzogenes Schreibrecht —, und vier davon sind am
Modell schon gedeckt; die Aufteilung in die drei Körbe steht in Abschnitt 2 von
`history/260814-1002-reconciliation.md`. Die acht Runden davor haben alle aus diesem einen Grund
beschränkt geschlossen, die neunte steht an derselben Stelle. Der Empfehlung „revise Artifact"
liegen die zwei mittleren Befunde zugrunde und nichts weiter; wer sie behebt, hat den Baum
soweit, wie ein Agent ihn bringen kann.

**Abgleich:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260814-1002-reconciliation.md`

## Zwei Antworten am Rebalance-Tor (Nutzer, 260814-1010)

**Obergrenze auch für die Kopie.** Dieselbe Zahl, `EDITORGRENZE` mit 16 MB, gilt auch für das Beiseitelegen: kopiert wird bis zur Grenze, darüber bricht es ab und meldet. Eine Zahl, zwei Verwendungen, keine zweite Konstante im Baum. Der Preis ist benannt: von einer sehr großen Fremddatei werden nur die ersten 16 MB gesichert. „Eine zweite Zahl" und „umbenennen statt kopieren" sind verworfen — Letzteres verstieße gegen die erste der drei Regeln von `beiseite_legen`.

**Turn 3 behebt die zwei mittleren Befunde**, danach fährt der Nutzer die Abnahme. Die Runde kann dann kohärent schließen wie die achte. Die fünf niedrigen Befunde der Durchsicht und die sieben des Abgleichs bleiben offen.

## Coherence

*(zweites Urteil, 260814-1247, nach Turn 3 und dem Abnahmelauf des Nutzers)*

**Verdict:** review-needed

**Edges:**
- Artifact↔Grounding: 46 am Baum nachweisbare Kriterien geprüft und alle 46 gehalten (43 ohne, 3 mit einer benannten Einschränkung), 16 von 16 Planschritten bestätigt, die zwei mittleren Befunde behoben und beide Behebungen gegen den Baum gelesen (`crates/krk-core/src/ablage/mod.rs:261`, `:320`, `:720`; acht Verweise in `crates/krk-ui/src/appkit/editor.rs`), `make check` exit 0 am Stand `a6098d9` — **geflaggt**, und zwar an der Abnahme und nicht am Baum: der Lauf vom 260814-1115 führt zwölf Beobachtungen für 29 Kriterien mit Bündelanteil, davon sind 8 belegt, 5 zur Hälfte berührt und **16 nie berührt** (`issues/260814-1247_o_sechzehn-der-neunundzwanzig-buendelkriterien-…`); die Streichung der Beobachtung 10 nimmt zusätzlich den Kriterien `:267` und `:268` ihren einzigen möglichen Beleg, weil `krk-ui` ohne Bibliotheksziel den Meldeweg nicht proben kann (`issues/260814-1247_o_die-streichung-der-beobachtung-10-…`); und die Grundmenge ist seit dem C5-Nachtrag vom 260814-1010 nicht mehr 72, sondern 75 (`issues/260814-1247_o_die-abnahmeliste-rechnet-gegen-72-kriterien-…`).
- Artifact↔Directive: der eine Commit seit dem ersten Abgleich, `a6098d9`, läuft auf die Directive zu und nicht daneben — er zieht die Kopiergrenze nach, die der Spec offengelassen hatte, und berichtigt acht Verweise; **nicht geflaggt**, mit demselben Vorbehalt an der Fassung wie beim ersten Urteil: die Directive im Circle-Datensatz nennt weiter drei Sicherungsmomente, der Bau folgt der Fassung mit vier (`issues/260814-0637_o_` und `issues/260814-1002_o_die-directive-abweichung-…`).
- Grounding↔Directive: 19 offene Fragen über alle Speicher, unverändert gegenüber dem ersten Urteil, keine in diesem Circle und keine im Widerspruch zur Directive; die zwei Entscheide dieses Circles stehen auf umgesetzt; **nicht geflaggt**, und die eine Lücke des ersten Urteils ist geschlossen: die Frage, wie groß „beiseite" werden darf, hat der Nutzer am 260814-1010 beantwortet, und die Antwort steht im Spec unter C5 mit drei Kriterien und drei Festlegungen — einer der zwei Orte, die `issues/260814-0910_*` dafür nannte.

**Rebalance recommendation:** revise Artifact

**Was der Gate daneben wissen muss.** Der Weg zu einem kohärenten Abschluss ist kurz und führt
noch einmal über den Vordergrund: ein zweiter Abnahmedurchgang, der die sechzehn nie berührten
Kriterien und die wieder aufgenommene Beobachtung 10 fährt. Mehrere lassen sich zusammenfassen —
die drei Beenden-Kriterien von C4 in einem Lauf mit getipptem Text, die Kriterien `:267`, `:268`
und `:269` in einem einzigen Durchgang mit entzogenem Schreibrecht. **Der Unterschied zur Runde 8
ist eine Formfrage und keine Eigenschaft dieser Runde:** dort trug jedes Kriterium einzeln die
Kennzeichnung `(Probe)` oder `(Bündel)`, zehn waren es, und der Abnahmelauf führte elf
Beobachtungen — eine je Kriterium. Diese Runde führt zwei Listen je Fähigkeit, und die Bindung
zwischen Beobachtung und Kriterium ist dabei verloren gegangen. **Ein beschränkter Abschluss ist
nicht die naheliegende Wahl**, weil der Grund der acht früheren Beschränkungen hier nicht mehr
trägt: der Nutzer hat den Vordergrundlauf gefahren, er kann ihn wieder fahren, und die Directive
ist erreichbar.

**Abgleich:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/history/260814-1247-reconciliation.md`
