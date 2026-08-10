# Fünf Defekte am Rückgängigverlauf des Editors

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1044
**Dateigrenze:** ausschließlich `crates/krk-ui/src/appkit/editor.rs`

---

## Auftrag

Fünf offene Defekte in `crates/krk-ui/src/appkit/editor.rs`: der
Rückgängigverlauf (`260810-0303`, `260810-0419`, `260810-0420`), das Richten der
Textfläche (`260810-0424`) und der Hauptfaden der Proben (`260810-1001`). Der
Auftrag verlangte, den gemeinsamen Schnitt zu suchen, statt drei getrennte Zweige
anzubauen, und bei `260810-0424` eine Messung statt einer Schätzung.

## Der gemeinsame Schnitt, und wo keiner war

Die Vermutung des Auftrags war, dass ein Rücken des Leerens an die richtige
Stelle mehrere Defekte zugleich erledigt. Sie trifft für einen der drei, und der
Grund gehört dazu:

- **Das Leeren allein zu verschieben, brächte `260809-1727` zurück.** Der Stapel
  einer `NSTextView` hält Handlungen der Form „ersetze Bereich B durch die alte
  Zeichenkette"; B ist gegen den Text aufgezeichnet, den die Fläche vor dem
  Umbau trug. Bleibt er nach einem `setString:` stehen, wirkt ein `cmd+z` gegen
  falsche Stellen. **Nicht zu leeren ist deshalb nur zulässig, wenn der Umbau
  selbst eine Handlung wird** — und das ist der Schnitt, den die Behebung nimmt.
- **`260810-0419` und `260810-0420` sind keine Codedefekte.** Beide sind Befunde
  über Aussagen: eine Begründung, die über die falsche Fläche geführt wurde, und
  zwei Proben, deren Doc-Kommentar mehr behauptet als sie messen. Sie werden mit
  Messungen behoben, nicht mit Mechanismen.

## Was gebaut ist

**Der Anlass kommt als Wert in die eine Schreibstelle.** Neuer Typ `Verlauf` mit
`Faellt` und `Traegt(Umkehrpunkt)`; `stand_einsetzen` und `stand_erneuern` nehmen
ihn, alle sieben Aufrufstellen nennen ihn, die Aufzählung ist vollständig und
ohne Auffangzweig.

```
  Anlass                     Verlauf danach
  Dateiwechsel, Schliessen ─> Faellt   der Verlauf gehoerte einer anderen Datei
  Ersetzen (S37)           ─> Traegt   der Nutzer nimmt das Ersetzen zurueck
  CRLF-Richten             ─> Faellt   der vorige Text der Flaeche ist kein
                                       gueltiger Stand
```

`treffer_ersetzen` und `alle_treffer_ersetzen` nehmen vor dem Ruf ins Modell
einen `Umkehrpunkt` (gehaltener Stand plus Auswahl der Fläche) und melden ihn
über `umkehrung_anmelden` beim Rückgängigverwalter der Fläche an. `umkehren`
stellt ihn her, meldet den Gegenweg an und setzt die Auswahl beschnitten zurück.
Der Verwalter ist derselbe, in dem die Fläche ihr Tippen führt — das ist die
Voraussetzung dafür, dass die älteren Handlungen der Fläche nach einer Umkehrung
wieder auf ihren Text passen.

## Was gemessen wurde

Alles auf macOS 15.7.7 (Build 24G720), Rust 1.97.1, am 260810.

**Der Rückgängigweg von AppKit** (Wegwerf-Programme in Swift im
Sitzungsverzeichnis):

| Frage | Antwort |
|---|---|
| Meldet `setString:` eine Handlung an? | nein, `canUndo` bleibt falsch |
| Wer beantwortet `undo:` in der Antwortkette? | allein `NSWindow` |
| Welchen Verwalter nimmt `NSWindow.undo:`? | den des Ersthelfers, nicht seinen eigenen |
| Verwalter des Feldeditors einer `NSTextField` | eigener `NSCellUndoManager` |
| `removeAllActions` am Verwalter des Fensters | lässt den Feldeditor unberührt |
| `undo` im Feld danach | nimmt den getippten Namen zurück |

Die letzten drei Zeilen widerlegen die Vermutung von `260810-0419`: die
Umbenennung „direkt in der Liste" verliert nichts. Die zweite und dritte halten
fest, dass der dort vorgeschlagene Ausweg `undoManagerForTextView:` gangbar
**wäre** — er wird nicht genommen, weil es keinen zweiten Anmelder gibt.

**Die Kopien der CRLF-Kette** (zählender Allokator, Anlagen ab 1 MB, Text 16,0 MB,
eingefügtes `\r\n` vorn, Schreibmarke dahinter):

```
  bearbeiten: in_gehaltene_form(stand)                1 Kopie,  16,0 MB
  versatz_nach_der_wandlung, Fassung im Baum          1 Kopie,  16,0 MB
  dieselbe Rechnung ueber Cow<str>                    0 Kopien,  0,0 MB
  Gegenfall (Rest traegt selbst ein \r\n): beide      2 Kopien, 32,0 MB
```

**Der Hauptfaden im Prüfstand:**

```
  cargo test                          MainThreadMarker::new() ─> None
  cargo test -- --test-threads=1      MainThreadMarker::new() ─> None
  [[test]] mit harness = false        MainThreadMarker::new() ─> Some
```

## Proben, die dazugekommen sind

- `ein_geleerter_stapel_ueberlebt_auch_die_ereignisgruppierung` — die
  Betriebsart der Laufzeit (`groupsByEvent` auf dem Werkswert, die Gruppe vom
  Verwalter selbst geöffnet, ein Umlauf der Laufschleife danach). Behebt
  `260810-0420`: die Messung stand in einem Wegwerf-Programm und steht jetzt im
  Baum.
- `eine_anmeldung_waehrend_eines_rueckgaengig_landet_im_wiederherstellungsstapel`
  — die Mechanik, auf der `umkehren` ruht. Ohne sie gäbe es kein
  Wiederherstellen und möglicherweise einen Ring.
- `handlung_anmelden` zieht die dreifach gleiche Anmeldung zusammen.

## Was offen bleibt, und warum

- **`260810-0424`** bleibt offen. Die eine vermeidbare Kopie steht in
  `krk-core/src/text/datei.rs` und damit außerhalb der Dateigrenze. In
  `editor.rs` allein ist nichts zu holen: `Editormodell::bearbeiten` verbraucht
  die Abschrift, und wer sie danach braucht, klont sie — dieselbe Kopie an
  anderer Stelle. Die Messung und die zwei geprüften Auswege stehen am
  Datensatz.
- **`260810-1001`** bleibt offen. Weg 2 des Datensatzes ist der richtige und
  kostet weniger als dort angenommen (kein zweites Prüfkommando), braucht aber
  zwei Dateien außerhalb der Grenze und eine Entscheidung über sechs
  modulinterne Stücke. Der Datensatz dazu ist
  `decisions/260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.
- **Die CRLF-Hälfte von `260810-0303`** ist als eigener Datensatz abgelegt:
  `issues/260810-1044_o_ein-eingefuegtes-crlf-bleibt-nicht-ruecknehmbar-und-der-grund-liegt-am-eingang-der-flaeche.md`.
  Sie ist an dieser Stelle nicht zu bauen, und die zwei Gründe stehen an
  `flaeche_richten`.

**Nutzerarbeit:** die Wirkung am laufenden Bündel, also dass ein `cmd+z` nach
`shift+cmd+r` und nach `ctrl+cmd+r` den vorigen Stand samt Schreibmarke zeigt
und ein zweites den Anschlag davor.

## Abnahme

| Kommando | Ausgang |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt -p krk-ui --check` | exit 0 |

`cargo fmt --all` wurde nicht gefahren: an den Nachbarkisten arbeiteten parallel
andere Agenten.

## Berührte Datensätze

| Datensatz | Was geschah |
|---|---|
| `issues/260810-0303_*` | `Resolved:` — Ersetzen umkehrbar, CRLF-Rest abgetrennt |
| `issues/260810-0419_*` | `Resolved:` — Wirkung gemessen, tritt nicht ein; Kommentar berichtigt |
| `issues/260810-0420_*` | `Resolved:` — dritte Probe im Baum, Kommentare eingeschränkt |
| `issues/260810-0424_*` | offen, Messung angehängt; Behebung liegt in `krk-core` |
| `issues/260810-1001_*` | offen, Messung angehängt; Weg 2 gewählt, Entscheidung vorgelegt |
| `issues/260810-1044_*` (neu) | die CRLF-Hälfte von `260810-0303` |
| `decisions/260810-1044_*` (neu) | Prüfziel ohne libtest-Harness, drei Optionen |
