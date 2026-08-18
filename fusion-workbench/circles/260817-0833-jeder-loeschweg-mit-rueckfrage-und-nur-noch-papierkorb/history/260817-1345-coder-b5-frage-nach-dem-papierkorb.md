# B5 — die Frage nach dem Papierkorb, mit selbst geprüften Untergrenzen

**Datum:** 260817-1345
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Schritt 5
(Bündel B, zweiter Schritt)
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C4
**Datensatz:** `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`
(bleibt `_a_`; er wandert nach Plan in Schritt 16)
**Baumstand vorher:** `4b50cc1`

---

## Was umgesetzt ist

Eine Datei: `crates/krk-ui/src/appkit/papierkorb.rs`.

- `#[must_use] pub fn fuehrt_einen_papierkorb(ordner: &Path) -> Befund` über
  `NSFileManager::defaultManager().URLForDirectory_inDomain_appropriateForURL_create_error(
  NSSearchPathDirectory::TrashDirectory, NSSearchPathDomainMask::UserDomainMask, Some(&url),
  false)`. Erfolg → `Befund::Ja`, Fehler → `Befund::Nein`, Pfad ohne gültiges UTF-8 →
  `Befund::Unentschieden`. Die Fallunterscheidung über das `Result` steht als `match` mit
  zwei ausgeschriebenen Zweigen, nicht als `is_ok()`.
- Der `#[must_use]`-Grund steht ausgeschrieben: *„der Befund ist die Erlaubnis zu loeschen;
  fallengelassen loescht der Aufrufer auf einem Ziel, das keinen Papierkorb fuehrt"*.
- `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]` in der Bauform aus
  `kommandos/rueckschritt.rs`. **Schritt 6 muss diese vier Zeilen entfernen**, weil die
  Erwartung mit dem Aufrufer unerfüllt wird und `-D warnings` den Bau dann anhält. Nötig
  war sie, weil `krk-ui` ein Binärziel ist und `pub` dort keine Verwendung darstellt.
- `use krk_core::verzeichnis::Befund;` und drei neue Namen aus `objc2_foundation`
  (`NSSearchPathDirectory`, `NSSearchPathDomainMask` — der dritte, `NSFileManager`, stand
  schon). Keine Änderung an `Cargo.toml`: `objc2-foundation` läuft mit Vorgabemerkmalen,
  und die Bindung verlangt `NSError`, `NSPathUtilities` und `NSURL`, die damit stehen.
- Der Modulkopf ist auf den erweiterten Gegenstand gezogen — die **eine** Hülle um den
  Papierkorb des Systems, Räumen und Vorprüfung. Das Bild führt jetzt zwei Wege: die
  Abhängigkeitsumkehr des `Systempapierkorb` bleibt als oberer, der gewöhnliche Weg von
  `appkit::anwendung` zur Vorprüfung kommt als unterer dazu.
- `Systempapierkorb` und `in_den_papierkorb` sind unverändert, Zeile für Zeile.

## Die drei Untergrenzen, selbst gelesen

Der Auftrag verlangt die Prüfung am SDK und nicht die Übernahme aus dem Plan. SDK-Pfad aus
`xcrun --show-sdk-path`:
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk`,
Header unter `System/Library/Frameworks/Foundation.framework/Headers/`.

| Berührung | gelesen in | Zeile | Angabe im Kopf |
|---|---|---|---|
| `URLForDirectory:inDomain:appropriateForURL:create:error:` | `NSFileManager.h` | 127 | `API_AVAILABLE(macos(10.6), ios(4.0), watchos(2.0), tvos(9.0))` |
| `NSTrashDirectory` | `NSPathUtilities.h` | 88 | `API_AVAILABLE(macos(10.8), ios(11.0)) API_UNAVAILABLE(watchos, tvos)` |
| `NSUserDomainMask` | `NSPathUtilities.h` | 93 | **keine** — damit seit 10.0 |

Alle drei Zahlen des Plans bestätigen sich. **Zwei Angaben stehen zusätzlich im Modulkopf**,
weil die Datei die Typnamen selbst schreibt und nicht nur ihre Werte: die Aufzählungen
`NSSearchPathDirectory` (`NSPathUtilities.h:61`) und `NSSearchPathDomainMask`
(`NSPathUtilities.h:92`) tragen an ihren `typedef`-Zeilen ebenfalls keine Angabe und stehen
damit seit 10.0.

**Zu `NSUserDomainMask` gehört ein Satz, der die fehlende Angabe als Angabe liest**, sonst
liest der nächste Prüfer sie als Lücke: es ist der einzige der fünf Werte von
`NSSearchPathDomainMask`, der kein `API_AVAILABLE` trägt, und die vier anderen tragen auch
keins — die ganze Aufzählung ist von Anfang an da. Das steht im Modulkopf, damit die Zahl
beim nächsten Durchgang nicht neu erhoben werden muss.

Die höchste Zahl ist 10.8 und liegt unter der Untergrenze 15.0 aus `.cargo/config.toml`.
Keine Verfügbarkeitsprüfung zur Laufzeit ist nötig.

## Die Polarität steht an der Funktion

Der Rückgabewert liegt auf der **zweiten** Polarität aus dem Modulkopf von
`krk_core::verzeichnis::Befund`: `Befund::Ja` ist hier die **Erlaubnis** und nicht der
Warngrund, `Unentschieden` gehört zu `Nein`. `ist_warnwuerdig` ist damit das falsche
Werkzeug, und das steht als eigener Abschnitt im Modulkopf und ein zweites Mal am
Doc-Kommentar der Funktion, wo der Leser von Schritt 6 zuerst hinsieht. Verwiesen ist auf
den Kern und nicht abgeschrieben, damit die Begründung nicht an zwei Stellen auseinanderläuft.

## Drei Sachen, die der Auftrag nicht nennt und die im Code begründet stehen

**Warum die Vorprüfung keine Methode von `trait Papierkorb` ist.** Sie wird gefragt, bevor
es einen Auftrag gibt: ihr Aufrufer ist der Kommandoweg vor der Rückfrage, und die
Operationsmaschine im Kern erreicht diesen Punkt nie — wenn sie läuft, ist die Entscheidung
zu löschen gefallen. Eine Methode an der Schnittstelle hätte einen Aufrufer gebraucht, den
es nicht gibt.

**Warum `create:` auf `false` steht.** Eine Prüfung, die den Papierkorb im Zweifel anlegt,
verändert das Ziel, über das sie gleich urteilt, und meldete danach ein `Ja` über einen
Datenträger, der einen Augenblick vorher keinen führte. Der Nutzer bekäme eine Rückfrage
statt der Meldung, und sein Eintrag läge in einem Papierkorb, den KRK ihm ungefragt
eingerichtet hat.

**Warum der Ordner aufgelöst hereinkommt.** Die Funktion ruft `canonicalize` nicht selbst;
eine Verknüpfung würde sonst den Papierkorb ihres eigenen Ortes melden statt den ihres
Ziels. Das Auflösen liegt beim Aufrufer, wie Schritt 6 es vorsieht.

## Ein Satz des alten Modulkopfs gilt für die neue Hälfte nicht, und das steht dort

Der Kopf sagte und sagt: *„Der Hauptfaden bleibt damit auch beim Loeschen frei, was L9
verlangt."* Für den `Systempapierkorb` stimmt das unverändert — er läuft auf dem Arbeitsfaden
der Operationsmaschine. **Für die Vorprüfung stimmt er nicht**, denn sie läuft auf dem
Hauptfaden, weil ihr Ausgang entscheidet, ob das Blatt überhaupt erscheint. Ohne einen Absatz
dazu läse der nächste Leser die Zusage als für beide Hälften geltend.

Nachgesehen statt angenommen: L9 misst *„Tastendruck waehrend laufender Kopie, bis Ende des
Zeichendurchgangs"* (`krk-bench/src/messen.rs:1122`). Die Spanne der Vorprüfung liegt
zwischen Tastendruck und Blatt, und **keine der zehn Zusagen aus C8 vermisst sie**. Der Rest
ist im Modulkopf benannt und nicht weggerechnet: hängt der Datenträger unter dem angezeigten
Ordner, verzögert sich das Blatt um die Antwort des Systems. Kein Defekt und keine offene
Frage — der Ort der Prüfung ist die Festlegung von Schritt 6 des Plans, und ein Nebenfaden
machte aus einer Entscheidung **vor** dem Blatt eine nach dem Blatt.

## Drei Proben, und der negative Ausgang ist keine Nutzerarbeit

Der Auftrag nennt zwei Proben und stellt in Aussicht, dass der negative Ausgang ohne Fenster
nicht messbar sei. **Er ist messbar**, und deshalb sind es drei geworden. Alle drei laufen
ohne Fenster und ohne Hauptfaden: `NSFileManager` ist von jedem Faden aus zu rufen, und die
Bauform mit `MainThreadMarker::new_unchecked` aus `appkit/editor.rs` ist nicht vermehrt.

```
appkit::papierkorb::tests::das_benutzerverzeichnis_fuehrt_einen_papierkorb ... ok
appkit::papierkorb::tests::ein_datentraeger_ohne_papierkorb_wird_erkannt ... ok
appkit::papierkorb::tests::ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden ... ok
```

**`ein_datentraeger_ohne_papierkorb_wird_erkannt` prüft `/dev` und erwartet `Befund::Nein`.**
`/dev` ist auf jedem macOS ein eigener Einhängepunkt mit einem Dateisystem für Gerätedateien
und kann keinen Papierkorb führen; kein Recht und kein Aufbau ist nötig. Ohne diese Probe
wäre die Funktion mit einem festen `Befund::Ja` grün und die Zusage von C4 ohne Beleg.

Gewählt ist ein Ort, der die Antwort aus seinem Wesen bezieht, und ausdrücklich **kein
fehlender Pfad**: der liefert dieselbe Antwort, sagt damit aber nichts über den Papierkorb
eines Datenträgers, und er kommt hier ohnehin nicht an — den löst der Aufrufer vorher auf und
zählt sein Scheitern als `Unentschieden`.

Erhoben ist die Unterscheidung an fünf Orten, mit einer Wegwerfprobe, die anschließend wieder
entfernt ist:

```
/                    -> Ja
/System/Volumes/VM   -> Nein
/dev                 -> Nein
/gibt-es-nicht-xyz   -> Nein
/private/var/vm      -> Ja
```

Die Prüfung unterscheidet also wirklich, und sie unterscheidet **je Datenträger**:
`/System/Volumes/VM` ist der eigene VM-Datenträger und antwortet `Nein`, `/private/var/vm`
liegt auf dem Datenträger mit den Nutzerdaten und antwortet `Ja`. Genau das ist die Aussage,
auf der C4 aufsetzt.

`ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden` baut den Pfad über
`OsStr::from_bytes(b"/tmp/krk-papierkorb-\xffkrumm")` — `0xff` ist in keiner UTF-8-Folge
zulässig — und prüft mit einem `assert!` vorweg, dass der Pfad wirklich kein gültiges UTF-8
ist, damit die Probe nicht still das Falsche messen kann. Kein Ordner wird angelegt: die
Funktion greift nicht auf das Dateisystem zu.

## Abnahme

`make check` — **exit 0**, alle vier Kommandos grün: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`.

Zusätzlich `cargo doc -p krk-ui --no-deps --document-private-items`: 67 Warnungen, alle
vorbestehend, **keine in dieser Datei** (nachgeprüft mit einem `touch` auf die Datei und
einem `grep` auf `appkit/papierkorb.rs` in der Ausgabe). Die Doc-Verweise auf
`krk_core::verzeichnis::Befund`, `Befund::ist_warnwuerdig` und `crate::appkit::anwendung`
lösen auf.

## Grenzen eingehalten

Angefasst ist **eine** Datei, `crates/krk-ui/src/appkit/papierkorb.rs`. Kein Aufrufer
eingeführt — der kommt in Schritt 6, und der muss dort die vier Zeilen
`#[cfg_attr(not(test), expect(dead_code, …))]` entfernen. Kein Commit; das macht der
Orchestrator. Der Planschritt bleibt unverändert; das `[DONE]` setzt der Orchestrator.

---
**Addendum 260818-0201 (analyst).** This log was added by commit `e2760cd`, author time
`260817-1341`. Its filename timestamp runs **4 minutes ahead** of that commit, which no clock produces: the
file cannot have been named after the moment it was committed. For placing this session against the
commit log, the author time in this line is what binds, not the filename.

The filename itself stays as it is. It is a pointer, and other records cite it; renaming it would
buy a correct timestamp at the price of dead citations. The finding is
`issues/260817-1807_*_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md`,
the rule `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## Timestamps`.
