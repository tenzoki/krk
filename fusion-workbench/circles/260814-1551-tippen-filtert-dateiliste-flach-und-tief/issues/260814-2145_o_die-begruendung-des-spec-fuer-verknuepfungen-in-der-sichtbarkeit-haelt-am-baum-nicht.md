# Die Begründung des Spec für Verknüpfungen in der Sichtbarkeit hält am Baum nicht

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, Abschnitt `## Wie eine Zeile entsteht`, Absatz „Für „Ordner" gibt es einen Schnitt und nicht zwei"; C1.6, C2.13, C3.9; `crates/krk-ui/src/appkit/tabelle.rs:1320` (`eintrag_in_zeile`); `crates/krk-core/src/verzeichnis/eintrag.rs:111-118` (`ist_ordner`, `ist_verknuepfung`); `crates/krk-core/src/verzeichnis/modell.rs` (`Ordnermodell::sichtbar`, Zweig „ist es ein Ordner?")

---

Der Spec begründet den Zweig `ist es ein Ordner?` für eine symbolische Verknüpfung so: „Für eine symbolische Verknüpfung auf einen Ordner antwortet dieser Knoten mit ja, **denn der Nutzer navigiert in sie hinein**." Die Begründung trifft für diesen Baum nicht zu, und die Regel selbst lässt sich am `Ordnermodell` nicht in der Form stellen, in der der Spec sie formuliert.

**Erstens: KRK navigiert heute nicht in eine Verknüpfung hinein.** `DateifensterQuelle::eintrag_in_zeile` (`appkit/tabelle.rs:1320`) beantwortet die Frage „ist das ein Ordner" mit `eintrag.ist_ordner()`, und `Eintrag::ist_ordner` ist `self.typ == Typ::Ordner` — eine `Typ::Verknuepfung` fällt nicht darunter. Ein Doppelklick auf eine Verknüpfung auf einen Ordner geht damit an das System und nicht in den Ordner hinein. Die Begründung des Spec beschreibt ein Verhalten, das dieser Baum nicht hat.

**Zweitens: „auf einen Ordner" ist am Modell nicht entscheidbar.** `Eintrag` trägt drei Typwerte und kein Ziel; der Leser folgt der Verknüpfung nicht (`verzeichnis/sys.rs:341`, `VLNK => Typ::Verknuepfung`). Ob eine Verknüpfung auf einen Ordner oder auf eine Datei zeigt, weiß `Ordnermodell::sichtbar` nicht und könnte es nur mit einem `stat(2)` je Zeile erfahren — genau die Art Arbeit, die der Sortierschlüssel und der gestückelte Lesevorgang aus dem Weg jeder Zeile heraushalten.

**Was Schritt A1 daraus gemacht hat.** Der Zweig lautet `eintrag.ist_ordner() || eintrag.ist_verknuepfung()`, also: **jede** Verknüpfung zählt für die Sichtbarkeit als Ordner. Das hält C1.6, C2.13 und C3.9 für Verknüpfungen auf Ordner wörtlich. Der Preis ist eine Übermenge: auch eine Verknüpfung auf eine **Datei** bleibt bei ausgeschaltetem „Deep" stehen, obwohl ihr Name den Filtertext nicht trägt. Kein Kriterium des Spec spricht diesen Fall an, und die Alternative — Verknüpfungen wie Dateien zu behandeln — bräche C2.13 („Ist „Deep" aus, bleibt sie sichtbar wie jeder Ordner") für Verknüpfungen auf Ordner.

Die Probe `eine_verknuepfung_zaehlt_fuer_die_sichtbarkeit_als_ordner` (`crates/krk-core/tests/verzeichnis.rs`) hält den umgesetzten Zuschnitt fest.

---

**Was zu entscheiden wäre.** Dreierlei, und keines hält einen Planschritt auf:

1. Ob die Begründung im Spec berichtigt wird — sie nennt ein Navigationsverhalten, das erst noch zu bauen wäre.
2. Ob eine Verknüpfung auf eine Datei bei flacher Suche stehen bleiben soll. Wenn nicht, braucht die Regel die Zielart, und die kostet einen Systemaufruf je Verknüpfung.
3. Ob KRK in eine Verknüpfung auf einen Ordner hineinnavigieren soll. Das liegt außerhalb dieser Runde und ist der Grund, aus dem die Frage überhaupt auffällt.

**Kontext.** Aufgefallen beim Umsetzen von Schritt A1 (`planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`), beim Nachbauen des ersten Spec-Bildes Zweig für Zweig. Aus dieser Directive entstanden, deshalb im Circle und nicht im gemeinsamen Speicher.
