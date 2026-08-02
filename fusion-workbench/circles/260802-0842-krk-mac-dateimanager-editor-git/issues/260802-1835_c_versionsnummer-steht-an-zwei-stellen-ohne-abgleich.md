Die Versionsnummer steht an zwei Stellen, und nichts hält sie zusammen

---

Seit Schritt 4 trägt das Projekt seine Version doppelt:

- `Cargo.toml`, Feld `version` des Workspace: `0.1.0`
- `resources/Info.plist`, Schlüssel `CFBundleShortVersionString`: `0.1.0`

Schritt 5 des Plans `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` kopiert die `Info.plist` unverändert ins Bündel. Es gibt keine Ersetzung beim Bauen und keine Prüfung, dass beide Werte übereinstimmen.

Beim nächsten Anheben der Version wird eine der beiden Stellen vergessen. Das fällt nicht beim Bauen auf, sondern erst, wenn jemand die Version der ausgelieferten Anwendung mit der des Quellstands vergleicht — oder gar nicht.

---

**Was zu tun ist.** Der `planner` entscheidet, an welcher Stelle die Version wohnt, und ergänzt Schritt 5 entsprechend. Zwei Wege bieten sich an:

- **`Cargo.toml` führt die Version, das Bauwerkzeug setzt sie ein.** `xtask bundle` liest `CARGO_PKG_VERSION` und ersetzt den Platzhalter in der `Info.plist` beim Kopieren. Eine Quelle, ein Ort zum Ändern. Kostet eine Ersetzung im Bauvorgang, die es bisher nicht gibt.
- **Beide bleiben stehen, das Bauwerkzeug prüft die Gleichheit.** `xtask bundle` bricht ab, wenn die Werte auseinanderlaufen. Billiger zu bauen, aber die Doppelung bleibt und muss weiterhin von Hand gepflegt werden.

Der erste Weg ist der gründlichere und entspricht der Maxime "supersimpel" besser: er beseitigt die Doppelung, statt sie zu bewachen.

**Kein Handlungsdruck vor Schritt 5.** Am jetzigen Dateibestand ist nichts falsch, beide Werte stehen auf `0.1.0`. Der Defekt beschreibt eine Falle, die beim ersten Versionswechsel zuschnappt.

**Aufgefallen bei:** der Umsetzung von Schritt 4 durch den `ontocoder`, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1829-buendelbeschreibung-info-plist.md`. Der `ontocoder` hat einen Kommentar an die Stelle in der `Info.plist` geschrieben und den Defekt gemeldet, weil ihm `fusion-workbench/` für diesen Schritt gesperrt war.

---
Resolved: Die Version wohnt ab jetzt allein in `[workspace.package]` der `Cargo.toml`. Der erste der beiden vorgeschlagenen Wege ist gewählt, und die Prüfung bestätigt die Einschätzung dieses Defekts: er beseitigt die Doppelung, statt sie zu bewachen, und der zweite Weg hätte eine zweite Stelle zu pflegen gelassen und dafür einen Wächter gebaut.

Der Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` ist am 260802-1859 dafür nachgezogen:

- **Neuer Schritt S4b** (`ontocoder`, zwischen S4 und S5): der Wert von `CFBundleShortVersionString` in `resources/Info.plist` wird zum Platzhalter `__KRK_VERSION__`, der Kommentar an dieser Stelle wird ersetzt. `CFBundleVersion` bleibt bei `1`; es ist die Baunummer, steht nirgends ein zweites Mal und gehört nicht zu dieser Doppelung.
- **S5 ersetzt beim Kopieren.** `bundle.rs` setzt `env!("CARGO_PKG_VERSION")` an die Stelle des Platzhalters. Der Wert stimmt, weil `xtask/Cargo.toml` `version.workspace = true` trägt, geprüft an der Datei; der Plan schreibt diese Erbschaft als Voraussetzung aus. Findet die Ersetzung den Platzhalter nicht, bricht `bundle` ab und baut kein Bündel, damit weder eine veraltete Zahl noch ein versionsloses Bündel stillschweigend entsteht.
- **Das Abnahmekriterium von S5 vergleicht gegen die `Cargo.toml`**, nicht gegen ein Literal im Testbefehl. Ein Literal wäre die dritte Stelle gewesen, an der `0.1.0` steht.

**Warum das ein eigener Schritt wurde.** Die Behebung fasst zwei Dateien an, eine Datendatei und ein Bauwerkzeug. Die Zuschnittregel des Plans erlaubt keinen Schritt mit zwei Ausführenden, also steht die Datenänderung in S4b und die Ersetzung in S5. Die Nummerierung bleibt bei S5 bis S23, damit kein Verweis in Plan, Spec, Defekten und Entscheidungsdatensätzen bricht.

Der Kommentar, den der `ontocoder` am 260802-1829 an die Stelle geschrieben hat, wird von S4b ersetzt; er beschreibt dann nicht mehr den Stand.
