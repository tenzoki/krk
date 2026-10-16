# Nachdurchsicht: die vier Behebungen an Verbindungen, Tilgen, Hauptfaden und Proben

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `6067ae99..9d1b1112`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`, `261016-0532-der-anhaengeweg-schreibt-bookmarks-toml-auf-dem-hauptfaden-und-die-zusage-nennt-nur-den-einen-stat.record.json`, `261016-0532-howto-verspricht-ein-tilgen-des-verbindungspassworts-das-klick-und-anhaengeweg-nicht-halten.record.json`, `261016-0532-verbinden-an-einer-leeren-im-editor-offenen-secrets-txt-legt-eine-zweite-pin-fest.record.json`
**Domain:** code
**Verdict:** revise

Ein Commit, `9d1b111`. Jede Code- und Doku-Datei des Commits ist als Diff gelesen, dazu im Umfeld ganz: `verbindung_erfragen` bis `anhaengen_einziehen` (`appkit/anwendung.rs:9075-9283`), `einhaengen` bis `url_aus` (`appkit/verbinden.rs:230-440`), `verbindung_vorbereiten`, `geheimnisse::finden`, `notizen::angehaengt` samt den zwei ersetzten Helfern, `datei::einlesen` und `datei::gehaltene_form`, `tresor::verschliessen`, der Rumpf von `der_merker_hat_genau_einen_schreiber`, `HowTo.md:1600-1645` und `:1740-1810`. Die Dateien aus dem `Not-opened` der Durchsicht `261016-0532-reviewer-secrets-sechs-felder-schluesselmerker-verbindungen.md`, soweit Code, sind als Diff `bb2f861..6067ae9` nachgelesen (Abschnitt unten).

**Verification:** `make check` auf `9d1b111` grün („alle fuenf gruen“), darin `ein_schreibgang_wartet_auf_den_kopf_und_den_gesicherten_stand`, `debug_einer_entsperrten_datei_zeigt_keinen_klartext`, `das_blatt_weist_ab_was_keine_url_ergibt`, `die_url_kodiert_was_sich_kodieren_laesst`, `anhaengen_einziehen_baut_das_lesezeichen_allein_aus_dem_namen` und `der_verbindungsweg_stellt_auf_dem_hauptfaden_nur_den_einen_stat`. Dazu eine Gegenprobe mit `NSURL(string:encodingInvalidCharacters:)` in Swift auf diesem Gerät (Ergebnisse unter Thema B). Der Commit ändert keine Cargo-Datei; die C-Freiheit ist nicht neu erhoben.

## Summary

Alle vier Befunde der Vordurchsicht sind behoben, und keine Behebung bringt einen Fehler an der Stelle, die sie ändert: die `Zeroizing`-Übernahmen verhalten sich wie vorher, die Sperre deckt den gemeldeten Fall, die Proben prüfen jetzt, was ihr Name sagt, und `HowTo.md`, `README.md` und `CLAUDE.md` sagen beim Tilgen genau, was der Code hält. Neu sind drei kleine Befunde am Rand der Behebungen: die Sperre sieht einen laufenden Ladevorgang nicht und wird nach dem Blatt nicht wieder gefragt; die jetzt zugesagte Kodierung der Adresse trifft auf einen Vergleich, der roh gegen kodiert hält, und auf `#` und `?`, die den Pfad still kürzen; `HowTo.md` nennt die neue URL-Prüfung des Blattes nicht.

## Totals

| Critical | High | Medium | Low |
|---|---|---|---|
| 0 | 0 | 0 | 3 |

## Die vier Befunde der Vordurchsicht

**A. Zweite PIN an einer leeren, offenen `secrets.txt`: behoben.** `Editormodell::schreibgang_gesperrt` (`editormodell.rs:1331`) sperrt bei `abweichung` (wie vorher `hat_ungesicherten_stand`, `:1292`) und zusätzlich bei `Schutz::Verschluesselt { kopf_auf_platte: false }`, vollständig über `Schutz` ohne Auffangzweig. Vorgeschaltet ist `haelt_geheimnisse` (`:1383`), das neben dem Pfadtext auch `Schutz::Verschluesselt` fragt; eine dritte Schreibweise des Pfads fällt also nicht durch. Die Akzeptanz des Datensatzes hält die Probe am Modell. Was sie nicht deckt, steht unter Thema A.

**B. Tilgen: behoben, und die Zusage passt jetzt.**
- `Entsperrt::stand` ist `Zeroizing<String>`. Die entschlüsselten Bytes stehen in einem `Zeroizing<Vec<u8>>` und werden geliehen gewandelt (`tresorweg.rs:250-285`).
- `std::str::from_utf8` mit `gehaltene_form(...).into_owned()` ist gleichwertig zu `datei::einlesen` (`text/datei.rs:832-834`, `:875-893`): dieselbe UTF-8-Prüfung, dieselbe Wandlung. Der Editor bekommt denselben Stand wie vorher.
- `std::mem::take(&mut *entsperrt.stand)` (`editormodell.rs:817`) verschiebt ohne Abschrift. `entsperrt` fällt danach mit leerer Zeichenkette, die Teilverschiebung von `schluessel` ist zulässig, weil `Entsperrt` kein `Drop` trägt.
- `Chiffrat::verschliessen` hält eine gewachsene Sicherungsform in `Zeroizing`. **inference:** `Aead::encrypt` legt den Puffer mit Platz für den Prüfwert an und verschlüsselt an Ort und Stelle, also bleibt dort kein Klartext zurück; im Quelltext der Kiste nicht nachgelesen.
- `notizen::angehaengt` schreibt ohne die zwei Helfer gleichwertig (`themenzeile` war `"## {thema}\n"`, `textzeilen` leer oder `"{text}\n"`). Die Kapazität `stand + thema + text + 8` reicht für höchstens `+6`, also kein Umzug.
- `geheimnisse::hinzufuegen` legt den Text in passender Kapazität an (`grundform_laenge`, je Feld Schlüssel, Wert und drei Zeichen) und hält ihn in `Zeroizing`.
- Das eigene `Debug` von `Entsperrt` zeigt allein die Länge, gehalten von `debug_einer_entsperrten_datei_zeigt_keinen_klartext`.
- `entwurf_aus(Vec<String>)` nimmt die Felder mit `mem::take` heraus, das Passwort geht ohne Abschrift in sein `Zeroizing`.
- `HowTo.md:1622-1636`, `README.md` und `CLAUDE.md` sagen dasselbe wie die Modulköpfe von `tresorweg.rs` und `heimordner/tresor.rs`, Satz für Satz nachgelesen.

**C. Hauptfaden: behoben, wie vorgeschlagen.** Die Probe nennt beide Ausnahmen und zählt `.notizordner_bereitstellen(` und `.lesezeichen_aendern(` je Rumpf. `CLAUDE.md` und der Doc-Kommentar von `anhaengen_einziehen` nennen den Schreibvorgang. Die Nutzerfrage zur Klausel unter „Where this work stops“ ist laut `Resolved:` weiter offen und steht beim Orchestrator; dieser Bericht ändert daran nichts.

**D. Proben: behoben.**
- (a) Die Kernprobe heißt `ein_verbindungslesezeichen_schreibt_allein_name_und_verbindung` und sagt, dass sie die Form hält. `anhaengen_einziehen_baut_das_lesezeichen_allein_aus_dem_namen` hält den Betriebsweg: die Zeile `let name = entwurf.name.trim().to_owned();`, den `Aenderung::Anlegen`-Block wörtlich ohne Leerraum, und kein `entwurf.<feld>` der fünf übrigen Felder im Rumpf.
- (b) Die Zählung von `preisgeben` zählt jedes ganze Wort in Codezeilen außer nach `fn` und prüft sich an drei Beispielzeilen.
- (c) `der_merker_hat_genau_einen_schreiber` zählt zwei Rufer von `schluessel_merken`, drei Rufstellen von `gemerkt_falls` (eine in `verbindung_einziehen`, zwei in `anhaengen_einziehen`), und die Doc-Kommentare sagen dasselbe.

**Leerzeichen in der Adresse: geklärt.** `url_aus` (`verbinden.rs:439-441`) geht über `URLWithString:encodingInvalidCharacters:`, die Untergrenze macOS 14 steht im Untergrenzen-Abschnitt und liegt unter dem Zielsystem 15. Das Blatt fragt dieselbe Funktion (`blaetter/verbindung.rs:155-165`), der Doc-Kommentar von `entwurf_pruefen` ist berichtigt. Die Swift-Gegenprobe bestätigt jede Zeile der neuen Probe: `Daten Ordner` wird `Daten%20Ordner`, eine schon kodierte Adresse bleibt, `nas local` und `nas.local:abc` ergeben `nil`. Die zwei Texte `VerbindenPinNochNichtGesichert` und `VerbindenKeineUrl` stehen in `schluessel.rs`, in `ALLE` (647 → 649) und in allen drei Tabellen.

## Neue Befunde

### A. Die Schreibgangsperre wird einmal und zu früh gefragt (Low)

`261016-0614-verbinden-prueft-die-schreibgangsperre-nur-vor-dem-blatt-und-sieht-einen-laufenden-ladevorgang-des-editors-nicht.md`

`schreibgang_gesperrt` liest `abweichung` und `schutz`, die allein `uebernehmen` am Ende eines Ladevorgangs setzt (`editormodell.rs:1674-1677`). Gefragt wird sie allein in `verbindung_erfragen` (`anwendung.rs:9080-9088`), nicht in `verbindungsschluessel_holen` und nicht in `verbindung_anhaengen`. Fällt „Verbinden…“ in die Ableitung beim Festlegen einer PIN für eine leere `secrets.txt`, antwortet die Sperre über die vorige Datei, und es entsteht der Zustand des behobenen Befunds A. **inference:** aus den Zweigen gelesen, nicht gefahren. Das Zeitfenster ist eine Ableitung lang. Fix: dritte Lage „lädt `secrets.txt`“ und eine zweite Frage unmittelbar vor dem Schreibgang.

### B. Kodierte Adresse gegen rohen Vergleich, und `#`/`?` (Low)

`261016-0614-die-verbindungs-url-kuerzt-den-pfad-an-raute-und-fragezeichen-und-der-vergleich-mit-der-eingehaengten-freigabe-liest-roh-gegen-kodiert.md`

- `eingehaengte_freigabe(url)` bekommt die rohe URL (`verbinden.rs:336`) und hält sie über `gleiche_freigabe` (`:395-411`) als Text gegen das kodierte `absoluteString` (`:380`). Eine schon eingehängte Freigabe mit Leerzeichen oder Umlaut im Freigabenamen wird nie gefunden; der Klick meldet „schon verbunden“ statt den Einhängepunkt zu zeigen. Den Rückfall nennt `HowTo.md:1799-1800`, also falsch ist hier keine Zusage, sondern der Weg dahin. **inference** für die Kodierung der Wiedereinhänge-URL.
- Swift-Gegenprobe: `smb://nas.local/Daten#1/x` → `path /Daten`, `smb://nas.local/a?b` → `path /a`. `url_aus` kodiert beide Zeichen nicht, das Blatt nimmt die Adresse an, und der Klick hängt still einen anderen Pfad ein.

Beides lag vor `9d1b111` schon im Code, weil `URLWithString:` unter dem aktuellen SDK genauso kodiert. Erst der Commit sagt die Adresse mit Leerzeichen aber zu.

### C. `HowTo.md` und die URL-Prüfung des Blattes (Low)

`261016-0614-howto-nennt-die-pruefregel-des-blattes-verbinden-ohne-die-url-pruefung.md`

`HowTo.md:1747-1750` zählt die Bedingungen für „Anlegen“ ohne `KeineUrl` auf und sagt, die Zeile unter den Feldern nenne den Grund. Beim Tippen zeigt sie allein `Umbruch` (`blaetter/verbindung.rs:279-281`); `KeineUrl` erscheint erst nach der Eingabetaste.

## Die früher nicht geöffneten Dateien

Nachgelesen als Diff `bb2f861..6067ae9`, ohne Befund:

- `crates/krk-core/src/sprache/schluessel.rs`: die neuen Schlüssel stehen in Aufzählung und `ALLE`, die Zahl der Feldbreite stimmt mit der Liste (gehalten von `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`, grün).
- `crates/krk-core/src/verzeichnis/mod.rs`: der Modulkopf nennt acht Schnittstellen und zwölf gebundene Funktionen. Nachgezählt über die `extern "C"`-Blöcke von `verzeichnis/sys.rs`: `clock_gettime_nsec_np`, `copyfile` mit vier `copyfile_state_*`, `fcntl`, `flock`, `fsync`, `getattrlistbulk`, `localtime_r`, `renamex_np`. Die Zahl stimmt auf `9d1b111`, ist aber eine behauptete und keine abgeleitete Zahl; sie wird mit der nächsten Bindung falsch, wie es die Datei selbst seit Runden zeigt.
- `crates/krk-core/tests/heimordner.rs`: Proben zu Merker, Wachzeit, Format, `finden` und `verbindungsziel`. Die Grenzwerte 599/600 und die Uhr vor dem Merken sind abgedeckt. `das_verbindungsziel_nennt_den_typ_der_zugangsdaten_nicht` liest allein Zeilen, die nicht mit `//` beginnen, und das sagt ihr Doc-Kommentar.
- `crates/krk-ui/src/appkit/mod.rs` und `crates/krk-ui/src/main.rs`: je eine Zeile `mod verbinden;` und `mod tresorweg;`. `verbinden.rs` liegt unter der Ausnahme `#![allow(unsafe_code)]` von `appkit/mod.rs`, also braucht es keine dritte Ausnahme.

## Cross-cutting observations

- **Eine Frage, die vor einem Blatt gestellt wird, hält nicht für das, was nach dem Blatt geschieht.** Thema A ist dieselbe Bauart wie die Vorprüfung `tresorgang`, die `verbindung_anhaengen` deshalb ein zweites Mal fragt (`anwendung.rs:9170`). Für die Schreibgangsperre fehlt diese zweite Frage.
- **Die Adresse hat zwei Formen, und nur eine Stelle kennt beide.** `url_aus` macht aus der rohen die kodierte; `gleiche_freigabe` und der Kern rechnen mit der rohen. Thema B behebt sich, wenn alle Vergleiche über `url_aus` laufen.

## Recommended sequencing

1. Vor der Auslieferung: A, weil es der behobene Befund A durch eine zweite Tür ist und die Behebung zwei Zeilen Prüfung plus eine Lage braucht.
2. Mit A oder danach: C, ein Satz in `HowTo.md` und gegebenenfalls ein Zweig in der Grundzeile.
3. Danach: B. Der Rückfall „schon verbunden“ ist dokumentiert, und `#` oder `?` in einem Freigabepfad sind selten.
