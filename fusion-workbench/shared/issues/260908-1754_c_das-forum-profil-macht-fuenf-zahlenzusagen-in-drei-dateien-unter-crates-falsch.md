# Das Forum-Profil macht fünf Zahlenzusagen in drei Dateien unter `crates/` falsch

**Status:** offen
**Gefunden:** 260908-1754, Bahn R1 (Leseprofil für den Forum-Speicher)
**Für:** `coder`
**Betrifft:** `crates/krk-core/src/ablage/leseprofile.rs`, `crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/leseprofil.rs`

## Der Anlass

`resources/default-readers.toml` trägt seit der Bahn R1 ein dreizehntes Profil
(`fusion-Werkbank: der Forum-Speicher`) und je eine achte Zeile in den zwei
gekoppelten Wurzelprofilen (`fusion-Werkbank: die Wurzel`,
`Projektwurzel mit fusion-Werkbank`). Die Änderung liegt im Arbeitsbaum und ist
nicht committet.

**Die Datei selbst ist gültig.** `datei::pruefen` beanstandet nichts: die
Zusicherung `meldungen.is_empty()` in
`die_eingebettete_fassung_besteht_ihre_eigene_pruefung` läuft durch, und erst
die Zahl dahinter fällt. Es ist kein Schreibfehler zu suchen; es sind fest
verdrahtete Zahlen nachzuziehen.

**Warum das hier steht und nicht im Code:** die Bahn R1 hatte den Auftrag,
keine Datei unter `crates/` oder `xtask/` anzufassen, weil zwei andere Bahnen
dort arbeiten.

## Der gemessene Nachzug

Nicht geschätzt, sondern gefahren: in einem Wegwerf-Arbeitsbaum
(`git worktree`, seither entfernt) auf `85bcbad`, mit der geänderten
`resources/default-readers.toml`. Nach genau diesen Änderungen ist
`cargo test -p krk-core` grün — 240 + 90 + 63 + … Proben, null Fehlschläge.

### 1. Drei Zählstellen `12` → `13`

| Datei | Zeile | Stelle |
|---|---|---|
| `crates/krk-core/src/ablage/leseprofile.rs` | 219 | `keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` |
| `crates/krk-core/src/ablage/leseprofile.rs` | 257 | `die_eingebettete_fassung_besteht_ihre_eigene_pruefung` |
| `crates/krk-core/tests/ablage.rs` | 2312 | `eine_fehlende_readers_toml_entsteht_byteweise_…` |
| `crates/krk-core/tests/leseprofil.rs` | 3081 | Helfer `ausgelieferte()` |

Die Meldungstexte sprechen dabei von „den zwölf mitgelieferten Profilen" und
sind mitzuziehen, sonst nennt eine rote Probe künftig die falsche Zahl.

### 2. Der Prüfordner braucht einen Forum-Speicher

`werkbankbestand` (`crates/krk-core/tests/leseprofil.rs`, ab 3012) legt
`shared/forum` nicht an. Ohne diesen Zusatz bleibt die neue Zeile auf `--`
stehen und die Identität `leselaeufe == projektorte.len() + 1` bricht, weil
`genannte_orte` vier Orte aus dem Profiltext liest und nur drei davon auf der
Platte liegen:

```rust
let forum = wurzel.join("shared/forum");
std::fs::create_dir_all(&forum).expect("der Forum-Speicher laesst sich nicht anlegen");
schreiben(
    &forum,
    "260907-2354-1d05b0e4-read-before-pull.md",
    "Neuer Befehl: sehen, was angekommen ist, bevor du ziehst\n",
);
```

**Der Prüfordner ist die richtige Stelle und nicht die Zusicherung.** Wer
stattdessen die Identität auf „vorhandene Orte plus eins" umschriebe, nähme ihr
den Gegenstand: sie belegt den Halbsatz „plus einen Lauf für die Erkennung",
und dafür muss jeder genannte Ort auch wirklich gelesen werden.

### 3. Sieben Zusicherungen in `tests/leseprofil.rs`

Alle in `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`
und `ohne_orchestrator_live_zeigt_allein_die_sitzungszeile_ihren_platzhalter`:

| Zeile | war | wird | was |
|---|---|---|---|
| 3958 | sieben Beschriftungen | `+ "Nachrichten"` | Wurzelprofil |
| 3965 | `(3, 5)` | `(4, 5)` | Wurzelprofil, Haushalt |
| 3989 | sieben Werte | `+ Wert::Zahl(1)` | Wurzelprofil, Werte |
| 4065 | drei Orte | `+ "fusion-workbench/shared/forum"` | Projektwurzel, `genannte_orte` |
| 4096 | `(4, 5)` | `(5, 5)` | Projektwurzel, Haushalt |
| 4126 | sieben Werte | `+ Wert::Zahl(1)` | Projektwurzel, Werte |
| 4175, 4194 | sieben | `+ "Nachrichten"`, `+ Wert::Zahl(1)` | Sitzungszeilen-Probe |

**Die Öffnungen bleiben bei fünf.** `zaehlung` liest den Verzeichniseintrag und
öffnet keine Datei; die neue Zeile kostet je Wurzelprofil genau einen Leselauf
und null Öffnungen.

## Was NICHT nachzuziehen ist

`fusion-Werkbank: der gemeinsame Speicher` ist unangetastet: weiter zehn Orte,
`(10, 0)`, Abstand zur Schranke zwei. Die Gegenprobe
`ein_elfter_unterspeicher_kostet_einen_elften_leselauf` bleibt damit gültig.
Der Grund für diese Zurückhaltung steht im Entscheidungsdatensatz
`260908-1754_*_bekommt-das-profil-des-gemeinsamen-speichers-die-zwei-forum-zeilen-die-es-jedem-anderen-unterspeicher-gibt.md`.

## Zwei Prosastellen in der Profildatei sind schon nachgezogen

Zur Kenntnis, nicht zu tun: die Sätze „das Wurzelprofil … kostet drei Läufe,
das Projektwurzelprofil mit denselben sieben Zeilen vier" und die zwei
Hinweise „DIESE SIEBEN ZEILEN" stehen in
`resources/default-readers.toml` bereits auf vier, fünf und acht.
Der Doc-Kommentar über
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`
(ab 3807) führt dieselben Zahlen als Tabelle und ist mitzuziehen.

---
Resolved: Alle fünf Zählwerte sind nachgezogen, jeder vorher am Baum geprüft.
Die drei Zählstellen stehen auf `13`
(`crates/krk-core/src/ablage/leseprofile.rs` zweimal,
`crates/krk-core/tests/ablage.rs`, `crates/krk-core/tests/leseprofil.rs` im
Helfer `ausgelieferte()`); die Zahl bleibt dort eine Zahl, weil die Zusicherung
sie hält und ein neues Profil an ihr auffallen soll. Gestrichen sind dagegen die
Zahlwörter „zwölf" aus den vier Abbruchmeldungen und aus den Doc-Köpfen: neben
einer Zusicherung, die `left`/`right` ohnehin ausgibt, war das die zweite Kopie,
und genau die driftet. Der Kopf von
`keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` nennt jetzt
`grep -c '^\[\[' resources/default-readers.toml` als Erhebung.

Der Leselauf-Zusicherung ist über den **Prüfbestand** geholfen und nicht über
die Zusicherung: `werkbankbestand` legt `shared/forum` mit einem Eintrag an,
also liest das Projektwurzelprofil seinen vierten Ort wirklich und
`leselaeufe == projektorte.len() + 1` hält weiter, was es behauptet. Der
Doc-Kopf von `werkbankbestand` schreibt die Regel jetzt aus.

Die sieben Zusicherungen der zwei Proben tragen die Zeile „Nachrichten" und
ihren Wert `Zahl(1)`; die Haushalte stehen auf `(4, 5)` und `(5, 5)`, die
Öffnungen unverändert bei fünf. Der Doc-Kommentar über
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` ist samt
seiner Tabelle mitgezogen, `fusion-Werkbank: der gemeinsame Speicher` ist wie
verlangt unangetastet geblieben.

Nebenbei berichtigt und im Bericht genannt: der Kopf von `projektwurzel` sagte
„das achte mitgelieferte Profil" und nennt das Profil jetzt beim Namen, wie es
`profil_der_auslieferung` für sich schon fordert. Eine sechste Stelle war schon
vor dieser Änderung falsch und trägt einen eigenen Datensatz
(`260908-1839_*_der-doc-kommentar-zu-c5-9-nennt-sechs-pfadmuster-und-sechs-kennzeichen-die-datei-trug-bei-seiner-niederschrift-sieben-und-fuenf.md`).

`resources/default-readers.toml` ist nicht angefasst.
