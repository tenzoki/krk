Zwei Prosastellen an `ohne_warten_oeffnen` zaehlen fuenf Rufer und nennen den Schwungleser als einzigen `File::open`-Oeffner

---

Der Commit `9c02863` traegt den Verzeichnisleser in die Aufruferliste von `ohne_warten_oeffnen` ein, laesst aber den Ordinalsatz derselben Doc-Stelle bei fuenf stehen, und er behauptet an zwei Stellen, `Schwungleser::oeffnen` sei bis dahin der einzige Oeffner mit `File::open` gewesen. Beides stimmt am Baum `9c02863` nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/verzeichnis/sys.rs:855-858`, `:900-902`; `CLAUDE.md:151`
**Tree state:** `9c02863`
**Domain:** code

## Was dasteht

`crates/krk-core/src/verzeichnis/sys.rs:900-902`:

```
/// Der Defekt, der die Funktion verlangt hat, ist `260809-1652`; der zweite
/// Aufrufer ist mit `260810-1247` dazugekommen, der dritte mit der Runde 16, der
/// vierte und der fuenfte mit der Runde 17.
```

Der sechste Rufer, `Schwungleser::oeffnen` (`sys.rs:240`), steht seit demselben Commit vierzig Zeilen darueber in der Aufzaehlung (`sys.rs:855-858`), im Ordinalsatz aber nicht. `git grep -n 'ohne_warten_oeffnen(' 9c02863 -- crates/krk-core/src` zaehlt sechs Rufer ausserhalb von Doc und Probe.

`sys.rs:856-858` und `CLAUDE.md:151` sagen beide, der Verzeichnisleser sei „bis dahin als einziger Oeffner mit `File::open` an einer benannten Roehre haengen“ geblieben. `git grep -n 'File::open' 9c02863 -- crates/krk-core/src` findet daneben zwei Aufrufe im Code: `operation/kopieren.rs:198` (`ordnerangaben_uebernehmen`, oeffnet das gerade angelegte Ziel) und `operation/entpacken.rs:413` (dasselbe fuer den entpackten Ordner). Beide oeffnen einen Pfad, den der Code selbst kurz vorher geschrieben hat, und haengen nur, wenn er dazwischen durch eine Roehre ersetzt wird; das ist ein Wettlauf und kein gewoehnlicher Weg. Der Defektdatensatz `260826-1221_*_der-schwungleser-…` sagt genauer „der eine Oeffner **dieser Datei**“; die zwei neuen Stellen haben „dieser Datei“ fallen gelassen.

## Was zu tun waere

Den Ordinalsatz an `sys.rs:900-902` um den sechsten Rufer ergaenzen oder auf das Zaehlkommando verweisen, das der Satz danach ohnehin nennt. An `sys.rs:857` und `CLAUDE.md:151` „einziger Oeffner“ auf „einziger Oeffner dieser Datei“ oder „einziger Oeffner eines fremden Pfades“ engen. Ob die zwei `File::open` in `kopieren.rs` und `entpacken.rs` selbst ueber die Huelle gehen sollen, ist eine eigene Frage und nicht Teil dieses Datensatzes.

## Was geprueft ist

Gelesen am Baum `9c02863` mit `git show` und `git grep`; kein Lauf.
