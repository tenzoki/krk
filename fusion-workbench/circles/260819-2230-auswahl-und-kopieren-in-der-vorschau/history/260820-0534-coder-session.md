# Coder-Sitzung: Schritt 1 — der Quellbezug entsteht im Durchgang, der rendert

**Status:** Complete

---

## Auftrag

Plan `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel A,
Schritt 1. Datei: `crates/krk-ui/src/markdown.rs`, allein diese.

## Umsetzung

**Die drei Typen** stehen neben `Gerendert` und `Zerlegung`, in den Formen aus
`## Data Structures`: `Quellbezug` mit `quelle`, `abschnitte` und `elemente`; `Abschnitt`
mit Textbereich (UTF-16), Quellbereich (Bytes) und `Abschnittsart`; `Quellelement` mit
Quellbereich und Klammer. `Gerendert` trägt das dritte Feld `quellbezug: Arc<Quellbezug>`.
`Abschnittsart::verdeckt_quelle` liest die Art über ein `match` ohne Auffangzweig, nach dem
Vorbild von `Inhaltsart::deckt_luecken`.

**Die Kachelung wächst an genau einer Stelle**, `Zerlegung::kacheln`. Sie ist zugleich die
einzige Stelle, an der `gelesen` vorrückt. Weil `stelle` und `gelesen` beide nur vorwärts
laufen und jeder Vorlauf einen Abschnitt anlegt, fallen die beiden Zusagen aus C2.6 aus der
Bauart heraus, statt nachträglich hergestellt zu werden. Ein `bis` hinter dem Lesestand
wird zum leeren Bereich und nicht zu einer Überschneidung.

**Die drei Schreibstellen** liefern jetzt ihre Herkunft mit:

- `schreiben(stueck, bis)` legt den Abschnitt `gelesen..bis` an; seine Art ist `Woertlich`,
  wenn jene Bytes genau das geschriebene Stück sind, sonst `Ersetzt`. Ein leeres Stück
  trägt die Quelle trotzdem ab, sonst risse die Kachelung ein Loch.
- `erzeugen(stueck)` ist die zweite Methode für die Zeichen, die KRK selbst setzt; `absetzen`
  und `merkzeichen_einloesen` rufen sie. Der Quellbereich bleibt leer und liegt am Lesestand.
- `gelesen_bis(bis)` legt für abgetragene Bytes ohne Zeichen einen Abschnitt mit leerem
  Textbereich an — das schließende `**`, das `](Ziel)`, der Umbruch hinter einem Absatz.

**Der Vorspann eines Containers** (`luecke_bis`, der Zweig für `gelesen <= anfang`) wird für
den Quellbezug abgetragen und bekommt seine Kachel. Für die Anzeige ändert sich nichts; die
Aussage des Modulkopfs unter „Wo die Deckung endet" gilt unverändert.

**Die Klammer** wird beim **innersten** offenen Element verbucht, und zwar in dem Augenblick,
in dem der Abschnitt entsteht (`klammer_verbuchen`). Damit trägt jeder Abschnitt zu genau
einem Element bei.

## Zwei Stellen, an denen der Plantext eine Entscheidung offenließ

**Erstens: was „Bytes, die im gerenderten Bereich nicht erscheinen" für einen Absatz mit
Kindern heißt.** Wörtlich gelesen trüge der Absatz des bindenden Datensatzes eine Klammer,
denn `**` und `[…](…)` seiner Kinder erscheinen im Text nicht. Dann läge aus einer Auswahl
darin der ganze Absatz in der Ablage — die vom Nutzer nicht gewählte Möglichkeit 3, und die
im Plan zugesagte Antwort `**fetter** Text mit [Verweis](https://example.com)` käme nicht
heraus. Umgesetzt ist deshalb die Lesart, unter der das Beispiel des Datensatzes aufgeht:
verbucht wird beim innersten offenen Element, die Auszeichnungszeichen eines Kindes gehen
nicht auf seinen Vater über. Die Probe schreibt beide Hälften aus.

**Zweitens: Leerraum ist keine Auszeichnung.** Der Quellbereich eines Absatzes endet hinter
seinem Zeilenumbruch, und der steht im Text nicht mehr; ohne diesen Halbsatz trüge **jeder**
Absatz eine Klammer, und wieder wäre es Möglichkeit 3. Ein `- ` oder ein `> ` bleibt übrig,
wenn man den Leerraum abzieht, ein Zeilenumbruch nicht. Beides steht als Begründung am
Doc-Kommentar von `klammer_verbuchen`.

## Zwei Abweichungen vom Plantext, mit Befund

**`#[cfg_attr(not(test), expect(dead_code, …))]` ist nicht nötig und wäre schädlich.** Der
Dispatch nannte es nicht, der Sache nach lag es aber nahe, weil `quelltext` erst in Schritt 2
liest. Gemessen: die abgeleiteten `PartialEq`- und `Clone`-Rümpfe lesen die Felder, also
meldet der Übersetzer sie nicht als tot. Mit der Zeile hielte `unfulfilled_lint_expectations`
unter `-D warnings` den Bau an — derselbe Befund, den die Sitzung zu Schritt 6 an ihrer
eigenen Stelle erhoben hat (`issues/260820-0511_c_…`). Kein zweiter Datensatz dafür.

**`Offen.rang` und ein Zeiger auf den Eintrag in `elemente` sind dieselbe Zahl.** Beide
zählen das Öffnen eines Elements. Statt einen zweiten Zähler danebenzustellen, ist der
vorhandene `Zerlegung::raenge` weggefallen: der Rang **ist** der Platz in `elemente`
(0-basiert statt 1-basiert, was die Sortierung nicht berührt), und der Doc-Kommentar sagt es
an der einen Stelle. Das ist derselbe Grund, aus dem `Zerlegung::tiefe` zählt, statt
mitzuführen. Die Sortierung der Auszeichnungen ist von der Umstellung nicht betroffen und
von der vorhandenen Probe zu `` **`code`** `` (Defekt `260812-1805`) gedeckt.

## Eine Umstellung in `schliessen`, die man sehen muss

Abgetragen wird jetzt, **solange das Element noch offen steht**, und erst danach wird es
abgeräumt. Grund: die Bytes, die sich beim Schließen noch abtragen, sind die eigene Klammer
des Elements und nicht die seines Vaters. Damit das Verhalten gleich bleibt, fällt das
Merkzeichen des sich schließenden Punktes ausdrücklich weg, bevor der wörtliche Quelltext es
einlösen könnte; bei einer Länge größer null ist es ohnehin schon eingelöst. Die vorhandenen
Proben zu den Defekten `260812-1920` und `260812-2019` laufen unverändert grün.

## Proben

- `die_kachelung_deckt_quelle_und_text_lueckenlos` über zehn Beispiele (Absatz, Überschrift,
  starke Betonung, Verweis, Liste über zwei Ebenen, Zitatblock, Quelltextblock,
  Verweisdefinition, Stück in fester Schrift, Umlaute und Emoji). Geprüft wird beidseitig
  lückenlos und überschneidungsfrei, dazu die Zusage von `Woertlich`: beide Seiten stehen
  Zeichen für Zeichen aneinander.
- `umlaute_und_ein_emoji_treffen_beide_enden_eines_abschnitts`: „Grüße 😀 an " sind 12
  UTF-16-Einheiten und 16 Bytes; geprüft am ersten Abschnitt und am letzten mit Zeichen.
- `ueberschrift_betonung_verweis_und_punkt_tragen_eine_klammer_ein_absatz_nicht`: vier Fälle
  mit der vollen Elementliste, darunter der Absatz des bindenden Datensatzes ohne Klammer und
  der Listenpunkt mit Klammer neben seiner Liste ohne.

## Prüfung

- `make check` — exit 0 („alle vier gruen"), also `cargo build --workspace`,
  `cargo test --workspace`, `cargo fmt --all --check` und
  `cargo clippy --workspace --all-targets -- -D warnings` in einem Zug. 21 Probenziele, alle
  `ok`, kein `FAILED`.
- Ein Zwischenstand war rot: `clippy::type_complexity` an der Falltabelle der Klammerprobe.
  Behoben durch Auflösen der Tabelle in vier `assert_eq!` mit einer Hilfsfunktion, nicht durch
  ein `allow`.

## Geänderte Dateien

- `crates/krk-ui/src/markdown.rs`
- `fusion-workbench/circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md` (Schritt 1 → `[DONE]`)

Nicht committet — der Orchestrator committet.
