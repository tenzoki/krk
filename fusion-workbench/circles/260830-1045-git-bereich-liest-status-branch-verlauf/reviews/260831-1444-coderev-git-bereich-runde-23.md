# Durchsicht der Runde 23: der Git-Bereich liest Status, Branch und Verlauf

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `d1fbaac..0a25ee0`
**Not-opened:** `Cargo.lock`

---

## Summary

Die eine Durchsicht der Runde 23, gefahren über die 24 Commits vor dem Abschluss. Der Bau ist grün — `cargo test --workspace` (24 Probenziele, 1 792 Proben, keine gescheitert), `cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all --check` sind in dieser Sitzung gelaufen. **Die Schreibfreiheit der Stufe A hält**, die neun stillen Stellen sind gefunden, die drei neuen Kommandos tragen ihre vier Pflichtstellen, und die Untergrenzen-Abschnitte stehen in jeder angefassten Datei. **Die zwei ernsten Befunde liegen beide im Verlauf und in der Nebenläufigkeit**, und keiner von beiden wäre am Abnahmelauf über KRKs eigenes, lineares Repository aufgefallen.

## Totals

| Rang | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 6 |
| Niedrig | 6 |

Jeder Befund liegt als eigener Defektdatensatz unter `issues/` mit dem Stempel `260831-1444`. Dazu eine `Also seen:`-Zeile an `shared/issues/260826-1221_*_zwei-fadenstarts-des-verzeichnisbaums-brechen-mit-panik-ab-…` (`Gitlauf::starten` ist die vierte Stelle derselben Form).

---

## Was geprüft ist und hält

**Die Schreibfreiheit (Stufe A, E8, C3.8, C10.3).** `grep -rn 'write_changes\|NeedsUpdate' crates/ xtask/` liefert außerhalb von Modulköpfen genau eine Stelle, `posten_deuten` (`crates/krk-core/src/git/leser.rs:398`), und die verwirft. Kein Weg unter `crates/krk-core/src/git/` ruft eine schreibende `gix`-Funktion; gesucht wurde nach `commit`, `write`, `edit_reference`, `index_mut`, `set_head`, `write_object` und `write_blob`, mit null Treffern. Die Zusage trägt allein der fehlende Aufruf, wie der Auftrag sagt; der gefilte `NeedsUpdate`-Befund (`260831-0855_o_…`) ändert daran nichts.

**Die neun stillen Stellen.** Die vier aus Schritt 1 (`Aufteilung::rahmen`, `Aufteilung::gemessene_breiten`, `bereichsbreiten`, `Fenstermodell::breiten_uebernehmen`) und die fünf aus Schritt 2 sind einzeln nachgelesen. Eine eigene Erhebung über jede Feldbreite unter `crates/krk-ui/src` und `crates/krk-core/src` sowie über jede von Hand geschriebene Aufzählung von `Fokus::`- und `Spalte::`-Werten hat **keine zehnte** gefunden: `fokus.rs:812-819`, `zulaessigkeit.rs:1224-1230`, `teilen.rs:322-328`, `rundweg.rs:207-214`, `fenstermodell.rs:2327-2333` und `bereichsleiste.rs:905-914` tragen alle den neuen Wert. Die Strukturliterale über `Sichtbarkeit`, `Breiten` und `Spaltensichtbarkeit` sind vollständig ausgeschrieben und ohne `..Default::default()`, hält also der Übersetzer. **Das ist eine Aussage über die Muster, die ich gesucht habe**, und keine über die Menge aller möglichen: eine Handsuche ist nicht erschöpfend, und die offene Nutzerfrage `shared/decisions/260826-1811_*_…` bleibt der Ort, an dem das aufhört.

**Der verspätete Befund.** Ein Ordnerwechsel geht durch `Tabliste::lesen_starten`, das in dieser Reihenfolge `modell.lesevorgang_beginnen(generation)` und danach `gitlauf_nachziehen_an` ruft (`tabs.rs:1284-1313`); der neue Lauf bekommt damit die neue Generation. `gitlauf_nachziehen_an` lässt zuerst den alten `Gitlauf` fallen, räumt `wartende_marken` und setzt das `Gitmodell` zurück (`tabs.rs:1199-1203`), der Empfänger geht also mit, und keine Meldung des alten Ordners kann noch ankommen. Darüber liegt die Generationsprüfung in `Ordnermodell::gitmarken_setzen` (`modell.rs:1261`) und die Wartestelle im Einzugstakt (`tabs.rs:1423`, die Umkehrung `!tab.gelesen || tab.liest()`). Drei Schichten für eine Frage, und jede hat ihren eigenen Grund. **Der Weg hält.**

**Die vier Pflichtstellen der drei neuen Kommandos.** `Kommando::wirkungsbereich` (`belegung.rs:990,995,1071`, alle drei `Ueberall`), `belegungsmodell::bereich` (`:294`, `:443`), `Kommando::KENNUNGEN` (`:898-900`) und — die vom Übersetzer nicht gehaltene vierte — der Ausführungszweig beim Anwendungsdelegierten (`anwendung.rs:3490`, `:3495`, `:3572`). Keines der drei fällt in den Auffangzweig. `opt+cmd+r` und `shift+cmd+b` sind in `resources/default-keymap.toml` je einmal vergeben; eine Erhebung über alle `tasten`-Werte der Datei zeigt keine neue Doppelbelegung.

**Die Untergrenzen.** Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` steht in allen dreizehn von der Runde angefassten Dateien unter `crates/krk-ui/src/appkit/`; die zwei begründeten Ausnahmen `koordinaten.rs` und `mod.rs` sind unverändert die zwei. Die höchste genannte Untergrenze von `appkit/git.rs` ist `NSTableViewStyle` mit 11.0, also unter dem Zielsystem. Der Abschnitt selbst ist aber unvollständig, siehe unten.

**Der Deskriptorvorrat eines Laufs.** Die Kindprobe unter `ulimit -n 64` (`crates/krk-core/tests/git.rs:489`) misst beides, was C7.8 und C7.9 verlangen: ohne freien Deskriptor `Oeffnung::Unentschieden` statt `KeinRepository`, und mit dreißig freien alle vier Auskünfte. Die Form ist die der Deskriptorproben aus der Runde 10, und eine vierte Prüfordner-Fassung entsteht nicht.

---

## Befunde nach Themen

### Der Verlauf: zwei Befunde in einer Funktion

**H1 — Der Nachschlag verliert jeden Nebenzweig.** `Gitleser::verlauf` (`leser.rs:194-220`) startet den Lauf bei `ab`, dem letzten angezeigten Commit, und liefert damit allein dessen **Vorfahren**. Jeder Commit, der beim Ende eines Schwungs noch in der Warteschlange des Graphenlaufs stand und kein Vorfahre von `ab` ist, kommt danach nie mehr in die Liste. Rang **hoch**: es ist ein falsches Ergebnis und keine Unschönheit, und C4.2 sagt das Gegenteil zu. Datensatz `260831-1444_*_der-nachschlag-des-verlaufs-setzt-am-letzten-commit-an-und-verliert-jeden-nebenzweig.md`.

**M1 — Die Reihenfolge ist die des Graphen und nicht die der Zeit.** `rev_walk(…).all()` ohne `sorting` läuft mit `Sorting::BreadthFirst` (`gix-0.87.1/src/revision/walk.rs:31-42`, `#[default]`). `gitmodell.rs:53` sagt „die jüngsten zuerst", der Spec sagt in C4 „die fünfzig jüngsten Commits"; beides folgt aus dem Lauf nicht. Datensatz `260831-1444_*_der-verlauf-laeuft-in-graphenreihenfolge-und-nicht-nach-commit-zeit.md`.

**Beide fallen in KRKs eigenem Repository nicht auf**, weil es linear ist, und die zwei Proben in `tests/git.rs` messen ebenfalls an linearen Ketten. Der Abnahmelauf aus Schritt 17 sähe sie nur, wenn der Nutzer den Git-Bereich auf ein Repository mit Zusammenführungen richtete.

### Die Nebenläufigkeit: eine Zusage, die dem Halter gilt und nicht dem Faden

**M2 — Ein abgebrochener Gitlauf läuft weiter.** `Gitlauf::drop` setzt das Abbruchkennzeichen und wartet nicht; das Kennzeichen wird an vier Stellen gelesen, und keine liegt **in** `Gitleser::marken`, das kein Abbruchkennzeichen entgegennimmt und nach dem Eintritt in jedem Fall zu Ende läuft (gemessen 12 bis 164 ms, in einem größeren Baum mehr). `thread_limit` bleibt bewusst ungesetzt, also nimmt jeder laufende Status so viele Fäden, wie das Gerät Kerne hat. Schnelle Navigation stapelt beliebig viele. A10 und C7.11 sagen wörtlich „nie nebeneinander"; gehalten ist „höchstens ein Halter je Tab", und genau das misst die Probe. Der Befund berührt C7.9 mit: die Deskriptorprobe misst **einen** Leser. Datensatz `260831-1444_*_ein-abgebrochener-gitlauf-laeuft-weiter-und-a10-gilt-nur-dem-halter-und-nicht-dem-faden.md`.

### Die Fallunterscheidungen: eine, die nicht disjunkt ist

**M3 — Jeder Fehlschlag von `discover` außer dem Deskriptormangel wird „kein Repository".** `Gitleser::oeffnen` (`leser.rs:148-152`) hat drei Zweige, und der dritte fängt alles: auch `discover::Error::Open(_)`, das heißt „ein Repository ist gefunden worden und ließ sich nicht öffnen", und die Ein-/Ausgabefehler aus `upwards::Error`. Der Nutzer liest dann den Satz „Dieser Ordner liegt in keinem Git-Repository." Das ist dieselbe Zusammenziehung, die der Modulkopf von `git/mod.rs:23-41` für sich ausschließt und mit dem Defekt `260815-0211` begründet. Datensatz `260831-1444_*_jeder-fehlschlag-von-discover-ausser-dem-deskriptormangel-wird-als-kein-repository-ausgegeben.md`.

### Die Zuordnung über den Namen

**M4 — Bytegenauer Vergleich gegen vorkomponierte Namen.** `Ordnermodell::gitmarken_setzen` schlägt bytegenau nach; der Bestand kommt roh aus `readdir`, der Befund aus `gix`, das auf den Verzeichnisdurchlauf `core.precomposeUnicode` anwendet. Ein zerlegt benannter Eintrag bekäme damit keine Marke und zählte auch in der Zusammenfassung nicht mit. **Gelesen und nicht gemessen**; der Datensatz nennt den Abnahmetest, der die Frage entscheidet. Datensatz `260831-1444_*_die-marken-werden-bytegenau-ueber-den-namen-zugeordnet-und-gix-liefert-ihn-vorkomponiert.md`.

### Prosa gegen Baum

**M5 — Drei Stellen sagen, die Auswahl der Verlaufsliste überstehe den Tabwechsel.** `Tabliste::waehlen` ruft `gitlauf_nachziehen_an(verlassen)`, und dessen dritte Zeile setzt das `Gitmodell` des verlassenen Tabs zurück — samt Auswahl. Die halbe Aussage (der Wechsel des aktiven Dateifensters) trifft zu. Der Befund zählt hier mit, weil er die **Begründung des Nutzerentscheids vom 260831** trägt und nicht bloß einen Kommentar. Datensatz `260831-1444_*_drei-prosastellen-sagen-die-auswahl-der-verlaufsliste-uebersteht-den-tabwechsel-sie-faellt-mit-ihm.md`.

**M6 — Der Untergrenzen-Abschnitt von `appkit/git.rs` ist unvollständig und ungenau.** `deselectAll:`, `documentView` und `NSTableColumn::initWithIdentifier:` werden gerufen und nicht genannt; `window` und `makeFirstResponder:` werden genannt und nicht gerufen. Kein Absturzrisiko — alle drei stehen seit 10.0 —, aber die Vorkehrung selbst ist die einzige, die es gibt. Datensatz `260831-1444_*_der-untergrenzen-abschnitt-von-appkit-git-rs-laesst-drei-gerufene-methoden-aus-und-nennt-zwei-ungerufene.md`.

**M7 — Vier Prosastellen in `rundweg.rs` zählen fünf Fokuswerte.** `:24`, `:26`, `:65` und `:160`, die letzte eine Zeile über `const JEDER_FOKUS: [Fokus; 6]`. Die Datei steht in der Dateiliste von Schritt 11 und ist dort angefasst worden; die Tafel ist nachgezogen, die Prosa darum herum nicht. Datensatz `260831-1444_*_vier-prosastellen-in-rundweg-rs-zaehlen-fuenf-fokuswerte-und-der-baum-traegt-sechs.md`.

**Vier weitere im Rang niedrig**, je mit eigenem Datensatz: die Kurzhashlänge steht an zwei Stellen und die zweite ist eine nackte Sieben (`texte.rs:113`); der Modulkopf der Bereichsleiste nennt eine Zählvorschrift `Bereichsleiste::alle_schalter()`, die es nicht gibt; der Doc-Kommentar des Giteinzugs sagt mehrdeutig, die wartende Markenmeldung falle mit dem Kanalschluss; ein Kommentar im Einzugstakt (`tabelle.rs:3677`) nennt zwei Kanäle, wo drei bedient werden; `krk-core/Cargo.toml:38-45` nennt `git::leser` als die einzige Stelle, die `gix` nennt, und übergeht die Wiederausfuhr in `git/mod.rs:64`.

---

## Übergreifende Beobachtungen

**Die zwei Verlaufsbefunde und der Nebenläufigkeitsbefund haben dieselbe Wurzel: die Abnahme dieser Runde läuft gegen ein lineares Repository auf einem schnellen Gerät.** Ein Prüfrepository mit einer Zusammenführung und ein Lauf über einen großen fremden Baum hätten alle drei gezeigt. Die Proben in `tests/git.rs` legen ihre Repositorys mit `git` an und könnten beides ohne neue Bauform.

**Die Prosabefunde sind nicht zufällig verteilt.** Von den sechs Stellen, an denen Prosa und Baum auseinanderlaufen, liegen fünf in Absätzen, die eine **Zusage begründen** — die Auswahl über den Tabwechsel, die wartende Markenmeldung, die Zahl der Kanäle, die eine Stelle, die `gix` nennt, die Zählvorschrift der Leiste. Das ist die teuerste Sorte: sie erklärt, warum eine Bauform so aussieht, und wer sie glaubt, baut auf einer Voraussetzung weiter, die nicht steht. Die Zählaussagen, die die Runde ausdrücklich nachgezogen hat (C9.4), sind dagegen bis auf `rundweg.rs` sauber — die Erhebung hat getragen, sie hat nur eine Wortform nicht geführt (`fuenf Werte` neben `fuenf Werten`).

**Die Regel „`None` heißt unentschieden" ist im Gitmodul vorbildlich durchgezogen und hat genau eine Lücke**, den Auffangzweig von `oeffnen`. Der Modulkopf beschreibt die Regel besser, als der eine Zweig sie einhält.

## Empfohlene Reihenfolge

1. **H1** (der verlorene Nebenzweig) und **M1** (die Reihenfolge) zusammen: dieselbe Funktion, und die eine Antwort auf beide ist eine andere Bauform des Nachladens.
2. **M2** (der weiterlaufende Lauf), weil er an C7.9 hängt und eine Nutzerfrage aufmacht.
3. **M3** und **M4**: beide entscheiden, was der Nutzer über ein Repository zu sehen bekommt.
4. **M5** bis **M7** und die vier niedrigen: Prosa, in einem Zug nachzuziehen.

Keiner der Befunde hält den Abschluss der Runde auf. Alle sind gefilt und damit nach der Schließung auffindbar.
