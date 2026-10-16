# Durchsicht: secrets.txt mit sechs Feldern, zehn Minuten gemerkter Schlüssel und Verbindungen

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `bb2f861..6067ae9`
**Not-opened:** `Cargo.lock`, `README.md`, `crates/krk-core/src/sprache/schluessel.rs`, `crates/krk-core/src/verzeichnis/mod.rs`, `crates/krk-core/tests/heimordner.rs`, `crates/krk-ui/src/appkit/mod.rs`, `crates/krk-ui/src/main.rs`, `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/work-packages/261016-0229-secrets-mit-sechs-feldern-schluesselcache-und-verbindungen/package.json`, `261016-0306-auf-welchem-weg-verlassen-zugangsdaten-aus-secrets-txt-krk-wenn-eine-verbindung-hergestellt-wird.record.json`, `261016-0306-in-welcher-form-traegt-secrets-txt-sechs-felder-und-was-wird-aus-den-alten-eintraegen.record.json`, `261016-0306-wie-lange-gilt-ein-eingegebener-schluessel-fuer-secrets-txt-und-was-wird-gemerkt.record.json`, `261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.record.json`, `261016-0228-kontextmenue-traegt-auswerfen-fuer-volumes.md`, `261016-0253-der-modulkopf-von-appkit-nennt-finder-das-eine-modul-ohne-rueckgabe-auswurf-ist-ein-zweites.md`, `261016-0253-der-modulkopf-von-appkit-nennt-finder-das-eine-modul-ohne-rueckgabe-auswurf-ist-ein-zweites.record.json`, `261016-0253-ein-zweiter-klick-auf-auswerfen-waehrend-des-laufenden-auswurfs-schickt-einen-zweiten-auftrag.md`, `261016-0253-ein-zweiter-klick-auf-auswerfen-waehrend-des-laufenden-auswurfs-schickt-einen-zweiten-auftrag.record.json`, `fusion-workbench/work-packages/261016-0228-kontextmenue-traegt-auswerfen-fuer-volumes/package.json`, `261016-0253-reviewer-auswerfen-im-kontextmenue.evidence.json`, `261016-0253-reviewer-auswerfen-im-kontextmenue.md`
**Domain:** code
**Verdict:** revise

Der Bereich beginnt am Elter von `e57b5c7`, damit dieser Commit mitgedeckt ist. Er ist ein reiner Werkbank-Commit des Vorgängerpakets, und nach dem Auftrag braucht er keinen Blick; seine Dateien stehen deshalb unter `Not-opened`. Gelesen gegen den Plan `261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md` und die drei Entscheidungsdatensätze des Pakets. Ganz gelesen: `appkit/verbinden.rs`, `tresorweg.rs`, `appkit/blaetter/verbindung.rs`, `heimordner/eintraege/geheimnisse.rs`, `heimordner/verbindung.rs`, `heimordner/schluesselmerker.rs`. Als Diff gelesen: `tresor.rs`, `verzeichnis/sys.rs`, `lib.rs`, die Cargo-Dateien, `lesezeichen.rs`, `eintraege.rs`, `belegung.rs`, `tests/baum.rs`, `editormodell.rs` (Betriebscode vollständig, Prüfmodul in Auszügen), `appkit/editor.rs` und `appkit/eintragsansicht.rs` (geänderte Zeilen), `leiste.rs`, `leistenmodell.rs`, `kontextmenue.rs`, `belegungsmodell.rs`, `blaetter/mod.rs`, `default-keymap.toml`, `HowTo.md`, die Zusätze in `CLAUDE.md`. Aus `appkit/anwendung.rs` die Verbindungswege (Zeilen 8859-9320), der Merker samt Wecker (10250-10330), der Ladeausgang (10125-10165), der Aufbau des Schlüsselmelders (1690-1720) und die Prüfmodule zu Verbindung und Merker. Die drei Sprachtabellen sind über die Platzhalter je Schlüssel abgeglichen, nicht Satz für Satz gelesen.

**Verification:** `make check` auf `6067ae9` grün („alle fuenf gruen“, 1:55 min). `cargo tree --workspace --target <ziel> -e normal,build` zeigt auf `aarch64-apple-darwin` und `x86_64-apple-darwin` je null Pakete `cc` oder `*-sys`.

## Summary

Die Sicherheitsarchitektur hält: das Passwort verlässt KRK allein als Argument von `NetFSMountURLSync`, in `bookmarks.toml` steht allein der Name, der Merker hat einen Schreiber, zählt mit `CLOCK_MONOTONIC` und lässt keinen abgelaufenen Schlüssel durch, und der Formatübergang schreibt nichts ohne Handlung. Zwei mittlere Befunde verlangen Nacharbeit: eine leere, im Editor offene `secrets.txt` bekommt über „Verbinden…“ eine zweite PIN, und `HowTo.md` sagt ein Tilgen zu, das Klick- und Anhängeweg nicht halten. Dazu zwei kleine: ein ungenannter Schreibvorgang auf dem Hauptfaden, der nach der Klausel des Plans eine Nutzerfrage auslöst, und drei Proben, deren Doc-Kommentare mehr behaupten, als sie prüfen.

## Totals

| Critical | High | Medium | Low |
|---|---|---|---|
| 0 | 0 | 2 | 2 |

## Findings by theme

### A. Zwei Schlüssel für eine Datei (Medium)

`261016-0532-verbinden-an-einer-leeren-im-editor-offenen-secrets-txt-legt-eine-zweite-pin-fest.md`

`verbindung_erfragen` (`crates/krk-ui/src/appkit/anwendung.rs:9072-9085`) weist allein einen Editor mit ungesichertem Stand ab. Eine eben festgelegte, leere `secrets.txt` hat keinen (`editormodell.rs:1268-1270`), also geht das Blatt auf, `geheimnisweg` verlangt bei Größe 0 ein neues Festlegen (`editormodell.rs:1064`), der Faden schreibt mit frischem Salz, und das Neuladen des Editors mit seinem alten Schlüssel scheitert (`anwendung.rs:9235-9240`, `editormodell.rs:1706`). Danach halten Editor und Merker verschiedene Schlüssel; ein Überschreiben im Editor nach der Nachfrage aus C4 nähme den neuen Eintrag und die zweite PIN wieder weg. Der Ablauf ist aus den Zweigen gelesen und nicht am Bündel gefahren (**inference**). Fix: dieselbe Vorprüfung weist auch ab, solange der Kopf des Editors nicht auf der Platte steht.

### B. Tilgen: Zusage an den Nutzer größer als der Code (Medium)

`261016-0532-howto-verspricht-ein-tilgen-des-verbindungspassworts-das-klick-und-anhaengeweg-nicht-halten.md`

Klick und „Verbinden…“ entschlüsseln die ganze Datei auf ihrem Faden; `Geoeffnet::klartext` und `Entsperrt::stand` (`tresorweg.rs:239`) tragen jedes Passwort und fallen ungetilgt. Das Blatt liest bei jedem Tastendruck alle Felder in ein `Vec<String>` und klont das Passwort daraus (`blaetter/verbindung.rs:127`, `:231-235`). `HowTo.md:1621-1626` nennt als ungetilgt nur Editor, Tabelle und Blattfelder. Daneben: `#[derive(Debug)] struct Entsperrt` (`tresorweg.rs:130`) zeigt den ganzen Klartext, gegen die Linie, die `Passwort`, `Geheimnis`, `Geheimniszeile` und `Verbindungsentwurf` halten. Der Typ `Passwort` selbst ist richtig gebaut: privates Feld, `Debug` verdeckt, `Drop` tilgt, `preisgeben(self)` gibt `Zeroizing<String>` heraus (`geheimnisse.rs:117-160`).

### C. Hauptfaden (Low)

`261016-0532-der-anhaengeweg-schreibt-bookmarks-toml-auf-dem-hauptfaden-und-die-zusage-nennt-nur-den-einen-stat.md`

Neben dem einen `stat(2)` (`tresorweg::groesse`) und der benannten Ausnahme `notizordner_bereitstellen` schreibt `anhaengen_einziehen` über `lesezeichen_aendern` (`anwendung.rs:9229`, `:2759-2770`) `bookmarks.toml` unter der Schreibsperre auf dem Hauptfaden. Plan (Entscheidung 20 gegen 13), `CLAUDE.md` und der Doc-Kommentar der Probe `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat` nennen ihn nicht. **inference:** kein Weg aus C8 ist berührt. Nach der Klausel unter „Where this work stops“ geht die Frage nach dem Abnahmelauf trotzdem an den Nutzer. Sonst fragt keiner der neuen Wege die Platte auf dem Hauptfaden: der Klick startet seinen Faden nach dem `stat(2)`, `NSWorkspace openURL:` steht nach Entscheidung 10 ausdrücklich dort, `ordner_lesen` startet seinen eigenen Lesevorgang.

### D. Proben, die mehr behaupten, als sie prüfen (Low)

`261016-0532-drei-proben-des-pakets-behaupten-mehr-als-sie-pruefen.md`

- `eine_verbindung_schreibt_aus_dem_eintrag_allein_den_namen` (`tests/ablage.rs:4538-4575`) baut das Lesezeichen selbst aus dem Namen und prüft den Typ, nicht den Betriebsweg `anhaengen_einziehen`.
- Die Ruferzählung von `preisgeben` (`verbinden.rs:437-456` über `quellbaum.rs:204-220`) übersieht einen Funktionsverweis ohne Klammer.
- `der_merker_hat_genau_einen_schreiber` und der Doc-Kommentar von `schluessel_merken` (`anwendung.rs:10261-10263`, `:15213-15220`) sagen „allein über den Schlüsselmelder“; der Code hat mit `gemerkt_falls` (`anwendung.rs:9305-9309`) einen zweiten Rufer, und der Rumpf der Probe prüft genau diese zwei.

## Geprüft und ohne Befund

**Wege des Passworts aus KRK (Schwerpunkt 1).**

- *`bookmarks.toml`*: `Ziel::Verbindung { verbindung: String }` (`lesezeichen.rs`) trägt keinen Feldwert, und `anhaengen_einziehen` baut Beschriftung und `verbindung` aus dem getrimmten Namen.
- *URL*: `Verbindungsziel` nimmt kein Passwort (`heimordner/verbindung.rs`), `Verbindungsplan::Uebergeben` trägt keines (`tresorweg.rs:440-444`), und die Einhänge-URL ist `<schema>://<adresse>` ohne Konto.
- *Prozessargument*: die Übergabe an `NSWorkspace` trägt bei `ssh`/`sftp`/`vnc` allein das Konto.
- *Standardausgabe*: in keiner neuen Datei steht `println!`, `eprintln!` oder `dbg!`. Das Blatt „Verbinden“ und das PIN-Blatt sind über `verdeckt_machen` verdeckt (`blaetter/verbindung.rs:284`, `blaetter/pin.rs:396`).
- *Statuszeile*: Meldungen nennen Name, Pfad, Protokoll oder Fehlercode, nie das Passwort.
- *Platte*: `tresorweg::anhaengen` schreibt allein ein `Chiffrat` über `atomar::schreiben`, und `nur_benannte_dateien_erreichen_das_atomare_schreiben` führt `tresorweg.rs` mit Namen.

**NetFS-Bindung.**

- *Signatur*: die von `NetFS.h` (`CFURLRef`, zwei `CFStringRef`, zwei `CFMutableDictionaryRef`, `CFArrayRef *`), mit gebrückten Foundation-Typen und `Option<&…>` für die nullbaren Zeiger (`verbinden.rs:157-172`).
- *Speicher*: die Liste der Einhängepunkte wird nur bei Rückgabe 0 und genau einmal über `CFRetained::from_raw` übernommen (`verbinden.rs:317-330`). **inference:** NetFS übergibt die Liste nach der Create-Regel, wie der Kommentar dort sagt; im SDK-Kopf ist das nicht nachgelesen.
- *Faden*: `krk-verbinden`, der Ausgang kommt über `DispatchQueue::main()`.
- *Optionen*: `UIOption = NoUI` sind die Zeichenketten der Makros.

**Schlüsselmerker (Schwerpunkt 2).**

- `restdauer` misst ab dem Merken, und die Grenze 600 s ist ausgeschlossen. Eine Wachzeit vor dem Merken gilt als abgelaufen (`schluesselmerker.rs:620-626`).
- `gueltiger` tilgt bei Ablauf und verlängert nichts. `tresorweg::gemerkter_schluessel` fragt bei jedem Gebrauch neu.
- Gemerkt wird allein nach einer Eingabe (`Geoeffnet::eingegeben`, `Entsperrt::eingegeben`, `Editormodell::zu_merken`), getilgt beim Ersetzen, Verwerfen, Ablauf und in `applicationWillTerminate:`.
- Der Wecker ist einmalig und stellt sich nach dem Aufwachen auf den Rest.
- Ein abgelaufener Schlüssel wird nirgends verwendet. Allein die im Editor offene Datei behält ihren Schlüssel, und das ist Entscheidung 5.
- `Pinwechsel` vergleicht die alte PIN durch Ableitung mit Salz und Parametern des gehaltenen Schlüssels (`editormodell.rs`, `pinwechsel_ableiten`).

**Formatübergang (Schwerpunkt 3).**

- `geheimnisse::aendern` fragt „unverändert“ vor jeder Prüfung und schreibt allein den berührten Block. Jeder andere Block bleibt Byte für Byte über `mit_block`.
- Ein Eintrag im alten Format wird ohne Handlung nicht geschrieben.
- Stelle und Zeile laufen auch für die Geheimnistabelle durch `stelle_der_zeile`/`zeile_der_stelle`.
- Die Leiste führt `Zeile::Lesezeichen` weiter mit der Stelle in der einen Liste und sucht Zeilen über `position`.
- `Lesezeichenliste::verschieben` bleibt in der Gruppe.

**Vollständigkeit (Schwerpunkt 5).**

- `Verbindungsausgang`, `Protokoll`, `Feld`, `Kontextbefehl` und `Ziel::gruppe` verzweigen ohne Auffangzweig.
- `Kommando::VerbindungHerstellen` steht in `KENNUNGEN`, `wirkungsbereich`, `bereich_des_kommandos` und im Ausführungszweig (`anwendung.rs:4567`).
- Die Untergrenzen-Abschnitte von `verbinden.rs`, `blaetter/verbindung.rs`, `leiste.rs` und `blaetter/mod.rs` nennen die neuen Berührungen.
- Die Platzhalter der neuen Texte stimmen in `de.rs`, `fr.rs` und `en.rs` überein.

## Cross-cutting observations

- **Eine Zusage wird an einer Stelle gemacht und an einer anderen gehalten, und die zwei laufen auseinander.** B, C und D sind dieselbe Bauart: `HowTo.md`, Plan und Doc-Kommentare beschreiben einen engeren Weg, als der Code geht (Klartext auf dem Faden, Lesezeichen-Schreibweg, zweiter Rufer). Der Code ist dabei jeweils vertretbar; falsch ist die Beschreibung.
- **Der Merker ist sauber gebaut, die Lücke liegt beim zweiten Halter eines Schlüssels.** A entsteht nicht im Merker, sondern daran, dass Editor und Verbindungsweg je einen Schlüssel für dieselbe Datei halten dürfen. Entscheidung 5 trennt sie bewusst, und an einer Datei ohne Kopf geht diese Trennung schief.

## Hinweise ohne Datensatz

- Schritt 9 des Plans verlangt am Block `verbindung_herstellen` in `resources/default-keymap.toml` einen Kommentar mit Begründung und Zählung. Er fehlt (Zeilen 603-605). Das ist kosmetisch.
- Die drei Entscheidungsdatensätze des Pakets stehen in ihrer Steuerdatei auf `open`, obwohl `ea6dac2` bis `97d3ce3` sie umsetzen. Nachziehen ist Sache des Orchestrators.
- **speculation:** Eine Adresse mit Leerzeichen (`nas.local/Daten Ordner`) nimmt das Blatt an. Ob `NSURL URLWithString:` sie beim Klick abweist (`Verbindungsausgang::UngueltigeUrl`) oder seit macOS 14 selbst kodiert, ist nicht geprüft. Weist es ab, stimmt der Doc-Kommentar von `entwurf_pruefen` („damit das Blatt nichts annimmt, was der Klick danach abweist“, `tresorweg.rs:336-339`) nicht. Ein Lauf am Bündel mit einer solchen Freigabe klärt es.

## Recommended sequencing

1. Vor der Auslieferung: A beheben. Dazu die Frage aus C (Abnahmelauf nach der Klausel des Plans) dem Nutzer vorlegen.
2. Vor der Auslieferung, weil es eine Zusage an den Nutzer betrifft: B, entweder im Code oder in `HowTo.md`.
3. Aufräumen, ohne Einfluss auf das Verhalten: D, die übrigen Doc-Kommentare aus C und der Kommentar in der Belegung.
