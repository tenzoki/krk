# Schritt 11: Der Nachzug der Zählaussagen über Bereiche und Fokuswerte

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 11
**Kriterien:** C9.4 (erste Hälfte), C9.5, C9.6, C9.8

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`.

---

## Erst das Muster, dann erheben, dann zählen

Die dritte Bedingung aus `## Stops when` verlangt diese Reihenfolge, und sie ist eingehalten.

### Zwei Erweiterungen, nicht eine

**Die Wortformen allein hätten das Gegenbeispiel nicht gefunden.** Der Defekt
`260830-1317_*_das-erhebungsmuster-…` nennt zehn fehlende Wortformen und `belegungsausgabe.rs`
als Beleg. Beim Nachlesen der Stelle stellte sich heraus: „die sieben" und „Beschriftungen"
stehen in **zwei aufeinanderfolgenden Zeilen** desselben Doc-Kommentars. Ein zeilenweises
`grep` findet das nie, gleich welche Wortformen es führt. Die Zeilengrenze ist die zweite
Falle neben der zu engen Wortliste, und sie ist die härtere: sie lässt sich mit keinem
`grep`-Muster schließen.

Das erweiterte Muster ist deshalb kein `grep` mehr, sondern ein Programm mit einem
**Fenster von zwei Zeilen**, das den Kommentarvorsatz (`//`, `///`, `//!`, `#`) der
Folgezeile abräumt, bevor es vergleicht, und einen Treffer nur meldet, wenn er in der
eigenen Zeile beginnt.

**Die erste Erweiterung geht über die zehn Wortformen des Defekts hinaus.** Das Muster aus
C9.4 suchte je Aussage die *heutige* Zahl mit *einem* Trägerwort („fuenf Bereiche",
„sechster Bereich"). Damit entging ihm jede Aussage mit einer anderen Zahl **und** jede mit
einem anderen Wort für dieselbe Sache. Das erweiterte Muster lässt stattdessen

- **jedes Zahlwort in jeder Form** zu: `zwei` bis `zwoelf`, Grundzahl und Ordnungszahl, mit
  und ohne Umlaut (`fünf`/`fuenf`, `zwölf`/`zwoelf`), dazu `genau ein…` als die eine Form,
  in der eine Eins eine Zählaussage ist;
- **die Trägerwörter der Aufzählungen**, um die es geht — `Bereich`, `Fokus`,
  `Wirkungsbereich`, `Spalte`, `Schalter`, `Ankreuzfeld` — und daneben die Wörter, mit
  denen der Baum dieselben Mengen sonst benennt: `Beschriftung`, `Kasten`, `Rahmen`,
  `Teilbaum`, `NSBox`, `Fläche`, `Rang`, `Wert`, `fokussierbar`;
- **bis zu zwei Wörter dazwischen**, damit „fuenf sichtbare Bereiche" und „jeder der sechs
  Bereichsschalter" mitkommen.

Die vom Plan namentlich verlangten Formen sind darin aufgegangen: `sieben Beschriftungen`,
`acht Wirkungsbereiche`, `vier fokussierbaren`, `fuenf Kaesten`, `fuenf Rahmen`,
`fuenf Teilbaeume`, `fuenf Werten`, `fuenf Bereichen`, `sechs Bereiche` — und mit ihnen
jede weitere Kombination derselben Bauform, die der Plan nicht aufzählen konnte.

**Der unbenutzte dritte Versuch gehört zum Befund.** Ein erstes erweitertes Muster nahm den
Trägerwortsatz weit (Zeile, Eintrag, Variante, Stelle, Feld, Aufzählung …) und lieferte
1 063 Treffer in 126 Dateien — je „drei Zeilen" und „zwei Wege" der ganzen Prosa. Eine
Erhebung, die niemand lesen kann, entscheidet nichts. Der Satz ist deshalb auf die
Trägerwörter der fraglichen Aufzählungen zurückgeschnitten, und der unbestimmte Artikel
(`ein`, `eine`, `einen` …) ist herausgenommen: er ist keine Zählaussage und stellte allein
523 der 1 063 Treffer.

### Das Erhebungsprogramm

```python
NUM  = (r'(?:(?:zwei|drei|vier|fünf|fuenf|sechs|sieben|siebent|acht|neun|zehn|elf|zwölf'
        r'|zwoelf)(?:te[rnms]?|t[erns]|e[rnms]?|s)?|genau ein(?:e|en|em|er)?)')
NOUN = (r'(?:[Bb]ereich|[Ff]okus|[Ww]irkungsbereich|[Ss]palte|[Ss]chalter|[Aa]nkreuzfeld'
        r'|[Bb]eschriftung|[Kk]asten|[Kk]ästen|[Kk]aesten|[Rr]ahmen|[Tt]eilbaum'
        r'|[Tt]eilbäume|[Tt]eilbaeume|NSBox|[Ff]läche|[Ff]laeche|[Rr]ang|[Rr]änge'
        r'|[Rr]aenge|[Ww]ert|fokussierbar)')
PAT  = re.compile(r'\b' + NUM + r'\b[ ,]+(?:\S+ +){0,2}' + NOUN)
```

Gelesen wird über `crates/`, `resources/`, `xtask/`, `README.md`, `CLAUDE.md` und
`Cargo.toml`, Endungen `.rs`, `.toml`, `.md` — derselbe Umfang wie in C9.4. Verglichen wird
je Zeile gegen die Zeile **plus** ihre um den Kommentarvorsatz gekürzte Nachfolgerin;
gemeldet wird, wenn der Treffer in der eigenen Zeile beginnt.

### Die Zahlen

| Erhebung | Stand | Treffer | Dateien |
|---|---|---|---|
| Muster aus C9.4 | `2059138`, vor der Runde | 92 | 21 |
| Muster aus C9.4 | nach Schritt 10 | 57 | — |
| **erweitertes Muster** | **nach Schritt 10** | **462** | **71** |
| erweitertes Muster | nach Schritt 11 | 438 | 71 |

Die 57 des alten Musters sind in den 462 restlos enthalten (`comm` gegen beide Ausgaben:
null Zeilen, die das alte findet und das neue nicht). Der Rückgang von 462 auf 438 ist kein
Maß der Arbeit: mehrere Stellen haben ihre Zahl gegen eine Erhebungsvorschrift getauscht
und fallen damit aus dem Muster, andere sind von „fuenf" auf „sechs" gezogen und stehen
weiter darin.

Von den 462 tragen **199** eine Aussage über Bereiche, Fokuswerte oder Wirkungsbereiche und
sind damit die erste Hälfte aus Entscheidung 9. **58 davon hat dieser Schritt angefasst.**

---

## Was angefasst ist, und was bewusst nicht

### Die vier namentlich verlangten Gruppen

**1. `appkit/bereichsleiste.rs`, Modulkopf (C9.5).** Der Satz „`Fokus` bekommt deshalb
keinen sechsten Wert, sondern der Fall wird ausgeschlossen" ist gefallen. An seiner Stelle
steht der Grund, und der Grund ist der Ansichtsbaum und nicht die Zahl: die Leiste ist
keine Unteransicht der `NSSplitView` und liegt damit in keinem der Teilbäume, die
`ersthelferbereich` durchgeht; der Git-Bereich **ist** eine und liegt in einem. Wer hier den
Ersthelferrang hielte, bekäme eine falsche Auskunft über den Fokus, wer ihn dort hält, eine
richtige. Der Absatz sagt ausdrücklich, dass die Leiste auch dann keinen Fokuswert bekäme,
wenn die Fensterzeile weiter wächst.

**2. Die fünf Stellen zum „sechsten Bereich" (C9.6).** `appkit/statuszeile.rs` (Modulkopf,
zwei Stellen), `appkit/fenster.rs` (vier Stellen), `appkit/anwendung.rs:1332` und
`appkit/titelzusatz.rs`. Sie sagen dasselbe über den **siebten**. Zwei Zusätze:

- `statuszeile.rs` trägt jetzt denselben Grund wie `bereichsleiste.rs`, damit die zwei
  Schwestern der Aufteilung ihn beide ausschreiben und niemand aus der Zahl schließt.
- `titelzusatz.rs` sagte „`Bereich` wie `Fokus` bleiben bei fuenf Werten". Der Satz ist
  ersetzt: der Titelleisten-Bereich hat weder `Bereich` noch `Fokus` je einen Wert gebracht,
  und **eine Zahl steht dort nicht mehr**, weil sie mit dem Gegenstand des Absatzes nichts
  zu tun hat.

**3. Die Feldbreiten-Behauptung (C9.8).** Sechs Stellen, davon vier schon von Schritt 1
nachgezogen; die drei in `kommandos/fokus.rs` und die sechste in `kommandos/zulaessigkeit.rs`
sind hier gefallen. Sie behaupten keine Sicherung mehr, sondern sagen, was gemessen hält:

- `Bereich::ALLE.map(…)` hält den Bau, weil die Länge des Feldes aus der Liste folgt —
  `Bereichsleiste::bereichsschalter`, die eine Stelle im Baum, die es so baut;
- ein Literal (`Aufteilung::rahmen`) hält ihn **nicht** und bricht zur Laufzeit am Index;
- ein `[0.0; N]` (`Aufteilung::gemessene_breiten`) hält ihn **nicht**, ebenso;
- ein fester Parameter (`Fenstermodell::breiten_uebernehmen`) hält **gar nichts**, beide
  Seiten bleiben stumm.

**Und der Befund, der diese Runde am teuersten war, steht jetzt im Baum:** die Coder der
Schritte 1 und 2 haben zusammen **neun** stille Stellen von Hand nachgezogen, und der
Übersetzer hat keine einzige davon genannt (vier in `260830-1421-coder-schritt-1-…`, fünf in
`260830-1447-coder-schritt-2-…`).

**Eine Berichtigung an der Vorlage.** Der Auftrag ließ offen, was die Vollständigkeit von
`Fokus::ALLE` stattdessen hält. Die naheliegende Antwort — die Probe
`die_aufzaehlung_der_fokuswerte_ist_vollstaendig_und_doppelt_keinen` — ist falsch: ihr Rumpf
führt seine **eigene** Liste derselben sechs Werte und fängt einen siebten ebenso wenig.
Anders als bei `Kommando::KENNUNGEN` und `Marke::ALLE`, deren Zählproben die Varianten aus
dem **Quelltext** der Aufzählung lesen, kann `Fokus::ALLE` keine solche Probe haben: die
stehen unter `crates/krk-core/tests/`, und `krk-ui` hat kein Bibliotheksziel. Der
Doc-Kommentar sagt das jetzt so und zeigt auf die offene Nutzerfrage `260826-1811_*`, statt
ihr vorzugreifen.

**4. Das Gegenbeispiel aus `belegungsausgabe.rs:233-239`.** „die sieben Beschriftungen von
`Wirkungsbereich`" ist ersetzt durch „je eine Beschriftung von `Wirkungsbereich`" plus das
Zählkommando `awk '/^pub enum Wirkungsbereich/,/^}/' crates/krk-core/src/tasten/belegung.rs`
und den Hinweis, dass die Sieben schon mit `Wirkungsbereich::Vorschau` aus der Runde 20
falsch geworden war.

### Ein Befund, den der Auftrag nicht vorhersah: zwei Skalen unter einem Wort

`kommandos/fokus.rs` zählte den Editor als „fünften fokussierbaren Bereich" und den
Git-Bereich als „sechsten fokussierbaren Ort", während `appkit/anwendung.rs` und
`appkit/mod.rs` die Leiste als „zweiten fokussierbaren Bereich" und die Vorschau als
„dritten" führen. Beides zugleich geht nicht: auf der Skala der fokussierbaren Orte ist der
Editor der **vierte** (die beiden Dateilisten teilen sich einen Wert), auf der Skala der
Bereiche der Fensterzeile der fünfte. `fokus.rs` führte still die zweite Skala unter dem
Wort der ersten. Beide Doc-Kommentare stehen jetzt auf der Fokus-Skala, und der am Editor
schreibt den Unterschied der zwei Skalen aus.

### Wo eine Zahl gegen eine Erhebungsvorschrift getauscht ist

Fünf Stellen, jede nach der Regel „wo eine Zahl mit der nächsten Runde wieder falsch würde":

- `fenstermodell.rs:60-63` — „alle fuenf Schalter der Bereichsleiste" → jeder
  Bereichsschalter, ohne Zahl; sie folgt der Zahl der Bereiche und ist seit jenem
  Nutzerentscheid zweimal gestiegen.
- `kommandos/fokus.rs`, Modulkopf — „Die drei Fokusbefehle" → einer je fokussierbarem Ort,
  also `Fokus::ALLE` ohne `Fokus::Anderswo`.
- `appkit/titelzusatz.rs` — siehe oben.
- `kommandos/loeschwarnung.rs:249-255` und `appkit/mod.rs:82-86` — beide führten „fünf
  Bereiche" und die Schalterzahl in **einem** Satz. Ein halber Nachzug hätte den Satz in
  sich widersprüchlich gelassen (6 + 3 + 2 ≠ 10), also trägt er jetzt die Zusammensetzung
  und keine Zahl — genau die Form, die Entscheidung 9 für die Ankreuzfelder der
  Bereichsleiste vorsieht.
- `appkit/bereichsleiste.rs:147-155` (`GRUPPENABSTAND`) — „die fuenf die Flaechen, die drei
  die Spalten" → die einen, die anderen, ohne Zahlen.

Alle übrigen Zahlen dieses Schritts sind an `Bereich::ALLE` (sechs) oder `Fokus::ALLE`
(sechs) gebunden und stehen deshalb als Zahl da, wie Entscheidung 9 es vorsieht.

### Bewusst stehengelassen

- **Historische Aussagen.** „die vier Bereiche der Runde 1" (`fenstermodell.rs` viermal,
  `appkit/fenster.rs` zweimal, `ablage/sitzung.rs` zweimal), „hat den fuenften Bereich der
  Editor-Runde stumm aufgenommen", „Bis Schritt 18 gab es genau einen fokussierbaren
  Bereich", „der Unterschied der beiden Antworten war genau ein Bereich: der Editor". Sie
  benennen einen damaligen Stand und sind als solche richtig.
- **Rechnungen über eine Lage.** „die vier sichtbaren Bereiche wuenschen zusammen 1480"
  (`fenstermodell.rs` zweimal): Lesezeichen 180 + Links 420 + Rechts 420 + Editor 460, bei
  ausgeblendeter Vorschau und ausgeblendetem Git-Bereich. Die Rechnung stimmt.
- **Richtige Zahlen.** „die drei rechten Bereiche" (Vorschau, Editor, Git), „die drei
  Bereiche, die ihre Flaeche allein beanspruchen", „sechs Bereiche und fuenf fokussierbare
  Orte", „acht Wirkungsbereiche mal sechs Fokuswerte", „die fuenf anderen
  Bereichsumschalter und die vier anderen Fokusbefehle", „die drei Flaechen des
  Git-Bereichs", „sechs Fokuswerte, sechs Ausgaenge".
- **Andere Gegenstände unter demselben Wort.** „vier Bereiche mit demselben Einzug"
  (`markdown.rs`, Einrückung), „als vierte Flaeche den Anwendungsdelegierten"
  (`appkit/menue.rs`, Helferkette), „dieselben sieben Beschriftungen"
  (`tests/leseprofil.rs`, Leseprofil), „zwei Flaechen" (`appkit/editor.rs`,
  `appkit/textautomatik.rs`).
- **`CLAUDE.md`.** Zwei Stellen sind unrichtig — `:145` sagt „die fünf Rahmenfarben der
  Aufteilung" und „eine reine Funktion über die fünf Fokuswerte"; beide sind sechs. Die
  Datei gehört Schritt 14, der Plan weist sie ihm ausdrücklich zu, und dieser Schritt hat
  sie nicht angefasst. (`:79`, „der fünfte Bereich der Fensterzeile" für den Editor, ist
  richtig.)
- **Spalten und Schalter.** Aussage von Schritt 12. Wo sie sich nicht vom selben Satz
  trennen ließen, sind sie hier mitgefallen (siehe oben); die übrigen stehen.

---

## Drei Datensätze außerhalb dieses Schritts

**Geschlossen mit diesem Schritt:**

1. `shared/issues/260830-1006_c_fuenf-prosastellen-behaupten-eine-feldbreite-halte-den-bau-an-…`
   — mit einer `Resolved:`-Zeile, die alle sechs Stellen mit ihrem heutigen Wortlaut nennt.
   Die offene Nutzerfrage `260826-1811_*` nach der Bauform ist unberührt.
2. `issues/260830-1317_c_das-erhebungsmuster-aus-c9-4-ist-zu-eng-…` — mit den Zahlen und dem
   Befund, dass die Zeilengrenze die härtere der beiden Fallen war.

**Neu gefilt:**

3. `shared/issues/260831-1212_o_kontextmenue-rs-behauptet-eine-feldbreite-halte-den-bau-an-…`
   — die **siebte** Stelle derselben Art, `kommandos/kontextmenue.rs:204-207` zu
   `Kontextbefehl::ALLE`. Sie handelt weder von `Bereich` noch von `Fokus`, liegt damit
   außerhalb der Aussage dieses Schritts, und die Runde 23 hat sie nicht falsch gemacht.
4. `issues/260831-1212_o_die-zaehlaussagen-ueber-spalten-und-schalter-stehen-in-sieben-dateien-die-schritt-12-nicht-fuehrt.md`
   — die Dateiliste von Schritt 12 stammt aus der Erhebung mit dem alten Muster; das
   erweiterte findet dieselbe Sorte Aussage in `fenstermodell.rs`, `appkit/anwendung.rs`,
   `appkit/leiste.rs`, `belegungsausgabe.rs` und `crates/krk-core/tests/belegung.rs`, die
   der Schritt nicht führt. Gefilt, damit die Stellen nicht zwischen den zwei Dateilisten
   hindurchfallen.

**Ergänzt:**

5. `shared/issues/260826-1420_o_zwei-probenkoepfe-in-statuszeile-rs-zaehlen-fuenf-raenge-…`
   — `Also seen:` mit fünf weiteren Stellen außerhalb von `statuszeile.rs`, die die Ränge
   der Statuszeile falsch zählen (`Rang::ALLE` trägt sieben). Kein Nachzug hier: die Ränge
   sind nicht die Aussage dieses Schritts, und die Runde hat keine dieser Stellen falsch
   gemacht.

---

## Dateien

17 Quelldateien, 58 Prosastellen:

`crates/krk-ui/src/appkit/anwendung.rs` (16), `crates/krk-ui/src/kommandos/fokus.rs` (11),
`crates/krk-ui/src/fenstermodell.rs` (6), `crates/krk-ui/src/appkit/fenster.rs` (4),
`crates/krk-core/src/tasten/belegung.rs` (3), `crates/krk-ui/src/appkit/aufteilung.rs` (2),
`crates/krk-ui/src/appkit/statuszeile.rs` (2), `crates/krk-ui/src/appkit/bereichsleiste.rs` (2),
`crates/krk-ui/src/appkit/mod.rs` (2), `crates/krk-ui/src/appkit/teilen.rs` (2),
`crates/krk-core/src/ablage/sitzung.rs` (2), `crates/krk-ui/src/appkit/titelzusatz.rs` (1),
`crates/krk-ui/src/fenstertitel.rs` (1), `crates/krk-ui/src/kommandos/zulaessigkeit.rs` (1),
`crates/krk-ui/src/main.rs` (1), `crates/krk-ui/src/belegungsausgabe.rs` (1),
`crates/krk-ui/src/kommandos/loeschwarnung.rs` (1).

Keine Codezeile geändert; der Diff besteht aus Doc-Kommentaren und Kommentaren.

**Kein whole-tree-git-Kommando abgesetzt.** Kein Commit; der Orchestrator committet.
