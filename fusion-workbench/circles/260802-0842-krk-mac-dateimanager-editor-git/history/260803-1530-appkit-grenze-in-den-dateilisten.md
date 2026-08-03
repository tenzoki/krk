# Die Grenze zum Modul `appkit` in die Dateilisten gezogen

**Datum:** 2026-08-03, 15:30
**Agent:** planner
**Status:** Complete
**Auslöser:** `issues/260803-1345_o_dateiliste-von-s8-legt-objc2-code-ausserhalb-von-appkit-ab.md`

---

## Auftrag

Zwei Teile. Erstens Schritt 8 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` übersetzbar machen, indem der `objc2`-Anteil aus `crates/krk-ui/src/messmodus.rs` unter `crates/krk-ui/src/appkit/` wandert. Zweitens jede Dateiliste durchsehen, die eine Datei unter `crates/krk-ui/` nennt, in beide Richtungen.

Grenzen des Auftrags: nur die Plandatei, der Defekt, diese Historiendatei und neue Datensätze unter `issues/`. Kein Eingriff in `crates/`, `xtask/`, `resources/`, `README.md` oder `CLAUDE.md`. Keine Schrittnummern, keine Abhängigkeiten, kein `[DONE]`-Vermerk geändert, nicht committet. Alle Grenzen eingehalten.

## Was geprüft wurde, und woran

Vor dem Planen ist der vorhandene Code gelesen worden, um den Zuschnitt anzuschließen statt einen zweiten danebenzustellen. Das Muster der fünf Module unter `crates/krk-ui/src/appkit/` ist durchgängig: ein Modul je AppKit-Objekt, eine sichere Hülle je Aufruf, und was die Hülle verlässt, ist ein gewöhnlicher Rust-Wert. `Tastenabgriff` in `appkit/ereignisse.rs:45` ist der nächste Verwandte des neuen `Zeichenende`: ein Wert, der beim Einrichten eine Senke nimmt und sich in seinem `Drop` bei AppKit wieder abmeldet.

Die Sicherheitseinstufung der Bindungen ist an den Quellen von `objc2-app-kit` 0.3.2 und `objc2-foundation` 0.3.2 unter `~/.cargo/registry/src/` nachgelesen, nicht aus dem Gedächtnis behauptet. Das Ergebnis hat die Aufgabe verändert, siehe unten.

## Schritt 8: wie geteilt

Kein neuer Schritt, keine neue Nummer. Die Teilung läuft innerhalb von S8 über die Dateien, weil beide Anteile Rust sind und derselbe Ausführende sie schreibt; ein zweiter Schritt wäre nur nötig gewesen, wenn `coder` und `ontocoder` sich den Schritt hätten teilen müssen, wie bei S4 und S4b.

- **`crates/krk-ui/src/appkit/bildtakt.rs`** (neu) hält beide Berührungen mit AppKit hinter je einer sicheren Hülle. `Zeichenende` umschließt den `CADisplayLink` auf der Inhaltsansicht samt dem `define_class!`-Ziel, das den Rückruf entgegennimmt. `bildwiederholrate` schlägt über `NSWindow.screen()` auf `maximumFramesPerSecond` nach und liefert `None`, wenn das Fenster auf keinem Bildschirm steht, damit der Aufrufer nach der Regel aus S21 abbricht statt auf den Hauptbildschirm auszuweichen.
- **`crates/krk-ui/src/messmodus.rs`** behält den Ablauf der Messung, die zwanzig Wiederholungen, das 95. Perzentil und den Bericht. Über die Grenze gehen zwei gewöhnliche Rust-Werte: die Rate als Zahl und die Zeitpunkte der Zeichenenden. Die Zusage ist als prüfbarer Satz formuliert: in `messmodus.rs` steht keine `use objc2`-Zeile.
- Dazu in der Dateiliste: `appkit/mod.rs` als einbindende Datei und `appkit/anwendung.rs`, weil der Messmodus dort eingerichtet wird, wo seit S7 auch der Tastenabgriff eingerichtet wird.

Das Abnahmekriterium und die fünf Zahlen des Gates sind unverändert: L1 ≤ 16 ms, L2 ≤ 100 ms, L3 ≤ 400 ms warm, L4 ≤ 1000 ms, erste Bildschirmseite bei 100.000 Einträgen ≤ 100 ms.

## Die Durchsicht: sechs Verstöße statt der vermuteten vier

Betroffen und nachgezogen:

| Schritt | Was falsch lag | Wohin es jetzt gehört |
|---|---|---|
| S8 | `CADisplayLink`, `NSScreen` in `messmodus.rs` | `appkit/bildtakt.rs` |
| S13 | das Blatt der Pfadeingabe in `kommandos/` | `appkit/blaetter/{mod.rs,pfadeingabe.rs}` |
| S15 | der Papierkorb-Aufruf hatte **gar keine** Datei | `appkit/papierkorb.rs` |
| S16 | vier Blätter in `crates/krk-ui/src/blaetter/` | `appkit/blaetter/` |
| S17 | das Umbenennungsblatt ebenda | `appkit/blaetter/stapelumbenennen.rs` |
| S21 | synthetische Tastenereignisse in `messmodus.rs` | `appkit/ereignisse.rs` (erweitert) |

Zwei davon nannte der Defekt nicht: S15 und S21. S15 ist der interessantere Fund, weil er kein Verschieben war, sondern eine Lücke: die `Änderungen` sagten seit jeher, der Aufruf liege "in `krk-ui/src/appkit/`", aber keine Dateiliste nannte die Datei, und die von `krk-core` injizierte Schnittstelle wäre damit ohne Implementierung geblieben.

Bei S12, S18 und S19 stand die AppKit-Arbeit ohne Zuordnung zwischen zwei Dateien. Die Dateilisten waren nicht falsch, aber sie ließen offen, ob etwa `NSFileManager.mountedVolumeURLs…` im Leistenmodell oder in der Leiste landet. Die Grenze ist dort jetzt ausgeschrieben, und S12 und S13 haben `appkit/tabelle.rs` als erweiterte Datei bekommen, weil Auswahl und Bildlaufposition AppKit-Aufrufe sind.

Sauber und unverändert: S1, S6, S7, S14, S20, S22, S23.

**Gegenrichtung: nichts gefunden.** Jede der zwölf Dateien, die vor dem Nachzug neben der `mod.rs` unter `src/appkit/` standen, trägt ein AppKit-Objekt oder eine C-Bindung. Ein Randfall ist die FSEvents-Bindung aus S14: sie ist kein AppKit, sondern eine C-Schnittstelle aus CoreServices, braucht aber denselben unsicheren Fremdaufruf. Sie liegt richtig, und `## Aufbau` schreibt die zweite Aufgabe des Moduls jetzt aus, statt es nur als "Brücke zu AppKit" zu führen.

## Der Befund, der über den Defekt hinausgeht

Die Nachprüfung an den `objc2`-Quellen hat eine Annahme des Defekts widerlegt und dabei etwas Größeres freigelegt. Der Defekt nennt den Nachschlag auf `NSScreen` als zweiten Grund, aus dem S8 nicht übersetzt. Das trifft nicht zu: `NSWindow.screen` und `NSScreen.maximumFramesPerSecond` sind beide `pub fn`.

Daraus folgt, dass `#![deny(unsafe_code)]` die Grenze nur zur Hälfte trägt. Von den sechs gefundenen Verstößen hätten drei den Bau abgebrochen (S8, S16, S17) und drei nicht (S13, S18, S21). Der Defekt zu S8 war also nur deshalb sichtbar, weil `CADisplayLink` zufällig auf der unsicheren Seite liegt. Die Zusage aus `## Aufbau`, jeder AppKit-Aufruf liege hinter einer Hülle, ist damit eine Planzusage und keine Übersetzerzusage.

Gemeldet als `issues/260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md`, mit einem Vorschlag für eine Prüfvorschrift in derselben Form wie die drei vorhandenen Attributprüfungen aus S2, S6 und S15. Der `planner` hat sie **nicht** selbst eingetragen: sie gehört an das Abnahmekriterium von S6, und S6 ist abgenommen. Ein abgenommenes Kriterium nachträglich zu verschärfen ist eine Nutzerentscheidung.

## Geänderte Dateien

- `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` — ein Nachzugsvermerk im Kopf, eine neue Regel im Kopf von `## Implementierungsschritte`, zwei Absätze in `## Aufbau`, acht Schritte (S8, S12, S13, S15, S16, S17, S18, S19, S21), eine Zeile der Risikotabelle, zwei Einträge im Abschnitt `## Angelegte Defekte und Entscheidungen`.
- `issues/260803-1345_c_dateiliste-von-s8-legt-objc2-code-ausserhalb-von-appkit-ab.md` — `Resolved:`-Zeile, Marker von offen auf geschlossen.
- `issues/260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md` — neu.

Die Diagramme sind unberührt: es kam kein Schritt und keine Kante dazu, der Abhängigkeitsgraph steht unverändert bei 24 Knoten und 34 Kanten, und die Diagramm-Selbstprüfung am Ende des Plans bleibt gültig.
