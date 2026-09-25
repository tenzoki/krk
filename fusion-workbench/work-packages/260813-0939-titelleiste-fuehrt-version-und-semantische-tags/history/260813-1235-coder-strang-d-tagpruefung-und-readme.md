# Coder: Strang D der Runde 8 — die Tag-Prüfung und der Abschnitt in `README.md`

**Datum:** 260813-1235
**Agent:** coder (autonom, keine Rückfrage an den Nutzer)
**Status:** Complete
**Auftrag:** die Schritte D1 bis D5 aus
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
nicht mehr und nicht weniger. Strang A, B und C bleiben ausdrücklich liegen; ein
zweiter `coder` arbeitet gleichzeitig an `crates/krk-ui/`.
**Abnahme:** `make check` Exit 0 (build, test, clippy unter `-D warnings`, fmt).
Proben in `xtask` vorher 49, nachher 60.
**Nicht gefahren:** kein `make bundle`, kein `cargo xtask bundle`, kein
`cargo xtask release`. Unter `target/KRK.app` liegt ein beglaubigtes Bündel.

## Was gebaut wurde

**D1 — die reine Vergleichsfunktion.** `stand_pruefen(version, tags_auf_head,
geaenderte) -> Result<(), String>` in `xtask/src/release.rs`. Drei Zeichenketten
hinein, `Ok(())` im grünen Fall, sonst die fertige Abbruchmeldung; kein
Prozessaufruf, kein Dateizugriff, kein Git-Verzeichnis. Der Tagvergleich läuft
über die Zeilen und vergleicht auf Gleichheit, nicht auf Präfix: `v0.1.0-rc1`
und `v0.1.10` decken `0.1.0` nicht. Der Baumvergleich zählt jede nichtleere
Zeile und schreibt sie in die Meldung.

Die Meldung nennt beide Befunde in einem Stück, dazu je Befund die Version aus
der `Cargo.toml` und die Abhilfe als kopierbares Kommando. Sie schließt mit „Es
entsteht kein Auslieferungspaket." und nennt weder `--force` noch `--no-verify`
noch eine Marke zum Überspringen. Der Singular ist ausgeschrieben („1 verfolgte
Datei ist geaendert" gegen „2 verfolgte Dateien sind geaendert"), weil die
Klammerform „Datei(en)" in der gerenderten Meldung unleserlich war.

Zu C6.7 steht ein Satz im Doc-Kommentar: der Rückgabetyp ist `Result`, und
`Result` trägt `#[must_use]` schon in der Standardbibliothek. Ein zweites
Attribut daneben wäre Rauschen, und die nächste Erhebung sucht nicht danach.

**D2 — der eine `git`-Aufruf und die neue Station 1.** `git_fragen(wurzel,
argumente)` nach dem Muster von `security_fragen` (`sign.rs`), mit absolutem
Pfad `/usr/bin/git`, `.current_dir(wurzel)` und `.output()`. Startfehler und
Rückgabewert ungleich null werden beide zum Laufabbruch.

`auslieferungsstand_pruefen(wurzel)` stellt drei Fragen, jede als benannte
Konstante: `GIT_VERZEICHNIS` (`rev-parse --git-dir`), `GIT_TAGS` (`tag
--points-at HEAD`) und `GIT_STAND` (`status --porcelain
--untracked-files=no`). Alle drei lesen. Die Sollversion kommt aus
`bundle::VERSION`, das dafür von modulprivat auf `pub(crate)` gehoben ist, wie
`bundle::PLATZHALTER` es schon war; eine zweite Quelle der Zahl entsteht nicht,
und ein Zerteiler für die `Cargo.toml` auch nicht.

`release::ausfuehren` ruft die Station als erste Zeile nach der Argumentprüfung,
also vor `bundle::vorbereiten()`.

**D3 — die Stationszählung an ihren drei Stellen.** Modulkopf von `release.rs`,
Hilfetext in `main.rs` und der Abschnitt „Auslieferung" in `README.md` führen
danach dieselben sieben durchgehend numerierten Stationen, mit der Tag-Prüfung
als Station 1 und den drei Vorläufen als `Vorlauf a` bis `c`, jeder mit der
Station, der er zuarbeitet. `README.md` „Die sechste Station hat zwei äußere
Voraussetzungen" heißt jetzt „Die siebte Station".

Nachgezählt am fertigen Baum: die Zahl steht an genau diesen drei Stellen und an
keiner vierten (`grep -rn "sechs Stationen\|sechste Station\|sieben
Stationen\|siebte Station"` über `*.rs`, `*.md` und das `Makefile`).

**D4 — der Abschnitt `### Versionsstufen`.** Unter `## Versionspflege`
eingehängt, nicht als eigener Hauptabschnitt, weil „Versionspflege" schon
beantwortet, wo die Zahl wohnt, und „welche Zahl wann steigt" die Nachbarfrage
ist. Er nennt die drei Stufen an KRKs eigenen Flächen (Tastenbelegung samt der
Bedeutung ihrer Befehle, die Dateien unter `~/Library/Application
Support/KRK/`, das Mindest-Zielsystem, die Befehle des Bauwerkzeugs), sagt, dass
der Nutzer jeden Tag von Hand setzt, dass `v0.1.0` den ersten getaggten Stand
benennt und keine Weitergabe, was `release` prüft und was nicht, und dass die
angezeigte Zahl an jedem Bau dieselbe ist. Die Herkunft der Zahl wiederholt er
nicht, sondern verweist auf den Abschnitt darüber.

**Berichtigt:** „Eine neue Version wird also allein in der `Cargo.toml` gesetzt.
Nachzuführen ist nichts." Der zweite Satz wird mit der neuen Prüfung falsch und
lautet jetzt: im Baum ist nichts nachzuführen, nachzuführen ist der Tag.

**D5 — `bundle` und `make check` bleiben ohne Vorbedingung.** Eine Zählprobe
hält fest, dass genau eine Datei im Baum `auslieferungsstand_pruefen` nennt und
dass diese Datei `release.rs` ist; `bundle.rs` nennt weder den Aufruf noch
`/usr/bin/git`. Das `Makefile` ist unberührt geblieben.

## Elf neue Proben, und was sie halten

| Probe | Zusage |
|---|---|
| `ein_getaggter_und_sauberer_stand_geht_durch` | C3.14, der grüne Fall an der reinen Funktion |
| `unter_mehreren_tags_genuegt_der_passende` | C3.2 |
| `ein_fehlender_tag_haelt_die_auslieferung_an` | C3.1 |
| `ein_aehnlicher_tag_deckt_die_version_nicht` | über den Plan hinaus, siehe unten |
| `ein_geaenderter_baum_haelt_die_auslieferung_an` | C3.4, vorgemerkt und nicht vorgemerkt |
| `eine_geloeschte_verfolgte_datei_zaehlt_mit` | C3.4, gelöschte verfolgte Datei |
| `beide_befunde_stehen_in_einer_meldung` | C3.7 |
| `die_meldung_nennt_bedingung_version_und_abhilfe` | C3.8 |
| `ohne_git_verzeichnis_bricht_station_eins_ab` | C3.11 |
| `keine_der_drei_fragen_schreibt` | C3.10 |
| `die_standabfrage_laesst_unbeachtete_dateien_aussen_vor` | C3.5 und C3.6 |
| `xtask_ruft_git_an_genau_einer_stelle` | C3.13 |
| `allein_release_fragt_nach_tag_und_arbeitsbaum` | C3.12 |
| `die_standpruefung_steht_vor_der_ersten_uebersetzung` | C3.9 |

Die Tabelle führt vierzehn Zeilen bei elf neuen Probenfunktionen: drei Zusagen
teilen sich eine Funktion mit einer Nachbarzusage.

**Zwei Proben lesen ihre eigene Datei, und beide brauchen dafür eine Nadel aus
`concat!`.** `xtask_ruft_git_an_genau_einer_stelle` zählt
`Command::new("/usr/bin/git")` über jede `.rs`-Datei des Baums ohne `target/`
und `.git/`; ausgeschrieben zählte sie sich selbst mit. Dasselbe gilt für den
Namen der Station in `allein_release_fragt_nach_tag_und_arbeitsbaum` und in der
Reihenfolgenprobe.

**Die Reihenfolgenprobe benennt ihre Blindheit im Doc-Kommentar.** Sie schneidet
den Rumpf von `ausfuehren` aus `include_str!("release.rs")` und verlangt, dass
die Textstelle des Rufs vor der des ersten `bundle::uebersetzen` steht. Sie
liest die Textreihenfolge und nicht den Ablauf; was sie hält, ist die eine
Zusage, dass kein Abbruch dieser Art einen Übersetzungslauf kostet. Der
Ausschnitt endet am ersten `\n}\n`, deshalb stören die gleichlautenden
Zeichenfolgen im Prüfmodul darunter nicht.

## Drei Entscheidungen, die der Plan offen ließ

**Der Tagvergleich prüft auf Gleichheit der ganzen Zeile.** Der Plan sagt „sucht
`v` gefolgt von der Version". Wörtlich genommen deckte `v0.1.0-rc1` die
Auslieferung von `0.1.0`, und `v0.1.10` täte es auch. Die Probe
`ein_aehnlicher_tag_deckt_die_version_nicht` hält beide draußen. Das ist enger
als der Planwortlaut und nicht weiter; wer die Lockerung will, ändert eine
Zeile.

**Der Abbruch ohne Git-Verzeichnis nennt den Grund wörtlich mit.** Der Plan
verlangt eine Meldung, die genau sagt, dass kein Git-Verzeichnis vorliegt. Zwei
verschiedene Lagen landen dort: kein Repository, und `/usr/bin/git` lässt sich
nicht starten. Die Meldung lautet deshalb „Die Auslieferung braucht ein
Git-Verzeichnis, und in <Wurzel> ist keines zu befragen: <Grund>" und zitiert
den Grund. So steht auch im zweiten Fall keine falsche Behauptung da. Die
Alternative wäre eine zweite Fehlerform in `git_fragen` gewesen, also eine
Struktur mehr für einen Fall, den der Plan nicht trennt.

**C3.11 hat eine Probe bekommen, obwohl D2 keine nennt.** Der Spec markiert das
Kriterium mit **(Probe)**. Sie legt einen Wegwerfordner im Temporärverzeichnis
an, also außerhalb jedes Arbeitsbaums, und ruft die Station dort; sie ist die
einzige Probe der Datei, die `git` wirklich startet. Benutzt wird die
`Wegwerfwurzel`, die `release.rs` schon führt — die vierte Fassung im Baum,
deren Befund als `issues/260813-1110_o_eine-vierte-wegwerfordner-fassung-…`
liegt und nicht Gegenstand dieser Runde ist. Eine fünfte anzulegen wäre der
falsche Zug gewesen.

## Was offen bleibt

**Die sieben Kriterien aus C4 tragen im Spec ein (Probe), und der Plan sieht für
D4 keine vor.** `README.md` ist die einzige Datei des Schrittes, eine Probe
hätte also entweder in `xtask` gestanden und die `README.md` gelesen, oder sie
wäre nicht entstanden. Der Plan ist verbindlich, und er nennt nur die Datei; die
sieben Kriterien sind am Text nachgelesen und nicht maschinell abgenommen. Wer
sie maschinell will, braucht einen Schritt, der eine Probe an der Datei
vorsieht.

**Der grüne Fall der Prüfung ist an der reinen Funktion abgenommen und nicht an
einem Lauf.** Der Baum trägt keinen Tag, und `cargo xtask release` ist nicht
gefahren worden, weil es das beglaubigte Bündel unter `target/KRK.app`
überschriebe. Das ist E2 und Nutzerarbeit: nach `git tag v0.1.0` auf dem
Abschlusscommit lässt sich der Lauf einmal sehen.

**Der Arbeitsbaum ist während der Runde durchgehend geändert.** Ein Lauf der
neuen Prüfung würde heute an beiden Befunden zugleich abbrechen. Das ist die
Bedeutung von „ausgeliefert wird ein eingetragener Stand" und kein Fehler der
Prüfung.

## Abnahme

```
make check   → Exit 0
```

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets -- -D warnings` und `cargo fmt --all --check` laufen grün.
`clippy` ist hier die eigentliche Prüfung, weil `unused_must_use` erst unter
`-D warnings` ein Fehler ist.

Berührte Dateien, und nur diese:

- `/Users/k1/Projects/productive/krk/xtask/src/release.rs`
- `/Users/k1/Projects/productive/krk/xtask/src/main.rs`
- `/Users/k1/Projects/productive/krk/xtask/src/bundle.rs`
- `/Users/k1/Projects/productive/krk/README.md`

Nichts unter `crates/`. Der einmalige `cargo fmt --all` am Anfang hat die beiden
Dateien des zweiten `coder` nicht angefasst (an den Änderungszeitpunkten
nachgesehen); jeder weitere Formatlauf ging über `cargo fmt --package xtask`.
