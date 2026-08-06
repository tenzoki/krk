Die Sortierfrage bindet S12, steht aber in keiner Stelle von Plan und Spec

---

Der Entscheidungsdatensatz `decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md` schreibt sich selbst eine Bindung zu: "Die Frage muss vor Schritt 12 beantwortet sein, weil die Sortierung dort in der Oberfläche sichtbar wird und Nutzererwartungen weckt."

S12 trägt seit dem 260804-1040 `[DONE]`, alle 36 Schritte des Plans stehen inzwischen auf `[DONE]`, und die Frage ist unbeantwortet. Ihr Marker ist unverändert `_o_`.

**Der Datensatz taucht in keiner Planstelle auf.** Gesucht wurde über den ganzen Projektbaum nach dem Thementeil des Dateinamens; getroffen hat allein `CLAUDE.md:79`. Weder `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` noch `planning/260802-1036_*_spec-navigator-geruest.md` nennt ihn — nicht in S2, nicht in S12, nicht in `## Angelegte Defekte und Entscheidungen`, nicht in `## Offene Fragen`. Die eine Planstelle, die den Sortierschlüssel überhaupt berührt (Zeile 115), begründet die Vorberechnung mit den Kosten eines sprachsensitiven Vergleichs und erwähnt die offene Frage dahinter nicht.

Die eine Stelle, die ihn führt, sagt es überdies falsch herum: `CLAUDE.md:81` schreibt "Die Sortierfrage bindet Schritt S12" im Präsens, als stünde S12 noch aus.

---

## Was tatsächlich ausgeliefert wird

Nachgesehen am Stand `89f5570`:

- `crates/krk-core/src/verzeichnis/eintrag.rs:80-86` — `sortierschluessel_bauen` vergleicht Zeichen für Zeichen nach Unicode-Position und schreibt im eigenen Kommentar aus, dass er die Kollation nicht leistet. In einer Anwendung mit deutschsprachiger Oberfläche sortiert damit jeder Ordner mit Umlauten an einer Stelle, an der ihn niemand sucht.
- `crates/krk-core/src/verzeichnis/sortierung.rs` — "Sortierung nach Typ" ordnet nach der Aufzählung Ordner/Datei/Verknüpfung. `Eintrag` trägt kein Feld für die Endung; da Ordner in jeder Sortierung ohnehin vorn stehen, unterscheidet die Funktion innerhalb der Dateien nur noch Datei von Verknüpfung.

Beides ist eine Vorbelegung des `coder` aus der Umsetzung von S2, ausdrücklich als solche gemeldet und nie bestätigt.

## Warum es zählt

Runde 1 steht vor der Schließung. Sie schließt nach dem Nutzerentscheid vom 260806 erst nach der Klärung der L9-Frage — eine zweite Frage, die eine sichtbare Eigenschaft der ausgelieferten Anwendung festlegt, sollte an derselben Stelle auf dem Tisch liegen und nicht allein in einer Datei stehen, die selbst als veraltet gemeldet ist (`issues/260806-0904_*_claude-md-fuehrt-projektstand-und-entscheidungsstand-vom-260803.md`).

Die Empfehlung des Datensatzes lautete "Möglichkeit 3 bis zur Messung, danach Möglichkeit 1 oder 2 entscheiden", und der Grund war das Messgate S8: eine vorher umgestellte Sortierung hätte die Messung mehrdeutig gemacht. Das Gate ist seit dem 260803-1755 durch, die vollständige Abnahme-Messreihe aus S22 liegt seit dem 260806-0018 vor. Die Bedingung, unter der die Empfehlung wartete, ist erfüllt.

**Zuständig:** Nachzug an Plan und Spec (planner) und ein Nutzerentscheid am Datensatz; kein Codeeingriff, solange die Frage offen ist.

**Aufgefallen bei:** dem Reconciler-Abgleich 260806-1647 nach Turn 23.
