# Coder: `readers.toml` wird die siebte Ablagedatei

**Datum:** 2026-08-24 09:40
**Status:** Complete
**Agent:** coder
**Baumstand:** `abecfb2`

## Auftrag

Schritt 2 des Plans
`planning/260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`,
Bündel A: `Datei::Leser` als siebter Wert der Ablageaufzählung, `Datei::ALLE` von
`[Datei; 6]` auf `[Datei; 7]`, je ein Zweig in `dateiname`, `format` und `leerbefund`, der
Modulkopf zieht mit, und in `tests/ablage.rs` wächst die ausgeschriebene Namensliste.
Dateien: `crates/krk-core/src/ablage/pfade.rs`, `crates/krk-core/tests/ablage.rs`.

## Was entstanden ist

`Datei::Leser` steht hinter `Datei::Einstellungen` und vor den zwei Zetteln, so wie
`Datei::ALLE` es nach ihrem Doc-Kommentar verlangt: erst die TOML-Dateien, danach die
Zettel. `dateiname` liefert `"readers.toml"`, `format` liefert `Format::Toml`,
`leerbefund` liefert `Leerbefund::Vorgabe` mit der Begründung aus C1.4 am Zweig — die
Datei wird von Hand gepflegt, KRK schreibt sie im Betrieb nie, also kann eine Datei ohne
obersten Schlüssel bei ihr kein Zeichen für einen Schaden sein.

**Der Übersetzer hat keine einzige Stelle außerhalb von `pfade.rs` verlangt.** Der Bau
lief nach der Erweiterung sofort durch; die drei vollständigen Fallunterscheidungen über
`Datei` stehen sämtlich in dieser einen Datei. Geraten wurde keine Stelle: gebaut und die
Fehlerliste gelesen, sie war leer.

Sechs Prosastellen in `pfade.rs` haben mitgezogen, weil die Zahl sonst falsch dastünde:
der Modulkopf (sechs → sieben Ablagedateien, vier → fünf TOML-Dateien), der
Doc-Kommentar der Aufzählung, der von `Datei::ALLE`, der von `Ablageort` und der von
`Ablageort::datei`. Dazu zwei Stellen, die „eine siebte Ablagedatei hält den Bau an"
sagten und jetzt „eine achte" sagen, und der Doc-Kommentar von `Datei::Einstellungen`,
der sich „die einzige der vier, die KRK im Betrieb nicht schreibt" nannte — es sind
seither zwei, und die andere ist genannt.

## Was der Plan nicht vorhergesehen hat

Der Plan sagt für `tests/ablage.rs`, die Rundläufe über `toml_dateien()` und `Datei::ALLE`
zögen von selbst mit. **Einer tut es nicht.**
`alle_vier_dateien_ueberstehen_schreiben_und_wiedereinlesen` schreibt vier TOML-Dateien
und prüft danach über `toml_dateien()`, dass jede von ihnen im Ablageordner liegt — mit
der fünften wurde die Zusicherung rot („readers.toml liegt nicht im Ablageordner").

Behoben ist es an der Wurzel und nicht an der Zusicherung: die Probe schreibt jetzt auch
`readers.toml`, und zwar auf demselben Weg wie `settings.toml`, also unter einem
`durchgang` über `atomar::schreiben`, weil beide von Hand gepflegt werden und nicht über
`Zugang::sichern` gehen. Zurückgelesen wird der nackte Text: die Ablage kennt von dieser
Datei bislang nur Namen und Pfad, und wer ihren Inhalt auswertet, kommt mit Schritt 3 und
Schritt 7.

## Ein Befund, gefiltert

`jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt` paart `toml_dateien()`
mit `ersetzungen_der_toml_dateien` über `zip`. Links stehen jetzt fünf Dateien, rechts
vier — `readers.toml` hat noch keinen Ladeweg —, und `zip` kürzt still. Die Probe bleibt
grün und prüft eine Datei weniger, als ihr Rundlauf verspricht.

Eine Zusicherung über die Länge wäre heute rot; ein fünfter Eintrag verlangt den Ladeweg
aus Schritt 7, und dessen Dateiliste nennt `tests/ablage.rs` nicht. Beide Doc-Kommentare
schreiben die Lücke deshalb aus, und der Datensatz
`issues/260824-0940_o_readers-toml-faellt-beim-zip-der-beiseitelegeprobe-still-heraus.md`
verlangt das Nachziehen.

Zwei bestehende Defekte im gemeinsamen Speicher haben je eine `Also seen:`-Zeile
bekommen, weil dieser Schritt ihren zitierten Wortlaut bewegt hat:
`shared/issues/260821-1023_o_…` (zwei ihrer sieben Fundstellen liegen in `pfade.rs`) und
`shared/issues/260816-2307_o_…` (der Datensatz erwartete die siebte Ablagedatei von der
zwölften Runde). Behoben ist keiner von beiden: beide sitzen in
`crates/krk-core/src/ablage/mod.rs`, und die Datei gehört Schritt 7.

## Prüfung

`make check` läuft grün: `cargo build --workspace`, `cargo test --workspace`,
`cargo fmt --all --check` und `cargo clippy --workspace --all-targets -- -D warnings`.
Exit-Code 0.

## Was nicht Gegenstand war

Kein Anlegen der Datei beim ersten Start (Schritt 7), keine Auslieferungsfassung
`resources/default-readers.toml` (Schritt 6, `ontocoder`), kein Profilmodell (Schritt 3).
`crates/krk-core/src/ablage/mod.rs` ist unberührt: sein Modulkopf zieht nach dem Plan mit
Schritt 7 von vier TOML-Dateien auf fünf.
