# Schritt 11: Die Zeile lässt sich nach rechts blättern

**Date:** 2026-08-12
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/planning/260812-1145_p_teilen-ordnersprung-ablage-sichern-vorschau-rendern.md`, Schritt 11
**Verification:** `cargo build --workspace` — exit 0; `cargo fmt --all --check` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo test --workspace` — exit 0; Probenzahl im Binärziel `krk` vorher 445, nachher 445

---

## Was gebaut wurde

Die eine Statuszeile aus Schritt 10 schneidet eine lange Meldung nicht mehr ab,
sondern lässt sich nach rechts blättern (C5.10). Das Textfeld sitzt dafür als
Dokumentansicht in einer `NSScrollView`, und `Statuszeile::sicht` gibt seither
die Rolle heraus und nicht mehr das Feld.

```
vorher                          nachher
──────                          ───────
Statuszeile                     Statuszeile
  feld: NSTextField ───> sicht()  rolle: NSScrollView ───> sicht()
                                    └─ documentView
                                         feld: NSTextField
```

Die Signaturen nach außen bleiben unverändert — `bauen`, `sicht`, `zeigen` —,
und deshalb ist `anwendung.rs` nicht angefasst: der Anwendungsdelegierte hängt
weiter dieselbe `&NSView` ein und ruft weiter dasselbe `zeigen`.

## Die drei Handgriffe

**Der Aufbau macht die Rolle unsichtbar, solange nichts zu blättern ist.**
`setDrawsBackground(false)` und `setBorderType(NSBorderType::NoBorder)` nehmen
ihr jedes eigene Bild, `setHasVerticalScroller(false)` den senkrechten Balken —
die Zeile ist einzeilig —, und `setAutohidesScrollers(true)` blendet den
waagerechten aus, sobald der Text hineinpasst. Vor der ersten zu langen Meldung
sieht der Nutzer damit genau das, was vor Schritt 11 dastand.

**Jeder neue Text zieht die Breite der Dokumentansicht nach**
(`Statuszeile::breite_nachziehen`). Ohne diesen Schritt gäbe es nichts zu
blättern: eine Dokumentansicht in der Breite der Rolle hat keinen Überhang, und
`NSScrollView` blendete den Balken aus, während der Text abgeschnitten dasteht.
`sizeToFit` misst am Feld, was der Text braucht; die neue Breite ist das Größere
von Textbreite und Sichtbreite, damit die Zeile bei kurzen Meldungen nicht
schmaler wird als ihre Rolle. **Die Höhe kommt nicht aus `sizeToFit`, sondern
bleibt `HOEHE`** — die Schrifthöhe der kleinen Systemschrift liegt darunter, und
ein Feld in dieser Höhe säße in der 18 Punkte hohen Rolle nicht dort, wo es vor
Schritt 11 saß.

**Nach jedem Text steht die Zeile an ihrem Anfang**
(`Statuszeile::an_den_anfang`). Eine Meldung, die in der Mitte anfängt, weil die
vorige weiter rechts gelesen wurde, wäre keine Meldung. `scrollToPoint:` setzt
den Ausschnitt, `reflectScrolledClipView:` bringt den Rollbalken auf denselben
Stand; ohne den zweiten Ruf zeigte er weiter die alte Stelle. Die Reihenfolge
ist gebunden: erst steht die Breite fest, dann lässt sich sagen, wo der Anfang
liegt.

## Der Einzug wandert von der Fläche an die Rolle

`fensterinhalt` setzt den Rahmen der Zeile weiterhin bei `statuszeile::EINZUG`
und mit `ANFANGSGROESSE.width - EINZUG`. Eingerückt wird damit seit Schritt 11
die Bildlaufansicht statt des Textfeldes; weil das Feld darin bei null beginnt,
fängt der Text weiterhin dieselben sechs Punkte vom Fensterrand entfernt an. Die
Autogrößen der drei Ansichten sind unverändert, und die Höhenrechnung an
`MINDESTGROESSE` ist nicht angefasst.

`fensterinhalt` nimmt `&NSView` und wusste nie, was es einhängt. Genau das trägt
diesen Schritt: die Datei kennt keine `NSScrollView` und braucht sie im
Untergrenzen-Abschnitt ihres Modulkopfes deshalb nicht.

## Keine neue Probe, und warum nicht

Die Probenzahl im Binärziel `krk` steht vorher wie nachher bei 445. Beide
Kriterien dieses Schrittes, C5.10 und C5.11, tragen im Plan **(Bündel)**; die
reinen Funktionen der Datei — `zeile`, `zeilentext`, `Rang::art` — sind nicht
angefasst, und ihre 19 Proben treffen unverändert dieselben Aussagen. Eine Probe
an einer Instanz hinge am Hauptfaden, den `libtest` nicht hergibt; das ist der
Zustand, den `issues/260810-1001_*` beschreibt, und dieser Schritt vermehrt ihn
nicht.

## Was C5.11 jetzt trägt, und was nicht

**Die Grundlage der Zusage hat sich geändert, und die offene Frage ist damit
größer und nicht kleiner geworden.** Der `coder` von Schritt 10 hat am Baum
festgestellt, dass `setRefusesFirstResponder` genau einmal vorkommt
(`bereichsleiste.rs:478`) und die Statuszeile es nicht braucht, weil
`labelWithString:` ein nicht auswählbares Textfeld baut. Das gilt für das Feld
weiter. Seit Schritt 11 steht aber eine zweite Ansicht dazwischen:

- `NSScrollView` erbt `acceptsFirstResponder` von `NSView` und antwortet von
  sich aus mit `NO`. Die Rolle selbst nimmt den Rang also nicht an.
- Ihre Rollbalken sind `NSScroller`, also `NSControl` — von derselben Art wie
  die Schalter der Bereichsleiste, für die die Runde 5 ihre Frage offen gelassen
  hat. Sichtbar sind sie nur, solange der Text breiter ist als die Zeile.

Die offene Frage der Runde 5 — ob `refusesFirstResponder` und eine nicht
auswählbare Bildlaufansicht den Ersthelferrang bei eingeschalteter
vollständiger Tastaturbedienung fernhalten — **betrifft damit eine Ansicht
mehr als vor diesem Schritt**, und sie ist hier nicht beantwortet worden. Der
Modulkopf von `statuszeile.rs` sagt das aus und behauptet nichts darüber
hinaus. Abzunehmen ist C5.11 am laufenden Bündel, mit eingeschalteter
vollständiger Tastaturbedienung und einer Meldung, die lang genug für einen
sichtbaren Rollbalken ist.

## Was am Bündel zu sehen ist

- Eine Meldung, die breiter ist als das Fenster, lässt sich mit Zweifingerstrich
  oder Rollbalken bis zum Ende lesen (C5.10).
- Der Rollbalken erscheint nur, wenn es etwas zu blättern gibt.
- Jede neue Meldung steht an ihrem Anfang, auch wenn die vorige weiter rechts
  gelesen wurde.
- **Der Preis:** mit dem Zeiger über den achtzehn Punkten am Fensterfuß bewegt
  ein Zweifingerstrich die Zeile und nicht die Liste darüber.
- Der Fokusrahmen aus C9 bleibt, wo er stand (C5.11, zu prüfen).

## Geänderte Dateien

- `crates/krk-ui/src/appkit/statuszeile.rs` — die Rolle um das Feld, die beiden
  neuen Handgriffe, der Modulkopf mit dem Abschnitt über Blättern statt Kürzen,
  die veränderte Grundlage von C5.11 und die am SDK gelesenen Untergrenzen.
- `crates/krk-ui/src/appkit/fenster.rs` — zwei Doc-Ergänzungen an
  `fensterinhalt` und die Skizze im Modulkopf. Kein Code.
