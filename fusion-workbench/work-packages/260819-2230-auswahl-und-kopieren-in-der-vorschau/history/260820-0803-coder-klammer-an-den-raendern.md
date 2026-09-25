# Die Klammer hängt an den Rändern eines Elements und nicht mehr an jedem verdeckten Byte darin

**Agent:** coder
**Datum:** 260820-0803
**Status:** Complete
**Circle:** `260819-2230-auswahl-und-kopieren-in-der-vorschau`
**Baumstand bei Beginn:** `fce0b6f` + unversionierte Änderungen der Runde 14
**Gegenstand:** die zwei hohen Befunde der Durchsicht `reviews/260820-0745-coderev-auswahl-und-kopieren-in-der-vorschau.md`
**Datei:** `crates/krk-ui/src/markdown.rs`, und keine zweite

---

## Was behoben ist

Zwei Befunde, eine Wurzel:

- `issues/260820-0728_*_ein-absatz-mit-entitaet-oder-escape-traegt-eine-klammer-…` — ein Absatz mit
  `&amp;`, `\*` oder einem harten Umbruch mit Backslash trug eine Klammer, und jede Auswahl darin
  blähte sich auf ihn auf: die vom Nutzer nicht gewählte Möglichkeit 3.
- `issues/260820-0731_*_eine-ueberschrift-die-mit-einem-kind-beginnt-verliert-ihre-eigene-klammer.md`
  — eine Überschrift, die mit einer Betonung, einem Stück fester Schrift oder einem Verweis
  beginnt, verlor ihr `# `.

Beide kamen aus derselben Stelle und zeigten in entgegengesetzte Richtungen. `klammer_verbuchen`
setzte die Klammer eines Elements, sobald **irgendwo** in seinem Quellbereich Bytes abgetragen
wurden, die im Text nicht wiederkehren. Der Begriff „Klammer" heißt im Plan und im Modulkopf aber,
dass ein Element an seinen **Rändern** Zeichen trägt, die beim Zerschneiden unbalanciert
zurückblieben.

## Die Lösung, und warum es eine ist und nicht zwei

Die Klammer hängt jetzt am **Vorspann und am Nachspann des Elements selbst**, also an den Bytes
zwischen dem Anfang seines Quellbereichs und dem ersten Ereignis darin sowie zwischen dem letzten
Ereignis und seinem Ende. Was dazwischen verdeckt bleibt, zählt nicht mehr: eine Entität mitten in
einem Absatz zerschneidet nichts, denn sie steht ganz in dem Stück, das die Auswahl ohnehin
liefert.

Drei Änderungen tragen das, und keine ist ein Sonderfall:

1. **`Zerlegung::ereignis_verbuchen`** (neu, ersetzt `klammer_verbuchen`). Jedes Ereignis verbucht
   seinen Quellbereich beim umgebenden Element und schiebt dessen `innen_ab` und `innen_bis`
   zusammen. Gerufen wird es in `rendern`, für jedes Ereignis außer dem Ende — dessen Bereich ist
   der des Elements selbst, das sich gerade schließt.
2. **`klammer_der_raender`** (neu, reine Funktion). Vorspann oder Nachspann nicht bloß Leerraum,
   dann Klammer. Ein Element ohne ein einziges Ereignis darin — ein Stück fester Schrift in der
   Zeile — fällt mit seinem ganzen Quellbereich in beide Spannen, ohne dass es jemand aufzählen
   müsste: `innen_ab` steht auf `None` und liest sich als sein Ende.
3. **`Zerlegung::luecke_bis`** trägt die Lücken **innerhalb** eines Elements aus Zeichen jetzt
   ebenso ab wie den Vorspann eines Elements aus Blöcken. Geschrieben wird dabei weiterhin nichts,
   die Anzeige ändert sich nicht — aber die Kachel des Stücks dahinter steht danach Zeichen für
   Zeichen an ihrer Quelle (`Abschnittsart::Woertlich`) statt auf die Auszeichnungszeichen davor
   aufzurunden. Ohne diesen dritten Teil lieferte die Auswahl „vielen" in
   `Ein \* Stern im Absatz mit vielen Woertern.` weiterhin den halben Absatz.

Weggefallen ist `Abschnittsart::verdeckt_quelle`: die Art eines Abschnitts entscheidet über die
Klammer nichts mehr, und die Methode hatte danach keinen Rufer.

## Die zwei gemessenen Fälle

```
"Ein &amp; hier im Absatz mit vielen Woertern.\n"   Auswahl "vielen"
   vorher: der ganze Absatz          jetzt: "vielen"
"# **Titel** und noch ein Stueck Text\n"           Auswahl "noch ein"
   vorher: "noch ein"                jetzt: "# **Titel** und noch ein Stueck Text\n"
```

**Der zweite Fall weicht von der Sollangabe der Aufgabenstellung ab**, die
`"# **Titel** und noch ein"` nennt, also nur bis zum Ende der Auswahl. Gebaut ist die ganze
Überschrift, und zwar aus zwei Gründen, die beide binden:

- Der Datensatz `shared/decisions/260819-2216_a_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md`
  schreibt in Möglichkeit b die Regel wörtlich aus: „erweitere den Ausschnitt auf die
  **Vereinigung** der Quellbereiche aller Elemente, die er nur teilweise überdeckt". Die Vereinigung
  wächst an beiden Enden.
- Die vorhandene Probe `eine_auswahl_in_einer_ueberschrift_liefert_ihr_doppelkreuz` misst genau
  das: `"# Überschrift\n"` mit der Auswahl `"berschr"` liefert die Überschrift **samt** ihrem
  Zeilenumbruch am Ende. Eine Klammer, die nur nach vorn erweiterte, ließe diese Probe rot werden.

Eine Regel, die beide Sollangaben zugleich erfüllt, gibt es nicht. Gebaut ist deshalb die des
bindenden Datensatzes; die Sollangabe der Aufgabenstellung ist in ihrem tragenden Teil — die
Überschrift bringt ihr `# ` mit — erfüllt.

## Was gegengemessen ist

Beide neuen Proben sind **vor** der Behebung gefahren worden und waren rot:

```
eine_entitaet_oder_ein_escape_im_absatz_blaeht_die_auswahl_nicht_auf
  left: "Ein &amp; hier im Absatz mit vielen Woertern.\n"   right: "vielen"
eine_ueberschrift_mit_einem_kind_am_anfang_behaelt_ihr_doppelkreuz
  left: "noch ein"   right: "# **Titel** und noch ein Stueck Text\n"
```

Die dritte neue Probe, `der_quelltextblock_und_das_stueck_in_der_zeile_tragen_ihre_zeichen`, ist
keine Rückfallprobe, sondern schließt die Deckungslücke der sechs Fälle, die richtig bleiben
mussten: der Quelltextblock und das Stück in der Zeile hatten keine.

## Die sechs Fälle, die richtig bleiben mussten

Jeder einzeln nachgeprüft, jeder von einer Probe gehalten:

| Fall | Klammer | Probe |
|---|---|---|
| Überschrift | `# Titel\n` → `true` | `ueberschrift_betonung_verweis_und_punkt_…` |
| Betonung | `**fetter**` → `true` | dieselbe, dazu `eine_auswahl_im_verschachtelten_element_…` |
| Verweis | `[Verweis](…)` → `true` | dieselbe, dazu `eine_auswahl_im_text_eines_verweises_…` |
| Listenpunkt | Punkt `true`, Liste darum `false` | dieselbe |
| Zitat | Zitat `true`, Absatz darin `false` | dieselbe |
| Quelltextblock | `` ```rust … ``` `` → `true` | `der_quelltextblock_und_das_stueck_in_der_zeile_…` (neu) |

Dazu grün geblieben: das wörtliche Beispiel des bindenden Datensatzes, die Probe
`eine_auswahl_in_einem_langen_absatz_liefert_nicht_den_absatz`, die die verworfene Möglichkeit 3
ausschließt, die Kachelungsprobe über die zehn Beispiele und C2.8, die Auswahl über alles.

## Prosastellen, die nachgezogen sind

Die Runde hat neun falsch gewordene Prosastellen gefunden; eine zehnte bleibt hier nicht stehen.
Nachgezogen sind sechs Stellen in `markdown.rs`:

1. Modulkopf, Absatz „Der Vorspann eines Elements bekommt deshalb einen Abschnitt" → gilt jetzt
   für **jede** Lücke, und die Genauigkeit der Abbildung steht dabei.
2. Modulkopf, Absatz „Die Klammer" → Ränder statt „irgendwo verdeckte Bytes".
3. `Quellelement::klammer` → die Aufzählung nennt das Stück fester Schrift mit und schließt den
   Absatz mit Entität ausdrücklich aus.
4. `Zerlegung::luecke_bis` → der Absatz „Für den Quellbezug dreht sich das Vorzeichen" gilt jetzt
   an beiden Stellen.
5. `Zerlegung::schliessen`, Doc-Kommentar und Kommentar im Rumpf → die Reihenfolge „erst abtragen,
   dann abräumen" hängt nicht mehr an der Klammer, sondern am Merkzeichen.
6. Probe `eine_auswahl_in_einem_langen_absatz_liefert_nicht_den_absatz` → die Begründung nennt den
   neuen Rechenweg.

`CLAUDE.md` ist nicht angefasst (vom Auftrag ausgenommen) und nennt weder `klammer_verbuchen` noch
`verdeckt_quelle`; nachzuziehen war dort nichts.

## Prüfung

```
export PATH="$HOME/.cargo/bin:$PATH" && make check      → exit 0, „alle vier gruen"
```

`cargo build --workspace`, `cargo test --workspace` (736 Proben in `krk-ui`, drei mehr als vorher),
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`. Daneben
`cargo doc -p krk-ui --no-deps --document-private-items`: keine unaufgelöste Verweisstelle aus
`markdown.rs`.

## Nicht angefasst

Die vier übrigen Befunde der Durchsicht (zwei mittlere, zwei niedrige), die sieben
`_a_`-Entscheidungsdatensätze, `CLAUDE.md`. Kein Git-Kommando über den Baum; committet der
Orchestrator.
