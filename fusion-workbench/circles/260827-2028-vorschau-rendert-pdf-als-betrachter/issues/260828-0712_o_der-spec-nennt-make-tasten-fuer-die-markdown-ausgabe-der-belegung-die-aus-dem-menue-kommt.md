Der Spec nennt `make tasten` für die Markdown-Ausgabe der Belegung, die aus dem Menü kommt

---

C1.3 und C3.6 des Specs `planning/260828-0649_*_spec-vorschau-rendert-pdf-als-betrachter.md` sagen „die Tastenbelegung als Markdown (`make tasten`)" und „`make tasten` gibt … dieselben Zeilen aus". `make tasten` ist das Ziel `Makefile:88-90` und ruft `krk --tasten-protokoll`: ein Protokoll der gedrückten Tastencodes am laufenden Bündel, beendet mit Cmd+Q, ohne Markdown. Die Markdown-Datei `~/Downloads/KRK-Tastenbelegung.md` entsteht aus dem Menüeintrag „Tastenbelegung als Markdown sichern" (`crates/krk-ui/src/appkit/anwendung.rs:856`, `tastenbelegungSichern:`) über `crates/krk-ui/src/belegungsausgabe.rs`; ihre dritte Spalte kommt aus `Wirkungsbereich::beschriftung` (`belegungsausgabe.rs:262`).

---

**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Betroffen:** Spec C1.3, C3.6 (Wortlaut); kein Code

Der Plan `planning/260828-0712_*_plan-vorschau-rendert-pdf-als-betrachter.md` ordnet beide Kriterien der Markdown-Ausgabe aus `belegungsausgabe.rs` zu und nicht dem Makefile-Ziel. Der Spec ist freigegeben und wird nicht angefasst; der Datensatz hält den Unterschied fest, damit der Abnahmelauf am richtigen Erzeugnis prüft. Abnahmetest: der Nutzer sichert die Belegung über den Menüeintrag und liest die drei Zeilen der Zoombefehle mit der dritten Spalte „Vorschau" in der Datei; `make tasten` wird für C1.3 und C3.6 nicht gefahren.

---
Abgleich 260828-1044: bleibt offen. `Makefile:89` (`tasten: bundle ## Tastencodes protokollieren`) ist unverändert kein Markdown-Erzeugnis; der Spec (`planning/260828-0649_*`, C1.3/C3.6) ist nach dem Abschluss der Runde nicht angefasst worden. Der Abnahmelauf (Plan Schritt 11, `48cd818`) hat über den Menüeintrag geprüft, wie dieser Datensatz es vorgibt.
