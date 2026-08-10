# Die neuen Proben behaupten den Hauptfaden, den libtest ihnen nicht gibt

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coder, beim Beheben von 260810-0748 und 260810-0750
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`an_einer_flaeche` und die vier Proben, die es rufen)
**Cross-references:** `issues/260810-0748_*_die-kopplung-der-zehn-paare-traegt-den-commit-und-ist-im-baum-durch-nichts-gehalten.md`, `issues/260810-0750_*_derselbe-speicher-ist-eine-stufe-staerker-als-die-messung-hergibt.md`, `issues/260810-0512_*_die-schreibwerkzeuge-aus-macos-15-schreiben-den-text-um-und-sind-nicht-abgewaehlt.md`

---

## Der Befund

Vier neue Proben bauen eine `NSTextView`, um zu messen statt zu behaupten: die
Kopplung der zehn Paare, die Nicht-Darstellbarkeit von `Default`, die Sammeltür
und die sieben abgeschalteten Automatiken. Alle vier gehen durch
`an_einer_flaeche`, und dort steht:

```rust
arbeit(unsafe { MainThreadMarker::new_unchecked() })
```

**Das ist eine Behauptung und keine Tatsache.** `libtest` fährt seine Proben auf
eigenen Fäden, und Apple sagt für eine `NSView` den Hauptfaden zu. Vor diesem
Durchgang stand `new_unchecked` in dieser Datei genau einmal, für einen
`NSUndoManager` — ein Foundation-Objekt ohne Fadenbindung. Auf eine
AppKit-Ansicht ist das eine Stufe weiter, und im ganzen Projekt gibt es keine
zweite Stelle, die es täte.

## Was dafür sprach

Der Gegenwert ist erheblich, und er stand vorher nirgends im Baum: die Kopplung
der zehn Paare trug die Entscheidung, `textflaeche_bauen` **nicht** um zehn
Zeilen zu ergänzen, und wurde von keiner Probe gehalten (`260810-0748`). Ebenso
der Vorgabewert der Schreibwerkzeuge, der als `speculation:` geführt war
(`260810-0512`), und die Aussage über `Default` (`260810-0750`). Alle drei sind
jetzt nachgemessen, auf jedem Gerät, das `cargo test` fährt, und damit auch auf
macOS 26.

Der Bereich ist eng gehalten: die Proben bauen eine Fläche, lesen und setzen
Merkmale und lassen sie fallen. Kein Fenster, keine Zeichnung, keine
Ereignisschlange, kein Ersthelfer. Eine Sperre serialisiert sie, damit nicht
zwei Fäden gleichzeitig AppKit-Objekte bauen.

**Gemessen:** sechs vollständige Läufe von `cargo test --workspace` nach dem
Umbau, alle exit 0, ohne Absturz und ohne Meldung, auf macOS 15.7.7 (Build
24G720). Davor fünf Läufe mit einer Vorform der Proben, in denen das Prüfziel
von `krk-ui` jedes Mal durchlief; der Arbeitsbaum war dabei aus einem anderen
Grund nicht grün (ein paralleler Agent änderte `krk-core/src/text/datei.rs`), und
diese fünf sind deshalb kein Beleg über den Arbeitsbaum, sondern nur über die
AppKit-Instanzen.

## Was das nicht belegt

Sechs saubere Läufe auf einem Gerät sind kein Nachweis, dass der Aufruf zulässig
ist. Sie belegen, dass er heute nicht auffällt. Der Unterschied ist genau der,
den `260810-0748` an der Kopplungsmessung bemängelt hat, eine Ebene höher: eine
Aussage, die niemand widerlegt hat, ist keine Aussage, die etwas hält.

Der Datensatz steht deshalb hier und nicht nur als Doc-Kommentar: wer die
Abwägung anders trifft, findet ihn und kann die vier Proben zurücknehmen, ohne
den Rest des Umbaus anzufassen.

## Vorschlag

Drei Wege, in der Reihenfolge, in der ich sie für tragfähig halte:

1. **Stehen lassen und beobachten.** Bricht eine Reihe auf einem anderen Gerät
   oder unter einer anderen Fassung von macOS, ist die Ursache am Namen der
   Probe sofort erkennbar, und der Rückbau kostet vier Proben.
2. **Die Proben auf den Hauptfaden bringen.** `libtest` gibt ihn nicht her;
   ein eigenes Prüfziel neben `cargo test` täte es, etwa unter `xtask`, das
   seinen Hauptfaden selbst hält. Kostet ein zweites Prüfkommando, das
   `make check` mitfahren müsste.
3. **Zurücknehmen und stattdessen ablegen**, wie `260810-0748` es vorschlug: das
   Messprogramm unter `spikes/`, ein Bericht unter `messungen/`. Hält weniger,
   behauptet aber nichts über Fäden.

Gemeldet von: `coder`, im Durchgang zu den acht Datensätzen vom 260810.

---
## Die Abwaegung ist getroffen: Weg 2, und er ist gemessen

**Weg 2 ist der richtige, und er kostet weniger als dieser Datensatz annimmt.**
Die Annahme "Kostet ein zweites Pruefkommando, das `make check` mitfahren
muesste" ist falsch: ein `[[test]]`-Ziel mit `harness = false` wird von `cargo
test` mitgefahren, also auch von `make check`. Gemessen am 260810-1044 auf macOS
15.7.7 (Build 24G720), Rust 1.97.1, an einer eigenen Kiste:

```
  cargo test                          MainThreadMarker::new() ─> None
  cargo test -- --test-threads=1      MainThreadMarker::new() ─> None
  [[test]] mit harness = false        MainThreadMarker::new() ─> Some
```

Die naheliegende Abhilfe traegt also **nicht**: `libtest` gibt den Hauptfaden auch
bei einem einzigen Prueffaden nicht her, es legt jede Probe auf einen eigenen
Faden. Ein Ziel ohne `libtest`-Harness bekommt ihn, weil es sein `main` selbst
haelt.

**Gebaut ist der Weg nicht**, und der Grund ist die Dateigrenze dieses
Durchgangs: er hatte ausschliesslich `crates/krk-ui/src/appkit/editor.rs` in der
Hand, und Weg 2 braucht zwei weitere Dateien —
einen `[[test]]`-Abschnitt in `crates/krk-ui/Cargo.toml` und die Prueflaufdatei
darunter. Dazu kommt eine Entscheidung, die nicht mechanisch ist: die vier Proben
rufen `textflaeche_bauen`, `EINSTELLUNGEN`, `merkmal`, `merkmal_setzen`,
`merkmalsname` und `probenrahmen`, und alle sechs sind heute modulintern. Ein
Pruefziel ausserhalb der Kiste erreicht sie nicht. Sie dafuer oeffentlich zu
machen, waere oeffentliche Schnittstelle ohne Aufrufer im Programm — genau das
Muster, das `260810-0212` in diesem Circle schon fuehrt.

Der Datensatz zu dieser Entscheidung ist
`decisions/260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.

**Weg 1 ist nicht gewaehlt, sondern die Lage bis zur Entscheidung**, und der
Doc-Kommentar von `an_einer_flaeche` sagt das jetzt so: er fuehrt die drei
Zeilen der Messung, nennt Weg 2 als den richtigen, benennt die zwei Dateien und
die sechs modulinternen Stuecke, die ihm noch fehlen, und sagt, dass die
Notluege nicht steht, weil sie zulaessig waere, sondern weil ihr Rueckbau die
vier Messungen kostete. Weg 3 bleibt ausgeschlossen: er nimmt die Messungen aus
dem Baum.

Dieser Datensatz bleibt offen, bis das Pruefziel steht.

---

## Nachgeprüft am 260810-1139: Weg 2 ist nicht baubar, wie er beschrieben ist

Die Messung, die Weg 2 zuschneidet, hält: ein `[[test]]`-Ziel mit
`harness = false` bekommt den Hauptfaden (`MainThreadMarker::new()` liefert
`Some`), und `cargo test` fährt es mit. Nachgemessen am 260810-1057 an einem
Prüfziel in diesem Projekt.

**Was der Abschnitt oben nicht wusste: `krk-ui` hat kein Bibliotheksziel.** Die
Kiste führt allein `[[bin]] name = "krk"`. Eine Prüflaufdatei unter `tests/` ist
eine eigene Kiste und erreicht deshalb nichts aus `krk-ui`, gleich ob
`textflaeche_bauen` und die fünf übrigen Stücke `pub` sind oder nicht:

```
  error[E0433]: cannot find module or crate `krk_ui` in this scope
```

Damit ist der Satz „was ihm noch fehlt, ist keine Messung mehr, sondern eine
Entscheidung über zwei Dateien" überholt: es fehlt ein Bibliotheksziel oder ein
zweiter Kistenkopf, und beides ist ein Umbau der Kiste und keine zwei Dateien. Die
beiden neuen Optionen und die geänderte Empfehlung stehen im Entscheidungsdatensatz
`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.

**Weg 1 bleibt damit die Lage und nicht die Wahl**, unverändert. Der Doc-Kommentar
von `an_einer_flaeche` sagt das schon so; nachzutragen ist dort allein, dass die
zwei genannten Dateien nicht genügen. Dieser Datensatz bleibt offen, bis das
Prüfziel steht.
