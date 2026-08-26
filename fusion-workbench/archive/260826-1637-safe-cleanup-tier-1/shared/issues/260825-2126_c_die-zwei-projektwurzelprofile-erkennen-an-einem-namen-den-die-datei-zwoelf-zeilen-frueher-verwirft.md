Die zwei Projektwurzelprofile erkennen an einem Namen, den die Datei zwölf Zeilen früher als untauglich verwirft

---

`resources/default-readers.toml:243-246` begründet, warum das Wurzelprofil einer fusion-Werkbank
über `.fusion-setup` erkennt und nicht über den Namen: „ein Pfadmuster auf `fusion-workbench`
träfe jeden Ordner dieses Namens, auch einen leeren, während `.fusion-setup` in einer
eingerichteten Werkbank steht und sonst nirgends." Das Profil „Projektwurzel mit
fusion-Werkbank" (`:605`) tut genau das Verworfene: `kennzeichen = '^fusion-workbench$'`. Es
greift an jedem Ordner, der einen Eintrag dieses Namens führt — auch an einem leeren
Verzeichnis und an einer gleichnamigen Datei —, und ersetzt dort die Metadatenanzeige durch
sieben Zeilen Platzhalter. Für flight (`:732`) gilt dasselbe.

---

**Filed by:** ontorev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `resources/default-readers.toml:243-246`, `:593-605`, `:649-659`,
`:721-732`, `:15-19` (die Zusage „Trifft keines zu, bleibt die Metadatenanzeige")

## Was gemessen ist

Gemessen am 260825-2126 über `leseprofil::zusammenfassen_gezaehlt`, Baum `8478753`, an
künstlichen Ordnern:

| Ordner enthält | Zusammenfassung |
|---|---|
| ein **leeres** Verzeichnis `fusion-workbench` | Profil „Projektwurzel mit fusion-Werkbank", alle sieben Zeilen `--` |
| eine **Datei** namens `fusion-workbench` | dasselbe, alle sieben Zeilen `--` |
| ein leeres Verzeichnis `flight-workbench` | Profil „Projektwurzel mit flight-Werkbank", alle sieben Zeilen `--` |
| ein Verzeichnis `fusion-workbench` **und** eines `flight-workbench` | das fusion-Profil, wie sein Kommentar es ansagt |

Der vierte Fall ist ausdrücklich dokumentiert (`:727-729`) und stimmt. Die ersten drei sind es
nicht.

## Warum das zählt

Die Datei verspricht dem Nutzer im Kopf: „Trifft keines zu, bleibt die Metadatenanzeige, wie
sie war." Sieben Zeilen `--` sind schlechter als die Metadatenanzeige und schlechter als kein
Profil: Größe, Änderungsdatum, Rechte und Typ sind weg, und was an ihre Stelle tritt, sagt
nichts. Ein leeres oder frisch angelegtes `fusion-workbench` ist keine Seltenheit — es ist der
Zustand vor `/fusion:setup`.

Der Kommentar des Projektwurzelprofils (`:595-597`) gibt eine eigene Begründung — „welcher
Ordner eine Projektwurzel ist, beantwortet der Nutzer damit hier, wo er es ändern kann" —, und
die ist für sich vertretbar. Was fehlt, ist die Verbindung zum Absatz zwölf Zeilen weiter
oben: dieselbe Erkennungsart wird an der einen Stelle verworfen und an der anderen gewählt,
und keine der zwei Stellen erwähnt die andere. Wer den Preis kennt, kann ihn zahlen; die Datei
nennt ihn nicht, obwohl sie den Preis der Doppelung zwei Absätze weiter ausdrücklich nennt.

## Möglichkeiten

1. **Der Kommentar nennt den Preis.** Ein Satz beim Projektwurzelprofil, der sagt, dass die
   Erkennung am Namen hängt, dass ein leeres `fusion-workbench` deshalb sieben Platzhalter
   liefert, und warum das in Kauf genommen ist. Das ist die kleinste Änderung und ändert kein
   Verhalten.
2. **Das Profil fällt.** Wer an der Projektwurzel steht, sieht die Metadaten und geht einen
   Ordner tiefer. Kostet die Auskunft, die dieses Profil hinzufügt.
3. **Der Mechanismus bekommt eine Erkennung über den Inhalt eines Unterordners.** Die
   Kennzeichendatei sieht heute allein auf die Namen der Einträge im ausgewählten Ordner. Das
   wäre die einzige Möglichkeit, die die Erkennung genauso streng macht wie die des
   Wurzelprofils, und die teuerste.

Möglichkeit 1 ist die, die diese Datei sich sonst gibt: die Doppelung der sieben Zeilen wird
mit demselben Mittel behandelt, nämlich mit einem Hinweis statt mit einem Mechanismus.

**Schwere:** mittel. Kein Bau hängt daran; an einem Ordner mit leerer Werkbank ist die
Vorschau ärmer als ohne jedes Profil.

---
Resolved: Möglichkeit 1, der Zuschnitt bleibt. Der Kommentar über „Projektwurzel mit fusion-Werkbank" sagt jetzt in Großschrift, dass dies die Erkennung ist, die das Wurzelprofil verwirft, dass ein `kennzeichen` nicht in den Ordner hineinsehen kann, dass ein leeres `fusion-workbench` (der Zustand vor `/fusion:setup`) oder eine Datei dieses Namens sieben Zeilen Platzhalter statt der Metadaten liefert, warum der Preis in Kauf genommen ist und wie man ihn abwählt; der flight-Block verweist darauf mit demselben Preis. Nachgemessen am 260825: leeres Verzeichnis `fusion-workbench` → sieben `--`, Datei `fusion-workbench` → sieben `--`, leeres `flight-workbench` → sieben `--`.
