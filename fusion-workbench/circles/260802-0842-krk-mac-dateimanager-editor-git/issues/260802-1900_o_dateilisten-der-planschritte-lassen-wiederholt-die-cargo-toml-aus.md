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
