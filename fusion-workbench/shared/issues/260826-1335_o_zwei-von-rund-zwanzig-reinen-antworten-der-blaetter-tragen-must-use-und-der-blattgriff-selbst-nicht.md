Zwei von rund zwanzig reinen Antworten der Blätter tragen `#[must_use]`, und der `Blattgriff` selbst nicht

---

Unter `crates/krk-ui/src/appkit/blaetter/` tragen genau zwei Stellen `#[must_use]`:
`abbruchstelle` (`mod.rs:435`) und `bestaetigungsstelle` (`:481`). Daneben stehen reine
Antworten ohne Wirkung, deren stilles Fallenlassen unbemerkt bliebe: `Taste::zeichen` und
`zusatztasten` (`mod.rs:334, 342`), `Schaltflaeche::neu` (`:388`), `standardschaltflaechen`
(`:576`), `Blattgriff::abbruchweg` (`:531`), `antwort_von_stelle` (`:845`);
`konflikt::schaltflaechen`, `antwort`, `tastenhinweis`, `AntwortAblesen::name`
(`konflikt.rs:139, 194, 217, 296`); `loeschbestaetigung::schaltflaechen` (`:109`);
`Spalte::kennung`, `titel`, `breite`, `aus_kennung`, `Regelfelder::regel`,
`Vorschauquelle::ergebnis`, `frage`, `zusammenfassung` (`stapelumbenennen.rs:153, 162, 176,
184, 223, 326, 444, 452`); `uebernimmt`, `zettel_an_stelle`, `textrahmen`
(`zettel.rs:312, 320, 441`).

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/`
**Cross-references:** `shared/issues/260826-1221_*_must-use-fehlt-an-fast-jeder-reinen-antwort-der-vorgangsmaschine-und-des-stapelumbenennens.md`, `shared/issues/260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…md` (dieselbe Regel, andere Kisten)

**Die eine Stelle, an der es schon gekostet hat:** der Typ `Blattgriff` (`mod.rs:496`). Ein
Griff, der fällt, nimmt dem Abbruchbefehl sein Blatt
(`shared/issues/260826-1325_*_esc-im-stapel-umbenennen-blatt-mit-fokus-in-der-vorschautabelle-schliesst-das-blatt-nicht-sondern-leert-den-filter-dahinter.md`). `Blatt::zeigen`
schreibt dafür `let _griff =` (`:765`), also genau die Form, mit der dieses Projekt „ich brauche
den Wert nicht" sagt — nur dass der Wert hier gebraucht würde. Ein `#[must_use]` am Typ
hätte an den fünf Aufrufern eine bewusste Entscheidung erzwungen, statt eines Vergessens.

Denkbarer Weg: `#[must_use]` an den Typ `Blattgriff` und an die aufgezählten reinen Antworten,
nach der Regel des Nutzers vom 260811-2140.
