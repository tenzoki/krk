# Schritt 6: die vier Bausteine, der Haushalt und die Regel über die Teillesung

**Datum:** 260824-1124 bis 260824-1150
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel B, Schritt 6
**Baumstand vorher:** die Schritte 1 bis 5 und 13 auf `[DONE]`

---

## Auftrag

Schritt 6 und sonst nichts: die Auswertung der vier Bausteine, der Einstieg
`zusammenfassen`, die Buchführung des Haushalts und die eine Regel darüber, was eine
unvollständige Verzeichnislesung sagen darf. Abnahmekriterien C3 und C6 des Specs.

## Was entstanden ist

### `crates/krk-core/src/leseprofil/bausteine.rs` (neu)

Der Einstieg `zusammenfassen(&Profile, &Path) -> Option<Zusammenfassung>` löst den Ordner
einmal über `std::fs::canonicalize` auf, ruft `erkennung::erkennen` mit einem Abschluss über
den einen gemerkten Leselauf und geht danach die Zeilen des erkannten Profils in
Dateireihenfolge durch. Die Struktur `Lauf` hält dabei drei Dinge zusammen: den aufgelösten
Ordner als Schranke aus C3.13, den `Haushalt` in einer `Cell` und den Leselauf über den
erkannten Ordner in einer `OnceCell`.

**Die Innenveränderlichkeit ist keine Bequemlichkeit.** `erkennen` nimmt die Einträge als
Abschluss entgegen, damit der Verzeichnisleselauf erst beim ersten Profil mit
Kennzeichendatei anfällt; ein `&mut self` wäre durch diesen Abschluss nicht zu reichen.

Die vier Bausteine, wie der Plan sie vorschreibt:

| Baustein | Leseläufe | Öffnungen | Was er liefert |
|---|---|---|---|
| `zaehlung` | 0 oder 1 | 0 | `Wert::Zahl`, abgeschnitten `Wert::UeberGrenze` |
| `juengste` | 0 oder 1 | N | `Wert::Titel`, abgeschnitten `Wert::Nicht` |
| `feld` | 0 oder 1 | 1 | `Wert::Text` der ersten Fanggruppe |
| `vorhandensein` | 0 oder 1 | 0 | `Wert::Vorhanden`, ohne Treffer und abgeschnitten `Wert::Nicht` |

Null Leseläufe stehen dort, wo die Ortsangabe leer ist: dann nimmt der Baustein den einen
Lauf, den die Erkennung ohnehin gebraucht hat. Damit fallen die Zahlen aus C6.7 genau so, wie
der Plan sie rechnet: das Profil des einzelnen Circles kostet fünf Läufe (erkannter Ordner,
`planning` zweimal, `decisions`, `history`) und elf Öffnungen.

### `crates/krk-core/src/leseprofil/mod.rs`

`pub mod bausteine;`, die Wiederausfuhr `pub use bausteine::zusammenfassen;`, `Wert::als_text`,
`Zusammenfassung::als_text`, die Konstante `PLATZHALTER = "--"` und die Einrückung der
Blockzeilen.

`Haushalt::oeffnung_nehmen()` ist zu `Haushalt::oeffnungen_nehmen(u32)` geworden und bucht
**ganz oder gar nicht**. Der Grund steht am Rumpf: der Baustein „jüngste N" braucht N
Öffnungen für eine Antwort, und passt die letzte nicht mehr hinein, ist die halbe Antwort
keine. Einzeln gebucht hätte er die ersten Öffnungen verbraucht und den Wert doch fallen
lassen, und die verbrauchten fehlten den Zeilen darunter. Die alte Fassung hatte keinen
Rufer; die Änderung erreicht nichts außerhalb dieses Schritts.

### `crates/krk-core/tests/leseprofil.rs`

Zehn neue Proben, alle gegen einen Prüfordner in der **Gestalt** einer Werkbank und
ausdrücklich nicht gegen die echte: deren Zahlen ändern sich mit jeder Sitzung. Sie decken
C3.1 bis C3.13, C4.2 und C4.3 sowie die drei Anwendungen der Teillesungsregel.

Der Änderungszeitpunkt der Verlaufsdateien wird über `File::set_times` gesetzt. Ohne ihn
hinge die Reihenfolge der jüngsten N daran, wie schnell die Probe läuft: vier nacheinander
geschriebene Dateien tragen auf einem schnellen Dateisystem denselben Zeitpunkt, und die
Probe prüfte dann den Zweitschlüssel statt der Sortierung.

## Die drei Entscheidungen, die im Plan offen standen

**1. Was `Wert::UeberGrenze` trägt: die Grenze oder das Gezählte.** Das Gezählte. Die
Anzeige „über 2.000" aus Festlegung A5 fällt daraus im ungefilterten Fall von selbst, während
eine Zählung mit Muster über einer abgeschnittenen Liste sonst „über 2.000" sagte, obwohl sie
drei Treffer gesehen hat. Der Doc-Kommentar des Wertes entscheidet es bereits: „gezählt sind
mehr Einträge, als die Zahl sagt."

**2. Eine Zählung von 0 ist eine Antwort und kein Platzhalter.** Der Ordner steht da und ist
gelesen, es trifft nur nichts darin; „0 offene Defekte" und „diesen Speicher gibt es nicht"
sind zwei verschiedene Auskünfte. Der Platzhalter bleibt dem zweiten Fall vorbehalten. Der
Entscheidungsdatensatz zum Platzhalter nennt als seine Fälle die fehlende Datei, den nicht
vorkommenden Präfix und den fehlenden Unterordner, und der gelesene leere Ordner ist keiner
davon.

**3. Wo ein mehrzeiliger Feldwert steht.** Unter seiner Beschriftung, eingerückt wie die
Titel. C4.3 nennt nur den Baustein „jüngste N" als Block, aber C3.9 verlangt vom Feldbaustein
einen Absatz, und einer der achtzehn Circle-Datensätze dieser Werkbank trägt seine Directive
auf vier Zeilen. Hinter der Beschriftung liefe er in die nächste Beschriftung hinein. Der
Defektdatensatz dazu ist unten genannt.

## Nachgemessen statt angenommen

**Die Naht des Deckels.** `anlesen` schneidet nach einer Zahl von Bytes ab und nicht nach
Zeichen. Fällt der Schnitt mitten in ein mehrbytiges Zeichen, wäre die ganze Datei nach
`String::from_utf8` „kein Text", obwohl ihre erste Zeile tadellos dasteht.
`bausteine::lesbarer_anfang` unterscheidet deshalb das unvollständige Zeichen **am Ende**
(`Utf8Error::error_len() == None`) vom ungültigen Byte **mitten** im Gelesenen; das erste
liefert den lesbaren Anfang, das zweite bleibt der Befund, der es ist. `verzeichnis::inhalt`
kennt die Unterscheidung nicht und braucht sie nicht: jene Stelle liest über
`bis_zur_grenze_lesen`, das eine zu große Datei abweist statt sie abzuschneiden.

**Zwei Muster aus Schritt 7 treffen nie.** Die Probe zu C3.9 hat das Feldmuster
`(?s)^## Directive\s*\n+(.+?)\n\n` aus Schritt 7 wörtlich übernommen und lief rot. `regex`
verankert `^` ohne die Angabe `m` am Anfang der ganzen Eingabe und nicht am Anfang einer
Zeile. Dasselbe trifft `^(.+)$` auf `.active-circle`, dort zusätzlich am abschließenden
Zeilenende. In einer Wegwerfprobe nachgemessen und wieder entfernt; der Defektdatensatz trägt
die vier Zeilen Ausgabe.

## Verifikation

```text
make check — exit 0
cargo test -p krk-core --test leseprofil — 23 Proben, 0 rot (13 aus den
  Schritten 3 und 5, 10 neu)
cargo test -p krk-core --lib — darin drei neue Proben im Pruefmodul von
  bausteine.rs (Titelregel, Naht des Deckels, Ganz-oder-gar-nicht des Haushalts)
```

Der erste Lauf von `make check` ist an `cargo fmt --all --check` gescheitert (zwei Stellen in
der Testdatei); nach `cargo fmt --all` grün.

## Gefilte Datensätze

- `issues/260824-1124_o_zwei-feldmuster-der-auslieferungsfassung-verankern-mit-dach-und-koennen-nie-treffen.md`
  — die zwei Muster aus Schritt 7, mit der Messung und den zwei Berichtigungen.
- `issues/260824-1124_o_c4-3-sagt-eine-zeile-je-profilzeile-und-c3-9-verlangt-einen-absatz.md`
  — der Wortlaut von C4.3 ist enger als die Anzeige, die C3.9 verlangt.
- `Also seen:` an `issues/260824-1014_o_c3-14-nennt-bis-zur-grenze-lesen-…` — Punkt 3 jener
  Liste ist entschieden: `bis_zur_grenze_lesen` hat in der Zusammenfassung keinen Rufer.
- `Also seen:` an drei gemeinsamen Defektdatensätzen zur Lastabhängigkeit
  (`shared/issues/260823-1210_o_…`, `260823-1436_o_…`, `260815-1019_o_…`): auf diesem Gerät
  laufen 22 verwaiste Endlosschleifen aus zwei Sitzungen vom 15. und 16.08., je bei rund 65
  Prozent eines Kerns. Sie sind die Grundlast, gegen die jene drei Datensätze gemessen haben.

## Was Schritt 6 nicht getan hat

- **Kein fünfter Baustein.** Der Satz ist fest, Nutzerentscheid vom 260823, Festlegung A7.
- **Keine Zählproben zu C6.** Die gehören Schritt 12. Der Einstieg ist an der Naht
  `zusammenfassen`/`gezaehlt` geteilt, damit jene Proben den Haushalt eines Laufs auslesen
  können, ohne eine zweite Zählstelle danebenzustellen; `gezaehlt` ist bis dahin privat.
- **Keine Zeile in `krk-ui`.** Der siebte Inhalt und der Anzeigezweig sind die Schritte 9
  und 10.
