# Coder — Schritt 8: Was AppKit über einen Ziehvorgang und einen Ordner sagt

**Datum:** 260818-2232
**Status:** Complete
**Modus:** Dispatch durch den Nutzer
**Plan:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 8
**Spec:** `shared/planning/260818-1510_o_spec-verzeichnis-angleichen-und-abwurf-aus-fremden-apps.md`, C5, C6 und C7
**Baumstand beim Beginn:** `07347b8`. Der Arbeitsbaum war **nicht** sauber: ein
weiterer Agent arbeitete zeitgleich in `crates/krk-ui/src/appkit/anwendung.rs`
(Schritt 9). Die Datei blieb unangetastet.

## Was der Auftrag war

Das Gegenstück zu `kommandos/abwurfregel.rs` aus Schritt 7: ein neues Modul
`crates/krk-ui/src/appkit/abwurf.rs` mit den vier Funktionen, die der reinen
Regel ihre Tatsachen beschaffen, dazu die Anmeldung in `appkit/mod.rs`.

## Was entstanden ist

**`crates/krk-ui/src/appkit/abwurf.rs`** (neu, 502 Zeilen) mit vier Funktionen:

| Funktion | Was sie tut |
|---|---|
| `sorten() -> Retained<NSArray<NSPasteboardType>>` | `NSPasteboardTypeFileURL` plus `NSFilePromiseReceiver::readableDraggedTypes()` |
| `beschreibbarkeit(&Path) -> Schreibrecht` | `NSURLIsWritableKey` über `resourceValuesForKeys_error` |
| `angebot(&ProtocolObject<dyn NSDraggingInfo>) -> (bool, bool)` | `draggingSourceOperationMask`, zweimal `contains` |
| `zeiger(Abwurfurteil) -> NSDragOperation` | die Rückrichtung |

`beschreibbarkeit`, `zeiger` und `sorten` tragen `#[must_use]` mit
ausgeschriebenem Grund; `angebot` ebenfalls.

**`crates/krk-ui/src/appkit/mod.rs`** — `mod abwurf;`, ein Absatz im
Modulverzeichnis des Kopfes, und „Neunundzwanzig Module" auf „Dreissig".

**`crates/krk-ui/src/kommandos/abwurfregel.rs`** — die `expect(dead_code)`-Zeile
an `Schreibrecht` ist gefallen, der Doc-Kommentar darüber berichtigt. Siehe
unten, der Punkt hat einen Befund.

## Die Untergrenzen, jede am SDK gelesen

Nicht aus dem Plan übernommen. Gelesen am 260818 unter
`$(xcrun --show-sdk-path)/System/Library/Frameworks/`, in
`AppKit.framework/Headers/` und `Foundation.framework/Headers/`. Die Spalte
„Kopfzeile" ist die Zeile, an der die Angabe steht.

| Angesprochen | Kopfzeile | Angabe dort | Seit |
|---|---|---|---|
| `NSDragOperation` (typedef) | `NSDragging.h:25` | keine | 10.0 |
| `NSDragOperationNone` | `NSDragging.h:26` | keine | 10.0 |
| `NSDragOperationCopy` | `NSDragging.h:27` | keine | 10.0 |
| `NSDragOperationMove` | `NSDragging.h:31` | keine | 10.0 |
| `NSDraggingInfo` (Protokoll) | `NSDragging.h:69` | keine | 10.0 |
| `draggingSourceOperationMask` | `NSDragging.h:72` | keine | 10.0 |
| `NSPasteboardType` (typedef) | `NSPasteboard.h:23` | keine | 10.0 |
| `NSPasteboardTypeFileURL` | `NSPasteboard.h:39` | `API_AVAILABLE(macos(10.13))` | 10.13 |
| `NSFilePromiseReceiver` (Klasse) | `NSFilePromiseReceiver.h:19` | `API_AVAILABLE(macos(10.12))` | 10.12 |
| `readableDraggedTypes` | `NSFilePromiseReceiver.h:23` | keine eigene, trägt die der Klasse | 10.12 |
| `NSURLResourceKey` (typedef) | `NSURL.h:17` | keine | 10.0 |
| `+fileURLWithPath:` (Form ohne weitere Argumente) | `NSURL.h:52` | keine | 10.0 |
| `-resourceValuesForKeys:error:` | `NSURL.h:183` | `API_AVAILABLE(macos(10.6), …)` | 10.6 |
| `NSURLIsWritableKey` | `NSURL.h:247` | `API_AVAILABLE(macos(10.7), …)` | 10.7 |
| `-objectForKey:` | `NSDictionary.h:17` | keine | 10.0 |
| `boolValue` | `NSValue.h:73` | keine | 10.0 |

Keine liegt über macOS 15. **Zwei Abweichungen gegenüber dem Plan**, beide zu
seinen Gunsten aufgelöst:

1. Der Plan nennt `NSDraggingInfo` und `NSDragOperation` zusammen unter
   `NSDragging.h:69-79`. Das ist die Kollaps-Schreibweise, gegen die der
   Modulkopf von `volumes.rs` ausdrücklich schreibt: `NSDragOperation` steht
   auf `:25`, das Protokoll auf `:69`. Beide stehen jetzt einzeln.
2. Der Plan nennt `NSPasteboardTypeFileURL` ohne Kopfzeile. Sie ist
   `NSPasteboard.h:39` und stimmt mit dem Spec überein.

**`registerForDraggedTypes:` steht nicht in diesem Kopf.** Es steht in
`NSView.h:488` ohne Angabe, also seit 10.0, und wird in Schritt 10 in
`tabelle.rs` angesprochen, nicht hier.

## Sechs Proben, alle grün

| Probe | Was sie misst |
|---|---|
| `die_liste_traegt_den_dateiverweis_und_die_zusagesorten` | C7: die angemeldete Liste ist nicht nur der Dateiverweis |
| `ein_frischer_ordner_ist_beschreibbar` | `Schreibrecht::Ja` |
| `ein_ordner_ohne_schreibrecht_meldet_nein` | `0o500` liefert ein gemessenes `Nein` |
| `ein_fehlender_ordner_bleibt_unbekannt` | `Unbekannt` und nicht `Nein` |
| `ein_pfad_ohne_gueltiges_utf8_bleibt_unbekannt` | dieselbe Rücknahme, die erste Zeile der Funktion |
| `jedes_urteil_hat_seinen_zeiger` | die Rückrichtung vollständig, die fünf Abweisungsgründe einzeln |

`angebot` trägt **keine** Probe: ein `NSDraggingInfo` lässt sich ohne
Ziehsitzung nicht bauen. Der Plan sieht das so vor.

Die Rechteprobe setzt die Rechte **vor** der Behauptung zurück und behauptet auf
einem vorher gemerkten Wert. Schlüge die Behauptung zuerst fehl, bliebe ein
Ordner mit `0o500` stehen, den `Pruefordner::drop` nicht mehr abräumen könnte.

## Am Gerät gemessen, weil der Plan es als Risiko führt

Der Plan führt als Risiko, dass `readableDraggedTypes()` eine leere Liste
liefert und eine Zusagedatei KRK deshalb nie erreicht. **Auf diesem Gerät
(Darwin 24.6) liefert sie drei Sorten**, `sorten()` trägt damit vier:

```
com.apple.NSFilePromiseItemMetaData
com.apple.pasteboard.promised-file-content-type
dyn.ah62d4rv4gu8yc6durvwwa3xmrvw1gkdusm1044pxqyuha2pxsvw0e55bsmwca7d3sbwu
```

Die Zahl steht als Aufzeichnung im Doc-Kommentar der Probe, nicht als
Behauptung: die Probe prüft, dass **jede** gelieferte Sorte in der Liste steht,
und nicht, wie viele es sind. Eine leere Liste ließe sie grün durchlaufen, und
das ist Absicht — die Zahl gehört dem System und darf sich ändern.

## Vier Entscheidungen, die der Plan offenließ

### 1. `sorten` nimmt **keinen** `MainThreadMarker`

Der Plan schreibt `sorten(mtm)`. Keine Zeile des Rumpfes braucht den Hauptfaden:
`NSPasteboardTypeFileURL` ist eine Konstante, und `NSFilePromiseReceiver` ist in
`objc2-app-kit 0.3.2` keine `MainThreadOnly`-Klasse, weshalb
`readableDraggedTypes` ohne Marke gebunden ist. Ein Parameter, den keine Zeile
verbraucht, müsste `_mtm` heißen; **diese Kiste führt heute keine einzige solche
Stelle** — alle 73 `mtm: MainThreadMarker` unter `src/appkit/` werden verbraucht,
keine heißt `_mtm` —, und eine erste zu eröffnen behauptete eine Bedingung, die die Bindung nicht
kennt.

**Für Schritt 10 heißt das:** der Aufruf lautet `abwurf::sorten()` und nicht
`abwurf::sorten(mtm)`. Der Übersetzer fängt es, falls der Plan wörtlich
befolgt wird. Der Grund steht am Doc-Kommentar der Funktion.

### 2. Der `expect(dead_code)`-Vermerk von `angebot` steht unbedingt

Drei der vier Funktionen tragen ihn unter `cfg_attr(not(test), …)` wie das
Vorbild aus Schritt 6 und 7, weil die Proben sie rufen und die Erwartung im
Probenbau sonst unerfüllt wäre. `angebot` hat auch im Probenbau keinen Aufrufer
und trägt ihn deshalb unbedingt. Der Modulkopf schreibt den Unterschied aus.

### 3. Die `expect(dead_code)`-Zeile an `Schreibrecht` ist gefallen — und der Übersetzer hat sie nicht eingefordert

Schritt 7 hat sie gesetzt und **Schritt 8 als ihr Ablaufdatum genannt**, weil
`beschreibbarkeit` die drei Werte baut. Sie ist gefallen. Der Doc-Kommentar der
Aufzählung, der sagte „wird hier gelesen und nirgends gebaut", ist berichtigt.

**Der Befund dazu gehört in den Bericht, weil er eine Zusage dieses Baums
einschränkt.** Gemessen, nicht angenommen:

- Mit der Zeile: `cargo clippy --workspace --all-targets -- -D warnings` grün.
- Ohne die Zeile: **ebenfalls grün.** Kein `dead_code`, keine unerfüllte
  Erwartung.

Die Zeile war also weder wirksam noch fällig — sie war **wirkungslos geworden,
ohne dass etwas es meldete.** Der Grund: ein `expect(dead_code)` an einer
Funktion macht sie für die Totlaufprüfung zu einer lebendigen Wurzel, und was
sie in ihrem Rumpf baut, gilt damit als gebaut, auch solange sie selbst noch
keinen Aufrufer hat. `beschreibbarkeit` steht unter genau so einem Vermerk.

Dass `unfulfilled_lint_expectations` in dieser Kiste überhaupt anschlägt, ist
eigens nachgeprüft: ein probeweise gesetztes `#[expect(unused_variables)]` an
`zeiger` wurde als „this lint expectation is unfulfilled" gemeldet. Der
Mechanismus greift, aber nicht für `dead_code` an einer Aufzählung, deren
Erbauer selbst unter einem Vermerk steht.

**Die Zusage aus dem Modulkopf von `rueckschritt.rs` bleibt für Funktionen
bestehen** — dort ist der Ablauf beim Setzen des echten Aufrufers in `1ac8842`
tatsächlich eingetreten. Für die drei Vermerke, die Schritt 10 fallen lässt, gilt
sie also weiter. Sie gilt **nicht** für einen Vermerk an einer Aufzählung, deren
einziger Erbauer selbst noch tot ist. Der Doc-Kommentar von `Schreibrecht`
schreibt das jetzt aus, damit die nächste Runde die Falle nicht ein zweites Mal
stellt.

### 4. `zwischenablage::dateiverweise` behält seinen Vermerk

Der Plan sagt unter Schritt 8: „Der `expect(dead_code)`-Vermerk aus Schritt 6
fällt hier, weil `dateiverweise` seinen Rufer bekommt." **Das trifft nicht zu.**
Keine der vier Funktionen dieses Schritts ruft `dateiverweise`; der Rufer ist
`DateifensterQuelle::abwurf_pruefen` und entsteht in Schritt 10. Der
`reason`-Text in `zwischenablage.rs:285` nennt aus demselben Irrtum ebenfalls
Schritt 8. Die Datei stand nicht in der Grenze dieses Auftrags und ist
unangetastet; der Vermerk und sein Text gehören nach Schritt 10 mit den drei
anderen.

## Was ausdrücklich nicht angefasst wurde

- `crates/krk-ui/src/appkit/anwendung.rs` — Schritt 9, in der Hand eines
  anderen Agenten.
- `crates/krk-ui/src/appkit/zwischenablage.rs` — siehe Punkt 4.
- Keine dritte `#![allow(unsafe_code)]`-Ausnahme. Das Modul erbt sie aus
  `appkit/mod.rs:1`; nachgeprüft, die Datei trägt keine eigene Lint-Zeile.
- Die zwei Aufruferzählungen in `abwurfregel.rs` bleiben auf null. Dieser
  Schritt ruft weder `marke` noch `urteil`.

## Formatierung

`rustfmt --edition 2024` auf den drei eigenen Dateien, **nicht**
`cargo fmt --all` und nicht `cargo fmt -p krk-ui`. Beide hätten die ganze Kiste
erfasst; zwei Agenten dieser Sitzung haben das getan und konnten hinterher nicht
feststellen, ob sie die Datei eines gleichzeitig arbeitenden Agenten
umformatiert haben. `git diff --stat` weist für diesen Lauf nur die drei eigenen
Dateien aus (`anwendung.rs` steht darin aus fremder Hand).

## Abnahme

`make check` — **Exit 0**, alle vier Kommandos grün (Bau, Proben,
`cargo fmt --all --check`, Clippy unter `-D warnings`). Die sechs neuen Proben
laufen grün. Vor jedem Lauf geprüft, dass weder `/tmp` noch `$TMPDIR` eine
`krk-messplan-*.toml` führt; es lief kein Messlauf.

**Nicht committet** — der Nutzer committet selbst.
