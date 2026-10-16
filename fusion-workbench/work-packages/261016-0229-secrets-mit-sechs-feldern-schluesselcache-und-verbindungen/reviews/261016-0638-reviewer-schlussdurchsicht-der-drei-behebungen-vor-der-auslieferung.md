# Schlussdurchsicht: die drei Behebungen an Sperre, Freigabevergleich und Blattprüfung

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `9d1b1112..0272a865`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Domain:** code
**Verdict:** revise

Ein Commit, `0272a86`. Jede Code- und Doku-Datei ist als Diff gelesen. Im Umfeld zusätzlich gelesen: `pruefen` und der Tipp-Rückruf in `appkit/blaetter/verbindung.rs`, `gleiche_freigabe` und `eingehaengte_freigabe` in `appkit/verbinden.rs`, `verbindungsschluessel_holen` und `verbindung_anhaengen` in `appkit/anwendung.rs`, die Stellen von `ladevorgang` und `zurueckgehalten` in `editormodell.rs`. Dazu `HowTo.md`, Abschnitte „Eine Verbindung anlegen“ und „Eine Verbindung herstellen“. Die drei Issue-Datensätze `261016-0614-*` und der Testfall-Issue zum 500-MB-Abbruch sind Werkbankbestand und gelesen, soweit sie den Code betreffen.

**Verification:** auf `0272a86` grün: `ein_laufendes_lesen_von_secrets_txt_sperrt_den_schreibgang`, `ein_schreibgang_wartet_auf_den_kopf_und_den_gesicherten_stand`, `der_verbindungsweg_reisst_nirgends_ab`, `eine_freigabe_wird_kodiert_und_roh_als_dieselbe_erkannt`, `raute_und_fragezeichen_bleiben_im_pfad_der_freigabe` (`krk-ui`) und `raute_und_fragezeichen_beenden_den_pfad_einer_freigabe_nicht` (`krk-core/tests/heimordner.rs`). `cargo fmt --all --check`, `cargo clippy --workspace --all-targets` ohne Warnung, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` grün. Der volle `cargo test --workspace` ist nicht gefahren. Dazu eine Swift-Gegenprobe mit `NSURL(string:encodingInvalidCharacters:)` auf diesem Gerät (Ergebnisse unter B und C).

## Summary

Alle drei Befunde der Nachdurchsicht sind im Code behoben, und keine Behebung bringt einen Fehler im Code. Neu ist ein Doku-Defekt: der neue Absatz in `HowTo.md` steht mitten in der Tabelle der Verbinden-Lagen und reißt deren letzte Zeile heraus. Ein kleiner Satz in `HowTo.md` und im Kommentar des Blattes ist für IPv6-Adressen ungenau, ohne Issue.

## Totals

| Critical | High | Medium | Low |
|---|---|---|---|
| 0 | 0 | 0 | 1 |

## Die drei Befunde der Nachdurchsicht

### A. Schreibgangsperre beim Laden und vor dem Schreiben: behoben

- `Schreibgangsperre::Laedt` (`editormodell.rs`, Enum vor `impl Schreibgangsperre`) sperrt in zwei Lagen. Erstens, wenn `ladevorgang` ein Lesen mit `Leseauftrag::Geheimnisse` trägt. Das neue Feld `Ladevorgang::geheimnisse` wird in `Ladevorgang::starten` aus genau dem Auftrag gesetzt, der dem Faden mitgegeben wird. Zweitens, wenn `zurueckgehalten` einen gelesenen Stand mit `Schutz::Verschluesselt` hält.
- Jede Stelle, die einen Vorgang beendet, setzt `ladevorgang = None`. Eine Sperre, die nach dem Einziehen stehen bleibt, gibt es also nicht. Die Probe zeigt das: nach `abwarten_mit_ableitung` kommt `OhneKopf` und nicht `Laedt`.
- Die Prüfung steht vor `haelt_geheimnisse`. Damit greift sie auch, wenn der Editor bis dahin eine gewöhnliche Datei hält, und das war der gemeldete Fall.
- `verbindung_anhaengen` fragt die Sperre ein zweites Mal, nach `tresorgang` und vor `tresorgang.set(true)` und `verbinden::anhaengen`. Beide Wege dorthin (gemerkter Schlüssel, PIN-Blatt) laufen durch diese Funktion. Bei Abweisung wird nichts geschrieben und kein Schlüssel gemerkt. Die Quelltextprobe hält die Reihenfolge Sperre vor Gang.
- Text `VerbindenEditorLaedt` steht in `schluessel.rs`, in `ALLE` (649 → 650) und in `de.rs`, `fr.rs`, `en.rs`. Der deutsche Satz trägt Umlaute, die Tabelle ist der erlaubte Ort.

### B. Freigabevergleich dekodiert, `#` und `?` auf dem Einhängeweg: behoben

- `gleiche_freigabe` dekodiert Rechner und Freigabe auf beiden Seiten über `prozent_dekodiert`. Die Funktion ist korrekt: `hexwert` über `char::from(byte).to_digit(16)` lässt Bytes ab `0x80` nicht als Hexziffer durch. `hi*16+lo` liegt höchstens bei 255. Ein `%` ohne zwei Hexziffern bleibt stehen. Die Probe deckt Leerzeichen, Umlaut samt Konto vor dem Rechner und die kodierte Raute.
- `ohne_anfrage_und_fragment` steht allein im Zweig `Weg::Einhaengen` von `verbindungsziel`. Der Übergabeweg bleibt unverändert, gehalten von der dritten Zeile der Kernprobe.
- Swift-Gegenprobe: `url_aus` kodiert ein schon gesetztes `%23` nicht doppelt (`smb://nas.local/Daten%231/x` → `path /Daten#1/x`). Das hält auch `raute_und_fragezeichen_bleiben_im_pfad_der_freigabe`. Die Plan-URL mit `%23` und die Wiedereinhänge-URL des Systems treffen sich dekodiert in `gleiche_freigabe`.
- Was die Kodierung nicht anfasst, lag schon vorher im Code und ist kein Befund: ein vom Nutzer getipptes `%` im Pfad, das `url_aus` je nach Folgezeichen als `%25` kodiert oder als Kodierung liest.

### C. `KeineUrl` beim Tippen und `HowTo.md`: behoben, mit einer Ungenauigkeit

- Der Tipp-Rückruf zeigt jetzt `Umbruch` und `KeineUrl`. `pruefen` gibt `KeineUrl` erst, nachdem `entwurf_pruefen` und `verbindungsziel` bestanden haben. Ein halb getipptes Protokoll kommt also als `Ziel(_)` an und bleibt beim Tippen still, wie Kommentar und `HowTo.md` sagen.
- `HowTo.md` nennt die vierte Bedingung, die Pfadzeichen und wann die Grundzeile spricht. Das stimmt mit dem Code überein.
- **Ungenau, ohne Issue:** Kommentar und `HowTo.md` begründen die sofortige Anzeige mit „Weitertippen heilt beide nicht“. Für eine IPv6-Adresse stimmt das nicht. Swift-Gegenprobe: `smb://[fe80::1` ergibt `nil`, `smb://[fe80::1]/x` eine URL. Wer `[fe80::1]` tippt, sieht den Grund, bis die Klammer zu ist. Der Knopf und die Meldung sind dabei richtig, nur die Begründung ist zu allgemein. Ein Rechnername als IPv6-Literal ist bei einer Freigabe selten. Kein Issue; wer den Absatz ohnehin anfasst (siehe D), kann „in der Regel“ setzen.

## Neuer Befund

### D. Der neue Absatz zerreißt die Tabelle der Verbinden-Lagen (Low)

`261016-0638-howto-zerreisst-die-tabelle-der-verbinden-lagen-mit-einem-eingeschobenen-absatz.md`

In `HowTo.md`, Abschnitt „Eine Verbindung anlegen“, steht der neue Absatz „Diese drei Lagen prüft KRK zweimal …“ zwischen der Zeile „`secrets.txt` wird im Editor gerade geöffnet“ und der Zeile „Der Eintrag steht, das Lesezeichen ließ sich nicht anlegen“. Die Leerzeile davor beendet die Tabelle. Die letzte Zeile folgt dem Absatz ohne Leerzeile und wird als Fortsetzung des Absatzes gesetzt, also als Fließtext mit Pipes. `HowTo.md` reist im Releasepaket mit, der Nutzer liest den Bruch also. Fix: Zeile zurück in die Tabelle, Absatz darunter, die drei Lagen beim Namen nennen.

## Beobachtung ohne Issue

**Die Gegenrichtung ist nicht Gegenstand dieses Commits.** Die Commit-Nachricht sagt „so entsteht auf keinem Weg eine zweite PIN“. Gedeckt ist der Weg über „Verbinden…“: die Sperre steht vor dem Blatt und vor dem Gang. Nicht gedeckt ist, dass der Editor `secrets.txt` öffnet, während ein Gang von „Verbinden…“ läuft. Der Weg in den Editor fragt `tresorgang` nicht (`grep -n tresorgang crates/krk-ui/src/appkit/*.rs` zeigt allein die Wege von Verbinden und Herstellen). Ob daraus bei einer vorher leeren `secrets.txt` eine zweite PIN entsteht, hängt davon ab, wie `geheimnisse_laden` ein Festlegen an einer inzwischen beschriebenen Datei beantwortet. Das habe ich nicht gelesen, und das Zeitfenster ist so lang wie eine Ableitung. **unverified.** Kein Issue, weil der Ablauf nicht belegt ist. Frage an den Orchestrator, ob das in die nächste Runde gehört.

## Cross-cutting observations

- Die zweite Frage vor dem Gang folgt jetzt der Bauart von `tresorgang`: einmal vor dem Blatt, damit niemand umsonst Felder füllt, und einmal unmittelbar vor der Wirkung. Das schließt den Befund der Nachdurchsicht an seiner Wurzel.
- Die Adresse hat jetzt drei Formen: roh beim Nutzer, mit `%23`/`%3F` im Kern, voll kodiert nach `url_aus`. Der Vergleich dekodiert, statt eine Form zu erzwingen. Das hält, solange niemand außer `gleiche_freigabe` zwei dieser Formen vergleicht.

## Recommended sequencing

1. Vor der Auslieferung: D. Ein Doku-Fix an Tabelle und Absatz ohne Codeänderung, ohne neuen Abnahmelauf.
2. Optional mit D: die Begründung „Weitertippen heilt beide nicht“ abschwächen.
3. Danach, falls gewünscht: die Gegenrichtung aus der Beobachtung an `geheimnisse_laden` nachlesen.
