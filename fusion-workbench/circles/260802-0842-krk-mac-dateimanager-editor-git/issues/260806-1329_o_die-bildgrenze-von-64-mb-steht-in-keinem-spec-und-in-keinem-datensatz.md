Die Bildgrenze von 64 MB steht in keinem Spec und in keinem Datensatz

---

`crates/krk-ui/src/vorschaumodell.rs:84` führt `BILDGRENZE = 64 MB` ein, und
`laden` weist darüber jede Bilddatei auf die Metadaten zurück
(`vorschaumodell.rs:471`). Die Zahl schränkt ein Abnahmekriterium von C6 ein,
ohne dass Spec, Plan oder ein Entscheidungsdatensatz sie kennen.

---

**Der Widerspruch im Wortlaut.** C6 zählt als fünftes Abnahmekriterium auf
(`planning/260802-1036_o_spec-navigator-geruest.md:288`):

> - [ ] Textdateien, Markdown-Dateien und die gängigen Bildformate erscheinen
>   mit ihrem Inhalt.

Ohne Vorbehalt. Ein TIFF von 200 MB ist ein gängiges Bildformat und erscheint
seit `fd5e3c5` nicht mehr mit seinem Inhalt.

**Warum die Textgrenze denselben Einwand nicht trägt.** Für Text steht der
Vorbehalt im Spec, und zwar in der Zusage L7 aus C8
(`260802-1036_o_spec-navigator-geruest.md:337`):

> | L7 | Vorschau einer Textdatei bis 1 MB sichtbar, sonst die Metadaten | 100 ms |

Der Modulkopf von `vorschaumodell.rs` beruft sich zu Recht darauf ("das
Abnahmekriterium des Schritts laesst beide Wege zu"). Für Bilder gibt es keine
solche Zeile. Der Modulkopf nennt seit `fd5e3c5` beide Grenzen als "dieselbe
Regel mit zwei Zahlen" — im Spec ist die eine belegt und die andere nicht.

**Geprüft.** `grep -rn "64 MB\|BILDGRENZE\|Bildgrenze"` über den ganzen
Circle-Ordner findet die Zahl allein im Historieneintrag des Coders
(`history/260806-1240-coder-vorschau-und-ui-defekte.md:27`). Weder Spec noch
Plan noch `decisions/` nennen sie.

**Was zu tun ist, und was der Coder nicht allein entscheiden darf.** Die Grenze
selbst ist sachlich richtig: ohne sie liest die Vorschau eine beliebig große
Datei vollständig in den Speicher, und genau das war der behobene Defekt. Zu
klären ist nicht *ob*, sondern *wo es steht* und *welche Zahl es ist*:

1. C6 bekommt den Vorbehalt in sein Abnahmekriterium, so wie L7 ihn für Text
   trägt, und die Zahl wird dabei vom Nutzer bestätigt. Der Vorschlag im Code
   (64 MB, hergeleitet aus Bildschirmfoto, Kamera-JPEG, HEIC gegen TIFF und
   PSD) ist nachvollziehbar, aber nicht abgenommen.
2. Ohne diesen Nachzug bleibt eine Abnahme von C6 am laufenden Bündel
   angreifbar: das Kriterium steht so da, dass es verfehlt ist.

**Betrifft:** `krk-ui` (`vorschaumodell.rs`) und das Spec-Dokument. Keine
Zeitzusage aus C8 berührt; L7 misst eine Textdatei.
