Zwei Prosastellen in `entpacken.rs` sagen, `enclosed_name` weise einen absoluten Pfad ab; die Probe belegt das Gegenteil

---

Der Modulkopf und ein Kommentar im Rumpf sagen beide, `enclosed_name` liefere `None` fuer einen Namen mit fuehrendem Schraegstrich. Die Probe derselben Runde schreibt aus, dass ein solcher Eintrag **nicht** ausgelassen, sondern um den Schraegstrich gekuerzt und im Zielordner abgelegt wird — und zaehlt genau zwei ausgelassene Eintraege von vieren.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die drei Stellen

- `crates/krk-core/src/operation/entpacken.rs:49-53`: "**Der erste Weg ist der Name selbst**: `../../etc/passwd` oder `/etc/passwd`. Ihn versperrt `ZipFile::enclosed_name`, das fuer jeden Namen `None` liefert, der aus dem Zielordner herausfuehrte".
- `crates/krk-core/src/operation/entpacken.rs:218-220`: "`None` heisst: der Name fuehrte aus dem Zielordner heraus, sei es ueber `..`, sei es ueber einen fuehrenden Schraegstrich."
- `crates/krk-core/tests/operation.rs`, `ein_eintrag_der_aus_dem_zielordner_herausfuehrt_entsteht_nirgends`: der Eintrag `/absolut.txt` wird gegen `ziel.join("absolut.txt")` geprueft, mit der Meldung "der fuehrende Schraegstrich wird abgestreift, nicht der Eintrag ausgelassen", und `bericht.uebersprungen.len()` steht auf 2, nicht auf 3.

## Warum es zaehlt

Das Verhalten ist sicher: der Eintrag landet im Zielordner und nicht in der Wurzel, und die Probe belegt es. Falsch ist allein die Beschreibung des Mechanismus, und sie steht an der Stelle, an der der naechste Leser nachschlaegt, welche der zwei Sperren welchen Weg schliesst. Wer den Modulkopf glaubt, haelt `enclosed_name` fuer strenger, als es ist, und prueft die Abstreifung nicht mehr nach.

## Vorschlag

Beide Stellen auf das trennen, was die Kiste wirklich tut: ein Name mit `..`, der aus dem Zielordner herausfuehrte, liefert `None`; ein fuehrender Schraegstrich wird abgestreift, und der Eintrag bleibt im Zielordner. Die Probe steht schon da und braucht nichts.

## Umfang

`krk-core`, `operation/entpacken.rs`, nur Prosa.

---
Resolved: Beide Stellen in `crates/krk-core/src/operation/entpacken.rs` trennen jetzt die zwei
Ausgaenge: ein Name, der ueber `..` aus dem Zielordner herausfuehrte, liefert `None` und wird
ausgelassen; ein fuehrender Schraegstrich wird abgestreift, und der Eintrag entsteht im
Zielordner statt in der Wurzel. Der Modulkopf nennt dafuer die Probe
`ein_eintrag_der_aus_dem_zielordner_herausfuehrt_entsteht_nirgends`, die beide Ausgaenge
ausschreibt; der Kommentar im Rumpf verweist auf den Modulkopf. Nur Prosa, kein Code und keine
Probe geaendert.
