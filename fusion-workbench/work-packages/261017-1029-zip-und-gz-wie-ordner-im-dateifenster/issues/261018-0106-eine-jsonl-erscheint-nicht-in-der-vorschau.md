Eine `.jsonl` erscheint nicht in der Vorschau, weder im Dateisystem noch in einer `.gz`
---
Fehlerbericht des Nutzers zu 3.7.0, wörtlich: „jsonl werden nicht angezeigt, weder innerhalb einer gz datei noch normal im fs“. Beispieldatei `/Users/k1/Downloads/entries.jsonl` (10,9 MB, 22 888 Zeilen, gültiges UTF-8).
---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Cross-references:** `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md` (E14), Commit `a62abdcd`.

## Ursache

**Im Dateisystem liegt kein Defekt im Lesen oder Anzeigen vor: die Vorschau war ausgeblendet.** Belege:

- Das laufende KRK 3.7.0 des Nutzers (gestartet 00:44:50, eine Minute nach der Installation, die Binärdatei enthält die Texte der `.jsonl`-Vorschau) zeigt auf einem Fensterbild (`screencapture -l` des Fensters, nur gelesen) in der Leiste am Fensterfuß „Vorschau“ nicht angekreuzt und „Editor“ angekreuzt; am rechten Rand steht der leere Editor mit der Zeilennummer 1. Links ist `entries.jsonl` gewählt, rechts ist `entries.jsonl.gz` betreten.
- `~/Library/Application Support/KRK/session.toml` dieses Laufs (00:45) trägt unter `[sichtbar]` `vorschau = false` und `editor = true`. Der Fehlerbericht kam um 00:46.
- Bei ausgeblendeter Vorschau legt `vorschau_fuellen` (`crates/krk-ui/src/appkit/anwendung.rs`) die Auswahl nur als Nachtrag ab; zu sehen ist sie erst nach `f3`.
- Derselbe Weg am gebauten Bündel (HEAD, eigener `HOME` mit einer Kopie von `readers.toml` und `settings.toml` des Nutzers, linkes Fenster auf `/Users/k1/Downloads`, Auswahl über `Dateifenster::eintrag_waehlen`, also über `auswahl_merken` → `vorschau_fuellen` → `Vorschaufenster::datei_anzeigen` → Arbeitsfaden → `einziehen` → `anzeigen`): die Textfläche hält 1 186 479 UTF-16-Einheiten, beginnt mit dem Kürzungshinweis, ist sichtbar, und das Fensterbild zeigt die Datensätze eingerückt und eingefärbt mit Nummernspalte. Aufbereiten 6–14 ms, Einfärben rund 1 s auf dem Faden, 83 791 Farbbereiche.

**In einer `.gz` war es kein Defekt, sondern der damalige Umfang:** E14 des Plans der Archive ließ die Vorschau für einen Eintrag im Archiv allein die Angaben aus der Liste zeigen.

## Behebung

- Kein Eingriff in den Weg der lokalen Vorschau. Die Probe `die_auswahl_einer_jsonl_datei_zeigt_sie_ueber_den_arbeitsfaden` (`crates/krk-ui/src/vorschaumodell.rs`) hält den Weg über `datei_anzeigen` und den Arbeitsfaden für eine `.jsonl` über der Grenze und eine kleine `.ndjson`; sie war vor der Arbeit grün, weil der Weg funktioniert.
- `HowTo.md` sagt unter den JSON-Zeilen, dass eine ausgeblendete Vorschau nichts zeigt und `f3` sie zurückholt.
- Die Vorschau liest seitdem eine Datei in einem lokalen Archiv an (Teil 2 desselben Auftrags, `Beschreibung::Archiv`, `archiveintrag_laden`), sodass eine `.jsonl` in einer `.gz` oder einem Zip aufbereitet erscheint.

## Abnahme

Mit eingeblendeter Vorschau erscheint eine gewählte `.jsonl` aufbereitet, im Dateisystem wie in einem Archiv. Der Nutzer prüft das an seinem Bündel, nachdem er `f3` gedrückt hat.

---
Resolved: kein Defekt im Lesen oder Anzeigen; die Vorschau war ausgeblendet (Fensterbild und `session.toml` des laufenden 3.7.0). Weg am Bündel mit der Datei des Nutzers geprüft, Probe über den Arbeitsfaden ergänzt, `HowTo.md` nennt `f3`; die Archivvorschau liest den Eintrag jetzt an (`crates/krk-ui/src/vorschaumodell.rs` `archiveintrag_laden`).
