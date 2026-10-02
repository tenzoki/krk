# Vollbaum-Durchsicht R11: `crates/krk-ui/src/kommandos/`

**Sender:** coderev
**Reviewed-range:** `004ff72..ca8072d`
**Not-opened:** none
**Gelesen:** alle zwölf Dateien unter `crates/krk-ui/src/kommandos/` (9.613 Zeilen), dazu als Rufer `appkit/anwendung.rs`, `appkit/tabelle.rs`, `appkit/teilen.rs`, `appkit/menue.rs`, `quellbaum.rs` und `krk-core/src/tasten/belegung.rs` in Auszügen.

## Zusammenfassung

Das Modul hält, was `CLAUDE.md` ihm zuschreibt: `zulaessig` ist die eine Erklärung mit genau zwei Rufern, die Vier-Kommandos-Probe iteriert über alle 79 Kommandos, `fokus::wirkt` ist über 7 × 5 vollständig ohne Auffangzweig, `rueckschritt` deckt alle acht Kombinationen und hat einen Rufer, `Kontextbefehl` verzweigt beim Delegierten vollständig. Eine dritte Stelle, die jedes neue Kommando bräuchte, gibt es in `kommandos/` nicht. Gefunden habe ich vier Dinge: die sechs älteren Module tragen kein `#[must_use]`, die Zulässigkeitsfrage für ein **Zeichen** wird im Anwendungsdelegierten ein zweites Mal von Hand beantwortet, die zwei Mauswege fragen die Vorstufe „läuft ein Vorgang" an verschiedenen Orten und die Lage nie, und eine Zahl im Modulkopf von `fokus.rs` ist veraltet.

## Totals

Critical 0 · High 0 · Medium 2 · Low 2 · dazu drei `Also seen`-Zeilen an bestehenden Datensätzen.

## Antworten auf die Prüffragen

**Vier Bestandteile von `zulaessig` (`zulaessigkeit.rs:177-186`).** Vollständig; keiner ist überflüssig, aber zwei greifen nur bei `Wirkungsbereich::Ueberall`: vor einem fremden Schlüsselfenster wie bei stehendem Blatt liefert `fokus_bei` schon `Anderswo` (`anwendung.rs`, `fn fokus_bei`), und `wirkt` weist damit jeden Bereichsbefehl ab. Der Modulkopf sagt das für (4) (`:44-53`), nicht für (1). Die Reihenfolge trägt nicht: alle Glieder sind `&&`/`||` ohne Nebenwirkung. Ein dritter Frager von `zulaessig` existiert nicht (Zählung `beide_frager_rufen_die_eine_regel`, gegengelesen an `anwendung.rs:901` und `:3140`). **Ein dritter Frager der Frage** existiert aber: der Zeichenzweig (`anwendung.rs:2967-2985`) setzt drei der vier Felder von Hand zusammen und ruft `zulaessig` nicht → Defekt `260826-1418_o_der-zeichenzweig-…`.

**Würde die Vier-Kommandos-Probe rot?** Ja. `waehrend_eines_blattes_kommen_genau_diese_vier_durch` (`zulaessigkeit.rs:666-692`) filtert `Kommando::KENNUNGEN` durch `zulaessig` und prüft erst `len() == 4`, dann die Mitgliedschaft der vier. Ein fünfter Eintrag in `immer_erreichbar` oder in `waehrend_blatt_erlaubt` macht sie rot, sofern er in `KENNUNGEN` steht. Ein Kommando, das dort fehlt, sieht sie nicht — das ist der blinde Fleck aus `260826-1223_o_kennungen-ist-die-programmweite-…`, dort als `Also seen` nachgetragen. Die Nachbarprobe `in_der_blattsperre_bleibt_es_bei_dem_einen_abbruch` (`operationen.rs:1473-1489`) hält daneben, dass `waehrend_blatt_erlaubt` allein den Abbruch nennt.

**`fokus::wirkt` (`fokus.rs:343-368`).** Vollständig über die sieben `Wirkungsbereich`-Werte ohne `_ =>`; die fünf Fokuswerte sind je Zeile positiv aufgezählt, ein sechster fiele auf `false`. Ein achter Wirkungsbereich hält den Bau an — hier und an der Tafel `[(Wirkungsbereich, [bool; 5]); 7]` (`:399`). Der achte Wert wird nicht still geschluckt.

**`rueckschritt` (`rueckschritt.rs:156-178`).** Vier Zweige über `(bool, bool, bool)`, vollständig, Tafel aus acht Fällen (`:232-252`), Aufruferzählung auf eins (`:208-220`); die Nadel `rueckschritt(` schließt `ist_nackter_rueckschritt(` über das `_` davor aus (`quellbaum.rs:141-146`). Hält.

**`Kontextbefehl` (`kontextmenue.rs:185-274`).** Drei Werte, `ALLE` ohne `cfg(test)`, Marke ab 1, Rundweg über die Marke geprobt. Beim Delegierten `kontextbefehl_ausfuehren` (`anwendung.rs:6205-6210`) ohne Auffangzweig. **Das Kontextmenü geht vollständig an `zulaessigkeit.rs` vorbei**, auch der Teilen-Eintrag: der ist `NSSharingServicePicker::standardShareMenuItem` (`teilen.rs`, `eintrag_anfuegen`) und nicht `Kommando::Teilen`. Die Sperre ist die Fenstermodalität des Blattes, nicht KRKs Regel → Defekt `260826-1419_o_die-zwei-mauswege-…`.

**`operationen.rs` und `Abschluss::ist_abgebrochen`.** `abschlusstext` (`:577-580`) verzweigt vollständig über beide Varianten; ein Rufer ist dort nicht nötig und wäre schlechter. `Also seen` an `260826-1221_o_abschluss-ist-abgebrochen-…`.

**Modulkopf von `mod.rs`.** Die Abgrenzung stimmt: `abwurfregel` und `kontextmenue` sind die zwei Module ohne Tastenbefehl, elf `pub mod` (`:110-120`). `:17-19` und `:68` sprechen noch von drei Bestandteilen — bereits gefiltert als `260813-1420_o_vier-modulkoepfe-…`, nicht neu gefiltert.

**Dritte Stelle für ein neues Kommando in `kommandos/`.** Keine: die einzigen Verzweigungen über `Kommando` sind `immer_erreichbar` (`matches!`, bewusst offen) und `waehrend_blatt_erlaubt` (`==`). Beide sind Listen mit Vorgabewert „gehört nicht dazu" und keine Pflichtstellen.

**`#[must_use]`.** Sechs von zwölf Modulen tragen es an jeder reinen Antwort mit Begründung, die sechs älteren an keiner → Defekt `260826-1417_o_sechs-der-zwoelf-kommandos-module-…`.

## Befunde

### Medium

1. **`#[must_use]` fehlt an 24 reinen Antworten in sechs Modulen, darunter `zulaessig`, `immer_erreichbar`, `waehrend_blatt_erlaubt`, `fokus::wirkt` und `Buendelung::melden`** (`zulaessigkeit.rs:177`, `:202`; `operationen.rs:283`, `:315`; `fokus.rs:343`). `melden` schaltet dabei das Atom um; ein Aufruf ohne Auswertung verliert den Zeichendurchgang. Querschnitt: Trennlinie ist der Nutzerentscheid vom 260811-2140, alle jüngeren Module tragen es. → `260826-1417_o_sechs-der-zwoelf-kommandos-module-tragen-kein-must-use-…`

2. **Der Zeichenzweig beantwortet die Zulässigkeitsfrage ein zweites Mal von Hand** (`anwendung.rs:2967-2985`): `lage.blatt_steht || lage.ersthelfer_gehoert_appkit` und ein `match lage.fokus`; der vierte Bestandteil nur mittelbar über `fokus_bei(Fremd) → Anderswo`. Keine Tafel, keine Aufruferzählung sieht ihn. Vorschlag: `zulaessigkeit::zeichen_zulaessig(lage)` mit 40-Felder-Tafel. → `260826-1418_o_der-zeichenzweig-setzt-die-zulaessigkeitsfrage-…`

### Low

3. **Die Vorstufe „läuft schon ein Vorgang" steht für Abwurf und Löschweg in der reinen Regel, für Zip/Unzip im Delegierten** (`abwurfregel.rs:324-337`, `loeschwarnung.rs:360-386` gegen `anwendung.rs:6243`, `:6293`), ohne Probe. Kein Mausweg fragt `blatt_steht`; ob ein Blatt einen Abwurf sperrt, ist im Baum nirgends gesagt und von mir nicht verifiziert. → `260826-1419_o_die-zwei-mauswege-fragen-die-vorstufe-…`

4. **`fokus.rs:34` „rund fünfzig Befehle"**, es sind 79. → `260826-1420_o_der-modulkopf-von-fokus-rs-spricht-von-rund-fuenfzig-befehlen-…`

### Nachgetragen an bestehende Datensätze (`Also seen`)

- `260813-1345_o_zwei-prosastellen-in-zulaessigkeit-rs-…` (Circle 260813-0939): `zulaessigkeit.rs:587` „ein künftiger dritter Eintrag" bei drei vorhandenen.
- `260826-1223_o_kennungen-ist-die-programmweite-…`: die Vier-Kommandos-Probe erbt den blinden Fleck.
- `260826-1221_o_abschluss-ist-abgebrochen-…`: in `operationen.rs:577-580` ist kein Rufer nötig.

## Was ich geprüft und nicht beanstandet habe

- `navigation::zielzeile` (`:46-61`): Sättigung vor Clamp, leere Liste `None`, geprobt mit `isize::MAX/MIN`.
- `pfadeingabe::pruefen` (`:52-96`): `metadata` folgt Verknüpfungen bewusst; `read_dir` als Leserechtsprobe vor dem Wechsel; `gleicher_ordner` kanonisiert mit wörtlichem Rückfall.
- `kontextmenue::ohne_die_eigenen_ziele` (`:789-808`): Festpunkt über absteigende Pfadlänge, Kette aus drei Archiven geprobt; `packziel` kann leere Quellen liefern, und `zipauftrag_stellen` fängt es (`anwendung.rs:6250`).
- `loeschwarnung::wortlaut` (`:596-607`) trägt die 25 fest, gehalten von `const _: () = assert!(nennt_die_zahl(…))` (`:513-522`). `Unentschieden` fällt mit `Nein` auf `OhnePapierkorb` zusammen, und der Text sagt „führt keinen Papierkorb" — als Wahl dokumentiert (`:397-407`), nicht beanstandet.
- `abwurfregel::urteil` (`:324-363`): vollständig über vier Eingaben, `Schreibrecht::Unbekannt` lässt durch — Nutzerentscheid 260818-1633.
- `Vorgangszustand::aendern` (`operationen.rs:402-408`) nimmt eine vergiftete Sperre über `into_inner`, richtig für einen Anzeigestand.
- Keine `use objc2`-Zeile in den zwölf Dateien; die Zusage des Modulkopfs hält.

## Querschnittsbeobachtungen

- **Die Regeln der Runden 10 bis 17 sind alle nach demselben Muster gebaut** (reine Funktion, ausgeschriebene Tafel, Aufruferzählung, `#[must_use]` mit Begründung). Die Regeln der Runden 1 bis 7 haben die ersten zwei Eigenschaften und nicht die letzte. Ein Nachzug in einem Zug schließt die Lücke.
- **„Eine Frage, eine Stelle" gilt für Kommandos und nicht für Zeichen und Mausgesten.** Drei Eingangsarten — Taste, Zeichen, Maus — und nur die erste läuft durch `zulaessigkeit.rs`. Das ist teils Absicht (Maus), teils Lücke (Zeichen).

## Empfohlene Reihenfolge

Kein Release-Blocker. Erst Befund 2 (eine neue reine Funktion, eine Tafel, ein Rufer), dann Befund 1 in einem Zug über alle 24 Stellen, dann 3 mit der Messung am Bündel, 4 nebenbei.

**Verification:** alle Zeilenangaben am Baum `ca8072d` abgelesen (`cat -n`, `grep -n`) und ein zweites Mal gegengelesen; die Rufer-Aussagen mit `grep -rn` über `crates/krk-ui/src` außerhalb von `kommandos/` erhoben; die Variantenzahl 79 mit dem `awk` aus `CLAUDE.md` gezählt; die Editor-Befehlszahl neun (`fokus.rs:812`) gegen `Kommando::wirkungsbereich` nachgezählt und als richtig befunden. Nicht übersetzt, nicht getestet; kein Kommando hat den Quellbaum angefasst.
