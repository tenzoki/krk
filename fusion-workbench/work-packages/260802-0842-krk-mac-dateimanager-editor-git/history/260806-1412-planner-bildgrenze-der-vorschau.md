# Planner: die Bildgrenze von 64 MB steht jetzt im Spec

**Datum:** 260806-1412
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`

## Auftrag

Der Nutzer hat am 260806 bestimmt, dass die Vorschau Bilder bis 64 MB mit ihrem Inhalt zeigt und darüber die Metadaten, so wie sie es für Textdateien über 1 MB schon tut. Die Grenze stand seit dem 260806-1240 im Code und in keinem Spec und keinem Datensatz; genau das meldet `issues/260806-1329_o_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md`. Der Nutzer hat sich für die Aufnahme in den Spec entschieden. Zu ändern waren allein Workbench-Dokumente, keine Code-Datei; die Marker von Spec und Plan bleiben `_o_`, und die Defektdatei bleibt unberührt, weil der Nutzer sie selbst schließt.

## Was geändert wurde

**Spec `planning/260802-1036_o_spec-navigator-geruest.md`, vier Stellen.** Das fünfte Abnahmekriterium von C6 sagte den gängigen Bildformaten ihren Inhalt ohne jeden Vorbehalt zu und lautet jetzt: "Textdateien und Markdown-Dateien bis 1 MB erscheinen mit ihrem Inhalt, die gängigen Bildformate bis 64 MB. Oberhalb ihrer Grenze erscheint die Datei als Metadaten, so wie das folgende Kriterium sie für alles Übrige beschreibt." Die Beschreibung von C6 nennt neben dem, was sich nicht darstellen lässt, jetzt auch das, was über seiner Größengrenze liegt. Eine neue Festlegung unter C6 trägt die Herleitung der Zahl, die Messung vom 260806 und den Grund, aus dem die Prüfung vor dem Lesen steht. Der Kopf bekommt einen Stand-Block zum 260806-1412.

**Die Textgrenze steht damit zum ersten Mal auch in C6.** Bisher trug sie allein die Zeitzusage L7 aus C8, die sie als Bedingung dessen mitführt, was in 100 ms sichtbar sein soll. Genau darauf beruft sich der Modulkopf von `vorschaumodell.rs`, und genau diese Deckung fehlte für Bilder. C6 beschreibt beide Fälle jetzt in derselben Form, und C8 bleibt unberührt.

**C10 hat einen Halbsatz bekommen, und das war nicht im Auftrag genannt.** Das zweite Abnahmekriterium von C10 sagte "ein Bild erscheint als Bild" und wiederholte damit eine Etage tiefer denselben unqualifizierten Satz, den C6 abgelegt hat; die Grenze gilt seit `3c69ae4` auch auf dem Weg über die Zwischenablage. Das Kriterium verweist jetzt auf die Grenze aus C6 und schreibt aus, was dort an die Stelle des Bildes tritt: ein Satz mit seiner Größe und der Grenze, weil ohne Datei keine Metadaten vorliegen und "zu groß" nicht dasselbe ist wie "leer". Ohne diesen Zusatz wäre der behobene Defekt in C10 stehen geblieben.

**Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, S19 und der Kopf.** Der Plan musste nachgezogen werden: die `Änderungen` von S19 nannten "Textdateien bis 1 MB" und dann "Bilder über `NSImage`" ohne Zahl, führten die eine Grenze also und ließen die andere aus. Sie lauten jetzt "Bilder bis 64 MB über `NSImage`", und die Metadaten-Zeile nennt die beiden Dateisorten oberhalb ihrer Grenze mit. Das Abnahmekriterium bekommt neben der Textdatei von 5 MB den Bildfall. Eine Notiz am Schritt benennt den Nachtrag samt Messung und hält fest, dass S19 abgenommen bleibt, weil sich an der Umsetzung nichts ändert. Dazu ein Nachzug-Absatz im Kopf, nach dem Muster der neunzehn vorhandenen.

**Entscheidungsdatensatz `decisions/260806-1412_i_bildgrenze-der-vorschau.md`, Marker `_i_`.** Der Auftrag sah `_a_` vor und `_i_`, falls Code und Spec beim Nachsehen deckungsgleich sind. Sie sind es, deshalb `_i_`.

## Nachweise

- `crates/krk-ui/src/vorschaumodell.rs:95` führt `pub const BILDGRENZE: u64 = 64 * 1024 * 1024;`, `laden` prüft in Zeile 506 vor `std::fs::read` und kehrt darüber mit `Inhalt::Metadaten` zurück. Die Textgrenze steht in Zeile 83, die Prüfung in Zeile 517. `const _: () = assert!(BILDGRENZE > TEXTGRENZE);` in Zeile 100 sichert das Verhältnis beim Übersetzen.
- `crates/krk-ui/src/appkit/zwischenablage.rs:103` prüft dieselbe Konstante vor `to_vec()` und liefert darüber `Zwischenablageinhalt::BildZuGross`, dessen Satz beide Zahlen aus je einer Quelle nimmt.
- Die Herkunft der beiden Stellen ist mit `git log -S` bestimmt und nicht angenommen: `fd5e3c5` für den Dateiweg, `3c69ae4` für den Weg über die Zwischenablage.
- Die Messung stammt aus `issues/260806-0834_c_die-vorschau-liest-bilddateien-ohne-groessengrenze-vollstaendig-in-den-speicher.md` und dem Historieneintrag `history/260806-1240-coder-vorschau-und-ui-defekte.md`: zwanzig Tastendrücke Pfeil-ab über 40 Bilddateien zu je 65 MB, 438 MB ohne die Grenze, 54 MB mit ihr.
- Keine der zehn Zahlen aus C8 ist berührt. L7 misst eine Textdatei, und eine elfte Zusage entsteht nicht.
- Keine Code-Datei angefasst. Parallel arbeitete ein zweiter Agent an `crates/krk-ui/src/`; die drei Rust-Dateien sind allein gelesen worden.

## Was offen bleibt

Der Defekt `issues/260806-1329_o_...` bleibt unverändert und ungeschlossen liegen; der Nutzer schließt ihn selbst. Der Spec bleibt `_o_`, der Plan ebenfalls: Runde 1 schließt erst nach der Klärung von `decisions/260806-0014_o_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`.
