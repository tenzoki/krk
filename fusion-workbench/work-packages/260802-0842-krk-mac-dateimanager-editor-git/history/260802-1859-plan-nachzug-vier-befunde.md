# Plan-Nachzug: vier Befunde aus den Umsetzungen der Schritte 2 bis 4

**Datum:** 2026-08-02, 18:59
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Bearbeitete Datei:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Marker bleibt offen)

---

## Auftrag

Vier Defekte aus den Umsetzungen der Schritte S2 bis S4 in einem Zug in den Plan nachziehen. S1 bis S4 sind umgesetzt, abgenommen und committet, zuletzt `dba616a`; S5 wartet. Grenzen: nur die Plandatei und die vier Defekte, plus ein etwaiger Entscheidungsdatensatz zu L8. Nicht committen. Spec, Circle-Datensatz, `crates/`, `resources/` und `spikes/` nur lesend.

## Was geändert wurde

### Befund 1: die Dateilisten lassen die einbindende Datei aus

Die Dateilisten aller Schritte sind einmal unter der Regel durchgegangen, dass ein Schritt auch die Datei anfassen muss, die sein neues Modul oder seine neue Abhängigkeit einbindet. **35 einbindende Dateien** sind in S5 bis S23 ergänzt, jeweils mit dem Vermerk `(einbindend)` und der Zeile, um die es geht. Die zwei bereits umgesetzten Nachträge stehen jetzt auch im Plan: S2 nennt `crates/krk-core/src/lib.rs`, S3 nennt `crates/krk-bench/Cargo.toml`.

Die Regel selbst steht jetzt im Kopf des Abschnitts `## Implementierungsschritte`, damit sie nicht wieder verloren geht. Sie ist um einen Fall erweitert, den die drei gemeldeten Vorkommen noch nicht zeigten: eine neue externe Abhängigkeit bindet in zwei Dateien ein, weil das Projekt die Versionsangaben in `[workspace.dependencies]` der Workspace-`Cargo.toml` führt und das Mitglied nur `workspace = true` nennt. Geprüft an `Cargo.toml`, `crates/krk-ui/Cargo.toml` und `crates/krk-bench/Cargo.toml`. Betroffen sind S8 und S14.

Zwei Auslassungen mit mehr als einer möglichen Auflösung sind beim Durchgang aufgefallen, also genau die Gefahr, die der Defekt benennt. S12 hätte eine zweite Datei `sitzung.rs` neben der aus S10 angelegt, S18 eine zweite `lesezeichen.rs`. Beide wachsen jetzt in die vorhandene Datei aus S10 hinein.

Verteilung der 35 Ergänzungen: S6 eine, S7 zwei, S8 drei, S10 eine, S11 eine, S12 drei, S13 drei, S14 vier, S15 zwei, S16 drei, S17 drei, S18 zwei, S19 zwei, S20 zwei, S21 zwei, S23 eine.

### Befund 2: die Bildwiederholrate ist per `system_profiler` nicht erhebbar

Die Rate kommt aus `NSScreen.maximumFramesPerSecond`, gelesen von dem Bildschirm, auf dem das gemessene Fenster steht, also über `NSWindow.screen()`. Hat das Fenster keinen Bildschirm, bricht der Messlauf ab statt auf den Hauptbildschirm auszuweichen; dieselbe Haltung wie bei `--kalt` ohne Rechte.

Die Verfügbarkeit ist geprüft, nicht angenommen: `objc2-app-kit-0.3.2`, `src/generated/NSScreen.rs:168-170`, Rückgabe `NSInteger`.

Eine Abweichung vom Vorschlag des Defekts: **S8 erhebt die Rate bereits**, nicht erst S21. S8 ist die erste Messung in der Anwendung und misst L1; die Rate erst in S21 zu erheben hieße, das Gate S8 gegen eine Zusage abzunehmen, deren Herleitung dort noch ungeprüft ist. S21 schreibt die Regel aus.

Nebenbefund erledigt: S3 verlangte sechs Kopfangaben, `### Frage 5` acht. Beide Stellen nennen jetzt acht, mit der Bildwiederholrate als ausgeschriebener Lücke auf der kopflosen Strecke.

### Befund 3: die Prüfordner sind dünnbesetzt

**Die Datenmenge für L8 ist ableitbar; es entsteht kein Entscheidungsdatensatz und kein dichter Prüfordner.** Die Frage nach der Datenmenge ruht auf einer Prämisse, die C8 nicht trägt: L8 sagt Sichtbarkeit des Fortschritts nach 200 ms zu, nicht Durchsatz. Ausgelöst wird die Sichtbarkeit nach der 150-ms-Regel aus `### Frage 6` von einem Zeitgeber. Der Prüfbestand muss deshalb nur nach 150 ms noch laufen.

Gemessen auf dem Referenzgerät, mit 10.000 dünnbesetzten Einträgen und dem Ziel auf demselben APFS-Datenträger: `cp -Rc` braucht 1,83 bis 1,95 s über drei Läufe, `cp -R` ohne Klonen 4,44 bis 4,51 s. Beide liegen mehr als das Zehnfache über 150 ms, weil die Laufzeit an der Zahl der Einträge hängt und nicht an den Bytes.

Zur Beobachtung des Defekts, die beiden Wege lägen um Größenordnungen auseinander: am Referenzgerät tun sie das nicht, weil macOS die Löcher auf beiden Wegen erhält. Eine 1-GB-Datei mit 512 echten Bytes belegt nach `cp -c` wie nach `cp` unverändert 8 Blöcke.

Die Löcher sind an anderer Stelle gefährlich, und dort steht jetzt eine Bedingung: ein Ziel auf einem Datenträger ohne Lochunterstützung zwingt `copyfile`, sie als Nullen auszuschreiben, aus 342 MB würden 197 GB. Die Messstrecke nimmt für L8 und L9 nur ein Ziel auf demselben APFS-Datenträger an und bricht sonst ab. Als Risikozeile aufgenommen.

### Befund 4: die Versionsnummer steht doppelt

Die Version wohnt allein in `[workspace.package]` der `Cargo.toml`. Der erste der beiden vorgeschlagenen Wege ist gewählt; die Prüfung bestätigt die Einschätzung des Defekts, dass er die Doppelung beseitigt statt sie zu bewachen.

Neuer Schritt **S4b** (`ontocoder`, zwischen S4 und S5) setzt den Platzhalter `__KRK_VERSION__` in `resources/Info.plist`. S5 ersetzt ihn beim Kopieren durch `env!("CARGO_PKG_VERSION")` und bricht ab, wenn er fehlt. Der Wert stimmt, weil `xtask/Cargo.toml` `version.workspace = true` trägt, geprüft an der Datei; der Plan schreibt die Erbschaft als Voraussetzung aus. Das Abnahmekriterium vergleicht gegen die `Cargo.toml` und nicht gegen ein Literal, das die dritte Stelle mit `0.1.0` gewesen wäre.

Ein eigener Schritt wurde nötig, weil die Behebung eine Datendatei und ein Bauwerkzeug anfasst und die Zuschnittregel des Plans keinen Schritt mit zwei Ausführenden erlaubt. Die Nummerierung bleibt bei S5 bis S23, damit kein Verweis bricht.

## Diagramm-Selbstprüfung

Der Abhängigkeitsgraph der Schritte hat jetzt 24 Knoten und 34 Kanten, Verhältnis 1,42, zyklenfrei, sechs Phasen. Maschinell nachgerechnet: keine Kante läuft gegen die Nummernfolge, kein Knoten ist verwaist, keine Kante zeigt auf einen unbekannten Knoten. Höchster Ausgangsgrad 4 bei S1, höchster Eingangsgrad 5 bei S23, beide unverändert. Die drei übrigen Diagramme sind nicht angefasst.

## Geschlossene Defekte

Alle vier auf Marker geschlossen, jeder mit einem `Resolved:`-Abschnitt:

- `issues/260802-1900_c_dateilisten-der-planschritte-lassen-wiederholt-die-cargo-toml-aus.md`
- `issues/260802-1900_c_bildwiederholrate-am-referenzgeraet-nicht-per-system-profiler-erhebbar.md`
- `issues/260802-1900_c_pruefordner-sind-duennbesetzt-und-taugen-nicht-fuer-die-kopiermessung.md`
- `issues/260802-1835_c_versionsnummer-steht-an-zwei-stellen-ohne-abgleich.md`

Kein neuer Entscheidungsdatensatz. Kein Commit, wie beauftragt. Der Plan bleibt auf Marker offen.

## Nicht angefasst, außerhalb des Auftrags

`issues/260802-1810_o_abnahmekriterium-mit-grep-unsafe-kann-nicht-aufgehen.md` ist offen. Der Defekt liegt im Plan, in den Abnahmekriterien von S2 und S15, und der Auftrag nannte ihn nicht. Er ist für die nächste Runde vorzumerken; S15 kommt erst in Phase D, S2 ist bereits abgenommen.
