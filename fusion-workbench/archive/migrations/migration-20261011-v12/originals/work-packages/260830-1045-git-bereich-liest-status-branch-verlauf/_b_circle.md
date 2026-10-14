# Der Git-Bereich liest Status, Branch und Verlauf

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Claim:** Unclaimed
**Active spec/plan:** 260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md
**Active session history:** 260830-0950-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

**Die Machbarkeit ist gemessen und nicht angenommen.** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` prüft `gix` 0.87.1 am laufenden Programm gegen KRKs eigenes Repository und fünf angelegte Wegwerf-Bäume, auf dem Referenzgerät der zehn Zeitzusagen. Alle vier Auskünfte der Stufe A stehen (`discover`, `status`, `head`, `rev_walk`), auf beiden Mac-Zielen übersetzt kein C-Code, und in jedem gemessenen Fall ist `gix` so schnell wie `git` oder schneller. Die Merkmalswahl steht dort ausgeschrieben.

**Der Risikopunkt liegt im eigenen Baum und nicht in der fremden Kiste.** `Bereich::ALLE` (`crates/krk-ui/src/fenstermodell.rs:122`) ist `[Bereich; 5]`, und die Feldbreite hält den Bau **nicht** an, wenn die Aufzählung wächst; ein sechster Bereich, der dort fehlt, übersetzt, besteht jede Probe und existiert nicht. Fünf Prosastellen im Baum behaupten das Gegenteil, mit eigenständig übersetztem Gegenbeweis widerlegt in `shared/issues/260830-1006_*_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-wenn-eine-aufzaehlung-waechst-sie-tut-es-nicht.md`. Erst wenn `ALLE` wächst, hält der Übersetzer die vier Feldbreiten dahinter und nennt alles Übrige. Dieselbe stille Stelle hat der sechste Fokuswert viermal: `Fokus::ALLE`, `fokus::wirkt` (dessen Zweige über `==` und `matches!` vergleichen), die Tafel in `fokus.rs:404` und `OHNE_SPERRE` in `zulaessigkeit.rs:670`.

**Der Status gehört auf einen Arbeitsfaden mit nachgetragenem Befund.** Synchron gegen die erste Bildschirmseite aus L10 bräche er das Budget (155 ms neben 100 ms), im Zeichendurchgang bräche er L1 in jedem Fall. Die Bauform steht seit der Runde 10 im Baum: nebenläufiger Auftrag, nachgetragener Befund, Sicht neu aufgebaut. Die Zuordnung der Befunde läuft über den **Namen** und nicht über den Eintragsindex, weil `Ordnermodell::lesevorgang_beginnen` den Bestand nicht vorab leert. Der Befundvektor des Filters wird nicht mitbenutzt: die Gitfrage hat eine andere Ungültigkeitsregel und fällt beim Ordnerwechsel statt beim Tippen. Der Status ist über die Pfadmuster von `into_iter` auf den angezeigten Ordner zu beschränken, sonst kostet ein Ordnerwechsel in einem großen Repository 220 ms statt 12 ms.

**Zwei der fünf gefilten Entscheidungen hat der Nutzer am 260830-1045 beantwortet**, beide im Zuge dieser Klärung: `shared/decisions/260830-1006_*_bekommt-der-git-bereich-einen-sechsten-fokuswert-oder-ist-er-nicht-fokussierbar.md` auf Möglichkeit 1, und `shared/decisions/260830-1006_*_was-zeigen-git-bereich-ankreuzfeld-und-dateiliste-in-einem-ordner-ohne-repository.md` auf je die erste Möglichkeit der drei Anzeigen. Die Kosten des sechsten Fokuswerts sind dabei ausdrücklich in Kauf genommen, die zehn Nachzugsstellen samt der vier stillen eingeschlossen.

**Eine dritte trägt eine angenommene Vorbelegung und bleibt offen.** `shared/decisions/260830-1006_*_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-oder-zahlt-sie-die-wiederholung.md`: die Stufe A schreibt den aufgefrischten Index nicht zurück und bleibt schreibfrei. Der Datensatz bleibt `_o_` mit Wiedervorlage, sobald der Posten gemessen ist, und die Messung gehört in diese Runde, weil die Messstrecke dann schon dasteht.

**Zwei bleiben offen und sind vor dem Plan zu beantworten, nicht vor der Direktive.** `shared/decisions/260830-1006_*_wohnt-die-git-anbindung-in-krk-core-oder-in-einer-fuenften-kiste-krk-git.md` und `shared/decisions/260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`. Beide berühren nicht, was der Nutzer bekommt, sondern wo der Code wohnt und wie eine Zusage formuliert ist, die an fünf Prosastellen und in der Wurzel-`Cargo.toml` steht. Der Analyst empfiehlt bei beiden die erste Möglichkeit.

**Drei Folgen des Zuschnitts gehören zur Lage und sind nicht bestritten.** Die fünfte Spalte steht auch in einem Ordner ohne Repository und bleibt dort leer; die Möglichkeit, sie einzuziehen, ist ausdrücklich nicht gewählt. Der Baum wächst um 98 Pakete auf dem Bauziel, das Fünffache dessen, was `syntect` und `two-face` zusammen gekostet haben. Und `gix` steht unter 1.0 mit vierzehn kleinen Fassungen in zehn Monaten, nimmt also eine wiederkehrende Pflege an, die dieses Projekt bei keiner seiner bisherigen fremden Kisten hat.

## Dependencies

- `260802-0842-krk-mac-dateimanager-editor-git` — die Runde 1 hat die Git-Anbindung in ihrer Directive genannt und aus ihrem Umfang herausgenommen; ihre zehn Zeitzusagen aus C8 binden diese Runde, und ihr Datensatz `decisions/260802-1036_*_leistungszusagen-navigator.md` nennt das Referenzgerät, auf dem die Zahlen der Machbarkeitsanalyse gemessen sind.
- `shared/decisions/260802-0842_*_git-verwerfen-bedeutung.md` — offen, betrifft die Stufe B und bindet den späteren Schreibweg. Diese Runde berührt ihn nicht.
- `shared/decisions/260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` — offen, nennt elf `ALLE`-Listen; `Bereich::ALLE` und `Fokus::ALLE` fallen darunter, und diese Runde fasst beide an.

## Turn log

## Closure note

**Beschränkt geschlossen am 260831-2024.** Der Git-Bereich der Stufe A steht: ein sechster Bereich
der Fensterzeile mit Branch, Statuszusammenfassung, Verlaufsliste und den Einzelheiten des
ausgewählten Commits, `Fokus::Git` als sechster Fokuswert auf `shift+cmd+b`, der
Umschalter auf `opt+cmd+r`, eine fünfte Spalte mit fünf Markenzuständen in beiden
Dateifenstern, gelesen mit `gix` 0.87.1 und ohne jeden Schreibweg ins Repository.

**Warum beschränkt und nicht kohärent:** der Abnahmelauf am laufenden Bündel ist nicht
gefahren. 25 der 90 Abnahmekriterien verlangen KRK im Vordergrund, weil die
Wirkungsbereichs-Prüfung aus dem Hintergrund jeden fokusgebundenen Befehl abweist; kein
Agent kann sie fahren. Sechzehn der siebzehn Planschritte stehen auf `[DONE]`, der
siebzehnte ist dieser Lauf. Vierzehn der fünfzehn Endbedingungen halten, nachgefahren und
nicht abgelesen; die eine offene ist derselbe Schritt.

**Was die Runde über ihren Gegenstand hinaus gelernt hat**, und das ist das Artefakt des
beschränkten Abschlusses:

Eine `ALLE`-Liste hält den Bau nur, wenn sie über `ALLE.map` gebaut ist. Ein Literal und
ein Feld fester Länge halten ihn nicht — sie brechen erst beim Start —, und ein fester
Parameter hält gar nichts. Fünf Prosastellen im Baum behaupteten das Gegenteil. Die Coder
dieser Runde haben deshalb neun stille Stellen von Hand nachziehen müssen, ohne dass der
Übersetzer eine einzige genannt hätte. Der Befund steht jetzt in `CLAUDE.md` unter „Was
man nicht sieht".

Und ein Erhebungsmuster, das seine Zahl aus einem zeilenweisen `grep` zieht, findet die
Stelle nicht, die ihre Zahl und ihr Trägerwort auf zwei Zeilen verteilt. Das alte Muster
fand 57 Stellen, das erweiterte 462 in 71 Dateien, und die 57 sind darin restlos enthalten.

**Die Durchsicht ist gelaufen** (`reviews/260831-1444-coderev-git-bereich-runde-23.md`,
50 von 51 Dateien geöffnet) und hat dreizehn Defekte gefunden. **Alle dreizehn sind
behoben**, darunter zwei, die ein Abnahmelauf an KRKs eigenem, linearem Repository nicht
gefunden hätte: der Nachschlag des Verlaufs verlor jeden Nebenzweig, und Dateien mit
zerlegt geschriebenen Namen bekamen keine Marke.

**Ungedeckt bleiben fünf Commits** — die vier Behebungen und der Commit der Durchsicht
selbst. Der Nutzer hat das am 260831-2024 ausdrücklich in Kauf genommen; jede Behebung ist gegen
einen ausgeschriebenen Abnahmetest gebaut und ihre Probe vor der Änderung gegen den alten
Stand gemessen. `Cargo.lock` hat auch die erste Durchsicht nicht geöffnet.

**Elf Defekte und drei Entscheidungen bleiben offen** und gehen an die nächste Runde. Keiner
hält etwas auf; zwei bestanden schon vor dieser Runde und sind von ihr nur sichtbar gemacht.

**Sitzungsaufzeichnung:** `shared/history/260830-0950-orchestrator-session.md`
**Abgleich:** `history/260831-1417-reconciliation.md`
