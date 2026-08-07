Zwei Leistenmodell-Proben benutzen feste Prüfordnernamen unter /tmp

---

Die neue Probe aus `2fbab30` legt ihren Prüfordner unter einem festen Namen an
und löscht ihn wieder. Zwei gleichzeitige Testläufe treffen denselben Ordner:
der eine löscht ihn, während der andere ihn als vorhanden erwartet.

---

## Die Stellen

- `crates/krk-ui/src/leistenmodell.rs:655` — neu:
  `std::env::temp_dir().join("krk-leiste-vorgang-beenden")`
- `crates/krk-ui/src/leistenmodell.rs:627` — schon vorher:
  `std::env::temp_dir().join("krk-leiste-gueltigkeit")`

Die neue Probe folgt der Schreibweise ihrer Nachbarin; der Befund gilt für
beide.

## Warum es ein Befund ist

Im selben Verzeichnisbaum steht die strengere Form. `Planordner::neu`
(`crates/krk-ui/src/messmodus.rs:1683-1694`) hängt die Prozesskennung und eine
Laufnummer an den Namen und räumt in `Drop` auf. Damit ist der Prüfordner je
Lauf eindeutig, und zwei gleichzeitige Läufe stören sich nicht.

Zwei Formen für dieselbe Sache in einer Kiste sind die Art von Vervielfachung,
die beim nächsten Eingriff auseinanderläuft. Dieselbe Klasse von Befund hat
`issues/260806-0014_c_pruefordner-unter-tmp-verlieren-leere-unterordner-an-die-systembereinigung.md`
schon einmal getroffen.

Ein zweiter Punkt betrifft die neue Probe für sich: sie räumt nur auf dem
Erfolgspfad auf. Scheitert eine der vier Zusicherungen, bleibt der Prüfordner
stehen — falls das `remove_dir_all` noch nicht gelaufen ist. `Planordner`
löst das über `Drop`.

## Denkbarer Weg

Beide Proben auf dieselbe Form bringen wie `Planordner`: Prozesskennung im
Namen und Aufräumen in `Drop`. Kein neuer Mechanismus, die Form steht bereits
in derselben Kiste.

## Dringlichkeit

Gering. `make check` läuft heute grün, und zwei gleichzeitige Testläufe im
selben Projekt sind kein Arbeitsablauf, den das Vorhaben kennt.

**Betrifft:** `crates/krk-ui/src/leistenmodell.rs`.

**Aufgefallen bei:** der inkrementellen Durchsicht nach Turn 25 der Sitzung
260806-2257, Diff `f9a0462..HEAD`, Commit `2fbab30`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-0014_c_pruefordner-unter-tmp-verlieren-leere-unterordner-an-die-systembereinigung.md`
