`bis_zur_grenze_lesen` öffnet die Datei vor der Größenprüfung, und auf einem Cloudlaufwerk holt das jede zu große Datei herunter
---
`krk_core::text::datei::bis_zur_grenze_lesen` (und `anlesen` daneben) öffnet über `sys::ohne_warten_oeffnen` und vergleicht erst danach `fstat`-Größe gegen die Grenze. Auf einem File-Provider-Laufwerk (Google Drive im Streaming-Modus) ist fast jede Datei ein Platzhalter (`SF_DATALESS`), und `inference:` das Öffnen zum Lesen lässt macOS die Datei vollständig herunterladen, bevor `open(2)` zurückkehrt. Eine Datei, die die Prüfung danach als `ZuGross` abweist, ist dann schon geholt. Die Vorschau (`vorschaumodell::laden`, Zweig `Leseart::Text` für jede Endung außer Bild, PDF, JSON-Zeilen und Trenntext) und der Inhaltsfilter (`verzeichnis::inhalt`) gehen beide diesen Weg, obwohl die Größe aus `lstat` (Vorschau) bzw. aus dem Lesevorgang (`Eintrag::groesse`, Durchlauf) schon vorliegt.
---
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1109-warum-die-navigation-auf-google-drive-zuweilen-langsam-ist.md` (Befund B2)

## Beleg

- `crates/krk-core/src/text/datei.rs`, `bis_zur_grenze_lesen`: Öffnen, dann `datei.metadata()`, dann `angaben.len() > grenze`.
- `crates/krk-ui/src/vorschaumodell.rs`, `laden`: `roh = symlink_metadata(pfad)` steht vorher da, `roh.len()` wird nicht gegen `Leseart::grenze` gehalten.
- Gemessen am 261025 unter `/Users/k1/Google Drive/Meine Ablage`: 27 897 von 27 901 Dateien tragen `SF_DATALESS`, zusammen 184 GB; darunter 5 843 `.m4a` und 3 178 `.mov`, die alle in den Zweig `Leseart::Text` fallen.
- Ob `open(2)` allein schon herunterlädt, ist nicht gemessen (Messen hätte einen Download ausgelöst). Gemessen werden kann es an einer bekannten großen Platzhalterdatei mit `ls -lO` vor und nach einem `open`/`close` ohne `read`.

## Abnahme

Für eine Datei, deren schon bekannte Größe über der Grenze liegt, ruft weder die Vorschau noch der Inhaltsfilter `open(2)`. Eine Probe hält das fest, etwa mit einer Datei über der Grenze ohne Leserecht: das Ergebnis ist `ZuGross` und nicht `Fehler`.
