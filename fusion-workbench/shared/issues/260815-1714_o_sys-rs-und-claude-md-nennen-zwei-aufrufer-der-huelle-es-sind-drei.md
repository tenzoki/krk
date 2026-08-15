`sys.rs` und `CLAUDE.md` nennen zwei Aufrufer von `ohne_warten_oeffnen`, es sind drei

---

`8c06747` hat `verweisziel::bestimmen` als dritten Rufer von
`verzeichnis::sys::ohne_warten_oeffnen` hinzugefügt. Nachgezogen ist der Modulkopf von
`verzeichnis/mod.rs` — er nennt den dritten Rufer ordentlich. Nicht nachgezogen ist die
Datei, in der die Hülle selbst steht, und `CLAUDE.md`.

---

**Schwere:** mittel. Kein Verhalten, kein Bau. Wer die Reichweite der Hülle am Ort der Hülle
nachliest, bekommt die falsche Auskunft, und eine Aussage über den gemeinsamen Ablauf ist
dadurch falsch geworden.
**Gefunden von:** coderev, Durchsicht des Bereichs `a2670db..8c06747`
**Betroffen:** `crates/krk-core/src/verzeichnis/sys.rs:15-16`, `:46-49`, `:787`, `:789-792`,
`:794-796`, `:810-811`; `CLAUDE.md:135`
**Domain:** code

## Die sechs Stellen

| Ort | Was dasteht |
|---|---|
| `sys.rs:15-16` | Die Übersichtsskizze führt zu `fcntl(2)` zwei Pfeile: `text::datei::oeffnen` und `krk-ui: vorschaumodell`. `verweisziel` fehlt. |
| `sys.rs:46-49` | „Es sind seit dem Defekt `260810-1247` **zwei**, und der zweite liegt ausserhalb der Kiste". |
| `sys.rs:787` | Die Überschrift des Abschnitts: „# **Zwei Aufrufer**, und die Zielpruefung bleibt bei beiden". |
| `sys.rs:789-792` | „Gerufen wird die Funktion von `text::datei::oeffnen` … und von `vorschaumodell::bis_zur_grenze_lesen`". Zwei genannt, drei vorhanden. |
| `sys.rs:810-811` | „der **zweite** Aufrufer ist mit `260810-1247` dazugekommen" — als Abschluss der Aufzählung gelesen. |
| `CLAUDE.md:135` | „**Die Hülle hat zwei Aufrufer**, den Editor … und seit der Runde 2 auch die Vorschau". |

Nachgezählt: `grep -rn 'ohne_warten_oeffnen' crates/` findet drei Aufrufstellen —
`text/datei.rs:414`, `vorschaumodell.rs:679`, `verweisziel.rs:85`.

## Eine Aussage ist dadurch nicht bloß unvollständig, sondern falsch

`sys.rs:794-796` schreibt: „**Gemeinsam ist beiden der Ablauf**: hier oeffnen, `fstat` am
Deskriptor fragen, alles abweisen, was `is_file()` nicht bejaht, die Groesse gegen eine
Grenze halten, erst danach lesen."

Der dritte Rufer tut nichts davon außer den ersten beiden Schritten: er fragt `is_dir()`
statt `is_file()`, hält keine Größengrenze und liest nie. Der Satz beschreibt seit `8c06747`
zwei von drei Rufern und behauptet, alle zu beschreiben. Der ganze Abschnitt „# Zwei
Aufrufer" ist die Begründung dafür, warum die Zielprüfung beim Rufer bleibt und nicht in die
Hülle wandert; der dritte Rufer stützt diese Begründung sogar am stärksten, weil seine
Antwort weder die des Editors noch die der Vorschau ist. Er fehlt nur.

## Warum das hier einen Datensatz bekommt

Die Familie ist in diesem Baum belegt und teuer: `shared/issues/260812-1438`,
`260812-2253`, `260813-1345`, `260815-1047` sind vier Erhebungen derselben Art in vier
Tagen, und `CLAUDE.md` hat als Antwort darauf mehrere Zahlen aus der Prosa genommen und
durch ein Zählkommando ersetzt. Hier steht sie wieder, an einer Stelle, die keine Zählregel
trägt.

## Vorschlag

Die sechs Stellen nachziehen und dabei prüfen, ob `CLAUDE.md:135` die Aufzählung überhaupt
führen muss: `grep -rn 'ohne_warten_oeffnen' crates/` beantwortet die Frage in einer Zeile,
und derselbe Absatz nennt drei Zeilen weiter oben schon eine Regel statt einer Zahl. Der
Satz in `sys.rs:794-796` braucht daneben eine sachliche Berichtigung und nicht nur einen
dritten Namen: gemeinsam sind Öffnen und `fstat`, verschieden sind Zieltyp, Grenze und
Antwort.

Ist `260815-1713_o_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md`
so entschieden, dass `bestimmen` auf `std::fs::metadata` wechselt, fällt der dritte Rufer
wieder weg und die sechs Stellen stimmen von selbst. Dieser Datensatz sollte deshalb nach
jenem bearbeitet werden.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und `CLAUDE.md` und nicht die Directive einer Runde.
