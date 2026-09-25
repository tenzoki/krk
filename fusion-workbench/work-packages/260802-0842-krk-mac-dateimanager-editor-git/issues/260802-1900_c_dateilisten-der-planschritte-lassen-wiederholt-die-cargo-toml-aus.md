Die Dateilisten der Planschritte lassen wiederholt Bau-Dateien aus, drittes Vorkommen

---

Drei aufeinanderfolgende Umsetzungen haben denselben Mangel gemeldet: die Dateiliste eines Schritts nennt die Bau-Datei nicht, die der Schritt zwingend anfassen muss.

| Schritt | Ausgelassen | Gemeldet von | Stand |
|---|---|---|---|
| S5 (über S1) | `[alias] xtask = …` in `.cargo/config.toml` | `coder`, 260802-1741 | behoben, `issues/260802-1755_c_…` |
| S3 | `crates/krk-bench/Cargo.toml` mit `krk-core` als Abhängigkeit | `coder`, 260802-1850 | in der Umsetzung ergänzt, Plan noch nicht |
| S2 | `crates/krk-core/src/lib.rs` mit `pub mod verzeichnis;` | `coder`, 260802-1803 | in der Umsetzung ergänzt, Plan noch nicht |

Jedes einzelne Vorkommen ist harmlos: es gibt genau eine mögliche Auflösung, der `coder` hat sie gefunden und gemeldet statt sie zu verschweigen. **Das Muster ist der Befund**, nicht die einzelne Zeile.

Alle drei folgen derselben Regel: **ein Schritt, der ein neues Modul oder eine neue Kiste in Betrieb nimmt, muss die Datei anfassen, die es einbindet** — die `Cargo.toml` für eine Abhängigkeit, die `lib.rs` für ein Modul, die `.cargo/config.toml` für einen Alias. Die Dateilisten führen konsequent die neu entstehenden Dateien und übergehen konsequent die einbindende.

---

**Was zu tun ist.** Der `planner` geht die Dateilisten der noch offenen Schritte S5 bis S23 einmal unter dieser Regel durch und ergänzt die einbindenden Dateien. Das ist billiger als drei weitere Einzelmeldungen, und es beugt der eigentlichen Gefahr vor: dass eine Auslassung mit **mehr als einer** möglichen Auflösung auftaucht und der `coder` dann rät statt zu melden.

Ebenfalls nachzutragen sind die zwei bereits umgesetzten Ergänzungen, damit der Plan den Dateibestand richtig beschreibt:
- S2: `crates/krk-core/src/lib.rs` (eine Zeile `pub mod verzeichnis;`)
- S3: `crates/krk-bench/Cargo.toml` (eine Zeile `krk-core = { path = "../krk-core" }`)

Am Dateibestand ändert sich dadurch nichts.

**Warum das nicht der `coder` behebt:** der Plan gehört dem `planner`, und alle drei Melder hatten `fusion-workbench/` außer ihrer Historie gesperrt.

---
Resolved: Der `planner` ist am 260802-1859 die Dateilisten aller Schritte einmal unter der Einbindungsregel durchgegangen und hat in S5 bis S23 **35 einbindende Dateien** ergänzt, jeweils mit dem Vermerk `(einbindend)` und der Zeile, um die es geht. Die zwei bereits umgesetzten Nachträge stehen jetzt auch im Plan: S2 nennt `crates/krk-core/src/lib.rs`, S3 nennt `crates/krk-bench/Cargo.toml`. Am Dateibestand ändert sich dadurch nichts.

Die Regel selbst steht nicht mehr nur in diesem Defekt, sondern im Kopf des Abschnitts `## Implementierungsschritte` des Plans, damit der nächste Planungsdurchgang sie nicht wieder verliert. Sie ist dabei um einen Fall erweitert, den die drei gemeldeten Vorkommen noch nicht zeigten: eine neue externe Abhängigkeit bindet in **zwei** Dateien ein, weil dieses Projekt die Versionsangaben in `[workspace.dependencies]` der Workspace-`Cargo.toml` führt und das Mitglied nur `workspace = true` nennt. Betroffen sind S8 (`objc2-quartz-core` für `CADisplayLink`) und S14 (`objc2-core-foundation` für die FSEvents-Parametertypen).

Zwei Auslassungen mit **mehr als einer** möglichen Auflösung sind beim Durchgang aufgefallen und aufgelöst, also genau die Gefahr, die dieser Defekt benennt:

- S12 nannte `crates/krk-core/src/sitzung.rs`, S10 legt `crates/krk-core/src/ablage/sitzung.rs` an. Zwei Dateien gleichen Namens in derselben Kiste für dieselbe Sache. S12 wächst jetzt in die vorhandene Datei hinein.
- S18 nannte `crates/krk-core/src/lesezeichen.rs` neben `crates/krk-core/src/ablage/lesezeichen.rs` aus S10. Dieselbe Auflösung.

Nachzulesen im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Kopfzeile "Nachzug 18:59".
