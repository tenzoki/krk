# Planung: Das Terminal im angezeigten Ordner (C11)

**Datum:** 2026-08-05, 16:23
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`

## Auftrag

Der Nutzer hat eine Funktion nachbeauftragt: eine Taste, die im angezeigten Ordner ein Terminal öffnet. Zwei Punkte hatte er beim Nachfragen selbst entschieden, die Kombination `ctrl+o` nach dem Vorbild von Total Commander und die sofortige Einstellbarkeit der gerufenen Anwendung. Fünf technische Punkte lagen beim Planner: wohin die Einstellung gehört, wie eine Anwendung benannt wird, wie KRK sie ruft, welcher Ordner gemeint ist und was scheitern kann. Dazu die Einordnung: eigene Fähigkeit oder Kriterium in einer vorhandenen.

Ein `coder` arbeitete parallel an S18 in `crates/`. Der Auftrag begrenzte diese Sitzung auf `planning/`, `decisions/` und die Historie.

## Was entstanden ist

**Ein Entscheidungsdatensatz.** `decisions/260805-1623_a_taste-und-einstellbarkeit-des-terminal-befehls.md`, Stand beantwortet. Er hält beide Nutzerantworten fest, samt den verworfenen Möglichkeiten: `shift+cmd+t` und `f9` für die Taste, die spätere Runde für die Einstellbarkeit. Auf `_i_` geht er, sobald S18c umgesetzt ist.

**Eine elfte Fähigkeit im Spec.** C11, "Den angezeigten Ordner im Terminal öffnen", mit acht Abnahmekriterien. Die Prüfung von C1 bis C10 hat keine gefunden, die sie trägt. C10 war der nächste Kandidat, weil dort schon eine Übergabe an eine fremde Anwendung steht, hält aber selbst fest, dass sein Zuschnitt an der gemeinsamen Auswertung der Zwischenablage hängt; ein dritter Befehl ohne Bezug dazu hätte C10 einen zweiten Gegenstand gegeben.

**Zwei Schritte im Plan.** S18b für den `ontocoder`, der den Eintrag in `resources/default-keymap.toml` und die neue `resources/default-settings.toml` schreibt. S18c für den `coder`, der die vierte Ablagedatei, das Kommando, den Wirkungsbereich und den Aufruf baut. Die Reihenfolge Daten vor Code ist zwingend: die Prüfung `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` bräche, käme der Code zuerst.

**Drei Antworten in `### Frage 4` des Plans.** Der Ort der Einstellung, die Namensform der Anwendung und der Aufrufweg, jeweils mit den verworfenen Möglichkeiten.

## Die Entscheidungen und woran sie hängen

**Eine vierte Ablagedatei `settings.toml`.** Alle drei vorhandenen scheiden aus, jede aus einem eigenen Grund. `keymap.toml` scheidet daran aus, dass C3 einen Befehl verlangt, der die gesamte Belegung zurücksetzt; er nähme die Terminal-Wahl mit. `session.toml` wird alle zwei Sekunden überschrieben und trägt keine Kommentare über einen Schreibvorgang hinweg. `bookmarks.toml` hält Ordnerverweise. Die Datei bekommt eine Aufnahmeregel, damit sie nicht zur Ablage für Übriges wird, und sie entsteht einmal beim ersten Start aus einer eingebetteten Auslieferungsfassung, weil in dieser Runde keine Ansicht sie schriebe.

**Die Bündelkennung als einzige Namensform.** Der entscheidende Grund kam aus der Systemschnittstelle und ist nachgesehen, nicht erinnert: `objc2-app-kit` 0.3.2 führt die beiden Auflösungswege über Name und Pfad, `fullPathForApplication:` und `absolutePathForAppBundleWithIdentifier:`, mit `#[deprecated]` und demselben Verweis auf `URLForApplicationWithBundleIdentifier:` (`src/generated/NSWorkspace.rs:935` und `:941`). Dazu kommt, dass ein Pfad bricht: Terminal.app liegt heute unter `/System/Applications/Utilities/` und lag bis Catalina unter `/Applications/Utilities/`.

**`NSWorkspace` statt Unterprozess.** `openURLs:withApplicationAtURL:configuration:completionHandler:` ist in der geführten Kistenfassung als sicher deklariert und liegt hinter Vorgabemerkmalen; die Abhängigkeiten des Workspace ändern sich nicht. `open -a` löst über den Namen auf, also über den abgekündigten Weg, meldet seinen Fehler auf einem Kanal, den C1 ausschließt, und wäre der erste Unterprozess des Vorhabens.

**Der Wirkungsbereich trägt den Fokusvorbehalt ohne Zusatz.** `Kommando::TerminalOeffnen` bekommt `Wirkungsbereich::Dateifenster`, die Eigenschaft, die S18 gerade für jedes Kommando an einer Stelle einführt. Bei Fokus in der Leiste verwirft die Zuleitung den Befehl stumm, wie sie dort `delete` und `right` verwirft.

## Zwei geprüfte Tatsachen und eine Korrektur

Am 260805-1623 auf dem Gerät des Nutzers nachgesehen: `mdls -name kMDItemCFBundleIdentifier -raw` liefert `com.apple.Terminal` und `com.mitchellh.ghostty`. Beide Anwendungen führen `public.directory` in ihren Dokumenttypen und nehmen einen Ordner damit an. Kennungen für iTerm, WezTerm, Alacritty und kitty sind **nicht** in den Plan gekommen, weil die Anwendungen auf diesem Gerät nicht installiert sind und eine geratene Kennung in einem Kommentar geprüft aussieht.

**Eine Korrektur am Auftrag.** Der Auftrag nannte die Statuszeile als Abhängigkeit an S16c. Die Fehlmeldungen aus C11 sind Befehlsantworten und damit Rang 1; den baut S16b, nicht S16c. S16c baut den fünften Rang, den Markierungsstand. S18c hängt deshalb an S16b, und da S16b vor S16c liegt und beide `[DONE]` tragen, ändert das an der Ausführbarkeit nichts.

## Der Graph, nachgerechnet

Der Abhängigkeitsgraph wächst um zwei Knoten und vier Kanten auf **36 Knoten und 56 Kanten**, Verhältnis 1,56. Maschinell nachgerechnet: zyklenfrei, kein Knoten ohne Kante, genau eine Quelle (S1), jede Kante von der kleineren zur größeren Schrittnummer, höchster Ausgangsgrad 4 bei S1, höchster Eingangsgrad 8 bei S23. Die Kante `S9 → S18b` verhindert, dass der Datenschritt eine zweite Quelle im Graphen wird.

Eine erste Fassung der Selbstprüfung behauptete, der neue Ast sei der erste, der eine Phase überspringt; `S11c → S20` und `S5 → S23` tun das schon. Der Satz ist berichtigt.

Der Datenflussgraph des Specs wächst um den Knoten `Terminal-Anwendung` und drei eingehende Kanten. Er bleibt ein Blatt wie der `Systembrowser`.

## Was offen bleibt

Zwei Punkte stehen unter `## Offene Fragen` des Plans. Erstens, dass `ctrl+o` bei Fokus in der Leiste stumm nichts tut; das ist die Wahl des Planners und die Stelle, an der der Nutzer die Fähigkeit am ehesten anders sehen könnte. Zweitens, dass die Einstellungsdatei keine Oberfläche hat und eine spätere Runde die Aufnahmeregel neu beantworten muss, sobald eine Ansicht sie schreiben soll.

Zwei Risiken sind in die Risikotabelle gekommen: eine eingestellte Anwendung, die keine Ordner annimmt, und der leere Rückrufparameter, der einen gescheiterten Start eines aufgelösten Bündels nicht meldet.

## Geänderte Dateien

- `planning/260802-1036_o_spec-navigator-geruest.md` — Fähigkeit C11, Datenflussgraph, je ein Abnahmekriterium in C2 und C3, zwei Festlegungen, Kopfstand, Directive, offene Nutzerentscheidungen
- `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` — `### Frage 4` um drei Abschnitte, Schritte S18b und S18c, Abhängigkeitsgraph, Selbstprüfung, Datenstrukturen, Risiken, offene Fragen, Datensatzaufstellung, S23-Abhängigkeiten
- `decisions/260805-1623_a_taste-und-einstellbarkeit-des-terminal-befehls.md` — neu

Nicht angefasst: `crates/`, `resources/`, `issues/`, `xtask/`, `README.md`, `CLAUDE.md`. Kein `[DONE]`-Vermerk geändert. Nicht committet.
