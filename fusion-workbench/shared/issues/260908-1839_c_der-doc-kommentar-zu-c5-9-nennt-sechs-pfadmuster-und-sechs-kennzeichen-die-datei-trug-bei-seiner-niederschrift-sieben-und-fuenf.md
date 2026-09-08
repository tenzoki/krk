# Der Doc-Kommentar zu C5.9 nennt sechs Pfadmuster und sechs Kennzeichen, die Datei trug bei seiner Niederschrift sieben und fünf

Der Kopf von `die_mitgelieferten_profile_greifen_ausserhalb_einer_werkbank_nicht`
(`crates/krk-core/tests/leseprofil.rs`) schreibt: „die sechs Profile mit
Pfadmuster verlangen `fusion-workbench/` oder `flight-workbench/` im Pfad, die
sechs mit Kennzeichendatei verlangen einen der vier Einträge …". Der Absatz
darunter setzt die Zahl fort: „Ein siebentes Pfadmuster, das den Werkbanknamen
weglässt, ergäbe einen grünen Bau".

**Beide Zahlen waren am Tag ihrer Niederschrift falsch.** Der Kommentar kam mit
`56e5c2d` (260906) in den Baum; `resources/default-readers.toml` trug in
demselben Commit sieben Profile mit `pfad` und fünf mit `kennzeichen`, nicht
sechs und sechs:

```
git show 56e5c2d:resources/default-readers.toml | grep -c '^pfad = '         # 7
git show 56e5c2d:resources/default-readers.toml | grep -c '^kennzeichen = '  # 5
```

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Woran es aufgefallen ist

Beim Nachzug der Zahlenzusagen aus
`260908-1754_*_das-forum-profil-macht-fuenf-zahlenzusagen-in-drei-dateien-unter-crates-falsch.md`.
Das dreizehnte Profil hebt die Pfadmusterseite auf acht und machte die Stelle
ein zweites Mal falsch — die erste Abweichung stand schon vorher da und gehört
nicht zu jenem Befund.

## Abnahme

Der Kopf nennt keine der zwei Zahlen mehr, sondern die zwei Zählkommandos, und
die Aussage der Probe hängt an keiner Zahl: jedes Profil trägt eine der zwei
Sperren. Die Probe selbst ist unverändert, sie hat nie über eine Zahl geprüft.

---
Resolved: Die zwei Zahlwörter und das „siebente Pfadmuster" sind aus dem Kopf
von `die_mitgelieferten_profile_greifen_ausserhalb_einer_werkbank_nicht`
gestrichen. An ihrer Stelle stehen `grep -c '^pfad = '` und
`grep -c '^kennzeichen = '` über `resources/default-readers.toml` und der Satz,
dass die Aussage der Probe an keiner der zwei Zahlen hängt. Der Befund selbst
ist im Kommentar festgehalten, damit die Stelle nicht ein drittes Mal
durchgezählt wird. `cargo test -p krk-core` grün.
