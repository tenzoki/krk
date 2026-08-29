# C6.6 sagt, ein einzelnes `*` stoße den Durchlauf an, und das Modell schickt ihm nichts

**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Spec-Prosa, kein Codefehler)
**Executor:** analyst oder Nutzer (Spec-Wortlaut)

Der Spec der Runde 21 (`planning/260829-1052_*_spec-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, C6.6 und B6) formuliert, ein einzelnes `*` als Filtertext stoße den tiefen Durchlauf an und dieser entscheide jeden Ordner am ersten Eintrag. Am Modell trifft die erste Hälfte nicht zu: der Kurzschluss des Namens in `zeilengrund_von` (`crates/krk-core/src/verzeichnis/modell.rs`) steht vor dem Unterbaumzweig, bei `*` trägt jeder Ordnername das Muster, und `auftraege()` bleibt leer — der Durchlauf bekommt nichts. Die zweite Hälfte hält, wenn man dem Durchlauf Aufträge von Hand gibt. Die Probe `ein_einzelnes_sternchen_stoesst_den_durchlauf_an_und_entscheidet_jeden_ordner_mit_dem_ersten_eintrag` (`crates/krk-core/tests/verzeichnis.rs`, Schritt 9) hält beides so, wie es ist. Das Verhalten ist günstiger als beschrieben; die Prosa des Specs ist an dieser Stelle ungenau und wird nach der Ortsregel nicht rückwirkend geändert. Gemeldet vom Schritt-9-Coder (`history/260829-1200-coder-schritt-9-…`).

---
Reconciled 260829-1223: weiter offen. Kein Commit zwischen `79d507a` und `8d64859` fasst die genannte Stelle an; die Lage ist am Baum nachgelesen (siehe `history/260829-1223-reconciliation.md` dieses Circles für den Beleg je Datensatz). Keine Vorbedingung des Abschlusses der Runde 21.
