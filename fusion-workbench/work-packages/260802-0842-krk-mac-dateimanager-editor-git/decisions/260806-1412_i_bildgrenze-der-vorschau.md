# Bis zu welcher Größe zeigt die Vorschau ein Bild, und wo steht diese Grenze?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1329_o_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-0834_c_die-vorschau-liest-bilddateien-ohne-groessengrenze-vollstaendig-in-den-speicher.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C6, C10), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S19)

---

## Question

Die Vorschau aus C6 las bis zum 260806 jede ausgewählte Bilddatei vollständig in den Arbeitsspeicher, gleich wie groß sie war. Zwanzig Tastendrücke Pfeil-ab über 40 Bilddateien zu je 65 MB kosteten dabei gemessene 438 MB; der Defekt `issues/260806-0834_*_die-vorschau-liest-bilddateien-ohne-groessengrenze-vollstaendig-in-den-speicher.md` hat ihn gemeldet, und die Behebung hat eine Größengrenze von 64 MB eingezogen, oberhalb derer die Datei als Metadaten erscheint. Dieselbe Grenze gilt seit dem Nachzug auch für ein Bild aus der Zwischenablage (C10).

Die Grenze selbst ist sachlich unstrittig. Strittig war, **welche Zahl gilt und wo sie steht**: der Code führte sie, während Spec, Plan und Entscheidungsspeicher sie nicht kannten. C6 sagte den gängigen Bildformaten ihren Inhalt ohne jeden Vorbehalt zu, und ein TIFF von 200 MB ist ein gängiges Bildformat. Eine Abnahme von C6 am laufenden Bündel wäre damit angreifbar gewesen: das Kriterium stand so da, dass es verfehlt ist. Gemeldet als `issues/260806-1329_*_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md`.

Für Text bestand dieses Problem nie. Der Vorbehalt steht dort seit dem 260802-1036 in der Zeitzusage L7 aus C8, und der Modulkopf von `vorschaumodell.rs` beruft sich zu Recht darauf.

## Options

1. **Die Grenze kommt in das Abnahmekriterium von C6, neben die Textgrenze und in derselben Form.**
   - Pros: der Ort, an dem der Widerspruch entstanden ist, und damit der Ort, an dem er verschwindet. C6 beschreibt danach beide Fälle einheitlich: Text bis 1 MB, Bilder bis 64 MB, darüber jeweils die Metadaten. Die Textgrenze wird dabei zum ersten Mal auch in C6 sichtbar, statt nur in einer Zeitzusage zu stehen, die sie nebenbei mitführt.
   - Cons: eine Zahl mehr im Spec, die der Nutzer verantwortet.
2. **Eine elfte Zeitzusage in C8, nach dem Muster von L7.**
   - Pros: L7 trägt die Textgrenze schon, die Symmetrie wäre auf den ersten Blick da.
   - Cons: C8 sagt Zeiten zu und keine Größen; L7 nennt die Textgrenze nur als Bedingung dessen, was in 100 ms sichtbar sein soll. Eine elfte Zahl entstünde für eine Aussage, die keine Zeit ist, und die zehn Zusagen sind vom Nutzer als geschlossene Menge bestätigt.
3. **Nur ein Entscheidungsdatensatz, der Spec bleibt unberührt.**
   - Pros: kein Eingriff in ein Dokument, dessen Fähigkeiten fast vollständig abgenommen sind.
   - Cons: das Abnahmekriterium von C6 bliebe verfehlt formuliert, und die Abnahme der Runde hinge an einem Datensatz, den sie nicht liest. Der Defekt bestünde fort.

## Constraints

- Die Grenze muss die Bilder, die der Nutzer im Alltag ansieht, von den Ausreißern trennen. Ein Bildschirmfoto eines Retina-Schirms liegt bei wenigen MB, ein Kamera-JPEG unter 20 MB, ein HEIC darunter, während ein TIFF- oder PSD-Export leicht über 100 MB wiegt.
- Die Textgrenze von 1 MB auf Bilder anzuwenden scheidet aus: sie schlösse jedes gewöhnliche Foto von der Anzeige aus und bräche die Zusage von C6.
- Ohne jede Grenze wird eine beliebig große Datei vollständig gelesen, und genau das war der behobene Defekt.
- Die Maxime "supersimpel" schließt eine zweite Regel mit eigenem Rückfallweg aus. Beide Grenzen führen auf dieselbe Antwort, die Metadaten, und die Prüfung steht in beiden Fällen vor dem Lesen.
- Keine der zehn Zahlen aus C8 darf sich ändern. L7 misst eine Textdatei.

## Recommendation

Möglichkeit 1.

---
Answered: Nutzer am 260806 — Möglichkeit 1, und die Zahl 64 MB ist bestätigt. Ausformuliert steht die Antwort im Spec `planning/260802-1036_o_spec-navigator-geruest.md`, Fähigkeit **C6**: das fünfte Abnahmekriterium lautet seither "Textdateien und Markdown-Dateien bis 1 MB erscheinen mit ihrem Inhalt, die gängigen Bildformate bis 64 MB. Oberhalb ihrer Grenze erscheint die Datei als Metadaten, so wie das folgende Kriterium sie für alles Übrige beschreibt.", und die Festlegung darunter trägt die Herleitung der Zahl, die Messung und den Grund, aus dem die Prüfung vor dem Lesen steht.

Berührt sind daneben zwei Stellen. Die Beschreibung von C6 nennt neben dem, was sich nicht darstellen lässt, jetzt auch das, was über seiner Größengrenze liegt. C10 verweist für die Zwischenablage auf dieselbe Grenze und schreibt aus, was dort an die Stelle des Bildes tritt: ein Satz mit seiner Größe und der Grenze, weil ohne Datei keine Metadaten vorliegen und "zu groß" nicht dasselbe ist wie "leer".

Der Umfang der Runde wächst nicht, und keine der zehn Zahlen aus C8 ist berührt.

---
Implemented: `fd5e3c5` (`crates/krk-ui/src/vorschaumodell.rs`, Konstante `BILDGRENZE = 64 * 1024 * 1024`, geprüft in `laden` vor `std::fs::read`, darüber der vorhandene Rückfall auf `Inhalt::Metadaten`) und `3c69ae4` (`crates/krk-ui/src/appkit/zwischenablage.rs`, dieselbe Konstante vor `to_vec()`, darüber `Zwischenablageinhalt::BildZuGross` mit dem Satz aus `zu_gross_text`). Eine zweite Zahl entsteht an keiner der beiden Stellen; `const _: () = assert!(BILDGRENZE > TEXTGRENZE);` sichert das Verhältnis der beiden Grenzen beim Übersetzen.

Code und Spec sind am 260806-1412 nachgesehen und deckungsgleich, deshalb steht dieser Datensatz auf `_i_` und nicht auf `_a_`. Der Plan trägt die Grenze in den `Änderungen` und im Abnahmekriterium von S19 sowie in einer Notiz am Schritt; S19 bleibt abgenommen, weil sich an der Umsetzung nichts ändert.
