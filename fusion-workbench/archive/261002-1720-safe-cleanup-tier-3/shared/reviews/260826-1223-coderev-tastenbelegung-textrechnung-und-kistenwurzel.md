# Durchsicht R4: `tasten/`, `text/` und die Kistenwurzel von `krk-core`

**Reviewed-range:** `004ff72..004ff72` — kein Commit-Bereich, Vollbaum-Durchsicht von crates/krk-core/src/{tasten,text}/ und der Kistenwurzel an HEAD 004ff72
**Not-opened:** none

**Sender:** coderev
**Datum:** 260826-1223
**Umfang:** 12 Dateien, 5.262 Zeilen — `crates/krk-core/src/tasten/{mod,belegung,konflikt,normalisierung,parser}.rs`, `crates/krk-core/src/text/{mod,datei,marke,suche,zeilen}.rs`, `crates/krk-core/src/{lib,zwischenablage}.rs`

## Summary

Der Codeteil hält, was `CLAUDE.md` über ihn sagt, an jeder Stelle, an der der Übersetzer die Zusage trägt — `Kommando::wirkungsbereich` ist mit 79 Zweigen und ohne Auffangzweig vollständig, `Wirkungsbereich` trägt seine sieben Werte, die drei Lesewege fragen sämtlich den offenen Deskriptor, und die Grenze `#![deny(unsafe_code)]` steht ohne eine Ausnahme in diesen zwölf Dateien. Die Befunde liegen fast alle dort, wo eine Zusage **nicht** vom Übersetzer, sondern von einer Probe oder von einem Kommentar gehalten wird: die programmweite Kommandoliste `KENNUNGEN` ist durch nichts vollständig gehalten und wird von einer Stelle als gehalten zitiert, der Ausschluss des Zehnerblocks steht auf einem Grund, den die Runde 2 aufgehoben hat, und die Nutzerdatei darf ein Feld setzen, an dem die Erreichbarkeit eines Befehls hängt. Dazu kommt die einzige echte Deckungslücke der Durchsicht: `tasten/` und `text/` tragen zusammen kein einziges `#[must_use]`, während dieselbe Kiste daneben 66 trägt.

## Totals

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 3 |
| Niedrig | 5 |

Dazu ein Entscheidungsdatensatz. Alle Befunde sind als eigene Datei in `shared/issues/` gefiltert; kein Circle ist aktiv, also gilt die Herkunftsregel auf den gemeinsamen Speicher.

## Befunde nach Thema

### Thema 1: Aufzählungen, die der Übersetzer nicht hält

Das ist das Thema mit dem meisten Gewicht, und es ist genau das, was `CLAUDE.md` unter „Was man nicht sieht" für den Ausführungszweig beschreibt — nur an anderen Stellen als den dort genannten.

**[Hoch] `Kommando::KENNUNGEN` ist die programmweite Kommandoliste, und nichts hält sie vollständig.**
`shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`

`belegung.rs:695-798` deklariert `[(Kommando, &'static str); 79]`. Die Längenangabe erzwingt 79 Einträge und sagt nichts darüber, welche. Beide Proben, die die Liste betreffen (`tests/belegung.rs:1696-1722` und `belegung.rs:1707-1718`), **iterieren über die Liste selbst** und können eine fehlende Variante nicht sehen. Es gibt im ganzen Baum keine Stelle, die über die Varianten iteriert.

Der Umfang macht den Befund schwer: `KENNUNGEN` ist auch außerhalb von `krk-core` **die** Kommandoaufzählung — `krk-ui/src/kommandos/zulaessigkeit.rs:592,620,668,768`, `kommandos/fokus.rs:587,763`, `kommandos/operationen.rs:1474`, `belegungsausgabe.rs:758`, `appkit/menue.rs:1069,1085,1109`. Ein Kommando ohne Eintrag ist für jede dieser Erhebungen unsichtbar, ist über keine `keymap.toml` erreichbar, und bringt zwei Stellen auf dem Hauptfaden zum Auslösen: `belegung.rs:1116` (`panic!`) und `menue.rs:445` (`expect`).

Dazu die Stelle, die die ungehaltene Hälfte als gehalten zitiert, `menue.rs:437-440`: „Die Liste fuehrt jedes Kommando genau einmal — `jedes_kommando_traegt_genau_einen_wirkungsbereich` […] haelt das fest". Der genannte Beleg trägt „höchstens einmal", nicht „genau einmal".

**Was daneben hält, und zwar nachgezählt.** `Kommando::wirkungsbereich` (`belegung.rs:849-1104`): 79 Varianten, 79 in den Zweigen genannt, kein `_ =>`, keine doppelte Nennung. `Wirkungsbereich`: sieben Werte, `beschriftung` (`belegung.rs:322-332`) vollständig ohne Auffangzweig. `Abweisung::meldung`, `Zeilenlage`, `Fund`, `Lesehindernis`, `Unlesbarkeit`, `Textstand` — alle vollständig ohne Auffangzweig, wie ihre Doc-Kommentare zusagen. Die Zusage von `CLAUDE.md` über `wirkungsbereich` trägt.

**[Niedrig] `Kommando::kennung` vergleicht über `as u8`.**
`shared/issues/260826-1223_*_kommando-kennung-vergleicht-ueber-as-u8-…md`

`belegung.rs:1111`. Der Umweg über die Umwandlung ist begründet (`PartialEq` ist nicht `const`), die Wahl von `u8` nicht. Ab 257 Varianten schneidet sie ab und liefert still die Kennung eines anderen Kommandos. Heute 79; nicht erreichbar, und die Schranke ist unbenannt in einer Aufzählung, von der `CLAUDE.md` sagt, sie wachse mit fast jeder Runde.

### Thema 2: Zusagen, die eine Runde aufgehoben hat, ohne dass die Prosa es merkte

**[Mittel] Der Grund für den Ausschluss des Zehnerblocks trägt seit der Zeichenkennung nicht mehr.**
`shared/issues/260826-1223_*_der-grund-fuer-den-ausschluss-des-zehnerblocks-traegt-seit-der-zeichenkennung-nicht-mehr.md`
Dazu `shared/decisions/260826-1223_*_loesen-die-zifferntasten-des-zehnerblocks-dieselbe-funktion-aus-wie-die-obere-reihe.md`

`parser.rs:237-238` und `:302-303` schließen den Zehnerblock aus, weil er „eigene Codes" trage. Seit der Runde 2 sieht die Kette `aus_ereignis` → `zeichen_als_kennung` → `kennung` → `nachschlag` (`mod.rs:98-115`, `parser.rs:393-396`, `belegung.rs:1289-1300`) den Tastencode für Ziffern **nicht mehr an**. `zeichen_als_kennung` nimmt jedes ASCII-alphanumerische Zeichen. Der genannte Schutz existiert nicht; was den Block noch heraushält, ist allein das Fehlen eines **Namens** in der Schreibweise, also die Belegbarkeit von Hand, nicht die Wirkung.

Die Folge ist eine Ungleichheit im selben Tastenblock: Zifferntasten treffen den Eintrag der oberen Reihe, die Eingabetaste desselben Blocks trifft nichts und tut insbesondere nicht, was `return` tut. `roh::ZEHNERBLOCK` (`normalisierung.rs:67`) hat außerhalb der Proben keinen Leser, und `normalisieren` löscht das Bit — es gibt keine Größe mehr, an der die zwei Tasten zu trennen wären.

Der Prosabefund ist am Quelltext allein entscheidbar und als Defekt gefiltert. Die Verhaltensfrage — bleibt es dabei? — ist eine Nutzerfrage und als Entscheidungsdatensatz gefiltert, mit drei Möglichkeiten und der Einschränkung, dass die Messung Nutzerarbeit ist (KRK muss im Vordergrund stehen).

**[Niedrig] Der Modulkopf von `text/datei.rs` sagt „immer einen abschließenden Umbruch", die leere Datei bekommt keinen.**
`shared/issues/260826-1223_*_der-modulkopf-von-datei-rs-sagt-immer-einen-abschliessenden-umbruch-…md`

`datei.rs:83-85` gegen `datei.rs:867-869` und den Rumpf in `:875-881`. Zwei Prosastellen derselben Datei, und die falsche ist die, auf die `text/mod.rs:29-31` und `CLAUDE.md` für die tragende Zusage des ganzen Verzeichnisses verweisen.

### Thema 3: Was die Nutzerdatei bestimmen darf

**[Mittel] Die Nutzerdatei setzt den Zusteller frei und macht damit ein gebautes Kommando still unerreichbar.**
`shared/issues/260826-1223_*_die-nutzerdatei-setzt-den-zusteller-frei-…md`

`Belegung::bauen` (`belegung.rs:1419-1451`) prüft von einem Eintrag allein `id` gegen den Wortschatz und übernimmt `name`, `reserviert_fuer` und `gehalten_von` unverändert. `gehalten_von` gatet `nachschlag` (`:1290-1293`) und `Funktion::kommando` (`:1169-1174`) — beide Stellen der vierteiligen Zustellerregel, die der Modulkopf über 29 Zeilen als „keine davon ist entbehrlich" ausschreibt. Ein von Hand gesetztes `gehalten_von = "menue"` an `kopieren` erzeugt keinen Konflikt (`konflikte` vergleicht nur innerhalb desselben Zustellers, `:1379-1397`), keine Meldung und keinen Rückfall auf die Auslieferung.

Der Mechanismus ist im Baum schon gemessen (`krk-ui/src/belegungsausgabe.rs:326-339`, am 260811-0955), aber nur so weit, wie er **jene** Stelle betraf: dort ist daraus die Folgerung „der Auffangzweig ist erreichbar und darf kein `panic!` sein" gezogen worden. Die Wirkung auf den Nutzer steht nirgends.

Abgegrenzt gegen `shared/issues/260814-0656_*` (dort der **fehlende** Eintrag, hier der vorhandene mit abweichendem Zusteller).

### Thema 4: Die Regel `#[must_use]`, modulweit übergangen

**[Mittel] `tasten/` und `text/` tragen kein einziges `#[must_use]`, während die Kiste daneben 66 trägt.**
`shared/issues/260826-1223_*_tasten-und-text-tragen-kein-einziges-must-use-…md`

Am 260826 erhoben: `verzeichnis/` 28, `leseprofil/` 15, `operation/` 13, `ablage/` 10 — `tasten/` 0 bei 47 öffentlichen Funktionen, `text/` 0 bei 26, `zwischenablage.rs` 0. (`stapelumbenennen/` steht mit auf der Null und liegt außerhalb dieses Umfangs.)

Es ist kein Formalbefund: die Stellen, an denen `krk-core` die Regel schon anwendet, haben genau die Gestalt der fehlenden — `verweisziel::bestimmen`, `inhalt::traegt_der_inhalt`, `umfang::zaehlen`, `filter::inhaltsschwelle`, `erkennung::erkennen`, `bausteine::zusammenfassen`. Der schärfste unter den fehlenden ist `Belegung::konflikte` (`belegung.rs:1379`): ein fallengelassenes `Vec<Konflikt>` übergeht jeden Konflikt schweigend, und `Vec` trägt keine eigene Marke. Dahinter `text::suche::alle_ersetzen`, `text::datei::in_gehaltene_form`, `text::marke::wiederfinden`, `zwischenablage::deuten`.

Nicht betroffen sind die Funktionen mit `Result`- oder `Option`-Fehlerkanal; dort trägt die Standardbibliothek die Marke.

### Thema 5: Kleine stille Fehlgriffe an den Rändern

**[Niedrig] Die Prozentschreibweise nimmt ein Vorzeichen an.**
`shared/issues/260826-1223_*_die-prozentschreibweise-nimmt-ein-vorzeichen-an-…md`

`zwischenablage.rs:118-133`. `u8::from_str_radix(ziffern, 16)` nimmt ein führendes `+`. Am 260826 mit `rustc -O` gegen eine wörtliche Kopie gemessen: `%+A` → `\n`, `%+5` → `\u{5}`; `%2G`, `% 5`, `%\t5` liefern korrekt `None`. Aus `file:///tmp/a%+Ab` wird der Pfad `/tmp/a\nb` — genau das Ergebnis, das der Doc-Kommentar zwei Zeilen darüber ausschließt („Beides still zu uebergehen hiesse, aus einem beschaedigten Verweis einen Pfad zu machen, den es nicht gibt"). Behebung: eine Zeile `is_ascii_hexdigit`.

**[Niedrig] `lesen` trennt den Deskriptormangel nicht, obwohl beide Nachbarlesewege es tun.**
`shared/issues/260826-1223_*_lesen-trennt-den-deskriptormangel-nicht-…md`

`datei.rs:434-440` gegen `:620-628` und `:692-700`. Der gemeinsame Doc-Kommentar der beiden nennt die Trennung „tragend und nicht bloss genauer" (`datei.rs:549-557`), weil `EMFILE`/`ENFILE` „etwas ueber den Prozess und nichts ueber die Datei" sagen. `lesen` wirft sie in `Textstand::KeinGueltigesZiel`, dessen Doc-Kommentar den Wert ausdrücklich als eine Aussage über die Datei definiert. Erreichbar, seit „Deep" ab Werk steht: der Durchlauf über den Unterbaum nimmt je Kandidat einen Deskriptor, und ein `f4` in dieser Lage ist der Fall.

**[Niedrig] `bis_zur_grenze_lesen` rechnet `grenze + 1` ohne Schutz.**
`shared/issues/260826-1223_*_bis-zur-grenze-lesen-rechnet-grenze-plus-eins-ohne-schutz-…md`

`datei.rs:640`. Bei `u64::MAX` bricht es im Profil `debug` ab und läuft im Profil `release` auf `take(0)` über — `Ok(Vec::new())`, also eine leere Datei statt der Bytes, in genau der Schranke, deren Zweck neun Zeilen Doc-Kommentar ausschreiben. Kein heutiger Aufrufer erreicht den Fall (alle drei bringen kleine Konstanten mit); die Hülle ist öffentlich und lädt in ihrem Vertrag ausdrücklich dazu ein, eine eigene Zahl mitzubringen. Behebung: `saturating_add`.

## Was geprüft wurde und hält

Ausdrücklich, weil eine Durchsicht ohne diese Liste den Eindruck hinterlässt, alles Ungenannte sei ungeprüft:

- **`Kommando::wirkungsbereich` vollständig, kein Auffangzweig.** Am 260826 maschinell nachgezählt: 79 Varianten, 79 abgedeckt, keine doppelt, kein `_ =>`.
- **`Wirkungsbereich` trägt sieben Werte**, wie `CLAUDE.md` sagt, und `beschriftung` deckt alle sieben ohne Auffangzweig.
- **`KENNUNGEN` ist in sich sauber:** 79 Paare, keine doppelte Kennung, kein doppeltes Kommando, und jede Variante hat heute einen Eintrag. Der Befund oben betrifft die Zukunft, nicht den Stand.
- **Der Leser kennt keine zweite Quelle neben `resources/default-keymap.toml`.** `AUSLIEFERUNGSTEXT` steht als einziges `include_str!` (`belegung.rs:159`), die fest verdrahtete Tabelle aus Schritt 7 ist fort, und die Tastencodes stehen allein in `parser::TASTEN`. Ein `parser::Taste`-Literal gibt es im ganzen Arbeitsbereich nur in `parser.rs`; die Treffer in `krk-ui/src/messmodus.rs` und `appkit/blaetter/mod.rs` sind gleichnamige, andere Typen.
- **Die Tastennormalisierung ist eindeutig.** Zwei Proben halten die Tabelle: `jeder_name_und_jeder_code_steht_genau_einmal` und `jede_taste_traegt_genau_eine_kennung_und_keine_zwei_dieselbe` (`parser.rs:626-692`). Groß- und Kleinschreibung sind über `zeichen_als_kennung` dieselbe Taste, `shift` steht als eigenes Bit. Die Umschalt-Varianten sind sauber: `krk-ui/src/appkit/ereignisse.rs:742-745` liest über `charactersByApplyingModifiers(empty)` und **nicht** über `charactersIgnoringModifiers`, und genau deshalb ist `shift+1` erreichbar, wo es über die zweite Frage tot wäre. Bleibt der Zehnerblock, Thema 2.
- **Die drei Lesewege fragen sämtlich `metadata()` am offenen Deskriptor**, nicht am Pfad: `datei.rs:441`, `:630`, `:702`. Die Reihenfolge öffnen → `fstat` → Typ → (Größe) → lesen hält in allen dreien. Die Grenzen passen zueinander: `lesen` hält `EDITORGRENZE`, die zwei anderen nehmen sie als Argument, `anlesen` prüft die Größe bewusst nicht und kann `Lesehindernis::ZuGross` deshalb nicht liefern, wie sein Doc-Kommentar sagt. Die Schranke gegen die wachsende Datei (`take(grenze + 1)`) steht in allen dreien, mit dem Vorbehalt oben.
- **`const _: () = assert!(EDITORGRENZE > 1024 * 1024)`** steht (`datei.rs:201`), und die Gegenrichtung in `krk-ui/src/appkit/editor.rs:885` (`STAPELBUDGET as u64 == datei::EDITORGRENZE`) hält daran fest, wie `CLAUDE.md` sagt.
- **`#![deny(unsafe_code)]` an der Kistenwurzel** (`lib.rs:1`), und in diesen zwölf Dateien steht kein `allow`. Das einzige `#![allow(unsafe_code)]` der Kiste ist `verzeichnis/sys.rs:130`.
- **Die Zahlen im Kopf von `lib.rs` stimmen.** „Sechs Schnittstellen und zehn gebundene Funktionen" (`lib.rs:19-22`) — nachgezählt: sechs (`getattrlistbulk`, `copyfile`, `renamex_np`, `fcntl`, `flock`, `localtime_r`), zehn (die vier `copyfile_state_*` dazu), in fünf `unsafe extern "C"`-Blöcken, wie `sys.rs:36` sagt.
- **Die vier Kommandonamen der Blattprobe existieren.** `Beenden`, `FensterSchliessen`, `FensterEinblenden` und `Abbrechen` stehen sämtlich in der Aufzählung; die Probe `zulaessigkeit::waehrend_eines_blattes_kommen_genau_diese_vier_durch` findet ihre Gegenstände.
- **Der Ring des Suchens steht genau einmal.** `umlaufen` (`suche.rs:195-200`) nimmt die **Zahl** der Kandidaten, und alle fünf Auswahlfunktionen gehen darüber, auch die zwei über Zeilennummern. Die scharfen Fälle — ein einziger Treffer, die leere Liste, der Rückwärtsschritt als Schritt um `len - 1` — sind je durch eine Probe belegt.
- **Der Zeilenindex und die Marke rechnen ohne zweite Meinung über das Zeilenende.** `marke::wiederfinden` fragt `Zeilenindex::inhalt_der_zeile` statt `str::lines`, und die Probe `eine_nummer_ueber_der_zeilenzahl_landet_am_dateiende_wie_der_zeilensprung` hält die beiden Wege gegeneinander.
- **Keine `unwrap` an einer Stelle mit echtem Fehlerfall.** Die Panikstellen dieser zwölf Dateien sind: `code_von_pflicht` (`parser.rs:360`, Übersetzungszeit für Namen aus dem Programmtext, ausdrücklich so gewollt), die zwei `expect` an `AUSLIEFERUNG` (`belegung.rs:164,166`, eingebettete Datei, ein Fehler dort ist ein Programmierfehler und wird von mehreren Proben gefangen) und der `panic!` in `Kommando::kennung` (Thema 1). Kein `let _ =`, kein leerer Fehlerzweig, kein `continue` über einen verschluckten Fehler.

## Querschnitt

Drei Muster laufen über die Befunde hinweg:

1. **Die Zusage steht in einer Prosa, deren Beleg eine andere Frage beantwortet.** `menue.rs:437-440` zitiert eine Probe für eine Hälfte, die sie nicht prüft. `parser.rs:302-303` nennt einen Schutz, den die Runde 2 aufgehoben hat. `datei.rs:83-85` sagt „immer", wo hundert Zeilen weiter unten die Ausnahme steht. Dreimal ist der Befund nicht, dass der Code falsch wäre, sondern dass die Stelle, an der ein Leser die Zusage prüft, sie bestätigt, ohne sie zu tragen.
2. **Zwei von drei Geschwistern halten eine Regel, das dritte nicht, und die Regel steht bei den zwei.** `bis_zur_grenze_lesen` und `anlesen` trennen den Deskriptormangel und begründen es; `lesen` nicht. Das ist dieselbe Gestalt wie Befund 1: die Begründung sitzt nicht dort, wo die Abweichung ist.
3. **Die Regel `#[must_use]` ist im Projekt bisher je Funktion verfolgt worden** — vier Datensätze, alle mit einer benannten Stelle. Die Erhebung über die Module zeigt, dass es nicht vier vergessene Stellen sind, sondern zwei Module ohne Deckung. Ein Parallellauf dieser Sitzung hat am selben Tag eine fünfte Einzelstelle in `ablage/` gefunden (`260826-1225_*_geladen-traegt-kein-must-use-…`); die zwei Datensätze sind gegeneinander vermerkt.

## Empfohlene Reihenfolge

Nichts hiervon hält eine Auslieferung auf. Der heutige Baum verhält sich an jeder Stelle richtig; die Befunde sind Fallen für die nächste Runde und drei falsche Sätze.

1. **Zuerst, weil es die nächste Runde trifft:** die Vollständigkeitsprobe für `KENNUNGEN` (Hoch). Sie ist billig und schließt eine Falle, die genau dann zuschlägt, wenn jemand ein Kommando hinzufügt — also in der nächsten Runde, die etwas baut.
2. **Danach die drei falschen Sätze** (Zehnerblock-Prosa, `datei.rs`-Kopf, `menue.rs:437-440` als Teil von 1). Kosten je zwei Zeilen und stehen an Stellen, an die andere Dateien für ihre tragende Zusage verweisen.
3. **Dann die zwei stillen Fehlgriffe mit je einer Zeile Behebung:** `is_ascii_hexdigit` in der Prozentschreibweise, `saturating_add` in `bis_zur_grenze_lesen`.
4. **Der `#[must_use]`-Durchgang über `tasten/`, `text/` und `zwischenablage.rs`** als eigener Schritt, mit `stapelumbenennen/` zusammen. Ein Durchgang und nicht zwanzig.
5. **Der Zusteller aus der Nutzerdatei** (Mittel) und **der Deskriptormangel in `lesen`** (Niedrig) ändern Verhalten und gehören in eine Runde mit Abnahme, nicht in einen Aufräumdurchgang.
6. **Die Zehnerblock-Frage** wartet auf den Nutzer und auf eine Messung, die nur er fahren kann.

## Verification

Jede Zeilenangabe dieses Berichts ist am Baum an HEAD `004ff72` gelesen und nicht aus einer Prosastelle übernommen. Die Zählungen über `Kommando`, `KENNUNGEN`, `wirkungsbereich` und die `#[must_use]`-Deckung sind maschinell erhoben (`awk`/`grep`/`python3` über den Quelltext), nicht abgeschrieben. Das Verhalten von `u8::from_str_radix` mit führendem Vorzeichen ist mit `rustc -O` gegen eine wörtliche Kopie der Funktion gemessen, in einem Wegwerfordner außerhalb des Quellbaums. Zwei Aussagen sind ausdrücklich **nicht** gemessen und im jeweiligen Datensatz als `inference:` gekennzeichnet: welches Zeichen AppKit für eine Zifferntaste des Zehnerblocks meldet, und ob `NSEventModifierFlagNumericPad` auch bei den Pfeiltasten gesetzt ist — beides verlangt KRK im Vordergrund und ist Nutzerarbeit. Keine Datei des Quellbaums ist verändert worden.
