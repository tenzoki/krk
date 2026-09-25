# Coder-Sitzung: Schritt 2 — die Klammerregel, aus einer Auswahl wird ein Quellausschnitt

**Status:** Complete

---

## Auftrag

Plan `planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Bündel A,
Schritt 2. Datei: `crates/krk-ui/src/markdown.rs`, allein diese. Ein zweiter `coder`
arbeitete gleichzeitig an `crates/krk-ui/src/appkit/vorschau.rs`; keine Zeile davon ist
angefasst, kein Git-Kommando über den ganzen Baum gefahren.

## Umsetzung

**`Quellbezug::quelltext(&self, auswahl: Range<usize>) -> &str`** ist der eine öffentliche
Zugang und trägt `#[must_use]`. Darunter vier private Methoden und eine freie Funktion; die
Oberfläche rechnet nichts.

**Erste Stufe, `huelle_der_abschnitte`.** Die Hülle über die Quellbereiche der berührten
Abschnitte. Was ein einzelner Abschnitt beiträgt, entscheidet `beitrag` über ein `match`
ohne Auffangzweig: `Woertlich` rechnet die Auswahlgrenze auf ein Byte um, `Ersetzt` rundet
auf die Ränder des Abschnitts, `Erzeugt` trägt nichts bei. Eine Hülle und keine Vereinigung
genügt, weil die Abschnitte die Quelle lückenlos kacheln.

**`Abschnitt::beruehrt` trägt die zwei Lesarten**, getrennt von einer einzigen Frage: trägt
der Abschnitt Zeichen? Mit Zeichen gilt der gewöhnliche halboffene Schnitt, ohne Zeichen das
**geschlossene** Auswahlintervall. Der Doc-Kommentar schreibt aus, dass die zweite keine
Ausnahme ist, sondern die einzige Lesart, unter der ein leerer Textbereich überhaupt
erreichbar ist.

**`byte_zur_stelle` ist die eine Umrechnung im Modul (C2.7).** Sie hat nur innerhalb eines
`Woertlich`-Abschnitts etwas zu rechnen, wo der Quellausschnitt und der geschriebene Text
Zeichen für Zeichen dieselben sind. Gezählt wird über `char_indices` und `len_utf16`. Eine
Stelle mitten in einem Ersatzpaar gibt das Byte hinter dem Zeichen, statt eine ungültige
Zeichengrenze zu liefern, an der der Zugriff auf die Quelle abbräche.

**Zweite Stufe, `klammern_schliessen`.** Der Fixpunkt aus dem Plantext, wörtlich: erweitere
auf den ganzen Quellbereich jedes Elements, das eine Klammer trägt, das der Ausschnitt
schneidet und nicht ganz enthält, bis er sich nicht mehr ändert. Der Doc-Kommentar führt das
Endeargument aus: der Ausschnitt wächst allein, die Quelle ist endlich, und ein einmal ganz
enthaltenes Element bleibt enthalten, weil der Ausschnitt nie schrumpft — jedes Element
erweitert ihn also höchstens einmal.

## Warum die geschlossene Halbregel keinen unwohlgeformten Rand erzeugt

Beim Bauen fiel ein Fall auf, den der Plantext nicht ausschreibt: die geschlossene Lesart
holt an der **oberen** Grenze auch Bytes herein, die zu einer Auszeichnung gehören, die die
Auswahl gar nicht berührt. In `Ein **fetter** Text mit x.` liefert die erste Stufe bei einer
Auswahl von ` Text mit x.` den Ausschnitt `** Text mit x.`, also eine Betonung, die nur
schließt. Die zweite Stufe räumt das auf: die starke Betonung trägt eine Klammer, wird
geschnitten und nicht enthalten, also fährt sie ganz mit, und heraus kommt
`**fetter** Text mit x.`. Was die Halbregel hereinholt, gehört damit entweder einem Element
mit Klammer, das die zweite Stufe vollständig macht, oder es ist der Zeilenumbruch hinter
einem Block, und der schadet in keiner Zwischenablage. Beides steht am Doc-Kommentar von
`quelltext`.

## Eine Abweichung vom Vorgehen des Schrittes 1, mit Befund

**`#[cfg_attr(not(test), expect(dead_code, …))]` steht an `quelltext`, und hier ist es
nötig.** Die Sitzung zu Schritt 1 hat die Zeile für die drei Typen ausdrücklich weggelassen
(ihre abgeleiteten Rümpfe lesen die Felder), die Sitzung zu Schritt 6 ebenso
(`issues/260820-0511_c_…`, dort ruft `text_schreiben` bereits). Für `quelltext` gilt beides
nicht: der Rufer entsteht erst in Schritt 7, und ohne die Zeile meldete `cargo build` fünf
tote Stücke — gemessen, nicht vermutet. Mit ihr ist der Bau still, und
`cargo clippy --workspace --all-targets -- -D warnings` läuft grün, die Erwartung ist also
erfüllt und nicht offen. Eine Zeile am Wurzelstück genügt: die vier privaten Methoden und
`byte_zur_stelle` gelten dem Übersetzer damit als erreichbar. Kein Defektdatensatz dafür;
der Plan sagt für diesen Schritt zur Zeile nichts, sie ist der Sache nach fällig, und sie
setzt ihr Ablaufdatum selbst durch.

## Proben

Sieben, wie der Schritt sie aufzählt. Sie fahren alle über `kachelung_pruefen` aus Schritt 1,
messen also die beiden Kachelzusagen gleich mit.

- `das_beispiel_des_datensatzes_liefert_wohlgeformtes_markdown`: die Quelle, die Auswahl und
  die Erwartung des bindenden Datensatzes im Wortlaut.
- `eine_auswahl_in_einer_ueberschrift_liefert_ihr_doppelkreuz`. Der Quellbereich einer
  Überschrift reicht bis hinter ihren Zeilenumbruch, also steht der in der Erwartung mit
  drin; das Doppelkreuz ist der Punkt.
- `eine_auswahl_im_text_eines_verweises_liefert_die_ganze_adresse`.
- `eine_auswahl_in_einem_langen_absatz_liefert_nicht_den_absatz` — **die Probe, die
  Möglichkeit 3 ausschließt.** Gegengemessen: mit ausgehängter Klammerbedingung liefert sie
  den ganzen Satz statt der zwei markierten Wörter und wird rot. Sie unterscheidet also
  wirklich zwischen der gewählten und der nicht gewählten Möglichkeit.
- `die_auswahl_ueber_alles_liefert_die_quelle_vollstaendig` (C2.8), über alle zehn
  Kachelbeispiele statt über eines. Darunter ist das Beispiel, das mit einem Listenpunkt
  beginnt und mit einem Zeilenumbruch endet.
- `eine_auswahl_im_verschachtelten_element_liefert_das_aeussere_ganz`: drei Buchstaben in
  `**fett *und kursiv* zugleich**` liefern das äußere Element ganz, über zwei Durchgänge
  desselben Verfahrens.
- `eine_auswahl_zwischen_umlauten_und_einem_emoji_trifft_die_bytegrenzen` (C2.7), zwei Fälle:
  einer innerhalb eines wörtlichen Abschnitts, einer über eine Betonung.

## Prüfung

`make check` — exit 0 („alle vier gruen"), also `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings` in einem Zug.

Ein erster Lauf hielt an `cargo fmt --all --check` an, und zwar an
`crates/krk-ui/src/appkit/vorschau.rs`, der Datei des gleichzeitig laufenden zweiten
`coder`. `crates/krk-ui/src/markdown.rs` wurde deshalb einzeln mit `rustfmt` formatiert statt
mit `cargo fmt --all`, das die fremde Datei mit angefasst hätte. Der zweite Lauf ist grün.

## Geänderte Dateien

- `crates/krk-ui/src/markdown.rs`
- `fusion-workbench/circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md` (Schritt 2 → `[DONE]`)

Nicht committet — der Orchestrator committet.
