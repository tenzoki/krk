# Sortiert KRK Dateinamen sprachsensitiv, und wonach ordnet "Sortierung nach Typ"?

---
**Domain:** code
**Status:** implemented
**Filed by:** orchestrator (gemeldet vom coder bei der Umsetzung von Schritt 2)
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Schritt 2 und Abschnitt `## Datenstrukturen`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C2, `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1803-verzeichnisleser-und-ordnermodell.md`

---

## Question

Der Plan verlangt in Schritt 2 vier Sortierungen, jeweils auf- und absteigend: nach Name, Größe, Änderungsdatum und Typ. Zwei davon lässt er unbestimmt, und der `coder` hat für beide begründet vorbelegt, statt eigenmächtig zu entscheiden. Beide Vorbelegungen sind sichtbar und sollten bestätigt oder geändert werden, bevor mehr darauf aufbaut.

**Die Namenssortierung ordnet ohne sprachsensitive Kollation.** Der Sortierschlüssel ignoriert Groß- und Kleinschreibung, vergleicht aber Zeichen für Zeichen nach Unicode-Position. Damit landet `Äpfel` hinter `Zebra`, weil `Ä` in der Unicode-Tabelle nach `Z` steht. Für eine Anwendung mit deutschsprachiger Oberfläche ist das im Alltag sichtbar: jeder Ordner mit Umlauten sortiert an einer Stelle, an der ihn niemand sucht. Der Finder sortiert `Äpfel` vor `Bäume`.

**Die Typsortierung ordnet nach der Aufzählung Ordner, Datei, Verknüpfung.** Da Ordner in jeder Sortierung ohnehin vorne stehen, unterscheidet sie innerhalb der Dateien nur noch Datei von Verknüpfung, und das ist für die meisten Ordner gar keine Unterscheidung. Üblich und vom Nutzer vermutlich erwartet wäre eine Ordnung nach Dateiendung. Der Grund für die Vorbelegung: `Eintrag` trägt im Abschnitt `## Datenstrukturen` kein Feld für die Endung; sie nachzurüsten ist eine Änderung an der Datenstruktur, kein Detail der Sortierfunktion.

Die Frage muss vor Schritt 12 beantwortet sein, weil die Sortierung dort in der Oberfläche sichtbar wird und Nutzererwartungen weckt. Sie blockiert die Schritte 3 bis 8 nicht.

## Options

1. **Beides jetzt richtigstellen** — sprachsensitive Kollation für die Namenssortierung, Endung als Feld in `Eintrag` und als Schlüssel der Typsortierung.
   - Pro: die Sortierung entspricht dann dem, was ein Mac-Nutzer vom Finder kennt, und niemand muss sich später umgewöhnen.
   - Cons: die Kollation ist keine Kleinigkeit. Rusts Standardbibliothek bringt sie nicht mit; entweder kommt eine Abhängigkeit dazu, oder KRK ruft über `objc2-foundation` die Systemfunktion `localizedStandardCompare:` auf. Der zweite Weg passt zur Technologiewahl, kostet aber je Vergleich einen Fremdaufruf, und die Namenssortierung läuft über 100.000 Einträge in einer Zusage von 4 Sekunden. Ob das trägt, ist ungemessen.

2. **Nur die Kollation richtigstellen, Typsortierung lassen** — Umlaute sortieren richtig, "nach Typ" bleibt Ordner/Datei/Verknüpfung.
   - Pro: behebt den sichtbaren Alltagsfehler und lässt die Datenstruktur unberührt.
   - Cons: "Sortierung nach Typ" bleibt eine Funktion, die im Alltag fast nichts tut.

3. **Beides in dieser Runde so lassen, mit Vermerk** — die jetzige Vorbelegung bleibt, die Frage wird für eine spätere Runde festgehalten.
   - Pro: kein Eingriff in eine Schicht, die gerade als Grundlage der Messung dient. Schritt 8 misst unter anderem das vollständige Lesen und Sortieren; eine Kollation über Fremdaufrufe verändert genau diese Messung, bevor überhaupt eine Vergleichszahl vorliegt.
   - Cons: die erste sichtbare Fassung sortiert Umlaute falsch, und der Nutzer sieht das sofort.

## Constraints

- Die Zusagen L3 und L10 aus C8 gelten für das vollständige Lesen **und Sortieren**: 10.000 Einträge in 400 ms warm, 100.000 in 4 s. Jede Änderung an der Sortierung wirkt unmittelbar auf beide.
- Die Maxime "supersimpel" wirkt als Ausschlussgrund. Zwei Sortierpfade nebeneinander, einer schnell und einer sprachsensitiv, wären eine Fallunterscheidung ohne Gegenwert.
- Der Plan hält in Schritt 8 fest, dass eine verfehlte Zusage nicht repariert, sondern als Entscheidungsvorlage zurückgemeldet wird. Eine Änderung an der Sortierung vor der Erstmessung nimmt dieser Regel die Vergleichsgrundlage.

## Recommendation

**Möglichkeit 3 bis zur Messung, danach Möglichkeit 1 oder 2 entscheiden.**

Der Grund liegt in der Reihenfolge, nicht in der Sache. Die Kollation gehört inhaltlich in eine deutschsprachige Anwendung, das ist unstrittig. Aber Schritt 8 ist als Gate gebaut: er misst, ob Rust mit AppKit die zugesagten Zahlen trägt, und bei einer verfehlten Zahl steht der Technologieentscheid zur Debatte. Wird die Sortierung vorher auf Fremdaufrufe umgestellt, misst dieses Gate eine andere Sache, und ein verfehlter Wert ließe sich nicht mehr zuordnen: liegt es an der Technologiewahl oder an der Kollation?

Nach der Messung liegt eine Vergleichszahl vor, und die Kosten der Kollation lassen sich gegen sie halten statt gegen eine Vermutung.

Diese Abwägung stützt sich auf den Aufbau des Plans, nicht auf eine Messung der Kollationskosten. Die Entscheidung liegt beim Nutzer.

---
Answered:
Implemented:
Deferred:
Superseded by:

---

**Abgleich 260806-1647 — die Frage ist unbeantwortet, und der Schritt, den sie binden sollte, ist längst abgenommen.**

Der Datensatz sagt oben: "Die Frage muss vor Schritt 12 beantwortet sein, weil die Sortierung dort in der Oberfläche sichtbar wird und Nutzererwartungen weckt." S12 trägt seit dem 260804-1040 `[DONE]`, und alle 36 Schritte des Plans stehen inzwischen auf `[DONE]`. Die Frage ist in der Zwischenzeit nicht beantwortet worden.

Nachgesehen am Code, Stand `89f5570`:

- `crates/krk-core/src/verzeichnis/eintrag.rs:80-86` — `sortierschluessel_bauen` schreibt im eigenen Kommentar aus, dass er die sprachsensitive Kollation **nicht** leistet. Die Vorbelegung aus dem Datensatz gilt unverändert: `Äpfel` sortiert hinter `Zebra`.
- `crates/krk-core/src/verzeichnis/eintrag.rs` trägt kein Feld für die Dateiendung; die Typsortierung ordnet weiter nach der Aufzählung Ordner/Datei/Verknüpfung (`crates/krk-core/src/verzeichnis/sortierung.rs`).

Der Marker bleibt `_o_`: beide Vorbelegungen stehen unverändert, und keine ist bestätigt worden. Die Empfehlung des Datensatzes hing an Schritt 8 als Messgate; das Gate ist seit dem 260803-1755 durch, und die Abnahme-Messreihe aus S22 liegt vor (`messungen/260805-2207-MacBookPro15-1-abnahme.txt`). Die Bedingung, unter der die Empfehlung "erst nach der Messung entscheiden" lautete, ist damit erfüllt.

Weder der Plan noch der Spec nennt diesen Datensatz an einer Stelle; gemeldet als `issues/260806-1647_*_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`.

---
Answered: Nutzerentscheid 260806 im Rebalance-Gate des Turns 23 — **Möglichkeit 1, beides richtigstellen.** Die Namenssortierung bekommt sprachsensitive Kollation, und die Dateiendung wird ein Feld in Eintrag und der Schlüssel der Typsortierung. Die Bedingung der ursprünglichen Empfehlung ("erst nach dem Messgate entscheiden") ist erfüllt: das Gate aus S8 ist seit dem 260803-1755 durch, und die Abnahme-Messreihe aus S22 liegt als Vergleichsgrundlage vor (messungen/260805-2207-MacBookPro15-1-abnahme.txt).

Auflage aus den Constraints dieses Datensatzes: L3 und L10 decken Lesen und Sortieren, und beide hängen an der Größe von Eintrag. Die Umsetzung ist deshalb ohne Nachmessung beider Zusagen nicht abgenommen. Verfehlt eine der beiden ihre Zahl, gilt die Regel des Plans — ein neuer Entscheidungsdatensatz, keine stillschweigende Lockerung.

---
Implemented: 16e4558 — crates/krk-core/src/verzeichnis/kollation.rs baut den sprachsensitiven Sortierschlüssel über icu_collator als Bytefolge, Eintrag trägt endungsschluessel und endung_ab, sortierung.rs ordnet die Typsortierung nach der Endung. Gemessen (messungen/260806-1716-MacBookPro15-1-kollation-l3-l10.txt, 95. Perzentil der schlechtesten von fünf Runden): L3 41,5 ms gegen 400 ms zugesagt, L10 463,8 ms gegen 4000 ms — beide gehalten in allen fünf Runden, Faktor 9,6 und 8,6.

Zur Wahl des Weges: icu_collator schreibt den Vergleich als Bytefolge aus (write_sort_key_to) und erhält damit den Zuschnitt aus Schritt 2 — der Schlüssel entsteht einmal beim Lesen, das Sortieren vergleicht nur Bytes. Beide Alternativen kennen allein den paarweisen Vergleich und hätten die Kollation von 100.000 Aufrufen auf rund 1,7 Millionen je Lauf verschoben: localizedStandardCompare: über objc2-foundation hat keine Schlüsselbildung und hätte krk-core zusätzlich an Foundation gebunden, feruca hält sein Modul sort_key privat.

Kosten: Eintrag wächst von 72 auf 88 Bytes, auf der Halde 44,4 statt 56,9 Bytes je Eintrag; das Programm wächst um 1,26 MB durch die eingebackenen CLDR-Tabellen.

Nicht mitentschieden und als eigener Datensatz weitergeführt: welche Sprache die Ordnung bestimmt (decisions/260806-1730_o_welche-sprache-bestimmt-die-sortierordnung.md).
