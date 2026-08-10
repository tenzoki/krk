Ein wiederholtes Sammelersetzen legt je Ruf einen Bereich in Dateigroesse in den Stapel
---
`Umkehrpunkt` traegt seit `260810-1241` den Bereich zwischen der ersten und der letzten geaenderten Stelle. Bei einem Sammelersetzen, dessen Ersatztext den Suchtext enthaelt, findet der naechste `ctrl+cmd+r` wieder Treffer, und dieser Bereich deckt beinahe die ganze Datei. Wiederholte Rufe legen deshalb je Ruf einen Bereich in Dateigroesse in einen Stapel ohne Tiefengrenze.
---
**Schwere:** Niedrig
**Gefunden:** bei der Behebung von `260810-1241`, als Restrisiko des dort gewaehlten Umbaus
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs`
**Zusammenhang:** `issues/260810-1241_*_der-rueckgaengigstapel-haelt-je-eigener-handlung-eine-ganze-abschrift-und-ist-unbegrenzt.md`

## Belegstellen

`Editorbereich::alle_treffer_ersetzen` benennt den Fall im Doc-Kommentar, und die
Rechnung steht dort:

```text
  Suchtext `a`, Ersatztext `aa`
  Ruf 1: jedes `a` wird `aa`     erster Treffer nahe dem Anfang,
                                 letzter nahe dem Ende
  Ruf 2: jedes `a` wird `aa`     dieselbe Lage, und der Stand ist gewachsen
  …
```

`Umkehrpunkt::zwischen` bildet den Bereich aus dem gemeinsamen Anfang und dem
gemeinsamen Schwanz beider Staende. Liegen die geaenderten Stellen ueber die
ganze Datei verteilt, sind beide kurz, und `entfernt` ist so lang wie die Datei.

`levelsOfUndo` steht bei einem `NSUndoManager` ab Werk auf `0`, also unbegrenzt;
`setLevelsOfUndo` steht nirgends im Baum, und der Grund dafuer steht an
`Umkehrpunkt`.

## Fehlszenario

Eine Datei nahe der Editorgrenze von 16 MB, `cmd+f` nach einem haeufigen
Buchstaben, ein Ersatztext, der ihn enthaelt, dann `ctrl+cmd+r` mehrfach. Je Ruf
kommt ein Bereich in Dateigroesse in den Stapel, und die Datei waechst dabei.

Der Fall ist am Code belegt und nicht gefahren. Er ist **nicht** der Fall aus
`260810-1241`: dort waren es hundert einzelne Ersetzungen mit `shift+cmd+r`, und
die halten seit dem Umbau je drei Bytes. Hier braucht es einen Ersatztext, der
den Suchtext enthaelt, und einen Nutzer, der den Sammelbefehl wiederholt.

## Warum keine Tiefengrenze hilft

`setLevelsOfUndo` begrenzt die Zahl der Handlungen und nicht die Bytes. Bei einer
Grenze von hundert Handlungen und einer Datei von 16 MB bliebe das Produkt
1,6 GB — dieselbe Zahl, die `260810-1241` gefunden hat. Dazu gaelte die Grenze
fuer den ganzen Verwalter und damit auch fuer das Tippen, dessen Tiefe heute
unbegrenzt ist und von keinem Abnahmekriterium beschraenkt wird.

## Was zu pruefen waere

Drei Wege, keiner davon empfohlen, weil keiner gemessen ist:

1. **Mehrere Bereiche je Handlung statt eines.** Ein Sammelersetzen kennt seine
   Stellen; ein Umkehrpunkt aus einer Liste von Bereichen waere in der Groesse
   des Ersetzten. Der Preis: `appkit/editor.rs` muesste die Stellen erfahren, und
   heute weiss sie allein `krk_core::text::suche`. Das ist ein Umbau an der
   Grenze zwischen Kern und Oberflaeche.
2. **Eine Schranke in Bytes ueber dem eigenen Stapel.** Sie verlangte einen
   eigenen Stapel neben dem des `NSUndoManager`, und der Modulkopf von
   `appkit/editor.rs` schliesst genau das aus: ein zweiter Verwalter truege den
   Umbau in einen anderen Stapel als das Tippen.
3. **Nichts tun und den Fall benannt lassen.** Das ist der heutige Stand. Er ist
   vertretbar, solange niemand gemessen hat, dass ein Nutzer diesen Weg geht.

Die Entscheidung darueber gehoert nicht in eine Behebung; sie ist die Frage, was
ein Editor an seiner Grenze von 16 MB an Speicher halten darf, und die
Durchsicht `260810-1248` fuehrt sie unter den uebergreifenden Beobachtungen.

---
Resolved: Der Stapel traegt seit `260810-1341` ein Budget in **Bytes**, und der
Fall ist damit gedeckelt statt angenommen. Drei Stuecke in
`crates/krk-ui/src/appkit/editor.rs`, keine Zeile ausserhalb:

- `STAPELBUDGET` — wie viele Bytes die angemeldeten Handlungen zusammen halten
  duerfen. Die Zahl ist `krk_core::text::datei::EDITORGRENZE`, also die
  Dateigrenze des Editors aus C2, und sie ist geliehen und nicht gewaehlt: ein
  Verlauf, der mehr haelt als die groesste Datei, die dieser Editor ueberhaupt
  oeffnet, kostet mehr als der Gegenstand, um den es geht.
- `Stapellast` — die Huelle, die im angemeldeten Block wohnt und die Bytes ihres
  Punktes an einem `Rc<Cell<usize>>` an- und in ihrem `Drop` abtraegt.
- `verlauf_fuer_umbau` — die Regel: passt der Punkt neben das, was schon im
  Stapel steht, wird er als `Verlauf::Traegt` angemeldet; passt er nicht, geht er
  als `Verlauf::TraegtNurDiese` durch die eine Schreibstelle und steht danach
  allein im Stapel. Beide Ersetzungsbefehle aus C5 gehen durch diese Stelle.

## Die Messung, vorher und nachher

`der_stapel_haelt_hoechstens_das_budget_und_die_letzte_handlung` faehrt den Fall
dieses Datensatzes an der Editorgrenze: eine Datei von 16 777 216 Bytes, ein `a`
nahe dem Anfang und ein `a` nahe dem Ende, Suchtext `a`, Ersatztext `aa`, ersetzt
mit `krk_core::text::suche::alle_ersetzen` — der Funktion hinter `ctrl+cmd+r`.
Gemessen wird die Summe, die die Handlungen im Stapel halten, nicht geschaetzt:

```text
  je Ruf                              16 777 214 B    gemessen, in allen drei Rufen gleich
  3 Rufe, ohne Budget (bis 260810-1341)   50 331 642 B
  3 Rufe, mit Budget                      16 777 214 B    zweimal geraeumt
  100 Rufe, ohne Budget                1 677 721 400 B    100 × die gemessene Zahl
  100 Rufe, mit Budget                    16 777 214 B    unabhaengig von der Zahl der Rufe
```

**Fuehlbar wurde es ab dem zweiten Ruf**, und das ist keine Schaetzung, sondern
die Teilung: ein Punkt ist an dieser Datei so gross wie das Budget, also greift
das Budget beim zweiten Ruf und bei jedem weiteren. An kleineren Dateien stehen
mehr nebeneinander — 16 an einer Datei von 1 MB, 64 an einer von 256 kB —, und die
Schranke bleibt dieselbe.

Die obere Schranke ist `STAPELBUDGET` **plus eine Handlung** und nicht
`STAPELBUDGET`: die Handlung, die das Budget sprengt, wird nicht abgewiesen,
sondern raeumt vor sich auf. Ein Ersetzen, das nicht ruecknehmbar waere,
widerspraeche C5; ein Verlauf, der davor faellt, widerspricht ihm nicht. Was der
Nutzer merkt: das erste `cmd+z` nimmt das letzte Sammelersetzen zurueck, ein
zweites tut nichts — dieselbe Wirkung, die das CRLF-Richten seit `260810-1044`
hat, und derselbe Wert `Verlauf::TraegtNurDiese` dahinter.

## Was aus den drei Wegen des Datensatzes geworden ist

**Weg 2 ist genommen, und seine Ablehnung oben war falsch.** Der Datensatz hat
ihn verworfen, weil eine Schranke in Bytes „einen eigenen Stapel neben dem des
`NSUndoManager`" verlange. Sie verlangt keinen: sie verlangt einen **Zaehler**
und den Raeumungsweg, den diese Datei schon hatte. Ein zweiter Verwalter entsteht
nicht, die Handlungen stehen weiter in demselben Stapel wie das Tippen, und der
Modulkopf bleibt, wie er ist.

**Weg 1 ist nicht genommen, und der Grund ist eine Rechnung und keine
Zustaendigkeit.** Eine Liste der einzelnen Stellen kostet je Stelle einen
Versatz. An derselben Datei von 16 MB, in der guenstigsten Form gerechnet — nur
die Versaetze, Such- und Ersatztext einmal daneben:

```text
  Abstand der Treffer   Treffer     ein Bereich   eine Liste (8 B je Stelle)
          16 Bytes      1 048 576      16,0 MB       8,0 MB
           8 Bytes      2 097 152      16,0 MB      16,0 MB
           4 Bytes      4 194 304      16,0 MB      32,0 MB
```

Der Umschlag liegt bei einem Treffer je acht Bytes, und **darunter ist die Liste
teurer als der Bereich**. Genau in dem Fall dieses Datensatzes — ein haeufiger
Buchstabe — liegt der Abstand der Treffer unter acht Bytes. Die Liste loeste den
Fall also nicht, sondern verschoebe ihn, und sie kostete dabei den Umbau an der
Grenze zwischen Kern und Oberflaeche, den der Datensatz oben nennt.

**Weg 3, nichts tun, ist damit nicht mehr der Stand.** Der Satz „ein
Sammelersetzen, das den ganzen Text aendert, muss den ganzen Text aufheben, wenn
es ruecknehmbar sein soll" ist richtig — fuer **ein** Sammelersetzen. Der Defekt
war nicht das eine, sondern die Summe ohne Grenze, und die Summe ist begrenzbar,
ohne einem einzigen die Ruecknahme zu nehmen.

## Was das Tippen davon merkt: nichts

Der Zaehler zaehlt allein die Handlungen, die `Editorbereich::umkehrung_anmelden`
anmeldet — die vier Anlaesse aus `Verlauf` —, und nicht die, die die `NSTextView`
fuer jeden Anschlag selbst anmeldet. Ein Anschlag laesst den Zaehler auf null
stehen, und `0` plus ein gewoehnlicher Punkt liegt unter jedem Budget; gemessen
von `ein_gewoehnlicher_umbau_bleibt_neben_dem_verlauf_und_erst_der_volle_stapel_wird_geraeumt`.
`setLevelsOfUndo` steht weiter nirgends, und der Grund dafuer ist derselbe
geblieben.

Ein `cmd+z` fragt das Budget nicht: `Editorbereich::umkehren` meldet seinen
Gegenweg unveraendert als `Verlauf::Traegt` an. Ein Rueckgaengig nimmt eine
Handlung vom einen Stapel und legt eine von derselben Groesse auf den anderen,
die Summe bleibt also stehen, und ein Budget, das dort zugriffe, koennte einem
Nutzer, der `cmd+z` und `shift+cmd+z` gegeneinander laufen laesst, den Verlauf
nehmen.

## Was offen bleibt

Dass der `NSUndoManager` den angemeldeten Block mit der Handlung wieder freigibt,
ist geschlossen und nicht gemessen; die Messung braeuchte eine fuenfte Probe mit
`MainThreadMarker::new_unchecked`, und darueber steht die offene Frage
`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`.
Der neue Datensatz dazu ist
`issues/260810-1341_o_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`.
**Die Schranke haengt an der Annahme nicht**: traefe sie nicht zu, ginge der
Zaehler nur hoch und nie herunter, das Budget griffe bei jedem Umbau, und der
Stapel hielte statt „Budget plus eine Handlung" genau eine Handlung. Falsch waere
dann die Tiefe des Verlaufs und nicht die Schranke.

Keine der zehn Zeitzusagen aus C8 der Runde 1 ist beruehrt: hinzugekommen sind je
Ersetzen eine Addition und ein Vergleich auf `usize`, und keine Zusage liegt auf
dem Ersetzungsweg.

Abnahme: `cargo build --workspace`, `cargo test --workspace` (753 Proben),
`cargo clippy --workspace --all-targets`, `cargo fmt -p krk-ui -- --check` — alle
vier exit 0.
