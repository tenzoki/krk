# Sechs Behauptungen werden Messungen

**Status:** Complete
**Agent:** coder
**Baumstand bei Beginn:** `926377f`

## Auftrag

Sechs Befunde aus den Durchsichten der Bündel C und D, alle mit derselben
Leitfrage: kann die behauptete Eigenschaft gemessen werden, und was kostet die
Messung? Auslöser ist der Polaritätsfehler vom 260817-1640, den der Nutzer
gefunden hat und keine Probe.

## Was gebaut wurde

### 1. `260817-2355` — der Prüfkörper mit dem alten Schaltflächentext

`blaetter/loeschbestaetigung.rs`: beide Vorkommen von `"Endgültig löschen"` auf
`"In den Papierkorb räumen"` gesetzt, den Wortlaut des einen Aufrufers. Die
Schwesterprobe zwei Blöcke darüber trug die Kurzform `"In den Papierkorb"` und
ist mitgegangen; der Datensatz nennt sie als schon richtig, das stimmte nur
ungefähr.

### 2. `260817-2354` — die zweite ab Werk freie Kombination

`krk-core/tests/belegung.rs`: `die_ab_werk_freien_kombinationen_kommen_nicht_vor`
läuft wieder über eine Liste, jetzt mit `shift+delete` und `opt+cmd+delete`.
Kommentarblock nachgezogen. Nachweis über eine Ersatzkombination (`f8`), weil
`resources/default-keymap.toml` nicht zu dieser Aufgabe gehörte.

### 3. `260817-1804` — die 25 an vier Stellen

Die **bestehende** Übersetzungszusicherung ist erweitert statt eine zweite
danebenzusetzen: sie liest die beiden Wortlaute selbst und sucht darin die
Dezimalschreibung von `SCHWELLE`. `Warngrund::wortlaut` ist dafür `const fn`,
`nennt_die_zahl` ist eine `const fn` daneben (`str::contains` ist nicht `const`).
Das zweite Zahlwort im `assert!` ist ganz weg. Dazu liest `warngruende` den Wert
an `Umfang::MehrAls(n)` statt ihn wegzuwerfen; gemessen an der bestehenden Tafel
`der_umfang_loest_ab_der_schwelle_aus`, von sechs auf acht Zeilen gewachsen.

### 4. `260817-1759` — die ungemessene Verdrahtung in `loeschtexte`

Neues Prüfmodul `loeschzielproben` in `appkit/anwendung.rs`, zwei Proben. Nicht
der vom Datensatz vorgeschlagene Fall, sondern ein stärkerer: ein Ziel, das auf
allen Auslösern `Nein` sagt, fängt keinen Tausch. Die zwei Proben stellen je
einen Ort her, an dem genau eine der beiden Tatsachen zutrifft — ein
Prüfordner mit `.git` und der Einhängepunkt der `/home`-Automatik.

### 5. `260817-1419` (Polarität) — die drei Prosaaussagen

Zählproben in `appkit/papierkorb.rs` und `kommandos/loeschwarnung.rs`, Bauform
der Vorlage aus `appkit/volumes.rs`. Der Einwand, eine Zählung über
`loeschwarnung.rs` überverspreche, trifft nicht mehr: der Modulkopf sagt selbst,
dass die Frage dort auch für die erste Polarität unbrauchbar ist.

### 6. `260817-1419` (Blattzusicherung) — die Zusicherung, die in keinem Bau griff

Aus dem `debug_assert!` ist ein `assert!` geworden, gemessen an der
Zeichenfolge im Auslieferungsbau (0 vorher, 1 nachher). Dazu erreicht
`ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf` die Zeile im Probenbau: die
Zusicherung steht vor `NSAlert::new`, also wird AppKit nie angesprochen. Der
Bauplan von `Blatt::neu` steht jetzt als reine Funktion da und ist eigens
gemessen.

## Zwei Entscheidungsdatensätze gestellt

Beide Wahlpunkte, die aus den Befunden hervorgingen, stehen jetzt in
`decisions/` statt in `issues/`, weil ihre Auflösung „entscheiden und
festhalten" heißt und nicht „hingehen und beheben":

- `260818-0249_o_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md`
- `260818-0250_o_verlangt-der-blattbauer-die-liegenlassende-schaltflaeche-am-typ.md`

## Nachweis je neuer Probe

Jede neu gebaute Probe ist mit eingebautem Fehler rot gesehen und danach
zurückgenommen worden. Die Einzelheiten stehen im jeweiligen Datensatz unter
`Resolved:`.

## Abnahme

`make check` — exit 0.
