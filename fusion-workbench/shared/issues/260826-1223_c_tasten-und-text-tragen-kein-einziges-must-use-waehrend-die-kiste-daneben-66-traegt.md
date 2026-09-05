`tasten/` und `text/` tragen kein einziges `#[must_use]`, während dieselbe Kiste daneben 66 trägt

---

Die Projektregel vom 260811-2140 sagt: ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt `#[must_use]`. `krk-core` hält sie an 66 Stellen. In `tasten/` (47 öffentliche Funktionen), in `text/` (26) und in `zwischenablage.rs` steht keine einzige — und zwar auch nicht an den Funktionen, die genau die Gestalt haben, an der die Regel anderswo angewandt ist.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die Zählung, am 260826 am Baum erhoben

| Modul | `#[must_use]` | öffentliche Funktionen |
|---|---|---|
| `verzeichnis/` | 28 | 94 |
| `leseprofil/` | 15 | 35 |
| `operation/` | 13 | 25 |
| `ablage/` | 10 | 66 |
| `stapelumbenennen/` | 0 | 12 |
| **`tasten/`** | **0** | **47** |
| **`text/`** | **0** | **26** |
| **`zwischenablage.rs`** | **0** | 3 |

Erhoben mit `grep -rn '#\[must_use' crates/krk-core/src/<modul>`. `stapelumbenennen/` steht mit auf der Null und liegt außerhalb des Umfangs dieser Durchsicht; es gehört in denselben Durchgang.

## Warum das kein Formalbefund ist

Die Regel ist nicht „jede reine Funktion", sondern „ein Fallenlassen, das unbemerkt bliebe". Die Stellen, an denen `krk-core` sie schon anwendet, haben genau die Gestalt der hier fehlenden: `verzeichnis::verweisziel::bestimmen`, `verzeichnis::inhalt::traegt_der_inhalt`, `verzeichnis::umfang::zaehlen`, `verzeichnis::filter::inhaltsschwelle`, `leseprofil::erkennung::erkennen`, `leseprofil::bausteine::zusammenfassen` — durchweg reine Funktionen, deren ganze Wirkung ihr Rückgabewert ist.

Dieselbe Gestalt, ohne Marke, in den zwei Modulen dieser Durchsicht:

- `tasten::belegung::Belegung::konflikte` (`belegung.rs:1379`) — liefert `Vec<Konflikt>`. Ein `belegung.konflikte();` übersetzt und übergeht jeden Konflikt schweigend. Das ist der schärfste Fall der Liste: `Vec` trägt keine eigene Marke, und die Zusage aus C3 hängt an dieser Antwort.
- `text::suche::alle_ersetzen`, `einen_ersetzen`, `alle`, `erster_ab`, `naechster`, `voriger`, `erster_ab_stelle`, `naechster_stelle` (`suche.rs:101-262`) — ein fallengelassenes `alle_ersetzen` heißt: nichts ist ersetzt worden, und niemand merkt es.
- `text::marke::wiederfinden` (`marke.rs:141`)
- `text::datei::sicherungsform`, `gehaltene_form`, `in_gehaltene_form`, `ist_in_gehaltener_form`, `versatz_nach_der_wandlung`, `einlesen` (`datei.rs:732-881`) — `in_gehaltene_form` fallenzulassen ist die Wandlung, die nicht stattgefunden hat, und der Modulkopf nennt sie die Bedingung, unter der drei andere Module rechnen dürfen (`datei.rs:41-43`).
- `text::zeilen::Zeilenindex::neu`, `anfang_der_zeile`, `inhalt_der_zeile`, `zeile_am_versatz`, `zeilenzahl` (`zeilen.rs:80-167`)
- `tasten::normalisierung::normalisieren` (`normalisierung.rs:181`)
- `tasten::parser::code_von`, `code_von_pflicht`, `zeichen_der_stelle`, `zeichen_als_kennung`, `taste_mit_namen`, `taste_mit_code`, `taste_mit_zeichen`, `Kombination::aus_tastendruck`, `Taste::kennung`, `Tastendruck::kennung` (`parser.rs:341-600`, `mod.rs:110`)
- `zwischenablage::deuten` (`zwischenablage.rs:54`)

Nicht betroffen sind die Funktionen, die `Result` oder `Option` als **Fehlerkanal** liefern — `Belegung::zuweisen`, `Belegung::sichern`, `Kombination::lesen`, `datei::sichern`, `bis_zur_grenze_lesen`, `anlesen`: dort trägt die Standardbibliothek die Marke schon.

## Wie die Regel im Bau greift

`CLAUDE.md` hält fest, dass `unused_must_use` erst unter `-D warnings` ein Fehler ist, `cargo build` und `cargo test` allein also grün bleiben. Der Bau, der es fängt, ist `cargo clippy --workspace --all-targets` aus `make check`. Die Marken nachzutragen kostet nichts an Verhalten und macht genau die stillen Fälle laut.

## Verwandte Datensätze

Das Projekt hat dieselbe Regel bisher **je Funktion** verfolgt: `shared/issues/260820-0739_*`, `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0942_c_packschritt-traegt-kein-must-use-…`, `260817-1112_c_frage-und-erlaeuterung-traegt-kein-must-use`, `260818-2335_c_vorgang-laeuft-carries-no-must-use-…`. Dieser Befund steht daneben und nicht darunter: er ist keine vergessene Einzelstelle, sondern eine Lücke über zwei ganze Module, und der Durchgang dafür ist einer und nicht zwanzig.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.

---
Also seen: 260826-1225 durch coderev (Parallellauf auf `ablage/`) — `shared/issues/260826-1225_*_geladen-traegt-kein-must-use-und-vier-der-fuenf-ladewege-koennen-ihre-ersetzung-still-fallen-lassen.md` findet dieselbe Regel an einem Typ in `ablage/` verletzt. Zwei Datensätze und nicht einer, weil die Gegenstände verschieden sind: dort eine benannte Stelle mit einer benannten Wirkung, hier die Deckung zweier ganzer Module. Wer den einen behebt, behebt den anderen nicht.

---
Resolved: `tasten/`, `text/` und `zwischenablage.rs` tragen die Marke jetzt. Die im
Datensatz benannten Stellen alle: `Belegung::konflikte`, `text::suche::{alle,
einen_ersetzen, alle_ersetzen}`, `text::marke::wiederfinden`,
`text::datei::{sicherungsform, gehaltene_form, in_gehaltene_form, ist_in_gehaltener_form,
versatz_nach_der_wandlung}`, `text::zeilen::{neu, anfang_der_zeile, zeile_am_versatz,
zeilenzahl}`, `tasten::normalisierung::normalisieren`,
`tasten::parser::{code_von_pflicht, Kombination::neu, Taste::kennung, Tastendruck::kennung}`
und `zwischenablage::deuten`, dazu die uebrigen reinen Leser derselben Bauart.
Zwei Abweichungen von der Liste des Datensatzes, beide aus demselben Grund: `Option` ist in
der Standardbibliothek selbst `#[must_use]`, also bricht ein zweiter Vermerk an Clippys
`double_must_use`. Betroffen sind `suche::{erster_ab, naechster, voriger, erster_ab_stelle,
naechster_stelle}`, `datei::einlesen`, `zeilen::inhalt_der_zeile`,
`parser::{code_von, zeichen_der_stelle, zeichen_als_kennung, taste_mit_namen, taste_mit_code,
taste_mit_zeichen}` und `Kombination::aus_tastendruck`. Sie sind bereits gedeckt, wie der
Datensatz es fuer `Result` schon festhaelt; die Liste dort haette sie mitnennen muessen.
`belegung::laden` liefert `Geladen<Belegung>` und ist ueber die Marke am Typ gedeckt, die
`shared/issues/260826-1225_*_geladen-traegt-kein-must-use-und-vier-der-fuenf-ladewege-koennen-ihre-ersetzung-still-fallen-lassen.md`
gesetzt hat.
`stapelumbenennen/`, das der Datensatz ausdruecklich in denselben Durchgang legt, ist mit
`shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-und-des-stapelumbenennens.md`
mitgeraeumt.
Geprueft mit `cargo clippy -p krk-core --all-targets -- -D warnings` (Rueckgabe 0) und
`cargo test -p krk-core` (Rueckgabe 0).
