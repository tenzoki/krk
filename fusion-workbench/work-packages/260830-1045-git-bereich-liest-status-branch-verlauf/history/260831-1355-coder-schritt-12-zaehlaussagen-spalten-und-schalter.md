# Schritt 12: Der Nachzug der Zählaussagen über Spalten und Schalter

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 12, Entscheidung 9
**Kriterien:** C9.4 (zweite Hälfte), C9.7

---

## Verification

```
make check — exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets`
unter `-D warnings`, `cargo fmt --all --check`.

---

## Die Erhebung, und warum ihr Muster ein zweites Mal wachsen musste

Das Erhebungsprogramm aus Schritt 11 ist unverändert nachgefahren: 438 Treffer in 71 Dateien,
dieselbe Zahl, die jener Schritt nach seiner Arbeit gemeldet hat. Davon tragen **97** eine
Aussage über Spalten, Schalter oder Ankreuzfelder.

**Diese 97 sind nicht die zweite Hälfte, sondern eine Untergrenze.** Der gefilte Defekt nennt
`crates/krk-ui/src/fenstermodell.rs:75` — „[`Spaltensichtbarkeit`] mit vier Feldern" — und das
Muster aus Schritt 11 findet die Zeile nicht: sein Trägerwortsatz führt `Ankreuzfeld`, aber
nicht `Feld`, und `Breite` überhaupt nicht. Die Falle ist dieselbe, die der Datensatz
`260830-1317_*_das-erhebungsmuster-…` für die erste Hälfte beschreibt, eine Runde später und
ein Wort weiter.

Der Trägerwortsatz ist für diese Hälfte deshalb um die Wörter erweitert, mit denen der Baum
Spalten und Schalter sonst benennt:

```python
NOUN = (r'(?:[Ss]palte|[Ss]chalter|[Aa]nkreuzfeld|[Ff]eld|[Kk]ästchen|[Kk]aestchen'
        r'|[Hh]äkchen|[Hh]aekchen|[Kk]nopf|[Kk]nöpfe|[Kk]noepfe|[Zz]elle|[Uu]mschalter'
        r'|[Ss]ichtbarkeit|[Bb]reite)')
```

Zahlwortsatz, Fensterbreite von zwei Zeilen, Abräumen des Kommentarvorsatzes und Leseumfang
sind die von Schritt 11 und unverändert.

| Erhebung | Treffer | Dateien |
|---|---|---|
| Schritt-11-Muster, davon Spalten/Schalter/Ankreuzfelder | 97 | 22 |
| erweiterter Trägerwortsatz | 166 | 39 |
| **Vereinigung beider, vor der Arbeit** | **167** | **39** |
| Vereinigung beider, nach der Arbeit | 138 | 38 |

Der Rückgang um 29 ist kein Maß der Arbeit: Stellen, die ihre Zahl gegen eine
Erhebungsvorschrift getauscht haben, fallen aus dem Muster, nachgezogene Zahlen bleiben darin.
**Jeder der 167 Treffer ist gelesen und einzeln entschieden**, kein Suchen-und-Ersetzen. Die
Differenz beider Erhebungsmengen ist nach der Arbeit geprüft: die 28 scheinbar neuen Treffer
sind ausnahmslos Zeilenverschiebungen bereits geprüfter Stellen, keine neue Aussage.

---

## Die drei namentlich verlangten Gegenstände

### 1. Der Modulkopf von `spalten.rs` nennt `Spalte::ALLE` (C9.7)

Der Kopf zählte die sieben Stellen auf, die der Übersetzer hält, und ließ die eine aus, die er
**nicht** hält. Er trägt jetzt einen eigenen Abschnitt dafür:

> **# Die eine Stelle, die der Uebersetzer nicht haelt: [`Spalte::ALLE`]**
> **Und sie ist die, die entscheidet, ob die neue Spalte ueberhaupt erscheint.**

Ausgeschrieben ist der Mechanismus und nicht bloß der Befund: wer eine sechste Variante anlegt
und die Fallunterscheidungen beantwortet, hat eine übersetzbare Datei, während
`pub const ALLE: [Spalte; 5]` stehen bleibt; die neue Spalte hat danach weder eine Zelle in der
Tabelle noch ein Ankreuzfeld in der Bereichsleiste, weil beide über diese Liste reihen.

**Der Befund, der beim Schreiben dazukam, ist die Richtung der Probe.** Die naheliegende
Sicherung, `tabs::tests::die_dateiliste_bleibt_flach_und_hat_fuenf_spalten`, hält
`Spalte::ALLE.len()` gegen eine hingeschriebene Zahl und fängt damit den **umgekehrten** Fall:
sie wird rot, wenn jemand die Liste erweitert, ohne die Probe nachzuziehen, und bleibt grün,
wenn jemand die Aufzählung erweitert, ohne die Liste nachzuziehen. Der Abschnitt sagt das so.
Dass eine quelltextlesende Probe wie bei `Kommando::KENNUNGEN` und `Marke::ALLE` hier nicht
geht — `krk-ui` hat kein Bibliotheksziel, jene stehen unter `crates/krk-core/tests/` —, steht
daneben, mit dem Zeiger auf die offene Nutzerfrage `260826-1811_*`. Es ist dieselbe Lage, die
Schritt 11 für `Fokus::ALLE` gefunden hat.

Im selben Zug ist die Aufzählung der gehaltenen Stellen um die vier ergänzt, die der Übersetzer
in der Git-Runde **außerhalb** dieser sieben genannt hat (`spalte_sichtbar_in`,
`spalte_umschalten`, `kommando_der_spalte`, ein Strukturliteral in
`crates/krk-core/tests/ablage.rs`; belegt im History-Eintrag von Schritt 2), und um das
Zählkommando `grep -rn 'Spalte::Marke =>' crates/krk-ui/src`, damit die Sieben nicht mit der
nächsten Fallunterscheidung falsch wird.

### 2. Die Zahl der Ankreuzfelder tritt hinter eine Erhebungsvorschrift zurück

Der Modulkopf von `appkit/bereichsleiste.rs` beginnt nicht mehr mit „zehn Ankreuzfelder"
(zwölf sind es heute), sondern mit „Ankreuzfelder, sonst nichts" und darunter der
Zusammensetzung: je einer je Bereich über `Bereich::ALLE`, je einer je schaltbarer Spalte über
`Spalte::ALLE` mit `kommando_der_spalte`, dazu die zwei Sucheinstellungen. Gezählt werden sie
mit `Bereichsleiste::alle_schalter().len()`, das über dieselben zwei Aufzählungen läuft, aus
denen `Bereichsleiste::bauen` baut; die Probe
`zwoelf_schalter_der_leiste_tragen_ein_kommando` hält den heutigen Stand. Der Absatz sagt
ausdrücklich, warum keine Zahl dasteht: sie ist seit der Bereichsleisten-Runde viermal
gewachsen.

Dieselbe Form haben zehn weitere Stellen derselben Datei bekommen, dazu die zwei Schreiber und
die Kommandoliste in `appkit/anwendung.rs` und die Kurznamenregel in `fenstermodell.rs:302`
(„neun Schalter"). Das Vorbild sind die zwei Stellen, die Schritt 11 schon so umgestellt hat
(`kommandos/loeschwarnung.rs`, `appkit/mod.rs`).

### 3. Der Doc-Kommentar an `Bereichsleiste::spaltenschalter` stimmt nach Schritt 2 und Schritt 8

Geprüft, nicht angenommen. Der Abschnitt `# Was die Feldbreite haelt, und was sie nicht haelt`
sagt weiter richtig, dass die Zahl eine Zusicherung zur **Laufzeit** ist und keine Bedingung
des Baus: das Feld entsteht aus einer gefilterten Liste mit `try_into` und nicht über
`ALLE.map(…)`, das `expect` in `Bereichsleiste::bauen` bricht beim Start ab. Die vier Stellen,
die Schritt 8 zusammen von drei auf vier gehoben hat, stehen alle vier im Baum und tragen die
Vier: die Feldbreite (`:492`), das `Vec::with_capacity(4)` (`:539`), der Text des `expect`
(`:567`) und die Zählprobe `genau_vier_spalten_sind_schaltbar` (`:900`).

**Gefallen ist allein die Kopfzeile darüber**, „Die Schalter der **drei** schaltbaren Spalten":
sie ist Schritt-8-Rückstand und steht jetzt ohne Zahl, weil der Typ `[Retained<NSButton>; 4]`
zwei Zeilen weiter ohnehin die Zahl trägt.

---

## Was sonst angefasst ist

39 Prosastellen in zehn Quelldateien:

`crates/krk-ui/src/appkit/bereichsleiste.rs` (11), `crates/krk-ui/src/appkit/anwendung.rs` (10),
`crates/krk-ui/src/appkit/tabelle.rs` (6), `crates/krk-ui/src/fenstermodell.rs` (4),
`crates/krk-core/tests/ablage.rs` (3), `crates/krk-ui/src/spalten.rs` (1),
`crates/krk-ui/src/belegungsausgabe.rs` (1), `crates/krk-ui/src/belegungsmodell.rs` (1),
`crates/krk-ui/src/appkit/leiste.rs` (1), `crates/krk-core/src/tasten/belegung.rs` (1).

**Wo die neue Zahl steht, weil sie an `Spalte::ALLE` oder `Bereich::ALLE` hängt** (die Regel aus
Entscheidung 9): die Skizze im Modulkopf von `appkit/tabelle.rs` und ihre fette Markierung
(`vier` → `fuenf Spalten`), `anwendung.rs:1595` und `:4580`, `appkit/leiste.rs:168` — dort stand
„vier Spalten", während die Schwesterstelle in `appkit/git.rs:416` schon „fuenf" sagt —, und
`fenstermodell.rs:2531`, wo „die Summe der fuenf Breiten" gegen ein `[f64; 6]` stand.

**Wo eine Erhebungsvorschrift oder eine zahlfreie Form an die Stelle der Zahl tritt**, weil sie
mit der nächsten Spalte oder dem nächsten Bereich wieder falsch würde: die elf Stellen der
Bereichsleiste, die beiden Verweise „denselben Weg gehen die drei Spaltenschalter" in
`anwendung.rs`, „die eine Stelle, durch die alle drei Spaltenbefehle gehen", die Gruppenregel im
Doc von `Kommando::SpalteGroesseUmschalten` (auf die `SpalteMarkeUmschalten` für „Alles Weitere"
verweist, weshalb ihre Drei definitiv falsch war), die zwei Verweise in `belegungsmodell.rs:297`
und `:299`, `belegungsausgabe.rs:667`, `fenstermodell.rs:302` und `:489`, sowie die drei
Feld- und Breitenaussagen in `crates/krk-core/tests/ablage.rs`.

**Zwei Messaussagen sind kalibriert statt nachgezogen.** `tabelle.rs:4518` sagte „Alle vier
Spalten ruecken gleich weit ein, und das ist gemessen", gemessen am 260811 an vier Feldern; die
Aussage gilt jeder Spalte, die Messung deckt die damaligen vier, und der Text sagt jetzt
beides, samt dem Hinweis, dass die Markenspalte durch dieselbe Zeile geht und nicht eigens
nachgemessen ist. Ebenso `tabelle.rs:5155`: die 603 Punkte sind am 260812 an vier Spalten
gemessen, und dass die Markenspalte seither hinzukommt, steht jetzt daneben.

---

## Bewusst stehengelassen

- **Historische Aussagen.** „Die drei Spaltenschalter der Bereichsleisten-Runde"
  (`crates/krk-core/tests/belegung.rs:90`), „die drei Felder der Bereichsleisten-Runde"
  (`tests/ablage.rs:880`), „Die vier Spalten der Bereichsleisten-Runde enden … bei 603 Punkten"
  (`tabelle.rs:5470`, von Schritt 2 schon um den Nachsatz zur fünften ergänzt). Sie benennen
  einen damaligen Stand.
- **Datierte Ketten, die sich selbst fortschreiben.** `belegungsausgabe.rs:532` sagt „Bis zum
  260812 war die Antwort darauf »keine«; seither sind es die drei Spaltenschalter" und trägt
  zehn Zeilen weiter „Seit dem 260831 tritt `spalte_marke_umschalten` hinzu, der vierte". Der
  Absatz ist als Ganzes richtig; ein halber Nachzug hätte ihn zerrissen. Dasselbe gilt der
  Konstante `OHNE_KOMBINATION_AB_WERK` in `crates/krk-core/tests/belegung.rs`, die sechs
  Einträge führt und ihre Zugänge einzeln datiert.
- **Ortsgebundene Verweise, die zufällig genau stimmen.** „wie die drei Spaltenschalter
  darueber" in `krk-core/src/tasten/belegung.rs:484` und `:507` und in
  `belegungsmodell.rs:441`: `SpalteMarkeUmschalten` steht in beiden Dateien am **Ende** der
  Aufzählung und damit unter der genannten Stelle, nicht darüber. Die Nachbarstellen `:297` und
  `:299` in `belegungsmodell.rs` stehen dagegen unter dem Markeneintrag und sind gefallen.
- **Andere Gegenstände unter demselben Wort.** Die drei Spalten der Markdown-Ausgabe
  (`belegungsausgabe.rs`, fünf Stellen), die zwei Spalten der Belegungsansicht
  (`belegungsmodell.rs`, `appkit/belegungsansicht.rs`), die drei Spalten des Blattes zum
  Stapelumbenennen, die vierte Spalte der Doc-Tabelle in `kommandos/abwurfregel.rs`, die vier
  Meldungsfelder eines Dateifensters (`appkit/statuszeile.rs`, `appkit/tabelle.rs`), die neun
  Felder der Wahrheitstafel in `krk-core/src/verzeichnis/loeschzielbefund.rs`, die elf Felder
  der Zeitzerlegung in `verzeichnis/sys.rs`, die Feldbausteine des Leseprofils, die sechs
  Felder eines `Commit` (`git/leser.rs:441`, nachgezählt) und die vier Felder von `Lage`
  (`kommandos/zulaessigkeit.rs:174`, nachgezählt).
- **`resources/default-keymap.toml`.** Zwei Kommentare dort sind unrichtig; die Datei gehört
  dem `ontocoder`. Siehe unten.
- **`CLAUDE.md`.** Schritt 14, wie der Plan sie zuweist. Nicht angefasst.

---

## Drei Berichtigungen an der Vorlage

Der gefilte Defekt zählt die Stellen namentlich auf, und der Auftrag sagt ausdrücklich, seine
Liste sei ein Stand von gestern Mittag. Drei seiner Einträge halten dem Lesen nicht stand, und
alle drei stehen unverändert im Baum:

1. **`fenstermodell.rs:75`, „[`Spaltensichtbarkeit`] mit vier Feldern", ist richtig.** Der
   Defekt behauptet, die Struktur trage seit Schritt 2 fünf Felder. Sie trägt vier
   (`crates/krk-core/src/ablage/sitzung.rs:313-327`: `groesse`, `geaendert`, `typ`, `marke`);
   die Namensspalte hat kein Feld, weil sie sich nicht wegschalten lässt. Was daneben
   **unrichtig** war und der Defekt nicht führt: `fenstermodell.rs:489` rechnete mit „acht
   Kombinationen der drei Felder".
2. **`belegungsausgabe.rs:532` ist richtig**, weil der Satz datiert ist und fortgeschrieben
   wird. Unrichtig war `:667` in derselben Datei, das der Defekt nicht führt.
3. **`crates/krk-core/tests/belegung.rs:90` ist eine historische Aussage** und bleibt nach der
   Regel stehen, die der Auftrag selbst ausschreibt.

Beide Erhebungen sind gefahren, bevor eine dieser drei Entscheidungen gefallen ist; entschieden
hat jeweils der Baum und nicht die Liste.

---

## Zwei Datensätze außerhalb dieses Schritts

**Geschlossen mit diesem Schritt:**

`issues/260831-1212_c_die-zaehlaussagen-ueber-spalten-und-schalter-stehen-in-sieben-dateien-die-schritt-12-nicht-fuehrt.md`
— mit einer `Resolved:`-Zeile, die die drei Berichtigungen oben und die Stellen nennt, die keine
der beiden Dateilisten führte.

**Neu gefilt:**

`issues/260831-1355_o_zwei-kommentare-in-default-keymap-toml-nennen-drei-spaltenschalter-und-darueber-stehen-vier.md`
— `resources/default-keymap.toml:464` und `:478` sagen „wie die drei Spaltenschalter darueber",
und seit Schritt 9 stehen dort vier. Die Datei ist die eine Quelle jeder Tastenbelegung und
gehört dem `ontocoder`; dieser Schritt hat sie deshalb nicht angefasst. Der Kommentar am
Markeneintrag selbst (`:458`, „wie die drei darueber") ist richtig und bleibt.

---

## Zum Stand der Erhebung nach diesem Schritt

Für die zweite Hälfte liefert die Erhebung im Code keine unrichtige Aussage mehr. **Die eine
verbleibende Ausnahme ist benannt und liegt außerhalb dieses Schritts**:
`resources/default-keymap.toml:464` und `:478`, gefilt für den `ontocoder`. Solange sie steht,
ist die Abnahmebedingung des geschlossenen Defekts für den Baum erfüllt und für die
Belegungsdatei nicht.

---

**Keine Codezeile geändert.** Geprüft mit
`git diff -U0 -- crates/ | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' | grep -vE '^[+-]\s*(//|///|//!)'`:
keine Ausgabe. Der Diff besteht aus 113 hinzugefügten und 60 entfernten Kommentarzeilen.

**Kein whole-tree-git-Kommando abgesetzt.** Kein Commit; der Orchestrator committet.
