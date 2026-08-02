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
