# Orchestrator Session — 260814-1500

**Directive:** Tippen filtert die Dateiliste im Dateifenster: jedes Zeichen ohne Zusatztaste blendet aus, was seinen Namen nicht traegt, und ein Ankreuzfeld "Deep" in der Bereichsleiste dehnt den Filter auf den ganzen Unterbaum aus.
**Mode:** plan (Circle 260814-1551-tippen-filtert-dateiliste-flach-und-tief)
**Status:** In Arbeit

## Bestandsaufnahme beim Start

| Größe | Wert |
|---|---|
| Arbeitsverzeichnis | `/Users/k1/Projects/productive/krk` |
| Plugin-Version | 8.2.0 |
| git HEAD | `43dfe90` |
| Aktiver Circle | keiner |
| Circles | 1 vorgesehen (`_a_`), 8 beschränkt geschlossen (`_b_`), 1 kohärent geschlossen (`_c_`) |
| Offene Defekte (gemeinsamer Speicher) | 11 |
| Offene Defekte (alle Circle-Speicher) | 78 |
| Offene Planschritte (gemeinsam) | 1 Plandatei |
| Offene Entscheidungsfragen (alle Speicher) | 19 |
| Analysen (gemeinsam) | 0 |
| Warteschlange `tasklist.md` | nicht vorhanden |
| Rundenbudget | 5 |
| Wächter | kein Halt aktiv (`haltActive: false`) |

## Erkannte Domäne

`code` — 135 Quelldateien gegen 11 Datendateien, gezählt über `git ls-files`
(`counted_by=git-ls-files`). Der Zweig `code_files > 0` greift; das Datenverhältnis
bleibt weit unter der Zweifachschwelle.

## Sitzungsmarker

Beim Start lag ein veralteter Sitzungsmarker vor (Herzschlag 3794 s alt, Schwelle 600 s),
also kein paralleler Lauf. Marker für diese Sitzung neu geschrieben.

## Häufig geänderte Dateien

Aus `fusion-churn-rank` (971 Einträge, 455 davon zu nicht mehr vorhandenen Dateien,
2 als Rauschen verworfen, 10 gewertet). Die vier obersten:

| Punkte | Datei |
|---|---|
| 183 | `crates/krk-ui/src/appkit/anwendung.rs` |
| 88 | `crates/krk-ui/src/appkit/editor.rs` |
| 71 | `crates/krk-ui/src/appkit/tabelle.rs` |
| 60 | `crates/krk-ui/src/appkit/vorschau.rs` |

## Hinweis Portfolio

Ein vorgesehener Circle liegt bereit. `/fusion:next` zeigt das Portfolio.

---

## Turn 1 — 260814-2131 bis 260815-0410

**Vierzehn Planschritte, zwölf Commits, `ba4af5f..50742a6`.** Der Baum ist nach jedem Schritt über `make check` grün gefahren: Bau, Proben, Clippy unter `-D warnings`, Formatprüfung.

| Aufgabe | Commit | Gegenstand |
|---|---|---|
| A1 | `ba4af5f` | Ein Prüfschritt für die Sichtbarkeit, vier Felder am Ordnermodell |
| C1 | `2ff4b5a` | Die Rückschritt-Regel als reine Funktion |
| E1, E2, E2b | `d73be91` | 78. Kommando, 84. Belegungseintrag, drei Proben nachgezogen |
| C2 | `1ac8842` | Der Anschlag reist mit, die Rücktaste fällt an der richtigen Stelle |
| B1 | `fbab27b` | Das Tippen filtert, die Sprungmarke ist abgelöst |
| B2 | `14718c2` | Der Filter überlebt den Tabwechsel, `Esc` räumt ihn als letztes weg |
| D1 | `2d3d971` | Der sechste Rang der Statuszeile |
| E3 | `a0f76fe` | Das neunte Ankreuzfeld |
| F1 | `2cdd299` | Der Durchlauf über den Unterbaum |
| A2 | `9e1892d` | Die Sprungmarke fällt, ein Vergleich bleibt |
| F2 | `9789115` | Der Tab hält den Durchlauf |
| G1, G2 | `50742a6` | Die Abnahmeliste nach dem Vorbild der Runde 8 |

**Ein Schritt außerhalb des Plans.** E2b zog drei Proben auf die vierte ab Werk unbelegte Funktion nach. E1 und E2 teilen eine Zusicherung, die eine Probe hält und keiner von beiden allein halten kann; der Baum ist zwischen ihnen rot und war es nur dort. Beide gingen deshalb in einen Commit.

**Was die Prüfungen vor dem Bau gefunden haben.** Zwei Diagrammprüfungen des Spec, je ein echter Fehler. Der ersten fehlte im Durchlauf der Ausgang für „kein Treffer darunter", den drei Abnahmekriterien voraussetzen; die zweite fand zwei Widersprüche zwischen Abnahmekriterien. Beide sind an der Wurzel behoben und nicht am Ausgang.

**Was die Zählproben während des Baus gefunden haben.** F2 rief zunächst den einen Vergleich unmittelbar aus `tabs.rs`, und die Probe aus A2 schlug fehl: drei Rufer statt zwei. Nicht die Zahl in der Probe ist erhöht worden, sondern die Wurzel behoben.

**Offen:** zehn Defekte und fünf Fragen im Circle. Der Abnahmelauf am laufenden Bündel steht aus und ist Nutzerarbeit; die Liste dafür ist `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`.
