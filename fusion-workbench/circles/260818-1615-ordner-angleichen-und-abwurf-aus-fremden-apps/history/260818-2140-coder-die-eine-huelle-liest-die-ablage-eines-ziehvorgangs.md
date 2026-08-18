# Coder — Schritt 6: Die eine Hülle liest die Ablage eines Ziehvorgangs

**Datum:** 260818-2140
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 6
**Baumstand beim Beginn:** `79f52af`. Der Arbeitsbaum war nicht sauber und ist es während des Laufs nicht geworden: zwei weitere Agenten haben gleichzeitig an `appkit/anwendung.rs` und am neuen `kommandos/abwurfregel.rs` gearbeitet.

## Was der Auftrag war

Der erste Schritt von Bündel B. `crates/krk-ui/src/appkit/zwischenablage.rs` ist die **eine** Hülle um `NSPasteboard`; sie bekommt eine vierte Funktion, die die Ablage eines Ziehvorgangs nach Dateien auf dem Datenträger fragt. Der Auftrag war ausdrücklich eine Erweiterung und keine zweite Hülle daneben.

## Was geändert wurde

Genau eine Datei: `crates/krk-ui/src/appkit/zwischenablage.rs`.

**Die Funktion.** `pub fn dateiverweise(ablage: &NSPasteboard) -> Vec<PathBuf>` über `readObjectsForClasses:options:` mit der Klassenliste `[NSURL::class()]` und der einen Vorgabe `{NSPasteboardURLReadingFileURLsOnlyKey: NSNumber::new_bool(true)}`. Jeder gelieferte Eintrag wird auf `NSURL` heruntergecastet, gibt seinen `path` her und wird zum `PathBuf`; ein Eintrag ohne Pfad fällt still weg. Eine leere Ablage und eine Ablage ohne Dateiverweise liefern beide einen leeren Vektor, und das ist die Antwort und kein Fehler — genau die Auskunft, die C7 braucht, um eine Zusagedatei abzuweisen, ohne je eine einzuordnen.

**Der Vermerk mit Ablaufdatum.** Bis Schritt 8 den Rufer setzt, trägt die Funktion `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]` in der Form, die `kommandos/rueckschritt.rs` in `2ff4b5a` eingeführt hat. `expect` und nicht `allow`, damit Schritt 8 die Zeile entfernen **muss**: mit dem Rufer wird die Erwartung unerfüllt und `-D warnings` hält an.

**Der Modulkopf.** Das Schaubild trennt jetzt die beiden Ablagen — die des Nutzers über `generalPasteboard`, die des Ziehvorgangs über `draggingPasteboard` — und ein neuer Abschnitt `# Seit der Runde 13 vier Fragen an zwei Ablagen` trägt die zwei Begründungen, die der Plan verlangt hat:

- **`lesen()` bekommt keinen Parameter.** Es beantwortet eine Frage an einen Gegenstand, und sein einziger Gegenstand steht deshalb im Rumpf und nicht in der Signatur. `dateiverweise` nimmt seine Ablage entgegen, weil AppKit sie ihm reicht und es sie nicht beschaffen kann.
- **`stringForType:` trägt hier nicht.** Es liefert eine Zeichenkette je Sorte. Ein Abwurf hat mehrere Einträge, und mehrere Dateiverweise als eine Zeichenkette zurückzubekommen hieße, sie hinterher an einem Trennzeichen zu schneiden, das in einem Dateinamen vorkommen darf.

## Die Verfügbarkeitsangaben, am SDK gelesen — und eine Berichtigung

Gelesen in `/Applications/Xcode.app/…/MacOSX.sdk/…/AppKit.framework/Headers/NSPasteboard.h`.

| Was | Ab | Fundstelle | Der Plan sagte |
|---|---|---|---|
| `readObjectsForClasses:options:` | 10.6 | `NSPasteboard.h:190` | 10.6, `:190` — stimmt |
| `NSPasteboardURLReadingFileURLsOnlyKey` | **10.6** | `NSPasteboard.h:146` | 10.13, `:39` — **falsch** |
| `NSPasteboardTypeFileURL` | 10.13 | `NSPasteboard.h:39` | 10.13, `:39` — stimmt |
| `pasteboardWithName:` | 10.0 | `NSPasteboard.h:160` | nicht genannt |

Der Plan hat die zwei Namen an einer Fundstelle zusammengezogen; `:39` trägt allein `NSPasteboardTypeFileURL`, und der Vorgabeschlüssel steht hundert Zeilen weiter unten mit `API_AVAILABLE(macos(10.6))`. Übernommen ist der SDK-Stand, nicht die Zahl des Plans.

**Dabei ist eine falsche Zahl im Baum aufgefallen und berichtigt worden.** Der Modulkopf zählte `NSPasteboardTypeFileURL` bis heute zu den Konstanten „seit 10.6"; das SDK schreibt an `:39` `API_AVAILABLE(macos(10.13))`. Die Angabe steht jetzt richtig und mit dem Vermerk, dass sie berichtigt wurde. An der Untergrenze des Bündels ändert das nichts — 10.13 liegt weit unter 15.0 —, aber die Angabe ist die eine Gegenmaßnahme dieses Projekts gegen eine Bindung, die keine Verfügbarkeitsangaben mitführt, und eine falsche Zahl darin ist wertlos.

## Die zwei Proben, und warum sie `generalPasteboard` nicht anfassen

`zwei_dateiverweise_kommen_als_zwei_pfade_zurueck` legt zwei Dateien in einem `Pruefordner` an, schreibt sie als Datei-`NSURL` in eine eigene Ablage und liest beide Pfade in der Reihenfolge der Ablage zurück. `eine_leere_ablage_liefert_einen_leeren_vektor` prüft die andere Seite.

Angefasst wird `generalPasteboard` dabei nicht, aus demselben Grund, aus dem `text_schreiben` keine Probe trägt: eine Probe, die sie beschriebe, würfe bei jedem `make check` weg, was der Entwickler gerade kopiert hat.

**Der Name der Probenablage ist fest und nicht eindeutig, und das ist eine Abweichung mit Grund.** `pasteboardWithUniqueName` stünde daneben, aber `objc2-app-kit 0.3.2` bindet `releaseGlobally` nicht — nachgesehen im erzeugten `NSPasteboard.rs` der Kiste, dort steht kein einziger Treffer. Eine eindeutig benannte Ablage bliebe damit beim Pasteboard-Server stehen, ohne dass die Probe sie wieder abgeben könnte, und zwar je Lauf eine weitere. Ein fester Name je Probe hält die Zahl bei zwei; `clearContents` macht den Anfangszustand jedes Laufs gleich.

## Was bewusst nicht getan wurde

**Kein `#[must_use]`.** Der Plan setzt es für die Funktionen der Schritte 7 und 8 ausdrücklich und für diese ausdrücklich nicht, und die drei vorhandenen Funktionen dieser Datei tragen es ebenfalls nicht. Die Projektregel zielt auf einen Wert, dessen stilles Fallenlassen unbemerkt bliebe; ein Aufruf einer reinen Abfrage ohne Nebenwirkung, deren Ergebnis niemand nimmt, ist dagegen offensichtlich sinnlos. Wer das anders sieht, setzt es in Schritt 8, wenn der Rufer dasteht.

**Keine Zahl in der Prosa.** Der Kopf sagt „jeder seiner Aufrufer" statt einer Zählung der Rufer von `lesen`. Eine Zahl an dieser Stelle veraltet, und `CLAUDE.md` führt für genau diesen Fehler eigene Befunde.

## Prüfung

`make check` — Abschluss `alle vier gruen`, Exit 0. Beide neuen Proben laufen darin mit:

```
test appkit::zwischenablage::proben::eine_leere_ablage_liefert_einen_leeren_vektor ... ok
test appkit::zwischenablage::proben::zwei_dateiverweise_kommen_als_zwei_pfade_zurueck ... ok
```

Vor dem Lauf geprüft: weder `/tmp` noch `$TMPDIR` trug eine `krk-messplan-*.toml`.

**Zwei Zwischenläufe standen rot, und beide Male an fremdem Code.** Ein `cargo fmt --all --check` fiel über `crates/krk-ui/src/kommandos/abwurfregel.rs` (Schritt 7, anderer Agent), ein `cargo clippy` über ein `cannot find value 'ziel'` in `appkit/anwendung.rs` (Schritt 9, anderer Agent). Beide waren Zwischenstände paralleler Arbeit und beim nächsten Lauf weg; angefasst wurde keine der beiden Dateien.

## Was als Nächstes ansteht

Schritt 8 (`appkit/abwurf.rs`) setzt den Rufer und **muss** dabei den `expect(dead_code)`-Vermerk an `dateiverweise` entfernen, sonst hält `-D warnings` den Bau an. Genau dafür steht dort `expect` und nicht `allow`.
