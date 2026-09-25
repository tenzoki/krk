Die Belegungsprüfung bindet `return` noch an das Öffnen und schlägt seit S11c fehl

---

`crates/krk-core/tests/belegung.rs:347` führt in der Prüfung
`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` die Zeile

    ("return", Kommando::Oeffnen),

S11c hat `oeffnen` am 260804-1214 auf `cmd+right` umbelegt; `return` steht seither in keiner Tastenliste von `resources/default-keymap.toml`. Die Prüfung fällt damit auf `Nachschlag::Sprungmarke` und bricht mit `return trifft keine Funktion` ab (`crates/krk-core/tests/belegung.rs:351`).

Gemessen nach der Änderung:

    cargo test -p krk-core --test belegung
    test result: FAILED. 25 passed; 1 failed; 0 ignored

Vor der Änderung liefen dieselben 26 Prüfungen durch. Es ist die einzige fehlschlagende Prüfung im ganzen Paket: `cargo test -p krk-core` meldet daneben 26 von 26 im Bibliotheksteil und 19 von 20 (1 ignoriert) in `ablage`.

---

## Warum der `ontocoder` es nicht selbst behoben hat

Der Auftrag zu S11c begrenzt den Eingriff auf `resources/default-keymap.toml` und zwei Defektdateien und schließt `crates/` ausdrücklich aus. Die Datenänderung verlangt hier eine Codeänderung, und die gehört dem `coder`.

## Was zu tun ist

Die Zeile auf die neue Belegung ziehen:

    ("cmd+right", Kommando::Oeffnen),

Die Aussage der Prüfung bleibt unverändert: jedes gebaute Kommando hängt an der Taste, die die Auslieferungsbelegung ihm gibt. Nur das Beispiel wechselt, so wie S11b es bei `eine_fehlende_oder_unbekannte_taste_ist_ein_fehler` schon einmal getan hat.

## Was daran hängt

Das Abnahmekriterium von S11c verlangt `cargo test -p krk-core --test belegung` mit Rückgabewert 0. Solange diese Zeile steht, ist es nicht erfüllbar, und zwar unabhängig davon, ob die Belegungsdatei richtig ist: die inhaltliche Prüfung `die_auslieferungsbelegung_ist_konfliktfrei` läuft durch, ebenso `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste` und `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11c am 260804-1214, beim Lauf des Abnahmekriteriums.

---

Resolved: 260804 — `crates/krk-core/tests/belegung.rs:347` zieht das Beispiel auf `("cmd+right", Kommando::Oeffnen)`. Die Zusage der Prüfung bleibt unverändert: jedes gebaute Kommando hängt an der Taste, die die Auslieferungsbelegung ihm gibt. `cargo test -p krk-core --test belegung` meldet 26 von 26.
