Nach einem fehlenden Notizordner oder einem gescheiterten Schreiben sichern weder das Verlassen noch das Beenden den letzten Stand der Quicknote, und keine Probe fährt die Vormerkung
---
Dass eine Änderung noch nicht gesichert ist, hält der Editorbereich allein in `quicknote_faellig`. `quicknote_sichern` nimmt diese Vormerkung schon beim Absenden und nicht erst, wenn der Faden `Gesichert` meldet. Antwortet der Faden mit `OrdnerFehlt` oder `Gescheitert`, gibt es keine Vormerkung mehr. `quicknote_verlassen` und `quicknote_abschliessen` (aus `wird_beendet`) schreiben dann nichts, und der Text geht beim Beenden verloren, obwohl die Ursache inzwischen behoben sein kann.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md` (E2, E4, Schritt 7), `261031-1156-reviewer-abschlussdurchsicht-quicknote-scratch-strip-format.md`

**Ort:** `crates/krk-ui/src/appkit/editor.rs`, `Editorbereich::quicknote_sichern` (`if self.ivars().quicknote_faellig.take().is_none() { return; }`) und `Editorbereich::quicknote_abschliessen`, das allein `quicknote_sichern` ruft. `crates/krk-ui/src/quicknotesicherung.rs`, `stand_nach` hält dazu fest: „der naechste Auftrag versucht es wieder“. Ein nächster Auftrag entsteht aber nur durch eine weitere Änderung.

**Ablauf, der Text verliert** (abgeleitet aus dem Code, nicht am Bündel gefahren). Dieser Ablauf ist der gewöhnliche erste Kontakt:
1. Neue Installation ohne `~/krkhome`: `f10`, tippen. Nach einer Sekunde meldet die Statuszeile „… der Notizordner … fehlt. F2 legt ihn an.“
2. F2 legt den Ordner an.
3. Ohne weiteres Tippen KRK beenden: `quicknote_abschliessen` findet keine Vormerkung, also kommt kein Schreibauftrag, und der Text ist fort.

Dasselbe gilt nach `Gescheitert` (etwa voller Datenträger, danach Platz geschaffen). Der Brief verlangt in Ziel 1, dass **jede** Änderung ohne Zutun des Nutzers in `scratch.txt` landet. `HowTo.md` sagt „Nach F2 sichert sie bei der nächsten Änderung von selbst“. Dass beim Beenden ohne weitere Änderung der Text verloren geht, sagt der Satz nicht.

**Warum es keine Probe gefunden hat:** Die Vormerk- und Abgabelogik (`quicknote_vormerken`, `quicknote_sichern`, `quicknote_sicherung_einziehen`, `quicknote_sicherung_laeuft`, `quicknote_abschliessen`) steht allein in `appkit/editor.rs` und wird von keiner Probe ausgeführt. `die_quicknote_stellt_auf_dem_hauptfaden_keinen_systemaufruf` liest ihre Rümpfe allein auf Nadeln. Die fünf AppKit-Proben, die Schritt 7 des Plans an einer Fläche verlangte, stehen als Quelltextproben da, weil eine Fläche unter `libtest` nicht läuft. Die Logik selbst braucht aber keine Fläche: Fälligkeit, „ungesichert seit“ und die Entscheidung, ob beim Verlassen oder Beenden geschrieben wird, sind Zustand und Zeit.

**Richtung:** Den Merker „ungesichert“ an eine Stelle ohne AppKit legen (etwa neben `Sicherung` in `quicknotesicherung.rs`) und ihn allein mit `Gesichert` löschen, nicht beim Absenden. `quicknote_verlassen` und `quicknote_abschliessen` schreiben, solange er steht und die Sicherung Aufträge annimmt, gleich ob eine Vormerkung läuft. Die Fälligkeit nach `QUICKNOTE_RUHE` hängt am selben Wert.

**Abnahme:**
- Eine Probe ohne AppKit: Änderung, Abgabe, Antwort `OrdnerFehlt`, Ordner anlegen, Abschluss ohne weitere Änderung, danach steht der Text in `scratch.txt`.
- Dieselbe Probe mit `Gescheitert` statt `OrdnerFehlt`.
- Eine Probe hält, dass eine Abgabe vor `QUICKNOTE_RUHE` nicht geschieht und danach genau einmal.
- `vor_der_antwort_auf_das_lesen_wird_nichts_geschrieben` und `die_quicknote_stellt_auf_dem_hauptfaden_keinen_systemaufruf` bleiben grün.
