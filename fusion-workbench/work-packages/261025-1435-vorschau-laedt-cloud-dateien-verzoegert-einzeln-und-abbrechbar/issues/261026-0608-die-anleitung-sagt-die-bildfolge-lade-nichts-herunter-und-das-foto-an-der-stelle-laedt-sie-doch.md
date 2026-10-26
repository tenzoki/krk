Die Anleitung sagt, die Bildfolge lade nichts herunter, und das Foto an der Stelle lädt sie doch
---
`HowTo.md` nennt die Bildfolge unter den Lesern, die nichts herunterladen; das Foto an der Stelle einer Bildfolge lädt die Vorschau nach 0,2 s herunter.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`

**Befund.** `HowTo.md`, Abschnitt zur Vorschau, der Absatz „Eine Datei, die auf einem Cloudlaufwerk …“ (eingefügt in `0650dd47`): „Die Bildfolge, der Inhaltsfilter mit „Content“ und die Zusammenfassung eines Ordners laden nichts herunter“. Der Modulkopf von `crates/krk-ui/src/vorschaumodell.rs` (`# Eine Datei, die nur in der Cloud liegt`) führt in seiner Tabelle dagegen „die Vorschau der ausgewaehlten Datei, auch das Foto an der Stelle einer Bildfolge“ als Leser, der lädt, und `Vorschautab::folge_nachziehen` und `Vorschautab::blaettern` starten das Foto an der Stelle über `Ladevorgang::starten`, also durch das Tor in `laden`. Wer durch eine Bildfolge aus Cloud-Fotos blättert, sieht das Foto erscheinen, nachdem die Anleitung ihm gesagt hat, die Bildfolge lade nichts.

Der Satz folgt dem Wortlaut von Schritt 8 des Plans; ungenau ist schon dieser.

**Abnahme.** Der Absatz in `HowTo.md` trennt das Ordnen der Bildfolge (lädt nichts, ein Foto in der Cloud steht nach seinem Änderungsdatum) vom Foto an der Stelle (lädt wie jede andere Vorschau nach 0,2 s, eines zur Zeit, bis 64 MB).

Resolved: 261026, behoben im Commit nach `d325032e` (Nachbesserung nach der Abschlussdurchsicht), `make check` gruen.
