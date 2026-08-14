Zwei in C3 zugesagte Proben stehen nicht im Baum

---

Zwei Zusagen der Fähigkeit C3 nennen ausdrücklich eine Probe, und beide Proben gibt es nicht.
Die Sache, die sie halten sollen, hält jeweils — nachgewiesen ist sie durch Lesen und nicht
durch einen Lauf.

| Zusage | Wo sie steht | Was im Baum steht |
|---|---|---|
| „Eine Probe hält fest, dass die Regel nach dieser Runde genau **eine** Ausnahme kennt, nämlich die Textfläche des Editors." | Spec, C3, erstes Kriterium der ersten Liste | `die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle` (`crates/krk-ui/src/appkit/ereignisse.rs:737`) zählt, dass die Regel **einmal erklärt** ist und die Typprüfung in **einer Datei** steht. Über die Zahl der Ausnahmen sagt sie nichts. |
| „eine Zählprobe über `krk_ui::quellbaum::quelldateien` hält fest, dass `zettel.rs` weder `Nummernspalte` noch `hervorhebung` noch `suche` nennt" | Plan, Schritt 11, Zeile `Prüfung` | Das Prüfmodul von `crates/krk-ui/src/appkit/blaetter/zettel.rs` führt drei Proben: `der_waechter_nimmt_die_escape_taste_und_nicht_die_eingabetaste` (`:502`), `nur_die_zwei_stellen_des_schalters_sind_zettel` (`:523`), `jeder_zettel_traegt_eine_beschriftung` (`:536`). Eine Zählprobe der ausgeschlossenen Fähigkeiten ist nicht darunter und steht auch sonst nirgends im Baum. |

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Beide Sachen halten am 260814-1002 nachgelesen:
`ersthelfer_gehoert_appkit` (`ereignisse.rs:581`) trägt genau eine Ausnahme, den Rückruf
`ist_editorflaeche`, und der eine Aufrufer reicht ihn aus `Anwendungsdelegierter::lage`
herein (`anwendung.rs:2727-2729`); die Textfläche des Zettels ist nirgends angemeldet.
`blaetter/zettel.rs` ruft weder Nummernspalte noch Hervorhebung noch Suche, und keine davon
steht in seinen `use`-Zeilen (`:106-122`).

**Die zweite Zusage ist so, wie sie dasteht, nicht baubar.** Eine Nadel auf die drei Wörter
fiele an der Datei selbst: ihr Modulkopf sagt an fünf Stellen, dass es keine Nummernspalte,
keine Syntaxhervorhebung und keine Suche gibt (`:9`, `:71`, `:445`, `:543`, `:551`). Wer die
Probe nachträgt, sucht den **Aufruf** und nicht das Wort — die Bauform dafür führt der Baum
mit `keine_ansicht_ueberschreibt_keydown` (`ereignisse.rs:780`) schon vor, wo die Nadel die
Anmeldeform einer Methode trägt und die Erwähnung im Doc-Kommentar deshalb nicht mitzählt.

**Es ist derselbe Befund, den der Abgleich der Runde 8 in größerer Zahl abgelegt hat.**
Dort trugen neun von 48 am Baum nachweisbaren Kriterien die Kennzeichnung „(Probe)" und
hatten keine (`circles/260813-0939-.../history/260813-1345-reconciliation.md`, Abschnitt 2).
Die Runde 9 steht mit zwei von 43 wesentlich besser da; die Bauart ist dieselbe.

**Kontext**

- Gefunden beim Abgleich der Runde 9, `history/260814-1002-reconciliation.md`.
- Zeilennummern am Stand `79dab20` gezählt.
