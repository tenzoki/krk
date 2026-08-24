# Analyst: die vier Stellen in Spec und Plan nach der Räumung der acht Befunde

**Datum:** 2026-08-24 17:51
**Agent:** analyst
**Status:** Complete
**Circle:** 260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten
**Auftrag:** `issues/260824-1739_*_die-raeumung-der-acht-befunde-macht-vier-stellen-in-spec-und-plan-falsch.md`

## Was zu tun war

Die Räumung der acht Durchsichtsbefunde hat mit `942172b` `resources/default-readers.toml` an
zwei Stellen inhaltlich geändert. Vier Stellen in Spec und Plan beschrieben danach den Stand
davor. Spec und Plan gehören dem `analyst`, deshalb hat der `ontocoder` sie gemessen und benannt,
statt sie anzufassen.

## Was geändert ist

Fünf Änderungen in zwei Dateien, dazu der Schluss des Datensatzes.

| Datei | Zeile | Änderung |
|---|---|---|
| `planning/260824-0613_o_spec-…` | 4 | Statuszeile um die Berichtigung vom 260824-1751 ergänzt |
| `planning/260824-0613_o_spec-…` | 280 | C5.4 bekommt den Verweis auf die Berichtigung; der freigegebene Wortlaut bleibt |
| `planning/260824-0613_o_spec-…` | 294, 296 | zwei neue Absätze unter der Kriterienliste von C5: die fünf Zeilen des Defektspeichers, und die Zahl „54 offene von 82" als datierter Stand |
| `planning/260824-0640_o_plan-…` | 4 | Statuszeile fortgeschrieben |
| `planning/260824-0640_o_plan-…` | 229 | „achtzehn Orte" ersetzt durch neun Speichernamen, fünf je Runde, 99 getroffene Ordner als Stand |
| `planning/260824-0640_o_plan-…` | 235 | Profiltabelle: vier Zählungen statt zwei |
| `planning/260824-0640_o_plan-…` | 331 bis 337 | Nachtrag an Schritt 7, der Schritt selbst unverändert |

## Die Form, und warum sie zweigeteilt ist

Im Spec steht die Berichtigung **neben** dem freigegebenen Wortlaut, im Plan **an seiner Stelle**
mit der alten Fassung in der Klammer. Der Grund steht im Spec schon ausgeschrieben: der
freigegebene Wortlaut ist der Beleg dafür, was der Nutzer am Tor bestätigt hat. Der Plan trägt
keine Freigabe dieser Art und darf deshalb ersetzen.

**Schritt 7 des Plans ist nicht umgeschrieben worden.** Er steht auf `[DONE]`, und sein Wortlaut
hält den gebauten Stand vom 260824-1313 fest. Eine Anweisung, die die heutige Datei nicht erfüllt,
entstünde erst durch das Umschreiben. Dieselbe Überlegung steht seit dem 260824-1508 in
`planning/260824-0640_o_plan-…:245` für die vierte Zustandszeile.

## Was zusätzlich gefunden ist

**Eine fünfte Stelle**, dieselbe Räumung, derselbe Planschritt: `:327` schreibt die jüngsten zehn
auf `ordner = "history"` ohne Muster, und die Zeile trägt seit `942172b` `muster = '\.md$'`. Sie
steht als zweiter Punkt im Nachtrag und hat keinen eigenen Datensatz bekommen.

**Eine sechste, die kein Bauauftrag ist:** „54 offene von 82" in C5.4 ist seit `942172b` „55 von
83", weil derselbe Commit einen weiteren offenen Datensatz im gemeinsamen Speicher angelegt hat.
Der Plan führt diese Zahlen ausdrücklich als Stände vom 260824. Der Wortlaut bleibt stehen, der
Spec sagt es jetzt dazu.

**Bewusst stehen geblieben:** die Beschreibung von B4 im Spec (`:192`) nennt den dritten Ausgang
des Bausteins nicht, den der Kommentar der Auslieferungsfassung bekommen hat. Sie beschreibt
keinen überholten Stand, und C3.12 deckt den Platzhalter für alle vier Bausteine ab.

## Nicht angefasst

`crates/` und `resources/` sind gelesen und nicht geändert. Kein Commit, kein baumweites
git-Kommando; die Umbenennung des Datensatzes ist ein `git mv`.
