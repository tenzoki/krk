Anlegen und einzelnes Umbenennen aus C4 baut kein Schritt des Plans

---

C4 nennt sechzehn Abnahmekriterien. Zwei davon gehören zu Funktionen, für die weder S15 noch S16 noch S17 eine Oberfläche vorsieht: das Anlegen eines Ordners und einer Datei, und das Umbenennen eines einzelnen Eintrags in der Liste.

---

## Die beiden Kriterien

> Anlegen: ein Tastenbefehl legt einen Ordner an, ein zweiter eine leere Datei, jeweils im Ordner des aktiven Fensters. Nach dem Anlegen steht die Auswahl auf dem neuen Eintrag.

> Umbenennen: ein Tastenbefehl benennt den ausgewählten Eintrag um, direkt in der Liste.

## Was vorliegt und was fehlt

Der Kern ist fertig. `crates/krk-core/src/operation/anlegen.rs` hält `ordner_anlegen` und `datei_anlegen`, `crates/krk-core/src/operation/umbenennen.rs` hält `umbenennen` samt Namensprüfung. `resources/default-keymap.toml` führt seit S9 die vier Kennungen `ordner_anlegen`, `datei_anlegen`, `umbenennen` und `umbenennen_stapel`.

Es fehlt die Oberfläche, und zwar in beiden Fällen etwas, das die vier Blätter aus S16 nicht abdecken:

- Das Anlegen braucht ein Eingabeblatt für den Namen. Die Hülle aus S13 trägt es, aber kein Schritt nennt eine Datei dafür.
- Das Umbenennen "direkt in der Liste" ist eine bearbeitbare Zelle der `NSTableView`, also `NSTableView.editColumn:row:withEvent:select:` und ein `NSTextField` mit `isEditable`. Das ist kein Blatt und steht in keiner Dateiliste.

S17 baut allein das Umbenennen **im Stapel** mit Musterregeln und Vorschau; sein Abnahmekriterium nennt die vier C4-Kriterien zum Stapel und keines der beiden hier.

## Warum S16 sie nicht mitgenommen hat

Die `Änderungen` von S16 zählen auf, was der Schritt baut: Fortschrittsblatt, Abbruch, Konfliktblatt, Rückfrage vor dem endgültigen Löschen, Abschlussliste, Fokusvorbehalt der Löschtasten. Anlegen und Umbenennen kommen dort nicht vor. Sein Abnahmekriterium sagt "die sechzehn Abnahmekriterien aus C4, soweit sie an der Oberfläche hängen"; die beiden hängen an der Oberfläche, aber an einer anderen als der, die S16 baut.

## Was zu tun ist

Ein Schritt zwischen S16 und S17, oder eine Erweiterung von S17. Vorgeschlagen: `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs` für das Anlegen, und die bearbeitbare Namenszelle in `crates/krk-ui/src/appkit/tabelle.rs` für das Umbenennen. Beide brauchen die vier Kommando-Kennungen, die `crates/krk-core/src/tasten/belegung.rs` noch nicht führt.

**Aufgefallen bei:** der Umsetzung von Schritt 16 am 260804-1815, beim Durchgehen der sechzehn Abnahmekriterien aus C4.

---
Resolved: Der Nutzer hat die beiden Funktionen am 260804-1832 S17 zugeordnet, und der Plan trägt sie jetzt. **Das Anlegen von Ordner und Datei steht in S17**, mit einem gemeinsamen Blatt `crates/krk-ui/src/appkit/blaetter/namenseingabe.rs`; die Zuordnung folgt der Begründung des Nutzers, dass ein Eingabeblatt für den Namen dieselbe Form hat wie das Blatt des Stapel-Umbenennens und auf derselben Hülle aus S13 steht. **Das einzelne Umbenennen hat mit S17b einen eigenen Schritt bekommen**, weil jene Begründung dafür nicht trägt: C4 verlangt es "direkt in der Liste", und das ist kein Blatt, sondern eine bearbeitbare Zelle der `NSTableView` mit Feldeditor und Delegiertem, in der mit 1.454 Zeilen größten Datei der Oberfläche. Mit ihm hätte S17 drei voneinander unabhängige Oberflächenmechanismen getragen. S17b trägt zugleich das C4-Kriterium zum Fokusvorbehalt im Umbenennen-Feld, das bis dahin nirgends nachweisbar war.

Die vom Datensatz verlangte Durchsicht der Belegungsdatei ist erfolgt und hat **genau diese drei Fälle** gefunden: von den 50 Kennungen in `resources/default-keymap.toml` haben vierzehn kein `Kommando`, elf davon gehören S18, S19, S20 oder S17, und übrig bleiben `ordner_anlegen`, `datei_anlegen` und `umbenennen`. `bearbeiten` ist kein Fall derselben Art, weil die Kennung `tasten = []` und `reserviert_fuer = "editor"` trägt.

Die Zahl "sechzehn" in der Überschrift dieses Datensatzes stimmt nicht; C4 führt achtzehn Abnahmekriterien. Der Zählfehler liegt als eigener Datensatz `260804-1832_c_die-zahl-der-c4-abnahmekriterien-steht-im-plan-auf-sechzehn-und-im-spec-auf-achtzehn.md` und bleibt hier als Zeitdokument stehen.
