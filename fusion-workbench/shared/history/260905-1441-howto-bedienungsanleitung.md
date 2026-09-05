# HowTo.md: eine kompakte Bedienungsanleitung im Projektwurzelverzeichnis

**Agent:** coder
**Datum:** 260905-1441
**Status:** Complete
**Circle:** keiner aktiv, Artefakte nach `shared/`

## Auftrag

Eine kompakte Bedienungsanleitung für den Nutzer von KRK, neun vom Nutzer
genannte Gegenstände, dazu zwei Betriebsregeln, die er sonst schmerzhaft lernt.
Abgegrenzt gegen `README.md`, die die Arbeit **am** Programm beschreibt.

## Ergebnis

`HowTo.md` im Projektwurzelverzeichnis, deutsch, mit diesen Abschnitten: Vor dem
Aktualisieren, Wo KRK seine eigenen Dateien ablegt, Der Dateilistenfilter, Die
Tastaturbelegung, Editor und Vorschau, Der Git-Bereich, Bereiche ein- und
ausblenden, Lesezeichen, Leseprofile, Der Notizzettel.

## Wie geprüft wurde

Jede genannte Tastenkombination stammt aus `resources/default-keymap.toml` und
aus keiner zweiten Quelle. `make tasten` wurde nicht gerufen: es ist der
interaktive Tastenlogger und verlangt KRK im Vordergrund.

Jedes beschriebene Verhalten ist am Baum gelesen. Die tragenden Stellen:

- `crates/krk-core/src/verzeichnis/filter.rs` (Zeichenregel, Musterabgleich,
  `inhaltsschwelle` mit 3 und 5)
- `crates/krk-core/src/verzeichnis/modell.rs` (`Ordnermodell::neu`, `tief: true`)
- `crates/krk-ui/src/kommandos/rueckschritt.rs` (die Tafel aus acht Fällen)
- `crates/krk-ui/src/appkit/anwendung.rs`, `fn abbrechen` (die drei Ränge von Esc)
- `crates/krk-core/src/zwischenablage.rs` (die fünf Schritte von `filtertext_aus`)
- `crates/krk-ui/src/appkit/belegungsansicht.rs` (Blatt, Suche, `SCHALTFLAECHEN`)
- `crates/krk-ui/src/kommandos/rundweg.rs` (die drei Wege von `cmd+e`)
- `crates/krk-ui/src/appkit/aufteilung.rs`, `crates/krk-ui/src/fenstermodell.rs`
  (die Fensterzeile, die Anteilsregel, das letzte sichtbare Dateifenster)
- `crates/krk-core/src/ablage/sitzung.rs` (Git-Bereich ab Werk aus, Markenspalte
  ab Werk an)
- `crates/krk-ui/src/appkit/anwendung.rs`, `fn gitbedarf_nachziehen` (die
  Oder-Verknüpfung von Bereich und Spalte)
- `crates/krk-core/src/ablage/pfade.rs` (`Datei::ALLE`)
- `crates/krk-ui/src/appkit/anwendung.rs`, `fn zettel_sichern` (die vier
  Sicherungsmomente)

Drei Bereiche sind über Analysten geprüft worden: der Git-Bereich, die
Leseprofile samt Ablage, und die Belegungsansicht.

## Keine Zahl, die veraltet

Statt Zahlen stehen in der Datei die Zählkommandos oder die Aufzählungen selbst:
die Ablagedateien namentlich mit `awk` auf `Datei::ALLE`, die Zahl der
Leseprofile mit `grep -c '^\[\[profil\]\]'`, die Schwungweite des Verlaufs als
Verweis auf `VERLAUFSSCHRITT`. Zwei Zahlen sind beim Gegenlesen entfernt worden:
„die fünfte Spalte" ist zu „die Spalte „Marke"" geworden, „sechs Bereiche" zur
bloßen Aufzählung der Bereiche.

## Verweise statt Wiederholung

Zwei Wortlaute stehen ausdrücklich **nicht** in `HowTo.md`, sondern als Verweis:

1. Die Betriebsregel beim Installieren. Sie steht in `README.md` unter
   „Herunterladen und installieren" und als `RELEASETEXT` in
   `xtask/src/veroeffentlichung.rs`. Eine dritte Fassung liefe von beiden weg.
2. Der Dreischritt, mit dem man neue Leseprofile übernimmt. Er steht in
   `README.md` unter „Neue Leseprofile übernehmen".

Beide Stellen sind in `HowTo.md` in der Sache genannt und im Wortlaut verwiesen.

## Verification

`make check` — exit 0. Ein Basislauf vor der Änderung lief ebenfalls auf exit 0.
Die Zählproben über den Baum lesen allein `crates/**/*.rs`
(`crates/krk-core/tests/gemeinsam/mod.rs`, `quelldateien`), eine neue
Markdown-Datei im Wurzelverzeichnis berührt sie nicht.

## Nebenbefunde, nicht behoben

Drei Stellen sind beim Nachlesen als überholt aufgefallen. Keine ist angefasst
worden, weil der Auftrag auf `HowTo.md` beschränkt war:

1. `README.md:6` sagt „Eine Git-Anbindung ist vorgesehen und noch nicht gebaut".
   Die Runde 23 (`circles/260830-1045-git-bereich-liest-status-branch-verlauf`,
   `_b_`) hat sie als Lesestufe gebaut.
2. `CLAUDE.md` nennt für `Bereich` fünf Werte und für `Fokus` fünf. Der Baum
   trägt für beide sechs, seit `Bereich::Git` und `Fokus::Git`.
3. Die Rundentabelle in `CLAUDE.md` endet bei der Runde 22. Die Runde 23 und die
   Arbeit vom 260901 bis 260905 fehlen darin.
