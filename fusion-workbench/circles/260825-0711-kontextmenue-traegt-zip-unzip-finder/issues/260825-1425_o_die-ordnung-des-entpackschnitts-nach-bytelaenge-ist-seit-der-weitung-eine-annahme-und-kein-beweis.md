Die Ordnung des Entpackschnitts nach Bytelaenge ist seit der Weitung eine Annahme und kein Beweis

---

`ohne_die_eigenen_ziele` (`crates/krk-ui/src/kommandos/kontextmenue.rs`) entscheidet vom laengsten
Archivpfad zum kuerzesten und ordnet dafuer nach Bytelaenge. Die vierte Durchsicht hat den
Festpunkt ueber alle Eingabereihenfolgen von drei bis fuenf gestaffelten Archiven nachgerechnet;
jener Beweis ruhte darauf, dass `eq_ignore_ascii_case` nur gleich lange Bytefolgen trifft. Seit
`F-5` faltet der Vergleich ueber `to_lowercase()`, und Unicode-Kleinschreibung kann die Bytelaenge
aendern. Damit ist die Ordnung eine begruendete Annahme und kein Beweis mehr.

---

**Filed by:** orchestrator, aus dem Bericht zu `F-5`

## Was gemessen ist

Ueber alle Codepunkte erhoben: gut zwei Dutzend Zeichen verlieren beim Kleinschreiben Bytes, das
Kelvinzeichen `K` (`U+212A`) als einziges zwei, drei Zeichen gewinnen eines. Der Vorsprung von vier
Bytes, den ein angehaengtes `.zip` gibt, traegt bis zu zwei solchen Zeichen im Namen; ab dreien
kehrt sich die Ordnung um.

## Die Folge, wenn es eintritt

Ein Archiv faellt aus dem Lauf, dessen Beansprucher selbst schon gefallen ist — genau der Befund
`260825-1249_c_der-entpackschnitt-ist-kein-festpunkt-…`, in einem Fall, den seine Behebung nicht
mehr deckt. Der Nutzer bekommt dann einen Zielordner weniger als erwartet, und die Meldung nennt
den ausgelassenen Eintrag.

## Wie erreichbar

Drei Zeichen aus jener kleinen Menge im selben Dateinamen, dazu die gestaffelte Form `a.zip`,
`a.zip.zip`. Konstruierbar, im Alltag nicht zu erwarten.

## Vorschlag

Der Doc-Kommentar von `ohne_die_eigenen_ziele` nennt den Weg, der die Grenze schloesse; gebaut ist
er nicht. Wer ihn baut, prueft zugleich, ob die Ordnung ueberhaupt an der Laenge haengen muss.

## Umfang

`krk-ui`, `kommandos/kontextmenue.rs`, `ohne_die_eigenen_ziele`.
