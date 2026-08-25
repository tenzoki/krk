# Coder: Die Kiste `zip` kommt in den Arbeitsbereich, und eine Zahl des Plans hält nicht

**Datum:** 2026-08-25 08:22
**Status:** Complete
**Agent:** coder
**Baumstand:** `fe1aff5`

## Auftrag

Schritt 1 des Plans
`planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`: `zip` und `flate2`
unter `[workspace.dependencies]` der Wurzel-`Cargo.toml` aufnehmen, beide mit der
ausführlichen Begründung, die jede fremde Kiste dieses Projekts an dieser Stelle trägt,
und beide in `crates/krk-core/Cargo.toml` unmittelbar nennen. Die sechs gemessenen
Aussagen aus dem Abschnitt `### Die Kiste und der Grund` sind wörtlich zu übernehmen und
nicht neu zu messen.

## Was entstanden ist

Die Wurzel-`Cargo.toml` trägt `zip` und `flate2` hinter `regex` und vor `signal-hook`,
also bei den Kisten des Kerns und nicht bei denen der Oberfläche. Davor stehen 66 Zeilen
Begründung, die vier Dinge nennt: warum ein Unterprozess auf `ditto(1)` oder `zip(1)`
ausscheidet und warum ein eigener Zip-Schreiber ebenfalls; dass keine bestehende
Abhängigkeit ein Archivformat leistet, `flate2` eingeschlossen, das den Datenstrom kennt
und nicht den Behälter um ihn; die sechs gemessenen Aussagen des Plans; und die
Merkmalswahl als Bedingung der Einbindung und nicht als Sparsamkeit.

`flate2` bekommt an beiden Stellen den Satz, dass es nicht überflüssig ist. In der
Wurzel-`Cargo.toml` verweist er auf den vierten Aufzählungspunkt darüber, in
`crates/krk-core/Cargo.toml` steht der Mechanismus noch einmal aus: `zip` schaltet
`flate2` ohne dessen Vorgabemerkmale ein und stellt damit gar kein Verfahren ein, worauf
`flate2` den Bau mit „No compression backend selected" anhält. Ohne diesen Satz fiele der
Eintrag beim nächsten Aufräumen als scheinbar unbenutzt weg; kein Modul nennt den Namen
`flate2` heute unmittelbar.

`crates/krk-core/Cargo.toml` nennt beide hinter `regex`, mit je einer Begründung, warum
sie im Kern und nicht in `krk-ui` stehen: der Lauf liegt in `operation::zippen` und
`operation::entpacken`, die Proben brauchen den selbstabräumenden Prüfordner aus
`tests/gemeinsam/mod.rs`, und `krk-ui` hat kein Bibliotheksziel.

## Die Messung, und wo sie vom Plan abweicht

Erhoben mit `cargo build --workspace`, `cargo tree --workspace -e normal,build` und dem
Vergleich der `Cargo.lock` vor und nach der Aufnahme.

| Frage | Erwartung des Plans | Gemessen im Baum |
|---|---|---|
| Wie viele Pakete kommen dazu? | zwei, `zip 8.6.0` und `typed-path 0.12.3` | zwei, genau diese; `cargo` meldet „Locking 2 packages" |
| Steigt `crc32fast`? | von 1.5.0 auf 1.5.1 | **nein, es bleibt bei 1.5.0** |
| `cc` oder ein `-sys`-Paket? | keines | keines; `cargo tree --workspace -e normal,build` nennt im ganzen Baum weder `cc` noch einen Namen auf `-sys`, auch `windows-sys` nicht, das wie zuvor allein in `Cargo.lock` steht |
| Läuft der Bau? | `make check` grün | grün, Exit-Code 0 |

**Die Zahl zu `crc32fast` hält nicht, und die Begründung in der Wurzel-`Cargo.toml`
schreibt das aus, statt sie zu wiederholen.** Der Plan hat sie in einem
Wegwerf-Workspace erhoben, dessen `Cargo.lock` frisch aufgelöst wurde und deshalb die
neueste Fassung nimmt. KRKs `Cargo.lock` hält 1.5.0, und `zip 8.6.0` begnügt sich damit;
`cargo` löst den Eintrag gar nicht erst neu auf. Der Zuwachs bleibt damit zwei von zwei
statt zwei plus eine Fehlerbehebungsstufe. Die übrigen fünf Aussagen sind wörtlich
übernommen und nicht neu gemessen, wie der Auftrag es verlangt.

Die neun Kisten, auf denen `zip` sonst noch aufsetzt, standen bereits in den Fassungen im
Baum, die der Plan nennt: `flate2 1.1.9`, `miniz_oxide 0.8.9`, `adler2 2.0.1`,
`simd-adler32 0.3.10`, `cfg-if 1.0.4`, `memchr 2.8.3`, `indexmap 2.14.0`,
`hashbrown 0.17.1` und `equivalent 1.0.2`.

## Prüfung

`make check` läuft grün über den ganzen Workspace: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings`. Exit-Code 0.

Ein unbenutzter Eintrag unter `[dependencies]` löst in diesem Baum keine Warnung aus,
`unused_crate_dependencies` ist nirgends eingeschaltet. Beide Kisten stehen damit bis
Schritt 2 ohne Rufer da, ohne dass der Bau daran hängt — und genau das ist der Grund,
aus dem der Satz zu `flate2` an beiden Stellen dastehen muss.

## Was nicht Gegenstand war

Kein Code, der eine der beiden Kisten nennt: `operation::zippen` entsteht in Schritt 2,
`operation::entpacken` in Schritt 3. `resources/default-keymap.toml` bleibt unberührt,
weil kein Befehl dieser Runde eine Tastenkombination bekommt. `Cargo.lock` hat sich als
Folge des Baus geändert und ist nicht von Hand angefasst.
