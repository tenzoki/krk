# Die vier flight-Speicher tragen jetzt ihre Datumszeile

**Agent:** ontocoder
**Datum:** 2026-08-26, ab 07:55
**Aufgabe:** Die vier flight-Profile von `resources/default-readers.toml` analog zu den
fusion-Profilen der Runde 18 vertiefen, dazu die zwei offenen Befunde im selben Textblock:
`shared/issues/260825-2126_*_der-flight-kommentar-nennt-drei-felder-in-fusion-setup-es-sind-zwei.md`
und `shared/issues/260825-2126_*_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md`
**Status:** Complete

## Teil 1 — die Analogie

Vorbild ist `fusion-Werkbank: der gemeinsame Speicher` (`pfad = 'fusion-workbench/shared$'`):
je Unterspeicher zwei Zeilen, `zaehlung` und `juengste = { …, anzahl = 1, zeigt = "datum" }`,
und die zwei nennen denselben Ort und teilen sich seine Lesung.

Eine flight-Werkbank hat kein `shared/`; ihre Speicher liegen flach an der Wurzel. Die
Entsprechung des Vorbilds sind deshalb die zwei Profile, die diese Speicher zählen. Beide
haben je vier Datumszeilen bekommen, für `decisions`, `history`, `memos` und `archive`:

| Profil | Zeilen vorher | Zeilen jetzt |
|---|---|---|
| `flight-Werkbank: die Wurzel` (`kennzeichen = '^\.flight-setup$'`) | 7 | 11 |
| `Projektwurzel mit flight-Werkbank` (`kennzeichen = '^flight-workbench$'`) | 7 | 11 |

Die Beschriftungen folgen dem Vorbild wörtlich: „Entscheidungen, zuletzt", „Verläufe, zuletzt",
„Notizen, zuletzt", „Ablagen, zuletzt".

**Der Ablagespeicher trägt kein `muster`.** Er führt seine Läufe als Ordner, und `zeigt = "datum"`
ist dort nicht eine Wahl unter zweien: die Titelform öffnete Dateien und sähe in diesem Speicher
keinen einzigen Eintrag. Die Form ist die des eigenen Profils `flight-Werkbank: der Ablagespeicher`,
das schon vorher `juengste = { anzahl = 1, zeigt = "datum" }` trug.

## Der Preis, gerechnet und gemessen

Gerechnet gegen die Regel im Abschnitt „Was eine Zusammenfassung höchstens kostet" derselben
Datei: **höchstens 12 Leseläufe und 24 Dateiöffnungen je Zusammenfassung**, ein Ort kostet genau
einen Lauf gleich wie viele Zeilen ihn nennen, plus einen Lauf für die Erkennung, wenn das Profil
über sein `kennzeichen` erkannt wurde und keine seiner Zeilen den erkannten Ordner selbst nennt.

| Profil | genannte Orte | Erkennungslauf | Leseläufe | Öffnungen |
|---|---|---|---|---|
| `flight-Werkbank: die Wurzel` | erkannter Ordner, `decisions`, `history`, `memos`, `archive` | geteilt (drei Feldzeilen nennen den erkannten Ordner) | **5** von 12 | **3** von 24 |
| `Projektwurzel mit flight-Werkbank` | `flight-workbench` und dessen vier Speicher | kommt obendrauf (keine Zeile nennt den erkannten Ordner) | **6** von 12 | **3** von 24 |

Die vier Datumszeilen kosten in beiden Profilen **null** zusätzliche Läufe und **null**
Öffnungen: sie nennen die Orte, die die Zählungen schon nennen, und `zeigt = "datum"` liest das
Änderungsdatum aus dem Verzeichniseintrag, den der Leselauf ohnehin liefert. Die Rechnung geht
also mit sieben beziehungsweise sechs Läufen Luft auf; die Grenze ist nicht angefasst.

**Nachgemessen und nicht nur hergeleitet.** Eine Messhilfe im Scratchpad
(`scratchpad/messen`, Pfadabhängigkeit auf `krk-core`, lädt über `toml::from_str` und
`leseprofil::datei::pruefen`, fährt `zusammenfassen_gezaehlt`) gegen einen Prüfordner in der
Gestalt einer flight-Werkbank — `.flight-setup` mit allen drei Feldern, je ein Datensatz in
`decisions`, `history` und `memos`, ein Lauf als Ordner in `archive`, dazu ein `stilwerk`:

```
flight-Werkbank: die Wurzel          5 Leselaeufe, 3 Oeffnungen, 11 Zeilen
Projektwurzel mit flight-Werkbank    6 Leselaeufe, 3 Oeffnungen, 11 Zeilen
```

Alle 22 Zeilen liefern einen Wert; keine steht auf `Wert::Nicht`. Die Zahl der Profile bleibt 12,
und `pruefen` beanstandet nichts.

## `flight-Werkbank: ein Speicher` bleibt unverändert

Es trägt „Datensätze" und „Die jüngsten zehn" und ist die Entsprechung von
`fusion-Werkbank: ein Speicher`, das dieselben zwei trägt. Das fusion-Vorbild ist unverändert,
also wächst dieses nicht mit. Ausdrücklich geprüft und ausdrücklich nichts geändert.

## Teil 2 — die zwei Befunde

**Der Kommentar nannte drei Felder in `.fusion-setup`.** Der Satz „Die Kennzeichendatei
`.flight-setup` trägt dieselben drei Felder wie `.fusion-setup`, also greifen dieselben Muster"
ist ersetzt. Er stimmte in seiner Wirkung und nicht in seiner Prämisse: `.fusion-setup` trägt
zwei Felder, `.flight-setup` dieselben zwei **und dazu** `setup_pwd`, das fusion nicht mehr
schreibt. Der neue Satz sagt das, sagt, dass alle drei Muster greifen, aber nicht mit derselben
Ausbeute, und verweist auf den Absatz beim fusion-Wurzelprofil, der es ausschreibt.

**Der Doppelungshinweis stand nur über einem der beiden flight-Blöcke.** Er steht jetzt über
beiden, in derselben Aufteilung wie beim fusion-Paar: über `flight-Werkbank: die Wurzel` in der
ausführlichen Form mit dem Grund („eine Vererbung oder eine Vorlage wäre ein neuer Mechanismus"),
über `Projektwurzel mit flight-Werkbank` in der kurzen, die auf diesen Grund zurückverweist. Die
Zahl im Hinweis ist in beiden auf elf nachgezogen.

Beide Datensätze tragen ihre `Resolved:`-Notiz und heißen jetzt `_c_` statt `_o_`; verschoben
ist keiner.

## Ein dritter Halbsatz, den die Änderung falsch gemacht hätte

Der Kopf des flight-Abschnitts sagte: „die vier Profile unten sind aus demselben Grund kürzer
als ihre Vorbilder". Das war schon vorher keine zutreffende Aussage — alle vier flight-Profile
hatten genau so viele Zeilen wie ihre fusion-Gegenstücke —, und mit elf gegen sieben wäre sie
offen falsch geworden. Der Satz sagt jetzt, was stimmt und was er sagen wollte: es stehen unten
**vier** Profile und nicht acht, weil der Defektspeicher mit seiner Aufschlüsselung nach Markern
und die zwei Rundenprofile hier keinen Gegenstand haben und die Aufgabe des gemeinsamen
Speichers dem Wurzelprofil zufällt. Das ist ein Nebenbefund dieser Aufgabe und stand in keinem
Datensatz; er ist hier festgehalten und nicht als eigener Defekt abgelegt, weil er im selben
Zug behoben ist.

## Die zwei Blöcke laufen nicht auseinander

Nach der Änderung nachgemessen, wie es der Datensatz zum Doppelungshinweis für den Stand davor
getan hat: normalisiert man im zweiten Block das vorangestellte `flight-workbench/` weg, sind
alle 22 Angaben je Paar zeichengleich.

## Abnahme

```
cargo test --workspace        exit 0   (alle Prüfziele grün, 0 failed)
cargo fmt --all --check       exit 0
cargo clippy --workspace --all-targets   exit 0
```

Die Probe, die die Auslieferungsfassung gegen ihre eigenen Regeln hält
(`crates/krk-core/tests/leseprofil.rs`, `ausgelieferte()` über `AUSLIEFERUNGSTEXT`), nimmt die
Datei an: keine Beanstandung, weiter zwölf Profile. Keine Probe des Baums nennt `flight`, die
Zahlen der vier flight-Profile sind also durch nichts im Baum gehalten — allein durch die
Messung oben und den Kommentar, der sie ausschreibt.

**Kein Profil hat eine Zeile verloren.** Gezählt vor und nach der Änderung:

```
7 fusion-Werkbank: die Wurzel              2 fusion-Werkbank: der Ablagespeicher
2 fusion-Werkbank: ein Speicher           20 fusion-Werkbank: der gemeinsame Speicher
5 fusion-Werkbank: ein Defektspeicher      9 fusion-Werkbank: eine Runde
8 fusion-Werkbank: alle Runden             7 Projektwurzel mit fusion-Werkbank
11 flight-Werkbank: die Wurzel             2 flight-Werkbank: der Ablagespeicher
2 flight-Werkbank: ein Speicher           11 Projektwurzel mit flight-Werkbank
```

Die acht fusion-Profile sind unverändert. 801 Zeilen vorher, 877 nachher.

## Geänderte Dateien

- `resources/default-readers.toml`
- `fusion-workbench/shared/issues/260825-2126_c_der-flight-kommentar-nennt-drei-felder-in-fusion-setup-es-sind-zwei.md` (aus `_o_`)
- `fusion-workbench/shared/issues/260825-2126_c_der-doppelungshinweis-steht-bei-flight-nur-ueber-einem-der-beiden-bloecke.md` (aus `_o_`)
