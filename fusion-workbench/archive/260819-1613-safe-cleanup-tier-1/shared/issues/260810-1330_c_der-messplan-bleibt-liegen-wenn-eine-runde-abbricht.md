Der Messplan bleibt liegen, wenn eine Runde abbricht

---

`krk-bench` schreibt seinen Messplan als `krk-messplan-<pid>.toml` in das
Temporaerverzeichnis und loescht ihn erst hinter der letzten Runde. Bricht eine
Runde mit `?` ab oder faehrt der Messende mit Strg+C dazwischen, bleibt die Datei
liegen. Auf dem Referenzgeraet liegen neun davon, die aelteste vom 260805.

---

**Schwere:** Niedrig
**Gefunden:** coder, beim Nachweis fuer den Defekt
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1256_*_die-proben-des-vorschaumodells-legen-ihre-ordner-unter-festen-namen-an.md`
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Domain:** code

## Belegstellen

Geschrieben wird in `plan_schreiben` (`crates/krk-bench/src/messen.rs:1551`):

```rust
let pfad = std::env::temp_dir().join(format!("krk-messplan-{}.toml", std::process::id()));
std::fs::write(&pfad, text)?;
```

Geloescht wird an einer Stelle, und die liegt hinter der Rundenschleife
(`messen.rs:1046`):

```rust
for nummer in 1..=self.runden {
    let (gemeldete_rate, runde) = self.eine_gesamtrunde(&plan)?;   // <- Ausgang
    ...
}
let systemlast_nachher = systemlast();
let _ = std::fs::remove_file(&plan);
```

Der Rueckstand auf dem Referenzgeraet, am 260810-1330 aufgenommen:

```text
1020  Aug  5 23:58  krk-messplan-55095.toml
1020  Aug  6 12:15  krk-messplan-16615.toml
1020  Aug  6 12:16  krk-messplan-16677.toml
1020  Aug  6 12:16  krk-messplan-16748.toml
1200  Aug  6 13:54  krk-messplan-91514.toml
1200  Aug  7 15:10  krk-messplan-29958.toml
1200  Aug  7 15:50  krk-messplan-60917.toml
1200  Aug  7 16:02  krk-messplan-61288.toml
1200  Aug  7 17:28  krk-messplan-85953.toml
```

## Fehlszenario

Kein falsches Messergebnis: der Name traegt die Prozesskennung, zwei Laeufe
treffen sich also nicht, und ein neuer Lauf schreibt seine Datei neu. Was liegen
bleibt, ist ein Abbild der Sitzung des Messenden samt Pfaden (`kopierziel`,
`unterordner`, die Tabs beider Dateifenster) im Temporaerverzeichnis, unbegrenzt
lange und je Fehlschlag eines mehr. Ein Abnahmelauf, der abbricht, ist der
gewoehnliche Fall und nicht der seltene; die neun Dateien sind der Beleg.

## Vorgeschlagene Behebung

Das Mittel steht acht Zeilen ueber der Fundstelle: `Sitzungssicherung::anlegen`
(`messen.rs:1034`) ist ein Waechter, der die Sitzung des Nutzers im `Drop`
zurueckspielt, und der Kommentar daneben nennt genau diesen Grund — "spielt die
Sitzung des Nutzers auch dann zurueck, wenn eine Runde mit `?` abbricht oder der
Messende mit Strg+C dazwischenfaehrt". Der Messplan braucht dieselbe Bauform:
`plan_schreiben` gibt statt eines `PathBuf` einen Waechter zurueck, der den Pfad
haelt und im `Drop` `remove_file` ruft, und die eine Zeile bei `messen.rs:1046`
faellt weg. Kein zweiter Mechanismus, sondern der vorhandene ein zweites Mal
angewandt.

Die Probe `der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung`
(`messen.rs:2568`) raeumt heute selbst mit `remove_file` ab; sie zieht mit dem
Waechter nach und wird dabei um eine Zeile kuerzer.

## Zustaendigkeit

`coder`.

---
Resolved: `plan_schreiben` gibt seit dieser Sitzung einen `Messplanwaechter` zurueck statt eines
`PathBuf`. Die Struktur haelt den Pfad und ruft in `Drop` `remove_file`; damit fallen Erfolgsweg,
`?`-Abbruch und Panik-Abwicklung zusammen, und die Abraeumzeile hinter der Rundenschleife
(vormals `messen.rs:1046`) ist entfallen. Kein neuer Mechanismus: es ist die Bauform, die
`Sitzungswaechter` in derselben Datei und `Wegwerfordner` in `crates/krk-bench/src/wegwerfordner.rs`
schon tragen. Der Name lautet `Messplanwaechter`, weil `krk_ui::messmodus::Messplan` bereits
existiert. Die Probe `der_messplan_traegt_die_pruefsitzung_in_der_serialisierung_der_sitzung`
raeumt nicht mehr selbst ab. Abgenommen mit `make check`, exit 0; nach dem Prueflauf lag keine
neue Datei im Temporaerverzeichnis. Die neun Altbestandsdateien vom 260805 bis 260807 stehen
noch dort und sind nicht angefasst.

Ein Rest bleibt und ist eigens erfasst: `signalwache_starten` endet in `std::process::exit`,
dabei laeuft kein `Drop`, also bleibt der Messplan bei Strg+C weiter liegen. Siehe
`shared/issues/260810-1745_o_der-messplanwaechter-greift-bei-strg-c-nicht-weil-process-exit-kein-drop-laeuft.md`.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 1.
