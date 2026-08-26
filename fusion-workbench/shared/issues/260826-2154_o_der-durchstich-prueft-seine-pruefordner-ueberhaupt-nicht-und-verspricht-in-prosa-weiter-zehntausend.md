Der Durchstich prüft seine Prüfordner überhaupt nicht und verspricht in Prosa weiter 10.000

---

`960900d` hat die Eintragszahl-Prüfung in `Gesamtlauf::fahren` eingezogen. `Durchstich::fahren` (`crates/krk-bench/src/messen.rs:763-771`) ist der zweite Weg, der auf denselben zwei Prüfordnern L2, L3 und L10 misst, und er prüft **gar nichts**: kein `is_dir()`, kein Steckbrief, keine Eintragszahl. Seine Felder versprechen in Prosa weiter „Prüfordner A mit 10.000 Einträgen" (`:709`) und „Der Prüfordner mit 100.000 Einträgen" (`:711`) — genau die zwei Sätze, die der Commit in `Gesamtlauf` durch Verweise auf `EINTRAEGE_A` und `EINTRAEGE_GROSS` ersetzt hat.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Medium
**Domain:** code
**Tree state:** `fc829c8`
**Affected:** `crates/krk-bench/src/messen.rs:701-726` (`Durchstich`), `:761-771` (`fahren`), `:1979-1980` (`durchstich_bericht`)
**Cross-references:** `shared/issues/260826-1301_c_kein-pruefordner-ausser-dem-l6-unterordner-wird-gegen-seine-zugesagte-eintragszahl-gehalten.md`; `crates/krk-bench/src/main.rs:129`, `:251-300`

## Der Befund

`Durchstich::fahren` beginnt unmittelbar mit der Rundenschleife:

```rust
pub fn fahren(&self) -> io::Result<Durchstichergebnis> {
    let mut rohrunden = Vec::with_capacity(self.runden);
    let mut rate = None;
    for nummer in 1..=self.runden {
```

`Gesamtlauf::fahren` (`:1061-1078`) hat davor seit `960900d` die Schleife über die drei Ordner mit `is_dir()` und `pruefordner_pruefen`. Der Durchstich hatte auch vorher kein `is_dir()`; der Unterschied ist mit der Behebung größer geworden und steht nirgends geschrieben.

`krk-bench durchstich` ist ein lebender Unterbefehl (`main.rs:129`, `:251-300`), und `CLAUDE.md` führt `messungen/` als „Messberichte: kopflose Strecke, **Durchstich**, Abnahmereihen".

## Was der Durchstich misst und wodurch er ungedeckt bleibt

`spannen_messen` (`:879-909`) reicht `--ordner-a` und `--ordner100k` an die Anwendung weiter und liest L1, L2, L3 und L10 aus ihren Meldungen. Es läuft **keine** `Messreihe` darüber, also greift auch die zweite neue Prüfung nicht: der Abgleich der gelesenen Zahl gegen den Steckbrief steht in `Messreihe::fahren` (`:182-198`), und der Durchstich ruft `Messreihe` nirgends.

Damit gilt für den Durchstich unverändert, was der Datensatz `260826-1301` für den Gesamtlauf beschrieben hat: ein Prüfordner mit 3.000 Einträgen hält L3 mühelos, und nichts im Lauf sagt es. Der Berichtskopf (`:1979-1980`) weist über `ordner_beschreiben` allein den Steckbrief aus; `ordner_beschreiben_mit_gelesenen` gibt es hier nicht, weil es keine gelesene Zahl gibt.

## Vorschlag

`pruefordner_pruefen` ist schon eine freie Funktion in derselben Datei und nimmt Ordner und erwartete Zahl. Zwei Zeilen am Anfang von `Durchstich::fahren` decken den Weg — oder eine bewusste Gegenentscheidung mit einem Satz am Typ, warum der Durchstich als Frühmessung ohne Zusage-Bindung ohne die Prüfung auskommt. Die beiden Doc-Kommentare `:709` und `:711` ziehen in jedem Fall auf `EINTRAEGE_A` und `EINTRAEGE_GROSS` nach, sonst stehen zwei Zahlen im Baum, die der Commit anderswo gerade entfernt hat.

Gefunden bei der Durchsicht der Behebungsrunde 1, zweiter Teil, Bereich `9c02863..fc829c8`.
