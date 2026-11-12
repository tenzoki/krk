# Schlussdurchsicht: die zwei Defekte nach den Protokollknöpfen

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `1cf9abbf..375bb474`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Work-item:** 261112-0826-zwei-offene-defekte-nach-den-protokollknoepfen-beheben.md
**Urteil:** accept

## Summary

Die drei Behebungen erfüllen die Abnahme ihrer Datensätze. `protokoll_umgeformt` verdirbt an keinem der geprüften Ränder Bytes, ändert nichts außerhalb des Ausschnitts und kann beim Schneiden nicht abbrechen. Die drei umgebauten Proben im Cloudladen halten, was ihr Name zusagt; eine überzeichnet in ihrem Kommentar, was ihre Fragenzahl hält. Offen bleibt eine Frage an den Nutzer zum Zellumbruch in einer CSV.

## Totals

Critical 0, High 0, Medium 0, Low 1, dazu eine Frage an den Nutzer und zwei Anmerkungen ohne Datensatz.

## Was geöffnet und geprüft wurde

- `crates/krk-ui/src/appkit/editor.rs`: `umgeformt`, `protokoll_umgeformt`, `ohne_geteiltes_zeilenende`, `zeilenende_des_ausschnitts`, der Rufer `Editorbereich::protokoll_umformen`, `stelle_zeigen`, beide Proben zur Umformung am Protokoll.
- `crates/krk-ui/src/appkit/koordinaten.rs`: `in_bytes`.
- `crates/krk-core/src/text/glaetten.rs`: `fliesstext`, der Kopf von `befehl`, `zerlegen`.
- `crates/krk-ui/src/cloudladen.rs`: `abwarten`, `Ladeplatz::belegen`, `Belegung::drop`, das ganze Prüfmodul.
- `.claude/rules/coding-was-man-nicht-sieht.md`: der geänderte Absatz über Singlestrip und Format.
- Die vier Defektdatensätze des Auftrags samt Steuerdatei, das Narrativ des Pakets und seine `package.json`.
- Ausgeführt: `cargo test -p krk-ui --bin krk -- cloudladen die_umformung_am_protokoll`, sechs Proben, alle grün, ein Lauf. Die zwanzig Läufe aus den `Resolved:`-Zeilen sind nicht wiederholt.

## Findings by theme

### Die Umformung am Protokoll (`f0a60eea`)

**Kein Befund an Schnitt, Bereich und Vollständigkeit.** Am Quelltext nachgerechnet:

- **Kein Abbruch beim Schneiden.** `in_bytes` ist monoton und klemmt am Textende, also gilt `anfang <= ende`. `ohne_geteiltes_zeilenende` rückt allein unter `anfang < ende`, um genau ein Byte, und dieses Byte ist `\n` oder `\r`, also ASCII: beide Enden bleiben auf einer Zeichengrenze, auch hinter einem Zeichen außerhalb der BMP. `geteilt(stelle)` liest über `bytes.get` und antwortet am Textende nein.
- **Außerhalb des Ausschnitts ändert sich nichts.** Der neue Stand ist `stand[..anfang]`, das Ergebnis, `stand[ende..]`.
- **Der Bytebereich stimmt.** `anfang` ist der gerückte Anfang, `ende` ist `anfang + nachher.len()`, mit `nachher` nach dem Einsetzen der Form. `stelle_zeigen` rechnet ihn gegen den Stand des Modells in UTF-16 zurück.
- **Die Fallunterscheidung ist disjunkt und vollständig.** Ohne `\r` im Ausschnitt läuft der Weg von vorher, Byte für Byte derselbe Ruf. Mit `\r` wird der Ausschnitt so gelesen, wie `zerlegen` in `glaetten.rs` ihn selbst liest (`\r\n` und `\r` zu `\n`), der Vergleich in `umgeformt` steht dann gegen diese Lesung, und die Form entscheidet eine Funktion mit zwei Armen.
- **Ränder.** Die Hälfte eines `\r\n` allein ergibt einen leeren Ausschnitt und `None`. Eine Auswahl aus allein `\r\n` verhält sich wie eine aus allein `\n` in einer Datei mit `\n`: Format `None`, Singlestrip nimmt den Umbruch (so schon vor der Änderung und in der Quicknote). Leerer Stand: `None`. Ein Ausschnitt ohne Zeilenende in einer Datei mit `\r\n` trägt kein `\r` und nimmt den alten Weg; `fliesstext` und `befehl` erzeugen kein `\n`, wo die Eingabe keines hatte (`aus.join("\n")` über höchstens so viele Zeilen wie die Eingabe, Schlussumbruch allein bei `schluss`).
- **Quicknote und gewöhnliche `.md`.** `quicknote_umformen` ist im Bereich nicht angefasst. Ein Stand allein mit `\n` trägt kein `\r`; `ohne_geteiltes_zeilenende` rückt dann nichts, und der Zweig ohne `\r` ist der Ruf von vorher.

**Frage an den Nutzer, kein Defekt:** ein geändertes Ergebnis bekommt eine einzige Zeilenendeform (`editor.rs`, `form => nachher.replace('\n', form)`). Ein Zellumbruch (einzelnes `\n`) in einer CSV mit `\r\n` kommt deshalb als `\r\n` zurück, sobald die Umformung irgendwo im Ausschnitt etwas ändert. Die Abnahme des Defekts verlangt genau das, und der Doc-Kommentar sagt es; der Defekt `261019-0037-ein-umbruch-in-einer-zelle-einer-crlf-csv-wird-bei-jedem-sichern-zu-crlf-auch-in-unberuehrten-zeilen.md` hat denselben Wandel beim Sichern als Fehler behandelt. Abgelegt als `261112-0851-darf-format-oder-singlestrip-in-einer-crlf-csv-einen-zellumbruch-im-ausschnitt-zu-crlf-machen.md`.

**Anmerkung ohne Datensatz:** der Zusatz in der Regeldatei sagt „in der Form des ersten Zeilenendes im Ausschnitt“. Genau ist es die Form des ersten `\n`: ein einzelnes `\r` als erstes Zeilenende entscheidet nichts. Der Satz verweist auf den Doc-Kommentar, der es genau sagt; der Zusatz beschreibt sonst den gebauten Stand.

### Die drei Proben im Cloudladen (`440d8b15`, `5d9f61fc`)

**Keine ist zur Tautologie geworden.** Ein `belegen` ohne Zeitgrenze am Warten stellt eine Frage und fällt an `HAENGT`; eines, das `weiter` nicht fragt, kehrt nach dem Abbruch nicht zurück. `neins == 1` ist ohne Wettlauf: nach dem Setzen der Marke ist die nächste Frage die erste mit Nein, und sie beendet die Schleife. Keine kann selbst hängen: jedes Warten der Probe trägt `HAENGT`, und `join` steht hinter dem Empfang. Der Doc-Kommentar von `HAENGT` sagt richtig, was nicht mehr gefangen wird (ein längerer Takt unter der Grenze); die Proben verlangen zwei Rückkehren aus dem Warten binnen `HAENGT`.

**Low: die Fragenzahl in `abwarten_bricht_ab_oder_wartet_die_dauer` hält weniger, als ihr Kommentar sagt.** `cloudladen.rs`, Zeilen 312 bis 314 und 350 bis 353. Ein `abwarten`, das die Dauer in einem Stück verschläft, stellt genau zwei Fragen und besteht `(2..=6)`; und die untere Grenze zwei fällt an einem richtigen `abwarten`, wenn der Faden zwischen `Instant::now()` und `elapsed()` fünf Schritte lang nicht zugeteilt wird (inference, selten, nicht gemessen). Den Takt hält die Schwesterprobe `ein_nein_mitten_in_der_wartezeit_beendet_sie`. Abgelegt als `261112-0851-die-fragenzahl-in-der-probe-zu-abwarten-haelt-den-takt-nicht-den-ihr-kommentar-zusagt.md`.

**Anmerkung ohne Datensatz:** die obere Grenze sechs ruht darauf, dass `thread::sleep` an `Instant` gemessen nie kürzer schläft als verlangt. Auf macOS teilen beide die Zeitbasis; inference, nicht gemessen.

### Abnahme der drei Datensätze

- `261112-0002-format-und-singlestrip-stellen-eine-crlf-csv-in-minutes-auf-lf-auch-wenn-sonst-nichts-zu-aendern-ist.md`: erfüllt. Die Probe hält `None` an vier Ständen mit `\r\n`, darunter gemischte, und kein einzelnes `\n` im umgeformten Ausschnitt.
- `261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md`: erfüllt über den zweiten Arm der Abnahme (Eigenschaft ohne Wanduhrschranke dieser Größe).
- `261112-0840-zwei-proben-zu-abwarten-im-cloudladen-tragen-dieselbe-freie-20-ms-wanduhrschranke-weiter.md`: erfüllt. Keine freie Schranke mehr; die Grenze hängt an `SCHRITT` und sagt, was sie nicht fängt. Der Befund oben betrifft den Kommentar zur Fragenzahl und nicht diese Abnahme.

## Cross-cutting observations

Der schon abgelegte Defekt `261112-0840-die-regel-welches-zeilenende-ein-text-traegt-steht-an-drei-stellen-und-keine-ruft-die-andere.md` steht; diese Durchsicht fügt ihm nichts hinzu. Die Lesung `\r\n` und `\r` zu `\n` steht seit `f0a60eea` ebenfalls zweimal, in `zerlegen` (`glaetten.rs`) und in `protokoll_umgeformt`; wer jenen Defekt behebt, nimmt sie mit.

## Recommended sequencing

Nichts hält das Schließen des Pakets auf. Der Low-Befund ist Aufräumarbeit am Prüfmodul. Die Frage zum Zellumbruch entscheidet der Nutzer; bis dahin gilt der gebaute Stand.
