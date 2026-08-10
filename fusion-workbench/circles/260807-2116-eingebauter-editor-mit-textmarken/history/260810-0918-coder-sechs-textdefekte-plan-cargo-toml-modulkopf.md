# Sechs Textdefekte: Plandokument, Begründungen in `Cargo.toml`, Modulkopf von `appkit`

**Datum:** 260810-0918
**Agent:** `coder`
**Status:** Complete
**Circle:** `260807-2116-eingebauter-editor-mit-textmarken`

---

## Auftrag

Sechs offene Defekte beheben, die Plantext, die geschriebenen Begründungen in der
Wurzel-`Cargo.toml` und den Modulkopf von `crates/krk-ui/src/appkit/mod.rs`
betreffen. Am Programm sollte nichts geändert werden; das Plandokument bleibt
geschlossen, alle 48 Schritte bleiben auf `[DONE]`, und die Umbenennung der
Defektmarker macht der Nutzer.

## Was getan wurde

### 1. `issues/260808-0930` — S11 änderte eine Kernschnittstelle ohne ihre Aufrufstellen

Der Abschnitt `### Was die Dateiliste eines Schrittes zusagt` trägt jetzt als
dritte Herleitungsregel die **Schnittstellen-Regel**: ein Schritt, der eine
öffentliche Form ändert, zieht ihre Aufrufstellen im selben Schritt nach. Sie
steht auf der bindenden Verbotsseite, weil nicht die Vollständigkeit der
Dateiliste geschuldet ist, sondern ein übersetzbarer Arbeitsbereich zwischen zwei
Schritten. Die Regel verweist auf die beiden Abschnitte des Nachtrags, die
dieselbe Lehre für S43 bis S48 schon anwenden, statt sie ein zweites Mal
auszuschreiben.

S11 nennt `crates/krk-ui/src/leistenmodell.rs` in seiner Dateiliste, verlangt
`cargo build --workspace` und `cargo test --workspace` im Abnahmekriterium und
trägt einen Umsetzungsvermerk zu `65c8efa`. Der Commit ist nachgeprüft
(`git show --stat 65c8efa`).

### 2. `issues/260808-0948` — das vierte Kriterium von S32 war in S32 nicht messbar

Das Kriterium ist geteilt. In S32 fragt es den Preis der beiden Kisten an einem
eigenständigen Prüfprogramm (1.591.544 gegen 418.968 Byte, rund 1,12 MiB, aus dem
Datensatz übernommen); das Wachstum des Bündels ist an S33 gewandert, wo der
eingebettete Bestand zum ersten Mal geladen wird.

**Dort ist es gemessen und nicht bloß gefordert.** `cargo xtask bundle` läuft
durch und signiert mit der Entwicklungsidentität; `target/KRK.app` trägt
7.191.902 Byte über vier Dateien, davon 7.180.576 im Programm. Gegenüber den
3.502.046 Byte vor S32 sind das 3.689.856 Byte Zuwachs, rund 3,52 MiB. Die Zahl
ist im Plan ausdrücklich als **obere Schranke** eingetragen: gemessen ist der
Stand nach allen 48 Schritten gegen den Stand vor S32, der Zuwachs enthält also
auch den Code von S33 bis S48. Für ein Kriterium, das eine Grenze nach oben
zieht, genügt die Schranke. Der Zahlenrahmen von 10 MB ist unverändert.

Mitgezogen: der Satz in `### Frage 2` über die beiden Größen, die S32 abnimmt,
und die Zeile in `## Risiken und Gegenmaßnahmen`.

### 3. `issues/260808-0949` — `dump-create` lässt sich nicht abschalten

`### Frage 2` und S32 nennen jetzt vier abgeschaltete Merkmale statt fünf und
sagen dazu, warum: `parsing = [regex-syntax, fnv, dump-create, dump-load]` zieht
es in `syntect` 5.3.0 mit, und ohne `parsing` tut die Kiste nichts. Der Kommentar
in `Cargo.toml` trug den Befund schon und ist unverändert.

### 4. `issues/260808-1413` (syntect) — der transitive Fußabdruck fehlte

Die Begründung zu `syntect` trägt ihn jetzt, in der Form des
`signal-hook`-Eintrags, und die Zahl ist **selbst erhoben**: 21 weitere Pakete,
bestätigt über den Namensvergleich der `Cargo.lock`-Fassungen von `4e86c02` (72
Einträge) und HEAD (95), 23 neue Namen abzüglich der beiden Kisten. Drei
gemessene Aussagen stehen dazu, jede mit ihrem Kommando:

- Auf dem Bauziel kommen 20 davon an; `winapi-util` hängt über `same-file` und
  `walkdir` allein am Windows-Ziel und steht nur in `Cargo.lock`. Die beiden
  `windows-*`-Einträge, an denen es hängt, sind nicht neu.
- `walkdir`, `same-file` und `winapi-util` lassen sich nicht abwählen:
  `[dependencies.walkdir]` steht in `syntect`s eigener `Cargo.toml` ohne
  `optional`.
- Keines der 21 ist ein `-sys`-Paket, keines bringt `cc` als Bauabhängigkeit mit
  (`cargo tree --workspace -e normal,build`, kein Treffer für `cc`, `onig`,
  `-sys`).

Der Eintrag zu `two-face` verweist auf den zu `syntect`, statt die Zahl zu
wiederholen.

### 5. `issues/260808-1413` (vier Platzhalter) — nichts zu ändern

Der Befund ist von der Arbeit der Runde überholt. Alle vier Stellen sind am Code
nachgesehen, und keine trägt mehr einen Vorwärtsverweis ohne Nummer:
`fenstermodell.rs` nennt den gegenseitigen Ausschluss als geltende Zusicherung
über `Bereich::teilt_flaeche_mit`, `aufteilung.rs` nennt Schritt 16 dreimal in
der Vergangenheit. Geprüft mit zwei `grep`-Läufen über beide Dateien. Der
Datensatz hält das Ergebnis samt Tabelle fest; kein Code und kein Plantext ist
dafür angefasst.

### 6. `issues/260809-1655` — „acht Pfeile" aus `appkit` heraus

Der Modulkopf sagt jetzt die Regel statt der Zählung und begründet, warum die
Zahl weggefallen ist. Die Aufzählung bleibt ohne Vollständigkeitsanspruch stehen,
mit `anwendung` und seinen neun Zielen, mit `leiste`, `vorschau` und
`zwischenablage`, und mit `volumes` an `crate::leistenmodell::Ort` statt an
`auffrischung`.

**Ein vierter Fehler stand nicht im Datensatz und ist mitberichtigt:** `bildtakt`
nennt `crate::messmodus` nicht, es trägt gar keine `use crate::`-Zeile. Der Kopf
unterscheidet deshalb zwei Lesarten ausdrücklich: der ASCII-Überblick zeichnet
den **Wertefluss**, eine `use crate::`-Zeile sagt, welches Modul einen Nachbarn
draußen **nennt**. Beide fallen nicht zusammen, `bildtakt` und `fsevents` stehen
im Überblick mit einem Pfeil und in der Aufzählung nicht. Der Überblick bleibt
unverändert, weil er als Wertefluss richtig ist.

Erhoben: 24 `use crate::`-Zeilen in 9 der 22 Dateien, 11 verschiedene Zielmodule.
Die Zahlen stehen ausdrücklich nicht im Modulkopf.

## Neu angelegt

`issues/260810-0918_o_der-plan-zitiert-einen-defekt-mit-einem-zeitstempel-den-sechs-datensaetze-tragen.md`.
Der Abschnitt `### Wie diese sechs Schritte geschnitten sind` kürzt drei
Defektverweise auf ihren Zeitstempel; den dritten (`260808-1413`) tragen sechs
Datensätze, und zwei von ihnen passen inhaltlich. Die Wahl ist aus dem Satz nicht
entscheidbar, deshalb ist der Satz unangetastet geblieben. Nebenbefund im selben
Datensatz: die drei Kürzungen tragen ihren Zustandsmarker und brechen damit die
Zitierregel, die der Plan selbst unter `## Wie dieser Plan auf Datensätze
verweist` aufstellt.

## Abnahme

Zuerst als Ausgangsstand, dann nach den Änderungen, alle vier mit Exitcode 0:

| Kommando | Exit |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0 |
| `cargo clippy --workspace --all-targets` | 0 |
| `cargo fmt --all --check` | 0 |
| `cargo xtask bundle` | 0 (für die Messung zu Defekt 2) |

Zusätzlich `cargo doc -p krk-ui --no-deps --document-private-items`: Exit 0, 23
Warnungen, keine davon an `appkit/mod.rs`. Die Verweise des neuen Absatzes lösen
also auf.

`resources/default-keymap.toml` war beim Sitzungsbeginn von einem parallel
laufenden Agenten geändert; der Ausgangslauf war trotzdem grün, die Exitcodes
oben sind deshalb belastbar meine.

## Dateien

- `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_c_plan-eingebauter-editor-mit-textmarken.md`
- `Cargo.toml`
- `crates/krk-ui/src/appkit/mod.rs`
- die sechs Defektdatensätze mit ihrer `Resolved:`-Zeile
- der neue Defektdatensatz `260810-0918_o_...`

Nicht angefasst, wie beauftragt: `crates/krk-core/src/text/`,
`crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/editormodell.rs`,
`resources/`. `crates/krk-ui/src/fenstermodell.rs` und
`crates/krk-ui/src/appkit/aufteilung.rs` standen in der Dateigrenze, brauchten
aber keine Änderung (Defekt 5).
