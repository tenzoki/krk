Fünf Zahlen in der Prosa des Anwendungsdelegierten sind mit dem fünften Bereich und den Nachzügen vom 260825 falsch geworden

---

`anwendung.rs` trägt an fünf Stellen eine Zahl, die der Baum nicht mehr hält: dreimal „vier Bereiche" (die Fensterzeile hat seit der Editor-Runde fünf), einmal „drei Aufrufer" von `fokus_setzen` (es sind fünf) und einmal „vier Anlässe" von `titel_nachziehen` (es sind sechs Rufstellen). Keine davon trägt Verhalten; jede führt den nächsten Leser auf eine Aufzählung, die er für vollständig hält.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Stellen (`crates/krk-ui/src/appkit/anwendung.rs`)

| Zeile | Text | Baum |
|---|---|---|
| `15` | „Aufteilung — die NSSplitView mit ihren vier Bereichen" | `Bereich` hat fünf Werte (`CLAUDE.md`, `fenstermodell.rs`); `bereich_des_ersthelfers` läuft über „die fuenf Unteransichten" (`5898`) |
| `80-81` | „vier fokussierbare Bereiche — die beiden Dateilisten, die Leiste, die Vorschau und die Textflaeche des Editors" | die Aufzählung im selben Satz nennt fünf |
| `1135` | „Baut die vier Bereiche, stellt die Sitzung her" | `oberflaeche_aufbauen` baut zwei Dateifenster, Leiste, Vorschau, Editor (`1148-1154`) |
| `2390-2392` | „Drei Aufrufer: die Fokusbefehle ueber `fokus_holen`, das Ausblenden eines Randbereichs, und der Aufbau" | Rufer bei `1458`, `2256`, `3210`, `4468`, `4584`; die zwei vom 260825 (`FensterWechseln`, `aktives_setzen`) fehlen |
| `5080-5083` | „Vier Anlaesse rufen sie, drei davon ueber `fokusanzeige_nachziehen` hinaus" | Rufstellen bei `1304`, `1465`, `3439`, `5069`, `6965`, `7459`; der Aufbau (`1465`) und `editor_ausblenden` (`7459`) fehlen |

`CLAUDE.md` hat für genau diese Klasse die Regel gezogen, Zahlen nicht in Prosa zu tragen, die mit jeder Runde wächst („eine Zahl an dieser Stelle veraltet"). Die zwei Aufruferzahlen widersprechen zudem den Zählproben derselben Datei, die für `aktives_setzen` und `zettel_sichern` gerade solche Zahlen halten (`8957-8965`, `8488-8498`): wer dort eine Zahl zusagt, hält sie; wer sie nur hinschreibt, hat sie hier fünfmal verloren.

## Vorschlag

Die drei „vier Bereiche" auf „fünf" oder auf „die Bereiche der Fensterzeile" ohne Zahl; die zwei Aufruferlisten entweder um die fehlenden Rufer ergänzen oder auf „wer ruft, sagt es an sich selbst" umstellen, wie `bildschirmbreiten_uebernehmen` es nach demselben Fehlschlag getan hat (`4760-4764`).

Gefunden bei der Vollbaum-Durchsicht R7 an HEAD `7ac511a`.
