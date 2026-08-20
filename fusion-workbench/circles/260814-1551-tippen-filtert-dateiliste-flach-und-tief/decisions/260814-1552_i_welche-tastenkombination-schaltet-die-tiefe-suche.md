# Welche Tastenkombination schaltet die tiefe Suche?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `resources/default-keymap.toml` (die eine Quelle jeder Belegung); `crates/krk-ui/src/appkit/bereichsleiste.rs` (die acht Ankreuzfelder); `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md` (C2, erstes Abnahmekriterium)

---

## Question

Der Schalter „Tief" bekommt ein neuntes Ankreuzfeld in der Bereichsleiste, und C2 der Runde 1 verlangt für jede Funktion mindestens einen Tastenbefehl: keine Funktion ist ausschließlich per Maus bedienbar. Welche Kombination das ist, ist nicht entschieden. Die Frage muss vor der Umsetzung fallen, weil sie in `resources/default-keymap.toml` steht, der einen Quelle jeder Belegung, und weil die Belegung am 260814 bereits 83 Einträge führt. Eine Kombination, die später umzieht, zieht die Belegungsansicht, das Hauptmenü und die Markdown-Ausgabe der Belegung mit.

## Options

1. **Eine Kombination der `f`-Familie**, etwa `cmd+f` oder `shift+cmd+f`.
   - Pro: `cmd+f` ist auf dem Mac die vertrauteste Taste für „Suchen" und träfe die Erwartung ohne Erklärung.
   - Kontra: KRK hat kein Suchfeld, in das `cmd+f` führte. Die Taste schaltete hier eine Eigenschaft einer Suche um, die der Nutzer bereits getippt hat, und wäre damit an eine vertraute Taste eine unvertraute Bedeutung gehängt. Ob eine der beiden heute frei ist, ist gegen die Belegungsdatei zu prüfen.
2. **Die `opt+cmd`-Ebene**, in der `opt+cmd+n`, `opt+cmd+c` und `opt+cmd+g` schon in Gebrauch sind.
   - Pro: die Ebene ist im Baum als Ort für die zweite Reihe von Befehlen etabliert und dort ist nach Ordnung Platz.
   - Kontra: schlechter erreichbar als eine Kombination mit einer Zusatztaste, und der Schalter gehört zu einer Handlung, die der Nutzer mitten im Tippen auslöst.
3. **Der Tabulator oder eine andere freie Taste ohne Zusatztaste**, solange ein Filtertext steht.
   - Pro: mitten im Tippen ohne Handwechsel erreichbar, was zu einer Funktion passt, die erst wirkt, sobald ein Filtertext steht.
   - Kontra: eine Taste, deren Bedeutung vom Zustand abhängt, ist in diesem Baum bisher die Ausnahme und keine Regel. Sie kollidierte außerdem mit der Regel, dass eine freie Taste ohne Zusatztaste im Dateifenster in den Filter fällt.

## Constraints

- Die Kombination steht in `resources/default-keymap.toml` und nirgends sonst; `make tasten` und `make menue` geben den gebauten Stand aus.
- Der Befehl braucht eine Zeile in `Kommando::wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs`) und in `bereich_des_kommandos` (`crates/krk-ui/src/belegungsmodell.rs`). Beide Fallunterscheidungen sind vollständig und halten den Bau an, wenn eine Zeile fehlt.
- Der Befehl gehört in das vollständige Hauptmenü aus der Runde 7.
- Die gewählte Kombination darf keine vergebene sein. Der Stand ist gegen `resources/default-keymap.toml` zu prüfen und nicht gegen eine Aufstellung in einem Dokument.
- Der Schalter wirkt erst, sobald ein Filtertext steht. Was der Befehl ohne Filtertext tut, gehört zur Antwort: nichts tun und nichts melden, oder melden, dass er nichts findet. Die Regel des Baums lautet, dass der Wirkungsbereich entscheidet, ob eine Taste durchkommt, und nicht, ob sie etwas findet.

## Recommendation

Keine. Die drei Möglichkeiten unterscheiden sich in einer Abwägung, die der Nutzer selbst führt, nämlich zwischen Vertrautheit und Erreichbarkeit, und der Baum trägt für keine der drei ein Argument, das die andere ausschlösse.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1610 im Orchestrator-Dialog — keine Tastenkombination. "Deep" ist ein Ankreuzfeld in der Bereichsleiste, neben dem Kaestchen "Typ". Der Nutzer hat das dreimal so gesagt; die Frage nach einer Kombination war eine Fehlvorlage des Orchestrators. Der Baum traegt das Muster bereits: `spalte_typ_umschalten` fuehrt `tasten = []` (`resources/default-keymap.toml:383`) und ist ueber sein Kaestchen und das Hauptmenue erreichbar. Der neue Befehl macht es gleich. Damit bleiben die drei genannten Moeglichkeiten unbenutzt; `shift+cmd+f`, `opt+cmd+f`, `ctrl+cmd+f` und der nackte Tabulator sind weiterhin frei.

---
Implemented: `d73be91` — `resources/default-keymap.toml:459-462` führt die Funktion `tiefe_suche_umschalten` mit `tasten = []` und nicht mit `reserviert_fuer`, genau nach dem Muster von `spalte_typ_umschalten`. Der Kommentar darüber schreibt die Folge aus: der Eintrag fällt aus der Markdown-Ausgabe der Runde 3, weil sie eine Funktion nur bei mindestens einer Kombination aufnimmt. Die drei erwogenen Kombinationen und der nackte Tabulator sind unbenutzt geblieben. Abgeglichen am 260820-2056 gegen `f5300f4`.
