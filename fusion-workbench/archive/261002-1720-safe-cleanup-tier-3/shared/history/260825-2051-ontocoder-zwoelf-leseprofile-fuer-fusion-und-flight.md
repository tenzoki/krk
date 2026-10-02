# Zwölf Leseprofile für fusion und flight

**Agent:** ontocoder
**Datum:** 2026-08-25, 20:44 bis 20:51
**Aufgabe:** T-8, Schritt 8 des Plans
`fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`,
Strang 3 — „Die Auslieferungsfassung führt acht Profile", vom Nutzer am 260825-2020
um einen zweiten Teil erweitert.
**Status:** Complete

## Was entstanden ist

`resources/default-readers.toml` führt statt fünf jetzt **zwölf** Profile: acht für eine
fusion-Werkbank und vier für eine flight-Werkbank. Die vier für flight sind die Abweichung
vom freigegebenen Plan, und sie ist vom Nutzer gewollt.

### Teil 1, die acht Profile des Plans

| Profil | Erkennung | Was neu ist |
|---|---|---|
| fusion-Werkbank: die Wurzel | `kennzeichen = '^\.fusion-setup$'` | unverändert, bis auf einen Hinweis im Kommentar |
| fusion-Werkbank: ein Speicher | Pfad | unverändert |
| fusion-Werkbank: ein Defektspeicher | Pfad | unverändert |
| fusion-Werkbank: alle Runden | `pfad = 'fusion-workbench/circles$'` | sechs Zustandszeilen und eine Defektzeile hinzu |
| fusion-Werkbank: der Ablagespeicher | `pfad = 'fusion-workbench/archive$'` | neu |
| fusion-Werkbank: der gemeinsame Speicher | `pfad = 'fusion-workbench/shared$'` | neu |
| fusion-Werkbank: eine Runde | `kennzeichen = '^_._circle\.md$'` | unverändert |
| Projektwurzel mit fusion-Werkbank | `kennzeichen = '^fusion-workbench$'` | neu |

Die sechs Zustandszeilen des Rundenverzeichnisses nennen alle `ordner = "*"` und teilen
sich damit einen Leselauf; die Defektzeile nennt `ordner = "*/issues"`. Sechs und nicht
vier, weil das Werkbankvokabular sechs Marker hat und dieses Projekt einen beschränkten
von einem kohärenten Abschluss ausdrücklich unterscheidet.

### Teil 2, die vier Profile für flight

Gelesen am Beispiel unter `/Users/k1/Projects/productive/example/`, das nur gelesen und
nicht angefasst wurde. Drei Unterschiede zu fusion bestimmen den Zuschnitt: keine Runden,
keine Zustandsmarker in den Dateinamen, und `.flight-setup` mit denselben drei Feldern wie
`.fusion-setup`.

| Profil | Erkennung |
|---|---|
| flight-Werkbank: die Wurzel | `kennzeichen = '^\.flight-setup$'` |
| flight-Werkbank: ein Speicher | `pfad = 'flight-workbench/(decisions\|history\|memos)$'` |
| flight-Werkbank: der Ablagespeicher | `pfad = 'flight-workbench/archive$'` |
| Projektwurzel mit flight-Werkbank | `kennzeichen = '^flight-workbench$'` |

**Die eine Ausnahme beim Speicherprofil ist `archive`**, und sie ist an dem entschieden,
was im Beispiel steht: der Ordner ist leer und führt Läufe als Ordner, keine Datensätze
als Dateien. Die zwei `.md`-Zeilen des Speicherprofils gäben dort nichts her, `zaehlung`
ohne Muster und `juengste` mit `zeigt = "datum"` dagegen schon. Die zweite Ausnahme des
fusion-Vorbilds, der Defektspeicher mit seiner Aufschlüsselung nach Markern, hat bei
flight keinen Gegenstand.

**`stilwerk` bekommt kein Profil und keine Zeile.** Es trägt die Stilprofile, mit denen
flight schreibt, und keine Datensätze; der Kommentar sagt das in einem Halbsatz, damit die
Auslassung nicht als Lücke gelesen wird.

### Die drei nachgezogenen Kommentarblöcke

- „Was eine Zusammenfassung höchstens kostet": aus „ein Baustein mit `ordner` kostet genau
  einen Leselauf" wird „ein Ort kostet genau einen Leselauf, gleich wie viele Zeilen ihn
  nennen". Dazu der Preis des Platzhalters: ein Leselauf öffnet nicht mehr genau ein
  Verzeichnis.
- Der Platzhalter hat einen **eigenen** Abschnitt bekommen, statt im Absatz über `ordner`
  mitzulaufen: was er trifft, dass genau einer erlaubt ist, dass er allein Ordner greift,
  dass `juengste` und `feld` ihn nicht annehmen, und dass ein Stern innerhalb eines Namens
  ein Name bleibt.
- Der Abschnitt über `juengste` trägt `zeigt` mit beiden Werten und dem Unterschied, aus
  dem beide anderen folgen: die Datumsform öffnet nichts und sieht deshalb Einträge jedes
  Typs.

## Gemessen und nicht behauptet

Gefahren über ein Wegwerfprogramm im Kritzelordner, das `leseprofil::datei::pruefen` und
`zusammenfassen_gezaehlt` gegen `resources/default-readers.toml` und gegen die wirklichen
Bäume ruft. Es liegt außerhalb dieses Baums und ist kein Artefakt dieser Runde.

An `fusion-workbench` dieses Projekts:

| Ordner | Leseläufe | Öffnungen | Auskunft |
|---|---|---|---|
| `circles` | 3 | 0 | Runden 19; 0/0/5/12/0/2; offene Defekte 116 |
| `shared` | 10 | 0 | zehn Paare aus Zahl und Datum |
| `archive` | 1 | 0 | Läufe 2; zuletzt 2026-08-20 21:15 |
| Werkbankwurzel | 3 | 4 | die sieben Zeilen |
| Projektwurzel | 4 | 4 | dieselben sieben |

Die zwei Kriterien, die eine Messung und keine Behauptung verlangen, gehen auf:

- `0 + 0 + 5 + 12 + 0 + 2 = 19`, und `ls fusion-workbench/circles | wc -l` liefert 19.
- Die Zeile „Offene Defekte, alle Runden" zeigt 116, und
  `find fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md' | wc -l` liefert
  116.

Am flight-Beispiel: Projektwurzel 6 Leseläufe / 3 Öffnungen, Werkbankwurzel 5/3, `decisions`
1/1, `history` 1/7, `memos` 1/3, `archive` 1/0. Jedes der zwölf Profile bleibt damit unter
zwölf Leseläufen und unter vierundzwanzig Öffnungen; der größte gemessene Wert ist 10 bei
`shared` und 11 Öffnungen bei einer einzelnen Runde.

**Kein Profil überholt ein vorhandenes.** Nachgemessen an sieben Ordnern, für die die
Erkennung eindeutig sein muss: `shared/planning`, `shared/issues`, `shared/history`, ein
Rundenverzeichnis, dessen `issues` und dessen `decisions`, dazu ein Ablagelauf. Jeder trifft
weiter das Profil, das ihn vor dieser Änderung getroffen hat. Ein Ablagelauf
(`archive/<lauf>`) trifft weiterhin kein Profil; das ist der offene Datensatz
`circles/260823-2208-…/issues/260824-1655_*_sechs-speicher-unter-archive-…` und unberührt.

`make check` → Rückgabewert 0.

## Was über die genannte Dateiliste hinausgegangen ist

Die Aufgabe nennt `resources/default-readers.toml` und `crates/krk-core/src/ablage/leseprofile.rs`,
letzteres „soweit die Probe eine Profilzahl führt, die nachzuziehen ist". **Die Zahl steht
an vier Stellen und nicht an einer**, und `make check` ist an zwei davon rot geworden:

| Stelle | Art | Was geschehen ist |
|---|---|---|
| `src/ablage/leseprofile.rs:179` | Zusicherung | 5 → 12 |
| `tests/ablage.rs:2047` | Zusicherung | 5 → 12 |
| `tests/leseprofil.rs:2240` | Zusicherung | 5 → 12 |
| vier Prosastellen in denselben drei Dateien | Doc-Kommentar | die Zahl **gestrichen** statt nachgezogen |

Die zwei Prüfdateien standen auf keiner der beiden Listen der Aufgabe, weder auf der
erlaubten noch auf der verbotenen. Geändert ist an ihnen je eine Zahl und der Satz
daneben; ohne sie bliebe der Baum rot, und die Zahl ist dieselbe, deren Nachzug die
Aufgabe ausdrücklich verlangt. Bei den Prosastellen ist die Zahl gestrichen und nicht
ersetzt, nach der Gewohnheit dieses Projekts: eine Zahl, die mit dem nächsten Profil
wieder falsch wird, gehört nicht in Prosa. Dieselbe Behandlung hat die Angabe „rund 180
Kommentarzeilen" im Modulkopf von `leseprofile.rs` bekommen; es sind heute 392.

## Ein Defektdatensatz

`fusion-workbench/shared/issues/260825-2044_o_die-zeile-projekt-der-werkbankprofile-haengt-an-einem-feld-das-fusion-nicht-mehr-schreibt.md`

Die Zeile „Projekt" der zwei fusion-Wurzelprofile zieht den Namen aus `setup_pwd` von
`.fusion-setup`. Fusion 7.2.0 und 8.1.0 schrieben das Feld (in der Historie dieses Baums
nachgelesen), fusion 10.7.0 schreibt es nicht mehr. Die Zeile zeigt an dieser Werkbank
`--` und kann dort nie einen Wert liefern. Behoben ist sie nicht: kein Baustein liefert
einen Ordnernamen, und die Behebung läge im Mechanismus unter
`crates/krk-core/src/leseprofil/`, den diese Aufgabe nicht anfassen darf. Der
Kommentarblock über dem Wurzelprofil sagt jetzt, warum die Zeile leer steht, damit ein
Leser sie nicht für einen Schreibfehler in seiner eigenen Datei hält. Bei flight liefert
dieselbe Zeile weiter einen Wert (`2026-Sommer-Adria`), denn flight 0.8.0 schreibt das
Feld noch.

Nebenbei berichtigt und keinen eigenen Datensatz wert, weil in derselben Datei behoben:
der Kommentar zur Verankerung sprach von „den fünf unten", während die Datei schon vor
dieser Änderung sechs `datei`-Muster führte. Die Zahl ist gestrichen.

## Geänderte Dateien

- `resources/default-readers.toml`
- `crates/krk-core/src/ablage/leseprofile.rs`
- `crates/krk-core/tests/ablage.rs`
- `crates/krk-core/tests/leseprofil.rs`
- `fusion-workbench/shared/issues/260825-2044_o_die-zeile-projekt-…md` (neu)
- `fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`
  (Schritt 8 auf `[DONE]`, mit einem Nachtrag über die Abweichung)

Kein Commit; die Änderungen stehen im Arbeitsbaum.
