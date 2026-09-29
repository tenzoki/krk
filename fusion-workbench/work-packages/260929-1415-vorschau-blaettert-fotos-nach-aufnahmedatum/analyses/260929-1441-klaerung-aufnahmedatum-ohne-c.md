# Analysis: Klärung, das Aufnahmedatum ohne C auf beiden Mac-Zielen

**Date:** 2026-09-29 14:41
**Type:** Feasibility
**Status:** Complete
**Requested by:** orchestrator, Schritt 1 des Plans `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (Haltepunkte 1 und 2 des Spec `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`)

## Question

Lässt sich `DateTimeOriginal` auf `aarch64-apple-darwin` und `x86_64-apple-darwin` lesen, ohne dass `cc` oder ein Paket mit einem Namen auf `-sys` in den Baum kommt? Wie viele Bytes kostet das je Format höchstens, gemessen und nicht aus der Dokumentation? Welches Format gibt sein Datum nur um den Preis der ganzen Datei her? Die Antwort entscheidet, ob Schritt 3 beginnt, welchen Leser Schritt 4 baut und welchen Wert `HOECHSTENS_BYTES_JE_FOTO` bekommt.

## Scope

Geprüft sind drei Kandidaten: `kamadak-exif` 0.6.1 (Bibliotheksname `exif`), `nom-exif` 3.8.0 und ImageIO über `objc2-image-io` 0.3.2. Jeder Kandidat lief in einer eigenen Kiste eines Wegwerf-Workspace im Scratchpad (`…/scratchpad/exif-probe/{k,n,i}`), mit der Werkzeugkette aus KRKs `rust-toolchain.toml` (1.97.1). Am KRK-Baum ist nichts geändert außer dieser Datei.

Gemessen wurde auf dem Entwicklungsgerät, nicht auf dem Referenzgerät: Intel Core i9-9880H, macOS 15.8 (24H23), Seitengröße 4096 Bytes. Die Zeiten sind deshalb Größenordnungen und keine Abnahmewerte.

KRK-Baum: HEAD `c374be2` vom 2026-09-29 14:31:55 +0200, Zweig `main`, `## main...origin/main [voraus 21]`. Im Arbeitsbaum geändert waren beim Start `fusion-workbench/orchestrator-events.jsonl` und der Plan dieses Arbeitspakets; beides berührt diese Klärung nicht.

## Findings

### (a) Kandidaten und ihr Abhängigkeitsbaum

Zwei der drei Kandidaten halten die Zusage auf beiden Zielen. `nom-exif` fällt heraus.

| Kandidat | Fassung | Merkmale | `cargo tree --target … -e normal,build` (beide Ziele gleich) | `cc` | Name auf `-sys` |
|---|---|---|---|---|---|
| `kamadak-exif` | `=0.6.1` | keine (die Kiste hat keine Merkmale) | `kamadak-exif` → `mutate_once 0.1.2` | nein | nein |
| `objc2-image-io` | `=0.3.2` | `default-features = false`, `std`, `CGImageSource`, `CGImageProperties`; dazu `objc2-core-foundation` 0.3 mit `std`, `CFURL`, `CFDictionary`, `CFString` | `objc2-image-io` → `objc2-core-foundation 0.3.2` → `bitflags 2.13.2` | nein | nein |
| `nom-exif` | `=3.8.0` | `default-features = false` | u. a. `chrono 0.4.45` → `iana-time-zone 0.1.65` → **`core-foundation-sys 0.8.7`**, dazu `regex`, `tracing`, `nom` 7 und 8, `geo-types` | nein | **ja** |

`nom-exif` bindet `chrono` ohne Merkmalsangabe ein (`[dependencies.chrono] version = "0.4"` in seiner `Cargo.toml`). Damit gilt `chrono`s Vorgabesatz samt `clock`, und `iana-time-zone` zieht auf Apple-Zielen `core-foundation-sys` herein. Von außen lässt sich das nicht abschalten. Der Kandidat verletzt die Zusage und scheidet aus.

`cargo tree -p k --target aarch64-apple-darwin -e normal,build -i cc` und dasselbe für `-p i` melden `package ID specification 'cc' did not match any packages`. Beide Kandidaten bauen unter `--target aarch64-apple-darwin` und `--target x86_64-apple-darwin` (`file` meldet `Mach-O 64-bit executable arm64` bzw. `x86_64`), und die Verzeichnisse `target/<ziel>/release/build` bleiben leer: kein Bauskript läuft. `bitflags` und `objc2-core-foundation` stehen schon heute in KRKs `Cargo.lock` (Zeilen 94, 100, 1753). `mutate_once` ist neu, Lizenz BSD-2-Clause wie `kamadak-exif`.

### (b) Bytes je Format

Gezählt wurde mit zwei Verfahren, weil die zwei Leser verschieden an die Datei gehen.

- **`kamadak-exif`**: ein zählender `Read + Seek` um die `File`, darüber ein `BufReader` mit der Vorgabegröße 8 KiB. Gezählt sind die Bytes, die tatsächlich aus der Datei kommen, samt dem Nachlesen nach einem `seek`. Wahlweise liefert derselbe Zähler ab einer Grenze einen Fehler; er ist damit der Begrenzer, den Schritt 4 bauen soll.
- **ImageIO**: `CGImageSourceCreateWithURL` bildet die ganze Datei per `mmap` ab (gesehen mit einer eingeschobenen Bibliothek, die `open`, `pread`, `read` und `mmap` protokolliert: ein `pread` von 1 Byte, dann `mmap len=<Dateigröße>`). Gelesen wird, welche Seiten es berührt. Deshalb wurde jede Datei mit `F_NOCACHE` neu geschrieben, vor dem Aufruf per `mincore` auf null residente Seiten geprüft und danach gezählt. Ein `CGDataProvider` taugt nicht als Zähler: mit direkten wie mit sequentiellen Rückrufen holt ImageIO die ganze Datei (gemessen: 11.888.197 von 11.888.197 Bytes beim großen JPEG).

Prüfbilder, groß: 4032 × 3024 Pixel mit Rauschen, von ImageIO geschrieben, `DateTimeOriginal` = `2008:03:14 15:09:26`. Dazu `Sonoma.heic` aus `/System/Library/Desktop Pictures/` als echte, von Apple erzeugte HEIC (gelesen, nicht kopiert; sie trägt einen Exif-Block ohne `DateTimeOriginal`). Die Varianten für HEIC und PNG sind aus den großen Bildern umgebaut; wie, steht unter (e).

| Datei | Größe | `kamadak-exif`: Datum | `kamadak-exif`: Bytes gelesen | ImageIO: Datum | ImageIO: berührte Bytes |
|---|---:|---|---:|---|---:|
| JPEG mit Exif | 11.888.197 | ja | 8.192 | ja | 4.096 |
| JPEG ohne APP1-Exif | 11.888.099 | nein | **11.888.099** (ganze Datei) | nein | 4.096 |
| JPEG ohne APP1-Exif, Grenze 256 KiB | 11.888.099 | nein, Fehler „Lesegrenze erreicht“ | 262.144 | – | – |
| PNG, `eXIf` vor `IDAT` | 42.398.944 | ja | 8.192 | ja | 20.480 |
| PNG, `eXIf` hinter `IDAT` (ohne XMP) | 42.398.944 | ja | **42.398.944** (ganze Datei) | **nein** | 20.480 |
| dieselbe, Grenze 256 KiB | 42.398.944 | nein, „Lesegrenze erreicht“ | 262.144 | – | – |
| PNG ohne `eXIf` | 42.399.239 | nein | **42.399.239** | ja, aus dem XMP-`iTXt` | 20.480 |
| HEIC, Exif am Anfang von `mdat` (ImageIO-Anordnung) | 12.797.255 | ja | 24.576 | ja | 16.384 |
| HEIC, Exif-Element am Ende von `mdat` | 12.797.329 | ja | 16.458 | ja | 28.672 |
| HEIC, `meta` hinter `mdat` | 12.797.255 | ja | 27.365 | ja | 20.480 |
| `Sonoma.heic` (Apple) | 20.362.274 | kein `DateTimeOriginal` | 24.576 | kein `DateTimeOriginal` | 16.384 |
| TIFF, IFD0 am Dateiende (ImageIO-Anordnung) | 48.774.380 | ja | **48.774.380** (ganze Datei) | ja | 16.384 |
| dieselbe, Grenze 256 KiB | 48.774.380 | nein, „Lesegrenze erreicht“ | 262.144 | – | – |
| TIFF klein | 12.572 | ja | 12.572 | ja | 4.096 |
| GIF, BMP | 2.781 / 12.426 | „Unknown image format“ | 2.781 / 8.192 | nein | 4.096 |

Die Ursachen der Ganzdatei-Fälle stehen im Quelltext von `kamadak-exif` 0.6.1:

- **TIFF**: `read_from_container` (`src/reader.rs:138`) liest die ersten 4096 Bytes und hängt bei TIFF den Rest mit `reader.read_to_end(&mut buf)` an (`src/reader.rs:143`). `read_raw` (`src/reader.rs:107`) braucht den ganzen TIFF-Puffer, weil die IFD-Verweise absolute Versätze sind. Einen Weg, der springt, bietet die Kiste nicht. Das gilt für jede TIFF-Datei, gleich wo ihr Exif-IFD liegt.
- **JPEG ohne Exif**: der Segmentlauf sucht mit `read_until(marker::P, …)` (`src/jpeg.rs:72`) auch durch die Scandaten bis `EOI`. Mit Exif endet er am APP1-Segment, also vor den Scandaten.
- **PNG**: jeder Block, der nicht `eXIf` ist, wird mit `discard_exact` überlesen (`src/png.rs:73`), also auch alle `IDAT`-Blöcke. Liegt `eXIf` davor, endet der Lauf dort.
- **HEIF** springt dagegen: fremde Boxen auf Dateiebene mit `seek(SeekFrom::Current(…))` (`src/isobmff.rs:158`), das Exif-Element mit `seek(SeekFrom::Start(off))` (`src/isobmff.rs:206`). Deshalb bleiben alle drei HEIC-Anordnungen unter 28 KiB.

Befund je Format für den gewählten Weg (`kamadak-exif`, Begrenzer 256 KiB, siehe (f)):

| Endung | Befund | Beleg |
|---|---|---|
| `jpg`, `jpeg` | **liest** (mit Exif ≤ 8.192 B); ohne Exif endet der Lauf an der Grenze und das Foto bekommt das Änderungsdatum | Tabelle oben, Zeilen 1 bis 3 |
| `heic`, `heif` | **liest** in allen drei Anordnungen (≤ 27.365 B) | Zeilen 8 bis 11 |
| `png` | **liest**, wenn `eXIf` vor `IDAT` steht (8.192 B); steht es dahinter oder fehlt es, endet der Lauf an der Grenze: **Änderungsdatum**, begrenzt. ImageIO liest `eXIf` hinter `IDAT` ebenfalls nicht | Zeilen 4 bis 7 |
| `tif`, `tiff` | **Stop für dieses Format** (Haltepunkt 2): `kamadak-exif` gibt das Datum nur um den Preis der ganzen Datei her. Unter der Grenze liest es kleine TIFF-Dateien; eine Foto-TIFF liegt darüber und bekäme ohne Nutzerentscheid still das Änderungsdatum | `src/reader.rs:143`, Zeilen 12 bis 14 |
| `gif`, `bmp`, `icns` | **Änderungsdatum, keine Öffnung** (kein EXIF-Träger im Sinn des Spec) | `kamadak-exif`: „Unknown image format“ |

Der Stop bei TIFF liegt am Leser und nicht am Format. ImageIO liest dieselbe 48-MB-Datei mit 16.384 berührten Bytes, weil es zu IFD0 am Ende und zum Exif-IFD bei Versatz 8 springt. Der Spec fasst Haltepunkt 2 als Eigenschaft des Formats („grundsätzlich nicht ohne das Lesen der ganzen Datei“). Wir benennen TIFF trotzdem als Stop, weil der Plan in (b) nach dem Kandidaten fragt und weil die Folge für den Nutzer dieselbe ist: ohne Entscheid fiele eine Foto-TIFF still auf das Änderungsdatum.

`nom-exif` ist zum Vergleich mitgemessen, obwohl es ausscheidet. Es liest das große PNG ganz, auch mit `eXIf` vor `IDAT` (42.359.714 Bytes), und findet in der HEIC-Anordnung „`meta` hinter `mdat`“ nach 12.794.466 Bytes kein Datum.

### (c) Zeit je Foto, warm

Gemessen auf Intel i9-9880H, Mittel über 200 Wiederholungen, jede mit eigenem Öffnen der Datei; Datei im Seitencache.

| Datei | `kamadak-exif`, Grenze 256 KiB | ImageIO über `objc2-image-io` |
|---|---:|---:|
| JPEG mit Exif, 11,9 MB | 21 µs | 310 µs |
| JPEG ohne Exif | 173 µs (bis zur Grenze) | 203 µs |
| PNG mit `eXIf` vor `IDAT`, 42 MB | 18 µs | 509 µs |
| PNG ohne `eXIf` | 67 µs (bis zur Grenze) | 764 µs |
| HEIC mit Exif, 12,8 MB | 25 µs | 1.148 µs |
| HEIC, `meta` hinter `mdat` | 27 µs | 1.130 µs |
| `Sonoma.heic`, 20 MB | 18 µs | 1.359 µs |
| TIFF, 48,8 MB | 84 µs (bis zur Grenze, ohne Datum) | 492 µs |

Hochgerechnet auf eine Gruppe von 100 Fotos, warm: mit `kamadak-exif` rund 2 bis 3 ms bei JPEG und HEIC mit Exif und höchstens rund 17 ms, wenn kein Foto ein Exif trägt. Mit ImageIO sind es rund 31 ms bei JPEG und rund 115 ms bei HEIC. Kalt, also mit Lesen von der SSD, lag ImageIO je Datei bei 0,5 bis 8 ms (Messlauf mit `F_NOCACHE`-Kopien, Spalte `kalt_us`). Für `kamadak-exif` fehlt ein Kaltwert; der Zähler begrenzt es auf höchstens 256 KiB je Foto.

### (d) Lage im Baum

**`kamadak-exif` läuft im Kern.** Es braucht kein `unsafe`, keine AppKit-Klasse und keinen Hauptfaden. Der Leser `krk_core::bild::aufnahmedatum` kann die Datei über `verzeichnis::sys::ohne_warten_oeffnen` öffnen, den Typ am Deskriptor fragen und die `File` in den Begrenzer und dann in einen `BufReader` geben. `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` ändert ihre Erwartung nicht. Das Datum kommt aus `Field::value` als `Value::Ascii` und wird mit `exif::DateTime::from_ascii` (`src/tiff.rs:357`) zerlegt; `display_value()` liefert nur Anzeigetext. Proben mit echten Prüfbildern laufen in `crates/krk-core/tests/bild.rs`, ohne Fenster.

**ImageIO** bräuchte `unsafe`: `CGImageSource::with_url` und `properties_at_index` sind `unsafe fn`, und das Auslesen des Wörterbuchs geht über `CFDictionary::value` mit rohem Zeiger. Der Ort wäre also eine neue Datei unter `crates/krk-ui/src/appkit/` mit dem Abschnitt „Ab welchem macOS die angesprochenen Klassen stehen“ (`CGImageSourceCreateWithURL`, `CGImageSourceCopyPropertiesAtIndex` und die zwei Schlüssel stehen seit macOS 10.4). Der Kern bekäme die Leserfunktion über `Datumsleser` eingespritzt. Den Begrenzer könnte KRK dort nicht halten, denn ImageIO öffnet und bildet selbst ab. Ein Weg über `CGImageSourceCreateWithData` auf ein selbst abgebildetes `CFData` würde KRKs Öffnungsregel wahren (gemessen: ≤ 32.768 berührte Bytes). Er bräuchte aber eine eigene `mmap`-Deklaration, und eine Grenze bliebe auch dann eine Beobachtung und keine Zusage.

### (e) Prüfbilder für Schritt 4

Die kleinen Prüfbilder sind 64 × 48 Pixel groß und haben einen Verlauf als Inhalt, `DateTimeOriginal` = `2008:03:14 15:09:26`. Sie entstehen in zwei Stufen, und keine davon gehört in den Bau.

1. **Ein Swift-Werkzeug über ImageIO** (`swiftc -O erzeugen.swift`): ein `CGContext` 64 × 48 wird gefüllt, dann schreibt `CGImageDestinationCreateWithURL` je Format einmal mit `[kCGImagePropertyExifDictionary: [kCGImagePropertyExifDateTimeOriginal: "2008:03:14 15:09:26"]]` als Eigenschaften und einmal mit leeren Eigenschaften. Die Typen sind `UTType.jpeg`, `.heic`, `.png`, `.tiff`, `.gif` und `.bmp`. ICNS schreibt ImageIO aus 64 × 48 nicht; die ICNS entsteht mit `sips -z 128 128` und `sips -s format icns`.
2. **Byteumbauten in Python** (Standardbibliothek, `struct`):
   - JPEG ohne Exif: das APP1-Segment mit Kennung `Exif\0\0` entfernen.
   - PNG ohne XMP: den Block `iTXt` entfernen, denn ImageIO schreibt dort ein XMP mit demselben Datum, und ImageIO läse es.
   - PNG „`eXIf` hinter `IDAT`“: den Block `eXIf` vor `IEND` verschieben. Die CRC bleibt gültig, weil sie nur Typ und Daten des Blocks deckt.
   - HEIC „Exif am Ende“: die Exif-Bytes ans Dateiende hängen, die Länge von `mdat` erhöhen (bei ImageIO ist das ein 64-Bit-`largesize`) und im `iloc` den Versatz des Exif-Elements auf die alte Dateilänge setzen.
   - HEIC „`meta` hinter `mdat`“: die Boxen zu `ftyp`, `mdat`, `meta` umordnen und jeden Versatz mit `construction_method` 0 im `iloc` um die Länge von `meta` verringern.

   Jede Variante ist mit `sips -g pixelWidth` gegengeprüft (liefert 64 bzw. 4032).

| Datei | Bytes | SHA-256 (Anfang) | erwartet |
|---|---:|---|---|
| `klein-mit-datum.jpg` | 1.236 | `9cf29929a364f72e` | 2008-03-14 15:09:26 |
| `klein-ohne-datum.jpg` (Exif ohne `DateTimeOriginal`) | 1.146 | `68be3cc32eeeb241` | kein Datum |
| `klein-ohne-exif.jpg` | 1.138 | `4b492cc8c345204c` | kein Datum |
| `klein-mit-datum.heic` | 3.219 | `a66f5ff4a587e23a` | 2008-03-14 15:09:26 |
| `klein-ohne-datum.heic` | 612 | `9bfd9cda995ee00f` | kein Datum |
| `klein-exif-am-ende.heic` | 3.293 | `e740e71ad1e3e0c6` | 2008-03-14 15:09:26 |
| `klein-meta-hinter-mdat.heic` | 3.219 | `bad6e5f25c8d7b18` | 2008-03-14 15:09:26 |
| `klein-mit-datum-ohne-xmp.png` | 317 | `514b02b838828792` | 2008-03-14 15:09:26 |
| `klein-ohne-datum.png` | 285 | `522cebcb3a57538b` | kein Datum |
| `klein-exif-hinter-idat-ohne-xmp.png` | 317 | `ead0892905965774` | mit `kamadak-exif` 2008-03-14 15:09:26, weil die Datei unter der Grenze liegt |
| `klein-mit-datum.tif` | 12.572 | `66b2f26f744d2eb9` | 2008-03-14 15:09:26, weil die Datei unter der Grenze liegt |
| `klein-ohne-datum.tif` | 12.498 | `2ab7cc8dcbaa03d8` | kein Datum |
| `klein-mit-datum.gif` | 2.781 | `19518ae484fd336e` | keine Öffnung |
| `klein-mit-datum.bmp` | 12.426 | `a7a5bdfd969d3f96` | keine Öffnung |
| `klein-ohne-datum.icns` | 1.514 | `5454c389f57b6bf2` | keine Öffnung |

Die Grenzfälle brauchen keine Prüfdatei, weil die Probe sie zur Laufzeit in einen `Wegwerfordner` schreibt. `FF D8` gefolgt von 1 MiB Nullen liest `kamadak-exif` bis zur Grenze und meldet dann den Fehler des Begrenzers (gemessen: 262.144 Bytes, „Lesegrenze erreicht“). Ebenso schreibt die Probe eine TIFF über der Grenze. Beide Wegwerfwerkzeuge liegen im Scratchpad unter `exif-probe/gen/` (`erzeugen.swift`, `varianten.py`, `ohne.py`). Für Schritt 4 genügen die Tabelle und diese Beschreibung, denn ins Repository gehören die 15 Dateien und nicht ihre Erzeuger.

### (f) `HOECHSTENS_BYTES_JE_FOTO`

**Wir empfehlen 256 KiB (262.144 Bytes)**, den Vorschlag des Plans. Der größte gemessene Bedarf eines nicht gestoppten Formats liegt bei 27.365 Bytes (HEIC, `meta` hinter `mdat`, samt Nachlesen des `BufReader` nach dem Sprung). Ein JPEG-APP1-Segment ist höchstens 65.535 Bytes lang, und davor stehen gewöhnlich APP0 und selten ein ICC-Profil in APP2. 256 KiB tragen beides mit Abstand. Die Grenze ist zugleich der Preis eines Fotos ohne Datum: gemessen 67 bis 173 µs warm und höchstens 256 KiB von der Platte. Eine Gruppe von 100 Fotos ohne Exif kostet damit höchstens 25 MiB Lesen.

## Implications

Der Plan kann mit `kamadak-exif` im Kern weitergehen, wie er für den Fall „Rust-Kiste“ gebaut ist. Schritt 4 übernimmt die Fassung `=0.6.1`, den Begrenzer und die Prüfbilder aus (e). Der Kommentarblock in der Wurzel-`Cargo.toml` nennt `mutate_once` als einzige mitgebrachte Kiste und führt die Wendung „Namen auf `-sys`“.

Die Wahl hat einen Preis, und er fällt auf TIFF. Mit `kamadak-exif` bekommt eine Foto-TIFF über 256 KiB das Änderungsdatum. Nach Haltepunkt 2 entscheidet darüber der Nutzer, bevor Schritt 4 TIFF wie ein Format ohne EXIF behandelt. Die Arbeit hält dafür nicht an: TIFF ist in Jahres- und Monatsordnern selten (Scans, Nachbearbeitungen), JPEG und HEIC tragen die Folge, und der Rückfall auf das Änderungsdatum ist in C2.3 ohnehin vorgesehen.

Wir haben ImageIO gegen `kamadak-exif` abgewogen. ImageIO liest jedes Format mit höchstens 32 KiB berührten Bytes, TIFF eingeschlossen, und hätte keinen Format-Stop. Der Preis wäre dreifach. Die Lesegrenze aus C5 wäre keine Zusage mehr, sondern eine Beobachtung an Apples Implementierung, und die Probe „ein Leser, der über die Grenze greifen müsste, liefert `None`“ aus Schritt 4 ließe sich nicht bauen. Der Leser läge in `krk-ui/src/appkit/`, mit `unsafe` und mit Proben, die nur im Prüfmodul einer Binärkiste laufen. Und je Foto kostete er 12- bis 60-mal so viel Zeit, bei HEIC rund 1,1 ms statt 25 µs. Das berührt Haltepunkt 3 unmittelbar, weil das erste Foto erst nach dem Ordnen der ersten Gruppe erscheint. Eine Mischung, also `kamadak-exif` für JPEG, PNG und HEIC und ImageIO für TIFF, wäre zwei Mechanismen für eine Frage. Genau diese Art von Sonderfall-Ausbreitung soll der Plan vermeiden, und wir empfehlen sie nicht.

## Recommendations

1. **Schritt 3 darf beginnen, sobald auch Schritt 2 `Go` meldet.** Schritt 4 baut den Leser mit `kamadak-exif =0.6.1` im Kern (`code-implementer`).
2. **Dem Nutzer vorlegen (Haltepunkt 2, TIFF):** Behält eine Foto-TIFF über 256 KiB ihr Änderungsdatum, oder soll der ganze Leser auf ImageIO wechseln, mit den Folgen aus den Implications? Bis zur Antwort behandelt Schritt 4 `tif` und `tiff` wie im Plan vorgesehen: sie lesen bis zur Grenze, darüber gilt das Änderungsdatum, und ein Kommentar verweist auf diesen Bericht.
3. **PNG mit `eXIf` hinter `IDAT` braucht keine Nutzerfrage.** Der Begrenzer hält die Kosten, der Rückfall ist C2.3, und ImageIO verhält sich dort genauso.
4. In Schritt 4 den Zähler ausdrücklich **unter** den `BufReader` legen, damit das Nachlesen nach einem `seek` mitzählt. So ist die Grenze an den Bytes gemessen, die von der Platte kommen.

## Filed Issues

- keine. Die TIFF-Frage ist ein Haltepunkt des Spec und kein Defekt. Sie geht über den Orchestrator an den Nutzer.

## Sources

- Plan `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md`, Schritt 1, Entscheidung 3, Schritt 4
- Spec `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`, `## Stops when`, `## Constraints`, C2, C5
- `CLAUDE.md`, Absatz zur C-Freiheit; Wurzel-`Cargo.toml`; `Cargo.lock` Zeilen 94, 100, 1753; `rust-toolchain.toml`
- `kamadak-exif` 0.6.1: `src/reader.rs:107`, `:138`, `:143`; `src/jpeg.rs:72`; `src/png.rs:73`; `src/isobmff.rs:158`, `:206`; `src/tiff.rs:357`
- `nom-exif` 3.8.0: `Cargo.toml`, `[dependencies.chrono]`
- `objc2-image-io` 0.3.2: `src/generated/CGImageSource.rs` (`with_url`, `properties_at_index`, beide `unsafe fn`); `objc2-core-foundation` 0.3.2 `src/generated/CFDictionary.rs:759`
- Messwerkzeuge im Scratchpad `exif-probe/`: `k/src/main.rs`, `n/src/main.rs`, `i/src/main.rs`, `zaehler.rs`, `gen/erzeugen.swift`, `gen/imageio.swift` (Datenanbieter), `gen/kalt.swift` (`F_NOCACHE` und `mincore`), `gen/spion.c` (Protokoll der Datei-E/A, nur zum Messen), `gen/aufbau.py`, `gen/heif.py`, `gen/varianten.py`, `gen/ohne.py`
- `/System/Library/Desktop Pictures/Sonoma.heic` (nur gelesen)

## Open Questions

- [ ] Haltepunkt 2, TIFF: bleibt eine Foto-TIFF über 256 KiB beim Änderungsdatum, oder wechselt der ganze Leser auf ImageIO mit den Folgen aus den Implications?
- [ ] Die Zeiten stammen vom Intel-Entwicklungsgerät. Auf dem Referenzgerät sind sie ungemessen, und Haltepunkt 3 bleibt Nutzerarbeit.

Urteil: Go (kamadak-exif =0.6.1, im Kern; tif/tiff: Stop für dieses Format)
