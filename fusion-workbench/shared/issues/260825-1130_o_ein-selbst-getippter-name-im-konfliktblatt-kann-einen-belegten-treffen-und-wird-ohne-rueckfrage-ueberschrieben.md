Ein selbst getippter Name im Konfliktblatt kann einen belegten treffen und wird ohne Rueckfrage ueberschrieben

---

Waehlt der Nutzer im Konfliktblatt "Umbenennen in" und tippt den Namen selbst, wird der neue Name
nicht noch einmal auf einen Konflikt geprueft. Trifft er einen belegten Namen, schneidet das
Kopieren und das Packen die getroffene Datei mit `File::create` ab, und das Entpacken schreibt in
den getroffenen Ordner hinein. Beides ohne Papierkorb und ohne zweite Rueckfrage.

---

**Filed by:** orchestrator, aus einem Nebenbefund der Durchsicht `260825-0942`

## Wo es steht

- `crates/krk-core/src/operation/zippen.rs`, Zweig `Konfliktantwort::UmbenennenIn` — der Doc-Kommentar
  schreibt ausdruecklich aus, dass nicht ein zweites Mal gefragt wird.
- `crates/krk-core/src/operation/entpacken.rs` — tut stillschweigend dasselbe.
- `crates/krk-core/src/operation/mod.rs`, `ziel_klaeren` — dieselbe Stelle fuer das Kopieren und
  Verschieben, und dort ist das Verhalten aelter als die Runde 17.

## Warum es kein Befund der Runde 17 ist

Die Wurzel liegt im Kopiervorgang und ist aelter als diese Runde; Zip und Unzip haben sie geerbt,
weil sie sich in dieselbe Vorgangsmaschine einfuegen. Deshalb liegt dieser Datensatz im
gemeinsamen Speicher und nicht im Circle: er ist neben der Directive gefunden und nicht aus ihr
entstanden.

## Wo die Zusage haelt und wo nicht

Bei `Konfliktregel::AutomatischUmbenennen` liefert `freier_name` einen freien Namen, und die Zusage
haelt. Allein der von Hand getippte Name kann daneben greifen.

## Was zu entscheiden waere

Ob der getippte Name ein zweites Mal gegen den Bestand geprueft wird, und ob eine Kette von
Rueckfragen ueber dieselbe eine Datei noch eine Auskunft ist. Der Doc-Kommentar in `zippen.rs`
verneint das Zweite ausdruecklich; er beantwortet aber nicht, was bei einem Treffer geschehen soll.
Gefunden am 260825 in der Durchsicht der Runde 17, Abschnitt "Nebenbei, im selben Zweig" des
Datensatzes `circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0942_c_ueberschreiben-loescht-beim-packen-endgueltig-und-beim-entpacken-in-den-papierkorb.md`.

Also seen: 260825-1144 by coderev — beim Packen kommt eine zweite Wirkung dazu: bricht der Nutzer den Lauf ab, loescht `zippen::halbes_archiv_wegraeumen` (`crates/krk-core/src/operation/zippen.rs:259-268`) die getroffene Datei mit `fs::remove_file` **endgueltig**, also ohne den Papierkorb, den derselbe Zweig seit dem 260825 sonst nimmt.
