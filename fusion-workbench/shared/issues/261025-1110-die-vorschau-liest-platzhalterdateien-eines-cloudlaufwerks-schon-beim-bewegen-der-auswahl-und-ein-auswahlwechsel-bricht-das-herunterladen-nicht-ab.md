Die Vorschau liest Platzhalterdateien eines Cloudlaufwerks schon beim Bewegen der Auswahl, und ein Auswahlwechsel bricht das Herunterladen nicht ab
---
Jede Auswahl im Dateifenster startet einen eigenen Faden `krk-vorschau` (`Ladevorgang::starten` in `crates/krk-ui/src/vorschaumodell.rs`), und `laden` liest den Inhalt der Datei: Bild und PDF bis 64 MB, jede andere Datei bis 1 MB. Für einen Ordner unter `/Fotos/<Jahr>` oder `/Fotos/<Jahr>/<Monat>` liest die Bildfolge (`readers.toml`, Profile „Fotos: ein Jahr“ und „Fotos: ein Monat“) das Aufnahmedatum jedes Fotos (`Bildverzeichnis::gruppe_ordnen`, bis 256 KB je Foto, bis 7 500 Fotos). Auf einem File-Provider-Laufwerk ist jede dieser Dateien ein Platzhalter (`SF_DATALESS`), und jedes Lesen lässt sie herunterladen. Der Nutzer hat das nicht verlangt, er hat nur den Cursor bewegt. Ein Auswahlwechsel lässt den alten Empfänger fallen, der Faden liest aber weiter bis zum Ende seiner Datei, und die Abbruchmarke der Bildfolge greift erst vor dem nächsten Foto. Wer schnell über zehn PDFs läuft, hat zehn gleichzeitige Downloads, und die Verzeichnisabfragen der Dateiliste warten hinter ihnen beim Google-Drive-Prozess (`speculation:` Reihenfolge innerhalb des Providers ungemessen). Das Kennzeichen `SF_DATALESS` kommt schon mit `ATTR_CMN_FLAGS` aus `getattrlistbulk` (`verzeichnis/sys.rs`, `satz_zerlegen`) und wird verworfen.
---
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261025-1109-warum-die-navigation-auf-google-drive-zuweilen-langsam-ist.md` (Befunde B1 und B3); `261025-1109-bis-zur-grenze-lesen-oeffnet-die-datei-vor-der-groessenpruefung-und-holt-auf-einem-cloudlaufwerk-jede-zu-grosse-datei-herunter.md`

## Beleg

- Gemessen am 261025: `Meine Ablage/pr/Fotos` hält 10 087 Dateien in Jahresordnern, alle `SF_DATALESS`, zusammen rund 57 GB; `Fotos/2024` allein 1 648 Dateien und 11,3 GB.
- `~/Library/Application Support/KRK/readers.toml`, Profile mit `pfad = '/Fotos/[0-9]{4}$'` und `'/Fotos/[0-9]{4}/[0-9]{2}$'`, treffen `.../Meine Ablage/pr/Fotos/2024`.
- Nicht in KRK gemessen: das hätte die Downloads ausgelöst, die hier beschrieben sind.

## Abnahme

Steht die Auswahl auf einer Datei mit `SF_DATALESS`, liest die Vorschau ohne ausdrückliche Anforderung des Nutzers keine Bytes und zeigt die Metadaten mit einem Hinweis, dass die Datei nur in der Cloud liegt. Die Bildfolge liest von einem Platzhalterfoto kein Aufnahmedatum, sondern nimmt das Änderungsdatum, den Rückfall, den `gruppe_ordnen` schon hat. Wie der Nutzer den Inhalt doch anfordert (Taste, Schalter in `settings.toml`), ist eine eigene Entscheidung und gehört vor den Bau.

Resolved: 261025 — der Wunsch des Nutzers vom 261025 (Narrativ `261025-1435-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`) ersetzt den ersten Satz der Abnahme: die Vorschau lädt eine Platzhalterdatei, aber erst nach 0,2 s Ruhe, höchstens eine zur Zeit über einen Ladeplatz, nur unter der Grenze ihrer Leseart, mit dem Hinweis „Liegt nur in der Cloud …“ davor, und ein Auswahlwechsel lässt keinen weiteren Download beginnen und beendet das Lesen zwischen zwei Stücken (Schritte 6 und 7 des Plans `261025-1444-plan-vorschau-laedt-cloud-dateien-verzoegert-einzeln-und-abbrechbar.md`, `crates/krk-ui/src/cloudladen.rs`, Tor in `laden` in `crates/krk-ui/src/vorschaumodell.rs`); der zweite Satz (Bildfolge) ist mit Schritt 5 erfüllt.
