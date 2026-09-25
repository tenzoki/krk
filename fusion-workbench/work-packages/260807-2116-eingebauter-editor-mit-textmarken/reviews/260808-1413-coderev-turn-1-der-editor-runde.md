# Codereview: Turn 1 der Editor-Runde

**Sender:** coderev
**Umfang:** `git diff 4e86c02..HEAD`, sieben Commits, 20 Programm- und Prüfdateien
**Maßstab:** Spec `260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`, Plan
`260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` (S1, S3, S8, S11, S13, S14,
S19 teilweise, S32), die acht Entscheidungsdatensätze unter `decisions/`, `CLAUDE.md`

---

## Zusammenfassung

Der Turn ist sauber. Alle vier Abnahmekommandos laufen grün (`cargo test --workspace`,
`cargo clippy --workspace --all-targets`, `cargo fmt --all --check`, Bau), die
Grenze `#![deny(unsafe_code)]` steht unverändert bei zwei Ausnahmedateien, und
kein Entscheidungsdatensatz ist verletzt. Sechs Befunde, keiner kritisch, keiner
hoch. Der schwerste ist ein Zwischenstand, den die Aufteilung nicht abfängt: ein
sichtbarer Editor ohne Unteransicht verliert seine zugeteilte Breite.

**Zahlen:** Kritisch 0, Hoch 0, Mittel 3, Niedrig 3.

---

## Was geprüft wurde und hält

| Frage | Ergebnis |
|---|---|
| Byteversätze in `krk-core::text` auf Zeichengrenzen | **hält**, an der Rechnung geprüft, nicht am Modulkopf |
| Gültigkeitsprüfung der Lesezeichen ohne Lesevorgang | **hält**, je Zweig genau ein `stat` |
| `#![deny(unsafe_code)]` mit zwei Ausnahmedateien | **hält** |
| Kein `_ =>` in eine erschöpfende Fallunterscheidung eingeschlichen | **hält** für die neuen; ein vorbestehender hat den fünften `Bereich` geschluckt (Befund 3) |
| `syntect`/`two-face`: kein `-sys`, kein `cc`, kein Oniguruma | **hält**, gemessen |
| Ton der Cargo-Begründung | hält bis auf den transitiven Fußabdruck (Befund 6) |
| Platzhalter der drei Nachziehstellen ehrlich | zwei von drei nennen ihren Schritt (Befund 4) |

### Die Versätze in `krk-core::text`

Geprüft an der Rechnung. Jeder gelieferte `usize` entsteht aus einer von vier
Quellen, und alle vier liefern nur Zeichengrenzen: `0`, `text.len()`,
`match_indices('\n')` plus 1 (ein Einbytezeichen), und `match_indices(gesucht)`
mit `anfang + stueck.len()`. `zeile_am_versatz` nimmt beliebige Versätze
entgegen, schneidet aber nichts, sondern zählt nur
(`zeilen.rs:127-133`). `einen_ersetzen` und `alle_ersetzen` schneiden ausschließlich
an Trefferrändern (`suche.rs:155-157`, `:190-194`). Die Voraussetzung, dass ein
Treffer aus `alle` stammen muss, steht als Vertrag an beiden Funktionen.

Die Randfälle sind sauber gezogen: der leere Text hat genau eine Zeile, ein
abschließender Umbruch öffnet eine leere letzte Zeile, die Nummer 0 und eine über
der Zeilenzahl tragen je ein Kennzeichen statt eines Fehlers. `zeilen.rs` stützt
sich auf die Zusage aus `decisions/260808-0021_*` (nur `\n` im gehaltenen Stand)
und sagt das an der Stelle, statt selbst eine zweite Meinung über Zeilenenden zu
führen.

### Die Gültigkeitsprüfung

`Lesezeichen::gueltig` (`lesezeichen.rs:195-200`) fragt `is_dir()` beziehungsweise
`is_file()`, also je einen `stat`, und liest keine Datei. Der gemerkte
Zeileninhalt kommt in der Prüfung nicht vor. Das ist genau, was
`decisions/260807-2147_*_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`
als tragenden Grund für Möglichkeit 1 nennt. Die Probe
`eine_textmarke_ist_gueltig_solange_ihre_datei_da_ist` (`tests/ablage.rs`) hält
beide Richtungen fest.

Die Ablageform ist zusätzlich gegen die Randfälle geprüft, die die Proben nicht
abdecken: ein Eintrag ohne `ordner` und ohne das Feldtrio, ein Eintrag mit
`datei` aber ohne `zeile`, und eine negative Zeilennummer scheitern alle beim
Einlesen, laufen also über `Geladen::ist_ersetzt` und werden nicht still zum
Auslieferungswert. `zeile = 0` wird angenommen und landet über
`Zeilenlage::VorDerErsten` am Textanfang — richtig behandelt, kein Sonderweg.

---

## Befunde nach Thema

### Zwischenstände, die eine Zusage offen lassen

**1. Ein sichtbarer `Bereich::Editor` ohne Unteransicht verliert seine Breite** — Mittel.
`aufteilung.rs:331-354`. `auslegen` zählt den Editor in `sichtbare` mit und
`bereichsbreiten` weist ihm 460 Punkte zu, aber die Schleife überspringt ihn, weil
`bereichsansicht(teiler, 4)` bis S16 `None` liefert. Die vier echten Bereiche
bekommen `gesamt.width - trenner - 460`. Dazu ein zweiter Teil desselben Befunds:
`sichtbar_im` (aus dem Modell) und `gemessene_sichtbarkeit` (vom Bildschirm) geben
für den Editor verschiedene Antworten, und beide speisen dasselbe `auslegen` —
`anwenden` die eine, `neu_auslegen` bei jeder Fenstergrößenänderung die andere.
Erreichbar heute über `editor = true` in einer von Hand geschriebenen
`session.toml`; ab S5/S6 zusätzlich über die Tastatur, weil `fokus_holen`
(`anwendung.rs:1063-1070`) den Bereich einblendet, bevor der Platzhalter
`Fokus::Editor => false` das Fokussetzen abweist.
→ `issues/260808-1413_o_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md`

**4. Vier Platzhalter nennen ihren ablösenden Schritt nicht** — Niedrig.
`anwendung.rs:1108`, `anwendung.rs:1562` und beide Stellen in `leistenmodell.rs`
nennen S17, S38 und S39. `fenstermodell.rs:19-21`, `:275-276` und
`aufteilung.rs:296-298`, `:329-330` sagen "ein späterer Schritt". Der Plan nennt
die Nummern (S18, S16); der Code hat den Satz übernommen und die Nummer fallen
lassen.
→ `issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`

### Text, der mehr behauptet als der Code hält

**2. `Wirkungsbereich::Navigator` ist dokumentiert, als trügen ihn schon drei Befehle** — Mittel.
`belegung.rs:197-203` gegen `belegung.rs:462` und `:474`. Der Umzug steht in S5;
`Kommando::wirkungsbereich` gibt `FensterWechseln`, `AuswahlHoch` und
`AuswahlRunter` weiter `Ueberall`, ohne Vorwärtsverweis an der Zuordnungsstelle.
Die Probe in `krk-ui` (`fokus.rs:236-240`) nennt S5, der Kern nicht — die
verlässlichere der beiden Stellen ist die entferntere.
→ `issues/260808-1413_o_der-wert-navigator-ist-dokumentiert-als-truegen-ihn-schon-drei-befehle.md`

**5. `umlaufen` behauptet, die eine Stelle des Umlaufs zu sein** — Niedrig.
`suche.rs:128-138`. Zwei der drei Auswahlfunktionen rufen sie, `voriger`
(`:119-126`) trägt Umlauf und Leerlistenbehandlung selbst. Das Verhalten stimmt;
der Satz stimmt nicht, und in einem Modul, das die Einzelstellen-Zusage an drei
weiteren Stellen ausschreibt, ist das mehr als ein Kommentarfehler.
→ `issues/260808-1413_o_umlaufen-behauptet-die-eine-stelle-des-umlaufs-zu-sein-voriger-laeuft-daneben-um.md`

### Vollständige Fallunterscheidungen

**3. `breite_aendern` trägt einen Auffangzweig über `Bereich`** — Mittel.
`fenstermodell.rs:337-340`, `_ => Bereich::Links`. Vorbestehend, aber diese Runde
hat `Bereich` um einen fünften Wert erweitert, und der Zweig hat ihn stumm
aufgenommen. Die Antwort ist heute richtig, weil `ist_beweglich()` davorsteht;
richtig ist sie aus dem Grund, den derselbe Turn 180 Zeilen darüber ausdrücklich
ausschließt ("mit der richtigen Antwort, aber aus dem falschen Grund").
Das Abnahmekriterium von S13 — grüner Bau belegt die Vollständigkeit — trägt für
diese Stelle nicht, weil sie keine erschöpfende Fallunterscheidung ist.
→ `issues/260808-1413_o_breite-aendern-traegt-einen-auffangzweig-ueber-bereich-und-hat-den-fuenften-wert-geschluckt.md`

Alle **neu geschriebenen** Fallunterscheidungen sind vollständig und ohne
Auffangzweig: `fokus::wirkt` über sieben Wirkungsbereiche, `holt_hervor` über fünf
Fokuswerte, `Lesezeichen::gueltig` über zwei Ziele, `sichtbar_im`,
`Fenstermodell::sichtbar`, `umschalten`, `breite`, `breite_setzen`,
`ist_beweglich` über fünf Bereiche, `Zeilenlage` über drei Lagen. `ist_beweglich`
ist in dieser Runde von `matches!` zu einer erschöpfenden Fallunterscheidung
geworden, und die Literalliste in `bereichsbreiten` ist entfallen — beides
Beseitigung einer zweiten Wahrheit, nicht Zutat des Editors.

### Fremde Kisten

**6. Die Begründung zu `syntect` nennt den transitiven Fußabdruck nicht** — Niedrig.
`Cargo.toml:100-161`. Alle vier Tatsachenbehauptungen sind nachgemessen und
halten: kein `-sys`-Paket, keine Bauabhängigkeit `cc`, kein Oniguruma; `dump-create`
kommt über `parsing` mit; `html`, `plist-load`, `yaml-load` und `metadata` bleiben
aus; 75 Sprachdefinitionen gegen 213, TOML nur in `two-face`. Was fehlt, ist die
Zahl der mitkommenden Pakete: 21 transitive neben den beiden Kisten selbst, der
größte Zuwachs am Abhängigkeitsbaum, den dieses Projekt bisher aufgenommen hat.
Der Eintrag zu `signal-hook` (`Cargo.toml:54-56`) behandelt genau diese Frage als
tragend.
→ `issues/260808-1413_o_die-begruendung-zu-syntect-nennt-den-transitiven-fussabdruck-nicht.md`

Ungeprüft geblieben ist eine Zahl: die 1.360 Byte, die `default-syntaxes` am
Programm kostet (`Cargo.toml:126-130`). Sie stammt aus einer Messung des
Umsetzenden; ein Nachmessen hätte zwei Bündelbauten gekostet und trägt keine
Zusage.

---

## Querschnittliches

**Der Turn hat drei Zwischenstände erzeugt, und alle drei sind derselbe Fall:**
eine Aufzählung im Kern ist gewachsen, bevor die Stelle in der Oberfläche existiert,
die den neuen Wert bedient. Bei `Fokus::Editor` ist das benannt (S17), bei
`Ziel::Textstelle` benannt (S38, S39), bei `Bereich::Editor` und
`Wirkungsbereich::Navigator` nicht. Die Befunde 1, 2 und 4 sind drei Ausprägungen
davon.

Der bereits offene Defekt
`issues/260808-0930_o_s11-aendert-eine-kernschnittstelle-deren-aufrufstellen-der-plan-erst-s38-und-s39-zuweist.md`
benennt die Regel, die dahinter fehlt, und nennt S3 und S5 als weitere Kandidaten.
Befund 2 ist der Beleg, dass die Lücke bei S3 doch greift, nur anders: nicht der
Bau bricht, sondern der Text sagt etwas, das erst S5 wahr macht. Das gehört in
denselben Planungsdurchgang.

**Kein Verstoß gegen einen der acht Entscheidungsdatensätze.** Vier davon sind in
diesem Turn berührt, alle vier korrekt umgesetzt:

| Datensatz | Berührt in | Befund |
|---|---|---|
| Suche in der Nähe einer Textmarke | `Lesezeichen::gueltig` | hält: ungültig heißt allein, die Datei fehlt |
| Bereich oder nur eine Stelle | `Ziel::Textstelle` | hält: Zeile und Zeileninhalt, kein zweiter Anker |
| Sicherungsform beim Sichern | `zeilen.rs` Modulkopf | hält: der Index kennt nur `\n` und sagt, woher die Zusage kommt |
| Sprachen der Syntaxhervorhebung | `Cargo.toml`, `tests/syntaxkiste.rs` | hält: fertige Kiste, alle vier Sprachen belegt |

Die vier übrigen sind in diesem Turn nicht berührt.

---

## Empfohlene Reihenfolge

Nichts davon hält einen Schritt auf.

1. **Befund 2 vor S5** — die zwei Kommentarzeilen kosten nichts und verhindern,
   dass S5 an einer Stelle vorbeiläuft, die schon so aussieht, als wäre sie
   erledigt.
2. **Befund 1 mit S16 oder S18** — die Entscheidung, welcher der beiden Wege
   (Filter über `bereichsansicht`, oder Zusicherung in `aus_sitzung`) gilt,
   gehört zu dem Schritt, der die Unteransicht einhängt.
3. **Befund 3 mit S18**, wo `fenstermodell.rs` ohnehin angefasst wird.
4. **Befunde 4, 5, 6** als Aufräumarbeit, jederzeit.

---

## Abgleichvermerk 260810-0805

Stand der sechs Befunde im Defektspeicher, am Dateibestand abgelesen: **drei geschlossen, drei offen.**

Geschlossen: `issues/260808-1413_c_breite-aendern-traegt-einen-auffangzweig-ueber-bereich-und-hat-den-fuenften-wert-geschluckt.md`, `..._c_der-wert-navigator-ist-dokumentiert-als-truegen-ihn-schon-drei-befehle.md`, `..._c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md` — darunter der schwerste des Berichts.

Offen: `..._o_die-begruendung-zu-syntect-nennt-den-transitiven-fussabdruck-nicht.md`, `..._o_umlaufen-behauptet-die-eine-stelle-des-umlaufs-zu-sein-voriger-laeuft-daneben-um.md`, `..._o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`. Alle drei tragen die Schwere niedrig.

Am Bericht selbst ist nichts geändert.
